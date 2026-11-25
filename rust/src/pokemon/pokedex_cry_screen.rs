//! Translated from `src/pokedex_cry_screen.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments
)]

use crate::agb_main::gPcmDmaCounter;
use crate::bg::ChangeBgX;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::m4a::{gMPlayInfo_BGM, gSoundInfo};
use crate::palette::LoadPalette;
use crate::sound::{IsCryPlaying, PlayCry_NormalNoDucking, StopCry};
use crate::sprite::gSprites;
use crate::sprite::{FreeSpritePaletteByTag, GetSpritePaletteTagByPaletteNum, SetOamMatrix};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{CopyWindowToVram, GetWindowAttribute};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CopyToWindowPixelBuffer` with this module's view of its types.
#[inline]
fn CopyToWindowPixelBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16) {
    unsafe {
        crate::window::CopyToWindowPixelBuffer(a0, a1 as _, a2, a3);
    }
}
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
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
/// `LoadSpritePalettes` with this module's view of its types.
#[inline]
fn LoadSpritePalettes(a0: *mut SpritePalette) {
    unsafe {
        crate::sprite::LoadSpritePalettes(a0 as _);
    }
}
/// `LoadSpriteSheets` with this module's view of its types.
#[inline]
fn LoadSpriteSheets(a0: *mut SpriteSheet) {
    unsafe {
        crate::sprite::LoadSpriteSheets(a0 as _);
    }
}
// Data tables (translate with cdata.py): sCryMeterNeedle_Pal sCryMeterNeedle_Gfx sCryMeter_Tilemap sCryMeter_Pal sCryMeter_Gfx sWaveformOffsets sCryScreenBg_Pal sCryScreenBg_Gfx sWaveformTileDataNybbleMasks sWaveformColor sSpriteAnim_CryMeterNeedle sSpriteAnimTable_CryMeterNeedle sOamData_CryMeterNeedle sCryMeterNeedleSpriteTemplate sCryMeterNeedleSpriteSheets sCryMeterNeedleSpritePalettes

/// `struct PokedexCryScreen`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct PokedexCryScreen {
    pub cryWaveformBuffer: CArray<u8, 16>,
    pub cryState: u8,
    pub playhead: u8,
    pub waveformPreviousY: u8,
    pub unk: u16,
    pub playStartPos: u8,
    pub species: u16,
    pub cryOverrideCountdown: u8,
    pub cryRepeatDelay: u8,
}

unsafe impl Sync for PokedexCryScreen {}

/// `struct PokedexCryMeterNeedle`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct PokedexCryMeterNeedle {
    pub rotation: i8,
    pub targetRotation: i8,
    pub moveIncrement: u8,
    pub spriteId: u16,
}

unsafe impl Sync for PokedexCryMeterNeedle {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PokedexCryScreen>() == 28);
    assert!(offset_of!(PokedexCryScreen, cryWaveformBuffer) == 0);
    assert!(offset_of!(PokedexCryScreen, cryState) == 16);
    assert!(offset_of!(PokedexCryScreen, playhead) == 17);
    assert!(offset_of!(PokedexCryScreen, waveformPreviousY) == 18);
    assert!(offset_of!(PokedexCryScreen, unk) == 20);
    assert!(offset_of!(PokedexCryScreen, playStartPos) == 22);
    assert!(offset_of!(PokedexCryScreen, species) == 24);
    assert!(offset_of!(PokedexCryScreen, cryOverrideCountdown) == 26);
    assert!(offset_of!(PokedexCryScreen, cryRepeatDelay) == 27);
    assert!(size_of::<PokedexCryMeterNeedle>() == 8);
    assert!(offset_of!(PokedexCryMeterNeedle, rotation) == 0);
    assert!(offset_of!(PokedexCryMeterNeedle, targetRotation) == 1);
    assert!(offset_of!(PokedexCryMeterNeedle, moveIncrement) == 2);
    assert!(offset_of!(PokedexCryMeterNeedle, spriteId) == 4);
};

const MAX_NEEDLE_POS: i32 = -32;
const MIN_NEEDLE_POS: i8 = 32;
const NEEDLE_MOVE_INCREMENT: u8 = 5;
const WAVEFORM_WINDOW_HEIGHT: i32 = 56;

static sCryMeterNeedleSpritePalettes: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::pokedex_cry_screen::sCryMeterNeedleSpritePalettes).cast());
static sCryMeterNeedleSpriteSheets: Table<CArray<SpriteSheet, 2>> =
    Table((&raw const crate::data::pokedex_cry_screen::sCryMeterNeedleSpriteSheets).cast());
static sCryMeterNeedleSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokedex_cry_screen::sCryMeterNeedleSpriteTemplate).cast());
static sCryMeter_Gfx: Table<CArray<u8, 824>> =
    Table((&raw const crate::data::pokedex_cry_screen::sCryMeter_Gfx).cast());
static sCryMeter_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokedex_cry_screen::sCryMeter_Pal).cast());
static sCryScreenBg_Gfx: Table<CArray<u8, 32>> =
    Table((&raw const crate::data::pokedex_cry_screen::sCryScreenBg_Gfx).cast());
static sCryScreenBg_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokedex_cry_screen::sCryScreenBg_Pal).cast());
static sWaveformColor: Table<CArray<CArray<u8, 16>, 2>> =
    Table((&raw const crate::data::pokedex_cry_screen::sWaveformColor).cast());
static sWaveformOffsets: Table<CArray<CArray<u16, 72>, 8>> =
    Table((&raw const crate::data::pokedex_cry_screen::sWaveformOffsets).cast());
static sWaveformTileDataNybbleMasks: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::pokedex_cry_screen::sWaveformTileDataNybbleMasks).cast());

#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gDexCryScreenState: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sDexCryScreen: crate::global::Global<*mut PokedexCryScreen> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sCryWaveformWindowTiledata: crate::global::Global<*mut u8> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sCryMeterNeedle: crate::global::Global<*mut PokedexCryMeterNeedle> =
    crate::global::Global::new(null_mut());

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `ObjAffineSet` with this module's view of its types.
#[inline]
fn ObjAffineSet(a0: *mut ObjAffineSrcData, a1: *mut c_void, a2: i32, a3: i32) {
    unsafe {
        crate::syscall::ObjAffineSet(a0 as _, a1 as _, a2, a3);
    }
}

pub unsafe fn LoadCryWaveformWindow(window: *mut CryScreenWindow, windowId: u8) -> u8 {
    let mut i: u8 = 0;
    let mut finished: u8 = FALSE;
    match gDexCryScreenState.get() {
        0 => {
            if (*sDexCryScreen.as_ptr()).is_null() {
                crate::gl!(sDexCryScreen = AllocZeroed(28) as *mut PokedexCryScreen);
                crate::gl!(
                    sCryWaveformWindowTiledata =
                        GetWindowAttribute(windowId, WINDOW_TILE_DATA) as usize as *mut u8
                );
            }
            (*crate::gl!(sDexCryScreen)).unk = (*window).unk0;
            (*crate::gl!(sDexCryScreen)).playStartPos = (*window).yPos;
            (*crate::gl!(sDexCryScreen)).cryOverrideCountdown = 0;
            (*crate::gl!(sDexCryScreen)).cryRepeatDelay = 0;
            (*crate::gl!(sDexCryScreen)).cryState = 0;
            (*crate::gl!(sDexCryScreen)).waveformPreviousY = 28;
            (*crate::gl!(sDexCryScreen)).playhead = 0;
            ShiftWaveformOver(windowId, -8 * (*window).xPos as i16, TRUE);
            for i in 0..224u8 {
                CopyToWindowPixelBuffer(
                    windowId,
                    sCryScreenBg_Gfx.as_ptr().cast_mut() as *mut c_void,
                    32,
                    i as u16,
                );
            }
            gDexCryScreenState.set(gDexCryScreenState.get() + (1));
        }
        1 => {
            i = 0;
            while (i as i32) < (*crate::gl!(sDexCryScreen)).playStartPos as i32 * 8 {
                DrawWaveformSegment(i, 0);
                i += 1;
            }
            gDexCryScreenState.set(gDexCryScreenState.get() + (1));
        }
        2 => {
            DrawWaveformWindow(windowId);
            LoadPalette(
                sCryScreenBg_Pal.as_ptr().cast_mut() as *mut c_void,
                (*window).paletteNo as u16 * 16,
                32,
            );
            finished = TRUE;
        }
        _ => {}
    }
    finished
}
pub unsafe fn UpdateCryWaveformWindow(windowId: u8) {
    DrawWaveformWindow(windowId);
    AdvancePlayhead(windowId);
    if (*crate::gl!(sDexCryScreen)).cryRepeatDelay != 0 {
        (*crate::gl!(sDexCryScreen)).cryRepeatDelay -= 1;
    }
    if (*crate::gl!(sDexCryScreen)).cryOverrideCountdown != 0 {
        (*crate::gl!(sDexCryScreen)).cryOverrideCountdown -= 1;
        if (*crate::gl!(sDexCryScreen)).cryOverrideCountdown == 0 {
            PlayCryScreenCry((*crate::gl!(sDexCryScreen)).species);
            DrawWaveformFlatline();
            return;
        }
    }
    if (*crate::gl!(sDexCryScreen)).cryState == 0 {
        DrawWaveformFlatline();
        return;
    }
    if (*crate::gl!(sDexCryScreen)).cryState == 1 {
        BufferCryWaveformSegment();
    } else if (*crate::gl!(sDexCryScreen)).cryState > 8 {
        if IsCryPlaying() == 0 {
            DrawWaveformFlatline();
            (*crate::gl!(sDexCryScreen)).cryState = 0;
            return;
        }
        BufferCryWaveformSegment();
        (*crate::gl!(sDexCryScreen)).cryState = 1;
    }
    let waveformIdx: u8 = 2 * ((*crate::gl!(sDexCryScreen)).cryState - 1);
    DrawWaveformSegment(
        (*crate::gl!(sDexCryScreen)).playStartPos * 8 + (*crate::gl!(sDexCryScreen)).playhead - 2,
        (*crate::gl!(sDexCryScreen)).cryWaveformBuffer[waveformIdx],
    );
    DrawWaveformSegment(
        (*crate::gl!(sDexCryScreen)).playStartPos * 8 + (*crate::gl!(sDexCryScreen)).playhead - 1,
        (*crate::gl!(sDexCryScreen)).cryWaveformBuffer[waveformIdx as i32 + 1],
    );
    (*crate::gl!(sDexCryScreen)).cryState += 1;
}
pub unsafe fn CryScreenPlayButton(species: u16) {
    if crate::gl!(gMPlayInfo_BGM.status) & MUSICPLAYER_STATUS_PAUSE != 0
        && (*crate::gl!(sDexCryScreen)).cryOverrideCountdown == 0
        && (*crate::gl!(sDexCryScreen)).cryRepeatDelay == 0
    {
        (*crate::gl!(sDexCryScreen)).cryRepeatDelay = 4;
        if IsCryPlaying() == TRUE {
            StopCry();
            (*crate::gl!(sDexCryScreen)).species = species;
            (*crate::gl!(sDexCryScreen)).cryOverrideCountdown = 2;
        } else {
            PlayCryScreenCry(species);
        }
    }
}
unsafe fn PlayCryScreenCry(species: u16) {
    PlayCry_NormalNoDucking(species, 0, CRY_VOLUME_RS, CRY_PRIORITY_NORMAL);
    (*crate::gl!(sDexCryScreen)).cryState = 1;
}
unsafe fn BufferCryWaveformSegment() {
    let mut baseBuffer: *mut i8 = null_mut();
    if gPcmDmaCounter.get() < 2 {
        baseBuffer = (*gSoundInfo.as_ptr()).pcmBuffer.as_mut_ptr();
    } else {
        baseBuffer = (*gSoundInfo.as_ptr())
            .pcmBuffer
            .as_mut_ptr()
            .at(
                (crate::gl!(gSoundInfo.pcmDmaPeriod) as i32 + 1 - gPcmDmaCounter.get() as i32)
                    * crate::gl!(gSoundInfo.pcmSamplesPerVBlank),
            );
    }
    let buffer: *mut i8 = baseBuffer.at(1584);
    for i in 0..16u8 {
        (*crate::gl!(sDexCryScreen)).cryWaveformBuffer[i] = *buffer.at(i as i32 * 2) as u8 * 2;
    }
}
unsafe fn DrawWaveformFlatline() {
    DrawWaveformSegment(
        (*crate::gl!(sDexCryScreen)).playStartPos * 8 + (*crate::gl!(sDexCryScreen)).playhead - 2,
        0,
    );
    DrawWaveformSegment(
        (*crate::gl!(sDexCryScreen)).playStartPos * 8 + (*crate::gl!(sDexCryScreen)).playhead - 1,
        0,
    );
}
unsafe fn AdvancePlayhead(windowId: u8) {
    ShiftWaveformOver(
        windowId,
        (*crate::gl!(sDexCryScreen)).playhead as i16,
        FALSE,
    );
    (*crate::gl!(sDexCryScreen)).playhead += 2;
    let offset: u16 = (((*crate::gl!(sDexCryScreen)).playhead as i32 / 8
        + (*crate::gl!(sDexCryScreen)).playStartPos as i32
        + 1)
        % 32) as u16;
    for i in 0..7u8 {
        CopyToWindowPixelBuffer(
            windowId,
            sCryScreenBg_Gfx.as_ptr().cast_mut() as *mut c_void,
            32,
            offset + i as u16 * 32,
        );
    }
}
unsafe fn DrawWaveformSegment(position: u8, amplitude: u8) {
    let mut offset: u16 = 0;
    let temp: u16 = (amplitude as u16 + 127) * 256;
    let mut y: u8 = (temp as f32 / 1152_f32) as u8;
    if y > 55 {
        y = 55;
    }
    let currentPointY: u8 = y;
    let nybble: u8 = position & 1;
    if y > (*crate::gl!(sDexCryScreen)).waveformPreviousY {
        loop {
            offset = sWaveformOffsets[position as i32 & 7][y] + (position >> 3) as u16 * 32;
            *(*sCryWaveformWindowTiledata.as_ptr()).at(offset) &=
                sWaveformTileDataNybbleMasks[nybble];
            *(*sCryWaveformWindowTiledata.as_ptr()).at(offset) |=
                sWaveformColor[nybble][(y as i32 / 3 - 1) & 0x0F];
            y -= 1;
            if y <= (*crate::gl!(sDexCryScreen)).waveformPreviousY {
                break;
            }
        }
    } else {
        loop {
            offset = sWaveformOffsets[position as i32 & 7][y] + (position >> 3) as u16 * 32;
            *(*sCryWaveformWindowTiledata.as_ptr()).at(offset) &=
                sWaveformTileDataNybbleMasks[nybble];
            *(*sCryWaveformWindowTiledata.as_ptr()).at(offset) |=
                sWaveformColor[nybble][(y as i32 / 3 - 1) & 0x0F];
            y += 1;
            if y >= (*crate::gl!(sDexCryScreen)).waveformPreviousY {
                break;
            }
        }
    }
    (*crate::gl!(sDexCryScreen)).waveformPreviousY = currentPointY;
}
unsafe fn DrawWaveformWindow(windowId: u8) {
    CopyWindowToVram(windowId, COPYWIN_GFX);
}
unsafe fn ShiftWaveformOver(windowId: u8, offset: i16, rsVertical: u8) {
    if rsVertical == 0 {
        let bg: u8 = GetWindowAttribute(windowId, WINDOW_BG) as u8;
        ChangeBgX(bg, (offset as i32) << 8, BG_COORD_SET);
    }
}
pub unsafe fn LoadCryMeter(window: *mut CryScreenWindow, windowId: u8) -> u8 {
    let mut finished: u8 = FALSE;
    match gDexCryScreenState.get() {
        0 => {
            if (*sCryMeterNeedle.as_ptr()).is_null() {
                crate::gl!(sCryMeterNeedle = AllocZeroed(8) as *mut PokedexCryMeterNeedle);
            }
            CopyToWindowPixelBuffer(
                windowId,
                sCryMeter_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            LoadPalette(
                sCryMeter_Pal.as_ptr().cast_mut() as *mut c_void,
                (*window).paletteNo as u16 * 16,
                32,
            );
            gDexCryScreenState.set(gDexCryScreenState.get() + (1));
        }
        1 => {
            LoadSpriteSheets(sCryMeterNeedleSpriteSheets.as_ptr().cast_mut());
            LoadSpritePalettes(sCryMeterNeedleSpritePalettes.as_ptr().cast_mut());
            (*crate::gl!(sCryMeterNeedle)).spriteId = CreateSprite(
                (&raw const *sCryMeterNeedleSpriteTemplate).cast_mut(),
                40 + (*window).xPos as i16 * 8,
                56 + (*window).yPos as i16 * 8,
                1,
            ) as u16;
            (*crate::gl!(sCryMeterNeedle)).rotation = MIN_NEEDLE_POS;
            (*crate::gl!(sCryMeterNeedle)).targetRotation = MIN_NEEDLE_POS;
            (*crate::gl!(sCryMeterNeedle)).moveIncrement = 0;
            finished = TRUE;
        }
        _ => {}
    }
    finished
}
pub unsafe fn FreeCryScreen() {
    FreeSpritePaletteByTag(GetSpritePaletteTagByPaletteNum(
        (*gSprites.as_ptr())[(*crate::gl!(sCryMeterNeedle)).spriteId]
            .oam
            .paletteNum() as u8,
    ));
    DestroySprite(
        (*gSprites.as_ptr())
            .as_mut_ptr()
            .at((*crate::gl!(sCryMeterNeedle)).spriteId),
    );
    Free(crate::gl!(sDexCryScreen) as *mut c_void);
    crate::gl!(sDexCryScreen = null_mut());
    Free(crate::gl!(sCryMeterNeedle) as *mut c_void);
    crate::gl!(sCryMeterNeedle = null_mut());
}
pub(crate) unsafe fn SpriteCB_CryMeterNeedle(sprite: *mut Sprite) {
    let mut peakAmplitude: i8 = 0;
    let mut affine: ObjAffineSrcData = zeroed();
    let mut matrix: OamMatrix = zeroed();
    let mut amplitude: u8 = 0;
    (*gSprites.as_ptr())[(*crate::gl!(sCryMeterNeedle)).spriteId]
        .oam
        .set_affineMode(ST_OAM_AFFINE_NORMAL);
    {
        let gl_i0 = (*crate::gl!(sCryMeterNeedle)).spriteId;
        crate::gl!(gSprites[gl_i0].oam.affineParam = 0)
    };
    match (*crate::gl!(sDexCryScreen)).cryState {
        0 => {
            (*crate::gl!(sCryMeterNeedle)).targetRotation = MIN_NEEDLE_POS;
            if (*crate::gl!(sCryMeterNeedle)).rotation > 0 {
                if (*crate::gl!(sCryMeterNeedle)).moveIncrement != 1 {
                    (*crate::gl!(sCryMeterNeedle)).moveIncrement -= 1;
                }
            } else {
                (*crate::gl!(sCryMeterNeedle)).moveIncrement = NEEDLE_MOVE_INCREMENT;
            }
        }
        2 => {
            peakAmplitude = 0;
            for i in 0..16u16 {
                if (peakAmplitude as i32) < (*crate::gl!(sDexCryScreen)).cryWaveformBuffer[i] as i32
                {
                    peakAmplitude = (*crate::gl!(sDexCryScreen)).cryWaveformBuffer[i] as i8;
                }
            }
            SetCryMeterNeedleTarget((peakAmplitude as i32 * 208 / 256) as i8);
        }
        6 => {
            amplitude = (*crate::gl!(sDexCryScreen)).cryWaveformBuffer[10];
            SetCryMeterNeedleTarget((amplitude as i32 * 208 / 256) as i8);
        }
        _ => {}
    }
    if (*crate::gl!(sCryMeterNeedle)).rotation == (*crate::gl!(sCryMeterNeedle)).targetRotation {
    } else if (*crate::gl!(sCryMeterNeedle)).rotation
        < (*crate::gl!(sCryMeterNeedle)).targetRotation
    {
        (*crate::gl!(sCryMeterNeedle)).rotation +=
            (*crate::gl!(sCryMeterNeedle)).moveIncrement as i8;
        if (*crate::gl!(sCryMeterNeedle)).rotation > (*crate::gl!(sCryMeterNeedle)).targetRotation {
            (*crate::gl!(sCryMeterNeedle)).rotation = (*crate::gl!(sCryMeterNeedle)).targetRotation;
            (*crate::gl!(sCryMeterNeedle)).targetRotation = 0;
        }
    } else {
        (*crate::gl!(sCryMeterNeedle)).rotation -=
            (*crate::gl!(sCryMeterNeedle)).moveIncrement as i8;
        if (*crate::gl!(sCryMeterNeedle)).rotation < (*crate::gl!(sCryMeterNeedle)).targetRotation {
            (*crate::gl!(sCryMeterNeedle)).rotation = (*crate::gl!(sCryMeterNeedle)).targetRotation;
            (*crate::gl!(sCryMeterNeedle)).targetRotation = 0;
        }
    }
    affine.xScale = 256;
    affine.yScale = 256;
    affine.rotation = (*crate::gl!(sCryMeterNeedle)).rotation as u16 * 256;
    ObjAffineSet(&raw mut affine, &raw mut matrix as *mut c_void, 1, 2);
    SetOamMatrix(
        0,
        matrix.a as u16,
        matrix.b as u16,
        matrix.c as u16,
        matrix.d as u16,
    );
    let x: i16 = (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
        [((*crate::gl!(sCryMeterNeedle)).rotation as i32 + 0x7F) & 0xFF];
    let y: i16 = (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
        [(((*crate::gl!(sCryMeterNeedle)).rotation as i32 + 0x7F) & 0xFF) + 64];
    (*sprite).x2 = (x as i32 * 24 / 256) as i16;
    (*sprite).y2 = (y as i32 * 24 / 256) as i16;
}
unsafe fn SetCryMeterNeedleTarget(offset: i8) {
    let mut rotation: u16 = (MIN_NEEDLE_POS as u16 - offset as u16) & 0xFF;
    if rotation > MIN_NEEDLE_POS as u16 && rotation < MAX_NEEDLE_POS as u16 {
        rotation = MAX_NEEDLE_POS as u16;
    }
    (*crate::gl!(sCryMeterNeedle)).targetRotation = rotation as i8;
    (*crate::gl!(sCryMeterNeedle)).moveIncrement = NEEDLE_MOVE_INCREMENT;
}
