//! Translated from `src/credits.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sCredits_Pal sCreditsCopyrightEnd_Gfx sTheEnd_LetterMap_T sTheEnd_LetterMap_H sTheEnd_LetterMap_E sTheEnd_LetterMap_N sTheEnd_LetterMap_D sCreditsText_EmptyString sCreditsText_PkmnEmeraldVersion sCreditsText_Credits sCreditsText_ExecutiveDirector sCreditsText_Director sCreditsText_ArtDirector sCreditsText_BattleDirector sCreditsText_MainProgrammer sCreditsText_BattleSystemPgrms sCreditsText_FieldSystemPgrms sCreditsText_Programmers sCreditsText_MainGraphicDesigner sCreditsText_GraphicDesigners sCreditsText_PkmnDesigners sCreditsText_MusicComposition sCreditsText_SoundEffectsAndPkmnVoices sCreditsText_GameDesigners sCreditsText_ScenarioPlot sCreditsText_Scenario sCreditsText_ScriptDesigners sCreditsText_MapDesigners sCreditsText_MapDataDesigners sCreditsText_ParametricDesigners sCreditsText_PokedexText sCreditsText_EnvAndToolPgrms sCreditsText_NCLProductTesting sCreditsText_SpecialThanks sCreditsText_Coordinators sCreditsText_Producers sCreditsText_ExecProducers sCreditsText_InfoSupervisors sCreditsText_TaskManagers sCreditsText_BrailleCodeCheck sCreditsText_WorldDirector sCreditsText_BattleFrontierData sCreditsText_SupportProgrammers sCreditsText_Artwork sCreditsText_LeadProgrammer sCreditsText_LeadGraphicArtist sCreditsText_SatoshiTajiri sCreditsText_JunichiMasuda sCreditsText_KenSugimori sCreditsText_ShigekiMorimoto sCreditsText_TetsuyaWatanabe sCreditsText_HisashiSogabe sCreditsText_SosukeTamada sCreditsText_AkitoMori sCreditsText_KeitaKagaya sCreditsText_YoshinoriMatsuda sCreditsText_HiroyukiNakamura sCreditsText_MasaoTaya sCreditsText_SatoshiNohara sCreditsText_TomomichiOhta sCreditsText_MiyukiIwasawa sCreditsText_TakenoriOhta sCreditsText_HironobuYoshida sCreditsText_MotofumiFujiwara sCreditsText_SatoshiOhta sCreditsText_AsukaIwashita sCreditsText_AimiTomita sCreditsText_TakaoUnno sCreditsText_KanakoEo sCreditsText_JunOkutani sCreditsText_AtsukoNishida sCreditsText_MuneoSaito sCreditsText_RenaYoshikawa sCreditsText_GoIchinose sCreditsText_MorikazuAoki sCreditsText_KojiNishino sCreditsText_KenjiMatsushima sCreditsText_TetsujiOhta sCreditsText_HitomiSato sCreditsText_TakeshiKawachimaru sCreditsText_TeruyukiShimoyamada sCreditsText_ShigeruOhmori sCreditsText_TadashiTakahashi sCreditsText_ToshinobuMatsumiya sCreditsText_AkihitoTomisawa sCreditsText_HirokiEnomoto sCreditsText_KazuyukiTerada sCreditsText_YuriSakurai sCreditsText_HiromiSagawa sCreditsText_KenjiTominaga sCreditsText_YoshioTajiri sCreditsText_TeikoSasaki sCreditsText_SachikoHamano sCreditsText_ChieMatsumiya sCreditsText_AkikoShinozaki sCreditsText_AstukoFujii sCreditsText_NozomuSaito sCreditsText_KenkichiToyama sCreditsText_SuguruNakatsui sCreditsText_YumiFunasaka sCreditsText_NaokoYanase sCreditsText_NCLSuperMarioClub sCreditsText_AtsushiTada sCreditsText_TakahiroOhnishi sCreditsText_NorihideOkamura sCreditsText_HiroNakamura sCreditsText_HiroyukiUesugi sCreditsText_TerukiMurakawa sCreditsText_AkiraKinashi sCreditsText_MichikoTakizawa sCreditsText_MakikoTakada sCreditsText_TakanaoKondo sCreditsText_AiMashima sCreditsText_GakujiNomoto sCreditsText_TakehiroIzushi sCreditsText_HitoshiYamagami sCreditsText_KyokoWatanabe sCreditsText_TakaoNakano sCreditsText_HiroyukiJinnai sCreditsText_HiroakiTsuru sCreditsText_TsunekazIshihara sCreditsText_SatoruIwata sCreditsText_KazuyaSuyama sCreditsText_SatoshiMitsuhara sCreditsText_JapanBrailleLibrary sCreditsText_TomotakaKomura sCreditsText_MikikoOhhashi sCreditsText_DaisukeHoshino sCreditsText_KenjiroIto sCreditsText_RuiKawaguchi sCreditsText_ShunsukeKohori sCreditsText_SachikoNakamichi sCreditsText_FujikoNomura sCreditsText_KazukiYoshihara sCreditsText_RetsujiNomoto sCreditsText_AzusaTajima sCreditsText_ShusakuEgami sCreditsText_PackageAndManual sCreditsText_EnglishVersion sCreditsText_Translator sCreditsText_TextEditor sCreditsText_NCLCoordinator sCreditsText_GraphicDesigner sCreditsText_NOAProductTesting sCreditsText_HideyukiNakajima sCreditsText_HidenoriSaeki sCreditsText_YokoWatanabe sCreditsText_SakaeKimura sCreditsText_ChiakiShinkai sCreditsText_SethMcMahill sCreditsText_NobOgasawara sCreditsText_TeresaLillygren sCreditsText_KimikoNakamichi sCreditsText_SouichiYamamoto sCreditsText_YuichiroIto sCreditsText_ThomasHertzog sCreditsText_MikaKurosawa sCreditsText_NationalFederationBlind sCreditsText_PatriciaAMaurer sCreditsText_EuropeanBlindUnion sCreditsText_AustralianBrailleAuthority sCreditsText_RoyalNewZealandFederationBlind sCreditsText_MotoyasuTojima sCreditsText_NicolaPrattBarlow sCreditsText_ShellieDow sCreditsText_ErikJohnson sCreditsEntry_EmptyString sCreditsEntry_PkmnEmeraldVersion sCreditsEntry_Credits sCreditsEntry_ExecutiveDirector sCreditsEntry_Director sCreditsEntry_ArtDirector sCreditsEntry_BattleDirector sCreditsEntry_MainProgrammer sCreditsEntry_BattleSystemPgrms sCreditsEntry_FieldSystemPgrms sCreditsEntry_Programmers sCreditsEntry_MainGraphicDesigner sCreditsEntry_GraphicDesigners sCreditsEntry_PkmnDesigners sCreditsEntry_MusicComposition sCreditsEntry_SoundEffectsAndPkmnVoices sCreditsEntry_GameDesigners sCreditsEntry_ScenarioPlot sCreditsEntry_Scenario sCreditsEntry_ScriptDesigners sCreditsEntry_MapDesigners sCreditsEntry_MapDataDesigners sCreditsEntry_ParametricDesigners sCreditsEntry_PokedexText sCreditsEntry_EnvAndToolPgrms sCreditsEntry_NCLProductTesting sCreditsEntry_SpecialThanks sCreditsEntry_Coordinators sCreditsEntry_Producers sCreditsEntry_ExecProducers sCreditsEntry_InfoSupervisors sCreditsEntry_TaskManagers sCreditsEntry_BrailleCodeCheck sCreditsEntry_WorldDirector sCreditsEntry_BattleFrontierData sCreditsEntry_SupportProgrammers sCreditsEntry_Artwork sCreditsEntry_LeadProgrammer sCreditsEntry_LeadGraphicArtist sCreditsEntry_SatoshiTajiri sCreditsEntry_JunichiMasuda sCreditsEntry_KenSugimori sCreditsEntry_ShigekiMorimoto sCreditsEntry_TetsuyaWatanabe sCreditsEntry_HisashiSogabe sCreditsEntry_SosukeTamada sCreditsEntry_AkitoMori sCreditsEntry_KeitaKagaya sCreditsEntry_YoshinoriMatsuda sCreditsEntry_HiroyukiNakamura sCreditsEntry_MasaoTaya sCreditsEntry_SatoshiNohara sCreditsEntry_TomomichiOhta sCreditsEntry_MiyukiIwasawa sCreditsEntry_TakenoriOhta sCreditsEntry_HironobuYoshida sCreditsEntry_MotofumiFujiwara sCreditsEntry_SatoshiOhta sCreditsEntry_AsukaIwashita sCreditsEntry_AimiTomita sCreditsEntry_TakaoUnno sCreditsEntry_KanakoEo sCreditsEntry_JunOkutani sCreditsEntry_AtsukoNishida sCreditsEntry_MuneoSaito sCreditsEntry_RenaYoshikawa sCreditsEntry_GoIchinose sCreditsEntry_MorikazuAoki sCreditsEntry_KojiNishino sCreditsEntry_KenjiMatsushima sCreditsEntry_TetsujiOhta sCreditsEntry_HitomiSato sCreditsEntry_TakeshiKawachimaru sCreditsEntry_TeruyukiShimoyamada sCreditsEntry_ShigeruOhmori sCreditsEntry_TadashiTakahashi sCreditsEntry_ToshinobuMatsumiya sCreditsEntry_AkihitoTomisawa sCreditsEntry_HirokiEnomoto sCreditsEntry_KazuyukiTerada sCreditsEntry_YuriSakurai sCreditsEntry_HiromiSagawa sCreditsEntry_KenjiTominaga sCreditsEntry_YoshioTajiri sCreditsEntry_TeikoSasaki sCreditsEntry_SachikoHamano sCreditsEntry_ChieMatsumiya sCreditsEntry_AkikoShinozaki sCreditsEntry_AstukoFujii sCreditsEntry_NozomuSaito sCreditsEntry_KenkichiToyama sCreditsEntry_SuguruNakatsui sCreditsEntry_YumiFunasaka sCreditsEntry_NaokoYanase sCreditsEntry_NCLSuperMarioClub sCreditsEntry_AtsushiTada sCreditsEntry_TakahiroOhnishi sCreditsEntry_NorihideOkamura sCreditsEntry_HiroNakamura sCreditsEntry_HiroyukiUesugi sCreditsEntry_TerukiMurakawa sCreditsEntry_AkiraKinashi sCreditsEntry_MichikoTakizawa sCreditsEntry_MakikoTakada sCreditsEntry_TakanaoKondo sCreditsEntry_AiMashima sCreditsEntry_GakujiNomoto sCreditsEntry_TakehiroIzushi sCreditsEntry_HitoshiYamagami sCreditsEntry_KyokoWatanabe sCreditsEntry_TakaoNakano sCreditsEntry_HiroyukiJinnai sCreditsEntry_HiroakiTsuru sCreditsEntry_TsunekazIshihara sCreditsEntry_SatoruIwata sCreditsEntry_KazuyaSuyama sCreditsEntry_SatoshiMitsuhara sCreditsEntry_JapanBrailleLibrary sCreditsEntry_TomotakaKomura sCreditsEntry_MikikoOhhashi sCreditsEntry_DaisukeHoshino sCreditsEntry_KenjiroIto sCreditsEntry_RuiKawaguchi sCreditsEntry_ShunsukeKohori sCreditsEntry_SachikoNakamichi sCreditsEntry_FujikoNomura sCreditsEntry_KazukiYoshihara sCreditsEntry_RetsujiNomoto sCreditsEntry_AzusaTajima sCreditsEntry_ShusakuEgami sCreditsEntry_PackageAndManual sCreditsEntry_EnglishVersion sCreditsEntry_Translator sCreditsEntry_TextEditor sCreditsEntry_NCLCoordinator sCreditsEntry_GraphicDesigner sCreditsEntry_NOAProductTesting sCreditsEntry_HideyukiNakajima sCreditsEntry_HidenoriSaeki sCreditsEntry_YokoWatanabe sCreditsEntry_SakaeKimura sCreditsEntry_ChiakiShinkai sCreditsEntry_SethMcMahill sCreditsEntry_NobOgasawara sCreditsEntry_TeresaLillygren sCreditsEntry_KimikoNakamichi sCreditsEntry_SouichiYamamoto sCreditsEntry_YuichiroIto sCreditsEntry_ThomasHertzog sCreditsEntry_MikaKurosawa sCreditsEntry_NationalFederationBlind sCreditsEntry_PatriciaAMaurer sCreditsEntry_EuropeanBlindUnion sCreditsEntry_AustralianBrailleAuthority sCreditsEntry_RoyalNewZealandFederationBlind sCreditsEntry_MotoyasuTojima sCreditsEntry_NicolaPrattBarlow sCreditsEntry_ShellieDow sCreditsEntry_ErikJohnson sCreditsEntryPointerTable sBackgroundTemplates sWindowTemplates sMonSpritePos sAnim_Player_Slow sAnim_Player_Fast sAnim_Player_LookBack sAnim_Player_LookForward sAnims_Player sAnim_Rival_Slow sAnim_Rival_Fast sAnim_Rival_Still sAnims_Rival sSpriteSheet_MonBg sSpritePalette_MonBg sOamData_MonBg sAnim_MonBg_Yellow sAnim_MonBg_Red sAnim_MonBg_Blue sAnims_MonBg sSpriteTemplate_CreditsMonBg

