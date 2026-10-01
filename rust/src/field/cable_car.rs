//! Translated from `src/cable_car.c` by tools/rustport/c2rs.py.
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
    clippy::missing_transmute_annotations,
    clippy::useless_transmute,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::bg::{
    CopyBgTilemapBufferToVram, FillBgTilemapBufferRect, HideBg, ResetBgsAndClearDma3BusyFlags,
    ShowBg, UnsetBgTilemapBuffer,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_object_movement::CreateObjectGraphicsSprite;
use crate::ffi::gSpecialVar_0x8004;
use crate::field_weather::{SetCurrentAndNextWeatherNoDelay, SetNextWeather, StartWeather};
use crate::gpu_regs::SetGpuReg;
use crate::load_save::gSaveBlock2Ptr;
use crate::menu::{
    DecompressAndCopyTileDataToVram, FreeTempTileDataBuffersIfPossible, ResetTempTileDataBuffers,
    malloc_and_decompress,
};
use crate::overworld::{CB2_LoadMap, WarpIntoMap, gFieldCallback};
use crate::palette::{
    BeginNormalPaletteFade, LoadPalette, ResetPaletteFade, TransferPlttBuffer, UpdatePaletteFade,
    gPaletteFade,
};
use crate::random::Random;
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::script::LockPlayerFieldControls;
use crate::sound::{FadeInNewBGM, FadeOutBGM, InitMapMusic, MapMusicMain, ResetMapMusic};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData, gSpriteCoordOffsetX, gSpriteCoordOffsetY,
};
use crate::task::{DestroyTask, ResetTasks, RunTasks};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CopyToBgTilemapBufferRect_ChangePalette` with this module's view of its types.
#[inline]
unsafe fn CopyToBgTilemapBufferRect_ChangePalette(
    a0: u8,
    a1: *mut c_void,
    a2: u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: u8,
) {
    unsafe {
        crate::bg::CopyToBgTilemapBufferRect_ChangePalette(a0, a1 as _, a2, a3, a4, a5, a6);
    }
}
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
unsafe fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `LoadSpritePalettes` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalettes(a0: *mut SpritePalette) {
    unsafe {
        crate::sprite::LoadSpritePalettes(a0 as _);
    }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
// The C's names for task and sprite data slots.
const sXPos: usize = 0;
const sSameDir: usize = 1;
const sYPos: usize = 1;
const sDelay: usize = 2;
const sState: usize = 2;
// Data tables (translate with cdata.py): sBgTemplates sGround_Tilemap sTrees_Tilemap sBgMountains_Tilemap sPylonTop_Tilemap sPylonPole_Tilemap sSpriteSheets sSpritePalettes sOam_CableCar sOam_CableCarDoor sOam_Cable sSpriteTemplates_CableCar sSpriteTemplate_Cable

/// `struct CableCar`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CableCar {
    pub bgTaskId: u8,
    pub state: u8,
    pub weather: u8,
    pub weatherDelay: u16,
    pub timer: u16,
    pub bg0HorizontalOffset: u8,
    pub bg0VerticalOffset: u8,
    pub unused0: CArray<u8, 2>,
    pub bg1HorizontalOffset: u8,
    pub bg1VerticalOffset: u8,
    pub unused1: CArray<u8, 6>,
    pub bg3HorizontalOffset: u8,
    pub bg3VerticalOffset: u8,
    pub unused2: CArray<u8, 2>,
    pub groundTileIdx: u8,
    pub groundSegmentXStart: u8,
    pub groundSegmentYStart: u8,
    pub groundTilemapOffset: u8,
    pub groundTimer: u8,
    pub groundXOffset: u8,
    pub groundYOffset: u8,
    pub groundXBase: u8,
    pub groundYBase: u8,
    pub groundTileBuffer: CArray<CArray<u16, 12>, 9>,
    pub unused3: CArray<u8, 2>,
    pub bgTilemapBuffers: CArray<CArray<u16, 2048>, 4>,
    pub groundTilemap: *mut u16,
    pub treesTilemap: *mut u16,
    pub bgMountainsTilemap: *mut u16,
    pub pylonTopTilemap: *mut u16,
    pub pylonPoleTilemap: *mut u16,
}

unsafe impl Sync for CableCar {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<CableCar>() == 16656);
    assert!(offset_of!(CableCar, bgTaskId) == 0);
    assert!(offset_of!(CableCar, state) == 1);
    assert!(offset_of!(CableCar, weather) == 2);
    assert!(offset_of!(CableCar, weatherDelay) == 4);
    assert!(offset_of!(CableCar, timer) == 6);
    assert!(offset_of!(CableCar, bg0HorizontalOffset) == 8);
    assert!(offset_of!(CableCar, bg0VerticalOffset) == 9);
    assert!(offset_of!(CableCar, unused0) == 10);
    assert!(offset_of!(CableCar, bg1HorizontalOffset) == 12);
    assert!(offset_of!(CableCar, bg1VerticalOffset) == 13);
    assert!(offset_of!(CableCar, unused1) == 14);
    assert!(offset_of!(CableCar, bg3HorizontalOffset) == 20);
    assert!(offset_of!(CableCar, bg3VerticalOffset) == 21);
    assert!(offset_of!(CableCar, unused2) == 22);
    assert!(offset_of!(CableCar, groundTileIdx) == 24);
    assert!(offset_of!(CableCar, groundSegmentXStart) == 25);
    assert!(offset_of!(CableCar, groundSegmentYStart) == 26);
    assert!(offset_of!(CableCar, groundTilemapOffset) == 27);
    assert!(offset_of!(CableCar, groundTimer) == 28);
    assert!(offset_of!(CableCar, groundXOffset) == 29);
    assert!(offset_of!(CableCar, groundYOffset) == 30);
    assert!(offset_of!(CableCar, groundXBase) == 31);
    assert!(offset_of!(CableCar, groundYBase) == 32);
    assert!(offset_of!(CableCar, groundTileBuffer) == 34);
    assert!(offset_of!(CableCar, unused3) == 250);
    assert!(offset_of!(CableCar, bgTilemapBuffers) == 252);
    assert!(offset_of!(CableCar, groundTilemap) == 16636);
    assert!(offset_of!(CableCar, treesTilemap) == 16640);
    assert!(offset_of!(CableCar, bgMountainsTilemap) == 16644);
    assert!(offset_of!(CableCar, pylonTopTilemap) == 16648);
    assert!(offset_of!(CableCar, pylonPoleTilemap) == 16652);
};

const STATE_END: u8 = 255;

static sBgMountains_Tilemap: Table<CArray<u16, 226>> =
    Table((&raw const crate::data::cable_car::sBgMountains_Tilemap).cast());
static sBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::cable_car::sBgTemplates).cast());
static sGround_Tilemap: Table<CArray<u16, 172>> =
    Table((&raw const crate::data::cable_car::sGround_Tilemap).cast());
static sPylonPole_Tilemap: Table<CArray<u16, 18>> =
    Table((&raw const crate::data::cable_car::sPylonPole_Tilemap).cast());
static sPylonTop_Tilemap: Table<CArray<u16, 10>> =
    Table((&raw const crate::data::cable_car::sPylonTop_Tilemap).cast());
static sSpritePalettes: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::cable_car::sSpritePalettes).cast());
static sSpriteSheets: Table<CArray<CompressedSpriteSheet, 4>> =
    Table((&raw const crate::data::cable_car::sSpriteSheets).cast());
static sSpriteTemplate_Cable: Table<SpriteTemplate> =
    Table((&raw const crate::data::cable_car::sSpriteTemplate_Cable).cast());
static sSpriteTemplates_CableCar: Table<CArray<SpriteTemplate, 2>> =
    Table((&raw const crate::data::cable_car::sSpriteTemplates_CableCar).cast());
static sTrees_Tilemap: Table<CArray<u16, 194>> =
    Table((&raw const crate::data::cable_car::sTrees_Tilemap).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCableCar: *mut CableCar = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static sGroundX_Up: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sGroundY_Up: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sGroundSegmentY_Up: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sGroundX_Down: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sGroundY_Down: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sGroundSegmentY_Down: crate::global::Global<u8> = crate::global::Global::new(0);

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub(crate) unsafe fn Task_LoadCableCar(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetMainCallback2(Some(CB2_LoadCableCar));
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CableCar() {
    LockPlayerFieldControls();
    CreateTask(Some(Task_LoadCableCar), 1);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
}
pub(crate) unsafe fn CB2_LoadCableCar() {
    let mut sizeOut: u32 = 0;
    match gMain.state {
        1 => {
            ResetSpriteData();
            ResetTasks();
            FreeAllSpritePalettes();
            ResetPaletteFade();
            ResetTempTileDataBuffers();
            StartWeather();
            for i in 0..NUM_ASH_SPRITES {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .ashSprites[i] = null_mut();
            }
            InitMapMusic();
            ResetMapMusic();
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 4);
            SetBgTilemapBuffer(
                0,
                (*sCableCar).bgTilemapBuffers[0].as_mut_ptr() as *mut c_void,
            );
            SetBgTilemapBuffer(
                1,
                (*sCableCar).bgTilemapBuffers[1].as_mut_ptr() as *mut c_void,
            );
            SetBgTilemapBuffer(
                2,
                (*sCableCar).bgTilemapBuffers[2].as_mut_ptr() as *mut c_void,
            );
            SetBgTilemapBuffer(
                3,
                (*sCableCar).bgTilemapBuffers[3].as_mut_ptr() as *mut c_void,
            );
            gSpriteCoordOffsetX = {
                gSpriteCoordOffsetY = 0;
                gSpriteCoordOffsetY
            };
            gMain.state += 1;
        }
        2 => {
            for i in 0..3u8 {
                LoadCompressedSpriteSheet((&raw const sSpriteSheets[i]).cast_mut());
            }
            LoadSpritePalettes(sSpritePalettes.as_ptr().cast_mut());
            (*sCableCar).groundTilemap = malloc_and_decompress(
                sGround_Tilemap.as_ptr().cast_mut() as *mut c_void,
                &raw mut sizeOut,
            ) as *mut u16;
            (*sCableCar).treesTilemap = malloc_and_decompress(
                sTrees_Tilemap.as_ptr().cast_mut() as *mut c_void,
                &raw mut sizeOut,
            ) as *mut u16;
            (*sCableCar).bgMountainsTilemap = malloc_and_decompress(
                sBgMountains_Tilemap.as_ptr().cast_mut() as *mut c_void,
                &raw mut sizeOut,
            ) as *mut u16;
            (*sCableCar).pylonPoleTilemap = malloc_and_decompress(
                sPylonPole_Tilemap.as_ptr().cast_mut() as *mut c_void,
                &raw mut sizeOut,
            ) as *mut u16;
            (*sCableCar).pylonTopTilemap = sPylonTop_Tilemap.as_ptr().cast_mut();
            DecompressAndCopyTileDataToVram(
                0,
                (*(&raw const crate::data::graphics::gCableCarBg_Gfx).cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            gMain.state += 1;
        }
        3 => {
            if FreeTempTileDataBuffersIfPossible() == 0 {
                LoadPalette(
                    (*(&raw const crate::data::graphics::gCableCarBg_Pal).cast::<CArray<u16, 0>>())
                        .as_ptr()
                        .cast_mut() as *mut c_void,
                    0,
                    128,
                );
                gMain.state += 1;
            }
        }
        4 => {
            CreateCableCarSprites();
            RunTasks();
            gMain.state += 1;
        }
        5 => {
            if (*sCableCar).weather == WEATHER_VOLCANIC_ASH {
                gMain.state += 1;
            } else if !(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                .cast::<*mut Weather>()))
            .sprites
            .s2
            .ashSprites[0]
                .is_null()
            {
                for i in 0..NUM_ASH_SPRITES {
                    if !(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .ashSprites[i]
                        .is_null()
                    {
                        (*(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                            .cast::<*mut Weather>()))
                        .sprites
                        .s2
                        .ashSprites[i])
                            .oam
                            .set_priority(0);
                    }
                }
                gMain.state += 1;
            }
        }
        6 => {
            CopyToBgTilemapBufferRect_ChangePalette(
                1,
                (*sCableCar).treesTilemap as *mut c_void,
                0,
                17,
                32,
                15,
                17,
            );
            CopyToBgTilemapBufferRect_ChangePalette(
                2,
                (*sCableCar).bgMountainsTilemap as *mut c_void,
                0,
                0,
                30,
                20,
                17,
            );
            CopyToBgTilemapBufferRect_ChangePalette(
                3,
                (*sCableCar).pylonTopTilemap as *mut c_void,
                0,
                0,
                5,
                2,
                17,
            );
            CopyToBgTilemapBufferRect_ChangePalette(
                3,
                (*sCableCar).pylonPoleTilemap as *mut c_void,
                0,
                2,
                2,
                20,
                17,
            );
            gMain.state += 1;
        }
        7 => {
            InitGroundTilemapData(gSpecialVar_0x8004 as u8);
            CopyToBgTilemapBufferRect_ChangePalette(
                0,
                (*sCableCar).groundTilemap.at(72) as *mut c_void,
                0,
                14,
                12,
                3,
                17,
            );
            CopyToBgTilemapBufferRect_ChangePalette(
                0,
                (*sCableCar).groundTilemap.at(108) as *mut c_void,
                12,
                17,
                12,
                3,
                17,
            );
            CopyToBgTilemapBufferRect_ChangePalette(
                0,
                (*sCableCar).groundTilemap.at(144) as *mut c_void,
                24,
                20,
                12,
                3,
                17,
            );
            CopyToBgTilemapBufferRect_ChangePalette(
                0,
                (*sCableCar).groundTilemap as *mut c_void,
                0,
                17,
                12,
                3,
                17,
            );
            CopyToBgTilemapBufferRect_ChangePalette(
                0,
                (*sCableCar).groundTilemap.at(36) as *mut c_void,
                0,
                20,
                12,
                3,
                17,
            );
            CopyToBgTilemapBufferRect_ChangePalette(
                0,
                (*sCableCar).groundTilemap as *mut c_void,
                12,
                20,
                12,
                3,
                17,
            );
            CopyToBgTilemapBufferRect_ChangePalette(
                0,
                (*sCableCar).groundTilemap.at(36) as *mut c_void,
                12,
                23,
                12,
                3,
                17,
            );
            CopyToBgTilemapBufferRect_ChangePalette(
                0,
                (*sCableCar).groundTilemap as *mut c_void,
                24,
                23,
                12,
                3,
                17,
            );
            gMain.state += 1;
        }
        8 => {
            BeginNormalPaletteFade(PALETTES_ALL, 3, 16, 0, 0);
            FadeInNewBGM(MUS_CABLE_CAR, 1);
            SetBgRegs(TRUE);
            gMain.state += 1;
        }
        9 => {
            {
                let imeTemp: u16 = (67109384_usize as *mut u16).read_volatile();
                volatile_write(67109384_usize as *mut u16, 0);
                volatile_write(
                    0x4000200_usize as *mut u16,
                    (0x4000200_usize as *mut u16).read_volatile() | INTR_FLAG_VBLANK,
                );
                volatile_write(67109384_usize as *mut u16, imeTemp);
            }
            SetVBlankCallback(Some(VBlankCB_CableCar));
            SetMainCallback2(Some(CB2_CableCar));
            CreateTask(Some(Task_CableCar), 0);
            if gSpecialVar_0x8004 == 0 {
                (*sCableCar).bgTaskId = CreateTask(Some(Task_AnimateBgGoingUp), 1);
            } else {
                (*sCableCar).bgTaskId = CreateTask(Some(Task_AnimateBgGoingDown), 1);
            }
        }
        _ => {
            SetVBlankCallback(None);
            SetBgRegs(FALSE);
            ScanlineEffect_Stop();
            {
                let mut _dest: *mut c_void = VRAM as usize as *mut c_void;
                let mut _size: u32 = VRAM_SIZE;
                loop {
                    {
                        {
                            let mut tmp: u16 = 0;
                            volatile_write(&raw mut tmp, 0);
                            {
                                {
                                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x81000800);
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
                                let mut tmp: u16 = 0;
                                volatile_write(&raw mut tmp, 0);
                                {
                                    {
                                        let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                        volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                        volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                        volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
                                        let _ = (dmaRegs.at(2)).read_volatile();
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
            }
            {
                let mut _dest: *mut c_void = OAM as i32 as usize as *mut c_void;
                let mut _size: u32 = OAM_SIZE;
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
            {
                let mut _dest: *mut c_void = PLTT as i32 as usize as *mut c_void;
                let mut _size: u32 = PLTT_SIZE;
                {
                    {
                        let mut tmp: u16 = 0;
                        volatile_write(&raw mut tmp, 0);
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
            }
            sCableCar = AllocZeroed(16656) as *mut CableCar;
            gMain.state += 1;
        }
    }
}
pub(crate) unsafe fn CB2_CableCar() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
    MapMusicMain();
}
pub(crate) unsafe fn CB2_EndCableCar() {
    HideBg(0);
    HideBg(1);
    HideBg(2);
    HideBg(3);
    SetBgRegs(FALSE);
    gSpriteCoordOffsetX = 0;
    SetCurrentAndNextWeatherNoDelay(WEATHER_NONE);
    for i in 0..NUM_ASH_SPRITES {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sprites
            .s2
            .ashSprites[i] = null_mut();
    }
    ResetTasks();
    ResetSpriteData();
    ResetPaletteFade();
    UnsetBgTilemapBuffer(0);
    UnsetBgTilemapBuffer(1);
    UnsetBgTilemapBuffer(2);
    UnsetBgTilemapBuffer(3);
    ResetBgsAndClearDma3BusyFlags(0);
    (*sCableCar).pylonTopTilemap = null_mut();
    Free((*sCableCar).pylonPoleTilemap as *mut c_void);
    (*sCableCar).pylonPoleTilemap = null_mut();
    Free((*sCableCar).bgMountainsTilemap as *mut c_void);
    (*sCableCar).bgMountainsTilemap = null_mut();
    Free((*sCableCar).treesTilemap as *mut c_void);
    (*sCableCar).treesTilemap = null_mut();
    Free((*sCableCar).groundTilemap as *mut c_void);
    (*sCableCar).groundTilemap = null_mut();
    Free(sCableCar as *mut c_void);
    sCableCar = null_mut();
    {
        let mut _dest: *mut c_void = VRAM as usize as *mut c_void;
        let mut _size: u32 = VRAM_SIZE;
        loop {
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000800);
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
                        let mut tmp: u16 = 0;
                        volatile_write(&raw mut tmp, 0);
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
                break;
            }
        }
    }
    {
        let mut _dest: *mut c_void = OAM as i32 as usize as *mut c_void;
        let mut _size: u32 = OAM_SIZE;
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
    {
        let mut _dest: *mut c_void = PLTT as i32 as usize as *mut c_void;
        let mut _size: u32 = PLTT_SIZE;
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 0);
                {
                    {
                        let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                        volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                        volatile_write(dmaRegs.at(1), _dest as usize as u32);
                        volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
                        let _ = (dmaRegs.at(2)).read_volatile();
                    }
                }
            }
        }
    }
    WarpIntoMap();
    gFieldCallback = None;
    SetMainCallback2(Some(CB2_LoadMap));
}
pub(crate) unsafe fn Task_CableCar(taskId: u8) {
    let mut i: u8 = 0;
    (*sCableCar).timer += 1;
    match (*sCableCar).state {
        0 => {
            if (*sCableCar).timer == (*sCableCar).weatherDelay {
                SetNextWeather((*sCableCar).weather);
                (*sCableCar).state = 1;
            }
        }
        1 => {
            match (*sCableCar).weather {
                WEATHER_VOLCANIC_ASH => {
                    if !(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .ashSprites[0]
                        .is_null()
                        && (*(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                            .cast::<*mut Weather>()))
                        .sprites
                        .s2
                        .ashSprites[0])
                            .oam
                            .priority()
                            != 0
                    {
                        while i < NUM_ASH_SPRITES {
                            if !(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                                .cast::<*mut Weather>()))
                            .sprites
                            .s2
                            .ashSprites[i]
                                .is_null()
                            {
                                (*(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                                    .cast::<*mut Weather>()))
                                .sprites
                                .s2
                                .ashSprites[i])
                                    .oam
                                    .set_priority(0);
                            }
                            i += 1;
                        }
                        (*sCableCar).state = 2;
                    }
                }
                WEATHER_SUNNY => {
                    if (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .currWeather
                        == WEATHER_SUNNY
                    {
                        (*sCableCar).state = 2;
                    } else if (*sCableCar).timer as i32 >= (*sCableCar).weatherDelay as i32 + 8 {
                        while i < NUM_ASH_SPRITES {
                            if !(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                                .cast::<*mut Weather>()))
                            .sprites
                            .s2
                            .ashSprites[i]
                                .is_null()
                            {
                                (*(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).sprites.s2.ashSprites[i]).set_invisible(
                                (*(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).sprites.s2.ashSprites[i]).invisible() ^ 1,
                            );
                            }
                            i += 1;
                        }
                    }
                }
                _ => {}
            }
        }
        2 => {
            if (*sCableCar).timer == 570 {
                (*sCableCar).state = 3;
                BeginNormalPaletteFade(PALETTES_ALL, 3, 0, 16, 0);
                FadeOutBGM(4);
            }
        }
        3 => {
            if gPaletteFade.active() == 0 {
                (*sCableCar).state = STATE_END;
            }
        }
        STATE_END => {
            SetVBlankCallback(None);
            DestroyTask(taskId);
            DestroyTask((*sCableCar).bgTaskId);
            SetMainCallback2(Some(CB2_EndCableCar));
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_AnimateBgGoingUp(taskId: u8) {
    if (*sCableCar).state != STATE_END {
        (*sCableCar).bg3HorizontalOffset -= 1;
        if (*sCableCar).timer as i32 % 2 == 0 {
            (*sCableCar).bg3VerticalOffset -= 1;
        }
        if (*sCableCar).timer as i32 % 8 == 0 {
            (*sCableCar).bg1HorizontalOffset -= 1;
            (*sCableCar).bg1VerticalOffset -= 1;
        }
        match (*sCableCar).bg3HorizontalOffset {
            175 => {
                FillBgTilemapBufferRect(3, 0, 0, 22, 2, 10, 17);
            }
            40 => {
                FillBgTilemapBufferRect(3, 0, 3, 0, 2, 2, 17);
            }
            32 => {
                FillBgTilemapBufferRect(3, 0, 2, 0, 1, 2, 17);
            }
            16 => {
                CopyToBgTilemapBufferRect_ChangePalette(
                    3,
                    (*sCableCar).pylonTopTilemap as *mut c_void,
                    0,
                    0,
                    5,
                    2,
                    17,
                );
                CopyToBgTilemapBufferRect_ChangePalette(
                    3,
                    (*sCableCar).pylonPoleTilemap as *mut c_void,
                    0,
                    2,
                    2,
                    30,
                    17,
                );
                (*sCableCar).bg3VerticalOffset = 64;
            }
            _ => {}
        }
    }
    AnimateGroundGoingUp();
    gSpriteCoordOffsetX = ((gSpriteCoordOffsetX as i32 + 1) % 128) as i16;
}
pub(crate) unsafe fn Task_AnimateBgGoingDown(taskId: u8) {
    if (*sCableCar).state != STATE_END {
        (*sCableCar).bg3HorizontalOffset += 1;
        if (*sCableCar).timer as i32 % 2 == 0 {
            (*sCableCar).bg3VerticalOffset += 1;
        }
        if (*sCableCar).timer as i32 % 8 == 0 {
            (*sCableCar).bg1HorizontalOffset += 1;
            (*sCableCar).bg1VerticalOffset += 1;
        }
        match (*sCableCar).bg3HorizontalOffset {
            176 => {
                CopyToBgTilemapBufferRect_ChangePalette(
                    3,
                    (*sCableCar).pylonPoleTilemap as *mut c_void,
                    0,
                    2,
                    2,
                    30,
                    17,
                );
            }
            16 => {
                FillBgTilemapBufferRect(3, 0, 2, 0, 3, 2, 17);
                FillBgTilemapBufferRect(3, 0, 0, 22, 2, 10, 17);
                (*sCableCar).bg3VerticalOffset = 192;
            }
            32 => {
                FillBgTilemapBufferRect(3, *(*sCableCar).pylonTopTilemap.at(2), 2, 0, 1, 1, 17);
                FillBgTilemapBufferRect(3, *(*sCableCar).pylonTopTilemap.at(3), 3, 0, 1, 1, 17);
                FillBgTilemapBufferRect(3, *(*sCableCar).pylonTopTilemap.at(7), 2, 1, 1, 1, 17);
                FillBgTilemapBufferRect(3, *(*sCableCar).pylonTopTilemap.at(8), 3, 1, 1, 1, 17);
            }
            40 => {
                FillBgTilemapBufferRect(3, *(*sCableCar).pylonTopTilemap.at(4), 4, 0, 1, 1, 17);
                FillBgTilemapBufferRect(3, *(*sCableCar).pylonTopTilemap.at(9), 4, 1, 1, 1, 17);
            }
            _ => {}
        }
    }
    AnimateGroundGoingDown();
    if (*sCableCar).timer < (*sCableCar).weatherDelay {
        gSpriteCoordOffsetX = ((gSpriteCoordOffsetX as i32 + 247) % 248) as i16;
    } else {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .ashBaseSpritesX = (((*(*(&raw const crate::data::field_weather::gWeatherPtr)
            .cast::<*mut Weather>()))
        .ashBaseSpritesX as i32
            + 247)
            % 248) as u16;
    }
}
pub(crate) unsafe fn VBlankCB_CableCar() {
    CopyBgTilemapBufferToVram(0);
    CopyBgTilemapBufferToVram(3);
    SetGpuReg(REG_OFFSET_BG3HOFS, (*sCableCar).bg3HorizontalOffset as u16);
    SetGpuReg(REG_OFFSET_BG3VOFS, (*sCableCar).bg3VerticalOffset as u16);
    SetGpuReg(REG_OFFSET_BG1HOFS, (*sCableCar).bg1HorizontalOffset as u16);
    SetGpuReg(REG_OFFSET_BG1VOFS, (*sCableCar).bg1VerticalOffset as u16);
    SetGpuReg(REG_OFFSET_BG0HOFS, (*sCableCar).bg0HorizontalOffset as u16);
    SetGpuReg(REG_OFFSET_BG0VOFS, (*sCableCar).bg0VerticalOffset as u16);
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) fn SpriteCB_Cable(sprite: *mut Sprite) {}
pub(crate) unsafe fn SpriteCB_CableCar(sprite: *mut Sprite) {
    if (*sCableCar).state != STATE_END {
        if gSpecialVar_0x8004 == 0 {
            (*sprite).x = (*sprite).data[sXPos]
                - (0_f32
                    * ({
                        let v1: i16 = (*sCableCar).timer as i16;
                        let mut f = v1 as f32;
                        if v1 < 0 {
                            f += 65536.0;
                        }
                        f
                    })) as u8 as i16;
            (*sprite).y = (*sprite).data[sYPos]
                - (0_f32
                    * ({
                        let v2: i16 = (*sCableCar).timer as i16;
                        let mut f = v2 as f32;
                        if v2 < 0 {
                            f += 65536.0;
                        }
                        f
                    })) as u8 as i16;
        } else {
            (*sprite).x = (*sprite).data[sXPos]
                + (0_f32
                    * ({
                        let v3: i16 = (*sCableCar).timer as i16;
                        let mut f = v3 as f32;
                        if v3 < 0 {
                            f += 65536.0;
                        }
                        f
                    })) as u8 as i16;
            (*sprite).y = (*sprite).data[sYPos]
                + (0_f32
                    * ({
                        let v4: i16 = (*sCableCar).timer as i16;
                        let mut f = v4 as f32;
                        if v4 < 0 {
                            f += 65536.0;
                        }
                        f
                    })) as u8 as i16;
        }
    }
}
pub(crate) unsafe fn SpriteCB_Player(sprite: *mut Sprite) {
    if (*sCableCar).state != STATE_END {
        if gSpecialVar_0x8004 == 0 {
            (*sprite).x = (*sprite).data[sXPos]
                - (0_f32
                    * ({
                        let v1: i16 = (*sCableCar).timer as i16;
                        let mut f = v1 as f32;
                        if v1 < 0 {
                            f += 65536.0;
                        }
                        f
                    })) as u8 as i16;
            (*sprite).y = (*sprite).data[sYPos]
                - (0_f32
                    * ({
                        let v2: i16 = (*sCableCar).timer as i16;
                        let mut f = v2 as f32;
                        if v2 < 0 {
                            f += 65536.0;
                        }
                        f
                    })) as u8 as i16;
        } else {
            (*sprite).x = (*sprite).data[sXPos]
                + (0_f32
                    * ({
                        let v3: i16 = (*sCableCar).timer as i16;
                        let mut f = v3 as f32;
                        if v3 < 0 {
                            f += 65536.0;
                        }
                        f
                    })) as u8 as i16;
            (*sprite).y = (*sprite).data[sYPos]
                + (0_f32
                    * ({
                        let v4: i16 = (*sCableCar).timer as i16;
                        let mut f = v4 as f32;
                        if v4 < 0 {
                            f += 65536.0;
                        }
                        f
                    })) as u8 as i16;
        }
        match (*sprite).data[sState] {
            0 => {
                (*sprite).y2 = 17;
                if ({
                    let t5 = (*sprite).data[3];
                    (*sprite).data[3] += 1;
                    t5
                }) > 9
                {
                    (*sprite).data[3] = 0;
                    (*sprite).data[sState] += 1;
                }
            }
            _ => {
                (*sprite).y2 = 16;
                if ({
                    let t6 = (*sprite).data[3];
                    (*sprite).data[3] += 1;
                    t6
                }) > 9
                {
                    (*sprite).data[3] = 0;
                    (*sprite).data[sState] = 0;
                }
            }
        }
    }
}
pub(crate) unsafe fn SpriteCB_HikerGoingUp(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        (*sprite).x += 2 * (*sprite).centerToCornerVecX as i16;
        (*sprite).y += 16 + (*sprite).centerToCornerVecY as i16;
    }
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) >= (*sprite).data[sDelay]
    {
        match (*sprite).data[sSameDir] {
            0 => {
                (*sprite).x += 1;
                if (*sprite).data[0] % 4 == 0 {
                    (*sprite).y += 1;
                }
            }
            1 if (*sprite).data[0] % 2 != 0 => {
                (*sprite).x += 1;
                if (*sprite).x % 4 == 0 {
                    (*sprite).y += 1;
                }
            }
            _ => {}
        }
        if (*sprite).y > DISPLAY_HEIGHT as i16 {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe fn SpriteCB_HikerGoingDown(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        (*sprite).y += 16 + (*sprite).centerToCornerVecY as i16;
    }
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) >= (*sprite).data[sDelay]
    {
        match (*sprite).data[sSameDir] {
            0 => {
                (*sprite).x -= 1;
                if (*sprite).data[0] % 4 == 0 {
                    (*sprite).y -= 1;
                }
            }
            1 if (*sprite).data[0] % 2 != 0 => {
                (*sprite).x -= 1;
                if (*sprite).x % 4 == 0 {
                    (*sprite).y -= 1;
                }
            }
            _ => {}
        }
        if (*sprite).y < 80 {
            DestroySprite(sprite);
        }
    }
}
unsafe fn SetBgRegs(active: u8) {
    match active {
        TRUE => {
            SetGpuReg(REG_OFFSET_WININ, 0);
            SetGpuReg(REG_OFFSET_WINOUT, 0);
            SetGpuReg(REG_OFFSET_WIN0H, 0);
            SetGpuReg(REG_OFFSET_WIN1H, 0);
            SetGpuReg(REG_OFFSET_WIN0V, 0);
            SetGpuReg(REG_OFFSET_WIN1V, 0);
            if gSpecialVar_0x8004 == 0 {
                (*sCableCar).bg3HorizontalOffset = 176;
                (*sCableCar).bg3VerticalOffset = 16;
                (*sCableCar).bg1HorizontalOffset = 0;
                (*sCableCar).bg1VerticalOffset = 80;
                (*sCableCar).bg0VerticalOffset = 0;
                (*sCableCar).bg0VerticalOffset = 0;
            } else {
                (*sCableCar).bg3HorizontalOffset = 96;
                (*sCableCar).bg3VerticalOffset = 232;
                (*sCableCar).bg1HorizontalOffset = 0;
                (*sCableCar).bg1VerticalOffset = 4;
                (*sCableCar).bg0VerticalOffset = 0;
                (*sCableCar).bg0VerticalOffset = 0;
            }
            SetGpuReg(REG_OFFSET_BG3HOFS, (*sCableCar).bg3HorizontalOffset as u16);
            SetGpuReg(REG_OFFSET_BG3VOFS, (*sCableCar).bg3VerticalOffset as u16);
            SetGpuReg(REG_OFFSET_BG2HOFS, 0);
            SetGpuReg(REG_OFFSET_BG2VOFS, 0);
            SetGpuReg(REG_OFFSET_BG1HOFS, (*sCableCar).bg1HorizontalOffset as u16);
            SetGpuReg(REG_OFFSET_BG1VOFS, (*sCableCar).bg1VerticalOffset as u16);
            SetGpuReg(REG_OFFSET_BG0HOFS, (*sCableCar).bg0HorizontalOffset as u16);
            SetGpuReg(REG_OFFSET_BG0VOFS, (*sCableCar).bg0VerticalOffset as u16);
            SetGpuReg(REG_OFFSET_DISPCNT, 4160);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            ShowBg(3);
            SetGpuReg(REG_OFFSET_BLDCNT, BLDCNT_TGT2_ALL);
        }
        _ => {
            SetGpuReg(REG_OFFSET_WININ, 0);
            SetGpuReg(REG_OFFSET_WINOUT, 0);
            SetGpuReg(REG_OFFSET_WIN0H, 0);
            SetGpuReg(REG_OFFSET_WIN1H, 0);
            SetGpuReg(REG_OFFSET_WIN0V, 0);
            SetGpuReg(REG_OFFSET_WIN1V, 0);
            SetGpuReg(0x0, 0);
            SetGpuReg(REG_OFFSET_BG3CNT, 0);
            SetGpuReg(REG_OFFSET_BG2CNT, 0);
            SetGpuReg(REG_OFFSET_BG1CNT, 0);
            SetGpuReg(REG_OFFSET_BG0CNT, 0);
            SetGpuReg(REG_OFFSET_BG3HOFS, 0);
            SetGpuReg(REG_OFFSET_BG3VOFS, 0);
            SetGpuReg(REG_OFFSET_BG2HOFS, 0);
            SetGpuReg(REG_OFFSET_BG2VOFS, 0);
            SetGpuReg(REG_OFFSET_BG1HOFS, 0);
            SetGpuReg(REG_OFFSET_BG1VOFS, 0);
            SetGpuReg(REG_OFFSET_BG0HOFS, 0);
            SetGpuReg(REG_OFFSET_BG0VOFS, 0);
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
        }
    }
}
unsafe fn CreateCableCarSprites() {
    let mut spriteId: u8 = 0;
    let mut playerGraphicsIds: CArray<u8, 2> = zeroed();
    playerGraphicsIds[0] = OBJ_EVENT_GFX_RIVAL_BRENDAN_NORMAL;
    playerGraphicsIds[1] = OBJ_EVENT_GFX_RIVAL_MAY_NORMAL;
    let rval: u16 = Random();
    let hikerGraphicsIds: CArray<u8, 4> = CArray([55, 31, 32, 98]);
    let mut hikerCoords: CArray<CArray<i16, 2>, 2> = zeroed();
    hikerCoords[0][0] = 0;
    hikerCoords[0][1] = 80;
    hikerCoords[1][0] = 240;
    hikerCoords[1][1] = 146;
    let hikerMovementDelayTable: CArray<u8, 4> = CArray([0, 60, 120, 170]);
    let mut hikerCallbacks: CArray<Option<unsafe fn(*mut Sprite)>, 2> = zeroed();
    hikerCallbacks[0] = Some(SpriteCB_HikerGoingUp);
    hikerCallbacks[1] = Some(SpriteCB_HikerGoingDown);
    match *(&raw const crate::ffi::gSpecialVar_0x8004)
        .cast::<u16>()
        .cast_mut()
    {
        1 => {
            CopyToBgTilemapBufferRect_ChangePalette(
                0,
                (*sCableCar).groundTilemap.at(36) as *mut c_void,
                24,
                26,
                12,
                3,
                17,
            );
            spriteId = CreateObjectGraphicsSprite(
                playerGraphicsIds[(*gSaveBlock2Ptr).playerGender] as u16,
                Some(SpriteCB_Player),
                128,
                39,
                102,
            );
            if spriteId != MAX_SPRITES {
                gSprites[spriteId].oam.set_priority(2);
                gSprites[spriteId].x2 = 8;
                gSprites[spriteId].y2 = 16;
                gSprites[spriteId].data[sXPos] = 128;
                gSprites[spriteId].data[1] = 39;
            }
            spriteId = CreateSprite(
                (&raw const sSpriteTemplates_CableCar[0]).cast_mut(),
                104,
                9,
                0x67,
            );
            gSprites[spriteId].x2 = {
                gSprites[spriteId].y2 = 32;
                gSprites[spriteId].y2
            };
            gSprites[spriteId].data[sXPos] = 104;
            gSprites[spriteId].data[1] = 9;
            spriteId = CreateSprite(
                (&raw const sSpriteTemplates_CableCar[1]).cast_mut(),
                128,
                65,
                0x65,
            );
            gSprites[spriteId].x2 = 8;
            gSprites[spriteId].y2 = 4;
            gSprites[spriteId].data[sXPos] = 128;
            gSprites[spriteId].data[1] = 65;
            (*sCableCar).weather = WEATHER_SUNNY;
            (*sCableCar).weatherDelay = 265;
            SetCurrentAndNextWeatherNoDelay(WEATHER_VOLCANIC_ASH);
        }
        _ => {
            spriteId = CreateObjectGraphicsSprite(
                playerGraphicsIds[(*gSaveBlock2Ptr).playerGender] as u16,
                Some(SpriteCB_Player),
                200,
                73,
                102,
            );
            if spriteId != MAX_SPRITES {
                gSprites[spriteId].oam.set_priority(2);
                gSprites[spriteId].x2 = 8;
                gSprites[spriteId].y2 = 16;
                gSprites[spriteId].data[sXPos] = 200;
                gSprites[spriteId].data[1] = 73;
            }
            spriteId = CreateSprite(
                (&raw const sSpriteTemplates_CableCar[0]).cast_mut(),
                176,
                43,
                0x67,
            );
            gSprites[spriteId].x2 = {
                gSprites[spriteId].y2 = 32;
                gSprites[spriteId].y2
            };
            gSprites[spriteId].data[sXPos] = 176;
            gSprites[spriteId].data[1] = 43;
            spriteId = CreateSprite(
                (&raw const sSpriteTemplates_CableCar[1]).cast_mut(),
                200,
                99,
                0x65,
            );
            gSprites[spriteId].x2 = 8;
            gSprites[spriteId].y2 = 4;
            gSprites[spriteId].data[sXPos] = 200;
            gSprites[spriteId].data[1] = 99;
            (*sCableCar).weather = WEATHER_VOLCANIC_ASH;
            (*sCableCar).weatherDelay = 350;
            SetCurrentAndNextWeatherNoDelay(WEATHER_SUNNY);
        }
    }
    for i in 0..9u8 {
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_Cable).cast_mut(),
            16 * i as i16 + 96,
            8 * i as i16 - 8,
            0x68,
        );
        gSprites[spriteId].x2 = 8;
        gSprites[spriteId].y2 = 8;
    }
    if rval as i32 % 64 == 0 {
        spriteId = CreateObjectGraphicsSprite(
            hikerGraphicsIds[rval % 3] as u16,
            hikerCallbacks[*(&raw const crate::ffi::gSpecialVar_0x8004)
                .cast::<u16>()
                .cast_mut()],
            hikerCoords[*(&raw const crate::ffi::gSpecialVar_0x8004)
                .cast::<u16>()
                .cast_mut()][0],
            hikerCoords[*(&raw const crate::ffi::gSpecialVar_0x8004)
                .cast::<u16>()
                .cast_mut()][1],
            106,
        );
        if spriteId != MAX_SPRITES {
            gSprites[spriteId].oam.set_priority(2);
            gSprites[spriteId].x2 = -(gSprites[spriteId].centerToCornerVecX as i16);
            gSprites[spriteId].y2 = -(gSprites[spriteId].centerToCornerVecY as i16);
            if gSpecialVar_0x8004 == 0 {
                if rval as i32 % 2 != 0 {
                    StartSpriteAnim(&raw mut gSprites[spriteId], ANIM_STD_GO_WEST);
                    gSprites[spriteId].data[1] = TRUE as i16;
                    gSprites[spriteId].y += 2;
                } else {
                    StartSpriteAnim(&raw mut gSprites[spriteId], ANIM_STD_GO_EAST);
                    gSprites[spriteId].data[1] = FALSE as i16;
                }
            } else {
                if rval as i32 % 2 != 0 {
                    StartSpriteAnim(&raw mut gSprites[spriteId], ANIM_STD_GO_EAST);
                    gSprites[spriteId].data[1] = TRUE as i16;
                    gSprites[spriteId].y += 2;
                } else {
                    StartSpriteAnim(&raw mut gSprites[spriteId], ANIM_STD_GO_WEST);
                    gSprites[spriteId].data[1] = FALSE as i16;
                }
            }
            gSprites[spriteId].data[sDelay] = hikerMovementDelayTable[rval % 4] as i16;
        }
    }
}
unsafe fn BufferNextGroundSegment() {
    let mut k: u8 = 0;
    let mut offset: u8 = 0x24 * ((*sCableCar).groundTilemapOffset + 2);
    for i in 0..3u8 {
        for j in 0..12u8 {
            (*sCableCar).groundTileBuffer[i][j] = *(*sCableCar).groundTilemap.at({
                let t1 = offset;
                offset += 1;
                t1
            });
            (*sCableCar).groundTileBuffer[i as i32 + 3][j] = *(*sCableCar).groundTilemap.at(k);
            (*sCableCar).groundTileBuffer[i as i32 + 6][j] =
                *(*sCableCar).groundTilemap.at(36).at(k);
            k += 1;
        }
    }
    (*sCableCar).groundTilemapOffset = (((*sCableCar).groundTilemapOffset as i32 + 1) % 3) as u8;
}
unsafe fn AnimateGroundGoingUp() {
    (*sCableCar).groundTimer = (((*sCableCar).groundTimer as i32 + 1) % 96) as u8;
    (*sCableCar).bg0HorizontalOffset = (*sCableCar).groundXBase - (*sCableCar).groundXOffset;
    (*sCableCar).bg0VerticalOffset = (*sCableCar).groundYBase - (*sCableCar).groundYOffset;
    (*sCableCar).groundXOffset += 1;
    if (*sCableCar).groundXOffset as i32 % 4 == 0 {
        (*sCableCar).groundYOffset += 1;
    }
    if (*sCableCar).groundXOffset > 16 {
        DrawNextGroundSegmentGoingUp();
    }
}
unsafe fn AnimateGroundGoingDown() {
    (*sCableCar).groundTimer = (((*sCableCar).groundTimer as i32 + 1) % 96) as u8;
    (*sCableCar).bg0HorizontalOffset = (*sCableCar).groundXBase + (*sCableCar).groundXOffset;
    (*sCableCar).bg0VerticalOffset = (*sCableCar).groundYBase + (*sCableCar).groundYOffset;
    (*sCableCar).groundXOffset += 1;
    if (*sCableCar).groundXOffset as i32 % 4 == 0 {
        (*sCableCar).groundYOffset += 1;
    }
    if (*sCableCar).groundXOffset > 16 {
        DrawNextGroundSegmentGoingDown();
    }
}
unsafe fn DrawNextGroundSegmentGoingUp() {
    (*sCableCar).groundXOffset = {
        (*sCableCar).groundYOffset = 0;
        (*sCableCar).groundYOffset
    };
    (*sCableCar).groundXBase = (*sCableCar).bg0HorizontalOffset;
    (*sCableCar).groundYBase = (*sCableCar).bg0VerticalOffset;
    (*sCableCar).groundSegmentXStart = (((*sCableCar).groundSegmentXStart as i32 + 30) % 32) as u8;
    (*sCableCar).groundTileIdx -= 2;
    sGroundSegmentY_Up.set((((*sCableCar).groundSegmentYStart as i32 + 23) % 32) as u8);
    for i in 0..9u8 {
        sGroundX_Up.set((*sCableCar).groundSegmentXStart);
        sGroundY_Up.set(((sGroundSegmentY_Up.get() as i32 + i as i32) % 32) as u8);
        FillBgTilemapBufferRect(
            0,
            (*sCableCar).groundTileBuffer[i][(*sCableCar).groundTileIdx],
            sGroundX_Up.get(),
            sGroundY_Up.get(),
            1,
            1,
            17,
        );
        sGroundX_Up.set(((sGroundX_Up.get() as i32 + 1) % 32) as u8);
        FillBgTilemapBufferRect(
            0,
            (*sCableCar).groundTileBuffer[i][(*sCableCar).groundTileIdx as i32 + 1],
            sGroundX_Up.get(),
            sGroundY_Up.get(),
            1,
            1,
            17,
        );
    }
    sGroundX_Up.set((((*sCableCar).groundSegmentXStart as i32 + 30) % 32) as u8);
    FillBgTilemapBufferRect(0, 0, sGroundX_Up.get(), 0, 2, 32, 17);
    if (*sCableCar).groundTileIdx == 0 {
        (*sCableCar).groundSegmentYStart =
            (((*sCableCar).groundSegmentYStart as i32 + 29) % 32) as u8;
        (*sCableCar).groundTileIdx = 12;
        BufferNextGroundSegment();
        sGroundX_Up.set((((*sCableCar).groundSegmentYStart as i32 + 1) % 32) as u8);
        FillBgTilemapBufferRect(0, 0, 0, sGroundX_Up.get(), 32, 9, 17);
    }
}
unsafe fn DrawNextGroundSegmentGoingDown() {
    (*sCableCar).groundXOffset = {
        (*sCableCar).groundYOffset = 0;
        (*sCableCar).groundYOffset
    };
    (*sCableCar).groundXBase = (*sCableCar).bg0HorizontalOffset;
    (*sCableCar).groundYBase = (*sCableCar).bg0VerticalOffset;
    (*sCableCar).groundSegmentXStart = (((*sCableCar).groundSegmentXStart as i32 + 2) % 32) as u8;
    (*sCableCar).groundTileIdx += 2;
    sGroundSegmentY_Down.set((*sCableCar).groundSegmentYStart);
    for i in 0..9u8 {
        sGroundX_Down.set((*sCableCar).groundSegmentXStart);
        sGroundY_Down.set(((sGroundSegmentY_Down.get() as i32 + i as i32) % 32) as u8);
        FillBgTilemapBufferRect(
            0,
            (*sCableCar).groundTileBuffer[i][(*sCableCar).groundTileIdx],
            sGroundX_Down.get(),
            sGroundY_Down.get(),
            1,
            1,
            17,
        );
        sGroundX_Down.set(((sGroundX_Down.get() as i32 + 1) % 32) as u8);
        FillBgTilemapBufferRect(
            0,
            (*sCableCar).groundTileBuffer[i][(*sCableCar).groundTileIdx as i32 + 1],
            sGroundX_Down.get(),
            sGroundY_Down.get(),
            1,
            1,
            17,
        );
    }
    sGroundY_Down.set((((*sCableCar).groundSegmentYStart as i32 + 23) % 32) as u8);
    FillBgTilemapBufferRect(
        0,
        0,
        (*sCableCar).groundSegmentXStart,
        sGroundY_Down.get(),
        2,
        9,
        17,
    );
    if (*sCableCar).groundTileIdx == 10 {
        (*sCableCar).groundSegmentYStart =
            (((*sCableCar).groundSegmentYStart as i32 + 3) % 32) as u8;
        (*sCableCar).groundTileIdx = 254;
        BufferNextGroundSegment();
    }
}
unsafe fn InitGroundTilemapData(goingDown: u8) {
    match goingDown {
        TRUE => {
            (*sCableCar).groundTilemapOffset = 2;
            (*sCableCar).groundSegmentXStart = 28;
            (*sCableCar).groundSegmentYStart = 20;
            (*sCableCar).groundTileIdx = 4;
            BufferNextGroundSegment();
            DrawNextGroundSegmentGoingDown();
        }
        _ => {
            (*sCableCar).groundTilemapOffset = 2;
            (*sCableCar).groundSegmentXStart = 0;
            (*sCableCar).groundSegmentYStart = 20;
            (*sCableCar).groundTileIdx = 12;
            BufferNextGroundSegment();
            DrawNextGroundSegmentGoingUp();
        }
    }
    (*sCableCar).groundTimer = 0;
}
