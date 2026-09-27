//! Translated from `src/roulette.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sWheel_Pal sGrid_Tilemap sWheel_Tilemap sBgTemplates sWindowTemplates sGridSelections sRouletteSlots sTableMinBets sRouletteTables sFlashData_Colors sFlashData_PokeIcons sYesNoTable_AcceptMinBet sYesNoTable_KeepPlaying sFiller sShadow_Pal sBall_Pal sBallCounter_Pal sCursor_Pal sCredit_Pal sShroomish_Pal sTaillow_Pal sGridIcons_Pal sWynaut_Pal sAzurill_Pal sSkitty_Pal sMakuhita_Pal sUnused1_Pal sUnused2_Pal sUnused3_Pal sUnused4_Pal sBall_Gfx sBallCounter_Gfx sShroomishTaillow_Gfx sGridIcons_Gfx sWheelIcons_Gfx sShadow_Gfx sCursor_Gfx sSpritePalettes sOam_GridHeader sOam_GridIcon sOam_WheelIcon sAffineAnim_Unused1 sAffineAnims_Unused1 sAffineAnim_Unused2 sAffineAnims_Unused2 sSpriteSheet_WheelIcons sAnim_WheelIcons sAnim_WheelIcon_OrangeWynaut sAnim_WheelIcon_GreenAzurill sAnim_WheelIcon_PurpleSkitty sAnim_WheelIcon_OrangeMakuhita sAnim_WheelIcon_GreenWynaut sAnim_WheelIcon_PurpleAzurill sAnim_WheelIcon_OrangeSkitty sAnim_WheelIcon_GreenMakuhita sAnim_WheelIcon_PurpleWynaut sAnim_WheelIcon_OrangeAzurill sAnim_WheelIcon_GreenSkitty sAnim_WheelIcon_PurpleMakuhita sSpriteSheet_Headers sSpriteSheet_GridIcons sAnim_Headers sAnim_GridIcons sAnim_WynautHeader sAnim_AzurillHeader sAnim_SkittyHeader sAnim_MakuhitaHeader sAnim_OrangeHeader sAnim_GreenHeader sAnim_PurpleHeader sAnim_GridIcon_Wynaut sAnim_GridIcon_Azurill sAnim_GridIcon_Skitty sAnim_GridIcon_Makuhita sSpriteTemplates_PokeHeaders sSpriteTemplates_ColorHeaders sSpriteTemplates_GridIcons sSpriteTemplates_WheelIcons sOam_Credit sOam_CreditDigit sOam_Multiplier sOam_BallCounter sSpriteSheets_Interface sAnim_CreditDigit sAnims_CreditDigit sAnim_Multiplier sAnims_Multiplier sAnim_BallCounter sAnims_BallCounter sSpriteTemplate_Credit sSpriteTemplate_CreditDigit sSpriteTemplate_Multiplier sSpriteTemplate_BallCounter sSpriteTemplate_Cursor sOam_Ball sSpriteSheet_Ball sAnim_Ball_RollFast sAnim_Ball_RollMedium sAnim_Ball_RollSlow sAnim_Ball_StopOnFrame1 sAnim_Ball_StopOnFrame3 sAnim_Ball_StopOnFrame4 sAnim_Ball_Still sAnim_Ball_StopOnFrame2 sAnims_Ball sSpriteTemplate_Ball sOam_WheelCenter sSpriteSheet_WheelCenter sSpriteTemplate_WheelCenter sOam_Shroomish sOam_Taillow sSpriteSheet_ShroomishTaillow sAnim_Shroomish sAnim_Taillow_WingDown_Left sAnim_Taillow_WingDown_Right sAnim_Taillow_FlapSlow_Left sAnim_Taillow_FlapSlow_Right sAnim_Taillow_FlapFast_Left sAnim_Taillow_FlapFast_Right sAnims_Shroomish sAnims_Taillow sSpriteTemplate_Shroomish sSpriteTemplate_Taillow sOam_ShroomishBallShadow sOam_ShroomishShadow sOam_TaillowShadow sSpriteSheet_Shadow sAffineAnim_Unused3 sAffineAnim_TaillowShadow sAffineAnims_Unused3 sAffineAnims_TaillowShadow sAffineAnim_Unused4 sAffineAnims_Unused4 sAnim_ShroomishBallShadow sAnim_UnstickMonShadow sAnims_ShroomishBallShadow sAnims_UnstickMonShadow sSpriteTemplate_ShroomishShadow sSpriteTemplate_TaillowShadow sShroomishShadowAlphas
#[allow(unused_imports)]
use crate::data::roulette::*;

pub(crate) static mut sRoulette: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTextWindowId: u8 = 0u8;

