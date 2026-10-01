//! Translated from `src/contest_painting.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
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
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): sPictureFramePalettes sPictureFrameTiles_Cool sPictureFrameTiles_Beauty sPictureFrameTiles_Cute sPictureFrameTiles_Smart sPictureFrameTiles_Tough sPictureFrameTiles_HallLobby sPictureFrameTilemap_Cool sPictureFrameTilemap_Beauty sPictureFrameTilemap_Cute sPictureFrameTilemap_Smart sPictureFrameTilemap_Tough sPictureFrameTilemap_HallLobby sContestCategoryNames_Unused sContestRankNames sBgTemplates sWindowTemplate sMuseumCaptions sContestPaintingMonOamData sBgPalette

static sBgPalette: Table<CArray<u16, 2>> =
    Table((&raw const crate::data::contest_painting::sBgPalette).cast());
static sBgTemplates: Table<CArray<BgTemplate, 1>> =
    Table((&raw const crate::data::contest_painting::sBgTemplates).cast());
static sContestPaintingMonOamData: Table<OamData> =
    Table((&raw const crate::data::contest_painting::sContestPaintingMonOamData).cast());
static sContestRankNames: Table<CArray<*mut u8, 5>> =
    Table((&raw const crate::data::contest_painting::sContestRankNames).cast());
static sMuseumCaptions: Table<CArray<*mut u8, 15>> =
    Table((&raw const crate::data::contest_painting::sMuseumCaptions).cast());
static sPictureFramePalettes: Table<CArray<u16, 256>> =
    Table((&raw const crate::data::contest_painting::sPictureFramePalettes).cast());
static sPictureFrameTilemap_Beauty: Table<CArray<u32, 323>> =
    Table((&raw const crate::data::contest_painting::sPictureFrameTilemap_Beauty).cast());
static sPictureFrameTilemap_Cool: Table<CArray<u32, 323>> =
    Table((&raw const crate::data::contest_painting::sPictureFrameTilemap_Cool).cast());
static sPictureFrameTilemap_Cute: Table<CArray<u32, 323>> =
    Table((&raw const crate::data::contest_painting::sPictureFrameTilemap_Cute).cast());
static sPictureFrameTilemap_HallLobby: Table<CArray<u32, 324>> =
    Table((&raw const crate::data::contest_painting::sPictureFrameTilemap_HallLobby).cast());
static sPictureFrameTilemap_Smart: Table<CArray<u32, 323>> =
    Table((&raw const crate::data::contest_painting::sPictureFrameTilemap_Smart).cast());
static sPictureFrameTilemap_Tough: Table<CArray<u32, 323>> =
    Table((&raw const crate::data::contest_painting::sPictureFrameTilemap_Tough).cast());
static sPictureFrameTiles_Beauty: Table<CArray<u32, 780>> =
    Table((&raw const crate::data::contest_painting::sPictureFrameTiles_Beauty).cast());
static sPictureFrameTiles_Cool: Table<CArray<u32, 1057>> =
    Table((&raw const crate::data::contest_painting::sPictureFrameTiles_Cool).cast());
static sPictureFrameTiles_Cute: Table<CArray<u32, 718>> =
    Table((&raw const crate::data::contest_painting::sPictureFrameTiles_Cute).cast());
static sPictureFrameTiles_HallLobby: Table<CArray<u32, 385>> =
    Table((&raw const crate::data::contest_painting::sPictureFrameTiles_HallLobby).cast());
static sPictureFrameTiles_Smart: Table<CArray<u32, 1006>> =
    Table((&raw const crate::data::contest_painting::sPictureFrameTiles_Smart).cast());
static sPictureFrameTiles_Tough: Table<CArray<u32, 1100>> =
    Table((&raw const crate::data::contest_painting::sPictureFrameTiles_Tough).cast());
static sWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::contest_painting::sWindowTemplate).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gContestMonPixels: *mut CArray<CArray<u16, 32>, 0> = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gImageProcessingContext: ImageProcessingContext = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gContestPaintingWinner: *mut ContestWinner = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gContestPaintingMonPalette: *mut u16 = null_mut();
pub(crate) static mut sHoldState: u8 = 0;
pub(crate) static mut sMosaicVal: u16 = 0;
pub(crate) static mut sFadeCounter: u16 = 0;
pub(crate) static mut sVarsInitialized: u8 = 0;
pub(crate) static mut sWindowId: u8 = 0;

