//! Translated from `src/egg_hatch.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sEggPalette sEggHatchTiles sEggShardTiles sOamData_Egg sSpriteAnim_Egg_Normal sSpriteAnim_Egg_Cracked1 sSpriteAnim_Egg_Cracked2 sSpriteAnim_Egg_Cracked3 sSpriteAnimTable_Egg sEggHatch_Sheet sEggShards_Sheet sEgg_SpritePalette sSpriteTemplate_Egg sOamData_EggShard sSpriteAnim_EggShard0 sSpriteAnim_EggShard1 sSpriteAnim_EggShard2 sSpriteAnim_EggShard3 sSpriteAnimTable_EggShard sSpriteTemplate_EggShard sBgTemplates_EggHatch sWinTemplates_EggHatch sYesNoWinTemplate sEggShardVelocities
#[allow(unused_imports)]
use crate::data::egg_hatch::*;

pub(crate) static mut sEggHatchData: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gBattleTextboxPalette: u8;
    static mut gBattleTextboxTilemap: u8;
    static mut gBattleTextboxTiles: u8;
    static mut gEnemyParty: u8;
    static mut gFieldCallback: u8;
    static mut gMain: u8;
    static mut gMonFrontPicTable: u8;
    static mut gMonSpritesGfxPtr: u8;
    static mut gMultiuseSpriteTemplate: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_HatchedFromEgg: u8;
    static mut gText_NicknameHatchPrompt: u8;
    static mut gTradeGba2_Pal: u8;
    static mut gTradeGba_Gfx: u8;
    static mut gTradePlatform_Tilemap: u8;
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
    fn Alloc(a0: u32) -> *mut u8;
    fn AllocateMonSpritesGfx();
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_ReturnToField();
    fn CalculateMonStats(a0: *mut u8);
    fn CalculatePlayerPartyCount() -> u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CleanupOverworldWindowsAndTilemaps();
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CountPartyAliveNonEggMonsExcept(a0: u8) -> u8;
    fn CountStorageNonEggMons() -> u32;
    fn CreateMon(a0: *mut u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u32, a6: u8, a7: u32);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateYesNoMenu(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DoMonFrontSpriteAnimation(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn DoNamingScreen(
        a0: u8,
        a1: *mut u8,
        a2: u16,
        a3: u16,
        a4: u32,
        a5: Option<unsafe extern "C" fn()>,
    );
    fn FadeScreen(a0: u8, a1: i8);
    fn FieldCB_ContinueScriptHandleMusic();
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeMonSpritesGfx();
    fn GetBoxMonNickname(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn GetCurrentMapMusic() -> u16;
    fn GetCurrentRegionMapSectionId() -> u8;
    fn GetMonAbility(a0: *mut u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut u8) -> u8;
    fn GetMonNickname2(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn GetMonSpritePalStruct(a0: *mut u8) -> *mut u8;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn GetSpeciesName(a0: *mut u8, a1: u16);
    fn HandleLoadSpecialPokePic_DontHandleDeoxys(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32);
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsFanfareTaskInactive() -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LockPlayerFieldControls();
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn MonRestorePP(a0: *mut u8);
    fn PlayBGM(a0: u16);
    fn PlayFanfare(a0: u16);
    fn PlayRainStoppingSoundEffect();
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveWindow(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn RunTasks();
    fn RunTextPrinters();
    fn ScanlineEffect_Stop();
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StopMapMusic();
    fn StringCompareWithoutExtCtrlCodes(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TVShowConvertInternationalString(a0: *mut u8, a1: *mut u8, a2: i32);
    fn TransferPlttBuffer();
    fn UnsetBgTilemapBuffer(a0: u8);
    fn UpdatePaletteFade() -> u8;
    fn m4aSoundVSyncOn();
}

pub(crate) unsafe extern "C" fn CreateHatchedMon(egg: *mut u8, temp: *mut u8) {
    unsafe {
        let mut egg = egg;
        let mut temp = temp;
        let mut species: u16 = 0u16;
        let mut personality: u32 = 0u32;
        let mut pokerus: u32 = 0u32;
        let mut i: u8 = 0u8;
        let mut friendship: u8 = 0u8;
        let mut language: u8 = 0u8;
        let mut gameMet: u8 = 0u8;
        let mut markings: u8 = 0u8;
        let mut isModernFatefulEncounter: u8 = 0u8;
        let mut moves = crate::ffi::Align4([0u8; 8]);
        let mut ivs = crate::ffi::Align4([0u8; 24]);
        species = ((GetMonData2(egg, 11i32)) as u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut moves).cast::<u16>()).wrapping_offset(((i) as i32) as isize))
                        .write(((GetMonData2(egg, (13i32).wrapping_add(((i) as i32)))) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
        personality = GetMonData2(egg, 0i32);
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l3;
                }
                'l4: {
                    (((&raw mut ivs).cast::<u32>()).wrapping_offset(((i) as i32) as isize))
                        .write(GetMonData2(egg, (39i32).wrapping_add(((i) as i32))));
                }
                i = (i).wrapping_add(1);
            }
        }
        language = ((GetMonData2(egg, 3i32)) as u8);
        gameMet = ((GetMonData2(egg, 37i32)) as u8);
        markings = ((GetMonData2(egg, 8i32)) as u8);
        pokerus = GetMonData2(egg, 34i32);
        isModernFatefulEncounter = ((GetMonData2(egg, 80i32)) as u8);
        CreateMon(temp, species, 5u8, 32u8, 1u8, personality, 0u8, 0u32);
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l5;
                }
                'l6: {
                    SetMonData(
                        temp,
                        (13i32).wrapping_add(((i) as i32)),
                        (((&raw mut moves).cast::<u16>()).wrapping_offset(((i) as i32) as isize))
                            .cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l7: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l7;
                }
                'l8: {
                    SetMonData(
                        temp,
                        (39i32).wrapping_add(((i) as i32)),
                        (((&raw mut ivs).cast::<u32>()).wrapping_offset(((i) as i32) as isize))
                            .cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        language = 2u8;
        SetMonData(temp, 3i32, &raw mut language);
        SetMonData(temp, 37i32, &raw mut gameMet);
        SetMonData(temp, 8i32, &raw mut markings);
        friendship = 120u8;
        SetMonData(temp, 32i32, &raw mut friendship);
        SetMonData(temp, 34i32, (&raw mut pokerus).cast::<u8>());
        SetMonData(temp, 80i32, &raw mut isModernFatefulEncounter);
        egg.cast::<crate::c::Rec4<100>>()
            .write_unaligned(temp.cast::<crate::c::Rec4<100>>().read_unaligned());
    }
}
pub(crate) unsafe extern "C" fn AddHatchedMonToParty(id: u8) {
    unsafe {
        let mut id = id;
        let mut isEgg: u8 = 70u8;
        let mut species: u16 = 0u16;
        let mut name = crate::ffi::Align4([0u8; 11]);
        let mut ball: u16 = 0u16;
        let mut metLevel: u16 = 0u16;
        let mut metLocation: u8 = 0u8;
        let mut mon: *mut u8 =
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 100);
        CreateHatchedMon(mon, (&raw mut gEnemyParty).cast::<u8>());
        SetMonData(mon, 45i32, &raw mut isEgg);
        species = ((GetMonData2(mon, 11i32)) as u16);
        GetSpeciesName((&raw mut name).cast::<u8>(), species);
        SetMonData(mon, 2i32, (&raw mut name).cast::<u8>());
        species = SpeciesToNationalPokedexNum(species);
        GetSetPokedexFlag(species, 2u8);
        GetSetPokedexFlag(species, 3u8);
        GetMonNickname2(mon, (&raw mut gStringVar1).cast::<u8>());
        ball = 4u16;
        SetMonData(mon, 38i32, (&raw mut ball).cast::<u8>());
        metLevel = 0u16;
        SetMonData(mon, 36i32, (&raw mut metLevel).cast::<u8>());
        metLocation = GetCurrentRegionMapSectionId();
        SetMonData(mon, 35i32, &raw mut metLocation);
        MonRestorePP(mon);
        CalculateMonStats(mon);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptHatchMon() {
    unsafe {
        AddHatchedMonToParty(((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8));
    }
}
pub(crate) unsafe extern "C" fn _CheckDaycareMonReceivedMail(
    daycare: *mut u8,
    daycareId: u8,
) -> u8 {
    unsafe {
        let mut daycare = daycare;
        let mut daycareId = daycareId;
        let mut nickname = crate::ffi::Align4([0u8; 32]);
        let mut daycareMon: *mut u8 =
            ((daycare).cast::<u8>()).wrapping_offset(((daycareId) as i32) as isize * 140);
        GetBoxMonNickname((daycareMon), (&raw mut nickname).cast::<u8>());
        if ((((((daycareMon).wrapping_add(80))
            .wrapping_add(32)
            .cast::<u16>())
        .read()) as i32)
            != 0i32)
            && ((StringCompareWithoutExtCtrlCodes(
                (&raw mut nickname).cast::<u8>(),
                (((daycareMon).wrapping_add(80)).wrapping_add(44)).cast::<u8>(),
            ) != 0i32)
                || (StringCompareWithoutExtCtrlCodes(
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                    (((daycareMon).wrapping_add(80)).wrapping_add(36)).cast::<u8>(),
                ) != 0i32))
        {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (&raw mut nickname).cast::<u8>(),
            );
            TVShowConvertInternationalString(
                (&raw mut gStringVar2).cast::<u8>(),
                (((daycareMon).wrapping_add(80)).wrapping_add(36)).cast::<u8>(),
                ((crate::c::bf_read(
                    ((daycareMon).wrapping_add(80)).wrapping_add(55),
                    0,
                    4,
                    false,
                ) as u8) as i32),
            );
            TVShowConvertInternationalString(
                (&raw mut gStringVar3).cast::<u8>(),
                (((daycareMon).wrapping_add(80)).wrapping_add(44)).cast::<u8>(),
                ((crate::c::bf_read(
                    ((daycareMon).wrapping_add(80)).wrapping_add(55),
                    4,
                    4,
                    false,
                ) as u8) as i32),
            );
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckDaycareMonReceivedMail() -> u8 {
    unsafe {
        return _CheckDaycareMonReceivedMail(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336),
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn EggHatchCreateMonSprite(
    useAlt: u8,
    state: u8,
    partyId: u8,
    speciesLoc: *mut u16,
) -> u8 {
    unsafe {
        let mut useAlt = useAlt;
        let mut state = state;
        let mut partyId = partyId;
        let mut speciesLoc = speciesLoc;
        let mut position: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        let mut mon: *mut u8 = core::ptr::null_mut();
        if ((useAlt) as i32) == 0i32 {
            mon = ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((partyId) as i32) as isize * 100);
            position = 1u8;
        }
        if ((useAlt) as i32) == 1i32 {
            mon = ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((partyId) as i32) as isize * 100);
            position = 3u8;
        }
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                {
                    let mut species: u16 = ((GetMonData2(mon, 11i32)) as u16);
                    let mut pid: u32 = GetMonData2(mon, 0i32);
                    HandleLoadSpecialPokePic_DontHandleDeoxys(
                        ((&raw mut gMonFrontPicTable).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 8),
                        ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset(
                            ((((useAlt) as i32).wrapping_mul(2i32)).wrapping_add(1i32)) as isize,
                        ))
                        .read(),
                        ((species) as i32),
                        pid,
                    );
                    LoadCompressedSpritePalette(GetMonSpritePalStruct(mon));
                    (speciesLoc).write(species);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetMultiuseSpriteTemplateToPokemon(
                    ((GetMonSpritePalStruct(mon)).wrapping_add(4).cast::<u16>()).read(),
                    position,
                );
                spriteId = CreateSprite(
                    (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
                    ((crate::c::div_i32(240i32, 2i32)) as i16),
                    (((crate::c::div_i32(160i32, 2i32)).wrapping_sub(5i32)) as i16),
                    6u8,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
                break 'l1;
            }
        }
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_EggHatch() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EggHatch() {
    unsafe {
        LockPlayerFieldControls();
        CreateTask(Some(Task_EggHatch), 10u8);
        FadeScreen(1u8, 0i8);
    }
}
pub(crate) unsafe extern "C" fn Task_EggHatch(taskId: u8) {
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
            CleanupOverworldWindowsAndTilemaps();
            SetMainCallback2(Some(CB2_LoadEggHatch));
            ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(FieldCB_ContinueScriptHandleMusic));
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_LoadEggHatch() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                SetGpuReg(0u8, 0u16);
                ((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).write(Alloc(20u32));
                AllocateMonSpritesGfx();
                ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .write(((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8));
                ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7))
                .write(0u8);
                SetVBlankCallback(Some(VBlankCB_EggHatch));
                ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(GetCurrentMapMusic());
                ResetTempTileDataBuffers();
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const sBgTemplates_EggHatch).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(8u32, 4u32)) as u8),
                );
                ChangeBgX(1u8, 0i32, 0u8);
                ChangeBgY(1u8, 0i32, 0u8);
                ChangeBgX(0u8, 0i32, 0u8);
                ChangeBgY(0u8, 0i32, 0u8);
                SetBgAttribute(1u8, 7u8, 2u8);
                SetBgTilemapBuffer(1u8, Alloc(4096u32));
                SetBgTilemapBuffer(0u8, Alloc(8192u32));
                DeactivateAllTextPrinters();
                ResetPaletteFade();
                FreeAllSpritePalettes();
                ResetSpriteData();
                ResetTasks();
                ScanlineEffect_Stop();
                m4aSoundVSyncOn();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                InitWindows(
                    ((&raw const sWinTemplates_EggHatch).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8))
                .write(0u8);
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                DecompressAndLoadBgGfxUsingHeap(
                    0u8,
                    (((&raw mut gBattleTextboxTiles).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                CopyToBgTilemapBuffer(
                    0u8,
                    (((&raw mut gBattleTextboxTilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u16,
                    0u16,
                );
                LoadCompressedPalette(
                    ((&raw mut gBattleTextboxPalette).cast::<u32>()).cast::<u32>(),
                    0u16,
                    32u16,
                );
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                LoadSpriteSheet((&raw const sEggHatch_Sheet).cast::<u8>().cast_mut());
                LoadSpriteSheet((&raw const sEggShards_Sheet).cast::<u8>().cast_mut());
                LoadSpritePalette((&raw const sEgg_SpritePalette).cast::<u8>().cast_mut());
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                CopyBgTilemapBufferToVram(0u8);
                AddHatchedMonToParty(
                    ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .read(),
                );
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                EggHatchCreateMonSprite(
                    0u8,
                    0u8,
                    ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .read(),
                    (((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<u16>(),
                );
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .write(EggHatchCreateMonSprite(
                    0u8,
                    1u8,
                    ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .read(),
                    (((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<u16>(),
                ));
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                SetGpuReg(0u8, 4160u16);
                LoadPalette(
                    (((&raw mut gTradeGba2_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                    16u16,
                    160u16,
                );
                LoadBgTiles(1u8, (&raw mut gTradeGba_Gfx).cast::<u8>(), 5152u16, 0u16);
                CopyToBgTilemapBuffer(
                    1u8,
                    (((&raw mut gTradePlatform_Tilemap).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                    4096u16,
                    0u16,
                );
                CopyBgTilemapBufferToVram(1u8);
                let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                SetMainCallback2(Some(CB2_EggHatch));
                ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .write(0u8);
                break 'l1;
            }
        }
        RunTasks();
        RunTextPrinters();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn EggHatchSetMonNickname() {
    unsafe {
        SetMonData(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            2i32,
            (&raw mut gStringVar3).cast::<u8>(),
        );
        FreeMonSpritesGfx();
        Free(((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read());
        SetMainCallback2(Some(CB2_ReturnToField));
    }
}
pub(crate) unsafe extern "C" fn Task_EggHatchPlayBGM(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            StopMapMusic();
            PlayRainStoppingSoundEffect();
        }
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 1i32
        {
            PlayBGM(376u16);
        }
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            > 60i32
        {
            PlayBGM(377u16);
            DestroyTask(taskId);
        }
        let __p1 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn CB2_EggHatch() {
    unsafe {
        let mut species: u16 = 0u16;
        let mut gender: u8 = 0u8;
        let mut personality: u32 = 0u32;
        'l1: {
            let __sw1 = ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32);
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                (((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read()).write(
                    CreateSprite(
                        (&raw const sSpriteTemplate_Egg).cast::<u8>().cast_mut(),
                        ((crate::c::div_i32(240i32, 2i32)) as i16),
                        (((crate::c::div_i32(160i32, 2i32)).wrapping_sub(5i32)) as i16),
                        5u8,
                    ),
                );
                ShowBg(0u8);
                ShowBg(1u8);
                let __p2 = (((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2);
                (__p2).write(((__p2).read()).wrapping_add(1));
                CreateTask(Some(Task_EggHatchPlayBGM), 5u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    FillWindowPixelBuffer(
                        ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read(),
                        0u8,
                    );
                    ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3))
                    .write(0u8);
                    let __p3 = (((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p4 = (((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 30i32
                {
                    let __p6 = (((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read()).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_Egg_Shake1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if core::mem::transmute::<_, usize>(
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read()).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .read(),
                ) == (SpriteCallbackDummy as *const () as usize)
                {
                    species = ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        11i32,
                    )) as u16);
                    DoMonFrontSpriteAnimation(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1))
                            .read()) as i32) as isize
                                * 68,
                        ),
                        species,
                        0u8,
                        1u8,
                    );
                    let __p7 = (((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2);
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if core::mem::transmute::<_, usize>(
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .read(),
                ) == (SpriteCallbackDummy as *const () as usize)
                {
                    let __p8 = (((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                GetMonNickname2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    (&raw mut gStringVar1).cast::<u8>(),
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_HatchedFromEgg).cast::<u8>(),
                );
                EggHatchPrintMessage(
                    ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8))
                    .read(),
                    (&raw mut gStringVar4).cast::<u8>(),
                    0u8,
                    3u8,
                    255u8,
                );
                PlayFanfare(371u16);
                let __p9 = (((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2);
                (__p9).write(((__p9).read()).wrapping_add(1));
                PutWindowTilemap(
                    ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8))
                    .read(),
                );
                CopyWindowToVram(
                    ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8))
                    .read(),
                    3u8,
                );
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (IsFanfareTaskInactive()) != 0 {
                    let __p10 = (((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2);
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (IsFanfareTaskInactive()) != 0 {
                    let __p11 = (((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2);
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                GetMonNickname2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    (&raw mut gStringVar1).cast::<u8>(),
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_NicknameHatchPrompt).cast::<u8>(),
                );
                EggHatchPrintMessage(
                    ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8))
                    .read(),
                    (&raw mut gStringVar4).cast::<u8>(),
                    0u8,
                    2u8,
                    1u8,
                );
                let __p12 = (((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2);
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                if !((IsTextPrinterActive(
                    ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8))
                    .read(),
                )) != 0)
                {
                    LoadUserWindowBorderGfx(
                        ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read(),
                        320u16,
                        224u8,
                    );
                    CreateYesNoMenu(
                        (&raw const sYesNoWinTemplate).cast::<u8>().cast_mut(),
                        320u16,
                        14u8,
                        0u8,
                    );
                    let __p13 = (((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2);
                    (__p13).write(((__p13).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                'l2: {
                    let __sw14 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
                    if __sw14 == 0i32 {
                        GetMonNickname2(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            (&raw mut gStringVar3).cast::<u8>(),
                        );
                        species = ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            11i32,
                        )) as u16);
                        gender = GetMonGender(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                        );
                        personality = GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            0i32,
                            core::ptr::null_mut(),
                        );
                        DoNamingScreen(
                            3u8,
                            (&raw mut gStringVar3).cast::<u8>(),
                            species,
                            ((gender) as u16),
                            personality,
                            Some(EggHatchSetMonNickname),
                        );
                        break 'l2;
                    }
                    if __sw14 == 1i32 || __sw14 == (-1i32) {
                        let __p15 = (((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(2);
                        (__p15).write(((__p15).read()).wrapping_add(1));
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                let __p16 = (((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2);
                (__p16).write(((__p16).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    FreeMonSpritesGfx();
                    RemoveWindow(
                        ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read(),
                    );
                    UnsetBgTilemapBuffer(0u8);
                    UnsetBgTilemapBuffer(1u8);
                    Free(((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read());
                    SetMainCallback2(Some(CB2_ReturnToField));
                }
                break 'l1;
            }
        }
        RunTasks();
        RunTextPrinters();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Egg_Shake1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 20i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_Egg_Shake2));
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    .wrapping_add(20i32)
                    & 255i32) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                1i16,
            ));
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 15i32 {
                PlaySE(23u16);
                StartSpriteAnim(sprite, 1u8);
                CreateRandomEggShardSprite();
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Egg_Shake2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 30i32
        {
            if (({
                let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                > 20i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_Egg_Shake3));
                (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        .wrapping_add(20i32)
                        & 255i32) as i16),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                    2i16,
                ));
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 15i32 {
                    PlaySE(23u16);
                    StartSpriteAnim(sprite, 2u8);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Egg_Shake3(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 30i32
        {
            if (({
                let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                > 38i32
            {
                let mut species: u16 = 0u16;
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_Egg_WaitHatch));
                (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                species = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    11i32,
                )) as u16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        .wrapping_add(20i32)
                        & 255i32) as i16),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                    2i16,
                ));
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 15i32 {
                    PlaySE(23u16);
                    StartSpriteAnim(sprite, 2u8);
                    CreateRandomEggShardSprite();
                    CreateRandomEggShardSprite();
                }
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 30i32 {
                    PlaySE(23u16);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Egg_WaitHatch(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 50i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_Egg_Hatch));
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Egg_Hatch(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: i16 = 0i16;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            BeginNormalPaletteFade(4294967295u32, (-1i8), 0u8, 16u8, 65535u16);
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u32) < 4u32 {
            {
                i = 0i16;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        CreateRandomEggShardSprite();
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            PlaySE(113u16);
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_Egg_Reveal));
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Egg_Reveal(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            StartSpriteAffineAnim(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1))
                    .read()) as i32) as isize
                        * 68,
                ),
                1u8,
            );
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 8i32 {
            BeginNormalPaletteFade(4294967295u32, (-1i8), 16u8, 0u8, 65535u16);
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) <= 9i32 {
            let __p1 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 40i32 {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
        let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_EggShard(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
                256i32,
            )) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
                256i32,
            )) as i16),
        );
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
            )) as i16),
        );
        if (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
            > ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(20i32))
            && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > 0i32)
        {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateRandomEggShardSprite() {
    unsafe {
        let mut spriteAnimIndex: u16 = 0u16;
        let mut velocityX: i16 = (((((&raw const sEggShardVelocities).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7))
                .read()) as i32) as isize
                * 4,
        ))
        .cast::<i16>())
        .read();
        let mut velocityY: i16 = ((((((&raw const sEggShardVelocities).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7))
                .read()) as i32) as isize
                * 4,
        ))
        .cast::<i16>())
        .wrapping_offset(1))
        .read();
        let __p1 =
            (((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7);
        (__p1).write(((__p1).read()).wrapping_add(1));
        spriteAnimIndex =
            ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(16u32, 4u32))) as u16);
        CreateEggShardSprite(
            ((crate::c::div_i32(240i32, 2i32)) as u8),
            ((((crate::c::div_i32(160i32, 2i32)).wrapping_sub(5i32)).wrapping_sub(15i32)) as u8),
            velocityX,
            velocityY,
            100i16,
            ((spriteAnimIndex) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateEggShardSprite(
    x: u8,
    y: u8,
    velocityX: i16,
    velocityY: i16,
    acceleration: i16,
    spriteAnimIndex: u8,
) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut velocityX = velocityX;
        let mut velocityY = velocityY;
        let mut acceleration = acceleration;
        let mut spriteAnimIndex = spriteAnimIndex;
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_EggShard)
                .cast::<u8>()
                .cast_mut(),
            ((x) as i16),
            ((y) as i16),
            4u8,
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(velocityX);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(velocityY);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(acceleration);
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            spriteAnimIndex,
        );
    }
}
pub(crate) unsafe extern "C" fn EggHatchPrintMessage(
    windowId: u8,
    string: *mut u8,
    x: u8,
    y: u8,
    speed: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut string = string;
        let mut x = x;
        let mut y = y;
        let mut speed = speed;
        FillWindowPixelBuffer(windowId, 255u8);
        (((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14))
            .cast::<u8>())
        .write(0u8);
        ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14))
            .cast::<u8>())
        .wrapping_offset(1))
        .write(5u8);
        ((((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14))
            .cast::<u8>())
        .wrapping_offset(2))
        .write(6u8);
        AddTextPrinterParameterized4(
            windowId,
            1u8,
            x,
            y,
            0u8,
            0u8,
            ((((&raw mut sEggHatchData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14))
                .cast::<u8>(),
            ((speed) as i8),
            string,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetEggCyclesToSubtract() -> u8 {
    unsafe {
        let mut count: u8 = 0u8;
        let mut i: u8 = 0u8;
        {
            count = CalculatePlayerPartyCount();
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((count) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if !((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        6i32,
                    )) != 0)
                    {
                        let mut ability: u8 = GetMonAbility(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                        );
                        if (((ability) as i32) == 40i32) || (((ability) as i32) == 49i32) {
                            return 2u8;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountPartyAliveNonEggMons() -> u16 {
    unsafe {
        let mut aliveNonEggMonsCount: u16 = ((CountStorageNonEggMons()) as u16);
        aliveNonEggMonsCount = ((((aliveNonEggMonsCount) as i32)
            .wrapping_add(((CountPartyAliveNonEggMonsExcept(6u8)) as i32)))
            as u16);
        return aliveNonEggMonsCount;
    }
}
