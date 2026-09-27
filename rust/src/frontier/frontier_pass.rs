//! Translated from `src/frontier_pass.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sMaleHead_Pal sFemaleHead_Pal sMapScreen_Gfx sCursor_Gfx sHeads_Gfx sMapCursor_Gfx sMapScreen_Tilemap sMapAndCard_ZoomedOut_Tilemap sCardBall_Filled_Tilemap sBattleRecord_Tilemap sMapAndCard_Zooming_Tilemap sBgAffineCoords sPassBgTemplates sMapBgTemplates sPassWindowTemplates sMapWindowTemplates sTextColors sPassAreasLayout sCursorSpriteSheets sHeadsSpriteSheet sSpritePalettes sAnim_Frame1_Unused sAnim_Frame1 sAnim_Frame2 sAnim_Frame3 sAnim_Frame4 sAnim_Frame5 sAnim_Frame6 sAnim_Frame7 sAnim_MapIndicatorCursor_Rectangle sAnim_MapIndicatorCursor_Square sAnims_TwoFrame sAnims_Medal sAnims_MapIndicatorCursor sAffineAnim_Unused sAffineAnims_Unused sSpriteTemplates_Cursors sSpriteTemplate_Medal sSpriteTemplate_PlayerHead sPassAreaDescriptions sMapLandmarks
#[allow(unused_imports)]
use crate::data::frontier_pass::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPassData: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPassGfx: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMapData: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavedPassData: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);

unsafe extern "C" {
    static mut gFrontierPassBg_Gfx: u8;
    static mut gFrontierPassBg_Pal: u8;
    static mut gFrontierPassBg_Tilemap: u8;
    static mut gFrontierPassCancelButtonHighlighted_Tilemap: u8;
    static mut gFrontierPassCancelButton_Tilemap: u8;
    static mut gFrontierPassMapAndCard_Gfx: u8;
    static mut gMain: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSprites: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_BattlePoints: u8;
    static mut gText_BattleRecord: u8;
    static mut gText_SymbolsEarned: u8;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CanCopyRecordedBattleSaveData() -> u32;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CountPlayerTrainerStars() -> u32;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CurrentBattlePyramidLocation() -> u8;
    fn DeactivateAllTextPrinters();
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DisableInterrupts(a0: u16);
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetCurrentRegionMapSectionId() -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetTextWindowPalette(a0: u8) -> *mut u16;
    fn HideBg(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut u8);
    fn MathUtil_Inv16(a0: i16) -> i16;
    fn Overworld_PlaySpecialMapMusic();
    fn PlayBGM(a0: u16);
    fn PlayRecordedBattle(a0: Option<unsafe extern "C" fn()>);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ResetAffineAnimData();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn RunTasks();
    fn ScanlineEffect_Stop();
    fn SetBgAffine(a0: u8, a1: i32, a2: i32, a3: i16, a4: i16, a5: i16, a6: i16, a7: u16);
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankHBlankCallbacksToNull();
    fn ShowBg(a0: u8);
    fn ShowPlayerTrainerCard(a0: Option<unsafe extern "C" fn()>);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn TransferPlttBuffer();
    fn UnsetBgTilemapBuffer(a0: u8);
    fn UpdatePaletteFade() -> u8;
    fn malloc_and_decompress(a0: *mut u8, a1: *mut u32) -> *mut u8;
}