/// `struct CreditsData`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct CreditsData {
    pub monToShow: CArray<u16, 71>,
    pub imgCounter: u16,
    pub nextImgPos: u16,
    pub currShownMon: u16,
    pub numMonToShow: u16,
    pub caughtMonIds: CArray<u16, 386>,
    pub numCaughtMon: u16,
    pub unused: CArray<u16, 7>,
}

unsafe impl Sync for CreditsData {}

/// `struct CreditsEntry`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CreditsEntry {
    pub unk: u8,
    pub isTitle: u8,
    pub text: *mut u8,
}

unsafe impl Sync for CreditsEntry {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<CreditsData>() == 940);
    assert!(offset_of!(CreditsData, monToShow) == 0);
    assert!(offset_of!(CreditsData, imgCounter) == 142);
    assert!(offset_of!(CreditsData, nextImgPos) == 144);
    assert!(offset_of!(CreditsData, currShownMon) == 146);
    assert!(offset_of!(CreditsData, numMonToShow) == 148);
    assert!(offset_of!(CreditsData, caughtMonIds) == 150);
    assert!(offset_of!(CreditsData, numCaughtMon) == 922);
    assert!(offset_of!(CreditsData, unused) == 924);
    assert!(size_of::<CreditsEntry>() == 8);
    assert!(offset_of!(CreditsEntry, unk) == 0);
    assert!(offset_of!(CreditsEntry, isTitle) == 1);
    assert!(offset_of!(CreditsEntry, text) == 4);
};

const ENTRIES_PER_PAGE: i32 = 5;
const MODE_BIKE_SCENE: i16 = 1;
const MODE_NONE: i16 = 0;
const MODE_SHOW_MONS: i16 = 2;
const MONBG_OFFSET: i32 = 6144;
const NUM_MON_SLIDES: u16 = 71;
const PAGE_COUNT: i16 = 57;
const POS_CENTER: i32 = 1;
const POS_LEFT: u16 = 0;
const POS_RIGHT: u16 = 2;
const TIMER_STOP: i16 = 32767;

static sAnims_Player: Table<CArray<*mut AnimCmd, 4>> =
    Table((&raw const crate::data::credits::sAnims_Player).cast());
static sAnims_Rival: Table<CArray<*mut AnimCmd, 3>> =
    Table((&raw const crate::data::credits::sAnims_Rival).cast());
static sBackgroundTemplates: Table<CArray<BgTemplate, 1>> =
    Table((&raw const crate::data::credits::sBackgroundTemplates).cast());
static sCreditsCopyrightEnd_Gfx: Table<CArray<u32, 271>> =
    Table((&raw const crate::data::credits::sCreditsCopyrightEnd_Gfx).cast());
static sCreditsEntryPointerTable: Table<CArray<CArray<*mut CreditsEntry, 5>, 57>> =
    Table((&raw const crate::data::credits::sCreditsEntryPointerTable).cast());
static sCredits_Pal: Table<CArray<u16, 64>> =
    Table((&raw const crate::data::credits::sCredits_Pal).cast());
static sMonSpritePos: Table<CArray<CArray<u8, 2>, 3>> =
    Table((&raw const crate::data::credits::sMonSpritePos).cast());
