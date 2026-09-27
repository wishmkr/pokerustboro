//! Translated from `src/pokedex.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gPokedexOrder_Alphabetical gPokedexOrder_Weight gPokedexOrder_Height sOamData_ScrollBar sOamData_ScrollArrow sOamData_InterfaceText sOamData_RotatingPokeBall sOamData_SeenOwnText sOamData_Dex8x16 sSpriteAnim_ScrollBar sSpriteAnim_ScrollArrow sSpriteAnim_RotatingPokeBall sSpriteAnim_StartButton sSpriteAnim_SearchText sSpriteAnim_SelectButton sSpriteAnim_MenuText sSpriteAnim_SeenText sSpriteAnim_OwnText sSpriteAnim_HoennText sSpriteAnim_NationalText sSpriteAnim_HoennSeenOwnDigit0 sSpriteAnim_HoennSeenOwnDigit1 sSpriteAnim_HoennSeenOwnDigit2 sSpriteAnim_HoennSeenOwnDigit3 sSpriteAnim_HoennSeenOwnDigit4 sSpriteAnim_HoennSeenOwnDigit5 sSpriteAnim_HoennSeenOwnDigit6 sSpriteAnim_HoennSeenOwnDigit7 sSpriteAnim_HoennSeenOwnDigit8 sSpriteAnim_HoennSeenOwnDigit9 sSpriteAnim_NationalSeenOwnDigit0 sSpriteAnim_NationalSeenOwnDigit1 sSpriteAnim_NationalSeenOwnDigit2 sSpriteAnim_NationalSeenOwnDigit3 sSpriteAnim_NationalSeenOwnDigit4 sSpriteAnim_NationalSeenOwnDigit5 sSpriteAnim_NationalSeenOwnDigit6 sSpriteAnim_NationalSeenOwnDigit7 sSpriteAnim_NationalSeenOwnDigit8 sSpriteAnim_NationalSeenOwnDigit9 sSpriteAnim_DexListStartMenuCursor sSpriteAnimTable_ScrollBar sSpriteAnimTable_ScrollArrow sSpriteAnimTable_RotatingPokeBall sSpriteAnimTable_InterfaceText sSpriteAnimTable_SeenOwnText sSpriteAnimTable_HoennNationalText sSpriteAnimTable_HoennSeenOwnNumber sSpriteAnimTable_NationalSeenOwnNumber sSpriteAnimTable_DexListStartMenuCursor sScrollBarSpriteTemplate sScrollArrowSpriteTemplate sInterfaceTextSpriteTemplate sRotatingPokeBallSpriteTemplate sSeenOwnTextSpriteTemplate sHoennNationalTextSpriteTemplate sHoennDexSeenOwnNumberSpriteTemplate sNationalDexSeenOwnNumberSpriteTemplate sDexListStartMenuCursorSpriteTemplate sInterfaceSpriteSheet sInterfaceSpritePalette sScrollMonIncrements sScrollTimers sPokedex_BgTemplate sPokemonList_WindowTemplate sText_No000 sCaughtBall_Gfx sText_TenDashes sExpandedPlaceholder_PokedexDescription gDummyPokedexText gBulbasaurPokedexText gIvysaurPokedexText gVenusaurPokedexText gCharmanderPokedexText gCharmeleonPokedexText gCharizardPokedexText gSquirtlePokedexText gWartortlePokedexText gBlastoisePokedexText gCaterpiePokedexText gMetapodPokedexText gButterfreePokedexText gWeedlePokedexText gKakunaPokedexText gBeedrillPokedexText gPidgeyPokedexText gPidgeottoPokedexText gPidgeotPokedexText gRattataPokedexText gRaticatePokedexText gSpearowPokedexText gFearowPokedexText gEkansPokedexText gArbokPokedexText gPikachuPokedexText gRaichuPokedexText gSandshrewPokedexText gSandslashPokedexText gNidoranFPokedexText gNidorinaPokedexText gNidoqueenPokedexText gNidoranMPokedexText gNidorinoPokedexText gNidokingPokedexText gClefairyPokedexText gClefablePokedexText gVulpixPokedexText gNinetalesPokedexText gJigglypuffPokedexText gWigglytuffPokedexText gZubatPokedexText gGolbatPokedexText gOddishPokedexText gGloomPokedexText gVileplumePokedexText gParasPokedexText gParasectPokedexText gVenonatPokedexText gVenomothPokedexText gDiglettPokedexText gDugtrioPokedexText gMeowthPokedexText gPersianPokedexText gPsyduckPokedexText gGolduckPokedexText gMankeyPokedexText gPrimeapePokedexText gGrowlithePokedexText gArcaninePokedexText gPoliwagPokedexText gPoliwhirlPokedexText gPoliwrathPokedexText gAbraPokedexText gKadabraPokedexText gAlakazamPokedexText gMachopPokedexText gMachokePokedexText gMachampPokedexText gBellsproutPokedexText gWeepinbellPokedexText gVictreebelPokedexText gTentacoolPokedexText gTentacruelPokedexText gGeodudePokedexText gGravelerPokedexText gGolemPokedexText gPonytaPokedexText gRapidashPokedexText gSlowpokePokedexText gSlowbroPokedexText gMagnemitePokedexText gMagnetonPokedexText gFarfetchdPokedexText gDoduoPokedexText gDodrioPokedexText gSeelPokedexText gDewgongPokedexText gGrimerPokedexText gMukPokedexText gShellderPokedexText gCloysterPokedexText gGastlyPokedexText gHaunterPokedexText gGengarPokedexText gOnixPokedexText gDrowzeePokedexText gHypnoPokedexText gKrabbyPokedexText gKinglerPokedexText gVoltorbPokedexText gElectrodePokedexText gExeggcutePokedexText gExeggutorPokedexText gCubonePokedexText gMarowakPokedexText gHitmonleePokedexText gHitmonchanPokedexText gLickitungPokedexText gKoffingPokedexText gWeezingPokedexText gRhyhornPokedexText gRhydonPokedexText gChanseyPokedexText gTangelaPokedexText gKangaskhanPokedexText gHorseaPokedexText gSeadraPokedexText gGoldeenPokedexText gSeakingPokedexText gStaryuPokedexText gStarmiePokedexText gMrMimePokedexText gScytherPokedexText gJynxPokedexText gElectabuzzPokedexText gMagmarPokedexText gPinsirPokedexText gTaurosPokedexText gMagikarpPokedexText gGyaradosPokedexText gLaprasPokedexText gDittoPokedexText gEeveePokedexText gVaporeonPokedexText gJolteonPokedexText gFlareonPokedexText gPorygonPokedexText gOmanytePokedexText gOmastarPokedexText gKabutoPokedexText gKabutopsPokedexText gAerodactylPokedexText gSnorlaxPokedexText gArticunoPokedexText gZapdosPokedexText gMoltresPokedexText gDratiniPokedexText gDragonairPokedexText gDragonitePokedexText gMewtwoPokedexText gMewPokedexText gChikoritaPokedexText gBayleefPokedexText gMeganiumPokedexText gCyndaquilPokedexText gQuilavaPokedexText gTyphlosionPokedexText gTotodilePokedexText gCroconawPokedexText gFeraligatrPokedexText gSentretPokedexText gFurretPokedexText gHoothootPokedexText gNoctowlPokedexText gLedybaPokedexText gLedianPokedexText gSpinarakPokedexText gAriadosPokedexText gCrobatPokedexText gChinchouPokedexText gLanturnPokedexText gPichuPokedexText gCleffaPokedexText gIgglybuffPokedexText gTogepiPokedexText gTogeticPokedexText gNatuPokedexText gXatuPokedexText gMareepPokedexText gFlaaffyPokedexText gAmpharosPokedexText gBellossomPokedexText gMarillPokedexText gAzumarillPokedexText gSudowoodoPokedexText gPolitoedPokedexText gHoppipPokedexText gSkiploomPokedexText gJumpluffPokedexText gAipomPokedexText gSunkernPokedexText gSunfloraPokedexText gYanmaPokedexText gWooperPokedexText gQuagsirePokedexText gEspeonPokedexText gUmbreonPokedexText gMurkrowPokedexText gSlowkingPokedexText gMisdreavusPokedexText gUnownPokedexText gWobbuffetPokedexText gGirafarigPokedexText gPinecoPokedexText gForretressPokedexText gDunsparcePokedexText gGligarPokedexText gSteelixPokedexText gSnubbullPokedexText gGranbullPokedexText gQwilfishPokedexText gScizorPokedexText gShucklePokedexText gHeracrossPokedexText gSneaselPokedexText gTeddiursaPokedexText gUrsaringPokedexText gSlugmaPokedexText gMagcargoPokedexText gSwinubPokedexText gPiloswinePokedexText gCorsolaPokedexText gRemoraidPokedexText gOctilleryPokedexText gDelibirdPokedexText gMantinePokedexText gSkarmoryPokedexText gHoundourPokedexText gHoundoomPokedexText gKingdraPokedexText gPhanpyPokedexText gDonphanPokedexText gPorygon2PokedexText gStantlerPokedexText gSmearglePokedexText gTyroguePokedexText gHitmontopPokedexText gSmoochumPokedexText gElekidPokedexText gMagbyPokedexText gMiltankPokedexText gBlisseyPokedexText gRaikouPokedexText gEnteiPokedexText gSuicunePokedexText gLarvitarPokedexText gPupitarPokedexText gTyranitarPokedexText gLugiaPokedexText gHoOhPokedexText gCelebiPokedexText gTreeckoPokedexText gGrovylePokedexText gSceptilePokedexText gTorchicPokedexText gCombuskenPokedexText gBlazikenPokedexText gMudkipPokedexText gMarshtompPokedexText gSwampertPokedexText gPoochyenaPokedexText gMightyenaPokedexText gZigzagoonPokedexText gLinoonePokedexText gWurmplePokedexText gSilcoonPokedexText gBeautiflyPokedexText gCascoonPokedexText gDustoxPokedexText gLotadPokedexText gLombrePokedexText gLudicoloPokedexText gSeedotPokedexText gNuzleafPokedexText gShiftryPokedexText gTaillowPokedexText gSwellowPokedexText gWingullPokedexText gPelipperPokedexText gRaltsPokedexText gKirliaPokedexText gGardevoirPokedexText gSurskitPokedexText gMasquerainPokedexText gShroomishPokedexText gBreloomPokedexText gSlakothPokedexText gVigorothPokedexText gSlakingPokedexText gNincadaPokedexText gNinjaskPokedexText gShedinjaPokedexText gWhismurPokedexText gLoudredPokedexText gExploudPokedexText gMakuhitaPokedexText gHariyamaPokedexText gAzurillPokedexText gNosepassPokedexText gSkittyPokedexText gDelcattyPokedexText gSableyePokedexText gMawilePokedexText gAronPokedexText gLaironPokedexText gAggronPokedexText gMedititePokedexText gMedichamPokedexText gElectrikePokedexText gManectricPokedexText gPluslePokedexText gMinunPokedexText gVolbeatPokedexText gIllumisePokedexText gRoseliaPokedexText gGulpinPokedexText gSwalotPokedexText gCarvanhaPokedexText gSharpedoPokedexText gWailmerPokedexText gWailordPokedexText gNumelPokedexText gCameruptPokedexText gTorkoalPokedexText gSpoinkPokedexText gGrumpigPokedexText gSpindaPokedexText gTrapinchPokedexText gVibravaPokedexText gFlygonPokedexText gCacneaPokedexText gCacturnePokedexText gSwabluPokedexText gAltariaPokedexText gZangoosePokedexText gSeviperPokedexText gLunatonePokedexText gSolrockPokedexText gBarboachPokedexText gWhiscashPokedexText gCorphishPokedexText gCrawdauntPokedexText gBaltoyPokedexText gClaydolPokedexText gLileepPokedexText gCradilyPokedexText gAnorithPokedexText gArmaldoPokedexText gFeebasPokedexText gMiloticPokedexText gCastformPokedexText gKecleonPokedexText gShuppetPokedexText gBanettePokedexText gDuskullPokedexText gDusclopsPokedexText gTropiusPokedexText gChimechoPokedexText gAbsolPokedexText gWynautPokedexText gSnoruntPokedexText gGlaliePokedexText gSphealPokedexText gSealeoPokedexText gWalreinPokedexText gClamperlPokedexText gHuntailPokedexText gGorebyssPokedexText gRelicanthPokedexText gLuvdiscPokedexText gBagonPokedexText gShelgonPokedexText gSalamencePokedexText gBeldumPokedexText gMetangPokedexText gMetagrossPokedexText gRegirockPokedexText gRegicePokedexText gRegisteelPokedexText gLatiasPokedexText gLatiosPokedexText gKyogrePokedexText gGroudonPokedexText gRayquazaPokedexText gJirachiPokedexText gDeoxysPokedexText gPokedexEntries sSizeScreenSilhouette_Pal sInfoScreen_BgTemplate sInfoScreen_WindowTemplates sNewEntryInfoScreen_BgTemplate sNewEntryInfoScreen_WindowTemplates sText_TenDashes2 gMonFootprintTable sLetterSearchRanges sSearchMenuTopBarItems sSearchMenuItems sSearchMovementMap_SearchNatDex sSearchMovementMap_ShiftNatDex sSearchMovementMap_SearchHoennDex sSearchMovementMap_ShiftHoennDex sDexModeOptions sDexOrderOptions sDexSearchNameOptions sDexSearchColorOptions sDexSearchTypeOptions sPokedexModes sOrderOptions sDexSearchTypeIds sSearchOptions sSearchMenu_BgTemplate sSearchMenu_WindowTemplate
#[allow(unused_imports)]
use crate::data::pokedex::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokedexView: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLastSelectedPokemon: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokeBallRotation: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokedexListItem: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gUnusedPokedexU8: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gPokedexVBlankCB: Option<unsafe extern "C" fn()> = None;

