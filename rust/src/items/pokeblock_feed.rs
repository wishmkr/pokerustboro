//! Translated from `src/pokeblock_feed.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sNatureToMonPokeblockAnim sMonPokeblockAnims sAffineAnim_Mon_None sAffineAnim_Mon_TurnUp sAffineAnim_Mon_TurnUp_Flipped sAffineAnim_Mon_TurnUpAndDown sAffineAnim_Mon_TurnUpAndDown_Flipped sAffineAnim_Mon_TurnDown sAffineAnim_Mon_TurnDown_Flipped sAffineAnim_Mon_TurnDownSlow sAffineAnim_Mon_TurnDownSlow_Flipped sAffineAnim_Mon_TurnDownSlight sAffineAnim_Mon_TurnDownSlight_Flipped sAffineAnim_Mon_TurnUpHigh sAffineAnim_Mon_TurnUpHigh_Flipped sAffineAnims_Mon sBackgroundTemplates sWindowTemplates sPokeblocksPals sAffineAnim_Still sSpriteAffineAnimTable_MonNoFlip sAffineAnim_PokeblockCase_ThrowFromVertical sAffineAnim_PokeblockCase_ThrowFromHorizontal sAffineAnims_PokeblockCase_Still sAffineAnims_PokeblockCase_ThrowFromVertical sAffineAnims_PokeblockCase_ThrowFromHorizontal sOamData_Pokeblock sAnim_Pokeblock sAnims_Pokeblock sAffineAnim_Pokeblock sAffineAnims_Pokeblock sSpriteSheet_Pokeblock sSpriteTemplate_Pokeblock
#[allow(unused_imports)]
use crate::data::pokeblock_feed::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokeblockFeed: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokeblockSpritePal: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);

