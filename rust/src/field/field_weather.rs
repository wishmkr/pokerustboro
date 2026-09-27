//! Translated from `src/field_weather.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sDroughtWeatherColors gWeatherPtr sWeatherFuncs gWeatherPalStateFuncs sBasePaletteColorMapTypes gFogPalette
#[allow(unused_imports)]
use crate::data::field_weather::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gWeather: crate::ffi::Align4<[u8; 1872]> = crate::ffi::Align4([0; 1872]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFieldEffectPaletteColorMapTypes: crate::ffi::Align4<[u8; 32]> =
    crate::ffi::Align4([0; 32]);
pub(crate) static mut sPaletteColorMapTypes: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gPaletteFade: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSineTable: u8;
    static mut gTasks: u8;
    fn AllocSpritePalette(a0: u16) -> u8;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn CpuFastSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn IsSpecialSEPlaying() -> u8;
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn PlaySE(a0: u16);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetWeather(a0: u32);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartWeather() {
    unsafe {
        if !((FuncIsActiveTask(Some(Task_WeatherMain))) != 0) {
            let mut index: u8 = AllocSpritePalette(4608u16);
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (((&raw const gFogPalette)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<u16>())
                                .cast::<u16>())
                                .cast::<u8>(),
                                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((256i32)
                                            .wrapping_add(((index) as i32).wrapping_mul(16i32)))
                                            as isize,
                                    ))
                                .cast::<u8>(),
                                (67108864u32
                                    | (crate::c::div_u32(
                                        32u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
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
            BuildColorMaps();
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1749))
            .write(index);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1748))
            .write(AllocSpritePalette(4609u16));
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1754))
            .write(0u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1752))
            .write(0u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1758))
            .write(0u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1764))
            .write(0u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1792))
            .write(0u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1787))
            .write(0u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1828))
            .write(0u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1814))
            .write(0u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1815))
            .write(0u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1838))
            .write(0u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1786))
            .write(0u8);
            Weather_SetBlendCoeffs(16u8, 0u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1744))
            .write(0u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1734))
            .write(3u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1736))
            .write(0u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1747))
            .write(1u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1737))
            .write(CreateTask(Some(Task_WeatherInit), 80u8));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetNextWeather(weather: u8) {
    unsafe {
        let mut weather = weather;
        if ((((weather) as i32) != 3i32) && (((weather) as i32) != 5i32))
            && (((weather) as i32) != 13i32)
        {
            PlayRainStoppingSoundEffect();
        }
        if (((((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1745))
        .read()) as i32)
            != ((weather) as i32))
            && (((((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1744))
            .read()) as i32)
                == ((weather) as i32))
        {
            ((((((&raw const sWeatherFuncs).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((weather) as i32) as isize * 16))
            .cast::<Option<unsafe extern "C" fn()>>())
            .read())
            .unwrap_unchecked()();
        }
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1747))
        .write(0u8);
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1745))
        .write(weather);
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1742)
        .cast::<u16>())
        .write(0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCurrentAndNextWeather(weather: u8) {
    unsafe {
        let mut weather = weather;
        PlayRainStoppingSoundEffect();
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1744))
        .write(weather);
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1745))
        .write(weather);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCurrentAndNextWeatherNoDelay(weather: u8) {
    unsafe {
        let mut weather = weather;
        PlayRainStoppingSoundEffect();
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1744))
        .write(weather);
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1745))
        .write(weather);
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1736))
        .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn Task_WeatherInit(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1736))
        .read())
            != 0
        {
            ((((((&raw const sWeatherFuncs).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1744))
                    .read()) as i32) as isize
                        * 16,
                ))
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
            .read())
            .unwrap_unchecked()();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_WeatherMain));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WeatherMain(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1744))
        .read()) as i32)
            != ((((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1745))
            .read()) as i32)
        {
            if (!((((((((&raw const sWeatherFuncs).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1744))
                    .read()) as i32) as isize
                        * 16,
                ))
            .wrapping_add(12)
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
            .read())
            .unwrap_unchecked()())
                != 0))
                && (((((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1734))
                .read()) as i32)
                    != 2i32)
            {
                ((((((&raw const sWeatherFuncs).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1745))
                        .read()) as i32) as isize
                            * 16,
                    ))
                .cast::<Option<unsafe extern "C" fn()>>())
                .read())
                .unwrap_unchecked()();
                ((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1731))
                .write(0u8);
                ((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1734))
                .write(0u8);
                ((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1744))
                .write(
                    ((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1745))
                    .read(),
                );
                ((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1747))
                .write(1u8);
            }
        } else {
            ((((((&raw const sWeatherFuncs).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1744))
                    .read()) as i32) as isize
                        * 16,
                ))
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn()>>())
            .read())
            .unwrap_unchecked()();
        }
        (((((&raw const gWeatherPalStateFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            ((((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1734))
            .read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn None_Init() {
    unsafe {
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1729)
        .cast::<i8>())
        .write(0i8);
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1730))
        .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn None_Main() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn None_Finish() -> u8 {
    unsafe {
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn BuildColorMaps() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut colorMaps: *mut u8 = core::ptr::null_mut();
        let mut colorVal: u16 = 0u16;
        let mut curBrightness: u16 = 0u16;
        let mut brightnessDelta: u16 = 0u16;
        let mut colorMapIndex: u16 = 0u16;
        let mut baseBrightness: u16 = 0u16;
        let mut diff: i16 = 0i16;
        ((&raw mut sPaletteColorMapTypes)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(
            ((&raw const sBasePaletteColorMapTypes)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    if ((i) as i32) == 0i32 {
                        colorMaps = ((((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(512))
                        .cast::<u8>();
                    } else {
                        colorMaps = ((((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1120))
                        .cast::<u8>();
                    }
                    {
                        colorVal = 0u16;
                        'l3: loop {
                            if !(((colorVal) as i32) < 32i32) {
                                break 'l3;
                            }
                            'l4: {
                                curBrightness = ((((colorVal) as i32) << 8) as u16);
                                if ((i) as i32) == 0i32 {
                                    brightnessDelta =
                                        ((crate::c::div_i32((((colorVal) as i32) << 8), 16i32))
                                            as u16);
                                } else {
                                    brightnessDelta = 0u16;
                                }
                                {
                                    colorMapIndex = 0u16;
                                    'l5: loop {
                                        if !(((colorMapIndex) as i32) < 3i32) {
                                            break 'l5;
                                        }
                                        'l6: {
                                            curBrightness = ((((curBrightness) as i32)
                                                .wrapping_sub(((brightnessDelta) as i32)))
                                                as u16);
                                            ((((colorMaps).wrapping_offset(
                                                ((colorMapIndex) as i32) as isize * 32,
                                            ))
                                            .cast::<u8>())
                                            .wrapping_offset(((colorVal) as i32) as isize))
                                            .write(((((curBrightness) as i32) >> 8) as u8));
                                        }
                                        colorMapIndex = (colorMapIndex).wrapping_add(1);
                                    }
                                }
                                baseBrightness = curBrightness;
                                brightnessDelta = ((crate::c::div_i32(
                                    (7936i32).wrapping_sub(((curBrightness) as i32)),
                                    16i32,
                                )) as u16);
                                if ((colorVal) as i32) < 12i32 {
                                    {
                                        'l7: loop {
                                            if !(((colorMapIndex) as i32) < 19i32) {
                                                break 'l7;
                                            }
                                            'l8: {
                                                curBrightness = ((((curBrightness) as i32)
                                                    .wrapping_add(((brightnessDelta) as i32)))
                                                    as u16);
                                                diff = ((((curBrightness) as i32)
                                                    .wrapping_sub(((baseBrightness) as i32)))
                                                    as i16);
                                                if ((diff) as i32) > 0i32 {
                                                    curBrightness = ((((curBrightness) as i32)
                                                        .wrapping_sub(crate::c::div_i32(
                                                            ((diff) as i32),
                                                            2i32,
                                                        )))
                                                        as u16);
                                                }
                                                ((((colorMaps).wrapping_offset(
                                                    ((colorMapIndex) as i32) as isize * 32,
                                                ))
                                                .cast::<u8>())
                                                .wrapping_offset(((colorVal) as i32) as isize))
                                                .write(((((curBrightness) as i32) >> 8) as u8));
                                                if ((((((colorMaps).wrapping_offset(
                                                    ((colorMapIndex) as i32) as isize * 32,
                                                ))
                                                .cast::<u8>())
                                                .wrapping_offset(((colorVal) as i32) as isize))
                                                .read())
                                                    as i32)
                                                    > 31i32
                                                {
                                                    ((((colorMaps).wrapping_offset(
                                                        ((colorMapIndex) as i32) as isize * 32,
                                                    ))
                                                    .cast::<u8>())
                                                    .wrapping_offset(((colorVal) as i32) as isize))
                                                    .write(31u8);
                                                }
                                            }
                                            colorMapIndex = (colorMapIndex).wrapping_add(1);
                                        }
                                    }
                                } else {
                                    {
                                        'l9: loop {
                                            if !(((colorMapIndex) as i32) < 19i32) {
                                                break 'l9;
                                            }
                                            'l10: {
                                                curBrightness = ((((curBrightness) as i32)
                                                    .wrapping_add(((brightnessDelta) as i32)))
                                                    as u16);
                                                ((((colorMaps).wrapping_offset(
                                                    ((colorMapIndex) as i32) as isize * 32,
                                                ))
                                                .cast::<u8>())
                                                .wrapping_offset(((colorVal) as i32) as isize))
                                                .write(((((curBrightness) as i32) >> 8) as u8));
                                                if ((((((colorMaps).wrapping_offset(
                                                    ((colorMapIndex) as i32) as isize * 32,
                                                ))
                                                .cast::<u8>())
                                                .wrapping_offset(((colorVal) as i32) as isize))
                                                .read())
                                                    as i32)
                                                    > 31i32
                                                {
                                                    ((((colorMaps).wrapping_offset(
                                                        ((colorMapIndex) as i32) as isize * 32,
                                                    ))
                                                    .cast::<u8>())
                                                    .wrapping_offset(((colorVal) as i32) as isize))
                                                    .write(31u8);
                                                }
                                            }
                                            colorMapIndex = (colorMapIndex).wrapping_add(1);
                                        }
                                    }
                                }
                            }
                            colorVal = (colorVal).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateWeatherColorMap() {
    unsafe {
        if ((((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1734))
        .read()) as i32)
            != 2i32
        {
            if ((((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1728)
            .cast::<i8>())
            .read()) as i32)
                == ((((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1729)
                .cast::<i8>())
                .read()) as i32)
            {
                ((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1734))
                .write(3u8);
            } else {
                if (({
                    let __p1 = (((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1731);
                    let __t2 = ((__p1).read()).wrapping_add(1);
                    (__p1).write(__t2);
                    __t2
                }) as i32)
                    >= ((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1730))
                    .read()) as i32)
                {
                    ((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1731))
                    .write(0u8);
                    if ((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1728)
                    .cast::<i8>())
                    .read()) as i32)
                        < ((((((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1729)
                        .cast::<i8>())
                        .read()) as i32)
                    {
                        let __p3 = (((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1728)
                        .cast::<i8>();
                        (__p3).write(((__p3).read()).wrapping_add(1));
                    } else {
                        let __p4 = (((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1728)
                        .cast::<i8>();
                        (__p4).write(((__p4).read()).wrapping_sub(1));
                    }
                    ApplyColorMap(
                        0u8,
                        32u8,
                        ((((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1728)
                        .cast::<i8>())
                        .read(),
                    );
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FadeInScreenWithWeather() {
    unsafe {
        if (({
            let __p1 = (((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1739);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 1i32
        {
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1738))
            .write(0u8);
        }
        'l1: {
            let __sw3 = ((((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1744))
            .read()) as i32);
            let __matched = __sw3 == 3i32
                || __sw3 == 5i32
                || __sw3 == 13i32
                || __sw3 == 4i32
                || __sw3 == 11i32
                || __sw3 == 12i32
                || __sw3 == 6i32
                || __sw3 == 7i32
                || __sw3 == 8i32
                || __sw3 == 9i32
                || __sw3 == 10i32;
            if __sw3 == 3i32 || __sw3 == 5i32 || __sw3 == 13i32 || __sw3 == 4i32 || __sw3 == 11i32 {
                if ((FadeInScreen_RainShowShade()) as i32) == 0i32 {
                    ((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1728)
                    .cast::<i8>())
                    .write(3i8);
                    ((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1734))
                    .write(3u8);
                }
                break 'l1;
            }
            if __sw3 == 12i32 {
                if ((FadeInScreen_Drought()) as i32) == 0i32 {
                    ((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1728)
                    .cast::<i8>())
                    .write((-6i8));
                    ((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1734))
                    .write(3u8);
                }
                break 'l1;
            }
            if __sw3 == 6i32 {
                if ((FadeInScreen_FogHorizontal()) as i32) == 0i32 {
                    ((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1728)
                    .cast::<i8>())
                    .write(0i8);
                    ((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1734))
                    .write(3u8);
                }
                break 'l1;
            }
            if __sw3 == 7i32 || __sw3 == 8i32 || __sw3 == 9i32 || __sw3 == 10i32 || !__matched {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1728)
                    .cast::<i8>())
                    .write(
                        ((((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1729)
                        .cast::<i8>())
                        .read(),
                    );
                    ((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1734))
                    .write(3u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FadeInScreen_RainShowShade() -> u8 {
    unsafe {
        if ((((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1735))
        .read()) as i32)
            == 16i32
        {
            return 0u8;
        }
        if (({
            let __p1 = (((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1735);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= 16i32
        {
            ApplyColorMap(0u8, 32u8, 3i8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1735))
            .write(16u8);
            return 0u8;
        }
        ApplyColorMapWithBlend(
            0u8,
            32u8,
            3i8,
            (((16i32).wrapping_sub(
                ((((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1735))
                .read()) as i32),
            )) as u8),
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1732)
            .cast::<u16>())
            .read(),
        );
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn FadeInScreen_Drought() -> u8 {
    unsafe {
        if ((((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1735))
        .read()) as i32)
            == 16i32
        {
            return 0u8;
        }
        if (({
            let __p1 = (((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1735);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= 16i32
        {
            ApplyColorMap(0u8, 32u8, (-6i8));
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1735))
            .write(16u8);
            return 0u8;
        }
        ApplyDroughtColorMapWithBlend(
            (-6i8),
            (((16i32).wrapping_sub(
                ((((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1735))
                .read()) as i32),
            )) as u8),
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1732)
            .cast::<u16>())
            .read(),
        );
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn FadeInScreen_FogHorizontal() -> u8 {
    unsafe {
        if ((((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1735))
        .read()) as i32)
            == 16i32
        {
            return 0u8;
        }
        let __p1 = (((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1735);
        (__p1).write(((__p1).read()).wrapping_add(1));
        ApplyFogBlend(
            (((16i32).wrapping_sub(
                ((((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1735))
                .read()) as i32),
            )) as u8),
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1732)
            .cast::<u16>())
            .read(),
        );
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn DoNothing() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn ApplyColorMap(
    startPalIndex: u8,
    numPalettes: u8,
    colorMapIndex: i8,
) {
    unsafe {
        let mut startPalIndex = startPalIndex;
        let mut numPalettes = numPalettes;
        let mut colorMapIndex = colorMapIndex;
        let mut curPalIndex: u16 = 0u16;
        let mut palOffset: u16 = 0u16;
        let mut colorMap: *mut u8 = core::ptr::null_mut();
        let mut i: u16 = 0u16;
        if ((colorMapIndex) as i32) > 0i32 {
            colorMapIndex = (colorMapIndex).wrapping_sub(1);
            palOffset = ((((startPalIndex) as i32).wrapping_mul(16i32)) as u16);
            numPalettes = ((((numPalettes) as i32).wrapping_add(((startPalIndex) as i32))) as u8);
            curPalIndex = ((startPalIndex) as u16);
            'l1: loop {
                if !(((curPalIndex) as i32) < ((numPalettes) as i32)) {
                    break 'l1;
                }
                if ((((((&raw mut sPaletteColorMapTypes)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((curPalIndex) as i32) as isize))
                .read()) as i32)
                    == 0i32
                {
                    'l2: loop {
                        'l3: {
                            CpuFastSet(
                                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(((palOffset) as i32) as isize))
                                .cast::<u8>(),
                                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(((palOffset) as i32) as isize))
                                .cast::<u8>(),
                                (crate::c::div_u32(
                                    32u32,
                                    ((crate::c::div_i32(32i32, 8i32)) as u32),
                                ) & 2097151u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l2;
                        }
                    }
                    palOffset = ((((palOffset) as i32).wrapping_add(16i32)) as u16);
                } else {
                    let mut r: u8 = 0u8;
                    let mut g: u8 = 0u8;
                    let mut b: u8 = 0u8;
                    if (((((((&raw mut sPaletteColorMapTypes)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((curPalIndex) as i32) as isize))
                    .read()) as i32)
                        == 2i32)
                        || (((curPalIndex) as i32).wrapping_sub(16i32)
                            == ((((((&raw const gWeatherPtr)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(1749))
                            .read()) as i32))
                    {
                        colorMap = ((((((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1120))
                        .cast::<u8>())
                        .wrapping_offset(((colorMapIndex) as i32) as isize * 32))
                        .cast::<u8>();
                    } else {
                        colorMap = ((((((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(512))
                        .cast::<u8>())
                        .wrapping_offset(((colorMapIndex) as i32) as isize * 32))
                        .cast::<u8>();
                    }
                    {
                        i = 0u16;
                        'l4: loop {
                            if !(((i) as i32) < 16i32) {
                                break 'l4;
                            }
                            'l5: {
                                let mut baseColor = crate::ffi::Align4([0u8; 4]);
                                (&raw mut baseColor)
                                    .cast::<u8>()
                                    .cast::<crate::c::Rec4<4>>()
                                    .write_unaligned(
                                        ((((&raw mut gPlttBufferUnfaded).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset(((palOffset) as i32) as isize))
                                        .cast::<u8>()
                                        .cast::<crate::c::Rec4<4>>()
                                        .read_unaligned(),
                                    );
                                r = ((colorMap).wrapping_offset(
                                    ((crate::c::bf_read(
                                        ((&raw mut baseColor).cast::<u8>()).wrapping_add(0),
                                        0,
                                        5,
                                        false,
                                    ) as u16) as i32) as isize,
                                ))
                                .read();
                                g = ((colorMap).wrapping_offset(
                                    ((crate::c::bf_read(
                                        ((&raw mut baseColor).cast::<u8>()).wrapping_add(0),
                                        5,
                                        5,
                                        false,
                                    ) as u16) as i32) as isize,
                                ))
                                .read();
                                b = ((colorMap).wrapping_offset(
                                    ((crate::c::bf_read(
                                        ((&raw mut baseColor).cast::<u8>()).wrapping_add(1),
                                        2,
                                        5,
                                        false,
                                    ) as u16) as i32) as isize,
                                ))
                                .read();
                                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        (({
                                            let __t1 = palOffset;
                                            palOffset = (palOffset).wrapping_add(1);
                                            __t1
                                        }) as i32) as isize,
                                    ))
                                .write(
                                    ((((((b) as i32) << 10) | (((g) as i32) << 5)) | ((r) as i32))
                                        as u16),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                curPalIndex = (curPalIndex).wrapping_add(1);
            }
        } else {
            if ((colorMapIndex) as i32) < 0i32 {
                colorMapIndex =
                    (((((colorMapIndex) as i32).wrapping_neg()).wrapping_sub(1i32)) as i8);
                palOffset = ((((startPalIndex) as i32).wrapping_mul(16i32)) as u16);
                numPalettes =
                    ((((numPalettes) as i32).wrapping_add(((startPalIndex) as i32))) as u8);
                curPalIndex = ((startPalIndex) as u16);
                'l6: loop {
                    if !(((curPalIndex) as i32) < ((numPalettes) as i32)) {
                        break 'l6;
                    }
                    if ((((((&raw mut sPaletteColorMapTypes)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((curPalIndex) as i32) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        'l7: loop {
                            'l8: {
                                CpuFastSet(
                                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(((palOffset) as i32) as isize))
                                    .cast::<u8>(),
                                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(((palOffset) as i32) as isize))
                                    .cast::<u8>(),
                                    (crate::c::div_u32(
                                        32u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l7;
                            }
                        }
                        palOffset = ((((palOffset) as i32).wrapping_add(16i32)) as u16);
                    } else {
                        {
                            i = 0u16;
                            'l9: loop {
                                if !(((i) as i32) < 16i32) {
                                    break 'l9;
                                }
                                'l10: {
                                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(((palOffset) as i32) as isize))
                                    .write(
                                        ((((((&raw const sDroughtWeatherColors)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((colorMapIndex) as i32) as isize * 8192,
                                        ))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            ((((((((((&raw mut gPlttBufferUnfaded).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(((palOffset) as i32) as isize))
                                            .read())
                                                as i32)
                                                >> 1)
                                                & 15i32)
                                                | ((((((((&raw mut gPlttBufferUnfaded)
                                                    .cast::<u16>())
                                                .cast::<u16>())
                                                .wrapping_offset(((palOffset) as i32) as isize))
                                                .read())
                                                    as i32)
                                                    >> 2)
                                                    & 240i32))
                                                | ((((((((&raw mut gPlttBufferUnfaded)
                                                    .cast::<u16>())
                                                .cast::<u16>())
                                                .wrapping_offset(((palOffset) as i32) as isize))
                                                .read())
                                                    as i32)
                                                    >> 3)
                                                    & 3840i32))
                                                as isize,
                                        ))
                                        .read(),
                                    );
                                    palOffset = (palOffset).wrapping_add(1);
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    }
                    curPalIndex = (curPalIndex).wrapping_add(1);
                }
            } else {
                'l11: loop {
                    'l12: {
                        CpuFastSet(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    (((startPalIndex) as i32).wrapping_mul(16i32)) as isize,
                                ))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    (((startPalIndex) as i32).wrapping_mul(16i32)) as isize,
                                ))
                            .cast::<u8>(),
                            (crate::c::div_u32(
                                ((numPalettes) as u32).wrapping_mul(32u32),
                                ((crate::c::div_i32(32i32, 8i32)) as u32),
                            ) & 2097151u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l11;
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyColorMapWithBlend(
    startPalIndex: u8,
    numPalettes: u8,
    colorMapIndex: i8,
    blendCoeff: u8,
    blendColor: u16,
) {
    unsafe {
        let mut startPalIndex = startPalIndex;
        let mut numPalettes = numPalettes;
        let mut colorMapIndex = colorMapIndex;
        let mut blendCoeff = blendCoeff;
        let mut blendColor = blendColor;
        let mut palOffset: u16 = 0u16;
        let mut curPalIndex: u16 = 0u16;
        let mut i: u16 = 0u16;
        let mut color = crate::ffi::Align4([0u8; 4]);
        (&raw mut color)
            .cast::<u8>()
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (&raw mut blendColor)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        let mut rBlend: u8 =
            ((crate::c::bf_read(((&raw mut color).cast::<u8>()).wrapping_add(0), 0, 5, false)
                as u16) as u8);
        let mut gBlend: u8 =
            ((crate::c::bf_read(((&raw mut color).cast::<u8>()).wrapping_add(0), 5, 5, false)
                as u16) as u8);
        let mut bBlend: u8 =
            ((crate::c::bf_read(((&raw mut color).cast::<u8>()).wrapping_add(1), 2, 5, false)
                as u16) as u8);
        palOffset = ((((startPalIndex) as i32).wrapping_mul(16i32)) as u16);
        numPalettes = ((((numPalettes) as i32).wrapping_add(((startPalIndex) as i32))) as u8);
        colorMapIndex = (colorMapIndex).wrapping_sub(1);
        curPalIndex = ((startPalIndex) as u16);
        'l1: loop {
            if !(((curPalIndex) as i32) < ((numPalettes) as i32)) {
                break 'l1;
            }
            if ((((((&raw mut sPaletteColorMapTypes)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((curPalIndex) as i32) as isize))
            .read()) as i32)
                == 0i32
            {
                BlendPalette(palOffset, 16u16, blendCoeff, blendColor);
                palOffset = ((((palOffset) as i32).wrapping_add(16i32)) as u16);
            } else {
                let mut colorMap: *mut u8 = core::ptr::null_mut();
                if ((((((&raw mut sPaletteColorMapTypes)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((curPalIndex) as i32) as isize))
                .read()) as i32)
                    == 1i32
                {
                    colorMap = ((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(512))
                    .cast::<u8>())
                    .wrapping_offset(((colorMapIndex) as i32) as isize * 32))
                    .cast::<u8>();
                } else {
                    colorMap = ((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1120))
                    .cast::<u8>())
                    .wrapping_offset(((colorMapIndex) as i32) as isize * 32))
                    .cast::<u8>();
                }
                {
                    i = 0u16;
                    'l2: loop {
                        if !(((i) as i32) < 16i32) {
                            break 'l2;
                        }
                        'l3: {
                            let mut baseColor = crate::ffi::Align4([0u8; 4]);
                            (&raw mut baseColor)
                                .cast::<u8>()
                                .cast::<crate::c::Rec4<4>>()
                                .write_unaligned(
                                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(((palOffset) as i32) as isize))
                                    .cast::<u8>()
                                    .cast::<crate::c::Rec4<4>>()
                                    .read_unaligned(),
                                );
                            let mut r: u8 = ((colorMap).wrapping_offset(
                                ((crate::c::bf_read(
                                    ((&raw mut baseColor).cast::<u8>()).wrapping_add(0),
                                    0,
                                    5,
                                    false,
                                ) as u16) as i32) as isize,
                            ))
                            .read();
                            let mut g: u8 = ((colorMap).wrapping_offset(
                                ((crate::c::bf_read(
                                    ((&raw mut baseColor).cast::<u8>()).wrapping_add(0),
                                    5,
                                    5,
                                    false,
                                ) as u16) as i32) as isize,
                            ))
                            .read();
                            let mut b: u8 = ((colorMap).wrapping_offset(
                                ((crate::c::bf_read(
                                    ((&raw mut baseColor).cast::<u8>()).wrapping_add(1),
                                    2,
                                    5,
                                    false,
                                ) as u16) as i32) as isize,
                            ))
                            .read();
                            r = ((((r) as i32).wrapping_add(
                                ((((rBlend) as i32).wrapping_sub(((r) as i32)))
                                    .wrapping_mul(((blendCoeff) as i32))
                                    >> 4),
                            )) as u8);
                            g = ((((g) as i32).wrapping_add(
                                ((((gBlend) as i32).wrapping_sub(((g) as i32)))
                                    .wrapping_mul(((blendCoeff) as i32))
                                    >> 4),
                            )) as u8);
                            b = ((((b) as i32).wrapping_add(
                                ((((bBlend) as i32).wrapping_sub(((b) as i32)))
                                    .wrapping_mul(((blendCoeff) as i32))
                                    >> 4),
                            )) as u8);
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    (({
                                        let __t1 = palOffset;
                                        palOffset = (palOffset).wrapping_add(1);
                                        __t1
                                    }) as i32) as isize,
                                ))
                            .write(
                                ((((((b) as i32) << 10) | (((g) as i32) << 5)) | ((r) as i32))
                                    as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            curPalIndex = (curPalIndex).wrapping_add(1);
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyDroughtColorMapWithBlend(
    colorMapIndex: i8,
    blendCoeff: u8,
    blendColor: u16,
) {
    unsafe {
        let mut colorMapIndex = colorMapIndex;
        let mut blendCoeff = blendCoeff;
        let mut blendColor = blendColor;
        let mut color = crate::ffi::Align4([0u8; 4]);
        let mut rBlend: u8 = 0u8;
        let mut gBlend: u8 = 0u8;
        let mut bBlend: u8 = 0u8;
        let mut curPalIndex: u16 = 0u16;
        let mut palOffset: u16 = 0u16;
        let mut i: u16 = 0u16;
        colorMapIndex = (((((colorMapIndex) as i32).wrapping_neg()).wrapping_sub(1i32)) as i8);
        (&raw mut color)
            .cast::<u8>()
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (&raw mut blendColor)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        rBlend = ((crate::c::bf_read(((&raw mut color).cast::<u8>()).wrapping_add(0), 0, 5, false)
            as u16) as u8);
        gBlend = ((crate::c::bf_read(((&raw mut color).cast::<u8>()).wrapping_add(0), 5, 5, false)
            as u16) as u8);
        bBlend = ((crate::c::bf_read(((&raw mut color).cast::<u8>()).wrapping_add(1), 2, 5, false)
            as u16) as u8);
        palOffset = 0u16;
        {
            curPalIndex = 0u16;
            'l1: loop {
                if !(((curPalIndex) as i32) < 32i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut sPaletteColorMapTypes)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((curPalIndex) as i32) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        BlendPalette(palOffset, 16u16, blendCoeff, blendColor);
                        palOffset = ((((palOffset) as i32).wrapping_add(16i32)) as u16);
                    } else {
                        {
                            i = 0u16;
                            'l3: loop {
                                if !(((i) as i32) < 16i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    let mut offset: u32 = 0u32;
                                    let mut color1 = crate::ffi::Align4([0u8; 4]);
                                    let mut color2 = crate::ffi::Align4([0u8; 4]);
                                    let mut r1: u8 = 0u8;
                                    let mut g1: u8 = 0u8;
                                    let mut b1: u8 = 0u8;
                                    let mut r2: u8 = 0u8;
                                    let mut g2: u8 = 0u8;
                                    let mut b2: u8 = 0u8;
                                    (&raw mut color1)
                                        .cast::<u8>()
                                        .cast::<crate::c::Rec4<4>>()
                                        .write_unaligned(
                                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(((palOffset) as i32) as isize))
                                            .cast::<u8>()
                                            .cast::<crate::c::Rec4<4>>()
                                            .read_unaligned(),
                                        );
                                    r1 = ((crate::c::bf_read(
                                        ((&raw mut color1).cast::<u8>()).wrapping_add(0),
                                        0,
                                        5,
                                        false,
                                    ) as u16) as u8);
                                    g1 = ((crate::c::bf_read(
                                        ((&raw mut color1).cast::<u8>()).wrapping_add(0),
                                        5,
                                        5,
                                        false,
                                    ) as u16) as u8);
                                    b1 = ((crate::c::bf_read(
                                        ((&raw mut color1).cast::<u8>()).wrapping_add(1),
                                        2,
                                        5,
                                        false,
                                    ) as u16) as u8);
                                    offset = (((((((b1) as i32) & 30i32) << 7)
                                        | ((((g1) as i32) & 30i32) << 3))
                                        | ((((r1) as i32) & 30i32) >> 1))
                                        as u32);
                                    (&raw mut color2)
                                        .cast::<u8>()
                                        .cast::<crate::c::Rec4<4>>()
                                        .write_unaligned(
                                            ((((((&raw const sDroughtWeatherColors)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((colorMapIndex) as i32) as isize * 8192,
                                            ))
                                            .cast::<u16>())
                                            .wrapping_offset(((offset) as i32) as isize))
                                            .cast::<u8>()
                                            .cast::<crate::c::Rec4<4>>()
                                            .read_unaligned(),
                                        );
                                    r2 = ((crate::c::bf_read(
                                        ((&raw mut color2).cast::<u8>()).wrapping_add(0),
                                        0,
                                        5,
                                        false,
                                    ) as u16) as u8);
                                    g2 = ((crate::c::bf_read(
                                        ((&raw mut color2).cast::<u8>()).wrapping_add(0),
                                        5,
                                        5,
                                        false,
                                    ) as u16) as u8);
                                    b2 = ((crate::c::bf_read(
                                        ((&raw mut color2).cast::<u8>()).wrapping_add(1),
                                        2,
                                        5,
                                        false,
                                    ) as u16) as u8);
                                    r2 = ((((r2) as i32).wrapping_add(
                                        ((((rBlend) as i32).wrapping_sub(((r2) as i32)))
                                            .wrapping_mul(((blendCoeff) as i32))
                                            >> 4),
                                    )) as u8);
                                    g2 = ((((g2) as i32).wrapping_add(
                                        ((((gBlend) as i32).wrapping_sub(((g2) as i32)))
                                            .wrapping_mul(((blendCoeff) as i32))
                                            >> 4),
                                    )) as u8);
                                    b2 = ((((b2) as i32).wrapping_add(
                                        ((((bBlend) as i32).wrapping_sub(((b2) as i32)))
                                            .wrapping_mul(((blendCoeff) as i32))
                                            >> 4),
                                    )) as u8);
                                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(
                                            (({
                                                let __t1 = palOffset;
                                                palOffset = (palOffset).wrapping_add(1);
                                                __t1
                                            }) as i32)
                                                as isize,
                                        ))
                                    .write(
                                        ((((((b2) as i32) << 10) | (((g2) as i32) << 5))
                                            | ((r2) as i32))
                                            as u16),
                                    );
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    }
                }
                curPalIndex = (curPalIndex).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyFogBlend(blendCoeff: u8, blendColor: u16) {
    unsafe {
        let mut blendCoeff = blendCoeff;
        let mut blendColor = blendColor;
        let mut color = crate::ffi::Align4([0u8; 4]);
        let mut rBlend: u8 = 0u8;
        let mut gBlend: u8 = 0u8;
        let mut bBlend: u8 = 0u8;
        let mut curPalIndex: u16 = 0u16;
        BlendPalette(0u16, 256u16, blendCoeff, blendColor);
        (&raw mut color)
            .cast::<u8>()
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (&raw mut blendColor)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        rBlend = ((crate::c::bf_read(((&raw mut color).cast::<u8>()).wrapping_add(0), 0, 5, false)
            as u16) as u8);
        gBlend = ((crate::c::bf_read(((&raw mut color).cast::<u8>()).wrapping_add(0), 5, 5, false)
            as u16) as u8);
        bBlend = ((crate::c::bf_read(((&raw mut color).cast::<u8>()).wrapping_add(1), 2, 5, false)
            as u16) as u8);
        {
            curPalIndex = 16u16;
            'l1: loop {
                if !(((curPalIndex) as i32) < 32i32) {
                    break 'l1;
                }
                'l2: {
                    if (LightenSpritePaletteInFog(((curPalIndex) as u8))) != 0 {
                        let mut palEnd: u16 = (((((curPalIndex) as i32).wrapping_add(1i32))
                            .wrapping_mul(16i32))
                            as u16);
                        let mut palOffset: u16 =
                            ((((curPalIndex) as i32).wrapping_mul(16i32)) as u16);
                        'l3: loop {
                            if !(((palOffset) as i32) < ((palEnd) as i32)) {
                                break 'l3;
                            }
                            let mut color = crate::ffi::Align4([0u8; 4]);
                            (&raw mut color)
                                .cast::<u8>()
                                .cast::<crate::c::Rec4<4>>()
                                .write_unaligned(
                                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(((palOffset) as i32) as isize))
                                    .cast::<u8>()
                                    .cast::<crate::c::Rec4<4>>()
                                    .read_unaligned(),
                                );
                            let mut r: u8 = ((crate::c::bf_read(
                                ((&raw mut color).cast::<u8>()).wrapping_add(0),
                                0,
                                5,
                                false,
                            ) as u16) as u8);
                            let mut g: u8 = ((crate::c::bf_read(
                                ((&raw mut color).cast::<u8>()).wrapping_add(0),
                                5,
                                5,
                                false,
                            ) as u16) as u8);
                            let mut b: u8 = ((crate::c::bf_read(
                                ((&raw mut color).cast::<u8>()).wrapping_add(1),
                                2,
                                5,
                                false,
                            ) as u16) as u8);
                            r = ((((r) as i32).wrapping_add(
                                (((28i32).wrapping_sub(((r) as i32))).wrapping_mul(3i32) >> 2),
                            )) as u8);
                            g = ((((g) as i32).wrapping_add(
                                (((31i32).wrapping_sub(((g) as i32))).wrapping_mul(3i32) >> 2),
                            )) as u8);
                            b = ((((b) as i32).wrapping_add(
                                (((28i32).wrapping_sub(((b) as i32))).wrapping_mul(3i32) >> 2),
                            )) as u8);
                            r = ((((r) as i32).wrapping_add(
                                ((((rBlend) as i32).wrapping_sub(((r) as i32)))
                                    .wrapping_mul(((blendCoeff) as i32))
                                    >> 4),
                            )) as u8);
                            g = ((((g) as i32).wrapping_add(
                                ((((gBlend) as i32).wrapping_sub(((g) as i32)))
                                    .wrapping_mul(((blendCoeff) as i32))
                                    >> 4),
                            )) as u8);
                            b = ((((b) as i32).wrapping_add(
                                ((((bBlend) as i32).wrapping_sub(((b) as i32)))
                                    .wrapping_mul(((blendCoeff) as i32))
                                    >> 4),
                            )) as u8);
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((palOffset) as i32) as isize))
                            .write(
                                ((((((b) as i32) << 10) | (((g) as i32) << 5)) | ((r) as i32))
                                    as u16),
                            );
                            palOffset = (palOffset).wrapping_add(1);
                        }
                    } else {
                        BlendPalette(
                            ((((curPalIndex) as i32).wrapping_mul(16i32)) as u16),
                            16u16,
                            blendCoeff,
                            blendColor,
                        );
                    }
                }
                curPalIndex = (curPalIndex).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn MarkFogSpritePalToLighten(paletteIndex: u8) {
    unsafe {
        let mut paletteIndex = paletteIndex;
        if ((((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1786))
        .read()) as i32)
            < 6i32
        {
            ((((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1780))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1786))
                .read()) as i32) as isize,
            ))
            .write(paletteIndex);
            let __p1 = (((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1786);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn LightenSpritePaletteInFog(paletteIndex: u8) -> u8 {
    unsafe {
        let mut paletteIndex = paletteIndex;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1786))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1780))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((paletteIndex) as i32)
                    {
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApplyWeatherColorMapIfIdle(colorMapIndex: i8) {
    unsafe {
        let mut colorMapIndex = colorMapIndex;
        if ((((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1734))
        .read()) as i32)
            == 3i32
        {
            ApplyColorMap(0u8, 32u8, colorMapIndex);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1728)
            .cast::<i8>())
            .write(colorMapIndex);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApplyWeatherColorMapIfIdle_Gradual(
    colorMapIndex: u8,
    targetColorMapIndex: u8,
    colorMapStepDelay: u8,
) {
    unsafe {
        let mut colorMapIndex = colorMapIndex;
        let mut targetColorMapIndex = targetColorMapIndex;
        let mut colorMapStepDelay = colorMapStepDelay;
        if ((((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1734))
        .read()) as i32)
            == 3i32
        {
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1734))
            .write(0u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1728)
            .cast::<i8>())
            .write(((colorMapIndex) as i8));
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1729)
            .cast::<i8>())
            .write(((targetColorMapIndex) as i8));
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1731))
            .write(0u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1730))
            .write(colorMapStepDelay);
            ApplyWeatherColorMapIfIdle(((colorMapIndex) as i8));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeScreen(mode: u8, delay: i8) {
    unsafe {
        let mut mode = mode;
        let mut delay = delay;
        let mut fadeColor: u32 = 0u32;
        let mut fadeOut: u8 = 0u8;
        let mut useWeatherPal: u8 = 0u8;
        'l1: {
            let __sw1 = ((mode) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 2i32 || __sw1 == 1i32 || __sw1 == 3i32;
            if __sw1 == 0i32 {
                fadeColor = 0u32;
                fadeOut = 0u8;
                break 'l1;
            }
            if __sw1 == 2i32 {
                fadeColor = 65535u32;
                fadeOut = 0u8;
                break 'l1;
            }
            if __sw1 == 1i32 {
                fadeColor = 0u32;
                fadeOut = 1u8;
                break 'l1;
            }
            if __sw1 == 3i32 {
                fadeColor = 65535u32;
                fadeOut = 1u8;
                break 'l1;
            }
            if !__matched {
                return;
            }
        }
        'l2: {
            let __sw2 = ((((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1744))
            .read()) as i32);
            let __matched = __sw2 == 3i32
                || __sw2 == 5i32
                || __sw2 == 13i32
                || __sw2 == 4i32
                || __sw2 == 6i32
                || __sw2 == 11i32
                || __sw2 == 12i32;
            if __sw2 == 3i32
                || __sw2 == 5i32
                || __sw2 == 13i32
                || __sw2 == 4i32
                || __sw2 == 6i32
                || __sw2 == 11i32
                || __sw2 == 12i32
            {
                useWeatherPal = 1u8;
                break 'l2;
            }
            if !__matched {
                useWeatherPal = 0u8;
                break 'l2;
            }
        }
        if (fadeOut) != 0 {
            if (useWeatherPal) != 0 {
                'l3: loop {
                    'l4: {
                        CpuFastSet(
                            (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .cast::<u8>(),
                            (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .cast::<u8>(),
                            ((crate::c::div_i32(1024i32, crate::c::div_i32(32i32, 8i32))
                                & 2097151i32) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l3;
                    }
                }
            }
            BeginNormalPaletteFade(4294967295u32, delay, 0u8, 16u8, ((fadeColor) as u16));
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1734))
            .write(2u8);
        } else {
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1732)
            .cast::<u16>())
            .write(((fadeColor) as u16));
            if (useWeatherPal) != 0 {
                ((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1735))
                .write(0u8);
            } else {
                BeginNormalPaletteFade(4294967295u32, delay, 16u8, 0u8, ((fadeColor) as u16));
            }
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1734))
            .write(1u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1738))
            .write(1u8);
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1739))
            .write(0u8);
            Weather_SetBlendCoeffs(
                ((((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1840)
                .cast::<u16>())
                .read()) as u8),
                ((((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1842)
                .cast::<u16>())
                .read()) as u8),
            );
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1736))
            .write(1u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsWeatherNotFadingIn() -> u8 {
    unsafe {
        return ((((((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1734))
        .read()) as i32)
            != 1i32) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateSpritePaletteWithWeather(spritePaletteIndex: u8) {
    unsafe {
        let mut spritePaletteIndex = spritePaletteIndex;
        let mut paletteIndex: u16 = (((16i32).wrapping_add(((spritePaletteIndex) as i32))) as u16);
        let mut i: u16 = 0u16;
        'l1: {
            let __sw1 = ((((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1734))
            .read()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 1i32 {
                if (((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1738))
                .read())
                    != 0
                {
                    if ((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1744))
                    .read()) as i32)
                        == 6i32
                    {
                        MarkFogSpritePalToLighten(((paletteIndex) as u8));
                    }
                    paletteIndex = ((((paletteIndex) as i32).wrapping_mul(16i32)) as u16);
                    {
                        i = 0u16;
                        'l2: loop {
                            if !(((i) as i32) < 16i32) {
                                break 'l2;
                            }
                            'l3: {
                                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        (((paletteIndex) as i32).wrapping_add(((i) as i32)))
                                            as isize,
                                    ))
                                .write(
                                    ((((&raw const gWeatherPtr)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(1732)
                                    .cast::<u16>())
                                    .read(),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                paletteIndex = ((((paletteIndex) as i32).wrapping_mul(16i32)) as u16);
                'l4: loop {
                    'l5: {
                        CpuFastSet(
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((paletteIndex) as i32) as isize))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((paletteIndex) as i32) as isize))
                            .cast::<u8>(),
                            (crate::c::div_u32(32u32, ((crate::c::div_i32(32i32, 8i32)) as u32))
                                & 2097151u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l4;
                    }
                }
                BlendPalette(
                    paletteIndex,
                    16u16,
                    ((crate::c::bf_read(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                        6,
                        5,
                        false,
                    ) as u16) as u8),
                    (crate::c::bf_read(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(6),
                        0,
                        15,
                        false,
                    ) as u16),
                );
                break 'l1;
            }
            if !__matched {
                if ((((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1744))
                .read()) as i32)
                    != 6i32
                {
                    ApplyColorMap(
                        ((paletteIndex) as u8),
                        1u8,
                        ((((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1728)
                        .cast::<i8>())
                        .read(),
                    );
                } else {
                    paletteIndex = ((((paletteIndex) as i32).wrapping_mul(16i32)) as u16);
                    BlendPalette(paletteIndex, 16u16, 12u8, 29692u16);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApplyWeatherColorMapToPal(paletteIndex: u8) {
    unsafe {
        let mut paletteIndex = paletteIndex;
        ApplyColorMap(
            paletteIndex,
            1u8,
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1728)
            .cast::<i8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn IsFirstFrameOfWeatherFadeIn() -> u8 {
    unsafe {
        if ((((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1734))
        .read()) as i32)
            == 1i32
        {
            return ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1738))
            .read();
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadCustomWeatherSpritePalette(palette: *mut u16) {
    unsafe {
        let mut palette = palette;
        LoadPalette(
            (palette).cast::<u8>(),
            (((256i32).wrapping_add(
                ((((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1748))
                .read()) as i32)
                    .wrapping_mul(16i32),
            )) as u16),
            32u16,
        );
        UpdateSpritePaletteWithWeather(
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1748))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn LoadDroughtWeatherPalette(palsIndex: *mut u8, palsOffset: *mut u8) {
    unsafe {
        let mut palsIndex = palsIndex;
        let mut palsOffset = palsOffset;
        (palsIndex).write(32u8);
        (palsOffset).write(32u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetDroughtWeatherPaletteLoading() {
    unsafe {
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1869)
        .cast::<i8>())
        .write(1i8);
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1870))
        .write(1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadDroughtWeatherPalettes() -> u8 {
    unsafe {
        if ((((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1869)
        .cast::<i8>())
        .read()) as i32)
            < 32i32
        {
            LoadDroughtWeatherPalette(
                ((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1869)
                .cast::<i8>())
                .cast::<u8>(),
                (((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1870),
            );
            if ((((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1869)
            .cast::<i8>())
            .read()) as i32)
                < 32i32
            {
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SetDroughtColorMap(colorMapIndex: i8) {
    unsafe {
        let mut colorMapIndex = colorMapIndex;
        ApplyWeatherColorMapIfIdle(
            (((((colorMapIndex) as i32).wrapping_neg()).wrapping_sub(1i32)) as i8),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DroughtStateInit() {
    unsafe {
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1852)
        .cast::<i16>())
        .write(0i16);
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1856)
        .cast::<i16>())
        .write(0i16);
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1858)
        .cast::<i16>())
        .write(0i16);
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1854)
        .cast::<i16>())
        .write(0i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DroughtStateRun() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1858)
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1856)
                    .cast::<i16>();
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 5i32
                {
                    ((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1856)
                    .cast::<i16>())
                    .write(0i16);
                    SetDroughtColorMap(
                        (({
                            let __p4 = (((&raw const gWeatherPtr)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(1852)
                            .cast::<i16>();
                            let __t5 = (__p4).read();
                            (__p4).write(((__p4).read()).wrapping_add(1));
                            __t5
                        }) as i8),
                    );
                    if ((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1852)
                    .cast::<i16>())
                    .read()) as i32)
                        > 5i32
                    {
                        ((((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1854)
                        .cast::<i16>())
                        .write(
                            ((((&raw const gWeatherPtr)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(1852)
                            .cast::<i16>())
                            .read(),
                        );
                        ((((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1858)
                        .cast::<i16>())
                        .write(1i16);
                        ((((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1856)
                        .cast::<i16>())
                        .write(60i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1856)
                .cast::<i16>())
                .write(
                    ((((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1856)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(3i32)
                        & 127i32) as i16),
                );
                ((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1852)
                .cast::<i16>())
                .write(
                    (((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        ((((((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1856)
                        .cast::<i16>())
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        .wrapping_sub(1i32)
                        >> 6)
                        .wrapping_add(2i32)) as i16),
                );
                if ((((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1852)
                .cast::<i16>())
                .read()) as i32)
                    != ((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1854)
                    .cast::<i16>())
                    .read()) as i32)
                {
                    SetDroughtColorMap(
                        ((((((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1852)
                        .cast::<i16>())
                        .read()) as i8),
                    );
                }
                ((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1854)
                .cast::<i16>())
                .write(
                    ((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1852)
                    .cast::<i16>())
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p6 = (((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1856)
                    .cast::<i16>();
                    let __t7 = ((__p6).read()).wrapping_add(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    > 5i32
                {
                    ((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1856)
                    .cast::<i16>())
                    .write(0i16);
                    SetDroughtColorMap(
                        (({
                            let __p8 = (((&raw const gWeatherPtr)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(1852)
                            .cast::<i16>();
                            let __t9 = ((__p8).read()).wrapping_sub(1);
                            (__p8).write(__t9);
                            __t9
                        }) as i8),
                    );
                    if ((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1852)
                    .cast::<i16>())
                    .read()) as i32)
                        == 3i32
                    {
                        ((((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1858)
                        .cast::<i16>())
                        .write(0i16);
                    }
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Weather_SetBlendCoeffs(eva: u8, evb: u8) {
    unsafe {
        let mut eva = eva;
        let mut evb = evb;
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1840)
        .cast::<u16>())
        .write(((eva) as u16));
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1842)
        .cast::<u16>())
        .write(((evb) as u16));
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1844)
        .cast::<u16>())
        .write(((eva) as u16));
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1846)
        .cast::<u16>())
        .write(((evb) as u16));
        SetGpuReg(82u8, (((((evb) as i32) << 8) | ((eva) as i32)) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Weather_SetTargetBlendCoeffs(eva: u8, evb: u8, delay: i32) {
    unsafe {
        let mut eva = eva;
        let mut evb = evb;
        let mut delay = delay;
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1844)
        .cast::<u16>())
        .write(((eva) as u16));
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1846)
        .cast::<u16>())
        .write(((evb) as u16));
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1850))
        .write(((delay) as u8));
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1849))
        .write(0u8);
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1848))
        .write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Weather_UpdateBlend() -> u8 {
    unsafe {
        if (((((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1840)
        .cast::<u16>())
        .read()) as i32)
            == ((((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1844)
            .cast::<u16>())
            .read()) as i32))
            && (((((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1842)
            .cast::<u16>())
            .read()) as i32)
                == ((((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1846)
                .cast::<u16>())
                .read()) as i32))
        {
            return 1u8;
        }
        if (({
            let __p1 = (((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1849);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > ((((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1850))
            .read()) as i32)
        {
            ((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1849))
            .write(0u8);
            let __p3 = (((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1848);
            (__p3).write(((__p3).read()).wrapping_add(1));
            if (((((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1848))
            .read()) as i32)
                & 1i32)
                != 0
            {
                if ((((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1840)
                .cast::<u16>())
                .read()) as i32)
                    < ((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1844)
                    .cast::<u16>())
                    .read()) as i32)
                {
                    let __p4 = (((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1840)
                    .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                } else {
                    if ((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1840)
                    .cast::<u16>())
                    .read()) as i32)
                        > ((((((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1844)
                        .cast::<u16>())
                        .read()) as i32)
                    {
                        let __p5 = (((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1840)
                        .cast::<u16>();
                        (__p5).write(((__p5).read()).wrapping_sub(1));
                    }
                }
            } else {
                if ((((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1842)
                .cast::<u16>())
                .read()) as i32)
                    < ((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1846)
                    .cast::<u16>())
                    .read()) as i32)
                {
                    let __p6 = (((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1842)
                    .cast::<u16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                } else {
                    if ((((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1842)
                    .cast::<u16>())
                    .read()) as i32)
                        > ((((((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1846)
                        .cast::<u16>())
                        .read()) as i32)
                    {
                        let __p7 = (((&raw const gWeatherPtr)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1842)
                        .cast::<u16>();
                        (__p7).write(((__p7).read()).wrapping_sub(1));
                    }
                }
            }
        }
        SetGpuReg(
            82u8,
            (((((((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1842)
            .cast::<u16>())
            .read()) as i32)
                << 8)
                | ((((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1840)
                .cast::<u16>())
                .read()) as i32)) as u16),
        );
        if (((((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1840)
        .cast::<u16>())
        .read()) as i32)
            == ((((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1844)
            .cast::<u16>())
            .read()) as i32))
            && (((((((&raw const gWeatherPtr)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1842)
            .cast::<u16>())
            .read()) as i32)
                == ((((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1846)
                .cast::<u16>())
                .read()) as i32))
        {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SetFieldWeather(weather: u8) {
    unsafe {
        let mut weather = weather;
        'l1: {
            let __sw1 = ((weather) as i32);
            if __sw1 == 1i32 {
                SetWeather(1u32);
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetWeather(2u32);
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetWeather(3u32);
                break 'l1;
            }
            if __sw1 == 4i32 {
                SetWeather(4u32);
                break 'l1;
            }
            if __sw1 == 5i32 {
                SetWeather(5u32);
                break 'l1;
            }
            if __sw1 == 6i32 {
                SetWeather(6u32);
                break 'l1;
            }
            if __sw1 == 7i32 {
                SetWeather(9u32);
                break 'l1;
            }
            if __sw1 == 8i32 {
                SetWeather(7u32);
                break 'l1;
            }
            if __sw1 == 9i32 {
                SetWeather(8u32);
                break 'l1;
            }
            if __sw1 == 10i32 {
                SetWeather(11u32);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentWeather() -> u8 {
    unsafe {
        return ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1744))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetRainStrengthFromSoundEffect(soundEffect: u16) {
    unsafe {
        let mut soundEffect = soundEffect;
        if ((((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1734))
        .read()) as i32)
            != 2i32
        {
            'l1: {
                let __sw1 = ((soundEffect) as i32);
                let __matched = __sw1 == 85i32 || __sw1 == 83i32 || __sw1 == 81i32;
                if __sw1 == 85i32 {
                    ((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1757))
                    .write(0u8);
                    break 'l1;
                }
                if __sw1 == 83i32 {
                    ((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1757))
                    .write(1u8);
                    break 'l1;
                }
                if __sw1 == 81i32 {
                    ((((&raw const gWeatherPtr)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1757))
                    .write(2u8);
                    break 'l1;
                }
                if !__matched {
                    return;
                }
            }
            PlaySE(soundEffect);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayRainStoppingSoundEffect() {
    unsafe {
        if (IsSpecialSEPlaying()) != 0 {
            'l1: {
                let __sw1 = ((((((&raw const gWeatherPtr)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1757))
                .read()) as i32);
                let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
                if __sw1 == 0i32 {
                    PlaySE(86u16);
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    PlaySE(84u16);
                    break 'l1;
                }
                if __sw1 == 2i32 || !__matched {
                    PlaySE(82u16);
                    break 'l1;
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsWeatherChangeComplete() -> u8 {
    unsafe {
        return ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1747))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWeatherScreenFadeOut() {
    unsafe {
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1734))
        .write(2u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWeatherPalStateIdle() {
    unsafe {
        ((((&raw const gWeatherPtr)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1734))
        .write(3u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PreservePaletteInWeather(preservedPalIndex: u8) {
    unsafe {
        let mut preservedPalIndex = preservedPalIndex;
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            ((&raw const sBasePaletteColorMapTypes)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>(),
                            ((&raw mut sFieldEffectPaletteColorMapTypes).cast::<u8>()).cast::<u8>(),
                            ((0i32
                                | (crate::c::div_i32(32i32, crate::c::div_i32(16i32, 8i32))
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
        ((((&raw mut sFieldEffectPaletteColorMapTypes).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((preservedPalIndex) as i32) as isize))
        .write(0u8);
        ((&raw mut sPaletteColorMapTypes)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(((&raw mut sFieldEffectPaletteColorMapTypes).cast::<u8>()).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetPreservedPalettesInWeather() {
    unsafe {
        ((&raw mut sPaletteColorMapTypes)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(
            ((&raw const sBasePaletteColorMapTypes)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
    }
}