unsafe extern "C" {
    static mut gDexCryScreenState: u8;
    static mut gMPlayInfo_BGM: u8;
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gPokedexBgHoenn_Pal: u8;
    static mut gPokedexBgNational_Pal: u8;
    static mut gPokedexCryScreen_Tilemap: u8;
    static mut gPokedexInfoScreen_Tilemap: u8;
    static mut gPokedexListUnderlay_Tilemap: u8;
    static mut gPokedexList_Tilemap: u8;
    static mut gPokedexMenu_Gfx: u8;
    static mut gPokedexScreenSelectBarMain_Tilemap: u8;
    static mut gPokedexScreenSelectBarSubmenu_Tilemap: u8;
    static mut gPokedexSearchMenuHoenn_Tilemap: u8;
    static mut gPokedexSearchMenuNational_Tilemap: u8;
    static mut gPokedexSearchMenu_Gfx: u8;
    static mut gPokedexSearchMenu_Pal: u8;
    static mut gPokedexSearchResults_Pal: u8;
    static mut gPokedexSizeScreen_Tilemap: u8;
    static mut gPokedexStartMenuMain_Tilemap: u8;
    static mut gPokedexStartMenuSearchResults_Tilemap: u8;
    static mut gReservedSpritePaletteCount: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSineTable: u8;
    static mut gSpeciesInfo: u8;
    static mut gSpeciesNames: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gText_5MarksPokemon: u8;
    static mut gText_CryOf: u8;
    static mut gText_HTHeight: u8;
    static mut gText_NoMatchingPkmnWereFound: u8;
    static mut gText_NumberClear01: u8;
    static mut gText_PokedexRegistration: u8;
    static mut gText_SearchCompleted: u8;
    static mut gText_SearchingPleaseWait: u8;
    static mut gText_SelectorArrow: u8;
    static mut gText_SizeComparedTo: u8;
    static mut gText_UnkHeight: u8;
    static mut gText_UnkWeight: u8;
    static mut gText_WTWeight: u8;
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
    fn BlitBitmapToWindow(a0: u8, a1: *mut u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn BuildOamBuffer();
    fn CB2_ReturnToFieldWithOpenMenu();
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyMonCategoryText(a0: i32, a1: *mut u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyToWindowPixelBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
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
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTrainerPicSprite(a0: u16, a1: u8, a2: i16, a3: i16, a4: u8, a5: u16) -> u16;
    fn CryScreenPlayButton(a0: u16);
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DisableNationalPokedex();
    fn EnableInterrupts(a0: u16);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeAndDestroyMonPicSprite(a0: u16) -> u16;
    fn FreeAndDestroyTrainerPicSprite(a0: u16) -> u16;
    fn FreeCryScreen();
    fn GetBgTilemapBuffer(a0: u8) -> *mut u8;
    fn GetMonSpritePalFromSpeciesAndPersonality(a0: u16, a1: u32, a2: u32) -> *mut u32;
    fn GetOverworldTextboxPalettePtr() -> *mut u16;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn HideBg(a0: u8);
    fn HoennToNationalOrder(a0: u16) -> u16;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsCryPlaying() -> u8;
    fn IsCryPlayingOrClearCrySongs() -> u8;
    fn IsNationalPokedexEnabled() -> u32;
    fn IsSEPlaying() -> u8;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadCryMeter(a0: *mut u8, a1: u8) -> u8;
    fn LoadCryWaveformWindow(a0: *mut u8, a1: u8) -> u8;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut u8);
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
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetOamMatrix(a0: u8, a1: u16, a2: u16, a3: u16, a4: u16);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn ShowPokedexAreaScreen(a0: u16, a1: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StopCryAndClearCrySongs();
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringLength(a0: *mut u8) -> u16;
    fn TransferPlttBuffer();
    fn UpdateCryWaveformWindow(a0: u8);
    fn UpdatePaletteFade() -> u8;
    fn m4aMPlayContinue(a0: *mut u8);
    fn m4aMPlayStop(a0: *mut u8);
    fn m4aMPlayVolumeControl(a0: *mut u8, a1: u16, a2: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetPokedex() {
    unsafe {
        let mut i: u16 = 0u16;
        ((&raw mut sLastSelectedPokemon).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut sPokeBallRotation).cast::<u8>().cast::<u8>()).write(64u8);
        ((&raw mut gUnusedPokedexU8).cast::<u8>().cast::<u8>()).write(0u8);
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24)).wrapping_add(1))
            .write(0u8);
        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24)).write(0u8);
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24)).wrapping_add(2))
            .write(0u8);
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24)).wrapping_add(3))
            .write(0u8);
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24))
            .wrapping_add(4)
            .cast::<u32>())
        .write(0u32);
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24))
            .wrapping_add(8)
            .cast::<u32>())
        .write(0u32);
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24))
            .wrapping_add(12)
            .cast::<u32>())
        .write(0u32);
        DisableNationalPokedex();
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < (crate::c::div_i32(412i32, 8i32)).wrapping_add(
                        (if (crate::c::rem_i32(412i32, 8i32)) != 0 {
                            1i32
                        } else {
                            0i32
                        }),
                    ))
                {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24))
                        .wrapping_add(16))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24))
                        .wrapping_add(68))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2440))
                        .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(15140))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetPokedexScrollPositions() {
    unsafe {
        ((&raw mut sLastSelectedPokemon).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut sPokeBallRotation).cast::<u8>().cast::<u8>()).write(64u8);
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_Pokedex() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn ResetPokedexView(pokedexView: *mut u8) {
    unsafe {
        let mut pokedexView = pokedexView;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 386i32) {
                    break 'l1;
                }
                'l2: {
                    ((((pokedexView).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                    .write(65535u16);
                    crate::c::bf_write(
                        (((pokedexView).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2),
                        0,
                        1,
                        (0u16) as i32,
                    );
                    crate::c::bf_write(
                        (((pokedexView).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2),
                        1,
                        1,
                        (0u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((pokedexView).cast::<u8>()).wrapping_offset(1544)).cast::<u16>()).write(0u16);
        crate::c::bf_write(
            (((pokedexView).cast::<u8>()).wrapping_offset(1544)).wrapping_add(2),
            0,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((pokedexView).cast::<u8>()).wrapping_offset(1544)).wrapping_add(2),
            1,
            1,
            (0u16) as i32,
        );
        ((pokedexView).wrapping_add(1548).cast::<u16>()).write(0u16);
        ((pokedexView).wrapping_add(1550).cast::<u16>()).write(0u16);
        ((pokedexView).wrapping_add(1552).cast::<u16>()).write(0u16);
        ((pokedexView).wrapping_add(1554).cast::<u16>()).write(0u16);
        ((pokedexView).wrapping_add(1556).cast::<u16>()).write(0u16);
        ((pokedexView).wrapping_add(1558).cast::<u16>()).write(0u16);
        ((pokedexView).wrapping_add(1560).cast::<u16>()).write(0u16);
        ((pokedexView).wrapping_add(1562).cast::<u16>()).write(0u16);
        ((pokedexView).wrapping_add(1564).cast::<u16>()).write(0u16);
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l3;
                }
                'l4: {
                    ((((pokedexView).wrapping_add(1566)).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(65535u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((pokedexView).wrapping_add(1576).cast::<u16>()).write(0u16);
        ((pokedexView).wrapping_add(1578).cast::<u16>()).write(0u16);
        ((pokedexView).wrapping_add(1580)).write(0u8);
        ((pokedexView).wrapping_add(1581)).write(0u8);
        ((pokedexView).wrapping_add(1582)).write(0u8);
        ((pokedexView).wrapping_add(1583)).write(0u8);
        ((pokedexView).wrapping_add(1584).cast::<i16>()).write(0i16);
        ((pokedexView).wrapping_add(1586).cast::<i16>()).write(0i16);
        ((pokedexView).wrapping_add(1588).cast::<u16>()).write(0u16);
        ((pokedexView).wrapping_add(1590).cast::<u16>()).write(0u16);
        ((pokedexView).wrapping_add(1592).cast::<u16>()).write(0u16);
        {
            i = 0u16;
            'l5: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 2u32)) {
                    break 'l5;
                }
                'l6: {
                    ((((pokedexView).wrapping_add(1594)).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((pokedexView).wrapping_add(1610)).write(0u8);
        ((pokedexView).wrapping_add(1611)).write(0u8);
        crate::c::bf_write((pokedexView).wrapping_add(1612), 0, 1, (0u8) as i32);
        ((pokedexView).wrapping_add(1613)).write(0u8);
        ((pokedexView).wrapping_add(1614)).write(0u8);
        ((pokedexView).wrapping_add(1615)).write(0u8);
        ((pokedexView).wrapping_add(1616).cast::<u16>()).write(0u16);
        ((pokedexView).wrapping_add(1618).cast::<i16>()).write(0i16);
        {
            i = 0u16;
            'l7: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 1u32)) {
                    break 'l7;
                }
                'l8: {
                    ((((pokedexView).wrapping_add(1620)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l9: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 1u32)) {
                    break 'l9;
                }
                'l10: {
                    ((((pokedexView).wrapping_add(1628)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_OpenPokedex() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 || !__matched {
                SetVBlankCallback(None);
                ResetOtherVideoRegisters(0u16);
                {
                    let mut _dest: *mut u8 = ((100663296i32) as usize as *mut u8);
                    let mut _size: u32 = 98304u32;
                    'l2: loop {
                        if !((1i32) != 0) {
                            break 'l2;
                        }
                        'l3: loop {
                            'l4: {
                                {
                                    let mut tmp: u16 = 0u16;
                                    (&raw mut tmp).write_volatile(0u16);
                                    'l5: loop {
                                        'l6: {
                                            {
                                                let mut dmaRegs: *mut u32 =
                                                    ((67109076i32) as usize as *mut u32);
                                                crate::c::volatile_write(
                                                    dmaRegs,
                                                    ((&raw mut tmp) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(1),
                                                    ((_dest) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(2),
                                                    (((-2130706432i32)
                                                        | crate::c::div_i32(
                                                            4096i32,
                                                            crate::c::div_i32(16i32, 8i32),
                                                        ))
                                                        as u32),
                                                );
                                                let _ =
                                                    ((dmaRegs).wrapping_offset(2)).read_volatile();
                                            }
                                        }
                                        if !((0i32) != 0) {
                                            break 'l5;
                                        }
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l3;
                            }
                        }
                        _dest = (_dest).wrapping_offset(4096);
                        _size = (_size).wrapping_sub(4096u32);
                        if _size <= 4096u32 {
                            'l7: loop {
                                'l8: {
                                    {
                                        let mut tmp: u16 = 0u16;
                                        (&raw mut tmp).write_volatile(0u16);
                                        'l9: loop {
                                            'l10: {
                                                {
                                                    let mut dmaRegs: *mut u32 =
                                                        ((67109076i32) as usize as *mut u32);
                                                    crate::c::volatile_write(
                                                        dmaRegs,
                                                        ((&raw mut tmp) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(1),
                                                        ((_dest) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(2),
                                                        (2164260864u32
                                                            | crate::c::div_u32(
                                                                _size,
                                                                ((crate::c::div_i32(16i32, 8i32))
                                                                    as u32),
                                                            )),
                                                    );
                                                    let _ = ((dmaRegs).wrapping_offset(2))
                                                        .read_volatile();
                                                }
                                            }
                                            if !((0i32) != 0) {
                                                break 'l9;
                                            }
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l7;
                                }
                            }
                            break 'l2;
                        }
                    }
                }
                'l11: loop {
                    'l12: {
                        {
                            let mut _dest: *mut u32 = ((117440512i32) as usize as *mut u32);
                            let mut _size: u32 = 1024u32;
                            'l13: loop {
                                'l14: {
                                    {
                                        let mut tmp: u32 = 0u32;
                                        (&raw mut tmp).write_volatile(0u32);
                                        'l15: loop {
                                            'l16: {
                                                {
                                                    let mut dmaRegs: *mut u32 =
                                                        ((67109076i32) as usize as *mut u32);
                                                    crate::c::volatile_write(
                                                        dmaRegs,
                                                        ((&raw mut tmp) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(1),
                                                        ((_dest) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(2),
                                                        (2231369728u32
                                                            | crate::c::div_u32(
                                                                _size,
                                                                ((crate::c::div_i32(32i32, 8i32))
                                                                    as u32),
                                                            )),
                                                    );
                                                    let _ = ((dmaRegs).wrapping_offset(2))
                                                        .read_volatile();
                                                }
                                            }
                                            if !((0i32) != 0) {
                                                break 'l15;
                                            }
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l13;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l11;
                    }
                }
                'l17: loop {
                    'l18: {
                        {
                            let mut _dest: *mut u16 = ((83886080i32) as usize as *mut u16);
                            let mut _size: u32 = 1024u32;
                            'l19: loop {
                                'l20: {
                                    {
                                        let mut tmp: u16 = 0u16;
                                        (&raw mut tmp).write_volatile(0u16);
                                        'l21: loop {
                                            'l22: {
                                                {
                                                    let mut dmaRegs: *mut u32 =
                                                        ((67109076i32) as usize as *mut u32);
                                                    crate::c::volatile_write(
                                                        dmaRegs,
                                                        ((&raw mut tmp) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(1),
                                                        ((_dest) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(2),
                                                        (2164260864u32
                                                            | crate::c::div_u32(
                                                                _size,
                                                                ((crate::c::div_i32(16i32, 8i32))
                                                                    as u32),
                                                            )),
                                                    );
                                                    let _ = ((dmaRegs).wrapping_offset(2))
                                                        .read_volatile();
                                                }
                                            }
                                            if !((0i32) != 0) {
                                                break 'l21;
                                            }
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l19;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l17;
                    }
                }
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ScanlineEffect_Stop();
                ResetTasks();
                ResetSpriteData();
                ResetPaletteFade();
                FreeAllSpritePalettes();
                ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(8u8);
                ResetAllPicSprites();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                    .write(AllocZeroed(1636u32));
                ResetPokedexView(((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read());
                CreateTask(Some(Task_OpenPokedexMainPage), 0u8);
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1554)
                    .cast::<u16>())
                .write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24))
                        .wrapping_add(1))
                    .read()) as u16),
                );
                if !((IsNationalPokedexEnabled()) != 0) {
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1554)
                        .cast::<u16>())
                    .write(0u16);
                }
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1558)
                    .cast::<u16>())
                .write(
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24))
                        .read()) as u16),
                );
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1550)
                    .cast::<u16>())
                .write(((&raw mut sLastSelectedPokemon).cast::<u8>().cast::<u16>()).read());
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1580))
                .write(((&raw mut sPokeBallRotation).cast::<u8>().cast::<u8>()).read());
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1613))
                .write(0u8);
                if !((IsNationalPokedexEnabled()) != 0) {
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1562)
                        .cast::<u16>())
                    .write(GetHoennPokedexCount(0u8));
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1564)
                        .cast::<u16>())
                    .write(GetHoennPokedexCount(1u8));
                } else {
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1562)
                        .cast::<u16>())
                    .write(GetNationalPokedexCount(0u8));
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1564)
                        .cast::<u16>())
                    .write(GetNationalPokedexCount(1u8));
                }
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1581))
                .write(8u8);
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                EnableInterrupts(1u16);
                SetVBlankCallback(Some(VBlankCB_Pokedex));
                SetMainCallback2(Some(CB2_Pokedex));
                CreatePokedexList(
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1554)
                        .cast::<u16>())
                    .read()) as u8),
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1558)
                        .cast::<u16>())
                    .read()) as u8),
                );
                m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 128u16);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_Pokedex() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_OpenPokedexMainPage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        crate::c::bf_write(
            (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1612),
            0,
            1,
            (0u8) as i32,
        );
        if (LoadPokedexListPage(0u8)) != 0 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandlePokedexInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandlePokedexInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetGpuReg(
            18u8,
            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1618)
                .cast::<i16>())
            .read()) as u16),
        );
        if (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1618)
            .cast::<i16>())
        .read())
            != 0
        {
            let __p1 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1618)
                .cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(8i32)) as i16));
        } else {
            if (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0)
                && ((crate::c::bf_read(
                    (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1550)
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(2),
                    0,
                    1,
                    false,
                ) as u16)
                    != 0)
            {
                UpdateSelectedMonSpriteId();
                BeginNormalPaletteFade(
                    ((!(crate::c::shl_i32(
                        1i32,
                        ((((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1574)
                                .cast::<u16>())
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            4,
                            4,
                            false,
                        ) as u16) as i32)
                            .wrapping_add(16i32)) as u32),
                    ))) as u32),
                    0i8,
                    0u8,
                    16u8,
                    0u16,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1574)
                        .cast::<u16>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_MoveMonForInfoScreen));
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_OpenInfoScreenAfterMonMovement));
                PlaySE(21u16);
                FreeWindowAndBgBuffers();
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 8i32)
                    != 0
                {
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1618)
                        .cast::<i16>())
                    .write(0i16);
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1615))
                    .write(1u8);
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1616)
                        .cast::<u16>())
                    .write(0u16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_HandlePokedexStartMenuInput));
                    PlaySE(5u16);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 4i32)
                        != 0
                    {
                        PlaySE(5u16);
                        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(((LoadSearchMenu()) as i16));
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1614))
                        .write(0u8);
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1578)
                            .cast::<u16>())
                        .write(
                            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1580))
                            .read()) as u16),
                        );
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1552)
                            .cast::<u16>())
                        .write(
                            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1550)
                                .cast::<u16>())
                            .read(),
                        );
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1556)
                            .cast::<u16>())
                        .write(
                            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1554)
                                .cast::<u16>())
                            .read(),
                        );
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1560)
                            .cast::<u16>())
                        .write(
                            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1558)
                                .cast::<u16>())
                            .read(),
                        );
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Task_WaitForExitSearch));
                        PlaySE(2u16);
                        FreeWindowAndBgBuffers();
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 2i32)
                            != 0
                        {
                            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                            ((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .cast::<Option<unsafe extern "C" fn(u8)>>())
                            .write(Some(Task_ClosePokedex));
                            PlaySE(3u16);
                        } else {
                            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1550)
                                .cast::<u16>())
                            .write(TryDoPokedexScroll(
                                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(1550)
                                    .cast::<u16>())
                                .read(),
                                14u16,
                            ));
                            if (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1582))
                            .read())
                                != 0
                            {
                                ((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .cast::<Option<unsafe extern "C" fn(u8)>>())
                                .write(Some(Task_WaitForScroll));
                            }
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForScroll(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (UpdateDexListScroll(
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1583))
                .read(),
            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1588)
                .cast::<u16>())
            .read()) as u8),
            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1590)
                .cast::<u16>())
            .read()) as u8),
        )) != 0
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandlePokedexInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandlePokedexStartMenuInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetGpuReg(
            18u8,
            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1618)
                .cast::<i16>())
            .read()) as u16),
        );
        if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1618)
            .cast::<i16>())
        .read()) as i32)
            != 80i32
        {
            let __p1 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1618)
                .cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0
            {
                'l1: {
                    let __sw2 = ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(1616)
                    .cast::<u16>())
                    .read()) as i32);
                    let __matched =
                        __sw2 == 0i32 || __sw2 == 1i32 || __sw2 == 2i32 || __sw2 == 3i32;
                    if __sw2 == 0i32 || !__matched {
                        let __p3 = ((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>();
                        (__p3).write((((((__p3).read()) as i32) | 8i32) as u16));
                        break 'l1;
                    }
                    if __sw2 == 1i32 {
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1550)
                            .cast::<u16>())
                        .write(0u16);
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1580))
                        .write(64u8);
                        ClearMonSprites();
                        CreateMonSpritesAtPos(
                            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1550)
                                .cast::<u16>())
                            .read(),
                            14u16,
                        );
                        let __p4 = ((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>();
                        (__p4).write((((((__p4).read()) as i32) | 8i32) as u16));
                        break 'l1;
                    }
                    if __sw2 == 2i32 {
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1550)
                            .cast::<u16>())
                        .write(
                            ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1548)
                                .cast::<u16>())
                            .read()) as i32)
                                .wrapping_sub(1i32)) as u16),
                        );
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1580))
                        .write(
                            (((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(1548)
                            .cast::<u16>())
                            .read()) as i32)
                                .wrapping_mul(16i32))
                            .wrapping_add(48i32)) as u8),
                        );
                        ClearMonSprites();
                        CreateMonSpritesAtPos(
                            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1550)
                                .cast::<u16>())
                            .read(),
                            14u16,
                        );
                        let __p5 = ((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>();
                        (__p5).write((((((__p5).read()) as i32) | 8i32) as u16));
                        break 'l1;
                    }
                    if __sw2 == 3i32 {
                        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Task_ClosePokedex));
                        PlaySE(3u16);
                        break 'l1;
                    }
                }
            }
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 10i32)
                != 0
            {
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1615))
                .write(0u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_HandlePokedexInput));
                PlaySE(5u16);
            } else {
                if (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0)
                    && (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1616)
                        .cast::<u16>())
                    .read()) as i32)
                        != 0i32)
                {
                    let __p6 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1616)
                        .cast::<u16>();
                    (__p6).write(((__p6).read()).wrapping_sub(1));
                    PlaySE(5u16);
                } else {
                    if (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(48)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0)
                        && (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1616)
                            .cast::<u16>())
                        .read()) as i32)
                            < 3i32)
                    {
                        let __p7 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(1616)
                        .cast::<u16>();
                        (__p7).write(((__p7).read()).wrapping_add(1));
                        PlaySE(5u16);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_OpenInfoScreenAfterMonMovement(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1574)
                .cast::<u16>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(32)
        .cast::<i16>())
        .read()) as i32)
            == 48i32)
            && (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1574)
                    .cast::<u16>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                == 56i32)
        {
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1611))
                .write(
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1610))
                    .read(),
                );
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(
                ((LoadInfoScreen(
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1550)
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 4,
                    ),
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1574)
                        .cast::<u16>())
                    .read()) as u8),
                )) as i16),
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_WaitForExitInfoScreen));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForExitInfoScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(4))
        .read())
            != 0
        {
            if ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1610))
            .read()) as i32)
                == 1i32)
                && (!((IsInfoScreenScrolling(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as u8),
                )) != 0)))
                && ((TryDoInfoScreenScroll()) != 0)
            {
                StartInfoScreenScroll(
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1550)
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 4,
                    ),
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as u8),
                );
            }
        } else {
            ((&raw mut sLastSelectedPokemon).cast::<u8>().cast::<u16>()).write(
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1550)
                    .cast::<u16>())
                .read(),
            );
            ((&raw mut sPokeBallRotation).cast::<u8>().cast::<u8>()).write(
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1580))
                .read(),
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_OpenPokedexMainPage));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForExitSearch(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(4))
        .read())
            != 0)
        {
            ClearMonSprites();
            if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1614))
            .read()) as i32)
                != 0i32
            {
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1550)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1580))
                .write(64u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_OpenSearchResults));
            } else {
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1580))
                .write(
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1578)
                        .cast::<u16>())
                    .read()) as u8),
                );
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1550)
                    .cast::<u16>())
                .write(
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1552)
                        .cast::<u16>())
                    .read(),
                );
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1554)
                    .cast::<u16>())
                .write(
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1556)
                        .cast::<u16>())
                    .read(),
                );
                if !((IsNationalPokedexEnabled()) != 0) {
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1554)
                        .cast::<u16>())
                    .write(0u16);
                }
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1558)
                    .cast::<u16>())
                .write(
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1560)
                        .cast::<u16>())
                    .read(),
                );
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_OpenPokedexMainPage));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ClosePokedex(taskId: u8) {
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
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24))
                .wrapping_add(1))
            .write(
                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1554)
                    .cast::<u16>())
                .read()) as u8),
            );
            if !((IsNationalPokedexEnabled()) != 0) {
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24))
                    .wrapping_add(1))
                .write(0u8);
            }
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24)).write(
                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1558)
                    .cast::<u16>())
                .read()) as u8),
            );
            ClearMonSprites();
            FreeWindowAndBgBuffers();
            DestroyTask(taskId);
            SetMainCallback2(Some(CB2_ReturnToFieldWithOpenMenu));
            m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 256u16);
            Free(((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read());
        }
    }
}
pub(crate) unsafe extern "C" fn Task_OpenSearchResults(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        crate::c::bf_write(
            (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1612),
            0,
            1,
            (1u8) as i32,
        );
        if (LoadPokedexListPage(3u8)) != 0 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleSearchResultsInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSearchResultsInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetGpuReg(
            18u8,
            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1618)
                .cast::<i16>())
            .read()) as u16),
        );
        if (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1618)
            .cast::<i16>())
        .read())
            != 0
        {
            let __p1 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1618)
                .cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(8i32)) as i16));
        } else {
            if (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0)
                && ((crate::c::bf_read(
                    (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1550)
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(2),
                    0,
                    1,
                    false,
                ) as u16)
                    != 0)
            {
                let mut a: u32 = 0u32;
                UpdateSelectedMonSpriteId();
                a = ((crate::c::shl_i32(
                    1i32,
                    ((((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1574)
                                .cast::<u16>())
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(5),
                        4,
                        4,
                        false,
                    ) as u16) as i32)
                        .wrapping_add(16i32)) as u32),
                )) as u32);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1574)
                        .cast::<u16>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_MoveMonForInfoScreen));
                BeginNormalPaletteFade(!(a), 0i8, 0u8, 16u8, 0u16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_OpenSearchResultsInfoScreenAfterMonMovement));
                PlaySE(21u16);
                FreeWindowAndBgBuffers();
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 8i32)
                    != 0
                {
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1618)
                        .cast::<i16>())
                    .write(0i16);
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1615))
                    .write(1u8);
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1616)
                        .cast::<u16>())
                    .write(0u16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_HandleSearchResultsStartMenuInput));
                    PlaySE(5u16);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 4i32)
                        != 0
                    {
                        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(((LoadSearchMenu()) as i16));
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1614))
                        .write(0u8);
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Task_WaitForExitSearch));
                        PlaySE(2u16);
                        FreeWindowAndBgBuffers();
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 2i32)
                            != 0
                        {
                            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                            ((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .cast::<Option<unsafe extern "C" fn(u8)>>())
                            .write(Some(Task_ReturnToPokedexFromSearchResults));
                            PlaySE(3u16);
                        } else {
                            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1550)
                                .cast::<u16>())
                            .write(TryDoPokedexScroll(
                                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(1550)
                                    .cast::<u16>())
                                .read(),
                                14u16,
                            ));
                            if (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1582))
                            .read())
                                != 0
                            {
                                ((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .cast::<Option<unsafe extern "C" fn(u8)>>())
                                .write(Some(Task_WaitForSearchResultsScroll));
                            }
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForSearchResultsScroll(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (UpdateDexListScroll(
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1583))
                .read(),
            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1588)
                .cast::<u16>())
            .read()) as u8),
            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1590)
                .cast::<u16>())
            .read()) as u8),
        )) != 0
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleSearchResultsInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSearchResultsStartMenuInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetGpuReg(
            18u8,
            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1618)
                .cast::<i16>())
            .read()) as u16),
        );
        if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1618)
            .cast::<i16>())
        .read()) as i32)
            != 96i32
        {
            let __p1 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1618)
                .cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0
            {
                'l1: {
                    let __sw2 = ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(1616)
                    .cast::<u16>())
                    .read()) as i32);
                    let __matched = __sw2 == 0i32
                        || __sw2 == 1i32
                        || __sw2 == 2i32
                        || __sw2 == 3i32
                        || __sw2 == 4i32;
                    if __sw2 == 0i32 || !__matched {
                        let __p3 = ((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>();
                        (__p3).write((((((__p3).read()) as i32) | 8i32) as u16));
                        break 'l1;
                    }
                    if __sw2 == 1i32 {
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1550)
                            .cast::<u16>())
                        .write(0u16);
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1580))
                        .write(64u8);
                        ClearMonSprites();
                        CreateMonSpritesAtPos(
                            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1550)
                                .cast::<u16>())
                            .read(),
                            14u16,
                        );
                        let __p4 = ((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>();
                        (__p4).write((((((__p4).read()) as i32) | 8i32) as u16));
                        break 'l1;
                    }
                    if __sw2 == 2i32 {
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1550)
                            .cast::<u16>())
                        .write(
                            ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1548)
                                .cast::<u16>())
                            .read()) as i32)
                                .wrapping_sub(1i32)) as u16),
                        );
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1580))
                        .write(
                            (((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(1548)
                            .cast::<u16>())
                            .read()) as i32)
                                .wrapping_mul(16i32))
                            .wrapping_add(48i32)) as u8),
                        );
                        ClearMonSprites();
                        CreateMonSpritesAtPos(
                            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1550)
                                .cast::<u16>())
                            .read(),
                            14u16,
                        );
                        let __p5 = ((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>();
                        (__p5).write((((((__p5).read()) as i32) | 8i32) as u16));
                        break 'l1;
                    }
                    if __sw2 == 3i32 {
                        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Task_ReturnToPokedexFromSearchResults));
                        PlaySE(52u16);
                        break 'l1;
                    }
                    if __sw2 == 4i32 {
                        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Task_ClosePokedexFromSearchResultsStartMenu));
                        PlaySE(3u16);
                        break 'l1;
                    }
                }
            }
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 10i32)
                != 0
            {
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1615))
                .write(0u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_HandleSearchResultsInput));
                PlaySE(5u16);
            } else {
                if (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0)
                    && ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1616)
                        .cast::<u16>())
                    .read())
                        != 0)
                {
                    let __p6 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1616)
                        .cast::<u16>();
                    (__p6).write(((__p6).read()).wrapping_sub(1));
                    PlaySE(5u16);
                } else {
                    if (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(48)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0)
                        && (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1616)
                            .cast::<u16>())
                        .read()) as i32)
                            < 4i32)
                    {
                        let __p7 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(1616)
                        .cast::<u16>();
                        (__p7).write(((__p7).read()).wrapping_add(1));
                        PlaySE(5u16);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_OpenSearchResultsInfoScreenAfterMonMovement(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1574)
                .cast::<u16>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(32)
        .cast::<i16>())
        .read()) as i32)
            == 48i32)
            && (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1574)
                    .cast::<u16>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                == 56i32)
        {
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1611))
                .write(
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1610))
                    .read(),
                );
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(
                ((LoadInfoScreen(
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1550)
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 4,
                    ),
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1574)
                        .cast::<u16>())
                    .read()) as u8),
                )) as i16),
            );
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1574)
                .cast::<u16>())
            .write(65535u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_WaitForExitSearchResultsInfoScreen));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForExitSearchResultsInfoScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(4))
        .read())
            != 0
        {
            if ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1610))
            .read()) as i32)
                == 1i32)
                && (!((IsInfoScreenScrolling(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as u8),
                )) != 0)))
                && ((TryDoInfoScreenScroll()) != 0)
            {
                StartInfoScreenScroll(
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1550)
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 4,
                    ),
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as u8),
                );
            }
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_OpenSearchResults));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReturnToPokedexFromSearchResults(taskId: u8) {
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
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1580))
                .write(
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1578)
                        .cast::<u16>())
                    .read()) as u8),
                );
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1550)
                .cast::<u16>())
            .write(
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1552)
                    .cast::<u16>())
                .read(),
            );
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1554)
                .cast::<u16>())
            .write(
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1556)
                    .cast::<u16>())
                .read(),
            );
            if !((IsNationalPokedexEnabled()) != 0) {
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1554)
                    .cast::<u16>())
                .write(0u16);
            }
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1558)
                .cast::<u16>())
            .write(
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1560)
                    .cast::<u16>())
                .read(),
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_OpenPokedexMainPage));
            ClearMonSprites();
            FreeWindowAndBgBuffers();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ClosePokedexFromSearchResultsStartMenu(taskId: u8) {
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
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1580))
                .write(
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1578)
                        .cast::<u16>())
                    .read()) as u8),
                );
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1550)
                .cast::<u16>())
            .write(
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1552)
                    .cast::<u16>())
                .read(),
            );
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1554)
                .cast::<u16>())
            .write(
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1556)
                    .cast::<u16>())
                .read(),
            );
            if !((IsNationalPokedexEnabled()) != 0) {
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1554)
                    .cast::<u16>())
                .write(0u16);
            }
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1558)
                .cast::<u16>())
            .write(
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1560)
                    .cast::<u16>())
                .read(),
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ClosePokedex));
        }
    }
}
pub(crate) unsafe extern "C" fn LoadPokedexListPage(page: u8) -> u8 {
    unsafe {
        let mut page = page;
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32;
            if __sw1 == 0i32 || !__matched {
                if (crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0
                {
                    return 0u8;
                }
                SetVBlankCallback(None);
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1610))
                .write(page);
                ResetOtherVideoRegisters(0u16);
                SetGpuReg(
                    26u8,
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1581))
                    .read()) as u16),
                );
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const sPokedex_BgTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(16u32, 4u32)) as u8),
                );
                SetBgTilemapBuffer(3u8, AllocZeroed(2048u32));
                SetBgTilemapBuffer(2u8, AllocZeroed(2048u32));
                SetBgTilemapBuffer(1u8, AllocZeroed(2048u32));
                SetBgTilemapBuffer(0u8, AllocZeroed(2048u32));
                DecompressAndLoadBgGfxUsingHeap(
                    3u8,
                    (((&raw mut gPokedexMenu_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    8192u32,
                    0u16,
                    0u8,
                );
                CopyToBgTilemapBuffer(
                    1u8,
                    (((&raw mut gPokedexList_Tilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u16,
                    0u16,
                );
                CopyToBgTilemapBuffer(
                    3u8,
                    (((&raw mut gPokedexListUnderlay_Tilemap).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    0u16,
                    0u16,
                );
                if ((page) as i32) == 0i32 {
                    CopyToBgTilemapBuffer(
                        0u8,
                        (((&raw mut gPokedexStartMenuMain_Tilemap).cast::<u32>()).cast::<u32>())
                            .cast::<u8>(),
                        0u16,
                        640u16,
                    );
                } else {
                    CopyToBgTilemapBuffer(
                        0u8,
                        (((&raw mut gPokedexStartMenuSearchResults_Tilemap).cast::<u32>())
                            .cast::<u32>())
                        .cast::<u8>(),
                        0u16,
                        640u16,
                    );
                }
                ResetPaletteFade();
                if ((page) as i32) == 0i32 {
                    crate::c::bf_write(
                        (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1612),
                        0,
                        1,
                        (0u8) as i32,
                    );
                } else {
                    crate::c::bf_write(
                        (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1612),
                        0,
                        1,
                        (1u8) as i32,
                    );
                }
                LoadPokedexBgPalette(
                    (crate::c::bf_read(
                        (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1612),
                        0,
                        1,
                        false,
                    ) as u8),
                );
                InitWindows(
                    ((&raw const sPokemonList_WindowTemplate)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                DeactivateAllTextPrinters();
                PutWindowTilemap(0u8);
                CopyWindowToVram(0u8, 3u8);
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ResetSpriteData();
                FreeAllSpritePalettes();
                ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(8u8);
                LoadCompressedSpriteSheet(
                    ((&raw const sInterfaceSpriteSheet).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                LoadSpritePalettes(
                    ((&raw const sInterfaceSpritePalette).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                CreateInterfaceSprites(page);
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((page) as i32) == 0i32 {
                    CreatePokedexList(
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1554)
                            .cast::<u16>())
                        .read()) as u8),
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1558)
                            .cast::<u16>())
                        .read()) as u8),
                    );
                }
                CreateMonSpritesAtPos(
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1550)
                        .cast::<u16>())
                    .read(),
                    14u16,
                );
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1615))
                .write(0u8);
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1618)
                    .cast::<i16>())
                .write(0i16);
                CopyBgTilemapBufferToVram(0u8);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(2u8);
                CopyBgTilemapBufferToVram(3u8);
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                SetVBlankCallback(Some(VBlankCB_Pokedex));
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                SetGpuReg(72u8, 16191u16);
                SetGpuReg(74u8, 7487u16);
                SetGpuReg(64u8, 0u16);
                SetGpuReg(68u8, 0u16);
                SetGpuReg(66u8, 0u16);
                SetGpuReg(70u8, 0u16);
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetGpuReg(84u8, 0u16);
                SetGpuReg(0u8, 36928u16);
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                ShowBg(3u8);
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(0u8);
                    return 1u8;
                }
                break 'l1;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn LoadPokedexBgPalette(isSearchResults: u8) {
    unsafe {
        let mut isSearchResults = isSearchResults;
        if ((isSearchResults) as i32) == 1i32 {
            LoadPalette(
                ((((&raw mut gPokedexSearchResults_Pal).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(1))
                .cast::<u8>(),
                1u16,
                190u16,
            );
        } else {
            if !((IsNationalPokedexEnabled()) != 0) {
                LoadPalette(
                    ((((&raw mut gPokedexBgHoenn_Pal).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(1))
                    .cast::<u8>(),
                    1u16,
                    190u16,
                );
            } else {
                LoadPalette(
                    ((((&raw mut gPokedexBgNational_Pal).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(1))
                    .cast::<u8>(),
                    1u16,
                    190u16,
                );
            }
        }
        LoadPalette(
            (GetOverworldTextboxPalettePtr()).cast::<u8>(),
            240u16,
            32u16,
        );
    }
}
pub(crate) unsafe extern "C" fn FreeWindowAndBgBuffers() {
    unsafe {
        let mut tilemapBuffer: *mut u8 = core::ptr::null_mut();
        FreeAllWindowBuffers();
        tilemapBuffer = GetBgTilemapBuffer(0u8);
        if !(tilemapBuffer).is_null() {
            Free(tilemapBuffer);
        }
        tilemapBuffer = GetBgTilemapBuffer(1u8);
        if !(tilemapBuffer).is_null() {
            Free(tilemapBuffer);
        }
        tilemapBuffer = GetBgTilemapBuffer(2u8);
        if !(tilemapBuffer).is_null() {
            Free(tilemapBuffer);
        }
        tilemapBuffer = GetBgTilemapBuffer(3u8);
        if !(tilemapBuffer).is_null() {
            Free(tilemapBuffer);
        }
    }
}
pub(crate) unsafe extern "C" fn CreatePokedexList(dexMode: u8, order: u8) {
    unsafe {
        let mut dexMode = dexMode;
        let mut order = order;
        let mut vars = crate::ffi::Align4([0u8; 6]);
        let mut i: i16 = 0i16;
        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1548)
            .cast::<u16>())
        .write(0u16);
        'l1: {
            let __sw1 = ((dexMode) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 || !__matched {
                ((&raw mut vars).cast::<u16>()).write(202u16);
                (((&raw mut vars).cast::<u16>()).wrapping_offset(1)).write(1u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (IsNationalPokedexEnabled()) != 0 {
                    ((&raw mut vars).cast::<u16>()).write(386u16);
                    (((&raw mut vars).cast::<u16>()).wrapping_offset(1)).write(0u16);
                } else {
                    ((&raw mut vars).cast::<u16>()).write(202u16);
                    (((&raw mut vars).cast::<u16>()).wrapping_offset(1)).write(1u16);
                }
                break 'l1;
            }
        }
        'l2: {
            let __sw2 = ((order) as i32);
            if __sw2 == 0i32 {
                if ((((&raw mut vars).cast::<u16>()).wrapping_offset(1)).read()) != 0 {
                    {
                        i = 0i16;
                        'l3: loop {
                            if !(((i) as i32) < ((((&raw mut vars).cast::<u16>()).read()) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).write(
                                    HoennToNationalOrder(
                                        ((((i) as i32).wrapping_add(1i32)) as u16),
                                    ),
                                );
                                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .cast::<u16>())
                                .write((((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read());
                                crate::c::bf_write(
                                    (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(2),
                                    0,
                                    1,
                                    ((GetSetPokedexFlag(
                                        (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read(),
                                        0u8,
                                    )) as u16) as i32,
                                );
                                crate::c::bf_write(
                                    (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(2),
                                    1,
                                    1,
                                    ((GetSetPokedexFlag(
                                        (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read(),
                                        1u8,
                                    )) as u16) as i32,
                                );
                                if (crate::c::bf_read(
                                    (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(2),
                                    0,
                                    1,
                                    false,
                                ) as u16)
                                    != 0
                                {
                                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1548)
                                    .cast::<u16>())
                                    .write(((((i) as i32).wrapping_add(1i32)) as u16));
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                } else {
                    let mut r5: i16 = 0i16;
                    let mut r10: i16 = 0i16;
                    {
                        i = 0i16;
                        r5 = 0i16;
                        r10 = 0i16;
                        'l5: loop {
                            if !(((i) as i32) < ((((&raw mut vars).cast::<u16>()).read()) as i32)) {
                                break 'l5;
                            }
                            'l6: {
                                (((&raw mut vars).cast::<u16>()).wrapping_offset(2))
                                    .write(((((i) as i32).wrapping_add(1i32)) as u16));
                                if (GetSetPokedexFlag(
                                    (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read(),
                                    0u8,
                                )) != 0
                                {
                                    r10 = 1i16;
                                }
                                if (r10) != 0 {
                                    ((((((&raw mut sPokedexView)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .cast::<u8>())
                                    .wrapping_offset(((r5) as i32) as isize * 4))
                                    .cast::<u16>())
                                    .write(
                                        (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read(),
                                    );
                                    crate::c::bf_write(
                                        (((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset(((r5) as i32) as isize * 4))
                                        .wrapping_add(2),
                                        0,
                                        1,
                                        ((GetSetPokedexFlag(
                                            (((&raw mut vars).cast::<u16>()).wrapping_offset(2))
                                                .read(),
                                            0u8,
                                        )) as u16) as i32,
                                    );
                                    crate::c::bf_write(
                                        (((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset(((r5) as i32) as isize * 4))
                                        .wrapping_add(2),
                                        1,
                                        1,
                                        ((GetSetPokedexFlag(
                                            (((&raw mut vars).cast::<u16>()).wrapping_offset(2))
                                                .read(),
                                            1u8,
                                        )) as u16) as i32,
                                    );
                                    if (crate::c::bf_read(
                                        (((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset(((r5) as i32) as isize * 4))
                                        .wrapping_add(2),
                                        0,
                                        1,
                                        false,
                                    ) as u16)
                                        != 0
                                    {
                                        ((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1548)
                                        .cast::<u16>())
                                        .write(((((r5) as i32).wrapping_add(1i32)) as u16));
                                    }
                                    r5 = (r5).wrapping_add(1);
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                break 'l2;
            }
            if __sw2 == 1i32 {
                {
                    i = 0i16;
                    'l7: loop {
                        if !(((i) as i32) < 411i32) {
                            break 'l7;
                        }
                        'l8: {
                            (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).write(
                                ((((&raw const gPokedexOrder_Alphabetical)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<u16>())
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                            if (((NationalToHoennOrder(
                                (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read(),
                            )) as i32)
                                <= ((((&raw mut vars).cast::<u16>()).read()) as i32))
                                && ((GetSetPokedexFlag(
                                    (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read(),
                                    0u8,
                                )) != 0)
                            {
                                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1548)
                                    .cast::<u16>())
                                    .read()) as i32) as isize
                                        * 4,
                                ))
                                .cast::<u16>())
                                .write((((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read());
                                crate::c::bf_write(
                                    (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1548)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize
                                            * 4,
                                    ))
                                    .wrapping_add(2),
                                    0,
                                    1,
                                    (1u16) as i32,
                                );
                                crate::c::bf_write(
                                    (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1548)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize
                                            * 4,
                                    ))
                                    .wrapping_add(2),
                                    1,
                                    1,
                                    ((GetSetPokedexFlag(
                                        (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read(),
                                        1u8,
                                    )) as u16) as i32,
                                );
                                let __p3 =
                                    (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1548)
                                    .cast::<u16>();
                                (__p3).write(((__p3).read()).wrapping_add(1));
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l2;
            }
            if __sw2 == 2i32 {
                {
                    i = 385i16;
                    'l9: loop {
                        if !(((i) as i32) >= 0i32) {
                            break 'l9;
                        }
                        'l10: {
                            (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).write(
                                ((((&raw const gPokedexOrder_Weight)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<u16>())
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                            if (((NationalToHoennOrder(
                                (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read(),
                            )) as i32)
                                <= ((((&raw mut vars).cast::<u16>()).read()) as i32))
                                && ((GetSetPokedexFlag(
                                    (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read(),
                                    1u8,
                                )) != 0)
                            {
                                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1548)
                                    .cast::<u16>())
                                    .read()) as i32) as isize
                                        * 4,
                                ))
                                .cast::<u16>())
                                .write((((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read());
                                crate::c::bf_write(
                                    (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1548)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize
                                            * 4,
                                    ))
                                    .wrapping_add(2),
                                    0,
                                    1,
                                    (1u16) as i32,
                                );
                                crate::c::bf_write(
                                    (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1548)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize
                                            * 4,
                                    ))
                                    .wrapping_add(2),
                                    1,
                                    1,
                                    (1u16) as i32,
                                );
                                let __p4 =
                                    (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1548)
                                    .cast::<u16>();
                                (__p4).write(((__p4).read()).wrapping_add(1));
                            }
                        }
                        i = (i).wrapping_sub(1);
                    }
                }
                break 'l2;
            }
            if __sw2 == 3i32 {
                {
                    i = 0i16;
                    'l11: loop {
                        if !(((i) as i32) < 386i32) {
                            break 'l11;
                        }
                        'l12: {
                            (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).write(
                                ((((&raw const gPokedexOrder_Weight)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<u16>())
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                            if (((NationalToHoennOrder(
                                (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read(),
                            )) as i32)
                                <= ((((&raw mut vars).cast::<u16>()).read()) as i32))
                                && ((GetSetPokedexFlag(
                                    (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read(),
                                    1u8,
                                )) != 0)
                            {
                                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1548)
                                    .cast::<u16>())
                                    .read()) as i32) as isize
                                        * 4,
                                ))
                                .cast::<u16>())
                                .write((((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read());
                                crate::c::bf_write(
                                    (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1548)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize
                                            * 4,
                                    ))
                                    .wrapping_add(2),
                                    0,
                                    1,
                                    (1u16) as i32,
                                );
                                crate::c::bf_write(
                                    (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1548)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize
                                            * 4,
                                    ))
                                    .wrapping_add(2),
                                    1,
                                    1,
                                    (1u16) as i32,
                                );
                                let __p5 =
                                    (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1548)
                                    .cast::<u16>();
                                (__p5).write(((__p5).read()).wrapping_add(1));
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l2;
            }
            if __sw2 == 4i32 {
                {
                    i = 385i16;
                    'l13: loop {
                        if !(((i) as i32) >= 0i32) {
                            break 'l13;
                        }
                        'l14: {
                            (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).write(
                                ((((&raw const gPokedexOrder_Height)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<u16>())
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                            if (((NationalToHoennOrder(
                                (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read(),
                            )) as i32)
                                <= ((((&raw mut vars).cast::<u16>()).read()) as i32))
                                && ((GetSetPokedexFlag(
                                    (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read(),
                                    1u8,
                                )) != 0)
                            {
                                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1548)
                                    .cast::<u16>())
                                    .read()) as i32) as isize
                                        * 4,
                                ))
                                .cast::<u16>())
                                .write((((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read());
                                crate::c::bf_write(
                                    (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1548)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize
                                            * 4,
                                    ))
                                    .wrapping_add(2),
                                    0,
                                    1,
                                    (1u16) as i32,
                                );
                                crate::c::bf_write(
                                    (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1548)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize
                                            * 4,
                                    ))
                                    .wrapping_add(2),
                                    1,
                                    1,
                                    (1u16) as i32,
                                );
                                let __p6 =
                                    (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1548)
                                    .cast::<u16>();
                                (__p6).write(((__p6).read()).wrapping_add(1));
                            }
                        }
                        i = (i).wrapping_sub(1);
                    }
                }
                break 'l2;
            }
            if __sw2 == 5i32 {
                {
                    i = 0i16;
                    'l15: loop {
                        if !(((i) as i32) < 386i32) {
                            break 'l15;
                        }
                        'l16: {
                            (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).write(
                                ((((&raw const gPokedexOrder_Height)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<u16>())
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                            if (((NationalToHoennOrder(
                                (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read(),
                            )) as i32)
                                <= ((((&raw mut vars).cast::<u16>()).read()) as i32))
                                && ((GetSetPokedexFlag(
                                    (((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read(),
                                    1u8,
                                )) != 0)
                            {
                                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1548)
                                    .cast::<u16>())
                                    .read()) as i32) as isize
                                        * 4,
                                ))
                                .cast::<u16>())
                                .write((((&raw mut vars).cast::<u16>()).wrapping_offset(2)).read());
                                crate::c::bf_write(
                                    (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1548)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize
                                            * 4,
                                    ))
                                    .wrapping_add(2),
                                    0,
                                    1,
                                    (1u16) as i32,
                                );
                                crate::c::bf_write(
                                    (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1548)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize
                                            * 4,
                                    ))
                                    .wrapping_add(2),
                                    1,
                                    1,
                                    (1u16) as i32,
                                );
                                let __p7 =
                                    (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1548)
                                    .cast::<u16>();
                                (__p7).write(((__p7).read()).wrapping_add(1));
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l2;
            }
        }
        {
            i = ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1548)
                .cast::<u16>())
            .read()) as i16);
            'l17: loop {
                if !(((i) as i32) < 386i32) {
                    break 'l17;
                }
                'l18: {
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .cast::<u16>())
                    .write(65535u16);
                    crate::c::bf_write(
                        (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(2),
                        0,
                        1,
                        (0u16) as i32,
                    );
                    crate::c::bf_write(
                        (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(2),
                        1,
                        1,
                        (0u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintMonDexNumAndName(
    windowId: u8,
    fontId: u8,
    str: *mut u8,
    left: u8,
    top: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut str = str;
        let mut left = left;
        let mut top = top;
        let mut color = crate::ffi::Align4([0u8; 3]);
        ((&raw mut color).cast::<u8>()).write(0u8);
        (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(15u8);
        (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(3u8);
        AddTextPrinterParameterized4(
            windowId,
            fontId,
            ((((left) as i32).wrapping_mul(8i32)) as u8),
            (((((top) as i32).wrapping_mul(8i32)).wrapping_add(1i32)) as u8),
            0u8,
            0u8,
            (&raw mut color).cast::<u8>(),
            (-1i8),
            str,
        );
    }
}
pub(crate) unsafe extern "C" fn CreateMonListEntry(position: u8, b: u16, ignored: u16) {
    unsafe {
        let mut position = position;
        let mut b = b;
        let mut ignored = ignored;
        let mut entryNum: i16 = 0i16;
        let mut i: u16 = 0u16;
        let mut vOffset: u16 = 0u16;
        'l1: {
            let __sw1 = ((position) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 || !__matched {
                entryNum = ((((b) as i32).wrapping_sub(5i32)) as i16);
                {
                    i = 0u16;
                    'l2: loop {
                        if !(((i) as i32) <= 10i32) {
                            break 'l2;
                        }
                        'l3: {
                            if ((((entryNum) as i32) < 0i32) || (((entryNum) as i32) >= 386i32))
                                || (((((((((&raw mut sPokedexView)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .cast::<u8>())
                                .wrapping_offset(((entryNum) as i32) as isize * 4))
                                .cast::<u16>())
                                .read()) as i32)
                                    == 65535i32)
                            {
                                ClearMonListEntry(
                                    17u8,
                                    ((((i) as i32).wrapping_mul(2i32)) as u8),
                                    ignored,
                                );
                            } else {
                                ClearMonListEntry(
                                    17u8,
                                    ((((i) as i32).wrapping_mul(2i32)) as u8),
                                    ignored,
                                );
                                if (crate::c::bf_read(
                                    (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(((entryNum) as i32) as isize * 4))
                                    .wrapping_add(2),
                                    0,
                                    1,
                                    false,
                                ) as u16)
                                    != 0
                                {
                                    CreateMonDexNum(
                                        ((entryNum) as u16),
                                        18u8,
                                        ((((i) as i32).wrapping_mul(2i32)) as u8),
                                        ignored,
                                    );
                                    CreateCaughtBall(
                                        (crate::c::bf_read(
                                            (((((&raw mut sPokedexView)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .cast::<u8>())
                                            .wrapping_offset(((entryNum) as i32) as isize * 4))
                                            .wrapping_add(2),
                                            1,
                                            1,
                                            false,
                                        ) as u16),
                                        17u8,
                                        ((((i) as i32).wrapping_mul(2i32)) as u8),
                                        ignored,
                                    );
                                    CreateMonName(
                                        ((((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset(((entryNum) as i32) as isize * 4))
                                        .cast::<u16>())
                                        .read(),
                                        22u8,
                                        ((((i) as i32).wrapping_mul(2i32)) as u8),
                                    );
                                } else {
                                    CreateMonDexNum(
                                        ((entryNum) as u16),
                                        18u8,
                                        ((((i) as i32).wrapping_mul(2i32)) as u8),
                                        ignored,
                                    );
                                    CreateCaughtBall(
                                        0u16,
                                        17u8,
                                        ((((i) as i32).wrapping_mul(2i32)) as u8),
                                        ignored,
                                    );
                                    CreateMonName(
                                        0u16,
                                        22u8,
                                        ((((i) as i32).wrapping_mul(2i32)) as u8),
                                    );
                                }
                            }
                            entryNum = (entryNum).wrapping_add(1);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                entryNum = ((((b) as i32).wrapping_sub(5i32)) as i16);
                if ((((entryNum) as i32) < 0i32) || (((entryNum) as i32) >= 386i32))
                    || (((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(((entryNum) as i32) as isize * 4))
                    .cast::<u16>())
                    .read()) as i32)
                        == 65535i32)
                {
                    ClearMonListEntry(
                        17u8,
                        ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1584)
                            .cast::<i16>())
                        .read()) as i32)
                            .wrapping_mul(2i32)) as u8),
                        ignored,
                    );
                } else {
                    ClearMonListEntry(
                        17u8,
                        ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1584)
                            .cast::<i16>())
                        .read()) as i32)
                            .wrapping_mul(2i32)) as u8),
                        ignored,
                    );
                    if (crate::c::bf_read(
                        (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((entryNum) as i32) as isize * 4))
                        .wrapping_add(2),
                        0,
                        1,
                        false,
                    ) as u16)
                        != 0
                    {
                        CreateMonDexNum(
                            ((entryNum) as u16),
                            18u8,
                            ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1584)
                                .cast::<i16>())
                            .read()) as i32)
                                .wrapping_mul(2i32)) as u8),
                            ignored,
                        );
                        CreateCaughtBall(
                            (crate::c::bf_read(
                                (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(((entryNum) as i32) as isize * 4))
                                .wrapping_add(2),
                                1,
                                1,
                                false,
                            ) as u16),
                            17u8,
                            ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1584)
                                .cast::<i16>())
                            .read()) as i32)
                                .wrapping_mul(2i32)) as u8),
                            ignored,
                        );
                        CreateMonName(
                            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((entryNum) as i32) as isize * 4))
                            .cast::<u16>())
                            .read(),
                            22u8,
                            ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1584)
                                .cast::<i16>())
                            .read()) as i32)
                                .wrapping_mul(2i32)) as u8),
                        );
                    } else {
                        CreateMonDexNum(
                            ((entryNum) as u16),
                            18u8,
                            ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1584)
                                .cast::<i16>())
                            .read()) as i32)
                                .wrapping_mul(2i32)) as u8),
                            ignored,
                        );
                        CreateCaughtBall(
                            0u16,
                            17u8,
                            ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1584)
                                .cast::<i16>())
                            .read()) as i32)
                                .wrapping_mul(2i32)) as u8),
                            ignored,
                        );
                        CreateMonName(
                            0u16,
                            22u8,
                            ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1584)
                                .cast::<i16>())
                            .read()) as i32)
                                .wrapping_mul(2i32)) as u8),
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                entryNum = ((((b) as i32).wrapping_add(5i32)) as i16);
                vOffset = ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1584)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(10i32)) as u16);
                if ((vOffset) as i32) >= 16i32 {
                    vOffset = ((((vOffset) as i32).wrapping_sub(16i32)) as u16);
                }
                if ((((entryNum) as i32) < 0i32) || (((entryNum) as i32) >= 386i32))
                    || (((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(((entryNum) as i32) as isize * 4))
                    .cast::<u16>())
                    .read()) as i32)
                        == 65535i32)
                {
                    ClearMonListEntry(
                        17u8,
                        ((((vOffset) as i32).wrapping_mul(2i32)) as u8),
                        ignored,
                    );
                } else {
                    ClearMonListEntry(
                        17u8,
                        ((((vOffset) as i32).wrapping_mul(2i32)) as u8),
                        ignored,
                    );
                    if (crate::c::bf_read(
                        (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((entryNum) as i32) as isize * 4))
                        .wrapping_add(2),
                        0,
                        1,
                        false,
                    ) as u16)
                        != 0
                    {
                        CreateMonDexNum(
                            ((entryNum) as u16),
                            18u8,
                            ((((vOffset) as i32).wrapping_mul(2i32)) as u8),
                            ignored,
                        );
                        CreateCaughtBall(
                            (crate::c::bf_read(
                                (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(((entryNum) as i32) as isize * 4))
                                .wrapping_add(2),
                                1,
                                1,
                                false,
                            ) as u16),
                            17u8,
                            ((((vOffset) as i32).wrapping_mul(2i32)) as u8),
                            ignored,
                        );
                        CreateMonName(
                            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((entryNum) as i32) as isize * 4))
                            .cast::<u16>())
                            .read(),
                            22u8,
                            ((((vOffset) as i32).wrapping_mul(2i32)) as u8),
                        );
                    } else {
                        CreateMonDexNum(
                            ((entryNum) as u16),
                            18u8,
                            ((((vOffset) as i32).wrapping_mul(2i32)) as u8),
                            ignored,
                        );
                        CreateCaughtBall(
                            0u16,
                            17u8,
                            ((((vOffset) as i32).wrapping_mul(2i32)) as u8),
                            ignored,
                        );
                        CreateMonName(0u16, 22u8, ((((vOffset) as i32).wrapping_mul(2i32)) as u8));
                    }
                }
                break 'l1;
            }
        }
        CopyWindowToVram(0u8, 2u8);
    }
}
pub(crate) unsafe extern "C" fn CreateMonDexNum(entryNum: u16, left: u8, top: u8, unused: u16) {
    unsafe {
        let mut entryNum = entryNum;
        let mut left = left;
        let mut top = top;
        let mut unused = unused;
        let mut text = crate::ffi::Align4([0u8; 6]);
        let mut dexNum: u16 = 0u16;
        crate::c::memcpy(
            (&raw mut text).cast::<u8>(),
            ((&raw const sText_No000).cast::<u8>().cast_mut()).cast::<u8>(),
            crate::c::div_u32(6u32, 1u32),
        );
        dexNum = ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<u8>())
        .wrapping_offset(((entryNum) as i32) as isize * 4))
        .cast::<u16>())
        .read();
        if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1554)
            .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            dexNum = NationalToHoennOrder(dexNum);
        }
        (((&raw mut text).cast::<u8>()).wrapping_offset(2))
            .write((((161i32).wrapping_add(crate::c::div_i32(((dexNum) as i32), 100i32))) as u8));
        (((&raw mut text).cast::<u8>()).wrapping_offset(3)).write(
            (((161i32).wrapping_add(crate::c::div_i32(
                crate::c::rem_i32(((dexNum) as i32), 100i32),
                10i32,
            ))) as u8),
        );
        (((&raw mut text).cast::<u8>()).wrapping_offset(4)).write(
            (((161i32).wrapping_add(crate::c::rem_i32(
                crate::c::rem_i32(((dexNum) as i32), 100i32),
                10i32,
            ))) as u8),
        );
        PrintMonDexNumAndName(0u8, 7u8, (&raw mut text).cast::<u8>(), left, top);
    }
}
pub(crate) unsafe extern "C" fn CreateCaughtBall(owned: u16, x: u8, y: u8, unused: u16) {
    unsafe {
        let mut owned = owned;
        let mut x = x;
        let mut y = y;
        let mut unused = unused;
        if (owned) != 0 {
            BlitBitmapToWindow(
                0u8,
                ((&raw const sCaughtBall_Gfx).cast::<u8>().cast_mut()).cast::<u8>(),
                ((((x) as i32).wrapping_mul(8i32)) as u16),
                ((((y) as i32).wrapping_mul(8i32)) as u16),
                8u16,
                16u16,
            );
        } else {
            FillWindowPixelRect(
                0u8,
                0u8,
                ((((x) as i32).wrapping_mul(8i32)) as u16),
                ((((y) as i32).wrapping_mul(8i32)) as u16),
                8u16,
                16u16,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CreateMonName(num: u16, left: u8, top: u8) -> u8 {
    unsafe {
        let mut num = num;
        let mut left = left;
        let mut top = top;
        let mut str: *mut u8 = core::ptr::null_mut();
        num = NationalPokedexNumToSpecies(num);
        if (num) != 0 {
            str = (((&raw mut gSpeciesNames).cast::<u8>())
                .wrapping_offset(((num) as i32) as isize * 11))
            .cast::<u8>();
        } else {
            str = ((&raw const sText_TenDashes).cast::<u8>().cast_mut()).cast::<u8>();
        }
        PrintMonDexNumAndName(0u8, 7u8, str, left, top);
        return ((StringLength(str)) as u8);
    }
}
pub(crate) unsafe extern "C" fn ClearMonListEntry(x: u8, y: u8, unused: u16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut unused = unused;
        FillWindowPixelRect(
            0u8,
            0u8,
            ((((x) as i32).wrapping_mul(8i32)) as u16),
            ((((y) as i32).wrapping_mul(8i32)) as u16),
            96u16,
            16u16,
        );
    }
}
pub(crate) unsafe extern "C" fn CreateMonSpritesAtPos(selectedMon: u16, ignored: u16) {
    unsafe {
        let mut selectedMon = selectedMon;
        let mut ignored = ignored;
        let mut i: u8 = 0u8;
        let mut dexNum: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
            7,
            1,
            (1u16) as i32,
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1566))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(65535u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1574)
            .cast::<u16>())
        .write(65535u16);
        dexNum = GetPokemonSpriteToDisplay(((((selectedMon) as i32).wrapping_sub(1i32)) as u16));
        if ((dexNum) as i32) != 65535i32 {
            spriteId = ((CreatePokedexMonSprite(dexNum, 96i16, 80i16)) as u8);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_PokedexListMonSprite));
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(5))
            .write((-32i16));
        }
        dexNum = GetPokemonSpriteToDisplay(selectedMon);
        if ((dexNum) as i32) != 65535i32 {
            spriteId = ((CreatePokedexMonSprite(dexNum, 96i16, 80i16)) as u8);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_PokedexListMonSprite));
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(0i16);
        }
        dexNum = GetPokemonSpriteToDisplay(((((selectedMon) as i32).wrapping_add(1i32)) as u16));
        if ((dexNum) as i32) != 65535i32 {
            spriteId = ((CreatePokedexMonSprite(dexNum, 96i16, 80i16)) as u8);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_PokedexListMonSprite));
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(32i16);
        }
        CreateMonListEntry(0u8, selectedMon, ignored);
        SetGpuReg(
            26u8,
            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1581))
            .read()) as u16),
        );
        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1584)
            .cast::<i16>())
        .write(0i16);
        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1586)
            .cast::<i16>())
        .write(0i16);
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
            7,
            1,
            (0u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateDexListScroll(
    direction: u8,
    monMoveIncrement: u8,
    scrollTimerMax: u8,
) -> u8 {
    unsafe {
        let mut direction = direction;
        let mut monMoveIncrement = monMoveIncrement;
        let mut scrollTimerMax = scrollTimerMax;
        let mut i: u16 = 0u16;
        let mut step: u8 = 0u8;
        if (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1582))
            .read())
            != 0
        {
            let __p1 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1582);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            'l1: {
                let __sw2 = ((direction) as i32);
                if __sw2 == 1i32 {
                    {
                        i = 0u16;
                        'l2: loop {
                            if !(((i) as i32) < 4i32) {
                                break 'l2;
                            }
                            'l3: {
                                if ((((((((&raw mut sPokedexView)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(1566))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    != 65535i32
                                {
                                    let __p3 = (((((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(
                                            ((((((((&raw mut sPokedexView)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1566))
                                            .cast::<u16>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                    .wrapping_add(46))
                                    .cast::<i16>())
                                    .wrapping_offset(5);
                                    (__p3).write(
                                        (((((__p3).read()) as i32)
                                            .wrapping_add(((monMoveIncrement) as i32)))
                                            as i16),
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    step = ((crate::c::div_i32(
                        (16i32).wrapping_mul(
                            ((scrollTimerMax) as i32).wrapping_sub(
                                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1582))
                                .read()) as i32),
                            ),
                        ),
                        ((scrollTimerMax) as i32),
                    )) as u8);
                    SetGpuReg(
                        26u8,
                        (((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1581))
                        .read()) as i32)
                            .wrapping_add(
                                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1586)
                                .cast::<i16>())
                                .read()) as i32)
                                    .wrapping_mul(16i32),
                            ))
                        .wrapping_sub(((step) as i32))) as u16),
                    );
                    let __p4 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1580);
                    (__p4).write(
                        (((((__p4).read()) as i32).wrapping_sub(
                            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1576)
                                .cast::<u16>())
                            .read()) as i32),
                        )) as u8),
                    );
                    break 'l1;
                }
                if __sw2 == 2i32 {
                    {
                        i = 0u16;
                        'l4: loop {
                            if !(((i) as i32) < 4i32) {
                                break 'l4;
                            }
                            'l5: {
                                if ((((((((&raw mut sPokedexView)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(1566))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    != 65535i32
                                {
                                    let __p5 = (((((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(
                                            ((((((((&raw mut sPokedexView)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1566))
                                            .cast::<u16>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                    .wrapping_add(46))
                                    .cast::<i16>())
                                    .wrapping_offset(5);
                                    (__p5).write(
                                        (((((__p5).read()) as i32)
                                            .wrapping_sub(((monMoveIncrement) as i32)))
                                            as i16),
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    step = ((crate::c::div_i32(
                        (16i32).wrapping_mul(
                            ((scrollTimerMax) as i32).wrapping_sub(
                                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1582))
                                .read()) as i32),
                            ),
                        ),
                        ((scrollTimerMax) as i32),
                    )) as u8);
                    SetGpuReg(
                        26u8,
                        (((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1581))
                        .read()) as i32)
                            .wrapping_add(
                                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1586)
                                .cast::<i16>())
                                .read()) as i32)
                                    .wrapping_mul(16i32),
                            ))
                        .wrapping_add(((step) as i32))) as u16),
                    );
                    let __p6 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1580);
                    (__p6).write(
                        (((((__p6).read()) as i32).wrapping_add(
                            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1576)
                                .cast::<u16>())
                            .read()) as i32),
                        )) as u8),
                    );
                    break 'l1;
                }
            }
            return 0u8;
        } else {
            SetGpuReg(
                26u8,
                ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1581))
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1584)
                            .cast::<i16>())
                        .read()) as i32)
                            .wrapping_mul(16i32),
                    )) as u16),
            );
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn CreateScrollingPokemonSprite(direction: u8, selectedMon: u16) {
    unsafe {
        let mut direction = direction;
        let mut selectedMon = selectedMon;
        let mut dexNum: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1586)
            .cast::<i16>())
        .write(
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1584)
                .cast::<i16>())
            .read(),
        );
        'l1: {
            let __sw1 = ((direction) as i32);
            if __sw1 == 1i32 {
                dexNum =
                    GetPokemonSpriteToDisplay(((((selectedMon) as i32).wrapping_sub(1i32)) as u16));
                if ((dexNum) as i32) != 65535i32 {
                    spriteId = ((CreatePokedexMonSprite(dexNum, 96i16, 80i16)) as u8);
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_PokedexListMonSprite));
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write((-64i16));
                }
                if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1584)
                    .cast::<i16>())
                .read()) as i32)
                    > 0i32
                {
                    let __p2 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1584)
                        .cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                } else {
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1584)
                        .cast::<i16>())
                    .write(15i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                dexNum =
                    GetPokemonSpriteToDisplay(((((selectedMon) as i32).wrapping_add(1i32)) as u16));
                if ((dexNum) as i32) != 65535i32 {
                    spriteId = ((CreatePokedexMonSprite(dexNum, 96i16, 80i16)) as u8);
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_PokedexListMonSprite));
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write(64i16);
                }
                if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1584)
                    .cast::<i16>())
                .read()) as i32)
                    < 15i32
                {
                    let __p3 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1584)
                        .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                } else {
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1584)
                        .cast::<i16>())
                    .write(0i16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryDoPokedexScroll(selectedMon: u16, ignored: u16) -> u16 {
    unsafe {
        let mut selectedMon = selectedMon;
        let mut ignored = ignored;
        let mut scrollTimer: u8 = 0u8;
        let mut scrollMonIncrement: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut startingPos: u16 = 0u16;
        let mut scrollDir: u8 = 0u8;
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0)
            && (((selectedMon) as i32) > 0i32)
        {
            scrollDir = 1u8;
            selectedMon = GetNextPosition(
                1u8,
                selectedMon,
                0u16,
                ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1548)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_sub(1i32)) as u16),
            );
            CreateScrollingPokemonSprite(1u8, selectedMon);
            CreateMonListEntry(1u8, selectedMon, ignored);
            PlaySE(108u16);
        } else {
            if (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(44)
                .cast::<u16>())
            .read()) as i32)
                & 128i32)
                != 0)
                && (((selectedMon) as i32)
                    < ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1548)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_sub(1i32))
            {
                scrollDir = 2u8;
                selectedMon = GetNextPosition(
                    0u8,
                    selectedMon,
                    0u16,
                    ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1548)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_sub(1i32)) as u16),
                );
                CreateScrollingPokemonSprite(2u8, selectedMon);
                CreateMonListEntry(2u8, selectedMon, ignored);
                PlaySE(108u16);
            } else {
                if (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 32i32)
                    != 0)
                    && (((selectedMon) as i32) > 0i32)
                {
                    startingPos = selectedMon;
                    {
                        i = 0u8;
                        'l1: loop {
                            if !(((i) as i32) < 7i32) {
                                break 'l1;
                            }
                            'l2: {
                                selectedMon = GetNextPosition(
                                    1u8,
                                    selectedMon,
                                    0u16,
                                    ((((((((&raw mut sPokedexView)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(1548)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        .wrapping_sub(1i32))
                                        as u16),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    let __p1 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1580);
                    (__p1).write(
                        (((((__p1).read()) as i32).wrapping_add((16i32).wrapping_mul(
                            ((selectedMon) as i32).wrapping_sub(((startingPos) as i32)),
                        ))) as u8),
                    );
                    ClearMonSprites();
                    CreateMonSpritesAtPos(selectedMon, 14u16);
                    PlaySE(109u16);
                } else {
                    if (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 16i32)
                        != 0)
                        && (((selectedMon) as i32)
                            < ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1548)
                                .cast::<u16>())
                            .read()) as i32)
                                .wrapping_sub(1i32))
                    {
                        startingPos = selectedMon;
                        {
                            i = 0u8;
                            'l3: loop {
                                if !(((i) as i32) < 7i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    selectedMon = GetNextPosition(
                                        0u8,
                                        selectedMon,
                                        0u16,
                                        ((((((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1548)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            .wrapping_sub(1i32))
                                            as u16),
                                    );
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        let __p2 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(1580);
                        (__p2).write(
                            (((((__p2).read()) as i32).wrapping_add((16i32).wrapping_mul(
                                ((selectedMon) as i32).wrapping_sub(((startingPos) as i32)),
                            ))) as u8),
                        );
                        ClearMonSprites();
                        CreateMonSpritesAtPos(selectedMon, 14u16);
                        PlaySE(109u16);
                    }
                }
            }
        }
        if ((scrollDir) as i32) == 0i32 {
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1592)
                .cast::<u16>())
            .write(0u16);
            return selectedMon;
        }
        scrollMonIncrement = ((((&raw const sScrollMonIncrements).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(
            (crate::c::div_i32(
                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1592)
                    .cast::<u16>())
                .read()) as i32),
                4i32,
            )) as isize,
        ))
        .read();
        scrollTimer = ((((&raw const sScrollTimers).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                (crate::c::div_i32(
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1592)
                        .cast::<u16>())
                    .read()) as i32),
                    4i32,
                )) as isize,
            ))
        .read();
        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1582))
            .write(scrollTimer);
        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1590)
            .cast::<u16>())
        .write(((scrollTimer) as u16));
        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1588)
            .cast::<u16>())
        .write(((scrollMonIncrement) as u16));
        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1583))
            .write(scrollDir);
        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1576)
            .cast::<u16>())
        .write(((crate::c::div_i32(((scrollMonIncrement) as i32), 2i32)) as u16));
        UpdateDexListScroll(
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1583))
                .read(),
            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1588)
                .cast::<u16>())
            .read()) as u8),
            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1590)
                .cast::<u16>())
            .read()) as u8),
        );
        if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1592)
            .cast::<u16>())
        .read()) as i32)
            < 12i32
        {
            let __p3 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1592)
                .cast::<u16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        return selectedMon;
    }
}
pub(crate) unsafe extern "C" fn UpdateSelectedMonSpriteId() {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut spriteId: u16 =
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1566))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read();
                    if ((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .read()) as i32)
                        == 0i32)
                        && (((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(38)
                        .cast::<i16>())
                        .read()) as i32)
                            == 0i32))
                        && (((spriteId) as i32) != 65535i32)
                    {
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1574)
                            .cast::<u16>())
                        .write(spriteId);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryDoInfoScreenScroll() -> u8 {
    unsafe {
        let mut nextPokemon: u16 = 0u16;
        let mut selectedPokemon: u16 = ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(1550)
        .cast::<u16>())
        .read();
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0)
            && ((selectedPokemon) != 0)
        {
            nextPokemon = selectedPokemon;
            'l1: loop {
                if !(((nextPokemon) as i32) != 0i32) {
                    break 'l1;
                }
                nextPokemon = GetNextPosition(
                    1u8,
                    nextPokemon,
                    0u16,
                    ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1548)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_sub(1i32)) as u16),
                );
                if (crate::c::bf_read(
                    (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(((nextPokemon) as i32) as isize * 4))
                    .wrapping_add(2),
                    0,
                    1,
                    false,
                ) as u16)
                    != 0
                {
                    selectedPokemon = nextPokemon;
                    break 'l1;
                }
            }
            if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1550)
                .cast::<u16>())
            .read()) as i32)
                == ((selectedPokemon) as i32)
            {
                return 0u8;
            } else {
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1550)
                    .cast::<u16>())
                .write(selectedPokemon);
                let __p1 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1580);
                (__p1).write((((((__p1).read()) as i32).wrapping_sub(16i32)) as u8));
                return 1u8;
            }
        } else {
            if (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 128i32)
                != 0)
                && (((selectedPokemon) as i32)
                    < ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1548)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_sub(1i32))
            {
                nextPokemon = selectedPokemon;
                'l2: loop {
                    if !(((nextPokemon) as i32)
                        < ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1548)
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_sub(1i32))
                    {
                        break 'l2;
                    }
                    nextPokemon = GetNextPosition(
                        0u8,
                        nextPokemon,
                        0u16,
                        ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1548)
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_sub(1i32)) as u16),
                    );
                    if (crate::c::bf_read(
                        (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((nextPokemon) as i32) as isize * 4))
                        .wrapping_add(2),
                        0,
                        1,
                        false,
                    ) as u16)
                        != 0
                    {
                        selectedPokemon = nextPokemon;
                        break 'l2;
                    }
                }
                if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1550)
                    .cast::<u16>())
                .read()) as i32)
                    == ((selectedPokemon) as i32)
                {
                    return 0u8;
                } else {
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1550)
                        .cast::<u16>())
                    .write(selectedPokemon);
                    let __p2 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1580);
                    (__p2).write((((((__p2).read()) as i32).wrapping_add(16i32)) as u8));
                    return 1u8;
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ClearMonSprites() -> u8 {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1566))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 65535i32
                    {
                        FreeAndDestroyMonPicSprite(
                            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1566))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1566))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(65535u16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn GetPokemonSpriteToDisplay(species: u16) -> u16 {
    unsafe {
        let mut species = species;
        if (((species) as i32) >= 386i32)
            || (((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u8>())
            .wrapping_offset(((species) as i32) as isize * 4))
            .cast::<u16>())
            .read()) as i32)
                == 65535i32)
        {
            return 65535u16;
        } else {
            if (crate::c::bf_read(
                (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 4))
                .wrapping_add(2),
                0,
                1,
                false,
            ) as u16)
                != 0
            {
                return ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 4))
                .cast::<u16>())
                .read();
            } else {
                return 0u16;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn CreatePokedexMonSprite(num: u16, x: i16, y: i16) -> u32 {
    unsafe {
        let mut num = num;
        let mut x = x;
        let mut y = y;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1566))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 65535i32
                    {
                        let mut spriteId: u8 =
                            ((CreateMonSpriteFromNationalDexNumber(num, x, y, ((i) as u16))) as u8);
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(1),
                            0,
                            2,
                            (1u32) as i32,
                        );
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(5),
                            2,
                            2,
                            (3u16) as i32,
                        );
                        (((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(0i16);
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(((i) as i16));
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(((NationalPokedexNumToSpecies(num)) as i16));
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1566))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(((spriteId) as u16));
                        return ((spriteId) as u32);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 65535u32;
    }
}
pub(crate) unsafe extern "C" fn CreateInterfaceSprites(page: u8) {
    unsafe {
        let mut page = page;
        let mut spriteId: u8 = 0u8;
        let mut digitNum: u16 = 0u16;
        spriteId = CreateSprite(
            (&raw const sScrollArrowSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            184i16,
            4i16,
            0u8,
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        spriteId = CreateSprite(
            (&raw const sScrollArrowSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            184i16,
            156i16,
            0u8,
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(1i16);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(63),
            1,
            1,
            (1u16) as i32,
        );
        CreateSprite(
            (&raw const sScrollBarSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            230i16,
            20i16,
            0u8,
        );
        CreateSprite(
            (&raw const sInterfaceTextSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            16i16,
            120i16,
            0u8,
        );
        spriteId = CreateSprite(
            (&raw const sInterfaceTextSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            48i16,
            120i16,
            0u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            3u8,
        );
        spriteId = CreateSprite(
            (&raw const sInterfaceTextSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            16i16,
            144i16,
            0u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            2u8,
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(128i16);
        spriteId = CreateSprite(
            (&raw const sInterfaceTextSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            48i16,
            144i16,
            0u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            1u8,
        );
        spriteId = CreateSprite(
            (&raw const sRotatingPokeBallSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            0i16,
            ((crate::c::div_i32(160i32, 2i32)) as i16),
            2u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            0,
            2,
            (1u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
            1,
            5,
            (30u32) as i32,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(30i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        spriteId = CreateSprite(
            (&raw const sRotatingPokeBallSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            0i16,
            ((crate::c::div_i32(160i32, 2i32)) as i16),
            2u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            0,
            2,
            (1u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
            1,
            5,
            (31u32) as i32,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(31i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(128i16);
        if ((page) as i32) == 0i32 {
            let mut drawNextDigit: u32 = 0u32;
            if !((IsNationalPokedexEnabled()) != 0) {
                CreateSprite(
                    (&raw const sSeenOwnTextSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    32i16,
                    40i16,
                    1u8,
                );
                spriteId = CreateSprite(
                    (&raw const sSeenOwnTextSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    32i16,
                    72i16,
                    1u8,
                );
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    1u8,
                );
                drawNextDigit = 0u32;
                spriteId = CreateSprite(
                    (&raw const sHoennDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    24i16,
                    48i16,
                    1u8,
                );
                digitNum = ((crate::c::div_i32(
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1562)
                        .cast::<u16>())
                    .read()) as i32),
                    100i32,
                )) as u16);
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    ((digitNum) as u8),
                );
                if ((digitNum) as i32) != 0i32 {
                    drawNextDigit = 1u32;
                } else {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                spriteId = CreateSprite(
                    (&raw const sHoennDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    32i16,
                    48i16,
                    1u8,
                );
                digitNum = ((crate::c::div_i32(
                    crate::c::rem_i32(
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1562)
                            .cast::<u16>())
                        .read()) as i32),
                        100i32,
                    ),
                    10i32,
                )) as u16);
                if (((digitNum) as i32) != 0i32) || ((drawNextDigit) != 0) {
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        ((digitNum) as u8),
                    );
                } else {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                spriteId = CreateSprite(
                    (&raw const sHoennDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    40i16,
                    48i16,
                    1u8,
                );
                digitNum = ((crate::c::rem_i32(
                    crate::c::rem_i32(
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1562)
                            .cast::<u16>())
                        .read()) as i32),
                        100i32,
                    ),
                    10i32,
                )) as u16);
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    ((digitNum) as u8),
                );
                drawNextDigit = 0u32;
                spriteId = CreateSprite(
                    (&raw const sHoennDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    24i16,
                    80i16,
                    1u8,
                );
                digitNum = ((crate::c::div_i32(
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1564)
                        .cast::<u16>())
                    .read()) as i32),
                    100i32,
                )) as u16);
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    ((digitNum) as u8),
                );
                if ((digitNum) as i32) != 0i32 {
                    drawNextDigit = 1u32;
                } else {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                spriteId = CreateSprite(
                    (&raw const sHoennDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    32i16,
                    80i16,
                    1u8,
                );
                digitNum = ((crate::c::div_i32(
                    crate::c::rem_i32(
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1564)
                            .cast::<u16>())
                        .read()) as i32),
                        100i32,
                    ),
                    10i32,
                )) as u16);
                if (((digitNum) as i32) != 0i32) || ((drawNextDigit) != 0) {
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        ((digitNum) as u8),
                    );
                } else {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                spriteId = CreateSprite(
                    (&raw const sHoennDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    40i16,
                    80i16,
                    1u8,
                );
                digitNum = ((crate::c::rem_i32(
                    crate::c::rem_i32(
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1564)
                            .cast::<u16>())
                        .read()) as i32),
                        100i32,
                    ),
                    10i32,
                )) as u16);
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    ((digitNum) as u8),
                );
            } else {
                let mut seenOwnedCount: u16 = 0u16;
                CreateSprite(
                    (&raw const sSeenOwnTextSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    32i16,
                    40i16,
                    1u8,
                );
                spriteId = CreateSprite(
                    (&raw const sSeenOwnTextSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    32i16,
                    76i16,
                    1u8,
                );
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    1u8,
                );
                CreateSprite(
                    (&raw const sHoennNationalTextSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    17i16,
                    45i16,
                    1u8,
                );
                spriteId = CreateSprite(
                    (&raw const sHoennNationalTextSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    17i16,
                    55i16,
                    1u8,
                );
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    1u8,
                );
                CreateSprite(
                    (&raw const sHoennNationalTextSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    17i16,
                    81i16,
                    1u8,
                );
                spriteId = CreateSprite(
                    (&raw const sHoennNationalTextSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    17i16,
                    91i16,
                    1u8,
                );
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    1u8,
                );
                seenOwnedCount = GetHoennPokedexCount(0u8);
                drawNextDigit = 0u32;
                spriteId = CreateSprite(
                    (&raw const sNationalDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    40i16,
                    45i16,
                    1u8,
                );
                digitNum = ((crate::c::div_i32(((seenOwnedCount) as i32), 100i32)) as u16);
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    ((digitNum) as u8),
                );
                if ((digitNum) as i32) != 0i32 {
                    drawNextDigit = 1u32;
                } else {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                spriteId = CreateSprite(
                    (&raw const sNationalDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    48i16,
                    45i16,
                    1u8,
                );
                digitNum = ((crate::c::div_i32(
                    crate::c::rem_i32(((seenOwnedCount) as i32), 100i32),
                    10i32,
                )) as u16);
                if (((digitNum) as i32) != 0i32) || ((drawNextDigit) != 0) {
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        ((digitNum) as u8),
                    );
                } else {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                spriteId = CreateSprite(
                    (&raw const sNationalDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    56i16,
                    45i16,
                    1u8,
                );
                digitNum = ((crate::c::rem_i32(
                    crate::c::rem_i32(((seenOwnedCount) as i32), 100i32),
                    10i32,
                )) as u16);
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    ((digitNum) as u8),
                );
                drawNextDigit = 0u32;
                spriteId = CreateSprite(
                    (&raw const sNationalDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    40i16,
                    55i16,
                    1u8,
                );
                digitNum = ((crate::c::div_i32(
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1562)
                        .cast::<u16>())
                    .read()) as i32),
                    100i32,
                )) as u16);
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    ((digitNum) as u8),
                );
                if ((digitNum) as i32) != 0i32 {
                    drawNextDigit = 1u32;
                } else {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                spriteId = CreateSprite(
                    (&raw const sNationalDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    48i16,
                    55i16,
                    1u8,
                );
                digitNum = ((crate::c::div_i32(
                    crate::c::rem_i32(
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1562)
                            .cast::<u16>())
                        .read()) as i32),
                        100i32,
                    ),
                    10i32,
                )) as u16);
                if (((digitNum) as i32) != 0i32) || ((drawNextDigit) != 0) {
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        ((digitNum) as u8),
                    );
                } else {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                spriteId = CreateSprite(
                    (&raw const sNationalDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    56i16,
                    55i16,
                    1u8,
                );
                digitNum = ((crate::c::rem_i32(
                    crate::c::rem_i32(
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1562)
                            .cast::<u16>())
                        .read()) as i32),
                        100i32,
                    ),
                    10i32,
                )) as u16);
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    ((digitNum) as u8),
                );
                seenOwnedCount = GetHoennPokedexCount(1u8);
                drawNextDigit = 0u32;
                spriteId = CreateSprite(
                    (&raw const sNationalDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    40i16,
                    81i16,
                    1u8,
                );
                digitNum = ((crate::c::div_i32(((seenOwnedCount) as i32), 100i32)) as u16);
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    ((digitNum) as u8),
                );
                if ((digitNum) as i32) != 0i32 {
                    drawNextDigit = 1u32;
                } else {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                spriteId = CreateSprite(
                    (&raw const sNationalDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    48i16,
                    81i16,
                    1u8,
                );
                digitNum = ((crate::c::div_i32(
                    crate::c::rem_i32(((seenOwnedCount) as i32), 100i32),
                    10i32,
                )) as u16);
                if (((digitNum) as i32) != 0i32) || ((drawNextDigit) != 0) {
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        ((digitNum) as u8),
                    );
                } else {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                spriteId = CreateSprite(
                    (&raw const sNationalDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    56i16,
                    81i16,
                    1u8,
                );
                digitNum = ((crate::c::rem_i32(
                    crate::c::rem_i32(((seenOwnedCount) as i32), 100i32),
                    10i32,
                )) as u16);
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    ((digitNum) as u8),
                );
                drawNextDigit = 0u32;
                spriteId = CreateSprite(
                    (&raw const sNationalDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    40i16,
                    91i16,
                    1u8,
                );
                digitNum = ((crate::c::div_i32(
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1564)
                        .cast::<u16>())
                    .read()) as i32),
                    100i32,
                )) as u16);
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    ((digitNum) as u8),
                );
                if ((digitNum) as i32) != 0i32 {
                    drawNextDigit = 1u32;
                } else {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                spriteId = CreateSprite(
                    (&raw const sNationalDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    48i16,
                    91i16,
                    1u8,
                );
                digitNum = ((crate::c::div_i32(
                    crate::c::rem_i32(
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1564)
                            .cast::<u16>())
                        .read()) as i32),
                        100i32,
                    ),
                    10i32,
                )) as u16);
                if (((digitNum) as i32) != 0i32) || ((drawNextDigit) != 0) {
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        ((digitNum) as u8),
                    );
                } else {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                spriteId = CreateSprite(
                    (&raw const sNationalDexSeenOwnNumberSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    56i16,
                    91i16,
                    1u8,
                );
                digitNum = ((crate::c::rem_i32(
                    crate::c::rem_i32(
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1564)
                            .cast::<u16>())
                        .read()) as i32),
                        100i32,
                    ),
                    10i32,
                )) as u16);
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    ((digitNum) as u8),
                );
            }
            spriteId = CreateSprite(
                (&raw const sDexListStartMenuCursorSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                136i16,
                96i16,
                1u8,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        } else {
            spriteId = CreateSprite(
                (&raw const sDexListStartMenuCursorSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                136i16,
                80i16,
                1u8,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_EndMoveMonForInfoScreen(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_SeenOwnInfo(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1610))
            .read()) as i32)
            != 0i32
        {
            DestroySprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_MoveMonForInfoScreen(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (0u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (0u32) as i32);
        ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        if (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) != 48i32)
            || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) != 56i32)
        {
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > 48i32 {
                let __p1 = (sprite).wrapping_add(32).cast::<i16>();
                (__p1).write(((__p1).read()).wrapping_sub(1));
            }
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < 48i32 {
                let __p2 = (sprite).wrapping_add(32).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) > 56i32 {
                let __p3 = (sprite).wrapping_add(34).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_sub(1));
            }
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) < 56i32 {
                let __p4 = (sprite).wrapping_add(34).cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
            }
        } else {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_EndMoveMonForInfoScreen));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PokedexListMonSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut monId: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8);
        if (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1610))
        .read()) as i32)
            != 0i32)
            && (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1610))
            .read()) as i32)
                != 3i32)
        {
            FreeAndDestroyMonPicSprite(
                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1566))
                .cast::<u16>())
                .wrapping_offset(((monId) as i32) as isize))
                .read(),
            );
            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1566))
            .cast::<u16>())
            .wrapping_offset(((monId) as i32) as isize))
            .write(65535u16);
        } else {
            let mut var: u32 = 0u32;
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((crate::c::div_i32(
                    ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as u8) as i32) as isize,
                    ))
                    .read()) as i32)
                        .wrapping_mul(76i32),
                    256i32,
                )) as i16),
            );
            var = ((if ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    .wrapping_add(64i32)) as isize,
            ))
            .read()) as i32)
                != 0i32
            {
                crate::c::div_i32(
                    65536i32,
                    ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32)
                            .wrapping_add(64i32)) as isize,
                    ))
                    .read()) as i32),
                )
            } else {
                0i32
            }) as u32);
            if var > 65535u32 {
                var = 65535u32;
            }
            SetOamMatrix(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    .wrapping_add(1i32)) as u8),
                256u16,
                0u16,
                0u16,
                ((var) as u16),
            );
            crate::c::bf_write(
                (sprite).wrapping_add(3),
                1,
                5,
                ((((monId) as i32).wrapping_add(1i32)) as u32) as i32,
            );
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                > (-64i32))
                && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    < 64i32)
            {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
            } else {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            }
            if ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                as i32)
                <= (-64i32))
                || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    >= 64i32))
                && ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32)
            {
                FreeAndDestroyMonPicSprite(
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1566))
                    .cast::<u16>())
                    .wrapping_offset(((monId) as i32) as isize))
                    .read(),
                );
                ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1566))
                .cast::<u16>())
                .wrapping_offset(((monId) as i32) as isize))
                .write(65535u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Scrollbar(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1610))
        .read()) as i32)
            != 0i32)
            && (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1610))
            .read()) as i32)
                != 3i32)
        {
            DestroySprite(sprite);
        } else {
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((crate::c::div_i32(
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1550)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_mul(120i32),
                    ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1548)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_sub(1i32),
                )) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ScrollArrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1610))
        .read()) as i32)
            != 0i32)
            && (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1610))
            .read()) as i32)
                != 3i32)
        {
            DestroySprite(sprite);
        } else {
            let mut r0: u8 = 0u8;
            if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0 {
                if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1550)
                    .cast::<u16>())
                .read()) as i32)
                    == ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1548)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_sub(1i32)
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                } else {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                }
                r0 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as u8);
            } else {
                if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1550)
                    .cast::<u16>())
                .read()) as i32)
                    == 0i32
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                } else {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                }
                r0 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    .wrapping_sub(128i32)) as u8);
            }
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((crate::c::div_i32(
                    ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(((r0) as i32) as isize))
                    .read()) as i32),
                    64i32,
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    .wrapping_add(8i32)) as i16),
            );
            if ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1615))
            .read()) as i32)
                == 0i32)
                && (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1618)
                    .cast::<i16>())
                .read()) as i32)
                    == 0i32))
                && (((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                    == 0i32)
            {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            } else {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DexListInterfaceText(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1610))
        .read()) as i32)
            != 0i32)
            && (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1610))
            .read()) as i32)
                != 3i32)
        {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_RotatingPokeBall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1610))
        .read()) as i32)
            != 0i32)
            && (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1610))
            .read()) as i32)
                != 3i32)
        {
            DestroySprite(sprite);
        } else {
            let mut val: u8 = 0u8;
            let mut r3: i16 = 0i16;
            let mut r0: i16 = 0i16;
            val = ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1580))
            .read()) as i32)
                .wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as u8);
            r3 = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                .wrapping_offset(((val) as i32) as isize))
            .read();
            r0 = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                .wrapping_offset((((val) as i32).wrapping_add(64i32)) as isize))
            .read();
            SetOamMatrix(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
                ((r0) as u16),
                ((r3) as u16),
                ((((r3) as i32).wrapping_neg()) as u16),
                ((r0) as u16),
            );
            val = ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1580))
            .read()) as i32)
                .wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        .wrapping_add(64i32),
                )) as u8);
            r3 = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                .wrapping_offset(((val) as i32) as isize))
            .read();
            r0 = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                .wrapping_offset((((val) as i32).wrapping_add(64i32)) as isize))
            .read();
            ((sprite).wrapping_add(36).cast::<i16>())
                .write(((crate::c::div_i32(((r0) as i32).wrapping_mul(40i32), 256i32)) as i16));
            ((sprite).wrapping_add(38).cast::<i16>())
                .write(((crate::c::div_i32(((r3) as i32).wrapping_mul(40i32), 256i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DexListStartMenuCursor(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1610))
        .read()) as i32)
            != 0i32)
            && (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1610))
            .read()) as i32)
                != 3i32)
        {
            DestroySprite(sprite);
        } else {
            let mut r1: u16 = ((if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                .read())
            .wrapping_add(1610))
            .read()) as i32)
                == 0i32
            {
                80i32
            } else {
                96i32
            }) as u16);
            if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1615))
            .read())
                != 0)
                && (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1618)
                    .cast::<i16>())
                .read()) as i32)
                    == ((r1) as i32))
            {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    ((((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1616)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_mul(16i32)) as i16),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    ((crate::c::div_i32(
                        ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                            (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                .read()) as u8) as i32) as isize,
                        ))
                        .read()) as i32),
                        64i32,
                    )) as i16),
                );
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
            } else {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintInfoScreenText(str: *mut u8, left: u8, top: u8) {
    unsafe {
        let mut str = str;
        let mut left = left;
        let mut top = top;
        let mut color = crate::ffi::Align4([0u8; 3]);
        ((&raw mut color).cast::<u8>()).write(0u8);
        (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(15u8);
        (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(3u8);
        AddTextPrinterParameterized4(
            0u8,
            1u8,
            left,
            top,
            0u8,
            0u8,
            (&raw mut color).cast::<u8>(),
            (-1i8),
            str,
        );
    }
}
pub(crate) unsafe extern "C" fn LoadInfoScreen(item: *mut u8, monSpriteId: u8) -> u8 {
    unsafe {
        let mut item = item;
        let mut monSpriteId = monSpriteId;
        let mut taskId: u8 = 0u8;
        ((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>()).write(item);
        taskId = CreateTask(Some(Task_LoadInfoScreen), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(1i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((monSpriteId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(255i16);
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sInfoScreen_BgTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(16u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(3u8, AllocZeroed(2048u32));
        SetBgTilemapBuffer(2u8, AllocZeroed(2048u32));
        SetBgTilemapBuffer(1u8, AllocZeroed(2048u32));
        SetBgTilemapBuffer(0u8, AllocZeroed(2048u32));
        InitWindows(
            ((&raw const sInfoScreen_WindowTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        DeactivateAllTextPrinters();
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn IsInfoScreenScrolling(taskId: u8) -> u8 {
    unsafe {
        let mut taskId = taskId;
        if (!(((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read())
            != 0))
            && (core::mem::transmute::<_, usize>(
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .read(),
            ) == (Task_HandleInfoScreenInput as *const () as usize))
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
pub(crate) unsafe extern "C" fn StartInfoScreenScroll(item: *mut u8, taskId: u8) -> u8 {
    unsafe {
        let mut item = item;
        let mut taskId = taskId;
        ((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>()).write(item);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(1i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(0i16);
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn Task_LoadInfoScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
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
                || __sw1 == 10i32;
            if __sw1 == 0i32 || !__matched {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let mut r2: u16 = 0u16;
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1610))
                    .write(1u8);
                    ((&raw mut gPokedexVBlankCB)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .write(
                        (((&raw mut gMain).cast::<u8>())
                            .wrapping_add(12)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                    );
                    SetVBlankCallback(None);
                    r2 = 0u16;
                    if (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read())
                        != 0
                    {
                        r2 = ((((r2) as i32).wrapping_add(4096i32)) as u16);
                    }
                    if (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read())
                        != 0
                    {
                        r2 = ((((r2) as i32) | 512i32) as u16);
                    }
                    ResetOtherVideoRegisters(r2);
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                DecompressAndLoadBgGfxUsingHeap(
                    3u8,
                    (((&raw mut gPokedexMenu_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    8192u32,
                    0u16,
                    0u8,
                );
                CopyToBgTilemapBuffer(
                    3u8,
                    (((&raw mut gPokedexInfoScreen_Tilemap).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    0u16,
                    0u16,
                );
                FillWindowPixelBuffer(0u8, 0u8);
                PutWindowTilemap(0u8);
                PutWindowTilemap(1u8);
                DrawFootprint(
                    1u8,
                    ((((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read(),
                );
                CopyWindowToVram(1u8, 2u8);
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                LoadScreenSelectBarMain(13u16);
                HighlightScreenSelectBarItem(
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1613))
                    .read(),
                    13u16,
                );
                LoadPokedexBgPalette(
                    (crate::c::bf_read(
                        (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1612),
                        0,
                        1,
                        false,
                    ) as u8),
                );
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                PrintMonInfo(
                    ((((((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read()) as u32),
                    ((if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1554)
                        .cast::<u16>())
                    .read()) as i32)
                        == 0i32
                    {
                        0i32
                    } else {
                        1i32
                    }) as u32),
                    ((crate::c::bf_read(
                        (((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2),
                        1,
                        1,
                        false,
                    ) as u16) as u32),
                    0u32,
                );
                if !((crate::c::bf_read(
                    (((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2),
                    1,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    LoadPalette(
                        ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(1))
                        .cast::<u8>(),
                        49u16,
                        30u16,
                    );
                }
                CopyWindowToVram(0u8, 3u8);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(2u8);
                CopyBgTilemapBufferToVram(3u8);
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read())
                    != 0)
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(
                        ((CreateMonSpriteFromNationalDexNumber(
                            ((((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u16>())
                            .read(),
                            48i16,
                            56i16,
                            0u16,
                        )) as i16),
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(4))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(5),
                        2,
                        2,
                        (0u16) as i32,
                    );
                }
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                {
                    let mut preservedPalettes: u32 = 0u32;
                    if (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read())
                        != 0
                    {
                        preservedPalettes = 20u32;
                    }
                    if (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read())
                        != 0
                    {
                        preservedPalettes = (preservedPalettes
                            | ((crate::c::shl_i32(
                                1i32,
                                ((((crate::c::bf_read(
                                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(4))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(5),
                                    4,
                                    4,
                                    false,
                                ) as u16) as i32)
                                    .wrapping_add(16i32)) as u32),
                            )) as u32));
                    }
                    BeginNormalPaletteFade(!(preservedPalettes), 0i8, 16u8, 0u8, 0u16);
                    SetVBlankCallback(
                        ((&raw mut gPokedexVBlankCB)
                            .cast::<u8>()
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                    );
                    let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetGpuReg(84u8, 0u16);
                SetGpuReg(0u8, 4160u16);
                HideBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                ShowBg(3u8);
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p9).write(((__p9).read()).wrapping_add(1));
                    if !((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read())
                        != 0)
                    {
                        StopCryAndClearCrySongs();
                        PlayCry_NormalNoDucking(
                            NationalPokedexNumToSpecies(
                                ((((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u16>())
                                .read(),
                            ),
                            0i8,
                            125i8,
                            10u8,
                        );
                    } else {
                        let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                        (__p10).write(((__p10).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                if !((IsCryPlayingOrClearCrySongs()) != 0) {
                    let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(1i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(1i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_HandleInfoScreenInput));
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(0u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FreeInfoScreenWindowAndBgBuffers() {
    unsafe {
        let mut tilemapBuffer: *mut u8 = core::ptr::null_mut();
        FreeAllWindowBuffers();
        tilemapBuffer = GetBgTilemapBuffer(0u8);
        if !(tilemapBuffer).is_null() {
            Free(tilemapBuffer);
        }
        tilemapBuffer = GetBgTilemapBuffer(1u8);
        if !(tilemapBuffer).is_null() {
            Free(tilemapBuffer);
        }
        tilemapBuffer = GetBgTilemapBuffer(2u8);
        if !(tilemapBuffer).is_null() {
            Free(tilemapBuffer);
        }
        tilemapBuffer = GetBgTilemapBuffer(3u8);
        if !(tilemapBuffer).is_null() {
            Free(tilemapBuffer);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleInfoScreenInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .read())
            != 0
        {
            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_LoadInfoScreenWaitForFade));
            PlaySE(108u16);
            return;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ExitInfoScreen));
            PlaySE(3u16);
            return;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            'l1: {
                let __sw1 = ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1613))
                .read()) as i32);
                if __sw1 == 0i32 {
                    BeginNormalPaletteFade(4294967275u32, 0i8, 0u8, 16u8, 0u16);
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1614))
                    .write(1u8);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_SwitchScreensFromInfoScreen));
                    PlaySE(21u16);
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    BeginNormalPaletteFade(4294967275u32, 0i8, 0u8, 16u8, 0u16);
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1614))
                    .write(2u8);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_SwitchScreensFromInfoScreen));
                    PlaySE(21u16);
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    if !((crate::c::bf_read(
                        (((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2),
                        1,
                        1,
                        false,
                    ) as u16)
                        != 0)
                    {
                        PlaySE(32u16);
                    } else {
                        BeginNormalPaletteFade(4294967275u32, 0i8, 0u8, 16u8, 0u16);
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1614))
                        .write(3u8);
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Task_SwitchScreensFromInfoScreen));
                        PlaySE(21u16);
                    }
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_ExitInfoScreen));
                    PlaySE(3u16);
                    break 'l1;
                }
            }
            return;
        }
        if ((((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0)
            || ((((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 512i32)
                != 0)
                && (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(19))
                    .read()) as i32)
                    == 1i32)))
            && (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1613))
            .read()) as i32)
                > 0i32)
        {
            let __p2 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1613);
            (__p2).write(((__p2).read()).wrapping_sub(1));
            HighlightScreenSelectBarItem(
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1613))
                .read(),
                13u16,
            );
            PlaySE(109u16);
            return;
        }
        if ((((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0)
            || ((((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 256i32)
                != 0)
                && (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(19))
                    .read()) as i32)
                    == 1i32)))
            && (((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1613))
            .read()) as i32)
                < 3i32)
        {
            let __p3 = (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1613);
            (__p3).write(((__p3).read()).wrapping_add(1));
            HighlightScreenSelectBarItem(
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1613))
                .read(),
                13u16,
            );
            PlaySE(109u16);
            return;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchScreensFromInfoScreen(taskId: u8) {
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
            FreeAndDestroyMonPicSprite(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as u16),
            );
            'l1: {
                let __sw1 = ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1614))
                .read()) as i32);
                let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
                if __sw1 == 1i32 || !__matched {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_LoadAreaScreen));
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_LoadCryScreen));
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_LoadSizeScreen));
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LoadInfoScreenWaitForFade(taskId: u8) {
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
            FreeAndDestroyMonPicSprite(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as u16),
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_LoadInfoScreen));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ExitInfoScreen(taskId: u8) {
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
            FreeAndDestroyMonPicSprite(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as u16),
            );
            FreeInfoScreenWindowAndBgBuffers();
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LoadAreaScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 || !__matched {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1610))
                    .write(5u8);
                    ((&raw mut gPokedexVBlankCB)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .write(
                        (((&raw mut gMain).cast::<u8>())
                            .wrapping_add(12)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                    );
                    SetVBlankCallback(None);
                    ResetOtherVideoRegisters(512u16);
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1613))
                    .write(0u8);
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                LoadScreenSelectBarSubmenu(13u16);
                HighlightSubmenuScreenSelectBarItem(0u8, 13u16);
                LoadPokedexBgPalette(
                    (crate::c::bf_read(
                        (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1612),
                        0,
                        1,
                        false,
                    ) as u8),
                );
                SetGpuReg(10u8, 3328u16);
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ShowPokedexAreaScreen(
                    NationalPokedexNumToSpecies(
                        ((((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u16>())
                        .read(),
                    ),
                    (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1614),
                );
                SetVBlankCallback(
                    ((&raw mut gPokedexVBlankCB)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                );
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1614))
                .write(0u8);
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(0u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_WaitForAreaScreenInput));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForAreaScreenInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1614))
            .read()) as i32)
            != 0i32
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_SwitchScreensFromAreaScreen));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchScreensFromAreaScreen(taskId: u8) {
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
            'l1: {
                let __sw1 = ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1614))
                .read()) as i32);
                let __matched = __sw1 == 1i32 || __sw1 == 2i32;
                if __sw1 == 1i32 || !__matched {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_LoadInfoScreen));
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_LoadCryScreen));
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LoadCryScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
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
                || __sw1 == 10i32;
            if __sw1 == 0i32 || !__matched {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    m4aMPlayStop((&raw mut gMPlayInfo_BGM).cast::<u8>());
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1610))
                    .write(6u8);
                    ((&raw mut gPokedexVBlankCB)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .write(
                        (((&raw mut gMain).cast::<u8>())
                            .wrapping_add(12)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                    );
                    SetVBlankCallback(None);
                    ResetOtherVideoRegisters(512u16);
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1613))
                    .write(1u8);
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                DecompressAndLoadBgGfxUsingHeap(
                    3u8,
                    (((&raw mut gPokedexMenu_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    8192u32,
                    0u16,
                    0u8,
                );
                CopyToBgTilemapBuffer(
                    3u8,
                    (((&raw mut gPokedexCryScreen_Tilemap).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    0u16,
                    0u16,
                );
                FillWindowPixelBuffer(0u8, 0u8);
                PutWindowTilemap(0u8);
                PutWindowTilemap(3u8);
                PutWindowTilemap(2u8);
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                LoadScreenSelectBarSubmenu(13u16);
                HighlightSubmenuScreenSelectBarItem(1u8, 13u16);
                LoadPokedexBgPalette(
                    (crate::c::bf_read(
                        (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1612),
                        0,
                        1,
                        false,
                    ) as u8),
                );
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                ResetPaletteFade();
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                PrintInfoScreenText((&raw mut gText_CryOf).cast::<u8>(), 82u8, 33u8);
                PrintCryScreenSpeciesName(
                    0u8,
                    ((((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read(),
                    82u8,
                    49u8,
                );
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(
                    ((CreateMonSpriteFromNationalDexNumber(
                        ((((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u16>())
                        .read(),
                        48i16,
                        56i16,
                        0u16,
                    )) as i16),
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(5),
                    2,
                    2,
                    (0u16) as i32,
                );
                ((&raw mut gDexCryScreenState).cast::<u8>()).write(0u8);
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                {
                    let mut waveformWindow = crate::ffi::Align4([0u8; 8]);
                    (((&raw mut waveformWindow).cast::<u8>()).cast::<u16>()).write(16416u16);
                    (((&raw mut waveformWindow).cast::<u8>()).wrapping_add(2)).write(31u8);
                    (((&raw mut waveformWindow).cast::<u8>()).wrapping_add(3)).write(8u8);
                    (((&raw mut waveformWindow).cast::<u8>()).wrapping_add(5)).write(30u8);
                    (((&raw mut waveformWindow).cast::<u8>()).wrapping_add(4)).write(12u8);
                    if (LoadCryWaveformWindow((&raw mut waveformWindow).cast::<u8>(), 2u8)) != 0 {
                        let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                        (__p7).write(((__p7).read()).wrapping_add(1));
                        ((&raw mut gDexCryScreenState).cast::<u8>()).write(0u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                {
                    let mut cryMeter = crate::ffi::Align4([0u8; 8]);
                    (((&raw mut cryMeter).cast::<u8>()).wrapping_add(3)).write(9u8);
                    (((&raw mut cryMeter).cast::<u8>()).wrapping_add(4)).write(18u8);
                    (((&raw mut cryMeter).cast::<u8>()).wrapping_add(5)).write(3u8);
                    if (LoadCryMeter((&raw mut cryMeter).cast::<u8>(), 3u8)) != 0 {
                        let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                        (__p8).write(((__p8).read()).wrapping_add(1));
                    }
                    CopyWindowToVram(3u8, 2u8);
                    CopyWindowToVram(0u8, 3u8);
                    CopyBgTilemapBufferToVram(0u8);
                    CopyBgTilemapBufferToVram(1u8);
                    CopyBgTilemapBufferToVram(2u8);
                    CopyBgTilemapBufferToVram(3u8);
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                BeginNormalPaletteFade(4294967275u32, 0i8, 16u8, 0u8, 0u16);
                SetVBlankCallback(
                    ((&raw mut gPokedexVBlankCB)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                );
                let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetGpuReg(84u8, 0u16);
                SetGpuReg(0u8, 4160u16);
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                ShowBg(3u8);
                let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1614))
                .write(0u8);
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(0u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_HandleCryScreenInput));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleCryScreenInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        UpdateCryWaveformWindow(2u8);
        if (IsCryPlaying()) != 0 {
            LoadPlayArrowPalette(1u8);
        } else {
            LoadPlayArrowPalette(0u8);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            LoadPlayArrowPalette(1u8);
            CryScreenPlayButton(NationalPokedexNumToSpecies(
                ((((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>())
                .read(),
            ));
            return;
        } else {
            if !((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0)
            {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    BeginNormalPaletteFade(4294967275u32, 0i8, 0u8, 16u8, 0u16);
                    m4aMPlayContinue((&raw mut gMPlayInfo_BGM).cast::<u8>());
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1614))
                    .write(1u8);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_SwitchScreensFromCryScreen));
                    PlaySE(3u16);
                    return;
                }
                if (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 32i32)
                    != 0)
                    || ((((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 512i32)
                        != 0)
                        && (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(19))
                        .read()) as i32)
                            == 1i32))
                {
                    BeginNormalPaletteFade(4294967275u32, 0i8, 0u8, 16u8, 0u16);
                    m4aMPlayContinue((&raw mut gMPlayInfo_BGM).cast::<u8>());
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1614))
                    .write(2u8);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_SwitchScreensFromCryScreen));
                    PlaySE(109u16);
                    return;
                }
                if (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 16i32)
                    != 0)
                    || ((((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 256i32)
                        != 0)
                        && (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(19))
                        .read()) as i32)
                            == 1i32))
                {
                    if !((crate::c::bf_read(
                        (((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2),
                        1,
                        1,
                        false,
                    ) as u16)
                        != 0)
                    {
                        PlaySE(32u16);
                    } else {
                        BeginNormalPaletteFade(4294967275u32, 0i8, 0u8, 16u8, 0u16);
                        m4aMPlayContinue((&raw mut gMPlayInfo_BGM).cast::<u8>());
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1614))
                        .write(3u8);
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Task_SwitchScreensFromCryScreen));
                        PlaySE(109u16);
                    }
                    return;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchScreensFromCryScreen(taskId: u8) {
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
            FreeCryScreen();
            FreeAndDestroyMonPicSprite(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as u16),
            );
            'l1: {
                let __sw1 = ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1614))
                .read()) as i32);
                let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
                if __sw1 == 1i32 || !__matched {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_LoadInfoScreen));
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_LoadAreaScreen));
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_LoadSizeScreen));
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadPlayArrowPalette(cryPlaying: u8) {
    unsafe {
        let mut cryPlaying = cryPlaying;
        let mut color: u16 = 0u16;
        if (cryPlaying) != 0 {
            color = 914u16;
        } else {
            color = 687u16;
        }
        LoadPalette((&raw mut color).cast::<u8>(), 93u16, 2u16);
    }
}
pub(crate) unsafe extern "C" fn Task_LoadSizeScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
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
                || __sw1 == 9i32;
            if __sw1 == 0i32 || !__matched {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1610))
                    .write(7u8);
                    ((&raw mut gPokedexVBlankCB)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .write(
                        (((&raw mut gMain).cast::<u8>())
                            .wrapping_add(12)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                    );
                    SetVBlankCallback(None);
                    ResetOtherVideoRegisters(512u16);
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1613))
                    .write(2u8);
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                DecompressAndLoadBgGfxUsingHeap(
                    3u8,
                    (((&raw mut gPokedexMenu_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    8192u32,
                    0u16,
                    0u8,
                );
                CopyToBgTilemapBuffer(
                    3u8,
                    (((&raw mut gPokedexSizeScreen_Tilemap).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    0u16,
                    0u16,
                );
                FillWindowPixelBuffer(0u8, 0u8);
                PutWindowTilemap(0u8);
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                LoadScreenSelectBarSubmenu(13u16);
                HighlightSubmenuScreenSelectBarItem(2u8, 13u16);
                LoadPokedexBgPalette(
                    (crate::c::bf_read(
                        (((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1612),
                        0,
                        1,
                        false,
                    ) as u8),
                );
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                {
                    let mut string = crate::ffi::Align4([0u8; 64]);
                    StringCopy(
                        (&raw mut string).cast::<u8>(),
                        (&raw mut gText_SizeComparedTo).cast::<u8>(),
                    );
                    StringAppend(
                        (&raw mut string).cast::<u8>(),
                        (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                    );
                    PrintInfoScreenText(
                        (&raw mut string).cast::<u8>(),
                        ((GetStringCenterAlignXOffset(1i32, (&raw mut string).cast::<u8>(), 240i32))
                            as u8),
                        121u8,
                    );
                    let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                ResetPaletteFade();
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                spriteId = ((CreateSizeScreenTrainerPic(
                    PlayerGenderToFrontTrainerPicId(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8))
                            .read(),
                    ),
                    152i16,
                    56i16,
                    0i8,
                )) as u8);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(1),
                    0,
                    2,
                    (1u32) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(3),
                    1,
                    5,
                    (1u32) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(5),
                    2,
                    2,
                    (0u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(
                    (((((((&raw const gPokedexEntries).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<u16>())
                            .read()) as i32) as isize
                                * 32,
                        ))
                    .wrapping_add(28)
                    .cast::<u16>())
                    .read()) as i16),
                );
                SetOamMatrix(
                    1u8,
                    (((((&raw const gPokedexEntries).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<u16>())
                            .read()) as i32) as isize
                                * 32,
                        ))
                    .wrapping_add(26)
                    .cast::<u16>())
                    .read(),
                    0u16,
                    0u16,
                    (((((&raw const gPokedexEntries).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<u16>())
                            .read()) as i32) as isize
                                * 32,
                        ))
                    .wrapping_add(26)
                    .cast::<u16>())
                    .read(),
                );
                LoadPalette(
                    (((&raw const sSizeScreenSilhouette_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    (((((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(5),
                        4,
                        4,
                        false,
                    ) as u16) as i32)
                        .wrapping_add(16i32))
                    .wrapping_mul(16i32)) as u16),
                    32u16,
                );
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(((spriteId) as i16));
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                spriteId = ((CreateMonSpriteFromNationalDexNumber(
                    ((((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read(),
                    88i16,
                    56i16,
                    1u16,
                )) as u8);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(1),
                    0,
                    2,
                    (1u32) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(3),
                    1,
                    5,
                    (2u32) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(5),
                    2,
                    2,
                    (0u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(
                    (((((((&raw const gPokedexEntries).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<u16>())
                            .read()) as i32) as isize
                                * 32,
                        ))
                    .wrapping_add(24)
                    .cast::<u16>())
                    .read()) as i16),
                );
                SetOamMatrix(
                    2u8,
                    (((((&raw const gPokedexEntries).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<u16>())
                            .read()) as i32) as isize
                                * 32,
                        ))
                    .wrapping_add(22)
                    .cast::<u16>())
                    .read(),
                    0u16,
                    0u16,
                    (((((&raw const gPokedexEntries).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sPokedexListItem).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<u16>())
                            .read()) as i32) as isize
                                * 32,
                        ))
                    .wrapping_add(22)
                    .cast::<u16>())
                    .read(),
                );
                LoadPalette(
                    (((&raw const sSizeScreenSilhouette_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    (((((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(5),
                        4,
                        4,
                        false,
                    ) as u16) as i32)
                        .wrapping_add(16i32))
                    .wrapping_mul(16i32)) as u16),
                    32u16,
                );
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(((spriteId) as i16));
                CopyWindowToVram(0u8, 3u8);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(2u8);
                CopyBgTilemapBufferToVram(3u8);
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                BeginNormalPaletteFade(4294967275u32, 0i8, 16u8, 0u8, 0u16);
                SetVBlankCallback(
                    ((&raw mut gPokedexVBlankCB)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                );
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetGpuReg(84u8, 0u16);
                SetGpuReg(0u8, 4160u16);
                HideBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                ShowBg(3u8);
                let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1614))
                    .write(0u8);
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(0u8);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_HandleSizeScreenInput));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSizeScreenInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            BeginNormalPaletteFade(4294967275u32, 0i8, 0u8, 16u8, 0u16);
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1614))
                .write(1u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_SwitchScreensFromSizeScreen));
            PlaySE(3u16);
        } else {
            if (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 32i32)
                != 0)
                || ((((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 512i32)
                    != 0)
                    && (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(19))
                        .read()) as i32)
                        == 1i32))
            {
                BeginNormalPaletteFade(4294967275u32, 0i8, 0u8, 16u8, 0u16);
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1614))
                .write(2u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_SwitchScreensFromSizeScreen));
                PlaySE(109u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchScreensFromSizeScreen(taskId: u8) {
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
            FreeAndDestroyMonPicSprite(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as u16),
            );
            FreeAndDestroyTrainerPicSprite(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as u16),
            );
            'l1: {
                let __sw1 = ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1614))
                .read()) as i32);
                let __matched = __sw1 == 1i32 || __sw1 == 2i32;
                if __sw1 == 1i32 || !__matched {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_LoadInfoScreen));
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_LoadCryScreen));
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadScreenSelectBarMain(unused: u16) {
    unsafe {
        let mut unused = unused;
        CopyToBgTilemapBuffer(
            1u8,
            (((&raw mut gPokedexScreenSelectBarMain_Tilemap).cast::<u32>()).cast::<u32>())
                .cast::<u8>(),
            0u16,
            0u16,
        );
    }
}
pub(crate) unsafe extern "C" fn LoadScreenSelectBarSubmenu(unused: u16) {
    unsafe {
        let mut unused = unused;
        CopyToBgTilemapBuffer(
            1u8,
            (((&raw mut gPokedexScreenSelectBarSubmenu_Tilemap).cast::<u32>()).cast::<u32>())
                .cast::<u8>(),
            0u16,
            0u16,
        );
    }
}
pub(crate) unsafe extern "C" fn HighlightScreenSelectBarItem(selectedScreen: u8, unused: u16) {
    unsafe {
        let mut selectedScreen = selectedScreen;
        let mut unused = unused;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut ptr: *mut u16 = (GetBgTilemapBuffer(1u8)).cast::<u16>();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut row: u8 =
                        (((((i) as i32).wrapping_mul(7i32)).wrapping_add(1i32)) as u8);
                    let mut newPalette: u16 = 0u16;
                    'l3: loop {
                        'l4: {
                            newPalette = 16384u16;
                            if ((i) as i32) == ((selectedScreen) as i32) {
                                newPalette = 8192u16;
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                    {
                        j = 0u8;
                        'l5: loop {
                            if !(((j) as i32) < 7i32) {
                                break 'l5;
                            }
                            'l6: {
                                ((ptr).wrapping_offset(
                                    (((row) as i32).wrapping_add(((j) as i32))) as isize,
                                ))
                                .write(
                                    ((crate::c::rem_i32(
                                        ((((ptr).wrapping_offset(
                                            (((row) as i32).wrapping_add(((j) as i32))) as isize,
                                        ))
                                        .read()) as i32),
                                        4096i32,
                                    ) | ((newPalette) as i32))
                                        as u16),
                                );
                                ((ptr).wrapping_offset(
                                    ((((row) as i32).wrapping_add(((j) as i32)))
                                        .wrapping_add(32i32))
                                        as isize,
                                ))
                                .write(
                                    ((crate::c::rem_i32(
                                        ((((ptr).wrapping_offset(
                                            ((((row) as i32).wrapping_add(((j) as i32)))
                                                .wrapping_add(32i32))
                                                as isize,
                                        ))
                                        .read()) as i32),
                                        4096i32,
                                    ) | ((newPalette) as i32))
                                        as u16),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyBgTilemapBufferToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn HighlightSubmenuScreenSelectBarItem(a: u8, b: u16) {
    unsafe {
        let mut a = a;
        let mut b = b;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut ptr: *mut u16 = (GetBgTilemapBuffer(1u8)).cast::<u16>();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut row: u8 =
                        (((((i) as i32).wrapping_mul(7i32)).wrapping_add(1i32)) as u8);
                    let mut newPalette: u32 = 0u32;
                    'l3: loop {
                        'l4: {
                            if (((i) as i32) == ((a) as i32)) || (((i) as i32) == 3i32) {
                                newPalette = 8192u32;
                            } else {
                                newPalette = 16384u32;
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                    {
                        j = 0u8;
                        'l5: loop {
                            if !(((j) as i32) < 7i32) {
                                break 'l5;
                            }
                            'l6: {
                                ((ptr).wrapping_offset(
                                    (((row) as i32).wrapping_add(((j) as i32))) as isize,
                                ))
                                .write(
                                    ((((crate::c::rem_i32(
                                        ((((ptr).wrapping_offset(
                                            (((row) as i32).wrapping_add(((j) as i32))) as isize,
                                        ))
                                        .read()) as i32),
                                        4096i32,
                                    )) as u32)
                                        | newPalette) as u16),
                                );
                                ((ptr).wrapping_offset(
                                    ((((row) as i32).wrapping_add(((j) as i32)))
                                        .wrapping_add(32i32))
                                        as isize,
                                ))
                                .write(
                                    ((((crate::c::rem_i32(
                                        ((((ptr).wrapping_offset(
                                            ((((row) as i32).wrapping_add(((j) as i32)))
                                                .wrapping_add(32i32))
                                                as isize,
                                        ))
                                        .read()) as i32),
                                        4096i32,
                                    )) as u32)
                                        | newPalette) as u16),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyBgTilemapBufferToVram(1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisplayCaughtMonDexPage(dexNum: u16, otId: u32, personality: u32) -> u8 {
    unsafe {
        let mut dexNum = dexNum;
        let mut otId = otId;
        let mut personality = personality;
        let mut taskId: u8 = CreateTask(Some(Task_DisplayCaughtMonDexPage), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((dexNum) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(12))
        .write(((otId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(13))
        .write(((otId >> 16) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(14))
        .write(((personality) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(((personality >> 16) as i16));
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn Task_DisplayCaughtMonDexPage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        let mut dexNum: u16 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u16);
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32;
            if __sw1 == 0i32 || !__matched {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ((&raw mut gPokedexVBlankCB)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .write(
                        (((&raw mut gMain).cast::<u8>())
                            .wrapping_add(12)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                    );
                    SetVBlankCallback(None);
                    ResetOtherVideoRegisters(256u16);
                    ResetBgsAndClearDma3BusyFlags(0u32);
                    InitBgsFromTemplates(
                        0u8,
                        ((&raw const sNewEntryInfoScreen_BgTemplate)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                        ((crate::c::div_u32(8u32, 4u32)) as u8),
                    );
                    SetBgTilemapBuffer(3u8, AllocZeroed(2048u32));
                    SetBgTilemapBuffer(2u8, AllocZeroed(2048u32));
                    InitWindows(
                        ((&raw const sNewEntryInfoScreen_WindowTemplates)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    );
                    DeactivateAllTextPrinters();
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                DecompressAndLoadBgGfxUsingHeap(
                    3u8,
                    (((&raw mut gPokedexMenu_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    8192u32,
                    0u16,
                    0u8,
                );
                CopyToBgTilemapBuffer(
                    3u8,
                    (((&raw mut gPokedexInfoScreen_Tilemap).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    0u16,
                    0u16,
                );
                FillWindowPixelBuffer(0u8, 0u8);
                PutWindowTilemap(0u8);
                PutWindowTilemap(1u8);
                DrawFootprint(
                    1u8,
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as u16),
                );
                CopyWindowToVram(1u8, 2u8);
                ResetPaletteFade();
                LoadPokedexBgPalette(0u8);
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p3 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                PrintMonInfo(((dexNum) as u32), IsNationalPokedexEnabled(), 1u32, 1u32);
                CopyWindowToVram(0u8, 3u8);
                CopyBgTilemapBufferToVram(2u8);
                CopyBgTilemapBufferToVram(3u8);
                let __p4 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                spriteId =
                    ((CreateMonSpriteFromNationalDexNumber(dexNum, 48i16, 56i16, 0u16)) as u8);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(5),
                    2,
                    2,
                    (0u16) as i32,
                );
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                SetVBlankCallback(
                    ((&raw mut gPokedexVBlankCB)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                );
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(((spriteId) as i16));
                let __p5 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetGpuReg(84u8, 0u16);
                SetGpuReg(0u8, 4160u16);
                ShowBg(2u8);
                ShowBg(3u8);
                let __p6 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    PlayCry_Normal(NationalPokedexNumToSpecies(dexNum), 0i8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(0i16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_HandleCaughtMonPageInput));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleCaughtMonPageInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            BeginNormalPaletteFade(65535u32, 0i8, 0u8, 16u8, 0u16);
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_SlideCaughtMonToCenter));
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ExitCaughtMonPage));
        } else {
            if ((({
                let __p1 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2);
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                & 16i32)
                != 0
            {
                LoadPalette(
                    ((((&raw mut gPokedexBgHoenn_Pal).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(1))
                    .cast::<u8>(),
                    49u16,
                    14u16,
                );
            } else {
                LoadPalette(
                    ((((&raw mut gPokedexBgHoenn_Pal).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(49))
                    .cast::<u8>(),
                    49u16,
                    14u16,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ExitCaughtMonPage(taskId: u8) {
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
            let mut species: u16 = 0u16;
            let mut otId: u32 = 0u32;
            let mut personality: u32 = 0u32;
            let mut paletteNum: u8 = 0u8;
            let mut lzPaletteData: *mut u32 = core::ptr::null_mut();
            let mut buffer: *mut u8 = core::ptr::null_mut();
            SetGpuReg(0u8, 4160u16);
            FreeAllWindowBuffers();
            buffer = GetBgTilemapBuffer(2u8);
            if !(buffer).is_null() {
                Free(buffer);
            }
            buffer = GetBgTilemapBuffer(3u8);
            if !(buffer).is_null() {
                Free(buffer);
            }
            species = NationalPokedexNumToSpecies(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u16),
            );
            otId = ((((((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(13))
            .read()) as u16) as i32)
                << 16)
                | (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(12))
                .read()) as u16) as i32)) as u32);
            personality = ((((((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .read()) as u16) as i32)
                << 16)
                | (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(14))
                .read()) as u16) as i32)) as u32);
            paletteNum = ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(5),
                4,
                4,
                false,
            ) as u16) as u8);
            lzPaletteData = GetMonSpritePalFromSpeciesAndPersonality(species, otId, personality);
            LoadCompressedPalette(
                lzPaletteData,
                (((256i32).wrapping_add(((paletteNum) as i32).wrapping_mul(16i32))) as u16),
                32u16,
            );
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_SlideCaughtMonToCenter(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            < crate::c::div_i32(240i32, 2i32)
        {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
        }
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            > crate::c::div_i32(240i32, 2i32)
        {
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_sub(2i32)) as i16));
        }
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            < crate::c::div_i32(160i32, 2i32)
        {
            let __p3 = (sprite).wrapping_add(34).cast::<i16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_add(1i32)) as i16));
        }
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            > crate::c::div_i32(160i32, 2i32)
        {
            let __p4 = (sprite).wrapping_add(34).cast::<i16>();
            (__p4).write((((((__p4).read()) as i32).wrapping_sub(1i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn PrintMonInfo(num: u32, value: u32, owned: u32, newEntry: u32) {
    unsafe {
        let mut num = num;
        let mut value = value;
        let mut owned = owned;
        let mut newEntry = newEntry;
        let mut str = crate::ffi::Align4([0u8; 16]);
        let mut str2 = crate::ffi::Align4([0u8; 32]);
        let mut natNum: u16 = 0u16;
        let mut name: *mut u8 = core::ptr::null_mut();
        let mut category: *mut u8 = core::ptr::null_mut();
        let mut description: *mut u8 = core::ptr::null_mut();
        if (newEntry) != 0 {
            PrintInfoScreenText(
                (&raw mut gText_PokedexRegistration).cast::<u8>(),
                ((GetStringCenterAlignXOffset(
                    1i32,
                    (&raw mut gText_PokedexRegistration).cast::<u8>(),
                    240i32,
                )) as u8),
                0u8,
            );
        }
        if value == 0u32 {
            value = ((NationalToHoennOrder(((num) as u16))) as u32);
        } else {
            value = num;
        }
        ConvertIntToDecimalStringN(
            StringCopy(
                (&raw mut str).cast::<u8>(),
                (&raw mut gText_NumberClear01).cast::<u8>(),
            ),
            ((value) as i32),
            2i32,
            3u8,
        );
        PrintInfoScreenText((&raw mut str).cast::<u8>(), 96u8, 25u8);
        natNum = NationalPokedexNumToSpecies(((num) as u16));
        if (natNum) != 0 {
            name = (((&raw mut gSpeciesNames).cast::<u8>())
                .wrapping_offset(((natNum) as i32) as isize * 11))
            .cast::<u8>();
        } else {
            name = ((&raw const sText_TenDashes2).cast::<u8>().cast_mut()).cast::<u8>();
        }
        PrintInfoScreenText(name, 132u8, 25u8);
        if (owned) != 0 {
            CopyMonCategoryText(((num) as i32), (&raw mut str2).cast::<u8>());
            category = (&raw mut str2).cast::<u8>();
        } else {
            category = (&raw mut gText_5MarksPokemon).cast::<u8>();
        }
        PrintInfoScreenText(category, 100u8, 41u8);
        PrintInfoScreenText((&raw mut gText_HTHeight).cast::<u8>(), 96u8, 57u8);
        PrintInfoScreenText((&raw mut gText_WTWeight).cast::<u8>(), 96u8, 73u8);
        if (owned) != 0 {
            PrintMonHeight(
                (((((&raw const gPokedexEntries).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((num) as i32) as isize * 32))
                .wrapping_add(12)
                .cast::<u16>())
                .read(),
                129u8,
                57u8,
            );
            PrintMonWeight(
                (((((&raw const gPokedexEntries).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((num) as i32) as isize * 32))
                .wrapping_add(14)
                .cast::<u16>())
                .read(),
                129u8,
                73u8,
            );
        } else {
            PrintInfoScreenText((&raw mut gText_UnkHeight).cast::<u8>(), 129u8, 57u8);
            PrintInfoScreenText((&raw mut gText_UnkWeight).cast::<u8>(), 129u8, 73u8);
        }
        if (owned) != 0 {
            description = (((((&raw const gPokedexEntries).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((num) as i32) as isize * 32))
            .wrapping_add(16)
            .cast::<*mut u8>())
            .read();
        } else {
            description = ((&raw const sExpandedPlaceholder_PokedexDescription)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>();
        }
        PrintInfoScreenText(
            description,
            ((GetStringCenterAlignXOffset(1i32, description, 240i32)) as u8),
            95u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintMonHeight(height: u16, left: u8, top: u8) {
    unsafe {
        let mut height = height;
        let mut left = left;
        let mut top = top;
        let mut buffer = crate::ffi::Align4([0u8; 16]);
        let mut inches: u32 = 0u32;
        let mut feet: u32 = 0u32;
        let mut i: u8 = 0u8;
        inches = ((crate::c::div_i32(((height) as i32).wrapping_mul(10000i32), 254i32)) as u32);
        if crate::c::rem_u32(inches, 10u32) >= 5u32 {
            inches = (inches).wrapping_add(10u32);
        }
        feet = crate::c::div_u32(inches, 120u32);
        inches = crate::c::div_u32((inches).wrapping_sub((feet).wrapping_mul(120u32)), 10u32);
        (((&raw mut buffer).cast::<u8>()).wrapping_offset(
            (({
                let __t1 = i;
                i = (i).wrapping_add(1);
                __t1
            }) as i32) as isize,
        ))
        .write(252u8);
        (((&raw mut buffer).cast::<u8>()).wrapping_offset(
            (({
                let __t2 = i;
                i = (i).wrapping_add(1);
                __t2
            }) as i32) as isize,
        ))
        .write(19u8);
        if crate::c::div_u32(feet, 10u32) == 0u32 {
            (((&raw mut buffer).cast::<u8>()).wrapping_offset(
                (({
                    let __t3 = i;
                    i = (i).wrapping_add(1);
                    __t3
                }) as i32) as isize,
            ))
            .write(18u8);
            (((&raw mut buffer).cast::<u8>()).wrapping_offset(
                (({
                    let __t4 = i;
                    i = (i).wrapping_add(1);
                    __t4
                }) as i32) as isize,
            ))
            .write((((feet).wrapping_add(161u32)) as u8));
        } else {
            (((&raw mut buffer).cast::<u8>()).wrapping_offset(
                (({
                    let __t5 = i;
                    i = (i).wrapping_add(1);
                    __t5
                }) as i32) as isize,
            ))
            .write(12u8);
            (((&raw mut buffer).cast::<u8>()).wrapping_offset(
                (({
                    let __t6 = i;
                    i = (i).wrapping_add(1);
                    __t6
                }) as i32) as isize,
            ))
            .write((((crate::c::div_u32(feet, 10u32)).wrapping_add(161u32)) as u8));
            (((&raw mut buffer).cast::<u8>()).wrapping_offset(
                (({
                    let __t7 = i;
                    i = (i).wrapping_add(1);
                    __t7
                }) as i32) as isize,
            ))
            .write((((crate::c::rem_u32(feet, 10u32)).wrapping_add(161u32)) as u8));
        }
        (((&raw mut buffer).cast::<u8>()).wrapping_offset(
            (({
                let __t8 = i;
                i = (i).wrapping_add(1);
                __t8
            }) as i32) as isize,
        ))
        .write(180u8);
        (((&raw mut buffer).cast::<u8>()).wrapping_offset(
            (({
                let __t9 = i;
                i = (i).wrapping_add(1);
                __t9
            }) as i32) as isize,
        ))
        .write((((crate::c::div_u32(inches, 10u32)).wrapping_add(161u32)) as u8));
        (((&raw mut buffer).cast::<u8>()).wrapping_offset(
            (({
                let __t10 = i;
                i = (i).wrapping_add(1);
                __t10
            }) as i32) as isize,
        ))
        .write((((crate::c::rem_u32(inches, 10u32)).wrapping_add(161u32)) as u8));
        (((&raw mut buffer).cast::<u8>()).wrapping_offset(
            (({
                let __t11 = i;
                i = (i).wrapping_add(1);
                __t11
            }) as i32) as isize,
        ))
        .write(178u8);
        (((&raw mut buffer).cast::<u8>()).wrapping_offset(
            (({
                let __t12 = i;
                i = (i).wrapping_add(1);
                __t12
            }) as i32) as isize,
        ))
        .write(255u8);
        PrintInfoScreenText((&raw mut buffer).cast::<u8>(), left, top);
    }
}
pub(crate) unsafe extern "C" fn PrintMonWeight(weight: u16, left: u8, top: u8) {
    unsafe {
        let mut weight = weight;
        let mut left = left;
        let mut top = top;
        let mut buffer = crate::ffi::Align4([0u8; 16]);
        let mut output: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut lbs: u32 =
            ((crate::c::div_i32(((weight) as i32).wrapping_mul(100000i32), 4536i32)) as u32);
        if crate::c::rem_u32(lbs, 10u32) >= 5u32 {
            lbs = (lbs).wrapping_add(10u32);
        }
        i = 0u8;
        output = 0u8;
        if ((({
            let __v1 = (((crate::c::div_u32(lbs, 100000u32)).wrapping_add(161u32)) as u8);
            (((&raw mut buffer).cast::<u8>()).wrapping_offset(((i) as i32) as isize)).write(__v1);
            __v1
        }) as i32)
            == 161i32)
            && (!((output) != 0))
        {
            (((&raw mut buffer).cast::<u8>()).wrapping_offset(
                (({
                    let __t2 = i;
                    i = (i).wrapping_add(1);
                    __t2
                }) as i32) as isize,
            ))
            .write(119u8);
        } else {
            output = 1u8;
            i = (i).wrapping_add(1);
        }
        lbs = crate::c::rem_u32(lbs, 100000u32);
        if ((({
            let __v3 = (((crate::c::div_u32(lbs, 10000u32)).wrapping_add(161u32)) as u8);
            (((&raw mut buffer).cast::<u8>()).wrapping_offset(((i) as i32) as isize)).write(__v3);
            __v3
        }) as i32)
            == 161i32)
            && (!((output) != 0))
        {
            (((&raw mut buffer).cast::<u8>()).wrapping_offset(
                (({
                    let __t4 = i;
                    i = (i).wrapping_add(1);
                    __t4
                }) as i32) as isize,
            ))
            .write(119u8);
        } else {
            output = 1u8;
            i = (i).wrapping_add(1);
        }
        lbs = crate::c::rem_u32(lbs, 10000u32);
        if ((({
            let __v5 = (((crate::c::div_u32(lbs, 1000u32)).wrapping_add(161u32)) as u8);
            (((&raw mut buffer).cast::<u8>()).wrapping_offset(((i) as i32) as isize)).write(__v5);
            __v5
        }) as i32)
            == 161i32)
            && (!((output) != 0))
        {
            (((&raw mut buffer).cast::<u8>()).wrapping_offset(
                (({
                    let __t6 = i;
                    i = (i).wrapping_add(1);
                    __t6
                }) as i32) as isize,
            ))
            .write(119u8);
        } else {
            output = 1u8;
            i = (i).wrapping_add(1);
        }
        lbs = crate::c::rem_u32(lbs, 1000u32);
        (((&raw mut buffer).cast::<u8>()).wrapping_offset(
            (({
                let __t7 = i;
                i = (i).wrapping_add(1);
                __t7
            }) as i32) as isize,
        ))
        .write((((crate::c::div_u32(lbs, 100u32)).wrapping_add(161u32)) as u8));
        lbs = crate::c::rem_u32(lbs, 100u32);
        (((&raw mut buffer).cast::<u8>()).wrapping_offset(
            (({
                let __t8 = i;
                i = (i).wrapping_add(1);
                __t8
            }) as i32) as isize,
        ))
        .write(173u8);
        (((&raw mut buffer).cast::<u8>()).wrapping_offset(
            (({
                let __t9 = i;
                i = (i).wrapping_add(1);
                __t9
            }) as i32) as isize,
        ))
        .write((((crate::c::div_u32(lbs, 10u32)).wrapping_add(161u32)) as u8));
        (((&raw mut buffer).cast::<u8>()).wrapping_offset(
            (({
                let __t10 = i;
                i = (i).wrapping_add(1);
                __t10
            }) as i32) as isize,
        ))
        .write(0u8);
        (((&raw mut buffer).cast::<u8>()).wrapping_offset(
            (({
                let __t11 = i;
                i = (i).wrapping_add(1);
                __t11
            }) as i32) as isize,
        ))
        .write(224u8);
        (((&raw mut buffer).cast::<u8>()).wrapping_offset(
            (({
                let __t12 = i;
                i = (i).wrapping_add(1);
                __t12
            }) as i32) as isize,
        ))
        .write(214u8);
        (((&raw mut buffer).cast::<u8>()).wrapping_offset(
            (({
                let __t13 = i;
                i = (i).wrapping_add(1);
                __t13
            }) as i32) as isize,
        ))
        .write(231u8);
        (((&raw mut buffer).cast::<u8>()).wrapping_offset(
            (({
                let __t14 = i;
                i = (i).wrapping_add(1);
                __t14
            }) as i32) as isize,
        ))
        .write(173u8);
        (((&raw mut buffer).cast::<u8>()).wrapping_offset(
            (({
                let __t15 = i;
                i = (i).wrapping_add(1);
                __t15
            }) as i32) as isize,
        ))
        .write(255u8);
        PrintInfoScreenText((&raw mut buffer).cast::<u8>(), left, top);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokedexCategoryName(dexNum: u16) -> *mut u8 {
    unsafe {
        let mut dexNum = dexNum;
        return ((((&raw const gPokedexEntries).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((dexNum) as i32) as isize * 32))
        .cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokedexHeightWeight(dexNum: u16, data: u8) -> u16 {
    unsafe {
        let mut dexNum = dexNum;
        let mut data = data;
        'l1: {
            let __sw1 = ((data) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 {
                return (((((&raw const gPokedexEntries).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((dexNum) as i32) as isize * 32))
                .wrapping_add(12)
                .cast::<u16>())
                .read();
            }
            if __sw1 == 1i32 {
                return (((((&raw const gPokedexEntries).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((dexNum) as i32) as isize * 32))
                .wrapping_add(14)
                .cast::<u16>())
                .read();
            }
            if !__matched {
                return 1u16;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSetPokedexFlag(nationalDexNo: u16, caseID: u8) -> i8 {
    unsafe {
        let mut nationalDexNo = nationalDexNo;
        let mut caseID = caseID;
        let mut index: u8 = 0u8;
        let mut bit: u8 = 0u8;
        let mut mask: u8 = 0u8;
        let mut retVal: i8 = 0i8;
        nationalDexNo = (nationalDexNo).wrapping_sub(1);
        index = ((crate::c::div_i32(((nationalDexNo) as i32), 8i32)) as u8);
        bit = ((crate::c::rem_i32(((nationalDexNo) as i32), 8i32)) as u8);
        mask = ((crate::c::shl_i32(1i32, ((bit) as u32))) as u8);
        retVal = 0i8;
        'l1: {
            let __sw1 = ((caseID) as i32);
            if __sw1 == 0i32 {
                if ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(24))
                .wrapping_add(68))
                .cast::<u8>())
                .wrapping_offset(((index) as i32) as isize))
                .read()) as i32)
                    & ((mask) as i32))
                    != 0
                {
                    if (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(24))
                    .wrapping_add(68))
                    .cast::<u8>())
                    .wrapping_offset(((index) as i32) as isize))
                    .read()) as i32)
                        & ((mask) as i32))
                        == (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(2440))
                        .cast::<u8>())
                        .wrapping_offset(((index) as i32) as isize))
                        .read()) as i32)
                            & ((mask) as i32)))
                        && (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(24))
                        .wrapping_add(68))
                        .cast::<u8>())
                        .wrapping_offset(((index) as i32) as isize))
                        .read()) as i32)
                            & ((mask) as i32))
                            == (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(15140))
                            .cast::<u8>())
                            .wrapping_offset(((index) as i32) as isize))
                            .read()) as i32)
                                & ((mask) as i32)))
                    {
                        retVal = 1i8;
                    } else {
                        let __p2 = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(24))
                        .wrapping_add(68))
                        .cast::<u8>())
                        .wrapping_offset(((index) as i32) as isize);
                        (__p2).write((((((__p2).read()) as i32) & !((mask) as i32)) as u8));
                        let __p3 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(2440))
                        .cast::<u8>())
                        .wrapping_offset(((index) as i32) as isize);
                        (__p3).write((((((__p3).read()) as i32) & !((mask) as i32)) as u8));
                        let __p4 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(15140))
                        .cast::<u8>())
                        .wrapping_offset(((index) as i32) as isize);
                        (__p4).write((((((__p4).read()) as i32) & !((mask) as i32)) as u8));
                        retVal = 0i8;
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(24))
                .wrapping_add(16))
                .cast::<u8>())
                .wrapping_offset(((index) as i32) as isize))
                .read()) as i32)
                    & ((mask) as i32))
                    != 0
                {
                    if ((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(24))
                    .wrapping_add(16))
                    .cast::<u8>())
                    .wrapping_offset(((index) as i32) as isize))
                    .read()) as i32)
                        & ((mask) as i32))
                        == ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(24))
                        .wrapping_add(68))
                        .cast::<u8>())
                        .wrapping_offset(((index) as i32) as isize))
                        .read()) as i32)
                            & ((mask) as i32)))
                        && (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(24))
                        .wrapping_add(16))
                        .cast::<u8>())
                        .wrapping_offset(((index) as i32) as isize))
                        .read()) as i32)
                            & ((mask) as i32))
                            == (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(2440))
                            .cast::<u8>())
                            .wrapping_offset(((index) as i32) as isize))
                            .read()) as i32)
                                & ((mask) as i32))))
                        && (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(24))
                        .wrapping_add(16))
                        .cast::<u8>())
                        .wrapping_offset(((index) as i32) as isize))
                        .read()) as i32)
                            & ((mask) as i32))
                            == (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(15140))
                            .cast::<u8>())
                            .wrapping_offset(((index) as i32) as isize))
                            .read()) as i32)
                                & ((mask) as i32)))
                    {
                        retVal = 1i8;
                    } else {
                        let __p5 = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(24))
                        .wrapping_add(16))
                        .cast::<u8>())
                        .wrapping_offset(((index) as i32) as isize);
                        (__p5).write((((((__p5).read()) as i32) & !((mask) as i32)) as u8));
                        let __p6 = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(24))
                        .wrapping_add(68))
                        .cast::<u8>())
                        .wrapping_offset(((index) as i32) as isize);
                        (__p6).write((((((__p6).read()) as i32) & !((mask) as i32)) as u8));
                        let __p7 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(2440))
                        .cast::<u8>())
                        .wrapping_offset(((index) as i32) as isize);
                        (__p7).write((((((__p7).read()) as i32) & !((mask) as i32)) as u8));
                        let __p8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(15140))
                        .cast::<u8>())
                        .wrapping_offset(((index) as i32) as isize);
                        (__p8).write((((((__p8).read()) as i32) & !((mask) as i32)) as u8));
                        retVal = 0i8;
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p9 = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(24))
                .wrapping_add(68))
                .cast::<u8>())
                .wrapping_offset(((index) as i32) as isize);
                (__p9).write((((((__p9).read()) as i32) | ((mask) as i32)) as u8));
                let __p10 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2440))
                .cast::<u8>())
                .wrapping_offset(((index) as i32) as isize);
                (__p10).write((((((__p10).read()) as i32) | ((mask) as i32)) as u8));
                let __p11 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(15140))
                .cast::<u8>())
                .wrapping_offset(((index) as i32) as isize);
                (__p11).write((((((__p11).read()) as i32) | ((mask) as i32)) as u8));
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p12 = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(24))
                .wrapping_add(16))
                .cast::<u8>())
                .wrapping_offset(((index) as i32) as isize);
                (__p12).write((((((__p12).read()) as i32) | ((mask) as i32)) as u8));
                break 'l1;
            }
        }
        return retVal;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNationalPokedexCount(caseID: u8) -> u16 {
    unsafe {
        let mut caseID = caseID;
        let mut count: u16 = 0u16;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 386i32) {
                    break 'l1;
                }
                'l2: {
                    'l3: {
                        let __sw1 = ((caseID) as i32);
                        if __sw1 == 0i32 {
                            if (GetSetPokedexFlag(((((i) as i32).wrapping_add(1i32)) as u16), 0u8))
                                != 0
                            {
                                count = (count).wrapping_add(1);
                            }
                            break 'l3;
                        }
                        if __sw1 == 1i32 {
                            if (GetSetPokedexFlag(((((i) as i32).wrapping_add(1i32)) as u16), 1u8))
                                != 0
                            {
                                count = (count).wrapping_add(1);
                            }
                            break 'l3;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return count;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetHoennPokedexCount(caseID: u8) -> u16 {
    unsafe {
        let mut caseID = caseID;
        let mut count: u16 = 0u16;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 202i32) {
                    break 'l1;
                }
                'l2: {
                    'l3: {
                        let __sw1 = ((caseID) as i32);
                        if __sw1 == 0i32 {
                            if (GetSetPokedexFlag(
                                HoennToNationalOrder(((((i) as i32).wrapping_add(1i32)) as u16)),
                                0u8,
                            )) != 0
                            {
                                count = (count).wrapping_add(1);
                            }
                            break 'l3;
                        }
                        if __sw1 == 1i32 {
                            if (GetSetPokedexFlag(
                                HoennToNationalOrder(((((i) as i32).wrapping_add(1i32)) as u16)),
                                1u8,
                            )) != 0
                            {
                                count = (count).wrapping_add(1);
                            }
                            break 'l3;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return count;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetKantoPokedexCount(caseID: u8) -> u16 {
    unsafe {
        let mut caseID = caseID;
        let mut count: u16 = 0u16;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 151i32) {
                    break 'l1;
                }
                'l2: {
                    'l3: {
                        let __sw1 = ((caseID) as i32);
                        if __sw1 == 0i32 {
                            if (GetSetPokedexFlag(((((i) as i32).wrapping_add(1i32)) as u16), 0u8))
                                != 0
                            {
                                count = (count).wrapping_add(1);
                            }
                            break 'l3;
                        }
                        if __sw1 == 1i32 {
                            if (GetSetPokedexFlag(((((i) as i32).wrapping_add(1i32)) as u16), 1u8))
                                != 0
                            {
                                count = (count).wrapping_add(1);
                            }
                            break 'l3;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return count;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasAllHoennMons() -> u16 {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 200i32) {
                    break 'l1;
                }
                'l2: {
                    if !((GetSetPokedexFlag(
                        HoennToNationalOrder(((((i) as i32).wrapping_add(1i32)) as u16)),
                        1u8,
                    )) != 0)
                    {
                        return 0u16;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasAllKantoMons() -> u8 {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 150i32) {
                    break 'l1;
                }
                'l2: {
                    if !((GetSetPokedexFlag(((((i) as i32).wrapping_add(1i32)) as u16), 1u8)) != 0)
                    {
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasAllMons() -> u16 {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 150i32) {
                    break 'l1;
                }
                'l2: {
                    if !((GetSetPokedexFlag(((((i) as i32).wrapping_add(1i32)) as u16), 1u8)) != 0)
                    {
                        return 0u16;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 151u16;
            'l3: loop {
                if !(((i) as i32) < 248i32) {
                    break 'l3;
                }
                'l4: {
                    if !((GetSetPokedexFlag(((((i) as i32).wrapping_add(1i32)) as u16), 1u8)) != 0)
                    {
                        return 0u16;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 251u16;
            'l5: loop {
                if !(((i) as i32) < 384i32) {
                    break 'l5;
                }
                'l6: {
                    if !((GetSetPokedexFlag(((((i) as i32).wrapping_add(1i32)) as u16), 1u8)) != 0)
                    {
                        return 0u16;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u16;
    }
}
pub(crate) unsafe extern "C" fn ResetOtherVideoRegisters(regBits: u16) {
    unsafe {
        let mut regBits = regBits;
        if !((((regBits) as i32) & 256i32) != 0) {
            ClearGpuRegBits(0u8, 256u16);
            SetGpuReg(8u8, 0u16);
            SetGpuReg(16u8, 0u16);
            SetGpuReg(18u8, 0u16);
        }
        if !((((regBits) as i32) & 512i32) != 0) {
            ClearGpuRegBits(0u8, 512u16);
            SetGpuReg(10u8, 0u16);
            SetGpuReg(20u8, 0u16);
            SetGpuReg(22u8, 0u16);
        }
        if !((((regBits) as i32) & 1024i32) != 0) {
            ClearGpuRegBits(0u8, 1024u16);
            SetGpuReg(12u8, 0u16);
            SetGpuReg(24u8, 0u16);
            SetGpuReg(26u8, 0u16);
        }
        if !((((regBits) as i32) & 2048i32) != 0) {
            ClearGpuRegBits(0u8, 2048u16);
            SetGpuReg(14u8, 0u16);
            SetGpuReg(28u8, 0u16);
            SetGpuReg(30u8, 0u16);
        }
        if !((((regBits) as i32) & 4096i32) != 0) {
            ClearGpuRegBits(0u8, 4096u16);
            ResetSpriteData();
            FreeAllSpritePalettes();
            ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(8u8);
        }
    }
}
pub(crate) unsafe extern "C" fn PrintInfoSubMenuText(
    windowId: u8,
    str: *mut u8,
    left: u8,
    top: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut str = str;
        let mut left = left;
        let mut top = top;
        let mut color = crate::ffi::Align4([0u8; 3]);
        ((&raw mut color).cast::<u8>()).write(0u8);
        (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(15u8);
        (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(3u8);
        AddTextPrinterParameterized4(
            windowId,
            1u8,
            left,
            top,
            0u8,
            0u8,
            (&raw mut color).cast::<u8>(),
            (-1i8),
            str,
        );
    }
}
pub(crate) unsafe extern "C" fn UnusedPrintNum(windowId: u8, num: u16, left: u8, top: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut num = num;
        let mut left = left;
        let mut top = top;
        let mut str = crate::ffi::Align4([0u8; 4]);
        ((&raw mut str).cast::<u8>())
            .write((((161i32).wrapping_add(crate::c::div_i32(((num) as i32), 100i32))) as u8));
        (((&raw mut str).cast::<u8>()).wrapping_offset(1)).write(
            (((161i32).wrapping_add(crate::c::div_i32(
                crate::c::rem_i32(((num) as i32), 100i32),
                10i32,
            ))) as u8),
        );
        (((&raw mut str).cast::<u8>()).wrapping_offset(2)).write(
            (((161i32).wrapping_add(crate::c::rem_i32(
                crate::c::rem_i32(((num) as i32), 100i32),
                10i32,
            ))) as u8),
        );
        (((&raw mut str).cast::<u8>()).wrapping_offset(3)).write(255u8);
        PrintInfoSubMenuText(windowId, (&raw mut str).cast::<u8>(), left, top);
    }
}
pub(crate) unsafe extern "C" fn PrintCryScreenSpeciesName(
    windowId: u8,
    num: u16,
    left: u8,
    top: u8,
) -> u8 {
    unsafe {
        let mut windowId = windowId;
        let mut num = num;
        let mut left = left;
        let mut top = top;
        let mut str = crate::ffi::Align4([0u8; 11]);
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(11u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut str).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(255u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        num = NationalPokedexNumToSpecies(num);
        'l3: {
            let __sw1 = ((num) as i32);
            let __matched = __sw1 == 0i32;
            if !__matched {
                {
                    i = 0u8;
                    'l4: loop {
                        if !(((((((((&raw mut gSpeciesNames).cast::<u8>())
                            .wrapping_offset(((num) as i32) as isize * 11))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            != 255i32)
                            && (((i) as i32) < 10i32))
                        {
                            break 'l4;
                        }
                        'l5: {
                            (((&raw mut str).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                                .write(
                                    (((((&raw mut gSpeciesNames).cast::<u8>())
                                        .wrapping_offset(((num) as i32) as isize * 11))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read(),
                                );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l3;
            }
            if __sw1 == 0i32 {
                {
                    i = 0u8;
                    'l6: loop {
                        if !(((i) as i32) < 5i32) {
                            break 'l6;
                        }
                        'l7: {
                            (((&raw mut str).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                                .write(174u8);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l3;
            }
        }
        PrintInfoSubMenuText(windowId, (&raw mut str).cast::<u8>(), left, top);
        return i;
    }
}
pub(crate) unsafe extern "C" fn UnusedPrintMonName(windowId: u8, name: *mut u8, left: u8, top: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut name = name;
        let mut left = left;
        let mut top = top;
        let mut str = crate::ffi::Align4([0u8; 11]);
        let mut i: u8 = 0u8;
        let mut nameLength: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(11u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut str).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            nameLength = 0u8;
            'l3: loop {
                if !((((((name).wrapping_offset(((nameLength) as i32) as isize)).read()) as i32)
                    != 0i32)
                    && (((nameLength) as u32) < crate::c::div_u32(11u32, 1u32)))
                {
                    break 'l3;
                }
                'l4: {}
                nameLength = (nameLength).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32) < ((nameLength) as i32)) {
                    break 'l5;
                }
                'l6: {
                    (((&raw mut str).cast::<u8>()).wrapping_offset(
                        ((((crate::c::div_u32(11u32, 1u32)).wrapping_sub(((nameLength) as u32)))
                            .wrapping_add(((i) as u32))) as i32) as isize,
                    ))
                    .write(((name).wrapping_offset(((i) as i32) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut str).cast::<u8>()).wrapping_offset(
            (((crate::c::div_u32(11u32, 1u32)).wrapping_sub(1u32)) as i32) as isize,
        ))
        .write(255u8);
        PrintInfoSubMenuText(windowId, (&raw mut str).cast::<u8>(), left, top);
    }
}
pub(crate) unsafe extern "C" fn PrintDecimalNum(windowId: u8, num: u16, left: u8, top: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut num = num;
        let mut left = left;
        let mut top = top;
        let mut str = crate::ffi::Align4([0u8; 6]);
        let mut outputted: u8 = 0u8;
        let mut result: u8 = 0u8;
        result = ((crate::c::div_i32(((num) as i32), 1000i32)) as u8);
        if ((result) as i32) == 0i32 {
            ((&raw mut str).cast::<u8>()).write(119u8);
            outputted = 0u8;
        } else {
            ((&raw mut str).cast::<u8>()).write((((161i32).wrapping_add(((result) as i32))) as u8));
            outputted = 1u8;
        }
        result = ((crate::c::div_i32(crate::c::rem_i32(((num) as i32), 1000i32), 100i32)) as u8);
        if (((result) as i32) == 0i32) && (!((outputted) != 0)) {
            (((&raw mut str).cast::<u8>()).wrapping_offset(1)).write(119u8);
            outputted = 0u8;
        } else {
            (((&raw mut str).cast::<u8>()).wrapping_offset(1))
                .write((((161i32).wrapping_add(((result) as i32))) as u8));
            outputted = 1u8;
        }
        (((&raw mut str).cast::<u8>()).wrapping_offset(2)).write(
            (((161i32).wrapping_add(crate::c::div_i32(
                crate::c::rem_i32(crate::c::rem_i32(((num) as i32), 1000i32), 100i32),
                10i32,
            ))) as u8),
        );
        (((&raw mut str).cast::<u8>()).wrapping_offset(3)).write(173u8);
        (((&raw mut str).cast::<u8>()).wrapping_offset(4)).write(
            (((161i32).wrapping_add(crate::c::rem_i32(
                crate::c::rem_i32(crate::c::rem_i32(((num) as i32), 1000i32), 100i32),
                10i32,
            ))) as u8),
        );
        (((&raw mut str).cast::<u8>()).wrapping_offset(5)).write(255u8);
        PrintInfoSubMenuText(windowId, (&raw mut str).cast::<u8>(), left, top);
    }
}
pub(crate) unsafe extern "C" fn DrawFootprint(windowId: u8, dexNum: u16) {
    unsafe {
        let mut windowId = windowId;
        let mut dexNum = dexNum;
        let mut footprint4bpp = crate::ffi::Align4([0u8; 128]);
        let mut footprintGfx: *mut u8 = ((((&raw const gMonFootprintTable)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(((NationalPokedexNumToSpecies(dexNum)) as i32) as isize))
        .read();
        let mut tileIdx: u16 = 0u16;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < (crate::c::div_i32(64i32, 8i32)).wrapping_mul(4i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut footprint1bpp: u8 =
                        ((footprintGfx).wrapping_offset(((i) as i32) as isize)).read();
                    {
                        j = 0u16;
                        'l3: loop {
                            if !(((j) as i32) < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                let mut tile: u8 = 0u8;
                                if (((footprint1bpp) as i32)
                                    & crate::c::shl_i32(
                                        1i32,
                                        (((2i32).wrapping_mul(((j) as i32))) as u32),
                                    ))
                                    != 0
                                {
                                    tile = ((((tile) as i32) | 2i32) as u8);
                                }
                                if (((footprint1bpp) as i32)
                                    & crate::c::shl_i32(
                                        2i32,
                                        (((2i32).wrapping_mul(((j) as i32))) as u32),
                                    ))
                                    != 0
                                {
                                    tile = ((((tile) as i32) | 32i32) as u8);
                                }
                                (((&raw mut footprint4bpp).cast::<u8>())
                                    .wrapping_offset(((tileIdx) as i32) as isize))
                                .write(tile);
                                tileIdx = (tileIdx).wrapping_add(1);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyToWindowPixelBuffer(
            windowId,
            (&raw mut footprint4bpp).cast::<u8>(),
            128u16,
            0u16,
        );
    }
}
pub(crate) unsafe extern "C" fn RS_DrawFootprint(offset: u16, tileNum: u16) {
    unsafe {
        let mut offset = offset;
        let mut tileNum = tileNum;
        ((((100663296i32).wrapping_add(((offset) as i32).wrapping_mul(2048i32)))
            .wrapping_add(562i32)) as usize as *mut u16)
            .write(((((61440i32).wrapping_add(((tileNum) as i32))).wrapping_add(0i32)) as u16));
        ((((100663296i32).wrapping_add(((offset) as i32).wrapping_mul(2048i32)))
            .wrapping_add(564i32)) as usize as *mut u16)
            .write(((((61440i32).wrapping_add(((tileNum) as i32))).wrapping_add(1i32)) as u16));
        ((((100663296i32).wrapping_add(((offset) as i32).wrapping_mul(2048i32)))
            .wrapping_add(626i32)) as usize as *mut u16)
            .write(((((61440i32).wrapping_add(((tileNum) as i32))).wrapping_add(2i32)) as u16));
        ((((100663296i32).wrapping_add(((offset) as i32).wrapping_mul(2048i32)))
            .wrapping_add(628i32)) as usize as *mut u16)
            .write(((((61440i32).wrapping_add(((tileNum) as i32))).wrapping_add(3i32)) as u16));
    }
}
pub(crate) unsafe extern "C" fn GetNextPosition(
    direction: u8,
    position: u16,
    min: u16,
    max: u16,
) -> u16 {
    unsafe {
        let mut direction = direction;
        let mut position = position;
        let mut min = min;
        let mut max = max;
        'l1: {
            let __sw1 = ((direction) as i32);
            if __sw1 == 1i32 {
                if ((position) as i32) > ((min) as i32) {
                    position = (position).wrapping_sub(1);
                }
                break 'l1;
            }
            if __sw1 == 0i32 {
                if ((position) as i32) < ((max) as i32) {
                    position = (position).wrapping_add(1);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((position) as i32) > ((min) as i32) {
                    position = (position).wrapping_sub(1);
                } else {
                    position = max;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((position) as i32) < ((max) as i32) {
                    position = (position).wrapping_add(1);
                } else {
                    position = min;
                }
                break 'l1;
            }
        }
        return position;
    }
}
pub(crate) unsafe extern "C" fn GetPokedexMonPersonality(species: u16) -> u32 {
    unsafe {
        let mut species = species;
        if (((species) as i32) == 201i32) || (((species) as i32) == 308i32) {
            if ((species) as i32) == 201i32 {
                return (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24))
                    .wrapping_add(4)
                    .cast::<u32>())
                .read();
            } else {
                return (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24))
                    .wrapping_add(8)
                    .cast::<u32>())
                .read();
            }
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonSpriteFromNationalDexNumber(
    nationalNum: u16,
    x: i16,
    y: i16,
    paletteSlot: u16,
) -> u16 {
    unsafe {
        let mut nationalNum = nationalNum;
        let mut x = x;
        let mut y = y;
        let mut paletteSlot = paletteSlot;
        nationalNum = NationalPokedexNumToSpecies(nationalNum);
        return CreateMonPicSprite_HandleDeoxys(
            nationalNum,
            8u32,
            GetPokedexMonPersonality(nationalNum),
            1u8,
            x,
            y,
            ((paletteSlot) as u8),
            65535u16,
        );
    }
}
pub(crate) unsafe extern "C" fn CreateSizeScreenTrainerPic(
    species: u16,
    x: i16,
    y: i16,
    paletteSlot: i8,
) -> u16 {
    unsafe {
        let mut species = species;
        let mut x = x;
        let mut y = y;
        let mut paletteSlot = paletteSlot;
        return CreateTrainerPicSprite(species, 1u8, x, y, ((paletteSlot) as u8), 65535u16);
    }
}
pub(crate) unsafe extern "C" fn DoPokedexSearch(
    dexMode: u8,
    order: u8,
    abcGroup: u8,
    bodyColor: u8,
    type1: u8,
    type2: u8,
) -> i32 {
    unsafe {
        let mut dexMode = dexMode;
        let mut order = order;
        let mut abcGroup = abcGroup;
        let mut bodyColor = bodyColor;
        let mut type1 = type1;
        let mut type2 = type2;
        let mut species: u16 = 0u16;
        let mut i: u16 = 0u16;
        let mut resultsCount: u16 = 0u16;
        let mut types = crate::ffi::Align4([0u8; 2]);
        CreatePokedexList(dexMode, order);
        {
            i = 0u16;
            resultsCount = 0u16;
            'l1: loop {
                if !(((i) as i32) < 386i32) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::bf_read(
                        (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(2),
                        0,
                        1,
                        false,
                    ) as u16)
                        != 0
                    {
                        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((resultsCount) as i32) as isize * 4)
                        .cast::<crate::c::Rec4<4>>()
                        .write_unaligned(
                            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4)
                            .cast::<crate::c::Rec4<4>>()
                            .read_unaligned(),
                        );
                        resultsCount = (resultsCount).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1548)
            .cast::<u16>())
        .write(resultsCount);
        if ((abcGroup) as i32) != 255i32 {
            {
                i = 0u16;
                resultsCount = 0u16;
                'l3: loop {
                    if !(((i) as i32)
                        < ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1548)
                            .cast::<u16>())
                        .read()) as i32))
                    {
                        break 'l3;
                    }
                    'l4: {
                        let mut firstLetter: u8 = 0u8;
                        species = NationalPokedexNumToSpecies(
                            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .cast::<u16>())
                            .read(),
                        );
                        firstLetter = ((((&raw mut gSpeciesNames).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 11))
                        .cast::<u8>())
                        .read();
                        if ((((firstLetter) as i32)
                            >= (((((((&raw const sLetterSearchRanges).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((abcGroup) as i32) as isize * 4))
                            .cast::<u8>())
                            .read()) as i32))
                            && (((firstLetter) as i32)
                                < (((((((&raw const sLetterSearchRanges)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((abcGroup) as i32) as isize * 4))
                                .cast::<u8>())
                                .read()) as i32)
                                    .wrapping_add(
                                        ((((((((&raw const sLetterSearchRanges)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((abcGroup) as i32) as isize * 4))
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .read()) as i32),
                                    )))
                            || ((((firstLetter) as i32)
                                >= ((((((((&raw const sLetterSearchRanges)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((abcGroup) as i32) as isize * 4))
                                .cast::<u8>())
                                .wrapping_offset(2))
                                .read()) as i32))
                                && (((firstLetter) as i32)
                                    < ((((((((&raw const sLetterSearchRanges)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((abcGroup) as i32) as isize * 4))
                                    .cast::<u8>())
                                    .wrapping_offset(2))
                                    .read()) as i32)
                                        .wrapping_add(
                                            ((((((((&raw const sLetterSearchRanges)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(((abcGroup) as i32) as isize * 4))
                                            .cast::<u8>())
                                            .wrapping_offset(3))
                                            .read())
                                                as i32),
                                        )))
                        {
                            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((resultsCount) as i32) as isize * 4)
                            .cast::<crate::c::Rec4<4>>()
                            .write_unaligned(
                                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4)
                                .cast::<crate::c::Rec4<4>>()
                                .read_unaligned(),
                            );
                            resultsCount = (resultsCount).wrapping_add(1);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1548)
                .cast::<u16>())
            .write(resultsCount);
        }
        if ((bodyColor) as i32) != 255i32 {
            {
                i = 0u16;
                resultsCount = 0u16;
                'l5: loop {
                    if !(((i) as i32)
                        < ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1548)
                            .cast::<u16>())
                        .read()) as i32))
                    {
                        break 'l5;
                    }
                    'l6: {
                        species = NationalPokedexNumToSpecies(
                            ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .cast::<u16>())
                            .read(),
                        );
                        if ((bodyColor) as i32)
                            == ((crate::c::bf_read(
                                (((&raw mut gSpeciesInfo).cast::<u8>())
                                    .wrapping_offset(((species) as i32) as isize * 28))
                                .wrapping_add(25),
                                0,
                                7,
                                false,
                            ) as u8) as i32)
                        {
                            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((resultsCount) as i32) as isize * 4)
                            .cast::<crate::c::Rec4<4>>()
                            .write_unaligned(
                                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4)
                                .cast::<crate::c::Rec4<4>>()
                                .read_unaligned(),
                            );
                            resultsCount = (resultsCount).wrapping_add(1);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1548)
                .cast::<u16>())
            .write(resultsCount);
        }
        if (((type1) as i32) != 255i32) || (((type2) as i32) != 255i32) {
            if ((type1) as i32) == 255i32 {
                type1 = type2;
                type2 = 255u8;
            }
            if ((type2) as i32) == 255i32 {
                {
                    i = 0u16;
                    resultsCount = 0u16;
                    'l7: loop {
                        if !(((i) as i32)
                            < ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1548)
                                .cast::<u16>())
                            .read()) as i32))
                        {
                            break 'l7;
                        }
                        'l8: {
                            if (crate::c::bf_read(
                                (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .wrapping_add(2),
                                1,
                                1,
                                false,
                            ) as u16)
                                != 0
                            {
                                species = NationalPokedexNumToSpecies(
                                    ((((((&raw mut sPokedexView)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                    .cast::<u16>())
                                    .read(),
                                );
                                ((&raw mut types).cast::<u8>()).write(
                                    (((((&raw mut gSpeciesInfo).cast::<u8>())
                                        .wrapping_offset(((species) as i32) as isize * 28))
                                    .wrapping_add(6))
                                    .cast::<u8>())
                                    .read(),
                                );
                                (((&raw mut types).cast::<u8>()).wrapping_offset(1)).write(
                                    ((((((&raw mut gSpeciesInfo).cast::<u8>())
                                        .wrapping_offset(((species) as i32) as isize * 28))
                                    .wrapping_add(6))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read(),
                                );
                                if (((((&raw mut types).cast::<u8>()).read()) as i32)
                                    == ((type1) as i32))
                                    || ((((((&raw mut types).cast::<u8>()).wrapping_offset(1))
                                        .read()) as i32)
                                        == ((type1) as i32))
                                {
                                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(((resultsCount) as i32) as isize * 4)
                                    .cast::<crate::c::Rec4<4>>()
                                    .write_unaligned(
                                        ((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 4)
                                        .cast::<crate::c::Rec4<4>>()
                                        .read_unaligned(),
                                    );
                                    resultsCount = (resultsCount).wrapping_add(1);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            } else {
                {
                    i = 0u16;
                    resultsCount = 0u16;
                    'l9: loop {
                        if !(((i) as i32)
                            < ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1548)
                                .cast::<u16>())
                            .read()) as i32))
                        {
                            break 'l9;
                        }
                        'l10: {
                            if (crate::c::bf_read(
                                (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .wrapping_add(2),
                                1,
                                1,
                                false,
                            ) as u16)
                                != 0
                            {
                                species = NationalPokedexNumToSpecies(
                                    ((((((&raw mut sPokedexView)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                    .cast::<u16>())
                                    .read(),
                                );
                                ((&raw mut types).cast::<u8>()).write(
                                    (((((&raw mut gSpeciesInfo).cast::<u8>())
                                        .wrapping_offset(((species) as i32) as isize * 28))
                                    .wrapping_add(6))
                                    .cast::<u8>())
                                    .read(),
                                );
                                (((&raw mut types).cast::<u8>()).wrapping_offset(1)).write(
                                    ((((((&raw mut gSpeciesInfo).cast::<u8>())
                                        .wrapping_offset(((species) as i32) as isize * 28))
                                    .wrapping_add(6))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read(),
                                );
                                if ((((((&raw mut types).cast::<u8>()).read()) as i32)
                                    == ((type1) as i32))
                                    && ((((((&raw mut types).cast::<u8>()).wrapping_offset(1))
                                        .read()) as i32)
                                        == ((type2) as i32)))
                                    || ((((((&raw mut types).cast::<u8>()).read()) as i32)
                                        == ((type2) as i32))
                                        && ((((((&raw mut types).cast::<u8>()).wrapping_offset(1))
                                            .read())
                                            as i32)
                                            == ((type1) as i32)))
                                {
                                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(((resultsCount) as i32) as isize * 4)
                                    .cast::<crate::c::Rec4<4>>()
                                    .write_unaligned(
                                        ((((&raw mut sPokedexView)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 4)
                                        .cast::<crate::c::Rec4<4>>()
                                        .read_unaligned(),
                                    );
                                    resultsCount = (resultsCount).wrapping_add(1);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1548)
                .cast::<u16>())
            .write(resultsCount);
        }
        if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1548)
            .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            {
                i = ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1548)
                    .cast::<u16>())
                .read();
                'l11: loop {
                    if !(((i) as i32) < 386i32) {
                        break 'l11;
                    }
                    'l12: {
                        ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                        .write(65535u16);
                        crate::c::bf_write(
                            (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2),
                            0,
                            1,
                            (0u16) as i32,
                        );
                        crate::c::bf_write(
                            (((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2),
                            1,
                            1,
                            (0u16) as i32,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        return ((resultsCount) as i32);
    }
}
pub(crate) unsafe extern "C" fn LoadSearchMenu() -> u8 {
    unsafe {
        return CreateTask(Some(Task_LoadSearchMenu), 0u8);
    }
}
pub(crate) unsafe extern "C" fn PrintSearchText(str: *mut u8, x: u32, y: u32) {
    unsafe {
        let mut str = str;
        let mut x = x;
        let mut y = y;
        let mut color = crate::ffi::Align4([0u8; 3]);
        ((&raw mut color).cast::<u8>()).write(0u8);
        (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(15u8);
        (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(2u8);
        AddTextPrinterParameterized4(
            0u8,
            1u8,
            ((x) as u8),
            ((y) as u8),
            0u8,
            0u8,
            (&raw mut color).cast::<u8>(),
            (-1i8),
            str,
        );
    }
}
pub(crate) unsafe extern "C" fn ClearSearchMenuRect(x: u32, y: u32, width: u32, height: u32) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut width = width;
        let mut height = height;
        FillWindowPixelRect(
            0u8,
            0u8,
            ((x) as u16),
            ((y) as u16),
            ((width) as u16),
            ((height) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_LoadSearchMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 || !__matched {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1610))
                    .write(2u8);
                    ResetOtherVideoRegisters(0u16);
                    ResetBgsAndClearDma3BusyFlags(0u32);
                    InitBgsFromTemplates(
                        0u8,
                        ((&raw const sSearchMenu_BgTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
                        ((crate::c::div_u32(16u32, 4u32)) as u8),
                    );
                    SetBgTilemapBuffer(3u8, AllocZeroed(2048u32));
                    SetBgTilemapBuffer(2u8, AllocZeroed(2048u32));
                    SetBgTilemapBuffer(1u8, AllocZeroed(2048u32));
                    SetBgTilemapBuffer(0u8, AllocZeroed(2048u32));
                    InitWindows(
                        ((&raw const sSearchMenu_WindowTemplate)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    );
                    DeactivateAllTextPrinters();
                    PutWindowTilemap(0u8);
                    DecompressAndLoadBgGfxUsingHeap(
                        3u8,
                        (((&raw mut gPokedexSearchMenu_Gfx).cast::<u32>()).cast::<u32>())
                            .cast::<u8>(),
                        8192u32,
                        0u16,
                        0u8,
                    );
                    if !((IsNationalPokedexEnabled()) != 0) {
                        CopyToBgTilemapBuffer(
                            3u8,
                            (((&raw mut gPokedexSearchMenuHoenn_Tilemap).cast::<u32>())
                                .cast::<u32>())
                            .cast::<u8>(),
                            0u16,
                            0u16,
                        );
                    } else {
                        CopyToBgTilemapBuffer(
                            3u8,
                            (((&raw mut gPokedexSearchMenuNational_Tilemap).cast::<u32>())
                                .cast::<u32>())
                            .cast::<u8>(),
                            0u16,
                            0u16,
                        );
                    }
                    LoadPalette(
                        ((((&raw mut gPokedexSearchMenu_Pal).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(1))
                        .cast::<u8>(),
                        1u16,
                        126u16,
                    );
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                LoadCompressedSpriteSheet(
                    ((&raw const sInterfaceSpriteSheet).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                LoadSpritePalettes(
                    ((&raw const sInterfaceSpritePalette).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                CreateSearchParameterScrollArrows(taskId);
                {
                    i = 0u16;
                    'l2: loop {
                        if !(((i) as i32) < 16i32) {
                            break 'l2;
                        }
                        'l3: {
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(0i16);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                SetDefaultSearchModeAndOrder(taskId);
                HighlightSelectedSearchTopBarItem(0u8);
                PrintSelectedSearchParameters(taskId);
                CopyWindowToVram(0u8, 3u8);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(2u8);
                CopyBgTilemapBufferToVram(3u8);
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetGpuReg(84u8, 0u16);
                SetGpuReg(0u8, 4160u16);
                HideBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                ShowBg(3u8);
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_SwitchToSearchMenuTopBar));
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(0u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FreeSearchWindowAndBgBuffers() {
    unsafe {
        let mut tilemapBuffer: *mut u8 = core::ptr::null_mut();
        FreeAllWindowBuffers();
        tilemapBuffer = GetBgTilemapBuffer(0u8);
        if !(tilemapBuffer).is_null() {
            Free(tilemapBuffer);
        }
        tilemapBuffer = GetBgTilemapBuffer(1u8);
        if !(tilemapBuffer).is_null() {
            Free(tilemapBuffer);
        }
        tilemapBuffer = GetBgTilemapBuffer(2u8);
        if !(tilemapBuffer).is_null() {
            Free(tilemapBuffer);
        }
        tilemapBuffer = GetBgTilemapBuffer(3u8);
        if !(tilemapBuffer).is_null() {
            Free(tilemapBuffer);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchToSearchMenuTopBar(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        HighlightSelectedSearchTopBarItem(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as u8),
        );
        PrintSelectedSearchParameters(taskId);
        CopyWindowToVram(0u8, 2u8);
        CopyBgTilemapBufferToVram(3u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleSearchTopBarInput));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSearchTopBarInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            PlaySE(3u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ExitSearch));
            return;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            'l1: {
                let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32);
                if __sw1 == 0i32 {
                    PlaySE(21u16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_SwitchToSearchMenu));
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    PlaySE(21u16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(4i16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_SwitchToSearchMenu));
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    PlaySE(3u16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_ExitSearch));
                    break 'l1;
                }
            }
            return;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0)
            && ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                > 0i32)
        {
            PlaySE(109u16);
            let __p2 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_sub(1));
            HighlightSelectedSearchTopBarItem(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u8),
            );
            CopyWindowToVram(0u8, 2u8);
            CopyBgTilemapBufferToVram(3u8);
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0)
            && ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                < 2i32)
        {
            PlaySE(109u16);
            let __p3 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            HighlightSelectedSearchTopBarItem(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u8),
            );
            CopyWindowToVram(0u8, 2u8);
            CopyBgTilemapBufferToVram(3u8);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchToSearchMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        HighlightSelectedSearchMenuItem(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as u8),
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as u8),
        );
        PrintSelectedSearchParameters(taskId);
        CopyWindowToVram(0u8, 2u8);
        CopyBgTilemapBufferToVram(3u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleSearchMenuInput));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSearchMenuInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut movementMap: *mut u8 = core::ptr::null_mut();
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            != 0i32
        {
            if !((IsNationalPokedexEnabled()) != 0) {
                movementMap = ((&raw const sSearchMovementMap_ShiftHoennDex)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>();
            } else {
                movementMap = ((&raw const sSearchMovementMap_ShiftNatDex)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>();
            }
        } else {
            if !((IsNationalPokedexEnabled()) != 0) {
                movementMap = ((&raw const sSearchMovementMap_SearchHoennDex)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>();
            } else {
                movementMap = ((&raw const sSearchMovementMap_SearchNatDex)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>();
            }
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            PlaySE(23u16);
            SetDefaultSearchModeAndOrder(taskId);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_SwitchToSearchMenuTopBar));
            return;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                == 6i32
            {
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32)
                    != 0i32
                {
                    ((&raw mut sPokeBallRotation).cast::<u8>().cast::<u8>()).write(64u8);
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1578)
                        .cast::<u16>())
                    .write(64u16);
                    ((&raw mut sLastSelectedPokemon).cast::<u8>().cast::<u16>()).write(0u16);
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1552)
                        .cast::<u16>())
                    .write(0u16);
                    (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24))
                        .wrapping_add(1))
                    .write(GetSearchModeSelection(taskId, 5u8));
                    if !((IsNationalPokedexEnabled()) != 0) {
                        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24))
                            .wrapping_add(1))
                        .write(0u8);
                    }
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1556)
                        .cast::<u16>())
                    .write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(24))
                        .wrapping_add(1))
                        .read()) as u16),
                    );
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24))
                        .write(GetSearchModeSelection(taskId, 4u8));
                    ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1560)
                        .cast::<u16>())
                    .write(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24))
                            .read()) as u16),
                    );
                    PlaySE(3u16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_ExitSearch));
                } else {
                    EraseAndPrintSearchTextBox((&raw mut gText_SearchingPleaseWait).cast::<u8>());
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_StartPokedexSearch));
                    PlaySE(112u16);
                    CopyWindowToVram(0u8, 2u8);
                }
            } else {
                PlaySE(21u16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_SelectSearchMenuItem));
            }
            return;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0)
            && ((((((movementMap).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 4,
            ))
            .cast::<u8>())
            .read()) as i32)
                != 255i32)
        {
            PlaySE(5u16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(
                (((((movementMap).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 4,
                ))
                .cast::<u8>())
                .read()) as i16),
            );
            HighlightSelectedSearchMenuItem(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u8),
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u8),
            );
            CopyWindowToVram(0u8, 2u8);
            CopyBgTilemapBufferToVram(3u8);
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0)
            && (((((((movementMap).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 4,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                != 255i32)
        {
            PlaySE(5u16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(
                ((((((movementMap).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 4,
                ))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i16),
            );
            HighlightSelectedSearchMenuItem(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u8),
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u8),
            );
            CopyWindowToVram(0u8, 2u8);
            CopyBgTilemapBufferToVram(3u8);
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0)
            && (((((((movementMap).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 4,
            ))
            .cast::<u8>())
            .wrapping_offset(2))
            .read()) as i32)
                != 255i32)
        {
            PlaySE(5u16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(
                ((((((movementMap).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 4,
                ))
                .cast::<u8>())
                .wrapping_offset(2))
                .read()) as i16),
            );
            HighlightSelectedSearchMenuItem(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u8),
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u8),
            );
            CopyWindowToVram(0u8, 2u8);
            CopyBgTilemapBufferToVram(3u8);
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0)
            && (((((((movementMap).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 4,
            ))
            .cast::<u8>())
            .wrapping_offset(3))
            .read()) as i32)
                != 255i32)
        {
            PlaySE(5u16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(
                ((((((movementMap).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 4,
                ))
                .cast::<u8>())
                .wrapping_offset(3))
                .read()) as i16),
            );
            HighlightSelectedSearchMenuItem(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u8),
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u8),
            );
            CopyWindowToVram(0u8, 2u8);
            CopyBgTilemapBufferToVram(3u8);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StartPokedexSearch(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut dexMode: u8 = GetSearchModeSelection(taskId, 5u8);
        let mut order: u8 = GetSearchModeSelection(taskId, 4u8);
        let mut abcGroup: u8 = GetSearchModeSelection(taskId, 0u8);
        let mut bodyColor: u8 = GetSearchModeSelection(taskId, 1u8);
        let mut type1: u8 = GetSearchModeSelection(taskId, 2u8);
        let mut type2: u8 = GetSearchModeSelection(taskId, 3u8);
        DoPokedexSearch(dexMode, order, abcGroup, bodyColor, type1, type2);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_WaitAndCompleteSearch));
    }
}
pub(crate) unsafe extern "C" fn Task_WaitAndCompleteSearch(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((IsSEPlaying()) != 0) {
            if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1548)
                .cast::<u16>())
            .read()) as i32)
                != 0i32
            {
                PlaySE(31u16);
                EraseAndPrintSearchTextBox((&raw mut gText_SearchCompleted).cast::<u8>());
            } else {
                PlaySE(32u16);
                EraseAndPrintSearchTextBox((&raw mut gText_NoMatchingPkmnWereFound).cast::<u8>());
            }
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_SearchCompleteWaitForInput));
            CopyWindowToVram(0u8, 2u8);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SearchCompleteWaitForInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            if ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1548)
                .cast::<u16>())
            .read()) as i32)
                != 0i32
            {
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1614))
                .write(1u8);
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1554)
                    .cast::<u16>())
                .write(((GetSearchModeSelection(taskId, 5u8)) as u16));
                ((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1558)
                    .cast::<u16>())
                .write(((GetSearchModeSelection(taskId, 4u8)) as u16));
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ExitSearch));
                PlaySE(3u16);
            } else {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_SwitchToSearchMenu));
                PlaySE(23u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SelectSearchMenuItem(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut menuItem: u8 = 0u8;
        let mut cursorPos: *mut u16 = core::ptr::null_mut();
        let mut scrollOffset: *mut u16 = core::ptr::null_mut();
        DrawOrEraseSearchParameterBox(0u8);
        menuItem = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u8);
        cursorPos = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(
            (((((((&raw const sSearchOptions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((menuItem) as i32) as isize * 8))
            .wrapping_add(4))
            .read()) as i32) as isize,
        ))
        .cast::<u16>();
        scrollOffset = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(
            (((((((&raw const sSearchOptions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((menuItem) as i32) as isize * 8))
            .wrapping_add(5))
            .read()) as i32) as isize,
        ))
        .cast::<u16>();
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(14))
        .write((((cursorPos).read()) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write((((scrollOffset).read()) as i16));
        PrintSearchParameterText(taskId);
        PrintSelectorArrow((((cursorPos).read()) as u32));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleSearchParameterInput));
        CopyWindowToVram(0u8, 2u8);
        CopyBgTilemapBufferToVram(3u8);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSearchParameterInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut menuItem: u8 = 0u8;
        let mut texts: *mut u8 = core::ptr::null_mut();
        let mut cursorPos: *mut u16 = core::ptr::null_mut();
        let mut scrollOffset: *mut u16 = core::ptr::null_mut();
        let mut maxOption: u16 = 0u16;
        let mut moved: u8 = 0u8;
        menuItem = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u8);
        texts = (((((&raw const sSearchOptions).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((menuItem) as i32) as isize * 8))
        .cast::<*mut u8>())
        .read();
        cursorPos = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(
            (((((((&raw const sSearchOptions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((menuItem) as i32) as isize * 8))
            .wrapping_add(4))
            .read()) as i32) as isize,
        ))
        .cast::<u16>();
        scrollOffset = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(
            (((((((&raw const sSearchOptions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((menuItem) as i32) as isize * 8))
            .wrapping_add(5))
            .read()) as i32) as isize,
        ))
        .cast::<u16>();
        maxOption = (((((((((&raw const sSearchOptions).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((menuItem) as i32) as isize * 8))
        .wrapping_add(6)
        .cast::<u16>())
        .read()) as i32)
            .wrapping_sub(1i32)) as u16);
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(21u16);
            ClearSearchParameterBoxText();
            DrawOrEraseSearchParameterBox(1u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_SwitchToSearchMenu));
            CopyWindowToVram(0u8, 2u8);
            CopyBgTilemapBufferToVram(3u8);
            return;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            PlaySE(23u16);
            ClearSearchParameterBoxText();
            DrawOrEraseSearchParameterBox(1u8);
            (cursorPos).write(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(14))
                .read()) as u16),
            );
            (scrollOffset).write(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as u16),
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_SwitchToSearchMenu));
            CopyWindowToVram(0u8, 2u8);
            CopyBgTilemapBufferToVram(3u8);
            return;
        }
        moved = 0u8;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0
        {
            if (((cursorPos).read()) as i32) != 0i32 {
                EraseSelectorArrow((((cursorPos).read()) as u32));
                (cursorPos).write(((cursorPos).read()).wrapping_sub(1));
                PrintSelectorArrow((((cursorPos).read()) as u32));
                moved = 1u8;
            } else {
                if (((scrollOffset).read()) as i32) != 0i32 {
                    (scrollOffset).write(((scrollOffset).read()).wrapping_sub(1));
                    PrintSearchParameterText(taskId);
                    PrintSelectorArrow((((cursorPos).read()) as u32));
                    moved = 1u8;
                }
            }
            if (moved) != 0 {
                PlaySE(5u16);
                EraseAndPrintSearchTextBox(
                    (((texts).wrapping_offset(
                        ((((cursorPos).read()) as i32)
                            .wrapping_add((((scrollOffset).read()) as i32)))
                            as isize
                            * 8,
                    ))
                    .cast::<*mut u8>())
                    .read(),
                );
                CopyWindowToVram(0u8, 2u8);
            }
            return;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0
        {
            if ((((cursorPos).read()) as i32) < 5i32)
                && ((((cursorPos).read()) as i32) < ((maxOption) as i32))
            {
                EraseSelectorArrow((((cursorPos).read()) as u32));
                (cursorPos).write(((cursorPos).read()).wrapping_add(1));
                PrintSelectorArrow((((cursorPos).read()) as u32));
                moved = 1u8;
            } else {
                if (((maxOption) as i32) > 5i32)
                    && ((((scrollOffset).read()) as i32) < ((maxOption) as i32).wrapping_sub(5i32))
                {
                    (scrollOffset).write(((scrollOffset).read()).wrapping_add(1));
                    PrintSearchParameterText(taskId);
                    PrintSelectorArrow(5u32);
                    moved = 1u8;
                }
            }
            if (moved) != 0 {
                PlaySE(5u16);
                EraseAndPrintSearchTextBox(
                    (((texts).wrapping_offset(
                        ((((cursorPos).read()) as i32)
                            .wrapping_add((((scrollOffset).read()) as i32)))
                            as isize
                            * 8,
                    ))
                    .cast::<*mut u8>())
                    .read(),
                );
                CopyWindowToVram(0u8, 2u8);
            }
            return;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ExitSearch(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ExitSearchWaitForFade));
    }
}
pub(crate) unsafe extern "C" fn Task_ExitSearchWaitForFade(taskId: u8) {
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
            FreeSearchWindowAndBgBuffers();
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSearchRectHighlight(flags: u8, x: u8, y: u8, width: u8) {
    unsafe {
        let mut flags = flags;
        let mut x = x;
        let mut y = y;
        let mut width = width;
        let mut i: u16 = 0u16;
        let mut temp: u16 = 0u16;
        let mut ptr: u32 = ((GetBgTilemapBuffer(3u8)) as usize as u32);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < ((width) as i32)) {
                    break 'l1;
                }
                'l2: {
                    temp = ((((ptr).wrapping_add(
                        (((((y) as i32).wrapping_add(0i32)).wrapping_mul(64i32)) as u32),
                    ))
                    .wrapping_add(
                        (((((x) as i32).wrapping_add(((i) as i32))).wrapping_mul(2i32)) as u32),
                    )) as usize as *mut u16)
                        .read();
                    temp = ((((temp) as i32) & 4095i32) as u16);
                    temp = ((((temp) as i32) | (((flags) as i32) << 12)) as u16);
                    ((((ptr).wrapping_add(
                        (((((y) as i32).wrapping_add(0i32)).wrapping_mul(64i32)) as u32),
                    ))
                    .wrapping_add(
                        (((((x) as i32).wrapping_add(((i) as i32))).wrapping_mul(2i32)) as u32),
                    )) as usize as *mut u16)
                        .write(temp);
                    temp = ((((ptr).wrapping_add(
                        (((((y) as i32).wrapping_add(1i32)).wrapping_mul(64i32)) as u32),
                    ))
                    .wrapping_add(
                        (((((x) as i32).wrapping_add(((i) as i32))).wrapping_mul(2i32)) as u32),
                    )) as usize as *mut u16)
                        .read();
                    temp = ((((temp) as i32) & 4095i32) as u16);
                    temp = ((((temp) as i32) | (((flags) as i32) << 12)) as u16);
                    ((((ptr).wrapping_add(
                        (((((y) as i32).wrapping_add(1i32)).wrapping_mul(64i32)) as u32),
                    ))
                    .wrapping_add(
                        (((((x) as i32).wrapping_add(((i) as i32))).wrapping_mul(2i32)) as u32),
                    )) as usize as *mut u16)
                        .write(temp);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawSearchMenuItemBgHighlight(
    searchBg: u8,
    unselected: u8,
    disabled: u8,
) {
    unsafe {
        let mut searchBg = searchBg;
        let mut unselected = unselected;
        let mut disabled = disabled;
        let mut highlightFlags: u8 =
            (((((unselected) as i32) & 1i32) | ((((disabled) as i32) & 1i32) << 1)) as u8);
        'l1: {
            let __sw1 = ((searchBg) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 {
                __fall = true;
                SetSearchRectHighlight(
                    highlightFlags,
                    (((((&raw const sSearchMenuTopBarItems).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((searchBg) as i32) as isize * 8))
                    .wrapping_add(4))
                    .read(),
                    (((((&raw const sSearchMenuTopBarItems).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((searchBg) as i32) as isize * 8))
                    .wrapping_add(5))
                    .read(),
                    (((((&raw const sSearchMenuTopBarItems).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((searchBg) as i32) as isize * 8))
                    .wrapping_add(6))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 3i32 || __sw1 == 4i32 || __sw1 == 7i32 || __sw1 == 8i32 {
                __fall = true;
                SetSearchRectHighlight(
                    highlightFlags,
                    (((((&raw const sSearchMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset((((searchBg) as i32).wrapping_sub(3i32)) as isize * 12))
                    .wrapping_add(4))
                    .read(),
                    (((((&raw const sSearchMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset((((searchBg) as i32).wrapping_sub(3i32)) as isize * 12))
                    .wrapping_add(5))
                    .read(),
                    (((((&raw const sSearchMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset((((searchBg) as i32).wrapping_sub(3i32)) as isize * 12))
                    .wrapping_add(6))
                    .read(),
                );
            }
            if __fall || __sw1 == 5i32 || __sw1 == 6i32 {
                __fall = true;
                SetSearchRectHighlight(
                    highlightFlags,
                    (((((&raw const sSearchMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset((((searchBg) as i32).wrapping_sub(3i32)) as isize * 12))
                    .wrapping_add(7))
                    .read(),
                    (((((&raw const sSearchMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset((((searchBg) as i32).wrapping_sub(3i32)) as isize * 12))
                    .wrapping_add(8))
                    .read(),
                    (((((&raw const sSearchMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset((((searchBg) as i32).wrapping_sub(3i32)) as isize * 12))
                    .wrapping_add(9))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 10i32 {
                __fall = true;
                SetSearchRectHighlight(
                    highlightFlags,
                    (((((&raw const sSearchMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(24))
                    .wrapping_add(4))
                    .read(),
                    (((((&raw const sSearchMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(24))
                    .wrapping_add(5))
                    .read(),
                    (((((&raw const sSearchMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(24))
                    .wrapping_add(6))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 9i32 {
                __fall = true;
                if !((IsNationalPokedexEnabled()) != 0) {
                    SetSearchRectHighlight(
                        highlightFlags,
                        (((((&raw const sSearchMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                (((searchBg) as i32).wrapping_sub(3i32)) as isize * 12,
                            ))
                        .wrapping_add(4))
                        .read(),
                        (((((((((&raw const sSearchMenuItems).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((((searchBg) as i32).wrapping_sub(3i32)) as isize * 12))
                        .wrapping_add(5))
                        .read()) as i32)
                            .wrapping_sub(2i32)) as u8),
                        (((((&raw const sSearchMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                (((searchBg) as i32).wrapping_sub(3i32)) as isize * 12,
                            ))
                        .wrapping_add(6))
                        .read(),
                    );
                } else {
                    SetSearchRectHighlight(
                        highlightFlags,
                        (((((&raw const sSearchMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                (((searchBg) as i32).wrapping_sub(3i32)) as isize * 12,
                            ))
                        .wrapping_add(4))
                        .read(),
                        (((((&raw const sSearchMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                (((searchBg) as i32).wrapping_sub(3i32)) as isize * 12,
                            ))
                        .wrapping_add(5))
                        .read(),
                        (((((&raw const sSearchMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                (((searchBg) as i32).wrapping_sub(3i32)) as isize * 12,
                            ))
                        .wrapping_add(6))
                        .read(),
                    );
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetInitialSearchMenuBgHighlights(topBarItem: u8) {
    unsafe {
        let mut topBarItem = topBarItem;
        'l1: {
            let __sw1 = ((topBarItem) as i32);
            if __sw1 == 0i32 {
                DrawSearchMenuItemBgHighlight(0u8, 0u8, 0u8);
                DrawSearchMenuItemBgHighlight(1u8, 1u8, 0u8);
                DrawSearchMenuItemBgHighlight(2u8, 1u8, 0u8);
                DrawSearchMenuItemBgHighlight(3u8, 1u8, 0u8);
                DrawSearchMenuItemBgHighlight(4u8, 1u8, 0u8);
                DrawSearchMenuItemBgHighlight(10u8, 1u8, 0u8);
                DrawSearchMenuItemBgHighlight(5u8, 1u8, 0u8);
                DrawSearchMenuItemBgHighlight(6u8, 1u8, 0u8);
                DrawSearchMenuItemBgHighlight(7u8, 1u8, 0u8);
                DrawSearchMenuItemBgHighlight(8u8, 1u8, 0u8);
                DrawSearchMenuItemBgHighlight(9u8, 1u8, 0u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                DrawSearchMenuItemBgHighlight(0u8, 1u8, 0u8);
                DrawSearchMenuItemBgHighlight(1u8, 0u8, 0u8);
                DrawSearchMenuItemBgHighlight(2u8, 1u8, 0u8);
                DrawSearchMenuItemBgHighlight(3u8, 1u8, 1u8);
                DrawSearchMenuItemBgHighlight(4u8, 1u8, 1u8);
                DrawSearchMenuItemBgHighlight(10u8, 1u8, 1u8);
                DrawSearchMenuItemBgHighlight(5u8, 1u8, 1u8);
                DrawSearchMenuItemBgHighlight(6u8, 1u8, 1u8);
                DrawSearchMenuItemBgHighlight(7u8, 1u8, 0u8);
                DrawSearchMenuItemBgHighlight(8u8, 1u8, 0u8);
                DrawSearchMenuItemBgHighlight(9u8, 1u8, 0u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                DrawSearchMenuItemBgHighlight(0u8, 1u8, 0u8);
                DrawSearchMenuItemBgHighlight(1u8, 1u8, 0u8);
                DrawSearchMenuItemBgHighlight(2u8, 0u8, 0u8);
                DrawSearchMenuItemBgHighlight(3u8, 1u8, 1u8);
                DrawSearchMenuItemBgHighlight(4u8, 1u8, 1u8);
                DrawSearchMenuItemBgHighlight(10u8, 1u8, 1u8);
                DrawSearchMenuItemBgHighlight(5u8, 1u8, 1u8);
                DrawSearchMenuItemBgHighlight(6u8, 1u8, 1u8);
                DrawSearchMenuItemBgHighlight(7u8, 1u8, 1u8);
                DrawSearchMenuItemBgHighlight(8u8, 1u8, 1u8);
                DrawSearchMenuItemBgHighlight(9u8, 1u8, 1u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HighlightSelectedSearchTopBarItem(topBarItem: u8) {
    unsafe {
        let mut topBarItem = topBarItem;
        SetInitialSearchMenuBgHighlights(topBarItem);
        EraseAndPrintSearchTextBox(
            (((((&raw const sSearchMenuTopBarItems).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((topBarItem) as i32) as isize * 8))
            .cast::<*mut u8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn HighlightSelectedSearchMenuItem(topBarItem: u8, menuItem: u8) {
    unsafe {
        let mut topBarItem = topBarItem;
        let mut menuItem = menuItem;
        SetInitialSearchMenuBgHighlights(topBarItem);
        'l1: {
            let __sw1 = ((menuItem) as i32);
            if __sw1 == 0i32 {
                DrawSearchMenuItemBgHighlight(3u8, 0u8, 0u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                DrawSearchMenuItemBgHighlight(4u8, 0u8, 0u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                DrawSearchMenuItemBgHighlight(10u8, 0u8, 0u8);
                DrawSearchMenuItemBgHighlight(5u8, 0u8, 0u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                DrawSearchMenuItemBgHighlight(10u8, 0u8, 0u8);
                DrawSearchMenuItemBgHighlight(6u8, 0u8, 0u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                DrawSearchMenuItemBgHighlight(7u8, 0u8, 0u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                DrawSearchMenuItemBgHighlight(8u8, 0u8, 0u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                DrawSearchMenuItemBgHighlight(9u8, 0u8, 0u8);
                break 'l1;
            }
        }
        EraseAndPrintSearchTextBox(
            (((((&raw const sSearchMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((menuItem) as i32) as isize * 12))
            .cast::<*mut u8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintSelectedSearchParameters(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut searchParamId: u16 = 0u16;
        ClearSearchMenuRect(40u32, 16u32, 96u32, 80u32);
        searchParamId = ((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .read()) as i32)
            .wrapping_add(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(7))
                .read()) as i32),
            )) as u16);
        PrintSearchText(
            (((((&raw const sDexSearchNameOptions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((searchParamId) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<*mut u8>())
            .read(),
            45u32,
            17u32,
        );
        searchParamId = ((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .read()) as i32)
            .wrapping_add(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(9))
                .read()) as i32),
            )) as u16);
        PrintSearchText(
            (((((&raw const sDexSearchColorOptions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((searchParamId) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<*mut u8>())
            .read(),
            45u32,
            33u32,
        );
        searchParamId = ((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .read()) as i32)
            .wrapping_add(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .read()) as i32),
            )) as u16);
        PrintSearchText(
            (((((&raw const sDexSearchTypeOptions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((searchParamId) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<*mut u8>())
            .read(),
            45u32,
            49u32,
        );
        searchParamId = ((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(12))
        .read()) as i32)
            .wrapping_add(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(13))
                .read()) as i32),
            )) as u16);
        PrintSearchText(
            (((((&raw const sDexSearchTypeOptions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((searchParamId) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<*mut u8>())
            .read(),
            93u32,
            49u32,
        );
        searchParamId = ((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as i32)
            .wrapping_add(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as i32),
            )) as u16);
        PrintSearchText(
            (((((&raw const sDexOrderOptions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((searchParamId) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<*mut u8>())
            .read(),
            45u32,
            65u32,
        );
        if (IsNationalPokedexEnabled()) != 0 {
            searchParamId = ((((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
                .wrapping_add(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32),
                )) as u16);
            PrintSearchText(
                (((((&raw const sDexModeOptions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((searchParamId) as i32) as isize * 8))
                .wrapping_add(4)
                .cast::<*mut u8>())
                .read(),
                45u32,
                81u32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DrawOrEraseSearchParameterBox(erase: u8) {
    unsafe {
        let mut erase = erase;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        let mut ptr: *mut u16 = (GetBgTilemapBuffer(3u8)).cast::<u16>();
        if !((erase) != 0) {
            ((ptr).wrapping_offset(17)).write(3083u16);
            {
                i = 18u16;
                'l1: loop {
                    if !(((i) as i32) < 31i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((ptr).wrapping_offset(((i) as i32) as isize)).write(2061u16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                j = 1u16;
                'l3: loop {
                    if !(((j) as i32) < 13i32) {
                        break 'l3;
                    }
                    'l4: {
                        (((ptr).wrapping_offset(17))
                            .wrapping_offset((((j) as i32).wrapping_mul(32i32)) as isize))
                        .write(1034u16);
                        {
                            i = 18u16;
                            'l5: loop {
                                if !(((i) as i32) < 31i32) {
                                    break 'l5;
                                }
                                'l6: {
                                    (((ptr).wrapping_offset(
                                        (((j) as i32).wrapping_mul(32i32)) as isize,
                                    ))
                                    .wrapping_offset(((i) as i32) as isize))
                                    .write(2u16);
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            ((ptr).wrapping_offset(433)).write(1035u16);
            {
                i = 18u16;
                'l7: loop {
                    if !(((i) as i32) < 31i32) {
                        break 'l7;
                    }
                    'l8: {
                        (((ptr).wrapping_offset(416)).wrapping_offset(((i) as i32) as isize))
                            .write(13u16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                j = 0u16;
                'l9: loop {
                    if !(((j) as i32) < 14i32) {
                        break 'l9;
                    }
                    'l10: {
                        {
                            i = 17u16;
                            'l11: loop {
                                if !(((i) as i32) < 30i32) {
                                    break 'l11;
                                }
                                'l12: {
                                    (((ptr).wrapping_offset(
                                        (((j) as i32).wrapping_mul(32i32)) as isize,
                                    ))
                                    .wrapping_offset(((i) as i32) as isize))
                                    .write(79u16);
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintSearchParameterText(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut texts: *mut u8 = (((((&raw const sSearchOptions).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32) as isize
                * 8,
        ))
        .cast::<*mut u8>())
        .read();
        let mut cursorPos: *mut u16 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(
            (((((((&raw const sSearchOptions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 8,
                ))
            .wrapping_add(4))
            .read()) as i32) as isize,
        ))
        .cast::<u16>();
        let mut scrollOffset: *mut u16 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(
            (((((((&raw const sSearchOptions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 8,
                ))
            .wrapping_add(5))
            .read()) as i32) as isize,
        ))
        .cast::<u16>();
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        ClearSearchParameterBoxText();
        {
            i = 0u16;
            j = (scrollOffset).read();
            'l1: loop {
                if !((((i) as i32) < 6i32)
                    && ((((((texts).wrapping_offset(((j) as i32) as isize * 8))
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read()) as usize)
                        != 0usize))
                {
                    break 'l1;
                }
                'l2: {
                    PrintSearchParameterTitle(
                        ((i) as u32),
                        (((texts).wrapping_offset(((j) as i32) as isize * 8))
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
                j = (j).wrapping_add(1);
            }
        }
        EraseAndPrintSearchTextBox(
            (((texts).wrapping_offset(
                ((((cursorPos).read()) as i32).wrapping_add((((scrollOffset).read()) as i32)))
                    as isize
                    * 8,
            ))
            .cast::<*mut u8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetSearchModeSelection(taskId: u8, option: u8) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut option = option;
        let mut cursorPos: *mut u16 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(
            (((((((&raw const sSearchOptions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((option) as i32) as isize * 8))
            .wrapping_add(4))
            .read()) as i32) as isize,
        ))
        .cast::<u16>();
        let mut scrollOffset: *mut u16 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(
            (((((((&raw const sSearchOptions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((option) as i32) as isize * 8))
            .wrapping_add(5))
            .read()) as i32) as isize,
        ))
        .cast::<u16>();
        let mut id: u16 =
            (((((cursorPos).read()) as i32).wrapping_add((((scrollOffset).read()) as i32))) as u16);
        'l1: {
            let __sw1 = ((option) as i32);
            let __matched = __sw1 == 5i32
                || __sw1 == 4i32
                || __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32;
            let mut __fall = false;
            if !__matched {
                __fall = true;
                return 0u8;
            }
            if __sw1 == 5i32 {
                __fall = true;
                return ((((&raw const sPokedexModes).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize))
                .read();
            }
            if __sw1 == 4i32 {
                __fall = true;
                return ((((&raw const sOrderOptions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize))
                .read();
            }
            if __sw1 == 0i32 {
                __fall = true;
                if ((id) as i32) == 0i32 {
                    return 255u8;
                } else {
                    return ((id) as u8);
                }
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if ((id) as i32) == 0i32 {
                    return 255u8;
                } else {
                    return ((((id) as i32).wrapping_sub(1i32)) as u8);
                }
            }
            if __fall || __sw1 == 2i32 || __sw1 == 3i32 {
                __fall = true;
                return ((((&raw const sDexSearchTypeIds).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize))
                .read();
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn SetDefaultSearchModeAndOrder(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut selected: u16 = 0u16;
        'l1: {
            let __sw1 = ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1556)
                .cast::<u16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 || !__matched {
                selected = 0u16;
                break 'l1;
            }
            if __sw1 == 1i32 {
                selected = 1u16;
                break 'l1;
            }
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((selected) as i16));
        'l2: {
            let __sw2 = ((((((&raw mut sPokedexView).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1560)
                .cast::<u16>())
            .read()) as i32);
            let __matched = __sw2 == 0i32
                || __sw2 == 1i32
                || __sw2 == 2i32
                || __sw2 == 3i32
                || __sw2 == 4i32
                || __sw2 == 5i32;
            if __sw2 == 0i32 || !__matched {
                selected = 0u16;
                break 'l2;
            }
            if __sw2 == 1i32 {
                selected = 1u16;
                break 'l2;
            }
            if __sw2 == 2i32 {
                selected = 2u16;
                break 'l2;
            }
            if __sw2 == 3i32 {
                selected = 3u16;
                break 'l2;
            }
            if __sw2 == 4i32 {
                selected = 4u16;
                break 'l2;
            }
            if __sw2 == 5i32 {
                selected = 5u16;
                break 'l2;
            }
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((selected) as i16));
    }
}
pub(crate) unsafe extern "C" fn SearchParamCantScrollUp(taskId: u8) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut menuItem: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u8);
        let mut scrollOffset: *mut u16 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(
            (((((((&raw const sSearchOptions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((menuItem) as i32) as isize * 8))
            .wrapping_add(5))
            .read()) as i32) as isize,
        ))
        .cast::<u16>();
        let mut lastOption: u16 = (((((((((&raw const sSearchOptions).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((menuItem) as i32) as isize * 8))
        .wrapping_add(6)
        .cast::<u16>())
        .read()) as i32)
            .wrapping_sub(1i32)) as u16);
        if (((lastOption) as i32) > 5i32) && ((((scrollOffset).read()) as i32) != 0i32) {
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
pub(crate) unsafe extern "C" fn SearchParamCantScrollDown(taskId: u8) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut menuItem: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u8);
        let mut scrollOffset: *mut u16 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(
            (((((((&raw const sSearchOptions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((menuItem) as i32) as isize * 8))
            .wrapping_add(5))
            .read()) as i32) as isize,
        ))
        .cast::<u16>();
        let mut lastOption: u16 = (((((((((&raw const sSearchOptions).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((menuItem) as i32) as isize * 8))
        .wrapping_add(6)
        .cast::<u16>())
        .read()) as i32)
            .wrapping_sub(1i32)) as u16);
        if (((lastOption) as i32) > 5i32)
            && ((((scrollOffset).read()) as i32) < ((lastOption) as i32).wrapping_sub(5i32))
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
pub(crate) unsafe extern "C" fn SpriteCB_SearchParameterScrollArrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if core::mem::transmute::<_, usize>(
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
            ))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
            .read(),
        ) == (Task_HandleSearchParameterInput as *const () as usize)
        {
            let mut val: u8 = 0u8;
            if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0 {
                if (SearchParamCantScrollDown(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
                )) != 0
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                } else {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                }
            } else {
                if (SearchParamCantScrollUp(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
                )) != 0
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                } else {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                }
            }
            val = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                as i32)
                .wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        .wrapping_mul(128i32),
                )) as u8);
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((crate::c::div_i32(
                    ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(((val) as i32) as isize))
                    .read()) as i32),
                    128i32,
                )) as i16),
            );
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
        } else {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateSearchParameterScrollArrows(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        spriteId = CreateSprite(
            (&raw const sScrollArrowSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            184i16,
            4i16,
            0u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((taskId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_SearchParameterScrollArrow));
        spriteId = CreateSprite(
            (&raw const sScrollArrowSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            184i16,
            108i16,
            0u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((taskId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(1i16);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(63),
            1,
            1,
            (1u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_SearchParameterScrollArrow));
    }
}
pub(crate) unsafe extern "C" fn EraseAndPrintSearchTextBox(str: *mut u8) {
    unsafe {
        let mut str = str;
        ClearSearchMenuRect(8u32, 120u32, 224u32, 32u32);
        PrintSearchText(str, 8u32, 121u32);
    }
}
pub(crate) unsafe extern "C" fn EraseSelectorArrow(y: u32) {
    unsafe {
        let mut y = y;
        ClearSearchMenuRect(
            144u32,
            ((y).wrapping_mul(16u32)).wrapping_add(8u32),
            8u32,
            16u32,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintSelectorArrow(y: u32) {
    unsafe {
        let mut y = y;
        PrintSearchText(
            (&raw mut gText_SelectorArrow).cast::<u8>(),
            144u32,
            ((y).wrapping_mul(16u32)).wrapping_add(9u32),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintSearchParameterTitle(y: u32, str: *mut u8) {
    unsafe {
        let mut y = y;
        let mut str = str;
        PrintSearchText(str, 152u32, ((y).wrapping_mul(16u32)).wrapping_add(9u32));
    }
}
pub(crate) unsafe extern "C" fn ClearSearchParameterBoxText() {
    unsafe {
        ClearSearchMenuRect(144u32, 8u32, 96u32, 96u32);
    }
}