pub(crate) unsafe extern "C" fn ResetGpuRegsAndBgs() {
    unsafe {
        SetGpuReg(0u8, 0u16);
        SetGpuReg(14u8, 0u16);
        SetGpuReg(12u8, 0u16);
        SetGpuReg(10u8, 0u16);
        SetGpuReg(8u8, 0u16);
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        ChangeBgX(1u8, 0i32, 0u8);
        ChangeBgY(1u8, 0i32, 0u8);
        ChangeBgX(2u8, 0i32, 0u8);
        ChangeBgY(2u8, 0i32, 0u8);
        ChangeBgX(3u8, 0i32, 0u8);
        ChangeBgY(3u8, 0i32, 0u8);
        SetGpuReg(80u8, 0u16);
        SetGpuReg(84u8, 0u16);
        SetGpuReg(82u8, 0u16);
        SetGpuReg(64u8, 0u16);
        SetGpuReg(68u8, 0u16);
        SetGpuReg(66u8, 0u16);
        SetGpuReg(70u8, 0u16);
        SetGpuReg(72u8, 0u16);
        SetGpuReg(74u8, 0u16);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((100663296i32) as usize as *mut u8),
                                ((16777216i32
                                    | (crate::c::div_i32(98304i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
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
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((117440512i32) as usize as *mut u8),
                                ((83886080i32
                                    | (crate::c::div_i32(1024i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
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
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowFrontierPass(callback: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut callback = callback;
        AllocateFrontierPassData(callback);
        SetMainCallback2(Some(CB2_InitFrontierPass));
    }
}
pub(crate) unsafe extern "C" fn LeaveFrontierPass() {
    unsafe {
        SetMainCallback2(
            ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        );
        FreeFrontierPassData();
    }
}
pub(crate) unsafe extern "C" fn AllocateFrontierPassData(
    callback: Option<unsafe extern "C" fn()>,
) -> u32 {
    unsafe {
        let mut callback = callback;
        let mut i: u8 = 0u8;
        if ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize {
            return 1u32;
        }
        ((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(24u32));
        if ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize {
            return 2u32;
        }
        ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(callback);
        i = GetCurrentRegionMapSectionId();
        if (((i) as i32) != 58i32) && (((i) as i32) != 202i32) {
            ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<i16>())
            .write(176i16);
            ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<i16>())
            .write(104i16);
        } else {
            ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<i16>())
            .write(176i16);
            ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<i16>())
            .write(48i16);
        }
        ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6)
            .cast::<u16>())
        .write(
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2156)
                .cast::<u16>())
            .read(),
        );
        crate::c::bf_write(
            (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14),
            0,
            1,
            ((CanCopyRecordedBattleSaveData()) as u8) as i32,
        );
        crate::c::bf_write(
            (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14),
            1,
            3,
            (0u8) as i32,
        );
        crate::c::bf_write(
            (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14),
            4,
            4,
            ((CountPlayerTrainerStars()) as u8) as i32,
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 7i32) {
                    break 'l1;
                }
                'l2: {
                    if (FlagGet((((2244i32).wrapping_add(((i) as i32).wrapping_mul(2i32))) as u16)))
                        != 0
                    {
                        let __p1 = (((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(15))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize);
                        (__p1).write(((__p1).read()).wrapping_add(1));
                    }
                    if (FlagGet((((2245i32).wrapping_add(((i) as i32).wrapping_mul(2i32))) as u16)))
                        != 0
                    {
                        let __p2 = (((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(15))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize);
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn FreeFrontierPassData() -> u32 {
    unsafe {
        if ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize {
            return 1u32;
        }
        crate::c::memset(
            ((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read(),
            0i32,
            24u32,
        );
        {
            Free(((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn AllocateFrontierPassGfx() -> u32 {
    unsafe {
        if ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize {
            return 1u32;
        }
        ((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(9268u32));
        if ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize {
            return 2u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn FreeFrontierPassGfx() -> u32 {
    unsafe {
        FreeAllWindowBuffers();
        if ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize {
            return 1u32;
        }
        if ((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(40)
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            Free(
                ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(40)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(40)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        if ((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36)
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            Free(
                ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        if ((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            Free(
                ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        crate::c::memset(
            ((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read(),
            0i32,
            9268u32,
        );
        {
            Free(((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_FrontierPass() {
    unsafe {
        if (((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44)).read())
            != 0
        {
            SetBgAffine(
                2u8,
                ((((((((&raw const sBgAffineCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((crate::c::bf_read(
                            (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(14),
                            1,
                            3,
                            false,
                        ) as u8) as i32)
                            .wrapping_sub(1i32)) as isize
                            * 4,
                    ))
                .cast::<i16>())
                .read()) as i32)
                    << 8),
                (((((((((&raw const sBgAffineCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((crate::c::bf_read(
                            (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(14),
                            1,
                            3,
                            false,
                        ) as u8) as i32)
                            .wrapping_sub(1i32)) as isize
                            * 4,
                    ))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    << 8),
                (((((&raw const sBgAffineCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((crate::c::bf_read(
                            (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(14),
                            1,
                            3,
                            false,
                        ) as u8) as i32)
                            .wrapping_sub(1i32)) as isize
                            * 4,
                    ))
                .cast::<i16>())
                .read(),
                ((((((&raw const sBgAffineCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((crate::c::bf_read(
                            (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(14),
                            1,
                            3,
                            false,
                        ) as u8) as i32)
                            .wrapping_sub(1i32)) as isize
                            * 4,
                    ))
                .cast::<i16>())
                .wrapping_offset(1))
                .read(),
                ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(46)
                    .cast::<i16>())
                .read(),
                ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(48)
                    .cast::<i16>())
                .read(),
                0u16,
            );
        }
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn CB2_FrontierPass() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
    }
}
pub(crate) unsafe extern "C" fn CB2_InitFrontierPass() {
    unsafe {
        if (InitFrontierPass()) != 0 {
            CreateTask(Some(Task_HandleFrontierPassInput), 0u8);
            SetMainCallback2(Some(CB2_FrontierPass));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_HideFrontierPass() {
    unsafe {
        if (HideFrontierPass()) != 0 {
            LeaveFrontierPass();
        }
    }
}
pub(crate) unsafe extern "C" fn InitFrontierPass() -> u32 {
    unsafe {
        let mut sizeOut: u32 = 0u32;
        'l1: {
            let __sw1 = ((((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                SetVBlankCallback(None);
                ScanlineEffect_Stop();
                SetVBlankHBlankCallbacksToNull();
                DisableInterrupts(2u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ResetGpuRegsAndBgs();
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetTasks();
                ResetSpriteData();
                FreeAllSpritePalettes();
                ResetPaletteFade();
                ResetTempTileDataBuffers();
                break 'l1;
            }
            if __sw1 == 3i32 {
                AllocateFrontierPassGfx();
                break 'l1;
            }
            if __sw1 == 4i32 {
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    1u8,
                    ((&raw const sPassBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(12u32, 4u32)) as u8),
                );
                SetBgTilemapBuffer(
                    1u8,
                    ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(50))
                    .cast::<u8>(),
                );
                SetBgTilemapBuffer(
                    2u8,
                    ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4146))
                    .cast::<u8>(),
                );
                SetBgTilemapBuffer(
                    3u8,
                    ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8242))
                    .cast::<u8>(),
                );
                SetBgAttribute(2u8, 6u8, 1u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                InitWindows(
                    ((&raw const sPassWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                DeactivateAllTextPrinters();
                break 'l1;
            }
            if __sw1 == 6i32 {
                ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<*mut u8>())
                .write(malloc_and_decompress(
                    (((&raw const sMapAndCard_Zooming_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    &raw mut sizeOut,
                ));
                ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36)
                    .cast::<*mut u8>())
                .write(malloc_and_decompress(
                    (((&raw const sMapAndCard_ZoomedOut_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    &raw mut sizeOut,
                ));
                ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(40)
                    .cast::<*mut u8>())
                .write(malloc_and_decompress(
                    (((&raw const sBattleRecord_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    &raw mut sizeOut,
                ));
                DecompressAndCopyTileDataToVram(
                    1u8,
                    (((&raw mut gFrontierPassBg_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                DecompressAndCopyTileDataToVram(
                    2u8,
                    (((&raw mut gFrontierPassMapAndCard_Gfx).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 0u32;
                }
                FillBgTilemapBufferRect_Palette0(
                    0u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                FillBgTilemapBufferRect_Palette0(
                    1u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                FillBgTilemapBufferRect_Palette0(
                    2u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                CopyBgTilemapBufferToVram(0u8);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(2u8);
                break 'l1;
            }
            if __sw1 == 8i32 {
                LoadPalette((&raw mut gFrontierPassBg_Pal).cast::<u8>(), 0u16, 416u16);
                LoadPalette(
                    ((((&raw mut gFrontierPassBg_Pal).cast::<u8>()).wrapping_offset(
                        ((1i32).wrapping_add(
                            ((crate::c::bf_read(
                                (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(14),
                                4,
                                4,
                                false,
                            ) as u8) as i32),
                        )) as isize
                            * 32,
                    ))
                    .cast::<u16>())
                    .cast::<u8>(),
                    16u16,
                    32u16,
                );
                LoadPalette((GetTextWindowPalette(0u8)).cast::<u8>(), 240u16, 32u16);
                DrawFrontierPassBg();
                UpdateAreaHighlight(
                    ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .read(),
                    ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(13))
                    .read(),
                );
                if (((crate::c::bf_read(
                    (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14),
                    1,
                    3,
                    false,
                ) as u8) as i32)
                    == 1i32)
                    || (((crate::c::bf_read(
                        (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(14),
                        1,
                        3,
                        false,
                    ) as u8) as i32)
                        == 2i32)
                {
                    ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<u16>())
                    .write(0u16);
                    return 1u32;
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                SetGpuReg(0u8, 4160u16);
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                LoadCursorAndSymbolSprites();
                SetVBlankCallback(Some(VBlankCB_FrontierPass));
                BlendPalettes(4294967295u32, 16u8, 0u16);
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                break 'l1;
            }
            if __sw1 == 10i32 {
                AnimateSprites();
                BuildOamBuffer();
                if (UpdatePaletteFade()) != 0 {
                    return 0u32;
                }
                ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>())
                .write(0u16);
                return 1u32;
            }
        }
        let __p2 = (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn HideFrontierPass() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if (((crate::c::bf_read(
                    (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14),
                    1,
                    3,
                    false,
                ) as u8) as i32)
                    != 1i32)
                    && (((crate::c::bf_read(
                        (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(14),
                        1,
                        3,
                        false,
                    ) as u8) as i32)
                        != 2i32)
                {
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                } else {
                    ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<u16>())
                    .write(2u16);
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (UpdatePaletteFade()) != 0 {
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetGpuReg(0u8, 0u16);
                HideBg(0u8);
                HideBg(1u8);
                HideBg(2u8);
                SetVBlankCallback(None);
                ScanlineEffect_Stop();
                SetVBlankHBlankCallbacksToNull();
                break 'l1;
            }
            if __sw1 == 3i32 {
                FreeCursorAndSymbolSprites();
                break 'l1;
            }
            if __sw1 == 4i32 {
                ResetGpuRegsAndBgs();
                ResetTasks();
                ResetSpriteData();
                FreeAllSpritePalettes();
                break 'l1;
            }
            if __sw1 == 5i32 {
                UnsetBgTilemapBuffer(0u8);
                UnsetBgTilemapBuffer(1u8);
                UnsetBgTilemapBuffer(2u8);
                FreeFrontierPassGfx();
                ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>())
                .write(0u16);
                return 1u32;
            }
        }
        let __p2 = (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn GetCursorAreaFromCoords(x: i16, y: i16) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 13i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((((&raw const sPassAreasLayout).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .cast::<i16>())
                    .read()) as i32)
                        <= ((y) as i32))
                        && ((((((((&raw const sPassAreasLayout).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(2)
                        .cast::<i16>())
                        .read()) as i32)
                            >= ((y) as i32)))
                        && ((((((((&raw const sPassAreasLayout).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(4)
                        .cast::<i16>())
                        .read()) as i32)
                            <= ((x) as i32)))
                        && ((((((((&raw const sPassAreasLayout).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(6)
                        .cast::<i16>())
                        .read()) as i32)
                            >= ((x) as i32))
                    {
                        if (((i) as i32) >= 6i32)
                            && (((((((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(15))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((i) as i32).wrapping_sub(7i32)).wrapping_add(1i32)) as isize,
                            ))
                            .read()) as i32)
                                == 0i32)
                        {
                            break 'l1;
                        }
                        return ((((i) as i32).wrapping_add(1i32)) as u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReshowFrontierPass() {
    unsafe {
        let mut taskId: u8 = 0u8;
        if !((InitFrontierPass()) != 0) {
            return;
        }
        'l1: {
            let __sw1 = ((crate::c::bf_read(
                (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14),
                1,
                3,
                false,
            ) as u8) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 1i32 || __sw1 == 2i32 {
                taskId = CreateTask(Some(Task_PassAreaZoom), 0u8);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(1i16);
                break 'l1;
            }
            if __sw1 == 3i32 || !__matched {
                crate::c::bf_write(
                    (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14),
                    1,
                    3,
                    (0u8) as i32,
                );
                taskId = CreateTask(Some(Task_HandleFrontierPassInput), 0u8);
                break 'l1;
            }
        }
        SetMainCallback2(Some(CB2_FrontierPass));
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnFromRecord() {
    unsafe {
        AllocateFrontierPassData(
            (((&raw mut sSavedPassData).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                .read(),
        );
        ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<i16>())
        .write(
            (((&raw mut sSavedPassData).cast::<u8>())
                .wrapping_add(4)
                .cast::<i16>())
            .read(),
        );
        ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(10)
            .cast::<i16>())
        .write(
            (((&raw mut sSavedPassData).cast::<u8>())
                .wrapping_add(6)
                .cast::<i16>())
            .read(),
        );
        crate::c::memset((&raw mut sSavedPassData).cast::<u8>(), 0i32, 8u32);
        'l1: {
            let __sw1 = ((CurrentBattlePyramidLocation()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 1i32 {
                PlayBGM(461u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                PlayBGM(462u16);
                break 'l1;
            }
            if !__matched {
                Overworld_PlaySpecialMapMusic();
                break 'l1;
            }
        }
        SetMainCallback2(Some(CB2_ReshowFrontierPass));
    }
}
pub(crate) unsafe extern "C" fn CB2_ShowFrontierPassFeature() {
    unsafe {
        if !((HideFrontierPass()) != 0) {
            return;
        }
        'l1: {
            let __sw1 = ((crate::c::bf_read(
                (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14),
                1,
                3,
                false,
            ) as u8) as i32);
            if __sw1 == 1i32 {
                ShowFrontierMap(Some(CB2_ReshowFrontierPass));
                break 'l1;
            }
            if __sw1 == 3i32 {
                (((&raw mut sSavedPassData).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                    .write(
                        ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                    );
                (((&raw mut sSavedPassData).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i16>())
                .write(
                    ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<i16>())
                    .read(),
                );
                (((&raw mut sSavedPassData).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<i16>())
                .write(
                    ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<i16>())
                    .read(),
                );
                FreeFrontierPassData();
                PlayRecordedBattle(Some(CB2_ReturnFromRecord));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ShowPlayerTrainerCard(Some(CB2_ReshowFrontierPass));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryCallPassAreaFunction(taskId: u8, cursorArea: u8) -> u32 {
    unsafe {
        let mut taskId = taskId;
        let mut cursorArea = cursorArea;
        'l1: {
            let __sw1 = ((cursorArea) as i32);
            let __matched = __sw1 == 3i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 3i32 {
                if !((crate::c::bf_read(
                    (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14),
                    0,
                    1,
                    false,
                ) as u8)
                    != 0)
                {
                    return 0u32;
                }
                crate::c::bf_write(
                    (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14),
                    1,
                    3,
                    (3u8) as i32,
                );
                DestroyTask(taskId);
                SetMainCallback2(Some(CB2_ShowFrontierPassFeature));
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 2i32 {
                crate::c::bf_write(
                    (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14),
                    1,
                    3,
                    (cursorArea) as i32,
                );
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_PassAreaZoom));
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                break 'l1;
            }
            if !__matched {
                return 0u32;
            }
        }
        ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<i16>())
        .write(
            ((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_add(32)
            .cast::<i16>())
            .read(),
        );
        ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(10)
            .cast::<i16>())
        .write(
            ((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_add(34)
            .cast::<i16>())
            .read(),
        );
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Task_HandleFrontierPassInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut var: u8 = 0u8;
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0)
            && (((((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                >= 9i32)
        {
            let __p1 = (((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(2i32)) as i16));
            if ((((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                <= 7i32
            {
                ((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(34)
                .cast::<i16>())
                .write(2i16);
            }
            var = 1u8;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0)
            && (((((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                <= 135i32)
        {
            let __p2 = (((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(2i32)) as i16));
            if ((((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                >= 137i32
            {
                ((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(34)
                .cast::<i16>())
                .write(136i16);
            }
            var = 1u8;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0)
            && (((((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .read()) as i32)
                >= 6i32)
        {
            let __p3 = (((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_sub(2i32)) as i16));
            if ((((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .read()) as i32)
                <= 4i32
            {
                ((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32)
                .cast::<i16>())
                .write(5i16);
            }
            var = 1u8;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0)
            && (((((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .read()) as i32)
                <= 231i32)
        {
            let __p4 = (((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>();
            (__p4).write((((((__p4).read()) as i32).wrapping_add(2i32)) as i16));
            if ((((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .read()) as i32)
                >= 233i32
            {
                ((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32)
                .cast::<i16>())
                .write(232i16);
            }
            var = 1u8;
        }
        if !((var) != 0) {
            if (((((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
                .read()) as i32)
                != 0i32)
                && (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0)
            {
                if ((((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12))
                .read()) as i32)
                    <= 3i32
                {
                    PlaySE(5u16);
                    if (TryCallPassAreaFunction(
                        taskId,
                        ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .read(),
                    )) != 0
                    {
                        return;
                    }
                } else {
                    if ((((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .read()) as i32)
                        == 4i32
                    {
                        PlaySE(3u16);
                        SetMainCallback2(Some(CB2_HideFrontierPass));
                        DestroyTask(taskId);
                    }
                }
            }
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                PlaySE(3u16);
                SetMainCallback2(Some(CB2_HideFrontierPass));
                DestroyTask(taskId);
            }
        } else {
            var = GetCursorAreaFromCoords(
                ((((((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_sub(5i32)) as i16),
                ((((((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(34)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(5i32)) as i16),
            );
            if ((((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
                .read()) as i32)
                != ((var) as i32)
            {
                PrintAreaDescription(var);
                ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13))
                    .write(
                        ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .read(),
                    );
                ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
                    .write(var);
                UpdateAreaHighlight(
                    ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .read(),
                    ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(13))
                    .read(),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PassAreaZoom(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = ((((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if !(((data).read()) != 0) {
                    ShowHideZoomingArea(1u8, 0u8);
                    ((data).wrapping_offset(1)).write(256i16);
                    ((data).wrapping_offset(2)).write(256i16);
                    ((data).wrapping_offset(3)).write(21i16);
                    ((data).wrapping_offset(4)).write(21i16);
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 32767u16);
                } else {
                    ((data).wrapping_offset(1))
                        .write(((((1.984375f32) as f32) * ((256i32) as f32)) as i16));
                    ((data).wrapping_offset(2))
                        .write(((((1.984375f32) as f32) * ((256i32) as f32)) as i16));
                    ((data).wrapping_offset(3)).write((-21i16));
                    ((data).wrapping_offset(4)).write((-21i16));
                    SetGpuReg(0u8, 4160u16);
                    ShowBg(0u8);
                    ShowBg(1u8);
                    ShowBg(2u8);
                    LoadCursorAndSymbolSprites();
                    SetVBlankCallback(Some(VBlankCB_FrontierPass));
                    BlendPalettes(4294967295u32, 16u8, 32767u16);
                    BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 32767u16);
                }
                ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
                    .write(1u8);
                ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(46)
                    .cast::<i16>())
                .write(MathUtil_Inv16(((data).wrapping_offset(1)).read()));
                ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(48)
                    .cast::<i16>())
                .write(MathUtil_Inv16(((data).wrapping_offset(2)).read()));
                break 'l1;
            }
            if __sw1 == 1i32 {
                UpdatePaletteFade();
                let __p2 = (data).wrapping_offset(1);
                (__p2).write(
                    (((((__p2).read()) as i32)
                        .wrapping_add(((((data).wrapping_offset(3)).read()) as i32)))
                        as i16),
                );
                let __p3 = (data).wrapping_offset(2);
                (__p3).write(
                    (((((__p3).read()) as i32)
                        .wrapping_add(((((data).wrapping_offset(4)).read()) as i32)))
                        as i16),
                );
                ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(46)
                    .cast::<i16>())
                .write(MathUtil_Inv16(((data).wrapping_offset(1)).read()));
                ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(48)
                    .cast::<i16>())
                .write(MathUtil_Inv16(((data).wrapping_offset(2)).read()));
                if !(((data).read()) != 0) {
                    if ((((data).wrapping_offset(1)).read()) as i32)
                        <= (((((1.984375f32) as f32) * ((256i32) as f32)) as i16) as i32)
                    {
                        return;
                    }
                } else {
                    if ((((data).wrapping_offset(1)).read()) as i32) != 256i32 {
                        return;
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(44))
                .read())
                    != 0
                {
                    ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(44))
                    .write(0u8);
                }
                if (UpdatePaletteFade()) != 0 {
                    return;
                }
                if !(((data).read()) != 0) {
                    DestroyTask(taskId);
                    SetMainCallback2(Some(CB2_ShowFrontierPassFeature));
                } else {
                    ShowHideZoomingArea(0u8, 0u8);
                    crate::c::bf_write(
                        (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(14),
                        1,
                        3,
                        (0u8) as i32,
                    );
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_HandleFrontierPassInput));
                }
                SetBgAttribute(2u8, 6u8, 0u8);
                ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>())
                .write(0u16);
                return;
            }
        }
        let __p4 = (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>();
        (__p4).write(((__p4).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn ShowAndPrintWindows() {
    unsafe {
        let mut x: i32 = 0i32;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    PutWindowTilemap(i);
                    FillWindowPixelBuffer(i, 0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        x = GetStringCenterAlignXOffset(1i32, (&raw mut gText_SymbolsEarned).cast::<u8>(), 96i32);
        AddTextPrinterParameterized3(
            0u8,
            1u8,
            ((x) as u8),
            5u8,
            (((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>(),
            0i8,
            (&raw mut gText_SymbolsEarned).cast::<u8>(),
        );
        x = GetStringCenterAlignXOffset(1i32, (&raw mut gText_BattleRecord).cast::<u8>(), 96i32);
        AddTextPrinterParameterized3(
            1u8,
            1u8,
            ((x) as u8),
            5u8,
            (((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>(),
            0i8,
            (&raw mut gText_BattleRecord).cast::<u8>(),
        );
        AddTextPrinterParameterized3(
            2u8,
            8u8,
            5u8,
            4u8,
            (((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>(),
            0i8,
            (&raw mut gText_BattlePoints).cast::<u8>(),
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar4).cast::<u8>(),
            ((((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<u16>())
            .read()) as i32),
            0i32,
            5u8,
        );
        x = GetStringRightAlignXOffset(8i32, (&raw mut gStringVar4).cast::<u8>(), 91i32);
        AddTextPrinterParameterized3(
            2u8,
            8u8,
            ((x) as u8),
            16u8,
            (((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>(),
            0i8,
            (&raw mut gStringVar4).cast::<u8>(),
        );
        ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12)).write(
            GetCursorAreaFromCoords(
                ((((((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_sub(5i32)) as i16),
                ((((((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(5i32)) as i16),
            ),
        );
        ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13))
            .write(0u8);
        PrintAreaDescription(
            ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
                .read(),
        );
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l3;
                }
                'l4: {
                    CopyWindowToVram(i, 3u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyBgTilemapBufferToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn PrintAreaDescription(cursorArea: u8) {
    unsafe {
        let mut cursorArea = cursorArea;
        FillWindowPixelBuffer(3u8, 0u8);
        if (((cursorArea) as i32) == 3i32)
            && (!((crate::c::bf_read(
                (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14),
                0,
                1,
                false,
            ) as u8)
                != 0))
        {
            AddTextPrinterParameterized3(
                3u8,
                1u8,
                2u8,
                0u8,
                ((((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(3))
                .cast::<u8>(),
                0i8,
                (((&raw const sPassAreaDescriptions)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .read(),
            );
        } else {
            if ((cursorArea) as i32) != 0i32 {
                AddTextPrinterParameterized3(
                    3u8,
                    1u8,
                    2u8,
                    0u8,
                    ((((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(3))
                    .cast::<u8>(),
                    0i8,
                    ((((&raw const sPassAreaDescriptions)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((cursorArea) as i32) as isize))
                    .read(),
                );
            }
        }
        CopyWindowToVram(3u8, 3u8);
        CopyBgTilemapBufferToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn ShowHideZoomingArea(show: u8, zoomedIn: u8) {
    unsafe {
        let mut show = show;
        let mut zoomedIn = zoomedIn;
        'l1: {
            let __sw1 = ((crate::c::bf_read(
                (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14),
                1,
                3,
                false,
            ) as u8) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 1i32 {
                if (show) != 0 {
                    CopyToBgTilemapBufferRect_ChangePalette(
                        2u8,
                        ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(32)
                            .cast::<*mut u8>())
                        .read(),
                        16u8,
                        3u8,
                        12u8,
                        7u8,
                        16u8,
                    );
                } else {
                    FillBgTilemapBufferRect(2u8, 0u16, 16u8, 3u8, 12u8, 7u8, 16u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (show) != 0 {
                    CopyToBgTilemapBufferRect_ChangePalette(
                        2u8,
                        (((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(32)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(84),
                        16u8,
                        10u8,
                        12u8,
                        7u8,
                        16u8,
                    );
                } else {
                    FillBgTilemapBufferRect(2u8, 0u16, 16u8, 10u8, 12u8, 7u8, 16u8);
                }
                break 'l1;
            }
            if !__matched {
                return;
            }
        }
        CopyBgTilemapBufferToVram(2u8);
        if (zoomedIn) != 0 {
            SetBgAffine(
                2u8,
                ((((((((&raw const sBgAffineCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((crate::c::bf_read(
                            (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(14),
                            1,
                            3,
                            false,
                        ) as u8) as i32)
                            .wrapping_sub(1i32)) as isize
                            * 4,
                    ))
                .cast::<i16>())
                .read()) as i32)
                    << 8),
                (((((((((&raw const sBgAffineCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((crate::c::bf_read(
                            (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(14),
                            1,
                            3,
                            false,
                        ) as u8) as i32)
                            .wrapping_sub(1i32)) as isize
                            * 4,
                    ))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    << 8),
                (((((&raw const sBgAffineCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((crate::c::bf_read(
                            (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(14),
                            1,
                            3,
                            false,
                        ) as u8) as i32)
                            .wrapping_sub(1i32)) as isize
                            * 4,
                    ))
                .cast::<i16>())
                .read(),
                ((((((&raw const sBgAffineCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((crate::c::bf_read(
                            (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(14),
                            1,
                            3,
                            false,
                        ) as u8) as i32)
                            .wrapping_sub(1i32)) as isize
                            * 4,
                    ))
                .cast::<i16>())
                .wrapping_offset(1))
                .read(),
                MathUtil_Inv16(((((1.984375f32) as f32) * ((256i32) as f32)) as i16)),
                MathUtil_Inv16(((((1.984375f32) as f32) * ((256i32) as f32)) as i16)),
                0u16,
            );
        } else {
            SetBgAffine(
                2u8,
                ((((((((&raw const sBgAffineCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((crate::c::bf_read(
                            (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(14),
                            1,
                            3,
                            false,
                        ) as u8) as i32)
                            .wrapping_sub(1i32)) as isize
                            * 4,
                    ))
                .cast::<i16>())
                .read()) as i32)
                    << 8),
                (((((((((&raw const sBgAffineCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((crate::c::bf_read(
                            (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(14),
                            1,
                            3,
                            false,
                        ) as u8) as i32)
                            .wrapping_sub(1i32)) as isize
                            * 4,
                    ))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    << 8),
                (((((&raw const sBgAffineCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((crate::c::bf_read(
                            (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(14),
                            1,
                            3,
                            false,
                        ) as u8) as i32)
                            .wrapping_sub(1i32)) as isize
                            * 4,
                    ))
                .cast::<i16>())
                .read(),
                ((((((&raw const sBgAffineCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((crate::c::bf_read(
                            (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(14),
                            1,
                            3,
                            false,
                        ) as u8) as i32)
                            .wrapping_sub(1i32)) as isize
                            * 4,
                    ))
                .cast::<i16>())
                .wrapping_offset(1))
                .read(),
                MathUtil_Inv16(256i16),
                MathUtil_Inv16(256i16),
                0u16,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateAreaHighlight(cursorArea: u8, previousCursorArea: u8) {
    unsafe {
        let mut cursorArea = cursorArea;
        let mut previousCursorArea = previousCursorArea;
        'l1: {
            let __sw1 = ((previousCursorArea) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 1i32 {
                CopyToBgTilemapBufferRect_ChangePalette(
                    1u8,
                    ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36)
                        .cast::<*mut u8>())
                    .read(),
                    16u8,
                    3u8,
                    12u8,
                    7u8,
                    17u8,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                CopyToBgTilemapBufferRect_ChangePalette(
                    1u8,
                    (((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(336),
                    16u8,
                    10u8,
                    12u8,
                    7u8,
                    17u8,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (crate::c::bf_read(
                    (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14),
                    0,
                    1,
                    false,
                ) as u8)
                    != 0
                {
                    CopyToBgTilemapBufferRect_ChangePalette(
                        1u8,
                        ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40)
                            .cast::<*mut u8>())
                        .read(),
                        2u8,
                        10u8,
                        12u8,
                        3u8,
                        17u8,
                    );
                } else {
                    if (((cursorArea) as i32) == 0i32) || (((cursorArea) as i32) > 4i32) {
                        return;
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                CopyToBgTilemapBufferRect_ChangePalette(
                    1u8,
                    (((&raw mut gFrontierPassCancelButton_Tilemap).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    21u8,
                    0u8,
                    9u8,
                    2u8,
                    17u8,
                );
                break 'l1;
            }
            if !__matched {
                if (((cursorArea) as i32) == 0i32) || (((cursorArea) as i32) > 4i32) {
                    return;
                }
                break 'l1;
            }
        }
        'l2: {
            let __sw2 = ((cursorArea) as i32);
            let __matched = __sw2 == 1i32 || __sw2 == 2i32 || __sw2 == 3i32 || __sw2 == 4i32;
            if __sw2 == 1i32 {
                CopyToBgTilemapBufferRect_ChangePalette(
                    1u8,
                    (((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(168),
                    16u8,
                    3u8,
                    12u8,
                    7u8,
                    17u8,
                );
                break 'l2;
            }
            if __sw2 == 2i32 {
                CopyToBgTilemapBufferRect_ChangePalette(
                    1u8,
                    (((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(504),
                    16u8,
                    10u8,
                    12u8,
                    7u8,
                    17u8,
                );
                break 'l2;
            }
            if __sw2 == 3i32 {
                if (crate::c::bf_read(
                    (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14),
                    0,
                    1,
                    false,
                ) as u8)
                    != 0
                {
                    CopyToBgTilemapBufferRect_ChangePalette(
                        1u8,
                        (((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(72),
                        2u8,
                        10u8,
                        12u8,
                        3u8,
                        17u8,
                    );
                } else {
                    return;
                }
                break 'l2;
            }
            if __sw2 == 4i32 {
                CopyToBgTilemapBufferRect_ChangePalette(
                    1u8,
                    (((&raw mut gFrontierPassCancelButtonHighlighted_Tilemap).cast::<u32>())
                        .cast::<u32>())
                    .cast::<u8>(),
                    21u8,
                    0u8,
                    9u8,
                    2u8,
                    17u8,
                );
                break 'l2;
            }
            if !__matched {
                if (((previousCursorArea) as i32) == 0i32) || (((previousCursorArea) as i32) > 4i32)
                {
                    return;
                }
            }
        }
        CopyBgTilemapBufferToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn DrawFrontierPassBg() {
    unsafe {
        CopyToBgTilemapBuffer(
            1u8,
            (((&raw mut gFrontierPassBg_Tilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u16,
            0u16,
        );
        UpdateAreaHighlight(
            ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
                .read(),
            ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13))
                .read(),
        );
        ShowHideZoomingArea(
            1u8,
            (crate::c::bf_read(
                (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14),
                1,
                3,
                false,
            ) as u8),
        );
        ShowAndPrintWindows();
        CopyBgTilemapBufferToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn LoadCursorAndSymbolSprites() {
    unsafe {
        let mut spriteId: u8 = 0u8;
        let mut i: u8 = 0u8;
        FreeAllSpritePalettes();
        ResetAffineAnimData();
        LoadSpritePalettes(((&raw const sSpritePalettes).cast::<u8>().cast_mut()).cast::<u8>());
        LoadCompressedSpriteSheet(
            ((&raw const sCursorSpriteSheets).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        LoadCompressedSpriteSheet(
            (((&raw const sCursorSpriteSheets).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(16),
        );
        spriteId = CreateSprite(
            ((&raw const sSpriteTemplates_Cursors)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<i16>())
            .read(),
            ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<i16>())
            .read(),
            0u8,
        );
        ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>()).write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        crate::c::bf_write(
            (((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_add(5),
            2,
            2,
            (0u16) as i32,
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 7i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(15))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        let mut sprite = crate::ffi::Align4([0u8; 24]);
                        (&raw mut sprite)
                            .cast::<u8>()
                            .cast::<crate::c::Rec4<24>>()
                            .write_unaligned(
                                (&raw const sSpriteTemplate_Medal)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<crate::c::Rec4<24>>()
                                    .read_unaligned(),
                            );
                        let __p1 = ((&raw mut sprite).cast::<u8>())
                            .wrapping_add(2)
                            .cast::<u16>();
                        (__p1).write(
                            (((((__p1).read()) as i32).wrapping_add(
                                ((((((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(15))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    .wrapping_sub(1i32),
                            )) as u16),
                        );
                        spriteId = CreateSprite(
                            (&raw mut sprite).cast::<u8>(),
                            (((((((((&raw const sPassAreasLayout).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((((i) as i32).wrapping_add(7i32)).wrapping_sub(1i32)) as isize * 8,
                            ))
                            .wrapping_add(4)
                            .cast::<i16>())
                            .read()) as i32)
                                .wrapping_add(8i32)) as i16),
                            (((((((((&raw const sPassAreasLayout).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((((i) as i32).wrapping_add(7i32)).wrapping_sub(1i32)) as isize * 8,
                            ))
                            .cast::<i16>())
                            .read()) as i32)
                                .wrapping_add(6i32)) as i16),
                            ((((i) as i32).wrapping_add(1i32)) as u8),
                        );
                        ((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                        );
                        crate::c::bf_write(
                            (((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(5),
                            2,
                            2,
                            (2u16) as i32,
                        );
                        StartSpriteAnim(
                            ((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                            i,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FreeCursorAndSymbolSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        DestroySprite(
            ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read(),
        );
        ((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
            .write(core::ptr::null_mut());
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 7i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as usize)
                        != 0usize
                    {
                        DestroySprite(
                            ((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                        ((((((&raw mut sPassGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(core::ptr::null_mut());
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        FreeAllSpritePalettes();
        FreeSpriteTilesByTag(2u16);
        FreeSpriteTilesByTag(0u16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PlayerHead(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
    }
}
pub(crate) unsafe extern "C" fn ShowFrontierMap(callback: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut callback = callback;
        if ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize {
            SetMainCallback2(callback);
        }
        ((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(12308u32));
        ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(callback);
        ResetTasks();
        CreateTask(Some(Task_HandleFrontierMap), 0u8);
        SetMainCallback2(Some(CB2_FrontierPass));
    }
}
pub(crate) unsafe extern "C" fn FreeFrontierMap() {
    unsafe {
        ResetTasks();
        SetMainCallback2(
            ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        );
        crate::c::memset(
            ((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read(),
            0i32,
            12308u32,
        );
        {
            Free(((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn InitFrontierMap() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                SetVBlankCallback(None);
                ScanlineEffect_Stop();
                SetVBlankHBlankCallbacksToNull();
                break 'l1;
            }
            if __sw1 == 1i32 {
                ResetGpuRegsAndBgs();
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetSpriteData();
                FreeAllSpritePalettes();
                ResetPaletteFade();
                ResetTempTileDataBuffers();
                break 'l1;
            }
            if __sw1 == 3i32 {
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const sMapBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(12u32, 4u32)) as u8),
                );
                SetBgTilemapBuffer(
                    0u8,
                    ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18))
                    .cast::<u8>(),
                );
                SetBgTilemapBuffer(
                    1u8,
                    ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4114))
                    .cast::<u8>(),
                );
                SetBgTilemapBuffer(
                    2u8,
                    ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8210))
                    .cast::<u8>(),
                );
                FillBgTilemapBufferRect_Palette0(
                    0u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                FillBgTilemapBufferRect_Palette0(
                    1u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                FillBgTilemapBufferRect_Palette0(
                    2u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                CopyBgTilemapBufferToVram(0u8);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(2u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                InitWindows(
                    ((&raw const sMapWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                DeactivateAllTextPrinters();
                PrintOnFrontierMap();
                DecompressAndCopyTileDataToVram(
                    1u8,
                    (((&raw const sMapScreen_Gfx)
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
            if __sw1 == 5i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 0u32;
                }
                LoadPalette((&raw mut gFrontierPassBg_Pal).cast::<u8>(), 0u16, 416u16);
                LoadPalette((GetTextWindowPalette(0u8)).cast::<u8>(), 240u16, 32u16);
                CopyToBgTilemapBuffer(
                    2u8,
                    (((&raw const sMapScreen_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u16,
                    0u16,
                );
                CopyBgTilemapBufferToVram(2u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                SetGpuReg(0u8, 4160u16);
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                InitFrontierMapSprites();
                SetVBlankCallback(Some(VBlankCB_FrontierPass));
                BlendPalettes(4294967295u32, 16u8, 32767u16);
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 32767u16);
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (UpdatePaletteFade()) != 0 {
                    return 0u32;
                }
                ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>())
                .write(0u16);
                return 1u32;
            }
        }
        let __p2 = (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn ExitFrontierMap() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 32767u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (UpdatePaletteFade()) != 0 {
                    return 0u32;
                }
                SetGpuReg(0u8, 0u16);
                HideBg(0u8);
                HideBg(1u8);
                HideBg(2u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetVBlankCallback(None);
                ScanlineEffect_Stop();
                SetVBlankHBlankCallbacksToNull();
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read()) as usize)
                    != 0usize
                {
                    DestroySprite(
                        ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read(),
                    );
                    FreeSpriteTilesByTag(0u16);
                }
                if ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read()) as usize)
                    != 0usize
                {
                    DestroySprite(
                        ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12)
                            .cast::<*mut u8>())
                        .read(),
                    );
                    FreeSpriteTilesByTag(1u16);
                }
                if ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read()) as usize)
                    != 0usize
                {
                    DestroySprite(
                        ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read(),
                    );
                    FreeSpriteTilesByTag(4u16);
                }
                FreeAllWindowBuffers();
                break 'l1;
            }
            if __sw1 == 4i32 {
                ResetGpuRegsAndBgs();
                ResetSpriteData();
                FreeAllSpritePalettes();
                break 'l1;
            }
            if __sw1 == 5i32 {
                UnsetBgTilemapBuffer(0u8);
                UnsetBgTilemapBuffer(1u8);
                UnsetBgTilemapBuffer(2u8);
                ((((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>())
                .write(0u16);
                return 1u32;
            }
        }
        let __p2 = (((&raw mut sPassData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Task_HandleFrontierMap(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                if (InitFrontierMap()) != 0 {
                    break 'l1;
                }
                return;
            }
            if __sw1 == 1i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    PlaySE(3u16);
                    (data).write(4i16);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0
                    {
                        if ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16))
                        .read()) as i32)
                            >= 6i32
                        {
                            HandleFrontierMapCursorMove(0u8);
                        } else {
                            (data).write(2i16);
                        }
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 64i32)
                            != 0
                        {
                            if ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(16))
                            .read()) as i32)
                                == 0i32
                            {
                                HandleFrontierMapCursorMove(1u8);
                            } else {
                                (data).write(3i16);
                            }
                        }
                    }
                }
                return;
            }
            if __sw1 == 2i32 {
                if ((((data).wrapping_offset(1)).read()) as i32) > 3i32 {
                    HandleFrontierMapCursorMove(0u8);
                    ((data).wrapping_offset(1)).write(0i16);
                    (data).write(1i16);
                } else {
                    let __p2 = (((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(34)
                    .cast::<i16>();
                    (__p2).write((((((__p2).read()) as i32).wrapping_add(4i32)) as i16));
                    let __p3 = (data).wrapping_offset(1);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                return;
            }
            if __sw1 == 3i32 {
                if ((((data).wrapping_offset(1)).read()) as i32) > 3i32 {
                    HandleFrontierMapCursorMove(1u8);
                    ((data).wrapping_offset(1)).write(0i16);
                    (data).write(1i16);
                } else {
                    let __p4 = (((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(34)
                    .cast::<i16>();
                    (__p4).write((((((__p4).read()) as i32).wrapping_sub(4i32)) as i16));
                    let __p5 = (data).wrapping_offset(1);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                return;
            }
            if __sw1 == 4i32 {
                if (ExitFrontierMap()) != 0 {
                    break 'l1;
                }
                return;
            }
            if __sw1 == 5i32 {
                DestroyTask(taskId);
                FreeFrontierMap();
                return;
            }
        }
        (data).write(((data).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn MapNumToFrontierFacilityId(mapNum: u16) -> u8 {
    unsafe {
        let mut mapNum = mapNum;
        if ((((mapNum) as i32) >= 5i32) && (((mapNum) as i32) <= 8i32))
            || ((((mapNum) as i32) >= 15i32) && (((mapNum) as i32) <= 17i32))
        {
            return 1u8;
        } else {
            if (((((mapNum) as i32) == 18i32) || (((mapNum) as i32) == 19i32))
                || (((mapNum) as i32) == 20i32))
                || (((mapNum) as i32) == 21i32)
            {
                return 2u8;
            } else {
                if ((((mapNum) as i32) == 22i32) || (((mapNum) as i32) == 23i32))
                    || (((mapNum) as i32) == 24i32)
                {
                    return 3u8;
                } else {
                    if ((((mapNum) as i32) == 28i32) || (((mapNum) as i32) == 29i32))
                        || (((mapNum) as i32) == 30i32)
                    {
                        return 4u8;
                    } else {
                        if ((((mapNum) as i32) == 31i32) || (((mapNum) as i32) == 32i32))
                            || (((mapNum) as i32) == 33i32)
                        {
                            return 5u8;
                        } else {
                            if (((((((mapNum) as i32) == 34i32) || (((mapNum) as i32) == 35i32))
                                || (((mapNum) as i32) == 36i32))
                                || (((mapNum) as i32) == 37i32))
                                || (((mapNum) as i32) == 38i32))
                                || (((mapNum) as i32) == 39i32)
                            {
                                return 6u8;
                            } else {
                                if ((((mapNum) as i32) == 25i32) || (((mapNum) as i32) == 26i32))
                                    || (((mapNum) as i32) == 27i32)
                                {
                                    return 7u8;
                                } else {
                                    return 0u8;
                                }
                            }
                        }
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn InitFrontierMapSprites() {
    unsafe {
        let mut sprite = crate::ffi::Align4([0u8; 24]);
        let mut spriteId: u8 = 0u8;
        let mut id: u8 = 0u8;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        FreeAllSpritePalettes();
        LoadSpritePalettes(((&raw const sSpritePalettes).cast::<u8>().cast_mut()).cast::<u8>());
        LoadCompressedSpriteSheet(
            ((&raw const sCursorSpriteSheets).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        spriteId = CreateSprite(
            ((&raw const sSpriteTemplates_Cursors)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            155i16,
            (((((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16))
                .read()) as i32)
                .wrapping_mul(16i32))
            .wrapping_add(8i32)) as i16),
            2u8,
        );
        ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        crate::c::bf_write(
            (((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(5),
            2,
            2,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(63),
            0,
            1,
            (1u16) as i32,
        );
        StartSpriteAnim(
            ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read(),
            1u8,
        );
        LoadCompressedSpriteSheet(
            (((&raw const sCursorSpriteSheets).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(8),
        );
        spriteId = CreateSprite(
            (((&raw const sSpriteTemplates_Cursors)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(24),
            (((((&raw const sMapLandmarks).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .read()) as i32) as isize
                        * 16,
                ))
            .wrapping_add(8)
            .cast::<i16>())
            .read(),
            (((((&raw const sMapLandmarks).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .read()) as i32) as isize
                        * 16,
                ))
            .wrapping_add(10)
            .cast::<i16>())
            .read(),
            1u8,
        );
        ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        crate::c::bf_write(
            (((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(5),
            2,
            2,
            (0u16) as i32,
        );
        StartSpriteAnim(
            ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read(),
            (((((&raw const sMapLandmarks).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .read()) as i32) as isize
                        * 16,
                ))
            .wrapping_add(12))
            .read(),
        );
        id = GetCurrentRegionMapSectionId();
        if (((id) as i32) == 58i32) || (((id) as i32) == 202i32) {
            let mut mapNum: i8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(4))
            .wrapping_add(1)
            .cast::<i8>())
            .read();
            if (((mapNum) as i32) == 4i32)
                || ((((mapNum) as i32) == 14i32)
                    && (({
                        let __v1 = 55i16;
                        x = __v1;
                        __v1
                    }) != 0))
            {
                x = ((((x) as i32).wrapping_add(
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                        as i32),
                )) as i16);
                y = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .read();
                x = ((crate::c::div_i32(((x) as i32), 8i32)) as i16);
                y = ((crate::c::div_i32(((y) as i32), 8i32)) as i16);
                id = 0u8;
            } else {
                id = MapNumToFrontierFacilityId(((mapNum) as u16));
                if ((id) as i32) != 0i32 {
                    x = (((((&raw const sMapLandmarks).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset((((id) as i32).wrapping_sub(1i32)) as isize * 16))
                    .wrapping_add(8)
                    .cast::<i16>())
                    .read();
                    y = (((((&raw const sMapLandmarks).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset((((id) as i32).wrapping_sub(1i32)) as isize * 16))
                    .wrapping_add(10)
                    .cast::<i16>())
                    .read();
                } else {
                    if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(36))
                    .wrapping_add(1)
                    .cast::<i8>())
                    .read()) as i32)
                        == 14i32
                    {
                        x = (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(36))
                        .wrapping_add(4)
                        .cast::<i16>())
                        .read()) as i32)
                            .wrapping_add(55i32)) as i16);
                    } else {
                        x = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(36))
                        .wrapping_add(4)
                        .cast::<i16>())
                        .read();
                    }
                    y = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(36))
                        .wrapping_add(6)
                        .cast::<i16>())
                    .read();
                    x = ((crate::c::div_i32(((x) as i32), 8i32)) as i16);
                    y = ((crate::c::div_i32(((y) as i32), 8i32)) as i16);
                }
            }
            LoadCompressedSpriteSheet(
                ((&raw const sHeadsSpriteSheet).cast::<u8>().cast_mut()).cast::<u8>(),
            );
            (&raw mut sprite)
                .cast::<u8>()
                .cast::<crate::c::Rec4<24>>()
                .write_unaligned(
                    (&raw const sSpriteTemplate_PlayerHead)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<crate::c::Rec4<24>>()
                        .read_unaligned(),
                );
            (((&raw mut sprite).cast::<u8>()).wrapping_add(2).cast::<u16>()).write(((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read()) as i32))).wrapping_add(4i32)) as u16));
            if ((id) as i32) != 0i32 {
                spriteId = CreateSprite((&raw mut sprite).cast::<u8>(), x, y, 0u8);
            } else {
                x = ((((x) as i32).wrapping_mul(8i32)) as i16);
                y = ((((y) as i32).wrapping_mul(8i32)) as i16);
                spriteId = CreateSprite(
                    (&raw mut sprite).cast::<u8>(),
                    ((((x) as i32).wrapping_add(20i32)) as i16),
                    ((((y) as i32).wrapping_add(36i32)) as i16),
                    0u8,
                );
            }
            ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .write(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
            crate::c::bf_write(
                (((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(5),
                2,
                2,
                (0u16) as i32,
            );
            if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                as i32)
                != 0i32
            {
                StartSpriteAnim(
                    ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read(),
                    1u8,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintOnFrontierMap() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    PutWindowTilemap(i);
                    FillWindowPixelBuffer(i, 0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 7i32) {
                    break 'l3;
                }
                'l4: {
                    if ((i) as i32)
                        == ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16))
                        .read()) as i32)
                    {
                        AddTextPrinterParameterized3(
                            1u8,
                            7u8,
                            4u8,
                            (((((i) as i32).wrapping_mul(16i32)).wrapping_add(1i32)) as u8),
                            ((((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset(6))
                            .cast::<u8>(),
                            0i8,
                            (((((&raw const sMapLandmarks).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 16))
                            .cast::<*mut u8>())
                            .read(),
                        );
                    } else {
                        AddTextPrinterParameterized3(
                            1u8,
                            7u8,
                            4u8,
                            (((((i) as i32).wrapping_mul(16i32)).wrapping_add(1i32)) as u8),
                            ((((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset(3))
                            .cast::<u8>(),
                            0i8,
                            (((((&raw const sMapLandmarks).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 16))
                            .cast::<*mut u8>())
                            .read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        AddTextPrinterParameterized3(
            2u8,
            1u8,
            4u8,
            0u8,
            (((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>(),
            0i8,
            (((((&raw const sMapLandmarks).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .read()) as i32) as isize
                        * 16,
                ))
            .wrapping_add(4)
            .cast::<*mut u8>())
            .read(),
        );
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l5;
                }
                'l6: {
                    CopyWindowToVram(i, 3u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyBgTilemapBufferToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn HandleFrontierMapCursorMove(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut oldCursorPos: u8 = 0u8;
        let mut i: u8 = 0u8;
        if (direction) != 0 {
            oldCursorPos = ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .read();
            ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16)).write(
                ((crate::c::rem_i32(((oldCursorPos) as i32).wrapping_add(6i32), 7i32)) as u8),
            );
        } else {
            oldCursorPos = ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .read();
            ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16)).write(
                ((crate::c::rem_i32(((oldCursorPos) as i32).wrapping_add(1i32), 7i32)) as u8),
            );
        }
        AddTextPrinterParameterized3(
            1u8,
            7u8,
            4u8,
            (((((oldCursorPos) as i32).wrapping_mul(16i32)).wrapping_add(1i32)) as u8),
            ((((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(3))
                .cast::<u8>(),
            0i8,
            (((((&raw const sMapLandmarks).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((oldCursorPos) as i32) as isize * 16))
            .cast::<*mut u8>())
            .read(),
        );
        AddTextPrinterParameterized3(
            1u8,
            7u8,
            4u8,
            (((((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16))
                .read()) as i32)
                .wrapping_mul(16i32))
            .wrapping_add(1i32)) as u8),
            ((((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(6))
                .cast::<u8>(),
            0i8,
            (((((&raw const sMapLandmarks).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .read()) as i32) as isize
                        * 16,
                ))
            .cast::<*mut u8>())
            .read(),
        );
        ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(34)
        .cast::<i16>())
        .write(
            (((((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16))
                .read()) as i32)
                .wrapping_mul(16i32))
            .wrapping_add(8i32)) as i16),
        );
        StartSpriteAnim(
            ((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read(),
            (((((&raw const sMapLandmarks).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .read()) as i32) as isize
                        * 16,
                ))
            .wrapping_add(12))
            .read(),
        );
        ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(32)
        .cast::<i16>())
        .write(
            (((((&raw const sMapLandmarks).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .read()) as i32) as isize
                        * 16,
                ))
            .wrapping_add(8)
            .cast::<i16>())
            .read(),
        );
        ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(34)
        .cast::<i16>())
        .write(
            (((((&raw const sMapLandmarks).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .read()) as i32) as isize
                        * 16,
                ))
            .wrapping_add(10)
            .cast::<i16>())
            .read(),
        );
        FillWindowPixelBuffer(2u8, 0u8);
        AddTextPrinterParameterized3(
            2u8,
            1u8,
            4u8,
            0u8,
            (((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>(),
            0i8,
            (((((&raw const sMapLandmarks).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sMapData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .read()) as i32) as isize
                        * 16,
                ))
            .wrapping_add(4)
            .cast::<*mut u8>())
            .read(),
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    CopyWindowToVram(i, 3u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyBgTilemapBufferToVram(0u8);
        PlaySE(108u16);
    }
}
