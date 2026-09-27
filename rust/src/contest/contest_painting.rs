//! Translated from `src/contest_painting.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sPictureFramePalettes sPictureFrameTiles_Cool sPictureFrameTiles_Beauty sPictureFrameTiles_Cute sPictureFrameTiles_Smart sPictureFrameTiles_Tough sPictureFrameTiles_HallLobby sPictureFrameTilemap_Cool sPictureFrameTilemap_Beauty sPictureFrameTilemap_Cute sPictureFrameTilemap_Smart sPictureFrameTilemap_Tough sPictureFrameTilemap_HallLobby sContestCategoryNames_Unused sContestRankNames sBgTemplates sWindowTemplate sMuseumCaptions sContestPaintingMonOamData sBgPalette
#[allow(unused_imports)]
use crate::data::contest_painting::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gContestMonPixels: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gImageProcessingContext: crate::ffi::Align4<[u8; 32]> = crate::ffi::Align4([0; 32]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gContestPaintingWinner: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gContestPaintingMonPalette: *mut u16 = core::ptr::null_mut();
pub(crate) static mut sHoldState: u8 = 0u8;
pub(crate) static mut sMosaicVal: u16 = 0u16;
pub(crate) static mut sFadeCounter: u16 = 0u16;
pub(crate) static mut sVarsInitialized: u8 = 0u8;
pub(crate) static mut sWindowId: u8 = 0u8;

unsafe extern "C" {
    static mut gContestHallPaintingCaption: u8;
    static mut gCurContestWinner: u8;
    static mut gCurContestWinnerIsForArtist: u8;
    static mut gCurContestWinnerSaveIdx: u8;
    static mut gMain: u8;
    static mut gMonBackPicTable: u8;
    static mut gMonFrontPicTable: u8;
    static mut gMonSpritesGfxPtr: u8;
    static mut gPaletteFade: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gText_Space: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut u8) -> u16;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AllocateMonSpritesGfx();
    fn ApplyImageProcessingEffects(a0: *mut u8);
    fn ApplyImageProcessingQuantization(a0: *mut u8);
    fn BeginFastPaletteFade(a0: u8);
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BufferContestName(a0: *mut u8, a1: u8);
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ConvertImageProcessingToGBA(a0: *mut u8);
    fn ConvertInternationalContestantName(a0: *mut u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn DeactivateAllTextPrinters();
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeMonSpritesGfx();
    fn GetBgTilemapBuffer(a0: u8) -> *mut u8;
    fn GetMonSpritePalFromSpeciesAndPersonality(a0: u16, a1: u32, a2: u32) -> *mut u32;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn HandleLoadSpecialPokePic_DontHandleDeoxys(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32);
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitKeys();
    fn LZDecompressVram(a0: *mut u32, a1: *mut u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RLUnCompVram(a0: *mut u32, a1: *mut u8);
    fn RLUnCompWram(a0: *mut u32, a1: *mut u8);
    fn RemoveWindow(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn RunTextPrinters();
    fn ScanlineEffect_Stop();
    fn SeedRng(a0: u16);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetContestWinnerForPainting(contestWinnerId: i32) {
    unsafe {
        let mut contestWinnerId = contestWinnerId;
        let mut saveIdx: *mut u8 = (&raw mut gCurContestWinnerSaveIdx).cast::<u8>();
        let mut isForArtist: *mut u8 = (&raw mut gCurContestWinnerIsForArtist).cast::<u8>();
        (&raw mut gCurContestWinner)
            .cast::<u8>()
            .cast::<crate::c::Rec4<32>>()
            .write_unaligned(
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11920))
                    .cast::<u8>())
                .wrapping_offset(((contestWinnerId).wrapping_sub(1i32)) as isize * 32)
                .cast::<crate::c::Rec4<32>>()
                .read_unaligned(),
            );
        (saveIdx).write((((contestWinnerId).wrapping_sub(1i32)) as u8));
        (isForArtist).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ContestPainting() {
    unsafe {
        ShowContestPainting();
    }
}
pub(crate) unsafe extern "C" fn CB2_HoldContestPainting() {
    unsafe {
        HoldContestPainting();
        RunTextPrinters();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn CB2_QuitContestPainting() {
    unsafe {
        SetMainCallback2(
            (((&raw mut gMain).cast::<u8>())
                .wrapping_add(8)
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        );
        {
            Free(
                (((&raw mut gContestPaintingMonPalette)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
            );
            ((&raw mut gContestPaintingMonPalette)
                .cast::<u8>()
                .cast::<*mut u16>())
            .write(core::ptr::null_mut());
        }
        {
            Free(((&raw mut gContestMonPixels).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut gContestMonPixels).cast::<u8>().cast::<*mut u8>())
                .write(core::ptr::null_mut());
        }
        RemoveWindow(((&raw mut sWindowId).cast::<u8>().cast::<u8>()).read());
        Free(GetBgTilemapBuffer(1u8));
        FreeMonSpritesGfx();
    }
}
pub(crate) unsafe extern "C" fn ShowContestPainting() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                ScanlineEffect_Stop();
                SetVBlankCallback(None);
                AllocateMonSpritesGfx();
                ((&raw mut gContestPaintingWinner)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write((&raw mut gCurContestWinner).cast::<u8>());
                InitContestPaintingVars(1u8);
                InitContestPaintingBg();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ResetPaletteFade();
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
                                    let mut tmp: u32 = 0u32;
                                    (&raw mut tmp).write_volatile(0u32);
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
                                                    (((-2063597568i32)
                                                        | crate::c::div_i32(
                                                            4096i32,
                                                            crate::c::div_i32(32i32, 8i32),
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
                                        let mut tmp: u32 = 0u32;
                                        (&raw mut tmp).write_volatile(0u32);
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
                ResetSpriteData();
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                SeedRng(
                    (((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(32)
                        .cast::<u32>())
                    .read()) as u16),
                );
                InitKeys();
                InitContestPaintingWindow();
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                CreateContestPaintingPicture(
                    ((&raw mut gCurContestWinnerSaveIdx).cast::<u8>()).read(),
                    ((&raw mut gCurContestWinnerIsForArtist).cast::<u8>()).read(),
                );
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                PrintContestPaintingCaption(
                    ((&raw mut gCurContestWinnerSaveIdx).cast::<u8>()).read(),
                    ((&raw mut gCurContestWinnerIsForArtist).cast::<u8>()).read(),
                );
                SetBackdropFromPalette(
                    ((&raw const sBgPalette)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>(),
                );
                'l11: loop {
                    'l12: {
                        {
                            let mut _dest: *mut u32 = ((83886080i32) as usize as *mut u32);
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
                BeginFastPaletteFade(2u8);
                SetVBlankCallback(Some(VBlankCB_ContestPainting));
                ((&raw mut sHoldState).cast::<u8>().cast::<u8>()).write(0u8);
                SetGpuReg(0u8, 4928u16);
                SetMainCallback2(Some(CB2_HoldContestPainting));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HoldContestPainting() {
    unsafe {
        'l1: {
            let __sw1 = ((((&raw mut sHoldState).cast::<u8>().cast::<u8>()).read()) as i32);
            if __sw1 == 0i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ((&raw mut sHoldState).cast::<u8>().cast::<u8>()).write(1u8);
                }
                if ((((&raw mut sVarsInitialized).cast::<u8>().cast::<u8>()).read()) != 0)
                    && ((((&raw mut sFadeCounter).cast::<u8>().cast::<u16>()).read()) != 0)
                {
                    let __p2 = (&raw mut sFadeCounter).cast::<u8>().cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0)
                    || (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 2i32)
                        != 0)
                {
                    let __p3 = (&raw mut sHoldState).cast::<u8>().cast::<u8>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                }
                if (((&raw mut sVarsInitialized).cast::<u8>().cast::<u8>()).read()) != 0 {
                    ((&raw mut sFadeCounter).cast::<u8>().cast::<u16>()).write(0u16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    SetMainCallback2(Some(CB2_QuitContestPainting));
                }
                if ((((&raw mut sVarsInitialized).cast::<u8>().cast::<u8>()).read()) != 0)
                    && (((((&raw mut sFadeCounter).cast::<u8>().cast::<u16>()).read()) as i32)
                        < 30i32)
                {
                    let __p4 = (&raw mut sFadeCounter).cast::<u8>().cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitContestPaintingWindow() {
    unsafe {
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(4u32, 4u32)) as u8),
        );
        ChangeBgX(1u8, 0i32, 0u8);
        ChangeBgY(1u8, 0i32, 0u8);
        SetBgTilemapBuffer(1u8, AllocZeroed(2048u32));
        ((&raw mut sWindowId).cast::<u8>().cast::<u8>())
            .write(((AddWindow((&raw const sWindowTemplate).cast::<u8>().cast_mut())) as u8));
        DeactivateAllTextPrinters();
        FillWindowPixelBuffer(((&raw mut sWindowId).cast::<u8>().cast::<u8>()).read(), 0u8);
        PutWindowTilemap(((&raw mut sWindowId).cast::<u8>().cast::<u8>()).read());
        CopyWindowToVram(((&raw mut sWindowId).cast::<u8>().cast::<u8>()).read(), 3u8);
        ShowBg(1u8);
    }
}
pub(crate) unsafe extern "C" fn PrintContestPaintingCaption(contestType: u8, isForArtist: u8) {
    unsafe {
        let mut contestType = contestType;
        let mut isForArtist = isForArtist;
        let mut x: i32 = 0i32;
        let mut category: u8 = 0u8;
        if ((isForArtist) as i32) == 1i32 {
            return;
        }
        category = ((((&raw mut gContestPaintingWinner)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(10))
        .read();
        if ((contestType) as i32) < 8i32 {
            BufferContestName((&raw mut gStringVar1).cast::<u8>(), category);
            StringAppend(
                (&raw mut gStringVar1).cast::<u8>(),
                (&raw mut gText_Space).cast::<u8>(),
            );
            StringAppend(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((&raw const sContestRankNames)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(
                    ((((((&raw mut gContestPaintingWinner)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(30))
                    .read()) as i32) as isize,
                ))
                .read(),
            );
            StringCopy(
                (&raw mut gStringVar2).cast::<u8>(),
                ((((&raw mut gContestPaintingWinner)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(22))
                .cast::<u8>(),
            );
            ConvertInternationalContestantName((&raw mut gStringVar2).cast::<u8>());
            StringCopy(
                (&raw mut gStringVar3).cast::<u8>(),
                ((((&raw mut gContestPaintingWinner)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(11))
                .cast::<u8>(),
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gContestHallPaintingCaption).cast::<u8>(),
            );
        } else {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((&raw mut gContestPaintingWinner)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(11))
                .cast::<u8>(),
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                ((((&raw const sMuseumCaptions)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(((category) as i32) as isize))
                .read(),
            );
        }
        x = GetStringCenterAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 208i32);
        AddTextPrinterParameterized(
            ((&raw mut sWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            ((x) as u8),
            1u8,
            0u8,
            None,
        );
        CopyBgTilemapBufferToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn InitContestPaintingBg() {
    unsafe {
        SetGpuReg(0u8, 0u16);
        let __p1 = ((67109376i32) as usize as *mut u16);
        crate::c::volatile_write(__p1, (((((__p1).read_volatile()) as i32) | 1i32) as u16));
        SetGpuReg(8u8, 3138u16);
        SetGpuReg(10u8, 2629u16);
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        SetGpuReg(84u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn InitContestPaintingVars(reset: u8) {
    unsafe {
        let mut reset = reset;
        if ((reset) as i32) == 0i32 {
            ((&raw mut sVarsInitialized).cast::<u8>().cast::<u8>()).write(0u8);
            ((&raw mut sMosaicVal).cast::<u8>().cast::<u16>()).write(0u16);
            ((&raw mut sFadeCounter).cast::<u8>().cast::<u16>()).write(0u16);
        } else {
            ((&raw mut sVarsInitialized).cast::<u8>().cast::<u8>()).write(1u8);
            ((&raw mut sMosaicVal).cast::<u8>().cast::<u16>()).write(15u16);
            ((&raw mut sFadeCounter).cast::<u8>().cast::<u16>()).write(30u16);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateContestPaintingMosaicEffect() {
    unsafe {
        if !((((&raw mut sVarsInitialized).cast::<u8>().cast::<u8>()).read()) != 0) {
            SetGpuReg(76u8, 0u16);
        } else {
            SetGpuReg(10u8, 2629u16);
            ((&raw mut sMosaicVal).cast::<u8>().cast::<u16>()).write(
                ((crate::c::div_i32(
                    ((((&raw mut sFadeCounter).cast::<u8>().cast::<u16>()).read()) as i32),
                    2i32,
                )) as u16),
            );
            SetGpuReg(
                76u8,
                (((((((((&raw mut sMosaicVal).cast::<u8>().cast::<u16>()).read()) as i32) << 12)
                    | (((((&raw mut sMosaicVal).cast::<u8>().cast::<u16>()).read()) as i32) << 8))
                    | (((((&raw mut sMosaicVal).cast::<u8>().cast::<u16>()).read()) as i32) << 4))
                    | (((((&raw mut sMosaicVal).cast::<u8>().cast::<u16>()).read()) as i32) << 0))
                    as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_ContestPainting() {
    unsafe {
        UpdateContestPaintingMosaicEffect();
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn InitContestMonPixels(species: u16, backPic: u8) {
    unsafe {
        let mut species = species;
        let mut backPic = backPic;
        let mut pal: *mut u8 = (GetMonSpritePalFromSpeciesAndPersonality(
            species,
            ((((&raw mut gContestPaintingWinner)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4)
            .cast::<u32>())
            .read(),
            ((((&raw mut gContestPaintingWinner)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .cast::<u32>())
            .read(),
        ))
        .cast::<u8>();
        LZDecompressVram(
            (pal).cast::<u32>(),
            (((&raw mut gContestPaintingMonPalette)
                .cast::<u8>()
                .cast::<*mut u16>())
            .read())
            .cast::<u8>(),
        );
        if !((backPic) != 0) {
            HandleLoadSpecialPokePic_DontHandleDeoxys(
                ((&raw mut gMonFrontPicTable).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 8),
                ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<*mut u8>())
                .wrapping_offset(1))
                .read(),
                ((species) as i32),
                ((((&raw mut gContestPaintingWinner)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .cast::<u32>())
                .read(),
            );
            _InitContestMonPixels(
                ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<*mut u8>())
                .wrapping_offset(1))
                .read(),
                ((&raw mut gContestPaintingMonPalette)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .read(),
                ((&raw mut gContestMonPixels).cast::<u8>().cast::<*mut u8>()).read(),
            );
        } else {
            HandleLoadSpecialPokePic_DontHandleDeoxys(
                ((&raw mut gMonBackPicTable).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 8),
                (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<*mut u8>())
                .read(),
                ((species) as i32),
                ((((&raw mut gContestPaintingWinner)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .cast::<u32>())
                .read(),
            );
            _InitContestMonPixels(
                (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<*mut u8>())
                .read(),
                ((&raw mut gContestPaintingMonPalette)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .read(),
                ((&raw mut gContestMonPixels).cast::<u8>().cast::<*mut u8>()).read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn _InitContestMonPixels(
    spriteGfx: *mut u8,
    palette: *mut u16,
    destPixels: *mut u8,
) {
    unsafe {
        let mut spriteGfx = spriteGfx;
        let mut palette = palette;
        let mut destPixels = destPixels;
        let mut tileY: u16 = 0u16;
        let mut tileX: u16 = 0u16;
        let mut pixelY: u16 = 0u16;
        let mut pixelX: u16 = 0u16;
        let mut colorIndex: u8 = 0u8;
        {
            tileY = 0u16;
            'l1: loop {
                if !(((tileY) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        tileX = 0u16;
                        'l3: loop {
                            if !(((tileX) as i32) < 8i32) {
                                break 'l3;
                            }
                            'l4: {
                                {
                                    pixelY = 0u16;
                                    'l5: loop {
                                        if !(((pixelY) as i32) < 8i32) {
                                            break 'l5;
                                        }
                                        'l6: {
                                            {
                                                pixelX = 0u16;
                                                'l7: loop {
                                                    if !(((pixelX) as i32) < 8i32) {
                                                        break 'l7;
                                                    }
                                                    'l8: {
                                                        colorIndex =
                                                            ((spriteGfx).wrapping_offset(
                                                                ((((32i32).wrapping_mul(
                                                                    (((tileY) as i32)
                                                                        .wrapping_mul(8i32))
                                                                    .wrapping_add(((tileX) as i32)),
                                                                ))
                                                                .wrapping_add(
                                                                    (((pixelY) as i32) << 2),
                                                                ))
                                                                .wrapping_add(
                                                                    (((pixelX) as i32) >> 1),
                                                                ))
                                                                    as isize,
                                                            ))
                                                            .read();
                                                        if (((pixelX) as i32) & 1i32) != 0 {
                                                            colorIndex = ((((colorIndex) as i32)
                                                                >> 4)
                                                                as u8);
                                                        } else {
                                                            colorIndex = ((((colorIndex) as i32)
                                                                & 15i32)
                                                                as u8);
                                                        }
                                                        if ((colorIndex) as i32) == 0i32 {
                                                            (((((destPixels).cast::<u8>())
                                                                .wrapping_offset(
                                                                (((8i32).wrapping_mul(
                                                                    ((tileY) as i32),
                                                                ))
                                                                .wrapping_add(((pixelY) as i32)))
                                                                    as isize
                                                                    * 128,
                                                            ))
                                                            .cast::<u16>())
                                                            .wrapping_offset(
                                                                ((((tileX) as i32)
                                                                    .wrapping_mul(8i32))
                                                                .wrapping_add(((pixelX) as i32)))
                                                                    as isize,
                                                            ))
                                                            .write(32768u16);
                                                        } else {
                                                            (((((destPixels).cast::<u8>())
                                                                .wrapping_offset(
                                                                (((8i32).wrapping_mul(
                                                                    ((tileY) as i32),
                                                                ))
                                                                .wrapping_add(((pixelY) as i32)))
                                                                    as isize
                                                                    * 128,
                                                            ))
                                                            .cast::<u16>())
                                                            .wrapping_offset(
                                                                ((((tileX) as i32)
                                                                    .wrapping_mul(8i32))
                                                                .wrapping_add(((pixelX) as i32)))
                                                                    as isize,
                                                            ))
                                                            .write(
                                                                ((palette).wrapping_offset(
                                                                    ((colorIndex) as i32) as isize,
                                                                ))
                                                                .read(),
                                                            );
                                                        }
                                                    }
                                                    pixelX = (pixelX).wrapping_add(1);
                                                }
                                            }
                                        }
                                        pixelY = (pixelY).wrapping_add(1);
                                    }
                                }
                            }
                            tileX = (tileX).wrapping_add(1);
                        }
                    }
                }
                tileY = (tileY).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadContestPaintingFrame(contestWinnerId: u8, isForArtist: u8) {
    unsafe {
        let mut contestWinnerId = contestWinnerId;
        let mut isForArtist = isForArtist;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        LoadPalette(
            (((&raw const sPictureFramePalettes)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            0u16,
            256u16,
        );
        if ((isForArtist) as i32) == 1i32 {
            'l1: {
                let __sw1 = crate::c::div_i32(
                    ((((((&raw mut gContestPaintingWinner)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(10))
                    .read()) as i32),
                    3i32,
                );
                if __sw1 == 0i32 {
                    RLUnCompVram(
                        ((&raw const sPictureFrameTiles_Cool)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        ((100663296i32) as usize as *mut u8),
                    );
                    RLUnCompWram(
                        ((&raw const sPictureFrameTilemap_Cool)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        ((&raw mut gContestMonPixels).cast::<u8>().cast::<*mut u8>()).read(),
                    );
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    RLUnCompVram(
                        ((&raw const sPictureFrameTiles_Beauty)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        ((100663296i32) as usize as *mut u8),
                    );
                    RLUnCompWram(
                        ((&raw const sPictureFrameTilemap_Beauty)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        ((&raw mut gContestMonPixels).cast::<u8>().cast::<*mut u8>()).read(),
                    );
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    RLUnCompVram(
                        ((&raw const sPictureFrameTiles_Cute)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        ((100663296i32) as usize as *mut u8),
                    );
                    RLUnCompWram(
                        ((&raw const sPictureFrameTilemap_Cute)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        ((&raw mut gContestMonPixels).cast::<u8>().cast::<*mut u8>()).read(),
                    );
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    RLUnCompVram(
                        ((&raw const sPictureFrameTiles_Smart)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        ((100663296i32) as usize as *mut u8),
                    );
                    RLUnCompWram(
                        ((&raw const sPictureFrameTilemap_Smart)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        ((&raw mut gContestMonPixels).cast::<u8>().cast::<*mut u8>()).read(),
                    );
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    RLUnCompVram(
                        ((&raw const sPictureFrameTiles_Tough)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        ((100663296i32) as usize as *mut u8),
                    );
                    RLUnCompWram(
                        ((&raw const sPictureFrameTilemap_Tough)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        ((&raw mut gContestMonPixels).cast::<u8>().cast::<*mut u8>()).read(),
                    );
                    break 'l1;
                }
            }
            {
                y = 0u8;
                'l2: loop {
                    if !(((y) as i32) < 20i32) {
                        break 'l2;
                    }
                    'l3: {
                        {
                            x = 0u8;
                            'l4: loop {
                                if !(((x) as i32) < 32i32) {
                                    break 'l4;
                                }
                                'l5: {
                                    (((100687872i32) as usize as *mut u16).wrapping_offset(
                                        ((((y) as i32).wrapping_mul(32i32))
                                            .wrapping_add(((x) as i32)))
                                            as isize,
                                    ))
                                    .write(4117u16);
                                }
                                x = (x).wrapping_add(1);
                            }
                        }
                    }
                    y = (y).wrapping_add(1);
                }
            }
            {
                y = 0u8;
                'l6: loop {
                    if !(((y) as i32) < 10i32) {
                        break 'l6;
                    }
                    'l7: {
                        {
                            x = 0u8;
                            'l8: loop {
                                if !(((x) as i32) < 18i32) {
                                    break 'l8;
                                }
                                'l9: {
                                    (((100687872i32) as usize as *mut u16).wrapping_offset(
                                        (((((y) as i32).wrapping_add(2i32)).wrapping_mul(32i32))
                                            .wrapping_add(((x) as i32).wrapping_add(6i32)))
                                            as isize,
                                    ))
                                    .write(
                                        (((((((&raw mut gContestMonPixels)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((y) as i32).wrapping_add(2i32)) as isize * 64,
                                        ))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            (((x) as i32).wrapping_add(6i32)) as isize,
                                        ))
                                        .read(),
                                    );
                                }
                                x = (x).wrapping_add(1);
                            }
                        }
                    }
                    y = (y).wrapping_add(1);
                }
            }
            {
                x = 0u8;
                'l10: loop {
                    if !(((x) as i32) < 16i32) {
                        break 'l10;
                    }
                    'l11: {
                        (((100687872i32) as usize as *mut u16).wrapping_offset(
                            ((64i32).wrapping_add(((x) as i32).wrapping_add(7i32))) as isize,
                        ))
                        .write(
                            (((((((&raw mut gContestMonPixels).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<u8>())
                            .wrapping_offset(128))
                            .cast::<u16>())
                            .wrapping_offset(7))
                            .read(),
                        );
                    }
                    x = (x).wrapping_add(1);
                }
            }
        } else {
            if ((contestWinnerId) as i32) < 8i32 {
                RLUnCompVram(
                    ((&raw const sPictureFrameTiles_HallLobby)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100663296i32) as usize as *mut u8),
                );
                RLUnCompVram(
                    ((&raw const sPictureFrameTilemap_HallLobby)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100687872i32) as usize as *mut u8),
                );
            } else {
                'l12: {
                    let __sw2 = crate::c::div_i32(
                        ((((((&raw mut gContestPaintingWinner)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(10))
                        .read()) as i32),
                        3i32,
                    );
                    if __sw2 == 0i32 {
                        RLUnCompVram(
                            ((&raw const sPictureFrameTiles_Cool)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>(),
                            ((100663296i32) as usize as *mut u8),
                        );
                        RLUnCompVram(
                            ((&raw const sPictureFrameTilemap_Cool)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>(),
                            ((100687872i32) as usize as *mut u8),
                        );
                        break 'l12;
                    }
                    if __sw2 == 1i32 {
                        RLUnCompVram(
                            ((&raw const sPictureFrameTiles_Beauty)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>(),
                            ((100663296i32) as usize as *mut u8),
                        );
                        RLUnCompVram(
                            ((&raw const sPictureFrameTilemap_Beauty)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>(),
                            ((100687872i32) as usize as *mut u8),
                        );
                        break 'l12;
                    }
                    if __sw2 == 2i32 {
                        RLUnCompVram(
                            ((&raw const sPictureFrameTiles_Cute)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>(),
                            ((100663296i32) as usize as *mut u8),
                        );
                        RLUnCompVram(
                            ((&raw const sPictureFrameTilemap_Cute)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>(),
                            ((100687872i32) as usize as *mut u8),
                        );
                        break 'l12;
                    }
                    if __sw2 == 3i32 {
                        RLUnCompVram(
                            ((&raw const sPictureFrameTiles_Smart)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>(),
                            ((100663296i32) as usize as *mut u8),
                        );
                        RLUnCompVram(
                            ((&raw const sPictureFrameTilemap_Smart)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>(),
                            ((100687872i32) as usize as *mut u8),
                        );
                        break 'l12;
                    }
                    if __sw2 == 4i32 {
                        RLUnCompVram(
                            ((&raw const sPictureFrameTiles_Tough)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>(),
                            ((100663296i32) as usize as *mut u8),
                        );
                        RLUnCompVram(
                            ((&raw const sPictureFrameTilemap_Tough)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>(),
                            ((100687872i32) as usize as *mut u8),
                        );
                        break 'l12;
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitPaintingMonOamData(contestWinnerId: u8) {
    unsafe {
        let mut contestWinnerId = contestWinnerId;
        (((&raw mut gMain).cast::<u8>()).wrapping_add(56))
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (&raw const sContestPaintingMonOamData)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
        crate::c::bf_write(
            ((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>()).wrapping_add(4),
            0,
            10,
            (0u16) as i32,
        );
        if ((contestWinnerId) as i32) > 1i32 {
            crate::c::bf_write(
                ((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>()).wrapping_add(2),
                0,
                9,
                (88u32) as i32,
            );
            crate::c::bf_write(
                ((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>()).wrapping_add(0),
                0,
                8,
                (24u32) as i32,
            );
        } else {
            crate::c::bf_write(
                ((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>()).wrapping_add(2),
                0,
                9,
                (88u32) as i32,
            );
            crate::c::bf_write(
                ((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>()).wrapping_add(0),
                0,
                8,
                (24u32) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn GetImageEffectForContestWinner(contestWinnerId: u8) -> u8 {
    unsafe {
        let mut contestWinnerId = contestWinnerId;
        let mut contestCategory: u8 = 0u8;
        if ((contestWinnerId) as i32) < 8i32 {
            contestCategory = ((((&raw mut gContestPaintingWinner)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(10))
            .read();
        } else {
            contestCategory = ((crate::c::div_i32(
                ((((((&raw mut gContestPaintingWinner)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(10))
                .read()) as i32),
                3i32,
            )) as u8);
        }
        'l1: {
            let __sw1 = ((contestCategory) as i32);
            if __sw1 == 0i32 {
                return 9u8;
            }
            if __sw1 == 1i32 {
                return 13u8;
            }
            if __sw1 == 2i32 {
                return 2u8;
            }
            if __sw1 == 3i32 {
                return 36u8;
            }
            if __sw1 == 4i32 {
                return 6u8;
            }
        }
        return contestCategory;
    }
}
pub(crate) unsafe extern "C" fn AllocPaintingResources() {
    unsafe {
        ((&raw mut gContestPaintingMonPalette)
            .cast::<u8>()
            .cast::<*mut u16>())
        .write((AllocZeroed(512u32)).cast::<u16>());
        ((&raw mut gContestMonPixels).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(8192u32));
    }
}
pub(crate) unsafe extern "C" fn DoContestPaintingImageProcessing(imageEffect: u8) {
    unsafe {
        let mut imageEffect = imageEffect;
        (((&raw mut gImageProcessingContext).cast::<u8>())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .write(((&raw mut gContestMonPixels).cast::<u8>().cast::<*mut u8>()).read());
        (((&raw mut gImageProcessingContext).cast::<u8>())
            .wrapping_add(8)
            .cast::<*mut u16>())
        .write(
            ((&raw mut gContestPaintingMonPalette)
                .cast::<u8>()
                .cast::<*mut u16>())
            .read(),
        );
        (((&raw mut gImageProcessingContext).cast::<u8>()).wrapping_add(24)).write(0u8);
        (((&raw mut gImageProcessingContext).cast::<u8>()).wrapping_add(31)).write(
            ((crate::c::rem_u32(
                ((((&raw mut gContestPaintingWinner)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .cast::<u32>())
                .read(),
                256u32,
            )) as u8),
        );
        (((&raw mut gImageProcessingContext).cast::<u8>()).wrapping_add(25)).write(0u8);
        (((&raw mut gImageProcessingContext).cast::<u8>()).wrapping_add(26)).write(0u8);
        (((&raw mut gImageProcessingContext).cast::<u8>()).wrapping_add(27)).write(64u8);
        (((&raw mut gImageProcessingContext).cast::<u8>()).wrapping_add(28)).write(64u8);
        (((&raw mut gImageProcessingContext).cast::<u8>()).wrapping_add(29)).write(64u8);
        (((&raw mut gImageProcessingContext).cast::<u8>()).wrapping_add(30)).write(64u8);
        'l1: {
            let __sw1 = ((imageEffect) as i32);
            let __matched =
                __sw1 == 36i32 || __sw1 == 6i32 || __sw1 == 9i32 || __sw1 == 13i32 || __sw1 == 2i32;
            if __sw1 == 36i32 || __sw1 == 6i32 {
                (((&raw mut gImageProcessingContext).cast::<u8>())
                    .wrapping_add(20)
                    .cast::<u16>())
                .write(3u16);
                break 'l1;
            }
            if __sw1 == 9i32 || __sw1 == 13i32 || __sw1 == 2i32 || !__matched {
                (((&raw mut gImageProcessingContext).cast::<u8>())
                    .wrapping_add(20)
                    .cast::<u16>())
                .write(1u16);
                break 'l1;
            }
        }
        (((&raw mut gImageProcessingContext).cast::<u8>())
            .wrapping_add(22)
            .cast::<u16>())
        .write(2u16);
        ((&raw mut gImageProcessingContext).cast::<u8>()).write(imageEffect);
        (((&raw mut gImageProcessingContext).cast::<u8>())
            .wrapping_add(16)
            .cast::<*mut u8>())
        .write(((100728832i32) as usize as *mut u8));
        ApplyImageProcessingEffects((&raw mut gImageProcessingContext).cast::<u8>());
        ApplyImageProcessingQuantization((&raw mut gImageProcessingContext).cast::<u8>());
        ConvertImageProcessingToGBA((&raw mut gImageProcessingContext).cast::<u8>());
        LoadPalette(
            (((&raw mut gContestPaintingMonPalette)
                .cast::<u8>()
                .cast::<*mut u16>())
            .read())
            .cast::<u8>(),
            256u16,
            512u16,
        );
    }
}
pub(crate) unsafe extern "C" fn CreateContestPaintingPicture(contestWinnerId: u8, isForArtist: u8) {
    unsafe {
        let mut contestWinnerId = contestWinnerId;
        let mut isForArtist = isForArtist;
        AllocPaintingResources();
        InitContestMonPixels(
            ((((&raw mut gContestPaintingWinner)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u16>())
            .read(),
            0u8,
        );
        DoContestPaintingImageProcessing(GetImageEffectForContestWinner(contestWinnerId));
        InitPaintingMonOamData(contestWinnerId);
        LoadContestPaintingFrame(contestWinnerId, isForArtist);
    }
}
pub(crate) unsafe extern "C" fn SetBackdropFromPalette(palette: *mut u16) {
    unsafe {
        let mut palette = palette;
        LoadPalette((palette).cast::<u8>(), 0u16, 2u16);
    }
}
