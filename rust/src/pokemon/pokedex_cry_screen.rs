//! Translated from `src/pokedex_cry_screen.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sCryMeterNeedle_Pal sCryMeterNeedle_Gfx sCryMeter_Tilemap sCryMeter_Pal sCryMeter_Gfx sWaveformOffsets sCryScreenBg_Pal sCryScreenBg_Gfx sWaveformTileDataNybbleMasks sWaveformColor sSpriteAnim_CryMeterNeedle sSpriteAnimTable_CryMeterNeedle sOamData_CryMeterNeedle sCryMeterNeedleSpriteTemplate sCryMeterNeedleSpriteSheets sCryMeterNeedleSpritePalettes
#[allow(unused_imports)]
use crate::data::pokedex_cry_screen::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gDexCryScreenState: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDexCryScreen: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCryWaveformWindowTiledata: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCryMeterNeedle: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gMPlayInfo_BGM: u8;
    static mut gPcmDmaCounter: u8;
    static mut gSineTable: u8;
    static mut gSoundInfo: u8;
    static mut gSprites: u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn CopyToWindowPixelBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn Free(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn GetSpritePaletteTagByPaletteNum(a0: u8) -> u16;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn IsCryPlaying() -> u8;
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut u8);
    fn LoadSpriteSheets(a0: *mut u8);
    fn ObjAffineSet(a0: *mut u8, a1: *mut u8, a2: i32, a3: i32);
    fn PlayCry_NormalNoDucking(a0: u16, a1: i8, a2: i8, a3: u8);
    fn SetOamMatrix(a0: u8, a1: u16, a2: u16, a3: u16, a4: u16);
    fn StopCry();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadCryWaveformWindow(window: *mut u8, windowId: u8) -> u8 {
    unsafe {
        let mut window = window;
        let mut windowId = windowId;
        let mut i: u8 = 0u8;
        let mut finished: u8 = 0u8;
        'l1: {
            let __sw1 = ((((&raw mut gDexCryScreenState).cast::<u8>().cast::<u8>()).read()) as i32);
            if __sw1 == 0i32 {
                if !(!(((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read()).is_null())
                {
                    ((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>())
                        .write(AllocZeroed(28u32));
                    ((&raw mut sCryWaveformWindowTiledata)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .write(((GetWindowAttribute(windowId, 7u8)) as usize as *mut u8));
                }
                ((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<u16>())
                .write(((window).cast::<u16>()).read());
                ((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(22))
                .write(((window).wrapping_add(5)).read());
                ((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(26))
                .write(0u8);
                ((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(27))
                .write(0u8);
                ((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16))
                .write(0u8);
                ((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(18))
                .write(((crate::c::div_i32(56i32, 2i32)) as u8));
                ((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(17))
                .write(0u8);
                ShiftWaveformOver(
                    windowId,
                    (((-8i32).wrapping_mul(((((window).wrapping_add(4)).read()) as i32))) as i16),
                    1u8,
                );
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < 224i32) {
                            break 'l2;
                        }
                        'l3: {
                            CopyToWindowPixelBuffer(
                                windowId,
                                ((&raw const sCryScreenBg_Gfx).cast::<u8>().cast_mut())
                                    .cast::<u8>(),
                                ((crate::c::div_i32(256i32, 8i32)) as u16),
                                ((i) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                let __p2 = (&raw mut gDexCryScreenState).cast::<u8>().cast::<u8>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                {
                    i = 0u8;
                    'l4: loop {
                        if !(((i) as i32)
                            < ((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(22))
                            .read()) as i32)
                                .wrapping_mul(8i32))
                        {
                            break 'l4;
                        }
                        'l5: {
                            DrawWaveformSegment(i, 0u8);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                let __p3 = (&raw mut gDexCryScreenState).cast::<u8>().cast::<u8>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                DrawWaveformWindow(windowId);
                LoadPalette(
                    (((&raw const sCryScreenBg_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    (((0i32).wrapping_add(
                        ((((window).wrapping_add(3)).read()) as i32).wrapping_mul(16i32),
                    )) as u16),
                    32u16,
                );
                finished = 1u8;
                break 'l1;
            }
        }
        return finished;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateCryWaveformWindow(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut waveformIdx: u8 = 0u8;
        DrawWaveformWindow(windowId);
        AdvancePlayhead(windowId);
        if (((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(27))
            .read())
            != 0
        {
            let __p1 =
                (((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(27);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
        if (((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26))
            .read())
            != 0
        {
            let __p2 =
                (((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26);
            (__p2).write(((__p2).read()).wrapping_sub(1));
            if !((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(26))
            .read())
                != 0)
            {
                PlayCryScreenCry(
                    ((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<u16>())
                    .read(),
                );
                DrawWaveformFlatline();
                return;
            }
        }
        if ((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16))
            .read()) as i32)
            == 0i32
        {
            DrawWaveformFlatline();
            return;
        }
        if ((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16))
            .read()) as i32)
            == 1i32
        {
            BufferCryWaveformSegment();
        } else {
            if ((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .read()) as i32)
                > 8i32
            {
                if !((IsCryPlaying()) != 0) {
                    DrawWaveformFlatline();
                    ((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .write(0u8);
                    return;
                }
                BufferCryWaveformSegment();
                ((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16))
                .write(1u8);
            }
        }
        waveformIdx = (((2i32).wrapping_mul(
            ((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16))
                .read()) as i32)
                .wrapping_sub(1i32),
        )) as u8);
        DrawWaveformSegment(
            ((((((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(22))
            .read()) as i32)
                .wrapping_mul(8i32))
            .wrapping_add(
                ((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(17))
                .read()) as i32),
            ))
            .wrapping_sub(2i32)) as u8),
            (((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset(((waveformIdx) as i32) as isize))
            .read(),
        );
        DrawWaveformSegment(
            ((((((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(22))
            .read()) as i32)
                .wrapping_mul(8i32))
            .wrapping_add(
                ((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(17))
                .read()) as i32),
            ))
            .wrapping_sub(1i32)) as u8),
            (((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset((((waveformIdx) as i32).wrapping_add(1i32)) as isize))
            .read(),
        );
        let __p3 =
            (((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
        (__p3).write(((__p3).read()).wrapping_add(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CryScreenPlayButton(species: u16) {
    unsafe {
        let mut species = species;
        if (((((&raw mut gMPlayInfo_BGM).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .read()
            & 2147483648u32)
            != 0)
            && (!((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(26))
            .read())
                != 0))
        {
            if !((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(27))
            .read())
                != 0)
            {
                ((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(27))
                .write(4u8);
                if ((IsCryPlaying()) as i32) == 1i32 {
                    StopCry();
                    ((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<u16>())
                    .write(species);
                    ((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(26))
                    .write(2u8);
                } else {
                    PlayCryScreenCry(species);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PlayCryScreenCry(species: u16) {
    unsafe {
        let mut species = species;
        PlayCry_NormalNoDucking(species, 0i8, 125i8, 10u8);
        ((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16))
            .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn BufferCryWaveformSegment() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut baseBuffer: *mut i8 = core::ptr::null_mut();
        let mut buffer: *mut i8 = core::ptr::null_mut();
        if ((((&raw mut gPcmDmaCounter).cast::<i8>()).read()) as i32) < 2i32 {
            baseBuffer = (((&raw mut gSoundInfo).cast::<u8>()).wrapping_add(848)).cast::<i8>();
        } else {
            baseBuffer = ((((&raw mut gSoundInfo).cast::<u8>()).wrapping_add(848)).cast::<i8>())
                .wrapping_offset(
                    ((((((((&raw mut gSoundInfo).cast::<u8>()).wrapping_add(11)).read()) as i32)
                        .wrapping_add(1i32))
                    .wrapping_sub(((((&raw mut gPcmDmaCounter).cast::<i8>()).read()) as i32)))
                    .wrapping_mul(
                        (((&raw mut gSoundInfo).cast::<u8>())
                            .wrapping_add(16)
                            .cast::<i32>())
                        .read(),
                    )) as isize,
                );
        }
        buffer = (baseBuffer).wrapping_offset(1584);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(16u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((buffer).wrapping_offset((((i) as i32).wrapping_mul(2i32)) as isize))
                            .read()) as i32)
                            .wrapping_mul(2i32)) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawWaveformFlatline() {
    unsafe {
        DrawWaveformSegment(
            ((((((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(22))
            .read()) as i32)
                .wrapping_mul(8i32))
            .wrapping_add(
                ((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(17))
                .read()) as i32),
            ))
            .wrapping_sub(2i32)) as u8),
            0u8,
        );
        DrawWaveformSegment(
            ((((((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(22))
            .read()) as i32)
                .wrapping_mul(8i32))
            .wrapping_add(
                ((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(17))
                .read()) as i32),
            ))
            .wrapping_sub(1i32)) as u8),
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn AdvancePlayhead(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut i: u8 = 0u8;
        let mut offset: u16 = 0u16;
        ShiftWaveformOver(
            windowId,
            ((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(17))
                .read()) as i16),
            0u8,
        );
        let __p1 =
            (((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(17);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as u8));
        offset = ((crate::c::rem_i32(
            ((crate::c::div_i32(
                ((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(17))
                .read()) as i32),
                8i32,
            ))
            .wrapping_add(
                ((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(22))
                .read()) as i32),
            ))
            .wrapping_add(1i32),
            32i32,
        )) as u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 7i32) {
                    break 'l1;
                }
                'l2: {
                    CopyToWindowPixelBuffer(
                        windowId,
                        ((&raw const sCryScreenBg_Gfx).cast::<u8>().cast_mut()).cast::<u8>(),
                        ((crate::c::div_i32(256i32, 8i32)) as u16),
                        ((((offset) as i32).wrapping_add(
                            ((i) as i32).wrapping_mul(crate::c::div_i32(256i32, 8i32)),
                        )) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawWaveformSegment(position: u8, amplitude: u8) {
    unsafe {
        let mut position = position;
        let mut amplitude = amplitude;
        let mut currentPointY: u8 = 0u8;
        let mut nybble: u8 = 0u8;
        let mut offset: u16 = 0u16;
        let mut temp: u16 = 0u16;
        let mut y: u8 = 0u8;
        temp = (((((amplitude) as i32).wrapping_add(127i32)).wrapping_mul(256i32)) as u16);
        y = ((((temp) as f32) / ((1152.0f32) as f32)) as u8);
        if ((y) as i32) > 55i32 {
            y = 55u8;
        }
        currentPointY = y;
        nybble = ((((position) as i32) & 1i32) as u8);
        if ((y) as i32)
            > ((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18))
            .read()) as i32)
        {
            'l1: loop {
                'l2: {
                    offset = ((((((((((&raw const sWaveformOffsets).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset((((position) as i32) & 7i32) as isize * 144))
                    .cast::<u16>())
                    .wrapping_offset(((y) as i32) as isize))
                    .read()) as i32)
                        .wrapping_add(
                            (((position) as i32) >> 3)
                                .wrapping_mul(crate::c::div_i32(256i32, 8i32)),
                        )) as u16);
                    let __p1 = (((&raw mut sCryWaveformWindowTiledata)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((offset) as i32) as isize);
                    (__p1).write(
                        (((((__p1).read()) as i32)
                            & ((((((&raw const sWaveformTileDataNybbleMasks)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((nybble) as i32) as isize))
                            .read()) as i32)) as u8),
                    );
                    let __p2 = (((&raw mut sCryWaveformWindowTiledata)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((offset) as i32) as isize);
                    (__p2).write(
                        (((((__p2).read()) as i32)
                            | ((((((((&raw const sWaveformColor).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((nybble) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::div_i32(((y) as i32), 3i32)).wrapping_sub(1i32) & 15i32)
                                    as isize,
                            ))
                            .read()) as i32)) as u8),
                    );
                    y = (y).wrapping_sub(1);
                }
                if !(((y) as i32)
                    > ((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18))
                    .read()) as i32))
                {
                    break 'l1;
                }
            }
        } else {
            'l3: loop {
                'l4: {
                    offset = ((((((((((&raw const sWaveformOffsets).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset((((position) as i32) & 7i32) as isize * 144))
                    .cast::<u16>())
                    .wrapping_offset(((y) as i32) as isize))
                    .read()) as i32)
                        .wrapping_add(
                            (((position) as i32) >> 3)
                                .wrapping_mul(crate::c::div_i32(256i32, 8i32)),
                        )) as u16);
                    let __p3 = (((&raw mut sCryWaveformWindowTiledata)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((offset) as i32) as isize);
                    (__p3).write(
                        (((((__p3).read()) as i32)
                            & ((((((&raw const sWaveformTileDataNybbleMasks)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((nybble) as i32) as isize))
                            .read()) as i32)) as u8),
                    );
                    let __p4 = (((&raw mut sCryWaveformWindowTiledata)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((offset) as i32) as isize);
                    (__p4).write(
                        (((((__p4).read()) as i32)
                            | ((((((((&raw const sWaveformColor).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((nybble) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::div_i32(((y) as i32), 3i32)).wrapping_sub(1i32) & 15i32)
                                    as isize,
                            ))
                            .read()) as i32)) as u8),
                    );
                    y = (y).wrapping_add(1);
                }
                if !(((y) as i32)
                    < ((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18))
                    .read()) as i32))
                {
                    break 'l3;
                }
            }
        }
        ((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(18))
            .write(currentPointY);
    }
}
pub(crate) unsafe extern "C" fn DrawWaveformWindow(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        CopyWindowToVram(windowId, 2u8);
    }
}
pub(crate) unsafe extern "C" fn ShiftWaveformOver(windowId: u8, offset: i16, rsVertical: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut offset = offset;
        let mut rsVertical = rsVertical;
        if !((rsVertical) != 0) {
            let mut bg: u8 = ((GetWindowAttribute(windowId, 0u8)) as u8);
            ChangeBgX(bg, (((offset) as i32) << 8), 0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadCryMeter(window: *mut u8, windowId: u8) -> u8 {
    unsafe {
        let mut window = window;
        let mut windowId = windowId;
        let mut finished: u8 = 0u8;
        'l1: {
            let __sw1 = ((((&raw mut gDexCryScreenState).cast::<u8>().cast::<u8>()).read()) as i32);
            if __sw1 == 0i32 {
                if !(!(((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                    .is_null())
                {
                    ((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>())
                        .write(AllocZeroed(8u32));
                }
                CopyToWindowPixelBuffer(
                    windowId,
                    ((&raw const sCryMeter_Gfx).cast::<u8>().cast_mut()).cast::<u8>(),
                    0u16,
                    0u16,
                );
                LoadPalette(
                    (((&raw const sCryMeter_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    (((0i32).wrapping_add(
                        ((((window).wrapping_add(3)).read()) as i32).wrapping_mul(16i32),
                    )) as u16),
                    32u16,
                );
                let __p2 = (&raw mut gDexCryScreenState).cast::<u8>().cast::<u8>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                LoadSpriteSheets(
                    ((&raw const sCryMeterNeedleSpriteSheets)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                LoadSpritePalettes(
                    ((&raw const sCryMeterNeedleSpritePalettes)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                ((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>())
                .write(
                    ((CreateSprite(
                        (&raw const sCryMeterNeedleSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        (((40i32).wrapping_add(
                            ((((window).wrapping_add(4)).read()) as i32).wrapping_mul(8i32),
                        )) as i16),
                        (((56i32).wrapping_add(
                            ((((window).wrapping_add(5)).read()) as i32).wrapping_mul(8i32),
                        )) as i16),
                        1u8,
                    )) as u16),
                );
                ((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read()).cast::<i8>())
                    .write(32i8);
                ((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1)
                    .cast::<i8>())
                .write(32i8);
                ((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .write(0u8);
                finished = 1u8;
                break 'l1;
            }
        }
        return finished;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeCryScreen() {
    unsafe {
        FreeSpritePaletteByTag(GetSpritePaletteTagByPaletteNum(
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<u16>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(5),
                4,
                4,
                false,
            ) as u16) as u8),
        ));
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>())
                .read()) as i32) as isize
                    * 68,
            ),
        );
        {
            Free(((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        }
        {
            Free(((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>())
                .write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CryMeterNeedle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: u16 = 0u16;
        let mut peakAmplitude: i8 = 0i8;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut affine = crate::ffi::Align4([0u8; 8]);
        let mut matrix = crate::ffi::Align4([0u8; 8]);
        let mut amplitude: u8 = 0u8;
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(1),
            0,
            2,
            (1u32) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(6)
        .cast::<u16>())
        .write(0u16);
        'l1: {
            let __sw1 = ((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1)
                    .cast::<i8>())
                .write(32i8);
                if ((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<i8>())
                .read()) as i32)
                    > 0i32
                {
                    if ((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2))
                    .read()) as i32)
                        != 1i32
                    {
                        let __p2 = (((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(2);
                        (__p2).write(((__p2).read()).wrapping_sub(1));
                    }
                } else {
                    ((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2))
                    .write(5u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                peakAmplitude = 0i8;
                {
                    i = 0u16;
                    'l2: loop {
                        if !(((i) as u32) < crate::c::div_u32(16u32, 1u32)) {
                            break 'l2;
                        }
                        'l3: {
                            if ((peakAmplitude) as i32)
                                < (((((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                            {
                                peakAmplitude = (((((((&raw mut sDexCryScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i8);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                SetCryMeterNeedleTarget(
                    ((crate::c::div_i32(((peakAmplitude) as i32).wrapping_mul(208i32), 256i32))
                        as i8),
                );
                break 'l1;
            }
            if __sw1 == 6i32 {
                amplitude = (((((&raw mut sDexCryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u8>())
                .wrapping_offset(10))
                .read();
                SetCryMeterNeedleTarget(
                    ((crate::c::div_i32(((amplitude) as i32).wrapping_mul(208i32), 256i32)) as i8),
                );
                break 'l1;
            }
        }
        if ((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read()).cast::<i8>())
            .read()) as i32)
            == ((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
        {
        } else {
            if ((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<i8>())
            .read()) as i32)
                < ((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as i32)
            {
                let __p3 = (((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<i8>();
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        ((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2))
                        .read()) as i32),
                    )) as i8),
                );
                if ((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<i8>())
                .read()) as i32)
                    > ((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1)
                        .cast::<i8>())
                    .read()) as i32)
                {
                    ((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<i8>())
                    .write(
                        ((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1)
                            .cast::<i8>())
                        .read(),
                    );
                    ((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1)
                        .cast::<i8>())
                    .write(0i8);
                }
            } else {
                let __p4 = (((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<i8>();
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_sub(
                        ((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2))
                        .read()) as i32),
                    )) as i8),
                );
                if ((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<i8>())
                .read()) as i32)
                    < ((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1)
                        .cast::<i8>())
                    .read()) as i32)
                {
                    ((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<i8>())
                    .write(
                        ((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1)
                            .cast::<i8>())
                        .read(),
                    );
                    ((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1)
                        .cast::<i8>())
                    .write(0i8);
                }
            }
        }
        (((&raw mut affine).cast::<u8>()).cast::<i16>()).write(256i16);
        (((&raw mut affine).cast::<u8>())
            .wrapping_add(2)
            .cast::<i16>())
        .write(256i16);
        (((&raw mut affine).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(
            ((((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read()).cast::<i8>())
                .read()) as i32)
                .wrapping_mul(256i32)) as u16),
        );
        ObjAffineSet(
            (&raw mut affine).cast::<u8>(),
            (&raw mut matrix).cast::<u8>(),
            1i32,
            2i32,
        );
        SetOamMatrix(
            0u8,
            (((((&raw mut matrix).cast::<u8>()).cast::<i16>()).read()) as u16),
            (((((&raw mut matrix).cast::<u8>())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as u16),
            (((((&raw mut matrix).cast::<u8>())
                .wrapping_add(4)
                .cast::<i16>())
            .read()) as u16),
            (((((&raw mut matrix).cast::<u8>())
                .wrapping_add(6)
                .cast::<i16>())
            .read()) as u16),
        );
        x = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
            (((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read()).cast::<i8>())
                .read()) as i32)
                .wrapping_add(127i32)
                & 255i32) as isize,
        ))
        .read();
        y = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
            ((((((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read()).cast::<i8>())
                .read()) as i32)
                .wrapping_add(127i32) & 255i32)
                .wrapping_add(64i32)) as isize,
        ))
        .read();
        ((sprite).wrapping_add(36).cast::<i16>())
            .write(((crate::c::div_i32(((x) as i32).wrapping_mul(24i32), 256i32)) as i16));
        ((sprite).wrapping_add(38).cast::<i16>())
            .write(((crate::c::div_i32(((y) as i32).wrapping_mul(24i32), 256i32)) as i16));
    }
}
pub(crate) unsafe extern "C" fn SetCryMeterNeedleTarget(offset: i8) {
    unsafe {
        let mut offset = offset;
        let mut rotation: u16 = (((32i32).wrapping_sub(((offset) as i32)) & 255i32) as u16);
        if (((rotation) as i32) > 32i32) && (((rotation) as i32) < 224i32) {
            rotation = 224u16;
        }
        ((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1)
            .cast::<i8>())
        .write(((rotation) as i8));
        ((((&raw mut sCryMeterNeedle).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
            .write(5u8);
    }
}