static sSpritePalette_MonBg: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::credits::sSpritePalette_MonBg).cast());
static sSpriteSheet_MonBg: Table<CArray<SpriteSheet, 2>> =
    Table((&raw const crate::data::credits::sSpriteSheet_MonBg).cast());
static sSpriteTemplate_CreditsMonBg: Table<SpriteTemplate> =
    Table((&raw const crate::data::credits::sSpriteTemplate_CreditsMonBg).cast());
static sTheEnd_LetterMap_D: Table<CArray<u8, 15>> =
    Table((&raw const crate::data::credits::sTheEnd_LetterMap_D).cast());
static sTheEnd_LetterMap_E: Table<CArray<u8, 15>> =
    Table((&raw const crate::data::credits::sTheEnd_LetterMap_E).cast());
static sTheEnd_LetterMap_H: Table<CArray<u8, 15>> =
    Table((&raw const crate::data::credits::sTheEnd_LetterMap_H).cast());
static sTheEnd_LetterMap_N: Table<CArray<u8, 15>> =
    Table((&raw const crate::data::credits::sTheEnd_LetterMap_N).cast());
static sTheEnd_LetterMap_T: Table<CArray<u8, 15>> =
    Table((&raw const crate::data::credits::sTheEnd_LetterMap_T).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::credits::sWindowTemplates).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnkVar: i16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavedTaskId: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gHasHallOfFameRecords: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUsedSpeedUp: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCreditsData: *mut CreditsData = null_mut();