unsafe extern "C" {
    static mut gBattleEnvironmentPalette_Frontier: u8;
    static mut gBattleEnvironmentTiles_Building: u8;
    static mut gMPlayInfo_BGM: u8;
    static mut gMain: u8;
    static mut gMonFrontPicTable: u8;
    static mut gMonSpritesGfxPtr: u8;
    static mut gMultiuseSpriteTemplate: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPokeblockCase_SpritePal: u8;
    static mut gPokeblockCase_SpriteSheet: u8;
    static mut gPokeblockFeedBg_Tilemap: u8;
    static mut gPokeblockGain: u8;
    static mut gPokeblockMonId: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSpecialVar_ItemId: u8;
    static mut gSprites: u8;
    static mut gStandardMenuPalette: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gTextFlags: u8;
    static mut gText_Var1AteTheVar2: u8;
    static mut gText_Var1DisdainfullyAteVar2: u8;
    static mut gText_Var1HappilyAteVar2: u8;
    fn AddTextPrinterParameterized2(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn(*mut u8, u16)>,
        a5: u8,
        a6: u8,
        a7: u8,
    ) -> u16;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AllocateMonSpritesGfx();
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CalcCenterToCornerVec(a0: *mut u8, a1: u8, a2: u8, a3: u8);
    fn ClearScheduledBgCopiesToVram();
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreatePokeblockCaseSprite(a0: i16, a1: i16, a2: u8) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DoScheduledBgTilemapCopiesToVram();
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeMonSpritesGfx();
    fn FreeOamMatrix(a0: u8);
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonNickname(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn GetMonSpritePalStructFromOtIdPersonality(a0: u16, a1: u32, a2: u32) -> *mut u8;
    fn GetNature(a0: *mut u8) -> u8;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetPokeblockData(a0: *mut u8, a1: u8) -> i16;
    fn HandleLoadSpecialPokePic_2(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32);
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitSpriteAffineAnim(a0: *mut u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsMonSpriteNotFlipped(a0: u16) -> u8;
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn MenuHelpers_IsLinkActive() -> u8;
    fn MenuHelpers_ShouldWaitForLinkRecv() -> u8;
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PokeblockCopyName(a0: *mut u8, a1: *mut u8);
    fn PokeblockGetGain(a0: u8, a1: *mut u8) -> i16;
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ResetAllBgsCoordinates();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTempTileDataBuffers();
    fn ResetVramOamAndBgCntRegs();
    fn RunTasks();
    fn RunTextPrintersRetIsActive(a0: u8) -> u16;
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankHBlankCallbacksToNull();
    fn ShowBg(a0: u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn m4aMPlayVolumeControl(a0: *mut u8, a1: u16, a2: u16);
}

pub(crate) unsafe extern "C" fn CB2_PokeblockFeed() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_PokeblockFeed() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn LoadPokeblockFeedScene() -> u8 {
    unsafe {
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
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 13i32;
            if __sw1 == 0i32 {
                ((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>())
                    .write(AllocZeroed(4228u32));
                SetVBlankHBlankCallbacksToNull();
                ClearScheduledBgCopiesToVram();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ResetPaletteFade();
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (1u16) as i32,
                );
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetSpriteData();
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                FreeAllSpritePalettes();
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                AllocateMonSpritesGfx();
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                HandleInitBackgrounds();
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                HandleInitWindows();
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (LoadMonAndSceneGfx(((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gPokeblockMonId).cast::<u8>()).read()) as i32) as isize * 100,
                ))) != 0
                {
                    let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                ((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4190))
                .write(CreatePokeblockCaseSpriteForFeeding());
                let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                ((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4189))
                .write(CreateMonSprite(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gPokeblockMonId).cast::<u8>()).read()) as i32) as isize * 100,
                    ),
                ));
                let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                DrawStdFrameWithCustomTileAndPalette(0u8, 1u8, 1u16, 14u8);
                let __p12 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                LaunchPokeblockFeedTask();
                let __p13 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                BlendPalettes(4294967295u32, 16u8, 0u16);
                let __p14 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p14).write(((__p14).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 13i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                let __p15 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p15).write(((__p15).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                SetVBlankCallback(Some(VBlankCB_PokeblockFeed));
                SetMainCallback2(Some(CB2_PokeblockFeed));
                return 1u8;
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PreparePokeblockFeedScene() {
    unsafe {
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            if ((MenuHelpers_ShouldWaitForLinkRecv()) as i32) == 1i32 {
                break 'l1;
            }
            if ((LoadPokeblockFeedScene()) as i32) == 1i32 {
                break 'l1;
            }
            if ((MenuHelpers_IsLinkActive()) as i32) == 1i32 {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HandleInitBackgrounds() {
    unsafe {
        ResetVramOamAndBgCntRegs();
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBackgroundTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(8u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(
            1u8,
            ((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(72))
                .cast::<u8>(),
        );
        ResetAllBgsCoordinates();
        ScheduleBgCopyTilemapToVram(1u8);
        SetGpuReg(0u8, 4160u16);
        ShowBg(0u8);
        ShowBg(1u8);
        SetGpuReg(80u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn LoadMonAndSceneGfx(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut species: u16 = 0u16;
        let mut personality: u32 = 0u32;
        let mut trainerId: u32 = 0u32;
        let mut palette: *mut u8 = core::ptr::null_mut();
        'l1: {
            let __sw1 = ((((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4222)
                .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                species = ((GetMonData2(mon, 65i32)) as u16);
                personality = GetMonData2(mon, 0i32);
                HandleLoadSpecialPokePic_2(
                    ((&raw mut gMonFrontPicTable).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 8),
                    ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .cast::<*mut u8>())
                    .wrapping_offset(1))
                    .read(),
                    ((species) as i32),
                    personality,
                );
                let __p2 = (((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4222)
                    .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                species = ((GetMonData2(mon, 65i32)) as u16);
                personality = GetMonData2(mon, 0i32);
                trainerId = GetMonData2(mon, 1i32);
                palette = GetMonSpritePalStructFromOtIdPersonality(species, trainerId, personality);
                LoadCompressedSpritePalette(palette);
                SetMultiuseSpriteTemplateToPokemon(
                    ((palette).wrapping_add(4).cast::<u16>()).read(),
                    1u8,
                );
                let __p3 = (((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4222)
                    .cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                LoadCompressedSpriteSheet((&raw mut gPokeblockCase_SpriteSheet).cast::<u8>());
                let __p4 = (((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4222)
                    .cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                LoadCompressedSpritePalette((&raw mut gPokeblockCase_SpritePal).cast::<u8>());
                let __p5 = (((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4222)
                    .cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                LoadCompressedSpriteSheet(
                    (&raw const sSpriteSheet_Pokeblock).cast::<u8>().cast_mut(),
                );
                let __p6 = (((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4222)
                    .cast::<i16>();
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                SetPokeblockSpritePal(
                    ((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as u8),
                );
                LoadCompressedSpritePalette((&raw mut sPokeblockSpritePal).cast::<u8>());
                let __p7 = (((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4222)
                    .cast::<i16>();
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                ResetTempTileDataBuffers();
                DecompressAndCopyTileDataToVram(
                    1u8,
                    (((&raw mut gBattleEnvironmentTiles_Building).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                let __p8 = (((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4222)
                    .cast::<i16>();
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                if ((FreeTempTileDataBuffersIfPossible()) as i32) != 1i32 {
                    LZDecompressWram(
                        ((&raw mut gPokeblockFeedBg_Tilemap).cast::<u32>()).cast::<u32>(),
                        ((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(72))
                        .cast::<u8>(),
                    );
                    let __p9 = (((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4222)
                        .cast::<i16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                LoadCompressedPalette(
                    ((&raw mut gBattleEnvironmentPalette_Frontier).cast::<u32>()).cast::<u32>(),
                    32u16,
                    96u16,
                );
                ((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4222)
                    .cast::<i16>())
                .write(0i16);
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn HandleInitWindows() {
    unsafe {
        InitWindows(((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>());
        DeactivateAllTextPrinters();
        LoadUserWindowBorderGfx(0u8, 1u16, 224u8);
        LoadPalette(
            (((&raw mut gStandardMenuPalette).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            240u16,
            32u16,
        );
        FillWindowPixelBuffer(0u8, 0u8);
        PutWindowTilemap(0u8);
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn SetPokeblockSpritePal(pokeblockCaseId: u8) {
    unsafe {
        let mut pokeblockCaseId = pokeblockCaseId;
        let mut colorId: u8 = ((GetPokeblockData(
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2120))
                .cast::<u8>())
            .wrapping_offset(((pokeblockCaseId) as i32) as isize * 8),
            0u8,
        )) as u8);
        (((&raw mut sPokeblockSpritePal).cast::<u8>()).cast::<*mut u32>()).write(
            ((((&raw const sPokeblocksPals)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u32>())
            .cast::<*mut u32>())
            .wrapping_offset((((colorId) as i32).wrapping_sub(1i32)) as isize))
            .read(),
        );
        (((&raw mut sPokeblockSpritePal).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(14818u16);
    }
}
pub(crate) unsafe extern "C" fn Task_HandlePokeblockFeed(taskId: u8) {
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
                let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32);
                if __sw1 == 0i32 {
                    ((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4176))
                    .write(0u8);
                    ((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4184)
                        .cast::<u16>())
                    .write(0u16);
                    CalculateMonAnimLength();
                    break 'l1;
                }
                if __sw1 == 255i32 {
                    DoPokeblockCaseThrowEffect(
                        ((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4190))
                        .read(),
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as u8),
                    );
                    break 'l1;
                }
                if __sw1 == 269i32 {
                    ((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4191))
                    .write(CreatePokeblockSprite());
                    break 'l1;
                }
                if __sw1 == 281i32 {
                    StartMonJumpForPokeblock(
                        ((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4189))
                        .read(),
                    );
                    break 'l1;
                }
                if __sw1 == 297i32 {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_PrintAtePokeblockMessage));
                    return;
                }
            }
            if ((((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4184)
                .cast::<u16>())
            .read()) as i32)
                < ((((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4182)
                    .cast::<u16>())
                .read()) as i32)
            {
                UpdateMonAnim();
            } else {
                if ((((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4184)
                    .cast::<u16>())
                .read()) as i32)
                    == ((((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4182)
                        .cast::<u16>())
                    .read()) as i32)
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(254i16);
                }
            }
            let __p2 = (((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4184)
                .cast::<u16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
            let __p3 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn LaunchPokeblockFeedTask() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_HandlePokeblockFeed), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(1i16);
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForAtePokeblockMessage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((RunTextPrintersRetIsActive(0u8)) as i32) != 1i32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_FadeOutPokeblockFeed));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PrintAtePokeblockMessage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            ((((&raw mut gPokeblockMonId).cast::<u8>()).read()) as i32) as isize * 100,
        );
        let mut pokeblock: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(2120))
        .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as i32) as isize * 8,
        );
        ((&raw mut gPokeblockGain).cast::<i16>())
            .write(PokeblockGetGain(GetNature(mon), pokeblock));
        GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
        PokeblockCopyName(pokeblock, (&raw mut gStringVar2).cast::<u8>());
        if ((((&raw mut gPokeblockGain).cast::<i16>()).read()) as i32) == 0i32 {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_Var1AteTheVar2).cast::<u8>(),
            );
        } else {
            if ((((&raw mut gPokeblockGain).cast::<i16>()).read()) as i32) > 0i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_Var1HappilyAteVar2).cast::<u8>(),
                );
            } else {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_Var1DisdainfullyAteVar2).cast::<u8>(),
                );
            }
        }
        crate::c::bf_write(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            0,
            1,
            (1u8) as i32,
        );
        AddTextPrinterParameterized2(
            0u8,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            GetPlayerTextSpeedDelay(),
            None,
            2u8,
            1u8,
            3u8,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_WaitForAtePokeblockMessage));
    }
}
pub(crate) unsafe extern "C" fn Task_ExitPokeblockFeed(taskId: u8) {
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
            ResetSpriteData();
            FreeAllSpritePalettes();
            m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 256u16);
            SetMainCallback2(
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            );
            DestroyTask(taskId);
            FreeAllWindowBuffers();
            Free(((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read());
            FreeMonSpritesGfx();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_FadeOutPokeblockFeed(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ExitPokeblockFeed));
    }
}
pub(crate) unsafe extern "C" fn CreateMonSprite(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut species: u16 = ((GetMonData2(mon, 65i32)) as u16);
        let mut spriteId: u8 = CreateSprite(
            (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
            48i16,
            80i16,
            2u8,
        );
        ((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4180)
            .cast::<u16>())
        .write(species);
        ((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4187))
            .write(spriteId);
        ((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4186))
            .write(GetNature(mon));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((species) as i16));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        ((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4179))
            .write(1u8);
        if !((IsMonSpriteNotFlipped(species)) != 0) {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(16)
            .cast::<*mut *mut u8>())
            .write(
                ((&raw const sSpriteAffineAnimTable_MonNoFlip)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                0,
                2,
                (3u32) as i32,
            );
            CalcCenterToCornerVec(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                ((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(1),
                    6,
                    2,
                    false,
                ) as u32) as u8),
                ((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(3),
                    6,
                    2,
                    false,
                ) as u32) as u8),
                ((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(1),
                    0,
                    2,
                    false,
                ) as u32) as u8),
            );
            ((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4179))
            .write(0u8);
        }
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn StartMonJumpForPokeblock(spriteId: u8) {
    unsafe {
        let mut spriteId = spriteId;
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
        .write(48i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
        .write(80i16);
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write((-8i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(1i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_MonJumpForPokeblock));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonJumpForPokeblock(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32)
                .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            PlayCry_Normal(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u16),
                0i8,
            );
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 9i32 {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn CreatePokeblockCaseSpriteForFeeding() -> u8 {
    unsafe {
        let mut spriteId: u8 = CreatePokeblockCaseSprite(188i16, 100i16, 2u8);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            0,
            2,
            (1u32) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(16)
            .cast::<*mut *mut u8>())
        .write(
            ((&raw const sAffineAnims_PokeblockCase_Still)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>(),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        InitSpriteAffineAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn DoPokeblockCaseThrowEffect(spriteId: u8, horizontalThrow: u8) {
    unsafe {
        let mut spriteId = spriteId;
        let mut horizontalThrow = horizontalThrow;
        FreeOamMatrix(
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
                1,
                5,
                false,
            ) as u32) as u8),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            0,
            2,
            (3u32) as i32,
        );
        if !((horizontalThrow) != 0) {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(16)
            .cast::<*mut *mut u8>())
            .write(
                ((&raw const sAffineAnims_PokeblockCase_ThrowFromVertical)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
        } else {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(16)
            .cast::<*mut *mut u8>())
            .write(
                ((&raw const sAffineAnims_PokeblockCase_ThrowFromHorizontal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
        }
        InitSpriteAffineAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
    }
}
pub(crate) unsafe extern "C" fn CreatePokeblockSprite() -> u8 {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_Pokeblock)
                .cast::<u8>()
                .cast_mut(),
            174i16,
            84i16,
            1u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write((-12i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(1i16);
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ThrownPokeblock(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(4i32)) as i16));
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32)
                .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 10i32 {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn CalculateMonAnimLength() {
    unsafe {
        let mut animId: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut pokeblockFeed: *mut u8 = core::ptr::null_mut();
        pokeblockFeed = ((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read();
        ((pokeblockFeed).wrapping_add(4182).cast::<u16>()).write(1u16);
        animId = (((((&raw const sNatureToMonPokeblockAnim)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(((((pokeblockFeed).wrapping_add(4186)).read()) as i32) as isize * 2))
        .cast::<u8>())
        .read();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    let __p1 = (pokeblockFeed).wrapping_add(4182).cast::<u16>();
                    (__p1).write(
                        (((((__p1).read()) as i32).wrapping_add(
                            ((((((((&raw const sMonPokeblockAnims).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((animId) as i32) as isize * 20))
                            .cast::<i16>())
                            .wrapping_offset(4))
                            .read()) as i32),
                        )) as u16),
                    );
                    if ((((((((&raw const sMonPokeblockAnims).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((animId) as i32) as isize * 20))
                    .cast::<i16>())
                    .wrapping_offset(9))
                    .read()) as i32)
                        == 1i32
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
                animId = (animId).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateMonAnim() {
    unsafe {
        let mut pokeblockFeed: *mut u8 =
            ((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read();
        'l1: {
            let __sw1 = ((((pokeblockFeed).wrapping_add(4176)).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((pokeblockFeed).wrapping_add(4177)).write(
                    (((((&raw const sNatureToMonPokeblockAnim)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((pokeblockFeed).wrapping_add(4186)).read()) as i32) as isize * 2,
                    ))
                    .cast::<u8>())
                    .read(),
                );
                ((pokeblockFeed).cast::<*mut u8>()).write(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((pokeblockFeed).wrapping_add(4187)).read()) as i32) as isize * 68,
                    ),
                );
                (pokeblockFeed)
                    .wrapping_add(4)
                    .cast::<crate::c::Rec4<68>>()
                    .write_unaligned(
                        ((pokeblockFeed).cast::<*mut u8>())
                            .read()
                            .cast::<crate::c::Rec4<68>>()
                            .read_unaligned(),
                    );
                ((pokeblockFeed).wrapping_add(4176)).write(10u8);
                break 'l1;
            }
            if (1i32..=9i32).contains(&__sw1) {
                __fall = true;
                break 'l1;
            }
            if __sw1 == 10i32 {
                __fall = true;
                InitMonAnimStage();
                if ((((((((&raw const sNatureToMonPokeblockAnim)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(
                    ((((pokeblockFeed).wrapping_add(4186)).read()) as i32) as isize * 2,
                ))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    != 0i32
                {
                    crate::c::bf_write(
                        (((pokeblockFeed).cast::<*mut u8>()).read()).wrapping_add(1),
                        0,
                        2,
                        (3u32) as i32,
                    );
                    crate::c::bf_write(
                        (((pokeblockFeed).cast::<*mut u8>()).read()).wrapping_add(3),
                        1,
                        5,
                        (0u32) as i32,
                    );
                    ((((pokeblockFeed).cast::<*mut u8>()).read())
                        .wrapping_add(16)
                        .cast::<*mut *mut u8>())
                    .write(
                        ((&raw const sAffineAnims_Mon)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>(),
                    );
                    InitSpriteAffineAnim(((pokeblockFeed).cast::<*mut u8>()).read());
                }
                ((pokeblockFeed).wrapping_add(4176)).write(50u8);
            }
            if __fall || __sw1 == 50i32 {
                __fall = true;
                if ((((((((&raw const sNatureToMonPokeblockAnim)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(
                    ((((pokeblockFeed).wrapping_add(4186)).read()) as i32) as isize * 2,
                ))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    != 0i32
                {
                    if !((((pokeblockFeed).wrapping_add(4179)).read()) != 0) {
                        StartSpriteAffineAnim(
                            ((pokeblockFeed).cast::<*mut u8>()).read(),
                            ((((((((((&raw const sNatureToMonPokeblockAnim)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((pokeblockFeed).wrapping_add(4186)).read()) as i32) as isize * 2,
                            ))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                .wrapping_add(10i32)) as u8),
                        );
                    } else {
                        StartSpriteAffineAnim(
                            ((pokeblockFeed).cast::<*mut u8>()).read(),
                            ((((((&raw const sNatureToMonPokeblockAnim)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((pokeblockFeed).wrapping_add(4186)).read()) as i32) as isize * 2,
                            ))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read(),
                        );
                    }
                }
                ((pokeblockFeed).wrapping_add(4176)).write(60u8);
                break 'l1;
            }
            if __sw1 == 60i32 {
                __fall = true;
                if ((DoMonAnimStep()) as i32) == 1i32 {
                    if !((((((pokeblockFeed).wrapping_add(4192)).cast::<i16>())
                        .wrapping_offset(9))
                    .read())
                        != 0)
                    {
                        let __p2 = (pokeblockFeed).wrapping_add(4177);
                        (__p2).write(((__p2).read()).wrapping_add(1));
                        InitMonAnimStage();
                        ((pokeblockFeed).wrapping_add(4176)).write(60u8);
                    } else {
                        FreeOamMatrix(
                            ((crate::c::bf_read(
                                (((pokeblockFeed).cast::<*mut u8>()).read()).wrapping_add(3),
                                1,
                                5,
                                false,
                            ) as u32) as u8),
                        );
                        ((pokeblockFeed).wrapping_add(4176)).write(70u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 70i32 {
                __fall = true;
                FreeMonSpriteOamMatrix();
                ((pokeblockFeed).wrapping_add(4177)).write(0u8);
                ((pokeblockFeed).wrapping_add(4176)).write(0u8);
                break 'l1;
            }
            if (71i32..=90i32).contains(&__sw1) {
                __fall = true;
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitMonAnimStage() -> u8 {
    unsafe {
        let mut pokeblockFeed: *mut u8 =
            ((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read();
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    ((((pokeblockFeed).wrapping_add(4192)).cast::<i16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((&raw const sMonPokeblockAnims).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((pokeblockFeed).wrapping_add(4177)).read()) as i32) as isize * 20,
                        ))
                        .cast::<i16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).wrapping_offset(4)).read())
            as i32)
            == 0i32
        {
            return 1u8;
        } else {
            ((pokeblockFeed).wrapping_add(4212).cast::<i16>()).write(Sin(
                (((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).read(),
                ((((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).wrapping_offset(2)).read(),
            ));
            ((pokeblockFeed).wrapping_add(4214).cast::<i16>()).write(Cos(
                (((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).read(),
                ((((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).wrapping_offset(3)).read(),
            ));
            ((pokeblockFeed).wrapping_add(4216).cast::<i16>()).write(
                ((((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).wrapping_offset(4)).read(),
            );
            ((pokeblockFeed).wrapping_add(4218).cast::<i16>()).write(
                ((((pokeblockFeed).cast::<*mut u8>()).read())
                    .wrapping_add(36)
                    .cast::<i16>())
                .read(),
            );
            ((pokeblockFeed).wrapping_add(4220).cast::<i16>()).write(
                ((((pokeblockFeed).cast::<*mut u8>()).read())
                    .wrapping_add(38)
                    .cast::<i16>())
                .read(),
            );
            CalculateMonAnimMovement();
            ((((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).wrapping_offset(4))
                .write(((pokeblockFeed).wrapping_add(4216).cast::<i16>()).read());
            CalculateMonAnimMovementEnd();
            ((((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).wrapping_offset(4))
                .write(((pokeblockFeed).wrapping_add(4216).cast::<i16>()).read());
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn DoMonAnimStep() -> u8 {
    unsafe {
        let mut time: u16 = ((((((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(4216)
        .cast::<i16>())
        .read()) as i32)
            .wrapping_sub(
                ((((((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4192))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as i32),
            )) as u16);
        ((((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
            .read())
        .wrapping_add(36)
        .cast::<i16>())
        .write(
            ((((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2128))
            .cast::<i16>())
            .wrapping_offset(((time) as i32) as isize))
            .read(),
        );
        ((((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
            .read())
        .wrapping_add(38)
        .cast::<i16>())
        .write(
            ((((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3152))
            .cast::<i16>())
            .wrapping_offset(((time) as i32) as isize))
            .read(),
        );
        if (({
            let __p1 = (((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4192))
            .cast::<i16>())
            .wrapping_offset(4);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn FreeMonSpriteOamMatrix() -> u8 {
    unsafe {
        FreeSpriteOamMatrix(
            ((((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read(),
        );
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CalculateMonAnimMovementEnd() {
    unsafe {
        let mut pokeblockFeed: *mut u8 =
            ((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read();
        let mut i: u16 = 0u16;
        let mut approachTime: u16 = ((((((pokeblockFeed).wrapping_add(4192)).cast::<i16>())
            .wrapping_offset(8))
        .read()) as u16);
        let mut time: u16 = ((((((pokeblockFeed).wrapping_add(4216).cast::<i16>()).read()) as i32)
            .wrapping_sub(((approachTime) as i32))) as u16);
        let mut x: i16 = ((((((pokeblockFeed).wrapping_add(4218).cast::<i16>()).read()) as i32)
            .wrapping_add(
                ((((((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).wrapping_offset(6)).read())
                    as i32),
            )) as i16);
        let mut y: i16 = ((((((pokeblockFeed).wrapping_add(4220).cast::<i16>()).read()) as i32)
            .wrapping_add(
                ((((((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32),
            )) as i16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < ((time) as i32).wrapping_sub(1i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut xOffset: i16 =
                        ((((((((pokeblockFeed).wrapping_add(2128)).cast::<i16>()).wrapping_offset(
                            (((approachTime) as i32).wrapping_add(((i) as i32))) as isize,
                        ))
                        .read()) as i32)
                            .wrapping_sub(((x) as i32))) as i16);
                    let mut yOffset: i16 =
                        ((((((((pokeblockFeed).wrapping_add(3152)).cast::<i16>()).wrapping_offset(
                            (((approachTime) as i32).wrapping_add(((i) as i32))) as isize,
                        ))
                        .read()) as i32)
                            .wrapping_sub(((y) as i32))) as i16);
                    let __p1 = (((pokeblockFeed).wrapping_add(2128)).cast::<i16>())
                        .wrapping_offset(
                            (((approachTime) as i32).wrapping_add(((i) as i32))) as isize,
                        );
                    (__p1).write(
                        (((((__p1).read()) as i32).wrapping_sub(crate::c::div_i32(
                            ((xOffset) as i32).wrapping_mul(((i) as i32).wrapping_add(1i32)),
                            ((time) as i32),
                        ))) as i16),
                    );
                    let __p2 = (((pokeblockFeed).wrapping_add(3152)).cast::<i16>())
                        .wrapping_offset(
                            (((approachTime) as i32).wrapping_add(((i) as i32))) as isize,
                        );
                    (__p2).write(
                        (((((__p2).read()) as i32).wrapping_sub(crate::c::div_i32(
                            ((yOffset) as i32).wrapping_mul(((i) as i32).wrapping_add(1i32)),
                            ((time) as i32),
                        ))) as i16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((pokeblockFeed).wrapping_add(2128)).cast::<i16>()).wrapping_offset(
            ((((approachTime) as i32).wrapping_add(((time) as i32))).wrapping_sub(1i32)) as isize,
        ))
        .write(x);
        ((((pokeblockFeed).wrapping_add(3152)).cast::<i16>()).wrapping_offset(
            ((((approachTime) as i32).wrapping_add(((time) as i32))).wrapping_sub(1i32)) as isize,
        ))
        .write(y);
    }
}
pub(crate) unsafe extern "C" fn CalculateMonAnimMovement() {
    unsafe {
        let mut pokeblockFeed: *mut u8 =
            ((&raw mut sPokeblockFeed).cast::<u8>().cast::<*mut u8>()).read();
        let mut negative: u8 = 0u8;
        let mut x: i16 = ((((((pokeblockFeed).wrapping_add(4218).cast::<i16>()).read()) as i32)
            .wrapping_sub(((((pokeblockFeed).wrapping_add(4212).cast::<i16>()).read()) as i32)))
            as i16);
        let mut y: i16 = ((((((pokeblockFeed).wrapping_add(4220).cast::<i16>()).read()) as i32)
            .wrapping_sub(((((pokeblockFeed).wrapping_add(4214).cast::<i16>()).read()) as i32)))
            as i16);
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            let mut amplitude: u16 = 0u16;
            let mut time: u16 = 0u16;
            let mut acceleration: u16 = 0u16;
            acceleration = ((if ((((((pokeblockFeed).wrapping_add(4192)).cast::<i16>())
                .wrapping_offset(5))
            .read()) as i32)
                < 0i32
            {
                ((((((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    .wrapping_neg()
            } else {
                ((((((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
            }) as u16);
            amplitude = ((((acceleration) as i32).wrapping_add(
                ((((((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32),
            )) as u16);
            ((((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).wrapping_offset(3))
                .write(((amplitude) as i16));
            if ((((((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).wrapping_offset(2)).read())
                as i32)
                < 0i32
            {
                negative = 1u8;
            }
            time = ((((((pokeblockFeed).wrapping_add(4216).cast::<i16>()).read()) as i32)
                .wrapping_sub(
                    ((((((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).wrapping_offset(4))
                        .read()) as i32),
                )) as u16);
            if ((((((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).wrapping_offset(4)).read())
                as i32)
                == 0i32
            {
                break 'l1;
            }
            if !((negative) != 0) {
                ((((pokeblockFeed).wrapping_add(2128)).cast::<i16>())
                    .wrapping_offset(((time) as i32) as isize))
                .write(
                    ((((Sin(
                        (((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).read(),
                        ((((((((pokeblockFeed).wrapping_add(4192)).cast::<i16>())
                            .wrapping_offset(2))
                        .read()) as i32)
                            .wrapping_add(crate::c::div_i32(((amplitude) as i32), 256i32)))
                            as i16),
                    )) as i32)
                        .wrapping_add(((x) as i32))) as i16),
                );
                ((((pokeblockFeed).wrapping_add(3152)).cast::<i16>())
                    .wrapping_offset(((time) as i32) as isize))
                .write(
                    ((((Cos(
                        (((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).read(),
                        ((((((((pokeblockFeed).wrapping_add(4192)).cast::<i16>())
                            .wrapping_offset(3))
                        .read()) as i32)
                            .wrapping_add(crate::c::div_i32(((amplitude) as i32), 256i32)))
                            as i16),
                    )) as i32)
                        .wrapping_add(((y) as i32))) as i16),
                );
            } else {
                ((((pokeblockFeed).wrapping_add(2128)).cast::<i16>())
                    .wrapping_offset(((time) as i32) as isize))
                .write(
                    ((((Sin(
                        (((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).read(),
                        ((((((((pokeblockFeed).wrapping_add(4192)).cast::<i16>())
                            .wrapping_offset(2))
                        .read()) as i32)
                            .wrapping_sub(crate::c::div_i32(((amplitude) as i32), 256i32)))
                            as i16),
                    )) as i32)
                        .wrapping_add(((x) as i32))) as i16),
                );
                ((((pokeblockFeed).wrapping_add(3152)).cast::<i16>())
                    .wrapping_offset(((time) as i32) as isize))
                .write(
                    ((((Cos(
                        (((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).read(),
                        ((((((((pokeblockFeed).wrapping_add(4192)).cast::<i16>())
                            .wrapping_offset(3))
                        .read()) as i32)
                            .wrapping_sub(crate::c::div_i32(((amplitude) as i32), 256i32)))
                            as i16),
                    )) as i32)
                        .wrapping_add(((y) as i32))) as i16),
                );
            }
            let __p1 = ((pokeblockFeed).wrapping_add(4192)).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).wrapping_offset(1))
                        .read()) as i32),
                )) as i16),
            );
            let __p2 = ((pokeblockFeed).wrapping_add(4192)).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32) & 255i32) as i16));
            let __p3 = (((pokeblockFeed).wrapping_add(4192)).cast::<i16>()).wrapping_offset(4);
            (__p3).write(((__p3).read()).wrapping_sub(1));
        }
    }
}
