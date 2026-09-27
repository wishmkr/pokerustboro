//! Translated from `src/intro.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sIntroDrops_Pal sIntroLogo_Pal sIntroDropsLogo_Gfx sIntro1Bg_Pal sIntro1Bg0_Tilemap sIntro1Bg1_Tilemap sIntro1Bg2_Tilemap sIntro1Bg3_Tilemap sIntro1Bg_Gfx sIntroPokeball_Pal sIntroPokeball_Tilemap sIntroPokeball_Gfx sIntroStreaks_Pal sIntroStreaks_Gfx sIntroStreaks_Tilemap sIntroRayquzaOrb_Pal sIntroMisc_Pal sIntroMisc_Gfx sIntroFlygonSilhouette_Pal sIntroLati_Gfx sUnusedData sSpriteSheet_Sparkle sSpritePalette_Sparkle sOamData_Sparkle sAnim_Sparkle sAnims_Sparkle sSpriteTemplate_Sparkle sSparkleCoords sSpriteSheet_RunningPokemon sSpritePalettes_RunningPokemon sOamData_Volbeat sAnim_Volbeat sAnims_Volbeat sSpriteTemplate_Volbeat sOamData_Torchic sAnim_Torchic_Walk sAnim_Torchic_Run sAnim_Torchic_Trip sAnims_Torchic sSpriteTemplate_Torchic sOamData_Manectric sAnim_Manectric sAnims_Manectric sSpriteTemplate_Manectric sSpriteSheet_Lightning sSpritePalette_Lightning sOamData_Lightning sAnim_Lightning_Top sAnim_Lightning_Middle sAnim_Lightning_Bottom sAnims_Lightning sSpriteTemplate_Lightning sGroudonRockData sSpriteSheet_Bubbles sSpritePalette_Bubbles sKyogreBubbleData sOamData_Bubbles sAnim_Bubbles sAnims_Bubbles sSpriteTemplate_Bubbles sOamData_WaterDrop sAnim_WaterDrop_UpperHalf sAnim_WaterDrop_LowerHalf sAnim_WaterDrop_Reflection sAnim_WaterDrop_Ripple sAnims_WaterDrop sSpriteTemplate_WaterDrop sAnim_PlayerBicycle_Fast sAnim_PlayerBicycle_Slow sAnim_PlayerBicycle_LookBack sAnim_PlayerBicycle_LookForward sAnims_PlayerBicycle sOamData_GameFreakLetter sOamData_PresentsLetter sOamData_GameFreakLogo sAnim_GameFreakLetter_G sAnim_GameFreakLetter_A sAnim_GameFreakLetter_M sAnim_GameFreakLetter_E sAnim_GameFreakLetter_F sAnim_GameFreakLetter_R sAnim_GameFreakLetter_K sAnim_PresentsLetter_P sAnim_PresentsLetter_R sAnim_PresentsLetter_E sAnim_PresentsLetter_S sAnim_PresentsLetter_N sAnim_PresentsLetter_T sAnim_GameFreakLogo sAnims_GameFreakLetter sAnims_PresentsLetter sAnims_GameFreakLogo sGameFreakLetterData sPresentsLetterData sAffineAnim_GameFreak_Small sAffineAnim_GameFreak_GrowAndShrink sAffineAnim_GameFreak_GrowBig sAffineAnim_GameFreak_GrowMedium sAffineAnims_GameFreak sGameFreakLettersMoveSpeed sSpriteTemplate_GameFreakLetter sSpriteTemplate_PresentsLetter sSpriteTemplate_GameFreakLogo sGameFreakLetterStartDelays sOamData_FlygonSilhouette sAnim_FlygonSilhouette sAnims_FlygonSilhouette sSpriteTemplate_FlygonSilhouette sSpriteSheet_WaterDropsAndLogo sSpriteSheet_FlygonSilhouette sSpritePalettes_Intro1 sOamData_RayquazaOrb sAnim_RayquazaOrb sAnims_RayquazaOrb sSpriteTemplate_RayquazaOrb sSpriteSheet_RayquazaOrb sSpritePalette_RayquazaOrb
#[allow(unused_imports)]
use crate::data::intro::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sIntroCharacterGender: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnusedVar: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFlygonYOffset: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gIntroFrameCounter: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMultibootProgramStruct: crate::ffi::Align4<[u8; 44]> = crate::ffi::Align4([0; 44]);