unsafe extern "C" {
    static mut Roulette_Text_BoardWillBeCleared: u8;
    static mut Roulette_Text_CoinCaseIsFull: u8;
    static mut Roulette_Text_ControlsInstruction: u8;
    static mut Roulette_Text_ItsAHit: u8;
    static mut Roulette_Text_Jackpot: u8;
    static mut Roulette_Text_KeepPlaying: u8;
    static mut Roulette_Text_NoCoinsLeft: u8;
    static mut Roulette_Text_NotEnoughCoins: u8;
    static mut Roulette_Text_NothingDoing: u8;
    static mut Roulette_Text_PlayMinimumWagerIsX: u8;
    static mut Roulette_Text_SpecialRateTable: u8;
    static mut Roulette_Text_YouveWonXCoins: u8;
    static mut gDecompressionBuffer: u8;
    static mut gFieldCallback: u8;
    static mut gLocalTime: u8;
    static mut gMPlayInfo_SE1: u8;
    static mut gMPlayInfo_SE2: u8;
    static mut gMain: u8;
    static mut gOamMatrices: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gRouletteMenu_Gfx: u8;
    static mut gRouletteWheel_Gfx: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpriteCoordOffsetX: u8;
    static mut gSpriteCoordOffsetY: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AlertTVThatPlayerPlayedRoulette(a0: u16);
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginHardwarePaletteFade(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8);
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_ReturnToField();
    fn ClearStdWindowAndFrame(a0: u8, a1: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn Cos2(a0: u16) -> i16;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DisplayYesNoMenuDefaultYes();
    fn DoYesNoFuncWithChoice(a0: u8, a1: *mut u8);
    fn DrawStdWindowFrame(a0: u8, a1: u8);
    fn EnableInterrupts(a0: u16);
    fn FieldCB_ContinueScriptHandleMusic();
    fn FillTilemapRect(a0: *mut u16, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeOamMatrix(a0: u8);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetCoins() -> u16;
    fn GetGameStat(a0: u8) -> u32;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn HideCoinsWindow();
    fn IncrementDailyRouletteUses();
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitTextBoxGfxAndPrinters();
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsFanfareTaskInactive() -> u8;
    fn IsSEPlaying() -> u8;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut u8);
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LockPlayerFieldControls();
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlayFanfare(a0: u16);
    fn PlaySE(a0: u16);
    fn PrintCoinsString(a0: u32);
    fn ProcessSpriteCopyRequests();
    fn Random() -> u16;
    fn ResetAllBgsCoordinates();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn ResetVramOamAndBgCntRegs();
    fn RouletteFlash_Add(a0: *mut u8, a1: u8, a2: *mut u8) -> u8;
    fn RouletteFlash_Enable(a0: *mut u8, a1: u16);
    fn RouletteFlash_Reset(a0: *mut u8);
    fn RouletteFlash_Run(a0: *mut u8);
    fn RouletteFlash_Stop(a0: *mut u8, a1: u16);
    fn RtcCalcLocalTime();
    fn RunTasks();
    fn ScanlineEffect_Stop();
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetCoins(a0: u16);
    fn SetGameStat(a0: u8, a1: u32);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetTilemapRect(a0: *mut u16, a1: *mut u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankHBlankCallbacksToNull();
    fn ShowBg(a0: u8);
    fn ShowCoinsWindow(a0: u32, a1: u8, a2: u8);
    fn Sin2(a0: u16) -> i16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn TryPutFindThatGamerOnAir(a0: u16);
    fn UnlockPlayerFieldControls();
    fn UnsetBgTilemapBuffer(a0: u8);
    fn UpdatePaletteFade() -> u8;
    fn m4aMPlayPanpotControl(a0: *mut u8, a1: u16, a2: i8);
    fn m4aSongNumStart(a0: u16);
    fn m4aSongNumStartOrChange(a0: u16);
    fn m4aSongNumStop(a0: u16);
    fn malloc_and_decompress(a0: *mut u8, a1: *mut u32) -> *mut u8;
}

pub(crate) unsafe extern "C" fn CB2_Roulette() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        if (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(184))
            .read())
            != 0
        {
            RouletteFlash_Run(
                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(184),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_Roulette() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
        UpdateWheelPosition();
        SetGpuReg(
            20u8,
            (((512i32).wrapping_sub(
                ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(38)
                    .cast::<i16>())
                .read()) as i32),
            )) as u16),
        );
        if (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            != 0
        {
            SetGpuReg(
                82u8,
                ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52)
                    .cast::<u16>())
                .read(),
            );
        }
        if (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(42)
            .cast::<i16>())
        .read())
            != 0
        {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(
                                    dmaRegs,
                                    (((((((((&raw mut sRoulette)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(380))
                                    .cast::<u8>())
                                    .wrapping_offset(4096))
                                    .cast::<u16>())
                                    .wrapping_offset(224))
                                        as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    ((((100671488i32) as usize as *mut u8).wrapping_offset(448))
                                        as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2147483648i32)
                                        | crate::c::div_i32(832i32, crate::c::div_i32(16i32, 8i32)))
                                        as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
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
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(42)
                .cast::<i16>())
            .write(0i16);
        }
        'l5: {
            let __sw1 = ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(40)
                .cast::<i16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 1i32 {
                __fall = true;
                SetBgAttribute(0u8, 1u8, 0u8);
                ShowBg(0u8);
                'l6: loop {
                    'l7: {
                        'l8: loop {
                            'l9: {
                                {
                                    let mut dmaRegs: *mut u32 =
                                        ((67109076i32) as usize as *mut u32);
                                    crate::c::volatile_write(
                                        dmaRegs,
                                        ((((((((&raw mut sRoulette)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(380))
                                        .cast::<u8>())
                                        .cast::<u16>())
                                        .wrapping_offset(224))
                                            as usize
                                            as u32),
                                    );
                                    crate::c::volatile_write(
                                        (dmaRegs).wrapping_offset(1),
                                        ((((100726784i32) as usize as *mut u8).wrapping_offset(448))
                                            as usize
                                            as u32),
                                    );
                                    crate::c::volatile_write(
                                        (dmaRegs).wrapping_offset(2),
                                        (((-2147483648i32)
                                            | crate::c::div_i32(
                                                832i32,
                                                crate::c::div_i32(16i32, 8i32),
                                            )) as u32),
                                    );
                                    let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                }
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
                ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(40)
                    .cast::<i16>())
                .write(2i16);
                break 'l5;
            }
            if __sw1 == 2i32 {
                __fall = true;
                'l10: loop {
                    'l11: {
                        'l12: loop {
                            'l13: {
                                {
                                    let mut dmaRegs: *mut u32 =
                                        ((67109076i32) as usize as *mut u32);
                                    crate::c::volatile_write(
                                        dmaRegs,
                                        ((((((((&raw mut sRoulette)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(380))
                                        .cast::<u8>())
                                        .cast::<u16>())
                                        .wrapping_offset(224))
                                            as usize
                                            as u32),
                                    );
                                    crate::c::volatile_write(
                                        (dmaRegs).wrapping_offset(1),
                                        ((((100726784i32) as usize as *mut u8).wrapping_offset(448))
                                            as usize
                                            as u32),
                                    );
                                    crate::c::volatile_write(
                                        (dmaRegs).wrapping_offset(2),
                                        (((-2147483648i32)
                                            | crate::c::div_i32(
                                                832i32,
                                                crate::c::div_i32(16i32, 8i32),
                                            )) as u32),
                                    );
                                    let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                }
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
                break 'l5;
            }
            if __sw1 == 255i32 {
                __fall = true;
                SetBgAttribute(0u8, 1u8, 2u8);
                ShowBg(0u8);
                'l14: loop {
                    'l15: {
                        {
                            let mut tmp: u16 = 0u16;
                            (&raw mut tmp).write_volatile(0u16);
                            'l16: loop {
                                'l17: {
                                    {
                                        let mut dmaRegs: *mut u32 =
                                            ((67109076i32) as usize as *mut u32);
                                        crate::c::volatile_write(
                                            dmaRegs,
                                            ((&raw mut tmp) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(1),
                                            ((((100726784i32) as usize as *mut u8)
                                                .wrapping_offset(448))
                                                as usize
                                                as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(2),
                                            (((-2130706432i32)
                                                | crate::c::div_i32(
                                                    832i32,
                                                    crate::c::div_i32(16i32, 8i32),
                                                ))
                                                as u32),
                                        );
                                        let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l16;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l14;
                    }
                }
                ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(40)
                    .cast::<i16>())
                .write(0i16);
            }
            if __fall || __sw1 == 0i32 {
                __fall = true;
                break 'l5;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitRouletteBgAndWindows() {
    unsafe {
        let mut size: u32 = 0u32;
        ((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(14720u32));
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            1u8,
            ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(12u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(
            0u8,
            ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(380))
                .cast::<u8>())
            .cast::<u16>())
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            1u8,
            (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(380))
            .cast::<u8>())
            .wrapping_offset(4096))
            .cast::<u16>())
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            2u8,
            (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(380))
            .cast::<u8>())
            .wrapping_offset(12288))
            .cast::<u16>())
            .cast::<u8>(),
        );
        InitWindows(((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>());
        InitTextBoxGfxAndPrinters();
        ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).write(0u8);
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14716)
            .cast::<*mut u16>())
        .write(
            (malloc_and_decompress(
                (((&raw const sGrid_Tilemap)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u32>())
                .cast::<u32>())
                .cast::<u8>(),
                &raw mut size,
            ))
            .cast::<u16>(),
        );
    }
}
pub(crate) unsafe extern "C" fn FreeRoulette() {
    unsafe {
        {
            Free(
                (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(14716)
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
            );
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14716)
                .cast::<*mut u16>())
            .write(core::ptr::null_mut());
        }
        FreeAllWindowBuffers();
        UnsetBgTilemapBuffer(0u8);
        UnsetBgTilemapBuffer(1u8);
        UnsetBgTilemapBuffer(2u8);
        ResetBgsAndClearDma3BusyFlags(0u32);
        crate::c::memset(
            ((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read(),
            0i32,
            14720u32,
        );
        {
            Free(((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn InitRouletteTableData() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut bgColors = crate::ffi::Align4([0u8; 6]);
        (&raw mut bgColors)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(10392u16);
        (&raw mut bgColors)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<u16>()
            .write(6762u16);
        (&raw mut bgColors)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(10392u16);
        crate::c::bf_write(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4),
            0,
            2,
            ((((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) & 1i32) as u8) as i32,
        );
        if (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) & 128i32) != 0 {
            crate::c::bf_write(
                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4),
                7,
                1,
                (1u8) as i32,
            );
        }
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(34)).write(
            (((((&raw const sRouletteTables).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((crate::c::bf_read(
                        (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4),
                        0,
                        2,
                        false,
                    ) as u8) as i32) as isize
                        * 32,
                ))
            .wrapping_add(3))
            .read(),
        );
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(35)).write(
            (((((&raw const sRouletteTables).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((crate::c::bf_read(
                        (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4),
                        0,
                        2,
                        false,
                    ) as u8) as i32) as isize
                        * 32,
                ))
            .wrapping_add(4))
            .read(),
        );
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25)).write(
            ((((&raw const sTableMinBets).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(
                (((crate::c::bf_read(
                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4),
                    0,
                    2,
                    false,
                ) as u8) as i32)
                    .wrapping_add(
                        ((crate::c::bf_read(
                            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4),
                            7,
                            1,
                            false,
                        ) as u8) as i32)
                            .wrapping_mul(2i32),
                    )) as isize,
            ))
            .read(),
        );
        crate::c::bf_write(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26),
            4,
            4,
            (1u8) as i32,
        );
        if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
            .read()) as i32)
            == 1i32
        {
            (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).write({
                let __v3 = {
                    let __v2 = {
                        let __v1 = ((&raw mut bgColors).cast::<u16>()).read();
                        ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(81))
                        .write(__v1);
                        __v1
                    };
                    (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).write(__v2);
                    __v2
                };
                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).wrapping_offset(81))
                    .write(__v3);
                __v3
            });
        } else {
            (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).write({
                let __v6 = {
                    let __v5 = {
                        let __v4 = (((&raw mut bgColors).cast::<u16>()).wrapping_offset(1)).read();
                        ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(81))
                        .write(__v4);
                        __v4
                    };
                    (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).write(__v5);
                    __v5
                };
                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).wrapping_offset(81))
                    .write(__v6);
                __v6
            });
        }
        RouletteFlash_Reset(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(184),
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 13i32) {
                    break 'l1;
                }
                'l2: {
                    RouletteFlash_Add(
                        (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(184),
                        i,
                        (((&raw const sFlashData_Colors).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l3;
                }
                'l4: {
                    'l5: {
                        let __sw7 = GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            65i32,
                        );
                        if __sw7 == 306u32 {
                            let __p8 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(2);
                            (__p8).write((((((__p8).read()) as i32) | 1i32) as u8));
                            break 'l5;
                        }
                        if __sw7 == 304u32 {
                            let __p9 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(2);
                            (__p9).write((((((__p9).read()) as i32) | 2i32) as u8));
                            break 'l5;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        RtcCalcLocalTime();
    }
}
pub(crate) unsafe extern "C" fn CB2_LoadRoulette() {
    unsafe {
        let mut taskId: u8 = 0u8;
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                SetVBlankCallback(None);
                ScanlineEffect_Stop();
                SetVBlankHBlankCallbacksToNull();
                ResetVramOamAndBgCntRegs();
                ResetAllBgsCoordinates();
                break 'l1;
            }
            if __sw1 == 1i32 {
                InitRouletteBgAndWindows();
                DeactivateAllTextPrinters();
                SetGpuReg(80u8, 9216u16);
                SetGpuReg(82u8, 1546u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetPaletteFade();
                ResetSpriteData();
                ResetTasks();
                ResetTempTileDataBuffers();
                break 'l1;
            }
            if __sw1 == 3i32 {
                LoadPalette(
                    (((&raw const sWheel_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    0u16,
                    448u16,
                );
                DecompressAndCopyTileDataToVram(
                    1u8,
                    (((&raw mut gRouletteMenu_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                DecompressAndCopyTileDataToVram(
                    2u8,
                    (((&raw mut gRouletteWheel_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return;
                }
                InitRouletteTableData();
                CopyToBgTilemapBuffer(
                    2u8,
                    (((&raw const sWheel_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u16,
                    0u16,
                );
                break 'l1;
            }
            if __sw1 == 5i32 {
                LoadOrFreeMiscSpritePalettesAndSheets(0u8);
                CreateWheelBallSprites();
                CreateWheelCenterSprite();
                CreateInterfaceSprites();
                CreateGridSprites();
                CreateGridBallSprites();
                CreateWheelIconSprites();
                break 'l1;
            }
            if __sw1 == 6i32 {
                AnimateSprites();
                BuildOamBuffer();
                SetCreditDigits(GetCoins());
                SetBallCounterNumLeft(6u8);
                SetMultiplierSprite(0u8);
                DrawGridBackground(0u8);
                DrawStdWindowFrame(
                    ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                    0u8,
                );
                AddTextPrinterParameterized(
                    ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                    1u8,
                    (&raw mut Roulette_Text_ControlsInstruction).cast::<u8>(),
                    0u8,
                    1u8,
                    255u8,
                    None,
                );
                CopyWindowToVram(
                    ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                    3u8,
                );
                ((&raw mut gSpriteCoordOffsetX).cast::<i16>()).write((-60i16));
                ((&raw mut gSpriteCoordOffsetY).cast::<i16>()).write(0i16);
                break 'l1;
            }
            if __sw1 == 7i32 {
                SetGpuReg(0u8, 4160u16);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(2u8);
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                break 'l1;
            }
            if __sw1 == 8i32 {
                EnableInterrupts(1u16);
                SetVBlankCallback(Some(VBlankCB_Roulette));
                BeginHardwarePaletteFade(255u8, 0u8, 16u8, 0u8, 1u8);
                taskId = {
                    let __v2 = CreateTask(Some(Task_StartPlaying), 0u8);
                    ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(164))
                    .write(__v2);
                    __v2
                };
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(6i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(13))
                .write(((GetCoins()) as i16));
                AlertTVThatPlayerPlayedRoulette(GetCoins());
                ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(165))
                    .write(CreateTask(Some(Task_SpinWheel), 1u8));
                SetMainCallback2(Some(CB2_Roulette));
                return;
            }
        }
        let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
        (__p3).write(((__p3).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Task_SpinWheel(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut sin: i16 = 0i16;
        let mut cos: i16 = 0i16;
        if (({
            let __p1 =
                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(33);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            == ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(35))
                .read()) as i32)
        {
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(33))
                .write(0u8);
            if (({
                let __p3 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36)
                    .cast::<i16>();
                let __v4 = (((((__p3).read()) as i32).wrapping_sub(
                    ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(34))
                    .read()) as i32),
                )) as i16);
                (__p3).write(__v4);
                __v4
            }) as i32)
                < 0i32
            {
                ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36)
                    .cast::<i16>())
                .write(
                    (((360i32).wrapping_sub(
                        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(34))
                        .read()) as i32),
                    )) as i16),
                );
            }
        }
        sin = Sin2(
            ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<i16>())
            .read()) as u16),
        );
        cos = Cos2(
            ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<i16>())
            .read()) as u16),
        );
        sin = ((crate::c::div_i32(((sin) as i32), 16i32)) as i16);
        (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
            .cast::<i16>())
        .write({
            let __v5 = ((crate::c::div_i32(((cos) as i32), 16i32)) as i16);
            (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
                .wrapping_add(6)
                .cast::<i16>())
            .write(__v5);
            __v5
        });
        (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
            .wrapping_add(2)
            .cast::<i16>())
        .write(sin);
        (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
            .wrapping_add(4)
            .cast::<i16>())
        .write(((((sin) as i32).wrapping_neg()) as i16));
    }
}
pub(crate) unsafe extern "C" fn Task_StartPlaying(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((UpdatePaletteFade()) as i32) == 0i32 {
            SetGpuReg(80u8, 9216u16);
            SetGpuReg(82u8, 2056u16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(0i16);
            ResetBallDataForNewSpin(taskId);
            ResetHits();
            HideWheelBalls();
            DrawGridBackground(0u8);
            SetBallCounterNumLeft(6u8);
            StartTaskAfterDelayOrInput(taskId, Some(Task_ContinuePlaying), 65535u16, 3u16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_AskKeepPlaying(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DisplayYesNoMenuDefaultYes();
        DrawStdWindowFrame(
            ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
            0u8,
        );
        AddTextPrinterParameterized(
            ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
            (&raw mut Roulette_Text_KeepPlaying).cast::<u8>(),
            0u8,
            1u8,
            255u8,
            None,
        );
        CopyWindowToVram(
            ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
            3u8,
        );
        DoYesNoFuncWithChoice(
            taskId,
            (&raw const sYesNoTable_KeepPlaying).cast::<u8>().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_ContinuePlaying(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ClearStdWindowAndFrame(0u8, 1u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_SelectFirstEmptySquare));
    }
}
pub(crate) unsafe extern "C" fn Task_StopPlaying(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DestroyTask(
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(165))
                .read(),
        );
        ExitRoulette(taskId);
    }
}
pub(crate) unsafe extern "C" fn UpdateGridSelectionRect(selectionId: u8) {
    unsafe {
        let mut selectionId = selectionId;
        let mut temp0: u8 = 0u8;
        let mut temp1: u8 = 0u8;
        'l1: {
            let __sw1 = ((selectionId) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 10i32
                || __sw1 == 15i32;
            if __sw1 == 0i32 {
                FillTilemapRect(
                    (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(380))
                    .cast::<u8>())
                    .cast::<u16>(),
                    0u16,
                    14u8,
                    7u8,
                    16u8,
                    13u8,
                );
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32 {
                temp0 = (((((selectionId) as i32).wrapping_mul(3i32)).wrapping_add(14i32)) as u8);
                FillTilemapRect(
                    (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(380))
                    .cast::<u8>())
                    .cast::<u16>(),
                    0u16,
                    14u8,
                    7u8,
                    16u8,
                    13u8,
                );
                SetTilemapRect(
                    (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(380))
                    .cast::<u8>())
                    .cast::<u16>(),
                    (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(14716)
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(281),
                    temp0,
                    7u8,
                    3u8,
                    13u8,
                );
                break 'l1;
            }
            if __sw1 == 5i32 || __sw1 == 10i32 || __sw1 == 15i32 {
                temp1 = ((((crate::c::div_i32(((selectionId) as i32).wrapping_sub(1i32), 5i32))
                    .wrapping_mul(3i32))
                .wrapping_add(10i32)) as u8);
                FillTilemapRect(
                    (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(380))
                    .cast::<u8>())
                    .cast::<u16>(),
                    0u16,
                    14u8,
                    7u8,
                    16u8,
                    13u8,
                );
                SetTilemapRect(
                    (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(380))
                    .cast::<u8>())
                    .cast::<u16>(),
                    (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(14716)
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(320),
                    14u8,
                    temp1,
                    16u8,
                    3u8,
                );
                break 'l1;
            }
            if !__matched {
                temp0 = ((((crate::c::rem_i32(((selectionId) as i32), 5i32)).wrapping_mul(3i32))
                    .wrapping_add(14i32)) as u8);
                temp1 = ((((crate::c::div_i32(((selectionId) as i32).wrapping_sub(1i32), 5i32))
                    .wrapping_mul(3i32))
                .wrapping_add(7i32)) as u8);
                FillTilemapRect(
                    (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(380))
                    .cast::<u8>())
                    .cast::<u16>(),
                    0u16,
                    14u8,
                    7u8,
                    16u8,
                    13u8,
                );
                SetTilemapRect(
                    (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(380))
                    .cast::<u8>())
                    .cast::<u16>(),
                    (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(14716)
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(272),
                    temp0,
                    temp1,
                    3u8,
                    3u8,
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateGridSelection(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetMultiplierSprite(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as u8),
        );
        UpdateGridSelectionRect(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_StartHandleBetGridInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(40)
            .cast::<i16>())
        .write(1i16);
        UpdateGridSelectionRect(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as u8),
        );
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(35))
            .write(2u8);
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(33))
            .write(0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleBetGridInput));
    }
}
pub(crate) unsafe extern "C" fn Task_SelectFirstEmptySquare(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i16 = 0i16;
        if (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u32>())
        .read()
            & 32u32)
            != 0
        {
            {
                i = 11i16;
                'l1: loop {
                    if !(((i) as i32) < 14i32) {
                        break 'l1;
                    }
                    'l2: {
                        if !((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<u32>())
                        .read()
                            & (((((&raw const sGridSelections).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 20))
                            .wrapping_add(8)
                            .cast::<u32>())
                            .read())
                            != 0)
                        {
                            break 'l1;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 6i16;
                'l3: loop {
                    if !(((i) as i32) <= 9i32) {
                        break 'l3;
                    }
                    'l4: {
                        if !((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<u32>())
                        .read()
                            & (((((&raw const sGridSelections).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 20))
                            .wrapping_add(8)
                            .cast::<u32>())
                            .read())
                            != 0)
                        {
                            break 'l3;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(i);
        ResetBallDataForNewSpin(taskId);
        DrawGridBackground(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as u8),
        );
        SetMultiplierSprite(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as u8),
        );
        FlashSelectionOnWheel(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as u8),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_StartHandleBetGridInput));
    }
}
pub(crate) unsafe extern "C" fn CanMoveSelectionInDir(selectionId: *mut i16, dir: u8) -> u8 {
    unsafe {
        let mut selectionId = selectionId;
        let mut dir = dir;
        let mut temp1: i8 = 0i8;
        let mut temp: i8 = 0i8;
        let mut moveOffsets = crate::ffi::Align4([0u8; 4]);
        (&raw mut moveOffsets)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i8>()
            .write((-5i8));
        (&raw mut moveOffsets)
            .cast::<u8>()
            .wrapping_add(1)
            .cast::<i8>()
            .write(5i8);
        (&raw mut moveOffsets)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<i8>()
            .write((-1i8));
        (&raw mut moveOffsets)
            .cast::<u8>()
            .wrapping_add(3)
            .cast::<i8>()
            .write(1i8);
        let mut originalSelection: i8 = (((selectionId).read()) as i8);
        'l1: {
            let __sw1 = ((dir) as i32);
            if __sw1 == 0i32 || __sw1 == 1i32 {
                temp1 = ((crate::c::rem_i32((((selectionId).read()) as i32), 5i32)) as i8);
                temp = ((((temp1) as i32).wrapping_add(15i32)) as i8);
                if ((temp1) as i32) == 0i32 {
                    temp1 = 5i8;
                }
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 3i32 {
                temp1 = (((crate::c::div_i32((((selectionId).read()) as i32), 5i32))
                    .wrapping_mul(5i32)) as i8);
                temp = ((((temp1) as i32).wrapping_add(4i32)) as i8);
                if ((temp1) as i32) == 0i32 {
                    temp1 = 1i8;
                }
                break 'l1;
            }
        }
        (selectionId).write(
            (((((selectionId).read()) as i32).wrapping_add(
                (((((&raw mut moveOffsets).cast::<i8>()).wrapping_offset(((dir) as i32) as isize))
                    .read()) as i32),
            )) as i16),
        );
        if (((selectionId).read()) as i32) < ((temp1) as i32) {
            (selectionId).write(((temp) as i16));
        }
        if (((selectionId).read()) as i32) > ((temp) as i32) {
            (selectionId).write(((temp1) as i16));
        }
        if (((selectionId).read()) as i32) != ((originalSelection) as i32) {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ProcessBetGridInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut headerOffset: u8 = 0u8;
        let mut dirPressed: u8 = 0u8;
        if (((((!(((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0))
            || ((({
                let __v1 = 1u8;
                dirPressed = __v1;
                __v1
            }) != 0)
                && ((CanMoveSelectionInDir(
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4),
                    0u8,
                )) != 0)))
            && ((!(((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 128i32)
                != 0))
                || ((({
                    let __v2 = 1u8;
                    dirPressed = __v2;
                    __v2
                }) != 0)
                    && ((CanMoveSelectionInDir(
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4),
                        1u8,
                    )) != 0))))
            && ((!(((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 32i32)
                != 0))
                || ((({
                    let __v3 = 1u8;
                    dirPressed = __v3;
                    __v3
                }) != 0)
                    && ((CanMoveSelectionInDir(
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4),
                        2u8,
                    )) != 0))))
            && ((!(((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 16i32)
                != 0))
                || ((({
                    let __v4 = 1u8;
                    dirPressed = __v4;
                    __v4
                }) != 0)
                    && ((CanMoveSelectionInDir(
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4),
                        3u8,
                    )) != 0))))
            && ((dirPressed) != 0)
        {
            let mut i: u8 = 0u8;
            DrawGridBackground(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as u8),
            );
            UpdateGridSelection(taskId);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(0i16);
            PlaySE(5u16);
            RouletteFlash_Stop(
                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(184),
                65535u16,
            );
            crate::c::bf_write(
                (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(184))
                .wrapping_add(4))
                .cast::<u8>())
                .wrapping_offset(156))
                .wrapping_add(0),
                7,
                1,
                ({
                    let __v6 = {
                        let __v5 = 0u8;
                        crate::c::bf_write(
                            (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(184))
                            .wrapping_add(4))
                            .cast::<u8>())
                            .wrapping_offset(180))
                            .wrapping_add(0),
                            7,
                            1,
                            (__v5) as i32,
                        );
                        __v5
                    };
                    crate::c::bf_write(
                        (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(184))
                        .wrapping_add(4))
                        .cast::<u8>())
                        .wrapping_offset(168))
                        .wrapping_add(0),
                        7,
                        1,
                        (__v6) as i32,
                    );
                    __v6
                }) as i32,
            );
            FlashSelectionOnWheel(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as u8),
            );
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(60))
                                .cast::<u8>())
                                .wrapping_offset((((i) as i32).wrapping_add(41i32)) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(4),
                            0,
                            10,
                            ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(60))
                                .cast::<u8>())
                                .wrapping_offset((((i) as i32).wrapping_add(41i32)) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(64)
                            .cast::<u16>())
                            .read()) as i32)
                                .wrapping_add(
                                    (((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((((((&raw mut sRoulette)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(60))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((i) as i32).wrapping_add(41i32)) as isize,
                                        ))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(8)
                                    .cast::<*mut *mut u8>())
                                    .read())
                                    .read())
                                    .cast::<i16>())
                                    .read()) as i32),
                                )) as u16) as i32,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if ((((((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as i32)
                .wrapping_sub(1i32)) as u16) as i32)
                < 4i32)
                && (!((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u32>())
                .read()
                    & (((((&raw const sGridSelections).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(4))
                            .read()) as i32) as isize
                                * 20,
                        ))
                    .wrapping_add(8)
                    .cast::<u32>())
                    .read())
                    != 0))
            {
                headerOffset = ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as i32)
                    .wrapping_sub(1i32)) as u8);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset((((headerOffset) as i32).wrapping_add(41i32)) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(4),
                    0,
                    10,
                    ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset((((headerOffset) as i32).wrapping_add(41i32)) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(64)
                    .cast::<u16>())
                    .read()) as i32)
                        .wrapping_add(
                            ((((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(60))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((headerOffset) as i32).wrapping_add(41i32)) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(8)
                            .cast::<*mut *mut u8>())
                            .read())
                            .read())
                            .wrapping_offset(4))
                            .cast::<i16>())
                            .read()) as i32),
                        )) as u16) as i32,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StartSpin(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        IncrementDailyRouletteUses();
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(40)
            .cast::<i16>())
        .write(255i16);
        if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
            .read()) as i32)
            == 1i32
        {
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(35))
                .write(1u8);
        } else {
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(35))
                .write(0u8);
        }
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(33))
            .write(0u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(32i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_SlideGridOffscreen));
    }
}
pub(crate) unsafe extern "C" fn Task_PlaceBet(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(27))
            .cast::<u8>())
        .wrapping_offset(
            ((crate::c::bf_read(
                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26),
                0,
                4,
                false,
            ) as u8) as i32) as isize,
        ))
        .write(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as u8),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            ((GetMultiplier(
                ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(27))
                .cast::<u8>())
                .wrapping_offset(
                    ((crate::c::bf_read(
                        (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(26),
                        0,
                        4,
                        false,
                    ) as u8) as i32) as isize,
                ))
                .read(),
            )) as i16),
        );
        SetMultiplierSprite(
            ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(27))
                .cast::<u8>())
            .wrapping_offset(
                ((crate::c::bf_read(
                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26),
                    0,
                    4,
                    false,
                ) as u8) as i32) as isize,
            ))
            .read(),
        );
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(13);
            let __v2 = (((((__p1).read()) as i32).wrapping_sub(
                ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                    .read()) as i32),
            )) as i16);
            (__p1).write(__v2);
            __v2
        }) as i32)
            < 0i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(13))
            .write(0i16);
        }
        SetCreditDigits(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(13))
            .read()) as u16),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_StartSpin));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleBetGridInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ProcessBetGridInput(taskId);
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 30i32 || __sw1 == 59i32;
            if __sw1 == 0i32 {
                UpdateGridSelectionRect(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as u8),
                );
                let __p2 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 30i32 {
                UpdateGridSelectionRect(0u8);
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 59i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(0i16);
                break 'l1;
            }
            if !__matched {
                let __p4 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p4).write(((__p4).read()).wrapping_add(1));
            }
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            if (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u32>())
            .read()
                & (((((&raw const sGridSelections).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .read()) as i32) as isize
                            * 20,
                    ))
                .wrapping_add(8)
                .cast::<u32>())
                .read())
                != 0
            {
                PlaySE(22u16);
            } else {
                m4aSongNumStart(95u16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_PlaceBet));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SlideGridOffscreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            __t2
        }) as i32)
            > 0i32
        {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                > 2i32
            {
                let __p3 = (&raw mut gSpriteCoordOffsetX).cast::<i16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_add(2i32)) as i16));
            }
            if (({
                let __p4 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(38)
                    .cast::<i16>();
                let __v5 = (((((__p4).read()) as i32).wrapping_add(4i32)) as i16);
                (__p4).write(__v5);
                __v5
            }) as i32)
                == 104i32
            {
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60))
                    .cast::<u8>())
                    .wrapping_offset(25))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
            }
        } else {
            ShowHideGridIcons(1u8, 255u8);
            ShowHideGridBalls(1u8, 255u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_InitBallRoll));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn GetRandomForBallTravelDistance(ballNum: u16, rand: u16) -> u8 {
    unsafe {
        let mut ballNum = ballNum;
        let mut rand = rand;
        'l1: {
            let __sw1 = ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 0i32;
            if __sw1 == 1i32 || __sw1 == 2i32 {
                if ((((((&raw mut gLocalTime).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<i8>())
                .read()) as i32)
                    > 3i32)
                    && ((((((&raw mut gLocalTime).cast::<u8>())
                        .wrapping_add(2)
                        .cast::<i8>())
                    .read()) as i32)
                        < 10i32)
                {
                    if (((ballNum) as i32) < 12i32) || ((((rand) as i32) & 1i32) != 0) {
                        return ((crate::c::div_i32(
                            (((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::bf_read(
                                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(4),
                                    0,
                                    2,
                                    false,
                                ) as u8) as i32) as isize
                                    * 32,
                            ))
                            .wrapping_add(2))
                            .read()) as i32),
                            2i32,
                        )) as u8);
                    } else {
                        return 1u8;
                    }
                } else {
                    if !((((rand) as i32) & 3i32) != 0) {
                        return ((crate::c::div_i32(
                            (((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::bf_read(
                                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(4),
                                    0,
                                    2,
                                    false,
                                ) as u8) as i32) as isize
                                    * 32,
                            ))
                            .wrapping_add(2))
                            .read()) as i32),
                            2i32,
                        )) as u8);
                    } else {
                        return (((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(4),
                                0,
                                2,
                                false,
                            ) as u8) as i32) as isize
                                * 32,
                        ))
                        .wrapping_add(2))
                        .read();
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut gLocalTime).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<i8>())
                .read()) as i32)
                    > 3i32)
                    && ((((((&raw mut gLocalTime).cast::<u8>())
                        .wrapping_add(2)
                        .cast::<i8>())
                    .read()) as i32)
                        < 11i32)
                {
                    if (((ballNum) as i32) < 6i32) || ((((rand) as i32) & 1i32) != 0) {
                        return ((crate::c::div_i32(
                            (((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::bf_read(
                                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(4),
                                    0,
                                    2,
                                    false,
                                ) as u8) as i32) as isize
                                    * 32,
                            ))
                            .wrapping_add(2))
                            .read()) as i32),
                            2i32,
                        )) as u8);
                    } else {
                        return 1u8;
                    }
                } else {
                    if ((((rand) as i32) & 1i32) != 0) && (((ballNum) as i32) > 6i32) {
                        return ((crate::c::div_i32(
                            (((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::bf_read(
                                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(4),
                                    0,
                                    2,
                                    false,
                                ) as u8) as i32) as isize
                                    * 32,
                            ))
                            .wrapping_add(2))
                            .read()) as i32),
                            4i32,
                        )) as u8);
                    } else {
                        return ((crate::c::div_i32(
                            (((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::bf_read(
                                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(4),
                                    0,
                                    2,
                                    false,
                                ) as u8) as i32) as isize
                                    * 32,
                            ))
                            .wrapping_add(2))
                            .read()) as i32),
                            2i32,
                        )) as u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 0i32 || !__matched {
                if ((((((&raw mut gLocalTime).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<i8>())
                .read()) as i32)
                    > 3i32)
                    && ((((((&raw mut gLocalTime).cast::<u8>())
                        .wrapping_add(2)
                        .cast::<i8>())
                    .read()) as i32)
                        < 10i32)
                {
                    if !((((rand) as i32) & 3i32) != 0) {
                        return 1u8;
                    } else {
                        return ((crate::c::div_i32(
                            (((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::bf_read(
                                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(4),
                                    0,
                                    2,
                                    false,
                                ) as u8) as i32) as isize
                                    * 32,
                            ))
                            .wrapping_add(2))
                            .read()) as i32),
                            2i32,
                        )) as u8);
                    }
                } else {
                    if !((((rand) as i32) & 3i32) != 0) {
                        if ((ballNum) as i32) > 12i32 {
                            return ((crate::c::div_i32(
                                (((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((crate::c::bf_read(
                                        (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(4),
                                        0,
                                        2,
                                        false,
                                    ) as u8) as i32) as isize
                                        * 32,
                                ))
                                .wrapping_add(2))
                                .read()) as i32),
                                2i32,
                            )) as u8);
                        } else {
                            return (((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::bf_read(
                                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(4),
                                    0,
                                    2,
                                    false,
                                ) as u8) as i32) as isize
                                    * 32,
                            ))
                            .wrapping_add(2))
                            .read();
                        }
                    } else {
                        if (((rand) as i32) & 32768i32) != 0 {
                            if ((ballNum) as i32) > 12i32 {
                                return (((((&raw const sRouletteTables)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((crate::c::bf_read(
                                        (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(4),
                                        0,
                                        2,
                                        false,
                                    ) as u8) as i32) as isize
                                        * 32,
                                ))
                                .wrapping_add(2))
                                .read();
                            } else {
                                return (((((&raw const sRouletteTables)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((crate::c::bf_read(
                                        (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(4),
                                        0,
                                        2,
                                        false,
                                    ) as u8) as i32) as isize
                                        * 32,
                                ))
                                .wrapping_add(1))
                                .read();
                            }
                        } else {
                            return (((((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::bf_read(
                                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(4),
                                    0,
                                    2,
                                    false,
                                ) as u8) as i32) as isize
                                    * 32,
                            ))
                            .wrapping_add(1))
                            .read()) as i32)
                                .wrapping_mul(2i32)) as u8);
                        }
                    }
                }
                break 'l1;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_InitBallRoll(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut randTravelMod: u8 = 0u8;
        let mut randTravelDist: i8 = 0i8;
        let mut startAngleId: i8 = 0i8;
        let mut travelDist: u16 = 0u16;
        let mut rand: u16 = 0u16;
        let mut randmod: u16 = 0u16;
        let mut startAngles = crate::ffi::Align4([0u8; 8]);
        (&raw mut startAngles)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(0u16);
        (&raw mut startAngles)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<u16>()
            .write(180u16);
        (&raw mut startAngles)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(90u16);
        (&raw mut startAngles)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<u16>()
            .write(270u16);
        rand = Random();
        randmod = ((crate::c::rem_i32(((rand) as i32), 100i32)) as u16);
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(124)).write(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as u8),
        );
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(125)).write({
            let __v2 = {
                let __v1 = 0u8;
                ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(127))
                    .write(__v1);
                __v1
            };
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(126))
                .write(__v2);
            __v2
        });
        randTravelMod = GetRandomForBallTravelDistance(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8))
            .read()) as u16),
            rand,
        );
        randTravelDist = (((crate::c::rem_i32(((rand) as i32), ((randTravelMod) as i32)))
            .wrapping_sub(crate::c::div_i32(((randTravelMod) as i32), 2i32)))
            as i8);
        if (((((&raw mut gLocalTime).cast::<u8>())
            .wrapping_add(2)
            .cast::<i8>())
        .read()) as i32)
            < 13i32
        {
            startAngleId = 0i8;
        } else {
            startAngleId = 1i8;
        }
        if ((randmod) as i32) < 80i32 {
            startAngleId = ((((startAngleId) as i32).wrapping_mul(2i32)) as i8);
        } else {
            startAngleId =
                ((((1i32).wrapping_sub(((startAngleId) as i32))).wrapping_mul(2i32)) as i8);
        }
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(128)
            .cast::<i16>())
        .write(
            (({
                let __v3 = (((((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(
                    ((crate::c::bf_read(
                        (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4),
                        0,
                        2,
                        false,
                    ) as u8) as i32) as isize
                        * 32,
                ))
                .wrapping_add(26)
                .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(((randTravelDist) as i32))) as u16);
                travelDist = __v3;
                __v3
            }) as i16),
        );
        travelDist = (((({
            let __v4 = ((travelDist) as i16);
            let mut f = __v4 as f32;
            if __v4 < 0 {
                f += 65536.0;
            }
            f
        }) as f32)
            / ((5.0f32) as f32)) as u16);
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(130)
            .cast::<i16>())
        .write(((((travelDist) as i32).wrapping_mul(3i32)) as i16));
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(134)
            .cast::<u16>())
        .write({
            let __v5 = travelDist;
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(132)
                .cast::<u16>())
            .write(__v5);
            __v5
        });
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(136)
            .cast::<f32>())
        .write(
            (({
                let __v6 = (((((&raw mut startAngles).cast::<u16>()).wrapping_offset(
                    ((((rand) as i32) & 1i32).wrapping_add(((startAngleId) as i32))) as isize,
                ))
                .read()) as i16);
                let mut f = __v6 as f32;
                if __v6 < 0 {
                    f += 65536.0;
                }
                f
            }) as f32),
        );
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(140)
            .cast::<f32>())
        .write(
            (({
                let __v7 = (((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(
                    ((crate::c::bf_read(
                        (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4),
                        0,
                        2,
                        false,
                    ) as u8) as i32) as isize
                        * 32,
                ))
                .wrapping_add(24)
                .cast::<u16>())
                .read()) as i16);
                let mut f = __v7 as f32;
                if __v7 < 0 {
                    f += 65536.0;
                }
                f
            }) as f32),
        );
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(144)
            .cast::<f32>())
        .write(
            ((((((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(140)
                .cast::<f32>())
            .read()) as f32)
                * ((0.5f32) as f32)) as f32)
                - ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(140)
                    .cast::<f32>())
                .read()) as f32)) as f32)
                / (({
                    let __v8 = ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(130)
                        .cast::<i16>())
                    .read();
                    let mut f = __v8 as f32;
                    if __v8 < 0 {
                        f += 65536.0;
                    }
                    f
                }) as f32)) as f32),
        );
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(148)
            .cast::<f32>())
        .write(((68.0f32) as f32));
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(156)
            .cast::<f32>())
        .write(((0.0f32) as f32));
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(152)
            .cast::<f32>())
        .write(
            ((-(((8.0f32) as f32)
                / (({
                    let __v9 = ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(130)
                        .cast::<i16>())
                    .read();
                    let mut f = __v9 as f32;
                    if __v9 < 0 {
                        f += 65536.0;
                    }
                    f
                }) as f32))) as f32),
        );
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(160)
            .cast::<f32>())
        .write(((36.0f32) as f32));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_RollBall));
    }
}
pub(crate) unsafe extern "C" fn Task_RollBall(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        crate::c::bf_write(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
            7,
            1,
            (1u8) as i32,
        );
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(56)
            .cast::<*mut u8>())
        .write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(124))
                    .read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ),
        );
        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(56)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_RollBall_Start));
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6);
        (__p1).write(((__p1).read()).wrapping_add(1));
        let __p2 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8);
        (__p2).write(((__p2).read()).wrapping_add(1));
        SetBallCounterNumLeft(
            (((6i32).wrapping_sub(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .read()) as i32),
            )) as u8),
        );
        m4aSongNumStart(92u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_RecordBallHit));
    }
}
pub(crate) unsafe extern "C" fn Task_RecordBallHit(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(125))
            .read()) as i32)
            != 0i32
        {
            if (crate::c::bf_read(
                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
                5,
                1,
                false,
            ) as u8)
                != 0
            {
                if (crate::c::bf_read(
                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
                    6,
                    1,
                    false,
                ) as u8)
                    != 0
                {
                    crate::c::bf_write(
                        (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3),
                        6,
                        1,
                        (0u8) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3),
                        5,
                        1,
                        (0u8) as i32,
                    );
                }
            } else {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 0i32
                {
                    let mut won: u8 = IsHitInBetSelection(
                        RecordHit(
                            taskId,
                            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(126))
                            .read(),
                        ),
                        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(27))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(26),
                                0,
                                4,
                                false,
                            ) as u8) as i32) as isize,
                        ))
                        .read(),
                    );
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write(((won) as i16));
                    if ((won) as i32) == 1i32 {
                        RouletteFlash_Enable(
                            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(184),
                            4096u16,
                        );
                    }
                }
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    <= 60i32
                {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0
                    {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(60i16);
                    }
                    let __p1 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p1).write(((__p1).read()).wrapping_add(1));
                } else {
                    DrawGridBackground(
                        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(27))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(26),
                                0,
                                4,
                                false,
                            ) as u8) as i32) as isize,
                        ))
                        .read(),
                    );
                    ShowHideGridIcons(
                        0u8,
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(12))
                        .read()) as u8),
                    );
                    ShowHideGridBalls(
                        0u8,
                        ((((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .read()) as i32)
                            .wrapping_sub(1i32)) as u8),
                    );
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(32i16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_SlideGridOnscreen));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SlideGridOnscreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            __t2
        }) as i32)
            > 0i32
        {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                > 2i32
            {
                let __p3 = (&raw mut gSpriteCoordOffsetX).cast::<i16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(2i32)) as i16));
            }
            if (({
                let __p4 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(38)
                    .cast::<i16>();
                let __v5 = (((((__p4).read()) as i32).wrapping_sub(4i32)) as i16);
                (__p4).write(__v5);
                __v5
            }) as i32)
                == 104i32
            {
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60))
                    .cast::<u8>())
                    .wrapping_offset(25))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_GridSquare));
            }
        } else {
            ShowHideWinSlotCursor(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(12))
                .read()) as u8),
            );
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as i32)
                == 1i32
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(121i16);
            } else {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(61i16);
            }
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_FlashBallOnWinningSquare));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_FlashBallOnWinningSquare(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            __t2
        }) as i32)
            > 1i32
        {
            'l1: {
                let __sw3 = crate::c::rem_i32(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32),
                    16i32,
                );
                if __sw3 == 8i32 {
                    ShowHideGridIcons(0u8, 255u8);
                    ShowHideGridBalls(0u8, 255u8);
                    break 'l1;
                }
                if __sw3 == 0i32 {
                    ShowHideGridIcons(
                        0u8,
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(12))
                        .read()) as u8),
                    );
                    ShowHideGridBalls(
                        0u8,
                        ((((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .read()) as i32)
                            .wrapping_sub(1i32)) as u8),
                    );
                    break 'l1;
                }
            }
        } else {
            StartTaskAfterDelayOrInput(taskId, Some(Task_PrintSpinResult), 30u16, 0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TryIncrementWins(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 0i32;
            if __sw1 == 1i32 || __sw1 == 2i32 {
                if (IsFanfareTaskInactive()) != 0 {
                    let mut wins: u32 = GetGameStat(29u8);
                    if wins
                        < (({
                            let __p2 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11);
                            let __t3 = ((__p2).read()).wrapping_add(1);
                            (__p2).write(__t3);
                            __t3
                        }) as u32)
                    {
                        SetGameStat(
                            29u8,
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11))
                            .read()) as u32),
                        );
                    }
                    StartTaskAfterDelayOrInput(taskId, Some(Task_PrintPayout), 65535u16, 3u16);
                }
                break 'l1;
            }
            if __sw1 == 0i32 || !__matched {
                if !((IsSEPlaying()) != 0) {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .write(0i16);
                    StartTaskAfterDelayOrInput(taskId, Some(Task_EndTurn), 65535u16, 3u16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PrintSpinResult(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 0i32;
            if __sw1 == 1i32 || __sw1 == 2i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32)
                    == 12i32
                {
                    PlayFanfare(389u16);
                    DrawStdWindowFrame(
                        ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                        0u8,
                    );
                    AddTextPrinterParameterized(
                        ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                        1u8,
                        (&raw mut Roulette_Text_Jackpot).cast::<u8>(),
                        0u8,
                        1u8,
                        255u8,
                        None,
                    );
                    CopyWindowToVram(
                        ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                        3u8,
                    );
                } else {
                    PlayFanfare(390u16);
                    DrawStdWindowFrame(
                        ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                        0u8,
                    );
                    AddTextPrinterParameterized(
                        ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                        1u8,
                        (&raw mut Roulette_Text_ItsAHit).cast::<u8>(),
                        0u8,
                        1u8,
                        255u8,
                        None,
                    );
                    CopyWindowToVram(
                        ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                        3u8,
                    );
                }
                break 'l1;
            }
            if __sw1 == 0i32 || !__matched {
                m4aSongNumStart(32u16);
                DrawStdWindowFrame(
                    ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                    0u8,
                );
                AddTextPrinterParameterized(
                    ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                    1u8,
                    (&raw mut Roulette_Text_NothingDoing).cast::<u8>(),
                    0u8,
                    1u8,
                    255u8,
                    None,
                );
                CopyWindowToVram(
                    ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                    3u8,
                );
                break 'l1;
            }
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_TryIncrementWins));
    }
}
pub(crate) unsafe extern "C" fn Task_GivePayout(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 3i32;
            if __sw1 == 0i32 {
                let __p2 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(13);
                (__p2).write(((__p2).read()).wrapping_add(1));
                m4aSongNumStart(21u16);
                SetCreditDigits(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(13))
                    .read()) as u16),
                );
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(13))
                .read()) as i32)
                    >= 9999i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                } else {
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p3).write(((__p3).read()).wrapping_sub(1));
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(7);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                m4aSongNumStop(21u16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(7))
                .write(0i16);
                break 'l1;
            }
            if !__matched {
                let __p5 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(7);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            == 0i32
        {
            StartTaskAfterDelayOrInput(taskId, Some(Task_EndTurn), 65535u16, 3u16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PrintPayout(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                .read()) as i32)
                .wrapping_mul(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32),
                ),
            0i32,
            2u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut Roulette_Text_YouveWonXCoins).cast::<u8>(),
        );
        DrawStdWindowFrame(
            ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
            0u8,
        );
        AddTextPrinterParameterized(
            ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            0u8,
            1u8,
            255u8,
            None,
        );
        CopyWindowToVram(
            ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
            3u8,
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                .read()) as i32)
                .wrapping_mul(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_GivePayout));
    }
}
pub(crate) unsafe extern "C" fn Task_EndTurn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        RouletteFlash_Stop(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(184),
            65535u16,
        );
        crate::c::bf_write(
            (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(184))
            .wrapping_add(4))
            .cast::<u8>())
            .wrapping_offset(156))
            .wrapping_add(0),
            7,
            1,
            ({
                let __v2 = {
                    let __v1 = 0u8;
                    crate::c::bf_write(
                        (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(184))
                        .wrapping_add(4))
                        .cast::<u8>())
                        .wrapping_offset(180))
                        .wrapping_add(0),
                        7,
                        1,
                        (__v1) as i32,
                    );
                    __v1
                };
                crate::c::bf_write(
                    (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(184))
                    .wrapping_add(4))
                    .cast::<u8>())
                    .wrapping_offset(168))
                    .wrapping_add(0),
                    7,
                    1,
                    (__v2) as i32,
                );
                __v2
            }) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60))
                .cast::<u8>())
                .wrapping_offset(
                    ((7i32).wrapping_add(
                        ((((((&raw const sGridSelections).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(12))
                                .read()) as i32) as isize
                                    * 20,
                            ))
                        .read()) as i32),
                    )) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_TryPrintEndTurnMsg));
    }
}
pub(crate) unsafe extern "C" fn Task_TryPrintEndTurnMsg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((i) as i16));
        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(27))
            .cast::<u8>())
        .wrapping_offset(
            ((crate::c::bf_read(
                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26),
                0,
                4,
                false,
            ) as u8) as i32) as isize,
        ))
        .write(0u8);
        DrawGridBackground(0u8);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60))
                .cast::<u8>())
                .wrapping_offset(48))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
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
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(60))
                            .cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_add(41i32)) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(4),
                        0,
                        10,
                        ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(60))
                            .cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_add(41i32)) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(64)
                        .cast::<u16>())
                        .read()) as i32)
                            .wrapping_add(
                                (((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(60))
                                    .cast::<u8>())
                                    .wrapping_offset((((i) as i32).wrapping_add(41i32)) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(8)
                                .cast::<*mut *mut u8>())
                                .read())
                                .read())
                                .cast::<i16>())
                                .read()) as i32),
                            )) as u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(13))
        .read()) as i32)
            >= ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                .read()) as i32)
        {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as i32)
                == 6i32
            {
                DrawStdWindowFrame(
                    ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                    0u8,
                );
                AddTextPrinterParameterized(
                    ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                    1u8,
                    (&raw mut Roulette_Text_BoardWillBeCleared).cast::<u8>(),
                    0u8,
                    1u8,
                    255u8,
                    None,
                );
                CopyWindowToVram(
                    ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                    3u8,
                );
                StartTaskAfterDelayOrInput(taskId, Some(Task_ClearBoard), 65535u16, 3u16);
            } else {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(13))
                .read()) as i32)
                    == 9999i32
                {
                    DrawStdWindowFrame(
                        ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                        0u8,
                    );
                    AddTextPrinterParameterized(
                        ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                        1u8,
                        (&raw mut Roulette_Text_CoinCaseIsFull).cast::<u8>(),
                        0u8,
                        1u8,
                        255u8,
                        None,
                    );
                    CopyWindowToVram(
                        ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                        3u8,
                    );
                    StartTaskAfterDelayOrInput(taskId, Some(Task_AskKeepPlaying), 65535u16, 3u16);
                } else {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_AskKeepPlaying));
                }
            }
        } else {
            DrawStdWindowFrame(
                ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                0u8,
            );
            AddTextPrinterParameterized(
                ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                1u8,
                (&raw mut Roulette_Text_NoCoinsLeft).cast::<u8>(),
                0u8,
                1u8,
                255u8,
                None,
            );
            CopyWindowToVram(
                ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                3u8,
            );
            StartTaskAfterDelayOrInput(taskId, Some(Task_StopPlaying), 60u16, 3u16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ClearBoard(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(0i16);
        ResetBallDataForNewSpin(taskId);
        ResetHits();
        HideWheelBalls();
        DrawGridBackground(0u8);
        SetBallCounterNumLeft(6u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 12i32) {
                    break 'l1;
                }
                'l2: {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(60))
                            .cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_add(7i32)) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (0u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(13))
        .read()) as i32)
            == 9999i32
        {
            DrawStdWindowFrame(
                ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                0u8,
            );
            AddTextPrinterParameterized(
                ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                1u8,
                (&raw mut Roulette_Text_CoinCaseIsFull).cast::<u8>(),
                0u8,
                1u8,
                255u8,
                None,
            );
            CopyWindowToVram(
                ((&raw mut sTextWindowId).cast::<u8>().cast::<u8>()).read(),
                3u8,
            );
            StartTaskAfterDelayOrInput(taskId, Some(Task_AskKeepPlaying), 65535u16, 3u16);
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_AskKeepPlaying));
        }
    }
}
pub(crate) unsafe extern "C" fn ExitRoulette(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        RouletteFlash_Stop(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(184),
            65535u16,
        );
        RouletteFlash_Reset(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(184),
        );
        SetCoins(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(13))
            .read()) as u16),
        );
        if ((GetCoins()) as i32)
            < ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                .read()) as i32)
        {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(1u16);
        } else {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(0u16);
        }
        TryPutFindThatGamerOnAir(GetCoins());
        BeginHardwarePaletteFade(255u8, 0u8, 0u8, 16u8, 0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ExitRoulette));
    }
}
pub(crate) unsafe extern "C" fn Task_ExitRoulette(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((UpdatePaletteFade()) as i32) == 0i32 {
            SetVBlankCallback(None);
            ((&raw mut gSpriteCoordOffsetX).cast::<i16>()).write({
                let __v1 = 0i16;
                ((&raw mut gSpriteCoordOffsetY).cast::<i16>()).write(__v1);
                __v1
            });
            ResetVramOamAndBgCntRegs();
            ResetAllBgsCoordinates();
            SetGpuReg(80u8, 0u16);
            SetGpuReg(82u8, 0u16);
            SetGpuReg(84u8, 0u16);
            FreeAllSpritePalettes();
            ResetPaletteFade();
            ResetSpriteData();
            FreeRoulette();
            ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(FieldCB_ContinueScriptHandleMusic));
            SetMainCallback2(Some(CB2_ReturnToField));
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForNextTask(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(168)
            .cast::<u16>())
        .read()) as i32)
            == 0i32)
            || (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(170)
                    .cast::<u16>())
                .read()) as i32))
                != 0)
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(
                ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(172)
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                .read(),
            );
            if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(170)
                .cast::<u16>())
            .read()) as i32)
                > 0i32
            {
                PlaySE(5u16);
            }
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(172)
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(None);
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(170)
                .cast::<u16>())
            .write(0u16);
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(168)
                .cast::<u16>())
            .write(0u16);
        }
        if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(168)
            .cast::<u16>())
        .read()) as i32)
            != 65535i32
        {
            let __p1 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(168)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn StartTaskAfterDelayOrInput(
    taskId: u8,
    task: Option<unsafe extern "C" fn(u8)>,
    delay: u16,
    key: u16,
) {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut delay = delay;
        let mut key = key;
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(180)
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .read(),
        );
        if core::mem::transmute::<_, usize>(task) == 0usize {
            task = ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(180)
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .read();
        }
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(172)
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(task);
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(168)
            .cast::<u16>())
        .write(delay);
        if (((delay) as i32) == 65535i32) && (((key) as i32) == 0i32) {
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(170)
                .cast::<u16>())
            .write(65535u16);
        } else {
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(170)
                .cast::<u16>())
            .write(key);
        }
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_WaitForNextTask));
    }
}
pub(crate) unsafe extern "C" fn ResetBallDataForNewSpin(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
        crate::c::bf_write(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
            7,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
            5,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
            6,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
            0,
            5,
            (0u8) as i32,
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(27))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::bf_write(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26),
            0,
            4,
            (0u8) as i32,
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn ResetHits() {
    unsafe {
        let mut i: u8 = 0u8;
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u32>())
        .write(0u32);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l3;
                }
                'l4: {
                    ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l5;
                }
                'l6: {
                    ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(22))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ShowHideGridBalls(1u8, 255u8);
    }
}
pub(crate) unsafe extern "C" fn RecordHit(taskId: u8, slotId: u8) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut slotId = slotId;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut columnFlags = crate::ffi::Align4([0u8; 16]);
        (&raw mut columnFlags)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u32>()
            .write(67650u32);
        (&raw mut columnFlags)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u32>()
            .write(135300u32);
        (&raw mut columnFlags)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<u32>()
            .write(270600u32);
        (&raw mut columnFlags)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<u32>()
            .write(541200u32);
        let mut rowFlags = crate::ffi::Align4([0u8; 12]);
        (&raw mut rowFlags)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u32>()
            .write(992u32);
        (&raw mut rowFlags)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u32>()
            .write(31744u32);
        (&raw mut rowFlags)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<u32>()
            .write(1015808u32);
        if ((slotId) as i32) >= 12i32 {
            return 0u8;
        }
        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
            .cast::<u8>())
        .wrapping_offset(
            (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as i32)
                .wrapping_sub(1i32)) as isize,
        ))
        .write(
            (((((&raw const sRouletteSlots).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((slotId) as i32) as isize * 8))
            .wrapping_add(2))
            .read(),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(12))
        .write(
            (((((((&raw const sRouletteSlots).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((slotId) as i32) as isize * 8))
            .wrapping_add(2))
            .read()) as i16),
        );
        let __p1 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u32>();
        (__p1).write(
            ((__p1).read()
                | (((((&raw const sRouletteSlots).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((slotId) as i32) as isize * 8))
                .wrapping_add(4)
                .cast::<u32>())
                .read()),
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sRouletteSlots).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((slotId) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<u32>())
                    .read()
                        & (((&raw mut columnFlags).cast::<u32>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        != 0
                    {
                        let __p2 = (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(18))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize);
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                    if ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        >= 3i32
                    {
                        let __p3 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<u32>();
                        (__p3).write(
                            ((__p3).read()
                                | (((&raw mut columnFlags).cast::<u32>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read()),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            j = 0u8;
            'l3: loop {
                if !(((j) as i32) < 3i32) {
                    break 'l3;
                }
                'l4: {
                    if ((((((&raw const sRouletteSlots).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((slotId) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<u32>())
                    .read()
                        & (((&raw mut rowFlags).cast::<u32>())
                            .wrapping_offset(((j) as i32) as isize))
                        .read())
                        != 0
                    {
                        let __p4 = (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(22))
                        .cast::<u8>())
                        .wrapping_offset(((j) as i32) as isize);
                        (__p4).write(((__p4).read()).wrapping_add(1));
                    }
                    if ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(22))
                    .cast::<u8>())
                    .wrapping_offset(((j) as i32) as isize))
                    .read()) as i32)
                        >= 4i32
                    {
                        let __p5 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<u32>();
                        (__p5).write(
                            ((__p5).read()
                                | (((&raw mut rowFlags).cast::<u32>())
                                    .wrapping_offset(((j) as i32) as isize))
                                .read()),
                        );
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
        return (((((&raw const sRouletteSlots).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((slotId) as i32) as isize * 8))
        .wrapping_add(2))
        .read();
    }
}
pub(crate) unsafe extern "C" fn IsHitInBetSelection(gridSquare: u8, betSelection: u8) -> u8 {
    unsafe {
        let mut gridSquare = gridSquare;
        let mut betSelection = betSelection;
        let mut hit: u8 = gridSquare;
        if (({
            let __t1 = (gridSquare).wrapping_sub(1);
            gridSquare = __t1;
            __t1
        }) as i32)
            < 19i32
        {
            'l1: {
                let __sw2 = ((betSelection) as i32);
                let __matched = __sw2 == 0i32
                    || __sw2 == 1i32
                    || __sw2 == 2i32
                    || __sw2 == 3i32
                    || __sw2 == 4i32
                    || __sw2 == 5i32
                    || __sw2 == 10i32
                    || __sw2 == 15i32;
                if __sw2 == 0i32 {
                    return 3u8;
                }
                if __sw2 == 1i32 || __sw2 == 2i32 || __sw2 == 3i32 || __sw2 == 4i32 {
                    if ((((hit) as i32) == ((betSelection) as i32).wrapping_add(5i32))
                        || (((hit) as i32) == ((betSelection) as i32).wrapping_add(10i32)))
                        || (((hit) as i32) == ((betSelection) as i32).wrapping_add(15i32))
                    {
                        return 1u8;
                    }
                    break 'l1;
                }
                if __sw2 == 5i32 || __sw2 == 10i32 || __sw2 == 15i32 {
                    if (((hit) as i32) >= ((betSelection) as i32).wrapping_add(1i32))
                        && (((hit) as i32) <= ((betSelection) as i32).wrapping_add(4i32))
                    {
                        return 1u8;
                    }
                    break 'l1;
                }
                if !__matched {
                    if ((hit) as i32) == ((betSelection) as i32) {
                        return 1u8;
                    }
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FlashSelectionOnWheel(selectionId: u8) {
    unsafe {
        let mut selectionId = selectionId;
        let mut flashFlags: u16 = 0u16;
        let mut numSelected: u8 = 0u8;
        let mut palOffset: u16 = 0u16;
        let mut i: u8 = 0u8;
        'l1: {
            let __sw1 = ((selectionId) as i32);
            let __matched = __sw1 == 5i32 || __sw1 == 10i32 || __sw1 == 15i32;
            if __sw1 == 5i32 || __sw1 == 10i32 || __sw1 == 15i32 {
                {
                    i = ((((selectionId) as i32).wrapping_add(1i32)) as u8);
                    'l2: loop {
                        if !(((i) as i32) < ((selectionId) as i32).wrapping_add(5i32)) {
                            break 'l2;
                        }
                        'l3: {
                            if !((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<u32>())
                            .read()
                                & (((((&raw const sGridSelections).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 20))
                                .wrapping_add(8)
                                .cast::<u32>())
                                .read())
                                != 0)
                            {
                                flashFlags = ((((flashFlags) as i32)
                                    | (((((((&raw const sGridSelections)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 20))
                                    .wrapping_add(16)
                                    .cast::<u16>())
                                    .read()) as i32))
                                    as u16);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                RouletteFlash_Enable(
                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(184),
                    {
                        let __v2 = ((((flashFlags) as i32) & (-8193i32)) as u16);
                        flashFlags = __v2;
                        __v2
                    },
                );
                break 'l1;
            }
            if !__matched {
                {
                    let mut iconFlash = crate::ffi::Align4([0u8; 24]);
                    crate::c::memcpy(
                        (&raw mut iconFlash).cast::<u8>(),
                        ((&raw const sFlashData_PokeIcons).cast::<u8>().cast_mut()).cast::<u8>(),
                        24u32,
                    );
                    if (((selectionId) as i32) >= 1i32) && (((selectionId) as i32) <= 4i32) {
                        numSelected = 3u8;
                    } else {
                        numSelected = 1u8;
                    }
                    palOffset = (((crate::c::div_i32(((selectionId) as i32), 5i32))
                        .wrapping_sub(1i32)) as u16);
                    'l4: {
                        let __sw3 = crate::c::rem_i32(((selectionId) as i32), 5i32);
                        if __sw3 == 1i32 {
                            palOffset = ((((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(60))
                                    .cast::<u8>())
                                    .wrapping_offset(7))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(5),
                                4,
                                4,
                                false,
                            ) as u16) as i32)
                                .wrapping_mul(16i32))
                                as u16);
                            break 'l4;
                        }
                        if __sw3 == 2i32 {
                            palOffset = ((((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(60))
                                    .cast::<u8>())
                                    .wrapping_offset(8))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(5),
                                4,
                                4,
                                false,
                            ) as u16) as i32)
                                .wrapping_mul(16i32))
                                as u16);
                            break 'l4;
                        }
                        if __sw3 == 3i32 {
                            palOffset = ((((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(60))
                                    .cast::<u8>())
                                    .wrapping_offset(9))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(5),
                                4,
                                4,
                                false,
                            ) as u16) as i32)
                                .wrapping_mul(16i32))
                                as u16);
                            break 'l4;
                        }
                        if __sw3 == 4i32 {
                            palOffset = ((((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(60))
                                    .cast::<u8>())
                                    .wrapping_offset(10))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(5),
                                4,
                                4,
                                false,
                            ) as u16) as i32)
                                .wrapping_mul(16i32))
                                as u16);
                            break 'l4;
                        }
                    }
                    if ((numSelected) as i32) == 1i32 {
                        if !((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<u32>())
                        .read()
                            & (((((&raw const sGridSelections).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((selectionId) as i32) as isize * 20))
                            .wrapping_add(8)
                            .cast::<u32>())
                            .read())
                            != 0)
                        {
                            let __p4 = (((&raw mut iconFlash).cast::<u8>()).wrapping_offset(
                                ((crate::c::div_i32(((selectionId) as i32), 5i32))
                                    .wrapping_sub(1i32)) as isize
                                    * 8,
                            ))
                            .wrapping_add(2)
                            .cast::<u16>();
                            (__p4).write(
                                (((((__p4).read()) as i32).wrapping_add(((palOffset) as i32)))
                                    as u16),
                            );
                            RouletteFlash_Add(
                                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(184),
                                13u8,
                                ((&raw mut iconFlash).cast::<u8>()).wrapping_offset(
                                    ((crate::c::div_i32(((selectionId) as i32), 5i32))
                                        .wrapping_sub(1i32))
                                        as isize
                                        * 8,
                                ),
                            );
                        } else {
                            break 'l1;
                        }
                    } else {
                        {
                            i = 0u8;
                            'l5: loop {
                                if !(((i) as i32) < 3i32) {
                                    break 'l5;
                                }
                                'l6: {
                                    let mut columnSlotId: u8 = ((((((i) as i32)
                                        .wrapping_mul(5i32))
                                    .wrapping_add(((selectionId) as i32)))
                                    .wrapping_add(5i32))
                                        as u8);
                                    if !((((((&raw mut sRoulette)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(8)
                                    .cast::<u32>())
                                    .read()
                                        & (((((&raw const sGridSelections)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((columnSlotId) as i32) as isize * 20))
                                        .wrapping_add(8)
                                        .cast::<u32>())
                                        .read())
                                        != 0)
                                    {
                                        let __p5 = (((&raw mut iconFlash).cast::<u8>())
                                            .wrapping_offset(
                                                ((crate::c::div_i32(((columnSlotId) as i32), 5i32))
                                                    .wrapping_sub(1i32))
                                                    as isize
                                                    * 8,
                                            ))
                                        .wrapping_add(2)
                                        .cast::<u16>();
                                        (__p5).write(
                                            (((((__p5).read()) as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                        );
                                        RouletteFlash_Add(
                                            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(184),
                                            (((((i) as i32).wrapping_add(12i32)).wrapping_add(1i32))
                                                as u8),
                                            ((&raw mut iconFlash).cast::<u8>()).wrapping_offset(
                                                ((crate::c::div_i32(((columnSlotId) as i32), 5i32))
                                                    .wrapping_sub(1i32))
                                                    as isize
                                                    * 8,
                                            ),
                                        );
                                        if ((numSelected) as i32) == 3i32 {
                                            flashFlags = (((((&raw const sGridSelections)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((columnSlotId) as i32) as isize * 20,
                                            ))
                                            .wrapping_add(16)
                                            .cast::<u16>())
                                            .read();
                                        }
                                        numSelected = (numSelected).wrapping_sub(1);
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        if ((numSelected) as i32) != 2i32 {
                            flashFlags = 0u16;
                        }
                    }
                    RouletteFlash_Enable(
                        (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(184),
                        {
                            let __v6 = ((((flashFlags) as i32)
                                | (((((((&raw const sGridSelections).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((selectionId) as i32) as isize * 20))
                                .wrapping_add(16)
                                .cast::<u16>())
                                .read()) as i32)) as u16);
                            flashFlags = __v6;
                            __v6
                        },
                    );
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawGridBackground(selectionId: u8) {
    unsafe {
        let mut selectionId = selectionId;
        let mut i: u8 = 0u8;
        (&raw mut i).write_volatile(0u8);
        let mut j: u8 = 0u8;
        (&raw mut j).write_volatile(0u8);
        let mut x: u16 = 0u16;
        (&raw mut x).write_volatile(0u16);
        let mut y: u16 = 0u16;
        (&raw mut y).write_volatile(0u16);
        let mut tilemapOffset: u8 = 0u8;
        (&raw mut tilemapOffset).write_volatile(0u8);
        let mut selectionIds = crate::ffi::Align4([0u8; 5]);
        let mut numSquares: u8 = 0u8;
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(42)
            .cast::<i16>())
        .write(1i16);
        ShowHideGridIcons(0u8, 0u8);
        SetTilemapRect(
            ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(380))
                .cast::<u8>())
            .wrapping_offset(4096))
            .cast::<u16>(),
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14716)
                .cast::<*mut u16>())
            .read(),
            14u8,
            7u8,
            16u8,
            13u8,
        );
        'l1: {
            let __sw1 = ((selectionId) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 10i32
                || __sw1 == 15i32;
            if __sw1 == 0i32 {
                return;
            }
            if __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32 {
                numSquares = 4u8;
                {
                    crate::c::volatile_write((&raw mut i), 0u8);
                    'l2: loop {
                        if !((((&raw mut i).read_volatile()) as i32) < ((numSquares) as i32)) {
                            break 'l2;
                        }
                        'l3: {
                            (((&raw mut selectionIds).cast::<u8>())
                                .wrapping_offset((((&raw mut i).read_volatile()) as i32) as isize))
                            .write(
                                ((((((&raw mut i).read_volatile()) as i32).wrapping_mul(5i32))
                                    .wrapping_add(((selectionId) as i32)))
                                    as u8),
                            );
                        }
                        crate::c::volatile_write(
                            (&raw mut i),
                            ((&raw mut i).read_volatile()).wrapping_add(1),
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 || __sw1 == 10i32 || __sw1 == 15i32 {
                numSquares = 5u8;
                {
                    crate::c::volatile_write((&raw mut i), 0u8);
                    'l4: loop {
                        if !((((&raw mut i).read_volatile()) as i32) < ((numSquares) as i32)) {
                            break 'l4;
                        }
                        'l5: {
                            (((&raw mut selectionIds).cast::<u8>())
                                .wrapping_offset((((&raw mut i).read_volatile()) as i32) as isize))
                            .write(
                                (((((&raw mut i).read_volatile()) as i32)
                                    .wrapping_add(((selectionId) as i32)))
                                    as u8),
                            );
                        }
                        crate::c::volatile_write(
                            (&raw mut i),
                            ((&raw mut i).read_volatile()).wrapping_add(1),
                        );
                    }
                }
                break 'l1;
            }
            if !__matched {
                numSquares = 1u8;
                ((&raw mut selectionIds).cast::<u8>()).write(selectionId);
            }
        }
        {
            crate::c::volatile_write((&raw mut i), 0u8);
            'l6: loop {
                if !((((&raw mut i).read_volatile()) as i32) < ((numSquares) as i32)) {
                    break 'l6;
                }
                'l7: {
                    crate::c::volatile_write(
                        (&raw mut tilemapOffset),
                        (((((&raw const sGridSelections).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                (((((&raw mut selectionIds).cast::<u8>()).wrapping_offset(
                                    (((&raw mut i).read_volatile()) as i32) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 20,
                            ))
                        .wrapping_add(6))
                        .read(),
                    );
                    crate::c::volatile_write(
                        (&raw mut x),
                        (((((((&raw const sGridSelections).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                (((((&raw mut selectionIds).cast::<u8>()).wrapping_offset(
                                    (((&raw mut i).read_volatile()) as i32) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 20,
                            ))
                        .wrapping_add(3))
                        .read()) as u16),
                    );
                    {
                        crate::c::volatile_write((&raw mut j), 0u8);
                        'l8: loop {
                            if !((((&raw mut j).read_volatile()) as i32) < 3i32) {
                                break 'l8;
                            }
                            'l9: {
                                crate::c::volatile_write(
                                    (&raw mut y),
                                    ((((((((((&raw const sGridSelections)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((((&raw mut selectionIds).cast::<u8>()).wrapping_offset(
                                            (((&raw mut i).read_volatile()) as i32) as isize,
                                        ))
                                        .read()) as i32)
                                            as isize
                                            * 20,
                                    ))
                                    .wrapping_add(4))
                                    .read()) as i32)
                                        .wrapping_add((((&raw mut j).read_volatile()) as i32)))
                                    .wrapping_mul(32i32))
                                        as u16),
                                );
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(380))
                                .cast::<u8>())
                                .wrapping_offset(4096))
                                .cast::<u16>())
                                .wrapping_offset(
                                    (((((&raw mut x).read_volatile()) as i32)
                                        .wrapping_add((((&raw mut y).read_volatile()) as i32)))
                                    .wrapping_add(0i32))
                                        as isize,
                                ))
                                .write(
                                    ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(14716)
                                    .cast::<*mut u16>())
                                    .read())
                                    .wrapping_offset(
                                        (((((((&raw mut tilemapOffset).read_volatile()) as i32)
                                            .wrapping_add(
                                                (((&raw mut j).read_volatile()) as i32),
                                            ))
                                        .wrapping_mul(3i32))
                                        .wrapping_add(208i32))
                                        .wrapping_add(0i32))
                                            as isize,
                                    ))
                                    .read(),
                                );
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(380))
                                .cast::<u8>())
                                .wrapping_offset(4096))
                                .cast::<u16>())
                                .wrapping_offset(
                                    (((((&raw mut x).read_volatile()) as i32)
                                        .wrapping_add((((&raw mut y).read_volatile()) as i32)))
                                    .wrapping_add(1i32))
                                        as isize,
                                ))
                                .write(
                                    ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(14716)
                                    .cast::<*mut u16>())
                                    .read())
                                    .wrapping_offset(
                                        (((((((&raw mut tilemapOffset).read_volatile()) as i32)
                                            .wrapping_add(
                                                (((&raw mut j).read_volatile()) as i32),
                                            ))
                                        .wrapping_mul(3i32))
                                        .wrapping_add(208i32))
                                        .wrapping_add(1i32))
                                            as isize,
                                    ))
                                    .read(),
                                );
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(380))
                                .cast::<u8>())
                                .wrapping_offset(4096))
                                .cast::<u16>())
                                .wrapping_offset(
                                    (((((&raw mut x).read_volatile()) as i32)
                                        .wrapping_add((((&raw mut y).read_volatile()) as i32)))
                                    .wrapping_add(2i32))
                                        as isize,
                                ))
                                .write(
                                    ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(14716)
                                    .cast::<*mut u16>())
                                    .read())
                                    .wrapping_offset(
                                        (((((((&raw mut tilemapOffset).read_volatile()) as i32)
                                            .wrapping_add(
                                                (((&raw mut j).read_volatile()) as i32),
                                            ))
                                        .wrapping_mul(3i32))
                                        .wrapping_add(208i32))
                                        .wrapping_add(2i32))
                                            as isize,
                                    ))
                                    .read(),
                                );
                            }
                            crate::c::volatile_write(
                                (&raw mut j),
                                ((&raw mut j).read_volatile()).wrapping_add(1),
                            );
                        }
                    }
                }
                crate::c::volatile_write(
                    (&raw mut i),
                    ((&raw mut i).read_volatile()).wrapping_add(1),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetMultiplier(selectionId: u8) -> u8 {
    unsafe {
        let mut selectionId = selectionId;
        let mut multipliers = crate::ffi::Align4([0u8; 5]);
        (&raw mut multipliers)
            .cast::<u8>()
            .wrapping_add(0)
            .write(0u8);
        (&raw mut multipliers)
            .cast::<u8>()
            .wrapping_add(1)
            .write(3u8);
        (&raw mut multipliers)
            .cast::<u8>()
            .wrapping_add(2)
            .write(4u8);
        (&raw mut multipliers)
            .cast::<u8>()
            .wrapping_add(3)
            .write(6u8);
        (&raw mut multipliers)
            .cast::<u8>()
            .wrapping_add(4)
            .write(12u8);
        if ((selectionId) as i32) > 19i32 {
            selectionId = 0u8;
        }
        'l1: {
            let __sw1 = ((crate::c::bf_read(
                ((((&raw const sGridSelections).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((selectionId) as i32) as isize * 20))
                .wrapping_add(1),
                0,
                4,
                false,
            ) as u8) as i32);
            if __sw1 == 3i32 {
                selectionId =
                    (((crate::c::div_i32(((selectionId) as i32), 5i32)).wrapping_sub(1i32)) as u8);
                if ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(22))
                .cast::<u8>())
                .wrapping_offset(((selectionId) as i32) as isize))
                .read()) as i32)
                    >= 4i32
                {
                    return 0u8;
                }
                return (((&raw mut multipliers).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(22))
                    .cast::<u8>())
                    .wrapping_offset(((selectionId) as i32) as isize))
                    .read()) as i32)
                        .wrapping_add(1i32)) as isize,
                ))
                .read();
            }
            if __sw1 == 4i32 {
                selectionId = ((((selectionId) as i32).wrapping_sub(1i32)) as u8);
                if ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(18))
                .cast::<u8>())
                .wrapping_offset(((selectionId) as i32) as isize))
                .read()) as i32)
                    >= 3i32
                {
                    return 0u8;
                }
                return (((&raw mut multipliers).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18))
                    .cast::<u8>())
                    .wrapping_offset(((selectionId) as i32) as isize))
                    .read()) as i32)
                        .wrapping_add(2i32)) as isize,
                ))
                .read();
            }
            if __sw1 == 12i32 {
                if (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u32>())
                .read()
                    & (((((&raw const sGridSelections).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((selectionId) as i32) as isize * 20))
                    .wrapping_add(8)
                    .cast::<u32>())
                    .read())
                    != 0
                {
                    return 0u8;
                }
                return (((&raw mut multipliers).cast::<u8>()).wrapping_offset(
                    (((crate::c::div_u32(5u32, 1u32)).wrapping_sub(1u32)) as i32) as isize,
                ))
                .read();
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn UpdateWheelPosition() {
    unsafe {
        let mut bg2x: i32 = 0i32;
        let mut bg2y: i32 = 0i32;
        SetGpuReg(
            32u8,
            (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
                .cast::<i16>())
            .read()) as u16),
        );
        SetGpuReg(
            34u8,
            (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as u16),
        );
        SetGpuReg(
            36u8,
            (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
                .wrapping_add(4)
                .cast::<i16>())
            .read()) as u16),
        );
        SetGpuReg(
            38u8,
            (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
                .wrapping_add(6)
                .cast::<i16>())
            .read()) as u16),
        );
        bg2x = ((29696i32).wrapping_sub(
            (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
                .cast::<i16>())
            .read()) as i32)
                .wrapping_mul(
                    ((((&raw mut gSpriteCoordOffsetX).cast::<i16>()).read()) as i32)
                        .wrapping_add(116i32),
                ),
        ))
        .wrapping_sub(
            (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_mul(
                    ((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as i32)
                        .wrapping_add(80i32),
                ),
        );
        bg2y = ((21504i32).wrapping_sub(
            (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
                .wrapping_add(4)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_mul(
                    ((((&raw mut gSpriteCoordOffsetX).cast::<i16>()).read()) as i32)
                        .wrapping_add(116i32),
                ),
        ))
        .wrapping_sub(
            (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
                .wrapping_add(6)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_mul(
                    ((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as i32)
                        .wrapping_add(80i32),
                ),
        );
        SetGpuReg(40u8, ((bg2x) as u16));
        SetGpuReg(42u8, (((bg2x & 268369920i32) >> 16) as u16));
        SetGpuReg(44u8, ((bg2y) as u16));
        SetGpuReg(46u8, (((bg2y & 268369920i32) >> 16) as u16));
    }
}
pub(crate) unsafe extern "C" fn Task_ShowMinBetYesNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DisplayYesNoMenuDefaultYes();
        DoYesNoFuncWithChoice(
            taskId,
            (&raw const sYesNoTable_AcceptMinBet)
                .cast::<u8>()
                .cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_FadeToRouletteGame(taskId: u8) {
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
            SetVBlankCallback(None);
            SetMainCallback2(Some(CB2_LoadRoulette));
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_AcceptMinBet(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ClearStdWindowAndFrame(0u8, 1u8);
        HideCoinsWindow();
        FreeAllWindowBuffers();
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
            0,
            6,
            ((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                0,
                6,
                false,
            ) as u16) as u8) as i32,
        );
        UpdatePaletteFade();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_FadeToRouletteGame));
    }
}
pub(crate) unsafe extern "C" fn Task_DeclineMinBet(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ClearStdWindowAndFrame(0u8, 0u8);
        HideCoinsWindow();
        UnlockPlayerFieldControls();
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_NotEnoughForMinBet(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let __p1 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(1u16);
            HideCoinsWindow();
            ClearStdWindowAndFrame(0u8, 1u8);
            UnlockPlayerFieldControls();
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PrintMinBet(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            let mut minBet: u32 = ((((((&raw const sTableMinBets).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) & 1i32)
                    .wrapping_add(
                        (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) >> 7)
                            .wrapping_mul(2i32),
                    )) as isize,
            ))
            .read()) as u32);
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                ((minBet) as i32),
                2i32,
                1u8,
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut Roulette_Text_PlayMinimumWagerIsX).cast::<u8>(),
            );
            DrawStdWindowFrame(0u8, 0u8);
            AddTextPrinterParameterized(
                0u8,
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                0u8,
                1u8,
                255u8,
                None,
            );
            CopyWindowToVram(0u8, 3u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ShowMinBetYesNo));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PrintRouletteEntryMsg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut minBet: i32 = 0i32;
        PrintCoinsString(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(13))
            .read()) as u32),
        );
        minBet = ((((((&raw const sTableMinBets).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) & 1i32)
                    .wrapping_add(
                        (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) >> 7)
                            .wrapping_mul(2i32),
                    )) as isize,
            ))
        .read()) as i32);
        ConvertIntToDecimalStringN((&raw mut gStringVar1).cast::<u8>(), minBet, 2i32, 1u8);
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(13))
        .read()) as i32)
            >= minBet
        {
            if ((((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) & 128i32) != 0)
                && ((((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) & 1i32) != 0)
            {
                DrawStdWindowFrame(0u8, 0u8);
                AddTextPrinterParameterized(
                    0u8,
                    1u8,
                    (&raw mut Roulette_Text_SpecialRateTable).cast::<u8>(),
                    0u8,
                    1u8,
                    255u8,
                    None,
                );
                CopyWindowToVram(0u8, 3u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_PrintMinBet));
            } else {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut Roulette_Text_PlayMinimumWagerIsX).cast::<u8>(),
                );
                DrawStdWindowFrame(0u8, 0u8);
                AddTextPrinterParameterized(
                    0u8,
                    1u8,
                    (&raw mut gStringVar4).cast::<u8>(),
                    0u8,
                    1u8,
                    255u8,
                    None,
                );
                CopyWindowToVram(0u8, 3u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ShowMinBetYesNo));
            }
        } else {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut Roulette_Text_NotEnoughCoins).cast::<u8>(),
            );
            DrawStdWindowFrame(0u8, 0u8);
            AddTextPrinterParameterized(
                0u8,
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                0u8,
                1u8,
                255u8,
                None,
            );
            CopyWindowToVram(0u8, 3u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NotEnoughForMinBet));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(13))
            .write(0i16);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayRoulette() {
    unsafe {
        let mut taskId: u8 = 0u8;
        LockPlayerFieldControls();
        ShowCoinsWindow(((GetCoins()) as u32), 1u8, 1u8);
        taskId = CreateTask(Some(Task_PrintRouletteEntryMsg), 0u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(13))
        .write(((GetCoins()) as i16));
    }
}
pub(crate) unsafe extern "C" fn LoadOrFreeMiscSpritePalettesAndSheets(free: u8) {
    unsafe {
        let mut free = free;
        if !((free) != 0) {
            FreeAllSpritePalettes();
            LoadSpritePalettes(((&raw const sSpritePalettes).cast::<u8>().cast_mut()).cast::<u8>());
            LoadCompressedSpriteSheet((&raw const sSpriteSheet_Ball).cast::<u8>().cast_mut());
            LoadCompressedSpriteSheet(
                (&raw const sSpriteSheet_ShroomishTaillow)
                    .cast::<u8>()
                    .cast_mut(),
            );
            LoadCompressedSpriteSheet((&raw const sSpriteSheet_Shadow).cast::<u8>().cast_mut());
        } else {
            FreeSpriteTilesByTag(14u16);
            FreeSpriteTilesByTag(13u16);
            FreeSpriteTilesByTag(12u16);
            FreeAllSpritePalettes();
        }
    }
}
pub(crate) unsafe extern "C" fn CreateWheelIconSprite(
    template: *mut u8,
    r1: u8,
    angle: *mut u16,
) -> u8 {
    unsafe {
        let mut template = template;
        let mut r1 = r1;
        let mut angle = angle;
        let mut temp: u16 = 0u16;
        let mut spriteId: u8 = CreateSprite(
            template,
            116i16,
            80i16,
            ((crate::c::bf_read(
                (((template).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_add(0),
                0,
                8,
                false,
            ) as u32) as u8),
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write((((angle).read()) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((r1) as i16));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            1,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(44),
            6,
            1,
            (1u8) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(44),
            7,
            1,
            (1u8) as i32,
        );
        temp = (angle).read();
        (angle).write(
            (((((angle).read()) as i32).wrapping_add(crate::c::div_i32(360i32, 12i32))) as u16),
        );
        if (((angle).read()) as i32) >= 360i32 {
            (angle).write(
                ((((temp) as i32)
                    .wrapping_sub((360i32).wrapping_sub(crate::c::div_i32(360i32, 12i32))))
                    as u16),
            );
        }
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn CreateGridSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        let mut s = crate::ffi::Align4([0u8; 8]);
        LZ77UnCompWram(
            (((&raw const sSpriteSheet_Headers).cast::<u8>().cast_mut()).cast::<*mut u32>()).read(),
            (&raw mut gDecompressionBuffer).cast::<u8>(),
        );
        (((&raw mut s).cast::<u8>()).cast::<*mut u8>())
            .write((&raw mut gDecompressionBuffer).cast::<u8>());
        (((&raw mut s).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(
            (((&raw const sSpriteSheet_Headers).cast::<u8>().cast_mut())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
        (((&raw mut s).cast::<u8>()).wrapping_add(6).cast::<u16>()).write(
            (((&raw const sSpriteSheet_Headers).cast::<u8>().cast_mut())
                .wrapping_add(6)
                .cast::<u16>())
            .read(),
        );
        LoadSpriteSheet((&raw mut s).cast::<u8>());
        LZ77UnCompWram(
            (((&raw const sSpriteSheet_GridIcons).cast::<u8>().cast_mut()).cast::<*mut u32>())
                .read(),
            (&raw mut gDecompressionBuffer).cast::<u8>(),
        );
        (((&raw mut s).cast::<u8>()).cast::<*mut u8>())
            .write((&raw mut gDecompressionBuffer).cast::<u8>());
        (((&raw mut s).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(
            (((&raw const sSpriteSheet_GridIcons).cast::<u8>().cast_mut())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
        (((&raw mut s).cast::<u8>()).wrapping_add(6).cast::<u16>()).write(
            (((&raw const sSpriteSheet_GridIcons).cast::<u8>().cast_mut())
                .wrapping_add(6)
                .cast::<u16>())
            .read(),
        );
        LoadSpriteSheet((&raw mut s).cast::<u8>());
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    let mut y: u8 = ((((i) as i32).wrapping_mul(24i32)) as u8);
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                spriteId = {
                                    let __v1 = CreateSprite(
                                        (((&raw const sSpriteTemplates_GridIcons)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 24),
                                        (((((j) as i32).wrapping_mul(24i32)).wrapping_add(148i32))
                                            as i16),
                                        ((((y) as i32).wrapping_add(92i32)) as i16),
                                        30u8,
                                    );
                                    ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(60))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((((i) as i32).wrapping_mul(4i32)).wrapping_add(29i32))
                                            .wrapping_add(((j) as i32)))
                                            as isize,
                                    ))
                                    .write(__v1);
                                    __v1
                                };
                                crate::c::bf_write(
                                    (((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                                    .wrapping_add(44),
                                    6,
                                    1,
                                    (1u8) as i32,
                                );
                                y = ((((y) as i32).wrapping_add(24i32)) as u8);
                                if ((y) as i32) >= 72i32 {
                                    y = 0u8;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as u32) < crate::c::div_u32(96u32, 24u32)) {
                    break 'l5;
                }
                'l6: {
                    spriteId = {
                        let __v2 = CreateSprite(
                            (((&raw const sSpriteTemplates_PokeHeaders)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 24),
                            (((((i) as i32).wrapping_mul(24i32)).wrapping_add(148i32)) as i16),
                            70i16,
                            30u8,
                        );
                        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset((((i) as i32).wrapping_add(41i32)) as isize))
                        .write(__v2);
                        __v2
                    };
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(44),
                        6,
                        1,
                        (1u8) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l7: loop {
                if !(((i) as u32) < crate::c::div_u32(72u32, 24u32)) {
                    break 'l7;
                }
                'l8: {
                    spriteId = {
                        let __v3 = CreateSprite(
                            (((&raw const sSpriteTemplates_ColorHeaders)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 24),
                            126i16,
                            (((((i) as i32).wrapping_mul(24i32)).wrapping_add(92i32)) as i16),
                            30u8,
                        );
                        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset((((i) as i32).wrapping_add(45i32)) as isize))
                        .write(__v3);
                        __v3
                    };
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(44),
                        6,
                        1,
                        (1u8) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyGridSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 12i32) {
                    break 'l1;
                }
                'l2: {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(60))
                            .cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_add(29i32)) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowHideGridIcons(hideAll: u8, hideSquare: u8) {
    unsafe {
        let mut hideAll = hideAll;
        let mut hideSquare = hideSquare;
        let mut i: u8 = 0u8;
        'l1: {
            let __sw1 = ((hideAll) as i32);
            if __sw1 == 1i32 {
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < 19i32) {
                            break 'l2;
                        }
                        'l3: {
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(60))
                                    .cast::<u8>())
                                    .wrapping_offset((((i) as i32).wrapping_add(29i32)) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(62),
                                2,
                                1,
                                (1u16) as i32,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 0i32 {
                {
                    i = 0u8;
                    'l4: loop {
                        if !(((i) as i32) < 12i32) {
                            break 'l4;
                        }
                        'l5: {
                            if !((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<u32>())
                            .read()
                                & (((((&raw const sRouletteSlots).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 8))
                                .wrapping_add(4)
                                .cast::<u32>())
                                .read())
                                != 0)
                            {
                                crate::c::bf_write(
                                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((((((&raw mut sRoulette)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(60))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((i) as i32).wrapping_add(29i32)) as isize,
                                        ))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(62),
                                    2,
                                    1,
                                    (0u16) as i32,
                                );
                            } else {
                                if (((((((&raw const sRouletteSlots).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 8))
                                .wrapping_add(2))
                                .read()) as i32)
                                    != ((hideSquare) as i32)
                                {
                                    crate::c::bf_write(
                                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            ((((((((&raw mut sRoulette)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(60))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                (((i) as i32).wrapping_add(29i32)) as isize,
                                            ))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                        .wrapping_add(62),
                                        2,
                                        1,
                                        (1u16) as i32,
                                    );
                                } else {
                                    crate::c::bf_write(
                                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            ((((((((&raw mut sRoulette)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(60))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                (((i) as i32).wrapping_add(29i32)) as isize,
                                            ))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                        .wrapping_add(62),
                                        2,
                                        1,
                                        (0u16) as i32,
                                    );
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    'l6: loop {
                        if !(((i) as i32) < 19i32) {
                            break 'l6;
                        }
                        'l7: {
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(60))
                                    .cast::<u8>())
                                    .wrapping_offset((((i) as i32).wrapping_add(29i32)) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(62),
                                2,
                                1,
                                (0u16) as i32,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateGridBallSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60))
                    .cast::<u8>())
                    .wrapping_offset((((i) as i32).wrapping_add(49i32)) as isize))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_Ball).cast::<u8>().cast_mut(),
                        116i16,
                        20i16,
                        10u8,
                    ));
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(60))
                            .cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_add(49i32)) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset((((i) as i32).wrapping_add(49i32)) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(1i16);
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset((((i) as i32).wrapping_add(49i32)) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_GridSquare));
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(60))
                            .cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_add(49i32)) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(5),
                        2,
                        2,
                        (1u16) as i32,
                    );
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(60))
                            .cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_add(49i32)) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ),
                        8u8,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowHideGridBalls(hideAll: u8, hideBallId: u8) {
    unsafe {
        let mut hideAll = hideAll;
        let mut hideBallId = hideBallId;
        let mut i: u8 = 0u8;
        if (hideAll) != 0 {
            {
                'l1: loop {
                    if !(((i) as i32) < 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(60))
                                .cast::<u8>())
                                .wrapping_offset((((i) as i32).wrapping_add(49i32)) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            (1u16) as i32,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                'l3: loop {
                    if !(((i) as i32) < 6i32) {
                        break 'l3;
                    }
                    'l4: {
                        if (!((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                            != 0))
                            || (((i) as i32) == ((hideBallId) as i32))
                        {
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(60))
                                    .cast::<u8>())
                                    .wrapping_offset((((i) as i32).wrapping_add(49i32)) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(62),
                                2,
                                1,
                                (1u16) as i32,
                            );
                        } else {
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(60))
                                    .cast::<u8>())
                                    .wrapping_offset((((i) as i32).wrapping_add(49i32)) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(62),
                                2,
                                1,
                                (0u16) as i32,
                            );
                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(60))
                                .cast::<u8>())
                                .wrapping_offset((((i) as i32).wrapping_add(49i32)) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(32)
                            .cast::<i16>())
                            .write(
                                (((((((((((&raw const sGridSelections).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(12))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 20,
                                ))
                                .wrapping_add(3))
                                .read()) as i32)
                                    .wrapping_add(1i32))
                                .wrapping_mul(8i32))
                                .wrapping_add(4i32)) as i16),
                            );
                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(60))
                                .cast::<u8>())
                                .wrapping_offset((((i) as i32).wrapping_add(49i32)) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(34)
                            .cast::<i16>())
                            .write(
                                (((((((((((&raw const sGridSelections).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(12))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 20,
                                ))
                                .wrapping_add(4))
                                .read()) as i32)
                                    .wrapping_add(1i32))
                                .wrapping_mul(8i32))
                                .wrapping_add(3i32)) as i16),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowHideWinSlotCursor(selectionId: u8) {
    unsafe {
        let mut selectionId = selectionId;
        if ((selectionId) as i32) == 0i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60))
                    .cast::<u8>())
                    .wrapping_offset(48))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60))
                    .cast::<u8>())
                    .wrapping_offset(48))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60))
                .cast::<u8>())
                .wrapping_offset(48))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>())
            .write(
                ((((((((((&raw const sGridSelections).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((selectionId) as i32) as isize * 20))
                .wrapping_add(3))
                .read()) as i32)
                    .wrapping_add(2i32))
                .wrapping_mul(8i32)) as i16),
            );
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60))
                .cast::<u8>())
                .wrapping_offset(48))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .write(
                ((((((((((&raw const sGridSelections).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((selectionId) as i32) as isize * 20))
                .wrapping_add(4))
                .read()) as i32)
                    .wrapping_add(2i32))
                .wrapping_mul(8i32)) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CreateWheelIconSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut angle: u16 = 0u16;
        let mut s = crate::ffi::Align4([0u8; 8]);
        LZ77UnCompWram(
            (((&raw const sSpriteSheet_WheelIcons).cast::<u8>().cast_mut()).cast::<*mut u32>())
                .read(),
            (&raw mut gDecompressionBuffer).cast::<u8>(),
        );
        (((&raw mut s).cast::<u8>()).cast::<*mut u8>())
            .write((&raw mut gDecompressionBuffer).cast::<u8>());
        (((&raw mut s).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(
            (((&raw const sSpriteSheet_WheelIcons).cast::<u8>().cast_mut())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
        (((&raw mut s).cast::<u8>()).wrapping_add(6).cast::<u16>()).write(
            (((&raw const sSpriteSheet_WheelIcons).cast::<u8>().cast_mut())
                .wrapping_add(6)
                .cast::<u16>())
            .read(),
        );
        LoadSpriteSheet((&raw mut s).cast::<u8>());
        angle = 15u16;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                let mut spriteId: u8 = 0u8;
                                spriteId = {
                                    let __v1 = CreateWheelIconSprite(
                                        (((&raw const sSpriteTemplates_WheelIcons)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((((i) as i32).wrapping_mul(4i32))
                                                .wrapping_add(((j) as i32)))
                                                as isize
                                                * 24,
                                        ),
                                        40u8,
                                        &raw mut angle,
                                    );
                                    ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(60))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((((i) as i32).wrapping_mul(4i32)).wrapping_add(7i32))
                                            .wrapping_add(((j) as i32)))
                                            as isize,
                                    ))
                                    .write(__v1);
                                    __v1
                                };
                                crate::c::bf_write(
                                    (((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                                    .wrapping_add(44),
                                    6,
                                    1,
                                    (1u8) as i32,
                                );
                                crate::c::bf_write(
                                    (((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                                    .wrapping_add(44),
                                    7,
                                    1,
                                    (1u8) as i32,
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WheelIcon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cos: i16 = 0i16;
        let mut sin: i16 = 0i16;
        let mut matrixNum: u32 = 0u32;
        let mut angle: i16 = ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36)
            .cast::<i16>())
        .read()) as i32)
            .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
            as i16);
        if ((angle) as i32) >= 360i32 {
            angle = ((((angle) as i32).wrapping_sub(360i32)) as i16);
        }
        sin = Sin2(((angle) as u16));
        cos = Cos2(((angle) as u16));
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((sin) as i32).wrapping_mul(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            ) >> 12) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            (((((cos) as i32).wrapping_neg()).wrapping_mul(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            ) >> 12) as i16),
        );
        matrixNum = (crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32);
        sin = ((crate::c::div_i32(((sin) as i32), 16i32)) as i16);
        ((((&raw mut gOamMatrices).cast::<u8>())
            .wrapping_offset(((matrixNum) as i32) as isize * 8))
        .wrapping_add(6)
        .cast::<i16>())
        .write({
            let __v1 = ((crate::c::div_i32(((cos) as i32), 16i32)) as i16);
            cos = __v1;
            __v1
        });
        ((((&raw mut gOamMatrices).cast::<u8>())
            .wrapping_offset(((matrixNum) as i32) as isize * 8))
        .cast::<i16>())
        .write(cos);
        ((((&raw mut gOamMatrices).cast::<u8>())
            .wrapping_offset(((matrixNum) as i32) as isize * 8))
        .wrapping_add(2)
        .cast::<i16>())
        .write(sin);
        ((((&raw mut gOamMatrices).cast::<u8>())
            .wrapping_offset(((matrixNum) as i32) as isize * 8))
        .wrapping_add(4)
        .cast::<i16>())
        .write(((((sin) as i32).wrapping_neg()) as i16));
    }
}
pub(crate) unsafe extern "C" fn CreateInterfaceSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < (crate::c::div_u32(48u32, 8u32)).wrapping_sub(1u32)) {
                    break 'l1;
                }
                'l2: {
                    let mut s = crate::ffi::Align4([0u8; 8]);
                    LZ77UnCompWram(
                        (((((&raw const sSpriteSheets_Interface).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .cast::<*mut u32>())
                        .read(),
                        (&raw mut gDecompressionBuffer).cast::<u8>(),
                    );
                    (((&raw mut s).cast::<u8>()).cast::<*mut u8>())
                        .write((&raw mut gDecompressionBuffer).cast::<u8>());
                    (((&raw mut s).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(
                        (((((&raw const sSpriteSheets_Interface).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .read(),
                    );
                    (((&raw mut s).cast::<u8>()).wrapping_add(6).cast::<u16>()).write(
                        (((((&raw const sSpriteSheets_Interface).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read(),
                    );
                    LoadSpriteSheet((&raw mut s).cast::<u8>());
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(60))
            .cast::<u8>())
        .wrapping_offset(20))
        .write(CreateSprite(
            (&raw const sSpriteTemplate_Credit).cast::<u8>().cast_mut(),
            208i16,
            16i16,
            4u8,
        ));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60))
                .cast::<u8>())
                .wrapping_offset(20))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(44),
            6,
            1,
            (1u8) as i32,
        );
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l3;
                }
                'l4: {
                    ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60))
                    .cast::<u8>())
                    .wrapping_offset((((i) as i32).wrapping_add(21i32)) as isize))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_CreditDigit)
                            .cast::<u8>()
                            .cast_mut(),
                        (((((i) as i32).wrapping_mul(8i32)).wrapping_add(196i32)) as i16),
                        24i16,
                        0u8,
                    ));
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(60))
                            .cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_add(21i32)) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(60))
                            .cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_add(21i32)) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(44),
                        6,
                        1,
                        (1u8) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(60))
            .cast::<u8>())
        .wrapping_offset(25))
        .write(CreateSprite(
            (&raw const sSpriteTemplate_Multiplier)
                .cast::<u8>()
                .cast_mut(),
            120i16,
            68i16,
            4u8,
        ));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60))
                .cast::<u8>())
                .wrapping_offset(25))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(44),
            6,
            1,
            (1u8) as i32,
        );
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32) < crate::c::div_i32(6i32, 2i32)) {
                    break 'l5;
                }
                'l6: {
                    ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60))
                    .cast::<u8>())
                    .wrapping_offset((((i) as i32).wrapping_add(26i32)) as isize))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_BallCounter)
                            .cast::<u8>()
                            .cast_mut(),
                        (((((i) as i32).wrapping_mul(16i32)).wrapping_add(192i32)) as i16),
                        36i16,
                        4u8,
                    ));
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(60))
                            .cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_add(26i32)) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(60))
                            .cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_add(26i32)) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(44),
                        6,
                        1,
                        (1u8) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(60))
            .cast::<u8>())
        .wrapping_offset(48))
        .write(CreateSprite(
            (&raw const sSpriteTemplate_Cursor).cast::<u8>().cast_mut(),
            152i16,
            96i16,
            9u8,
        ));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60))
                .cast::<u8>())
                .wrapping_offset(48))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60))
                .cast::<u8>())
                .wrapping_offset(48))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(44),
            6,
            1,
            (1u8) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60))
                .cast::<u8>())
                .wrapping_offset(48))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn SetCreditDigits(num: u16) {
    unsafe {
        let mut num = num;
        let mut i: u8 = 0u8;
        let mut d: u16 = 1000u16;
        let mut printZero: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut digit: u8 = ((crate::c::div_i32(((num) as i32), ((d) as i32))) as u8);
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(60))
                            .cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_add(21i32)) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    if ((((digit) as i32) > 0i32) || ((printZero) != 0)) || (((i) as i32) == 3i32) {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(60))
                                .cast::<u8>())
                                .wrapping_offset((((i) as i32).wrapping_add(21i32)) as isize))
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
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(60))
                                .cast::<u8>())
                                .wrapping_offset((((i) as i32).wrapping_add(21i32)) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(4),
                            0,
                            10,
                            ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(60))
                                .cast::<u8>())
                                .wrapping_offset((((i) as i32).wrapping_add(21i32)) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(64)
                            .cast::<u16>())
                            .read()) as i32)
                                .wrapping_add(
                                    ((((((((((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(
                                            ((((((((&raw mut sRoulette)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(60))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                (((i) as i32).wrapping_add(21i32)) as isize,
                                            ))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                    .wrapping_add(8)
                                    .cast::<*mut *mut u8>())
                                    .read())
                                    .read())
                                    .wrapping_offset(((digit) as i32) as isize * 4))
                                    .cast::<i16>())
                                    .read()) as i32),
                                )) as u16) as i32,
                        );
                        printZero = 1u8;
                    }
                    num = ((crate::c::rem_i32(((num) as i32), ((d) as i32))) as u16);
                    d = ((crate::c::div_i32(((d) as i32), 10i32)) as u16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetMultiplierAnimId(selectionId: u8) -> u8 {
    unsafe {
        let mut selectionId = selectionId;
        let mut animIds = crate::ffi::Align4([0u8; 5]);
        (&raw mut animIds).cast::<u8>().wrapping_add(0).write(0u8);
        (&raw mut animIds).cast::<u8>().wrapping_add(1).write(1u8);
        (&raw mut animIds).cast::<u8>().wrapping_add(2).write(2u8);
        (&raw mut animIds).cast::<u8>().wrapping_add(3).write(3u8);
        (&raw mut animIds).cast::<u8>().wrapping_add(4).write(4u8);
        if ((selectionId) as i32) > 19i32 {
            selectionId = 0u8;
        }
        'l1: {
            let __sw1 = ((crate::c::bf_read(
                ((((&raw const sGridSelections).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((selectionId) as i32) as isize * 20))
                .wrapping_add(1),
                0,
                4,
                false,
            ) as u8) as i32);
            if __sw1 == 3i32 {
                selectionId =
                    (((crate::c::div_i32(((selectionId) as i32), 5i32)).wrapping_sub(1i32)) as u8);
                if ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(22))
                .cast::<u8>())
                .wrapping_offset(((selectionId) as i32) as isize))
                .read()) as i32)
                    > 3i32
                {
                    return 0u8;
                }
                return (((&raw mut animIds).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(22))
                    .cast::<u8>())
                    .wrapping_offset(((selectionId) as i32) as isize))
                    .read()) as i32)
                        .wrapping_add(1i32)) as isize,
                ))
                .read();
            }
            if __sw1 == 4i32 {
                selectionId = ((((selectionId) as i32).wrapping_sub(1i32)) as u8);
                if ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(18))
                .cast::<u8>())
                .wrapping_offset(((selectionId) as i32) as isize))
                .read()) as i32)
                    > 2i32
                {
                    return 0u8;
                }
                return (((&raw mut animIds).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18))
                    .cast::<u8>())
                    .wrapping_offset(((selectionId) as i32) as isize))
                    .read()) as i32)
                        .wrapping_add(2i32)) as isize,
                ))
                .read();
            }
            if __sw1 == 12i32 {
                if (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u32>())
                .read()
                    & (((((&raw const sGridSelections).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((selectionId) as i32) as isize * 20))
                    .wrapping_add(8)
                    .cast::<u32>())
                    .read())
                    != 0
                {
                    return 0u8;
                }
                return (((&raw mut animIds).cast::<u8>()).wrapping_offset(4)).read();
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SetMultiplierSprite(selectionId: u8) {
    unsafe {
        let mut selectionId = selectionId;
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(60))
                .cast::<u8>())
            .wrapping_offset(25))
            .read()) as i32) as isize
                * 68,
        );
        ((sprite).wrapping_add(43)).write(GetMultiplierAnimId(selectionId));
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((((sprite).wrapping_add(64).cast::<u16>()).read()) as i32).wrapping_add(
                ((((((((sprite).wrapping_add(8).cast::<*mut *mut u8>()).read()).read())
                    .wrapping_offset(((((sprite).wrapping_add(43)).read()) as i32) as isize * 4))
                .cast::<i16>())
                .read()) as i32),
            )) as u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn SetBallCounterNumLeft(numBalls: u8) {
    unsafe {
        let mut numBalls = numBalls;
        let mut i: u8 = 0u8;
        let mut t: u8 = 0u8;
        if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
            .read()) as i32)
            == 1i32
        {
            t = 2u8;
        }
        'l1: {
            let __sw1 = ((numBalls) as i32);
            let __matched = __sw1 == 6i32
                || __sw1 == 5i32
                || __sw1 == 4i32
                || __sw1 == 3i32
                || __sw1 == 2i32
                || __sw1 == 1i32
                || __sw1 == 0i32;
            if __sw1 == 6i32 {
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < crate::c::div_i32(6i32, 2i32)) {
                            break 'l2;
                        }
                        'l3: {
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(60))
                                    .cast::<u8>())
                                    .wrapping_offset((((i) as i32).wrapping_add(26i32)) as isize))
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
                                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(60))
                                    .cast::<u8>())
                                    .wrapping_offset((((i) as i32).wrapping_add(26i32)) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(4),
                                0,
                                10,
                                ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(60))
                                    .cast::<u8>())
                                    .wrapping_offset((((i) as i32).wrapping_add(26i32)) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(64)
                                .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_add(
                                        (((((((((&raw mut gSprites).cast::<u8>())
                                            .wrapping_offset(
                                                ((((((((&raw mut sRoulette)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(60))
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    (((i) as i32).wrapping_add(26i32)) as isize,
                                                ))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                        .wrapping_add(8)
                                        .cast::<*mut *mut u8>())
                                        .read())
                                        .read())
                                        .cast::<i16>())
                                        .read()) as i32),
                                    )) as u16) as i32,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(28))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(4),
                    0,
                    10,
                    ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(28))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(64)
                    .cast::<u16>())
                    .read()) as i32)
                        .wrapping_add(
                            (((((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(60))
                                .cast::<u8>())
                                .wrapping_offset(28))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(8)
                            .cast::<*mut *mut u8>())
                            .read())
                            .read())
                            .wrapping_offset(((t) as i32) as isize * 4))
                            .wrapping_offset(4))
                            .cast::<i16>())
                            .read()) as i32),
                        )) as u16) as i32,
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(28))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(4),
                    0,
                    10,
                    ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(28))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(64)
                    .cast::<u16>())
                    .read()) as i32)
                        .wrapping_add(
                            (((((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(60))
                                .cast::<u8>())
                                .wrapping_offset(28))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(8)
                            .cast::<*mut *mut u8>())
                            .read())
                            .read())
                            .wrapping_offset(((t) as i32) as isize * 4))
                            .wrapping_offset(8))
                            .cast::<i16>())
                            .read()) as i32),
                        )) as u16) as i32,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(27))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(4),
                    0,
                    10,
                    ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(27))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(64)
                    .cast::<u16>())
                    .read()) as i32)
                        .wrapping_add(
                            (((((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(60))
                                .cast::<u8>())
                                .wrapping_offset(27))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(8)
                            .cast::<*mut *mut u8>())
                            .read())
                            .read())
                            .wrapping_offset(((t) as i32) as isize * 4))
                            .wrapping_offset(4))
                            .cast::<i16>())
                            .read()) as i32),
                        )) as u16) as i32,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(27))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(4),
                    0,
                    10,
                    ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(27))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(64)
                    .cast::<u16>())
                    .read()) as i32)
                        .wrapping_add(
                            (((((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(60))
                                .cast::<u8>())
                                .wrapping_offset(27))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(8)
                            .cast::<*mut *mut u8>())
                            .read())
                            .read())
                            .wrapping_offset(((t) as i32) as isize * 4))
                            .wrapping_offset(8))
                            .cast::<i16>())
                            .read()) as i32),
                        )) as u16) as i32,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(26))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(4),
                    0,
                    10,
                    ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(26))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(64)
                    .cast::<u16>())
                    .read()) as i32)
                        .wrapping_add(
                            (((((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(60))
                                .cast::<u8>())
                                .wrapping_offset(26))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(8)
                            .cast::<*mut *mut u8>())
                            .read())
                            .read())
                            .wrapping_offset(((t) as i32) as isize * 4))
                            .wrapping_offset(4))
                            .cast::<i16>())
                            .read()) as i32),
                        )) as u16) as i32,
                );
                break 'l1;
            }
            if __sw1 == 0i32 || !__matched {
                {
                    i = 0u8;
                    'l4: loop {
                        if !(((i) as i32) < crate::c::div_i32(6i32, 2i32)) {
                            break 'l4;
                        }
                        'l5: {
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(60))
                                    .cast::<u8>())
                                    .wrapping_offset((((i) as i32).wrapping_add(26i32)) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(4),
                                0,
                                10,
                                ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(60))
                                    .cast::<u8>())
                                    .wrapping_offset((((i) as i32).wrapping_add(26i32)) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(64)
                                .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_add(
                                        (((((((((((&raw mut gSprites).cast::<u8>())
                                            .wrapping_offset(
                                                ((((((((&raw mut sRoulette)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(60))
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    (((i) as i32).wrapping_add(26i32)) as isize,
                                                ))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                        .wrapping_add(8)
                                        .cast::<*mut *mut u8>())
                                        .read())
                                        .read())
                                        .wrapping_offset(((t) as i32) as isize * 4))
                                        .wrapping_offset(8))
                                        .cast::<i16>())
                                        .read()) as i32),
                                    )) as u16) as i32,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_GridSquare(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(38)
                .cast::<i16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateWheelCenterSprite() {
    unsafe {
        let mut spriteId: u8 = 0u8;
        let mut s = crate::ffi::Align4([0u8; 8]);
        LZ77UnCompWram(
            (((&raw const sSpriteSheet_WheelCenter)
                .cast::<u8>()
                .cast_mut())
            .cast::<*mut u32>())
            .read(),
            (&raw mut gDecompressionBuffer).cast::<u8>(),
        );
        (((&raw mut s).cast::<u8>()).cast::<*mut u8>())
            .write((&raw mut gDecompressionBuffer).cast::<u8>());
        (((&raw mut s).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(
            (((&raw const sSpriteSheet_WheelCenter)
                .cast::<u8>()
                .cast_mut())
            .wrapping_add(4)
            .cast::<u16>())
            .read(),
        );
        (((&raw mut s).cast::<u8>()).wrapping_add(6).cast::<u16>()).write(
            (((&raw const sSpriteSheet_WheelCenter)
                .cast::<u8>()
                .cast_mut())
            .wrapping_add(6)
            .cast::<u16>())
            .read(),
        );
        LoadSpriteSheet((&raw mut s).cast::<u8>());
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_WheelCenter)
                .cast::<u8>()
                .cast_mut(),
            116i16,
            80i16,
            81u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<i16>())
            .read(),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(44),
            6,
            1,
            (1u8) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(44),
            7,
            1,
            (1u8) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            1,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WheelCenter(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut matrixNum: u32 = (crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32);
        let mut matrix: *mut u8 = (&raw mut gOamMatrices).cast::<u8>();
        (((matrix).wrapping_offset(((matrixNum) as i32) as isize * 8))
            .wrapping_add(6)
            .cast::<i16>())
        .write(
            (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
                .cast::<i16>())
            .read(),
        );
        (((matrix).wrapping_offset(((matrixNum) as i32) as isize * 8)).cast::<i16>()).write(
            (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
                .cast::<i16>())
            .read(),
        );
        (((matrix).wrapping_offset(((matrixNum) as i32) as isize * 8))
            .wrapping_add(2)
            .cast::<i16>())
        .write(
            (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
                .wrapping_add(2)
                .cast::<i16>())
            .read(),
        );
        (((matrix).wrapping_offset(((matrixNum) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<i16>())
        .write(
            (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
                .wrapping_add(4)
                .cast::<i16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateWheelBallSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_Ball).cast::<u8>().cast_mut(),
                        116i16,
                        80i16,
                        (((57i32).wrapping_sub(((i) as i32))) as u8),
                    ));
                    if ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 64i32
                    {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(60))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            (1u16) as i32,
                        );
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(60))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            1,
                            1,
                            (1u16) as i32,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HideWheelBalls() {
    unsafe {
        let mut spriteId: u8 = (((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(60))
        .cast::<u8>())
        .read();
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    let mut j: u8 = 0u8;
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
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        0u8,
                    );
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 8i32) {
                                break 'l3;
                            }
                            'l4: {
                                ((((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(((j) as i32) as isize))
                                .write(0i16);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    spriteId = (spriteId).wrapping_add(1);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateBallRelativeWheelAngle(sprite: *mut u8) -> i16 {
    unsafe {
        let mut sprite = sprite;
        if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36)
            .cast::<i16>())
        .read()) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
                ((((360i32).wrapping_sub(
                    ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36)
                        .cast::<i16>())
                    .read()) as i32),
                ))
                .wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32),
                )) as i16),
            );
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                >= 360i32
            {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                (__p1).write((((((__p1).read()) as i32).wrapping_sub(360i32)) as i16));
            }
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    .wrapping_sub(
                        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(36)
                            .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
            );
        }
        return ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read();
    }
}
pub(crate) unsafe extern "C" fn UpdateSlotBelowBall(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(126)).write(
            ((((UpdateBallRelativeWheelAngle(sprite)) as f32)
                / (((crate::c::div_i32(360i32, 12i32)) as f32) as f32)) as u8),
        );
        return ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(126))
            .read();
    }
}
pub(crate) unsafe extern "C" fn GetBallDistanceToSlotMidpoint(sprite: *mut u8) -> i16 {
    unsafe {
        let mut sprite = sprite;
        let mut angleIntoSlot: i16 = ((crate::c::rem_i32(
            ((UpdateBallRelativeWheelAngle(sprite)) as i32),
            crate::c::div_i32(360i32, 12i32),
        )) as i16);
        let mut distanceToMidpoint: u16 = 0u16;
        if ((angleIntoSlot) as i32)
            == (crate::c::div_i32(crate::c::div_i32(360i32, 12i32), 2i32)).wrapping_sub(1i32)
        {
            distanceToMidpoint = 0u16;
            return {
                let __v1 = ((distanceToMidpoint) as i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(__v1);
                __v1
            };
        } else {
            if ((angleIntoSlot) as i32)
                >= (crate::c::div_i32(crate::c::div_i32(360i32, 12i32), 2i32)).wrapping_sub(1i32)
            {
                distanceToMidpoint =
                    (((((crate::c::div_i32(360i32, 12i32)).wrapping_sub(1i32)).wrapping_add(
                        (crate::c::div_i32(crate::c::div_i32(360i32, 12i32), 2i32))
                            .wrapping_sub(1i32),
                    ))
                    .wrapping_sub(((angleIntoSlot) as i32))) as u16);
                return {
                    let __v2 = ((distanceToMidpoint) as i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(__v2);
                    __v2
                };
            } else {
                distanceToMidpoint =
                    ((((crate::c::div_i32(crate::c::div_i32(360i32, 12i32), 2i32))
                        .wrapping_sub(1i32))
                    .wrapping_sub(((angleIntoSlot) as i32))) as u16);
                return {
                    let __v3 = ((distanceToMidpoint) as i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(__v3);
                    __v3
                };
            }
        }
        #[allow(unreachable_code)]
        {
            return 0i16;
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateBallPos(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut sin: i16 = 0i16;
        let mut cos: i16 = 0i16;
        let __p1 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(140)
            .cast::<f32>();
        (__p1).write(
            (((((__p1).read()) as f32)
                + ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(144)
                    .cast::<f32>())
                .read()) as f32)) as f32),
        );
        let __p2 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(136)
            .cast::<f32>();
        (__p2).write(
            (((((__p2).read()) as f32)
                + ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(140)
                    .cast::<f32>())
                .read()) as f32)) as f32),
        );
        if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(136)
            .cast::<f32>())
        .read()) as f32)
            >= ((360i32) as f32)
        {
            let __p3 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(136)
                .cast::<f32>();
            (__p3).write((((((__p3).read()) as f32) - ((360.0f32) as f32)) as f32));
        } else {
            if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(136)
                .cast::<f32>())
            .read()) as f32)
                < ((0.0f32) as f32)
            {
                let __p4 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(136)
                    .cast::<f32>();
                (__p4).write((((((__p4).read()) as f32) + ((360.0f32) as f32)) as f32));
            }
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(136)
                .cast::<f32>())
            .read()) as i16),
        );
        let __p5 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(152)
            .cast::<f32>();
        (__p5).write(
            (((((__p5).read()) as f32)
                + ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(156)
                    .cast::<f32>())
                .read()) as f32)) as f32),
        );
        let __p6 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(148)
            .cast::<f32>();
        (__p6).write(
            (((((__p6).read()) as f32)
                + ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(152)
                    .cast::<f32>())
                .read()) as f32)) as f32),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(148)
                .cast::<f32>())
            .read()) as i16),
        );
        sin = Sin2(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u16),
        );
        cos = Cos2(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((sin) as i32).wrapping_mul(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            ) >> 12) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            (((((cos) as i32).wrapping_neg()).wrapping_mul(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            ) >> 12) as i16),
        );
        if (IsSEPlaying()) != 0 {
            m4aMPlayPanpotControl(
                (&raw mut gMPlayInfo_SE1).cast::<u8>(),
                65535u16,
                ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i8),
            );
            m4aMPlayPanpotControl(
                (&raw mut gMPlayInfo_SE2).cast::<u8>(),
                65535u16,
                ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BallLandInSlot(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut sin: i16 = 0i16;
        let mut cos: i16 = 0i16;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32),
                )) as i16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            >= 360i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(360i32)) as i16));
        }
        sin = Sin2(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u16),
        );
        cos = Cos2(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((sin) as i32).wrapping_mul(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            ) >> 12) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            (((((cos) as i32).wrapping_neg()).wrapping_mul(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            ) >> 12) as i16),
        );
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32)
                .wrapping_add(((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as i32)))
                as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UnstickBall_ShroomishBallFall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        UpdateBallPos(sprite);
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            < (-132i32))
            || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                > 80i32)
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        } else {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            >= crate::c::div_i32(360i32, 12i32)
        {
            if !(((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0) {
                if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<f32>())
                .read()) as f32)
                    <= ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(160)
                        .cast::<f32>())
                    .read()) as f32)
                        - ((2.0f32) as f32)) as f32)
                {
                    {
                        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(125))
                        .write(255u8);
                        crate::c::bf_write(
                            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3),
                            7,
                            1,
                            (0u8) as i32,
                        );
                        StartSpriteAnim(
                            sprite,
                            ((((((sprite).wrapping_add(43)).read()) as i32).wrapping_add(3i32))
                                as u8),
                        );
                        UpdateSlotBelowBall(sprite);
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                            .write(30i16);
                        UpdateBallRelativeWheelAngle(sprite);
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
                            ((((crate::c::div_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                                    .read()) as i32),
                                crate::c::div_i32(360i32, 12i32),
                            ))
                            .wrapping_mul(crate::c::div_i32(360i32, 12i32)))
                            .wrapping_add(15i32)) as i16),
                        );
                        ((sprite)
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(SpriteCB_BallLandInSlot));
                        m4aSongNumStartOrChange(71u16);
                    }
                    ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(156)
                        .cast::<f32>())
                    .write(
                        (({
                            let __v2 = ((0.0f32) as f32);
                            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(152)
                                .cast::<f32>())
                            .write(__v2);
                            __v2
                        }) as f32),
                    );
                    ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(140)
                        .cast::<f32>())
                    .write(((-(1.0f32)) as f32));
                }
            } else {
                if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<f32>())
                .read()) as f32)
                    >= ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(160)
                        .cast::<f32>())
                    .read()) as f32)
                        - ((2.0f32) as f32)) as f32)
                {
                    {
                        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(125))
                        .write(255u8);
                        crate::c::bf_write(
                            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3),
                            7,
                            1,
                            (0u8) as i32,
                        );
                        StartSpriteAnim(
                            sprite,
                            ((((((sprite).wrapping_add(43)).read()) as i32).wrapping_add(3i32))
                                as u8),
                        );
                        UpdateSlotBelowBall(sprite);
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                            .write(30i16);
                        UpdateBallRelativeWheelAngle(sprite);
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
                            ((((crate::c::div_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                                    .read()) as i32),
                                crate::c::div_i32(360i32, 12i32),
                            ))
                            .wrapping_mul(crate::c::div_i32(360i32, 12i32)))
                            .wrapping_add(15i32)) as i16),
                        );
                        ((sprite)
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(SpriteCB_BallLandInSlot));
                        m4aSongNumStartOrChange(71u16);
                    }
                    ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(156)
                        .cast::<f32>())
                    .write(
                        (({
                            let __v3 = ((0.0f32) as f32);
                            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(152)
                                .cast::<f32>())
                            .write(__v3);
                            __v3
                        }) as f32),
                    );
                    ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(140)
                        .cast::<f32>())
                    .write(((-(1.0f32)) as f32));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UnstickBall_Shroomish(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut slotOffset: f32 = 0.0;
        let mut ballFallDist: f32 = 0.0;
        let mut ballFallSpeed: f32 = 0.0;
        UpdateBallPos(sprite);
        'l1: {
            let __sw1 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 180i32;
            if __sw1 == 0i32 {
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 1i32 {
                    slotOffset = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                        .read()) as f32);
                    ballFallDist = ((((((slotOffset) as f32)
                        * (((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(4),
                                0,
                                2,
                                false,
                            ) as u8) as i32) as isize
                                * 32,
                        ))
                        .wrapping_add(1))
                        .read()) as f32)) as f32)
                        + (((((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(4),
                                0,
                                2,
                                false,
                            ) as u8) as i32) as isize
                                * 32,
                        ))
                        .wrapping_add(2))
                        .read()) as i32)
                            .wrapping_sub(1i32)) as f32))
                        as f32);
                    ballFallSpeed = ((((slotOffset) as f32)
                        / ((((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(4),
                                0,
                                2,
                                false,
                            ) as u8) as i32) as isize
                                * 32,
                        ))
                        .wrapping_add(8))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .read()) as f32)) as f32);
                } else {
                    return;
                }
                break 'l1;
            }
            if __sw1 == 180i32 {
                if ((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0 {
                    slotOffset = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                        .read()) as f32);
                    ballFallDist = ((((((slotOffset) as f32)
                        * (((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(4),
                                0,
                                2,
                                false,
                            ) as u8) as i32) as isize
                                * 32,
                        ))
                        .wrapping_add(1))
                        .read()) as f32)) as f32)
                        + (((((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(4),
                                0,
                                2,
                                false,
                            ) as u8) as i32) as isize
                                * 32,
                        ))
                        .wrapping_add(2))
                        .read()) as i32)
                            .wrapping_sub(1i32)) as f32))
                        as f32);
                    ballFallSpeed = ((-(((slotOffset) as f32)
                        / ((((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(4),
                                0,
                                2,
                                false,
                            ) as u8) as i32) as isize
                                * 32,
                        ))
                        .wrapping_add(8))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .read()) as f32))) as f32);
                } else {
                    return;
                }
                break 'l1;
            }
            if !__matched {
                return;
            }
        }
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(160)
            .cast::<f32>())
        .write(
            ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(148)
                .cast::<f32>())
            .read()) as f32),
        );
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(152)
            .cast::<f32>())
        .write(((ballFallSpeed) as f32));
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(156)
            .cast::<f32>())
        .write(
            ((-(((((((ballFallSpeed) as f32) * ((2.0f32) as f32)) as f32) / ((ballFallDist) as f32))
                as f32)
                + ((((2.0f32) as f32)
                    / ((((ballFallDist) as f32) * ((ballFallDist) as f32)) as f32))
                    as f32))) as f32),
        );
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(140)
            .cast::<f32>())
        .write(((0.0f32) as f32));
        crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (0u8) as i32);
        ((sprite).wrapping_add(42)).write(0u8);
        crate::c::bf_write((sprite).wrapping_add(63), 2, 1, (1u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(63), 4, 1, (0u16) as i32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_UnstickBall_ShroomishBallFall));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UnstickBall_TaillowDrop(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            (((((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                as f32)
                * ((0.05f32) as f32)) as f32)
                * ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as f32)) as i16) as i32)
                .wrapping_sub(45i32)) as i16),
        );
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            >= crate::c::div_i32(360i32, 12i32))
            && (((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) >= 0i32)
        {
            {
                ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(125))
                    .write(255u8);
                crate::c::bf_write(
                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
                    7,
                    1,
                    (0u8) as i32,
                );
                StartSpriteAnim(
                    sprite,
                    ((((((sprite).wrapping_add(43)).read()) as i32).wrapping_add(3i32)) as u8),
                );
                UpdateSlotBelowBall(sprite);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(30i16);
                UpdateBallRelativeWheelAngle(sprite);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
                    ((((crate::c::div_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32),
                        crate::c::div_i32(360i32, 12i32),
                    ))
                    .wrapping_mul(crate::c::div_i32(360i32, 12i32)))
                    .wrapping_add(15i32)) as i16),
                );
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_BallLandInSlot));
                m4aSongNumStartOrChange(71u16);
            }
            crate::c::bf_write(
                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
                6,
                1,
                (1u8) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UnstickBall_TaillowPickUp(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            < 45i32
        {
            let __p3 = (sprite).wrapping_add(38).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_sub(1));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                == 45i32
            {
                if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60))
                    .cast::<u8>())
                    .wrapping_offset(55))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(43))
                .read()) as i32)
                    == 1i32
                {
                    let __p4 = (sprite).wrapping_add(38).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
            }
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            {
                if ((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(55))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(44),
                    0,
                    6,
                    false,
                ) as u8) as i32)
                    == 0i32
                {
                    if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(55))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(43))
                    .read()) as i32)
                        == 1i32
                    {
                        let __p5 = (sprite).wrapping_add(38).cast::<i16>();
                        (__p5).write(((__p5).read()).wrapping_add(1));
                    } else {
                        let __p6 = (sprite).wrapping_add(38).cast::<i16>();
                        (__p6).write(((__p6).read()).wrapping_sub(1));
                    }
                }
            } else {
                crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (0u8) as i32);
                ((sprite).wrapping_add(42)).write(1u8);
                crate::c::bf_write((sprite).wrapping_add(63), 2, 1, (1u16) as i32);
                crate::c::bf_write((sprite).wrapping_add(63), 4, 1, (0u16) as i32);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_UnstickBall_TaillowDrop));
                m4aSongNumStart(61u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UnstickBall_Taillow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        UpdateBallPos(sprite);
        'l1: {
            let __sw1 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32);
            if __sw1 == 90i32 {
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 1i32 {
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_UnstickBall_TaillowPickUp));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 270i32 {
                if ((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0 {
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_UnstickBall_TaillowPickUp));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UnstickBall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        UpdateBallPos(sprite);
        'l1: {
            let __sw1 = ((crate::c::bf_read(
                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
                0,
                5,
                false,
            ) as u8) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 || !__matched {
                CreateShroomishSprite(sprite);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_UnstickBall_Shroomish));
                break 'l1;
            }
            if __sw1 == 1i32 {
                CreateTaillowSprite(sprite);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_UnstickBall_Taillow));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_RollBall_TryLandAdjacent(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        UpdateBallPos(sprite);
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            __t2
        }) as i32)
            == 16i32
        {
            let __p3 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(152)
                .cast::<f32>();
            (__p3).write((((((__p3).read()) as f32) * ((-(1.0f32)) as f32)) as f32));
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            if !(((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0) {
                {
                    ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(125))
                    .write(255u8);
                    crate::c::bf_write(
                        (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3),
                        7,
                        1,
                        (0u8) as i32,
                    );
                    StartSpriteAnim(
                        sprite,
                        ((((((sprite).wrapping_add(43)).read()) as i32).wrapping_add(3i32)) as u8),
                    );
                    UpdateSlotBelowBall(sprite);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(30i16);
                    UpdateBallRelativeWheelAngle(sprite);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
                        ((((crate::c::div_i32(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                                .read()) as i32),
                            crate::c::div_i32(360i32, 12i32),
                        ))
                        .wrapping_mul(crate::c::div_i32(360i32, 12i32)))
                        .wrapping_add(15i32)) as i16),
                    );
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_BallLandInSlot));
                    m4aSongNumStartOrChange(71u16);
                }
            } else {
                crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (1u8) as i32);
                m4aSongNumStart(56u16);
                SetBallStuck(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_RollBall_TryLand(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        UpdateBallPos(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        UpdateSlotBelowBall(sprite);
        if !(((((((&raw const sRouletteSlots).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(126))
                .read()) as i32) as isize
                    * 8,
            ))
        .wrapping_add(4)
        .cast::<u32>())
        .read()
            & ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u32>())
            .read())
            != 0)
        {
            {
                ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(125))
                    .write(255u8);
                crate::c::bf_write(
                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
                    7,
                    1,
                    (0u8) as i32,
                );
                StartSpriteAnim(
                    sprite,
                    ((((((sprite).wrapping_add(43)).read()) as i32).wrapping_add(3i32)) as u8),
                );
                UpdateSlotBelowBall(sprite);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(30i16);
                UpdateBallRelativeWheelAngle(sprite);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
                    ((((crate::c::div_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32),
                        crate::c::div_i32(360i32, 12i32),
                    ))
                    .wrapping_mul(crate::c::div_i32(360i32, 12i32)))
                    .wrapping_add(15i32)) as i16),
                );
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_BallLandInSlot));
                m4aSongNumStartOrChange(71u16);
            }
        } else {
            let mut slotId: u8 = 0u8;
            let mut fallRight: u32 = 0u32;
            m4aSongNumStart(56u16);
            fallRight = ((((Random()) as i32) & 1i32) as u32);
            if (fallRight) != 0 {
                ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(140)
                    .cast::<f32>())
                .write(((0.0f32) as f32));
                ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(127))
                    .write({
                        let __v1 = ((crate::c::rem_i32(
                            ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(126))
                            .read()) as i32)
                                .wrapping_add(1i32),
                            12i32,
                        )) as u8);
                        slotId = __v1;
                        __v1
                    });
            } else {
                let mut temp: f32 = 0.0;
                ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(140)
                    .cast::<f32>())
                .write(
                    (((({
                        let __v2 = (((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(4),
                                0,
                                2,
                                false,
                            ) as u8) as i32) as isize
                                * 32,
                        ))
                        .wrapping_add(28)
                        .cast::<f32>())
                        .read()) as f32);
                        temp = __v2;
                        __v2
                    }) as f32)
                        * ((2.0f32) as f32)) as f32),
                );
                slotId = ((crate::c::rem_i32(
                    (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(126))
                    .read()) as i32)
                        .wrapping_add(12i32))
                    .wrapping_sub(1i32),
                    12i32,
                )) as u8);
                ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(127))
                    .write(
                        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(126))
                        .read(),
                    );
            }
            if ((((((&raw const sRouletteSlots).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((slotId) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<u32>())
            .read()
                & ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u32>())
                .read())
                != 0
            {
                (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    (((((((&raw const sRouletteTables).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(4),
                                0,
                                2,
                                false,
                            ) as u8) as i32) as isize
                                * 32,
                        ))
                    .wrapping_add(2))
                    .read()) as i16),
                );
            } else {
                (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                if (crate::c::bf_read(
                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4),
                    0,
                    2,
                    false,
                ) as u8)
                    != 0
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                        (((((((&raw const sRouletteTables).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::bf_read(
                                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(4),
                                    0,
                                    2,
                                    false,
                                ) as u8) as i32) as isize
                                    * 32,
                            ))
                        .wrapping_add(1))
                        .read()) as i16),
                    );
                } else {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                        (((((((&raw const sRouletteTables).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::bf_read(
                                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(4),
                                    0,
                                    2,
                                    false,
                                ) as u8) as i32) as isize
                                    * 32,
                            ))
                        .wrapping_add(2))
                        .read()) as i16),
                    );
                    if (fallRight) != 0 {
                        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(140)
                            .cast::<f32>())
                        .write(((0.5f32) as f32));
                    } else {
                        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(140)
                            .cast::<f32>())
                        .write(((-(1.5f32)) as f32));
                    }
                }
            }
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(152)
                .cast::<f32>())
            .write(((0.085f32) as f32));
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_RollBall_TryLandAdjacent));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(5i16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_RollBall_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        UpdateBallPos(sprite);
        if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(140)
            .cast::<f32>())
        .read()) as f32)
            > ((0.5f32) as f32)
        {
            return;
        }
        UpdateSlotBelowBall(sprite);
        if ((GetBallDistanceToSlotMidpoint(sprite)) as i32) == 0i32 {
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(144)
                .cast::<f32>())
            .write(((0.0f32) as f32));
            let __p1 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(140)
                .cast::<f32>();
            (__p1).write(
                (((((__p1).read()) as f32)
                    - ((((((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize
                            * 32,
                    ))
                    .wrapping_add(3))
                    .read()) as f32) as f32)
                        / (((((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(4),
                                0,
                                2,
                                false,
                            ) as u8) as i32) as isize
                                * 32,
                        ))
                        .wrapping_add(4))
                        .read()) as i32)
                            .wrapping_add(1i32)) as f32)) as f32)) as f32),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(4i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_RollBall_TryLand));
        } else {
            if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(144)
                .cast::<f32>())
            .read()) as f32)
                != ((0.0f32) as f32)
            {
                if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(140)
                    .cast::<f32>())
                .read()) as f32)
                    < ((0.0f32) as f32)
                {
                    ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144)
                        .cast::<f32>())
                    .write(((0.0f32) as f32));
                    ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(140)
                        .cast::<f32>())
                    .write(((0.0f32) as f32));
                    let __p2 = (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(152)
                        .cast::<f32>();
                    (__p2).write((((((__p2).read()) as f32) / ((1.2f32) as f32)) as f32));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_RollBall_Medium(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        UpdateBallPos(sprite);
        if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(148)
            .cast::<f32>())
        .read()) as f32)
            > ((40.0f32) as f32)
        {
            return;
        }
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(152)
            .cast::<f32>())
        .write(
            ((-(((4.0f32) as f32)
                / (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(134)
                    .cast::<u16>())
                .read()) as f32) as f32))) as f32),
        );
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(144)
            .cast::<f32>())
        .write(
            ((-(((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(140)
                .cast::<f32>())
            .read()) as f32)
                / (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(134)
                    .cast::<u16>())
                .read()) as f32) as f32))) as f32),
        );
        ((sprite).wrapping_add(42)).write(2u8);
        crate::c::bf_write((sprite).wrapping_add(63), 2, 1, (1u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(63), 4, 1, (0u16) as i32);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(3i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_RollBall_Slow));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_RollBall_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        UpdateBallPos(sprite);
        if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(148)
            .cast::<f32>())
        .read()) as f32)
            > ((60.0f32) as f32)
        {
            return;
        }
        m4aSongNumStartOrChange(93u16);
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(152)
            .cast::<f32>())
        .write(
            ((-(((20.0f32) as f32)
                / (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(132)
                    .cast::<u16>())
                .read()) as f32) as f32))) as f32),
        );
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(144)
            .cast::<f32>())
        .write(
            ((((((1.0f32) as f32)
                - ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(140)
                    .cast::<f32>())
                .read()) as f32)) as f32)
                / (((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(132)
                    .cast::<u16>())
                .read()) as f32) as f32)) as f32),
        );
        ((sprite).wrapping_add(42)).write(1u8);
        crate::c::bf_write((sprite).wrapping_add(63), 2, 1, (1u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(63), 4, 1, (0u16) as i32);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(2i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_RollBall_Medium));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_RollBall_Start(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(1i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        UpdateBallPos(sprite);
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_RollBall_Fast));
    }
}
pub(crate) unsafe extern "C" fn CreateShroomishSprite(ball: *mut u8) {
    unsafe {
        let mut ball = ball;
        let mut t: u16 = 0u16;
        let mut i: u8 = 0u8;
        let mut coords = crate::ffi::Align4([0u8; 8]);
        (&raw mut coords)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(0)
            .cast::<i16>()
            .write(116i16);
        (&raw mut coords)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(2)
            .cast::<i16>()
            .write(44i16);
        (&raw mut coords)
            .cast::<u8>()
            .wrapping_add(4)
            .wrapping_add(0)
            .cast::<i16>()
            .write(116i16);
        (&raw mut coords)
            .cast::<u8>()
            .wrapping_add(4)
            .wrapping_add(2)
            .cast::<i16>()
            .write(112i16);
        let mut roulette: *mut u8 = core::ptr::null_mut();
        t = ((((((((ball).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            .wrapping_sub(2i32)) as u16);
        roulette = ((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read();
        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(60))
            .cast::<u8>())
        .wrapping_offset(55))
        .write(CreateSprite(
            (&raw const sSpriteTemplate_Shroomish)
                .cast::<u8>()
                .cast_mut(),
            36i16,
            (-12i16),
            50u8,
        ));
        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(60))
            .cast::<u8>())
        .wrapping_offset(56))
        .write(CreateSprite(
            ((&raw const sSpriteTemplate_ShroomishShadow)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            ((((&raw mut coords).cast::<u8>()).wrapping_offset(
                (((((ball).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 4,
            ))
            .cast::<i16>())
            .read(),
            (((((&raw mut coords).cast::<u8>()).wrapping_offset(
                (((((ball).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 4,
            ))
            .cast::<i16>())
            .wrapping_offset(1))
            .read(),
            59u8,
        ));
        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(60))
            .cast::<u8>())
        .wrapping_offset(57))
        .write(CreateSprite(
            (((&raw const sSpriteTemplate_ShroomishShadow)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(24),
            36i16,
            140i16,
            51u8,
        ));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60))
                .cast::<u8>())
                .wrapping_offset(57))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(1),
            2,
            2,
            (1u32) as i32,
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(60))
                            .cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_add(55i32)) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        1,
                        1,
                        (0u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(60))
                            .cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_add(55i32)) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(60))
                            .cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_add(55i32)) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(44),
                        6,
                        1,
                        (1u8) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(60))
                            .cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_add(55i32)) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(44),
                        7,
                        1,
                        (1u8) as i32,
                    );
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset((((i) as i32).wrapping_add(55i32)) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(55))
                        .read()) as i16),
                    );
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset((((i) as i32).wrapping_add(55i32)) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(56))
                        .read()) as i16),
                    );
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset((((i) as i32).wrapping_add(55i32)) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .write(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(57))
                        .read()) as i16),
                    );
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset((((i) as i32).wrapping_add(55i32)) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(((t) as i16));
                    (((((((&raw mut gSprites)).cast::<u8>()).wrapping_offset((((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(60)).cast::<u8>()).wrapping_offset((((((i) as i32))).wrapping_add(55i32)) as isize)).read()) as i32)) as isize * 68)).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write((((((((((((ball).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32))).wrapping_mul(((((((((&raw const sRouletteTables).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(((((crate::c::bf_read((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4), 0, 2, false) as u8)) as i32)) as isize * 32)).wrapping_add(1)).read()) as i32)))).wrapping_add((((((((((&raw const sRouletteTables).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(((((crate::c::bf_read((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4), 0, 2, false) as u8)) as i32)) as isize * 32)).wrapping_add(2)).read()) as i32))).wrapping_add(65535i32))) as i16));
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60))
                .cast::<u8>())
                .wrapping_offset(56))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            1,
            1,
            (1u16) as i32,
        );
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(56)
            .cast::<*mut u8>())
        .write(ball);
    }
}
pub(crate) unsafe extern "C" fn CreateTaillowSprite(ball: *mut u8) {
    unsafe {
        let mut ball = ball;
        let mut i: u8 = 0u8;
        let mut t: i16 = 0i16;
        let mut coords = crate::ffi::Align4([0u8; 8]);
        (&raw mut coords)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(0)
            .cast::<i16>()
            .write(256i16);
        (&raw mut coords)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(2)
            .cast::<i16>()
            .write(84i16);
        (&raw mut coords)
            .cast::<u8>()
            .wrapping_add(4)
            .wrapping_add(0)
            .cast::<i16>()
            .write((-16i16));
        (&raw mut coords)
            .cast::<u8>()
            .wrapping_add(4)
            .wrapping_add(2)
            .cast::<i16>()
            .write(84i16);
        t = ((((((((ball).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            .wrapping_sub(2i32)) as i16);
        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(60))
            .cast::<u8>())
        .wrapping_offset(55))
        .write(CreateSprite(
            (&raw const sSpriteTemplate_Taillow).cast::<u8>().cast_mut(),
            ((((&raw mut coords).cast::<u8>()).wrapping_offset(
                (((((ball).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 4,
            ))
            .cast::<i16>())
            .read(),
            (((((&raw mut coords).cast::<u8>()).wrapping_offset(
                (((((ball).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 4,
            ))
            .cast::<i16>())
            .wrapping_offset(1))
            .read(),
            50u8,
        ));
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60))
                .cast::<u8>())
                .wrapping_offset(55))
                .read()) as i32) as isize
                    * 68,
            ),
            (((((ball).wrapping_add(46)).cast::<i16>()).read()) as u8),
        );
        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(60))
            .cast::<u8>())
        .wrapping_offset(56))
        .write(CreateSprite(
            (&raw const sSpriteTemplate_TaillowShadow)
                .cast::<u8>()
                .cast_mut(),
            ((((&raw mut coords).cast::<u8>()).wrapping_offset(
                (((((ball).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 4,
            ))
            .cast::<i16>())
            .read(),
            (((((&raw mut coords).cast::<u8>()).wrapping_offset(
                (((((ball).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 4,
            ))
            .cast::<i16>())
            .wrapping_offset(1))
            .read(),
            51u8,
        ));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60))
                .cast::<u8>())
                .wrapping_offset(56))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(44),
            7,
            1,
            (1u8) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60))
                .cast::<u8>())
                .wrapping_offset(56))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(44),
            6,
            1,
            (1u8) as i32,
        );
        ((((ball).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            (((((t) as i32).wrapping_mul(
                (((((((&raw const sRouletteTables).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize
                            * 32,
                    ))
                .wrapping_add(1))
                .read()) as i32),
            ))
            .wrapping_add(
                ((((((((&raw const sRouletteTables).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize
                            * 32,
                    ))
                .wrapping_add(16))
                .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(45i32),
            )) as i16),
        );
        {
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(((55i32).wrapping_add(((i) as i32))) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(55))
                        .read()) as i16),
                    );
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(((55i32).wrapping_add(((i) as i32))) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(56))
                        .read()) as i16),
                    );
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(((55i32).wrapping_add(((i) as i32))) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .write(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(56))
                        .read()) as i16),
                    );
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(((55i32).wrapping_add(((i) as i32))) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(t);
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(((55i32).wrapping_add(((i) as i32))) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(
                        ((((((((ball).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            .wrapping_sub(45i32)) as i16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(56)
            .cast::<*mut u8>())
        .write(ball);
    }
}
pub(crate) unsafe extern "C" fn SetBallStuck(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut slotId: u8 = 0u8;
        let mut angle: u16 = 0u16;
        let mut numCandidates: u8 = 0u8;
        let mut maxSlotToCheck: u8 = 5u8;
        let mut betSlotId: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut slotsToSkip: u8 = 0u8;
        let mut slotCandidates = crate::ffi::Align4([0u8; 10]);
        let mut rand: u16 = Random();
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(125))
            .write(1u8);
        crate::c::bf_write(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
            5,
            1,
            (1u8) as i32,
        );
        crate::c::bf_write(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
            6,
            1,
            (0u8) as i32,
        );
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(126))
            .write(255u8);
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(136)
            .cast::<f32>())
        .write(((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as f32));
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(152)
            .cast::<f32>())
        .write(((0.0f32) as f32));
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(140)
            .cast::<f32>())
        .write(
            (((((((&raw const sRouletteTables).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((crate::c::bf_read(
                        (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4),
                        0,
                        2,
                        false,
                    ) as u8) as i32) as isize
                        * 32,
                ))
            .wrapping_add(28)
            .cast::<f32>())
            .read()) as f32),
        );
        angle = ((((((crate::c::bf_read(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4),
            0,
            2,
            false,
        ) as u8) as i32)
            .wrapping_mul(crate::c::div_i32(360i32, 12i32)))
        .wrapping_add(33i32))
        .wrapping_add(
            ((1i32).wrapping_sub(
                ((crate::c::bf_read(
                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
                    0,
                    5,
                    false,
                ) as u8) as i32),
            ))
            .wrapping_mul(15i32),
        )) as u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((angle) as i32)
                        < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32))
                        && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                            .read()) as i32)
                            <= ((angle) as i32).wrapping_add(90i32))
                    {
                        (((sprite).wrapping_add(46)).cast::<i16>())
                            .write(((crate::c::div_i32(((i) as i32), 2i32)) as i16));
                        crate::c::bf_write(
                            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3),
                            0,
                            5,
                            ((crate::c::rem_i32(((i) as i32), 2i32)) as u8) as i32,
                        );
                        break 'l1;
                    }
                    if ((i) as i32) == 3i32 {
                        (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
                        crate::c::bf_write(
                            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3),
                            0,
                            5,
                            (1u8) as i32,
                        );
                        break 'l1;
                    }
                    angle = ((((angle) as i32).wrapping_add(90i32)) as u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        if (crate::c::bf_read(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
            0,
            5,
            false,
        ) as u8)
            != 0
        {
            if ((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0 {
                PlayCry_Normal(304u16, (-63i8));
            } else {
                PlayCry_Normal(304u16, 63i8);
            }
        } else {
            PlayCry_Normal(306u16, (-63i8));
        }
        slotsToSkip = 2u8;
        slotId = ((crate::c::rem_i32(
            ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(127))
                .read()) as i32)
                .wrapping_add(2i32),
            12i32,
        )) as u8);
        if (((crate::c::bf_read(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
            0,
            5,
            false,
        ) as u8) as i32)
            == 1i32)
            && (((crate::c::bf_read(
                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4),
                0,
                2,
                false,
            ) as u8) as i32)
                == 1i32)
        {
            maxSlotToCheck = ((((maxSlotToCheck) as i32).wrapping_add(6i32)) as u8);
        } else {
            maxSlotToCheck =
                ((((maxSlotToCheck) as i32).wrapping_add(((slotsToSkip) as i32))) as u8);
        }
        {
            i = slotsToSkip;
            'l3: loop {
                if !(((i) as i32) < ((maxSlotToCheck) as i32)) {
                    break 'l3;
                }
                'l4: {
                    if !((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<u32>())
                    .read()
                        & (((((&raw const sRouletteSlots).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((slotId) as i32) as isize * 8))
                        .wrapping_add(4)
                        .cast::<u32>())
                        .read())
                        != 0)
                    {
                        (((&raw mut slotCandidates).cast::<u8>()).wrapping_offset(
                            (({
                                let __t1 = numCandidates;
                                numCandidates = (numCandidates).wrapping_add(1);
                                __t1
                            }) as i32) as isize,
                        ))
                        .write(i);
                        if (((betSlotId) as i32) == 0i32)
                            && (((((((&raw const sRouletteSlots).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((slotId) as i32) as isize * 8))
                            .wrapping_add(4)
                            .cast::<u32>())
                            .read()
                                & (((((&raw const sGridSelections).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(27))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((crate::c::bf_read(
                                            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(26),
                                            0,
                                            4,
                                            false,
                                        ) as u8) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 20,
                                ))
                                .wrapping_add(12)
                                .cast::<u32>())
                                .read())
                                != 0)
                        {
                            betSlotId = i;
                        }
                    }
                    slotId =
                        ((crate::c::rem_i32(((slotId) as i32).wrapping_add(1i32), 12i32)) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((crate::c::bf_read(
            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
            0,
            5,
            false,
        ) as u8) as i32)
            .wrapping_add(1i32)
            & ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                .read()) as i32))
            != 0
        {
            if ((betSlotId) != 0) && (crate::c::rem_i32(((rand) as i32), 256i32) < 192i32) {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                    .write(((betSlotId) as i16));
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                    (((((&raw mut slotCandidates).cast::<u8>()).wrapping_offset(
                        (crate::c::rem_i32(((rand) as i32), ((numCandidates) as i32))) as isize,
                    ))
                    .read()) as i16),
                );
            }
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                (((((&raw mut slotCandidates).cast::<u8>()).wrapping_offset(
                    (crate::c::rem_i32(((rand) as i32), ((numCandidates) as i32))) as isize,
                ))
                .read()) as i16),
            );
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_UnstickBall));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ShroomishExit(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            >= ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
        {
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_sub(2i32)) as i16));
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < (-16i32) {
                if !((crate::c::bf_read(
                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
                    6,
                    1,
                    false,
                ) as u8)
                    != 0)
                {
                    crate::c::bf_write(
                        (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3),
                        6,
                        1,
                        (1u8) as i32,
                    );
                }
                DestroySprite(sprite);
                ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
                    .write(0u8);
                ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52)
                    .cast::<u16>())
                .write(
                    (((&raw const sShroomishShadowAlphas)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .read(),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ShroomishShakeScreen(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut screenShakeIdx: i32 = 0i32;
        let mut screenShakeOffsets = crate::ffi::Align4([0u8; 24]);
        (&raw mut screenShakeOffsets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(0)
            .cast::<u16>()
            .write(65535u16);
        (&raw mut screenShakeOffsets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(2)
            .cast::<u16>()
            .write(0u16);
        (&raw mut screenShakeOffsets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(4)
            .cast::<u16>()
            .write(1u16);
        (&raw mut screenShakeOffsets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(6)
            .cast::<u16>()
            .write(0u16);
        (&raw mut screenShakeOffsets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(0)
            .cast::<u16>()
            .write(65534u16);
        (&raw mut screenShakeOffsets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(2)
            .cast::<u16>()
            .write(0u16);
        (&raw mut screenShakeOffsets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(4)
            .cast::<u16>()
            .write(2u16);
        (&raw mut screenShakeOffsets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(6)
            .cast::<u16>()
            .write(0u16);
        (&raw mut screenShakeOffsets)
            .cast::<u8>()
            .wrapping_add(16)
            .wrapping_add(0)
            .cast::<u16>()
            .write(65533u16);
        (&raw mut screenShakeOffsets)
            .cast::<u8>()
            .wrapping_add(16)
            .wrapping_add(2)
            .cast::<u16>()
            .write(0u16);
        (&raw mut screenShakeOffsets)
            .cast::<u8>()
            .wrapping_add(16)
            .wrapping_add(4)
            .cast::<u16>()
            .write(3u16);
        (&raw mut screenShakeOffsets)
            .cast::<u8>()
            .wrapping_add(16)
            .wrapping_add(6)
            .cast::<u16>()
            .write(0u16);
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
        {
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                & 1i32)
                != 0
            {
                ((&raw mut gSpriteCoordOffsetY).cast::<i16>()).write(
                    (((((((&raw mut screenShakeOffsets).cast::<u8>()).wrapping_offset(
                        (crate::c::div_i32(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                .read()) as i32),
                            2i32,
                        )) as isize
                            * 8,
                    ))
                    .cast::<u16>())
                    .wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32) as isize,
                    ))
                    .read()) as i16),
                );
                screenShakeIdx = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                    .read()) as i32)
                    .wrapping_add(1i32);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                    (((screenShakeIdx)
                        .wrapping_sub((crate::c::div_i32(screenShakeIdx, 4i32)).wrapping_mul(4i32)))
                        as i16),
                );
            }
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                    ^ 1i32) as u16) as i32,
            );
        } else {
            ((&raw mut gSpriteCoordOffsetY).cast::<i16>()).write(0i16);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60))
                    .cast::<u8>())
                    .wrapping_offset(55))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(44),
                6,
                1,
                (0u8) as i32,
            );
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ShroomishFall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut timer: f32 = 0.0;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        timer = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as f32);
        ((sprite).wrapping_add(38).cast::<i16>())
            .write(((((((timer) as f32) * ((0.039f32) as f32)) as f32) * ((timer) as f32)) as i16));
        ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(52)
            .cast::<u16>())
        .write(
            ((((&raw const sShroomishShadowAlphas)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(
                (crate::c::div_i32(
                    ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1))
                    .read()) as i32)
                        .wrapping_sub(1i32),
                    2i32,
                )) as isize,
            ))
            .read(),
        );
        if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .read()) as u32)
            < ((crate::c::div_u32(20u32, 2u32)).wrapping_mul(2u32)).wrapping_sub(1u32)
        {
            let __p2 =
                (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            > 60i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_ShroomishExit));
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_ShroomishExit));
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write((-2i16));
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_ShroomishShakeScreen));
            m4aSongNumStart(214u16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Shroomish(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            == 0i32
        {
            if !(((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(56)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .read())
                != 0)
            {
                if ((((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(56)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    != ((((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize
                            * 32,
                    ))
                    .wrapping_add(8))
                    .cast::<u16>())
                    .read()) as i32)
                {
                    return;
                }
            } else {
                if ((((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(56)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    != ((((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize
                            * 32,
                    ))
                    .wrapping_add(8))
                    .cast::<u16>())
                    .read()) as i32)
                        .wrapping_add(180i32)
                {
                    return;
                }
            }
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p1).write(((__p1).read()).wrapping_add(1));
            m4aSongNumStart(43u16);
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
                .write(1u8);
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(52)
                .cast::<u16>())
            .write(
                (((&raw const sShroomishShadowAlphas)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .read(),
            );
        } else {
            ((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(52)
                .cast::<u16>())
            .write(
                ((((&raw const sShroomishShadowAlphas)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(
                    (crate::c::div_i32(
                        ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1))
                        .read()) as i32)
                            .wrapping_sub(1i32),
                        2i32,
                    )) as isize,
                ))
                .read(),
            );
            if ((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
                .read()) as i32)
                < 19i32
            {
                let __p2 =
                    (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1);
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if !(((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(56)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .read())
                != 0)
            {
                if ((((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(56)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    != ((((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize
                            * 32,
                    ))
                    .wrapping_add(8))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read()) as i32)
                {
                    return;
                }
            } else {
                if ((((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(56)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    != ((((((((&raw const sRouletteTables).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize
                            * 32,
                    ))
                    .wrapping_add(8))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read()) as i32)
                        .wrapping_add(180i32)
                {
                    return;
                }
            }
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_ShroomishFall));
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TaillowShadow_Flash(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write(
            (sprite).wrapping_add(62),
            2,
            1,
            ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32) ^ 1i32)
                as u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Taillow_FlyAway(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) > (-16i32) {
            let __p1 = (sprite).wrapping_add(34).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (1u8) as i32);
            m4aSongNumStop(94u16);
            DestroySprite(sprite);
            FreeOamMatrix(
                ((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u8>())
                        .wrapping_offset(56))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(3),
                    1,
                    5,
                    false,
                ) as u32) as u8),
            );
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60))
                    .cast::<u8>())
                    .wrapping_offset(56))
                    .read()) as i32) as isize
                        * 68,
                ),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Taillow_PickUpBall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            >= 0i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_sub(1));
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                == 0i32)
                && (((((sprite).wrapping_add(43)).read()) as i32) == 1i32)
            {
                let __p3 = (sprite).wrapping_add(38).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                >= 0i32
            {
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p4).write(((__p4).read()).wrapping_sub(1));
                if ((crate::c::bf_read((sprite).wrapping_add(44), 0, 6, false) as u8) as i32)
                    == 0i32
                {
                    if ((((sprite).wrapping_add(43)).read()) as i32) == 1i32 {
                        let __p5 = (sprite).wrapping_add(38).cast::<i16>();
                        (__p5).write(((__p5).read()).wrapping_add(1));
                    } else {
                        let __p6 = (sprite).wrapping_add(38).cast::<i16>();
                        (__p6).write(((__p6).read()).wrapping_sub(1));
                    }
                }
            } else {
                m4aSongNumStart(43u16);
                StartSpriteAnim(
                    sprite,
                    (((((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(56)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(46))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(4i32)) as u8),
                );
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_Taillow_FlyAway));
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(44),
                    7,
                    1,
                    (0u8) as i32,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Taillow_FlyIn(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut xMoveOffsets = crate::ffi::Align4([0u8; 2]);
        (&raw mut xMoveOffsets)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i8>()
            .write((-1i8));
        (&raw mut xMoveOffsets)
            .cast::<u8>()
            .wrapping_add(1)
            .cast::<i8>()
            .write(1i8);
        let mut yMoveOffsets = crate::ffi::Align4([0u8; 16]);
        (&raw mut yMoveOffsets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(0)
            .cast::<i8>()
            .write(2i8);
        (&raw mut yMoveOffsets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(1)
            .cast::<i8>()
            .write(0i8);
        (&raw mut yMoveOffsets)
            .cast::<u8>()
            .wrapping_add(2)
            .wrapping_add(0)
            .cast::<i8>()
            .write(2i8);
        (&raw mut yMoveOffsets)
            .cast::<u8>()
            .wrapping_add(2)
            .wrapping_add(1)
            .cast::<i8>()
            .write(0i8);
        (&raw mut yMoveOffsets)
            .cast::<u8>()
            .wrapping_add(4)
            .wrapping_add(0)
            .cast::<i8>()
            .write(2i8);
        (&raw mut yMoveOffsets)
            .cast::<u8>()
            .wrapping_add(4)
            .wrapping_add(1)
            .cast::<i8>()
            .write((-1i8));
        (&raw mut yMoveOffsets)
            .cast::<u8>()
            .wrapping_add(6)
            .wrapping_add(0)
            .cast::<i8>()
            .write(2i8);
        (&raw mut yMoveOffsets)
            .cast::<u8>()
            .wrapping_add(6)
            .wrapping_add(1)
            .cast::<i8>()
            .write((-1i8));
        (&raw mut yMoveOffsets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(0)
            .cast::<i8>()
            .write(2i8);
        (&raw mut yMoveOffsets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(1)
            .cast::<i8>()
            .write((-1i8));
        (&raw mut yMoveOffsets)
            .cast::<u8>()
            .wrapping_add(10)
            .wrapping_add(0)
            .cast::<i8>()
            .write(2i8);
        (&raw mut yMoveOffsets)
            .cast::<u8>()
            .wrapping_add(10)
            .wrapping_add(1)
            .cast::<i8>()
            .write((-1i8));
        (&raw mut yMoveOffsets)
            .cast::<u8>()
            .wrapping_add(12)
            .wrapping_add(0)
            .cast::<i8>()
            .write(2i8);
        (&raw mut yMoveOffsets)
            .cast::<u8>()
            .wrapping_add(12)
            .wrapping_add(1)
            .cast::<i8>()
            .write((-2i8));
        (&raw mut yMoveOffsets)
            .cast::<u8>()
            .wrapping_add(14)
            .wrapping_add(0)
            .cast::<i8>()
            .write(2i8);
        (&raw mut yMoveOffsets)
            .cast::<u8>()
            .wrapping_add(14)
            .wrapping_add(1)
            .cast::<i8>()
            .write((-2i8));
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            __t2
        }) as i32)
            > 7i32
        {
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    (((((&raw mut xMoveOffsets).cast::<i8>()).wrapping_offset(
                        (((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(56)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(46))
                        .cast::<i16>())
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        .wrapping_mul(2i32),
                )) as i16),
            );
            if (IsSEPlaying()) != 0 {
                let mut pan: i8 = (((crate::c::div_i32(
                    (116i32)
                        .wrapping_sub(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)),
                    2i32,
                ))
                .wrapping_neg()) as i8);
                m4aMPlayPanpotControl((&raw mut gMPlayInfo_SE1).cast::<u8>(), 65535u16, pan);
                m4aMPlayPanpotControl((&raw mut gMPlayInfo_SE2).cast::<u8>(), 65535u16, pan);
            }
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                >= 0i32
            {
                let __p4 = (sprite).wrapping_add(32).cast::<i16>();
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_add(
                        (((((&raw mut xMoveOffsets).cast::<i8>()).wrapping_offset(
                            (((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(56)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(46))
                            .cast::<i16>())
                            .read()) as i32) as isize,
                        ))
                        .read()) as i32)
                            .wrapping_mul(
                                ((((((&raw mut yMoveOffsets).cast::<u8>()).wrapping_offset(
                                    ((7i32).wrapping_sub(
                                        ((((((sprite).wrapping_add(46)).cast::<i16>())
                                            .wrapping_offset(1))
                                        .read()) as i32),
                                    )) as isize
                                        * 2,
                                ))
                                .cast::<i8>())
                                .read()) as i32),
                            ),
                    )) as i16),
                );
                let __p5 = (sprite).wrapping_add(34).cast::<i16>();
                (__p5).write(
                    (((((__p5).read()) as i32).wrapping_add(
                        (((((((&raw mut yMoveOffsets).cast::<u8>()).wrapping_offset(
                            ((7i32).wrapping_sub(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                    .read()) as i32),
                            )) as isize
                                * 2,
                        ))
                        .cast::<i8>())
                        .wrapping_offset(1))
                        .read()) as i32),
                    )) as i16),
                );
            } else {
                m4aSongNumStartOrChange(94u16);
                if (((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(56)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .read()) as i32)
                    == 0i32
                {
                    PlayCry_Normal(304u16, 63i8);
                } else {
                    PlayCry_Normal(304u16, (-63i8));
                }
                StartSpriteAnim(
                    sprite,
                    (((((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(56)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(46))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(2i32)) as u8),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(45i16);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_Taillow_PickUpBall));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TaillowShadow_FlyIn(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut moveDir = crate::ffi::Align4([0u8; 2]);
        (&raw mut moveDir)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i8>()
            .write((-1i8));
        (&raw mut moveDir)
            .cast::<u8>()
            .wrapping_add(1)
            .cast::<i8>()
            .write(1i8);
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            __t2
        }) as i32)
            >= 0i32
        {
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    (((((&raw mut moveDir).cast::<i8>()).wrapping_offset(
                        (((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(56)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(46))
                        .cast::<i16>())
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        .wrapping_mul(2i32),
                )) as i16),
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                ((((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    false,
                ) as u16) as i32)
                    ^ 1i32) as u16) as i32,
            );
        } else {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_TaillowShadow_Flash));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Taillow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(56)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(46))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            if ((((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(56)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as i32)
                == ((((((((&raw const sRouletteTables).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize
                            * 32,
                    ))
                .wrapping_add(16))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(90i32)
            {
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(52i16);
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(52i16);
            } else {
                return;
            }
        } else {
            if ((((((((((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(56)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as i32)
                == ((((((((&raw const sRouletteTables).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            (((&raw mut sRoulette).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize
                            * 32,
                    ))
                .wrapping_add(16))
                .wrapping_add(4)
                .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(270i32)
            {
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(46i16);
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(46i16);
            } else {
                return;
            }
        }
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_TaillowShadow_FlyIn));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Taillow_FlyIn));
        m4aSongNumStart(43u16);
    }
}
