//! Translated from `src/credits.c` by tools/rustport/c2rs.py.
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
    clippy::missing_transmute_annotations,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::bg::{ResetBgsAndClearDma3BusyFlags, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::VarGet;
use crate::gpu_regs::{EnableInterrupts, SetGpuReg};
use crate::intro_credits_graphics::{
    CreateBicycleBgAnimationTask, CreateIntroBrendanSprite, CreateIntroMaySprite,
    CycleSceneryPalette, LoadCreditsSceneGraphics, SetCreditsSceneBgCnt,
    gIntroCredits_MovingSceneryState, gIntroCredits_MovingSceneryVBase,
    gIntroCredits_MovingSceneryVOffset,
};
use crate::load_save::gSaveBlock2Ptr;
use crate::m4a::m4aSongNumStart;
use crate::menu::AddTextPrinterParameterized4;
use crate::palette::{
    BeginNormalPaletteFade, LoadPalette, ResetPaletteFade, TransferPlttBuffer, UpdatePaletteFade,
    gPaletteFade,
};
use crate::pokedex::{CreateMonSpriteFromNationalDexNumber, GetSetPokedexFlag};
use crate::pokemon::SpeciesToNationalPokedexNum;
use crate::random::Random;
use crate::sound::FadeOutBGM;
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData, SetOamMatrix, gReservedSpritePaletteCount,
};
use crate::starter_choose::GetStarterPokemon;
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{gTasks, task_func, task_get, task_set, task_set_func};
use crate::text::DeactivateAllTextPrinters;
use crate::trainer_pokemon_sprites::{FreeAndDestroyMonPicSprite, ResetAllPicSprites};
use crate::trig::Sin;
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{
    CopyWindowToVram, FillWindowPixelBuffer, FreeAllWindowBuffers, PutWindowTilemap,
};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
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
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `GetStringCenterAlignXOffsetWithLetterSpacing` with this module's view of its types.
#[inline]
unsafe fn GetStringCenterAlignXOffsetWithLetterSpacing(
    a0: i32,
    a1: *mut u8,
    a2: i32,
    a3: i32,
) -> i32 {
    unsafe {
        crate::international_string_util::GetStringCenterAlignXOffsetWithLetterSpacing(
            a0, a1 as _, a2, a3,
        )
    }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
unsafe fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `InitHeap` with this module's view of its types.
#[inline]
unsafe fn InitHeap(a0: *mut c_void, a1: u32) {
    unsafe {
        crate::malloc::InitHeap(a0 as _, a1);
    }
}
/// `InitWindows` with this module's view of its types.
#[inline]
unsafe fn InitWindows(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::InitWindows(a0 as _) }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
}
/// `LoadSpritePalettes` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalettes(a0: *mut SpritePalette) {
    unsafe {
        crate::sprite::LoadSpritePalettes(a0 as _);
    }
}
/// `LoadSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16 {
    unsafe { crate::sprite::LoadSpriteSheet(a0 as _) }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
