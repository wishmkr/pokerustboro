//! Translated from `src/pokedex.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): gPokedexOrder_Alphabetical gPokedexOrder_Weight gPokedexOrder_Height sOamData_ScrollBar sOamData_ScrollArrow sOamData_InterfaceText sOamData_RotatingPokeBall sOamData_SeenOwnText sOamData_Dex8x16 sSpriteAnim_ScrollBar sSpriteAnim_ScrollArrow sSpriteAnim_RotatingPokeBall sSpriteAnim_StartButton sSpriteAnim_SearchText sSpriteAnim_SelectButton sSpriteAnim_MenuText sSpriteAnim_SeenText sSpriteAnim_OwnText sSpriteAnim_HoennText sSpriteAnim_NationalText sSpriteAnim_HoennSeenOwnDigit0 sSpriteAnim_HoennSeenOwnDigit1 sSpriteAnim_HoennSeenOwnDigit2 sSpriteAnim_HoennSeenOwnDigit3 sSpriteAnim_HoennSeenOwnDigit4 sSpriteAnim_HoennSeenOwnDigit5 sSpriteAnim_HoennSeenOwnDigit6 sSpriteAnim_HoennSeenOwnDigit7 sSpriteAnim_HoennSeenOwnDigit8 sSpriteAnim_HoennSeenOwnDigit9 sSpriteAnim_NationalSeenOwnDigit0 sSpriteAnim_NationalSeenOwnDigit1 sSpriteAnim_NationalSeenOwnDigit2 sSpriteAnim_NationalSeenOwnDigit3 sSpriteAnim_NationalSeenOwnDigit4 sSpriteAnim_NationalSeenOwnDigit5 sSpriteAnim_NationalSeenOwnDigit6 sSpriteAnim_NationalSeenOwnDigit7 sSpriteAnim_NationalSeenOwnDigit8 sSpriteAnim_NationalSeenOwnDigit9 sSpriteAnim_DexListStartMenuCursor sSpriteAnimTable_ScrollBar sSpriteAnimTable_ScrollArrow sSpriteAnimTable_RotatingPokeBall sSpriteAnimTable_InterfaceText sSpriteAnimTable_SeenOwnText sSpriteAnimTable_HoennNationalText sSpriteAnimTable_HoennSeenOwnNumber sSpriteAnimTable_NationalSeenOwnNumber sSpriteAnimTable_DexListStartMenuCursor sScrollBarSpriteTemplate sScrollArrowSpriteTemplate sInterfaceTextSpriteTemplate sRotatingPokeBallSpriteTemplate sSeenOwnTextSpriteTemplate sHoennNationalTextSpriteTemplate sHoennDexSeenOwnNumberSpriteTemplate sNationalDexSeenOwnNumberSpriteTemplate sDexListStartMenuCursorSpriteTemplate sInterfaceSpriteSheet sInterfaceSpritePalette sScrollMonIncrements sScrollTimers sPokedex_BgTemplate sPokemonList_WindowTemplate sText_No000 sCaughtBall_Gfx sText_TenDashes sExpandedPlaceholder_PokedexDescription gDummyPokedexText gBulbasaurPokedexText gIvysaurPokedexText gVenusaurPokedexText gCharmanderPokedexText gCharmeleonPokedexText gCharizardPokedexText gSquirtlePokedexText gWartortlePokedexText gBlastoisePokedexText gCaterpiePokedexText gMetapodPokedexText gButterfreePokedexText gWeedlePokedexText gKakunaPokedexText gBeedrillPokedexText gPidgeyPokedexText gPidgeottoPokedexText gPidgeotPokedexText gRattataPokedexText gRaticatePokedexText gSpearowPokedexText gFearowPokedexText gEkansPokedexText gArbokPokedexText gPikachuPokedexText gRaichuPokedexText gSandshrewPokedexText gSandslashPokedexText gNidoranFPokedexText gNidorinaPokedexText gNidoqueenPokedexText gNidoranMPokedexText gNidorinoPokedexText gNidokingPokedexText gClefairyPokedexText gClefablePokedexText gVulpixPokedexText gNinetalesPokedexText gJigglypuffPokedexText gWigglytuffPokedexText gZubatPokedexText gGolbatPokedexText gOddishPokedexText gGloomPokedexText gVileplumePokedexText gParasPokedexText gParasectPokedexText gVenonatPokedexText gVenomothPokedexText gDiglettPokedexText gDugtrioPokedexText gMeowthPokedexText gPersianPokedexText gPsyduckPokedexText gGolduckPokedexText gMankeyPokedexText gPrimeapePokedexText gGrowlithePokedexText gArcaninePokedexText gPoliwagPokedexText gPoliwhirlPokedexText gPoliwrathPokedexText gAbraPokedexText gKadabraPokedexText gAlakazamPokedexText gMachopPokedexText gMachokePokedexText gMachampPokedexText gBellsproutPokedexText gWeepinbellPokedexText gVictreebelPokedexText gTentacoolPokedexText gTentacruelPokedexText gGeodudePokedexText gGravelerPokedexText gGolemPokedexText gPonytaPokedexText gRapidashPokedexText gSlowpokePokedexText gSlowbroPokedexText gMagnemitePokedexText gMagnetonPokedexText gFarfetchdPokedexText gDoduoPokedexText gDodrioPokedexText gSeelPokedexText gDewgongPokedexText gGrimerPokedexText gMukPokedexText gShellderPokedexText gCloysterPokedexText gGastlyPokedexText gHaunterPokedexText gGengarPokedexText gOnixPokedexText gDrowzeePokedexText gHypnoPokedexText gKrabbyPokedexText gKinglerPokedexText gVoltorbPokedexText gElectrodePokedexText gExeggcutePokedexText gExeggutorPokedexText gCubonePokedexText gMarowakPokedexText gHitmonleePokedexText gHitmonchanPokedexText gLickitungPokedexText gKoffingPokedexText gWeezingPokedexText gRhyhornPokedexText gRhydonPokedexText gChanseyPokedexText gTangelaPokedexText gKangaskhanPokedexText gHorseaPokedexText gSeadraPokedexText gGoldeenPokedexText gSeakingPokedexText gStaryuPokedexText gStarmiePokedexText gMrMimePokedexText gScytherPokedexText gJynxPokedexText gElectabuzzPokedexText gMagmarPokedexText gPinsirPokedexText gTaurosPokedexText gMagikarpPokedexText gGyaradosPokedexText gLaprasPokedexText gDittoPokedexText gEeveePokedexText gVaporeonPokedexText gJolteonPokedexText gFlareonPokedexText gPorygonPokedexText gOmanytePokedexText gOmastarPokedexText gKabutoPokedexText gKabutopsPokedexText gAerodactylPokedexText gSnorlaxPokedexText gArticunoPokedexText gZapdosPokedexText gMoltresPokedexText gDratiniPokedexText gDragonairPokedexText gDragonitePokedexText gMewtwoPokedexText gMewPokedexText gChikoritaPokedexText gBayleefPokedexText gMeganiumPokedexText gCyndaquilPokedexText gQuilavaPokedexText gTyphlosionPokedexText gTotodilePokedexText gCroconawPokedexText gFeraligatrPokedexText gSentretPokedexText gFurretPokedexText gHoothootPokedexText gNoctowlPokedexText gLedybaPokedexText gLedianPokedexText gSpinarakPokedexText gAriadosPokedexText gCrobatPokedexText gChinchouPokedexText gLanturnPokedexText gPichuPokedexText gCleffaPokedexText gIgglybuffPokedexText gTogepiPokedexText gTogeticPokedexText gNatuPokedexText gXatuPokedexText gMareepPokedexText gFlaaffyPokedexText gAmpharosPokedexText gBellossomPokedexText gMarillPokedexText gAzumarillPokedexText gSudowoodoPokedexText gPolitoedPokedexText gHoppipPokedexText gSkiploomPokedexText gJumpluffPokedexText gAipomPokedexText gSunkernPokedexText gSunfloraPokedexText gYanmaPokedexText gWooperPokedexText gQuagsirePokedexText gEspeonPokedexText gUmbreonPokedexText gMurkrowPokedexText gSlowkingPokedexText gMisdreavusPokedexText gUnownPokedexText gWobbuffetPokedexText gGirafarigPokedexText gPinecoPokedexText gForretressPokedexText gDunsparcePokedexText gGligarPokedexText gSteelixPokedexText gSnubbullPokedexText gGranbullPokedexText gQwilfishPokedexText gScizorPokedexText gShucklePokedexText gHeracrossPokedexText gSneaselPokedexText gTeddiursaPokedexText gUrsaringPokedexText gSlugmaPokedexText gMagcargoPokedexText gSwinubPokedexText gPiloswinePokedexText gCorsolaPokedexText gRemoraidPokedexText gOctilleryPokedexText gDelibirdPokedexText gMantinePokedexText gSkarmoryPokedexText gHoundourPokedexText gHoundoomPokedexText gKingdraPokedexText gPhanpyPokedexText gDonphanPokedexText gPorygon2PokedexText gStantlerPokedexText gSmearglePokedexText gTyroguePokedexText gHitmontopPokedexText gSmoochumPokedexText gElekidPokedexText gMagbyPokedexText gMiltankPokedexText gBlisseyPokedexText gRaikouPokedexText gEnteiPokedexText gSuicunePokedexText gLarvitarPokedexText gPupitarPokedexText gTyranitarPokedexText gLugiaPokedexText gHoOhPokedexText gCelebiPokedexText gTreeckoPokedexText gGrovylePokedexText gSceptilePokedexText gTorchicPokedexText gCombuskenPokedexText gBlazikenPokedexText gMudkipPokedexText gMarshtompPokedexText gSwampertPokedexText gPoochyenaPokedexText gMightyenaPokedexText gZigzagoonPokedexText gLinoonePokedexText gWurmplePokedexText gSilcoonPokedexText gBeautiflyPokedexText gCascoonPokedexText gDustoxPokedexText gLotadPokedexText gLombrePokedexText gLudicoloPokedexText gSeedotPokedexText gNuzleafPokedexText gShiftryPokedexText gTaillowPokedexText gSwellowPokedexText gWingullPokedexText gPelipperPokedexText gRaltsPokedexText gKirliaPokedexText gGardevoirPokedexText gSurskitPokedexText gMasquerainPokedexText gShroomishPokedexText gBreloomPokedexText gSlakothPokedexText gVigorothPokedexText gSlakingPokedexText gNincadaPokedexText gNinjaskPokedexText gShedinjaPokedexText gWhismurPokedexText gLoudredPokedexText gExploudPokedexText gMakuhitaPokedexText gHariyamaPokedexText gAzurillPokedexText gNosepassPokedexText gSkittyPokedexText gDelcattyPokedexText gSableyePokedexText gMawilePokedexText gAronPokedexText gLaironPokedexText gAggronPokedexText gMedititePokedexText gMedichamPokedexText gElectrikePokedexText gManectricPokedexText gPluslePokedexText gMinunPokedexText gVolbeatPokedexText gIllumisePokedexText gRoseliaPokedexText gGulpinPokedexText gSwalotPokedexText gCarvanhaPokedexText gSharpedoPokedexText gWailmerPokedexText gWailordPokedexText gNumelPokedexText gCameruptPokedexText gTorkoalPokedexText gSpoinkPokedexText gGrumpigPokedexText gSpindaPokedexText gTrapinchPokedexText gVibravaPokedexText gFlygonPokedexText gCacneaPokedexText gCacturnePokedexText gSwabluPokedexText gAltariaPokedexText gZangoosePokedexText gSeviperPokedexText gLunatonePokedexText gSolrockPokedexText gBarboachPokedexText gWhiscashPokedexText gCorphishPokedexText gCrawdauntPokedexText gBaltoyPokedexText gClaydolPokedexText gLileepPokedexText gCradilyPokedexText gAnorithPokedexText gArmaldoPokedexText gFeebasPokedexText gMiloticPokedexText gCastformPokedexText gKecleonPokedexText gShuppetPokedexText gBanettePokedexText gDuskullPokedexText gDusclopsPokedexText gTropiusPokedexText gChimechoPokedexText gAbsolPokedexText gWynautPokedexText gSnoruntPokedexText gGlaliePokedexText gSphealPokedexText gSealeoPokedexText gWalreinPokedexText gClamperlPokedexText gHuntailPokedexText gGorebyssPokedexText gRelicanthPokedexText gLuvdiscPokedexText gBagonPokedexText gShelgonPokedexText gSalamencePokedexText gBeldumPokedexText gMetangPokedexText gMetagrossPokedexText gRegirockPokedexText gRegicePokedexText gRegisteelPokedexText gLatiasPokedexText gLatiosPokedexText gKyogrePokedexText gGroudonPokedexText gRayquazaPokedexText gJirachiPokedexText gDeoxysPokedexText gPokedexEntries sSizeScreenSilhouette_Pal sInfoScreen_BgTemplate sInfoScreen_WindowTemplates sNewEntryInfoScreen_BgTemplate sNewEntryInfoScreen_WindowTemplates sText_TenDashes2 gMonFootprintTable sLetterSearchRanges sSearchMenuTopBarItems sSearchMenuItems sSearchMovementMap_SearchNatDex sSearchMovementMap_ShiftNatDex sSearchMovementMap_SearchHoennDex sSearchMovementMap_ShiftHoennDex sDexModeOptions sDexOrderOptions sDexSearchNameOptions sDexSearchColorOptions sDexSearchTypeOptions sPokedexModes sOrderOptions sDexSearchTypeIds sSearchOptions sSearchMenu_BgTemplate sSearchMenu_WindowTemplate

/// `struct PokedexView`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokedexView {
    pub pokedexList: CArray<PokedexListItem, 387>,
    pub pokemonListCount: u16,
    pub selectedPokemon: u16,
    pub selectedPokemonBackup: u16,
    pub dexMode: u16,
    pub dexModeBackup: u16,
    pub dexOrder: u16,
    pub dexOrderBackup: u16,
    pub seenCount: u16,
    pub ownCount: u16,
    pub monSpriteIds: CArray<u16, 4>,
    pub selectedMonSpriteId: u16,
    pub pokeBallRotationStep: u16,
    pub pokeBallRotationBackup: u16,
    pub pokeBallRotation: u8,
    pub initialVOffset: u8,
    pub scrollTimer: u8,
    pub scrollDirection: u8,
    pub listVOffset: i16,
    pub listMovingVOffset: i16,
    pub scrollMonIncrement: u16,
    pub maxScrollTimer: u16,
    pub scrollSpeed: u16,
    pub unkArr1: CArray<u16, 4>,
    pub filler: CArray<u8, 8>,
    pub currentPage: u8,
    pub currentPageBackup: u8,
    bits_1612: u8,
    pub selectedScreen: u8,
    pub screenSwitchState: u8,
    pub menuIsOpen: u8,
    pub menuCursorPos: u16,
    pub menuY: i16,
    pub unkArr2: CArray<u8, 8>,
    pub unkArr3: CArray<u8, 8>,
}

impl PokedexView {
    #[inline(always)]
    pub fn isSearchResults(&self) -> u8 {
        ((self.bits_1612 as u32 >> 0) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_isSearchResults(&mut self, v: u8) {
        self.bits_1612 = (self.bits_1612 & !(0x1 << 0)) | ((v as u8 & 0x1) << 0);
    }
}

unsafe impl Sync for PokedexView {}

/// `struct PokedexListItem`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct PokedexListItem {
    pub dexNum: u16,
    bits_2: u8,
}

impl PokedexListItem {
    #[inline(always)]
    pub fn seen(&self) -> u16 {
        ((self.bits_2 as u32 >> 0) & 0x1) as u16
    }
    #[inline(always)]
    pub fn set_seen(&mut self, v: u16) {
        self.bits_2 = (self.bits_2 & !(0x1 << 0)) | ((v as u8 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn owned(&self) -> u16 {
        ((self.bits_2 as u32 >> 1) & 0x1) as u16
    }
    #[inline(always)]
    pub fn set_owned(&mut self, v: u16) {
        self.bits_2 = (self.bits_2 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
}

unsafe impl Sync for PokedexListItem {}

/// `struct SearchOptionText`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SearchOptionText {
    pub description: *mut u8,
    pub title: *mut u8,
}

unsafe impl Sync for SearchOptionText {}

/// `struct SearchMenuItem`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SearchMenuItem {
    pub description: *mut u8,
    pub titleBgX: u8,
    pub titleBgY: u8,
    pub titleBgWidth: u8,
    pub selectionBgX: u8,
    pub selectionBgY: u8,
    pub selectionBgWidth: u8,
}

unsafe impl Sync for SearchMenuItem {}

/// `struct SearchMenuTopBarItem`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SearchMenuTopBarItem {
    pub description: *mut u8,
    pub highlightX: u8,
    pub highlightY: u8,
    pub highlightWidth: u8,
}

unsafe impl Sync for SearchMenuTopBarItem {}

/// `struct SearchOption`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SearchOption {
    pub texts: *mut SearchOptionText,
    pub taskDataCursorPos: u8,
    pub taskDataScrollOffset: u8,
    pub numOptions: u16,
}

unsafe impl Sync for SearchOption {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PokedexView>() == 1636);
    assert!(offset_of!(PokedexView, pokedexList) == 0);
    assert!(offset_of!(PokedexView, pokemonListCount) == 1548);
    assert!(offset_of!(PokedexView, selectedPokemon) == 1550);
    assert!(offset_of!(PokedexView, selectedPokemonBackup) == 1552);
    assert!(offset_of!(PokedexView, dexMode) == 1554);
    assert!(offset_of!(PokedexView, dexModeBackup) == 1556);
    assert!(offset_of!(PokedexView, dexOrder) == 1558);
    assert!(offset_of!(PokedexView, dexOrderBackup) == 1560);
    assert!(offset_of!(PokedexView, seenCount) == 1562);
    assert!(offset_of!(PokedexView, ownCount) == 1564);
    assert!(offset_of!(PokedexView, monSpriteIds) == 1566);
    assert!(offset_of!(PokedexView, selectedMonSpriteId) == 1574);
    assert!(offset_of!(PokedexView, pokeBallRotationStep) == 1576);
    assert!(offset_of!(PokedexView, pokeBallRotationBackup) == 1578);
    assert!(offset_of!(PokedexView, pokeBallRotation) == 1580);
    assert!(offset_of!(PokedexView, initialVOffset) == 1581);
    assert!(offset_of!(PokedexView, scrollTimer) == 1582);
    assert!(offset_of!(PokedexView, scrollDirection) == 1583);
    assert!(offset_of!(PokedexView, listVOffset) == 1584);
    assert!(offset_of!(PokedexView, listMovingVOffset) == 1586);
    assert!(offset_of!(PokedexView, scrollMonIncrement) == 1588);
    assert!(offset_of!(PokedexView, maxScrollTimer) == 1590);
    assert!(offset_of!(PokedexView, scrollSpeed) == 1592);
    assert!(offset_of!(PokedexView, unkArr1) == 1594);
    assert!(offset_of!(PokedexView, filler) == 1602);
    assert!(offset_of!(PokedexView, currentPage) == 1610);
    assert!(offset_of!(PokedexView, currentPageBackup) == 1611);
    assert!(offset_of!(PokedexView, bits_1612) == 1612);
    assert!(offset_of!(PokedexView, selectedScreen) == 1613);
    assert!(offset_of!(PokedexView, screenSwitchState) == 1614);
    assert!(offset_of!(PokedexView, menuIsOpen) == 1615);
    assert!(offset_of!(PokedexView, menuCursorPos) == 1616);
    assert!(offset_of!(PokedexView, menuY) == 1618);
    assert!(offset_of!(PokedexView, unkArr2) == 1620);
    assert!(offset_of!(PokedexView, unkArr3) == 1628);
    assert!(size_of::<PokedexListItem>() == 4);
    assert!(offset_of!(PokedexListItem, dexNum) == 0);
    assert!(offset_of!(PokedexListItem, bits_2) == 2);
    assert!(size_of::<SearchOptionText>() == 8);
    assert!(offset_of!(SearchOptionText, description) == 0);
    assert!(offset_of!(SearchOptionText, title) == 4);
    assert!(size_of::<SearchMenuItem>() == 12);
    assert!(offset_of!(SearchMenuItem, description) == 0);
    assert!(offset_of!(SearchMenuItem, titleBgX) == 4);
    assert!(offset_of!(SearchMenuItem, titleBgY) == 5);
    assert!(offset_of!(SearchMenuItem, titleBgWidth) == 6);
    assert!(offset_of!(SearchMenuItem, selectionBgX) == 7);
    assert!(offset_of!(SearchMenuItem, selectionBgY) == 8);
    assert!(offset_of!(SearchMenuItem, selectionBgWidth) == 9);
    assert!(size_of::<SearchMenuTopBarItem>() == 8);
    assert!(offset_of!(SearchMenuTopBarItem, description) == 0);
    assert!(offset_of!(SearchMenuTopBarItem, highlightX) == 4);
    assert!(offset_of!(SearchMenuTopBarItem, highlightY) == 5);
    assert!(offset_of!(SearchMenuTopBarItem, highlightWidth) == 6);
    assert!(size_of::<SearchOption>() == 8);
    assert!(offset_of!(SearchOption, texts) == 0);
    assert!(offset_of!(SearchOption, taskDataCursorPos) == 4);
    assert!(offset_of!(SearchOption, taskDataScrollOffset) == 5);
    assert!(offset_of!(SearchOption, numOptions) == 6);
};

const AREA_SCREEN: u8 = 0;
const CANCEL_SCREEN: u8 = 3;
const CRY_SCREEN: u8 = 1;
const FOOTPRINT_COLOR_IDX: u8 = 2;
const LIST_SCROLL_STEP: u16 = 16;
const MAX_MONS_ON_SCREEN: u16 = 4;
const MAX_SEARCH_PARAM_CURSOR_POS: u16 = 5;
const MAX_SEARCH_PARAM_ON_SCREEN: u16 = 6;
const MON_PAGE_X: i16 = 48;
const MON_PAGE_Y: i16 = 56;
const NUM_FOOTPRINT_TILES: i32 = 4;
const ORDER_ALPHABETICAL: u16 = 1;
const ORDER_HEAVIEST: u16 = 2;
const ORDER_LIGHTEST: u16 = 3;
const ORDER_NUMERICAL: u16 = 0;
const ORDER_SMALLEST: u16 = 5;
const ORDER_TALLEST: u16 = 4;
const PAGE_AREA: u8 = 5;
const PAGE_CRY: u8 = 6;
const PAGE_INFO: u8 = 1;
const PAGE_MAIN: u8 = 0;
const PAGE_SEARCH: u8 = 2;
const PAGE_SEARCH_RESULTS: u8 = 3;
const PAGE_SIZE: u8 = 7;
const POKEBALL_ROTATION_BOTTOM: u8 = 48;
const POKEBALL_ROTATION_TOP: u8 = 64;
const SCREEN_COUNT: u8 = 4;
const SEARCH_COLOR: u8 = 1;
const SEARCH_COUNT: i32 = 7;
const SEARCH_MODE: u8 = 5;
const SEARCH_NAME: u8 = 0;
const SEARCH_OK: i16 = 6;
const SEARCH_ORDER: u8 = 4;
const SEARCH_TOPBAR_CANCEL: u8 = 2;
const SEARCH_TOPBAR_COUNT: i32 = 3;
const SEARCH_TOPBAR_SEARCH: u8 = 0;
const SEARCH_TOPBAR_SHIFT: u8 = 1;
const SEARCH_TYPE_LEFT: u8 = 2;
const SEARCH_TYPE_RIGHT: u8 = 3;
const SIZE_SCREEN: u8 = 2;
const WIN_CRY_WAVE: u8 = 2;
const WIN_FOOTPRINT: u8 = 1;
const WIN_INFO: u8 = 0;
const WIN_VU_METER: u8 = 3;

static gMonFootprintTable: Table<CArray<*mut u8, 413>> =
    Table((&raw const crate::data::pokedex::gMonFootprintTable).cast());
static gPokedexEntries: Table<CArray<PokedexEntry, 387>> =
    Table((&raw const crate::data::pokedex::gPokedexEntries).cast());
static gPokedexOrder_Alphabetical: Table<CArray<u16, 411>> =
    Table((&raw const crate::data::pokedex::gPokedexOrder_Alphabetical).cast());
static gPokedexOrder_Height: Table<CArray<u16, 386>> =
    Table((&raw const crate::data::pokedex::gPokedexOrder_Height).cast());
static gPokedexOrder_Weight: Table<CArray<u16, 386>> =
    Table((&raw const crate::data::pokedex::gPokedexOrder_Weight).cast());
static sCaughtBall_Gfx: Table<CArray<u8, 64>> =
    Table((&raw const crate::data::pokedex::sCaughtBall_Gfx).cast());
static sDexListStartMenuCursorSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokedex::sDexListStartMenuCursorSpriteTemplate).cast());
static sDexModeOptions: Table<CArray<SearchOptionText, 3>> =
    Table((&raw const crate::data::pokedex::sDexModeOptions).cast());
static sDexOrderOptions: Table<CArray<SearchOptionText, 7>> =
    Table((&raw const crate::data::pokedex::sDexOrderOptions).cast());
static sDexSearchColorOptions: Table<CArray<SearchOptionText, 12>> =
    Table((&raw const crate::data::pokedex::sDexSearchColorOptions).cast());
static sDexSearchNameOptions: Table<CArray<SearchOptionText, 11>> =
    Table((&raw const crate::data::pokedex::sDexSearchNameOptions).cast());
static sDexSearchTypeIds: Table<CArray<u8, 18>> =
    Table((&raw const crate::data::pokedex::sDexSearchTypeIds).cast());
static sDexSearchTypeOptions: Table<CArray<SearchOptionText, 19>> =
    Table((&raw const crate::data::pokedex::sDexSearchTypeOptions).cast());
static sExpandedPlaceholder_PokedexDescription: Table<CArray<u8, 1>> =
    Table((&raw const crate::data::pokedex::sExpandedPlaceholder_PokedexDescription).cast());
static sHoennDexSeenOwnNumberSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokedex::sHoennDexSeenOwnNumberSpriteTemplate).cast());
static sHoennNationalTextSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokedex::sHoennNationalTextSpriteTemplate).cast());
static sInfoScreen_BgTemplate: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::pokedex::sInfoScreen_BgTemplate).cast());
static sInfoScreen_WindowTemplates: Table<CArray<WindowTemplate, 5>> =
    Table((&raw const crate::data::pokedex::sInfoScreen_WindowTemplates).cast());
static sInterfaceSpritePalette: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::pokedex::sInterfaceSpritePalette).cast());
static sInterfaceSpriteSheet: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::pokedex::sInterfaceSpriteSheet).cast());
static sInterfaceTextSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokedex::sInterfaceTextSpriteTemplate).cast());
static sLetterSearchRanges: Table<CArray<CArray<u8, 4>, 10>> =
    Table((&raw const crate::data::pokedex::sLetterSearchRanges).cast());
static sNationalDexSeenOwnNumberSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokedex::sNationalDexSeenOwnNumberSpriteTemplate).cast());
static sNewEntryInfoScreen_BgTemplate: Table<CArray<BgTemplate, 2>> =
    Table((&raw const crate::data::pokedex::sNewEntryInfoScreen_BgTemplate).cast());
static sNewEntryInfoScreen_WindowTemplates: Table<CArray<WindowTemplate, 3>> =
    Table((&raw const crate::data::pokedex::sNewEntryInfoScreen_WindowTemplates).cast());
static sOrderOptions: Table<CArray<u8, 6>> =
    Table((&raw const crate::data::pokedex::sOrderOptions).cast());
static sPokedexModes: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::pokedex::sPokedexModes).cast());
static sPokedex_BgTemplate: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::pokedex::sPokedex_BgTemplate).cast());
static sPokemonList_WindowTemplate: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::pokedex::sPokemonList_WindowTemplate).cast());
static sRotatingPokeBallSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokedex::sRotatingPokeBallSpriteTemplate).cast());
static sScrollArrowSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokedex::sScrollArrowSpriteTemplate).cast());
static sScrollBarSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokedex::sScrollBarSpriteTemplate).cast());
static sScrollMonIncrements: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::pokedex::sScrollMonIncrements).cast());
static sScrollTimers: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::pokedex::sScrollTimers).cast());
static sSearchMenuItems: Table<CArray<SearchMenuItem, 7>> =
    Table((&raw const crate::data::pokedex::sSearchMenuItems).cast());
static sSearchMenuTopBarItems: Table<CArray<SearchMenuTopBarItem, 3>> =
    Table((&raw const crate::data::pokedex::sSearchMenuTopBarItems).cast());
static sSearchMenu_BgTemplate: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::pokedex::sSearchMenu_BgTemplate).cast());
static sSearchMenu_WindowTemplate: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::pokedex::sSearchMenu_WindowTemplate).cast());
static sSearchMovementMap_SearchHoennDex: Table<CArray<CArray<u8, 4>, 7>> =
    Table((&raw const crate::data::pokedex::sSearchMovementMap_SearchHoennDex).cast());
static sSearchMovementMap_SearchNatDex: Table<CArray<CArray<u8, 4>, 7>> =
    Table((&raw const crate::data::pokedex::sSearchMovementMap_SearchNatDex).cast());
static sSearchMovementMap_ShiftHoennDex: Table<CArray<CArray<u8, 4>, 7>> =
    Table((&raw const crate::data::pokedex::sSearchMovementMap_ShiftHoennDex).cast());
static sSearchMovementMap_ShiftNatDex: Table<CArray<CArray<u8, 4>, 7>> =
    Table((&raw const crate::data::pokedex::sSearchMovementMap_ShiftNatDex).cast());
static sSearchOptions: Table<CArray<SearchOption, 6>> =
    Table((&raw const crate::data::pokedex::sSearchOptions).cast());
static sSeenOwnTextSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokedex::sSeenOwnTextSpriteTemplate).cast());
static sSizeScreenSilhouette_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokedex::sSizeScreenSilhouette_Pal).cast());
static sText_No000: Table<CArray<u8, 6>> =
    Table((&raw const crate::data::pokedex::sText_No000).cast());
static sText_TenDashes: Table<CArray<u8, 11>> =
    Table((&raw const crate::data::pokedex::sText_TenDashes).cast());
