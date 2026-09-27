//! Translated from `src/credits.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sCredits_Pal sCreditsCopyrightEnd_Gfx sTheEnd_LetterMap_T sTheEnd_LetterMap_H sTheEnd_LetterMap_E sTheEnd_LetterMap_N sTheEnd_LetterMap_D sCreditsText_EmptyString sCreditsText_PkmnEmeraldVersion sCreditsText_Credits sCreditsText_ExecutiveDirector sCreditsText_Director sCreditsText_ArtDirector sCreditsText_BattleDirector sCreditsText_MainProgrammer sCreditsText_BattleSystemPgrms sCreditsText_FieldSystemPgrms sCreditsText_Programmers sCreditsText_MainGraphicDesigner sCreditsText_GraphicDesigners sCreditsText_PkmnDesigners sCreditsText_MusicComposition sCreditsText_SoundEffectsAndPkmnVoices sCreditsText_GameDesigners sCreditsText_ScenarioPlot sCreditsText_Scenario sCreditsText_ScriptDesigners sCreditsText_MapDesigners sCreditsText_MapDataDesigners sCreditsText_ParametricDesigners sCreditsText_PokedexText sCreditsText_EnvAndToolPgrms sCreditsText_NCLProductTesting sCreditsText_SpecialThanks sCreditsText_Coordinators sCreditsText_Producers sCreditsText_ExecProducers sCreditsText_InfoSupervisors sCreditsText_TaskManagers sCreditsText_BrailleCodeCheck sCreditsText_WorldDirector sCreditsText_BattleFrontierData sCreditsText_SupportProgrammers sCreditsText_Artwork sCreditsText_LeadProgrammer sCreditsText_LeadGraphicArtist sCreditsText_SatoshiTajiri sCreditsText_JunichiMasuda sCreditsText_KenSugimori sCreditsText_ShigekiMorimoto sCreditsText_TetsuyaWatanabe sCreditsText_HisashiSogabe sCreditsText_SosukeTamada sCreditsText_AkitoMori sCreditsText_KeitaKagaya sCreditsText_YoshinoriMatsuda sCreditsText_HiroyukiNakamura sCreditsText_MasaoTaya sCreditsText_SatoshiNohara sCreditsText_TomomichiOhta sCreditsText_MiyukiIwasawa sCreditsText_TakenoriOhta sCreditsText_HironobuYoshida sCreditsText_MotofumiFujiwara sCreditsText_SatoshiOhta sCreditsText_AsukaIwashita sCreditsText_AimiTomita sCreditsText_TakaoUnno sCreditsText_KanakoEo sCreditsText_JunOkutani sCreditsText_AtsukoNishida sCreditsText_MuneoSaito sCreditsText_RenaYoshikawa sCreditsText_GoIchinose sCreditsText_MorikazuAoki sCreditsText_KojiNishino sCreditsText_KenjiMatsushima sCreditsText_TetsujiOhta sCreditsText_HitomiSato sCreditsText_TakeshiKawachimaru sCreditsText_TeruyukiShimoyamada sCreditsText_ShigeruOhmori sCreditsText_TadashiTakahashi sCreditsText_ToshinobuMatsumiya sCreditsText_AkihitoTomisawa sCreditsText_HirokiEnomoto sCreditsText_KazuyukiTerada sCreditsText_YuriSakurai sCreditsText_HiromiSagawa sCreditsText_KenjiTominaga sCreditsText_YoshioTajiri sCreditsText_TeikoSasaki sCreditsText_SachikoHamano sCreditsText_ChieMatsumiya sCreditsText_AkikoShinozaki sCreditsText_AstukoFujii sCreditsText_NozomuSaito sCreditsText_KenkichiToyama sCreditsText_SuguruNakatsui sCreditsText_YumiFunasaka sCreditsText_NaokoYanase sCreditsText_NCLSuperMarioClub sCreditsText_AtsushiTada sCreditsText_TakahiroOhnishi sCreditsText_NorihideOkamura sCreditsText_HiroNakamura sCreditsText_HiroyukiUesugi sCreditsText_TerukiMurakawa sCreditsText_AkiraKinashi sCreditsText_MichikoTakizawa sCreditsText_MakikoTakada sCreditsText_TakanaoKondo sCreditsText_AiMashima sCreditsText_GakujiNomoto sCreditsText_TakehiroIzushi sCreditsText_HitoshiYamagami sCreditsText_KyokoWatanabe sCreditsText_TakaoNakano sCreditsText_HiroyukiJinnai sCreditsText_HiroakiTsuru sCreditsText_TsunekazIshihara sCreditsText_SatoruIwata sCreditsText_KazuyaSuyama sCreditsText_SatoshiMitsuhara sCreditsText_JapanBrailleLibrary sCreditsText_TomotakaKomura sCreditsText_MikikoOhhashi sCreditsText_DaisukeHoshino sCreditsText_KenjiroIto sCreditsText_RuiKawaguchi sCreditsText_ShunsukeKohori sCreditsText_SachikoNakamichi sCreditsText_FujikoNomura sCreditsText_KazukiYoshihara sCreditsText_RetsujiNomoto sCreditsText_AzusaTajima sCreditsText_ShusakuEgami sCreditsText_PackageAndManual sCreditsText_EnglishVersion sCreditsText_Translator sCreditsText_TextEditor sCreditsText_NCLCoordinator sCreditsText_GraphicDesigner sCreditsText_NOAProductTesting sCreditsText_HideyukiNakajima sCreditsText_HidenoriSaeki sCreditsText_YokoWatanabe sCreditsText_SakaeKimura sCreditsText_ChiakiShinkai sCreditsText_SethMcMahill sCreditsText_NobOgasawara sCreditsText_TeresaLillygren sCreditsText_KimikoNakamichi sCreditsText_SouichiYamamoto sCreditsText_YuichiroIto sCreditsText_ThomasHertzog sCreditsText_MikaKurosawa sCreditsText_NationalFederationBlind sCreditsText_PatriciaAMaurer sCreditsText_EuropeanBlindUnion sCreditsText_AustralianBrailleAuthority sCreditsText_RoyalNewZealandFederationBlind sCreditsText_MotoyasuTojima sCreditsText_NicolaPrattBarlow sCreditsText_ShellieDow sCreditsText_ErikJohnson sCreditsEntry_EmptyString sCreditsEntry_PkmnEmeraldVersion sCreditsEntry_Credits sCreditsEntry_ExecutiveDirector sCreditsEntry_Director sCreditsEntry_ArtDirector sCreditsEntry_BattleDirector sCreditsEntry_MainProgrammer sCreditsEntry_BattleSystemPgrms sCreditsEntry_FieldSystemPgrms sCreditsEntry_Programmers sCreditsEntry_MainGraphicDesigner sCreditsEntry_GraphicDesigners sCreditsEntry_PkmnDesigners sCreditsEntry_MusicComposition sCreditsEntry_SoundEffectsAndPkmnVoices sCreditsEntry_GameDesigners sCreditsEntry_ScenarioPlot sCreditsEntry_Scenario sCreditsEntry_ScriptDesigners sCreditsEntry_MapDesigners sCreditsEntry_MapDataDesigners sCreditsEntry_ParametricDesigners sCreditsEntry_PokedexText sCreditsEntry_EnvAndToolPgrms sCreditsEntry_NCLProductTesting sCreditsEntry_SpecialThanks sCreditsEntry_Coordinators sCreditsEntry_Producers sCreditsEntry_ExecProducers sCreditsEntry_InfoSupervisors sCreditsEntry_TaskManagers sCreditsEntry_BrailleCodeCheck sCreditsEntry_WorldDirector sCreditsEntry_BattleFrontierData sCreditsEntry_SupportProgrammers sCreditsEntry_Artwork sCreditsEntry_LeadProgrammer sCreditsEntry_LeadGraphicArtist sCreditsEntry_SatoshiTajiri sCreditsEntry_JunichiMasuda sCreditsEntry_KenSugimori sCreditsEntry_ShigekiMorimoto sCreditsEntry_TetsuyaWatanabe sCreditsEntry_HisashiSogabe sCreditsEntry_SosukeTamada sCreditsEntry_AkitoMori sCreditsEntry_KeitaKagaya sCreditsEntry_YoshinoriMatsuda sCreditsEntry_HiroyukiNakamura sCreditsEntry_MasaoTaya sCreditsEntry_SatoshiNohara sCreditsEntry_TomomichiOhta sCreditsEntry_MiyukiIwasawa sCreditsEntry_TakenoriOhta sCreditsEntry_HironobuYoshida sCreditsEntry_MotofumiFujiwara sCreditsEntry_SatoshiOhta sCreditsEntry_AsukaIwashita sCreditsEntry_AimiTomita sCreditsEntry_TakaoUnno sCreditsEntry_KanakoEo sCreditsEntry_JunOkutani sCreditsEntry_AtsukoNishida sCreditsEntry_MuneoSaito sCreditsEntry_RenaYoshikawa sCreditsEntry_GoIchinose sCreditsEntry_MorikazuAoki sCreditsEntry_KojiNishino sCreditsEntry_KenjiMatsushima sCreditsEntry_TetsujiOhta sCreditsEntry_HitomiSato sCreditsEntry_TakeshiKawachimaru sCreditsEntry_TeruyukiShimoyamada sCreditsEntry_ShigeruOhmori sCreditsEntry_TadashiTakahashi sCreditsEntry_ToshinobuMatsumiya sCreditsEntry_AkihitoTomisawa sCreditsEntry_HirokiEnomoto sCreditsEntry_KazuyukiTerada sCreditsEntry_YuriSakurai sCreditsEntry_HiromiSagawa sCreditsEntry_KenjiTominaga sCreditsEntry_YoshioTajiri sCreditsEntry_TeikoSasaki sCreditsEntry_SachikoHamano sCreditsEntry_ChieMatsumiya sCreditsEntry_AkikoShinozaki sCreditsEntry_AstukoFujii sCreditsEntry_NozomuSaito sCreditsEntry_KenkichiToyama sCreditsEntry_SuguruNakatsui sCreditsEntry_YumiFunasaka sCreditsEntry_NaokoYanase sCreditsEntry_NCLSuperMarioClub sCreditsEntry_AtsushiTada sCreditsEntry_TakahiroOhnishi sCreditsEntry_NorihideOkamura sCreditsEntry_HiroNakamura sCreditsEntry_HiroyukiUesugi sCreditsEntry_TerukiMurakawa sCreditsEntry_AkiraKinashi sCreditsEntry_MichikoTakizawa sCreditsEntry_MakikoTakada sCreditsEntry_TakanaoKondo sCreditsEntry_AiMashima sCreditsEntry_GakujiNomoto sCreditsEntry_TakehiroIzushi sCreditsEntry_HitoshiYamagami sCreditsEntry_KyokoWatanabe sCreditsEntry_TakaoNakano sCreditsEntry_HiroyukiJinnai sCreditsEntry_HiroakiTsuru sCreditsEntry_TsunekazIshihara sCreditsEntry_SatoruIwata sCreditsEntry_KazuyaSuyama sCreditsEntry_SatoshiMitsuhara sCreditsEntry_JapanBrailleLibrary sCreditsEntry_TomotakaKomura sCreditsEntry_MikikoOhhashi sCreditsEntry_DaisukeHoshino sCreditsEntry_KenjiroIto sCreditsEntry_RuiKawaguchi sCreditsEntry_ShunsukeKohori sCreditsEntry_SachikoNakamichi sCreditsEntry_FujikoNomura sCreditsEntry_KazukiYoshihara sCreditsEntry_RetsujiNomoto sCreditsEntry_AzusaTajima sCreditsEntry_ShusakuEgami sCreditsEntry_PackageAndManual sCreditsEntry_EnglishVersion sCreditsEntry_Translator sCreditsEntry_TextEditor sCreditsEntry_NCLCoordinator sCreditsEntry_GraphicDesigner sCreditsEntry_NOAProductTesting sCreditsEntry_HideyukiNakajima sCreditsEntry_HidenoriSaeki sCreditsEntry_YokoWatanabe sCreditsEntry_SakaeKimura sCreditsEntry_ChiakiShinkai sCreditsEntry_SethMcMahill sCreditsEntry_NobOgasawara sCreditsEntry_TeresaLillygren sCreditsEntry_KimikoNakamichi sCreditsEntry_SouichiYamamoto sCreditsEntry_YuichiroIto sCreditsEntry_ThomasHertzog sCreditsEntry_MikaKurosawa sCreditsEntry_NationalFederationBlind sCreditsEntry_PatriciaAMaurer sCreditsEntry_EuropeanBlindUnion sCreditsEntry_AustralianBrailleAuthority sCreditsEntry_RoyalNewZealandFederationBlind sCreditsEntry_MotoyasuTojima sCreditsEntry_NicolaPrattBarlow sCreditsEntry_ShellieDow sCreditsEntry_ErikJohnson sCreditsEntryPointerTable sBackgroundTemplates sWindowTemplates sMonSpritePos sAnim_Player_Slow sAnim_Player_Fast sAnim_Player_LookBack sAnim_Player_LookForward sAnims_Player sAnim_Rival_Slow sAnim_Rival_Fast sAnim_Rival_Still sAnims_Rival sSpriteSheet_MonBg sSpritePalette_MonBg sOamData_MonBg sAnim_MonBg_Yellow sAnim_MonBg_Red sAnim_MonBg_Blue sAnims_MonBg sSpriteTemplate_CreditsMonBg
#[allow(unused_imports)]
use crate::data::credits::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnkVar: i16 = 0i16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavedTaskId: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gHasHallOfFameRecords: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUsedSpeedUp: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCreditsData: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gBirchBagGrass_Gfx: u8;
    static mut gBirchBagGrass_Pal: u8;
    static mut gBirchGrassTilemap: u8;
    static mut gDecompressionBuffer: u8;
    static mut gHeap: u8;
    static mut gIntroCopyright_Pal: u8;
    static mut gIntroCredits_MovingSceneryState: u8;
    static mut gIntroCredits_MovingSceneryVBase: u8;
    static mut gIntroCredits_MovingSceneryVOffset: u8;
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gReservedSpritePaletteCount: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpritePalettes_Credits: u8;
    static mut gSpriteSheet_CreditsBicycle: u8;
    static mut gSpriteSheet_CreditsBrendan: u8;
    static mut gSpriteSheet_CreditsMay: u8;
    static mut gSpriteSheet_CreditsRivalBrendan: u8;
    static mut gSpriteSheet_CreditsRivalMay: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
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
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateBicycleBgAnimationTask(a0: u8, a1: u16, a2: u16, a3: u16) -> u8;
    fn CreateIntroBrendanSprite(a0: i16, a1: i16) -> u8;
    fn CreateIntroMaySprite(a0: i16, a1: i16) -> u8;
    fn CreateMonSpriteFromNationalDexNumber(a0: u16, a1: i16, a2: i16, a3: u16) -> u16;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CycleSceneryPalette(a0: u8);
    fn DeactivateAllTextPrinters();
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn EnableInterrupts(a0: u16);
    fn FadeOutBGM(a0: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeAndDestroyMonPicSprite(a0: u16) -> u16;
    fn GetBgTilemapBuffer(a0: u8) -> *mut u8;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn GetStarterPokemon(a0: u16) -> u16;
    fn GetStringCenterAlignXOffsetWithLetterSpacing(a0: i32, a1: *mut u8, a2: i32, a3: i32) -> i32;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitHeap(a0: *mut u8, a1: u32);
    fn InitWindows(a0: *mut u8) -> u16;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadCreditsSceneGraphics(a0: u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpritePalettes(a0: *mut u8);
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn ResetAllPicSprites() -> u16;
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetCreditsSceneBgCnt(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetOamMatrix(a0: u8, a1: u16, a2: u16, a3: u16, a4: u16);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SoftReset(a0: u32);
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn StartSpriteAnimIfDifferent(a0: *mut u8, a1: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn VarGet(a0: u16) -> u16;
    fn m4aSongNumStart(a0: u16);
}

pub(crate) unsafe extern "C" fn VBlankCB_Credits() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn CB2_Credits() {
    unsafe {
        RunTasks();
        AnimateSprites();
        if ((((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0)
            && ((((&raw mut gHasHallOfFameRecords).cast::<u8>().cast::<u8>()).read()) != 0))
            && (core::mem::transmute::<_, usize>(
                ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sSavedTaskId).cast::<u8>().cast::<u16>()).read()) as i32) as isize
                        * 40,
                ))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .read(),
            ) == (Task_CreditsMain as *const () as usize))
        {
            VBlankCB_Credits();
            RunTasks();
            AnimateSprites();
            ((&raw mut sUsedSpeedUp).cast::<u8>().cast::<u8>()).write(1u8);
        }
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn InitCreditsBgsAndWindows() {
    unsafe {
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBackgroundTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(4u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(0u8, AllocZeroed(2048u32));
        LoadPalette(
            (((&raw const sCredits_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            128u16,
            64u16,
        );
        InitWindows(((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>());
        DeactivateAllTextPrinters();
        PutWindowTilemap(0u8);
        CopyWindowToVram(0u8, 3u8);
        ShowBg(0u8);
    }
}
pub(crate) unsafe extern "C" fn FreeCreditsBgsAndWindows() {
    unsafe {
        let mut ptr: *mut u8 = core::ptr::null_mut();
        FreeAllWindowBuffers();
        ptr = GetBgTilemapBuffer(0u8);
        if !(ptr).is_null() {
            Free(ptr);
        }
    }
}
pub(crate) unsafe extern "C" fn PrintCreditsText(string: *mut u8, y: u8, isTitle: u8) {
    unsafe {
        let mut string = string;
        let mut y = y;
        let mut isTitle = isTitle;
        let mut x: u8 = 0u8;
        let mut color = crate::ffi::Align4([0u8; 3]);
        ((&raw mut color).cast::<u8>()).write(0u8);
        if ((isTitle) as i32) == 1i32 {
            (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(3u8);
            (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(4u8);
        } else {
            (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(1u8);
            (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(2u8);
        }
        x = ((GetStringCenterAlignXOffsetWithLetterSpacing(1i32, string, 240i32, 1i32)) as u8);
        AddTextPrinterParameterized4(
            0u8,
            1u8,
            x,
            y,
            1u8,
            0u8,
            (&raw mut color).cast::<u8>(),
            (-1i8),
            string,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_StartCreditsSequence() {
    unsafe {
        let mut taskId: u8 = 0u8;
        let mut bikeTaskId: i16 = 0i16;
        let mut pageTaskId: u8 = 0u8;
        ResetGpuAndVram();
        SetVBlankCallback(None);
        InitHeap((&raw mut gHeap).cast::<u8>(), 114688u32);
        ResetPaletteFade();
        ResetTasks();
        InitCreditsBgsAndWindows();
        taskId = CreateTask(Some(Task_WaitPaletteFade), 0u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(13))
        .write(1i16);
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            if (LoadBikeScene(0u8, taskId)) != 0 {
                break 'l1;
            }
        }
        bikeTaskId = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read();
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((bikeTaskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(40i16);
        SetGpuReg(18u8, 65532u16);
        pageTaskId = CreateTask(Some(Task_UpdatePage), 0u8);
        ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((pageTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((taskId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(((pageTaskId) as i16));
        BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
        EnableInterrupts(1u16);
        SetVBlankCallback(Some(VBlankCB_Credits));
        m4aSongNumStart(455u16);
        SetMainCallback2(Some(CB2_Credits));
        ((&raw mut sUsedSpeedUp).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(940u32));
        DeterminePokemonToShow();
        ((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(142)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(144)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(146)
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSavedTaskId).cast::<u8>().cast::<u16>()).write(((taskId) as u16));
    }
}
pub(crate) unsafe extern "C" fn Task_WaitPaletteFade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_CreditsMain));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CreditsMain(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mode: u16 = 0u16;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read())
            != 0
        {
            let mut bikeTaskId: i16 = ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read();
            (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((bikeTaskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .write(30i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12))
            .write(256i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_CreditsTheEnd1));
            return;
        }
        ((&raw mut sUnkVar).cast::<u8>().cast::<i16>()).write(0i16);
        mode = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11))
        .read()) as u16);
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11))
        .read()) as i32)
            == 1i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(13))
            .write(((mode) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(0i16);
            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ReadyBikeScene));
        } else {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .read()) as i32)
                == 2i32
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(13))
                .write(((mode) as i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .write(0i16);
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ReadyShowMons));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReadyBikeScene(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            SetGpuReg(0u8, 0u16);
            ResetCreditsTasks(taskId);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_SetBikeScene));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SetBikeScene(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetVBlankCallback(None);
        if (LoadBikeScene(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .read()) as u8),
            taskId,
        )) != 0
        {
            BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
            EnableInterrupts(1u16);
            SetVBlankCallback(Some(VBlankCB_Credits));
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_WaitPaletteFade));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReadyShowMons(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            SetGpuReg(0u8, 0u16);
            ResetCreditsTasks(taskId);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_LoadShowMons));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LoadShowMons(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 || !__matched {
                {
                    let mut i: u16 = 0u16;
                    let mut temp: *mut u16 = core::ptr::null_mut();
                    ResetSpriteData();
                    ResetAllPicSprites();
                    FreeAllSpritePalettes();
                    ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(8u8);
                    LZ77UnCompVram(
                        ((&raw mut gBirchBagGrass_Gfx).cast::<u32>()).cast::<u32>(),
                        ((100663296i32) as usize as *mut u8),
                    );
                    LZ77UnCompVram(
                        ((&raw mut gBirchGrassTilemap).cast::<u32>()).cast::<u32>(),
                        ((100677632i32) as usize as *mut u8),
                    );
                    LoadPalette(
                        ((((&raw mut gBirchBagGrass_Pal).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(1))
                        .cast::<u8>(),
                        1u16,
                        62u16,
                    );
                    {
                        i = 0u16;
                        'l2: loop {
                            if !(((i) as i32) < crate::c::div_i32(4096i32, 2i32)) {
                                break 'l2;
                            }
                            'l3: {
                                (((&raw mut gDecompressionBuffer).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .write(17u8);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    {
                        i = 0u16;
                        'l4: loop {
                            if !(((i) as i32) < crate::c::div_i32(4096i32, 2i32)) {
                                break 'l4;
                            }
                            'l5: {
                                ((((&raw mut gDecompressionBuffer).cast::<u8>())
                                    .wrapping_offset((crate::c::div_i32(4096i32, 2i32)) as isize))
                                .wrapping_offset(((i) as i32) as isize))
                                .write(34u8);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    {
                        i = 0u16;
                        'l6: loop {
                            if !(((i) as i32) < crate::c::div_i32(4096i32, 2i32)) {
                                break 'l6;
                            }
                            'l7: {
                                ((((&raw mut gDecompressionBuffer).cast::<u8>()).wrapping_offset(
                                    ((crate::c::div_i32(4096i32, 2i32)).wrapping_mul(2i32))
                                        as isize,
                                ))
                                .wrapping_offset(((i) as i32) as isize))
                                .write(51u8);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    temp = (((&raw mut gDecompressionBuffer).cast::<u8>()).wrapping_offset(
                        ((crate::c::div_i32(4096i32, 2i32)).wrapping_mul(3i32)) as isize,
                    ))
                    .cast::<u16>();
                    (temp).write(0u16);
                    ((temp).wrapping_offset(1)).write(21503u16);
                    ((temp).wrapping_offset(2)).write(21151u16);
                    ((temp).wrapping_offset(3)).write(32404u16);
                    LoadSpriteSheet(
                        ((&raw const sSpriteSheet_MonBg).cast::<u8>().cast_mut()).cast::<u8>(),
                    );
                    LoadSpritePalette(
                        ((&raw const sSpritePalette_MonBg).cast::<u8>().cast_mut()).cast::<u8>(),
                    );
                    let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    break 'l1;
                }
            }
            if __sw1 == 1i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(((CreateTask(Some(Task_ShowMons), 0u8)) as i16));
                (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32) as isize
                        * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .write(1i16);
                ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32) as isize
                        * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(((taskId) as i16));
                ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32) as isize
                        * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(7))
                    .read(),
                );
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                SetGpuReg(28u8, 0u16);
                SetGpuReg(30u8, 32u16);
                SetGpuReg(14u8, 1795u16);
                SetGpuReg(0u8, 6464u16);
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(0u8);
                ((&raw mut gIntroCredits_MovingSceneryState).cast::<i16>()).write(0i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_WaitPaletteFade));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CreditsTheEnd1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(12))
        .read())
            != 0
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            return;
        }
        BeginNormalPaletteFade(4294967295u32, 12i8, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_CreditsTheEnd2));
    }
}
pub(crate) unsafe extern "C" fn Task_CreditsTheEnd2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            ResetCreditsTasks(taskId);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_CreditsTheEnd3));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CreditsTheEnd3(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ResetGpuAndVram();
        ResetPaletteFade();
        LoadTheEndScreen(0u16, 14336u16, 0u16);
        ResetSpriteData();
        FreeAllSpritePalettes();
        BeginNormalPaletteFade(4294967295u32, 8i8, 16u8, 0u8, 0u16);
        SetGpuReg(8u8, 1792u16);
        EnableInterrupts(1u16);
        SetGpuReg(0u8, 320u16);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(235i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_CreditsTheEnd4));
    }
}
pub(crate) unsafe extern "C" fn Task_CreditsTheEnd4(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .read())
            != 0
        {
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            return;
        }
        BeginNormalPaletteFade(4294967295u32, 6i8, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_CreditsTheEnd5));
    }
}
pub(crate) unsafe extern "C" fn Task_CreditsTheEnd5(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            DrawTheEnd(14336u16, 0u16);
            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 0u8, 0u16);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(7200i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_CreditsTheEnd6));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CreditsTheEnd6(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                == 0i32)
                || (((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read())
                    != 0)
            {
                FadeOutBGM(4u8);
                BeginNormalPaletteFade(4294967295u32, 8i8, 0u8, 16u8, 65535u16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_CreditsSoftReset));
                return;
            }
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                == 7144i32
            {
                FadeOutBGM(8u8);
            }
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                == 6840i32
            {
                m4aSongNumStart(456u16);
            }
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CreditsSoftReset(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            SoftReset(255u32);
        }
    }
}
pub(crate) unsafe extern "C" fn ResetGpuAndVram() {
    unsafe {
        SetGpuReg(0u8, 0u16);
        SetGpuReg(28u8, 0u16);
        SetGpuReg(30u8, 0u16);
        SetGpuReg(24u8, 0u16);
        SetGpuReg(26u8, 0u16);
        SetGpuReg(20u8, 0u16);
        SetGpuReg(22u8, 0u16);
        SetGpuReg(16u8, 0u16);
        SetGpuReg(18u8, 0u16);
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        SetGpuReg(84u8, 0u16);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((&raw mut tmp) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((100663296i32) as usize as *mut u8) as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2130706432i32)
                                        | crate::c::div_i32(
                                            98304i32,
                                            crate::c::div_i32(16i32, 8i32),
                                        )) as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
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
        'l5: loop {
            'l6: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l7: loop {
                        'l8: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((&raw mut tmp) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((117440512i32) as usize as *mut u8) as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2063597568i32)
                                        | crate::c::div_i32(
                                            1024i32,
                                            crate::c::div_i32(32i32, 8i32),
                                        )) as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
        'l9: loop {
            'l10: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l11: loop {
                        'l12: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((&raw mut tmp) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((83886082i32) as usize as *mut u8) as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2130706432i32)
                                        | crate::c::div_i32(
                                            1022i32,
                                            crate::c::div_i32(16i32, 8i32),
                                        )) as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l11;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l9;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_UpdatePage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 10i32;
            if __sw1 == 0i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || !__matched
            {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(72i16);
                    ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(14))
                    .write(0i16);
                    ((&raw mut sUnkVar).cast::<u8>().cast::<i16>()).write(0i16);
                }
                return;
            }
            if __sw1 == 1i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    != 0i32
                {
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3);
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                    return;
                }
                let __p3 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                return;
            }
            if __sw1 == 2i32 {
                if core::mem::transmute::<_, usize>(
                    ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32) as isize
                            * 40,
                    ))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .read(),
                ) == (Task_CreditsMain as *const () as usize)
                {
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        < 57i32
                    {
                        {
                            i = 0i32;
                            'l2: loop {
                                if !(i < 5i32) {
                                    break 'l2;
                                }
                                'l3: {
                                    PrintCreditsText(
                                        ((((((((&raw const sCreditsEntryPointerTable)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((((((((&raw mut gTasks).cast::<u8>())
                                                .wrapping_offset(
                                                    ((taskId) as i32) as isize * 40,
                                                ))
                                            .wrapping_add(8))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .read())
                                                as i32)
                                                as isize
                                                * 20,
                                        ))
                                        .cast::<*mut u8>())
                                        .wrapping_offset((i) as isize))
                                        .read())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                        .read(),
                                        (((5i32).wrapping_add((i).wrapping_mul(16i32))) as u8),
                                        ((((((((&raw const sCreditsEntryPointerTable)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((((((((&raw mut gTasks).cast::<u8>())
                                                .wrapping_offset(
                                                    ((taskId) as i32) as isize * 40,
                                                ))
                                            .wrapping_add(8))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .read())
                                                as i32)
                                                as isize
                                                * 20,
                                        ))
                                        .cast::<*mut u8>())
                                        .wrapping_offset((i) as isize))
                                        .read())
                                        .wrapping_add(1))
                                        .read(),
                                    );
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        CopyWindowToVram(0u8, 2u8);
                        let __p4 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2);
                        (__p4).write(((__p4).read()).wrapping_add(1));
                        let __p5 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p5).write(((__p5).read()).wrapping_add(1));
                        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize
                                * 40,
                        ))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(14))
                        .write(1i16);
                        if ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize
                                * 40,
                        ))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(13))
                        .read()) as i32)
                            == 1i32
                        {
                            BeginNormalPaletteFade(768u32, 0i8, 16u8, 0u8, 12941u16);
                        } else {
                            BeginNormalPaletteFade(768u32, 0i8, 16u8, 0u8, 6503u16);
                        }
                        return;
                    }
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(10i16);
                    return;
                }
                ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(14))
                .write(0i16);
                return;
            }
            if __sw1 == 3i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(115i16);
                    let __p6 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                return;
            }
            if __sw1 == 4i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    != 0i32
                {
                    let __p7 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3);
                    (__p7).write(((__p7).read()).wrapping_sub(1));
                    return;
                }
                if (CheckChangeScene(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as u8),
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as u8),
                )) != 0
                {
                    let __p8 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                    return;
                }
                let __p9 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p9).write(((__p9).read()).wrapping_add(1));
                if ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(13))
                .read()) as i32)
                    == 1i32
                {
                    BeginNormalPaletteFade(768u32, 0i8, 0u8, 16u8, 12941u16);
                } else {
                    BeginNormalPaletteFade(768u32, 0i8, 0u8, 16u8, 6503u16);
                }
                return;
            }
            if __sw1 == 5i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    FillWindowPixelBuffer(0u8, 0u8);
                    CopyWindowToVram(0u8, 2u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(2i16);
                }
                return;
            }
            if __sw1 == 10i32 {
                ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(1i16);
                DestroyTask(taskId);
                FreeCreditsBgsAndWindows();
                {
                    Free(((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read());
                    ((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                }
                return;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CheckChangeScene(page: u8, taskId: u8) -> u8 {
    unsafe {
        let mut page = page;
        let mut taskId = taskId;
        if ((page) as i32) == (crate::c::div_i32(57i32, 9i32)).wrapping_mul(1i32) {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(2i16);
        }
        if ((page) as i32) == (crate::c::div_i32(57i32, 9i32)).wrapping_mul(2i32) {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(1i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(1i16);
        }
        if ((page) as i32) == (crate::c::div_i32(57i32, 9i32)).wrapping_mul(3i32) {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(2i16);
        }
        if ((page) as i32) == (crate::c::div_i32(57i32, 9i32)).wrapping_mul(4i32) {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(2i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(1i16);
        }
        if ((page) as i32) == (crate::c::div_i32(57i32, 9i32)).wrapping_mul(5i32) {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(2i16);
        }
        if ((page) as i32) == (crate::c::div_i32(57i32, 9i32)).wrapping_mul(6i32) {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(3i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(1i16);
        }
        if ((page) as i32) == (crate::c::div_i32(57i32, 9i32)).wrapping_mul(7i32) {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(2i16);
        }
        if ((page) as i32) == (crate::c::div_i32(57i32, 9i32)).wrapping_mul(8i32) {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(4i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(1i16);
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11))
        .read()) as i32)
            != 0i32
        {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_ShowMons(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(144)
                    .cast::<u16>())
                .read()) as i32)
                    == 0i32)
                    && (((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(14))
                    .read()) as i32)
                        == 0i32)
                {
                    break 'l1;
                }
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(142)
                    .cast::<u16>())
                .read()) as i32)
                    == 71i32)
                    || (core::mem::transmute::<_, usize>(
                        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize
                                * 40,
                        ))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .read(),
                    ) != (Task_CreditsMain as *const () as usize))
                {
                    break 'l1;
                }
                spriteId = CreateCreditsMonSprite(
                    (((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .wrapping_offset(
                        ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(146)
                            .cast::<u16>())
                        .read()) as i32) as isize,
                    ))
                    .read(),
                    (((((((&raw const sMonSpritePos).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(144)
                                .cast::<u16>())
                            .read()) as i32) as isize
                                * 2,
                        ))
                    .cast::<u8>())
                    .read()) as i16),
                    ((((((((&raw const sMonSpritePos).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(144)
                                .cast::<u16>())
                            .read()) as i32) as isize
                                * 2,
                        ))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i16),
                    ((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144)
                        .cast::<u16>())
                    .read(),
                );
                if ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(146)
                    .cast::<u16>())
                .read()) as i32)
                    < ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_sub(1i32)
                {
                    let __p3 = (((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(146)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(50i16);
                } else {
                    ((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(146)
                        .cast::<u16>())
                    .write(0u16);
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(512i16);
                }
                let __p4 = (((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(142)
                    .cast::<u16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                if ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(144)
                    .cast::<u16>())
                .read()) as i32)
                    == 2i32
                {
                    ((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144)
                        .cast::<u16>())
                    .write(0u16);
                } else {
                    let __p5 = (((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144)
                        .cast::<u16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(50i16);
                let __p6 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    != 0i32
                {
                    let __p7 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3);
                    (__p7).write(((__p7).read()).wrapping_sub(1));
                } else {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_BikeScene(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut gIntroCredits_MovingSceneryVOffset).cast::<i16>()).write(Sin(
                    (((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32)
                        >> 1)
                        & 127i32) as i16),
                    12i16,
                ));
                let __p2 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((&raw mut gIntroCredits_MovingSceneryVOffset).cast::<i16>()).read()) as i32)
                    != 0i32
                {
                    ((&raw mut gIntroCredits_MovingSceneryVOffset).cast::<i16>()).write(Sin(
                        (((((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .read()) as i32)
                            >> 1)
                            & 127i32) as i16),
                        12i16,
                    ));
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                } else {
                    (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(2i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write(0i16);
                    let __p4 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as i32)
                    < 64i32
                {
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    ((&raw mut gIntroCredits_MovingSceneryVOffset).cast::<i16>()).write(Sin(
                        ((((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .read()) as i32)
                            & 127i32) as i16),
                        20i16,
                    ));
                } else {
                    let __p6 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .write(3i16);
                (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .write(1i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(120i16);
                let __p7 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as i32)
                    != 0i32
                {
                    let __p8 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4);
                    (__p8).write(((__p8).read()).wrapping_sub(1));
                } else {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write(64i16);
                    let __p9 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as i32)
                    > 0i32
                {
                    let __p10 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5);
                    (__p10).write(((__p10).read()).wrapping_sub(1));
                    ((&raw mut gIntroCredits_MovingSceneryVOffset).cast::<i16>()).write(Sin(
                        ((((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .read()) as i32)
                            & 127i32) as i16),
                        20i16,
                    ));
                } else {
                    (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(1i16);
                    let __p11 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(50i16);
                break 'l1;
            }
            if __sw1 == 10i32 {
                (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .write(2i16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(50i16);
                break 'l1;
            }
            if __sw1 == 20i32 {
                (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .write(4i16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(50i16);
                break 'l1;
            }
            if __sw1 == 30i32 {
                (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .write(5i16);
                (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .write(3i16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(50i16);
                break 'l1;
            }
            if __sw1 == 50i32 {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CycleSceneryPalette(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut bikeTaskId: i16 = 0i16;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 || !__matched {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    != 32767i32
                {
                    if ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(2))
                            .read()) as i32) as isize
                                * 40,
                        ))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(15))
                        .read()) as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        == 2i32
                    {
                        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as i32) as isize
                                    * 40,
                            ))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize
                                * 40,
                        ))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(20i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(32767i16);
                    }
                }
                CycleSceneryPalette(0u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                CycleSceneryPalette(0u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    != 32767i32
                {
                    bikeTaskId = ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read();
                    if (((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((bikeTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32)
                        & (-128i32))
                        == 640i32
                    {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((bikeTaskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(1i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(32767i16);
                    }
                }
                CycleSceneryPalette(1u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    != 32767i32
                {
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        == 584i32
                    {
                        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as i32) as isize
                                    * 40,
                            ))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize
                                * 40,
                        ))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(10i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(32767i16);
                    } else {
                        let __p2 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1);
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                }
                CycleSceneryPalette(1u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                CycleSceneryPalette(2u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetBikeScene(scene: u8, taskId: u8) {
    unsafe {
        let mut scene = scene;
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((scene) as i32);
            if __sw1 == 0i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write(272i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write(272i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .write(46i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .write(46i16);
                (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .write(0i16);
                (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .write(0i16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(((CreateBicycleBgAnimationTask(0u8, 8192u16, 32u16, 8u16)) as i16));
                break 'l1;
            }
            if __sw1 == 1i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write(120i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write(272i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .write(46i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .write(46i16);
                (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .write(0i16);
                (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .write(0i16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(((CreateBicycleBgAnimationTask(0u8, 8192u16, 32u16, 8u16)) as i16));
                break 'l1;
            }
            if __sw1 == 2i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write(120i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write(272i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .write(46i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .write(46i16);
                (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .write(0i16);
                (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .write(0i16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(((CreateBicycleBgAnimationTask(1u8, 8192u16, 512u16, 8u16)) as i16));
                break 'l1;
            }
            if __sw1 == 3i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write(120i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write((-32i16));
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .write(46i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .write(46i16);
                (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .write(0i16);
                (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .write(0i16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(((CreateBicycleBgAnimationTask(1u8, 8192u16, 512u16, 8u16)) as i16));
                break 'l1;
            }
            if __sw1 == 4i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write(88i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write(152i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .write(46i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .write(46i16);
                (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .write(0i16);
                (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .write(0i16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(((CreateBicycleBgAnimationTask(2u8, 8192u16, 512u16, 8u16)) as i16));
                break 'l1;
            }
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((CreateTask(Some(Task_CycleSceneryPalette), 0u8)) as i16));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .write(((scene) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((taskId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((CreateTask(Some(Task_BikeScene), 0u8)) as i16));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((taskId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read(),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .read(),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(0i16);
        if ((scene) as i32) == 2i32 {
            ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(69i16);
        }
    }
}
pub(crate) unsafe extern "C" fn LoadBikeScene(scene: u8, taskId: u8) -> u8 {
    unsafe {
        let mut scene = scene;
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 || !__matched {
                SetGpuReg(0u8, 0u16);
                SetGpuReg(28u8, 8u16);
                SetGpuReg(30u8, 0u16);
                SetGpuReg(24u8, 0u16);
                SetGpuReg(26u8, 0u16);
                SetGpuReg(20u8, 0u16);
                SetGpuReg(22u8, 0u16);
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                ResetSpriteData();
                FreeAllSpritePalettes();
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut gIntroCredits_MovingSceneryVBase).cast::<u16>()).write(34u16);
                ((&raw mut gIntroCredits_MovingSceneryVOffset).cast::<i16>()).write(0i16);
                LoadCreditsSceneGraphics(scene);
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8))
                    .read()) as i32)
                    == 0i32
                {
                    LoadCompressedSpriteSheet((&raw mut gSpriteSheet_CreditsBrendan).cast::<u8>());
                    LoadCompressedSpriteSheet((&raw mut gSpriteSheet_CreditsRivalMay).cast::<u8>());
                    LoadCompressedSpriteSheet((&raw mut gSpriteSheet_CreditsBicycle).cast::<u8>());
                    LoadSpritePalettes((&raw mut gSpritePalettes_Credits).cast::<u8>());
                    spriteId = CreateIntroBrendanSprite(120i16, 46i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write(((spriteId) as i16));
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_Player));
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(8)
                    .cast::<*mut *mut u8>())
                    .write(
                        ((&raw const sAnims_Player)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>(),
                    );
                    spriteId = CreateIntroMaySprite(272i16, 46i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .write(((spriteId) as i16));
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_Rival));
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(8)
                    .cast::<*mut *mut u8>())
                    .write(
                        ((&raw const sAnims_Rival)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>(),
                    );
                } else {
                    LoadCompressedSpriteSheet((&raw mut gSpriteSheet_CreditsMay).cast::<u8>());
                    LoadCompressedSpriteSheet(
                        (&raw mut gSpriteSheet_CreditsRivalBrendan).cast::<u8>(),
                    );
                    LoadCompressedSpriteSheet((&raw mut gSpriteSheet_CreditsBicycle).cast::<u8>());
                    LoadSpritePalettes((&raw mut gSpritePalettes_Credits).cast::<u8>());
                    spriteId = CreateIntroMaySprite(120i16, 46i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write(((spriteId) as i16));
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_Player));
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(8)
                    .cast::<*mut *mut u8>())
                    .write(
                        ((&raw const sAnims_Player)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>(),
                    );
                    spriteId = CreateIntroBrendanSprite(272i16, 46i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .write(((spriteId) as i16));
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_Rival));
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(8)
                    .cast::<*mut *mut u8>())
                    .write(
                        ((&raw const sAnims_Rival)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>(),
                    );
                }
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetBikeScene(scene, taskId);
                SetCreditsSceneBgCnt(scene);
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(0u8);
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ResetCreditsTasks(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            != 0i32
        {
            DestroyTask(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u8),
            );
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            != 0i32
        {
            DestroyTask(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u8),
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(0i16);
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            != 0i32
        {
            DestroyTask(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as u8),
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(0i16);
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            != 0i32
        {
            DestroyTask(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as u8),
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(0i16);
        }
        ((&raw mut gIntroCredits_MovingSceneryState).cast::<i16>()).write(1i16);
    }
}
pub(crate) unsafe extern "C" fn LoadTheEndScreen(
    tileOffsetLoad: u16,
    tileOffsetWrite: u16,
    palOffset: u16,
) {
    unsafe {
        let mut tileOffsetLoad = tileOffsetLoad;
        let mut tileOffsetWrite = tileOffsetWrite;
        let mut palOffset = palOffset;
        let mut baseTile: u16 = 0u16;
        let mut i: u16 = 0u16;
        LZ77UnCompVram(
            ((&raw const sCreditsCopyrightEnd_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (((100663296i32).wrapping_add(((tileOffsetLoad) as i32))) as usize as *mut u8),
        );
        LoadPalette(
            (((&raw mut gIntroCopyright_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            palOffset,
            32u16,
        );
        baseTile = ((crate::c::div_i32(((palOffset) as i32), 16i32) << 12) as u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 1024i32) {
                    break 'l1;
                }
                'l2: {
                    ((((100663296i32).wrapping_add(((tileOffsetWrite) as i32))) as usize
                        as *mut u16)
                        .wrapping_offset(((i) as i32) as isize))
                    .write(((((baseTile) as i32).wrapping_add(1i32)) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetLetterMapTile(baseTiles: u8) -> u16 {
    unsafe {
        let mut baseTiles = baseTiles;
        let mut out: u16 = (((((baseTiles) as i32) & 63i32).wrapping_add(80i32)) as u16);
        if ((baseTiles) as i32) == 255i32 {
            return 1u16;
        }
        if (((baseTiles) as i32) & 128i32) != 0 {
            out = ((((out) as i32) | 2048i32) as u16);
        }
        if (((baseTiles) as i32) & 64i32) != 0 {
            out = ((((out) as i32) | 1024i32) as u16);
        }
        return out;
    }
}
pub(crate) unsafe extern "C" fn DrawLetterMapTiles(
    baseTiles: *mut u8,
    baseX: u8,
    baseY: u8,
    offset: u16,
    palette: u16,
) {
    unsafe {
        let mut baseTiles = baseTiles;
        let mut baseX = baseX;
        let mut baseY = baseY;
        let mut offset = offset;
        let mut palette = palette;
        let mut y: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut tileOffset: u16 = ((crate::c::div_i32(((palette) as i32), 16i32) << 12) as u16);
        {
            y = 0u8;
            'l1: loop {
                if !(((y) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        x = 0u8;
                        'l3: loop {
                            if !(((x) as i32) < 3i32) {
                                break 'l3;
                            }
                            'l4: {
                                (((((100663296i32).wrapping_add(((offset) as i32))).wrapping_add(
                                    (((baseY) as i32).wrapping_add(((y) as i32)))
                                        .wrapping_mul(64i32),
                                )) as usize as *mut u16)
                                    .wrapping_offset(
                                        (((baseX) as i32).wrapping_add(((x) as i32))) as isize,
                                    ))
                                .write(
                                    ((((tileOffset) as i32).wrapping_add(
                                        ((GetLetterMapTile(
                                            ((baseTiles).wrapping_offset(
                                                ((((y) as i32).wrapping_mul(3i32))
                                                    .wrapping_add(((x) as i32)))
                                                    as isize,
                                            ))
                                            .read(),
                                        )) as i32),
                                    )) as u16),
                                );
                            }
                            x = (x).wrapping_add(1);
                        }
                    }
                }
                y = (y).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawTheEnd(offset: u16, palette: u16) {
    unsafe {
        let mut offset = offset;
        let mut palette = palette;
        let mut pos: u16 = 0u16;
        let mut baseTile: u16 = ((crate::c::div_i32(((palette) as i32), 16i32) << 12) as u16);
        {
            pos = 0u16;
            'l1: loop {
                if !(((pos) as i32) < 1024i32) {
                    break 'l1;
                }
                'l2: {
                    ((((100663296i32).wrapping_add(((offset) as i32))) as usize as *mut u16)
                        .wrapping_offset(((pos) as i32) as isize))
                    .write(((((baseTile) as i32).wrapping_add(1i32)) as u16));
                }
                pos = (pos).wrapping_add(1);
            }
        }
        DrawLetterMapTiles(
            ((&raw const sTheEnd_LetterMap_T).cast::<u8>().cast_mut()).cast::<u8>(),
            3u8,
            7u8,
            offset,
            palette,
        );
        DrawLetterMapTiles(
            ((&raw const sTheEnd_LetterMap_H).cast::<u8>().cast_mut()).cast::<u8>(),
            7u8,
            7u8,
            offset,
            palette,
        );
        DrawLetterMapTiles(
            ((&raw const sTheEnd_LetterMap_E).cast::<u8>().cast_mut()).cast::<u8>(),
            11u8,
            7u8,
            offset,
            palette,
        );
        DrawLetterMapTiles(
            ((&raw const sTheEnd_LetterMap_E).cast::<u8>().cast_mut()).cast::<u8>(),
            16u8,
            7u8,
            offset,
            palette,
        );
        DrawLetterMapTiles(
            ((&raw const sTheEnd_LetterMap_N).cast::<u8>().cast_mut()).cast::<u8>(),
            20u8,
            7u8,
            offset,
            palette,
        );
        DrawLetterMapTiles(
            ((&raw const sTheEnd_LetterMap_D).cast::<u8>().cast_mut()).cast::<u8>(),
            24u8,
            7u8,
            offset,
            palette,
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Player(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((&raw mut gIntroCredits_MovingSceneryState).cast::<i16>()).read()) as i32) != 0i32 {
            DestroySprite(sprite);
            return;
        }
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                StartSpriteAnimIfDifferent(sprite, 0u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                StartSpriteAnimIfDifferent(sprite, 1u8);
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > (-32i32) {
                    let __p2 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                StartSpriteAnimIfDifferent(sprite, 2u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                StartSpriteAnimIfDifferent(sprite, 3u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                StartSpriteAnimIfDifferent(sprite, 0u8);
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    > crate::c::div_i32(240i32, 2i32)
                {
                    let __p3 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_sub(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                StartSpriteAnimIfDifferent(sprite, 0u8);
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > (-32i32) {
                    let __p4 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_sub(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Rival(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((&raw mut gIntroCredits_MovingSceneryState).cast::<i16>()).read()) as i32) != 0i32 {
            DestroySprite(sprite);
            return;
        }
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                StartSpriteAnimIfDifferent(sprite, 0u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > 200i32 {
                    StartSpriteAnimIfDifferent(sprite, 1u8);
                } else {
                    StartSpriteAnimIfDifferent(sprite, 2u8);
                }
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > (-32i32) {
                    let __p2 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p2).write((((((__p2).read()) as i32).wrapping_sub(2i32)) as i16));
                }
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    ((((((&raw mut gIntroCredits_MovingSceneryVOffset).cast::<i16>()).read())
                        as i32)
                        .wrapping_neg()) as i16),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                (__p3).write(((__p3).read()).wrapping_add(1));
                StartSpriteAnimIfDifferent(sprite, 0u8);
                if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    & 3i32)
                    == 0i32
                {
                    let __p4 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                StartSpriteAnimIfDifferent(sprite, 0u8);
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > (-32i32) {
                    let __p5 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_sub(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CreditsMon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((&raw mut gIntroCredits_MovingSceneryState).cast::<i16>()).read()) as i32) != 0i32 {
            FreeAndDestroyMonPicSprite(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u16),
            );
            return;
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p1).write(((__p1).read()).wrapping_add(1));
        'l1: {
            let __sw2 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let __matched = __sw2 == 0i32
                || __sw2 == 1i32
                || __sw2 == 2i32
                || __sw2 == 3i32
                || __sw2 == 9i32
                || __sw2 == 10i32;
            if __sw2 == 0i32 || !__matched {
                crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (1u32) as i32);
                crate::c::bf_write(
                    (sprite).wrapping_add(3),
                    1,
                    5,
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as u32) as i32,
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(16i16);
                SetOamMatrix(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as u8),
                    ((crate::c::div_i32(
                        65536i32,
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                    )) as u16),
                    0u16,
                    0u16,
                    ((crate::c::div_i32(
                        65536i32,
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                    )) as u16),
                );
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
                break 'l1;
            }
            if __sw2 == 1i32 {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    < 256i32
                {
                    let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    (__p3).write((((((__p3).read()) as i32).wrapping_add(8i32)) as i16));
                    SetOamMatrix(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as u8),
                        ((crate::c::div_i32(
                            65536i32,
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                .read()) as i32),
                        )) as u16),
                        0u16,
                        0u16,
                        ((crate::c::div_i32(
                            65536i32,
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                .read()) as i32),
                        )) as u16),
                    );
                } else {
                    let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                'l2: {
                    let __sw5 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                        .read()) as i32);
                    if __sw5 == 1i32 {
                        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                            .read()) as i32)
                            & 3i32)
                            == 0i32
                        {
                            let __p6 = (sprite).wrapping_add(34).cast::<i16>();
                            (__p6).write(((__p6).read()).wrapping_add(1));
                        }
                        let __p7 = (sprite).wrapping_add(32).cast::<i16>();
                        (__p7).write((((((__p7).read()) as i32).wrapping_sub(2i32)) as i16));
                        break 'l2;
                    }
                    if __sw5 == 2i32 {
                        break 'l2;
                    }
                    if __sw5 == 3i32 {
                        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                            .read()) as i32)
                            & 3i32)
                            == 0i32
                        {
                            let __p8 = (sprite).wrapping_add(34).cast::<i16>();
                            (__p8).write(((__p8).read()).wrapping_add(1));
                        }
                        let __p9 = (sprite).wrapping_add(32).cast::<i16>();
                        (__p9).write((((((__p9).read()) as i32).wrapping_add(2i32)) as i16));
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw2 == 2i32 {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    != 0i32
                {
                    let __p10 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p10).write(((__p10).read()).wrapping_sub(1));
                } else {
                    SetGpuReg(80u8, 3904u16);
                    SetGpuReg(82u8, 16u16);
                    crate::c::bf_write((sprite).wrapping_add(1), 2, 2, (1u32) as i32);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(16i16);
                    let __p11 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw2 == 3i32 {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    != 0i32
                {
                    let mut data3: i32 = 0i32;
                    let __p12 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p12).write(((__p12).read()).wrapping_sub(1));
                    data3 = (16i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                    );
                    SetGpuReg(
                        82u8,
                        (((data3 << 8).wrapping_add(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                                .read()) as i32),
                        )) as u16),
                    );
                } else {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(9i16);
                }
                break 'l1;
            }
            if __sw2 == 9i32 {
                let __p13 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw2 == 10i32 {
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                FreeAndDestroyMonPicSprite(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as u16),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateCreditsMonSprite(
    nationalDexNum: u16,
    x: i16,
    y: i16,
    position: u16,
) -> u8 {
    unsafe {
        let mut nationalDexNum = nationalDexNum;
        let mut x = x;
        let mut y = y;
        let mut position = position;
        let mut monSpriteId: u8 = 0u8;
        let mut bgSpriteId: u8 = 0u8;
        monSpriteId =
            ((CreateMonSpriteFromNationalDexNumber(nationalDexNum, x, y, position)) as u8);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((monSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((((position) as i32).wrapping_add(1i32)) as i16));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((monSpriteId) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_CreditsMon));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((monSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((monSpriteId) as i16));
        bgSpriteId = CreateSprite(
            (&raw const sSpriteTemplate_CreditsMonBg)
                .cast::<u8>()
                .cast_mut(),
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
            .read(),
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
            .read(),
            1u8,
        );
        (((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((bgSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .write(((monSpriteId) as i16));
        StartSpriteAnimIfDifferent(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((bgSpriteId) as i32) as isize * 68),
            ((position) as u8),
        );
        return monSpriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CreditsMonBg(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .read()) as i32)
            == 10i32)
            || (((((&raw mut gIntroCredits_MovingSceneryState).cast::<i16>()).read()) as i32)
                != 0i32)
        {
            DestroySprite(sprite);
            return;
        }
        crate::c::bf_write(
            (sprite).wrapping_add(62),
            2,
            1,
            (crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                false,
            ) as u16) as i32,
        );
        crate::c::bf_write(
            (sprite).wrapping_add(1),
            2,
            2,
            (crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(1),
                2,
                2,
                false,
            ) as u32) as i32,
        );
        crate::c::bf_write(
            (sprite).wrapping_add(1),
            0,
            2,
            (crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(1),
                0,
                2,
                false,
            ) as u32) as i32,
        );
        crate::c::bf_write(
            (sprite).wrapping_add(3),
            1,
            5,
            (crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(3),
                1,
                5,
                false,
            ) as u32) as i32,
        );
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>())
            .read(),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DeterminePokemonToShow() {
    unsafe {
        let mut starter: u16 = SpeciesToNationalPokedexNum(GetStarterPokemon(VarGet(16419u16)));
        let mut page: u16 = 0u16;
        let mut dexNum: u16 = 0u16;
        let mut j: u16 = 0u16;
        {
            dexNum = 1u16;
            j = 0u16;
            'l1: loop {
                if !(((dexNum) as i32) < 386i32) {
                    break 'l1;
                }
                'l2: {
                    if (GetSetPokedexFlag(dexNum, 1u8)) != 0 {
                        ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(150))
                        .cast::<u16>())
                        .wrapping_offset(((j) as i32) as isize))
                        .write(dexNum);
                        j = (j).wrapping_add(1);
                    }
                }
                dexNum = (dexNum).wrapping_add(1);
            }
        }
        {
            dexNum = j;
            'l3: loop {
                if !(((dexNum) as i32) < 386i32) {
                    break 'l3;
                }
                'l4: {
                    ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(150))
                    .cast::<u16>())
                    .wrapping_offset(((dexNum) as i32) as isize))
                    .write(0u16);
                }
                dexNum = (dexNum).wrapping_add(1);
            }
        }
        ((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(922)
            .cast::<u16>())
        .write(j);
        if ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(922)
            .cast::<u16>())
        .read()) as i32)
            < 71i32
        {
            ((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(148)
                .cast::<u16>())
            .write(j);
        } else {
            ((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(148)
                .cast::<u16>())
            .write(71u16);
        }
        j = 0u16;
        'l5: loop {
            'l6: {
                page = ((crate::c::rem_i32(
                    ((Random()) as i32),
                    ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(922)
                        .cast::<u16>())
                    .read()) as i32),
                )) as u16);
                (((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                    .wrapping_offset(((j) as i32) as isize))
                .write(
                    ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(150))
                    .cast::<u16>())
                    .wrapping_offset(((page) as i32) as isize))
                    .read(),
                );
                j = (j).wrapping_add(1);
                ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(150))
                .cast::<u16>())
                .wrapping_offset(((page) as i32) as isize))
                .write(0u16);
                let __p1 = (((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(922)
                    .cast::<u16>();
                (__p1).write(((__p1).read()).wrapping_sub(1));
                if ((page) as i32)
                    != ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(922)
                        .cast::<u16>())
                    .read()) as i32)
                {
                    ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(150))
                    .cast::<u16>())
                    .wrapping_offset(((page) as i32) as isize))
                    .write(
                        ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(150))
                        .cast::<u16>())
                        .wrapping_offset(
                            ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(922)
                                .cast::<u16>())
                            .read()) as i32) as isize,
                        ))
                        .read(),
                    );
                    ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(150))
                    .cast::<u16>())
                    .wrapping_offset(
                        ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(922)
                            .cast::<u16>())
                        .read()) as i32) as isize,
                    ))
                    .write(0u16);
                }
            }
            if !((((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(922)
                .cast::<u16>())
            .read()) as i32)
                != 0i32)
                && (((j) as i32) < 71i32))
            {
                break 'l5;
            }
        }
        if ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(148)
            .cast::<u16>())
        .read()) as i32)
            < 71i32
        {
            {
                j = ((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>())
                .read();
                page = 0u16;
                'l7: loop {
                    if !(((j) as i32) < 71i32) {
                        break 'l7;
                    }
                    'l8: {
                        (((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u16>())
                        .wrapping_offset(((j) as i32) as isize))
                        .write(
                            (((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u16>())
                            .wrapping_offset(((page) as i32) as isize))
                            .read(),
                        );
                        page = (page).wrapping_add(1);
                        if ((page) as i32)
                            == ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(148)
                            .cast::<u16>())
                            .read()) as i32)
                        {
                            page = 0u16;
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            (((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                .wrapping_offset(70))
            .write(starter);
        } else {
            {
                dexNum = 0u16;
                'l9: loop {
                    if !(((((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .wrapping_offset(((dexNum) as i32) as isize))
                    .read()) as i32)
                        != ((starter) as i32))
                        && (((dexNum) as i32) < 71i32))
                    {
                        break 'l9;
                    }
                    'l10: {}
                    dexNum = (dexNum).wrapping_add(1);
                }
            }
            if ((dexNum) as i32)
                < ((((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_sub(1i32)
            {
                (((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                    .wrapping_offset(((dexNum) as i32) as isize))
                .write(
                    (((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .wrapping_offset(70))
                    .read(),
                );
                (((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                    .wrapping_offset(70))
                .write(starter);
            } else {
                (((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                    .wrapping_offset(70))
                .write(starter);
            }
        }
        ((((&raw mut sCreditsData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(148)
            .cast::<u16>())
        .write(71u16);
    }
}