unsafe extern "C" {
    static mut gAncientPowerRockSpriteTemplate: u8;
    static mut gBattleAnimPaletteTable: u8;
    static mut gBattleAnimPicTable: u8;
    static mut gHeap: u8;
    static mut gIntro3Bg_Pal: u8;
    static mut gIntroCloudsLeft_Tilemap: u8;
    static mut gIntroCloudsRight_Tilemap: u8;
    static mut gIntroCloudsSun_Tilemap: u8;
    static mut gIntroClouds_Gfx: u8;
    static mut gIntroCopyright_Gfx: u8;
    static mut gIntroCopyright_Pal: u8;
    static mut gIntroCopyright_Tilemap: u8;
    static mut gIntroCredits_MovingSceneryState: u8;
    static mut gIntroCredits_MovingSceneryVBase: u8;
    static mut gIntroCredits_MovingSceneryVOffset: u8;
    static mut gIntroGameFreakTextFade_Pal: u8;
    static mut gIntroGroudonBg_Tilemap: u8;
    static mut gIntroGroudon_Gfx: u8;
    static mut gIntroGroudon_Tilemap: u8;
    static mut gIntroKyogreBg_Tilemap: u8;
    static mut gIntroKyogre_Gfx: u8;
    static mut gIntroKyogre_Tilemap: u8;
    static mut gIntroLegendBg_Gfx: u8;
    static mut gIntroRayquazaClouds_Gfx: u8;
    static mut gIntroRayquazaClouds_Tilemap: u8;
    static mut gIntroRayquaza_Gfx: u8;
    static mut gIntroRayquaza_Tilemap: u8;
    static mut gMain: u8;
    static mut gMultiBootProgram_PokemonColosseum_Start: u8;
    static mut gPaletteFade: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gReservedSpritePaletteCount: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSaveFileStatus: u8;
    static mut gScanlineEffect: u8;
    static mut gSineTable: u8;
    static mut gSpritePalettes_IntroPlayerFlygon: u8;
    static mut gSpriteSheet_IntroBicycle: u8;
    static mut gSpriteSheet_IntroBrendan: u8;
    static mut gSpriteSheet_IntroFlygon: u8;
    static mut gSpriteSheet_IntroMay: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gTitleScreenAlphaBlend: u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BgAffineSet(a0: *mut u8, a1: *mut u8, a2: i32);
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn BuildOamBuffer();
    fn CB2_InitTitleScreen();
    fn CalcCenterToCornerVec(a0: *mut u8, a1: u8, a2: u8, a3: u8);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateBicycleBgAnimationTask(a0: u8, a1: u16, a2: u16, a3: u16) -> u8;
    fn CreateIntroBrendanSprite(a0: i16, a1: i16) -> u8;
    fn CreateIntroFlygonSprite(a0: i16, a1: i16) -> u8;
    fn CreateIntroMaySprite(a0: i16, a1: i16) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CycleSceneryPalette(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn EnableInterrupts(a0: u16);
    fn FreeAllSpritePalettes();
    fn GameCubeMultiBoot_ExecuteProgram(a0: *mut u8);
    fn GameCubeMultiBoot_HandleSerialInterrupt(a0: *mut u8);
    fn GameCubeMultiBoot_Init(a0: *mut u8);
    fn GameCubeMultiBoot_Main(a0: *mut u8);
    fn GameCubeMultiBoot_Quit();
    fn GetSaveBlocksPointersBaseOffset() -> u16;
    fn InitHeap(a0: *mut u8, a1: u32);
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut u8);
    fn LZDecompressVram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedSpritePaletteUsingHeap(a0: *mut u8) -> u8;
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadCompressedSpriteSheetUsingHeap(a0: *mut u8) -> u8;
    fn LoadGameSave(a0: u8) -> u8;
    fn LoadIntroPart2Graphics(a0: u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpritePalettes(a0: *mut u8);
    fn PlayCryInternal(a0: u16, a1: i8, a2: i8, a3: u8, a4: u8);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn Random() -> u16;
    fn ResetMenuAndMonGlobals();
    fn ResetPaletteFade();
    fn ResetSerial();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn Sav2_ClearSetDefault();
    fn Save_ResetSaveCounters();
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn ScanlineEffect_InitWave(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u8;
    fn ScanlineEffect_Stop();
    fn SerialCB();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetIntroPart2BgCnt(a0: u8);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetOamMatrix(a0: u8, a1: u16, a2: u16, a3: u16, a4: u16);
    fn SetPokemonCryStereo(a0: u32);
    fn SetSaveBlocksPointers(a0: u16);
    fn SetSerialCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnimIfDifferent(a0: *mut u8, a1: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn m4aSongNumStart(a0: u16);
}

pub(crate) unsafe extern "C" fn VBlankCB_Intro() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
        ScanlineEffect_InitHBlankDmaTransfer();
    }
}
pub(crate) unsafe extern "C" fn MainCB2_Intro() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            != 0i32)
            && (!((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0))
        {
            SetMainCallback2(Some(MainCB2_EndIntro));
        } else {
            if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() != 4294967295u32 {
                let __p1 = (&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn MainCB2_EndIntro() {
    unsafe {
        if !((UpdatePaletteFade()) != 0) {
            SetMainCallback2(Some(CB2_InitTitleScreen));
        }
    }
}
pub(crate) unsafe extern "C" fn LoadCopyrightGraphics(
    tilesetAddress: u16,
    tilemapAddress: u16,
    paletteOffset: u16,
) {
    unsafe {
        let mut tilesetAddress = tilesetAddress;
        let mut tilemapAddress = tilemapAddress;
        let mut paletteOffset = paletteOffset;
        LZ77UnCompVram(
            ((&raw mut gIntroCopyright_Gfx).cast::<u32>()).cast::<u32>(),
            (((100663296i32).wrapping_add(((tilesetAddress) as i32))) as usize as *mut u8),
        );
        LZ77UnCompVram(
            ((&raw mut gIntroCopyright_Tilemap).cast::<u32>()).cast::<u32>(),
            (((100663296i32).wrapping_add(((tilemapAddress) as i32))) as usize as *mut u8),
        );
        LoadPalette(
            (((&raw mut gIntroCopyright_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            paletteOffset,
            32u16,
        );
    }
}
pub(crate) unsafe extern "C" fn SerialCB_CopyrightScreen() {
    unsafe {
        GameCubeMultiBoot_HandleSerialInterrupt((&raw mut gMultibootProgramStruct).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn SetUpCopyrightScreen() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 140i32 || __sw1 == 141i32;
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                SetVBlankCallback(None);
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetGpuReg(84u8, 0u16);
                ((83886080i32) as usize as *mut u16).write(32767u16);
                SetGpuReg(0u8, 0u16);
                SetGpuReg(16u8, 0u16);
                SetGpuReg(18u8, 0u16);
                'l2: loop {
                    'l3: {
                        {
                            let mut tmp: u32 = 0u32;
                            (&raw mut tmp).write_volatile(0u32);
                            'l4: loop {
                                'l5: {
                                    CpuSet(
                                        (&raw mut tmp).cast::<u8>(),
                                        ((100663296i32) as usize as *mut u8),
                                        ((83886080i32
                                            | (crate::c::div_i32(
                                                98304i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                'l6: loop {
                    'l7: {
                        {
                            let mut tmp: u32 = 0u32;
                            (&raw mut tmp).write_volatile(0u32);
                            'l8: loop {
                                'l9: {
                                    CpuSet(
                                        (&raw mut tmp).cast::<u8>(),
                                        ((117440512i32) as usize as *mut u8),
                                        ((83886080i32
                                            | (crate::c::div_i32(
                                                1024i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l8;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l6;
                    }
                }
                'l10: loop {
                    'l11: {
                        {
                            let mut tmp: u16 = 0u16;
                            (&raw mut tmp).write_volatile(0u16);
                            'l12: loop {
                                'l13: {
                                    CpuSet(
                                        (&raw mut tmp).cast::<u8>(),
                                        ((83886082i32) as usize as *mut u8),
                                        ((16777216i32
                                            | (crate::c::div_i32(
                                                1022i32,
                                                crate::c::div_i32(16i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l12;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l10;
                    }
                }
                ResetPaletteFade();
                LoadCopyrightGraphics(0u16, 14336u16, 0u16);
                ScanlineEffect_Stop();
                ResetTasks();
                ResetSpriteData();
                FreeAllSpritePalettes();
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 65535u16);
                SetGpuReg(8u8, 1792u16);
                EnableInterrupts(1u16);
                SetVBlankCallback(Some(VBlankCB_Intro));
                crate::c::volatile_write(((67108864i32) as usize as *mut u16), 320u16);
                SetSerialCallback(Some(SerialCB_CopyrightScreen));
                GameCubeMultiBoot_Init((&raw mut gMultibootProgramStruct).cast::<u8>());
            }
            if __fall || !__matched {
                __fall = true;
                UpdatePaletteFade();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                GameCubeMultiBoot_Main((&raw mut gMultibootProgramStruct).cast::<u8>());
                break 'l1;
            }
            if __sw1 == 140i32 {
                __fall = true;
                GameCubeMultiBoot_Main((&raw mut gMultibootProgramStruct).cast::<u8>());
                if (((((&raw mut gMultibootProgramStruct).cast::<u8>()).wrapping_add(2))
                    .read_volatile()) as i32)
                    != 1i32
                {
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                    let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 141i32 {
                __fall = true;
                if (UpdatePaletteFade()) != 0 {
                    break 'l1;
                }
                CreateTask(Some(Task_Scene1_Load), 0u8);
                SetMainCallback2(Some(MainCB2_Intro));
                if (((((&raw mut gMultibootProgramStruct).cast::<u8>()).wrapping_add(2))
                    .read_volatile()) as i32)
                    != 0i32
                {
                    if (((((&raw mut gMultibootProgramStruct).cast::<u8>()).wrapping_add(2))
                        .read_volatile()) as i32)
                        == 2i32
                    {
                        if ((33554604i32) as usize as *mut u32).read() == 1698063175u32 {
                            'l14: loop {
                                'l15: {
                                    'l16: loop {
                                        'l17: {
                                            CpuSet((((&raw mut gMultiBootProgram_PokemonColosseum_Start).cast::<u16>()).cast::<u16>()).cast::<u8>(), ((33554432i32) as usize as *mut u8), (0u32 | (crate::c::div_u32(163840u32, (((crate::c::div_i32(16i32, 8i32)) as u32))) & 2097151u32)));
                                        }
                                        if !((0i32) != 0) {
                                            break 'l16;
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l14;
                                }
                            }
                            ((33554604i32) as usize as *mut u32).write(1698063175u32);
                        }
                        GameCubeMultiBoot_ExecuteProgram(
                            (&raw mut gMultibootProgramStruct).cast::<u8>(),
                        );
                    }
                } else {
                    GameCubeMultiBoot_Quit();
                    SetSerialCallback(Some(SerialCB));
                }
                return 0u8;
            }
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitCopyrightScreenAfterBootup() {
    unsafe {
        if !((SetUpCopyrightScreen()) != 0) {
            SetSaveBlocksPointers(GetSaveBlocksPointersBaseOffset());
            ResetMenuAndMonGlobals();
            Save_ResetSaveCounters();
            LoadGameSave(0u8);
            if (((((&raw mut gSaveFileStatus).cast::<u16>()).read()) as i32) == 0i32)
                || (((((&raw mut gSaveFileStatus).cast::<u16>()).read()) as i32) == 2i32)
            {
                Sav2_ClearSetDefault();
            }
            SetPokemonCryStereo(
                ((crate::c::bf_read(
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(21),
                    0,
                    1,
                    false,
                ) as u16) as u32),
            );
            InitHeap((&raw mut gHeap).cast::<u8>(), 114688u32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitCopyrightScreenAfterTitleScreen() {
    unsafe {
        SetUpCopyrightScreen();
    }
}
pub(crate) unsafe extern "C" fn Task_Scene1_Load(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetVBlankCallback(None);
        ((&raw mut sIntroCharacterGender).cast::<u8>().cast::<u16>()).write(
            ((if (0i32) != 0 {
                crate::c::rem_i32(((Random()) as i32), 2i32)
            } else {
                (((Random()) as i32) & 1i32)
            }) as u16),
        );
        IntroResetGpuRegs();
        SetGpuReg(30u8, 0u16);
        SetGpuReg(26u8, 80u16);
        SetGpuReg(22u8, 24u16);
        SetGpuReg(18u8, 40u16);
        LZ77UnCompVram(
            ((&raw const sIntro1Bg_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100663296i32) as usize as *mut u8),
        );
        LZ77UnCompVram(
            ((&raw const sIntro1Bg0_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100696064i32) as usize as *mut u8),
        );
        'l1: loop {
            'l2: {
                {
                    let mut _dest: *mut u16 = ((100698112i32) as usize as *mut u16);
                    let mut _size: u32 = 2048u32;
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
                                                (2164260864u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
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
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        LZ77UnCompVram(
            ((&raw const sIntro1Bg1_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100700160i32) as usize as *mut u8),
        );
        'l7: loop {
            'l8: {
                {
                    let mut _dest: *mut u16 = ((100702208i32) as usize as *mut u16);
                    let mut _size: u32 = 2048u32;
                    'l9: loop {
                        'l10: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l11: loop {
                                    'l12: {
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
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
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
            if !((0i32) != 0) {
                break 'l7;
            }
        }
        LZ77UnCompVram(
            ((&raw const sIntro1Bg2_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100704256i32) as usize as *mut u8),
        );
        'l13: loop {
            'l14: {
                {
                    let mut _dest: *mut u16 = ((100706304i32) as usize as *mut u16);
                    let mut _size: u32 = 2048u32;
                    'l15: loop {
                        'l16: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l17: loop {
                                    'l18: {
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
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l17;
                                    }
                                }
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
        LZ77UnCompVram(
            ((&raw const sIntro1Bg3_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100708352i32) as usize as *mut u8),
        );
        'l19: loop {
            'l20: {
                {
                    let mut _dest: *mut u16 = ((100710400i32) as usize as *mut u16);
                    let mut _size: u32 = 2048u32;
                    'l21: loop {
                        'l22: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l23: loop {
                                    'l24: {
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
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l23;
                                    }
                                }
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
        LoadPalette(
            (((&raw const sIntro1Bg_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            0u16,
            512u16,
        );
        SetGpuReg(14u8, 38403u16);
        SetGpuReg(12u8, 37890u16);
        SetGpuReg(10u8, 37377u16);
        SetGpuReg(8u8, 36864u16);
        LoadCompressedSpriteSheet(
            ((&raw const sSpriteSheet_WaterDropsAndLogo)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        LoadCompressedSpriteSheet(
            ((&raw const sSpriteSheet_FlygonSilhouette)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        LoadSpritePalettes(
            ((&raw const sSpritePalettes_Intro1).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        LoadCompressedSpriteSheet(
            ((&raw const sSpriteSheet_Sparkle).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        LoadSpritePalettes(
            ((&raw const sSpritePalette_Sparkle).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        'l25: loop {
            'l26: {
                'l27: loop {
                    'l28: {
                        CpuSet(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(256))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(496))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    32u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l27;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l25;
            }
        }
        'l29: loop {
            'l30: {
                'l31: loop {
                    'l32: {
                        CpuSet(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(256))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(481))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    31u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l31;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l29;
            }
        }
        'l33: loop {
            'l34: {
                'l35: loop {
                    'l36: {
                        CpuSet(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(256))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(466))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    28u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l35;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l33;
            }
        }
        'l37: loop {
            'l38: {
                'l39: loop {
                    'l40: {
                        CpuSet(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(256))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(451))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    26u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l39;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l37;
            }
        }
        'l41: loop {
            'l42: {
                'l43: loop {
                    'l44: {
                        CpuSet(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(256))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(436))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    24u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l43;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l41;
            }
        }
        'l45: loop {
            'l46: {
                'l47: loop {
                    'l48: {
                        CpuSet(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(256))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(421))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    22u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l47;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l45;
            }
        }
        'l49: loop {
            'l50: {
                'l51: loop {
                    'l52: {
                        CpuSet(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(256))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(406))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    20u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l51;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l49;
            }
        }
        CreateGameFreakLogoSprites(
            ((crate::c::div_i32(240i32, 2i32)) as i16),
            ((crate::c::div_i32(160i32, 2i32)) as i16),
            0i16,
        );
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((CreateWaterDrop(236i16, (-14i16), 512u16, 1u16, 120u16, 0u8)) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Scene1_FadeIn));
    }
}
pub(crate) unsafe extern "C" fn Task_Scene1_FadeIn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
        SetVBlankCallback(Some(VBlankCB_Intro));
        SetGpuReg(0u8, 8000u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Scene1_WaterDrops));
        ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).write(0u32);
        m4aSongNumStart(414u16);
        ResetSerial();
    }
}
pub(crate) unsafe extern "C" fn Task_Scene1_WaterDrops(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 76u32 {
            (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .write(1i16);
        }
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 128u32 {
            CreateTask(Some(Task_BlendLogoIn), 0u8);
        }
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 251u32 {
            (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .write(2i16);
        }
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 256u32 {
            CreateTask(Some(Task_BlendLogoOut), 0u8);
        }
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 368u32 {
            CreateWaterDrop(48i16, 0i16, 1024u16, 5u16, 112u16, 1u8);
        }
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 384u32 {
            CreateWaterDrop(200i16, 60i16, 1024u16, 9u16, 128u16, 1u8);
        }
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 560u32 {
            CreateTask(Some(Task_CreateSparkles), 0u8);
        }
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() > 560u32 {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(80i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(24i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(40i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Scene1_PanUp));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CreateSparkles(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((({
            let __p1 = (data).wrapping_offset(2);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            & 1i32)
            != 0
        {
            let __p3 = (data).wrapping_offset(3);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        'l1: {
            let __sw4 = (((data).read()) as i32);
            if __sw4 == 0i32 {
                CreateSprite(
                    (&raw const sSpriteTemplate_Sparkle).cast::<u8>().cast_mut(),
                    (((((((&raw const sSparkleCoords).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((data).wrapping_offset(4)).read()) as i32) as isize * 2,
                        ))
                    .cast::<u8>())
                    .read()) as i16),
                    ((((((((((&raw const sSparkleCoords).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((data).wrapping_offset(4)).read()) as i32) as isize * 2,
                        ))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_add(((((data).wrapping_offset(3)).read()) as i32)))
                        as i16),
                    0u8,
                );
                (data).write(((data).read()).wrapping_add(1));
                ((data).wrapping_offset(1)).write(12i16);
                let __p5 = (data).wrapping_offset(4);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw4 == 1i32 {
                if (({
                    let __p6 = (data).wrapping_offset(1);
                    let __t7 = ((__p6).read()).wrapping_sub(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    == 0i32
                {
                    (data).write(0i16);
                }
                break 'l1;
            }
        }
        if ((((data).wrapping_offset(3)).read()) as i32) > 60i32 {
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Sparkle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 12i32
        {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Scene1_PanUp(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() < 904u32 {
            let mut offset: i32 = 0i32;
            offset = (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                << 16)
                .wrapping_add(
                    (((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as u16) as i32),
                );
            offset = (offset).wrapping_sub(24576i32);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((offset >> 16) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(((offset) as i16));
            SetGpuReg(
                26u8,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u16),
            );
            offset = (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as i32)
                << 16)
                .wrapping_add(
                    (((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as u16) as i32),
                );
            offset = (offset).wrapping_sub(32768i32);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(((offset >> 16) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(((offset) as i16));
            SetGpuReg(
                22u8,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as u16),
            );
            offset = (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as i32)
                << 16)
                .wrapping_add(
                    (((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as u16) as i32),
                );
            offset = (offset).wrapping_sub(49152i32);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(((offset >> 16) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(((offset) as i16));
            SetGpuReg(
                18u8,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as u16),
            );
            if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 832u32 {
                let mut spriteId: u8 = CreateSprite(
                    (&raw const sSpriteTemplate_FlygonSilhouette)
                        .cast::<u8>()
                        .cast_mut(),
                    120i16,
                    160i16,
                    10u8,
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
        } else {
            if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() > 1007u32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 65535u16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_Scene1_End));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Scene1_End(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() > 1026u32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Scene2_Load));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Scene2_Load(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        IntroResetGpuRegs();
        SetVBlankCallback(None);
        ResetSpriteData();
        FreeAllSpritePalettes();
        ((&raw mut gIntroCredits_MovingSceneryVBase).cast::<u16>()).write(0u16);
        ((&raw mut gIntroCredits_MovingSceneryVOffset).cast::<i16>()).write(0i16);
        ((&raw mut sFlygonYOffset).cast::<u8>().cast::<u16>()).write(0u16);
        LoadIntroPart2Graphics(1u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Scene2_CreateSprites));
    }
}
pub(crate) unsafe extern "C" fn Task_Scene2_CreateSprites(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        if ((((&raw mut sIntroCharacterGender).cast::<u8>().cast::<u16>()).read()) as i32) == 0i32 {
            LoadCompressedSpriteSheet((&raw mut gSpriteSheet_IntroBrendan).cast::<u8>());
        } else {
            LoadCompressedSpriteSheet((&raw mut gSpriteSheet_IntroMay).cast::<u8>());
        }
        LoadCompressedSpriteSheet((&raw mut gSpriteSheet_IntroBicycle).cast::<u8>());
        LoadCompressedSpriteSheet((&raw mut gSpriteSheet_IntroFlygon).cast::<u8>());
        {
            spriteId = 0u8;
            'l1: loop {
                if !(((spriteId) as u32) < (crate::c::div_u32(32u32, 8u32)).wrapping_sub(1u32)) {
                    break 'l1;
                }
                'l2: {
                    LoadCompressedSpriteSheet(
                        (((&raw const sSpriteSheet_RunningPokemon)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 8),
                    );
                }
                spriteId = (spriteId).wrapping_add(1);
            }
        }
        LoadSpritePalettes((&raw mut gSpritePalettes_IntroPlayerFlygon).cast::<u8>());
        LoadSpritePalettes(
            ((&raw const sSpritePalettes_RunningPokemon)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        CreateSprite(
            (&raw const sSpriteTemplate_Manectric)
                .cast::<u8>()
                .cast_mut(),
            272i16,
            128i16,
            0u8,
        );
        CreateSprite(
            (&raw const sSpriteTemplate_Torchic).cast::<u8>().cast_mut(),
            288i16,
            110i16,
            1u8,
        );
        if ((((&raw mut sIntroCharacterGender).cast::<u8>().cast::<u16>()).read()) as i32) == 0i32 {
            spriteId = CreateIntroBrendanSprite(272i16, 100i16);
        } else {
            spriteId = CreateIntroMaySprite(272i16, 100i16);
        }
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_PlayerOnBicycle));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(8)
            .cast::<*mut *mut u8>())
        .write(
            ((&raw const sAnims_PlayerBicycle)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>(),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((spriteId) as i16));
        CreateSprite(
            (&raw const sSpriteTemplate_Volbeat).cast::<u8>().cast_mut(),
            272i16,
            80i16,
            4u8,
        );
        spriteId = CreateIntroFlygonSprite((-64i16), 60i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Flygon));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((spriteId) as i16));
        BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 65535u16);
        SetVBlankCallback(Some(VBlankCB_Intro));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((CreateBicycleBgAnimationTask(1u8, 16384u16, 1024u16, 16u16)) as i16));
        SetIntroPart2BgCnt(1u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Scene2_BikeRide));
    }
}
pub(crate) unsafe extern "C" fn Task_Scene2_BikeRide(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut offset: u16 = 0u16;
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 1856u32 {
            ((&raw mut gIntroCredits_MovingSceneryState).cast::<i16>()).write(2i16);
            DestroyTask(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u8),
            );
        }
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() > 1946u32 {
            BeginNormalPaletteFade(4294967295u32, 8i8, 0u8, 16u8, 65535u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Scene2_End));
        }
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 1109u32 {
            (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .write(1i16);
        }
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 1214u32 {
            (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .write(0i16);
        }
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 1394u32 {
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
        }
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 1398u32 {
            (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .write(2i16);
        }
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 1576u32 {
            (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .write(3i16);
        }
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 1727u32 {
            (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .write(4i16);
        }
        offset = ((Sin(
            (((((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as i32)
                >> 2)
                & 127i32) as i16),
            48i16,
        )) as u16);
        ((&raw mut sFlygonYOffset).cast::<u8>().cast::<u16>()).write(offset);
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            < 512i32
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        CycleSceneryPalette(0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_Scene2_End(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() > 2068u32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Scene3_Load));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Volbeat(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        'l1: {
            let __sw2 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw2 == 0i32 {
                __fall = true;
                if (({
                    let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    < 180i32
                {
                    break 'l1;
                }
                let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
            }
            if __fall || __sw2 == 1i32 {
                __fall = true;
                let __p6 = (sprite).wrapping_add(32).cast::<i16>();
                (__p6).write((((((__p6).read()) as i32).wrapping_sub(4i32)) as i16));
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) == 60i32 {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(8i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(20i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(2i16);
                }
                break 'l1;
            }
            if __sw2 == 2i32 {
                __fall = true;
                let __p7 = (sprite).wrapping_add(32).cast::<i16>();
                (__p7).write((((((__p7).read()) as i32).wrapping_add(8i32)) as i16));
                let __p8 = (sprite).wrapping_add(34).cast::<i16>();
                (__p8).write((((((__p8).read()) as i32).wrapping_sub(2i32)) as i16));
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) == 124i32 {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(8i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(20i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(3i16);
                }
                break 'l1;
            }
            if __sw2 == 3i32 {
                __fall = true;
                let __p9 = (sprite).wrapping_add(34).cast::<i16>();
                (__p9).write((((((__p9).read()) as i32).wrapping_add(4i32)) as i16));
                if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) == 80i32 {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(8i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(10i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(4i16);
                }
                break 'l1;
            }
            if __sw2 == 4i32 {
                __fall = true;
                let __p10 = (sprite).wrapping_add(32).cast::<i16>();
                (__p10).write((((((__p10).read()) as i32).wrapping_sub(8i32)) as i16));
                let __p11 = (sprite).wrapping_add(34).cast::<i16>();
                (__p11).write((((((__p11).read()) as i32).wrapping_sub(2i32)) as i16));
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) == 60i32 {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(8i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(10i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(5i16);
                }
                break 'l1;
            }
            if __sw2 == 5i32 {
                __fall = true;
                let __p12 = (sprite).wrapping_add(32).cast::<i16>();
                (__p12).write((((((__p12).read()) as i32).wrapping_add(60i32)) as i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(192i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(128i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(3i16);
                let __p13 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p13).write(((__p13).read()).wrapping_add(1));
            }
            if __fall || __sw2 == 6i32 {
                __fall = true;
                ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as u8) as i16),
                    60i16,
                ));
                ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as u8) as i16),
                    20i16,
                ));
                let __p14 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                (__p14).write((((((__p14).read()) as i32).wrapping_add(2i32)) as i16));
                let __p15 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p15).write((((((__p15).read()) as i32).wrapping_add(4i32)) as i16));
                if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    & 255i32)
                    == 64i32
                {
                    crate::c::bf_write((sprite).wrapping_add(63), 0, 1, (0u16) as i32);
                    if (({
                        let __p16 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                        let __t17 = ((__p16).read()).wrapping_sub(1);
                        (__p16).write(__t17);
                        __t17
                    }) as i32)
                        == 0i32
                    {
                        let __p18 = (sprite).wrapping_add(32).cast::<i16>();
                        (__p18).write(
                            (((((__p18).read()) as i32).wrapping_add(
                                ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32),
                            )) as i16),
                        );
                        ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                        let __p19 = ((sprite).wrapping_add(46)).cast::<i16>();
                        (__p19).write(((__p19).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw2 == 7i32 {
                __fall = true;
                let __p20 = (sprite).wrapping_add(32).cast::<i16>();
                (__p20).write((((((__p20).read()) as i32).wrapping_sub(2i32)) as i16));
                ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as u8) as i16),
                    20i16,
                ));
                let __p21 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p21).write((((((__p21).read()) as i32).wrapping_add(4i32)) as i16));
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < (-16i32) {
                    DestroySprite(sprite);
                }
                break 'l1;
            }
            if __sw2 == 8i32 {
                __fall = true;
                ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as u8) as i16),
                    2i16,
                ));
                if !(({
                    let __p22 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t23 = ((__p22).read()).wrapping_sub(1);
                    (__p22).write(__t23);
                    __t23
                }) != 0)
                {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                    );
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Torchic(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 1224u32 {
                    StartSpriteAnim(sprite, 1u8);
                    let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 1576u32 {
                    StartSpriteAnim(sprite, 0u8);
                    let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                } else {
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    (__p4).write((((((__p4).read()) as i32).wrapping_add(64i32)) as i16));
                    if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        & 65280i32)
                        != 0
                    {
                        let __p5 = (sprite).wrapping_add(32).cast::<i16>();
                        (__p5).write(((__p5).read()).wrapping_sub(1));
                        let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                        (__p6).write((((((__p6).read()) as i32) & 255i32) as i16));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() != 1735u32 {
                    let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    (__p7).write((((((__p7).read()) as i32).wrapping_add(32i32)) as i16));
                    if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        & 65280i32)
                        != 0
                    {
                        let __p8 = (sprite).wrapping_add(32).cast::<i16>();
                        (__p8).write(((__p8).read()).wrapping_add(1));
                        let __p9 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                        (__p9).write((((((__p9).read()) as i32) & 255i32) as i16));
                    }
                } else {
                    StartSpriteAnim(sprite, 1u8);
                    let __p10 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(80i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ({
                    let __p11 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t12 = ((__p11).read()).wrapping_sub(1);
                    (__p11).write(__t12);
                    __t12
                }) != 0
                {
                    let __p13 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    (__p13).write((((((__p13).read()) as i32).wrapping_add(64i32)) as i16));
                    if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        & 65280i32)
                        != 0
                    {
                        let __p14 = (sprite).wrapping_add(32).cast::<i16>();
                        (__p14).write(((__p14).read()).wrapping_sub(1));
                        let __p15 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                        (__p15).write((((((__p15).read()) as i32) & 255i32) as i16));
                    }
                } else {
                    StartSpriteAnim(sprite, 2u8);
                    let __p16 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p16).write(((__p16).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
                    let __p17 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p17).write((((((__p17).read()) as i32).wrapping_add(4i32)) as i16));
                }
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > 336i32 {
                    StartSpriteAnim(sprite, 1u8);
                    let __p18 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p18).write(((__p18).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() >= 1856u32 {
                    let __p19 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p19).write((((((__p19).read()) as i32).wrapping_sub(2i32)) as i16));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Manectric(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 1088u32 {
                    let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                let __p3 = (sprite).wrapping_add(32).cast::<i16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(2i32)) as i16));
                if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() != 1168u32 {
                    break 'l1;
                }
                let __p4 = (sprite).wrapping_add(34).cast::<i16>();
                (__p4).write((((((__p4).read()) as i32).wrapping_sub(12i32)) as i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(128i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 2i32 {
                __fall = true;
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32))
                    <= (-32i32)
                {
                    DestroySprite(sprite);
                } else {
                    if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        & 255i32)
                        < 64i32
                    {
                        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                            (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                .read()) as u8) as i16),
                            16i16,
                        ));
                    } else {
                        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                            .read()) as i32)
                            & 255i32)
                            == 64i32
                        {
                            let __p6 = (sprite).wrapping_add(32).cast::<i16>();
                            (__p6).write((((((__p6).read()) as i32).wrapping_sub(48i32)) as i16));
                        }
                        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                            (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                .read()) as u8) as i16),
                            64i16,
                        ));
                    }
                    let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as u8) as i16),
                        12i16,
                    ));
                    let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_Load(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        IntroResetGpuRegs();
        LZ77UnCompVram(
            ((&raw const sIntroPokeball_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100663296i32) as usize as *mut u8),
        );
        LZ77UnCompVram(
            ((&raw const sIntroPokeball_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100679680i32) as usize as *mut u8),
        );
        LoadPalette(
            (((&raw const sIntroPokeball_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            0u16,
            512u16,
        );
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
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
        PanFadeAndZoomScreen(
            ((crate::c::div_i32(240i32, 2i32)) as u16),
            ((crate::c::div_i32(160i32, 2i32)) as u16),
            0u16,
            0u16,
        );
        ResetSpriteData();
        FreeAllSpritePalettes();
        BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 65535u16);
        SetGpuReg(12u8, 18563u16);
        SetGpuReg(0u8, 5185u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Scene3_SpinPokeball));
        ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).write(0u32);
        m4aSongNumStart(442u16);
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_SpinPokeball(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let __p1 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(1024i32)) as i16));
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            <= 1727i32
        {
            let __p2 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
            );
            let __p3 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p3).write((((((__p3).read()) as i32).wrapping_add(2i32)) as i16));
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Scene3_WaitGroudon));
        }
        PanFadeAndZoomScreen(
            ((crate::c::div_i32(240i32, 2i32)) as u16),
            ((crate::c::div_i32(160i32, 2i32)) as u16),
            ((if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                != 0i32
            {
                crate::c::div_i32(
                    65536i32,
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32),
                )
            } else {
                0i32
            }) as u16),
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as u16),
        );
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 28u32 {
            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 65535u16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_WaitGroudon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() > 43u32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Scene3_LoadGroudon));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_LoadGroudon(taskId: u8) {
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
            IntroResetGpuRegs();
            ResetSpriteData();
            FreeAllSpritePalettes();
            ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(8u8);
            LZDecompressVram(
                ((&raw mut gIntroGroudon_Gfx).cast::<u32>()).cast::<u32>(),
                ((100663296i32) as usize as *mut u8),
            );
            LZDecompressVram(
                ((&raw mut gIntroGroudon_Tilemap).cast::<u32>()).cast::<u32>(),
                ((100712448i32) as usize as *mut u8),
            );
            LZDecompressVram(
                ((&raw mut gIntroLegendBg_Gfx).cast::<u32>()).cast::<u32>(),
                ((100679680i32) as usize as *mut u8),
            );
            LZDecompressVram(
                ((&raw mut gIntroGroudonBg_Tilemap).cast::<u32>()).cast::<u32>(),
                ((100720640i32) as usize as *mut u8),
            );
            LoadCompressedSpriteSheetUsingHeap(
                ((&raw mut gBattleAnimPicTable).cast::<u8>()).wrapping_offset(464),
            );
            LoadCompressedSpritePaletteUsingHeap(
                ((&raw mut gBattleAnimPaletteTable).cast::<u8>()).wrapping_offset(464),
            );
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut gIntro3Bg_Pal).cast::<u8>(),
                                (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                    .cast::<u8>(),
                                (0u32
                                    | (crate::c::div_u32(
                                        512u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Scene3_InitGroudonBg));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_InitGroudonBg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetGpuReg(64u8, 240u16);
        SetGpuReg(68u8, 160u16);
        SetGpuReg(72u8, 63u16);
        SetGpuReg(74u8, 0u16);
        SetGpuReg(12u8, 47232u16);
        SetGpuReg(10u8, 7173u16);
        SetGpuReg(0u8, 13889u16);
        BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 65535u16);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write((-96i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write((-175i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(256i16);
        PanFadeAndZoomScreen(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as u16),
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as u16),
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as u16),
            0u16,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Scene3_NarrowWindow));
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_NarrowWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            != 32i32
        {
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
            SetGpuReg(
                68u8,
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_mul(256i32))
                .wrapping_sub(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_sub(160i32),
                )) as u16),
            );
        } else {
            SetGpuReg(68u8, 8320u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Scene3_EndNarrowWindow));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_EndNarrowWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Scene3_StartGroudon));
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_StartGroudon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Scene3_Groudon));
        ScanlineEffect_InitWave(0u8, 160u8, 4u8, 4u8, 1u8, 4u8, 0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_Groudon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let __p1 = (data).wrapping_offset(5);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((data).read()) as i32) >= 1i32) && ((((data).read()) as i32) <= 7i32))
            && (crate::c::rem_i32(((((data).wrapping_offset(5)).read()) as i32), 2i32) == 0i32)
        {
            let __p2 = (data).wrapping_offset(4);
            (__p2).write((((((__p2).read()) as i32) ^ 3i32) as i16));
        }
        PanFadeAndZoomScreen(
            ((((data).wrapping_offset(1)).read()) as u16),
            ((((((data).wrapping_offset(2)).read()) as i32)
                .wrapping_add(((((data).wrapping_offset(4)).read()) as i32))) as u16),
            ((((data).wrapping_offset(3)).read()) as u16),
            0u16,
        );
        'l1: {
            let __sw3 = (((data).read()) as i32);
            if __sw3 == 0i32 {
                let __p4 = (data).wrapping_offset(1);
                (__p4).write((((((__p4).read()) as i32).wrapping_add(16i32)) as i16));
                if ((((data).wrapping_offset(1)).read()) as i32) == 160i32 {
                    (data).write(((data).read()).wrapping_add(1));
                    ((data).wrapping_offset(6)).write(2i16);
                    ((data).wrapping_offset(7)).write(482i16);
                    CreateGroudonRockSprites(taskId);
                }
                break 'l1;
            }
            if __sw3 == 1i32 {
                if (({
                    let __p5 = (data).wrapping_offset(6);
                    let __t6 = ((__p5).read()).wrapping_sub(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    == 0i32
                {
                    ((data).wrapping_offset(6)).write(2i16);
                    'l2: loop {
                        'l3: {
                            'l4: loop {
                                'l5: {
                                    CpuSet(
                                        (((&raw mut gIntro3Bg_Pal).cast::<u8>()).cast::<u8>())
                                            .wrapping_offset(
                                                ((((data).wrapping_offset(7)).read()) as i32)
                                                    as isize
                                                    * 1,
                                            ),
                                        ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset(31))
                                        .cast::<u8>(),
                                        (0u32
                                            | (crate::c::div_u32(
                                                2u32,
                                                ((crate::c::div_i32(16i32, 8i32)) as u32),
                                            ) & 2097151u32)),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l2;
                        }
                    }
                    let __p7 = (data).wrapping_offset(7);
                    (__p7).write((((((__p7).read()) as i32).wrapping_add(2i32)) as i16));
                    if ((((data).wrapping_offset(7)).read()) as i32) == 492i32 {
                        (data).write(((data).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw3 == 2i32 {
                if (({
                    let __p8 = (data).wrapping_offset(6);
                    let __t9 = ((__p8).read()).wrapping_sub(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    == 0i32
                {
                    ((data).wrapping_offset(6)).write(2i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw3 == 3i32 {
                if (({
                    let __p10 = (data).wrapping_offset(6);
                    let __t11 = ((__p10).read()).wrapping_sub(1);
                    (__p10).write(__t11);
                    __t11
                }) as i32)
                    == 0i32
                {
                    ((data).wrapping_offset(6)).write(2i16);
                    'l6: loop {
                        'l7: {
                            'l8: loop {
                                'l9: {
                                    CpuSet(
                                        (((&raw mut gIntro3Bg_Pal).cast::<u8>()).cast::<u8>())
                                            .wrapping_offset(
                                                ((((data).wrapping_offset(7)).read()) as i32)
                                                    as isize
                                                    * 1,
                                            ),
                                        ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset(31))
                                        .cast::<u8>(),
                                        (0u32
                                            | (crate::c::div_u32(
                                                2u32,
                                                ((crate::c::div_i32(16i32, 8i32)) as u32),
                                            ) & 2097151u32)),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l8;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l6;
                        }
                    }
                    let __p12 = (data).wrapping_offset(7);
                    (__p12).write((((((__p12).read()) as i32).wrapping_sub(2i32)) as i16));
                    if ((((data).wrapping_offset(7)).read()) as i32) == 480i32 {
                        ((data).wrapping_offset(6)).write(8i16);
                        (data).write(((data).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw3 == 4i32 {
                if (({
                    let __p13 = (data).wrapping_offset(6);
                    let __t14 = ((__p13).read()).wrapping_sub(1);
                    (__p13).write(__t14);
                    __t14
                }) as i32)
                    == 0i32
                {
                    ((data).wrapping_offset(1)).write((-96i16));
                    ((data).wrapping_offset(2)).write(169i16);
                    ((data).wrapping_offset(6)).write(3i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw3 == 5i32 {
                if (({
                    let __p15 = (data).wrapping_offset(6);
                    let __t16 = ((__p15).read()).wrapping_sub(1);
                    (__p15).write(__t16);
                    __t16
                }) as i32)
                    == 0i32
                {
                    ((data).wrapping_offset(1)).write(80i16);
                    ((data).wrapping_offset(2)).write(41i16);
                    ((data).wrapping_offset(6)).write(16i16);
                    PlayCryInternal(405u16, 0i8, 100i8, 10u8, 0u8);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw3 == 6i32 {
                if (({
                    let __p17 = (data).wrapping_offset(6);
                    let __t18 = ((__p17).read()).wrapping_sub(1);
                    (__p17).write(__t18);
                    __t18
                }) as i32)
                    == 0i32
                {
                    ((data).wrapping_offset(1)).write(80i16);
                    ((data).wrapping_offset(2)).write(40i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw3 == 7i32 {
                let __p19 = (data).wrapping_offset(1);
                (__p19).write((((((__p19).read()) as i32).wrapping_add(4i32)) as i16));
                let __p20 = (data).wrapping_offset(2);
                (__p20).write((((((__p20).read()) as i32).wrapping_add(4i32)) as i16));
                let __p21 = (data).wrapping_offset(6);
                (__p21).write((((((__p21).read()) as i32).wrapping_add(1638i32)) as i16));
                ((data).wrapping_offset(3)).write(
                    ((((Sin(
                        (((((((data).wrapping_offset(6)).read()) as i32) & 65280i32) >> 8) as i16),
                        64i16,
                    )) as i32)
                        .wrapping_add(256i32)) as i16),
                );
                if ((((data).wrapping_offset(1)).read()) as i32) == 120i32 {
                    BeginNormalPaletteFade(4294967294u32, 3i8, 0u8, 16u8, 32767u16);
                    ((data).wrapping_offset(3)).write(256i16);
                    ((data).wrapping_offset(4)).write(0i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw3 == 8i32 {
                if (((data).wrapping_offset(3)).read()) != 0 {
                    let __p22 = (data).wrapping_offset(3);
                    (__p22).write((((((__p22).read()) as i32).wrapping_sub(8i32)) as i16));
                } else {
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw3 == 9i32 {
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
                    .write(Some(Task_Scene3_LoadKyogre));
                    (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateGroudonRockSprites(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut spriteId: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(36u32, 6u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (&raw mut gAncientPowerRockSpriteTemplate).cast::<u8>(),
                        (((((&raw const sGroudonRockData).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset((i) as isize * 6))
                        .cast::<i16>())
                        .read(),
                        160i16,
                        ((i) as u8),
                    );
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_GroudonRocks));
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(5),
                        2,
                        2,
                        (0u16) as i32,
                    );
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
                    .wrapping_offset(4))
                    .write(((taskId) as i16));
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        ((((((((&raw const sGroudonRockData).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize * 6))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_GroudonRocks(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if crate::c::rem_i32(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
            2i32,
        ) == 0i32
        {
            let __p2 = (sprite).wrapping_add(38).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32) ^ 3i32) as i16));
        }
        'l1: {
            let __sw3 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw3 == 0i32 {
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_add(
                        ((((((((&raw const sGroudonRockData).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                .read()) as i32) as isize
                                * 6,
                        ))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as i32),
                    )) as i16),
                );
                let __p5 = (sprite).wrapping_add(34).cast::<i16>();
                (__p5).write(
                    (((((__p5).read()) as i32).wrapping_sub(
                        ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            & 65280i32)
                            >> 8),
                    )) as i16),
                );
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p6).write((((((__p6).read()) as i32) & 255i32) as i16));
                if (((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32) as isize
                        * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32)
                    > 7i32
                {
                    let __p7 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw3 == 1i32 {
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    < crate::c::div_i32(240i32, 2i32)
                {
                    let __p8 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p8).write((((((__p8).read()) as i32).wrapping_sub(2i32)) as i16));
                } else {
                    let __p9 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p9).write((((((__p9).read()) as i32).wrapping_add(2i32)) as i16));
                }
                if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    < crate::c::div_i32(160i32, 2i32)
                {
                    let __p10 = (sprite).wrapping_add(34).cast::<i16>();
                    (__p10).write((((((__p10).read()) as i32).wrapping_sub(2i32)) as i16));
                } else {
                    let __p11 = (sprite).wrapping_add(34).cast::<i16>();
                    (__p11).write((((((__p11).read()) as i32).wrapping_add(2i32)) as i16));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_LoadKyogre(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ResetSpriteData();
        LZDecompressVram(
            ((&raw mut gIntroKyogre_Gfx).cast::<u32>()).cast::<u32>(),
            ((100663296i32) as usize as *mut u8),
        );
        LZDecompressVram(
            ((&raw mut gIntroKyogre_Tilemap).cast::<u32>()).cast::<u32>(),
            ((100712448i32) as usize as *mut u8),
        );
        LZDecompressVram(
            ((&raw mut gIntroKyogreBg_Tilemap).cast::<u32>()).cast::<u32>(),
            ((100720640i32) as usize as *mut u8),
        );
        LoadCompressedSpriteSheet(
            ((&raw const sSpriteSheet_Bubbles).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        LoadSpritePalette(
            ((&raw const sSpritePalette_Bubbles).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        BeginNormalPaletteFade(4294967294u32, 0i8, 16u8, 0u8, 65535u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Scene3_Kyogre));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(336i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(80i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(16i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(256i16);
        PanFadeAndZoomScreen(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as u16),
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as u16),
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as u16),
            0u16,
        );
        ScanlineEffect_InitWave(0u8, 160u8, 4u8, 4u8, 1u8, 6u8, 0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_Kyogre(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        PanFadeAndZoomScreen(
            ((((data).wrapping_offset(1)).read()) as u16),
            ((((data).wrapping_offset(2)).read()) as u16),
            ((((data).wrapping_offset(3)).read()) as u16),
            0u16,
        );
        'l1: {
            let __sw1 = (((data).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if (({
                    let __p2 = (data).wrapping_offset(6);
                    let __t3 = ((__p2).read()).wrapping_sub(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    != 0i32
                {
                    break 'l1;
                }
                (data).write(((data).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                let __p4 = (data).wrapping_offset(6);
                (__p4).write((((((__p4).read()) as i32).wrapping_add(4i32)) as i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(
                    (((344i32)
                        .wrapping_sub(((Sin(((data).wrapping_offset(6)).read(), 256i16)) as i32)))
                        as i16),
                );
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(
                    (((84i32)
                        .wrapping_sub(((Cos(((data).wrapping_offset(6)).read(), 64i16)) as i32)))
                        as i16),
                );
                if ((((data).wrapping_offset(6)).read()) as i32) == 64i32 {
                    ((data).wrapping_offset(6)).write(25i16);
                    ((data).wrapping_offset(7)).write(1i16);
                    (data).write(((data).read()).wrapping_add(1));
                    CreateKyogreBubbleSprites_Body(0u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if (({
                    let __p5 = (data).wrapping_offset(6);
                    let __t6 = ((__p5).read()).wrapping_sub(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    == 0i32
                {
                    let __p7 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p7).write((((((__p7).read()) as i32).wrapping_add(256i32)) as i16));
                    let __p8 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    (__p8).write((((((__p8).read()) as i32).wrapping_sub(258i32)) as i16));
                    ((data).wrapping_offset(6)).write(8i16);
                    (data).write(((data).read()).wrapping_add(1));
                    CreateKyogreBubbleSprites_Body(0u8);
                    CreateKyogreBubbleSprites_Fins();
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if (({
                    let __p9 = (data).wrapping_offset(6);
                    let __t10 = ((__p9).read()).wrapping_sub(1);
                    (__p9).write(__t10);
                    __t10
                }) as i32)
                    == 0i32
                {
                    let __p11 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p11).write((((((__p11).read()) as i32).wrapping_sub(256i32)) as i16));
                    let __p12 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    (__p12).write((((((__p12).read()) as i32).wrapping_add(258i32)) as i16));
                    ((data).wrapping_offset(6)).write(8i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                if (({
                    let __p13 = (data).wrapping_offset(6);
                    let __t14 = ((__p13).read()).wrapping_sub(1);
                    (__p13).write(__t14);
                    __t14
                }) as i32)
                    == 0i32
                {
                    let __p15 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    (__p15).write((((((__p15).read()) as i32).wrapping_sub(252i32)) as i16));
                    ((data).wrapping_offset(6)).write(8i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                if (({
                    let __p16 = (data).wrapping_offset(6);
                    let __t17 = ((__p16).read()).wrapping_sub(1);
                    (__p16).write(__t17);
                    __t17
                }) as i32)
                    == 0i32
                {
                    let __p18 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    (__p18).write((((((__p18).read()) as i32).wrapping_add(252i32)) as i16));
                    if ((((data).wrapping_offset(7)).read()) as i32) != 0i32 {
                        ((data).wrapping_offset(6)).write(12i16);
                        let __p19 = (data).wrapping_offset(7);
                        (__p19).write(((__p19).read()).wrapping_sub(1));
                        (data).write(2i16);
                    } else {
                        ((data).wrapping_offset(6)).write(1i16);
                        (data).write(((data).read()).wrapping_add(1));
                        PlayCryInternal(404u16, 0i8, 120i8, 10u8, 0u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                if (({
                    let __p20 = (data).wrapping_offset(6);
                    let __t21 = ((__p20).read()).wrapping_sub(1);
                    (__p20).write(__t21);
                    __t21
                }) as i32)
                    == 0i32
                {
                    ((data).wrapping_offset(6)).write(4i16);
                    ((data).wrapping_offset(7)).write(490i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                __fall = true;
                if (({
                    let __p22 = (data).wrapping_offset(6);
                    let __t23 = ((__p22).read()).wrapping_sub(1);
                    (__p22).write(__t23);
                    __t23
                }) as i32)
                    == 0i32
                {
                    ((data).wrapping_offset(6)).write(4i16);
                    'l2: loop {
                        'l3: {
                            'l4: loop {
                                'l5: {
                                    CpuSet(
                                        (((&raw mut gIntro3Bg_Pal).cast::<u8>()).cast::<u8>())
                                            .wrapping_offset(
                                                ((((data).wrapping_offset(7)).read()) as i32)
                                                    as isize
                                                    * 1,
                                            ),
                                        ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset(47))
                                        .cast::<u8>(),
                                        (0u32
                                            | (crate::c::div_u32(
                                                2u32,
                                                ((crate::c::div_i32(16i32, 8i32)) as u32),
                                            ) & 2097151u32)),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l2;
                        }
                    }
                    let __p24 = (data).wrapping_offset(7);
                    (__p24).write((((((__p24).read()) as i32).wrapping_sub(2i32)) as i16));
                    if ((((data).wrapping_offset(7)).read()) as i32) == 480i32 {
                        (data).write(((data).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                __fall = true;
                if (({
                    let __p25 = (data).wrapping_offset(6);
                    let __t26 = ((__p25).read()).wrapping_sub(1);
                    (__p25).write(__t26);
                    __t26
                }) as i32)
                    == 0i32
                {
                    ((data).wrapping_offset(6)).write(4i16);
                    ((data).wrapping_offset(7)).write(482i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                __fall = true;
                if (({
                    let __p27 = (data).wrapping_offset(6);
                    let __t28 = ((__p27).read()).wrapping_sub(1);
                    (__p27).write(__t28);
                    __t28
                }) as i32)
                    == 0i32
                {
                    ((data).wrapping_offset(6)).write(4i16);
                    'l6: loop {
                        'l7: {
                            'l8: loop {
                                'l9: {
                                    CpuSet(
                                        (((&raw mut gIntro3Bg_Pal).cast::<u8>()).cast::<u8>())
                                            .wrapping_offset(
                                                ((((data).wrapping_offset(7)).read()) as i32)
                                                    as isize
                                                    * 1,
                                            ),
                                        ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset(47))
                                        .cast::<u8>(),
                                        (0u32
                                            | (crate::c::div_u32(
                                                2u32,
                                                ((crate::c::div_i32(16i32, 8i32)) as u32),
                                            ) & 2097151u32)),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l8;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l6;
                        }
                    }
                    let __p29 = (data).wrapping_offset(7);
                    (__p29).write((((((__p29).read()) as i32).wrapping_add(2i32)) as i16));
                    if ((((data).wrapping_offset(7)).read()) as i32) == 494i32 {
                        ((data).wrapping_offset(6)).write(16i16);
                        (data).write(((data).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                __fall = true;
                if (({
                    let __p30 = (data).wrapping_offset(6);
                    let __t31 = ((__p30).read()).wrapping_sub(1);
                    (__p30).write(__t31);
                    __t31
                }) as i32)
                    == 0i32
                {
                    ((data).wrapping_offset(6)).write(0i16);
                    (data).write(((data).read()).wrapping_add(1));
                    CreateKyogreBubbleSprites_Body(taskId);
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                __fall = true;
                let __p32 = (data).wrapping_offset(6);
                (__p32).write((((((__p32).read()) as i32).wrapping_add(4i32)) as i16));
                let __p33 = (data).wrapping_offset(3);
                (__p33).write((((((__p33).read()) as i32).wrapping_sub(8i32)) as i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(
                    ((((Sin(((data).wrapping_offset(6)).read(), 60i16)) as i32).wrapping_add(88i32))
                        as i16),
                );
                if ((((data).wrapping_offset(6)).read()) as i32) == 64i32 {
                    BeginNormalPaletteFade(4294967294u32, 3i8, 0u8, 16u8, 32767u16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                __fall = true;
                let __p34 = (data).wrapping_offset(6);
                (__p34).write((((((__p34).read()) as i32).wrapping_add(4i32)) as i16));
                let __p35 = (data).wrapping_offset(3);
                (__p35).write((((((__p35).read()) as i32).wrapping_sub(8i32)) as i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(
                    ((((Sin(((data).wrapping_offset(6)).read(), 20i16)) as i32)
                        .wrapping_add(128i32)) as i16),
                );
                if ((((data).wrapping_offset(6)).read()) as i32) == 128i32 {
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                __fall = true;
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
                    .write(Some(Task_Scene3_LoadClouds1));
                    (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateKyogreBubbleSprites_Body(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut spriteId: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (&raw const sSpriteTemplate_Bubbles).cast::<u8>().cast_mut(),
                        (((((&raw const sKyogreBubbleData).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset((i) as isize * 6))
                        .cast::<i16>())
                        .read(),
                        ((((((&raw const sKyogreBubbleData).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize * 6))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read(),
                        ((i) as u8),
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write(((taskId) as i16));
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .write(
                        ((((((&raw const sKyogreBubbleData).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize * 6))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read(),
                    );
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(7))
                    .write(64i16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateKyogreBubbleSprites_Fins() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut spriteId: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (&raw const sSpriteTemplate_Bubbles).cast::<u8>().cast_mut(),
                        (((((&raw const sKyogreBubbleData).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((i).wrapping_add(6i32)) as isize * 6))
                        .cast::<i16>())
                        .read(),
                        ((((((&raw const sKyogreBubbleData).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i).wrapping_add(6i32)) as isize * 6))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read(),
                        ((i) as u8),
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .write(
                        ((((((&raw const sKyogreBubbleData).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize * 6))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read(),
                    );
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(7))
                    .write(64i16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_KyogreBubbles(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                    as i32)
                    == 0i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                        ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32)
                            .wrapping_add(11i32) & 255i32) as i16),
                    );
                    ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                        4i16,
                    ));
                    let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    (__p2).write((((((__p2).read()) as i32).wrapping_add(48i32)) as i16));
                    ((sprite).wrapping_add(38).cast::<i16>()).write(
                        (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                            .read()) as i32)
                            >> 8)
                            .wrapping_neg()) as i16),
                    );
                    if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
                        DestroySprite(sprite);
                    }
                } else {
                    if (({
                        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                        let __t4 = ((__p3).read()).wrapping_sub(1);
                        (__p3).write(__t4);
                        __t4
                    }) as i32)
                        == 0i32
                    {
                        StartSpriteAnim(sprite, 0u8);
                        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    }
                }
                if (((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32) as isize
                        * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32)
                    > 11i32
                {
                    let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    < crate::c::div_i32(240i32, 2i32)
                {
                    let __p6 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p6).write((((((__p6).read()) as i32).wrapping_sub(3i32)) as i16));
                } else {
                    let __p7 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p7).write((((((__p7).read()) as i32).wrapping_add(3i32)) as i16));
                }
                if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    < crate::c::div_i32(160i32, 2i32)
                {
                    let __p8 = (sprite).wrapping_add(34).cast::<i16>();
                    (__p8).write((((((__p8).read()) as i32).wrapping_sub(3i32)) as i16));
                } else {
                    let __p9 = (sprite).wrapping_add(34).cast::<i16>();
                    (__p9).write((((((__p9).read()) as i32).wrapping_add(3i32)) as i16));
                }
                if (((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    .wrapping_sub(20i32)) as u16) as i32)
                    > 140i32
                {
                    DestroySprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_LoadClouds1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetGpuReg(80u8, 135u16);
        SetGpuReg(82u8, 7967u16);
        SetGpuReg(84u8, 31u16);
        SetGpuReg(8u8, 22528u16);
        SetGpuReg(10u8, 23044u16);
        SetGpuReg(12u8, 7174u16);
        SetGpuReg(0u8, 14144u16);
        SetGpuReg(16u8, 80u16);
        SetGpuReg(18u8, 0u16);
        SetGpuReg(20u8, 65456u16);
        SetGpuReg(22u8, 0u16);
        SetGpuReg(24u8, 0u16);
        SetGpuReg(26u8, 0u16);
        LZDecompressVram(
            ((&raw mut gIntroClouds_Gfx).cast::<u32>()).cast::<u32>(),
            ((100663296i32) as usize as *mut u8),
        );
        LZDecompressVram(
            ((&raw mut gIntroClouds_Gfx).cast::<u32>()).cast::<u32>(),
            ((100679680i32) as usize as *mut u8),
        );
        LZDecompressVram(
            ((&raw mut gIntroCloudsSun_Tilemap).cast::<u32>()).cast::<u32>(),
            ((100720640i32) as usize as *mut u8),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Scene3_LoadClouds2));
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_LoadClouds2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        LZDecompressVram(
            ((&raw mut gIntroCloudsLeft_Tilemap).cast::<u32>()).cast::<u32>(),
            ((100712448i32) as usize as *mut u8),
        );
        LZDecompressVram(
            ((&raw mut gIntroCloudsRight_Tilemap).cast::<u32>()).cast::<u32>(),
            ((100716544i32) as usize as *mut u8),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Scene3_InitClouds));
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_InitClouds(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        SetGpuReg(84u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Scene3_Clouds));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(16i16);
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_Clouds(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        SetGpuReg(
            16u8,
            ((((((data).wrapping_offset(6)).read()) as i32) >> 8) as u16),
        );
        SetGpuReg(
            20u8,
            (((((((data).wrapping_offset(6)).read()) as i32) >> 8).wrapping_neg()) as u16),
        );
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (data).wrapping_offset(6);
                    let __t3 = ((__p2).read()).wrapping_sub(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    == 0i32
                {
                    BeginNormalPaletteFade(4294967294u32, 0i8, 16u8, 0u8, 65535u16);
                    ((data).wrapping_offset(6)).write(20480i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((data).wrapping_offset(6)).read()) as i32) == 10240i32 {
                    BeginNormalPaletteFade(65534u32, 3i8, 0u8, 16u8, 10569u16);
                }
                if ((((data).wrapping_offset(6)).read()) as i32) != 0i32 {
                    let __p4 = (data).wrapping_offset(6);
                    (__p4).write((((((__p4).read()) as i32).wrapping_sub(128i32)) as i16));
                } else {
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
                        .write(Some(Task_Scene3_LoadLightning));
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_LoadLightning(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        LZDecompressVram(
            ((&raw mut gIntroRayquaza_Tilemap).cast::<u32>()).cast::<u32>(),
            ((100720640i32) as usize as *mut u8),
        );
        LZDecompressVram(
            ((&raw mut gIntroRayquazaClouds_Tilemap).cast::<u32>()).cast::<u32>(),
            ((100712448i32) as usize as *mut u8),
        );
        LZDecompressVram(
            ((&raw mut gIntroRayquaza_Gfx).cast::<u32>()).cast::<u32>(),
            ((100679680i32) as usize as *mut u8),
        );
        LZDecompressVram(
            ((&raw mut gIntroRayquazaClouds_Gfx).cast::<u32>()).cast::<u32>(),
            ((100663296i32) as usize as *mut u8),
        );
        SetGpuReg(0u8, 13632u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Scene3_Lightning));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(1i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(0i16);
        LoadCompressedSpriteSheetUsingHeap(
            ((&raw const sSpriteSheet_Lightning).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        LoadSpritePalettes(
            ((&raw const sSpritePalette_Lightning)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_Lightning(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut spriteId: u8 = 0u8;
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (data).wrapping_offset(6);
                    let __t3 = ((__p2).read()).wrapping_sub(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    == 0i32
                {
                    CreateSprite(
                        (&raw const sSpriteTemplate_Lightning)
                            .cast::<u8>()
                            .cast_mut(),
                        200i16,
                        48i16,
                        0u8,
                    );
                    spriteId = CreateSprite(
                        (&raw const sSpriteTemplate_Lightning)
                            .cast::<u8>()
                            .cast_mut(),
                        200i16,
                        80i16,
                        1u8,
                    );
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        1u8,
                    );
                    spriteId = CreateSprite(
                        (&raw const sSpriteTemplate_Lightning)
                            .cast::<u8>()
                            .cast_mut(),
                        200i16,
                        112i16,
                        2u8,
                    );
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        2u8,
                    );
                    (data).write(((data).read()).wrapping_add(1));
                    ((data).wrapping_offset(6)).write(72i16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p4 = (data).wrapping_offset(6);
                    let __t5 = ((__p4).read()).wrapping_sub(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    == 0i32
                {
                    CreateSprite(
                        (&raw const sSpriteTemplate_Lightning)
                            .cast::<u8>()
                            .cast_mut(),
                        40i16,
                        48i16,
                        0u8,
                    );
                    spriteId = CreateSprite(
                        (&raw const sSpriteTemplate_Lightning)
                            .cast::<u8>()
                            .cast_mut(),
                        40i16,
                        80i16,
                        1u8,
                    );
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        1u8,
                    );
                    spriteId = CreateSprite(
                        (&raw const sSpriteTemplate_Lightning)
                            .cast::<u8>()
                            .cast_mut(),
                        40i16,
                        112i16,
                        2u8,
                    );
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        2u8,
                    );
                    (data).write(((data).read()).wrapping_add(1));
                    ((data).wrapping_offset(6)).write(48i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p6 = (data).wrapping_offset(6);
                    let __t7 = ((__p6).read()).wrapping_sub(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    == 0i32
                {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_Scene3_LoadRayquazaAttack));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Lightning(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(450i16);
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                'l2: loop {
                    'l3: {
                        'l4: loop {
                            'l5: {
                                CpuSet(
                                    (((&raw mut gIntro3Bg_Pal).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(
                                            ((((((sprite).wrapping_add(46)).cast::<i16>())
                                                .wrapping_offset(1))
                                            .read())
                                                as i32)
                                                as isize
                                                * 1,
                                        ),
                                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(93))
                                    .cast::<u8>(),
                                    (0u32
                                        | (crate::c::div_u32(
                                            2u32,
                                            ((crate::c::div_i32(16i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l4;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(2i32)) as i16));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    != 462i32
                {
                    break 'l1;
                }
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(460i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(4i16);
                let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 2i32 {
                __fall = true;
                if (({
                    let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t6 = ((__p5).read()).wrapping_sub(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    == 0i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(4i16);
                    'l6: loop {
                        'l7: {
                            'l8: loop {
                                'l9: {
                                    CpuSet(
                                        (((&raw mut gIntro3Bg_Pal).cast::<u8>()).cast::<u8>())
                                            .wrapping_offset(
                                                ((((((sprite).wrapping_add(46)).cast::<i16>())
                                                    .wrapping_offset(1))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 1,
                                            ),
                                        ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset(93))
                                        .cast::<u8>(),
                                        (0u32
                                            | (crate::c::div_u32(
                                                2u32,
                                                ((crate::c::div_i32(16i32, 8i32)) as u32),
                                            ) & 2097151u32)),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l8;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l6;
                        }
                    }
                    let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    (__p7).write((((((__p7).read()) as i32).wrapping_sub(2i32)) as i16));
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        == 448i32
                    {
                        DestroySprite(sprite);
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_LoadRayquazaAttack(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut attackTaskId: u8 = 0u8;
        LoadCompressedSpriteSheet(
            ((&raw const sSpriteSheet_RayquazaOrb)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        LoadSpritePalettes(
            ((&raw const sSpritePalette_RayquazaOrb)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        SetGpuReg(0u8, 13632u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Scene3_Rayquaza));
        BeginNormalPaletteFade(65502u32, 0i8, 16u8, 0u8, 10569u16);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(168i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write((-16i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write((-136i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write((-16i16));
        attackTaskId = CreateTask(Some(Task_RayquazaAttack), 0u8);
        ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((attackTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((taskId) as i16));
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_Rayquaza(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if crate::c::rem_i32(((((data).wrapping_offset(7)).read()) as i32), 2i32) == 0i32 {
            let __p1 = (data).wrapping_offset(6);
            (__p1).write((((((__p1).read()) as i32) ^ 2i32) as i16));
        }
        let __p2 = (data).wrapping_offset(7);
        (__p2).write(((__p2).read()).wrapping_add(1));
        'l1: {
            let __sw3 = (((data).read()) as i32);
            if __sw3 == 0i32 {
                if (((((data).wrapping_offset(7)).read()) as i32) & 1i32) != 0i32 {
                    let __p4 = (data).wrapping_offset(1);
                    (__p4).write((((((__p4).read()) as i32).wrapping_sub(2i32)) as i16));
                    let __p5 = (data).wrapping_offset(2);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    let __p6 = (data).wrapping_offset(3);
                    (__p6).write((((((__p6).read()) as i32).wrapping_add(2i32)) as i16));
                    let __p7 = (data).wrapping_offset(4);
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                if ((((data).wrapping_offset(1)).read()) as i32) == 104i32 {
                    (data).write(((data).read()).wrapping_add(1));
                    ((data).wrapping_offset(5)).write(1i16);
                }
                break 'l1;
            }
            if __sw3 == 1i32 {
                (data).write(((data).read()).wrapping_add(1));
                ((data).wrapping_offset(5)).write(4i16);
                break 'l1;
            }
            if __sw3 == 2i32 {
                let __p8 = (data).wrapping_offset(1);
                (__p8).write((((((__p8).read()) as i32).wrapping_add(4i32)) as i16));
                let __p9 = (data).wrapping_offset(2);
                (__p9).write((((((__p9).read()) as i32).wrapping_sub(2i32)) as i16));
                let __p10 = (data).wrapping_offset(3);
                (__p10).write((((((__p10).read()) as i32).wrapping_sub(4i32)) as i16));
                let __p11 = (data).wrapping_offset(4);
                (__p11).write((((((__p11).read()) as i32).wrapping_sub(2i32)) as i16));
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ((data).wrapping_offset(5)).write(140i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw3 == 3i32 {
                if (({
                    let __p12 = (data).wrapping_offset(5);
                    let __t13 = ((__p12).read()).wrapping_sub(1);
                    (__p12).write(__t13);
                    __t13
                }) as i32)
                    == 0i32
                {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_EndIntroMovie));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_EndIntroMovie(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DestroyTask(taskId);
        SetMainCallback2(Some(MainCB2_EndIntro));
    }
}
pub(crate) unsafe extern "C" fn Task_RayquazaAttack(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let __p1 = (data).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
        'l1: {
            let __sw2 = (((data).read()) as i32);
            if __sw2 == 0i32 {
                if (((((data).wrapping_offset(2)).read()) as i32) & 1i32) != 0i32 {
                    'l2: loop {
                        'l3: {
                            'l4: loop {
                                'l5: {
                                    CpuSet(
                                        ((((&raw mut gIntro3Bg_Pal).cast::<u8>()).cast::<u8>())
                                            .wrapping_offset(418))
                                        .wrapping_offset(
                                            (((((data).wrapping_offset(1)).read()) as i32)
                                                .wrapping_mul(2i32))
                                                as isize
                                                * 1,
                                        ),
                                        ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset(94))
                                        .cast::<u8>(),
                                        (0u32
                                            | (crate::c::div_u32(
                                                2u32,
                                                ((crate::c::div_i32(16i32, 8i32)) as u32),
                                            ) & 2097151u32)),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l2;
                        }
                    }
                    let __p3 = (data).wrapping_offset(1);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                if ((((data).wrapping_offset(1)).read()) as i32) == 6i32 {
                    (data).write(((data).read()).wrapping_add(1));
                    ((data).wrapping_offset(1)).write(0i16);
                    ((data).wrapping_offset(3)).write(10i16);
                }
                break 'l1;
            }
            if __sw2 == 1i32 {
                if ((((data).wrapping_offset(3)).read()) as i32) == 0i32 {
                    if (((((data).wrapping_offset(2)).read()) as i32) & 1i32) != 0i32 {
                        'l6: loop {
                            'l7: {
                                'l8: loop {
                                    'l9: {
                                        CpuSet(
                                            ((((&raw mut gIntro3Bg_Pal).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(418))
                                            .wrapping_offset(
                                                (((((data).wrapping_offset(1)).read()) as i32)
                                                    .wrapping_mul(2i32))
                                                    as isize
                                                    * 1,
                                            ),
                                            ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(88))
                                            .cast::<u8>(),
                                            (0u32
                                                | (crate::c::div_u32(
                                                    2u32,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                ) & 2097151u32)),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l8;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l6;
                            }
                        }
                        let __p4 = (data).wrapping_offset(1);
                        (__p4).write(((__p4).read()).wrapping_add(1));
                    }
                    if ((((data).wrapping_offset(1)).read()) as i32) == 6i32 {
                        (data).write(((data).read()).wrapping_add(1));
                        ((data).wrapping_offset(3)).write(10i16);
                    }
                } else {
                    let __p5 = (data).wrapping_offset(3);
                    (__p5).write(((__p5).read()).wrapping_sub(1));
                }
                break 'l1;
            }
            if __sw2 == 2i32 {
                if ((((data).wrapping_offset(3)).read()) as i32) == 0i32 {
                    if (((((data).wrapping_offset(2)).read()) as i32) & 1i32) != 0i32 {
                        'l10: loop {
                            'l11: {
                                'l12: loop {
                                    'l13: {
                                        CpuSet(
                                            ((((&raw mut gIntro3Bg_Pal).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(386))
                                            .wrapping_offset(
                                                (((((data).wrapping_offset(1)).read()) as i32)
                                                    .wrapping_mul(2i32))
                                                    as isize
                                                    * 1,
                                            ),
                                            ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(92))
                                            .cast::<u8>(),
                                            (0u32
                                                | (crate::c::div_u32(
                                                    2u32,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                ) & 2097151u32)),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l12;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l10;
                            }
                        }
                        let __p6 = (data).wrapping_offset(1);
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                    if ((((data).wrapping_offset(1)).read()) as i32) == 6i32 {
                        spriteId = CreateSprite(
                            (&raw const sSpriteTemplate_RayquazaOrb)
                                .cast::<u8>()
                                .cast_mut(),
                            120i16,
                            88i16,
                            15u8,
                        );
                        PlaySE(103u16);
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(62),
                            2,
                            1,
                            (1u16) as i32,
                        );
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .write(((data).wrapping_offset(4)).read());
                        (data).write(((data).read()).wrapping_add(1));
                        ((data).wrapping_offset(3)).write(16i16);
                    }
                } else {
                    let __p7 = (data).wrapping_offset(3);
                    (__p7).write(((__p7).read()).wrapping_sub(1));
                }
                break 'l1;
            }
            if __sw2 == 3i32 {
                if (((((data).wrapping_offset(2)).read()) as i32) & 1i32) != 0i32 {
                    if (({
                        let __p8 = (data).wrapping_offset(3);
                        let __t9 = ((__p8).read()).wrapping_sub(1);
                        (__p8).write(__t9);
                        __t9
                    }) as i32)
                        != 0i32
                    {
                        BlendPalette(
                            80u16,
                            16u16,
                            ((((data).wrapping_offset(3)).read()) as u8),
                            10569u16,
                        );
                        'l14: loop {
                            'l15: {
                                'l16: loop {
                                    'l17: {
                                        CpuSet(
                                            (((&raw mut gIntro3Bg_Pal).cast::<u8>()).cast::<u8>())
                                                .wrapping_offset(428),
                                            ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(94))
                                            .cast::<u8>(),
                                            (0u32
                                                | (crate::c::div_u32(
                                                    2u32,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                ) & 2097151u32)),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l16;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l14;
                            }
                        }
                        'l18: loop {
                            'l19: {
                                'l20: loop {
                                    'l21: {
                                        CpuSet(
                                            (((&raw mut gIntro3Bg_Pal).cast::<u8>()).cast::<u8>())
                                                .wrapping_offset(428),
                                            ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(88))
                                            .cast::<u8>(),
                                            (0u32
                                                | (crate::c::div_u32(
                                                    2u32,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                ) & 2097151u32)),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l20;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l18;
                            }
                        }
                        'l22: loop {
                            'l23: {
                                'l24: loop {
                                    'l25: {
                                        CpuSet(
                                            (((&raw mut gIntro3Bg_Pal).cast::<u8>()).cast::<u8>())
                                                .wrapping_offset(396),
                                            ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(92))
                                            .cast::<u8>(),
                                            (0u32
                                                | (crate::c::div_u32(
                                                    2u32,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                ) & 2097151u32)),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l24;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l22;
                            }
                        }
                    } else {
                        (data).write(((data).read()).wrapping_add(1));
                        ((data).wrapping_offset(3)).write(53i16);
                    }
                }
                break 'l1;
            }
            if __sw2 == 4i32 {
                if (({
                    let __p10 = (data).wrapping_offset(3);
                    let __t11 = ((__p10).read()).wrapping_sub(1);
                    (__p10).write(__t11);
                    __t11
                }) as i32)
                    == 0i32
                {
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 32767u16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw2 == 5i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IntroResetGpuRegs() {
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
    }
}
pub(crate) unsafe extern "C" fn Task_BlendLogoIn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 || !__matched {
                SetGpuReg(80u8, 16192u16);
                SetGpuReg(
                    82u8,
                    ((((&raw mut gTitleScreenAlphaBlend).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(31))
                    .read(),
                );
                SetGpuReg(84u8, 0u16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(((crate::c::div_u32(128u32, 2u32)) as i16));
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    != 0i32
                {
                    let mut tmp: u8 = 0u8;
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p3).write(((__p3).read()).wrapping_sub(1));
                    tmp = ((crate::c::div_i32(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32),
                        2i32,
                    )) as u8);
                    SetGpuReg(
                        82u8,
                        ((((&raw mut gTitleScreenAlphaBlend).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((tmp) as i32) as isize))
                        .read(),
                    );
                } else {
                    SetGpuReg(
                        82u8,
                        (((&raw mut gTitleScreenAlphaBlend).cast::<u16>()).cast::<u16>()).read(),
                    );
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(((crate::c::div_u32(crate::c::div_u32(128u32, 2u32), 4u32)) as i16));
                    let __p4 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetGpuReg(84u8, 0u16);
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_BlendLogoOut(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 || !__matched {
                SetGpuReg(80u8, 16192u16);
                SetGpuReg(
                    82u8,
                    (((&raw mut gTitleScreenAlphaBlend).cast::<u16>()).cast::<u16>()).read(),
                );
                SetGpuReg(84u8, 0u16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(0i16);
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    < ((crate::c::div_u32(128u32, 2u32)) as i32).wrapping_sub(2i32)
                {
                    let mut tmp: u8 = 0u8;
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    tmp = ((crate::c::div_i32(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32),
                        2i32,
                    )) as u8);
                    SetGpuReg(
                        82u8,
                        ((((&raw mut gTitleScreenAlphaBlend).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((tmp) as i32) as isize))
                        .read(),
                    );
                } else {
                    SetGpuReg(
                        82u8,
                        ((((&raw mut gTitleScreenAlphaBlend).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(31))
                        .read(),
                    );
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(((crate::c::div_u32(crate::c::div_u32(128u32, 2u32), 4u32)) as i16));
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
                .wrapping_offset(1))
                .read()) as i32)
                    != 0i32
                {
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p5).write(((__p5).read()).wrapping_sub(1));
                } else {
                    SetGpuReg(80u8, 0u16);
                    SetGpuReg(82u8, 0u16);
                    SetGpuReg(84u8, 0u16);
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PanFadeAndZoomScreen(screenX: u16, screenY: u16, zoom: u16, alpha: u16) {
    unsafe {
        let mut screenX = screenX;
        let mut screenY = screenY;
        let mut zoom = zoom;
        let mut alpha = alpha;
        let mut src = crate::ffi::Align4([0u8; 20]);
        let mut dest = crate::ffi::Align4([0u8; 16]);
        (((&raw mut src).cast::<u8>()).cast::<i32>()).write(32768i32);
        (((&raw mut src).cast::<u8>()).wrapping_add(4).cast::<i32>()).write(32768i32);
        (((&raw mut src).cast::<u8>()).wrapping_add(8).cast::<i16>()).write(((screenX) as i16));
        (((&raw mut src).cast::<u8>()).wrapping_add(10).cast::<i16>()).write(((screenY) as i16));
        (((&raw mut src).cast::<u8>()).wrapping_add(12).cast::<i16>()).write(((zoom) as i16));
        (((&raw mut src).cast::<u8>()).wrapping_add(14).cast::<i16>()).write(((zoom) as i16));
        (((&raw mut src).cast::<u8>()).wrapping_add(16).cast::<u16>()).write(alpha);
        BgAffineSet(
            (&raw mut src).cast::<u8>(),
            (&raw mut dest).cast::<u8>(),
            1i32,
        );
        SetGpuReg(
            32u8,
            (((((&raw mut dest).cast::<u8>()).cast::<i16>()).read()) as u16),
        );
        SetGpuReg(
            34u8,
            (((((&raw mut dest).cast::<u8>()).wrapping_add(2).cast::<i16>()).read()) as u16),
        );
        SetGpuReg(
            36u8,
            (((((&raw mut dest).cast::<u8>()).wrapping_add(4).cast::<i16>()).read()) as u16),
        );
        SetGpuReg(
            38u8,
            (((((&raw mut dest).cast::<u8>()).wrapping_add(6).cast::<i16>()).read()) as u16),
        );
        SetGpuReg(
            40u8,
            (((((&raw mut dest).cast::<u8>()).wrapping_add(8).cast::<i32>()).read()) as u16),
        );
        SetGpuReg(
            42u8,
            (((((&raw mut dest).cast::<u8>()).wrapping_add(8).cast::<i32>()).read() >> 16) as u16),
        );
        SetGpuReg(
            44u8,
            (((((&raw mut dest).cast::<u8>())
                .wrapping_add(12)
                .cast::<i32>())
            .read()) as u16),
        );
        SetGpuReg(
            46u8,
            (((((&raw mut dest).cast::<u8>())
                .wrapping_add(12)
                .cast::<i32>())
            .read()
                >> 16) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WaterDrop_Ripple(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut palNum: u8 = 0u8;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            >= 192i32
        {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                != 0i32
            {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p1).write(((__p1).read()).wrapping_sub(1));
            } else {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                SetOamMatrix(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as u8),
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as u16),
                    0u16,
                    0u16,
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as u16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((crate::c::div_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            .wrapping_mul(95i32),
                        100i32,
                    )) as i16),
                );
                palNum = (((crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        .wrapping_sub(192i32),
                    128i32,
                ))
                .wrapping_add(9i32)) as u8);
                if ((palNum) as i32) > 15i32 {
                    palNum = 15u8;
                }
                crate::c::bf_write((sprite).wrapping_add(5), 4, 4, ((palNum) as u16) as i32);
            }
        } else {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WaterDropHalf(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .read()) as i32)
            != 0i32
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            StartSpriteAnim(sprite, 3u8);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1024i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                (((8i32).wrapping_mul(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        & 3i32),
                )) as i16),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_WaterDrop_Ripple));
            crate::c::bf_write((sprite).wrapping_add(1), 6, 2, (1u32) as i32);
            crate::c::bf_write((sprite).wrapping_add(3), 6, 2, (3u32) as i32);
            CalcCenterToCornerVec(sprite, 1u8, 3u8, 2u8);
        } else {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .read(),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .read(),
            );
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .read(),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WaterDrop(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_WaterDrop_Slide));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WaterDrop_Slide(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) <= 116i32 {
            let __p1 = (sprite).wrapping_add(34).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(4i32)) as i16));
            ((sprite).wrapping_add(36).cast::<i16>()).write((-4i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(128i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_WaterDrop_ReachLeafEnd));
        } else {
            let mut data2: u16 = 0u16;
            let mut data3: u16 = 0u16;
            let mut data4: u16 = 0u16;
            let mut sin1: i16 = 0i16;
            let mut sin2: i16 = 0i16;
            let mut sin3: i16 = 0i16;
            let mut sin4: i16 = 0i16;
            let mut var1: i16 = 0i16;
            let mut var2: i16 = 0i16;
            let mut var3: i16 = 0i16;
            let mut var4: i16 = 0i16;
            let mut temp: i16 = 0i16;
            data4 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as u16);
            sin1 = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                .wrapping_offset((((data4) as u8) as i32) as isize))
            .read();
            sin2 = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                .wrapping_offset((((((data4) as i32).wrapping_add(64i32)) as u8) as i32) as isize))
            .read();
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p3).write((((((__p3).read()) as i32).wrapping_add(2i32)) as i16));
            ((sprite).wrapping_add(38).cast::<i16>())
                .write(((crate::c::div_i32(((sin1) as i32), 32i32)) as i16));
            let __p4 = (sprite).wrapping_add(32).cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_sub(1));
            if (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) & 1i32) != 0 {
                let __p5 = (sprite).wrapping_add(34).cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
            }
            temp = ((crate::c::div_i32(((sin2) as i32).wrapping_neg(), 16i32)) as i16);
            data2 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u16);
            data3 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u16);
            sin3 = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                .wrapping_offset((((((temp) as i32).wrapping_sub(16i32)) as u8) as i32) as isize))
            .read();
            sin4 = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                .wrapping_offset((((((temp) as i32).wrapping_add(48i32)) as u8) as i32) as isize))
            .read();
            var1 = ((crate::c::div_i32(((sin4) as i32).wrapping_mul(((data2) as i32)), 256i32))
                as i16);
            var2 = ((crate::c::div_i32(
                (((sin3) as i32).wrapping_neg()).wrapping_mul(((data3) as i32)),
                256i32,
            )) as i16);
            var3 = ((crate::c::div_i32(((sin3) as i32).wrapping_mul(((data2) as i32)), 256i32))
                as i16);
            var4 = ((crate::c::div_i32(((sin4) as i32).wrapping_mul(((data3) as i32)), 256i32))
                as i16);
            SetOamMatrix(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
                data2,
                0u16,
                0u16,
                data3,
            );
            SetOamMatrix(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    .wrapping_add(1i32)) as u8),
                ((var1) as u16),
                ((var3) as u16),
                ((var2) as u16),
                ((var4) as u16),
            );
            SetOamMatrix(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    .wrapping_add(2i32)) as u8),
                ((var1) as u16),
                ((var3) as u16),
                ((((var2) as i32).wrapping_mul(2i32)) as u16),
                ((((var4) as i32).wrapping_mul(2i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WaterDrop_ReachLeafEnd(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetOamMatrix(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                .wrapping_add(64i32)) as u16),
            0u16,
            0u16,
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                .wrapping_add(64i32)) as u16),
        );
        SetOamMatrix(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_add(1i32)) as u8),
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                .wrapping_add(64i32)) as u16),
            0u16,
            0u16,
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                .wrapping_add(64i32)) as u16),
        );
        SetOamMatrix(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_add(2i32)) as u8),
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                .wrapping_add(64i32)) as u16),
            0u16,
            0u16,
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                .wrapping_add(64i32)) as u16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            != 64i32
        {
            let mut sinIdx: u16 = 0u16;
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(8i32)) as i16));
            sinIdx =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as u16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((crate::c::div_i32(
                    ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        (((((sinIdx) as i32).wrapping_add(64i32)) as u8) as i32) as isize,
                    ))
                    .read()) as i32),
                    64i32,
                )) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((crate::c::div_i32(
                    ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                        .wrapping_offset((((sinIdx) as u8) as i32) as isize))
                    .read()) as i32),
                    64i32,
                )) as i16),
            );
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_WaterDrop_DangleFromLeaf));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WaterDrop_DangleFromLeaf(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 2i32 {
            let mut r2: i16 = 0i16;
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
            r2 = (((crate::c::div_i32(
                ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as u8) as i32) as isize,
                ))
                .read()) as i32),
                16i32,
            ))
            .wrapping_add(64i32)) as i16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((crate::c::div_i32(
                    ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        (((((r2) as i32).wrapping_add(64i32)) as u8) as i32) as isize,
                    ))
                    .read()) as i32),
                    64i32,
                )) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((crate::c::div_i32(
                    ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                        .wrapping_offset((((r2) as u8) as i32) as isize))
                    .read()) as i32),
                    64i32,
                )) as i16),
            );
        } else {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_WaterDrop_Fall));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WaterDrop_Fall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
        {
            let __p1 = (sprite).wrapping_add(34).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(1i16);
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p3 = (sprite).wrapping_add(34).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            StartSpriteAnim(sprite, 3u8);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1024i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                (((8i32).wrapping_mul(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        & 3i32),
                )) as i16),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_WaterDrop_Ripple));
            crate::c::bf_write((sprite).wrapping_add(1), 6, 2, (1u32) as i32);
            crate::c::bf_write((sprite).wrapping_add(3), 6, 2, (3u32) as i32);
            CalcCenterToCornerVec(sprite, 1u8, 3u8, 2u8);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WaterDropShort(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
        {
            let __p1 = (sprite).wrapping_add(34).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(1i16);
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p3 = (sprite).wrapping_add(34).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            StartSpriteAnim(sprite, 3u8);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1024i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                (((8i32).wrapping_mul(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        & 3i32),
                )) as i16),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_WaterDrop_Ripple));
            crate::c::bf_write((sprite).wrapping_add(1), 6, 2, (1u32) as i32);
            crate::c::bf_write((sprite).wrapping_add(3), 6, 2, (3u32) as i32);
            CalcCenterToCornerVec(sprite, 1u8, 3u8, 2u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateWaterDrop(
    x: i16,
    y: i16,
    c: u16,
    d: u16,
    e: u16,
    fallImmediately: u8,
) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut c = c;
        let mut d = d;
        let mut e = e;
        let mut fallImmediately = fallImmediately;
        let mut spriteId: u8 = 0u8;
        let mut oldSpriteId: u8 = 0u8;
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_WaterDrop)
                .cast::<u8>()
                .cast_mut(),
            x,
            y,
            1u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((d) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((c) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((c) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(((e) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((c) as i16));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            0,
            2,
            (3u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
            1,
            5,
            ((d) as u32) as i32,
        );
        CalcCenterToCornerVec(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            0u8,
            2u8,
            2u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            2u8,
        );
        if !((fallImmediately) != 0) {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_WaterDrop));
        } else {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_WaterDropShort));
        }
        oldSpriteId = spriteId;
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_WaterDrop)
                .cast::<u8>()
                .cast_mut(),
            x,
            y,
            1u8,
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((oldSpriteId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((((d) as i32).wrapping_add(1i32)) as i16));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            0,
            2,
            (3u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
            1,
            5,
            ((((d) as i32).wrapping_add(1i32)) as u32) as i32,
        );
        CalcCenterToCornerVec(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            0u8,
            2u8,
            2u8,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_WaterDropHalf));
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_WaterDrop)
                .cast::<u8>()
                .cast_mut(),
            x,
            y,
            1u8,
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((oldSpriteId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((((d) as i32).wrapping_add(2i32)) as i16));
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            1u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            0,
            2,
            (3u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
            1,
            5,
            ((((d) as i32).wrapping_add(2i32)) as u32) as i32,
        );
        CalcCenterToCornerVec(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            0u8,
            2u8,
            2u8,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_WaterDropHalf));
        SetOamMatrix(
            ((d) as u8),
            ((((c) as i32).wrapping_add(32i32)) as u16),
            0u16,
            0u16,
            ((((c) as i32).wrapping_add(32i32)) as u16),
        );
        SetOamMatrix(
            ((((d) as i32).wrapping_add(1i32)) as u8),
            ((((c) as i32).wrapping_add(32i32)) as u16),
            0u16,
            0u16,
            ((((c) as i32).wrapping_add(32i32)) as u16),
        );
        SetOamMatrix(
            ((((d) as i32).wrapping_add(2i32)) as u8),
            ((((c) as i32).wrapping_add(32i32)) as u16),
            0u16,
            0u16,
            (((2i32).wrapping_mul(((c) as i32).wrapping_add(32i32))) as u16),
        );
        return oldSpriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PlayerOnBicycle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                StartSpriteAnimIfDifferent(sprite, 0u8);
                let __p2 = (sprite).wrapping_add(32).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_sub(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                StartSpriteAnimIfDifferent(sprite, 0u8);
                if (((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() & 7u32) != 0 {
                    return;
                }
                let __p3 = (sprite).wrapping_add(32).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) <= 120i32)
                    || ((((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() & 7u32)
                        != 0)
                {
                    let __p4 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > (-32i32) {
                    let __p5 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p5).write((((((__p5).read()) as i32).wrapping_sub(2i32)) as i16));
                }
                break 'l1;
            }
        }
        if (((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() & 7u32) != 0 {
            return;
        }
        if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) != 0i32 {
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        } else {
            'l2: {
                let __sw6 = (((Random()) as i32) & 3i32);
                if __sw6 == 0i32 {
                    ((sprite).wrapping_add(38).cast::<i16>()).write((-1i16));
                    break 'l2;
                }
                if __sw6 == 1i32 {
                    ((sprite).wrapping_add(38).cast::<i16>()).write(1i16);
                    break 'l2;
                }
                if __sw6 == 2i32 || __sw6 == 3i32 {
                    ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                    break 'l2;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Flygon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32))
                    < 304i32
                {
                    let __p2 = (sprite).wrapping_add(36).cast::<i16>();
                    (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
                } else {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32))
                    > 120i32
                {
                    let __p3 = (sprite).wrapping_add(36).cast::<i16>();
                    (__p3).write((((((__p3).read()) as i32).wrapping_sub(1i32)) as i16));
                } else {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(3i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) > 0i32 {
                    let __p4 = (sprite).wrapping_add(36).cast::<i16>();
                    (__p4).write((((((__p4).read()) as i32).wrapping_sub(2i32)) as i16));
                }
                break 'l1;
            }
        }
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((Sin(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8)
                    as i16),
                8i16,
            )) as i32)
                .wrapping_sub(
                    ((((&raw mut sFlygonYOffset).cast::<u8>().cast::<u16>()).read()) as i32),
                )) as i16),
        );
        let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p5).write((((((__p5).read()) as i32).wrapping_add(4i32)) as i16));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_LogoLetter(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    != 0i32
                {
                    let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                } else {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    StartSpriteAffineAnim(sprite, 1u8);
                    let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 144u32 {
                    let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(9i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    == 0i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(2i16);
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        != 0i32
                    {
                        'l2: loop {
                            'l3: {
                                'l4: loop {
                                    'l5: {
                                        CpuSet(
                                            ((((&raw mut gIntroGameFreakTextFade_Pal)
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(
                                                ((((((sprite).wrapping_add(46)).cast::<i16>())
                                                    .wrapping_offset(1))
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .cast::<u8>(),
                                            ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(287))
                                            .cast::<u8>(),
                                            (0u32
                                                | (crate::c::div_u32(
                                                    2u32,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                ) & 2097151u32)),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l4;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l2;
                            }
                        }
                        'l6: loop {
                            'l7: {
                                'l8: loop {
                                    'l9: {
                                        CpuSet(
                                            ((((&raw mut gIntroGameFreakTextFade_Pal)
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(
                                                (((((((sprite).wrapping_add(46)).cast::<i16>())
                                                    .wrapping_offset(1))
                                                .read())
                                                    as i32)
                                                    .wrapping_add(16i32))
                                                    as isize,
                                            ))
                                            .cast::<u8>(),
                                            ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(276))
                                            .cast::<u8>(),
                                            (0u32
                                                | (crate::c::div_u32(
                                                    2u32,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                ) & 2097151u32)),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l8;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l6;
                            }
                        }
                        'l10: loop {
                            'l11: {
                                'l12: loop {
                                    'l13: {
                                        CpuSet(
                                            ((((&raw mut gIntroGameFreakTextFade_Pal)
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(
                                                (((((((sprite).wrapping_add(46)).cast::<i16>())
                                                    .wrapping_offset(1))
                                                .read())
                                                    as i32)
                                                    .wrapping_add(32i32))
                                                    as isize,
                                            ))
                                            .cast::<u8>(),
                                            ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(282))
                                            .cast::<u8>(),
                                            (0u32
                                                | (crate::c::div_u32(
                                                    2u32,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                ) & 2097151u32)),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l12;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l10;
                            }
                        }
                        let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                        (__p5).write(((__p5).read()).wrapping_sub(1));
                    } else {
                        'l14: loop {
                            'l15: {
                                'l16: loop {
                                    'l17: {
                                        CpuSet(
                                            ((((&raw mut gIntroGameFreakTextFade_Pal)
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(
                                                ((((((sprite).wrapping_add(46)).cast::<i16>())
                                                    .wrapping_offset(1))
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .cast::<u8>(),
                                            ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(287))
                                            .cast::<u8>(),
                                            (0u32
                                                | (crate::c::div_u32(
                                                    2u32,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                ) & 2097151u32)),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l16;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l14;
                            }
                        }
                        'l18: loop {
                            'l19: {
                                'l20: loop {
                                    'l21: {
                                        CpuSet(
                                            ((((&raw mut gIntroGameFreakTextFade_Pal)
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(
                                                (((((((sprite).wrapping_add(46)).cast::<i16>())
                                                    .wrapping_offset(1))
                                                .read())
                                                    as i32)
                                                    .wrapping_add(16i32))
                                                    as isize,
                                            ))
                                            .cast::<u8>(),
                                            ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(276))
                                            .cast::<u8>(),
                                            (0u32
                                                | (crate::c::div_u32(
                                                    2u32,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                ) & 2097151u32)),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l20;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l18;
                            }
                        }
                        'l22: loop {
                            'l23: {
                                'l24: loop {
                                    'l25: {
                                        CpuSet(
                                            ((((&raw mut gIntroGameFreakTextFade_Pal)
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(
                                                (((((((sprite).wrapping_add(46)).cast::<i16>())
                                                    .wrapping_offset(1))
                                                .read())
                                                    as i32)
                                                    .wrapping_add(32i32))
                                                    as isize,
                                            ))
                                            .cast::<u8>(),
                                            ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(282))
                                            .cast::<u8>(),
                                            (0u32
                                                | (crate::c::div_u32(
                                                    2u32,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                ) & 2097151u32)),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l24;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l22;
                            }
                        }
                        let __p6 = ((sprite).wrapping_add(46)).cast::<i16>();
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                } else {
                    let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p7).write(((__p7).read()).wrapping_sub(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    != 0i32
                {
                    let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p8).write(((__p8).read()).wrapping_sub(1));
                } else {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(2i16);
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        <= 9i32
                    {
                        'l26: loop {
                            'l27: {
                                'l28: loop {
                                    'l29: {
                                        CpuSet(
                                            ((((&raw mut gIntroGameFreakTextFade_Pal)
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(
                                                ((((((sprite).wrapping_add(46)).cast::<i16>())
                                                    .wrapping_offset(1))
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .cast::<u8>(),
                                            ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(287))
                                            .cast::<u8>(),
                                            (0u32
                                                | (crate::c::div_u32(
                                                    2u32,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                ) & 2097151u32)),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l28;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l26;
                            }
                        }
                        'l30: loop {
                            'l31: {
                                'l32: loop {
                                    'l33: {
                                        CpuSet(
                                            ((((&raw mut gIntroGameFreakTextFade_Pal)
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(
                                                (((((((sprite).wrapping_add(46)).cast::<i16>())
                                                    .wrapping_offset(1))
                                                .read())
                                                    as i32)
                                                    .wrapping_add(16i32))
                                                    as isize,
                                            ))
                                            .cast::<u8>(),
                                            ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(276))
                                            .cast::<u8>(),
                                            (0u32
                                                | (crate::c::div_u32(
                                                    2u32,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                ) & 2097151u32)),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l32;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l30;
                            }
                        }
                        'l34: loop {
                            'l35: {
                                'l36: loop {
                                    'l37: {
                                        CpuSet(
                                            ((((&raw mut gIntroGameFreakTextFade_Pal)
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(
                                                (((((((sprite).wrapping_add(46)).cast::<i16>())
                                                    .wrapping_offset(1))
                                                .read())
                                                    as i32)
                                                    .wrapping_add(32i32))
                                                    as isize,
                                            ))
                                            .cast::<u8>(),
                                            ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(282))
                                            .cast::<u8>(),
                                            (0u32
                                                | (crate::c::div_u32(
                                                    2u32,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                ) & 2097151u32)),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l36;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l34;
                            }
                        }
                        let __p9 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                        (__p9).write(((__p9).read()).wrapping_add(1));
                    } else {
                        let __p10 = ((sprite).wrapping_add(46)).cast::<i16>();
                        (__p10).write(((__p10).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 272u32 {
                    StartSpriteAffineAnim(sprite, 2u8);
                    crate::c::bf_write((sprite).wrapping_add(1), 2, 2, (1u32) as i32);
                    let __p11 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                let __p12 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p12).write(
                    (((((__p12).read()) as i32).wrapping_add(
                        ((((((&raw const sGameFreakLettersMoveSpeed)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                .read()) as i32) as isize,
                        ))
                        .read()) as i32),
                    )) as i16),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        & 65280i32)
                        >> 8) as i16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    < 4i32
                {
                    let mut temp: i16 = ((sprite).wrapping_add(36).cast::<i16>()).read();
                    ((sprite).wrapping_add(36).cast::<i16>())
                        .write(((((temp) as i32).wrapping_neg()) as i16));
                }
                if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
                    DestroySprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_GameFreakLogo(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 128u32 {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((&raw mut gIntroFrameCounter).cast::<u8>().cast::<u32>()).read() == 272u32 {
                    StartSpriteAffineAnim(sprite, 3u8);
                    let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
                    DestroySprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateGameFreakLogoSprites(x: i16, y: i16, unused: i16) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut unused = unused;
        let mut i: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 9i32) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (&raw const sSpriteTemplate_GameFreakLetter)
                            .cast::<u8>()
                            .cast_mut(),
                        ((((((((((&raw const sGameFreakLetterData).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            .wrapping_add(((x) as i32))) as i16),
                        ((((y) as i32).wrapping_sub(4i32)) as i16),
                        0u8,
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
                    .write(
                        ((((((&raw const sGameFreakLetterStartDelays)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i16),
                    );
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(((i) as i16));
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(3),
                        1,
                        5,
                        ((((i) as i32).wrapping_add(12i32)) as u32) as i32,
                    );
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        (((((((&raw const sGameFreakLetterData).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<i16>())
                        .read()) as u8),
                    );
                    StartSpriteAffineAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        0u8,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_GameFreakLogo)
                .cast::<u8>()
                .cast_mut(),
            120i16,
            ((((y) as i32).wrapping_sub(6i32)) as i16),
            0u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(0i16);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
            1,
            5,
            ((((i) as i32).wrapping_add(12i32)) as u32) as i32,
        );
        StartSpriteAffineAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            1u8,
        );
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FlygonSilhouette(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
            let mut sin: i16 = 0i16;
            let mut cos: i16 = 0i16;
            let mut a: i16 = 0i16;
            let mut b: i16 = 0i16;
            let mut c: i16 = 0i16;
            let mut d: i16 = 0i16;
            sin = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8)
                    as i32) as isize,
            ))
            .read();
            cos = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    .wrapping_add(64i32)) as u8) as i32) as isize,
            ))
            .read();
            d = ((crate::c::div_i32(
                ((cos) as i32).wrapping_mul(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                ),
                256i32,
            )) as i16);
            c = ((crate::c::div_i32(
                (((sin) as i32).wrapping_neg()).wrapping_mul(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                ),
                256i32,
            )) as i16);
            b = ((crate::c::div_i32(
                ((sin) as i32).wrapping_mul(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                ),
                256i32,
            )) as i16);
            a = ((crate::c::div_i32(
                ((cos) as i32).wrapping_mul(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                ),
                256i32,
            )) as i16);
            SetOamMatrix(1u8, ((a) as u16), ((b) as u16), ((c) as u16), ((d) as u16));
        }
        'l1: {
            let __sw2 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let __matched = __sw2 == 0i32 || __sw2 == 1i32 || __sw2 == 2i32;
            if __sw2 == 0i32 || !__matched {
                crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (3u32) as i32);
                crate::c::bf_write((sprite).wrapping_add(3), 1, 5, (1u32) as i32);
                CalcCenterToCornerVec(sprite, 1u8, 3u8, 3u8);
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(128i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                break 'l1;
            }
            if __sw2 == 1i32 {
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    ((((Sin(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as u8) as i16),
                        140i16,
                    )) as i32)
                        .wrapping_neg()) as i16),
                );
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    ((((Sin(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as u8) as i16),
                        120i16,
                    )) as i32)
                        .wrapping_neg()) as i16),
                );
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(7i32)) as i16));
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p4).write((((((__p4).read()) as i32).wrapping_add(3i32)) as i16));
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32))
                    <= (-16i32)
                {
                    crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (3u16) as i32);
                    let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    ((sprite).wrapping_add(32).cast::<i16>()).write(20i16);
                    ((sprite).wrapping_add(34).cast::<i16>()).write(40i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(512i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(16i16);
                }
                break 'l1;
            }
            if __sw2 == 2i32 {
                ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as u8) as i16),
                    34i16,
                ));
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    ((((Cos(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as u8) as i16),
                        60i16,
                    )) as i32)
                        .wrapping_neg()) as i16),
                );
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p6).write((((((__p6).read()) as i32).wrapping_add(2i32)) as i16));
                if crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32),
                    5i32,
                ) == 0i32
                {
                    let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_RayquazaOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut foo: u16 = 0u16;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            let mut __fall = false;
            if __sw1 == 0i32 || !__matched {
                __fall = true;
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (3u32) as i32);
                crate::c::bf_write((sprite).wrapping_add(3), 1, 5, (18u32) as i32);
                CalcCenterToCornerVec(sprite, 0u8, 3u8, 3u8);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                (__p2).write(((__p2).read()).wrapping_add(1));
                if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    & 1i32)
                    != 0
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                } else {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        < 64i32
                    {
                        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                        (__p3).write(((__p3).read()).wrapping_add(1));
                    }
                }
                foo = (((256i32).wrapping_sub(crate::c::div_i32(
                    ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as u8) as i32) as isize,
                    ))
                    .read()) as i32),
                    2i32,
                ))) as u16);
                SetOamMatrix(18u8, foo, 0u16, 0u16, foo);
                break 'l1;
            }
        }
    }
}