/// `StartSpriteAnimIfDifferent` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnimIfDifferent(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnimIfDifferent(a0 as _, a1);
    }
}
// The C's names for task and sprite data slots.
const sMonSpriteId: usize = 0;
const sState: usize = 0;
const tState: usize = 0;
const tTaskId_BgScenery: usize = 0;
const sPosition: usize = 1;
const tMainTaskId: usize = 1;
const tTaskId_BikeScene: usize = 1;
const tCurrentPage: usize = 2;
const tPlayer: usize = 2;
const tTaskId_SceneryPal: usize = 2;
const tRival: usize = 3;
const tTaskId_ShowMons: usize = 3;
const tEndCredits: usize = 4;
const tPlayerSpriteId: usize = 5;
const tSinIdx: usize = 5;
const sSpriteId: usize = 6;
const tRivalSpriteId: usize = 6;
const tSceneNum: usize = 7;
const tNextMode: usize = 11;
const tTheEndDelay: usize = 12;
const tCurrentMode: usize = 13;
const tPrintedPage: usize = 14;
const tTaskId_UpdatePage: usize = 15;
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

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `GetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn GetBgTilemapBuffer(a0: u8) -> *mut c_void {
    unsafe { crate::bg::GetBgTilemapBuffer(a0) as *mut c_void }
}
/// `LZ77UnCompVram` with this module's view of its types.
#[inline]
unsafe fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompVram(a0 as _, a1 as _);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}
/// `SoftReset` with this module's view of its types.
#[inline]
unsafe fn SoftReset(a0: u32) {
    unsafe {
        crate::syscall::SoftReset(a0);
    }
}

pub(crate) unsafe fn VBlankCB_Credits() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe fn CB2_Credits() {
    RunTasks();
    AnimateSprites();
    if gMain.heldKeys as i32 & B_BUTTON != 0
        && gHasHallOfFameRecords != 0
        && task_func(sSavedTaskId) == Some(Task_CreditsMain as unsafe fn(u8))
    {
        VBlankCB_Credits();
        RunTasks();
        AnimateSprites();
        sUsedSpeedUp = TRUE;
    }
    BuildOamBuffer();
    UpdatePaletteFade();
}
unsafe fn InitCreditsBgsAndWindows() {
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
unsafe fn FreeCreditsBgsAndWindows() {
    FreeAllWindowBuffers();
    let ptr: *mut c_void = GetBgTilemapBuffer(0);
    if !ptr.is_null() {
        Free(ptr);
    }
}
unsafe fn PrintCreditsText(string: *mut u8, y: u8, isTitle: u8) {
    let mut color: CArray<u8, 3> = zeroed();
    color[0] = 0x0;
    if isTitle == TRUE {
        color[1] = TEXT_COLOR_LIGHT_GRAY;
        color[2] = TEXT_COLOR_RED;
    } else {
        color[1] = 0x1;
        color[2] = 0x2;
    }
    let x: u8 = GetStringCenterAlignXOffsetWithLetterSpacing(
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
pub unsafe fn CB2_StartCreditsSequence() {
    ResetGpuAndVram();
    SetVBlankCallback(None);
    InitHeap(
        (*(&raw const crate::malloc::gHeap)
            .cast::<CArray<u8, 114688>>()
            .cast_mut())
        .as_mut_ptr() as *mut c_void,
        HEAP_SIZE,
    );
    ResetPaletteFade();
    ResetTasks();
    InitCreditsBgsAndWindows();
    let taskId: u8 = CreateTask(Some(Task_WaitPaletteFade), 0);
    task_set(taskId, tEndCredits, FALSE as i16);
    task_set(taskId, tSceneNum, SCENE_OCEAN_MORNING as i16);
    task_set(taskId, tNextMode, MODE_NONE);
    task_set(taskId, tCurrentMode, MODE_BIKE_SCENE);
    loop {
        if LoadBikeScene(SCENE_OCEAN_MORNING, taskId) != 0 {
            break;
        }
    }
    let bikeTaskId: i16 = task_get(taskId, 1);
    task_set(bikeTaskId, tState, 40);
    SetGpuReg(REG_OFFSET_BG0VOFS, 0xFFFC);
    let pageTaskId: u8 = CreateTask(Some(Task_UpdatePage), 0);
    task_set(pageTaskId, 1, taskId as i16);
    task_set(taskId, tTaskId_UpdatePage, pageTaskId as i16);
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
pub(crate) unsafe fn Task_WaitPaletteFade(taskId: u8) {
    if gPaletteFade.active() == 0 {
        task_set_func(taskId, Some(Task_CreditsMain));
    }
}
pub(crate) unsafe fn Task_CreditsMain(taskId: u8) {
    if task_get(taskId, tEndCredits) != 0 {
        let bikeTaskId: i16 = task_get(taskId, tTaskId_BikeScene);
        task_set(bikeTaskId, tState, 30);
        task_set(taskId, tTheEndDelay, 256);
        task_set_func(taskId, Some(Task_CreditsTheEnd1));
        return;
    }
    sUnkVar = 0;
    let mode: u16 = task_get(taskId, tNextMode) as u16;
    if task_get(taskId, tNextMode) == MODE_BIKE_SCENE {
        task_set(taskId, tCurrentMode, mode as i16);
        task_set(taskId, tNextMode, MODE_NONE);
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        task_set_func(taskId, Some(Task_ReadyBikeScene));
    } else if task_get(taskId, tNextMode) == MODE_SHOW_MONS {
        task_set(taskId, tCurrentMode, mode as i16);
        task_set(taskId, tNextMode, MODE_NONE);
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        task_set_func(taskId, Some(Task_ReadyShowMons));
    }
}
pub(crate) unsafe fn Task_ReadyBikeScene(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetGpuReg(0x0, 0);
        ResetCreditsTasks(taskId);
        task_set_func(taskId, Some(Task_SetBikeScene));
    }
}
pub(crate) unsafe fn Task_SetBikeScene(taskId: u8) {
    SetVBlankCallback(None);
    if LoadBikeScene(task_get(taskId, tSceneNum) as u8, taskId) != 0 {
        BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
        EnableInterrupts(INTR_FLAG_VBLANK);
        SetVBlankCallback(Some(VBlankCB_Credits));
        task_set_func(taskId, Some(Task_WaitPaletteFade));
    }
}
pub(crate) unsafe fn Task_ReadyShowMons(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetGpuReg(0x0, 0);
        ResetCreditsTasks(taskId);
        task_set_func(taskId, Some(Task_LoadShowMons));
    }
}
pub(crate) unsafe fn Task_LoadShowMons(taskId: u8) {
    'l1: {
        match gMain.state {
            1 => {
                task_set(
                    taskId,
                    tTaskId_ShowMons,
                    CreateTask(Some(Task_ShowMons), 0) as i16,
                );
                (*gTasks.as_ptr())[task_get(taskId, tTaskId_ShowMons)].data[tState] = 1;
                (*gTasks.as_ptr())[task_get(taskId, tTaskId_ShowMons)].data[tMainTaskId] =
                    taskId as i16;
                (*gTasks.as_ptr())[task_get(taskId, tTaskId_ShowMons)].data[2] =
                    task_get(taskId, tSceneNum);
                BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
                SetGpuReg(REG_OFFSET_BG3HOFS, 0);
                SetGpuReg(REG_OFFSET_BG3VOFS, 32);
                SetGpuReg(REG_OFFSET_BG3CNT, 1795);
                SetGpuReg(0x0, 6464);
                gMain.state = 0;
                gIntroCredits_MovingSceneryState = INTROCRED_SCENERY_NORMAL;
                task_set_func(taskId, Some(Task_WaitPaletteFade));
            }
            _ => {
                ResetSpriteData();
                ResetAllPicSprites();
                FreeAllSpritePalettes();
                gReservedSpritePaletteCount = 8;
                LZ77UnCompVram(
                    (*(&raw const crate::data::starter_choose::gBirchBagGrass_Gfx)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    VRAM as usize as *mut c_void,
                );
                LZ77UnCompVram(
                    (*(&raw const crate::data::starter_choose::gBirchGrassTilemap)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x6003800_usize as *mut c_void,
                );
                LoadPalette(
                    (*(&raw const crate::data::starter_choose::gBirchBagGrass_Pal)
                        .cast::<CArray<u16, 0>>())
                    .as_ptr()
                    .cast_mut()
                    .at(1) as *mut c_void,
                    1,
                    62,
                );
                for i in 0..MON_PIC_SIZE {
                    (*(&raw const crate::decompress::gDecompressionBuffer)
                        .cast::<CArray<u8, 16384>>()
                        .cast_mut())[i] = 0x11;
                }
                let mut i: u16 = 0;
                while i < MON_PIC_SIZE {
                    *(*(&raw const crate::decompress::gDecompressionBuffer)
                        .cast::<CArray<u8, 16384>>()
                        .cast_mut())
                    .as_mut_ptr()
                    .at(2048)
                    .at(i) = 0x22;
                    i += 1;
                }
                for i in 0..MON_PIC_SIZE {
                    *(*(&raw const crate::decompress::gDecompressionBuffer)
                        .cast::<CArray<u8, 16384>>()
                        .cast_mut())
                    .as_mut_ptr()
                    .at(4096)
                    .at(i) = 0x33;
                }
                let temp: *mut u16 =
                    &raw mut (*(&raw const crate::decompress::gDecompressionBuffer)
                        .cast::<CArray<u8, 16384>>()
                        .cast_mut())[6144] as *mut u16;
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
pub(crate) unsafe fn Task_CreditsTheEnd1(taskId: u8) {
    if task_get(taskId, tTheEndDelay) != 0 {
        task_set(taskId, tTheEndDelay, task_get(taskId, tTheEndDelay) - 1);
        return;
    }
    BeginNormalPaletteFade(PALETTES_ALL, 12, 0, 16, 0);
    task_set_func(taskId, Some(Task_CreditsTheEnd2));
}
pub(crate) unsafe fn Task_CreditsTheEnd2(taskId: u8) {
    if gPaletteFade.active() == 0 {
        ResetCreditsTasks(taskId);
        task_set_func(taskId, Some(Task_CreditsTheEnd3));
    }
}
pub(crate) unsafe fn Task_CreditsTheEnd3(taskId: u8) {
    ResetGpuAndVram();
    ResetPaletteFade();
    LoadTheEndScreen(0, 0x3800, 0);
    ResetSpriteData();
    FreeAllSpritePalettes();
    BeginNormalPaletteFade(PALETTES_ALL, 8, 16, 0, 0);
    SetGpuReg(REG_OFFSET_BG0CNT, 1792);
    EnableInterrupts(INTR_FLAG_VBLANK);
    SetGpuReg(0x0, 320);
    task_set(taskId, 0, 235);
    task_set_func(taskId, Some(Task_CreditsTheEnd4));
}
pub(crate) unsafe fn Task_CreditsTheEnd4(taskId: u8) {
    if task_get(taskId, 0) != 0 {
        task_set(taskId, 0, task_get(taskId, 0) - 1);
        return;
    }
    BeginNormalPaletteFade(PALETTES_ALL, 6, 0, 16, 0);
    task_set_func(taskId, Some(Task_CreditsTheEnd5));
}
pub(crate) unsafe fn Task_CreditsTheEnd5(taskId: u8) {
    if gPaletteFade.active() == 0 {
        DrawTheEnd(0x3800, 0);
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0, 0);
        task_set(taskId, 0, 7200);
        task_set_func(taskId, Some(Task_CreditsTheEnd6));
    }
}
pub(crate) unsafe fn Task_CreditsTheEnd6(taskId: u8) {
    if gPaletteFade.active() == 0 {
        if task_get(taskId, 0) == 0 || gMain.newKeys != 0 {
            FadeOutBGM(4);
            BeginNormalPaletteFade(PALETTES_ALL, 8, 0, 16, 65535);
            task_set_func(taskId, Some(Task_CreditsSoftReset));
            return;
        }
        if task_get(taskId, 0) == 7144 {
            FadeOutBGM(8);
        }
        if task_get(taskId, 0) == 6840 {
            m4aSongNumStart(MUS_END);
        }
        task_set(taskId, 0, task_get(taskId, 0) - 1);
    }
}
pub(crate) unsafe fn Task_CreditsSoftReset(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SoftReset(RESET_ALL);
    }
}
unsafe fn ResetGpuAndVram() {
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
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
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
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
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
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(dmaRegs.at(1), 83886082_usize as *mut c_void as usize as u32);
                    volatile_write(dmaRegs.at(2), 0x810001ff);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
}
pub(crate) unsafe fn Task_UpdatePage(taskId: u8) {
    match task_get(taskId, tState) {
        1 => {
            if task_get(taskId, 3) != 0 {
                task_set(taskId, 3, task_get(taskId, 3) - 1);
                return;
            }
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        2 => {
            if (*gTasks.as_ptr())[task_get(taskId, tMainTaskId)].func
                == Some(Task_CreditsMain as unsafe fn(u8))
            {
                if task_get(taskId, tCurrentPage) < PAGE_COUNT {
                    for i in 0..ENTRIES_PER_PAGE {
                        PrintCreditsText(
                            (*sCreditsEntryPointerTable[task_get(taskId, tCurrentPage)][i]).text,
                            5 + i as u8 * 16,
                            (*sCreditsEntryPointerTable[task_get(taskId, tCurrentPage)][i]).isTitle,
                        );
                    }
                    CopyWindowToVram(0, COPYWIN_GFX);
                    task_set(taskId, tCurrentPage, task_get(taskId, tCurrentPage) + 1);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                    (*gTasks.as_ptr())[task_get(taskId, tMainTaskId)].data[tPrintedPage] =
                        TRUE as i16;
                    if (*gTasks.as_ptr())[task_get(taskId, tMainTaskId)].data[tCurrentMode]
                        == MODE_BIKE_SCENE
                    {
                        BeginNormalPaletteFade(0x300, 0, 16, 0, 12941);
                    } else {
                        BeginNormalPaletteFade(0x300, 0, 16, 0, 6503);
                    }
                    return;
                }
                task_set(taskId, tState, 10);
                return;
            }
            (*gTasks.as_ptr())[task_get(taskId, tMainTaskId)].data[tPrintedPage] = FALSE as i16;
        }
        3 => {
            if gPaletteFade.active() == 0 {
                task_set(taskId, 3, 115);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        4 => {
            if task_get(taskId, 3) != 0 {
                task_set(taskId, 3, task_get(taskId, 3) - 1);
                return;
            }
            if CheckChangeScene(
                task_get(taskId, tCurrentPage) as u8,
                task_get(taskId, tMainTaskId) as u8,
            ) != 0
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
                return;
            }
            task_set(taskId, tState, task_get(taskId, tState) + 1);
            if (*gTasks.as_ptr())[task_get(taskId, tMainTaskId)].data[tCurrentMode]
                == MODE_BIKE_SCENE
            {
                BeginNormalPaletteFade(0x300, 0, 0, 16, 12941);
            } else {
                BeginNormalPaletteFade(0x300, 0, 0, 16, 6503);
            }
        }
        5 => {
            if gPaletteFade.active() == 0 {
                FillWindowPixelBuffer(0, 0);
                CopyWindowToVram(0, COPYWIN_GFX);
                task_set(taskId, tState, 2);
            }
        }
        10 => {
            (*gTasks.as_ptr())[task_get(taskId, tMainTaskId)].data[tEndCredits] = TRUE as i16;
            DestroyTask(taskId);
            FreeCreditsBgsAndWindows();
            Free(sCreditsData as *mut c_void);
            sCreditsData = null_mut();
        }
        _ => {
            if gPaletteFade.active() == 0 {
                task_set(taskId, tState, 1);
                task_set(taskId, 3, 72);
                (*gTasks.as_ptr())[task_get(taskId, tMainTaskId)].data[tPrintedPage] = FALSE as i16;
                sUnkVar = 0;
            }
        }
    }
}
fn CheckChangeScene(page: u8, taskId: u8) -> u8 {
    if page == 6 {
        task_set(taskId, tNextMode, MODE_SHOW_MONS);
    }
    if page == 12 {
        task_set(taskId, tSceneNum, SCENE_OCEAN_SUNSET);
        task_set(taskId, tNextMode, MODE_BIKE_SCENE);
    }
    if page == 18 {
        task_set(taskId, tNextMode, MODE_SHOW_MONS);
    }
    if page == 24 {
        task_set(taskId, tSceneNum, SCENE_FOREST_RIVAL_ARRIVE as i16);
        task_set(taskId, tNextMode, MODE_BIKE_SCENE);
    }
    if page == 30 {
        task_set(taskId, tNextMode, MODE_SHOW_MONS);
    }
    if page == 36 {
        task_set(taskId, tSceneNum, SCENE_FOREST_CATCH_RIVAL);
        task_set(taskId, tNextMode, MODE_BIKE_SCENE);
    }
    if page == 42 {
        task_set(taskId, tNextMode, MODE_SHOW_MONS);
    }
    if page == 48 {
        task_set(taskId, tSceneNum, SCENE_CITY_NIGHT);
        task_set(taskId, tNextMode, MODE_BIKE_SCENE);
    }
    if task_get(taskId, tNextMode) != MODE_NONE {
        return TRUE;
    }
    FALSE
}
pub(crate) unsafe fn Task_ShowMons(taskId: u8) {
    let mut spriteId: u8 = 0;
    'l1: {
        match task_get(taskId, tState) {
            0 => {}
            1 => {
                if (*sCreditsData).nextImgPos == POS_LEFT
                    && (*gTasks.as_ptr())[task_get(taskId, tMainTaskId)].data[tPrintedPage]
                        == FALSE as i16
                {
                    break 'l1;
                }
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
            2 => {
                if (*sCreditsData).imgCounter == NUM_MON_SLIDES
                    || (*gTasks.as_ptr())[task_get(taskId, tMainTaskId)].func
                        != Some(Task_CreditsMain as unsafe fn(u8))
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
                task_set(taskId, 3, 50);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
            3 => {
                if task_get(taskId, 3) != 0 {
                    task_set(taskId, 3, task_get(taskId, 3) - 1);
                } else {
                    task_set(taskId, tState, 1);
                }
            }
            _ => {}
        }
    }
}
pub(crate) unsafe fn Task_BikeScene(taskId: u8) {
    match task_get(taskId, 0) {
        0 => {
            gIntroCredits_MovingSceneryVOffset = Sin(task_get(taskId, tSinIdx) >> 1 & 0x7F, 12);
            task_set(taskId, tSinIdx, task_get(taskId, tSinIdx) + 1);
        }
        1 => {
            if gIntroCredits_MovingSceneryVOffset != 0 {
                gIntroCredits_MovingSceneryVOffset = Sin(task_get(taskId, tSinIdx) >> 1 & 0x7F, 12);
                task_set(taskId, tSinIdx, task_get(taskId, tSinIdx) + 1);
            } else {
                gSprites[task_get(taskId, tPlayer)].data[0] = 2;
                task_set(taskId, tSinIdx, 0);
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        2 => {
            if task_get(taskId, tSinIdx) < 64 {
                task_set(taskId, tSinIdx, task_get(taskId, tSinIdx) + 1);
                gIntroCredits_MovingSceneryVOffset = Sin(task_get(taskId, tSinIdx) & 0x7F, 20);
            } else {
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        3 => {
            gSprites[task_get(taskId, tPlayer)].data[0] = 3;
            gSprites[task_get(taskId, tRival)].data[0] = 1;
            task_set(taskId, 4, 120);
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
        4 => {
            if task_get(taskId, 4) != 0 {
                task_set(taskId, 4, task_get(taskId, 4) - 1);
            } else {
                task_set(taskId, tSinIdx, 64);
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        5 => {
            if task_get(taskId, tSinIdx) > 0 {
                task_set(taskId, tSinIdx, task_get(taskId, tSinIdx) - 1);
                gIntroCredits_MovingSceneryVOffset = Sin(task_get(taskId, tSinIdx) & 0x7F, 20);
            } else {
                gSprites[task_get(taskId, tPlayer)].data[0] = 1;
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        6 => {
            task_set(taskId, 0, 50);
        }
        10 => {
            gSprites[task_get(taskId, tRival)].data[0] = 2;
            task_set(taskId, 0, 50);
        }
        20 => {
            gSprites[task_get(taskId, tPlayer)].data[0] = 4;
            task_set(taskId, 0, 50);
        }
        30 => {
            gSprites[task_get(taskId, tPlayer)].data[0] = 5;
            gSprites[task_get(taskId, tRival)].data[0] = 3;
            task_set(taskId, 0, 50);
        }
        50 => {
            task_set(taskId, 0, 0);
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_CycleSceneryPalette(taskId: u8) {
    let mut bikeTaskId: i16 = 0;
    match task_get(taskId, tState) {
        SCENE_OCEAN_SUNSET => {
            CycleSceneryPalette(0);
        }
        2 => {
            if task_get(taskId, 1) != TIMER_STOP {
                bikeTaskId = (*gTasks.as_ptr())[task_get(taskId, 2)].data[1];
                if task_get(bikeTaskId, tSinIdx) as i32 & -128 == 640 {
                    task_set(bikeTaskId, tState, 1);
                    task_set(taskId, 1, TIMER_STOP);
                }
            }
            CycleSceneryPalette(1);
        }
        SCENE_FOREST_CATCH_RIVAL => {
            if task_get(taskId, 1) != TIMER_STOP {
                if task_get(taskId, 1) == 584 {
                    (*gTasks.as_ptr())[(*gTasks.as_ptr())[task_get(taskId, 2)].data[1]].data
                        [tState] = 10;
                    task_set(taskId, 1, TIMER_STOP);
                } else {
                    task_set(taskId, 1, task_get(taskId, 1) + 1);
                }
            }
            CycleSceneryPalette(1);
        }
        SCENE_CITY_NIGHT => {
            CycleSceneryPalette(2);
        }
        _ => {
            if task_get(taskId, 1) != TIMER_STOP
                && (*gTasks.as_ptr())
                    [(*gTasks.as_ptr())[task_get(taskId, 2)].data[tTaskId_UpdatePage]]
                    .data[2]
                    == 2
            {
                (*gTasks.as_ptr())[(*gTasks.as_ptr())[task_get(taskId, 2)].data[1]].data[tState] =
                    20;
                task_set(taskId, 1, TIMER_STOP);
            }
            CycleSceneryPalette(0);
        }
    }
}
unsafe fn SetBikeScene(scene: u8, taskId: u8) {
    match scene {
        SCENE_OCEAN_MORNING => {
            gSprites[task_get(taskId, 5)].set_invisible(FALSE as u16);
            gSprites[task_get(taskId, tRivalSpriteId)].set_invisible(FALSE as u16);
            gSprites[task_get(taskId, 5)].x = 272;
            gSprites[task_get(taskId, tRivalSpriteId)].x = 272;
            gSprites[task_get(taskId, 5)].y = 46;
            gSprites[task_get(taskId, tRivalSpriteId)].y = 46;
            gSprites[task_get(taskId, 5)].data[0] = 0;
            gSprites[task_get(taskId, tRivalSpriteId)].data[0] = 0;
            task_set(
                taskId,
                0,
                CreateBicycleBgAnimationTask(0, 0x2000, 0x20, 8) as i16,
            );
        }
        1 => {
            gSprites[task_get(taskId, 5)].set_invisible(FALSE as u16);
            gSprites[task_get(taskId, tRivalSpriteId)].set_invisible(FALSE as u16);
            gSprites[task_get(taskId, 5)].x = 120;
            gSprites[task_get(taskId, tRivalSpriteId)].x = 272;
            gSprites[task_get(taskId, 5)].y = 46;
            gSprites[task_get(taskId, tRivalSpriteId)].y = 46;
            gSprites[task_get(taskId, 5)].data[0] = 0;
            gSprites[task_get(taskId, tRivalSpriteId)].data[0] = 0;
            task_set(
                taskId,
                0,
                CreateBicycleBgAnimationTask(0, 0x2000, 0x20, 8) as i16,
            );
        }
        SCENE_FOREST_RIVAL_ARRIVE => {
            gSprites[task_get(taskId, 5)].set_invisible(FALSE as u16);
            gSprites[task_get(taskId, tRivalSpriteId)].set_invisible(FALSE as u16);
            gSprites[task_get(taskId, 5)].x = 120;
            gSprites[task_get(taskId, tRivalSpriteId)].x = 272;
            gSprites[task_get(taskId, 5)].y = 46;
            gSprites[task_get(taskId, tRivalSpriteId)].y = 46;
            gSprites[task_get(taskId, 5)].data[0] = 0;
            gSprites[task_get(taskId, tRivalSpriteId)].data[0] = 0;
            task_set(
                taskId,
                0,
                CreateBicycleBgAnimationTask(1, 0x2000, 0x200, 8) as i16,
            );
        }
        3 => {
            gSprites[task_get(taskId, 5)].set_invisible(FALSE as u16);
            gSprites[task_get(taskId, tRivalSpriteId)].set_invisible(FALSE as u16);
            gSprites[task_get(taskId, 5)].x = 120;
            gSprites[task_get(taskId, tRivalSpriteId)].x = -32;
            gSprites[task_get(taskId, 5)].y = 46;
            gSprites[task_get(taskId, tRivalSpriteId)].y = 46;
            gSprites[task_get(taskId, 5)].data[0] = 0;
            gSprites[task_get(taskId, tRivalSpriteId)].data[0] = 0;
            task_set(
                taskId,
                0,
                CreateBicycleBgAnimationTask(1, 0x2000, 0x200, 8) as i16,
            );
        }
        4 => {
            gSprites[task_get(taskId, 5)].set_invisible(FALSE as u16);
            gSprites[task_get(taskId, tRivalSpriteId)].set_invisible(FALSE as u16);
            gSprites[task_get(taskId, 5)].x = 88;
            gSprites[task_get(taskId, tRivalSpriteId)].x = 152;
            gSprites[task_get(taskId, 5)].y = 46;
            gSprites[task_get(taskId, tRivalSpriteId)].y = 46;
            gSprites[task_get(taskId, 5)].data[0] = 0;
            gSprites[task_get(taskId, tRivalSpriteId)].data[0] = 0;
            task_set(
                taskId,
                0,
                CreateBicycleBgAnimationTask(2, 0x2000, 0x200, 8) as i16,
            );
        }
        _ => {}
    }
    task_set(
        taskId,
        2,
        CreateTask(Some(Task_CycleSceneryPalette), 0) as i16,
    );
    (*gTasks.as_ptr())[task_get(taskId, 2)].data[0] = scene as i16;
    (*gTasks.as_ptr())[task_get(taskId, 2)].data[1] = 0;
    (*gTasks.as_ptr())[task_get(taskId, 2)].data[2] = taskId as i16;
    task_set(taskId, 1, CreateTask(Some(Task_BikeScene), 0) as i16);
    (*gTasks.as_ptr())[task_get(taskId, 1)].data[0] = 0;
    (*gTasks.as_ptr())[task_get(taskId, 1)].data[1] = taskId as i16;
    (*gTasks.as_ptr())[task_get(taskId, 1)].data[2] = task_get(taskId, 5);
    (*gTasks.as_ptr())[task_get(taskId, 1)].data[tRival] = task_get(taskId, tRivalSpriteId);
    (*gTasks.as_ptr())[task_get(taskId, 1)].data[4] = 0;
    if scene == SCENE_FOREST_RIVAL_ARRIVE {
        (*gTasks.as_ptr())[task_get(taskId, 1)].data[5] = 69;
    }
}
unsafe fn LoadBikeScene(scene: u8, taskId: u8) -> u8 {
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
                LoadCompressedSpriteSheet((*(&raw const crate::data::intro_credits_graphics::gSpriteSheet_CreditsBrendan).cast::<CArray<CompressedSpriteSheet, 0>>()).as_ptr().cast_mut());
                LoadCompressedSpriteSheet((*(&raw const crate::data::intro_credits_graphics::gSpriteSheet_CreditsRivalMay).cast::<CArray<CompressedSpriteSheet, 0>>()).as_ptr().cast_mut());
                LoadCompressedSpriteSheet((*(&raw const crate::data::intro_credits_graphics::gSpriteSheet_CreditsBicycle).cast::<CArray<CompressedSpriteSheet, 0>>()).as_ptr().cast_mut());
                LoadSpritePalettes(
                    (*(&raw const crate::data::intro_credits_graphics::gSpritePalettes_Credits)
                        .cast::<CArray<SpritePalette, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                spriteId = CreateIntroBrendanSprite(120, 46);
                task_set(taskId, tPlayerSpriteId, spriteId as i16);
                gSprites[spriteId].callback = Some(SpriteCB_Player);
                gSprites[spriteId].anims = sAnims_Player.as_ptr().cast_mut();
                spriteId = CreateIntroMaySprite(272, 46);
                task_set(taskId, tRivalSpriteId, spriteId as i16);
                gSprites[spriteId].callback = Some(SpriteCB_Rival);
                gSprites[spriteId].anims = sAnims_Rival.as_ptr().cast_mut();
            } else {
                LoadCompressedSpriteSheet(
                    (*(&raw const crate::data::intro_credits_graphics::gSpriteSheet_CreditsMay)
                        .cast::<CArray<CompressedSpriteSheet, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                LoadCompressedSpriteSheet((*(&raw const crate::data::intro_credits_graphics::gSpriteSheet_CreditsRivalBrendan).cast::<CArray<CompressedSpriteSheet, 0>>()).as_ptr().cast_mut());
                LoadCompressedSpriteSheet((*(&raw const crate::data::intro_credits_graphics::gSpriteSheet_CreditsBicycle).cast::<CArray<CompressedSpriteSheet, 0>>()).as_ptr().cast_mut());
                LoadSpritePalettes(
                    (*(&raw const crate::data::intro_credits_graphics::gSpritePalettes_Credits)
                        .cast::<CArray<SpritePalette, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                spriteId = CreateIntroMaySprite(120, 46);
                task_set(taskId, tPlayerSpriteId, spriteId as i16);
                gSprites[spriteId].callback = Some(SpriteCB_Player);
                gSprites[spriteId].anims = sAnims_Player.as_ptr().cast_mut();
                spriteId = CreateIntroBrendanSprite(272, 46);
                task_set(taskId, tRivalSpriteId, spriteId as i16);
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
    FALSE
}
unsafe fn ResetCreditsTasks(taskId: u8) {
    if task_get(taskId, tTaskId_BgScenery) != 0 {
        DestroyTask(task_get(taskId, tTaskId_BgScenery) as u8);
        task_set(taskId, tTaskId_BgScenery, 0);
    }
    if task_get(taskId, tTaskId_BikeScene) != 0 {
        DestroyTask(task_get(taskId, tTaskId_BikeScene) as u8);
        task_set(taskId, tTaskId_BikeScene, 0);
    }
    if task_get(taskId, tTaskId_SceneryPal) != 0 {
        DestroyTask(task_get(taskId, tTaskId_SceneryPal) as u8);
        task_set(taskId, tTaskId_SceneryPal, 0);
    }
    if task_get(taskId, tTaskId_ShowMons) != 0 {
        DestroyTask(task_get(taskId, tTaskId_ShowMons) as u8);
        task_set(taskId, tTaskId_ShowMons, 0);
    }
    gIntroCredits_MovingSceneryState = INTROCRED_SCENERY_DESTROY;
}
unsafe fn LoadTheEndScreen(tileOffsetLoad: u16, tileOffsetWrite: u16, palOffset: u16) {
    LZ77UnCompVram(
        sCreditsCopyrightEnd_Gfx.as_ptr().cast_mut(),
        (VRAM + tileOffsetLoad as i32) as usize as *mut c_void,
    );
    LoadPalette(
        (*(&raw const crate::data::graphics::gIntroCopyright_Pal).cast::<CArray<u16, 16>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        palOffset,
        32,
    );
    let baseTile: u16 = ((palOffset as i32 / 16) as u16) << 12;
    for i in 0..1024u16 {
        *((VRAM + tileOffsetWrite as i32) as usize as *mut u16).at(i) = baseTile + 1;
    }
}
unsafe fn GetLetterMapTile(baseTiles: u8) -> u16 {
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
    out
}
unsafe fn DrawLetterMapTiles(baseTiles: *mut u8, baseX: u8, baseY: u8, offset: u16, palette: u16) {
    let tileOffset: u16 = ((palette as i32 / 16) as u16) << 12;
    for y in 0..5u8 {
        for x in 0..3u8 {
            *((VRAM + offset as i32 + (baseY as i32 + y as i32) * 64) as usize as *mut u16)
                .at(baseX as i32 + x as i32) =
                tileOffset + GetLetterMapTile(*baseTiles.at(y as i32 * 3 + x as i32));
        }
    }
}
unsafe fn DrawTheEnd(offset: u16, palette: u16) {
    let baseTile: u16 = ((palette as i32 / 16) as u16) << 12;
    for pos in 0..1024u16 {
        *((VRAM + offset as i32) as usize as *mut u16).at(pos) = baseTile + 1;
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
pub(crate) unsafe fn SpriteCB_Player(sprite: *mut Sprite) {
    if gIntroCredits_MovingSceneryState != INTROCRED_SCENERY_NORMAL {
        DestroySprite(sprite);
        return;
    }
    match (*sprite).data[sState] {
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
pub(crate) unsafe fn SpriteCB_Rival(sprite: *mut Sprite) {
    if gIntroCredits_MovingSceneryState != INTROCRED_SCENERY_NORMAL {
        DestroySprite(sprite);
        return;
    }
    match (*sprite).data[sState] {
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
pub(crate) unsafe fn SpriteCB_CreditsMon(sprite: *mut Sprite) {
    if gIntroCredits_MovingSceneryState != INTROCRED_SCENERY_NORMAL {
        FreeAndDestroyMonPicSprite((*sprite).data[6] as u16);
        return;
    }
    (*sprite).data[7] += 1;
    match (*sprite).data[sState] {
        1 => {
            if (*sprite).data[2] < 256 {
                (*sprite).data[2] += 8;
                SetOamMatrix(
                    (*sprite).data[sPosition] as u8,
                    div_i32(0x10000, (*sprite).data[2] as i32) as u16,
                    0,
                    0,
                    div_i32(0x10000, (*sprite).data[2] as i32) as u16,
                );
            } else {
                (*sprite).data[sState] += 1;
            }
            match (*sprite).data[sPosition] {
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
                (*sprite).data[sState] += 1;
            }
        }
        3 => {
            if (*sprite).data[3] != 0 {
                (*sprite).data[3] -= 1;
                let data3: i32 = 16 - (*sprite).data[3] as i32;
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    ((data3 as u16) << 8) + (*sprite).data[3] as u16,
                );
            } else {
                (*sprite).set_invisible(TRUE as u16);
                (*sprite).data[sState] = 9;
            }
        }
        9 => {
            (*sprite).data[sState] += 1;
        }
        10 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            FreeAndDestroyMonPicSprite((*sprite).data[6] as u16);
        }
        _ => {
            (*sprite).oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
            (*sprite)
                .oam
                .set_matrixNum((*sprite).data[sPosition] as u32);
            (*sprite).data[2] = 16;
            SetOamMatrix(
                (*sprite).data[sPosition] as u8,
                div_i32(0x10000, (*sprite).data[2] as i32) as u16,
                0,
                0,
                div_i32(0x10000, (*sprite).data[2] as i32) as u16,
            );
            (*sprite).set_invisible(FALSE as u16);
            (*sprite).data[sState] = 1;
        }
    }
}
unsafe fn CreateCreditsMonSprite(nationalDexNum: u16, x: i16, y: i16, position: u16) -> u8 {
    let monSpriteId: u8 =
        CreateMonSpriteFromNationalDexNumber(nationalDexNum, x, y, position) as u8;
    gSprites[monSpriteId].oam.set_priority(1);
    gSprites[monSpriteId].data[sPosition] = position as i16 + 1;
    gSprites[monSpriteId].set_invisible(TRUE as u16);
    gSprites[monSpriteId].callback = Some(SpriteCB_CreditsMon);
    gSprites[monSpriteId].data[sSpriteId] = monSpriteId as i16;
    let bgSpriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_CreditsMonBg).cast_mut(),
        gSprites[monSpriteId].x,
        gSprites[monSpriteId].y,
        1,
    );
    gSprites[bgSpriteId].data[sMonSpriteId] = monSpriteId as i16;
    StartSpriteAnimIfDifferent(&raw mut gSprites[bgSpriteId], position as u8);
    monSpriteId
}
pub(crate) unsafe fn SpriteCB_CreditsMonBg(sprite: *mut Sprite) {
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
unsafe fn DeterminePokemonToShow() {
    let starter: u16 = SpeciesToNationalPokedexNum(GetStarterPokemon(VarGet(VAR_STARTER_MON)));
    let mut page: u16 = 0;
    let mut j: u16 = 0;
    for dexNum in 1..NATIONAL_DEX_DEOXYS {
        if GetSetPokedexFlag(dexNum, FLAG_GET_CAUGHT) != 0 {
            (*sCreditsData).caughtMonIds[j] = dexNum;
            j += 1;
        }
    }
    let mut dexNum: u16 = j;
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