static sText_TenDashes2: Table<CArray<u8, 11>> =
    Table((&raw const crate::data::pokedex::sText_TenDashes2).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokedexView: *mut PokedexView = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLastSelectedPokemon: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokeBallRotation: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokedexListItem: *mut PokedexListItem = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gUnusedPokedexU8: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gPokedexVBlankCB: Option<unsafe extern "C" fn()> = None;

unsafe extern "C" {
    static mut gDexCryScreenState: u8;
    static mut gMPlayInfo_BGM: MusicPlayerInfo;
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static gPokedexBgHoenn_Pal: CArray<u16, 0>;
    static gPokedexBgNational_Pal: CArray<u16, 0>;
    static gPokedexCryScreen_Tilemap: CArray<u32, 0>;
    static gPokedexInfoScreen_Tilemap: CArray<u32, 0>;
    static gPokedexListUnderlay_Tilemap: CArray<u32, 0>;
    static gPokedexList_Tilemap: CArray<u32, 0>;
    static gPokedexMenu_Gfx: CArray<u32, 0>;
    static gPokedexScreenSelectBarMain_Tilemap: CArray<u32, 0>;
    static gPokedexScreenSelectBarSubmenu_Tilemap: CArray<u32, 0>;
    static gPokedexSearchMenuHoenn_Tilemap: CArray<u32, 0>;
    static gPokedexSearchMenuNational_Tilemap: CArray<u32, 0>;
    static gPokedexSearchMenu_Gfx: CArray<u32, 0>;
    static gPokedexSearchMenu_Pal: CArray<u16, 0>;
    static gPokedexSearchResults_Pal: CArray<u16, 0>;
    static gPokedexSizeScreen_Tilemap: CArray<u32, 0>;
    static gPokedexStartMenuMain_Tilemap: CArray<u32, 0>;
    static gPokedexStartMenuSearchResults_Tilemap: CArray<u32, 0>;
    static mut gReservedSpritePaletteCount: u8;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static gSineTable: CArray<i16, 0>;
    static gSpeciesInfo: CArray<SpeciesInfo, 0>;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static gText_5MarksPokemon: CArray<u8, 0>;
    static gText_CryOf: CArray<u8, 0>;
    static gText_HTHeight: CArray<u8, 0>;
    static gText_NoMatchingPkmnWereFound: CArray<u8, 0>;
    static gText_NumberClear01: CArray<u8, 0>;
    static gText_PokedexRegistration: CArray<u8, 0>;
    static gText_SearchCompleted: CArray<u8, 0>;
    static gText_SearchingPleaseWait: CArray<u8, 0>;
    static gText_SelectorArrow: CArray<u8, 0>;
    static gText_SizeComparedTo: CArray<u8, 0>;
    static gText_UnkHeight: CArray<u8, 0>;
    static gText_UnkWeight: CArray<u8, 0>;
    static gText_WTWeight: CArray<u8, 0>;
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
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlitBitmapToWindow(a0: u8, a1: *mut u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn BuildOamBuffer();
    fn CB2_ReturnToFieldWithOpenMenu();
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyMonCategoryText(a0: i32, a1: *mut u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyToWindowPixelBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateMonPicSprite_HandleDeoxys(
        a0: u16,
        a1: u32,
        a2: u32,
        a3: u8,
        a4: i16,
        a5: i16,
        a6: u8,
        a7: u16,
    ) -> u16;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTrainerPicSprite(a0: u16, a1: u8, a2: i16, a3: i16, a4: u8, a5: u16) -> u16;
    fn CryScreenPlayButton(a0: u16);
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut c_void, a2: u32, a3: u16, a4: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DisableNationalPokedex();
    fn EnableInterrupts(a0: u16);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeAndDestroyMonPicSprite(a0: u16) -> u16;
    fn FreeAndDestroyTrainerPicSprite(a0: u16) -> u16;
    fn FreeCryScreen();
    fn GetBgTilemapBuffer(a0: u8) -> *mut c_void;
    fn GetMonSpritePalFromSpeciesAndPersonality(a0: u16, a1: u32, a2: u32) -> *mut u32;
    fn GetOverworldTextboxPalettePtr() -> *mut u16;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn HideBg(a0: u8);
    fn HoennToNationalOrder(a0: u16) -> u16;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn IsCryPlaying() -> u8;
    fn IsCryPlayingOrClearCrySongs() -> u8;
    fn IsNationalPokedexEnabled() -> u32;
    fn IsSEPlaying() -> u8;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadCryMeter(a0: *mut CryScreenWindow, a1: u8) -> u8;
    fn LoadCryWaveformWindow(a0: *mut CryScreenWindow, a1: u8) -> u8;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut SpritePalette);
    fn NationalPokedexNumToSpecies(a0: u16) -> u16;
    fn NationalToHoennOrder(a0: u16) -> u16;
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlayCry_NormalNoDucking(a0: u16, a1: i8, a2: i8, a3: u8);
    fn PlaySE(a0: u16);
    fn PlayerGenderToFrontTrainerPicId(a0: u8) -> u16;
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ResetAllPicSprites() -> u16;
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn ScanlineEffect_Stop();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetOamMatrix(a0: u8, a1: u16, a2: u16, a3: u16, a4: u16);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn ShowPokedexAreaScreen(a0: u16, a1: *mut u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StopCryAndClearCrySongs();
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringLength(a0: *mut u8) -> u16;
    fn TransferPlttBuffer();
    fn UpdateCryWaveformWindow(a0: u8);
    fn UpdatePaletteFade() -> u8;
    fn m4aMPlayContinue(a0: *mut MusicPlayerInfo);
    fn m4aMPlayStop(a0: *mut MusicPlayerInfo);
    fn m4aMPlayVolumeControl(a0: *mut MusicPlayerInfo, a1: u16, a2: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetPokedex() {
    let mut i: u16 = 0;
    sLastSelectedPokemon = 0;
    sPokeBallRotation = POKEBALL_ROTATION_TOP;
    gUnusedPokedexU8 = 0;
    (*gSaveBlock2Ptr).pokedex.mode = DEX_MODE_HOENN as u8;
    (*gSaveBlock2Ptr).pokedex.order = 0;
    (*gSaveBlock2Ptr).pokedex.nationalMagic = 0;
    (*gSaveBlock2Ptr).pokedex.unknown2 = 0;
    (*gSaveBlock2Ptr).pokedex.unownPersonality = 0;
    (*gSaveBlock2Ptr).pokedex.spindaPersonality = 0;
    (*gSaveBlock2Ptr).pokedex.unknown3 = 0;
    DisableNationalPokedex();
    i = 0;
    while (i as i32) < 51 + (if 4 != 0 { 1 } else { 0 }) {
        (*gSaveBlock2Ptr).pokedex.owned[i] = 0;
        (*gSaveBlock2Ptr).pokedex.seen[i] = 0;
        (*gSaveBlock1Ptr).seen1[i] = 0;
        (*gSaveBlock1Ptr).seen2[i] = 0;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetPokedexScrollPositions() {
    sLastSelectedPokemon = 0;
    sPokeBallRotation = POKEBALL_ROTATION_TOP;
}
pub(crate) unsafe extern "C" fn VBlankCB_Pokedex() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn ResetPokedexView(pokedexView: *mut PokedexView) {
    let mut i: u16 = 0;
    i = 0;
    while i < NATIONAL_DEX_DEOXYS {
        (*pokedexView).pokedexList[i].dexNum = 0xFFFF;
        (*pokedexView).pokedexList[i].set_seen(FALSE as u16);
        (*pokedexView).pokedexList[i].set_owned(FALSE as u16);
        i += 1;
    }
    (*pokedexView).pokedexList[386].dexNum = 0;
    (*pokedexView).pokedexList[386].set_seen(FALSE as u16);
    (*pokedexView).pokedexList[386].set_owned(FALSE as u16);
    (*pokedexView).pokemonListCount = 0;
    (*pokedexView).selectedPokemon = 0;
    (*pokedexView).selectedPokemonBackup = 0;
    (*pokedexView).dexMode = DEX_MODE_HOENN;
    (*pokedexView).dexModeBackup = DEX_MODE_HOENN;
    (*pokedexView).dexOrder = ORDER_NUMERICAL;
    (*pokedexView).dexOrderBackup = ORDER_NUMERICAL;
    (*pokedexView).seenCount = 0;
    (*pokedexView).ownCount = 0;
    i = 0;
    while i < MAX_MONS_ON_SCREEN {
        (*pokedexView).monSpriteIds[i] = 0xFFFF;
        i += 1;
    }
    (*pokedexView).pokeBallRotationStep = 0;
    (*pokedexView).pokeBallRotationBackup = 0;
    (*pokedexView).pokeBallRotation = 0;
    (*pokedexView).initialVOffset = 0;
    (*pokedexView).scrollTimer = 0;
    (*pokedexView).scrollDirection = 0;
    (*pokedexView).listVOffset = 0;
    (*pokedexView).listMovingVOffset = 0;
    (*pokedexView).scrollMonIncrement = 0;
    (*pokedexView).maxScrollTimer = 0;
    (*pokedexView).scrollSpeed = 0;
    i = 0;
    while i < 4 {
        (*pokedexView).unkArr1[i] = 0;
        i += 1;
    }
    (*pokedexView).currentPage = PAGE_MAIN;
    (*pokedexView).currentPageBackup = PAGE_MAIN;
    (*pokedexView).set_isSearchResults(FALSE);
    (*pokedexView).selectedScreen = AREA_SCREEN;
    (*pokedexView).screenSwitchState = 0;
    (*pokedexView).menuIsOpen = 0;
    (*pokedexView).menuCursorPos = 0;
    (*pokedexView).menuY = 0;
    i = 0;
    while i < 8 {
        (*pokedexView).unkArr2[i] = 0;
        i += 1;
    }
    i = 0;
    while i < 8 {
        (*pokedexView).unkArr3[i] = 0;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_OpenPokedex() {
    match gMain.state {
        1 => {
            ScanlineEffect_Stop();
            ResetTasks();
            ResetSpriteData();
            ResetPaletteFade();
            FreeAllSpritePalettes();
            gReservedSpritePaletteCount = 8;
            ResetAllPicSprites();
            gMain.state += 1;
        }
        2 => {
            sPokedexView = AllocZeroed(1636) as *mut PokedexView;
            ResetPokedexView(sPokedexView);
            CreateTask(Some(Task_OpenPokedexMainPage), 0);
            (*sPokedexView).dexMode = (*gSaveBlock2Ptr).pokedex.mode as u16;
            if IsNationalPokedexEnabled() == 0 {
                (*sPokedexView).dexMode = DEX_MODE_HOENN;
            }
            (*sPokedexView).dexOrder = (*gSaveBlock2Ptr).pokedex.order as u16;
            (*sPokedexView).selectedPokemon = sLastSelectedPokemon;
            (*sPokedexView).pokeBallRotation = sPokeBallRotation;
            (*sPokedexView).selectedScreen = AREA_SCREEN;
            if IsNationalPokedexEnabled() == 0 {
                (*sPokedexView).seenCount = GetHoennPokedexCount(FLAG_GET_SEEN);
                (*sPokedexView).ownCount = GetHoennPokedexCount(FLAG_GET_CAUGHT);
            } else {
                (*sPokedexView).seenCount = GetNationalPokedexCount(FLAG_GET_SEEN);
                (*sPokedexView).ownCount = GetNationalPokedexCount(FLAG_GET_CAUGHT);
            }
            (*sPokedexView).initialVOffset = 8;
            gMain.state += 1;
        }
        3 => {
            EnableInterrupts(1);
            SetVBlankCallback(Some(VBlankCB_Pokedex));
            SetMainCallback2(Some(CB2_Pokedex));
            CreatePokedexList(
                (*sPokedexView).dexMode as u8,
                (*sPokedexView).dexOrder as u8,
            );
            m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 0x80);
        }
        _ => {
            SetVBlankCallback(None);
            ResetOtherVideoRegisters(0);
            {
                let mut _dest: *mut c_void = VRAM as usize as *mut c_void;
                let mut _size: u32 = VRAM_SIZE;
                loop {
                    {
                        {
                            let mut tmp: u16 = 0;
                            volatile_write(&raw mut tmp, 0);
                            {
                                {
                                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x81000800);
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                    }
                    _dest = (_dest as *mut u8).at(4096) as *mut c_void;
                    _size -= 0x1000;
                    if _size <= 0x1000 {
                        {
                            {
                                let mut tmp: u16 = 0;
                                volatile_write(&raw mut tmp, 0);
                                {
                                    {
                                        let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                        volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                        volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                        volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
                                        let _ = (dmaRegs.at(2)).read_volatile();
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
            }
            {
                {
                    let mut _dest: *mut u32 = OAM as i32 as usize as *mut u32;
                    let mut _size: u32 = OAM_SIZE;
                    {
                        {
                            let mut tmp: u32 = 0;
                            volatile_write(&raw mut tmp, 0);
                            {
                                {
                                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x85000000 | _size / 4);
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                    }
                }
            }
            {
                {
                    let mut _dest: *mut u16 = PLTT as i32 as usize as *mut u16;
                    let mut _size: u32 = PLTT_SIZE;
                    {
                        {
                            let mut tmp: u16 = 0;
                            volatile_write(&raw mut tmp, 0);
                            {
                                {
                                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                    }
                }
            }
            gMain.state = 1;
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_Pokedex() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_OpenPokedexMainPage(taskId: u8) {
    (*sPokedexView).set_isSearchResults(FALSE);
    if LoadPokedexListPage(PAGE_MAIN) != 0 {
        gTasks[taskId].func = Some(Task_HandlePokedexInput);
    }
}
pub(crate) unsafe extern "C" fn Task_HandlePokedexInput(taskId: u8) {
    SetGpuReg(REG_OFFSET_BG0VOFS, (*sPokedexView).menuY as u16);
    if (*sPokedexView).menuY != 0 {
        (*sPokedexView).menuY -= 8;
    } else {
        if gMain.newKeys as i32 & A_BUTTON != 0
            && (*sPokedexView).pokedexList[(*sPokedexView).selectedPokemon].seen() != 0
        {
            UpdateSelectedMonSpriteId();
            BeginNormalPaletteFade(
                !(shl_i32(
                    1,
                    gSprites[(*sPokedexView).selectedMonSpriteId]
                        .oam
                        .paletteNum() as u32
                        + 16,
                ) as u32),
                0,
                0,
                0x10,
                0,
            );
            gSprites[(*sPokedexView).selectedMonSpriteId].callback =
                Some(SpriteCB_MoveMonForInfoScreen);
            gTasks[taskId].func = Some(Task_OpenInfoScreenAfterMonMovement);
            PlaySE(SE_PIN);
            FreeWindowAndBgBuffers();
        } else if gMain.newKeys as i32 & START_BUTTON != 0 {
            (*sPokedexView).menuY = 0;
            (*sPokedexView).menuIsOpen = TRUE;
            (*sPokedexView).menuCursorPos = 0;
            gTasks[taskId].func = Some(Task_HandlePokedexStartMenuInput);
            PlaySE(SE_SELECT);
        } else if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
            PlaySE(SE_SELECT);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
            gTasks[taskId].data[0] = LoadSearchMenu() as i16;
            (*sPokedexView).screenSwitchState = 0;
            (*sPokedexView).pokeBallRotationBackup = (*sPokedexView).pokeBallRotation as u16;
            (*sPokedexView).selectedPokemonBackup = (*sPokedexView).selectedPokemon;
            (*sPokedexView).dexModeBackup = (*sPokedexView).dexMode;
            (*sPokedexView).dexOrderBackup = (*sPokedexView).dexOrder;
            gTasks[taskId].func = Some(Task_WaitForExitSearch);
            PlaySE(SE_PC_LOGIN);
            FreeWindowAndBgBuffers();
        } else if gMain.newKeys as i32 & B_BUTTON != 0 {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
            gTasks[taskId].func = Some(Task_ClosePokedex);
            PlaySE(SE_PC_OFF);
        } else {
            (*sPokedexView).selectedPokemon =
                TryDoPokedexScroll((*sPokedexView).selectedPokemon, 0xE);
            if (*sPokedexView).scrollTimer != 0 {
                gTasks[taskId].func = Some(Task_WaitForScroll);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForScroll(taskId: u8) {
    if UpdateDexListScroll(
        (*sPokedexView).scrollDirection,
        (*sPokedexView).scrollMonIncrement as u8,
        (*sPokedexView).maxScrollTimer as u8,
    ) != 0
    {
        gTasks[taskId].func = Some(Task_HandlePokedexInput);
    }
}
pub(crate) unsafe extern "C" fn Task_HandlePokedexStartMenuInput(taskId: u8) {
    SetGpuReg(REG_OFFSET_BG0VOFS, (*sPokedexView).menuY as u16);
    if (*sPokedexView).menuY != 80 {
        (*sPokedexView).menuY += 8;
    } else {
        if gMain.newKeys as i32 & A_BUTTON != 0 {
            match (*sPokedexView).menuCursorPos {
                1 => {
                    (*sPokedexView).selectedPokemon = 0;
                    (*sPokedexView).pokeBallRotation = POKEBALL_ROTATION_TOP;
                    ClearMonSprites();
                    CreateMonSpritesAtPos((*sPokedexView).selectedPokemon, 0xE);
                    gMain.newKeys |= START_BUTTON as u16;
                }
                2 => {
                    (*sPokedexView).selectedPokemon = (*sPokedexView).pokemonListCount - 1;
                    (*sPokedexView).pokeBallRotation =
                        (*sPokedexView).pokemonListCount as u8 * 16 + POKEBALL_ROTATION_BOTTOM;
                    ClearMonSprites();
                    CreateMonSpritesAtPos((*sPokedexView).selectedPokemon, 0xE);
                    gMain.newKeys |= START_BUTTON as u16;
                }
                3 => {
                    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
                    gTasks[taskId].func = Some(Task_ClosePokedex);
                    PlaySE(SE_PC_OFF);
                }
                _ => {
                    gMain.newKeys |= START_BUTTON as u16;
                }
            }
        }
        if gMain.newKeys as i32 & 10 != 0 {
            (*sPokedexView).menuIsOpen = FALSE;
            gTasks[taskId].func = Some(Task_HandlePokedexInput);
            PlaySE(SE_SELECT);
        } else if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0
            && (*sPokedexView).menuCursorPos != 0
        {
            (*sPokedexView).menuCursorPos -= 1;
            PlaySE(SE_SELECT);
        } else if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0
            && (*sPokedexView).menuCursorPos < 3
        {
            (*sPokedexView).menuCursorPos += 1;
            PlaySE(SE_SELECT);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_OpenInfoScreenAfterMonMovement(taskId: u8) {
    if gSprites[(*sPokedexView).selectedMonSpriteId].x == MON_PAGE_X
        && gSprites[(*sPokedexView).selectedMonSpriteId].y == MON_PAGE_Y
    {
        (*sPokedexView).currentPageBackup = (*sPokedexView).currentPage;
        gTasks[taskId].data[0] = LoadInfoScreen(
            &raw mut (*sPokedexView).pokedexList[(*sPokedexView).selectedPokemon],
            (*sPokedexView).selectedMonSpriteId as u8,
        ) as i16;
        gTasks[taskId].func = Some(Task_WaitForExitInfoScreen);
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForExitInfoScreen(taskId: u8) {
    if gTasks[gTasks[taskId].data[0]].isActive != 0 {
        if (*sPokedexView).currentPage == PAGE_INFO
            && IsInfoScreenScrolling(gTasks[taskId].data[0] as u8) == 0
            && TryDoInfoScreenScroll() != 0
        {
            StartInfoScreenScroll(
                &raw mut (*sPokedexView).pokedexList[(*sPokedexView).selectedPokemon],
                gTasks[taskId].data[0] as u8,
            );
        }
    } else {
        sLastSelectedPokemon = (*sPokedexView).selectedPokemon;
        sPokeBallRotation = (*sPokedexView).pokeBallRotation;
        gTasks[taskId].func = Some(Task_OpenPokedexMainPage);
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForExitSearch(taskId: u8) {
    if gTasks[gTasks[taskId].data[0]].isActive == 0 {
        ClearMonSprites();
        if (*sPokedexView).screenSwitchState != 0 {
            (*sPokedexView).selectedPokemon = 0;
            (*sPokedexView).pokeBallRotation = POKEBALL_ROTATION_TOP;
            gTasks[taskId].func = Some(Task_OpenSearchResults);
        } else {
            (*sPokedexView).pokeBallRotation = (*sPokedexView).pokeBallRotationBackup as u8;
            (*sPokedexView).selectedPokemon = (*sPokedexView).selectedPokemonBackup;
            (*sPokedexView).dexMode = (*sPokedexView).dexModeBackup;
            if IsNationalPokedexEnabled() == 0 {
                (*sPokedexView).dexMode = DEX_MODE_HOENN;
            }
            (*sPokedexView).dexOrder = (*sPokedexView).dexOrderBackup;
            gTasks[taskId].func = Some(Task_OpenPokedexMainPage);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ClosePokedex(taskId: u8) {
    if gPaletteFade.active() == 0 {
        (*gSaveBlock2Ptr).pokedex.mode = (*sPokedexView).dexMode as u8;
        if IsNationalPokedexEnabled() == 0 {
            (*gSaveBlock2Ptr).pokedex.mode = DEX_MODE_HOENN as u8;
        }
        (*gSaveBlock2Ptr).pokedex.order = (*sPokedexView).dexOrder as u8;
        ClearMonSprites();
        FreeWindowAndBgBuffers();
        DestroyTask(taskId);
        SetMainCallback2(Some(CB2_ReturnToFieldWithOpenMenu));
        m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 0x100);
        Free(sPokedexView as *mut c_void);
    }
}
pub(crate) unsafe extern "C" fn Task_OpenSearchResults(taskId: u8) {
    (*sPokedexView).set_isSearchResults(TRUE);
    if LoadPokedexListPage(PAGE_SEARCH_RESULTS) != 0 {
        gTasks[taskId].func = Some(Task_HandleSearchResultsInput);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSearchResultsInput(taskId: u8) {
    SetGpuReg(REG_OFFSET_BG0VOFS, (*sPokedexView).menuY as u16);
    if (*sPokedexView).menuY != 0 {
        (*sPokedexView).menuY -= 8;
    } else {
        if gMain.newKeys as i32 & A_BUTTON != 0
            && (*sPokedexView).pokedexList[(*sPokedexView).selectedPokemon].seen() != 0
        {
            let mut a: u32 = 0;
            UpdateSelectedMonSpriteId();
            a = shl_i32(
                1,
                gSprites[(*sPokedexView).selectedMonSpriteId]
                    .oam
                    .paletteNum() as u32
                    + 16,
            ) as u32;
            gSprites[(*sPokedexView).selectedMonSpriteId].callback =
                Some(SpriteCB_MoveMonForInfoScreen);
            BeginNormalPaletteFade(!a, 0, 0, 0x10, 0);
            gTasks[taskId].func = Some(Task_OpenSearchResultsInfoScreenAfterMonMovement);
            PlaySE(SE_PIN);
            FreeWindowAndBgBuffers();
        } else if gMain.newKeys as i32 & START_BUTTON != 0 {
            (*sPokedexView).menuY = 0;
            (*sPokedexView).menuIsOpen = TRUE;
            (*sPokedexView).menuCursorPos = 0;
            gTasks[taskId].func = Some(Task_HandleSearchResultsStartMenuInput);
            PlaySE(SE_SELECT);
        } else if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
            gTasks[taskId].data[0] = LoadSearchMenu() as i16;
            (*sPokedexView).screenSwitchState = 0;
            gTasks[taskId].func = Some(Task_WaitForExitSearch);
            PlaySE(SE_PC_LOGIN);
            FreeWindowAndBgBuffers();
        } else if gMain.newKeys as i32 & B_BUTTON != 0 {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
            gTasks[taskId].func = Some(Task_ReturnToPokedexFromSearchResults);
            PlaySE(SE_PC_OFF);
        } else {
            (*sPokedexView).selectedPokemon =
                TryDoPokedexScroll((*sPokedexView).selectedPokemon, 0xE);
            if (*sPokedexView).scrollTimer != 0 {
                gTasks[taskId].func = Some(Task_WaitForSearchResultsScroll);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForSearchResultsScroll(taskId: u8) {
    if UpdateDexListScroll(
        (*sPokedexView).scrollDirection,
        (*sPokedexView).scrollMonIncrement as u8,
        (*sPokedexView).maxScrollTimer as u8,
    ) != 0
    {
        gTasks[taskId].func = Some(Task_HandleSearchResultsInput);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSearchResultsStartMenuInput(taskId: u8) {
    SetGpuReg(REG_OFFSET_BG0VOFS, (*sPokedexView).menuY as u16);
    if (*sPokedexView).menuY != 96 {
        (*sPokedexView).menuY += 8;
    } else {
        if gMain.newKeys as i32 & A_BUTTON != 0 {
            match (*sPokedexView).menuCursorPos {
                1 => {
                    (*sPokedexView).selectedPokemon = 0;
                    (*sPokedexView).pokeBallRotation = POKEBALL_ROTATION_TOP;
                    ClearMonSprites();
                    CreateMonSpritesAtPos((*sPokedexView).selectedPokemon, 0xE);
                    gMain.newKeys |= START_BUTTON as u16;
                }
                2 => {
                    (*sPokedexView).selectedPokemon = (*sPokedexView).pokemonListCount - 1;
                    (*sPokedexView).pokeBallRotation =
                        (*sPokedexView).pokemonListCount as u8 * 16 + POKEBALL_ROTATION_BOTTOM;
                    ClearMonSprites();
                    CreateMonSpritesAtPos((*sPokedexView).selectedPokemon, 0xE);
                    gMain.newKeys |= START_BUTTON as u16;
                }
                3 => {
                    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
                    gTasks[taskId].func = Some(Task_ReturnToPokedexFromSearchResults);
                    PlaySE(SE_TRUCK_DOOR);
                }
                4 => {
                    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
                    gTasks[taskId].func = Some(Task_ClosePokedexFromSearchResultsStartMenu);
                    PlaySE(SE_PC_OFF);
                }
                _ => {
                    gMain.newKeys |= START_BUTTON as u16;
                }
            }
        }
        if gMain.newKeys as i32 & 10 != 0 {
            (*sPokedexView).menuIsOpen = FALSE;
            gTasks[taskId].func = Some(Task_HandleSearchResultsInput);
            PlaySE(SE_SELECT);
        } else if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0
            && (*sPokedexView).menuCursorPos != 0
        {
            (*sPokedexView).menuCursorPos -= 1;
            PlaySE(SE_SELECT);
        } else if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0
            && (*sPokedexView).menuCursorPos < 4
        {
            (*sPokedexView).menuCursorPos += 1;
            PlaySE(SE_SELECT);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_OpenSearchResultsInfoScreenAfterMonMovement(taskId: u8) {
    if gSprites[(*sPokedexView).selectedMonSpriteId].x == MON_PAGE_X
        && gSprites[(*sPokedexView).selectedMonSpriteId].y == MON_PAGE_Y
    {
        (*sPokedexView).currentPageBackup = (*sPokedexView).currentPage;
        gTasks[taskId].data[0] = LoadInfoScreen(
            &raw mut (*sPokedexView).pokedexList[(*sPokedexView).selectedPokemon],
            (*sPokedexView).selectedMonSpriteId as u8,
        ) as i16;
        (*sPokedexView).selectedMonSpriteId = 65535;
        gTasks[taskId].func = Some(Task_WaitForExitSearchResultsInfoScreen);
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForExitSearchResultsInfoScreen(taskId: u8) {
    if gTasks[gTasks[taskId].data[0]].isActive != 0 {
        if (*sPokedexView).currentPage == PAGE_INFO
            && IsInfoScreenScrolling(gTasks[taskId].data[0] as u8) == 0
            && TryDoInfoScreenScroll() != 0
        {
            StartInfoScreenScroll(
                &raw mut (*sPokedexView).pokedexList[(*sPokedexView).selectedPokemon],
                gTasks[taskId].data[0] as u8,
            );
        }
    } else {
        gTasks[taskId].func = Some(Task_OpenSearchResults);
    }
}
pub(crate) unsafe extern "C" fn Task_ReturnToPokedexFromSearchResults(taskId: u8) {
    if gPaletteFade.active() == 0 {
        (*sPokedexView).pokeBallRotation = (*sPokedexView).pokeBallRotationBackup as u8;
        (*sPokedexView).selectedPokemon = (*sPokedexView).selectedPokemonBackup;
        (*sPokedexView).dexMode = (*sPokedexView).dexModeBackup;
        if IsNationalPokedexEnabled() == 0 {
            (*sPokedexView).dexMode = DEX_MODE_HOENN;
        }
        (*sPokedexView).dexOrder = (*sPokedexView).dexOrderBackup;
        gTasks[taskId].func = Some(Task_OpenPokedexMainPage);
        ClearMonSprites();
        FreeWindowAndBgBuffers();
    }
}
pub(crate) unsafe extern "C" fn Task_ClosePokedexFromSearchResultsStartMenu(taskId: u8) {
    if gPaletteFade.active() == 0 {
        (*sPokedexView).pokeBallRotation = (*sPokedexView).pokeBallRotationBackup as u8;
        (*sPokedexView).selectedPokemon = (*sPokedexView).selectedPokemonBackup;
        (*sPokedexView).dexMode = (*sPokedexView).dexModeBackup;
        if IsNationalPokedexEnabled() == 0 {
            (*sPokedexView).dexMode = DEX_MODE_HOENN;
        }
        (*sPokedexView).dexOrder = (*sPokedexView).dexOrderBackup;
        gTasks[taskId].func = Some(Task_ClosePokedex);
    }
}
pub(crate) unsafe extern "C" fn LoadPokedexListPage(page: u8) -> u8 {
    match gMain.state {
        1 => {
            ResetSpriteData();
            FreeAllSpritePalettes();
            gReservedSpritePaletteCount = 8;
            LoadCompressedSpriteSheet((&raw const sInterfaceSpriteSheet[0]).cast_mut());
            LoadSpritePalettes(sInterfaceSpritePalette.as_ptr().cast_mut());
            CreateInterfaceSprites(page);
            gMain.state += 1;
        }
        2 => {
            gMain.state += 1;
        }
        3 => {
            if page == PAGE_MAIN {
                CreatePokedexList(
                    (*sPokedexView).dexMode as u8,
                    (*sPokedexView).dexOrder as u8,
                );
            }
            CreateMonSpritesAtPos((*sPokedexView).selectedPokemon, 0xE);
            (*sPokedexView).menuIsOpen = FALSE;
            (*sPokedexView).menuY = 0;
            CopyBgTilemapBufferToVram(0);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
            CopyBgTilemapBufferToVram(3);
            gMain.state += 1;
        }
        4 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
            SetVBlankCallback(Some(VBlankCB_Pokedex));
            gMain.state += 1;
        }
        5 => {
            SetGpuReg(REG_OFFSET_WININ, 16191);
            SetGpuReg(REG_OFFSET_WINOUT, 7487);
            SetGpuReg(REG_OFFSET_WIN0H, 0);
            SetGpuReg(REG_OFFSET_WIN0V, 0);
            SetGpuReg(REG_OFFSET_WIN1H, 0);
            SetGpuReg(REG_OFFSET_WIN1V, 0);
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            SetGpuReg(0x0, 36928);
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            ShowBg(3);
            gMain.state += 1;
        }
        6 => {
            if gPaletteFade.active() == 0 {
                gMain.state = 0;
                return TRUE;
            }
        }
        _ => {
            if gPaletteFade.active() != 0 {
                return 0;
            }
            SetVBlankCallback(None);
            (*sPokedexView).currentPage = page;
            ResetOtherVideoRegisters(0);
            SetGpuReg(REG_OFFSET_BG2VOFS, (*sPokedexView).initialVOffset as u16);
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sPokedex_BgTemplate.as_ptr().cast_mut(), 4);
            SetBgTilemapBuffer(3, AllocZeroed(BG_SCREEN_SIZE));
            SetBgTilemapBuffer(2, AllocZeroed(BG_SCREEN_SIZE));
            SetBgTilemapBuffer(1, AllocZeroed(BG_SCREEN_SIZE));
            SetBgTilemapBuffer(0, AllocZeroed(BG_SCREEN_SIZE));
            DecompressAndLoadBgGfxUsingHeap(
                3,
                gPokedexMenu_Gfx.as_ptr().cast_mut() as *mut c_void,
                0x2000,
                0,
                0,
            );
            CopyToBgTilemapBuffer(
                1,
                gPokedexList_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyToBgTilemapBuffer(
                3,
                gPokedexListUnderlay_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            if page == PAGE_MAIN {
                CopyToBgTilemapBuffer(
                    0,
                    gPokedexStartMenuMain_Tilemap.as_ptr().cast_mut() as *mut c_void,
                    0,
                    0x280,
                );
            } else {
                CopyToBgTilemapBuffer(
                    0,
                    gPokedexStartMenuSearchResults_Tilemap.as_ptr().cast_mut() as *mut c_void,
                    0,
                    0x280,
                );
            }
            ResetPaletteFade();
            if page == PAGE_MAIN {
                (*sPokedexView).set_isSearchResults(FALSE);
            } else {
                (*sPokedexView).set_isSearchResults(TRUE);
            }
            LoadPokedexBgPalette((*sPokedexView).isSearchResults());
            InitWindows(sPokemonList_WindowTemplate.as_ptr().cast_mut());
            DeactivateAllTextPrinters();
            PutWindowTilemap(0);
            CopyWindowToVram(0, COPYWIN_FULL);
            gMain.state = 1;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn LoadPokedexBgPalette(isSearchResults: u8) {
    if isSearchResults == TRUE {
        LoadPalette(
            gPokedexSearchResults_Pal.as_ptr().cast_mut().at(1) as *mut c_void,
            1,
            190,
        );
    } else if IsNationalPokedexEnabled() == 0 {
        LoadPalette(
            gPokedexBgHoenn_Pal.as_ptr().cast_mut().at(1) as *mut c_void,
            1,
            190,
        );
    } else {
        LoadPalette(
            gPokedexBgNational_Pal.as_ptr().cast_mut().at(1) as *mut c_void,
            1,
            190,
        );
    }
    LoadPalette(GetOverworldTextboxPalettePtr() as *mut c_void, 240, 32);
}
pub(crate) unsafe extern "C" fn FreeWindowAndBgBuffers() {
    let mut tilemapBuffer: *mut c_void = null_mut();
    FreeAllWindowBuffers();
    tilemapBuffer = GetBgTilemapBuffer(0);
    if !tilemapBuffer.is_null() {
        Free(tilemapBuffer);
    }
    tilemapBuffer = GetBgTilemapBuffer(1);
    if !tilemapBuffer.is_null() {
        Free(tilemapBuffer);
    }
    tilemapBuffer = GetBgTilemapBuffer(2);
    if !tilemapBuffer.is_null() {
        Free(tilemapBuffer);
    }
    tilemapBuffer = GetBgTilemapBuffer(3);
    if !tilemapBuffer.is_null() {
        Free(tilemapBuffer);
    }
}
pub(crate) unsafe extern "C" fn CreatePokedexList(dexMode: u8, order: u8) {
    let mut vars: CArray<u16, 3> = zeroed();
    let mut i: i16 = 0;
    (*sPokedexView).pokemonListCount = 0;
    match dexMode {
        1 => {
            if IsNationalPokedexEnabled() != 0 {
                vars[0] = NATIONAL_DEX_DEOXYS;
                vars[1] = FALSE as u16;
            } else {
                vars[0] = HOENN_DEX_DEOXYS;
                vars[1] = TRUE as u16;
            }
        }
        _ => {
            vars[0] = HOENN_DEX_DEOXYS;
            vars[1] = TRUE as u16;
        }
    }
    match order {
        0 => {
            if vars[1] != 0 {
                i = 0;
                while (i as i32) < vars[0] as i32 {
                    vars[2] = HoennToNationalOrder(i as u16 + 1);
                    (*sPokedexView).pokedexList[i].dexNum = vars[2];
                    (*sPokedexView).pokedexList[i]
                        .set_seen(GetSetPokedexFlag(vars[2], FLAG_GET_SEEN) as u16);
                    (*sPokedexView).pokedexList[i]
                        .set_owned(GetSetPokedexFlag(vars[2], FLAG_GET_CAUGHT) as u16);
                    if (*sPokedexView).pokedexList[i].seen() != 0 {
                        (*sPokedexView).pokemonListCount = i as u16 + 1;
                    }
                    i += 1;
                }
            } else {
                let mut r5: i16 = 0;
                let mut r10: i16 = 0;
                i = 0;
                r5 = 0;
                r10 = 0;
                while (i as i32) < vars[0] as i32 {
                    vars[2] = i as u16 + 1;
                    if GetSetPokedexFlag(vars[2], FLAG_GET_SEEN) != 0 {
                        r10 = 1;
                    }
                    if r10 != 0 {
                        (*sPokedexView).pokedexList[r5].dexNum = vars[2];
                        (*sPokedexView).pokedexList[r5]
                            .set_seen(GetSetPokedexFlag(vars[2], FLAG_GET_SEEN) as u16);
                        (*sPokedexView).pokedexList[r5]
                            .set_owned(GetSetPokedexFlag(vars[2], FLAG_GET_CAUGHT) as u16);
                        if (*sPokedexView).pokedexList[r5].seen() != 0 {
                            (*sPokedexView).pokemonListCount = r5 as u16 + 1;
                        }
                        r5 += 1;
                    }
                    i += 1;
                }
            }
        }
        1 => {
            i = 0;
            while i < 411 {
                vars[2] = gPokedexOrder_Alphabetical[i];
                if NationalToHoennOrder(vars[2]) <= vars[0]
                    && GetSetPokedexFlag(vars[2], FLAG_GET_SEEN) != 0
                {
                    (*sPokedexView).pokedexList[(*sPokedexView).pokemonListCount].dexNum = vars[2];
                    (*sPokedexView).pokedexList[(*sPokedexView).pokemonListCount]
                        .set_seen(TRUE as u16);
                    (*sPokedexView).pokedexList[(*sPokedexView).pokemonListCount]
                        .set_owned(GetSetPokedexFlag(vars[2], FLAG_GET_CAUGHT) as u16);
                    (*sPokedexView).pokemonListCount += 1;
                }
                i += 1;
            }
        }
        2 => {
            i = 385;
            while i >= 0 {
                vars[2] = gPokedexOrder_Weight[i];
                if NationalToHoennOrder(vars[2]) <= vars[0]
                    && GetSetPokedexFlag(vars[2], FLAG_GET_CAUGHT) != 0
                {
                    (*sPokedexView).pokedexList[(*sPokedexView).pokemonListCount].dexNum = vars[2];
                    (*sPokedexView).pokedexList[(*sPokedexView).pokemonListCount]
                        .set_seen(TRUE as u16);
                    (*sPokedexView).pokedexList[(*sPokedexView).pokemonListCount]
                        .set_owned(TRUE as u16);
                    (*sPokedexView).pokemonListCount += 1;
                }
                i -= 1;
            }
        }
        3 => {
            i = 0;
            while i < NATIONAL_DEX_DEOXYS as i16 {
                vars[2] = gPokedexOrder_Weight[i];
                if NationalToHoennOrder(vars[2]) <= vars[0]
                    && GetSetPokedexFlag(vars[2], FLAG_GET_CAUGHT) != 0
                {
                    (*sPokedexView).pokedexList[(*sPokedexView).pokemonListCount].dexNum = vars[2];
                    (*sPokedexView).pokedexList[(*sPokedexView).pokemonListCount]
                        .set_seen(TRUE as u16);
                    (*sPokedexView).pokedexList[(*sPokedexView).pokemonListCount]
                        .set_owned(TRUE as u16);
                    (*sPokedexView).pokemonListCount += 1;
                }
                i += 1;
            }
        }
        4 => {
            i = 385;
            while i >= 0 {
                vars[2] = gPokedexOrder_Height[i];
                if NationalToHoennOrder(vars[2]) <= vars[0]
                    && GetSetPokedexFlag(vars[2], FLAG_GET_CAUGHT) != 0
                {
                    (*sPokedexView).pokedexList[(*sPokedexView).pokemonListCount].dexNum = vars[2];
                    (*sPokedexView).pokedexList[(*sPokedexView).pokemonListCount]
                        .set_seen(TRUE as u16);
                    (*sPokedexView).pokedexList[(*sPokedexView).pokemonListCount]
                        .set_owned(TRUE as u16);
                    (*sPokedexView).pokemonListCount += 1;
                }
                i -= 1;
            }
        }
        5 => {
            i = 0;
            while i < NATIONAL_DEX_DEOXYS as i16 {
                vars[2] = gPokedexOrder_Height[i];
                if NationalToHoennOrder(vars[2]) <= vars[0]
                    && GetSetPokedexFlag(vars[2], FLAG_GET_CAUGHT) != 0
                {
                    (*sPokedexView).pokedexList[(*sPokedexView).pokemonListCount].dexNum = vars[2];
                    (*sPokedexView).pokedexList[(*sPokedexView).pokemonListCount]
                        .set_seen(TRUE as u16);
                    (*sPokedexView).pokedexList[(*sPokedexView).pokemonListCount]
                        .set_owned(TRUE as u16);
                    (*sPokedexView).pokemonListCount += 1;
                }
                i += 1;
            }
        }
        _ => {}
    }
    i = (*sPokedexView).pokemonListCount as i16;
    while i < NATIONAL_DEX_DEOXYS as i16 {
        (*sPokedexView).pokedexList[i].dexNum = 0xFFFF;
        (*sPokedexView).pokedexList[i].set_seen(FALSE as u16);
        (*sPokedexView).pokedexList[i].set_owned(FALSE as u16);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn PrintMonDexNumAndName(
    windowId: u8,
    fontId: u8,
    str: *mut u8,
    left: u8,
    top: u8,
) {
    let mut color: CArray<u8, 3> = zeroed();
    color[0] = 0x0;
    color[1] = TEXT_DYNAMIC_COLOR_6;
    color[2] = TEXT_COLOR_LIGHT_GRAY;
    AddTextPrinterParameterized4(
        windowId,
        fontId,
        left * 8,
        top * 8 + 1,
        0,
        0,
        color.as_mut_ptr(),
        TEXT_SKIP_DRAW as i8,
        str,
    );
}
pub(crate) unsafe extern "C" fn CreateMonListEntry(position: u8, b: u16, ignored: u16) {
    let mut entryNum: i16 = 0;
    let mut i: u16 = 0;
    let mut vOffset: u16 = 0;
    match position {
        1 => {
            entryNum = b as i16 - 5;
            if entryNum < 0
                || entryNum >= NATIONAL_DEX_DEOXYS as i16
                || (*sPokedexView).pokedexList[entryNum].dexNum == 0xFFFF
            {
                ClearMonListEntry(17, (*sPokedexView).listVOffset as u8 * 2, ignored);
            } else {
                ClearMonListEntry(17, (*sPokedexView).listVOffset as u8 * 2, ignored);
                if (*sPokedexView).pokedexList[entryNum].seen() != 0 {
                    CreateMonDexNum(
                        entryNum as u16,
                        18,
                        (*sPokedexView).listVOffset as u8 * 2,
                        ignored,
                    );
                    CreateCaughtBall(
                        (*sPokedexView).pokedexList[entryNum].owned(),
                        0x11,
                        (*sPokedexView).listVOffset as u8 * 2,
                        ignored,
                    );
                    CreateMonName(
                        (*sPokedexView).pokedexList[entryNum].dexNum,
                        0x16,
                        (*sPokedexView).listVOffset as u8 * 2,
                    );
                } else {
                    CreateMonDexNum(
                        entryNum as u16,
                        18,
                        (*sPokedexView).listVOffset as u8 * 2,
                        ignored,
                    );
                    CreateCaughtBall(
                        FALSE as u16,
                        17,
                        (*sPokedexView).listVOffset as u8 * 2,
                        ignored,
                    );
                    CreateMonName(0, 0x16, (*sPokedexView).listVOffset as u8 * 2);
                }
            }
        }
        2 => {
            entryNum = b as i16 + 5;
            vOffset = (*sPokedexView).listVOffset as u16 + 10;
            if vOffset >= LIST_SCROLL_STEP {
                vOffset -= LIST_SCROLL_STEP;
            }
            if entryNum < 0
                || entryNum >= NATIONAL_DEX_DEOXYS as i16
                || (*sPokedexView).pokedexList[entryNum].dexNum == 0xFFFF
            {
                ClearMonListEntry(17, vOffset as u8 * 2, ignored);
            } else {
                ClearMonListEntry(17, vOffset as u8 * 2, ignored);
                if (*sPokedexView).pokedexList[entryNum].seen() != 0 {
                    CreateMonDexNum(entryNum as u16, 18, vOffset as u8 * 2, ignored);
                    CreateCaughtBall(
                        (*sPokedexView).pokedexList[entryNum].owned(),
                        0x11,
                        vOffset as u8 * 2,
                        ignored,
                    );
                    CreateMonName(
                        (*sPokedexView).pokedexList[entryNum].dexNum,
                        0x16,
                        vOffset as u8 * 2,
                    );
                } else {
                    CreateMonDexNum(entryNum as u16, 18, vOffset as u8 * 2, ignored);
                    CreateCaughtBall(FALSE as u16, 0x11, vOffset as u8 * 2, ignored);
                    CreateMonName(0, 0x16, vOffset as u8 * 2);
                }
            }
        }
        _ => {
            entryNum = b as i16 - 5;
            i = 0;
            while i <= 10 {
                if entryNum < 0
                    || entryNum >= NATIONAL_DEX_DEOXYS as i16
                    || (*sPokedexView).pokedexList[entryNum].dexNum == 0xFFFF
                {
                    ClearMonListEntry(17, i as u8 * 2, ignored);
                } else {
                    ClearMonListEntry(17, i as u8 * 2, ignored);
                    if (*sPokedexView).pokedexList[entryNum].seen() != 0 {
                        CreateMonDexNum(entryNum as u16, 0x12, i as u8 * 2, ignored);
                        CreateCaughtBall(
                            (*sPokedexView).pokedexList[entryNum].owned(),
                            0x11,
                            i as u8 * 2,
                            ignored,
                        );
                        CreateMonName(
                            (*sPokedexView).pokedexList[entryNum].dexNum,
                            0x16,
                            i as u8 * 2,
                        );
                    } else {
                        CreateMonDexNum(entryNum as u16, 0x12, i as u8 * 2, ignored);
                        CreateCaughtBall(FALSE as u16, 0x11, i as u8 * 2, ignored);
                        CreateMonName(0, 0x16, i as u8 * 2);
                    }
                }
                entryNum += 1;
                i += 1;
            }
        }
    }
    CopyWindowToVram(0, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn CreateMonDexNum(entryNum: u16, left: u8, top: u8, unused: u16) {
    let mut text: CArray<u8, 6> = zeroed();
    let mut dexNum: u16 = 0;
    memcpy(text.as_mut_ptr(), sText_No000.as_ptr().cast_mut(), 6);
    dexNum = (*sPokedexView).pokedexList[entryNum].dexNum;
    if (*sPokedexView).dexMode == DEX_MODE_HOENN {
        dexNum = NationalToHoennOrder(dexNum);
    }
    text[2] = CHAR_0 + (dexNum as i32 / 100) as u8;
    text[3] = CHAR_0 + (dexNum as i32 % 100 / 10) as u8;
    text[4] = CHAR_0 + (dexNum as i32 % 100 % 10) as u8;
    PrintMonDexNumAndName(0, FONT_NARROW, text.as_mut_ptr(), left, top);
}
pub(crate) unsafe extern "C" fn CreateCaughtBall(owned: u16, x: u8, y: u8, unused: u16) {
    if owned != 0 {
        BlitBitmapToWindow(
            0,
            sCaughtBall_Gfx.as_ptr().cast_mut(),
            x as u16 * 8,
            y as u16 * 8,
            8,
            16,
        );
    } else {
        FillWindowPixelRect(0, 0, x as u16 * 8, y as u16 * 8, 8, 16);
    }
}
pub(crate) unsafe extern "C" fn CreateMonName(mut num: u16, left: u8, top: u8) -> u8 {
    let mut str: *mut u8 = null_mut();
    num = NationalPokedexNumToSpecies(num);
    if num != 0 {
        str = gSpeciesNames[num].as_ptr().cast_mut();
    } else {
        str = sText_TenDashes.as_ptr().cast_mut();
    }
    PrintMonDexNumAndName(0, FONT_NARROW, str, left, top);
    return StringLength(str) as u8;
}
pub(crate) unsafe extern "C" fn ClearMonListEntry(x: u8, y: u8, unused: u16) {
    FillWindowPixelRect(0, 0, x as u16 * 8, y as u16 * 8, 0x60, 16);
}
pub(crate) unsafe extern "C" fn CreateMonSpritesAtPos(selectedMon: u16, ignored: u16) {
    let mut i: u8 = 0;
    let mut dexNum: u16 = 0;
    let mut spriteId: u8 = 0;
    gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
    i = 0;
    while i < MAX_MONS_ON_SCREEN as u8 {
        (*sPokedexView).monSpriteIds[i] = 0xFFFF;
        i += 1;
    }
    (*sPokedexView).selectedMonSpriteId = 0xFFFF;
    dexNum = GetPokemonSpriteToDisplay(selectedMon - 1);
    if dexNum != 0xFFFF {
        spriteId = CreatePokedexMonSprite(dexNum, 0x60, 0x50) as u8;
        gSprites[spriteId].callback = Some(SpriteCB_PokedexListMonSprite);
        gSprites[spriteId].data[5] = -32;
    }
    dexNum = GetPokemonSpriteToDisplay(selectedMon);
    if dexNum != 0xFFFF {
        spriteId = CreatePokedexMonSprite(dexNum, 0x60, 0x50) as u8;
        gSprites[spriteId].callback = Some(SpriteCB_PokedexListMonSprite);
        gSprites[spriteId].data[5] = 0;
    }
    dexNum = GetPokemonSpriteToDisplay(selectedMon + 1);
    if dexNum != 0xFFFF {
        spriteId = CreatePokedexMonSprite(dexNum, 0x60, 0x50) as u8;
        gSprites[spriteId].callback = Some(SpriteCB_PokedexListMonSprite);
        gSprites[spriteId].data[5] = 32;
    }
    CreateMonListEntry(0, selectedMon, ignored);
    SetGpuReg(REG_OFFSET_BG2VOFS, (*sPokedexView).initialVOffset as u16);
    (*sPokedexView).listVOffset = 0;
    (*sPokedexView).listMovingVOffset = 0;
    gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
}
pub(crate) unsafe extern "C" fn UpdateDexListScroll(
    direction: u8,
    monMoveIncrement: u8,
    scrollTimerMax: u8,
) -> u8 {
    let mut i: u16 = 0;
    let mut step: u8 = 0;
    if (*sPokedexView).scrollTimer != 0 {
        (*sPokedexView).scrollTimer -= 1;
        match direction {
            1 => {
                i = 0;
                while i < MAX_MONS_ON_SCREEN {
                    if (*sPokedexView).monSpriteIds[i] != 0xFFFF {
                        gSprites[(*sPokedexView).monSpriteIds[i]].data[5] +=
                            monMoveIncrement as i16;
                    }
                    i += 1;
                }
                step = div_i32(
                    LIST_SCROLL_STEP as i32
                        * (scrollTimerMax as i32 - (*sPokedexView).scrollTimer as i32),
                    scrollTimerMax as i32,
                ) as u8;
                SetGpuReg(
                    REG_OFFSET_BG2VOFS,
                    (*sPokedexView).initialVOffset as u16
                        + (*sPokedexView).listMovingVOffset as u16 * LIST_SCROLL_STEP
                        - step as u16,
                );
                (*sPokedexView).pokeBallRotation -= (*sPokedexView).pokeBallRotationStep as u8;
            }
            2 => {
                i = 0;
                while i < MAX_MONS_ON_SCREEN {
                    if (*sPokedexView).monSpriteIds[i] != 0xFFFF {
                        gSprites[(*sPokedexView).monSpriteIds[i]].data[5] -=
                            monMoveIncrement as i16;
                    }
                    i += 1;
                }
                step = div_i32(
                    LIST_SCROLL_STEP as i32
                        * (scrollTimerMax as i32 - (*sPokedexView).scrollTimer as i32),
                    scrollTimerMax as i32,
                ) as u8;
                SetGpuReg(
                    REG_OFFSET_BG2VOFS,
                    (*sPokedexView).initialVOffset as u16
                        + (*sPokedexView).listMovingVOffset as u16 * LIST_SCROLL_STEP
                        + step as u16,
                );
                (*sPokedexView).pokeBallRotation += (*sPokedexView).pokeBallRotationStep as u8;
            }
            _ => {}
        }
        return FALSE;
    } else {
        SetGpuReg(
            REG_OFFSET_BG2VOFS,
            (*sPokedexView).initialVOffset as u16
                + (*sPokedexView).listVOffset as u16 * LIST_SCROLL_STEP,
        );
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CreateScrollingPokemonSprite(direction: u8, selectedMon: u16) {
    let mut dexNum: u16 = 0;
    let mut spriteId: u8 = 0;
    (*sPokedexView).listMovingVOffset = (*sPokedexView).listVOffset;
    match direction {
        1 => {
            dexNum = GetPokemonSpriteToDisplay(selectedMon - 1);
            if dexNum != 0xFFFF {
                spriteId = CreatePokedexMonSprite(dexNum, 0x60, 0x50) as u8;
                gSprites[spriteId].callback = Some(SpriteCB_PokedexListMonSprite);
                gSprites[spriteId].data[5] = -64;
            }
            if (*sPokedexView).listVOffset > 0 {
                (*sPokedexView).listVOffset -= 1;
            } else {
                (*sPokedexView).listVOffset = 15;
            }
        }
        2 => {
            dexNum = GetPokemonSpriteToDisplay(selectedMon + 1);
            if dexNum != 0xFFFF {
                spriteId = CreatePokedexMonSprite(dexNum, 0x60, 0x50) as u8;
                gSprites[spriteId].callback = Some(SpriteCB_PokedexListMonSprite);
                gSprites[spriteId].data[5] = 64;
            }
            if (*sPokedexView).listVOffset < 15 {
                (*sPokedexView).listVOffset += 1;
            } else {
                (*sPokedexView).listVOffset = 0;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn TryDoPokedexScroll(mut selectedMon: u16, ignored: u16) -> u16 {
    let mut scrollTimer: u8 = 0;
    let mut scrollMonIncrement: u8 = 0;
    let mut i: u8 = 0;
    let mut startingPos: u16 = 0;
    let mut scrollDir: u8 = 0;
    if gMain.heldKeys as i32 & DPAD_UP != 0 && selectedMon > 0 {
        scrollDir = 1;
        selectedMon = GetNextPosition(1, selectedMon, 0, (*sPokedexView).pokemonListCount - 1);
        CreateScrollingPokemonSprite(1, selectedMon);
        CreateMonListEntry(1, selectedMon, ignored);
        PlaySE(SE_DEX_SCROLL);
    } else if gMain.heldKeys as i32 & DPAD_DOWN != 0
        && (selectedMon as i32) < (*sPokedexView).pokemonListCount as i32 - 1
    {
        scrollDir = 2;
        selectedMon = GetNextPosition(0, selectedMon, 0, (*sPokedexView).pokemonListCount - 1);
        CreateScrollingPokemonSprite(2, selectedMon);
        CreateMonListEntry(2, selectedMon, ignored);
        PlaySE(SE_DEX_SCROLL);
    } else if gMain.newKeys as i32 & DPAD_LEFT != 0 && selectedMon > 0 {
        startingPos = selectedMon;
        i = 0;
        while i < 7 {
            selectedMon = GetNextPosition(1, selectedMon, 0, (*sPokedexView).pokemonListCount - 1);
            i += 1;
        }
        (*sPokedexView).pokeBallRotation += 16 * (selectedMon as u8 - startingPos as u8);
        ClearMonSprites();
        CreateMonSpritesAtPos(selectedMon, 0xE);
        PlaySE(SE_DEX_PAGE);
    } else if gMain.newKeys as i32 & DPAD_RIGHT != 0
        && (selectedMon as i32) < (*sPokedexView).pokemonListCount as i32 - 1
    {
        startingPos = selectedMon;
        i = 0;
        while i < 7 {
            selectedMon = GetNextPosition(0, selectedMon, 0, (*sPokedexView).pokemonListCount - 1);
            i += 1;
        }
        (*sPokedexView).pokeBallRotation += 16 * (selectedMon as u8 - startingPos as u8);
        ClearMonSprites();
        CreateMonSpritesAtPos(selectedMon, 0xE);
        PlaySE(SE_DEX_PAGE);
    }
    if scrollDir == 0 {
        (*sPokedexView).scrollSpeed = 0;
        return selectedMon;
    }
    scrollMonIncrement = sScrollMonIncrements[(*sPokedexView).scrollSpeed as i32 / 4];
    scrollTimer = sScrollTimers[(*sPokedexView).scrollSpeed as i32 / 4];
    (*sPokedexView).scrollTimer = scrollTimer;
    (*sPokedexView).maxScrollTimer = scrollTimer as u16;
    (*sPokedexView).scrollMonIncrement = scrollMonIncrement as u16;
    (*sPokedexView).scrollDirection = scrollDir;
    (*sPokedexView).pokeBallRotationStep = (scrollMonIncrement as i32 / 2) as u16;
    UpdateDexListScroll(
        (*sPokedexView).scrollDirection,
        (*sPokedexView).scrollMonIncrement as u8,
        (*sPokedexView).maxScrollTimer as u8,
    );
    if (*sPokedexView).scrollSpeed < 12 {
        (*sPokedexView).scrollSpeed += 1;
    }
    return selectedMon;
}
pub(crate) unsafe extern "C" fn UpdateSelectedMonSpriteId() {
    let mut i: u16 = 0;
    i = 0;
    while i < MAX_MONS_ON_SCREEN {
        let mut spriteId: u16 = (*sPokedexView).monSpriteIds[i];
        if gSprites[spriteId].x2 == 0 && gSprites[spriteId].y2 == 0 && spriteId != 0xFFFF {
            (*sPokedexView).selectedMonSpriteId = spriteId;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn TryDoInfoScreenScroll() -> u8 {
    let mut nextPokemon: u16 = 0;
    let mut selectedPokemon: u16 = (*sPokedexView).selectedPokemon;
    if gMain.newKeys as i32 & DPAD_UP != 0 && selectedPokemon != 0 {
        nextPokemon = selectedPokemon;
        while nextPokemon != 0 {
            nextPokemon = GetNextPosition(1, nextPokemon, 0, (*sPokedexView).pokemonListCount - 1);
            if (*sPokedexView).pokedexList[nextPokemon].seen() != 0 {
                selectedPokemon = nextPokemon;
                break;
            }
        }
        if (*sPokedexView).selectedPokemon == selectedPokemon {
            return FALSE;
        } else {
            (*sPokedexView).selectedPokemon = selectedPokemon;
            (*sPokedexView).pokeBallRotation -= 16;
            return TRUE;
        }
    } else if gMain.newKeys as i32 & DPAD_DOWN != 0
        && (selectedPokemon as i32) < (*sPokedexView).pokemonListCount as i32 - 1
    {
        nextPokemon = selectedPokemon;
        while (nextPokemon as i32) < (*sPokedexView).pokemonListCount as i32 - 1 {
            nextPokemon = GetNextPosition(0, nextPokemon, 0, (*sPokedexView).pokemonListCount - 1);
            if (*sPokedexView).pokedexList[nextPokemon].seen() != 0 {
                selectedPokemon = nextPokemon;
                break;
            }
        }
        if (*sPokedexView).selectedPokemon == selectedPokemon {
            return FALSE;
        } else {
            (*sPokedexView).selectedPokemon = selectedPokemon;
            (*sPokedexView).pokeBallRotation += 16;
            return TRUE;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ClearMonSprites() -> u8 {
    let mut i: u16 = 0;
    i = 0;
    while i < MAX_MONS_ON_SCREEN {
        if (*sPokedexView).monSpriteIds[i] != 0xFFFF {
            FreeAndDestroyMonPicSprite((*sPokedexView).monSpriteIds[i]);
            (*sPokedexView).monSpriteIds[i] = 0xFFFF;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn GetPokemonSpriteToDisplay(species: u16) -> u16 {
    if species >= NATIONAL_DEX_DEOXYS || (*sPokedexView).pokedexList[species].dexNum == 0xFFFF {
        return 0xFFFF;
    } else if (*sPokedexView).pokedexList[species].seen() != 0 {
        return (*sPokedexView).pokedexList[species].dexNum;
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CreatePokedexMonSprite(num: u16, x: i16, y: i16) -> u32 {
    let mut i: u8 = 0;
    i = 0;
    while i < MAX_MONS_ON_SCREEN as u8 {
        if (*sPokedexView).monSpriteIds[i] == 0xFFFF {
            let mut spriteId: u8 = CreateMonSpriteFromNationalDexNumber(num, x, y, i as u16) as u8;
            gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
            gSprites[spriteId].oam.set_priority(3);
            gSprites[spriteId].data[0] = 0;
            gSprites[spriteId].data[1] = i as i16;
            gSprites[spriteId].data[2] = NationalPokedexNumToSpecies(num) as i16;
            (*sPokedexView).monSpriteIds[i] = spriteId as u16;
            return spriteId as u32;
        }
        i += 1;
    }
    return 0xFFFF;
}
pub(crate) unsafe extern "C" fn CreateInterfaceSprites(page: u8) {
    let mut spriteId: u8 = 0;
    let mut digitNum: u16 = 0;
    spriteId = CreateSprite(
        (&raw const *sScrollArrowSpriteTemplate).cast_mut(),
        184,
        4,
        0,
    );
    gSprites[spriteId].data[1] = FALSE as i16;
    spriteId = CreateSprite(
        (&raw const *sScrollArrowSpriteTemplate).cast_mut(),
        184,
        156,
        0,
    );
    gSprites[spriteId].data[1] = TRUE as i16;
    gSprites[spriteId].set_vFlip(TRUE as u16);
    CreateSprite(
        (&raw const *sScrollBarSpriteTemplate).cast_mut(),
        230,
        20,
        0,
    );
    CreateSprite(
        (&raw const *sInterfaceTextSpriteTemplate).cast_mut(),
        16,
        120,
        0,
    );
    spriteId = CreateSprite(
        (&raw const *sInterfaceTextSpriteTemplate).cast_mut(),
        48,
        120,
        0,
    );
    StartSpriteAnim(&raw mut gSprites[spriteId], 3);
    spriteId = CreateSprite(
        (&raw const *sInterfaceTextSpriteTemplate).cast_mut(),
        16,
        144,
        0,
    );
    StartSpriteAnim(&raw mut gSprites[spriteId], 2);
    gSprites[spriteId].data[2] = 0x80;
    spriteId = CreateSprite(
        (&raw const *sInterfaceTextSpriteTemplate).cast_mut(),
        48,
        144,
        0,
    );
    StartSpriteAnim(&raw mut gSprites[spriteId], 1);
    spriteId = CreateSprite(
        (&raw const *sRotatingPokeBallSpriteTemplate).cast_mut(),
        0,
        80,
        2,
    );
    gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
    gSprites[spriteId].oam.set_matrixNum(30);
    gSprites[spriteId].data[0] = 30;
    gSprites[spriteId].data[1] = 0;
    spriteId = CreateSprite(
        (&raw const *sRotatingPokeBallSpriteTemplate).cast_mut(),
        0,
        80,
        2,
    );
    gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
    gSprites[spriteId].oam.set_matrixNum(31);
    gSprites[spriteId].data[0] = 31;
    gSprites[spriteId].data[1] = 128;
    if page == PAGE_MAIN {
        let mut drawNextDigit: u32 = 0;
        if IsNationalPokedexEnabled() == 0 {
            CreateSprite(
                (&raw const *sSeenOwnTextSpriteTemplate).cast_mut(),
                32,
                40,
                1,
            );
            spriteId = CreateSprite(
                (&raw const *sSeenOwnTextSpriteTemplate).cast_mut(),
                32,
                72,
                1,
            );
            StartSpriteAnim(&raw mut gSprites[spriteId], 1);
            drawNextDigit = FALSE as u32;
            spriteId = CreateSprite(
                (&raw const *sHoennDexSeenOwnNumberSpriteTemplate).cast_mut(),
                24,
                48,
                1,
            );
            digitNum = ((*sPokedexView).seenCount as i32 / 100) as u16;
            StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
            if digitNum != 0 {
                drawNextDigit = TRUE as u32;
            } else {
                gSprites[spriteId].set_invisible(TRUE as u16);
            }
            spriteId = CreateSprite(
                (&raw const *sHoennDexSeenOwnNumberSpriteTemplate).cast_mut(),
                32,
                48,
                1,
            );
            digitNum = ((*sPokedexView).seenCount as i32 % 100 / 10) as u16;
            if digitNum != 0 || drawNextDigit != 0 {
                StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
            } else {
                gSprites[spriteId].set_invisible(TRUE as u16);
            }
            spriteId = CreateSprite(
                (&raw const *sHoennDexSeenOwnNumberSpriteTemplate).cast_mut(),
                40,
                48,
                1,
            );
            digitNum = ((*sPokedexView).seenCount as i32 % 100 % 10) as u16;
            StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
            drawNextDigit = FALSE as u32;
            spriteId = CreateSprite(
                (&raw const *sHoennDexSeenOwnNumberSpriteTemplate).cast_mut(),
                24,
                80,
                1,
            );
            digitNum = ((*sPokedexView).ownCount as i32 / 100) as u16;
            StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
            if digitNum != 0 {
                drawNextDigit = TRUE as u32;
            } else {
                gSprites[spriteId].set_invisible(TRUE as u16);
            }
            spriteId = CreateSprite(
                (&raw const *sHoennDexSeenOwnNumberSpriteTemplate).cast_mut(),
                32,
                80,
                1,
            );
            digitNum = ((*sPokedexView).ownCount as i32 % 100 / 10) as u16;
            if digitNum != 0 || drawNextDigit != 0 {
                StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
            } else {
                gSprites[spriteId].set_invisible(TRUE as u16);
            }
            spriteId = CreateSprite(
                (&raw const *sHoennDexSeenOwnNumberSpriteTemplate).cast_mut(),
                40,
                80,
                1,
            );
            digitNum = ((*sPokedexView).ownCount as i32 % 100 % 10) as u16;
            StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
        } else {
            let mut seenOwnedCount: u16 = 0;
            CreateSprite(
                (&raw const *sSeenOwnTextSpriteTemplate).cast_mut(),
                32,
                40,
                1,
            );
            spriteId = CreateSprite(
                (&raw const *sSeenOwnTextSpriteTemplate).cast_mut(),
                32,
                76,
                1,
            );
            StartSpriteAnim(&raw mut gSprites[spriteId], 1);
            CreateSprite(
                (&raw const *sHoennNationalTextSpriteTemplate).cast_mut(),
                17,
                45,
                1,
            );
            spriteId = CreateSprite(
                (&raw const *sHoennNationalTextSpriteTemplate).cast_mut(),
                17,
                55,
                1,
            );
            StartSpriteAnim(&raw mut gSprites[spriteId], 1);
            CreateSprite(
                (&raw const *sHoennNationalTextSpriteTemplate).cast_mut(),
                17,
                81,
                1,
            );
            spriteId = CreateSprite(
                (&raw const *sHoennNationalTextSpriteTemplate).cast_mut(),
                17,
                91,
                1,
            );
            StartSpriteAnim(&raw mut gSprites[spriteId], 1);
            seenOwnedCount = GetHoennPokedexCount(FLAG_GET_SEEN);
            drawNextDigit = FALSE as u32;
            spriteId = CreateSprite(
                (&raw const *sNationalDexSeenOwnNumberSpriteTemplate).cast_mut(),
                40,
                45,
                1,
            );
            digitNum = (seenOwnedCount as i32 / 100) as u16;
            StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
            if digitNum != 0 {
                drawNextDigit = TRUE as u32;
            } else {
                gSprites[spriteId].set_invisible(TRUE as u16);
            }
            spriteId = CreateSprite(
                (&raw const *sNationalDexSeenOwnNumberSpriteTemplate).cast_mut(),
                48,
                45,
                1,
            );
            digitNum = (seenOwnedCount as i32 % 100 / 10) as u16;
            if digitNum != 0 || drawNextDigit != 0 {
                StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
            } else {
                gSprites[spriteId].set_invisible(TRUE as u16);
            }
            spriteId = CreateSprite(
                (&raw const *sNationalDexSeenOwnNumberSpriteTemplate).cast_mut(),
                56,
                45,
                1,
            );
            digitNum = (seenOwnedCount as i32 % 100 % 10) as u16;
            StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
            drawNextDigit = FALSE as u32;
            spriteId = CreateSprite(
                (&raw const *sNationalDexSeenOwnNumberSpriteTemplate).cast_mut(),
                40,
                55,
                1,
            );
            digitNum = ((*sPokedexView).seenCount as i32 / 100) as u16;
            StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
            if digitNum != 0 {
                drawNextDigit = TRUE as u32;
            } else {
                gSprites[spriteId].set_invisible(TRUE as u16);
            }
            spriteId = CreateSprite(
                (&raw const *sNationalDexSeenOwnNumberSpriteTemplate).cast_mut(),
                48,
                55,
                1,
            );
            digitNum = ((*sPokedexView).seenCount as i32 % 100 / 10) as u16;
            if digitNum != 0 || drawNextDigit != 0 {
                StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
            } else {
                gSprites[spriteId].set_invisible(TRUE as u16);
            }
            spriteId = CreateSprite(
                (&raw const *sNationalDexSeenOwnNumberSpriteTemplate).cast_mut(),
                56,
                55,
                1,
            );
            digitNum = ((*sPokedexView).seenCount as i32 % 100 % 10) as u16;
            StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
            seenOwnedCount = GetHoennPokedexCount(FLAG_GET_CAUGHT);
            drawNextDigit = FALSE as u32;
            spriteId = CreateSprite(
                (&raw const *sNationalDexSeenOwnNumberSpriteTemplate).cast_mut(),
                40,
                81,
                1,
            );
            digitNum = (seenOwnedCount as i32 / 100) as u16;
            StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
            if digitNum != 0 {
                drawNextDigit = TRUE as u32;
            } else {
                gSprites[spriteId].set_invisible(TRUE as u16);
            }
            spriteId = CreateSprite(
                (&raw const *sNationalDexSeenOwnNumberSpriteTemplate).cast_mut(),
                48,
                81,
                1,
            );
            digitNum = (seenOwnedCount as i32 % 100 / 10) as u16;
            if digitNum != 0 || drawNextDigit != 0 {
                StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
            } else {
                gSprites[spriteId].set_invisible(TRUE as u16);
            }
            spriteId = CreateSprite(
                (&raw const *sNationalDexSeenOwnNumberSpriteTemplate).cast_mut(),
                56,
                81,
                1,
            );
            digitNum = (seenOwnedCount as i32 % 100 % 10) as u16;
            StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
            drawNextDigit = FALSE as u32;
            spriteId = CreateSprite(
                (&raw const *sNationalDexSeenOwnNumberSpriteTemplate).cast_mut(),
                40,
                91,
                1,
            );
            digitNum = ((*sPokedexView).ownCount as i32 / 100) as u16;
            StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
            if digitNum != 0 {
                drawNextDigit = TRUE as u32;
            } else {
                gSprites[spriteId].set_invisible(TRUE as u16);
            }
            spriteId = CreateSprite(
                (&raw const *sNationalDexSeenOwnNumberSpriteTemplate).cast_mut(),
                48,
                91,
                1,
            );
            digitNum = ((*sPokedexView).ownCount as i32 % 100 / 10) as u16;
            if digitNum != 0 || drawNextDigit != 0 {
                StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
            } else {
                gSprites[spriteId].set_invisible(TRUE as u16);
            }
            spriteId = CreateSprite(
                (&raw const *sNationalDexSeenOwnNumberSpriteTemplate).cast_mut(),
                56,
                91,
                1,
            );
            digitNum = ((*sPokedexView).ownCount as i32 % 100 % 10) as u16;
            StartSpriteAnim(&raw mut gSprites[spriteId], digitNum as u8);
        }
        spriteId = CreateSprite(
            (&raw const *sDexListStartMenuCursorSpriteTemplate).cast_mut(),
            136,
            96,
            1,
        );
        gSprites[spriteId].set_invisible(TRUE as u16);
    } else {
        spriteId = CreateSprite(
            (&raw const *sDexListStartMenuCursorSpriteTemplate).cast_mut(),
            136,
            80,
            1,
        );
        gSprites[spriteId].set_invisible(TRUE as u16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_EndMoveMonForInfoScreen(sprite: *mut Sprite) {}
pub(crate) unsafe extern "C" fn SpriteCB_SeenOwnInfo(sprite: *mut Sprite) {
    if (*sPokedexView).currentPage != PAGE_MAIN {
        DestroySprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_MoveMonForInfoScreen(sprite: *mut Sprite) {
    (*sprite).oam.set_priority(0);
    (*sprite).oam.set_affineMode(ST_OAM_AFFINE_OFF);
    (*sprite).x2 = 0;
    (*sprite).y2 = 0;
    if (*sprite).x != MON_PAGE_X || (*sprite).y != MON_PAGE_Y {
        if (*sprite).x > MON_PAGE_X {
            (*sprite).x -= 1;
        }
        if (*sprite).x < MON_PAGE_X {
            (*sprite).x += 1;
        }
        if (*sprite).y > MON_PAGE_Y {
            (*sprite).y -= 1;
        }
        if (*sprite).y < MON_PAGE_Y {
            (*sprite).y += 1;
        }
    } else {
        (*sprite).callback = Some(SpriteCB_EndMoveMonForInfoScreen);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PokedexListMonSprite(sprite: *mut Sprite) {
    let mut monId: u8 = (*sprite).data[1] as u8;
    if (*sPokedexView).currentPage != PAGE_MAIN
        && (*sPokedexView).currentPage != PAGE_SEARCH_RESULTS
    {
        FreeAndDestroyMonPicSprite((*sPokedexView).monSpriteIds[monId]);
        (*sPokedexView).monSpriteIds[monId] = 0xFFFF;
    } else {
        let mut var: u32 = 0;
        (*sprite).y2 = (gSineTable[(*sprite).data[5] as u8] as i32 * 76 / 256) as i16;
        var = (if gSineTable[(*sprite).data[5] as i32 + 64] != 0 {
            div_i32(0x10000, gSineTable[(*sprite).data[5] as i32 + 64] as i32)
        } else {
            0
        }) as u32;
        if var > 0xFFFF {
            var = 0xFFFF;
        }
        SetOamMatrix((*sprite).data[1] as u8 + 1, 0x100, 0, 0, var as u16);
        (*sprite).oam.set_matrixNum(monId as u32 + 1);
        if (*sprite).data[5] > -64 && (*sprite).data[5] < 64 {
            (*sprite).set_invisible(FALSE as u16);
            (*sprite).data[0] = 1;
        } else {
            (*sprite).set_invisible(TRUE as u16);
        }
        if ((*sprite).data[5] <= -64 || (*sprite).data[5] >= 64) && (*sprite).data[0] != 0 {
            FreeAndDestroyMonPicSprite((*sPokedexView).monSpriteIds[monId]);
            (*sPokedexView).monSpriteIds[monId] = 0xFFFF;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Scrollbar(sprite: *mut Sprite) {
    if (*sPokedexView).currentPage != PAGE_MAIN
        && (*sPokedexView).currentPage != PAGE_SEARCH_RESULTS
    {
        DestroySprite(sprite);
    } else {
        (*sprite).y2 = div_i32(
            (*sPokedexView).selectedPokemon as i32 * 120,
            (*sPokedexView).pokemonListCount as i32 - 1,
        ) as i16;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ScrollArrow(sprite: *mut Sprite) {
    if (*sPokedexView).currentPage != PAGE_MAIN
        && (*sPokedexView).currentPage != PAGE_SEARCH_RESULTS
    {
        DestroySprite(sprite);
    } else {
        let mut r0: u8 = 0;
        if (*sprite).data[1] != 0 {
            if (*sPokedexView).selectedPokemon as i32 == (*sPokedexView).pokemonListCount as i32 - 1
            {
                (*sprite).set_invisible(TRUE as u16);
            } else {
                (*sprite).set_invisible(FALSE as u16);
            }
            r0 = (*sprite).data[2] as u8;
        } else {
            if (*sPokedexView).selectedPokemon == 0 {
                (*sprite).set_invisible(TRUE as u16);
            } else {
                (*sprite).set_invisible(FALSE as u16);
            }
            r0 = (*sprite).data[2] as u8 - 128;
        }
        (*sprite).y2 = gSineTable[r0] / 64;
        (*sprite).data[2] = (*sprite).data[2] + 8;
        if (*sPokedexView).menuIsOpen == 0
            && (*sPokedexView).menuY == 0
            && (*sprite).invisible() == 0
        {
            (*sprite).set_invisible(FALSE as u16);
        } else {
            (*sprite).set_invisible(TRUE as u16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DexListInterfaceText(sprite: *mut Sprite) {
    if (*sPokedexView).currentPage != PAGE_MAIN
        && (*sPokedexView).currentPage != PAGE_SEARCH_RESULTS
    {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_RotatingPokeBall(sprite: *mut Sprite) {
    if (*sPokedexView).currentPage != PAGE_MAIN
        && (*sPokedexView).currentPage != PAGE_SEARCH_RESULTS
    {
        DestroySprite(sprite);
    } else {
        let mut val: u8 = 0;
        let mut r3: i16 = 0;
        let mut r0: i16 = 0;
        val = (*sPokedexView).pokeBallRotation + (*sprite).data[1] as u8;
        r3 = gSineTable[val];
        r0 = gSineTable[val as i32 + 64];
        SetOamMatrix(
            (*sprite).data[0] as u8,
            r0 as u16,
            r3 as u16,
            (r3 as u16).wrapping_neg(),
            r0 as u16,
        );
        val = (*sPokedexView).pokeBallRotation + ((*sprite).data[1] as u8 + 64);
        r3 = gSineTable[val];
        r0 = gSineTable[val as i32 + 64];
        (*sprite).x2 = (r0 as i32 * 40 / 256) as i16;
        (*sprite).y2 = (r3 as i32 * 40 / 256) as i16;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DexListStartMenuCursor(sprite: *mut Sprite) {
    if (*sPokedexView).currentPage != PAGE_MAIN
        && (*sPokedexView).currentPage != PAGE_SEARCH_RESULTS
    {
        DestroySprite(sprite);
    } else {
        let mut r1: u16 = (if (*sPokedexView).currentPage == PAGE_MAIN {
            80
        } else {
            96
        }) as u16;
        if (*sPokedexView).menuIsOpen != 0 && (*sPokedexView).menuY as i32 == r1 as i32 {
            (*sprite).set_invisible(FALSE as u16);
            (*sprite).y2 = (*sPokedexView).menuCursorPos as i16 * 16;
            (*sprite).x2 = gSineTable[(*sprite).data[2] as u8] / 64;
            (*sprite).data[2] += 8;
        } else {
            (*sprite).set_invisible(TRUE as u16);
        }
    }
}
pub(crate) unsafe extern "C" fn PrintInfoScreenText(str: *mut u8, left: u8, top: u8) {
    let mut color: CArray<u8, 3> = zeroed();
    color[0] = 0x0;
    color[1] = TEXT_DYNAMIC_COLOR_6;
    color[2] = TEXT_COLOR_LIGHT_GRAY;
    AddTextPrinterParameterized4(
        0,
        FONT_NORMAL,
        left,
        top,
        0,
        0,
        color.as_mut_ptr(),
        TEXT_SKIP_DRAW as i8,
        str,
    );
}
pub(crate) unsafe extern "C" fn LoadInfoScreen(item: *mut PokedexListItem, monSpriteId: u8) -> u8 {
    let mut taskId: u8 = 0;
    sPokedexListItem = item;
    taskId = CreateTask(Some(Task_LoadInfoScreen), 0);
    gTasks[taskId].data[0] = FALSE as i16;
    gTasks[taskId].data[1] = TRUE as i16;
    gTasks[taskId].data[2] = FALSE as i16;
    gTasks[taskId].data[3] = FALSE as i16;
    gTasks[taskId].data[4] = monSpriteId as i16;
    gTasks[taskId].data[5] = SPRITE_NONE as i16;
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sInfoScreen_BgTemplate.as_ptr().cast_mut(), 4);
    SetBgTilemapBuffer(3, AllocZeroed(BG_SCREEN_SIZE));
    SetBgTilemapBuffer(2, AllocZeroed(BG_SCREEN_SIZE));
    SetBgTilemapBuffer(1, AllocZeroed(BG_SCREEN_SIZE));
    SetBgTilemapBuffer(0, AllocZeroed(BG_SCREEN_SIZE));
    InitWindows(sInfoScreen_WindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    return taskId;
}
pub(crate) unsafe extern "C" fn IsInfoScreenScrolling(taskId: u8) -> u8 {
    if gTasks[taskId].data[0] == 0
        && gTasks[taskId].func == Some(Task_HandleInfoScreenInput as unsafe extern "C" fn(u8))
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
pub(crate) unsafe extern "C" fn StartInfoScreenScroll(
    item: *mut PokedexListItem,
    taskId: u8,
) -> u8 {
    sPokedexListItem = item;
    gTasks[taskId].data[0] = TRUE as i16;
    gTasks[taskId].data[1] = FALSE as i16;
    gTasks[taskId].data[2] = FALSE as i16;
    gTasks[taskId].data[3] = FALSE as i16;
    return taskId;
}
pub(crate) unsafe extern "C" fn Task_LoadInfoScreen(taskId: u8) {
    match gMain.state {
        1 => {
            DecompressAndLoadBgGfxUsingHeap(
                3,
                gPokedexMenu_Gfx.as_ptr().cast_mut() as *mut c_void,
                0x2000,
                0,
                0,
            );
            CopyToBgTilemapBuffer(
                3,
                gPokedexInfoScreen_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            FillWindowPixelBuffer(WIN_INFO, 0);
            PutWindowTilemap(WIN_INFO);
            PutWindowTilemap(WIN_FOOTPRINT);
            DrawFootprint(WIN_FOOTPRINT, (*sPokedexListItem).dexNum);
            CopyWindowToVram(WIN_FOOTPRINT, COPYWIN_GFX);
            gMain.state += 1;
        }
        2 => {
            LoadScreenSelectBarMain(0xD);
            HighlightScreenSelectBarItem((*sPokedexView).selectedScreen, 0xD);
            LoadPokedexBgPalette((*sPokedexView).isSearchResults());
            gMain.state += 1;
        }
        3 => {
            gMain.state += 1;
        }
        4 => {
            PrintMonInfo(
                (*sPokedexListItem).dexNum as u32,
                (if (*sPokedexView).dexMode == DEX_MODE_HOENN {
                    0
                } else {
                    TRUE as i32
                }) as u32,
                (*sPokedexListItem).owned() as u32,
                0,
            );
            if (*sPokedexListItem).owned() == 0 {
                LoadPalette(&raw mut gPlttBufferUnfaded[1] as *mut c_void, 49, 30);
            }
            CopyWindowToVram(WIN_INFO, COPYWIN_FULL);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
            CopyBgTilemapBufferToVram(3);
            gMain.state += 1;
        }
        5 => {
            if gTasks[taskId].data[1] == 0 {
                gTasks[taskId].data[4] = CreateMonSpriteFromNationalDexNumber(
                    (*sPokedexListItem).dexNum,
                    MON_PAGE_X,
                    MON_PAGE_Y,
                    0,
                ) as i16;
                gSprites[gTasks[taskId].data[4]].oam.set_priority(0);
            }
            gMain.state += 1;
        }
        6 => {
            let mut preservedPalettes: u32 = 0;
            if gTasks[taskId].data[2] != 0 {
                preservedPalettes = 0x14;
            }
            if gTasks[taskId].data[1] != 0 {
                preservedPalettes |= shl_i32(
                    1,
                    gSprites[gTasks[taskId].data[4]].oam.paletteNum() as u32 + 16,
                ) as u32;
            }
            BeginNormalPaletteFade(!preservedPalettes, 0, 16, 0, 0);
            SetVBlankCallback(gPokedexVBlankCB);
            gMain.state += 1;
        }
        7 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            SetGpuReg(REG_OFFSET_DISPCNT, 4160);
            HideBg(0);
            ShowBg(1);
            ShowBg(2);
            ShowBg(3);
            gMain.state += 1;
        }
        8 => {
            if gPaletteFade.active() == 0 {
                gMain.state += 1;
                if gTasks[taskId].data[3] == 0 {
                    StopCryAndClearCrySongs();
                    PlayCry_NormalNoDucking(
                        NationalPokedexNumToSpecies((*sPokedexListItem).dexNum),
                        0,
                        CRY_VOLUME_RS,
                        CRY_PRIORITY_NORMAL,
                    );
                } else {
                    gMain.state += 1;
                }
            }
        }
        9 => {
            if IsCryPlayingOrClearCrySongs() == 0 {
                gMain.state += 1;
            }
        }
        10 => {
            gTasks[taskId].data[0] = FALSE as i16;
            gTasks[taskId].data[1] = FALSE as i16;
            gTasks[taskId].data[2] = TRUE as i16;
            gTasks[taskId].data[3] = TRUE as i16;
            gTasks[taskId].func = Some(Task_HandleInfoScreenInput);
            gMain.state = 0;
        }
        _ => {
            if gPaletteFade.active() == 0 {
                let mut r2: u16 = 0;
                (*sPokedexView).currentPage = PAGE_INFO;
                gPokedexVBlankCB = gMain.vblankCallback;
                SetVBlankCallback(None);
                r2 = 0;
                if gTasks[taskId].data[1] != 0 {
                    r2 += DISPCNT_OBJ_ON;
                }
                if gTasks[taskId].data[2] != 0 {
                    r2 |= DISPCNT_BG1_ON;
                }
                ResetOtherVideoRegisters(r2);
                gMain.state = 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FreeInfoScreenWindowAndBgBuffers() {
    let mut tilemapBuffer: *mut c_void = null_mut();
    FreeAllWindowBuffers();
    tilemapBuffer = GetBgTilemapBuffer(0);
    if !tilemapBuffer.is_null() {
        Free(tilemapBuffer);
    }
    tilemapBuffer = GetBgTilemapBuffer(1);
    if !tilemapBuffer.is_null() {
        Free(tilemapBuffer);
    }
    tilemapBuffer = GetBgTilemapBuffer(2);
    if !tilemapBuffer.is_null() {
        Free(tilemapBuffer);
    }
    tilemapBuffer = GetBgTilemapBuffer(3);
    if !tilemapBuffer.is_null() {
        Free(tilemapBuffer);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleInfoScreenInput(taskId: u8) {
    if gTasks[taskId].data[0] != 0 {
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        gTasks[taskId].func = Some(Task_LoadInfoScreenWaitForFade);
        PlaySE(SE_DEX_SCROLL);
        return;
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        gTasks[taskId].func = Some(Task_ExitInfoScreen);
        PlaySE(SE_PC_OFF);
        return;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        match (*sPokedexView).selectedScreen {
            AREA_SCREEN => {
                BeginNormalPaletteFade(0xffffffeb, 0, 0, 16, 0);
                (*sPokedexView).screenSwitchState = 1;
                gTasks[taskId].func = Some(Task_SwitchScreensFromInfoScreen);
                PlaySE(SE_PIN);
            }
            CRY_SCREEN => {
                BeginNormalPaletteFade(0xffffffeb, 0, 0, 0x10, 0);
                (*sPokedexView).screenSwitchState = 2;
                gTasks[taskId].func = Some(Task_SwitchScreensFromInfoScreen);
                PlaySE(SE_PIN);
            }
            SIZE_SCREEN => {
                if (*sPokedexListItem).owned() == 0 {
                    PlaySE(SE_FAILURE);
                } else {
                    BeginNormalPaletteFade(0xffffffeb, 0, 0, 0x10, 0);
                    (*sPokedexView).screenSwitchState = 3;
                    gTasks[taskId].func = Some(Task_SwitchScreensFromInfoScreen);
                    PlaySE(SE_PIN);
                }
            }
            CANCEL_SCREEN => {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
                gTasks[taskId].func = Some(Task_ExitInfoScreen);
                PlaySE(SE_PC_OFF);
            }
            _ => {}
        }
        return;
    }
    if (gMain.newKeys as i32 & DPAD_LEFT != 0
        || gMain.newKeys as i32 & L_BUTTON != 0
            && (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_LR)
        && (*sPokedexView).selectedScreen > 0
    {
        (*sPokedexView).selectedScreen -= 1;
        HighlightScreenSelectBarItem((*sPokedexView).selectedScreen, 0xD);
        PlaySE(SE_DEX_PAGE);
        return;
    }
    if (gMain.newKeys as i32 & DPAD_RIGHT != 0
        || gMain.newKeys as i32 & R_BUTTON != 0
            && (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_LR)
        && (*sPokedexView).selectedScreen < CANCEL_SCREEN
    {
        (*sPokedexView).selectedScreen += 1;
        HighlightScreenSelectBarItem((*sPokedexView).selectedScreen, 0xD);
        PlaySE(SE_DEX_PAGE);
        return;
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchScreensFromInfoScreen(taskId: u8) {
    if gPaletteFade.active() == 0 {
        FreeAndDestroyMonPicSprite(gTasks[taskId].data[4] as u16);
        match (*sPokedexView).screenSwitchState {
            2 => {
                gTasks[taskId].func = Some(Task_LoadCryScreen);
            }
            3 => {
                gTasks[taskId].func = Some(Task_LoadSizeScreen);
            }
            _ => {
                gTasks[taskId].func = Some(Task_LoadAreaScreen);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LoadInfoScreenWaitForFade(taskId: u8) {
    if gPaletteFade.active() == 0 {
        FreeAndDestroyMonPicSprite(gTasks[taskId].data[4] as u16);
        gTasks[taskId].func = Some(Task_LoadInfoScreen);
    }
}
pub(crate) unsafe extern "C" fn Task_ExitInfoScreen(taskId: u8) {
    if gPaletteFade.active() == 0 {
        FreeAndDestroyMonPicSprite(gTasks[taskId].data[4] as u16);
        FreeInfoScreenWindowAndBgBuffers();
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_LoadAreaScreen(taskId: u8) {
    match gMain.state {
        1 => {
            LoadScreenSelectBarSubmenu(0xD);
            HighlightSubmenuScreenSelectBarItem(0, 0xD);
            LoadPokedexBgPalette((*sPokedexView).isSearchResults());
            SetGpuReg(REG_OFFSET_BG1CNT, 3328);
            gMain.state += 1;
        }
        2 => {
            ShowPokedexAreaScreen(
                NationalPokedexNumToSpecies((*sPokedexListItem).dexNum),
                &raw mut (*sPokedexView).screenSwitchState,
            );
            SetVBlankCallback(gPokedexVBlankCB);
            (*sPokedexView).screenSwitchState = 0;
            gMain.state = 0;
            gTasks[taskId].func = Some(Task_WaitForAreaScreenInput);
        }
        _ => {
            if gPaletteFade.active() == 0 {
                (*sPokedexView).currentPage = PAGE_AREA;
                gPokedexVBlankCB = gMain.vblankCallback;
                SetVBlankCallback(None);
                ResetOtherVideoRegisters(DISPCNT_BG1_ON);
                (*sPokedexView).selectedScreen = AREA_SCREEN;
                gMain.state = 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForAreaScreenInput(taskId: u8) {
    if (*sPokedexView).screenSwitchState != 0 {
        gTasks[taskId].func = Some(Task_SwitchScreensFromAreaScreen);
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchScreensFromAreaScreen(taskId: u8) {
    if gPaletteFade.active() == 0 {
        match (*sPokedexView).screenSwitchState {
            2 => {
                gTasks[taskId].func = Some(Task_LoadCryScreen);
            }
            _ => {
                gTasks[taskId].func = Some(Task_LoadInfoScreen);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LoadCryScreen(taskId: u8) {
    match gMain.state {
        1 => {
            DecompressAndLoadBgGfxUsingHeap(
                3,
                (&raw const gPokedexMenu_Gfx).cast_mut() as *mut c_void,
                0x2000,
                0,
                0,
            );
            CopyToBgTilemapBuffer(
                3,
                (&raw const gPokedexCryScreen_Tilemap).cast_mut() as *mut c_void,
                0,
                0,
            );
            FillWindowPixelBuffer(WIN_INFO, 0);
            PutWindowTilemap(WIN_INFO);
            PutWindowTilemap(WIN_VU_METER);
            PutWindowTilemap(WIN_CRY_WAVE);
            gMain.state += 1;
        }
        2 => {
            LoadScreenSelectBarSubmenu(0xD);
            HighlightSubmenuScreenSelectBarItem(1, 0xD);
            LoadPokedexBgPalette((*sPokedexView).isSearchResults());
            gMain.state += 1;
        }
        3 => {
            ResetPaletteFade();
            gMain.state += 1;
        }
        4 => {
            PrintInfoScreenText(gText_CryOf.as_ptr().cast_mut(), 82, 33);
            PrintCryScreenSpeciesName(0, (*sPokedexListItem).dexNum, 82, 49);
            gMain.state += 1;
        }
        5 => {
            gTasks[taskId].data[4] = CreateMonSpriteFromNationalDexNumber(
                (*sPokedexListItem).dexNum,
                MON_PAGE_X,
                MON_PAGE_Y,
                0,
            ) as i16;
            gSprites[gTasks[taskId].data[4]].oam.set_priority(0);
            gDexCryScreenState = 0;
            gMain.state += 1;
        }
        6 => {
            let mut waveformWindow: CryScreenWindow = zeroed();
            waveformWindow.unk0 = 0x4020;
            waveformWindow.unk2 = 31;
            waveformWindow.paletteNo = 8;
            waveformWindow.yPos = 30;
            waveformWindow.xPos = 12;
            if LoadCryWaveformWindow(&raw mut waveformWindow, 2) != 0 {
                gMain.state += 1;
                gDexCryScreenState = 0;
            }
        }
        7 => {
            let mut cryMeter: CryScreenWindow = zeroed();
            cryMeter.paletteNo = 9;
            cryMeter.xPos = 18;
            cryMeter.yPos = 3;
            if LoadCryMeter(&raw mut cryMeter, 3) != 0 {
                gMain.state += 1;
            }
            CopyWindowToVram(WIN_VU_METER, COPYWIN_GFX);
            CopyWindowToVram(WIN_INFO, COPYWIN_FULL);
            CopyBgTilemapBufferToVram(0);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
            CopyBgTilemapBufferToVram(3);
        }
        8 => {
            BeginNormalPaletteFade(0xffffffeb, 0, 0x10, 0, 0);
            SetVBlankCallback(gPokedexVBlankCB);
            gMain.state += 1;
        }
        9 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            SetGpuReg(REG_OFFSET_DISPCNT, 4160);
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            ShowBg(3);
            gMain.state += 1;
        }
        10 => {
            (*sPokedexView).screenSwitchState = 0;
            gMain.state = 0;
            gTasks[taskId].func = Some(Task_HandleCryScreenInput);
        }
        _ => {
            if gPaletteFade.active() == 0 {
                m4aMPlayStop(&raw mut gMPlayInfo_BGM);
                (*sPokedexView).currentPage = PAGE_CRY;
                gPokedexVBlankCB = gMain.vblankCallback;
                SetVBlankCallback(None);
                ResetOtherVideoRegisters(DISPCNT_BG1_ON);
                (*sPokedexView).selectedScreen = CRY_SCREEN;
                gMain.state = 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleCryScreenInput(taskId: u8) {
    UpdateCryWaveformWindow(2);
    if IsCryPlaying() != 0 {
        LoadPlayArrowPalette(TRUE);
    } else {
        LoadPlayArrowPalette(FALSE);
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        LoadPlayArrowPalette(TRUE);
        CryScreenPlayButton(NationalPokedexNumToSpecies((*sPokedexListItem).dexNum));
        return;
    } else if gPaletteFade.active() == 0 {
        if gMain.newKeys as i32 & B_BUTTON != 0 {
            BeginNormalPaletteFade(0xffffffeb, 0, 0, 0x10, 0);
            m4aMPlayContinue(&raw mut gMPlayInfo_BGM);
            (*sPokedexView).screenSwitchState = 1;
            gTasks[taskId].func = Some(Task_SwitchScreensFromCryScreen);
            PlaySE(SE_PC_OFF);
            return;
        }
        if gMain.newKeys as i32 & DPAD_LEFT != 0
            || gMain.newKeys as i32 & L_BUTTON != 0
                && (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_LR
        {
            BeginNormalPaletteFade(0xffffffeb, 0, 0, 0x10, 0);
            m4aMPlayContinue(&raw mut gMPlayInfo_BGM);
            (*sPokedexView).screenSwitchState = 2;
            gTasks[taskId].func = Some(Task_SwitchScreensFromCryScreen);
            PlaySE(SE_DEX_PAGE);
            return;
        }
        if gMain.newKeys as i32 & DPAD_RIGHT != 0
            || gMain.newKeys as i32 & R_BUTTON != 0
                && (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_LR
        {
            if (*sPokedexListItem).owned() == 0 {
                PlaySE(SE_FAILURE);
            } else {
                BeginNormalPaletteFade(0xffffffeb, 0, 0, 0x10, 0);
                m4aMPlayContinue(&raw mut gMPlayInfo_BGM);
                (*sPokedexView).screenSwitchState = 3;
                gTasks[taskId].func = Some(Task_SwitchScreensFromCryScreen);
                PlaySE(SE_DEX_PAGE);
            }
            return;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchScreensFromCryScreen(taskId: u8) {
    if gPaletteFade.active() == 0 {
        FreeCryScreen();
        FreeAndDestroyMonPicSprite(gTasks[taskId].data[4] as u16);
        match (*sPokedexView).screenSwitchState {
            2 => {
                gTasks[taskId].func = Some(Task_LoadAreaScreen);
            }
            3 => {
                gTasks[taskId].func = Some(Task_LoadSizeScreen);
            }
            _ => {
                gTasks[taskId].func = Some(Task_LoadInfoScreen);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadPlayArrowPalette(cryPlaying: u8) {
    let mut color: u16 = 0;
    if cryPlaying != 0 {
        color = 914;
    } else {
        color = 687;
    }
    LoadPalette(&raw mut color as *mut c_void, 93, 2);
}
pub(crate) unsafe extern "C" fn Task_LoadSizeScreen(taskId: u8) {
    let mut spriteId: u8 = 0;
    match gMain.state {
        1 => {
            DecompressAndLoadBgGfxUsingHeap(
                3,
                gPokedexMenu_Gfx.as_ptr().cast_mut() as *mut c_void,
                0x2000,
                0,
                0,
            );
            CopyToBgTilemapBuffer(
                3,
                gPokedexSizeScreen_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            FillWindowPixelBuffer(WIN_INFO, 0);
            PutWindowTilemap(WIN_INFO);
            gMain.state += 1;
        }
        2 => {
            LoadScreenSelectBarSubmenu(0xD);
            HighlightSubmenuScreenSelectBarItem(2, 0xD);
            LoadPokedexBgPalette((*sPokedexView).isSearchResults());
            gMain.state += 1;
        }
        3 => {
            let mut string: CArray<u8, 64> = zeroed();
            StringCopy(
                string.as_mut_ptr(),
                gText_SizeComparedTo.as_ptr().cast_mut(),
            );
            StringAppend(
                string.as_mut_ptr(),
                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
            );
            PrintInfoScreenText(
                string.as_mut_ptr(),
                GetStringCenterAlignXOffset(
                    FONT_NORMAL as i32,
                    string.as_mut_ptr(),
                    DISPLAY_WIDTH as i32,
                ) as u8,
                121,
            );
            gMain.state += 1;
        }
        4 => {
            ResetPaletteFade();
            gMain.state += 1;
        }
        5 => {
            spriteId = CreateSizeScreenTrainerPic(
                PlayerGenderToFrontTrainerPicId((*gSaveBlock2Ptr).playerGender),
                152,
                56,
                0,
            ) as u8;
            gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
            gSprites[spriteId].oam.set_matrixNum(1);
            gSprites[spriteId].oam.set_priority(0);
            gSprites[spriteId].y2 =
                gPokedexEntries[(*sPokedexListItem).dexNum].trainerOffset as i16;
            SetOamMatrix(
                1,
                gPokedexEntries[(*sPokedexListItem).dexNum].trainerScale,
                0,
                0,
                gPokedexEntries[(*sPokedexListItem).dexNum].trainerScale,
            );
            LoadPalette(
                sSizeScreenSilhouette_Pal.as_ptr().cast_mut() as *mut c_void,
                (gSprites[spriteId].oam.paletteNum() + 16) * 16,
                32,
            );
            gTasks[taskId].data[5] = spriteId as i16;
            gMain.state += 1;
        }
        6 => {
            spriteId =
                CreateMonSpriteFromNationalDexNumber((*sPokedexListItem).dexNum, 88, 56, 1) as u8;
            gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
            gSprites[spriteId].oam.set_matrixNum(2);
            gSprites[spriteId].oam.set_priority(0);
            gSprites[spriteId].y2 =
                gPokedexEntries[(*sPokedexListItem).dexNum].pokemonOffset as i16;
            SetOamMatrix(
                2,
                gPokedexEntries[(*sPokedexListItem).dexNum].pokemonScale,
                0,
                0,
                gPokedexEntries[(*sPokedexListItem).dexNum].pokemonScale,
            );
            LoadPalette(
                sSizeScreenSilhouette_Pal.as_ptr().cast_mut() as *mut c_void,
                (gSprites[spriteId].oam.paletteNum() + 16) * 16,
                32,
            );
            gTasks[taskId].data[4] = spriteId as i16;
            CopyWindowToVram(WIN_INFO, COPYWIN_FULL);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
            CopyBgTilemapBufferToVram(3);
            gMain.state += 1;
        }
        7 => {
            BeginNormalPaletteFade(0xffffffeb, 0, 0x10, 0, 0);
            SetVBlankCallback(gPokedexVBlankCB);
            gMain.state += 1;
        }
        8 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            SetGpuReg(REG_OFFSET_DISPCNT, 4160);
            HideBg(0);
            ShowBg(1);
            ShowBg(2);
            ShowBg(3);
            gMain.state += 1;
        }
        9 => {
            if gPaletteFade.active() == 0 {
                (*sPokedexView).screenSwitchState = 0;
                gMain.state = 0;
                gTasks[taskId].func = Some(Task_HandleSizeScreenInput);
            }
        }
        _ => {
            if gPaletteFade.active() == 0 {
                (*sPokedexView).currentPage = PAGE_SIZE;
                gPokedexVBlankCB = gMain.vblankCallback;
                SetVBlankCallback(None);
                ResetOtherVideoRegisters(DISPCNT_BG1_ON);
                (*sPokedexView).selectedScreen = SIZE_SCREEN;
                gMain.state = 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSizeScreenInput(taskId: u8) {
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        BeginNormalPaletteFade(0xffffffeb, 0, 0, 0x10, 0);
        (*sPokedexView).screenSwitchState = 1;
        gTasks[taskId].func = Some(Task_SwitchScreensFromSizeScreen);
        PlaySE(SE_PC_OFF);
    } else if gMain.newKeys as i32 & DPAD_LEFT != 0
        || gMain.newKeys as i32 & L_BUTTON != 0
            && (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_LR
    {
        BeginNormalPaletteFade(0xffffffeb, 0, 0, 0x10, 0);
        (*sPokedexView).screenSwitchState = 2;
        gTasks[taskId].func = Some(Task_SwitchScreensFromSizeScreen);
        PlaySE(SE_DEX_PAGE);
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchScreensFromSizeScreen(taskId: u8) {
    if gPaletteFade.active() == 0 {
        FreeAndDestroyMonPicSprite(gTasks[taskId].data[4] as u16);
        FreeAndDestroyTrainerPicSprite(gTasks[taskId].data[5] as u16);
        match (*sPokedexView).screenSwitchState {
            2 => {
                gTasks[taskId].func = Some(Task_LoadCryScreen);
            }
            _ => {
                gTasks[taskId].func = Some(Task_LoadInfoScreen);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadScreenSelectBarMain(unused: u16) {
    CopyToBgTilemapBuffer(
        1,
        gPokedexScreenSelectBarMain_Tilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
    );
}
pub(crate) unsafe extern "C" fn LoadScreenSelectBarSubmenu(unused: u16) {
    CopyToBgTilemapBuffer(
        1,
        gPokedexScreenSelectBarSubmenu_Tilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
    );
}
pub(crate) unsafe extern "C" fn HighlightScreenSelectBarItem(selectedScreen: u8, unused: u16) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut ptr: *mut u16 = GetBgTilemapBuffer(1) as *mut u16;
    i = 0;
    while i < SCREEN_COUNT {
        let mut row: u8 = i * 7 + 1;
        let mut newPalette: u16 = 0;
        newPalette = 0x4000;
        if i == selectedScreen {
            newPalette = 0x2000;
        }
        j = 0;
        while j < 7 {
            *ptr.at(row as i32 + j as i32) =
                (*ptr.at(row as i32 + j as i32) as i32 % 4096) as u16 | newPalette;
            *ptr.at(row as i32 + j as i32 + 0x20) =
                (*ptr.at(row as i32 + j as i32 + 0x20) as i32 % 4096) as u16 | newPalette;
            j += 1;
        }
        i += 1;
    }
    CopyBgTilemapBufferToVram(1);
}
pub(crate) unsafe extern "C" fn HighlightSubmenuScreenSelectBarItem(a: u8, b: u16) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut ptr: *mut u16 = GetBgTilemapBuffer(1) as *mut u16;
    i = 0;
    while i < 4 {
        let mut row: u8 = i * 7 + 1;
        let mut newPalette: u32 = 0;
        if i == a || i == 3 {
            newPalette = 0x2000;
        } else {
            newPalette = 0x4000;
        }
        j = 0;
        while j < 7 {
            *ptr.at(row as i32 + j as i32) =
                (*ptr.at(row as i32 + j as i32) as i32 % 4096) as u16 | newPalette as u16;
            *ptr.at(row as i32 + j as i32 + 0x20) =
                (*ptr.at(row as i32 + j as i32 + 0x20) as i32 % 4096) as u16 | newPalette as u16;
            j += 1;
        }
        i += 1;
    }
    CopyBgTilemapBufferToVram(1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisplayCaughtMonDexPage(dexNum: u16, otId: u32, personality: u32) -> u8 {
    let mut taskId: u8 = CreateTask(Some(Task_DisplayCaughtMonDexPage), 0);
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[1] = dexNum as i16;
    gTasks[taskId].data[12] = otId as i16;
    gTasks[taskId].data[13] = (otId >> 16) as i16;
    gTasks[taskId].data[14] = personality as i16;
    gTasks[taskId].data[15] = (personality >> 16) as i16;
    return taskId;
}
pub(crate) unsafe extern "C" fn Task_DisplayCaughtMonDexPage(taskId: u8) {
    let mut spriteId: u8 = 0;
    let mut dexNum: u16 = gTasks[taskId].data[1] as u16;
    match gTasks[taskId].data[0] {
        1 => {
            DecompressAndLoadBgGfxUsingHeap(
                3,
                gPokedexMenu_Gfx.as_ptr().cast_mut() as *mut c_void,
                0x2000,
                0,
                0,
            );
            CopyToBgTilemapBuffer(
                3,
                gPokedexInfoScreen_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            FillWindowPixelBuffer(WIN_INFO, 0);
            PutWindowTilemap(WIN_INFO);
            PutWindowTilemap(WIN_FOOTPRINT);
            DrawFootprint(WIN_FOOTPRINT, gTasks[taskId].data[1] as u16);
            CopyWindowToVram(WIN_FOOTPRINT, COPYWIN_GFX);
            ResetPaletteFade();
            LoadPokedexBgPalette(FALSE);
            gTasks[taskId].data[0] += 1;
        }
        2 => {
            gTasks[taskId].data[0] += 1;
        }
        3 => {
            PrintMonInfo(dexNum as u32, IsNationalPokedexEnabled(), 1, 1);
            CopyWindowToVram(WIN_INFO, COPYWIN_FULL);
            CopyBgTilemapBufferToVram(2);
            CopyBgTilemapBufferToVram(3);
            gTasks[taskId].data[0] += 1;
        }
        4 => {
            spriteId =
                CreateMonSpriteFromNationalDexNumber(dexNum, MON_PAGE_X, MON_PAGE_Y, 0) as u8;
            gSprites[spriteId].oam.set_priority(0);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
            SetVBlankCallback(gPokedexVBlankCB);
            gTasks[taskId].data[3] = spriteId as i16;
            gTasks[taskId].data[0] += 1;
        }
        5 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            SetGpuReg(REG_OFFSET_DISPCNT, 4160);
            ShowBg(2);
            ShowBg(3);
            gTasks[taskId].data[0] += 1;
        }
        6 => {
            if gPaletteFade.active() == 0 {
                PlayCry_Normal(NationalPokedexNumToSpecies(dexNum), 0);
                gTasks[taskId].data[2] = 0;
                gTasks[taskId].func = Some(Task_HandleCaughtMonPageInput);
            }
        }
        _ => {
            if gPaletteFade.active() == 0 {
                gPokedexVBlankCB = gMain.vblankCallback;
                SetVBlankCallback(None);
                ResetOtherVideoRegisters(DISPCNT_BG0_ON);
                ResetBgsAndClearDma3BusyFlags(0);
                InitBgsFromTemplates(0, sNewEntryInfoScreen_BgTemplate.as_ptr().cast_mut(), 2);
                SetBgTilemapBuffer(3, AllocZeroed(BG_SCREEN_SIZE));
                SetBgTilemapBuffer(2, AllocZeroed(BG_SCREEN_SIZE));
                InitWindows(sNewEntryInfoScreen_WindowTemplates.as_ptr().cast_mut());
                DeactivateAllTextPrinters();
                gTasks[taskId].data[0] = 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleCaughtMonPageInput(taskId: u8) {
    if gMain.newKeys as i32 & 3 != 0 {
        BeginNormalPaletteFade(PALETTES_BG, 0, 0, 16, 0);
        gSprites[gTasks[taskId].data[3]].callback = Some(SpriteCB_SlideCaughtMonToCenter);
        gTasks[taskId].func = Some(Task_ExitCaughtMonPage);
    } else if ({
        gTasks[taskId].data[2] += 1;
        gTasks[taskId].data[2]
    }) as i32
        & 16
        != 0
    {
        LoadPalette(
            gPokedexBgHoenn_Pal.as_ptr().cast_mut().at(1) as *mut c_void,
            49,
            14,
        );
    } else {
        LoadPalette(
            gPokedexBgHoenn_Pal.as_ptr().cast_mut().at(49) as *mut c_void,
            49,
            14,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_ExitCaughtMonPage(taskId: u8) {
    if gPaletteFade.active() == 0 {
        let mut species: u16 = 0;
        let mut otId: u32 = 0;
        let mut personality: u32 = 0;
        let mut paletteNum: u8 = 0;
        let mut lzPaletteData: *mut u32 = null_mut();
        let mut buffer: *mut c_void = null_mut();
        SetGpuReg(REG_OFFSET_DISPCNT, 4160);
        FreeAllWindowBuffers();
        buffer = GetBgTilemapBuffer(2);
        if !buffer.is_null() {
            Free(buffer);
        }
        buffer = GetBgTilemapBuffer(3);
        if !buffer.is_null() {
            Free(buffer);
        }
        species = NationalPokedexNumToSpecies(gTasks[taskId].data[1] as u16);
        otId =
            (gTasks[taskId].data[13] as u16 as u32) << 16 | gTasks[taskId].data[12] as u16 as u32;
        personality =
            (gTasks[taskId].data[15] as u16 as u32) << 16 | gTasks[taskId].data[14] as u16 as u32;
        paletteNum = gSprites[gTasks[taskId].data[3]].oam.paletteNum() as u8;
        lzPaletteData = GetMonSpritePalFromSpeciesAndPersonality(species, otId, personality);
        LoadCompressedPalette(lzPaletteData, 0x100 + paletteNum as u16 * 16, 32);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_SlideCaughtMonToCenter(sprite: *mut Sprite) {
    if (*sprite).x < 120 {
        (*sprite).x += 2;
    }
    if (*sprite).x > 120 {
        (*sprite).x -= 2;
    }
    if (*sprite).y < 80 {
        (*sprite).y += 1;
    }
    if (*sprite).y > 80 {
        (*sprite).y -= 1;
    }
}
pub(crate) unsafe extern "C" fn PrintMonInfo(num: u32, mut value: u32, owned: u32, newEntry: u32) {
    let mut str: CArray<u8, 16> = zeroed();
    let mut str2: CArray<u8, 32> = zeroed();
    let mut natNum: u16 = 0;
    let mut name: *mut u8 = null_mut();
    let mut category: *mut u8 = null_mut();
    let mut description: *mut u8 = null_mut();
    if newEntry != 0 {
        PrintInfoScreenText(
            gText_PokedexRegistration.as_ptr().cast_mut(),
            GetStringCenterAlignXOffset(
                FONT_NORMAL as i32,
                gText_PokedexRegistration.as_ptr().cast_mut(),
                DISPLAY_WIDTH as i32,
            ) as u8,
            0,
        );
    }
    if value == 0 {
        value = NationalToHoennOrder(num as u16) as u32;
    } else {
        value = num;
    }
    ConvertIntToDecimalStringN(
        StringCopy(str.as_mut_ptr(), gText_NumberClear01.as_ptr().cast_mut()),
        value as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        3,
    );
    PrintInfoScreenText(str.as_mut_ptr(), 0x60, 0x19);
    natNum = NationalPokedexNumToSpecies(num as u16);
    if natNum != 0 {
        name = gSpeciesNames[natNum].as_ptr().cast_mut();
    } else {
        name = sText_TenDashes2.as_ptr().cast_mut();
    }
    PrintInfoScreenText(name, 0x84, 0x19);
    if owned != 0 {
        CopyMonCategoryText(num as i32, str2.as_mut_ptr());
        category = str2.as_mut_ptr();
    } else {
        category = gText_5MarksPokemon.as_ptr().cast_mut();
    }
    PrintInfoScreenText(category, 0x64, 0x29);
    PrintInfoScreenText(gText_HTHeight.as_ptr().cast_mut(), 0x60, 0x39);
    PrintInfoScreenText(gText_WTWeight.as_ptr().cast_mut(), 0x60, 0x49);
    if owned != 0 {
        PrintMonHeight(gPokedexEntries[num].height, 0x81, 0x39);
        PrintMonWeight(gPokedexEntries[num].weight, 0x81, 0x49);
    } else {
        PrintInfoScreenText(gText_UnkHeight.as_ptr().cast_mut(), 0x81, 0x39);
        PrintInfoScreenText(gText_UnkWeight.as_ptr().cast_mut(), 0x81, 0x49);
    }
    if owned != 0 {
        description = gPokedexEntries[num].description;
    } else {
        description = sExpandedPlaceholder_PokedexDescription.as_ptr().cast_mut();
    }
    PrintInfoScreenText(
        description,
        GetStringCenterAlignXOffset(FONT_NORMAL as i32, description, DISPLAY_WIDTH as i32) as u8,
        95,
    );
}
pub(crate) unsafe extern "C" fn PrintMonHeight(height: u16, left: u8, top: u8) {
    let mut buffer: CArray<u8, 16> = zeroed();
    let mut inches: u32 = 0;
    let mut feet: u32 = 0;
    let mut i: u8 = 0;
    inches = (height as i32 * 10000 / 254) as u32;
    if inches % 10 >= 5 {
        inches += 10;
    }
    feet = inches / 120;
    inches = (inches - feet * 120) / 10;
    buffer[{
        let t1 = i;
        i += 1;
        t1
    }] = EXT_CTRL_CODE_BEGIN;
    buffer[{
        let t2 = i;
        i += 1;
        t2
    }] = EXT_CTRL_CODE_CLEAR_TO;
    if feet / 10 == 0 {
        buffer[{
            let t3 = i;
            i += 1;
            t3
        }] = 18;
        buffer[{
            let t4 = i;
            i += 1;
            t4
        }] = feet as u8 + CHAR_0;
    } else {
        buffer[{
            let t5 = i;
            i += 1;
            t5
        }] = 12;
        buffer[{
            let t6 = i;
            i += 1;
            t6
        }] = (feet / 10) as u8 + CHAR_0;
        buffer[{
            let t7 = i;
            i += 1;
            t7
        }] = (feet % 10) as u8 + CHAR_0;
    }
    buffer[{
        let t8 = i;
        i += 1;
        t8
    }] = CHAR_SGL_QUOTE_RIGHT;
    buffer[{
        let t9 = i;
        i += 1;
        t9
    }] = (inches / 10) as u8 + CHAR_0;
    buffer[{
        let t10 = i;
        i += 1;
        t10
    }] = (inches % 10) as u8 + CHAR_0;
    buffer[{
        let t11 = i;
        i += 1;
        t11
    }] = CHAR_DBL_QUOTE_RIGHT;
    buffer[{
        let t12 = i;
        i += 1;
        t12
    }] = EOS;
    PrintInfoScreenText(buffer.as_mut_ptr(), left, top);
}
pub(crate) unsafe extern "C" fn PrintMonWeight(weight: u16, left: u8, top: u8) {
    let mut buffer: CArray<u8, 16> = zeroed();
    let mut output: u8 = 0;
    let mut i: u8 = 0;
    let mut lbs: u32 = (weight as i32 * 100000 / 4536) as u32;
    if lbs % 10 >= 5 {
        lbs += 10;
    }
    i = 0;
    output = FALSE;
    if ({
        buffer[i] = (lbs / 0x186a0) as u8 + CHAR_0;
        buffer[i]
    }) == CHAR_0
        && output == 0
    {
        buffer[{
            let t1 = i;
            i += 1;
            t1
        }] = CHAR_SPACER;
    } else {
        output = TRUE;
        i += 1;
    }
    lbs = lbs % 0x186a0;
    if ({
        buffer[i] = (lbs / 10000) as u8 + CHAR_0;
        buffer[i]
    }) == CHAR_0
        && output == 0
    {
        buffer[{
            let t2 = i;
            i += 1;
            t2
        }] = CHAR_SPACER;
    } else {
        output = TRUE;
        i += 1;
    }
    lbs = lbs % 10000;
    if ({
        buffer[i] = (lbs / 1000) as u8 + CHAR_0;
        buffer[i]
    }) == CHAR_0
        && output == 0
    {
        buffer[{
            let t3 = i;
            i += 1;
            t3
        }] = CHAR_SPACER;
    } else {
        output = TRUE;
        i += 1;
    }
    lbs = lbs % 1000;
    buffer[{
        let t4 = i;
        i += 1;
        t4
    }] = (lbs / 100) as u8 + CHAR_0;
    lbs = lbs % 100;
    buffer[{
        let t5 = i;
        i += 1;
        t5
    }] = CHAR_PERIOD;
    buffer[{
        let t6 = i;
        i += 1;
        t6
    }] = (lbs / 10) as u8 + CHAR_0;
    buffer[{
        let t7 = i;
        i += 1;
        t7
    }] = CHAR_SPACE;
    buffer[{
        let t8 = i;
        i += 1;
        t8
    }] = CHAR_l;
    buffer[{
        let t9 = i;
        i += 1;
        t9
    }] = CHAR_b;
    buffer[{
        let t10 = i;
        i += 1;
        t10
    }] = CHAR_s;
    buffer[{
        let t11 = i;
        i += 1;
        t11
    }] = CHAR_PERIOD;
    buffer[{
        let t12 = i;
        i += 1;
        t12
    }] = EOS;
    PrintInfoScreenText(buffer.as_mut_ptr(), left, top);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokedexCategoryName(dexNum: u16) -> *mut u8 {
    return gPokedexEntries[dexNum].categoryName.as_ptr().cast_mut();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokedexHeightWeight(dexNum: u16, data: u8) -> u16 {
    match data {
        0 => {
            return gPokedexEntries[dexNum].height;
        }
        1 => {
            return gPokedexEntries[dexNum].weight;
        }
        _ => {
            return 1;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSetPokedexFlag(mut nationalDexNo: u16, caseID: u8) -> i8 {
    let mut index: u8 = 0;
    let mut bit: u8 = 0;
    let mut mask: u8 = 0;
    let mut retVal: i8 = 0;
    nationalDexNo -= 1;
    index = (nationalDexNo as i32 / 8) as u8;
    bit = (nationalDexNo as i32 % 8) as u8;
    mask = shl_i32(1, bit as u32) as u8;
    retVal = 0;
    match caseID {
        FLAG_GET_SEEN => {
            if (*gSaveBlock2Ptr).pokedex.seen[index] as i32 & mask as i32 != 0 {
                if (*gSaveBlock2Ptr).pokedex.seen[index] as i32 & mask as i32
                    == (*gSaveBlock1Ptr).seen1[index] as i32 & mask as i32
                    && (*gSaveBlock2Ptr).pokedex.seen[index] as i32 & mask as i32
                        == (*gSaveBlock1Ptr).seen2[index] as i32 & mask as i32
                {
                    retVal = 1;
                } else {
                    (*gSaveBlock2Ptr).pokedex.seen[index] &= !mask;
                    (*gSaveBlock1Ptr).seen1[index] &= !mask;
                    (*gSaveBlock1Ptr).seen2[index] &= !mask;
                    retVal = 0;
                }
            }
        }
        FLAG_GET_CAUGHT => {
            if (*gSaveBlock2Ptr).pokedex.owned[index] as i32 & mask as i32 != 0 {
                if (*gSaveBlock2Ptr).pokedex.owned[index] as i32 & mask as i32
                    == (*gSaveBlock2Ptr).pokedex.seen[index] as i32 & mask as i32
                    && (*gSaveBlock2Ptr).pokedex.owned[index] as i32 & mask as i32
                        == (*gSaveBlock1Ptr).seen1[index] as i32 & mask as i32
                    && (*gSaveBlock2Ptr).pokedex.owned[index] as i32 & mask as i32
                        == (*gSaveBlock1Ptr).seen2[index] as i32 & mask as i32
                {
                    retVal = 1;
                } else {
                    (*gSaveBlock2Ptr).pokedex.owned[index] &= !mask;
                    (*gSaveBlock2Ptr).pokedex.seen[index] &= !mask;
                    (*gSaveBlock1Ptr).seen1[index] &= !mask;
                    (*gSaveBlock1Ptr).seen2[index] &= !mask;
                    retVal = 0;
                }
            }
        }
        FLAG_SET_SEEN => {
            (*gSaveBlock2Ptr).pokedex.seen[index] |= mask;
            (*gSaveBlock1Ptr).seen1[index] |= mask;
            (*gSaveBlock1Ptr).seen2[index] |= mask;
        }
        FLAG_SET_CAUGHT => {
            (*gSaveBlock2Ptr).pokedex.owned[index] |= mask;
        }
        _ => {}
    }
    return retVal;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNationalPokedexCount(caseID: u8) -> u16 {
    let mut count: u16 = 0;
    let mut i: u16 = 0;
    i = 0;
    while i < NATIONAL_DEX_DEOXYS {
        match caseID {
            FLAG_GET_SEEN => {
                if GetSetPokedexFlag(i + 1, FLAG_GET_SEEN) != 0 {
                    count += 1;
                }
            }
            FLAG_GET_CAUGHT => {
                if GetSetPokedexFlag(i + 1, FLAG_GET_CAUGHT) != 0 {
                    count += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetHoennPokedexCount(caseID: u8) -> u16 {
    let mut count: u16 = 0;
    let mut i: u16 = 0;
    i = 0;
    while i < HOENN_DEX_DEOXYS {
        match caseID {
            FLAG_GET_SEEN => {
                if GetSetPokedexFlag(HoennToNationalOrder(i + 1), FLAG_GET_SEEN) != 0 {
                    count += 1;
                }
            }
            FLAG_GET_CAUGHT => {
                if GetSetPokedexFlag(HoennToNationalOrder(i + 1), FLAG_GET_CAUGHT) != 0 {
                    count += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetKantoPokedexCount(caseID: u8) -> u16 {
    let mut count: u16 = 0;
    let mut i: u16 = 0;
    i = 0;
    while i < NATIONAL_DEX_MEW {
        match caseID {
            FLAG_GET_SEEN => {
                if GetSetPokedexFlag(i + 1, FLAG_GET_SEEN) != 0 {
                    count += 1;
                }
            }
            FLAG_GET_CAUGHT => {
                if GetSetPokedexFlag(i + 1, FLAG_GET_CAUGHT) != 0 {
                    count += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasAllHoennMons() -> u16 {
    let mut i: u16 = 0;
    i = 0;
    while i < 200 {
        if GetSetPokedexFlag(HoennToNationalOrder(i + 1), FLAG_GET_CAUGHT) == 0 {
            return FALSE as u16;
        }
        i += 1;
    }
    return TRUE as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasAllKantoMons() -> u8 {
    let mut i: u16 = 0;
    i = 0;
    while i < 150 {
        if GetSetPokedexFlag(i + 1, FLAG_GET_CAUGHT) == 0 {
            return FALSE;
        }
        i += 1;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasAllMons() -> u16 {
    let mut i: u16 = 0;
    i = 0;
    while i < 150 {
        if GetSetPokedexFlag(i + 1, FLAG_GET_CAUGHT) == 0 {
            return FALSE as u16;
        }
        i += 1;
    }
    i = NATIONAL_DEX_MEW;
    while i < 248 {
        if GetSetPokedexFlag(i + 1, FLAG_GET_CAUGHT) == 0 {
            return FALSE as u16;
        }
        i += 1;
    }
    i = NATIONAL_DEX_CELEBI;
    while i < 384 {
        if GetSetPokedexFlag(i + 1, FLAG_GET_CAUGHT) == 0 {
            return FALSE as u16;
        }
        i += 1;
    }
    return TRUE as u16;
}
pub(crate) unsafe extern "C" fn ResetOtherVideoRegisters(regBits: u16) {
    if regBits as i32 & DISPCNT_BG0_ON as i32 == 0 {
        ClearGpuRegBits(0, DISPCNT_BG0_ON);
        SetGpuReg(REG_OFFSET_BG0CNT, 0);
        SetGpuReg(REG_OFFSET_BG0HOFS, 0);
        SetGpuReg(REG_OFFSET_BG0VOFS, 0);
    }
    if regBits as i32 & DISPCNT_BG1_ON as i32 == 0 {
        ClearGpuRegBits(0, DISPCNT_BG1_ON);
        SetGpuReg(REG_OFFSET_BG1CNT, 0);
        SetGpuReg(REG_OFFSET_BG1HOFS, 0);
        SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    }
    if regBits as i32 & DISPCNT_BG2_ON as i32 == 0 {
        ClearGpuRegBits(0, DISPCNT_BG2_ON);
        SetGpuReg(REG_OFFSET_BG2CNT, 0);
        SetGpuReg(REG_OFFSET_BG2HOFS, 0);
        SetGpuReg(REG_OFFSET_BG2VOFS, 0);
    }
    if regBits as i32 & DISPCNT_BG3_ON == 0 {
        ClearGpuRegBits(0, DISPCNT_BG3_ON as u16);
        SetGpuReg(REG_OFFSET_BG3CNT, 0);
        SetGpuReg(REG_OFFSET_BG3HOFS, 0);
        SetGpuReg(REG_OFFSET_BG3VOFS, 0);
    }
    if regBits as i32 & DISPCNT_OBJ_ON as i32 == 0 {
        ClearGpuRegBits(0, DISPCNT_OBJ_ON);
        ResetSpriteData();
        FreeAllSpritePalettes();
        gReservedSpritePaletteCount = 8;
    }
}
pub(crate) unsafe extern "C" fn PrintInfoSubMenuText(
    windowId: u8,
    str: *mut u8,
    left: u8,
    top: u8,
) {
    let mut color: CArray<u8, 3> = zeroed();
    color[0] = 0x0;
    color[1] = TEXT_DYNAMIC_COLOR_6;
    color[2] = TEXT_COLOR_LIGHT_GRAY;
    AddTextPrinterParameterized4(
        windowId,
        FONT_NORMAL,
        left,
        top,
        0,
        0,
        color.as_mut_ptr(),
        TEXT_SKIP_DRAW as i8,
        str,
    );
}
pub(crate) unsafe extern "C" fn UnusedPrintNum(windowId: u8, num: u16, left: u8, top: u8) {
    let mut str: CArray<u8, 4> = zeroed();
    str[0] = CHAR_0 + (num as i32 / 100) as u8;
    str[1] = CHAR_0 + (num as i32 % 100 / 10) as u8;
    str[2] = CHAR_0 + (num as i32 % 100 % 10) as u8;
    str[3] = EOS;
    PrintInfoSubMenuText(windowId, str.as_mut_ptr(), left, top);
}
pub(crate) unsafe extern "C" fn PrintCryScreenSpeciesName(
    windowId: u8,
    mut num: u16,
    left: u8,
    top: u8,
) -> u8 {
    let mut str: CArray<u8, 11> = zeroed();
    let mut i: u8 = 0;
    i = 0;
    while i < 11 {
        str[i] = EOS;
        i += 1;
    }
    num = NationalPokedexNumToSpecies(num);
    match num {
        0 => {
            i = 0;
            while i < 5 {
                str[i] = CHAR_HYPHEN;
                i += 1;
            }
        }
        _ => {
            i = 0;
            while gSpeciesNames[num][i] != EOS && i < POKEMON_NAME_LENGTH as u8 {
                str[i] = gSpeciesNames[num][i];
                i += 1;
            }
        }
    }
    PrintInfoSubMenuText(windowId, str.as_mut_ptr(), left, top);
    return i;
}
pub(crate) unsafe extern "C" fn UnusedPrintMonName(windowId: u8, name: *mut u8, left: u8, top: u8) {
    let mut str: CArray<u8, 11> = zeroed();
    let mut i: u8 = 0;
    let mut nameLength: u8 = 0;
    i = 0;
    while i < 11 {
        str[i] = CHAR_SPACE;
        i += 1;
    }
    nameLength = 0;
    while *name.at(nameLength) != 0x00 && nameLength < 11 {
        nameLength += 1;
    }
    i = 0;
    while i < nameLength {
        str[11 - nameLength as u32 + i as u32] = *name.at(i);
        i += 1;
    }
    str[10] = EOS;
    PrintInfoSubMenuText(windowId, str.as_mut_ptr(), left, top);
}
pub(crate) unsafe extern "C" fn PrintDecimalNum(windowId: u8, num: u16, left: u8, top: u8) {
    let mut str: CArray<u8, 6> = zeroed();
    let mut outputted: u8 = FALSE;
    let mut result: u8 = 0;
    result = (num as i32 / 1000) as u8;
    if result == 0 {
        str[0] = CHAR_SPACER;
        outputted = FALSE;
    } else {
        str[0] = CHAR_0 + result;
        outputted = TRUE;
    }
    result = (num as i32 % 1000 / 100) as u8;
    if result == 0 && outputted == 0 {
        str[1] = CHAR_SPACER;
        outputted = FALSE;
    } else {
        str[1] = CHAR_0 + result;
        outputted = TRUE;
    }
    str[2] = CHAR_0 + (num as i32 % 1000 % 100 / 10) as u8;
    str[3] = CHAR_DEC_SEPARATOR;
    str[4] = CHAR_0 + (num as i32 % 1000 % 100 % 10) as u8;
    str[5] = EOS;
    PrintInfoSubMenuText(windowId, str.as_mut_ptr(), left, top);
}
pub(crate) unsafe extern "C" fn DrawFootprint(windowId: u8, dexNum: u16) {
    let mut footprint4bpp: CArray<u8, 128> = zeroed();
    let mut footprintGfx: *mut u8 = gMonFootprintTable[NationalPokedexNumToSpecies(dexNum)];
    let mut tileIdx: u16 = 0;
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    i = 0;
    while i < 32 {
        let mut footprint1bpp: u8 = *footprintGfx.at(i);
        j = 0;
        while j < 4 {
            let mut tile: u8 = 0;
            if footprint1bpp as i32 & shl_i32(1, 2 * j as u32) != 0 {
                tile |= FOOTPRINT_COLOR_IDX;
            }
            if footprint1bpp as i32 & shl_i32(2, 2 * j as u32) != 0 {
                tile |= 32;
            }
            footprint4bpp[tileIdx] = tile;
            tileIdx += 1;
            j += 1;
        }
        i += 1;
    }
    CopyToWindowPixelBuffer(windowId, footprint4bpp.as_mut_ptr() as *mut c_void, 128, 0);
}
pub(crate) unsafe extern "C" fn RS_DrawFootprint(offset: u16, tileNum: u16) {
    *((VRAM + offset as i32 * 0x800 + 0x232) as usize as *mut u16) = 0xF000 + tileNum + 0;
    *((VRAM + offset as i32 * 0x800 + 0x234) as usize as *mut u16) = 0xF000 + tileNum + 1;
    *((VRAM + offset as i32 * 0x800 + 0x272) as usize as *mut u16) = 0xF000 + tileNum + 2;
    *((VRAM + offset as i32 * 0x800 + 0x274) as usize as *mut u16) = 0xF000 + tileNum + 3;
}
pub(crate) unsafe extern "C" fn GetNextPosition(
    direction: u8,
    mut position: u16,
    min: u16,
    max: u16,
) -> u16 {
    match direction {
        1 => {
            if position > min {
                position -= 1;
            }
        }
        0 => {
            if position < max {
                position += 1;
            }
        }
        3 => {
            if position > min {
                position -= 1;
            } else {
                position = max;
            }
        }
        2 => {
            if position < max {
                position += 1;
            } else {
                position = min;
            }
        }
        _ => {}
    }
    return position;
}
pub(crate) unsafe extern "C" fn GetPokedexMonPersonality(species: u16) -> u32 {
    if species == SPECIES_UNOWN || species == SPECIES_SPINDA {
        if species == SPECIES_UNOWN {
            return (*gSaveBlock2Ptr).pokedex.unownPersonality;
        } else {
            return (*gSaveBlock2Ptr).pokedex.spindaPersonality;
        }
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonSpriteFromNationalDexNumber(
    mut nationalNum: u16,
    x: i16,
    y: i16,
    paletteSlot: u16,
) -> u16 {
    nationalNum = NationalPokedexNumToSpecies(nationalNum);
    return CreateMonPicSprite_HandleDeoxys(
        nationalNum,
        SHINY_ODDS,
        GetPokedexMonPersonality(nationalNum),
        TRUE,
        x,
        y,
        paletteSlot as u8,
        TAG_NONE,
    );
}
pub(crate) unsafe extern "C" fn CreateSizeScreenTrainerPic(
    species: u16,
    x: i16,
    y: i16,
    paletteSlot: i8,
) -> u16 {
    return CreateTrainerPicSprite(species, TRUE, x, y, paletteSlot as u8, TAG_NONE);
}
pub(crate) unsafe extern "C" fn DoPokedexSearch(
    dexMode: u8,
    order: u8,
    abcGroup: u8,
    bodyColor: u8,
    mut type1: u8,
    mut type2: u8,
) -> i32 {
    let mut species: u16 = 0;
    let mut i: u16 = 0;
    let mut resultsCount: u16 = 0;
    let mut types: CArray<u8, 2> = zeroed();
    CreatePokedexList(dexMode, order);
    i = 0;
    resultsCount = 0;
    while i < NATIONAL_DEX_DEOXYS {
        if (*sPokedexView).pokedexList[i].seen() != 0 {
            (*sPokedexView).pokedexList[resultsCount] = (*sPokedexView).pokedexList[i];
            resultsCount += 1;
        }
        i += 1;
    }
    (*sPokedexView).pokemonListCount = resultsCount;
    if abcGroup != 0xFF {
        i = 0;
        resultsCount = 0;
        while i < (*sPokedexView).pokemonListCount {
            let mut firstLetter: u8 = 0;
            species = NationalPokedexNumToSpecies((*sPokedexView).pokedexList[i].dexNum);
            firstLetter = gSpeciesNames[species][0];
            if firstLetter >= sLetterSearchRanges[abcGroup][0]
                && (firstLetter as i32)
                    < sLetterSearchRanges[abcGroup][0] as i32
                        + sLetterSearchRanges[abcGroup][1] as i32
                || firstLetter >= sLetterSearchRanges[abcGroup][2]
                    && (firstLetter as i32)
                        < sLetterSearchRanges[abcGroup][2] as i32
                            + sLetterSearchRanges[abcGroup][3] as i32
            {
                (*sPokedexView).pokedexList[resultsCount] = (*sPokedexView).pokedexList[i];
                resultsCount += 1;
            }
            i += 1;
        }
        (*sPokedexView).pokemonListCount = resultsCount;
    }
    if bodyColor != 0xFF {
        i = 0;
        resultsCount = 0;
        while i < (*sPokedexView).pokemonListCount {
            species = NationalPokedexNumToSpecies((*sPokedexView).pokedexList[i].dexNum);
            if bodyColor == gSpeciesInfo[species].bodyColor() {
                (*sPokedexView).pokedexList[resultsCount] = (*sPokedexView).pokedexList[i];
                resultsCount += 1;
            }
            i += 1;
        }
        (*sPokedexView).pokemonListCount = resultsCount;
    }
    if type1 != TYPE_NONE || type2 != TYPE_NONE {
        if type1 == TYPE_NONE {
            type1 = type2;
            type2 = TYPE_NONE;
        }
        if type2 == TYPE_NONE {
            i = 0;
            resultsCount = 0;
            while i < (*sPokedexView).pokemonListCount {
                if (*sPokedexView).pokedexList[i].owned() != 0 {
                    species = NationalPokedexNumToSpecies((*sPokedexView).pokedexList[i].dexNum);
                    types[0] = gSpeciesInfo[species].types[0];
                    types[1] = gSpeciesInfo[species].types[1];
                    if types[0] == type1 || types[1] == type1 {
                        (*sPokedexView).pokedexList[resultsCount] = (*sPokedexView).pokedexList[i];
                        resultsCount += 1;
                    }
                }
                i += 1;
            }
        } else {
            i = 0;
            resultsCount = 0;
            while i < (*sPokedexView).pokemonListCount {
                if (*sPokedexView).pokedexList[i].owned() != 0 {
                    species = NationalPokedexNumToSpecies((*sPokedexView).pokedexList[i].dexNum);
                    types[0] = gSpeciesInfo[species].types[0];
                    types[1] = gSpeciesInfo[species].types[1];
                    if types[0] == type1 && types[1] == type2
                        || types[0] == type2 && types[1] == type1
                    {
                        (*sPokedexView).pokedexList[resultsCount] = (*sPokedexView).pokedexList[i];
                        resultsCount += 1;
                    }
                }
                i += 1;
            }
        }
        (*sPokedexView).pokemonListCount = resultsCount;
    }
    if (*sPokedexView).pokemonListCount != 0 {
        i = (*sPokedexView).pokemonListCount;
        while i < NATIONAL_DEX_DEOXYS {
            (*sPokedexView).pokedexList[i].dexNum = 0xFFFF;
            (*sPokedexView).pokedexList[i].set_seen(FALSE as u16);
            (*sPokedexView).pokedexList[i].set_owned(FALSE as u16);
            i += 1;
        }
    }
    return resultsCount as i32;
}
pub(crate) unsafe extern "C" fn LoadSearchMenu() -> u8 {
    return CreateTask(Some(Task_LoadSearchMenu), 0);
}
pub(crate) unsafe extern "C" fn PrintSearchText(str: *mut u8, x: u32, y: u32) {
    let mut color: CArray<u8, 3> = zeroed();
    color[0] = 0x0;
    color[1] = TEXT_DYNAMIC_COLOR_6;
    color[2] = 0x2;
    AddTextPrinterParameterized4(
        0,
        FONT_NORMAL,
        x as u8,
        y as u8,
        0,
        0,
        color.as_mut_ptr(),
        TEXT_SKIP_DRAW as i8,
        str,
    );
}
pub(crate) unsafe extern "C" fn ClearSearchMenuRect(x: u32, y: u32, width: u32, height: u32) {
    FillWindowPixelRect(0, 0, x as u16, y as u16, width as u16, height as u16);
}
pub(crate) unsafe extern "C" fn Task_LoadSearchMenu(taskId: u8) {
    let mut i: u16 = 0;
    match gMain.state {
        1 => {
            LoadCompressedSpriteSheet(sInterfaceSpriteSheet.as_ptr().cast_mut());
            LoadSpritePalettes(sInterfaceSpritePalette.as_ptr().cast_mut());
            CreateSearchParameterScrollArrows(taskId);
            i = 0;
            while i < NUM_TASK_DATA as u16 {
                gTasks[taskId].data[i] = 0;
                i += 1;
            }
            SetDefaultSearchModeAndOrder(taskId);
            HighlightSelectedSearchTopBarItem(SEARCH_TOPBAR_SEARCH);
            PrintSelectedSearchParameters(taskId);
            CopyWindowToVram(0, COPYWIN_FULL);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
            CopyBgTilemapBufferToVram(3);
            gMain.state += 1;
        }
        2 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            gMain.state += 1;
        }
        3 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            SetGpuReg(REG_OFFSET_DISPCNT, 4160);
            HideBg(0);
            ShowBg(1);
            ShowBg(2);
            ShowBg(3);
            gMain.state += 1;
        }
        4 => {
            if gPaletteFade.active() == 0 {
                gTasks[taskId].func = Some(Task_SwitchToSearchMenuTopBar);
                gMain.state = 0;
            }
        }
        _ => {
            if gPaletteFade.active() == 0 {
                (*sPokedexView).currentPage = PAGE_SEARCH;
                ResetOtherVideoRegisters(0);
                ResetBgsAndClearDma3BusyFlags(0);
                InitBgsFromTemplates(0, sSearchMenu_BgTemplate.as_ptr().cast_mut(), 4);
                SetBgTilemapBuffer(3, AllocZeroed(BG_SCREEN_SIZE));
                SetBgTilemapBuffer(2, AllocZeroed(BG_SCREEN_SIZE));
                SetBgTilemapBuffer(1, AllocZeroed(BG_SCREEN_SIZE));
                SetBgTilemapBuffer(0, AllocZeroed(BG_SCREEN_SIZE));
                InitWindows(sSearchMenu_WindowTemplate.as_ptr().cast_mut());
                DeactivateAllTextPrinters();
                PutWindowTilemap(0);
                DecompressAndLoadBgGfxUsingHeap(
                    3,
                    gPokedexSearchMenu_Gfx.as_ptr().cast_mut() as *mut c_void,
                    0x2000,
                    0,
                    0,
                );
                if IsNationalPokedexEnabled() == 0 {
                    CopyToBgTilemapBuffer(
                        3,
                        gPokedexSearchMenuHoenn_Tilemap.as_ptr().cast_mut() as *mut c_void,
                        0,
                        0,
                    );
                } else {
                    CopyToBgTilemapBuffer(
                        3,
                        gPokedexSearchMenuNational_Tilemap.as_ptr().cast_mut() as *mut c_void,
                        0,
                        0,
                    );
                }
                LoadPalette(
                    gPokedexSearchMenu_Pal.as_ptr().cast_mut().at(1) as *mut c_void,
                    1,
                    126,
                );
                gMain.state = 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FreeSearchWindowAndBgBuffers() {
    let mut tilemapBuffer: *mut c_void = null_mut();
    FreeAllWindowBuffers();
    tilemapBuffer = GetBgTilemapBuffer(0);
    if !tilemapBuffer.is_null() {
        Free(tilemapBuffer);
    }
    tilemapBuffer = GetBgTilemapBuffer(1);
    if !tilemapBuffer.is_null() {
        Free(tilemapBuffer);
    }
    tilemapBuffer = GetBgTilemapBuffer(2);
    if !tilemapBuffer.is_null() {
        Free(tilemapBuffer);
    }
    tilemapBuffer = GetBgTilemapBuffer(3);
    if !tilemapBuffer.is_null() {
        Free(tilemapBuffer);
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchToSearchMenuTopBar(taskId: u8) {
    HighlightSelectedSearchTopBarItem(gTasks[taskId].data[0] as u8);
    PrintSelectedSearchParameters(taskId);
    CopyWindowToVram(0, COPYWIN_GFX);
    CopyBgTilemapBufferToVram(3);
    gTasks[taskId].func = Some(Task_HandleSearchTopBarInput);
}
pub(crate) unsafe extern "C" fn Task_HandleSearchTopBarInput(taskId: u8) {
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_PC_OFF);
        gTasks[taskId].func = Some(Task_ExitSearch);
        return;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        match gTasks[taskId].data[0] {
            0 => {
                PlaySE(SE_PIN);
                gTasks[taskId].data[1] = SEARCH_NAME as i16;
                gTasks[taskId].func = Some(Task_SwitchToSearchMenu);
            }
            1 => {
                PlaySE(SE_PIN);
                gTasks[taskId].data[1] = SEARCH_ORDER as i16;
                gTasks[taskId].func = Some(Task_SwitchToSearchMenu);
            }
            2 => {
                PlaySE(SE_PC_OFF);
                gTasks[taskId].func = Some(Task_ExitSearch);
            }
            _ => {}
        }
        return;
    }
    if gMain.newKeys as i32 & DPAD_LEFT != 0 && gTasks[taskId].data[0] > SEARCH_TOPBAR_SEARCH as i16
    {
        PlaySE(SE_DEX_PAGE);
        gTasks[taskId].data[0] -= 1;
        HighlightSelectedSearchTopBarItem(gTasks[taskId].data[0] as u8);
        CopyWindowToVram(0, COPYWIN_GFX);
        CopyBgTilemapBufferToVram(3);
    }
    if gMain.newKeys as i32 & DPAD_RIGHT != 0
        && gTasks[taskId].data[0] < SEARCH_TOPBAR_CANCEL as i16
    {
        PlaySE(SE_DEX_PAGE);
        gTasks[taskId].data[0] += 1;
        HighlightSelectedSearchTopBarItem(gTasks[taskId].data[0] as u8);
        CopyWindowToVram(0, COPYWIN_GFX);
        CopyBgTilemapBufferToVram(3);
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchToSearchMenu(taskId: u8) {
    HighlightSelectedSearchMenuItem(gTasks[taskId].data[0] as u8, gTasks[taskId].data[1] as u8);
    PrintSelectedSearchParameters(taskId);
    CopyWindowToVram(0, COPYWIN_GFX);
    CopyBgTilemapBufferToVram(3);
    gTasks[taskId].func = Some(Task_HandleSearchMenuInput);
}
pub(crate) unsafe extern "C" fn Task_HandleSearchMenuInput(taskId: u8) {
    let mut movementMap: *mut CArray<u8, 4> = null_mut();
    if gTasks[taskId].data[0] != SEARCH_TOPBAR_SEARCH as i16 {
        if IsNationalPokedexEnabled() == 0 {
            movementMap = sSearchMovementMap_ShiftHoennDex.as_ptr().cast_mut();
        } else {
            movementMap = sSearchMovementMap_ShiftNatDex.as_ptr().cast_mut();
        }
    } else {
        if IsNationalPokedexEnabled() == 0 {
            movementMap = sSearchMovementMap_SearchHoennDex.as_ptr().cast_mut();
        } else {
            movementMap = sSearchMovementMap_SearchNatDex.as_ptr().cast_mut();
        }
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_BALL);
        SetDefaultSearchModeAndOrder(taskId);
        gTasks[taskId].func = Some(Task_SwitchToSearchMenuTopBar);
        return;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        if gTasks[taskId].data[1] == SEARCH_OK {
            if gTasks[taskId].data[0] != SEARCH_TOPBAR_SEARCH as i16 {
                sPokeBallRotation = POKEBALL_ROTATION_TOP;
                (*sPokedexView).pokeBallRotationBackup = POKEBALL_ROTATION_TOP as u16;
                sLastSelectedPokemon = 0;
                (*sPokedexView).selectedPokemonBackup = 0;
                (*gSaveBlock2Ptr).pokedex.mode = GetSearchModeSelection(taskId, SEARCH_MODE);
                if IsNationalPokedexEnabled() == 0 {
                    (*gSaveBlock2Ptr).pokedex.mode = DEX_MODE_HOENN as u8;
                }
                (*sPokedexView).dexModeBackup = (*gSaveBlock2Ptr).pokedex.mode as u16;
                (*gSaveBlock2Ptr).pokedex.order = GetSearchModeSelection(taskId, SEARCH_ORDER);
                (*sPokedexView).dexOrderBackup = (*gSaveBlock2Ptr).pokedex.order as u16;
                PlaySE(SE_PC_OFF);
                gTasks[taskId].func = Some(Task_ExitSearch);
            } else {
                EraseAndPrintSearchTextBox(gText_SearchingPleaseWait.as_ptr().cast_mut());
                gTasks[taskId].func = Some(Task_StartPokedexSearch);
                PlaySE(SE_DEX_SEARCH);
                CopyWindowToVram(0, COPYWIN_GFX);
            }
        } else {
            PlaySE(SE_PIN);
            gTasks[taskId].func = Some(Task_SelectSearchMenuItem);
        }
        return;
    }
    if gMain.newKeys as i32 & DPAD_LEFT != 0 && (*movementMap.at(gTasks[taskId].data[1]))[0] != 0xFF
    {
        PlaySE(SE_SELECT);
        gTasks[taskId].data[1] = (*movementMap.at(gTasks[taskId].data[1]))[0] as i16;
        HighlightSelectedSearchMenuItem(gTasks[taskId].data[0] as u8, gTasks[taskId].data[1] as u8);
        CopyWindowToVram(0, COPYWIN_GFX);
        CopyBgTilemapBufferToVram(3);
    }
    if gMain.newKeys as i32 & DPAD_RIGHT != 0
        && (*movementMap.at(gTasks[taskId].data[1]))[1] != 0xFF
    {
        PlaySE(SE_SELECT);
        gTasks[taskId].data[1] = (*movementMap.at(gTasks[taskId].data[1]))[1] as i16;
        HighlightSelectedSearchMenuItem(gTasks[taskId].data[0] as u8, gTasks[taskId].data[1] as u8);
        CopyWindowToVram(0, COPYWIN_GFX);
        CopyBgTilemapBufferToVram(3);
    }
    if gMain.newKeys as i32 & DPAD_UP != 0 && (*movementMap.at(gTasks[taskId].data[1]))[2] != 0xFF {
        PlaySE(SE_SELECT);
        gTasks[taskId].data[1] = (*movementMap.at(gTasks[taskId].data[1]))[2] as i16;
        HighlightSelectedSearchMenuItem(gTasks[taskId].data[0] as u8, gTasks[taskId].data[1] as u8);
        CopyWindowToVram(0, COPYWIN_GFX);
        CopyBgTilemapBufferToVram(3);
    }
    if gMain.newKeys as i32 & DPAD_DOWN != 0 && (*movementMap.at(gTasks[taskId].data[1]))[3] != 0xFF
    {
        PlaySE(SE_SELECT);
        gTasks[taskId].data[1] = (*movementMap.at(gTasks[taskId].data[1]))[3] as i16;
        HighlightSelectedSearchMenuItem(gTasks[taskId].data[0] as u8, gTasks[taskId].data[1] as u8);
        CopyWindowToVram(0, COPYWIN_GFX);
        CopyBgTilemapBufferToVram(3);
    }
}
pub(crate) unsafe extern "C" fn Task_StartPokedexSearch(taskId: u8) {
    let mut dexMode: u8 = GetSearchModeSelection(taskId, SEARCH_MODE);
    let mut order: u8 = GetSearchModeSelection(taskId, SEARCH_ORDER);
    let mut abcGroup: u8 = GetSearchModeSelection(taskId, SEARCH_NAME);
    let mut bodyColor: u8 = GetSearchModeSelection(taskId, SEARCH_COLOR);
    let mut type1: u8 = GetSearchModeSelection(taskId, SEARCH_TYPE_LEFT);
    let mut type2: u8 = GetSearchModeSelection(taskId, SEARCH_TYPE_RIGHT);
    DoPokedexSearch(dexMode, order, abcGroup, bodyColor, type1, type2);
    gTasks[taskId].func = Some(Task_WaitAndCompleteSearch);
}
pub(crate) unsafe extern "C" fn Task_WaitAndCompleteSearch(taskId: u8) {
    if IsSEPlaying() == 0 {
        if (*sPokedexView).pokemonListCount != 0 {
            PlaySE(SE_SUCCESS);
            EraseAndPrintSearchTextBox(gText_SearchCompleted.as_ptr().cast_mut());
        } else {
            PlaySE(SE_FAILURE);
            EraseAndPrintSearchTextBox(gText_NoMatchingPkmnWereFound.as_ptr().cast_mut());
        }
        gTasks[taskId].func = Some(Task_SearchCompleteWaitForInput);
        CopyWindowToVram(0, COPYWIN_GFX);
    }
}
pub(crate) unsafe extern "C" fn Task_SearchCompleteWaitForInput(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        if (*sPokedexView).pokemonListCount != 0 {
            (*sPokedexView).screenSwitchState = 1;
            (*sPokedexView).dexMode = GetSearchModeSelection(taskId, SEARCH_MODE) as u16;
            (*sPokedexView).dexOrder = GetSearchModeSelection(taskId, SEARCH_ORDER) as u16;
            gTasks[taskId].func = Some(Task_ExitSearch);
            PlaySE(SE_PC_OFF);
        } else {
            gTasks[taskId].func = Some(Task_SwitchToSearchMenu);
            PlaySE(SE_BALL);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SelectSearchMenuItem(taskId: u8) {
    let mut menuItem: u8 = 0;
    let mut cursorPos: *mut u16 = null_mut();
    let mut scrollOffset: *mut u16 = null_mut();
    DrawOrEraseSearchParameterBox(FALSE);
    menuItem = gTasks[taskId].data[1] as u8;
    cursorPos =
        &raw mut gTasks[taskId].data[sSearchOptions[menuItem].taskDataCursorPos] as *mut u16;
    scrollOffset =
        &raw mut gTasks[taskId].data[sSearchOptions[menuItem].taskDataScrollOffset] as *mut u16;
    gTasks[taskId].data[14] = *cursorPos as i16;
    gTasks[taskId].data[15] = *scrollOffset as i16;
    PrintSearchParameterText(taskId);
    PrintSelectorArrow(*cursorPos as u32);
    gTasks[taskId].func = Some(Task_HandleSearchParameterInput);
    CopyWindowToVram(0, COPYWIN_GFX);
    CopyBgTilemapBufferToVram(3);
}
pub(crate) unsafe extern "C" fn Task_HandleSearchParameterInput(taskId: u8) {
    let mut menuItem: u8 = 0;
    let mut texts: *mut SearchOptionText = null_mut();
    let mut cursorPos: *mut u16 = null_mut();
    let mut scrollOffset: *mut u16 = null_mut();
    let mut maxOption: u16 = 0;
    let mut moved: u8 = 0;
    menuItem = gTasks[taskId].data[1] as u8;
    texts = sSearchOptions[menuItem].texts;
    cursorPos =
        &raw mut gTasks[taskId].data[sSearchOptions[menuItem].taskDataCursorPos] as *mut u16;
    scrollOffset =
        &raw mut gTasks[taskId].data[sSearchOptions[menuItem].taskDataScrollOffset] as *mut u16;
    maxOption = sSearchOptions[menuItem].numOptions - 1;
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_PIN);
        ClearSearchParameterBoxText();
        DrawOrEraseSearchParameterBox(TRUE);
        gTasks[taskId].func = Some(Task_SwitchToSearchMenu);
        CopyWindowToVram(0, COPYWIN_GFX);
        CopyBgTilemapBufferToVram(3);
        return;
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_BALL);
        ClearSearchParameterBoxText();
        DrawOrEraseSearchParameterBox(TRUE);
        *cursorPos = gTasks[taskId].data[14] as u16;
        *scrollOffset = gTasks[taskId].data[15] as u16;
        gTasks[taskId].func = Some(Task_SwitchToSearchMenu);
        CopyWindowToVram(0, COPYWIN_GFX);
        CopyBgTilemapBufferToVram(3);
        return;
    }
    moved = FALSE;
    if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
        if *cursorPos != 0 {
            EraseSelectorArrow(*cursorPos as u32);
            *cursorPos -= 1;
            PrintSelectorArrow(*cursorPos as u32);
            moved = TRUE;
        } else if *scrollOffset != 0 {
            *scrollOffset -= 1;
            PrintSearchParameterText(taskId);
            PrintSelectorArrow(*cursorPos as u32);
            moved = TRUE;
        }
        if moved != 0 {
            PlaySE(SE_SELECT);
            EraseAndPrintSearchTextBox(
                (*texts.at(*cursorPos as i32 + *scrollOffset as i32)).description,
            );
            CopyWindowToVram(0, COPYWIN_GFX);
        }
        return;
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
        if *cursorPos < MAX_SEARCH_PARAM_CURSOR_POS && *cursorPos < maxOption {
            EraseSelectorArrow(*cursorPos as u32);
            *cursorPos += 1;
            PrintSelectorArrow(*cursorPos as u32);
            moved = TRUE;
        } else if maxOption > MAX_SEARCH_PARAM_CURSOR_POS
            && (*scrollOffset as i32) < maxOption as i32 - MAX_SEARCH_PARAM_CURSOR_POS as i32
        {
            *scrollOffset += 1;
            PrintSearchParameterText(taskId);
            PrintSelectorArrow(5);
            moved = TRUE;
        }
        if moved != 0 {
            PlaySE(SE_SELECT);
            EraseAndPrintSearchTextBox(
                (*texts.at(*cursorPos as i32 + *scrollOffset as i32)).description,
            );
            CopyWindowToVram(0, COPYWIN_GFX);
        }
        return;
    }
}
pub(crate) unsafe extern "C" fn Task_ExitSearch(taskId: u8) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    gTasks[taskId].func = Some(Task_ExitSearchWaitForFade);
}
pub(crate) unsafe extern "C" fn Task_ExitSearchWaitForFade(taskId: u8) {
    if gPaletteFade.active() == 0 {
        FreeSearchWindowAndBgBuffers();
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSearchRectHighlight(flags: u8, x: u8, y: u8, width: u8) {
    let mut i: u16 = 0;
    let mut temp: u16 = 0;
    let mut ptr: u32 = GetBgTilemapBuffer(3) as usize as u32;
    i = 0;
    while i < width as u16 {
        temp = *((ptr + (y as u32 + 0) * 64 + (x as u32 + i as u32) * 2) as usize as *mut u16);
        temp &= 0x0fff;
        temp |= (flags as u16) << 12;
        *((ptr + (y as u32 + 0) * 64 + (x as u32 + i as u32) * 2) as usize as *mut u16) = temp;
        temp = *((ptr + (y as u32 + 1) * 64 + (x as u32 + i as u32) * 2) as usize as *mut u16);
        temp &= 0x0fff;
        temp |= (flags as u16) << 12;
        *((ptr + (y as u32 + 1) * 64 + (x as u32 + i as u32) * 2) as usize as *mut u16) = temp;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DrawSearchMenuItemBgHighlight(
    searchBg: u8,
    unselected: u8,
    disabled: u8,
) {
    let mut highlightFlags: u8 = unselected & 1 | (disabled & 1) << 1;
    'l1: {
        let sw1: u8 = searchBg;
        let mut fall = false;
        if sw1 == SEARCH_TOPBAR_SEARCH || sw1 == SEARCH_TOPBAR_SHIFT || sw1 == SEARCH_TOPBAR_CANCEL
        {
            fall = true;
            SetSearchRectHighlight(
                highlightFlags,
                sSearchMenuTopBarItems[searchBg].highlightX,
                sSearchMenuTopBarItems[searchBg].highlightY,
                sSearchMenuTopBarItems[searchBg].highlightWidth,
            );
            break 'l1;
        }
        if sw1 == 3 || sw1 == 4 || sw1 == 7 || sw1 == 8 {
            fall = true;
            SetSearchRectHighlight(
                highlightFlags,
                sSearchMenuItems[searchBg as i32 - SEARCH_TOPBAR_COUNT].titleBgX,
                sSearchMenuItems[searchBg as i32 - SEARCH_TOPBAR_COUNT].titleBgY,
                sSearchMenuItems[searchBg as i32 - SEARCH_TOPBAR_COUNT].titleBgWidth,
            );
        }
        if fall || sw1 == 5 || sw1 == 6 {
            fall = true;
            SetSearchRectHighlight(
                highlightFlags,
                sSearchMenuItems[searchBg as i32 - SEARCH_TOPBAR_COUNT].selectionBgX,
                sSearchMenuItems[searchBg as i32 - SEARCH_TOPBAR_COUNT].selectionBgY,
                sSearchMenuItems[searchBg as i32 - SEARCH_TOPBAR_COUNT].selectionBgWidth,
            );
            break 'l1;
        }
        if sw1 == 10 {
            fall = true;
            SetSearchRectHighlight(
                highlightFlags,
                sSearchMenuItems[2].titleBgX,
                sSearchMenuItems[2].titleBgY,
                sSearchMenuItems[2].titleBgWidth,
            );
            break 'l1;
        }
        if sw1 == 9 {
            fall = true;
            if IsNationalPokedexEnabled() == 0 {
                SetSearchRectHighlight(
                    highlightFlags,
                    sSearchMenuItems[searchBg as i32 - SEARCH_TOPBAR_COUNT].titleBgX,
                    sSearchMenuItems[searchBg as i32 - SEARCH_TOPBAR_COUNT].titleBgY - 2,
                    sSearchMenuItems[searchBg as i32 - SEARCH_TOPBAR_COUNT].titleBgWidth,
                );
            } else {
                SetSearchRectHighlight(
                    highlightFlags,
                    sSearchMenuItems[searchBg as i32 - SEARCH_TOPBAR_COUNT].titleBgX,
                    sSearchMenuItems[searchBg as i32 - SEARCH_TOPBAR_COUNT].titleBgY,
                    sSearchMenuItems[searchBg as i32 - SEARCH_TOPBAR_COUNT].titleBgWidth,
                );
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn SetInitialSearchMenuBgHighlights(topBarItem: u8) {
    match topBarItem {
        SEARCH_TOPBAR_SEARCH => {
            DrawSearchMenuItemBgHighlight(SEARCH_TOPBAR_SEARCH, FALSE, FALSE);
            DrawSearchMenuItemBgHighlight(SEARCH_TOPBAR_SHIFT, TRUE, FALSE);
            DrawSearchMenuItemBgHighlight(SEARCH_TOPBAR_CANCEL, TRUE, FALSE);
            DrawSearchMenuItemBgHighlight(3, TRUE, FALSE);
            DrawSearchMenuItemBgHighlight(4, TRUE, FALSE);
            DrawSearchMenuItemBgHighlight(10, TRUE, FALSE);
            DrawSearchMenuItemBgHighlight(5, TRUE, FALSE);
            DrawSearchMenuItemBgHighlight(6, TRUE, FALSE);
            DrawSearchMenuItemBgHighlight(7, TRUE, FALSE);
            DrawSearchMenuItemBgHighlight(8, TRUE, FALSE);
            DrawSearchMenuItemBgHighlight(9, TRUE, FALSE);
        }
        SEARCH_TOPBAR_SHIFT => {
            DrawSearchMenuItemBgHighlight(SEARCH_TOPBAR_SEARCH, TRUE, FALSE);
            DrawSearchMenuItemBgHighlight(SEARCH_TOPBAR_SHIFT, FALSE, FALSE);
            DrawSearchMenuItemBgHighlight(SEARCH_TOPBAR_CANCEL, TRUE, FALSE);
            DrawSearchMenuItemBgHighlight(3, TRUE, TRUE);
            DrawSearchMenuItemBgHighlight(4, TRUE, TRUE);
            DrawSearchMenuItemBgHighlight(10, TRUE, TRUE);
            DrawSearchMenuItemBgHighlight(5, TRUE, TRUE);
            DrawSearchMenuItemBgHighlight(6, TRUE, TRUE);
            DrawSearchMenuItemBgHighlight(7, TRUE, FALSE);
            DrawSearchMenuItemBgHighlight(8, TRUE, FALSE);
            DrawSearchMenuItemBgHighlight(9, TRUE, FALSE);
        }
        SEARCH_TOPBAR_CANCEL => {
            DrawSearchMenuItemBgHighlight(SEARCH_TOPBAR_SEARCH, TRUE, FALSE);
            DrawSearchMenuItemBgHighlight(SEARCH_TOPBAR_SHIFT, TRUE, FALSE);
            DrawSearchMenuItemBgHighlight(SEARCH_TOPBAR_CANCEL, FALSE, FALSE);
            DrawSearchMenuItemBgHighlight(3, TRUE, TRUE);
            DrawSearchMenuItemBgHighlight(4, TRUE, TRUE);
            DrawSearchMenuItemBgHighlight(10, TRUE, TRUE);
            DrawSearchMenuItemBgHighlight(5, TRUE, TRUE);
            DrawSearchMenuItemBgHighlight(6, TRUE, TRUE);
            DrawSearchMenuItemBgHighlight(7, TRUE, TRUE);
            DrawSearchMenuItemBgHighlight(8, TRUE, TRUE);
            DrawSearchMenuItemBgHighlight(9, TRUE, TRUE);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn HighlightSelectedSearchTopBarItem(topBarItem: u8) {
    SetInitialSearchMenuBgHighlights(topBarItem);
    EraseAndPrintSearchTextBox(sSearchMenuTopBarItems[topBarItem].description);
}
pub(crate) unsafe extern "C" fn HighlightSelectedSearchMenuItem(topBarItem: u8, menuItem: u8) {
    SetInitialSearchMenuBgHighlights(topBarItem);
    match menuItem {
        SEARCH_NAME => {
            DrawSearchMenuItemBgHighlight(3, FALSE, FALSE);
        }
        SEARCH_COLOR => {
            DrawSearchMenuItemBgHighlight(4, FALSE, FALSE);
        }
        SEARCH_TYPE_LEFT => {
            DrawSearchMenuItemBgHighlight(10, FALSE, FALSE);
            DrawSearchMenuItemBgHighlight(5, FALSE, FALSE);
        }
        SEARCH_TYPE_RIGHT => {
            DrawSearchMenuItemBgHighlight(10, FALSE, FALSE);
            DrawSearchMenuItemBgHighlight(6, FALSE, FALSE);
        }
        SEARCH_ORDER => {
            DrawSearchMenuItemBgHighlight(7, FALSE, FALSE);
        }
        SEARCH_MODE => {
            DrawSearchMenuItemBgHighlight(8, FALSE, FALSE);
        }
        6 => {
            DrawSearchMenuItemBgHighlight(9, FALSE, FALSE);
        }
        _ => {}
    }
    EraseAndPrintSearchTextBox(sSearchMenuItems[menuItem].description);
}
pub(crate) unsafe extern "C" fn PrintSelectedSearchParameters(taskId: u8) {
    let mut searchParamId: u16 = 0;
    ClearSearchMenuRect(40, 16, 96, 80);
    searchParamId = gTasks[taskId].data[6] as u16 + gTasks[taskId].data[7] as u16;
    PrintSearchText(sDexSearchNameOptions[searchParamId].title, 0x2D, 0x11);
    searchParamId = gTasks[taskId].data[8] as u16 + gTasks[taskId].data[9] as u16;
    PrintSearchText(sDexSearchColorOptions[searchParamId].title, 0x2D, 0x21);
    searchParamId = gTasks[taskId].data[10] as u16 + gTasks[taskId].data[11] as u16;
    PrintSearchText(sDexSearchTypeOptions[searchParamId].title, 0x2D, 0x31);
    searchParamId = gTasks[taskId].data[12] as u16 + gTasks[taskId].data[13] as u16;
    PrintSearchText(sDexSearchTypeOptions[searchParamId].title, 0x5D, 0x31);
    searchParamId = gTasks[taskId].data[4] as u16 + gTasks[taskId].data[5] as u16;
    PrintSearchText(sDexOrderOptions[searchParamId].title, 0x2D, 0x41);
    if IsNationalPokedexEnabled() != 0 {
        searchParamId = gTasks[taskId].data[2] as u16 + gTasks[taskId].data[3] as u16;
        PrintSearchText(sDexModeOptions[searchParamId].title, 0x2D, 0x51);
    }
}
pub(crate) unsafe extern "C" fn DrawOrEraseSearchParameterBox(erase: u8) {
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    let mut ptr: *mut u16 = GetBgTilemapBuffer(3) as *mut u16;
    if erase == 0 {
        *ptr.at(17) = 0xC0B;
        i = 0x12;
        while i < 0x1F {
            *ptr.at(i) = 0x80D;
            i += 1;
        }
        j = 1;
        while j < 13 {
            *ptr.at(17).at(j as i32 * 32) = 0x40A;
            i = 0x12;
            while i < 0x1F {
                *ptr.at(j as i32 * 32).at(i) = 2;
                i += 1;
            }
            j += 1;
        }
        *ptr.at(433) = 0x40B;
        i = 0x12;
        while i < 0x1F {
            *ptr.at(416).at(i) = 0xD;
            i += 1;
        }
    } else {
        j = 0;
        while j < 14 {
            i = 0x11;
            while i < 0x1E {
                *ptr.at(j as i32 * 32).at(i) = 0x4F;
                i += 1;
            }
            j += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn PrintSearchParameterText(taskId: u8) {
    let mut texts: *mut SearchOptionText = sSearchOptions[gTasks[taskId].data[1]].texts;
    let mut cursorPos: *mut u16 = &raw mut gTasks[taskId].data
        [sSearchOptions[gTasks[taskId].data[1]].taskDataCursorPos]
        as *mut u16;
    let mut scrollOffset: *mut u16 = &raw mut gTasks[taskId].data
        [sSearchOptions[gTasks[taskId].data[1]].taskDataScrollOffset]
        as *mut u16;
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    ClearSearchParameterBoxText();
    i = 0;
    j = *scrollOffset;
    while i < MAX_SEARCH_PARAM_ON_SCREEN && !(*texts.at(j)).title.is_null() {
        PrintSearchParameterTitle(i as u32, (*texts.at(j)).title);
        i += 1;
        j += 1;
    }
    EraseAndPrintSearchTextBox((*texts.at(*cursorPos as i32 + *scrollOffset as i32)).description);
}
pub(crate) unsafe extern "C" fn GetSearchModeSelection(taskId: u8, option: u8) -> u8 {
    let mut cursorPos: *mut u16 =
        &raw mut gTasks[taskId].data[sSearchOptions[option].taskDataCursorPos] as *mut u16;
    let mut scrollOffset: *mut u16 =
        &raw mut gTasks[taskId].data[sSearchOptions[option].taskDataScrollOffset] as *mut u16;
    let mut id: u16 = *cursorPos + *scrollOffset;
    'l1: {
        let sw1: u8 = option;
        let matched = sw1 == SEARCH_MODE
            || sw1 == SEARCH_ORDER
            || sw1 == SEARCH_NAME
            || sw1 == SEARCH_COLOR
            || sw1 == SEARCH_TYPE_LEFT
            || sw1 == SEARCH_TYPE_RIGHT;
        let mut fall = false;
        if !matched {
            fall = true;
            return 0;
        }
        if sw1 == SEARCH_MODE {
            fall = true;
            return sPokedexModes[id];
        }
        if sw1 == SEARCH_ORDER {
            fall = true;
            return sOrderOptions[id];
        }
        if sw1 == SEARCH_NAME {
            fall = true;
            if id == 0 {
                return 0xFF;
            } else {
                return id as u8;
            }
        }
        if fall || sw1 == SEARCH_COLOR {
            fall = true;
            if id == 0 {
                return 0xFF;
            } else {
                return id as u8 - 1;
            }
        }
        if fall || sw1 == SEARCH_TYPE_LEFT || sw1 == SEARCH_TYPE_RIGHT {
            fall = true;
            return sDexSearchTypeIds[id];
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SetDefaultSearchModeAndOrder(taskId: u8) {
    let mut selected: u16 = 0;
    match (*sPokedexView).dexModeBackup {
        DEX_MODE_NATIONAL => {
            selected = DEX_MODE_NATIONAL;
        }
        _ => {
            selected = DEX_MODE_HOENN;
        }
    }
    gTasks[taskId].data[2] = selected as i16;
    match (*sPokedexView).dexOrderBackup {
        ORDER_ALPHABETICAL => {
            selected = ORDER_ALPHABETICAL;
        }
        ORDER_HEAVIEST => {
            selected = ORDER_HEAVIEST;
        }
        ORDER_LIGHTEST => {
            selected = ORDER_LIGHTEST;
        }
        ORDER_TALLEST => {
            selected = ORDER_TALLEST;
        }
        ORDER_SMALLEST => {
            selected = ORDER_SMALLEST;
        }
        _ => {
            selected = ORDER_NUMERICAL;
        }
    }
    gTasks[taskId].data[4] = selected as i16;
}
pub(crate) unsafe extern "C" fn SearchParamCantScrollUp(taskId: u8) -> u8 {
    let mut menuItem: u8 = gTasks[taskId].data[1] as u8;
    let mut scrollOffset: *mut u16 =
        &raw mut gTasks[taskId].data[sSearchOptions[menuItem].taskDataScrollOffset] as *mut u16;
    let mut lastOption: u16 = sSearchOptions[menuItem].numOptions - 1;
    if lastOption > MAX_SEARCH_PARAM_CURSOR_POS && *scrollOffset != 0 {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SearchParamCantScrollDown(taskId: u8) -> u8 {
    let mut menuItem: u8 = gTasks[taskId].data[1] as u8;
    let mut scrollOffset: *mut u16 =
        &raw mut gTasks[taskId].data[sSearchOptions[menuItem].taskDataScrollOffset] as *mut u16;
    let mut lastOption: u16 = sSearchOptions[menuItem].numOptions - 1;
    if lastOption > MAX_SEARCH_PARAM_CURSOR_POS
        && (*scrollOffset as i32) < lastOption as i32 - MAX_SEARCH_PARAM_CURSOR_POS as i32
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
pub(crate) unsafe extern "C" fn SpriteCB_SearchParameterScrollArrow(sprite: *mut Sprite) {
    if gTasks[(*sprite).data[0]].func
        == Some(Task_HandleSearchParameterInput as unsafe extern "C" fn(u8))
    {
        let mut val: u8 = 0;
        if (*sprite).data[1] != 0 {
            if SearchParamCantScrollDown((*sprite).data[0] as u8) != 0 {
                (*sprite).set_invisible(TRUE as u16);
            } else {
                (*sprite).set_invisible(FALSE as u16);
            }
        } else {
            if SearchParamCantScrollUp((*sprite).data[0] as u8) != 0 {
                (*sprite).set_invisible(TRUE as u16);
            } else {
                (*sprite).set_invisible(FALSE as u16);
            }
        }
        val = (*sprite).data[2] as u8 + (*sprite).data[1] as u8 * 128;
        (*sprite).y2 = gSineTable[val] / 128;
        (*sprite).data[2] += 8;
    } else {
        (*sprite).set_invisible(TRUE as u16);
    }
}
pub(crate) unsafe extern "C" fn CreateSearchParameterScrollArrows(taskId: u8) {
    let mut spriteId: u8 = 0;
    spriteId = CreateSprite(
        (&raw const *sScrollArrowSpriteTemplate).cast_mut(),
        184,
        4,
        0,
    );
    gSprites[spriteId].data[0] = taskId as i16;
    gSprites[spriteId].data[1] = FALSE as i16;
    gSprites[spriteId].callback = Some(SpriteCB_SearchParameterScrollArrow);
    spriteId = CreateSprite(
        (&raw const *sScrollArrowSpriteTemplate).cast_mut(),
        184,
        108,
        0,
    );
    gSprites[spriteId].data[0] = taskId as i16;
    gSprites[spriteId].data[1] = TRUE as i16;
    gSprites[spriteId].set_vFlip(TRUE as u16);
    gSprites[spriteId].callback = Some(SpriteCB_SearchParameterScrollArrow);
}
pub(crate) unsafe extern "C" fn EraseAndPrintSearchTextBox(str: *mut u8) {
    ClearSearchMenuRect(8, 120, 224, 32);
    PrintSearchText(str, 8, 121);
}
pub(crate) unsafe extern "C" fn EraseSelectorArrow(y: u32) {
    ClearSearchMenuRect(144, y * 16 + 8, 8, 16);
}
pub(crate) unsafe extern "C" fn PrintSelectorArrow(y: u32) {
    PrintSearchText(gText_SelectorArrow.as_ptr().cast_mut(), 144, y * 16 + 9);
}
pub(crate) unsafe extern "C" fn PrintSearchParameterTitle(y: u32, str: *mut u8) {
    PrintSearchText(str, 152, y * 16 + 9);
}
pub(crate) unsafe extern "C" fn ClearSearchParameterBoxText() {
    ClearSearchMenuRect(144, 8, 96, 96);
}