unsafe extern "C" {
    static gContestHallPaintingCaption: CArray<u8, 0>;
    static mut gCurContestWinner: ContestWinner;
    static mut gCurContestWinnerIsForArtist: u8;
    static mut gCurContestWinnerSaveIdx: u8;
    static mut gMain: Main;
    static gMonBackPicTable: CArray<CompressedSpriteSheet, 0>;
    static gMonFrontPicTable: CArray<CompressedSpriteSheet, 0>;
    static mut gMonSpritesGfxPtr: *mut MonSpritesGfx;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static gText_Space: CArray<u8, 0>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AllocateMonSpritesGfx();
    fn ApplyImageProcessingEffects(a0: *mut ImageProcessingContext);
    fn ApplyImageProcessingQuantization(a0: *mut ImageProcessingContext);
    fn BeginFastPaletteFade(a0: u8);
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BufferContestName(a0: *mut u8, a1: u8);
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ConvertImageProcessingToGBA(a0: *mut ImageProcessingContext);
    fn ConvertInternationalContestantName(a0: *mut u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn DeactivateAllTextPrinters();
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut c_void);
    fn FreeMonSpritesGfx();
    fn GetBgTilemapBuffer(a0: u8) -> *mut c_void;
    fn GetMonSpritePalFromSpeciesAndPersonality(a0: u16, a1: u32, a2: u32) -> *mut u32;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn HandleLoadSpecialPokePic_DontHandleDeoxys(
        a0: *mut CompressedSpriteSheet,
        a1: *mut c_void,
        a2: i32,
        a3: u32,
    );
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitKeys();
    fn LZDecompressVram(a0: *mut u32, a1: *mut c_void);
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RLUnCompVram(a0: *mut u32, a1: *mut c_void);
    fn RLUnCompWram(a0: *mut u32, a1: *mut c_void);
    fn RemoveWindow(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn RunTextPrinters();
    fn ScanlineEffect_Stop();
    fn SeedRng(a0: u16);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
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
    let mut saveIdx: *mut u8 = &raw mut gCurContestWinnerSaveIdx;
    let mut isForArtist: *mut u8 = &raw mut gCurContestWinnerIsForArtist;
    gCurContestWinner = (*gSaveBlock1Ptr).contestWinners[contestWinnerId - 1];
    *saveIdx = contestWinnerId as u8 - 1;
    *isForArtist = FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ContestPainting() {
    ShowContestPainting();
}
pub(crate) unsafe extern "C" fn CB2_HoldContestPainting() {
    HoldContestPainting();
    RunTextPrinters();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn CB2_QuitContestPainting() {
    SetMainCallback2(gMain.savedCallback);
    Free(gContestPaintingMonPalette as *mut c_void);
    gContestPaintingMonPalette = null_mut();
    Free(gContestMonPixels as *mut c_void);
    gContestMonPixels = null_mut();
    RemoveWindow(sWindowId);
    Free(GetBgTilemapBuffer(1));
    FreeMonSpritesGfx();
}
pub(crate) unsafe extern "C" fn ShowContestPainting() {
    match gMain.state {
        0 => {
            ScanlineEffect_Stop();
            SetVBlankCallback(None);
            AllocateMonSpritesGfx();
            gContestPaintingWinner = &raw mut gCurContestWinner;
            InitContestPaintingVars(TRUE);
            InitContestPaintingBg();
            gMain.state += 1;
        }
        1 => {
            ResetPaletteFade();
            {
                let mut _dest: *mut c_void = VRAM as usize as *mut c_void;
                let mut _size: u32 = VRAM_SIZE;
                loop {
                    {
                        {
                            let mut tmp: u32 = 0;
                            volatile_write(&raw mut tmp, 0);
                            {
                                {
                                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x85000400);
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                    }
                    _dest = (_dest as *mut u8).at(4096) as *mut c_void;
                    _size -= 0x1000;
                    if _size <= 0x1000 {
                        {
                            {
                                let mut tmp: u32 = 0;
                                volatile_write(&raw mut tmp, 0);
                                {
                                    {
                                        let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                        volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                        volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                        volatile_write(dmaRegs.at(2), 0x85000000 | _size / 4);
                                        let _ = (dmaRegs.at(2)).read_volatile();
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
            }
            ResetSpriteData();
            gMain.state += 1;
        }
        2 => {
            SeedRng(gMain.vblankCounter1 as u16);
            InitKeys();
            InitContestPaintingWindow();
            gMain.state += 1;
        }
        3 => {
            CreateContestPaintingPicture(gCurContestWinnerSaveIdx, gCurContestWinnerIsForArtist);
            gMain.state += 1;
        }
        4 => {
            PrintContestPaintingCaption(gCurContestWinnerSaveIdx, gCurContestWinnerIsForArtist);
            SetBackdropFromPalette(sBgPalette.as_ptr().cast_mut());
            {
                {
                    let mut _dest: *mut u32 = PLTT as i32 as usize as *mut u32;
                    let mut _size: u32 = PLTT_SIZE;
                    {
                        {
                            let mut tmp: u32 = 0;
                            volatile_write(&raw mut tmp, 0);
                            {
                                {
                                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x85000000 | _size / 4);
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                    }
                }
            }
            BeginFastPaletteFade(2);
            SetVBlankCallback(Some(VBlankCB_ContestPainting));
            sHoldState = 0;
            SetGpuReg(0x0, 4928);
            SetMainCallback2(Some(CB2_HoldContestPainting));
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn HoldContestPainting() {
    match sHoldState {
        0 => {
            if gPaletteFade.active() == 0 {
                sHoldState = 1;
            }
            if sVarsInitialized != 0 && sFadeCounter != 0 {
                sFadeCounter -= 1;
            }
        }
        1 => {
            if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
                sHoldState += 1;
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            }
            if sVarsInitialized != 0 {
                sFadeCounter = 0;
            }
        }
        2 => {
            if gPaletteFade.active() == 0 {
                SetMainCallback2(Some(CB2_QuitContestPainting));
            }
            if sVarsInitialized != 0 && sFadeCounter < 30 {
                sFadeCounter += 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn InitContestPaintingWindow() {
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 1);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    SetBgTilemapBuffer(1, AllocZeroed(BG_SCREEN_SIZE));
    sWindowId = AddWindow((&raw const *sWindowTemplate).cast_mut()) as u8;
    DeactivateAllTextPrinters();
    FillWindowPixelBuffer(sWindowId, 0);
    PutWindowTilemap(sWindowId);
    CopyWindowToVram(sWindowId, COPYWIN_FULL);
    ShowBg(1);
}
pub(crate) unsafe extern "C" fn PrintContestPaintingCaption(contestType: u8, isForArtist: u8) {
    let mut x: i32 = 0;
    let mut category: u8 = 0;
    if isForArtist == TRUE {
        return;
    }
    category = (*gContestPaintingWinner).contestCategory;
    if contestType < MUSEUM_CONTEST_WINNERS_START {
        BufferContestName(gStringVar1.as_mut_ptr(), category);
        StringAppend(gStringVar1.as_mut_ptr(), gText_Space.as_ptr().cast_mut());
        StringAppend(
            gStringVar1.as_mut_ptr(),
            sContestRankNames[(*gContestPaintingWinner).contestRank],
        );
        StringCopy(
            gStringVar2.as_mut_ptr(),
            (*gContestPaintingWinner).trainerName.as_mut_ptr(),
        );
        ConvertInternationalContestantName(gStringVar2.as_mut_ptr());
        StringCopy(
            gStringVar3.as_mut_ptr(),
            (*gContestPaintingWinner).monName.as_mut_ptr(),
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gContestHallPaintingCaption.as_ptr().cast_mut(),
        );
    } else {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*gContestPaintingWinner).monName.as_mut_ptr(),
        );
        StringExpandPlaceholders(gStringVar4.as_mut_ptr(), sMuseumCaptions[category]);
    }
    x = GetStringCenterAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 208);
    AddTextPrinterParameterized(
        sWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x as u8,
        1,
        0,
        None,
    );
    CopyBgTilemapBufferToVram(1);
}
pub(crate) unsafe extern "C" fn InitContestPaintingBg() {
    SetGpuReg(0x0, 0);
    volatile_write(
        0x4000200 as usize as *mut u16,
        (0x4000200 as usize as *mut u16).read_volatile() | INTR_FLAG_VBLANK,
    );
    SetGpuReg(REG_OFFSET_BG0CNT, 3138);
    SetGpuReg(0xa, 2629);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
}
pub(crate) unsafe extern "C" fn InitContestPaintingVars(reset: u8) {
    if reset == FALSE {
        sVarsInitialized = FALSE;
        sMosaicVal = 0;
        sFadeCounter = 0;
    } else {
        sVarsInitialized = TRUE;
        sMosaicVal = 15;
        sFadeCounter = 30;
    }
}
pub(crate) unsafe extern "C" fn UpdateContestPaintingMosaicEffect() {
    if sVarsInitialized == 0 {
        SetGpuReg(REG_OFFSET_MOSAIC, 0);
    } else {
        SetGpuReg(0xa, 2629);
        sMosaicVal = (sFadeCounter as i32 / 2) as u16;
        SetGpuReg(
            REG_OFFSET_MOSAIC,
            sMosaicVal << 12 | sMosaicVal << 8 | sMosaicVal << 4 | sMosaicVal << 0,
        );
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_ContestPainting() {
    UpdateContestPaintingMosaicEffect();
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn InitContestMonPixels(species: u16, backPic: u8) {
    let mut pal: *mut c_void = GetMonSpritePalFromSpeciesAndPersonality(
        species,
        (*gContestPaintingWinner).trainerId,
        (*gContestPaintingWinner).personality,
    ) as *mut c_void;
    LZDecompressVram(pal as *mut u32, gContestPaintingMonPalette as *mut c_void);
    if backPic == 0 {
        HandleLoadSpecialPokePic_DontHandleDeoxys(
            (&raw const gMonFrontPicTable[species]).cast_mut(),
            (*gMonSpritesGfxPtr).sprites.ptr[1],
            species as i32,
            (*gContestPaintingWinner).personality,
        );
        _InitContestMonPixels(
            (*gMonSpritesGfxPtr).sprites.ptr[1] as *mut u8,
            gContestPaintingMonPalette,
            gContestMonPixels as *mut c_void as *mut CArray<CArray<u16, 64>, 64>,
        );
    } else {
        HandleLoadSpecialPokePic_DontHandleDeoxys(
            (&raw const gMonBackPicTable[species]).cast_mut(),
            (*gMonSpritesGfxPtr).sprites.ptr[0],
            species as i32,
            (*gContestPaintingWinner).personality,
        );
        _InitContestMonPixels(
            (*gMonSpritesGfxPtr).sprites.ptr[0] as *mut u8,
            gContestPaintingMonPalette,
            gContestMonPixels as *mut c_void as *mut CArray<CArray<u16, 64>, 64>,
        );
    }
}
pub(crate) unsafe extern "C" fn _InitContestMonPixels(
    spriteGfx: *mut u8,
    palette: *mut u16,
    destPixels: *mut CArray<CArray<u16, 64>, 64>,
) {
    let mut tileY: u16 = 0;
    let mut tileX: u16 = 0;
    let mut pixelY: u16 = 0;
    let mut pixelX: u16 = 0;
    let mut colorIndex: u8 = 0;
    tileY = 0;
    while tileY < 8 {
        tileX = 0;
        while tileX < 8 {
            pixelY = 0;
            while pixelY < 8 {
                pixelX = 0;
                while pixelX < 8 {
                    colorIndex = *spriteGfx.at(32 * (tileY as i32 * 8 + tileX as i32)
                        + ((pixelY as i32) << 2)
                        + (pixelX >> 1) as i32);
                    if pixelX as i32 & 1 != 0 {
                        colorIndex >>= 4;
                    } else {
                        colorIndex &= 0xF;
                    }
                    if colorIndex == 0 {
                        (*destPixels)[8 * tileY as i32 + pixelY as i32]
                            [tileX as i32 * 8 + pixelX as i32] = 0x8000;
                    } else {
                        (*destPixels)[8 * tileY as i32 + pixelY as i32]
                            [tileX as i32 * 8 + pixelX as i32] = *palette.at(colorIndex);
                    }
                    pixelX += 1;
                }
                pixelY += 1;
            }
            tileX += 1;
        }
        tileY += 1;
    }
}
pub(crate) unsafe extern "C" fn LoadContestPaintingFrame(contestWinnerId: u8, isForArtist: u8) {
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    LoadPalette(
        sPictureFramePalettes.as_ptr().cast_mut() as *mut c_void,
        0,
        256,
    );
    if isForArtist == TRUE {
        match (*gContestPaintingWinner).contestCategory as i32 / 3 {
            0 => {
                RLUnCompVram(
                    sPictureFrameTiles_Cool.as_ptr().cast_mut(),
                    VRAM as usize as *mut c_void,
                );
                RLUnCompWram(
                    sPictureFrameTilemap_Cool.as_ptr().cast_mut(),
                    gContestMonPixels as *mut c_void,
                );
            }
            1 => {
                RLUnCompVram(
                    sPictureFrameTiles_Beauty.as_ptr().cast_mut(),
                    VRAM as usize as *mut c_void,
                );
                RLUnCompWram(
                    sPictureFrameTilemap_Beauty.as_ptr().cast_mut(),
                    gContestMonPixels as *mut c_void,
                );
            }
            2 => {
                RLUnCompVram(
                    sPictureFrameTiles_Cute.as_ptr().cast_mut(),
                    VRAM as usize as *mut c_void,
                );
                RLUnCompWram(
                    sPictureFrameTilemap_Cute.as_ptr().cast_mut(),
                    gContestMonPixels as *mut c_void,
                );
            }
            3 => {
                RLUnCompVram(
                    sPictureFrameTiles_Smart.as_ptr().cast_mut(),
                    VRAM as usize as *mut c_void,
                );
                RLUnCompWram(
                    sPictureFrameTilemap_Smart.as_ptr().cast_mut(),
                    gContestMonPixels as *mut c_void,
                );
            }
            4 => {
                RLUnCompVram(
                    sPictureFrameTiles_Tough.as_ptr().cast_mut(),
                    VRAM as usize as *mut c_void,
                );
                RLUnCompWram(
                    sPictureFrameTilemap_Tough.as_ptr().cast_mut(),
                    gContestMonPixels as *mut c_void,
                );
            }
            _ => {}
        }
        y = 0;
        while y < 20 {
            x = 0;
            while x < 32 {
                *(0x6006000 as usize as *mut u16).at(y as i32 * 32 + x as i32) = 0x1015;
                x += 1;
            }
            y += 1;
        }
        y = 0;
        while y < 10 {
            x = 0;
            while x < 18 {
                *(0x6006000 as usize as *mut u16).at((y as i32 + 2) * 32 + (x as i32 + 6)) =
                    (*gContestMonPixels)[y as i32 + 2][x as i32 + 6];
                x += 1;
            }
            y += 1;
        }
        x = 0;
        while x < 16 {
            *(0x6006000 as usize as *mut u16).at(64 + (x as i32 + 7)) = (*gContestMonPixels)[2][7];
            x += 1;
        }
    } else if contestWinnerId < MUSEUM_CONTEST_WINNERS_START {
        RLUnCompVram(
            sPictureFrameTiles_HallLobby.as_ptr().cast_mut(),
            VRAM as usize as *mut c_void,
        );
        RLUnCompVram(
            sPictureFrameTilemap_HallLobby.as_ptr().cast_mut(),
            0x6006000 as usize as *mut c_void,
        );
    } else {
        match (*gContestPaintingWinner).contestCategory as i32 / 3 {
            0 => {
                RLUnCompVram(
                    sPictureFrameTiles_Cool.as_ptr().cast_mut(),
                    VRAM as usize as *mut c_void,
                );
                RLUnCompVram(
                    sPictureFrameTilemap_Cool.as_ptr().cast_mut(),
                    0x6006000 as usize as *mut c_void,
                );
            }
            1 => {
                RLUnCompVram(
                    sPictureFrameTiles_Beauty.as_ptr().cast_mut(),
                    VRAM as usize as *mut c_void,
                );
                RLUnCompVram(
                    sPictureFrameTilemap_Beauty.as_ptr().cast_mut(),
                    0x6006000 as usize as *mut c_void,
                );
            }
            2 => {
                RLUnCompVram(
                    sPictureFrameTiles_Cute.as_ptr().cast_mut(),
                    VRAM as usize as *mut c_void,
                );
                RLUnCompVram(
                    sPictureFrameTilemap_Cute.as_ptr().cast_mut(),
                    0x6006000 as usize as *mut c_void,
                );
            }
            3 => {
                RLUnCompVram(
                    sPictureFrameTiles_Smart.as_ptr().cast_mut(),
                    VRAM as usize as *mut c_void,
                );
                RLUnCompVram(
                    sPictureFrameTilemap_Smart.as_ptr().cast_mut(),
                    0x6006000 as usize as *mut c_void,
                );
            }
            4 => {
                RLUnCompVram(
                    sPictureFrameTiles_Tough.as_ptr().cast_mut(),
                    VRAM as usize as *mut c_void,
                );
                RLUnCompVram(
                    sPictureFrameTilemap_Tough.as_ptr().cast_mut(),
                    0x6006000 as usize as *mut c_void,
                );
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn InitPaintingMonOamData(contestWinnerId: u8) {
    gMain.oamBuffer[0] = *sContestPaintingMonOamData;
    gMain.oamBuffer[0].set_tileNum(0);
    if contestWinnerId > 1 {
        gMain.oamBuffer[0].set_x(88);
        gMain.oamBuffer[0].set_y(24);
    } else {
        gMain.oamBuffer[0].set_x(88);
        gMain.oamBuffer[0].set_y(24);
    }
}
pub(crate) unsafe extern "C" fn GetImageEffectForContestWinner(contestWinnerId: u8) -> u8 {
    let mut contestCategory: u8 = 0;
    if contestWinnerId < MUSEUM_CONTEST_WINNERS_START {
        contestCategory = (*gContestPaintingWinner).contestCategory;
    } else {
        contestCategory = ((*gContestPaintingWinner).contestCategory as i32 / 3) as u8;
    }
    match contestCategory {
        CONTEST_CATEGORY_COOL => {
            return IMAGE_EFFECT_OUTLINE_COLORED;
        }
        CONTEST_CATEGORY_BEAUTY => {
            return IMAGE_EFFECT_SHIMMER;
        }
        CONTEST_CATEGORY_CUTE => {
            return IMAGE_EFFECT_POINTILLISM;
        }
        CONTEST_CATEGORY_SMART => {
            return IMAGE_EFFECT_CHARCOAL;
        }
        CONTEST_CATEGORY_TOUGH => {
            return IMAGE_EFFECT_GRAYSCALE_LIGHT;
        }
        _ => {}
    }
    return contestCategory;
}
pub(crate) unsafe extern "C" fn AllocPaintingResources() {
    gContestPaintingMonPalette = AllocZeroed(OBJ_PLTT_SIZE as u32) as *mut u16;
    gContestMonPixels = AllocZeroed(0x2000) as *mut CArray<CArray<u16, 32>, 0>;
}
pub(crate) unsafe extern "C" fn DoContestPaintingImageProcessing(imageEffect: u8) {
    gImageProcessingContext.canvasPixels = gContestMonPixels as *mut c_void;
    gImageProcessingContext.canvasPalette = gContestPaintingMonPalette;
    gImageProcessingContext.paletteStart = 0;
    gImageProcessingContext.personality = ((*gContestPaintingWinner).personality % 256) as u8;
    gImageProcessingContext.columnStart = 0;
    gImageProcessingContext.rowStart = 0;
    gImageProcessingContext.columnEnd = 64;
    gImageProcessingContext.rowEnd = 64;
    gImageProcessingContext.canvasWidth = 64;
    gImageProcessingContext.canvasHeight = 64;
    match imageEffect {
        IMAGE_EFFECT_CHARCOAL | IMAGE_EFFECT_GRAYSCALE_LIGHT => {
            gImageProcessingContext.quantizeEffect = QUANTIZE_EFFECT_GRAYSCALE;
        }
        _ => {
            gImageProcessingContext.quantizeEffect = QUANTIZE_EFFECT_STANDARD_LIMITED_COLORS;
        }
    }
    gImageProcessingContext.var_16 = 2;
    gImageProcessingContext.effect = imageEffect;
    gImageProcessingContext.dest = OBJ_VRAM0 as usize as *mut c_void;
    ApplyImageProcessingEffects(&raw mut gImageProcessingContext);
    ApplyImageProcessingQuantization(&raw mut gImageProcessingContext);
    ConvertImageProcessingToGBA(&raw mut gImageProcessingContext);
    LoadPalette(gContestPaintingMonPalette as *mut c_void, 256, 512);
}
pub(crate) unsafe extern "C" fn CreateContestPaintingPicture(contestWinnerId: u8, isForArtist: u8) {
    AllocPaintingResources();
    InitContestMonPixels((*gContestPaintingWinner).species, FALSE);
    DoContestPaintingImageProcessing(GetImageEffectForContestWinner(contestWinnerId));
    InitPaintingMonOamData(contestWinnerId);
    LoadContestPaintingFrame(contestWinnerId, isForArtist);
}
pub(crate) unsafe extern "C" fn SetBackdropFromPalette(palette: *mut u16) {
    LoadPalette(palette as *mut c_void, 0, 2);
}
