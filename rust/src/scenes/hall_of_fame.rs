//! Translated from `src/hall_of_fame.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sHof_BgTemplates sHof_WindowTemplate sMonInfoTextColors sPlayerInfoTextColors sUnusedTextColors sSpriteSheet_Confetti sSpritePalette_Confetti sHallOfFame_MonFullTeamPositions sHallOfFame_MonHalfTeamPositions sOamData_Confetti sAnim_PinkConfettiA sAnim_RedConfettiA sAnim_BlueConfettiA sAnim_RedConfettiB sAnim_BlueConfettiB sAnim_YellowConfettiA sAnim_WhiteConfettiA sAnim_GreenConfettiA sAnim_PinkConfettiB sAnim_BlueConfettiC sAnim_YellowConfettiB sAnim_WhiteConfettiB sAnim_GreenConfettiB sAnim_PinkConfettiC sAnim_RedConfettiC sAnim_YellowConfettiC sAnim_WhiteConfettiC sAnims_Confetti sSpriteTemplate_HofConfetti sHallOfFame_Pal sHallOfFame_Gfx sDummyFameMon sHallOfFame_SlotOrder
#[allow(unused_imports)]
use crate::data::hall_of_fame::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHofFadePalettes: u32 = 0u32;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHofMonPtr: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHofGfxPtr: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gDamagedSaveSectors: u8;
    static mut gDecompressionBuffer: u8;
    static mut gGameContinueCallback: u8;
    static mut gHasHallOfFameRecords: u8;
    static mut gMPlayInfo_BGM: u8;
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gReservedSpritePaletteCount: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSineTable: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpeciesNames: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_AButtonExit: u8;
    static mut gText_HOFCorrupted: u8;
    static mut gText_HOFNumber: u8;
    static mut gText_IDNumber: u8;
    static mut gText_LeagueChamp: u8;
    static mut gText_Level: u8;
    static mut gText_Name: u8;
    static mut gText_Number: u8;
    static mut gText_PickCancel: u8;
    static mut gText_PickNextCancel: u8;
    static mut gText_SavingDontTurnOffPower: u8;
    static mut gText_Time: u8;
    static mut gText_WelcomeToHOF: u8;
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
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AddWindow(a0: *mut u8) -> u16;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BlendPalettesUnfaded(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_StartCreditsSequence();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearDialogWindowAndFrame(a0: u8, a1: u8);
    fn ComputerScreenCloseEffect(a0: u16, a1: u16, a2: u8);
    fn ComputerScreenOpenEffect(a0: u16, a1: u16, a2: u8);
    fn ConfettiUtil_AddNew(a0: *mut u8, a1: u16, a2: u16, a3: i16, a4: i16, a5: u8, a6: u8) -> u8;
    fn ConfettiUtil_Free() -> u32;
    fn ConfettiUtil_Init(a0: u8) -> u32;
    fn ConfettiUtil_Remove(a0: u8) -> u8;
    fn ConfettiUtil_SetCallback(a0: u8, a1: Option<unsafe extern "C" fn(*mut u8)>) -> u8;
    fn ConfettiUtil_SetData(a0: u8, a1: u8, a2: i16) -> u8;
    fn ConfettiUtil_Update() -> u32;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateMonPicSprite_Affine(
        a0: u16,
        a1: u32,
        a2: u32,
        a3: u8,
        a4: i16,
        a5: i16,
        a6: u8,
        a7: u16,
    ) -> u16;
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
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DoMonFrontSpriteAnimation(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn FadeOutBGM(a0: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeAndDestroyMonPicSprite(a0: u16) -> u16;
    fn FreeAndDestroyTrainerPicSprite(a0: u16) -> u16;
    fn FreeOamMatrix(a0: u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetGameStat(a0: u8) -> u32;
    fn GetGenderFromSpeciesAndPersonality(a0: u16, a1: u32) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetTextWindowPalette(a0: u8) -> *mut u16;
    fn HideBg(a0: u8);
    fn HofPCTopBar_AddWindow(a0: u8, a1: u8, a2: u8, a3: u8, a4: u16) -> u8;
    fn HofPCTopBar_Print(a0: *mut u8, a1: u8, a2: u8);
    fn HofPCTopBar_PrintPair(a0: *mut u8, a1: *mut u8, a2: u8, a3: u8, a4: u8);
    fn HofPCTopBar_RemoveWindow();
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitStandardTextBoxWindows();
    fn InitTextBoxGfxAndPrinters();
    fn IsComputerScreenCloseEffectActive() -> u8;
    fn IsComputerScreenOpenEffectActive() -> u8;
    fn IsCryPlayingOrClearCrySongs() -> u8;
    fn LoadCompressedSpritePalette(a0: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadGameSave(a0: u8) -> u8;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadWindowGfx(a0: u8, a1: u8, a2: u16, a3: u8);
    fn PlayBGM(a0: u16);
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlaySE(a0: u16);
    fn PlayerGenderToFrontTrainerPicId_Debug(a0: u8, a1: u8) -> u16;
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn ResetAllPicSprites() -> u16;
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn ReturnFromHallOfFamePC();
    fn RunTasks();
    fn RunTextPrinters();
    fn ScanlineEffect_Stop();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn SpeciesToPokedexNum(a0: u16) -> u16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StopCryAndClearCrySongs();
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn TrySavingData(a0: u8) -> u8;
    fn UnsetBgTilemapBuffer(a0: u8);
    fn UpdatePaletteFade() -> u8;
    fn m4aMPlayVolumeControl(a0: *mut u8, a1: u16, a2: u16);
}

pub(crate) unsafe extern "C" fn VBlankCB_HallOfFame() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn CB2_HallOfFame() {
    unsafe {
        RunTasks();
        RunTextPrinters();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn InitHallOfFameScreen() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                SetVBlankCallback(None);
                ClearVramOamPltt_LoadHofPal();
                ((&raw mut sHofGfxPtr).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(8212u32));
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                LoadHofGfx();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetGpuReg(80u8, 16194u16);
                SetGpuReg(82u8, 1808u16);
                SetGpuReg(84u8, 0u16);
                InitHofBgs();
                ((((&raw mut sHofGfxPtr).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                    .write(0u16);
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((LoadHofBgs()) != 0) {
                    SetVBlankCallback(Some(VBlankCB_HallOfFame));
                    BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                    let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                UpdatePaletteFade();
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    SetMainCallback2(Some(CB2_HallOfFame));
                    PlayBGM(436u16);
                    return 0u8;
                }
                break 'l1;
            }
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_DoHallOfFameScreen() {
    unsafe {
        if !((InitHallOfFameScreen()) != 0) {
            let mut taskId: u8 = CreateTask(Some(Task_Hof_InitMonData), 0u8);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            ((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(120u32));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_DoHallOfFameScreenDontSaveData() {
    unsafe {
        if !((InitHallOfFameScreen()) != 0) {
            let mut taskId: u8 = CreateTask(Some(Task_Hof_InitMonData), 0u8);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(1i16);
            ((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(120u32));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_InitMonData(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    let mut nickname = crate::ffi::Align4([0u8; 11]);
                    if (GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        11i32,
                    )) != 0
                    {
                        crate::c::bf_write(
                            (((((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 20))
                            .wrapping_add(8),
                            0,
                            9,
                            ((GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 100),
                                65i32,
                            )) as u16) as i32,
                        );
                        ((((((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 20))
                        .cast::<u32>())
                        .write(GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            1i32,
                        ));
                        ((((((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 20))
                        .wrapping_add(4)
                        .cast::<u32>())
                        .write(GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            0i32,
                        ));
                        crate::c::bf_write(
                            (((((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 20))
                            .wrapping_add(9),
                            1,
                            7,
                            ((GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 100),
                                56i32,
                            )) as u16) as i32,
                        );
                        GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            2i32,
                            (&raw mut nickname).cast::<u8>(),
                        );
                        {
                            j = 0u16;
                            'l3: loop {
                                if !(((j) as i32) < 10i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    ((((((((&raw mut sHofMonPtr)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 20))
                                    .wrapping_add(10))
                                    .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .write(
                                        (((&raw mut nickname).cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize))
                                        .read(),
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        let __p1 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2);
                        (__p1).write(((__p1).read()).wrapping_add(1));
                    } else {
                        crate::c::bf_write(
                            (((((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 20))
                            .wrapping_add(8),
                            0,
                            9,
                            (0u16) as i32,
                        );
                        ((((((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 20))
                        .cast::<u32>())
                        .write(0u32);
                        ((((((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 20))
                        .wrapping_add(4)
                        .cast::<u32>())
                        .write(0u32);
                        crate::c::bf_write(
                            (((((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 20))
                            .wrapping_add(9),
                            1,
                            7,
                            (0u16) as i32,
                        );
                        (((((((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 20))
                        .wrapping_add(10))
                        .cast::<u8>())
                        .write(255u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut sHofFadePalettes).cast::<u8>().cast::<u32>()).write(0u32);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(255i16);
        {
            i = 0u16;
            'l5: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l5;
                }
                'l6: {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset((((i) as i32).wrapping_add(5i32)) as isize))
                    .write(255i16);
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .read())
            != 0
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Hof_SetMonDisplayTask));
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Hof_InitTeamSaveData));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_InitTeamSaveData(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        let mut lastSavedTeam: *mut u8 = (&raw mut gDecompressionBuffer).cast::<u8>();
        if !((((&raw mut gHasHallOfFameRecords).cast::<u8>()).read()) != 0) {
            crate::c::memset((&raw mut gDecompressionBuffer).cast::<u8>(), 0i32, 8192u32);
        } else {
            if ((LoadGameSave(3u8)) as i32) != 1i32 {
                crate::c::memset((&raw mut gDecompressionBuffer).cast::<u8>(), 0i32, 8192u32);
            }
        }
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 50i32) {
                    break 'l1;
                }
                'l2: {
                    if ((crate::c::bf_read(
                        ((lastSavedTeam).cast::<u8>()).wrapping_add(8),
                        0,
                        9,
                        false,
                    ) as u16) as i32)
                        == 0i32
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
                lastSavedTeam = (lastSavedTeam).wrapping_offset(120);
            }
        }
        if ((i) as i32) >= 50i32 {
            let mut afterTeam: *mut u8 = (&raw mut gDecompressionBuffer).cast::<u8>();
            let mut beforeTeam: *mut u8 = (&raw mut gDecompressionBuffer).cast::<u8>();
            afterTeam = (afterTeam).wrapping_offset(120);
            {
                i = 0u16;
                'l3: loop {
                    if !(((i) as i32) < 49i32) {
                        break 'l3;
                    }
                    'l4: {
                        beforeTeam.cast::<crate::c::Rec4<120>>().write_unaligned(
                            afterTeam.cast::<crate::c::Rec4<120>>().read_unaligned(),
                        );
                    }
                    i = (i).wrapping_add(1);
                    beforeTeam = (beforeTeam).wrapping_offset(120);
                    afterTeam = (afterTeam).wrapping_offset(120);
                }
            }
            lastSavedTeam = (lastSavedTeam).wrapping_offset(-120);
        }
        lastSavedTeam.cast::<crate::c::Rec4<120>>().write_unaligned(
            ((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>())
                .read()
                .cast::<crate::c::Rec4<120>>()
                .read_unaligned(),
        );
        DrawDialogueFrame(0u8, 0u8);
        AddTextPrinterParameterized2(
            0u8,
            1u8,
            (&raw mut gText_SavingDontTurnOffPower).cast::<u8>(),
            0u8,
            None,
            2u8,
            1u8,
            3u8,
        );
        CopyWindowToVram(0u8, 3u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Hof_TrySaveData));
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_TrySaveData(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gGameContinueCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(CB2_DoHallOfFameScreenDontSaveData));
        if (((TrySavingData(3u8)) as i32) == 255i32)
            && (((&raw mut gDamagedSaveSectors).cast::<u32>()).read() != 0u32)
        {
            UnsetBgTilemapBuffer(1u8);
            UnsetBgTilemapBuffer(3u8);
            FreeAllWindowBuffers();
            if ((((&raw mut sHofGfxPtr).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize
            {
                Free(((&raw mut sHofGfxPtr).cast::<u8>().cast::<*mut u8>()).read());
                ((&raw mut sHofGfxPtr).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
            }
            if ((((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize
            {
                Free(((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read());
                ((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
            }
            DestroyTask(taskId);
        } else {
            PlaySE(55u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Hof_WaitToDisplayMon));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(32i16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_WaitToDisplayMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read())
            != 0
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Hof_SetMonDisplayTask));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_SetMonDisplayTask(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Hof_DisplayMon));
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_DisplayMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        let mut startX: i16 = 0i16;
        let mut startY: i16 = 0i16;
        let mut destX: i16 = 0i16;
        let mut destY: i16 = 0i16;
        let mut currMonId: u16 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u16);
        let mut currMon: *mut u8 =
            ((((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset(((currMonId) as i32) as isize * 20);
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            > crate::c::div_i32(6i32, 2i32)
        {
            startX = (((((&raw const sHallOfFame_MonFullTeamPositions)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((currMonId) as i32) as isize * 8))
            .cast::<i16>())
            .read();
            startY = ((((((&raw const sHallOfFame_MonFullTeamPositions)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((currMonId) as i32) as isize * 8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read();
            destX = ((((((&raw const sHallOfFame_MonFullTeamPositions)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((currMonId) as i32) as isize * 8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read();
            destY = ((((((&raw const sHallOfFame_MonFullTeamPositions)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((currMonId) as i32) as isize * 8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read();
        } else {
            startX = (((((&raw const sHallOfFame_MonHalfTeamPositions)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((currMonId) as i32) as isize * 8))
            .cast::<i16>())
            .read();
            startY = ((((((&raw const sHallOfFame_MonHalfTeamPositions)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((currMonId) as i32) as isize * 8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read();
            destX = ((((((&raw const sHallOfFame_MonHalfTeamPositions)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((currMonId) as i32) as isize * 8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read();
            destY = ((((((&raw const sHallOfFame_MonHalfTeamPositions)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((currMonId) as i32) as isize * 8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read();
        }
        if ((crate::c::bf_read((currMon).wrapping_add(8), 0, 9, false) as u16) as i32) == 412i32 {
            destY = ((((destY) as i32).wrapping_add(10i32)) as i16);
        }
        spriteId = ((CreateMonPicSprite_Affine(
            (crate::c::bf_read((currMon).wrapping_add(8), 0, 9, false) as u16),
            ((currMon).cast::<u32>()).read(),
            ((currMon).wrapping_add(4).cast::<u32>()).read(),
            1u8,
            startX,
            startY,
            ((currMonId) as u8),
            65535u16,
        )) as u8);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(destX);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(destY);
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((crate::c::bf_read((currMon).wrapping_add(8), 0, 9, false) as u16) as i16));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_GetOnScreenAndAnimate));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset((((currMonId) as i32).wrapping_add(5i32)) as isize))
        .write(((spriteId) as i16));
        ClearDialogWindowAndFrame(0u8, 1u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Hof_PrintMonInfoAfterAnimating));
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_PrintMonInfoAfterAnimating(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut currMonId: u16 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u16);
        let mut currMon: *mut u8 =
            ((((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset(((currMonId) as i32) as isize * 20);
        let mut monSprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset((((currMonId) as i32).wrapping_add(5i32)) as isize))
            .read()) as i32) as isize
                * 68,
        );
        if core::mem::transmute::<_, usize>(
            ((monSprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read(),
        ) == (SpriteCallbackDummy as *const () as usize)
        {
            crate::c::bf_write((monSprite).wrapping_add(1), 0, 2, (0u32) as i32);
            HallOfFame_PrintMonInfo(currMon, 0u8, 14u8);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(120i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Hof_TryDisplayAnotherMon));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_TryDisplayAnotherMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut currPokeID: u16 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u16);
        let mut currMon: *mut u8 =
            ((((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset(((currPokeID) as i32) as isize * 20);
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            != 0i32
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            let __p2 = (&raw mut sHofFadePalettes).cast::<u8>().cast::<u32>();
            (__p2).write(
                ((__p2).read()
                    | ((crate::c::shl_i32(
                        65536i32,
                        ((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(
                                    (((currPokeID) as i32).wrapping_add(5i32)) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            4,
                            4,
                            false,
                        ) as u16) as u32),
                    )) as u32)),
            );
            if (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                < 5i32)
                && (((crate::c::bf_read(
                    ((currMon).wrapping_offset(20)).wrapping_add(8),
                    0,
                    9,
                    false,
                ) as u16) as i32)
                    != 0i32)
            {
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p3).write(((__p3).read()).wrapping_add(1));
                BeginNormalPaletteFade(
                    ((&raw mut sHofFadePalettes).cast::<u8>().cast::<u32>()).read(),
                    0i8,
                    12u8,
                    12u8,
                    25520u16,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset((((currPokeID) as i32).wrapping_add(5i32)) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(5),
                    2,
                    2,
                    (1u16) as i32,
                );
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_Hof_DisplayMon));
            } else {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_Hof_PaletteFadeAndPrintWelcomeText));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_PaletteFadeAndPrintWelcomeText(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        BeginNormalPaletteFade(4294901760u32, 0i8, 0u8, 0u8, 0u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset((((i) as i32).wrapping_add(5i32)) as isize))
                    .read()) as i32)
                        != 255i32
                    {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset((((i) as i32).wrapping_add(5i32)) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            2,
                            2,
                            (0u16) as i32,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        HallOfFame_PrintWelcomeText(0u8, 15u8);
        PlaySE(105u16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(400i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Hof_DoConfetti));
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_DoConfetti(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            != 0i32
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            if ((((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as i32)
                & 3i32)
                == 0i32)
                && (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    > 110i32)
            {
                CreateHofConfettiSprite();
            }
        } else {
            let mut i: u16 = 0u16;
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset((((i) as i32).wrapping_add(5i32)) as isize))
                        .read()) as i32)
                            != 255i32
                        {
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset((((i) as i32).wrapping_add(5i32)) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(5),
                                2,
                                2,
                                (1u16) as i32,
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            BeginNormalPaletteFade(
                ((&raw mut sHofFadePalettes).cast::<u8>().cast::<u32>()).read(),
                0i8,
                12u8,
                12u8,
                25520u16,
            );
            FillWindowPixelBuffer(0u8, 0u8);
            CopyWindowToVram(0u8, 3u8);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(7i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Hof_WaitToDisplayPlayer));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_WaitToDisplayPlayer(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            >= 16i32
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Hof_DisplayPlayer));
        } else {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3);
            (__p1).write(((__p1).read()).wrapping_add(1));
            SetGpuReg(
                82u8,
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    .wrapping_mul(256i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_DisplayPlayer(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetGpuReg(0u8, 4160u16);
        ShowBg(0u8);
        ShowBg(1u8);
        ShowBg(3u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(
            ((CreateTrainerPicSprite(
                PlayerGenderToFrontTrainerPicId_Debug(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read(),
                    1u8,
                ),
                1u8,
                120i16,
                72i16,
                6u8,
                65535u16,
            )) as i16),
        );
        AddWindow((&raw const sHof_WindowTemplate).cast::<u8>().cast_mut());
        LoadWindowGfx(
            1u8,
            ((crate::c::bf_read(
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
                3,
                5,
                false,
            ) as u16) as u8),
            541u16,
            208u8,
        );
        LoadPalette((GetTextWindowPalette(1u8)).cast::<u8>(), 224u16, 32u16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(120i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Hof_WaitAndPrintPlayerInfo));
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_WaitAndPrintPlayerInfo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            != 0i32
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>())
            .read()) as i32)
                != 192i32
            {
                let __p2 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            } else {
                FillBgTilemapBufferRect_Palette0(0u8, 0u16, 0u8, 0u8, 32u8, 32u8);
                HallOfFame_PrintPlayerInfo(1u8, 2u8);
                DrawDialogueFrame(0u8, 0u8);
                AddTextPrinterParameterized2(
                    0u8,
                    1u8,
                    (&raw mut gText_LeagueChamp).cast::<u8>(),
                    0u8,
                    None,
                    2u8,
                    1u8,
                    3u8,
                );
                CopyWindowToVram(0u8, 3u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_Hof_ExitOnKeyPressed));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_ExitOnKeyPressed(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            FadeOutBGM(4u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Hof_HandlePaletteOnExit));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_HandlePaletteOnExit(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .cast::<u8>(),
                            (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .cast::<u8>(),
                            ((0i32
                                | (crate::c::div_i32(1024i32, crate::c::div_i32(16i32, 8i32))
                                    & 2097151i32)) as u32),
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
        BeginNormalPaletteFade(4294967295u32, 8i8, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Hof_HandleExit));
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_HandleExit(taskId: u8) {
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
            let mut i: i32 = 0i32;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        let mut spriteId: u8 = ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(((i).wrapping_add(5i32)) as isize))
                        .read()) as u8);
                        if ((spriteId) as i32) != 255i32 {
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
                            FreeAndDestroyMonPicSprite(((spriteId) as u16));
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            FreeAndDestroyTrainerPicSprite(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as u16),
            );
            HideBg(0u8);
            HideBg(1u8);
            HideBg(3u8);
            FreeAllWindowBuffers();
            UnsetBgTilemapBuffer(1u8);
            UnsetBgTilemapBuffer(3u8);
            ResetBgsAndClearDma3BusyFlags(0u32);
            DestroyTask(taskId);
            if ((((&raw mut sHofGfxPtr).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize
            {
                Free(((&raw mut sHofGfxPtr).cast::<u8>().cast::<*mut u8>()).read());
                ((&raw mut sHofGfxPtr).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
            }
            if ((((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize
            {
                Free(((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read());
                ((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
            }
            StartCredits();
        }
    }
}
pub(crate) unsafe extern "C" fn StartCredits() {
    unsafe {
        SetMainCallback2(Some(CB2_StartCreditsSequence));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_DoHallOfFamePC() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32;
            if __sw1 == 0i32 || !__matched {
                SetVBlankCallback(None);
                ClearVramOamPltt_LoadHofPal();
                ((&raw mut sHofGfxPtr).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(8212u32));
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                LoadHofGfx();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetGpuReg(84u8, 0u16);
                InitHofBgs();
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((LoadHofBgs()) != 0) {
                    let mut fameTeam: *mut u8 = (&raw mut gDecompressionBuffer).cast::<u8>();
                    (fameTeam)
                        .cast::<u8>()
                        .cast::<crate::c::Rec4<20>>()
                        .write_unaligned(
                            (&raw const sDummyFameMon)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<crate::c::Rec4<20>>()
                                .read_unaligned(),
                        );
                    ComputerScreenOpenEffect(0u16, 0u16, 0u8);
                    SetVBlankCallback(Some(VBlankCB_HallOfFame));
                    let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                RunTasks();
                AnimateSprites();
                BuildOamBuffer();
                UpdatePaletteFade();
                if !((IsComputerScreenOpenEffectActive()) != 0) {
                    let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                {
                    let mut taskId: u8 = 0u8;
                    let mut i: u8 = 0u8;
                    SetGpuReg(80u8, 16194u16);
                    SetGpuReg(82u8, 1808u16);
                    SetGpuReg(84u8, 0u16);
                    taskId = CreateTask(Some(Task_HofPC_CopySaveData), 0u8);
                    {
                        i = 0u8;
                        'l2: loop {
                            if !(((i) as i32) < 6i32) {
                                break 'l2;
                            }
                            'l3: {
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset((((i) as i32).wrapping_add(5i32)) as isize))
                                .write(255i16);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    ((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>())
                        .write(AllocZeroed(8192u32));
                    SetMainCallback2(Some(CB2_HallOfFame));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HofPC_CopySaveData(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        HofPCTopBar_AddWindow(0u8, 30u8, 0u8, 12u8, 550u16);
        if ((LoadGameSave(3u8)) as i32) != 1i32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HofPC_PrintDataIsCorrupted));
        } else {
            let mut i: u16 = 0u16;
            let mut savedTeams: *mut u8 = core::ptr::null_mut();
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut gDecompressionBuffer).cast::<u8>(),
                                ((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read(),
                                ((0i32
                                    | (crate::c::div_i32(8192i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
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
            savedTeams = ((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read();
            {
                i = 0u16;
                'l5: loop {
                    if !(((i) as i32) < 50i32) {
                        break 'l5;
                    }
                    'l6: {
                        if ((crate::c::bf_read(
                            ((savedTeams).cast::<u8>()).wrapping_add(8),
                            0,
                            9,
                            false,
                        ) as u16) as i32)
                            == 0i32
                        {
                            break 'l5;
                        }
                    }
                    i = (i).wrapping_add(1);
                    savedTeams = (savedTeams).wrapping_offset(120);
                }
            }
            if ((i) as i32) < 50i32 {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(((((i) as i32).wrapping_sub(1i32)) as i16));
            } else {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(49i16);
            }
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((GetGameStat(10u8)) as i16));
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HofPC_DrawSpritesPrintText));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HofPC_DrawSpritesPrintText(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut savedTeams: *mut u8 = ((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read();
        let mut currMon: *mut u8 = core::ptr::null_mut();
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    savedTeams = (savedTeams).wrapping_offset(120);
                }
                i = (i).wrapping_add(1);
            }
        }
        currMon = (savedTeams).cast::<u8>();
        ((&raw mut sHofFadePalettes).cast::<u8>().cast::<u32>()).write(0u32);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(0i16);
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l3;
                }
                'l4: {
                    if ((crate::c::bf_read((currMon).wrapping_add(8), 0, 9, false) as u16) as i32)
                        != 0i32
                    {
                        let __p1 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4);
                        (__p1).write(((__p1).read()).wrapping_add(1));
                    }
                }
                i = (i).wrapping_add(1);
                currMon = (currMon).wrapping_offset(20);
            }
        }
        currMon = (savedTeams).cast::<u8>();
        {
            i = 0u16;
            'l5: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l5;
                }
                'l6: {
                    if ((crate::c::bf_read((currMon).wrapping_add(8), 0, 9, false) as u16) as i32)
                        != 0i32
                    {
                        let mut spriteId: u16 = 0u16;
                        let mut posX: i16 = 0i16;
                        let mut posY: i16 = 0i16;
                        if ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .read()) as i32)
                            > crate::c::div_i32(6i32, 2i32)
                        {
                            posX = ((((((&raw const sHallOfFame_MonFullTeamPositions)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8))
                            .cast::<i16>())
                            .wrapping_offset(2))
                            .read();
                            posY = ((((((&raw const sHallOfFame_MonFullTeamPositions)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8))
                            .cast::<i16>())
                            .wrapping_offset(3))
                            .read();
                        } else {
                            posX = ((((((&raw const sHallOfFame_MonHalfTeamPositions)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8))
                            .cast::<i16>())
                            .wrapping_offset(2))
                            .read();
                            posY = ((((((&raw const sHallOfFame_MonHalfTeamPositions)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8))
                            .cast::<i16>())
                            .wrapping_offset(3))
                            .read();
                        }
                        if ((crate::c::bf_read((currMon).wrapping_add(8), 0, 9, false) as u16)
                            as i32)
                            == 412i32
                        {
                            posY = ((((posY) as i32).wrapping_add(10i32)) as i16);
                        }
                        spriteId = CreateMonPicSprite_HandleDeoxys(
                            (crate::c::bf_read((currMon).wrapping_add(8), 0, 9, false) as u16),
                            ((currMon).cast::<u32>()).read(),
                            ((currMon).wrapping_add(4).cast::<u32>()).read(),
                            1u8,
                            posX,
                            posY,
                            ((i) as u8),
                            65535u16,
                        );
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(5),
                            2,
                            2,
                            (1u16) as i32,
                        );
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset((((i) as i32).wrapping_add(5i32)) as isize))
                        .write(((spriteId) as i16));
                    } else {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset((((i) as i32).wrapping_add(5i32)) as isize))
                        .write(255i16);
                    }
                }
                i = (i).wrapping_add(1);
                currMon = (currMon).wrapping_offset(20);
            }
        }
        BlendPalettes(4294901760u32, 12u8, 25520u16);
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32),
            1i32,
            3u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_HOFNumber).cast::<u8>(),
        );
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            <= 0i32
        {
            HofPCTopBar_PrintPair(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_PickCancel).cast::<u8>(),
                0u8,
                0u8,
                1u8,
            );
        } else {
            HofPCTopBar_PrintPair(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_PickNextCancel).cast::<u8>(),
                0u8,
                0u8,
                1u8,
            );
        }
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HofPC_PrintMonInfo));
    }
}
pub(crate) unsafe extern "C" fn Task_HofPC_PrintMonInfo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut savedTeams: *mut u8 = ((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read();
        let mut currMon: *mut u8 = core::ptr::null_mut();
        let mut i: u16 = 0u16;
        let mut currMonID: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    savedTeams = (savedTeams).wrapping_offset(120);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l3;
                }
                'l4: {
                    let mut spriteId: u16 = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset((((i) as i32).wrapping_add(5i32)) as isize))
                    .read()) as u16);
                    if ((spriteId) as i32) != 255i32 {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(5),
                            2,
                            2,
                            (1u16) as i32,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        currMonID = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(
            (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
                .wrapping_add(5i32)) as isize,
        ))
        .read()) as u16);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((currMonID) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (0u16) as i32,
        );
        ((&raw mut sHofFadePalettes).cast::<u8>().cast::<u32>()).write(
            (((crate::c::shl_i32(
                65536i32,
                ((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((currMonID) as i32) as isize * 68))
                    .wrapping_add(5),
                    4,
                    4,
                    false,
                ) as u16) as u32),
            )) as u32)
                ^ 4294901760u32),
        );
        BlendPalettesUnfaded(
            ((&raw mut sHofFadePalettes).cast::<u8>().cast::<u32>()).read(),
            12u8,
            25520u16,
        );
        currMon = ((savedTeams).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32) as isize
                * 20,
        );
        if ((crate::c::bf_read((currMon).wrapping_add(8), 0, 9, false) as u16) as i32) != 412i32 {
            StopCryAndClearCrySongs();
            PlayCry_Normal(
                (crate::c::bf_read((currMon).wrapping_add(8), 0, 9, false) as u16),
                0i8,
            );
        }
        HallOfFame_PrintMonInfo(currMon, 0u8, 14u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HofPC_HandleInput));
    }
}
pub(crate) unsafe extern "C" fn Task_HofPC_HandleInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                != 0i32
            {
                let __p1 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p1).write(((__p1).read()).wrapping_sub(1));
                {
                    i = 0u16;
                    'l1: loop {
                        if !(((i) as i32) < 6i32) {
                            break 'l1;
                        }
                        'l2: {
                            let mut spriteId: u8 = ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset((((i) as i32).wrapping_add(5i32)) as isize))
                            .read()) as u8);
                            if ((spriteId) as i32) != 255i32 {
                                FreeAndDestroyMonPicSprite(((spriteId) as u16));
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset((((i) as i32).wrapping_add(5i32)) as isize))
                                .write(255i16);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    != 0i32
                {
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                }
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_HofPC_DrawSpritesPrintText));
            } else {
                if (IsCryPlayingOrClearCrySongs()) != 0 {
                    StopCryAndClearCrySongs();
                    m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 256u16);
                }
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_HofPC_HandlePaletteOnExit));
            }
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                if (IsCryPlayingOrClearCrySongs()) != 0 {
                    StopCryAndClearCrySongs();
                    m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 256u16);
                }
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_HofPC_HandlePaletteOnExit));
            } else {
                if (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0)
                    && (((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        != 0i32)
                {
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    (__p3).write(((__p3).read()).wrapping_sub(1));
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_HofPC_PrintMonInfo));
                } else {
                    if (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0)
                        && (((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as i32)
                            < ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(4))
                            .read()) as i32)
                                .wrapping_sub(1i32))
                    {
                        let __p4 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2);
                        (__p4).write(((__p4).read()).wrapping_add(1));
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Task_HofPC_PrintMonInfo));
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HofPC_HandlePaletteOnExit(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut fameTeam: *mut u8 = core::ptr::null_mut();
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .cast::<u8>(),
                            (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .cast::<u8>(),
                            ((0i32
                                | (crate::c::div_i32(1024i32, crate::c::div_i32(16i32, 8i32))
                                    & 2097151i32)) as u32),
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
        fameTeam = (&raw mut gDecompressionBuffer).cast::<u8>();
        (fameTeam)
            .cast::<u8>()
            .cast::<crate::c::Rec4<20>>()
            .write_unaligned(
                (&raw const sDummyFameMon)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<20>>()
                    .read_unaligned(),
            );
        ComputerScreenCloseEffect(0u16, 0u16, 0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HofPC_HandleExit));
    }
}
pub(crate) unsafe extern "C" fn Task_HofPC_HandleExit(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((IsComputerScreenCloseEffectActive()) != 0) {
            let mut i: u8 = 0u8;
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        let mut spriteId: u16 = ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset((((i) as i32).wrapping_add(5i32)) as isize))
                        .read()) as u16);
                        if ((spriteId) as i32) != 255i32 {
                            FreeAndDestroyMonPicSprite(spriteId);
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset((((i) as i32).wrapping_add(5i32)) as isize))
                            .write(255i16);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            HideBg(0u8);
            HideBg(1u8);
            HideBg(3u8);
            HofPCTopBar_RemoveWindow();
            FreeAllWindowBuffers();
            UnsetBgTilemapBuffer(1u8);
            UnsetBgTilemapBuffer(3u8);
            ResetBgsAndClearDma3BusyFlags(0u32);
            DestroyTask(taskId);
            if ((((&raw mut sHofGfxPtr).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize
            {
                Free(((&raw mut sHofGfxPtr).cast::<u8>().cast::<*mut u8>()).read());
                ((&raw mut sHofGfxPtr).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
            }
            if ((((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize
            {
                Free(((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).read());
                ((&raw mut sHofMonPtr).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
            }
            ReturnFromHallOfFamePC();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HofPC_PrintDataIsCorrupted(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        HofPCTopBar_Print((&raw mut gText_AButtonExit).cast::<u8>(), 8u8, 1u8);
        DrawDialogueFrame(0u8, 0u8);
        AddTextPrinterParameterized2(
            0u8,
            1u8,
            (&raw mut gText_HOFCorrupted).cast::<u8>(),
            0u8,
            None,
            2u8,
            1u8,
            3u8,
        );
        CopyWindowToVram(0u8, 3u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HofPC_ExitOnButtonPress));
    }
}
pub(crate) unsafe extern "C" fn Task_HofPC_ExitOnButtonPress(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HofPC_HandlePaletteOnExit));
        }
    }
}
pub(crate) unsafe extern "C" fn HallOfFame_PrintWelcomeText(
    unusedPossiblyWindowId: u8,
    unused2: u8,
) {
    unsafe {
        let mut unusedPossiblyWindowId = unusedPossiblyWindowId;
        let mut unused2 = unused2;
        FillWindowPixelBuffer(0u8, 0u8);
        PutWindowTilemap(0u8);
        AddTextPrinterParameterized3(
            0u8,
            1u8,
            ((GetStringCenterAlignXOffset(1i32, (&raw mut gText_WelcomeToHOF).cast::<u8>(), 208i32))
                as u8),
            1u8,
            ((&raw const sMonInfoTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            (&raw mut gText_WelcomeToHOF).cast::<u8>(),
        );
        CopyWindowToVram(0u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn HallOfFame_PrintMonInfo(
    currMon: *mut u8,
    unused1: u8,
    unused2: u8,
) {
    unsafe {
        let mut currMon = currMon;
        let mut unused1 = unused1;
        let mut unused2 = unused2;
        let mut text = crate::ffi::Align4([0u8; 32]);
        let mut stringPtr: *mut u8 = core::ptr::null_mut();
        let mut dexNumber: i32 = 0i32;
        let mut width: i32 = 0i32;
        FillWindowPixelBuffer(0u8, 0u8);
        PutWindowTilemap(0u8);
        if ((crate::c::bf_read((currMon).wrapping_add(8), 0, 9, false) as u16) as i32) != 412i32 {
            stringPtr = StringCopy(
                (&raw mut text).cast::<u8>(),
                (&raw mut gText_Number).cast::<u8>(),
            );
            dexNumber = ((SpeciesToPokedexNum(
                (crate::c::bf_read((currMon).wrapping_add(8), 0, 9, false) as u16),
            )) as i32);
            if dexNumber != 65535i32 {
                (stringPtr)
                    .write((((crate::c::div_i32(dexNumber, 100i32)).wrapping_add(161i32)) as u8));
                stringPtr = (stringPtr).wrapping_offset(1);
                dexNumber = crate::c::rem_i32(dexNumber, 100i32);
                (stringPtr)
                    .write((((crate::c::div_i32(dexNumber, 10i32)).wrapping_add(161i32)) as u8));
                stringPtr = (stringPtr).wrapping_offset(1);
                (stringPtr)
                    .write((((crate::c::rem_i32(dexNumber, 10i32)).wrapping_add(161i32)) as u8));
                stringPtr = (stringPtr).wrapping_offset(1);
            } else {
                ({
                    let __t1 = stringPtr;
                    stringPtr = (stringPtr).wrapping_offset(1);
                    __t1
                })
                .write(172u8);
                ({
                    let __t2 = stringPtr;
                    stringPtr = (stringPtr).wrapping_offset(1);
                    __t2
                })
                .write(172u8);
                ({
                    let __t3 = stringPtr;
                    stringPtr = (stringPtr).wrapping_offset(1);
                    __t3
                })
                .write(172u8);
            }
            (stringPtr).write(255u8);
            AddTextPrinterParameterized3(
                0u8,
                1u8,
                16u8,
                1u8,
                ((&raw const sMonInfoTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                (&raw mut text).cast::<u8>(),
            );
        }
        crate::c::memcpy(
            (&raw mut text).cast::<u8>(),
            ((currMon).wrapping_add(10)).cast::<u8>(),
            10u32,
        );
        (((&raw mut text).cast::<u8>()).wrapping_offset(10)).write(255u8);
        if ((crate::c::bf_read((currMon).wrapping_add(8), 0, 9, false) as u16) as i32) == 412i32 {
            width = GetStringCenterAlignXOffset(1i32, (&raw mut text).cast::<u8>(), 208i32);
            AddTextPrinterParameterized3(
                0u8,
                1u8,
                ((width) as u8),
                1u8,
                ((&raw const sMonInfoTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                (&raw mut text).cast::<u8>(),
            );
            CopyWindowToVram(0u8, 3u8);
        } else {
            width = GetStringRightAlignXOffset(1i32, (&raw mut text).cast::<u8>(), 128i32);
            AddTextPrinterParameterized3(
                0u8,
                1u8,
                ((width) as u8),
                1u8,
                ((&raw const sMonInfoTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                (&raw mut text).cast::<u8>(),
            );
            ((&raw mut text).cast::<u8>()).write(186u8);
            stringPtr = StringCopy(
                ((&raw mut text).cast::<u8>()).wrapping_offset(1),
                (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                    ((crate::c::bf_read((currMon).wrapping_add(8), 0, 9, false) as u16) as i32)
                        as isize
                        * 11,
                ))
                .cast::<u8>(),
            );
            if (((crate::c::bf_read((currMon).wrapping_add(8), 0, 9, false) as u16) as i32)
                != 32i32)
                && (((crate::c::bf_read((currMon).wrapping_add(8), 0, 9, false) as u16) as i32)
                    != 29i32)
            {
                'l1: {
                    let __sw4 = ((GetGenderFromSpeciesAndPersonality(
                        (crate::c::bf_read((currMon).wrapping_add(8), 0, 9, false) as u16),
                        ((currMon).wrapping_add(4).cast::<u32>()).read(),
                    )) as i32);
                    if __sw4 == 0i32 {
                        (stringPtr).write(181u8);
                        stringPtr = (stringPtr).wrapping_offset(1);
                        break 'l1;
                    }
                    if __sw4 == 254i32 {
                        (stringPtr).write(182u8);
                        stringPtr = (stringPtr).wrapping_offset(1);
                        break 'l1;
                    }
                }
            }
            (stringPtr).write(255u8);
            AddTextPrinterParameterized3(
                0u8,
                1u8,
                128u8,
                1u8,
                ((&raw const sMonInfoTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                (&raw mut text).cast::<u8>(),
            );
            stringPtr = StringCopy(
                (&raw mut text).cast::<u8>(),
                (&raw mut gText_Level).cast::<u8>(),
            );
            ConvertIntToDecimalStringN(
                stringPtr,
                ((crate::c::bf_read((currMon).wrapping_add(9), 1, 7, false) as u16) as i32),
                0i32,
                3u8,
            );
            AddTextPrinterParameterized3(
                0u8,
                1u8,
                36u8,
                17u8,
                ((&raw const sMonInfoTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                (&raw mut text).cast::<u8>(),
            );
            stringPtr = StringCopy(
                (&raw mut text).cast::<u8>(),
                (&raw mut gText_IDNumber).cast::<u8>(),
            );
            ConvertIntToDecimalStringN(
                stringPtr,
                (((((currMon).cast::<u32>()).read()) as u16) as i32),
                2i32,
                5u8,
            );
            AddTextPrinterParameterized3(
                0u8,
                1u8,
                104u8,
                17u8,
                ((&raw const sMonInfoTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                (&raw mut text).cast::<u8>(),
            );
            CopyWindowToVram(0u8, 3u8);
        }
    }
}
pub(crate) unsafe extern "C" fn HallOfFame_PrintPlayerInfo(unused1: u8, unused2: u8) {
    unsafe {
        let mut unused1 = unused1;
        let mut unused2 = unused2;
        let mut text = crate::ffi::Align4([0u8; 20]);
        let mut width: u32 = 0u32;
        let mut trainerId: u16 = 0u16;
        FillWindowPixelBuffer(1u8, 17u8);
        PutWindowTilemap(1u8);
        DrawStdFrameWithCustomTileAndPalette(1u8, 0u8, 541u16, 13u8);
        AddTextPrinterParameterized3(
            1u8,
            1u8,
            0u8,
            1u8,
            ((&raw const sPlayerInfoTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            (&raw mut gText_Name).cast::<u8>(),
        );
        width = ((GetStringRightAlignXOffset(
            1i32,
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            112i32,
        )) as u32);
        AddTextPrinterParameterized3(
            1u8,
            1u8,
            ((width) as u8),
            1u8,
            ((&raw const sPlayerInfoTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        trainerId = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
            .cast::<u8>())
        .read()) as i32)
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                << 8)) as u16);
        AddTextPrinterParameterized3(
            1u8,
            1u8,
            0u8,
            17u8,
            ((&raw const sPlayerInfoTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            (&raw mut gText_IDNumber).cast::<u8>(),
        );
        ((&raw mut text).cast::<u8>()).write(
            (((crate::c::div_i32(crate::c::rem_i32(((trainerId) as i32), 100000i32), 10000i32))
                .wrapping_add(161i32)) as u8),
        );
        (((&raw mut text).cast::<u8>()).wrapping_offset(1)).write(
            (((crate::c::div_i32(crate::c::rem_i32(((trainerId) as i32), 10000i32), 1000i32))
                .wrapping_add(161i32)) as u8),
        );
        (((&raw mut text).cast::<u8>()).wrapping_offset(2)).write(
            (((crate::c::div_i32(crate::c::rem_i32(((trainerId) as i32), 1000i32), 100i32))
                .wrapping_add(161i32)) as u8),
        );
        (((&raw mut text).cast::<u8>()).wrapping_offset(3)).write(
            (((crate::c::div_i32(crate::c::rem_i32(((trainerId) as i32), 100i32), 10i32))
                .wrapping_add(161i32)) as u8),
        );
        (((&raw mut text).cast::<u8>()).wrapping_offset(4)).write(
            (((crate::c::div_i32(crate::c::rem_i32(((trainerId) as i32), 10i32), 1i32))
                .wrapping_add(161i32)) as u8),
        );
        (((&raw mut text).cast::<u8>()).wrapping_offset(5)).write(255u8);
        width = ((GetStringRightAlignXOffset(1i32, (&raw mut text).cast::<u8>(), 112i32)) as u32);
        AddTextPrinterParameterized3(
            1u8,
            1u8,
            ((width) as u8),
            17u8,
            ((&raw const sPlayerInfoTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            (&raw mut text).cast::<u8>(),
        );
        AddTextPrinterParameterized3(
            1u8,
            1u8,
            0u8,
            33u8,
            ((&raw const sPlayerInfoTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            (&raw mut gText_Time).cast::<u8>(),
        );
        ((&raw mut text).cast::<u8>()).write(
            (((crate::c::div_i32(
                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(14)
                    .cast::<u16>())
                .read()) as i32),
                100i32,
            ))
            .wrapping_add(161i32)) as u8),
        );
        (((&raw mut text).cast::<u8>()).wrapping_offset(1)).write(
            (((crate::c::div_i32(
                crate::c::rem_i32(
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(14)
                        .cast::<u16>())
                    .read()) as i32),
                    100i32,
                ),
                10i32,
            ))
            .wrapping_add(161i32)) as u8),
        );
        (((&raw mut text).cast::<u8>()).wrapping_offset(2)).write(
            (((crate::c::rem_i32(
                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(14)
                    .cast::<u16>())
                .read()) as i32),
                10i32,
            ))
            .wrapping_add(161i32)) as u8),
        );
        if ((((&raw mut text).cast::<u8>()).read()) as i32) == 161i32 {
            ((&raw mut text).cast::<u8>()).write(0u8);
        }
        if (((((&raw mut text).cast::<u8>()).read()) as i32) == 0i32)
            && ((((((&raw mut text).cast::<u8>()).wrapping_offset(1)).read()) as i32) == 161i32)
        {
            (((&raw mut text).cast::<u8>()).wrapping_offset(8)).write(0u8);
        }
        (((&raw mut text).cast::<u8>()).wrapping_offset(3)).write(240u8);
        (((&raw mut text).cast::<u8>()).wrapping_offset(4)).write(
            (((crate::c::div_i32(
                crate::c::rem_i32(
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(16))
                        .read()) as i32),
                    100i32,
                ),
                10i32,
            ))
            .wrapping_add(161i32)) as u8),
        );
        (((&raw mut text).cast::<u8>()).wrapping_offset(5)).write(
            (((crate::c::rem_i32(
                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(16)).read())
                    as i32),
                10i32,
            ))
            .wrapping_add(161i32)) as u8),
        );
        (((&raw mut text).cast::<u8>()).wrapping_offset(6)).write(255u8);
        width = ((GetStringRightAlignXOffset(1i32, (&raw mut text).cast::<u8>(), 112i32)) as u32);
        AddTextPrinterParameterized3(
            1u8,
            1u8,
            ((width) as u8),
            33u8,
            ((&raw const sPlayerInfoTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            (&raw mut text).cast::<u8>(),
        );
        CopyWindowToVram(1u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn ClearVramOamPltt_LoadHofPal() {
    unsafe {
        {
            let mut _dest: *mut u8 = ((100663296i32) as usize as *mut u8);
            let mut _size: u32 = 98304u32;
            'l1: loop {
                if !((1i32) != 0) {
                    break 'l1;
                }
                'l2: loop {
                    'l3: {
                        {
                            let mut tmp: u16 = 0u16;
                            (&raw mut tmp).write_volatile(0u16);
                            'l4: loop {
                                'l5: {
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
                                        let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                    }
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
                _dest = (_dest).wrapping_offset(4096);
                _size = (_size).wrapping_sub(4096u32);
                if _size <= 4096u32 {
                    'l6: loop {
                        'l7: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l8: loop {
                                    'l9: {
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
                                        break 'l8;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l6;
                        }
                    }
                    break 'l1;
                }
            }
        }
        'l10: loop {
            'l11: {
                {
                    let mut _dest: *mut u32 = ((117440512i32) as usize as *mut u8).cast::<u32>();
                    let mut _size: u32 = 1024u32;
                    'l12: loop {
                        'l13: {
                            {
                                let mut tmp: u32 = 0u32;
                                (&raw mut tmp).write_volatile(0u32);
                                'l14: loop {
                                    'l15: {
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
                                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l14;
                                    }
                                }
                            }
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
        'l16: loop {
            'l17: {
                {
                    let mut _dest: *mut u16 = ((83886080i32) as usize as *mut u8).cast::<u16>();
                    let mut _size: u32 = 1024u32;
                    'l18: loop {
                        'l19: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l20: loop {
                                    'l21: {
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
                                        break 'l20;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l18;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l16;
            }
        }
        ResetPaletteFade();
        LoadPalette(
            (((&raw const sHallOfFame_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            0u16,
            32u16,
        );
    }
}
pub(crate) unsafe extern "C" fn LoadHofGfx() {
    unsafe {
        ScanlineEffect_Stop();
        ResetTasks();
        ResetSpriteData();
        ResetTempTileDataBuffers();
        ResetAllPicSprites();
        FreeAllSpritePalettes();
        ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(8u8);
        LoadCompressedSpriteSheet(
            ((&raw const sSpriteSheet_Confetti).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        LoadCompressedSpritePalette(
            ((&raw const sSpritePalette_Confetti).cast::<u8>().cast_mut()).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn InitHofBgs() {
    unsafe {
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sHof_BgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(12u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(
            1u8,
            ((((&raw mut sHofGfxPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(18))
                .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            3u8,
            ((((&raw mut sHofGfxPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4114))
                .cast::<u8>(),
        );
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        ChangeBgX(1u8, 0i32, 0u8);
        ChangeBgY(1u8, 0i32, 0u8);
        ChangeBgX(3u8, 0i32, 0u8);
        ChangeBgY(3u8, 0i32, 0u8);
    }
}
pub(crate) unsafe extern "C" fn LoadHofBgs() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sHofGfxPtr).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                DecompressAndCopyTileDataToVram(
                    1u8,
                    (((&raw const sHallOfFame_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 1u8;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                FillBgTilemapBufferRect_Palette0(1u8, 1u16, 0u8, 0u8, 32u8, 2u8);
                FillBgTilemapBufferRect_Palette0(1u8, 0u16, 0u8, 3u8, 32u8, 11u8);
                FillBgTilemapBufferRect_Palette0(1u8, 1u16, 0u8, 14u8, 32u8, 6u8);
                FillBgTilemapBufferRect_Palette0(3u8, 2u16, 0u8, 0u8, 32u8, 32u8);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(3u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                InitStandardTextBoxWindows();
                InitTextBoxGfxAndPrinters();
                break 'l1;
            }
            if __sw1 == 4i32 {
                SetGpuReg(0u8, 4160u16);
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(3u8);
                ((((&raw mut sHofGfxPtr).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                    .write(0u16);
                return 0u8;
            }
        }
        let __p2 = (((&raw mut sHofGfxPtr).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_GetOnScreenAndAnimate(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32))
            || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32))
        {
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            {
                let __p1 = (sprite).wrapping_add(32).cast::<i16>();
                (__p1).write((((((__p1).read()) as i32).wrapping_add(15i32)) as i16));
            }
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            {
                let __p2 = (sprite).wrapping_add(32).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_sub(15i32)) as i16));
            }
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            {
                let __p3 = (sprite).wrapping_add(34).cast::<i16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_add(10i32)) as i16));
            }
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            {
                let __p4 = (sprite).wrapping_add(34).cast::<i16>();
                (__p4).write((((((__p4).read()) as i32).wrapping_sub(10i32)) as i16));
            }
        } else {
            let mut species: i16 =
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read();
            if ((species) as i32) == 412i32 {
                DoMonFrontSpriteAnimation(sprite, ((species) as u16), 1u8, 3u8);
            } else {
                DoMonFrontSpriteAnimation(sprite, ((species) as u16), 0u8, 3u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_HofConfetti(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) > 120i32 {
            DestroySprite(sprite);
        } else {
            let mut rand: u16 = 0u16;
            let mut sineIdx: u8 = 0u8;
            let __p1 = (sprite).wrapping_add(38).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            let __p2 = (sprite).wrapping_add(38).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            sineIdx = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8);
            rand = (((crate::c::rem_i32(((Random()) as i32), 4i32)).wrapping_add(8i32)) as u16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((crate::c::div_i32(
                    ((rand) as i32).wrapping_mul(
                        ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(((sineIdx) as i32) as isize))
                        .read()) as i32),
                    ),
                    256i32,
                )) as i16),
            );
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_add(4i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn CreateHofConfettiSprite() -> u8 {
    unsafe {
        let mut spriteID: u8 = 0u8;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        let mut posX: i16 = ((crate::c::rem_i32(((Random()) as i32), 240i32)) as i16);
        let mut posY: i16 =
            (((crate::c::rem_i32(((Random()) as i32), 8i32)).wrapping_neg()) as i16);
        spriteID = CreateSprite(
            (&raw const sSpriteTemplate_HofConfetti)
                .cast::<u8>()
                .cast_mut(),
            posX,
            posY,
            0u8,
        );
        sprite =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteID) as i32) as isize * 68);
        StartSpriteAnim(
            sprite,
            ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(68u32, 4u32))) as u8),
        );
        if (((Random()) as i32) & 3i32) != 0 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(1i16);
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoDomeConfetti() {
    unsafe {
        let mut taskId: u8 = 0u8;
        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(180u16);
        taskId = CreateTask(Some(Task_DoDomeConfetti), 0u8);
        if ((taskId) as i32) != 255i32 {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i16));
            ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(((taskId) as u16));
        }
    }
}
pub(crate) unsafe extern "C" fn StopDomeConfetti() {
    unsafe {
        let mut taskId: u8 = 0u8;
        if (({
            let __v1 = FindTaskIdByFunc(Some(Task_DoDomeConfetti));
            taskId = __v1;
            __v1
        }) as i32)
            != 255i32
        {
            DestroyTask(taskId);
        }
        ConfettiUtil_Free();
        FreeSpriteTilesByTag(1001u16);
        FreeSpritePaletteByTag(1001u16);
    }
}
pub(crate) unsafe extern "C" fn UpdateDomeConfetti(util: *mut u8) {
    unsafe {
        let mut util = util;
        if ((((util).wrapping_add(14).cast::<i16>()).read()) as i32) > 110i32 {
            let __p1 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((util).wrapping_add(26)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                    as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            ConfettiUtil_Remove(((util).wrapping_add(22)).read());
        } else {
            let mut sineIdx: u8 = 0u8;
            let mut rand: i32 = 0i32;
            let __p2 = (util).wrapping_add(14).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
            let __p3 = (util).wrapping_add(14).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((util).wrapping_add(26)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            sineIdx = (((((util).wrapping_add(26)).cast::<i16>()).read()) as u8);
            rand = ((Random()) as i32);
            rand = (rand & 3i32);
            rand = (rand).wrapping_add(8i32);
            ((util).wrapping_add(12).cast::<i16>()).write(
                ((crate::c::div_i32(
                    (rand).wrapping_mul(
                        ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(((sineIdx) as i32) as isize))
                        .read()) as i32),
                    ),
                    256i32,
                )) as i16),
            );
            let __p4 = ((util).wrapping_add(26)).cast::<i16>();
            (__p4).write((((((__p4).read()) as i32).wrapping_add(4i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DoDomeConfetti(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut id: u32 = 0u32;
        let mut data: *mut u16 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                if !((ConfettiUtil_Init(64u8)) != 0) {
                    DestroyTask(taskId);
                    ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(0u16);
                    ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(65535u16);
                }
                LoadCompressedSpriteSheet(
                    ((&raw const sSpriteSheet_Confetti).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                LoadCompressedSpritePalette(
                    ((&raw const sSpritePalette_Confetti).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                (data).write(((data).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((data).wrapping_offset(1)).read()) as i32) != 0i32)
                    && (crate::c::rem_i32(((((data).wrapping_offset(1)).read()) as i32), 3i32)
                        == 0i32)
                {
                    id = ((ConfettiUtil_AddNew(
                        (&raw const sOamData_Confetti).cast::<u8>().cast_mut(),
                        1001u16,
                        1001u16,
                        ((crate::c::rem_i32(((Random()) as i32), 240i32)) as i16),
                        (((crate::c::rem_i32(((Random()) as i32), 8i32)).wrapping_neg()) as i16),
                        ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(68u32, 4u32)))
                            as u8),
                        ((id) as u8),
                    )) as u32);
                    if id != 255u32 {
                        ConfettiUtil_SetCallback(((id) as u8), Some(UpdateDomeConfetti));
                        if crate::c::rem_i32(((Random()) as i32), 4i32) == 0i32 {
                            ConfettiUtil_SetData(((id) as u8), 1u8, 1i16);
                        }
                        ConfettiUtil_SetData(((id) as u8), 7u8, ((taskId) as i16));
                        let __p2 = (data).wrapping_offset(15);
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                }
                ConfettiUtil_Update();
                if ((((data).wrapping_offset(1)).read()) as i32) != 0i32 {
                    let __p3 = (data).wrapping_offset(1);
                    (__p3).write(((__p3).read()).wrapping_sub(1));
                } else {
                    if ((((data).wrapping_offset(15)).read()) as i32) == 0i32 {
                        (data).write(255u16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 255i32 {
                StopDomeConfetti();
                ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(0u16);
                ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(65535u16);
                break 'l1;
            }
        }
    }
}
