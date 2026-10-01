//! Translated from `src/contest_painting.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs,
    overflowing_literals,
    clippy::if_same_then_else,
    clippy::missing_transmute_annotations,
    clippy::useless_transmute,
    unused_assignments
)]

use crate::agb_main::gMain;
use crate::agb_main::{InitKeys, SetVBlankCallback};
use crate::battle_gfx_sfx_util::{AllocateMonSpritesGfx, FreeMonSpritesGfx};
use crate::battle_main::gMonSpritesGfxPtr;
use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, ResetBgsAndClearDma3BusyFlags, ShowBg,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::contest::{gCurContestWinner, gCurContestWinnerIsForArtist, gCurContestWinnerSaveIdx};
use crate::gpu_regs::SetGpuReg;
use crate::image_processing_effects::{
    ApplyImageProcessingEffects, ApplyImageProcessingQuantization, ConvertImageProcessingToGBA,
};
use crate::international_string_util::ConvertInternationalContestantName;
use crate::international_string_util::GetStringCenterAlignXOffset;
use crate::lilycove_lady::BufferContestName;
use crate::load_save::gSaveBlock1Ptr;
use crate::palette::{
    BeginFastPaletteFade, BeginNormalPaletteFade, LoadPalette, ResetPaletteFade,
    TransferPlttBuffer, UpdatePaletteFade, gPaletteFade,
};
use crate::pokemon::GetMonSpritePalFromSpeciesAndPersonality;
use crate::random::SeedRng;
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::sprite::{LoadOam, ProcessSpriteCopyRequests, ResetSpriteData};
use crate::string_util::{StringAppend, StringCopy, StringExpandPlaceholders};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3, gStringVar4};
use crate::text::{DeactivateAllTextPrinters, RunTextPrinters};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{CopyWindowToVram, FillWindowPixelBuffer, PutWindowTilemap, RemoveWindow};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AddWindow` with this module's view of its types.
#[inline]
unsafe fn AddWindow(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::AddWindow(a0 as _) }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `HandleLoadSpecialPokePic_DontHandleDeoxys` with this module's view of its types.
#[inline]
unsafe fn HandleLoadSpecialPokePic_DontHandleDeoxys(
    a0: *mut CompressedSpriteSheet,
    a1: *mut c_void,
    a2: i32,
    a3: u32,
) {
    unsafe {
        crate::decompress::HandleLoadSpecialPokePic_DontHandleDeoxys(a0 as _, a1 as _, a2, a3);
    }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
unsafe fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `LZDecompressVram` with this module's view of its types.
#[inline]
unsafe fn LZDecompressVram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressVram(a0 as _, a1 as _);
    }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
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

#[unsafe(link_section = "common_data")]
pub static mut gContestMonPixels: *mut CArray<CArray<u16, 32>, 0> = null_mut();
#[unsafe(link_section = "common_data")]
pub static mut gImageProcessingContext: ImageProcessingContext = unsafe { zeroed() };
#[unsafe(link_section = "common_data")]
pub static mut gContestPaintingWinner: *mut ContestWinner = null_mut();
#[unsafe(link_section = "common_data")]
pub static mut gContestPaintingMonPalette: *mut u16 = null_mut();
pub(crate) static sHoldState: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sMosaicVal: crate::global::Global<u16> = crate::global::Global::new(0);
pub(crate) static sFadeCounter: crate::global::Global<u16> = crate::global::Global::new(0);
pub(crate) static sVarsInitialized: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sWindowId: crate::global::Global<u8> = crate::global::Global::new(0);

/// `AddTextPrinterParameterized` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinterParameterized(
    a0: u8,
    a1: u8,
    a2: *mut u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe {
        crate::text::AddTextPrinterParameterized(
            a0,
            a1,
            a2 as _,
            a3,
            a4,
            a5,
            core::mem::transmute(a6),
        )
    }
}
/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `GetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn GetBgTilemapBuffer(a0: u8) -> *mut c_void {
    unsafe { crate::bg::GetBgTilemapBuffer(a0) as *mut c_void }
}
/// `RLUnCompVram` with this module's view of its types.
#[inline]
unsafe fn RLUnCompVram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::RLUnCompVram(a0 as _, a1 as _);
    }
}
/// `RLUnCompWram` with this module's view of its types.
#[inline]
unsafe fn RLUnCompWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::RLUnCompWram(a0 as _, a1 as _);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub unsafe fn SetContestWinnerForPainting(contestWinnerId: i32) {
    let saveIdx: *mut u8 = &raw mut gCurContestWinnerSaveIdx;
    let isForArtist: *mut u8 = &raw mut gCurContestWinnerIsForArtist;
    gCurContestWinner = (*gSaveBlock1Ptr).contestWinners[contestWinnerId - 1];
    *saveIdx = contestWinnerId as u8 - 1;
    *isForArtist = FALSE;
}
pub unsafe fn CB2_ContestPainting() {
    ShowContestPainting();
}
pub(crate) unsafe fn CB2_HoldContestPainting() {
    HoldContestPainting();
    RunTextPrinters();
    UpdatePaletteFade();
}
pub(crate) unsafe fn CB2_QuitContestPainting() {
    SetMainCallback2(gMain.savedCallback);
    Free(gContestPaintingMonPalette as *mut c_void);
    gContestPaintingMonPalette = null_mut();
    Free(gContestMonPixels as *mut c_void);
    gContestMonPixels = null_mut();
    RemoveWindow(sWindowId.get());
    Free(GetBgTilemapBuffer(1));
    FreeMonSpritesGfx();
}
pub(crate) unsafe fn ShowContestPainting() {
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
                                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
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
                                        let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                        volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                        volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                        volatile_write(dmaRegs.at(2), 0x85000000 | (_size / 4));
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
                                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x85000000 | (_size / 4));
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                    }
                }
            }
            BeginFastPaletteFade(2);
            SetVBlankCallback(Some(VBlankCB_ContestPainting));
            sHoldState.set(0);
            SetGpuReg(0x0, 4928);
            SetMainCallback2(Some(CB2_HoldContestPainting));
        }
        _ => {}
    }
}
unsafe fn HoldContestPainting() {
    match sHoldState.get() {
        0 => {
            if gPaletteFade.active() == 0 {
                sHoldState.set(1);
            }
            if sVarsInitialized.get() != 0 && sFadeCounter.get() != 0 {
                sFadeCounter.set(sFadeCounter.get() - 1);
            }
        }
        1 => {
            if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
                sHoldState.set(sHoldState.get() + 1);
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            }
            if sVarsInitialized.get() != 0 {
                sFadeCounter.set(0);
            }
        }
        2 => {
            if gPaletteFade.active() == 0 {
                SetMainCallback2(Some(CB2_QuitContestPainting));
            }
            if sVarsInitialized.get() != 0 && sFadeCounter.get() < 30 {
                sFadeCounter.set(sFadeCounter.get() + 1);
            }
        }
        _ => {}
    }
}
unsafe fn InitContestPaintingWindow() {
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 1);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    SetBgTilemapBuffer(1, AllocZeroed(BG_SCREEN_SIZE));
    sWindowId.set(AddWindow((&raw const *sWindowTemplate).cast_mut()) as u8);
    DeactivateAllTextPrinters();
    FillWindowPixelBuffer(sWindowId.get(), 0);
    PutWindowTilemap(sWindowId.get());
    CopyWindowToVram(sWindowId.get(), COPYWIN_FULL);
    ShowBg(1);
}
unsafe fn PrintContestPaintingCaption(contestType: u8, isForArtist: u8) {
    if isForArtist == TRUE {
        return;
    }
    let category: u8 = (*gContestPaintingWinner).contestCategory;
    if contestType < MUSEUM_CONTEST_WINNERS_START {
        BufferContestName(gStringVar1.as_mut_ptr(), category);
        StringAppend(
            gStringVar1.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_Space).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
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
            (*crate::asmdata::gContestHallPaintingCaption.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    } else {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*gContestPaintingWinner).monName.as_mut_ptr(),
        );
        StringExpandPlaceholders(gStringVar4.as_mut_ptr(), sMuseumCaptions[category]);
    }
    let x: i32 = GetStringCenterAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 208);
    AddTextPrinterParameterized(
        sWindowId.get(),
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x as u8,
        1,
        0,
        None,
    );
    CopyBgTilemapBufferToVram(1);
}
unsafe fn InitContestPaintingBg() {
    SetGpuReg(0x0, 0);
    volatile_write(
        0x4000200_usize as *mut u16,
        (0x4000200_usize as *mut u16).read_volatile() | INTR_FLAG_VBLANK,
    );
    SetGpuReg(REG_OFFSET_BG0CNT, 3138);
    SetGpuReg(0xa, 2629);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
}
fn InitContestPaintingVars(reset: u8) {
    if reset == FALSE {
        sVarsInitialized.set(FALSE);
        sMosaicVal.set(0);
        sFadeCounter.set(0);
    } else {
        sVarsInitialized.set(TRUE);
        sMosaicVal.set(15);
        sFadeCounter.set(30);
    }
}
unsafe fn UpdateContestPaintingMosaicEffect() {
    if sVarsInitialized.get() == 0 {
        SetGpuReg(REG_OFFSET_MOSAIC, 0);
    } else {
        SetGpuReg(0xa, 2629);
        sMosaicVal.set((sFadeCounter.get() as i32 / 2) as u16);
        SetGpuReg(
            REG_OFFSET_MOSAIC,
            sMosaicVal.get() << 12
                | sMosaicVal.get() << 8
                | sMosaicVal.get() << 4
                | sMosaicVal.get(),
        );
    }
}
pub(crate) unsafe fn VBlankCB_ContestPainting() {
    UpdateContestPaintingMosaicEffect();
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
unsafe fn InitContestMonPixels(species: u16, backPic: u8) {
    let pal: *mut c_void = GetMonSpritePalFromSpeciesAndPersonality(
        species,
        (*gContestPaintingWinner).trainerId,
        (*gContestPaintingWinner).personality,
    ) as *mut c_void;
    LZDecompressVram(pal as *mut u32, gContestPaintingMonPalette as *mut c_void);
    if backPic == 0 {
        HandleLoadSpecialPokePic_DontHandleDeoxys(
            (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable).cast::<CArray<
                CompressedSpriteSheet,
                0,
            >>(
            ))[species])
                .cast_mut(),
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
            (&raw const (*(&raw const crate::data::data_tables::gMonBackPicTable)
                .cast::<CArray<CompressedSpriteSheet, 0>>())[species])
                .cast_mut(),
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
unsafe fn _InitContestMonPixels(
    spriteGfx: *mut u8,
    palette: *mut u16,
    destPixels: *mut CArray<CArray<u16, 64>, 64>,
) {
    let mut colorIndex: u8 = 0;
    for tileY in 0..8u16 {
        for tileX in 0..8u16 {
            for pixelY in 0..8u16 {
                for pixelX in 0..8u16 {
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
                }
            }
        }
    }
}
unsafe fn LoadContestPaintingFrame(contestWinnerId: u8, isForArtist: u8) {
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
        for y in 0..20u8 {
            for x in 0..32u8 {
                *(0x6006000_usize as *mut u16).at(y as i32 * 32 + x as i32) = 0x1015;
            }
        }
        for y in 0..10u8 {
            for x in 0..18u8 {
                *(0x6006000_usize as *mut u16).at((y as i32 + 2) * 32 + (x as i32 + 6)) =
                    (*gContestMonPixels)[y as i32 + 2][x as i32 + 6];
            }
        }
        for x in 0..16u8 {
            *(0x6006000_usize as *mut u16).at(64 + (x as i32 + 7)) = (*gContestMonPixels)[2][7];
        }
    } else if contestWinnerId < MUSEUM_CONTEST_WINNERS_START {
        RLUnCompVram(
            sPictureFrameTiles_HallLobby.as_ptr().cast_mut(),
            VRAM as usize as *mut c_void,
        );
        RLUnCompVram(
            sPictureFrameTilemap_HallLobby.as_ptr().cast_mut(),
            0x6006000_usize as *mut c_void,
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
                    0x6006000_usize as *mut c_void,
                );
            }
            1 => {
                RLUnCompVram(
                    sPictureFrameTiles_Beauty.as_ptr().cast_mut(),
                    VRAM as usize as *mut c_void,
                );
                RLUnCompVram(
                    sPictureFrameTilemap_Beauty.as_ptr().cast_mut(),
                    0x6006000_usize as *mut c_void,
                );
            }
            2 => {
                RLUnCompVram(
                    sPictureFrameTiles_Cute.as_ptr().cast_mut(),
                    VRAM as usize as *mut c_void,
                );
                RLUnCompVram(
                    sPictureFrameTilemap_Cute.as_ptr().cast_mut(),
                    0x6006000_usize as *mut c_void,
                );
            }
            3 => {
                RLUnCompVram(
                    sPictureFrameTiles_Smart.as_ptr().cast_mut(),
                    VRAM as usize as *mut c_void,
                );
                RLUnCompVram(
                    sPictureFrameTilemap_Smart.as_ptr().cast_mut(),
                    0x6006000_usize as *mut c_void,
                );
            }
            4 => {
                RLUnCompVram(
                    sPictureFrameTiles_Tough.as_ptr().cast_mut(),
                    VRAM as usize as *mut c_void,
                );
                RLUnCompVram(
                    sPictureFrameTilemap_Tough.as_ptr().cast_mut(),
                    0x6006000_usize as *mut c_void,
                );
            }
            _ => {}
        }
    }
}
unsafe fn InitPaintingMonOamData(contestWinnerId: u8) {
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
unsafe fn GetImageEffectForContestWinner(contestWinnerId: u8) -> u8 {
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
    contestCategory
}
unsafe fn AllocPaintingResources() {
    gContestPaintingMonPalette = AllocZeroed(OBJ_PLTT_SIZE as u32) as *mut u16;
    gContestMonPixels = AllocZeroed(0x2000) as *mut CArray<CArray<u16, 32>, 0>;
}
unsafe fn DoContestPaintingImageProcessing(imageEffect: u8) {
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
unsafe fn CreateContestPaintingPicture(contestWinnerId: u8, isForArtist: u8) {
    AllocPaintingResources();
    InitContestMonPixels((*gContestPaintingWinner).species, FALSE);
    DoContestPaintingImageProcessing(GetImageEffectForContestWinner(contestWinnerId));
    InitPaintingMonOamData(contestWinnerId);
    LoadContestPaintingFrame(contestWinnerId, isForArtist);
}
unsafe fn SetBackdropFromPalette(palette: *mut u16) {
    LoadPalette(palette as *mut c_void, 0, 2);
}