unsafe extern "C" {
    static gBirchBagGrass_Gfx: CArray<u32, 0>;
    static gBirchBagGrass_Pal: CArray<u16, 0>;
    static gBirchGrassTilemap: CArray<u32, 0>;
    static mut gDecompressionBuffer: CArray<u8, 16384>;
    static mut gHeap: CArray<u8, 114688>;
    static gIntroCopyright_Pal: CArray<u16, 16>;
    static mut gIntroCredits_MovingSceneryState: i16;
    static mut gIntroCredits_MovingSceneryVBase: u16;
    static mut gIntroCredits_MovingSceneryVOffset: i16;
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gReservedSpritePaletteCount: u8;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static gSpritePalettes_Credits: CArray<SpritePalette, 0>;
    static gSpriteSheet_CreditsBicycle: CArray<CompressedSpriteSheet, 0>;
    static gSpriteSheet_CreditsBrendan: CArray<CompressedSpriteSheet, 0>;
    static gSpriteSheet_CreditsMay: CArray<CompressedSpriteSheet, 0>;
    static gSpriteSheet_CreditsRivalBrendan: CArray<CompressedSpriteSheet, 0>;
    static gSpriteSheet_CreditsRivalMay: CArray<CompressedSpriteSheet, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
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
    fn BuildOamBuffer();
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateBicycleBgAnimationTask(a0: u8, a1: u16, a2: u16, a3: u16) -> u8;
    fn CreateIntroBrendanSprite(a0: i16, a1: i16) -> u8;
    fn CreateIntroMaySprite(a0: i16, a1: i16) -> u8;
    fn CreateMonSpriteFromNationalDexNumber(a0: u16, a1: i16, a2: i16, a3: u16) -> u16;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CycleSceneryPalette(a0: u8);
    fn DeactivateAllTextPrinters();
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn EnableInterrupts(a0: u16);
    fn FadeOutBGM(a0: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeAndDestroyMonPicSprite(a0: u16) -> u16;
    fn GetBgTilemapBuffer(a0: u8) -> *mut c_void;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn GetStarterPokemon(a0: u16) -> u16;
    fn GetStringCenterAlignXOffsetWithLetterSpacing(a0: i32, a1: *mut u8, a2: i32, a3: i32) -> i32;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitHeap(a0: *mut c_void, a1: u32);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadCreditsSceneGraphics(a0: u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpritePalettes(a0: *mut SpritePalette);
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn ResetAllPicSprites() -> u16;
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetCreditsSceneBgCnt(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetOamMatrix(a0: u8, a1: u16, a2: u16, a3: u16, a4: u16);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SoftReset(a0: u32);
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn StartSpriteAnimIfDifferent(a0: *mut Sprite, a1: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn VarGet(a0: u16) -> u16;
    fn m4aSongNumStart(a0: u16);
}

pub(crate) unsafe extern "C" fn VBlankCB_Credits() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn CB2_Credits() {
    RunTasks();
    AnimateSprites();
    if gMain.heldKeys as i32 & B_BUTTON != 0
        && gHasHallOfFameRecords != 0
        && gTasks[sSavedTaskId].func == Some(Task_CreditsMain as unsafe extern "C" fn(u8))
    {
        VBlankCB_Credits();
        RunTasks();
        AnimateSprites();
        sUsedSpeedUp = TRUE;
    }
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn InitCreditsBgsAndWindows() {
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBackgroundTemplates.as_ptr().cast_mut(), 1);
    SetBgTilemapBuffer(0, AllocZeroed(BG_SCREEN_SIZE));
    LoadPalette(sCredits_Pal.as_ptr().cast_mut() as *mut c_void, 128, 64);
    InitWindows(sWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    PutWindowTilemap(0);
    CopyWindowToVram(0, COPYWIN_FULL);
    ShowBg(0);
}
pub(crate) unsafe extern "C" fn FreeCreditsBgsAndWindows() {
    let mut ptr: *mut c_void = null_mut();
    FreeAllWindowBuffers();
    ptr = GetBgTilemapBuffer(0);
    if !ptr.is_null() {
        Free(ptr);
    }
}
pub(crate) unsafe extern "C" fn PrintCreditsText(string: *mut u8, y: u8, isTitle: u8) {
    let mut x: u8 = 0;
    let mut color: CArray<u8, 3> = zeroed();
    color[0] = 0x0;
    if isTitle == TRUE {
        color[1] = TEXT_COLOR_LIGHT_GRAY;
        color[2] = TEXT_COLOR_RED;
    } else {
        color[1] = 0x1;
        color[2] = 0x2;
    }
    x = GetStringCenterAlignXOffsetWithLetterSpacing(
        FONT_NORMAL as i32,
        string,
        DISPLAY_WIDTH as i32,
        1,
    ) as u8;
    AddTextPrinterParameterized4(
        0,
        FONT_NORMAL,
        x,
        y,
        1,
        0,
        color.as_mut_ptr(),
        TEXT_SKIP_DRAW as i8,
        string,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_StartCreditsSequence() {
    let mut taskId: u8 = 0;
    let mut bikeTaskId: i16 = 0;
    let mut pageTaskId: u8 = 0;
    ResetGpuAndVram();
    SetVBlankCallback(None);
    InitHeap(gHeap.as_mut_ptr() as *mut c_void, HEAP_SIZE);
    ResetPaletteFade();
    ResetTasks();
    InitCreditsBgsAndWindows();
    taskId = CreateTask(Some(Task_WaitPaletteFade), 0);
    gTasks[taskId].data[4] = FALSE as i16;
    gTasks[taskId].data[7] = SCENE_OCEAN_MORNING as i16;
    gTasks[taskId].data[11] = MODE_NONE;
    gTasks[taskId].data[13] = MODE_BIKE_SCENE;
    loop {
        if LoadBikeScene(SCENE_OCEAN_MORNING, taskId) != 0 {
            break;
        }
    }
    bikeTaskId = gTasks[taskId].data[1];
    gTasks[bikeTaskId].data[0] = 40;
    SetGpuReg(REG_OFFSET_BG0VOFS, 0xFFFC);
    pageTaskId = CreateTask(Some(Task_UpdatePage), 0);
    gTasks[pageTaskId].data[1] = taskId as i16;
    gTasks[taskId].data[15] = pageTaskId as i16;
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
    EnableInterrupts(INTR_FLAG_VBLANK);
    SetVBlankCallback(Some(VBlankCB_Credits));
    m4aSongNumStart(MUS_CREDITS);
    SetMainCallback2(Some(CB2_Credits));
    sUsedSpeedUp = FALSE;
    sCreditsData = AllocZeroed(940) as *mut CreditsData;
    DeterminePokemonToShow();
    (*sCreditsData).imgCounter = 0;
    (*sCreditsData).nextImgPos = POS_LEFT;
    (*sCreditsData).currShownMon = 0;
    sSavedTaskId = taskId as u16;
}
pub(crate) unsafe extern "C" fn Task_WaitPaletteFade(taskId: u8) {
    if gPaletteFade.active() == 0 {
        gTasks[taskId].func = Some(Task_CreditsMain);
    }
}
pub(crate) unsafe extern "C" fn Task_CreditsMain(taskId: u8) {
    let mut mode: u16 = 0;
    if gTasks[taskId].data[4] != 0 {
        let mut bikeTaskId: i16 = gTasks[taskId].data[1];
        gTasks[bikeTaskId].data[0] = 30;
        gTasks[taskId].data[12] = 256;
        gTasks[taskId].func = Some(Task_CreditsTheEnd1);
        return;
    }
    sUnkVar = 0;
    mode = gTasks[taskId].data[11] as u16;
    if gTasks[taskId].data[11] == MODE_BIKE_SCENE {
        gTasks[taskId].data[13] = mode as i16;
        gTasks[taskId].data[11] = MODE_NONE;
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        gTasks[taskId].func = Some(Task_ReadyBikeScene);
    } else if gTasks[taskId].data[11] == MODE_SHOW_MONS {
        gTasks[taskId].data[13] = mode as i16;
        gTasks[taskId].data[11] = MODE_NONE;
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        gTasks[taskId].func = Some(Task_ReadyShowMons);
    }
}
pub(crate) unsafe extern "C" fn Task_ReadyBikeScene(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetGpuReg(0x0, 0);
        ResetCreditsTasks(taskId);
        gTasks[taskId].func = Some(Task_SetBikeScene);
    }
}
pub(crate) unsafe extern "C" fn Task_SetBikeScene(taskId: u8) {
    SetVBlankCallback(None);
    if LoadBikeScene(gTasks[taskId].data[7] as u8, taskId) != 0 {
        BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
        EnableInterrupts(INTR_FLAG_VBLANK);
        SetVBlankCallback(Some(VBlankCB_Credits));
        gTasks[taskId].func = Some(Task_WaitPaletteFade);
    }
}
pub(crate) unsafe extern "C" fn Task_ReadyShowMons(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetGpuReg(0x0, 0);
        ResetCreditsTasks(taskId);
        gTasks[taskId].func = Some(Task_LoadShowMons);
    }
}
pub(crate) unsafe extern "C" fn Task_LoadShowMons(taskId: u8) {
    'l1: {
        match gMain.state {
            1 => {
                gTasks[taskId].data[3] = CreateTask(Some(Task_ShowMons), 0) as i16;
                gTasks[gTasks[taskId].data[3]].data[0] = 1;
                gTasks[gTasks[taskId].data[3]].data[1] = taskId as i16;
                gTasks[gTasks[taskId].data[3]].data[2] = gTasks[taskId].data[7];
                BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
                SetGpuReg(REG_OFFSET_BG3HOFS, 0);
                SetGpuReg(REG_OFFSET_BG3VOFS, 32);
                SetGpuReg(REG_OFFSET_BG3CNT, 1795);
                SetGpuReg(0x0, 6464);
                gMain.state = 0;
                gIntroCredits_MovingSceneryState = INTROCRED_SCENERY_NORMAL;
                gTasks[taskId].func = Some(Task_WaitPaletteFade);
            }
            _ => {
                let mut i: u16 = 0;
                let mut temp: *mut u16 = null_mut();
                ResetSpriteData();
                ResetAllPicSprites();
                FreeAllSpritePalettes();
                gReservedSpritePaletteCount = 8;
                LZ77UnCompVram(
                    gBirchBagGrass_Gfx.as_ptr().cast_mut(),
                    VRAM as usize as *mut c_void,
                );
                LZ77UnCompVram(
                    gBirchGrassTilemap.as_ptr().cast_mut(),
                    0x6003800 as usize as *mut c_void,
                );
                LoadPalette(
                    gBirchBagGrass_Pal.as_ptr().cast_mut().at(1) as *mut c_void,
                    1,
                    62,
                );
                i = 0;
                while i < MON_PIC_SIZE {
                    gDecompressionBuffer[i] = 0x11;
                    i += 1;
                }
                i = 0;
                while i < MON_PIC_SIZE {
                    *gDecompressionBuffer.as_mut_ptr().at(2048).at(i) = 0x22;
                    i += 1;
                }
                i = 0;
                while i < MON_PIC_SIZE {
                    *gDecompressionBuffer.as_mut_ptr().at(4096).at(i) = 0x33;
                    i += 1;
                }
                temp = &raw mut gDecompressionBuffer[6144] as *mut u16;
                *temp = 0;
                *temp.at(1) = 21503;
                *temp.at(2) = 21151;
                *temp.at(3) = 32404;
                LoadSpriteSheet(sSpriteSheet_MonBg.as_ptr().cast_mut());
                LoadSpritePalette(sSpritePalette_MonBg.as_ptr().cast_mut());
                gMain.state += 1;
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CreditsTheEnd1(taskId: u8) {
    if gTasks[taskId].data[12] != 0 {
        gTasks[taskId].data[12] -= 1;
        return;
    }
    BeginNormalPaletteFade(PALETTES_ALL, 12, 0, 16, 0);
    gTasks[taskId].func = Some(Task_CreditsTheEnd2);
}
pub(crate) unsafe extern "C" fn Task_CreditsTheEnd2(taskId: u8) {
    if gPaletteFade.active() == 0 {
        ResetCreditsTasks(taskId);
        gTasks[taskId].func = Some(Task_CreditsTheEnd3);
    }
}
pub(crate) unsafe extern "C" fn Task_CreditsTheEnd3(taskId: u8) {
    ResetGpuAndVram();
    ResetPaletteFade();
    LoadTheEndScreen(0, 0x3800, 0);
    ResetSpriteData();
    FreeAllSpritePalettes();
    BeginNormalPaletteFade(PALETTES_ALL, 8, 16, 0, 0);
    SetGpuReg(REG_OFFSET_BG0CNT, 1792);
    EnableInterrupts(INTR_FLAG_VBLANK);
    SetGpuReg(0x0, 320);
    gTasks[taskId].data[0] = 235;
    gTasks[taskId].func = Some(Task_CreditsTheEnd4);
}
pub(crate) unsafe extern "C" fn Task_CreditsTheEnd4(taskId: u8) {
    if gTasks[taskId].data[0] != 0 {
        gTasks[taskId].data[0] -= 1;
        return;
    }
    BeginNormalPaletteFade(PALETTES_ALL, 6, 0, 16, 0);
    gTasks[taskId].func = Some(Task_CreditsTheEnd5);
}
pub(crate) unsafe extern "C" fn Task_CreditsTheEnd5(taskId: u8) {
    if gPaletteFade.active() == 0 {
        DrawTheEnd(0x3800, 0);
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0, 0);
        gTasks[taskId].data[0] = 7200;
        gTasks[taskId].func = Some(Task_CreditsTheEnd6);
    }
}
pub(crate) unsafe extern "C" fn Task_CreditsTheEnd6(taskId: u8) {
    if gPaletteFade.active() == 0 {
        if gTasks[taskId].data[0] == 0 || gMain.newKeys != 0 {
            FadeOutBGM(4);
            BeginNormalPaletteFade(PALETTES_ALL, 8, 0, 16, 65535);
            gTasks[taskId].func = Some(Task_CreditsSoftReset);
            return;
        }
        if gTasks[taskId].data[0] == 7144 {
            FadeOutBGM(8);
        }
        if gTasks[taskId].data[0] == 6840 {
            m4aSongNumStart(MUS_END);
        }
        gTasks[taskId].data[0] -= 1;
    }
}
pub(crate) unsafe extern "C" fn Task_CreditsSoftReset(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SoftReset(RESET_ALL);
    }
}
pub(crate) unsafe extern "C" fn ResetGpuAndVram() {
    SetGpuReg(0x0, 0);
    SetGpuReg(REG_OFFSET_BG3HOFS, 0);
    SetGpuReg(REG_OFFSET_BG3VOFS, 0);
    SetGpuReg(REG_OFFSET_BG2HOFS, 0);
    SetGpuReg(REG_OFFSET_BG2VOFS, 0);
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    SetGpuReg(REG_OFFSET_BG0HOFS, 0);
    SetGpuReg(REG_OFFSET_BG0VOFS, 0);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(dmaRegs.at(1), VRAM as usize as *mut c_void as usize as u32);
                    volatile_write(dmaRegs.at(2), 0x8100c000);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(
                        dmaRegs.at(1),
                        OAM as i32 as usize as *mut c_void as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x85000100);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(
                        dmaRegs.at(1),
                        83886082 as usize as *mut c_void as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x810001ff);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_UpdatePage(taskId: u8) {
    let mut i: i32 = 0;
    match gTasks[taskId].data[0] {
        1 => {
            if gTasks[taskId].data[3] != 0 {
                gTasks[taskId].data[3] -= 1;
                return;
            }
            gTasks[taskId].data[0] += 1;
            return;
        }
        2 => {
            if gTasks[gTasks[taskId].data[1]].func
                == Some(Task_CreditsMain as unsafe extern "C" fn(u8))
            {
                if gTasks[taskId].data[2] < PAGE_COUNT {
                    i = 0;
                    while i < ENTRIES_PER_PAGE {
                        PrintCreditsText(
                            (*sCreditsEntryPointerTable[gTasks[taskId].data[2]][i]).text,
                            5 + i as u8 * 16,
                            (*sCreditsEntryPointerTable[gTasks[taskId].data[2]][i]).isTitle,
                        );
                        i += 1;
                    }
                    CopyWindowToVram(0, COPYWIN_GFX);
                    gTasks[taskId].data[2] += 1;
                    gTasks[taskId].data[0] += 1;
                    gTasks[gTasks[taskId].data[1]].data[14] = TRUE as i16;
                    if gTasks[gTasks[taskId].data[1]].data[13] == MODE_BIKE_SCENE {
                        BeginNormalPaletteFade(0x300, 0, 16, 0, 12941);
                    } else {
                        BeginNormalPaletteFade(0x300, 0, 16, 0, 6503);
                    }
                    return;
                }
                gTasks[taskId].data[0] = 10;
                return;
            }
            gTasks[gTasks[taskId].data[1]].data[14] = FALSE as i16;
            return;
        }
        3 => {
            if gPaletteFade.active() == 0 {
                gTasks[taskId].data[3] = 115;
                gTasks[taskId].data[0] += 1;
            }
            return;
        }
        4 => {
            if gTasks[taskId].data[3] != 0 {
                gTasks[taskId].data[3] -= 1;
                return;
            }
            if CheckChangeScene(gTasks[taskId].data[2] as u8, gTasks[taskId].data[1] as u8) != 0 {
                gTasks[taskId].data[0] += 1;
                return;
            }
            gTasks[taskId].data[0] += 1;
            if gTasks[gTasks[taskId].data[1]].data[13] == MODE_BIKE_SCENE {
                BeginNormalPaletteFade(0x300, 0, 0, 16, 12941);
            } else {
                BeginNormalPaletteFade(0x300, 0, 0, 16, 6503);
            }
            return;
        }
        5 => {
            if gPaletteFade.active() == 0 {
                FillWindowPixelBuffer(0, 0);
                CopyWindowToVram(0, COPYWIN_GFX);
                gTasks[taskId].data[0] = 2;
            }
            return;
        }
        10 => {
            gTasks[gTasks[taskId].data[1]].data[4] = TRUE as i16;
            DestroyTask(taskId);
            FreeCreditsBgsAndWindows();
            Free(sCreditsData as *mut c_void);
            sCreditsData = null_mut();
            return;
        }
        _ => {
            if gPaletteFade.active() == 0 {
                gTasks[taskId].data[0] = 1;
                gTasks[taskId].data[3] = 72;
                gTasks[gTasks[taskId].data[1]].data[14] = FALSE as i16;
                sUnkVar = 0;
            }
            return;
        }
    }
}
pub(crate) unsafe extern "C" fn CheckChangeScene(page: u8, taskId: u8) -> u8 {
    if page == 6 {
        gTasks[taskId].data[11] = MODE_SHOW_MONS;
    }
    if page == 12 {
        gTasks[taskId].data[7] = SCENE_OCEAN_SUNSET;
        gTasks[taskId].data[11] = MODE_BIKE_SCENE;
    }
    if page == 18 {
        gTasks[taskId].data[11] = MODE_SHOW_MONS;
    }
    if page == 24 {
        gTasks[taskId].data[7] = SCENE_FOREST_RIVAL_ARRIVE as i16;
        gTasks[taskId].data[11] = MODE_BIKE_SCENE;
    }
    if page == 30 {
        gTasks[taskId].data[11] = MODE_SHOW_MONS;
    }
    if page == 36 {
        gTasks[taskId].data[7] = SCENE_FOREST_CATCH_RIVAL;
        gTasks[taskId].data[11] = MODE_BIKE_SCENE;
    }
    if page == 42 {
        gTasks[taskId].data[11] = MODE_SHOW_MONS;
    }
    if page == 48 {
        gTasks[taskId].data[7] = SCENE_CITY_NIGHT;
        gTasks[taskId].data[11] = MODE_BIKE_SCENE;
    }
    if gTasks[taskId].data[11] != MODE_NONE {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_ShowMons(taskId: u8) {
    let mut spriteId: u8 = 0;
    'l1: {
        match gTasks[taskId].data[0] {
            0 => {}
            1 => {
                if (*sCreditsData).nextImgPos == POS_LEFT
                    && gTasks[gTasks[taskId].data[1]].data[14] == FALSE as i16
                {
                    break 'l1;
                }
                gTasks[taskId].data[0] += 1;
            }
            2 => {
                if (*sCreditsData).imgCounter == NUM_MON_SLIDES
                    || gTasks[gTasks[taskId].data[1]].func
                        != Some(Task_CreditsMain as unsafe extern "C" fn(u8))
                {
                    break 'l1;
                }
                spriteId = CreateCreditsMonSprite(
                    (*sCreditsData).monToShow[(*sCreditsData).currShownMon],
                    sMonSpritePos[(*sCreditsData).nextImgPos][0] as i16,
                    sMonSpritePos[(*sCreditsData).nextImgPos][1] as i16,
                    (*sCreditsData).nextImgPos,
                );
                if ((*sCreditsData).currShownMon as i32) < (*sCreditsData).numMonToShow as i32 - 1 {
                    (*sCreditsData).currShownMon += 1;
                    gSprites[spriteId].data[3] = 50;
                } else {
                    (*sCreditsData).currShownMon = 0;
                    gSprites[spriteId].data[3] = 512;
                }
                (*sCreditsData).imgCounter += 1;
                if (*sCreditsData).nextImgPos == POS_RIGHT {
                    (*sCreditsData).nextImgPos = POS_LEFT;
                } else {
                    (*sCreditsData).nextImgPos += 1;
                }
                gTasks[taskId].data[3] = 50;
                gTasks[taskId].data[0] += 1;
            }
            3 => {
                if gTasks[taskId].data[3] != 0 {
                    gTasks[taskId].data[3] -= 1;
                } else {
                    gTasks[taskId].data[0] = 1;
                }
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn Task_BikeScene(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            gIntroCredits_MovingSceneryVOffset = Sin(gTasks[taskId].data[5] >> 1 & 0x7F, 12);
            gTasks[taskId].data[5] += 1;
        }
        1 => {
            if gIntroCredits_MovingSceneryVOffset != 0 {
                gIntroCredits_MovingSceneryVOffset = Sin(gTasks[taskId].data[5] >> 1 & 0x7F, 12);
                gTasks[taskId].data[5] += 1;
            } else {
                gSprites[gTasks[taskId].data[2]].data[0] = 2;
                gTasks[taskId].data[5] = 0;
                gTasks[taskId].data[0] += 1;
            }
        }
        2 => {
            if gTasks[taskId].data[5] < 64 {
                gTasks[taskId].data[5] += 1;
                gIntroCredits_MovingSceneryVOffset = Sin(gTasks[taskId].data[5] & 0x7F, 20);
            } else {
                gTasks[taskId].data[0] += 1;
            }
        }
        3 => {
            gSprites[gTasks[taskId].data[2]].data[0] = 3;
            gSprites[gTasks[taskId].data[3]].data[0] = 1;
            gTasks[taskId].data[4] = 120;
            gTasks[taskId].data[0] += 1;
        }
        4 => {
            if gTasks[taskId].data[4] != 0 {
                gTasks[taskId].data[4] -= 1;
            } else {
                gTasks[taskId].data[5] = 64;
                gTasks[taskId].data[0] += 1;
            }
        }
        5 => {
            if gTasks[taskId].data[5] > 0 {
                gTasks[taskId].data[5] -= 1;
                gIntroCredits_MovingSceneryVOffset = Sin(gTasks[taskId].data[5] & 0x7F, 20);
            } else {
                gSprites[gTasks[taskId].data[2]].data[0] = 1;
                gTasks[taskId].data[0] += 1;
            }
        }
        6 => {
            gTasks[taskId].data[0] = 50;
        }
        10 => {
            gSprites[gTasks[taskId].data[3]].data[0] = 2;
            gTasks[taskId].data[0] = 50;
        }
        20 => {
            gSprites[gTasks[taskId].data[2]].data[0] = 4;
            gTasks[taskId].data[0] = 50;
        }
        30 => {
            gSprites[gTasks[taskId].data[2]].data[0] = 5;
            gSprites[gTasks[taskId].data[3]].data[0] = 3;
            gTasks[taskId].data[0] = 50;
        }
        50 => {
            gTasks[taskId].data[0] = 0;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_CycleSceneryPalette(taskId: u8) {
    let mut bikeTaskId: i16 = 0;
    match gTasks[taskId].data[0] {
        SCENE_OCEAN_SUNSET => {
            CycleSceneryPalette(0);
        }
        2 => {
            if gTasks[taskId].data[1] != TIMER_STOP {
                bikeTaskId = gTasks[gTasks[taskId].data[2]].data[1];
                if gTasks[bikeTaskId].data[5] as i32 & -128 == 640 {
                    gTasks[bikeTaskId].data[0] = 1;
                    gTasks[taskId].data[1] = TIMER_STOP;
                }
            }
            CycleSceneryPalette(1);
        }
        SCENE_FOREST_CATCH_RIVAL => {
            if gTasks[taskId].data[1] != TIMER_STOP {
                if gTasks[taskId].data[1] == 584 {
                    gTasks[gTasks[gTasks[taskId].data[2]].data[1]].data[0] = 10;
                    gTasks[taskId].data[1] = TIMER_STOP;
                } else {
                    gTasks[taskId].data[1] += 1;
                }
            }
            CycleSceneryPalette(1);
        }
        SCENE_CITY_NIGHT => {
            CycleSceneryPalette(2);
        }
        _ => {
            if gTasks[taskId].data[1] != TIMER_STOP {
                if gTasks[gTasks[gTasks[taskId].data[2]].data[15]].data[2] == 2 {
                    gTasks[gTasks[gTasks[taskId].data[2]].data[1]].data[0] = 20;
                    gTasks[taskId].data[1] = TIMER_STOP;
                }
            }
            CycleSceneryPalette(0);
        }
    }
}
pub(crate) unsafe extern "C" fn SetBikeScene(scene: u8, taskId: u8) {
    match scene {
        SCENE_OCEAN_MORNING => {
            gSprites[gTasks[taskId].data[5]].set_invisible(FALSE as u16);
            gSprites[gTasks[taskId].data[6]].set_invisible(FALSE as u16);
            gSprites[gTasks[taskId].data[5]].x = 272;
            gSprites[gTasks[taskId].data[6]].x = 272;
            gSprites[gTasks[taskId].data[5]].y = 46;
            gSprites[gTasks[taskId].data[6]].y = 46;
            gSprites[gTasks[taskId].data[5]].data[0] = 0;
            gSprites[gTasks[taskId].data[6]].data[0] = 0;
            gTasks[taskId].data[0] = CreateBicycleBgAnimationTask(0, 0x2000, 0x20, 8) as i16;
        }
        1 => {
            gSprites[gTasks[taskId].data[5]].set_invisible(FALSE as u16);
            gSprites[gTasks[taskId].data[6]].set_invisible(FALSE as u16);
            gSprites[gTasks[taskId].data[5]].x = 120;
            gSprites[gTasks[taskId].data[6]].x = 272;
            gSprites[gTasks[taskId].data[5]].y = 46;
            gSprites[gTasks[taskId].data[6]].y = 46;
            gSprites[gTasks[taskId].data[5]].data[0] = 0;
            gSprites[gTasks[taskId].data[6]].data[0] = 0;
            gTasks[taskId].data[0] = CreateBicycleBgAnimationTask(0, 0x2000, 0x20, 8) as i16;
        }
        SCENE_FOREST_RIVAL_ARRIVE => {
            gSprites[gTasks[taskId].data[5]].set_invisible(FALSE as u16);
            gSprites[gTasks[taskId].data[6]].set_invisible(FALSE as u16);
            gSprites[gTasks[taskId].data[5]].x = 120;
            gSprites[gTasks[taskId].data[6]].x = 272;
            gSprites[gTasks[taskId].data[5]].y = 46;
            gSprites[gTasks[taskId].data[6]].y = 46;
            gSprites[gTasks[taskId].data[5]].data[0] = 0;
            gSprites[gTasks[taskId].data[6]].data[0] = 0;
            gTasks[taskId].data[0] = CreateBicycleBgAnimationTask(1, 0x2000, 0x200, 8) as i16;
        }
        3 => {
            gSprites[gTasks[taskId].data[5]].set_invisible(FALSE as u16);
            gSprites[gTasks[taskId].data[6]].set_invisible(FALSE as u16);
            gSprites[gTasks[taskId].data[5]].x = 120;
            gSprites[gTasks[taskId].data[6]].x = -32;
            gSprites[gTasks[taskId].data[5]].y = 46;
            gSprites[gTasks[taskId].data[6]].y = 46;
            gSprites[gTasks[taskId].data[5]].data[0] = 0;
            gSprites[gTasks[taskId].data[6]].data[0] = 0;
            gTasks[taskId].data[0] = CreateBicycleBgAnimationTask(1, 0x2000, 0x200, 8) as i16;
        }
        4 => {
            gSprites[gTasks[taskId].data[5]].set_invisible(FALSE as u16);
            gSprites[gTasks[taskId].data[6]].set_invisible(FALSE as u16);
            gSprites[gTasks[taskId].data[5]].x = 88;
            gSprites[gTasks[taskId].data[6]].x = 152;
            gSprites[gTasks[taskId].data[5]].y = 46;
            gSprites[gTasks[taskId].data[6]].y = 46;
            gSprites[gTasks[taskId].data[5]].data[0] = 0;
            gSprites[gTasks[taskId].data[6]].data[0] = 0;
            gTasks[taskId].data[0] = CreateBicycleBgAnimationTask(2, 0x2000, 0x200, 8) as i16;
        }
        _ => {}
    }
    gTasks[taskId].data[2] = CreateTask(Some(Task_CycleSceneryPalette), 0) as i16;
    gTasks[gTasks[taskId].data[2]].data[0] = scene as i16;
    gTasks[gTasks[taskId].data[2]].data[1] = 0;
    gTasks[gTasks[taskId].data[2]].data[2] = taskId as i16;
    gTasks[taskId].data[1] = CreateTask(Some(Task_BikeScene), 0) as i16;
    gTasks[gTasks[taskId].data[1]].data[0] = 0;
    gTasks[gTasks[taskId].data[1]].data[1] = taskId as i16;
    gTasks[gTasks[taskId].data[1]].data[2] = gTasks[taskId].data[5];
    gTasks[gTasks[taskId].data[1]].data[3] = gTasks[taskId].data[6];
    gTasks[gTasks[taskId].data[1]].data[4] = 0;
    if scene == SCENE_FOREST_RIVAL_ARRIVE {
        gTasks[gTasks[taskId].data[1]].data[5] = 69;
    }
}
pub(crate) unsafe extern "C" fn LoadBikeScene(scene: u8, taskId: u8) -> u8 {
    let mut spriteId: u8 = 0;
    match gMain.state {
        1 => {
            gIntroCredits_MovingSceneryVBase = 34;
            gIntroCredits_MovingSceneryVOffset = 0;
            LoadCreditsSceneGraphics(scene);
            gMain.state += 1;
        }
        2 => {
            if (*gSaveBlock2Ptr).playerGender == MALE {
                LoadCompressedSpriteSheet(gSpriteSheet_CreditsBrendan.as_ptr().cast_mut());
                LoadCompressedSpriteSheet(gSpriteSheet_CreditsRivalMay.as_ptr().cast_mut());
                LoadCompressedSpriteSheet(gSpriteSheet_CreditsBicycle.as_ptr().cast_mut());
                LoadSpritePalettes(gSpritePalettes_Credits.as_ptr().cast_mut());
                spriteId = CreateIntroBrendanSprite(120, 46);
                gTasks[taskId].data[5] = spriteId as i16;
                gSprites[spriteId].callback = Some(SpriteCB_Player);
                gSprites[spriteId].anims = sAnims_Player.as_ptr().cast_mut();
                spriteId = CreateIntroMaySprite(272, 46);
                gTasks[taskId].data[6] = spriteId as i16;
                gSprites[spriteId].callback = Some(SpriteCB_Rival);
                gSprites[spriteId].anims = sAnims_Rival.as_ptr().cast_mut();
            } else {
                LoadCompressedSpriteSheet(gSpriteSheet_CreditsMay.as_ptr().cast_mut());
                LoadCompressedSpriteSheet(gSpriteSheet_CreditsRivalBrendan.as_ptr().cast_mut());
                LoadCompressedSpriteSheet(gSpriteSheet_CreditsBicycle.as_ptr().cast_mut());
                LoadSpritePalettes(gSpritePalettes_Credits.as_ptr().cast_mut());
                spriteId = CreateIntroMaySprite(120, 46);
                gTasks[taskId].data[5] = spriteId as i16;
                gSprites[spriteId].callback = Some(SpriteCB_Player);
                gSprites[spriteId].anims = sAnims_Player.as_ptr().cast_mut();
                spriteId = CreateIntroBrendanSprite(272, 46);
                gTasks[taskId].data[6] = spriteId as i16;
                gSprites[spriteId].callback = Some(SpriteCB_Rival);
                gSprites[spriteId].anims = sAnims_Rival.as_ptr().cast_mut();
            }
            gMain.state += 1;
        }
        3 => {
            SetBikeScene(scene, taskId);
            SetCreditsSceneBgCnt(scene);
            gMain.state = 0;
            return TRUE;
        }
        _ => {
            SetGpuReg(0x0, 0);
            SetGpuReg(REG_OFFSET_BG3HOFS, 8);
            SetGpuReg(REG_OFFSET_BG3VOFS, 0);
            SetGpuReg(REG_OFFSET_BG2HOFS, 0);
            SetGpuReg(REG_OFFSET_BG2VOFS, 0);
            SetGpuReg(REG_OFFSET_BG1HOFS, 0);
            SetGpuReg(REG_OFFSET_BG1VOFS, 0);
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            ResetSpriteData();
            FreeAllSpritePalettes();
            gMain.state = 1;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ResetCreditsTasks(taskId: u8) {
    if gTasks[taskId].data[0] != 0 {
        DestroyTask(gTasks[taskId].data[0] as u8);
        gTasks[taskId].data[0] = 0;
    }
    if gTasks[taskId].data[1] != 0 {
        DestroyTask(gTasks[taskId].data[1] as u8);
        gTasks[taskId].data[1] = 0;
    }
    if gTasks[taskId].data[2] != 0 {
        DestroyTask(gTasks[taskId].data[2] as u8);
        gTasks[taskId].data[2] = 0;
    }
    if gTasks[taskId].data[3] != 0 {
        DestroyTask(gTasks[taskId].data[3] as u8);
        gTasks[taskId].data[3] = 0;
    }
    gIntroCredits_MovingSceneryState = INTROCRED_SCENERY_DESTROY;
}
pub(crate) unsafe extern "C" fn LoadTheEndScreen(
    tileOffsetLoad: u16,
    tileOffsetWrite: u16,
    palOffset: u16,
) {
    let mut baseTile: u16 = 0;
    let mut i: u16 = 0;
    LZ77UnCompVram(
        sCreditsCopyrightEnd_Gfx.as_ptr().cast_mut(),
        (VRAM + tileOffsetLoad as i32) as usize as *mut c_void,
    );
    LoadPalette(
        gIntroCopyright_Pal.as_ptr().cast_mut() as *mut c_void,
        palOffset,
        32,
    );
    baseTile = ((palOffset as i32 / 16) as u16) << 12;
    i = 0;
    while i < 1024 {
        *((VRAM + tileOffsetWrite as i32) as usize as *mut u16).at(i) = baseTile + 1;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetLetterMapTile(baseTiles: u8) -> u16 {
    let mut out: u16 = (baseTiles as u16 & 0x3F) + 80;
    if baseTiles == 0xFF {
        return 1;
    }
    if baseTiles as i32 & 128 != 0 {
        out |= 2048;
    }
    if baseTiles as i32 & 64 != 0 {
        out |= 1024;
    }
    return out;
}
pub(crate) unsafe extern "C" fn DrawLetterMapTiles(
    baseTiles: *mut u8,
    baseX: u8,
    baseY: u8,
    offset: u16,
    palette: u16,
) {
    let mut y: u8 = 0;
    let mut x: u8 = 0;
    let mut tileOffset: u16 = ((palette as i32 / 16) as u16) << 12;
    y = 0;
    while y < 5 {
        x = 0;
        while x < 3 {
            *((VRAM + offset as i32 + (baseY as i32 + y as i32) * 64) as usize as *mut u16)
                .at(baseX as i32 + x as i32) =
                tileOffset + GetLetterMapTile(*baseTiles.at(y as i32 * 3 + x as i32));
            x += 1;
        }
        y += 1;
    }
}
pub(crate) unsafe extern "C" fn DrawTheEnd(offset: u16, palette: u16) {
    let mut pos: u16 = 0;
    let mut baseTile: u16 = ((palette as i32 / 16) as u16) << 12;
    pos = 0;
    while pos < 1024 {
        *((VRAM + offset as i32) as usize as *mut u16).at(pos) = baseTile + 1;
        pos += 1;
    }
    DrawLetterMapTiles(
        sTheEnd_LetterMap_T.as_ptr().cast_mut(),
        3,
        7,
        offset,
        palette,
    );
    DrawLetterMapTiles(
        sTheEnd_LetterMap_H.as_ptr().cast_mut(),
        7,
        7,
        offset,
        palette,
    );
    DrawLetterMapTiles(
        sTheEnd_LetterMap_E.as_ptr().cast_mut(),
        11,
        7,
        offset,
        palette,
    );
    DrawLetterMapTiles(
        sTheEnd_LetterMap_E.as_ptr().cast_mut(),
        16,
        7,
        offset,
        palette,
    );
    DrawLetterMapTiles(
        sTheEnd_LetterMap_N.as_ptr().cast_mut(),
        20,
        7,
        offset,
        palette,
    );
    DrawLetterMapTiles(
        sTheEnd_LetterMap_D.as_ptr().cast_mut(),
        24,
        7,
        offset,
        palette,
    );
}
pub(crate) unsafe extern "C" fn SpriteCB_Player(sprite: *mut Sprite) {
    if gIntroCredits_MovingSceneryState != INTROCRED_SCENERY_NORMAL {
        DestroySprite(sprite);
        return;
    }
    match (*sprite).data[0] {
        0 => {
            StartSpriteAnimIfDifferent(sprite, 0);
        }
        1 => {
            StartSpriteAnimIfDifferent(sprite, 1);
            if (*sprite).x > -32 {
                (*sprite).x -= 1;
            }
        }
        2 => {
            StartSpriteAnimIfDifferent(sprite, 2);
        }
        3 => {
            StartSpriteAnimIfDifferent(sprite, 3);
        }
        4 => {
            StartSpriteAnimIfDifferent(sprite, 0);
            if (*sprite).x > 120 {
                (*sprite).x -= 1;
            }
        }
        5 => {
            StartSpriteAnimIfDifferent(sprite, 0);
            if (*sprite).x > -32 {
                (*sprite).x -= 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Rival(sprite: *mut Sprite) {
    if gIntroCredits_MovingSceneryState != INTROCRED_SCENERY_NORMAL {
        DestroySprite(sprite);
        return;
    }
    match (*sprite).data[0] {
        0 => {
            (*sprite).y2 = 0;
            StartSpriteAnimIfDifferent(sprite, 0);
        }
        1 => {
            if (*sprite).x > 200 {
                StartSpriteAnimIfDifferent(sprite, 1);
            } else {
                StartSpriteAnimIfDifferent(sprite, 2);
            }
            if (*sprite).x > -32 {
                (*sprite).x -= 2;
            }
            (*sprite).y2 = -gIntroCredits_MovingSceneryVOffset;
        }
        2 => {
            (*sprite).data[7] += 1;
            StartSpriteAnimIfDifferent(sprite, 0);
            if (*sprite).data[7] as i32 & 3 == 0 {
                (*sprite).x += 1;
            }
        }
        3 => {
            StartSpriteAnimIfDifferent(sprite, 0);
            if (*sprite).x > -32 {
                (*sprite).x -= 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CreditsMon(sprite: *mut Sprite) {
    if gIntroCredits_MovingSceneryState != INTROCRED_SCENERY_NORMAL {
        FreeAndDestroyMonPicSprite((*sprite).data[6] as u16);
        return;
    }
    (*sprite).data[7] += 1;
    match (*sprite).data[0] {
        1 => {
            if (*sprite).data[2] < 256 {
                (*sprite).data[2] += 8;
                SetOamMatrix(
                    (*sprite).data[1] as u8,
                    div_i32(0x10000, (*sprite).data[2] as i32) as u16,
                    0,
                    0,
                    div_i32(0x10000, (*sprite).data[2] as i32) as u16,
                );
            } else {
                (*sprite).data[0] += 1;
            }
            match (*sprite).data[1] {
                1 => {
                    if (*sprite).data[7] as i32 & 3 == 0 {
                        (*sprite).y += 1;
                    }
                    (*sprite).x -= 2;
                }
                2 => {}
                3 => {
                    if (*sprite).data[7] as i32 & 3 == 0 {
                        (*sprite).y += 1;
                    }
                    (*sprite).x += 2;
                }
                _ => {}
            }
        }
        2 => {
            if (*sprite).data[3] != 0 {
                (*sprite).data[3] -= 1;
            } else {
                SetGpuReg(REG_OFFSET_BLDCNT, 3904);
                SetGpuReg(REG_OFFSET_BLDALPHA, 16);
                (*sprite).oam.set_objMode(ST_OAM_OBJ_BLEND);
                (*sprite).data[3] = 16;
                (*sprite).data[0] += 1;
            }
        }
        3 => {
            if (*sprite).data[3] != 0 {
                let mut data3: i32 = 0;
                (*sprite).data[3] -= 1;
                data3 = 16 - (*sprite).data[3] as i32;
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    ((data3 as u16) << 8) + (*sprite).data[3] as u16,
                );
            } else {
                (*sprite).set_invisible(TRUE as u16);
                (*sprite).data[0] = 9;
            }
        }
        9 => {
            (*sprite).data[0] += 1;
        }
        10 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            FreeAndDestroyMonPicSprite((*sprite).data[6] as u16);
        }
        _ => {
            (*sprite).oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
            (*sprite).oam.set_matrixNum((*sprite).data[1] as u32);
            (*sprite).data[2] = 16;
            SetOamMatrix(
                (*sprite).data[1] as u8,
                div_i32(0x10000, (*sprite).data[2] as i32) as u16,
                0,
                0,
                div_i32(0x10000, (*sprite).data[2] as i32) as u16,
            );
            (*sprite).set_invisible(FALSE as u16);
            (*sprite).data[0] = 1;
        }
    }
}
pub(crate) unsafe extern "C" fn CreateCreditsMonSprite(
    nationalDexNum: u16,
    x: i16,
    y: i16,
    position: u16,
) -> u8 {
    let mut monSpriteId: u8 = 0;
    let mut bgSpriteId: u8 = 0;
    monSpriteId = CreateMonSpriteFromNationalDexNumber(nationalDexNum, x, y, position) as u8;
    gSprites[monSpriteId].oam.set_priority(1);
    gSprites[monSpriteId].data[1] = position as i16 + 1;
    gSprites[monSpriteId].set_invisible(TRUE as u16);
    gSprites[monSpriteId].callback = Some(SpriteCB_CreditsMon);
    gSprites[monSpriteId].data[6] = monSpriteId as i16;
    bgSpriteId = CreateSprite(
        (&raw const *sSpriteTemplate_CreditsMonBg).cast_mut(),
        gSprites[monSpriteId].x,
        gSprites[monSpriteId].y,
        1,
    );
    gSprites[bgSpriteId].data[0] = monSpriteId as i16;
    StartSpriteAnimIfDifferent(&raw mut gSprites[bgSpriteId], position as u8);
    return monSpriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_CreditsMonBg(sprite: *mut Sprite) {
    if gSprites[(*sprite).data[0]].data[0] == 10
        || gIntroCredits_MovingSceneryState != INTROCRED_SCENERY_NORMAL
    {
        DestroySprite(sprite);
        return;
    }
    (*sprite).set_invisible(gSprites[(*sprite).data[0]].invisible());
    (*sprite)
        .oam
        .set_objMode(gSprites[(*sprite).data[0]].oam.objMode());
    (*sprite)
        .oam
        .set_affineMode(gSprites[(*sprite).data[0]].oam.affineMode());
    (*sprite)
        .oam
        .set_matrixNum(gSprites[(*sprite).data[0]].oam.matrixNum());
    (*sprite).x = gSprites[(*sprite).data[0]].x;
    (*sprite).y = gSprites[(*sprite).data[0]].y;
}
pub(crate) unsafe extern "C" fn DeterminePokemonToShow() {
    let mut starter: u16 = SpeciesToNationalPokedexNum(GetStarterPokemon(VarGet(VAR_STARTER_MON)));
    let mut page: u16 = 0;
    let mut dexNum: u16 = 0;
    let mut j: u16 = 0;
    dexNum = 1;
    j = 0;
    while dexNum < NATIONAL_DEX_DEOXYS {
        if GetSetPokedexFlag(dexNum, FLAG_GET_CAUGHT) != 0 {
            (*sCreditsData).caughtMonIds[j] = dexNum;
            j += 1;
        }
        dexNum += 1;
    }
    dexNum = j;
    while dexNum < NATIONAL_DEX_DEOXYS {
        (*sCreditsData).caughtMonIds[dexNum] = NATIONAL_DEX_NONE;
        dexNum += 1;
    }
    (*sCreditsData).numCaughtMon = j;
    if (*sCreditsData).numCaughtMon < NUM_MON_SLIDES {
        (*sCreditsData).numMonToShow = j;
    } else {
        (*sCreditsData).numMonToShow = NUM_MON_SLIDES;
    }
    j = 0;
    loop {
        page = rem_i32(Random() as i32, (*sCreditsData).numCaughtMon as i32) as u16;
        (*sCreditsData).monToShow[j] = (*sCreditsData).caughtMonIds[page];
        j += 1;
        (*sCreditsData).caughtMonIds[page] = 0;
        (*sCreditsData).numCaughtMon -= 1;
        if page != (*sCreditsData).numCaughtMon {
            (*sCreditsData).caughtMonIds[page] =
                (*sCreditsData).caughtMonIds[(*sCreditsData).numCaughtMon];
            (*sCreditsData).caughtMonIds[(*sCreditsData).numCaughtMon] = 0;
        }
        if !((*sCreditsData).numCaughtMon != 0 && j < NUM_MON_SLIDES) {
            break;
        }
    }
    if (*sCreditsData).numMonToShow < NUM_MON_SLIDES {
        j = (*sCreditsData).numMonToShow;
        page = 0;
        while j < NUM_MON_SLIDES {
            (*sCreditsData).monToShow[j] = (*sCreditsData).monToShow[page];
            page += 1;
            if page == (*sCreditsData).numMonToShow {
                page = 0;
            }
            j += 1;
        }
        (*sCreditsData).monToShow[70] = starter;
    } else {
        dexNum = 0;
        while (*sCreditsData).monToShow[dexNum] != starter && dexNum < NUM_MON_SLIDES {
            dexNum += 1;
        }
        if (dexNum as i32) < (*sCreditsData).numMonToShow as i32 - 1 {
            (*sCreditsData).monToShow[dexNum] = (*sCreditsData).monToShow[70];
            (*sCreditsData).monToShow[70] = starter;
        } else {
            (*sCreditsData).monToShow[70] = starter;
        }
    }
    (*sCreditsData).numMonToShow = NUM_MON_SLIDES;
}
