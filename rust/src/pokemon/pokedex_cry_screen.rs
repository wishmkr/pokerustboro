//! Translated from `src/pokedex_cry_screen.c` by tools/rustport/c2rs.py.
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

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gDexCryScreenState: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDexCryScreen: *mut PokedexCryScreen = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCryWaveformWindowTiledata: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCryMeterNeedle: *mut PokedexCryMeterNeedle = null_mut();

unsafe extern "C" {
    static mut gMPlayInfo_BGM: MusicPlayerInfo;
    static mut gPcmDmaCounter: i8;
    static gSineTable: CArray<i16, 0>;
    static mut gSoundInfo: SoundInfo;
    static mut gSprites: CArray<Sprite, 65>;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn CopyToWindowPixelBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn Free(a0: *mut c_void);
    fn FreeSpritePaletteByTag(a0: u16);
    fn GetSpritePaletteTagByPaletteNum(a0: u8) -> u16;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn IsCryPlaying() -> u8;
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut SpritePalette);
    fn LoadSpriteSheets(a0: *mut SpriteSheet);
    fn ObjAffineSet(a0: *mut ObjAffineSrcData, a1: *mut c_void, a2: i32, a3: i32);
    fn PlayCry_NormalNoDucking(a0: u16, a1: i8, a2: i8, a3: u8);
    fn SetOamMatrix(a0: u8, a1: u16, a2: u16, a3: u16, a4: u16);
    fn StopCry();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadCryWaveformWindow(window: *mut CryScreenWindow, windowId: u8) -> u8 {
    let mut i: u8 = 0;
    let mut finished: u8 = FALSE;
    match gDexCryScreenState {
        0 => {
            if sDexCryScreen.is_null() {
                sDexCryScreen = AllocZeroed(28) as *mut PokedexCryScreen;
                sCryWaveformWindowTiledata =
                    GetWindowAttribute(windowId, WINDOW_TILE_DATA) as usize as *mut u8;
            }
            (*sDexCryScreen).unk = (*window).unk0;
            (*sDexCryScreen).playStartPos = (*window).yPos;
            (*sDexCryScreen).cryOverrideCountdown = 0;
            (*sDexCryScreen).cryRepeatDelay = 0;
            (*sDexCryScreen).cryState = 0;
            (*sDexCryScreen).waveformPreviousY = 28;
            (*sDexCryScreen).playhead = 0;
            ShiftWaveformOver(windowId, -8 * (*window).xPos as i16, TRUE);
            i = 0;
            while i < 224 {
                CopyToWindowPixelBuffer(
                    windowId,
                    sCryScreenBg_Gfx.as_ptr().cast_mut() as *mut c_void,
                    32,
                    i as u16,
                );
                i += 1;
            }
            gDexCryScreenState += 1;
        }
        1 => {
            i = 0;
            while (i as i32) < (*sDexCryScreen).playStartPos as i32 * 8 {
                DrawWaveformSegment(i, 0);
                i += 1;
            }
            gDexCryScreenState += 1;
        }
        2 => {
            DrawWaveformWindow(windowId);
            LoadPalette(
                sCryScreenBg_Pal.as_ptr().cast_mut() as *mut c_void,
                0x000 + (*window).paletteNo as u16 * 16,
                32,
            );
            finished = TRUE;
        }
        _ => {}
    }
    return finished;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateCryWaveformWindow(windowId: u8) {
    let mut waveformIdx: u8 = 0;
    DrawWaveformWindow(windowId);
    AdvancePlayhead(windowId);
    if (*sDexCryScreen).cryRepeatDelay != 0 {
        (*sDexCryScreen).cryRepeatDelay -= 1;
    }
    if (*sDexCryScreen).cryOverrideCountdown != 0 {
        (*sDexCryScreen).cryOverrideCountdown -= 1;
        if (*sDexCryScreen).cryOverrideCountdown == 0 {
            PlayCryScreenCry((*sDexCryScreen).species);
            DrawWaveformFlatline();
            return;
        }
    }
    if (*sDexCryScreen).cryState == 0 {
        DrawWaveformFlatline();
        return;
    }
    if (*sDexCryScreen).cryState == 1 {
        BufferCryWaveformSegment();
    } else if (*sDexCryScreen).cryState > 8 {
        if IsCryPlaying() == 0 {
            DrawWaveformFlatline();
            (*sDexCryScreen).cryState = 0;
            return;
        }
        BufferCryWaveformSegment();
        (*sDexCryScreen).cryState = 1;
    }
    waveformIdx = 2 * ((*sDexCryScreen).cryState - 1);
    DrawWaveformSegment(
        (*sDexCryScreen).playStartPos * 8 + (*sDexCryScreen).playhead - 2,
        (*sDexCryScreen).cryWaveformBuffer[waveformIdx],
    );
    DrawWaveformSegment(
        (*sDexCryScreen).playStartPos * 8 + (*sDexCryScreen).playhead - 1,
        (*sDexCryScreen).cryWaveformBuffer[waveformIdx as i32 + 1],
    );
    (*sDexCryScreen).cryState += 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CryScreenPlayButton(species: u16) {
    if gMPlayInfo_BGM.status & MUSICPLAYER_STATUS_PAUSE != 0
        && (*sDexCryScreen).cryOverrideCountdown == 0
    {
        if (*sDexCryScreen).cryRepeatDelay == 0 {
            (*sDexCryScreen).cryRepeatDelay = 4;
            if IsCryPlaying() == TRUE {
                StopCry();
                (*sDexCryScreen).species = species;
                (*sDexCryScreen).cryOverrideCountdown = 2;
            } else {
                PlayCryScreenCry(species);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PlayCryScreenCry(species: u16) {
    PlayCry_NormalNoDucking(species, 0, CRY_VOLUME_RS, CRY_PRIORITY_NORMAL);
    (*sDexCryScreen).cryState = 1;
}
pub(crate) unsafe extern "C" fn BufferCryWaveformSegment() {
    let mut i: u8 = 0;
    let mut baseBuffer: *mut i8 = null_mut();
    let mut buffer: *mut i8 = null_mut();
    if gPcmDmaCounter < 2 {
        baseBuffer = gSoundInfo.pcmBuffer.as_mut_ptr();
    } else {
        baseBuffer = gSoundInfo
            .pcmBuffer
            .as_mut_ptr()
            .at((gSoundInfo.pcmDmaPeriod as i32 + 1 - gPcmDmaCounter as i32)
                * gSoundInfo.pcmSamplesPerVBlank);
    }
    buffer = baseBuffer.at(1584);
    i = 0;
    while i < 16 {
        (*sDexCryScreen).cryWaveformBuffer[i] = *buffer.at(i as i32 * 2) as u8 * 2;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DrawWaveformFlatline() {
    DrawWaveformSegment(
        (*sDexCryScreen).playStartPos * 8 + (*sDexCryScreen).playhead - 2,
        0,
    );
    DrawWaveformSegment(
        (*sDexCryScreen).playStartPos * 8 + (*sDexCryScreen).playhead - 1,
        0,
    );
}
pub(crate) unsafe extern "C" fn AdvancePlayhead(windowId: u8) {
    let mut i: u8 = 0;
    let mut offset: u16 = 0;
    ShiftWaveformOver(windowId, (*sDexCryScreen).playhead as i16, FALSE);
    (*sDexCryScreen).playhead += 2;
    offset = (((*sDexCryScreen).playhead as i32 / 8 + (*sDexCryScreen).playStartPos as i32 + 1)
        % 32) as u16;
    i = 0;
    while i < 7 {
        CopyToWindowPixelBuffer(
            windowId,
            sCryScreenBg_Gfx.as_ptr().cast_mut() as *mut c_void,
            32,
            offset + i as u16 * 32,
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DrawWaveformSegment(position: u8, amplitude: u8) {
    let mut currentPointY: u8 = 0;
    let mut nybble: u8 = 0;
    let mut offset: u16 = 0;
    let mut temp: u16 = 0;
    let mut y: u8 = 0;
    temp = (amplitude as u16 + 127) * 256;
    y = (temp as f32 / 1152.0f32 as f32) as u8;
    if y > 55 {
        y = 55;
    }
    currentPointY = y;
    nybble = position & 1;
    if y > (*sDexCryScreen).waveformPreviousY {
        loop {
            offset = sWaveformOffsets[position as i32 & 7][y] + (position >> 3) as u16 * 32;
            *sCryWaveformWindowTiledata.at(offset) &= sWaveformTileDataNybbleMasks[nybble];
            *sCryWaveformWindowTiledata.at(offset) |=
                sWaveformColor[nybble][y as i32 / 3 - 1 & 0x0F];
            y -= 1;
            if y <= (*sDexCryScreen).waveformPreviousY {
                break;
            }
        }
    } else {
        loop {
            offset = sWaveformOffsets[position as i32 & 7][y] + (position >> 3) as u16 * 32;
            *sCryWaveformWindowTiledata.at(offset) &= sWaveformTileDataNybbleMasks[nybble];
            *sCryWaveformWindowTiledata.at(offset) |=
                sWaveformColor[nybble][y as i32 / 3 - 1 & 0x0F];
            y += 1;
            if y >= (*sDexCryScreen).waveformPreviousY {
                break;
            }
        }
    }
    (*sDexCryScreen).waveformPreviousY = currentPointY;
}
pub(crate) unsafe extern "C" fn DrawWaveformWindow(windowId: u8) {
    CopyWindowToVram(windowId, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn ShiftWaveformOver(windowId: u8, offset: i16, rsVertical: u8) {
    if rsVertical == 0 {
        let mut bg: u8 = GetWindowAttribute(windowId, WINDOW_BG) as u8;
        ChangeBgX(bg, (offset as i32) << 8, BG_COORD_SET);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadCryMeter(window: *mut CryScreenWindow, windowId: u8) -> u8 {
    let mut finished: u8 = FALSE;
    match gDexCryScreenState {
        0 => {
            if sCryMeterNeedle.is_null() {
                sCryMeterNeedle = AllocZeroed(8) as *mut PokedexCryMeterNeedle;
            }
            CopyToWindowPixelBuffer(
                windowId,
                sCryMeter_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            LoadPalette(
                sCryMeter_Pal.as_ptr().cast_mut() as *mut c_void,
                0x000 + (*window).paletteNo as u16 * 16,
                32,
            );
            gDexCryScreenState += 1;
        }
        1 => {
            LoadSpriteSheets(sCryMeterNeedleSpriteSheets.as_ptr().cast_mut());
            LoadSpritePalettes(sCryMeterNeedleSpritePalettes.as_ptr().cast_mut());
            (*sCryMeterNeedle).spriteId = CreateSprite(
                (&raw const *sCryMeterNeedleSpriteTemplate).cast_mut(),
                40 + (*window).xPos as i16 * 8,
                56 + (*window).yPos as i16 * 8,
                1,
            ) as u16;
            (*sCryMeterNeedle).rotation = MIN_NEEDLE_POS;
            (*sCryMeterNeedle).targetRotation = MIN_NEEDLE_POS;
            (*sCryMeterNeedle).moveIncrement = 0;
            finished = TRUE;
        }
        _ => {}
    }
    return finished;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeCryScreen() {
    FreeSpritePaletteByTag(GetSpritePaletteTagByPaletteNum(
        gSprites[(*sCryMeterNeedle).spriteId].oam.paletteNum() as u8,
    ));
    DestroySprite(gSprites.as_mut_ptr().at((*sCryMeterNeedle).spriteId));
    Free(sDexCryScreen as *mut c_void);
    sDexCryScreen = null_mut();
    Free(sCryMeterNeedle as *mut c_void);
    sCryMeterNeedle = null_mut();
}
pub(crate) unsafe extern "C" fn SpriteCB_CryMeterNeedle(sprite: *mut Sprite) {
    let mut i: u16 = 0;
    let mut peakAmplitude: i8 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut affine: ObjAffineSrcData = zeroed();
    let mut matrix: OamMatrix = zeroed();
    let mut amplitude: u8 = 0;
    gSprites[(*sCryMeterNeedle).spriteId]
        .oam
        .set_affineMode(ST_OAM_AFFINE_NORMAL);
    gSprites[(*sCryMeterNeedle).spriteId].oam.affineParam = 0;
    match (*sDexCryScreen).cryState {
        0 => {
            (*sCryMeterNeedle).targetRotation = MIN_NEEDLE_POS;
            if (*sCryMeterNeedle).rotation > 0 {
                if (*sCryMeterNeedle).moveIncrement != 1 {
                    (*sCryMeterNeedle).moveIncrement -= 1;
                }
            } else {
                (*sCryMeterNeedle).moveIncrement = NEEDLE_MOVE_INCREMENT;
            }
        }
        2 => {
            peakAmplitude = 0;
            i = 0;
            while i < 16 {
                if (peakAmplitude as i32) < (*sDexCryScreen).cryWaveformBuffer[i] as i32 {
                    peakAmplitude = (*sDexCryScreen).cryWaveformBuffer[i] as i8;
                }
                i += 1;
            }
            SetCryMeterNeedleTarget((peakAmplitude as i32 * 208 / 256) as i8);
        }
        6 => {
            amplitude = (*sDexCryScreen).cryWaveformBuffer[10];
            SetCryMeterNeedleTarget((amplitude as i32 * 208 / 256) as i8);
        }
        _ => {}
    }
    if (*sCryMeterNeedle).rotation == (*sCryMeterNeedle).targetRotation {
    } else if (*sCryMeterNeedle).rotation < (*sCryMeterNeedle).targetRotation {
        (*sCryMeterNeedle).rotation += (*sCryMeterNeedle).moveIncrement as i8;
        if (*sCryMeterNeedle).rotation > (*sCryMeterNeedle).targetRotation {
            (*sCryMeterNeedle).rotation = (*sCryMeterNeedle).targetRotation;
            (*sCryMeterNeedle).targetRotation = 0;
        }
    } else {
        (*sCryMeterNeedle).rotation -= (*sCryMeterNeedle).moveIncrement as i8;
        if (*sCryMeterNeedle).rotation < (*sCryMeterNeedle).targetRotation {
            (*sCryMeterNeedle).rotation = (*sCryMeterNeedle).targetRotation;
            (*sCryMeterNeedle).targetRotation = 0;
        }
    }
    affine.xScale = 256;
    affine.yScale = 256;
    affine.rotation = (*sCryMeterNeedle).rotation as u16 * 256;
    ObjAffineSet(&raw mut affine, &raw mut matrix as *mut c_void, 1, 2);
    SetOamMatrix(
        0,
        matrix.a as u16,
        matrix.b as u16,
        matrix.c as u16,
        matrix.d as u16,
    );
    x = gSineTable[(*sCryMeterNeedle).rotation as i32 + 0x7F & 0xFF];
    y = gSineTable[((*sCryMeterNeedle).rotation as i32 + 0x7F & 0xFF) + 64];
    (*sprite).x2 = (x as i32 * 24 / 256) as i16;
    (*sprite).y2 = (y as i32 * 24 / 256) as i16;
}
pub(crate) unsafe extern "C" fn SetCryMeterNeedleTarget(offset: i8) {
    let mut rotation: u16 = MIN_NEEDLE_POS as u16 - offset as u16 & 0xFF;
    if rotation > MIN_NEEDLE_POS as u16 && rotation < MAX_NEEDLE_POS as u16 {
        rotation = MAX_NEEDLE_POS as u16;
    }
    (*sCryMeterNeedle).targetRotation = rotation as i8;
    (*sCryMeterNeedle).moveIncrement = NEEDLE_MOVE_INCREMENT;
}
