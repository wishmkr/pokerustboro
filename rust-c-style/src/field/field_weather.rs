//! Translated from `src/field_weather.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sDroughtWeatherColors gWeatherPtr sWeatherFuncs gWeatherPalStateFuncs sBasePaletteColorMapTypes gFogPalette

/// `struct RGBColor`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct RGBColor {
    bits_0: u16,
}

impl RGBColor {
    #[inline(always)]
    pub fn r(&self) -> u16 {
        ((self.bits_0 as u32 >> 0) & 0x1f) as u16
    }
    #[inline(always)]
    pub fn set_r(&mut self, v: u16) {
        self.bits_0 = (self.bits_0 & !(0x1f << 0)) | ((v as u16 & 0x1f) << 0);
    }
    #[inline(always)]
    pub fn g(&self) -> u16 {
        ((self.bits_0 as u32 >> 5) & 0x1f) as u16
    }
    #[inline(always)]
    pub fn set_g(&mut self, v: u16) {
        self.bits_0 = (self.bits_0 & !(0x1f << 5)) | ((v as u16 & 0x1f) << 5);
    }
    #[inline(always)]
    pub fn b(&self) -> u16 {
        ((self.bits_0 as u32 >> 10) & 0x1f) as u16
    }
    #[inline(always)]
    pub fn set_b(&mut self, v: u16) {
        self.bits_0 = (self.bits_0 & !(0x1f << 10)) | ((v as u16 & 0x1f) << 10);
    }
}

unsafe impl Sync for RGBColor {}

/// `struct WeatherCallbacks`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WeatherCallbacks {
    pub initVars: Option<unsafe extern "C" fn()>,
    pub main: Option<unsafe extern "C" fn()>,
    pub initAll: Option<unsafe extern "C" fn()>,
    pub finish: Option<unsafe extern "C" fn() -> u8>,
}

unsafe impl Sync for WeatherCallbacks {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<RGBColor>() == 4);
    assert!(offset_of!(RGBColor, bits_0) == 0);
    assert!(size_of::<WeatherCallbacks>() == 16);
    assert!(offset_of!(WeatherCallbacks, initVars) == 0);
    assert!(offset_of!(WeatherCallbacks, main) == 4);
    assert!(offset_of!(WeatherCallbacks, initAll) == 8);
    assert!(offset_of!(WeatherCallbacks, finish) == 12);
};

const COLOR_MAP_CONTRAST: u8 = 2;
const COLOR_MAP_DARK_CONTRAST: u8 = 1;
const COLOR_MAP_NONE: u8 = 0;

static gFogPalette: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::field_weather::gFogPalette).cast());
static gWeatherPalStateFuncs: Table<CArray<Option<unsafe extern "C" fn()>, 4>> =
    Table((&raw const crate::data::field_weather::gWeatherPalStateFuncs).cast());
static gWeatherPtr: Table<*mut Weather> =
    Table((&raw const crate::data::field_weather::gWeatherPtr).cast());
static sBasePaletteColorMapTypes: Table<CArray<u8, 32>> =
    Table((&raw const crate::data::field_weather::sBasePaletteColorMapTypes).cast());
static sDroughtWeatherColors: Table<CArray<CArray<u16, 4096>, 6>> =
    Table((&raw const crate::data::field_weather::sDroughtWeatherColors).cast());
static sWeatherFuncs: Table<CArray<WeatherCallbacks, 15>> =
    Table((&raw const crate::data::field_weather::sWeatherFuncs).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gWeather: Weather = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFieldEffectPaletteColorMapTypes: Aligned<CArray<u8, 32>> =
    Aligned(unsafe { zeroed() });
pub(crate) static mut sPaletteColorMapTypes: *mut u8 = null_mut();

unsafe extern "C" {
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static gSineTable: CArray<i16, 0>;
    static mut gTasks: CArray<Task, 0>;
    fn AllocSpritePalette(a0: u16) -> u8;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn IsSpecialSEPlaying() -> u8;
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn PlaySE(a0: u16);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetWeather(a0: u32);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartWeather() {
    if FuncIsActiveTask(Some(Task_WeatherMain)) == 0 {
        let mut index: u8 = AllocSpritePalette(PALTAG_WEATHER);
        CpuSet(
            gFogPalette.as_ptr().cast_mut() as *mut c_void,
            &raw mut gPlttBufferUnfaded[0x100 + index as i32 * 16] as *mut c_void,
            0x4000008,
        );
        BuildColorMaps();
        (*(*gWeatherPtr)).contrastColorMapSpritePalIndex = index;
        (*(*gWeatherPtr)).weatherPicSpritePalIndex = AllocSpritePalette(PALTAG_WEATHER_2);
        (*(*gWeatherPtr)).rainSpriteCount = 0;
        (*(*gWeatherPtr)).curRainSpriteIndex = 0;
        (*(*gWeatherPtr)).cloudSpritesCreated = 0;
        (*(*gWeatherPtr)).snowflakeSpriteCount = 0;
        (*(*gWeatherPtr)).ashSpritesCreated = 0;
        (*(*gWeatherPtr)).fogHSpritesCreated = 0;
        (*(*gWeatherPtr)).fogDSpritesCreated = 0;
        (*(*gWeatherPtr)).sandstormSpritesCreated = 0;
        (*(*gWeatherPtr)).sandstormSwirlSpritesCreated = 0;
        (*(*gWeatherPtr)).bubblesSpritesCreated = 0;
        (*(*gWeatherPtr)).lightenedFogSpritePalsCount = 0;
        Weather_SetBlendCoeffs(16, 0);
        (*(*gWeatherPtr)).currWeather = 0;
        (*(*gWeatherPtr)).palProcessingState = WEATHER_PAL_STATE_IDLE;
        (*(*gWeatherPtr)).readyForInit = FALSE;
        (*(*gWeatherPtr)).weatherChangeComplete = TRUE;
        (*(*gWeatherPtr)).taskId = CreateTask(Some(Task_WeatherInit), 80);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetNextWeather(weather: u8) {
    if weather != WEATHER_RAIN
        && weather != WEATHER_RAIN_THUNDERSTORM
        && weather != WEATHER_DOWNPOUR
    {
        PlayRainStoppingSoundEffect();
    }
    if (*(*gWeatherPtr)).nextWeather != weather && (*(*gWeatherPtr)).currWeather == weather {
        sWeatherFuncs[weather].initVars.unwrap_unchecked()();
    }
    (*(*gWeatherPtr)).weatherChangeComplete = FALSE;
    (*(*gWeatherPtr)).nextWeather = weather;
    (*(*gWeatherPtr)).finishStep = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCurrentAndNextWeather(weather: u8) {
    PlayRainStoppingSoundEffect();
    (*(*gWeatherPtr)).currWeather = weather;
    (*(*gWeatherPtr)).nextWeather = weather;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCurrentAndNextWeatherNoDelay(weather: u8) {
    PlayRainStoppingSoundEffect();
    (*(*gWeatherPtr)).currWeather = weather;
    (*(*gWeatherPtr)).nextWeather = weather;
    (*(*gWeatherPtr)).readyForInit = TRUE;
}
pub(crate) unsafe extern "C" fn Task_WeatherInit(taskId: u8) {
    if (*(*gWeatherPtr)).readyForInit != 0 {
        sWeatherFuncs[(*(*gWeatherPtr)).currWeather]
            .initAll
            .unwrap_unchecked()();
        gTasks[taskId].func = Some(Task_WeatherMain);
    }
}
pub(crate) unsafe extern "C" fn Task_WeatherMain(taskId: u8) {
    if (*(*gWeatherPtr)).currWeather != (*(*gWeatherPtr)).nextWeather {
        if sWeatherFuncs[(*(*gWeatherPtr)).currWeather]
            .finish
            .unwrap_unchecked()()
            == 0
            && (*(*gWeatherPtr)).palProcessingState != WEATHER_PAL_STATE_SCREEN_FADING_OUT
        {
            sWeatherFuncs[(*(*gWeatherPtr)).nextWeather]
                .initVars
                .unwrap_unchecked()();
            (*(*gWeatherPtr)).colorMapStepCounter = 0;
            (*(*gWeatherPtr)).palProcessingState = WEATHER_PAL_STATE_CHANGING_WEATHER;
            (*(*gWeatherPtr)).currWeather = (*(*gWeatherPtr)).nextWeather;
            (*(*gWeatherPtr)).weatherChangeComplete = TRUE;
        }
    } else {
        sWeatherFuncs[(*(*gWeatherPtr)).currWeather]
            .main
            .unwrap_unchecked()();
    }
    gWeatherPalStateFuncs[(*(*gWeatherPtr)).palProcessingState].unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn None_Init() {
    (*(*gWeatherPtr)).targetColorMapIndex = 0;
    (*(*gWeatherPtr)).colorMapStepDelay = 0;
}
pub(crate) unsafe extern "C" fn None_Main() {}
pub(crate) unsafe extern "C" fn None_Finish() -> u8 {
    return 0;
}
pub(crate) unsafe extern "C" fn BuildColorMaps() {
    let mut i: u16 = 0;
    let mut colorMaps: *mut CArray<u8, 32> = null_mut();
    let mut colorVal: u16 = 0;
    let mut curBrightness: u16 = 0;
    let mut brightnessDelta: u16 = 0;
    let mut colorMapIndex: u16 = 0;
    let mut baseBrightness: u16 = 0;
    let mut diff: i16 = 0;
    sPaletteColorMapTypes = sBasePaletteColorMapTypes.as_ptr().cast_mut();
    i = 0;
    while i < 2 {
        if i == 0 {
            colorMaps = (*(*gWeatherPtr)).darkenedContrastColorMaps.as_mut_ptr();
        } else {
            colorMaps = (*(*gWeatherPtr)).contrastColorMaps.as_mut_ptr();
        }
        colorVal = 0;
        while colorVal < 32 {
            curBrightness = colorVal << 8;
            if i == 0 {
                brightnessDelta = (((colorVal as i32) << 8) / 16) as u16;
            } else {
                brightnessDelta = 0;
            }
            colorMapIndex = 0;
            while colorMapIndex < 3 {
                curBrightness -= brightnessDelta;
                (*colorMaps.at(colorMapIndex))[colorVal] = (curBrightness >> 8) as u8;
                colorMapIndex += 1;
            }
            baseBrightness = curBrightness;
            brightnessDelta = ((0x1f00 - curBrightness as i32) / 16) as u16;
            if colorVal < 12 {
                while colorMapIndex < NUM_WEATHER_COLOR_MAPS {
                    curBrightness += brightnessDelta;
                    diff = curBrightness as i16 - baseBrightness as i16;
                    if diff > 0 {
                        curBrightness -= (diff / 2) as u16;
                    }
                    (*colorMaps.at(colorMapIndex))[colorVal] = (curBrightness >> 8) as u8;
                    if (*colorMaps.at(colorMapIndex))[colorVal] > 31 {
                        (*colorMaps.at(colorMapIndex))[colorVal] = 31;
                    }
                    colorMapIndex += 1;
                }
            } else {
                while colorMapIndex < NUM_WEATHER_COLOR_MAPS {
                    curBrightness += brightnessDelta;
                    (*colorMaps.at(colorMapIndex))[colorVal] = (curBrightness >> 8) as u8;
                    if (*colorMaps.at(colorMapIndex))[colorVal] > 31 {
                        (*colorMaps.at(colorMapIndex))[colorVal] = 31;
                    }
                    colorMapIndex += 1;
                }
            }
            colorVal += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn UpdateWeatherColorMap() {
    if (*(*gWeatherPtr)).palProcessingState != WEATHER_PAL_STATE_SCREEN_FADING_OUT {
        if (*(*gWeatherPtr)).colorMapIndex == (*(*gWeatherPtr)).targetColorMapIndex {
            (*(*gWeatherPtr)).palProcessingState = WEATHER_PAL_STATE_IDLE;
        } else {
            if ({
                (*(*gWeatherPtr)).colorMapStepCounter += 1;
                (*(*gWeatherPtr)).colorMapStepCounter
            }) >= (*(*gWeatherPtr)).colorMapStepDelay
            {
                (*(*gWeatherPtr)).colorMapStepCounter = 0;
                if (*(*gWeatherPtr)).colorMapIndex < (*(*gWeatherPtr)).targetColorMapIndex {
                    (*(*gWeatherPtr)).colorMapIndex += 1;
                } else {
                    (*(*gWeatherPtr)).colorMapIndex -= 1;
                }
                ApplyColorMap(0, 32, (*(*gWeatherPtr)).colorMapIndex);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FadeInScreenWithWeather() {
    if ({
        (*(*gWeatherPtr)).fadeInTimer += 1;
        (*(*gWeatherPtr)).fadeInTimer
    }) > 1
    {
        (*(*gWeatherPtr)).fadeInFirstFrame = FALSE;
    }
    match (*(*gWeatherPtr)).currWeather {
        WEATHER_RAIN
        | WEATHER_RAIN_THUNDERSTORM
        | WEATHER_DOWNPOUR
        | WEATHER_SNOW
        | WEATHER_SHADE => {
            if FadeInScreen_RainShowShade() == FALSE {
                (*(*gWeatherPtr)).colorMapIndex = 3;
                (*(*gWeatherPtr)).palProcessingState = WEATHER_PAL_STATE_IDLE;
            }
        }
        WEATHER_DROUGHT => {
            if FadeInScreen_Drought() == FALSE {
                (*(*gWeatherPtr)).colorMapIndex = -6;
                (*(*gWeatherPtr)).palProcessingState = WEATHER_PAL_STATE_IDLE;
            }
        }
        WEATHER_FOG_HORIZONTAL => {
            if FadeInScreen_FogHorizontal() == FALSE {
                (*(*gWeatherPtr)).colorMapIndex = 0;
                (*(*gWeatherPtr)).palProcessingState = WEATHER_PAL_STATE_IDLE;
            }
        }
        _ => {
            if gPaletteFade.active() == 0 {
                (*(*gWeatherPtr)).colorMapIndex = (*(*gWeatherPtr)).targetColorMapIndex;
                (*(*gWeatherPtr)).palProcessingState = WEATHER_PAL_STATE_IDLE;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FadeInScreen_RainShowShade() -> u8 {
    if (*(*gWeatherPtr)).fadeScreenCounter == 16 {
        return FALSE;
    }
    if ({
        (*(*gWeatherPtr)).fadeScreenCounter += 1;
        (*(*gWeatherPtr)).fadeScreenCounter
    }) >= 16
    {
        ApplyColorMap(0, 32, 3);
        (*(*gWeatherPtr)).fadeScreenCounter = 16;
        return FALSE;
    }
    ApplyColorMapWithBlend(
        0,
        32,
        3,
        16 - (*(*gWeatherPtr)).fadeScreenCounter,
        (*(*gWeatherPtr)).fadeDestColor,
    );
    return TRUE;
}
pub(crate) unsafe extern "C" fn FadeInScreen_Drought() -> u8 {
    if (*(*gWeatherPtr)).fadeScreenCounter == 16 {
        return FALSE;
    }
    if ({
        (*(*gWeatherPtr)).fadeScreenCounter += 1;
        (*(*gWeatherPtr)).fadeScreenCounter
    }) >= 16
    {
        ApplyColorMap(0, 32, -6);
        (*(*gWeatherPtr)).fadeScreenCounter = 16;
        return FALSE;
    }
    ApplyDroughtColorMapWithBlend(
        -6,
        16 - (*(*gWeatherPtr)).fadeScreenCounter,
        (*(*gWeatherPtr)).fadeDestColor,
    );
    return TRUE;
}
pub(crate) unsafe extern "C" fn FadeInScreen_FogHorizontal() -> u8 {
    if (*(*gWeatherPtr)).fadeScreenCounter == 16 {
        return FALSE;
    }
    (*(*gWeatherPtr)).fadeScreenCounter += 1;
    ApplyFogBlend(
        16 - (*(*gWeatherPtr)).fadeScreenCounter,
        (*(*gWeatherPtr)).fadeDestColor,
    );
    return TRUE;
}
pub(crate) unsafe extern "C" fn DoNothing() {}
pub(crate) unsafe extern "C" fn ApplyColorMap(
    startPalIndex: u8,
    mut numPalettes: u8,
    mut colorMapIndex: i8,
) {
    let mut curPalIndex: u16 = 0;
    let mut palOffset: u16 = 0;
    let mut colorMap: *mut u8 = null_mut();
    let mut i: u16 = 0;
    if colorMapIndex > 0 {
        colorMapIndex -= 1;
        palOffset = startPalIndex as u16 * 16;
        numPalettes += startPalIndex;
        curPalIndex = startPalIndex as u16;
        while curPalIndex < numPalettes as u16 {
            if *sPaletteColorMapTypes.at(curPalIndex) == COLOR_MAP_NONE {
                CpuFastSet(
                    &raw mut gPlttBufferUnfaded[palOffset] as *mut c_void,
                    &raw mut gPlttBufferFaded[palOffset] as *mut c_void,
                    8,
                );
                palOffset += 16;
            } else {
                let mut r: u8 = 0;
                let mut g: u8 = 0;
                let mut b: u8 = 0;
                if *sPaletteColorMapTypes.at(curPalIndex) == COLOR_MAP_CONTRAST
                    || curPalIndex as i32 - 16
                        == (*(*gWeatherPtr)).contrastColorMapSpritePalIndex as i32
                {
                    colorMap = (*(*gWeatherPtr)).contrastColorMaps[colorMapIndex].as_mut_ptr();
                } else {
                    colorMap =
                        (*(*gWeatherPtr)).darkenedContrastColorMaps[colorMapIndex].as_mut_ptr();
                }
                i = 0;
                while i < 16 {
                    let mut baseColor: RGBColor = zeroed();
                    baseColor = *(&raw mut gPlttBufferUnfaded[palOffset] as *mut RGBColor);
                    r = *colorMap.at(baseColor.r());
                    g = *colorMap.at(baseColor.g());
                    b = *colorMap.at(baseColor.b());
                    gPlttBufferFaded[{
                        let t1 = palOffset;
                        palOffset += 1;
                        t1
                    }] = (b as u16) << 10 | (g as u16) << 5 | r as u16;
                    i += 1;
                }
            }
            curPalIndex += 1;
        }
    } else if colorMapIndex < 0 {
        colorMapIndex = -colorMapIndex - 1;
        palOffset = startPalIndex as u16 * 16;
        numPalettes += startPalIndex;
        curPalIndex = startPalIndex as u16;
        while curPalIndex < numPalettes as u16 {
            if *sPaletteColorMapTypes.at(curPalIndex) == COLOR_MAP_NONE {
                CpuFastSet(
                    &raw mut gPlttBufferUnfaded[palOffset] as *mut c_void,
                    &raw mut gPlttBufferFaded[palOffset] as *mut c_void,
                    8,
                );
                palOffset += 16;
            } else {
                i = 0;
                while i < 16 {
                    gPlttBufferFaded[palOffset] = sDroughtWeatherColors[colorMapIndex]
                        [(gPlttBufferUnfaded[palOffset] >> 1) as i32 & 0xF
                            | (gPlttBufferUnfaded[palOffset] >> 2) as i32 & 0xF0
                            | (gPlttBufferUnfaded[palOffset] >> 3) as i32 & 0xF00];
                    palOffset += 1;
                    i += 1;
                }
            }
            curPalIndex += 1;
        }
    } else {
        CpuFastSet(
            &raw mut gPlttBufferUnfaded[startPalIndex as i32 * 16] as *mut c_void,
            &raw mut gPlttBufferFaded[startPalIndex as i32 * 16] as *mut c_void,
            numPalettes as u32 * 32 / 4 & 0x1FFFFF,
        );
    }
}
pub(crate) unsafe extern "C" fn ApplyColorMapWithBlend(
    startPalIndex: u8,
    mut numPalettes: u8,
    mut colorMapIndex: i8,
    blendCoeff: u8,
    mut blendColor: u16,
) {
    let mut palOffset: u16 = 0;
    let mut curPalIndex: u16 = 0;
    let mut i: u16 = 0;
    let mut color: RGBColor = zeroed();
    color = *(&raw mut blendColor as *mut RGBColor);
    let mut rBlend: u8 = color.r() as u8;
    let mut gBlend: u8 = color.g() as u8;
    let mut bBlend: u8 = color.b() as u8;
    palOffset = startPalIndex as u16 * 16;
    numPalettes += startPalIndex;
    colorMapIndex -= 1;
    curPalIndex = startPalIndex as u16;
    while curPalIndex < numPalettes as u16 {
        if *sPaletteColorMapTypes.at(curPalIndex) == COLOR_MAP_NONE {
            BlendPalette(palOffset, 16, blendCoeff, blendColor);
            palOffset += 16;
        } else {
            let mut colorMap: *mut u8 = null_mut();
            if *sPaletteColorMapTypes.at(curPalIndex) == COLOR_MAP_DARK_CONTRAST {
                colorMap = (*(*gWeatherPtr)).darkenedContrastColorMaps[colorMapIndex].as_mut_ptr();
            } else {
                colorMap = (*(*gWeatherPtr)).contrastColorMaps[colorMapIndex].as_mut_ptr();
            }
            i = 0;
            while i < 16 {
                let mut baseColor: RGBColor = zeroed();
                baseColor = *(&raw mut gPlttBufferUnfaded[palOffset] as *mut RGBColor);
                let mut r: u8 = *colorMap.at(baseColor.r());
                let mut g: u8 = *colorMap.at(baseColor.g());
                let mut b: u8 = *colorMap.at(baseColor.b());
                r += ((rBlend as i32 - r as i32) * blendCoeff as i32 >> 4) as u8;
                g += ((gBlend as i32 - g as i32) * blendCoeff as i32 >> 4) as u8;
                b += ((bBlend as i32 - b as i32) * blendCoeff as i32 >> 4) as u8;
                gPlttBufferFaded[{
                    let t1 = palOffset;
                    palOffset += 1;
                    t1
                }] = (b as u16) << 10 | (g as u16) << 5 | r as u16;
                i += 1;
            }
        }
        curPalIndex += 1;
    }
}
pub(crate) unsafe extern "C" fn ApplyDroughtColorMapWithBlend(
    mut colorMapIndex: i8,
    blendCoeff: u8,
    mut blendColor: u16,
) {
    let mut color: RGBColor = zeroed();
    let mut rBlend: u8 = 0;
    let mut gBlend: u8 = 0;
    let mut bBlend: u8 = 0;
    let mut curPalIndex: u16 = 0;
    let mut palOffset: u16 = 0;
    let mut i: u16 = 0;
    colorMapIndex = -colorMapIndex - 1;
    color = *(&raw mut blendColor as *mut RGBColor);
    rBlend = color.r() as u8;
    gBlend = color.g() as u8;
    bBlend = color.b() as u8;
    palOffset = 0;
    curPalIndex = 0;
    while curPalIndex < 32 {
        if *sPaletteColorMapTypes.at(curPalIndex) == COLOR_MAP_NONE {
            BlendPalette(palOffset, 16, blendCoeff, blendColor);
            palOffset += 16;
        } else {
            i = 0;
            while i < 16 {
                let mut offset: u32 = 0;
                let mut color1: RGBColor = zeroed();
                let mut color2: RGBColor = zeroed();
                let mut r1: u8 = 0;
                let mut g1: u8 = 0;
                let mut b1: u8 = 0;
                let mut r2: u8 = 0;
                let mut g2: u8 = 0;
                let mut b2: u8 = 0;
                color1 = *(&raw mut gPlttBufferUnfaded[palOffset] as *mut RGBColor);
                r1 = color1.r() as u8;
                g1 = color1.g() as u8;
                b1 = color1.b() as u8;
                offset = (b1 as u32 & 0x1E) << 7
                    | (g1 as u32 & 0x1E) << 3
                    | ((r1 as i32 & 0x1E) >> 1) as u32;
                color2 = *((&raw const sDroughtWeatherColors[colorMapIndex][offset]).cast_mut()
                    as *mut RGBColor);
                r2 = color2.r() as u8;
                g2 = color2.g() as u8;
                b2 = color2.b() as u8;
                r2 += ((rBlend as i32 - r2 as i32) * blendCoeff as i32 >> 4) as u8;
                g2 += ((gBlend as i32 - g2 as i32) * blendCoeff as i32 >> 4) as u8;
                b2 += ((bBlend as i32 - b2 as i32) * blendCoeff as i32 >> 4) as u8;
                gPlttBufferFaded[{
                    let t1 = palOffset;
                    palOffset += 1;
                    t1
                }] = (b2 as u16) << 10 | (g2 as u16) << 5 | r2 as u16;
                i += 1;
            }
        }
        curPalIndex += 1;
    }
}
pub(crate) unsafe extern "C" fn ApplyFogBlend(blendCoeff: u8, mut blendColor: u16) {
    let mut color: RGBColor = zeroed();
    let mut rBlend: u8 = 0;
    let mut gBlend: u8 = 0;
    let mut bBlend: u8 = 0;
    let mut curPalIndex: u16 = 0;
    BlendPalette(0, 256, blendCoeff, blendColor);
    color = *(&raw mut blendColor as *mut RGBColor);
    rBlend = color.r() as u8;
    gBlend = color.g() as u8;
    bBlend = color.b() as u8;
    curPalIndex = 16;
    while curPalIndex < 32 {
        if LightenSpritePaletteInFog(curPalIndex as u8) != 0 {
            let mut palEnd: u16 = (curPalIndex + 1) * 16;
            let mut palOffset: u16 = curPalIndex * 16;
            while palOffset < palEnd {
                let mut color: RGBColor = zeroed();
                color = *(&raw mut gPlttBufferUnfaded[palOffset] as *mut RGBColor);
                let mut r: u8 = color.r() as u8;
                let mut g: u8 = color.g() as u8;
                let mut b: u8 = color.b() as u8;
                r += ((28 - r as i32) * 3 >> 2) as u8;
                g += ((31 - g as i32) * 3 >> 2) as u8;
                b += ((28 - b as i32) * 3 >> 2) as u8;
                r += ((rBlend as i32 - r as i32) * blendCoeff as i32 >> 4) as u8;
                g += ((gBlend as i32 - g as i32) * blendCoeff as i32 >> 4) as u8;
                b += ((bBlend as i32 - b as i32) * blendCoeff as i32 >> 4) as u8;
                gPlttBufferFaded[palOffset] = (b as u16) << 10 | (g as u16) << 5 | r as u16;
                palOffset += 1;
            }
        } else {
            BlendPalette(curPalIndex * 16, 16, blendCoeff, blendColor);
        }
        curPalIndex += 1;
    }
}
pub(crate) unsafe extern "C" fn MarkFogSpritePalToLighten(paletteIndex: u8) {
    if (*(*gWeatherPtr)).lightenedFogSpritePalsCount < 6 {
        (*(*gWeatherPtr)).lightenedFogSpritePals[(*(*gWeatherPtr)).lightenedFogSpritePalsCount] =
            paletteIndex;
        (*(*gWeatherPtr)).lightenedFogSpritePalsCount += 1;
    }
}
pub(crate) unsafe extern "C" fn LightenSpritePaletteInFog(paletteIndex: u8) -> u8 {
    let mut i: u16 = 0;
    i = 0;
    while i < (*(*gWeatherPtr)).lightenedFogSpritePalsCount as u16 {
        if (*(*gWeatherPtr)).lightenedFogSpritePals[i] == paletteIndex {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApplyWeatherColorMapIfIdle(colorMapIndex: i8) {
    if (*(*gWeatherPtr)).palProcessingState == WEATHER_PAL_STATE_IDLE {
        ApplyColorMap(0, 32, colorMapIndex);
        (*(*gWeatherPtr)).colorMapIndex = colorMapIndex;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApplyWeatherColorMapIfIdle_Gradual(
    colorMapIndex: u8,
    targetColorMapIndex: u8,
    colorMapStepDelay: u8,
) {
    if (*(*gWeatherPtr)).palProcessingState == WEATHER_PAL_STATE_IDLE {
        (*(*gWeatherPtr)).palProcessingState = WEATHER_PAL_STATE_CHANGING_WEATHER;
        (*(*gWeatherPtr)).colorMapIndex = colorMapIndex as i8;
        (*(*gWeatherPtr)).targetColorMapIndex = targetColorMapIndex as i8;
        (*(*gWeatherPtr)).colorMapStepCounter = 0;
        (*(*gWeatherPtr)).colorMapStepDelay = colorMapStepDelay;
        ApplyWeatherColorMapIfIdle(colorMapIndex as i8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeScreen(mode: u8, delay: i8) {
    let mut fadeColor: u32 = 0;
    let mut fadeOut: u8 = 0;
    let mut useWeatherPal: u8 = 0;
    match mode {
        FADE_FROM_BLACK => {
            fadeColor = 0;
            fadeOut = FALSE;
        }
        FADE_FROM_WHITE => {
            fadeColor = 65535;
            fadeOut = FALSE;
        }
        FADE_TO_BLACK => {
            fadeColor = 0;
            fadeOut = TRUE;
        }
        FADE_TO_WHITE => {
            fadeColor = 65535;
            fadeOut = TRUE;
        }
        _ => {
            return;
        }
    }
    match (*(*gWeatherPtr)).currWeather {
        WEATHER_RAIN
        | WEATHER_RAIN_THUNDERSTORM
        | WEATHER_DOWNPOUR
        | WEATHER_SNOW
        | WEATHER_FOG_HORIZONTAL
        | WEATHER_SHADE
        | WEATHER_DROUGHT => {
            useWeatherPal = TRUE;
        }
        _ => {
            useWeatherPal = FALSE;
        }
    }
    if fadeOut != 0 {
        if useWeatherPal != 0 {
            CpuFastSet(
                gPlttBufferFaded.as_mut_ptr() as *mut c_void,
                gPlttBufferUnfaded.as_mut_ptr() as *mut c_void,
                256,
            );
        }
        BeginNormalPaletteFade(PALETTES_ALL, delay, 0, 16, fadeColor as u16);
        (*(*gWeatherPtr)).palProcessingState = WEATHER_PAL_STATE_SCREEN_FADING_OUT;
    } else {
        (*(*gWeatherPtr)).fadeDestColor = fadeColor as u16;
        if useWeatherPal != 0 {
            (*(*gWeatherPtr)).fadeScreenCounter = 0;
        } else {
            BeginNormalPaletteFade(PALETTES_ALL, delay, 16, 0, fadeColor as u16);
        }
        (*(*gWeatherPtr)).palProcessingState = WEATHER_PAL_STATE_SCREEN_FADING_IN;
        (*(*gWeatherPtr)).fadeInFirstFrame = TRUE;
        (*(*gWeatherPtr)).fadeInTimer = 0;
        Weather_SetBlendCoeffs(
            (*(*gWeatherPtr)).currBlendEVA as u8,
            (*(*gWeatherPtr)).currBlendEVB as u8,
        );
        (*(*gWeatherPtr)).readyForInit = TRUE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsWeatherNotFadingIn() -> u8 {
    return ((*(*gWeatherPtr)).palProcessingState != WEATHER_PAL_STATE_SCREEN_FADING_IN) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateSpritePaletteWithWeather(spritePaletteIndex: u8) {
    let mut paletteIndex: u16 = 16 + spritePaletteIndex as u16;
    let mut i: u16 = 0;
    match (*(*gWeatherPtr)).palProcessingState {
        WEATHER_PAL_STATE_SCREEN_FADING_IN => {
            if (*(*gWeatherPtr)).fadeInFirstFrame != 0 {
                if (*(*gWeatherPtr)).currWeather == WEATHER_FOG_HORIZONTAL {
                    MarkFogSpritePalToLighten(paletteIndex as u8);
                }
                paletteIndex = paletteIndex * 16;
                i = 0;
                while i < 16 {
                    gPlttBufferFaded[paletteIndex as i32 + i as i32] =
                        (*(*gWeatherPtr)).fadeDestColor;
                    i += 1;
                }
            }
        }
        WEATHER_PAL_STATE_SCREEN_FADING_OUT => {
            paletteIndex = paletteIndex * 16;
            CpuFastSet(
                &raw mut gPlttBufferFaded[paletteIndex] as *mut c_void,
                &raw mut gPlttBufferUnfaded[paletteIndex] as *mut c_void,
                8,
            );
            BlendPalette(
                paletteIndex,
                16,
                gPaletteFade.y() as u8,
                gPaletteFade.blendColor(),
            );
        }
        _ => {
            if (*(*gWeatherPtr)).currWeather != WEATHER_FOG_HORIZONTAL {
                ApplyColorMap(paletteIndex as u8, 1, (*(*gWeatherPtr)).colorMapIndex);
            } else {
                paletteIndex = paletteIndex * 16;
                BlendPalette(paletteIndex, 16, 12, 29692);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApplyWeatherColorMapToPal(paletteIndex: u8) {
    ApplyColorMap(paletteIndex, 1, (*(*gWeatherPtr)).colorMapIndex);
}
pub(crate) unsafe extern "C" fn IsFirstFrameOfWeatherFadeIn() -> u8 {
    if (*(*gWeatherPtr)).palProcessingState == WEATHER_PAL_STATE_SCREEN_FADING_IN {
        return (*(*gWeatherPtr)).fadeInFirstFrame;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadCustomWeatherSpritePalette(palette: *mut u16) {
    LoadPalette(
        palette as *mut c_void,
        0x100 + (*(*gWeatherPtr)).weatherPicSpritePalIndex as u16 * 16,
        32,
    );
    UpdateSpritePaletteWithWeather((*(*gWeatherPtr)).weatherPicSpritePalIndex);
}
pub(crate) unsafe extern "C" fn LoadDroughtWeatherPalette(palsIndex: *mut u8, palsOffset: *mut u8) {
    *palsIndex = 0x20;
    *palsOffset = 0x20;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetDroughtWeatherPaletteLoading() {
    (*(*gWeatherPtr)).loadDroughtPalsIndex = 1;
    (*(*gWeatherPtr)).loadDroughtPalsOffset = 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadDroughtWeatherPalettes() -> u8 {
    if (*(*gWeatherPtr)).loadDroughtPalsIndex < 32 {
        LoadDroughtWeatherPalette(
            &raw mut (*(*gWeatherPtr)).loadDroughtPalsIndex as *mut u8,
            &raw mut (*(*gWeatherPtr)).loadDroughtPalsOffset,
        );
        if (*(*gWeatherPtr)).loadDroughtPalsIndex < 32 {
            return TRUE;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SetDroughtColorMap(colorMapIndex: i8) {
    ApplyWeatherColorMapIfIdle(-colorMapIndex - 1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DroughtStateInit() {
    (*(*gWeatherPtr)).droughtBrightnessStage = 0;
    (*(*gWeatherPtr)).droughtTimer = 0;
    (*(*gWeatherPtr)).droughtState = 0;
    (*(*gWeatherPtr)).droughtLastBrightnessStage = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DroughtStateRun() {
    match (*(*gWeatherPtr)).droughtState {
        0 => {
            if ({
                (*(*gWeatherPtr)).droughtTimer += 1;
                (*(*gWeatherPtr)).droughtTimer
            }) > 5
            {
                (*(*gWeatherPtr)).droughtTimer = 0;
                SetDroughtColorMap(
                    ({
                        let t2 = (*(*gWeatherPtr)).droughtBrightnessStage;
                        (*(*gWeatherPtr)).droughtBrightnessStage += 1;
                        t2
                    }) as i8,
                );
                if (*(*gWeatherPtr)).droughtBrightnessStage > 5 {
                    (*(*gWeatherPtr)).droughtLastBrightnessStage =
                        (*(*gWeatherPtr)).droughtBrightnessStage;
                    (*(*gWeatherPtr)).droughtState = 1;
                    (*(*gWeatherPtr)).droughtTimer = 60;
                }
            }
        }
        1 => {
            (*(*gWeatherPtr)).droughtTimer = (*(*gWeatherPtr)).droughtTimer + 3 & 0x7F;
            (*(*gWeatherPtr)).droughtBrightnessStage =
                (gSineTable[(*(*gWeatherPtr)).droughtTimer] as i32 - 1 >> 6) as i16 + 2;
            if (*(*gWeatherPtr)).droughtBrightnessStage
                != (*(*gWeatherPtr)).droughtLastBrightnessStage
            {
                SetDroughtColorMap((*(*gWeatherPtr)).droughtBrightnessStage as i8);
            }
            (*(*gWeatherPtr)).droughtLastBrightnessStage = (*(*gWeatherPtr)).droughtBrightnessStage;
        }
        2 => {
            if ({
                (*(*gWeatherPtr)).droughtTimer += 1;
                (*(*gWeatherPtr)).droughtTimer
            }) > 5
            {
                (*(*gWeatherPtr)).droughtTimer = 0;
                SetDroughtColorMap(
                    ({
                        (*(*gWeatherPtr)).droughtBrightnessStage -= 1;
                        (*(*gWeatherPtr)).droughtBrightnessStage
                    }) as i8,
                );
                if (*(*gWeatherPtr)).droughtBrightnessStage == 3 {
                    (*(*gWeatherPtr)).droughtState = 0;
                }
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Weather_SetBlendCoeffs(eva: u8, evb: u8) {
    (*(*gWeatherPtr)).currBlendEVA = eva as u16;
    (*(*gWeatherPtr)).currBlendEVB = evb as u16;
    (*(*gWeatherPtr)).targetBlendEVA = eva as u16;
    (*(*gWeatherPtr)).targetBlendEVB = evb as u16;
    SetGpuReg(REG_OFFSET_BLDALPHA, (evb as u16) << 8 | eva as u16);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Weather_SetTargetBlendCoeffs(eva: u8, evb: u8, delay: i32) {
    (*(*gWeatherPtr)).targetBlendEVA = eva as u16;
    (*(*gWeatherPtr)).targetBlendEVB = evb as u16;
    (*(*gWeatherPtr)).blendDelay = delay as u8;
    (*(*gWeatherPtr)).blendFrameCounter = 0;
    (*(*gWeatherPtr)).blendUpdateCounter = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Weather_UpdateBlend() -> u8 {
    if (*(*gWeatherPtr)).currBlendEVA == (*(*gWeatherPtr)).targetBlendEVA
        && (*(*gWeatherPtr)).currBlendEVB == (*(*gWeatherPtr)).targetBlendEVB
    {
        return TRUE;
    }
    if ({
        (*(*gWeatherPtr)).blendFrameCounter += 1;
        (*(*gWeatherPtr)).blendFrameCounter
    }) > (*(*gWeatherPtr)).blendDelay
    {
        (*(*gWeatherPtr)).blendFrameCounter = 0;
        (*(*gWeatherPtr)).blendUpdateCounter += 1;
        if (*(*gWeatherPtr)).blendUpdateCounter as i32 & 1 != 0 {
            if (*(*gWeatherPtr)).currBlendEVA < (*(*gWeatherPtr)).targetBlendEVA {
                (*(*gWeatherPtr)).currBlendEVA += 1;
            } else if (*(*gWeatherPtr)).currBlendEVA > (*(*gWeatherPtr)).targetBlendEVA {
                (*(*gWeatherPtr)).currBlendEVA -= 1;
            }
        } else {
            if (*(*gWeatherPtr)).currBlendEVB < (*(*gWeatherPtr)).targetBlendEVB {
                (*(*gWeatherPtr)).currBlendEVB += 1;
            } else if (*(*gWeatherPtr)).currBlendEVB > (*(*gWeatherPtr)).targetBlendEVB {
                (*(*gWeatherPtr)).currBlendEVB -= 1;
            }
        }
    }
    SetGpuReg(
        REG_OFFSET_BLDALPHA,
        (*(*gWeatherPtr)).currBlendEVB << 8 | (*(*gWeatherPtr)).currBlendEVA,
    );
    if (*(*gWeatherPtr)).currBlendEVA == (*(*gWeatherPtr)).targetBlendEVA
        && (*(*gWeatherPtr)).currBlendEVB == (*(*gWeatherPtr)).targetBlendEVB
    {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SetFieldWeather(weather: u8) {
    match weather {
        COORD_EVENT_WEATHER_SUNNY_CLOUDS => {
            SetWeather(WEATHER_SUNNY_CLOUDS as u32);
        }
        COORD_EVENT_WEATHER_SUNNY => {
            SetWeather(WEATHER_SUNNY as u32);
        }
        COORD_EVENT_WEATHER_RAIN => {
            SetWeather(WEATHER_RAIN as u32);
        }
        COORD_EVENT_WEATHER_SNOW => {
            SetWeather(WEATHER_SNOW as u32);
        }
        COORD_EVENT_WEATHER_RAIN_THUNDERSTORM => {
            SetWeather(WEATHER_RAIN_THUNDERSTORM as u32);
        }
        COORD_EVENT_WEATHER_FOG_HORIZONTAL => {
            SetWeather(WEATHER_FOG_HORIZONTAL as u32);
        }
        COORD_EVENT_WEATHER_FOG_DIAGONAL => {
            SetWeather(WEATHER_FOG_DIAGONAL as u32);
        }
        COORD_EVENT_WEATHER_VOLCANIC_ASH => {
            SetWeather(WEATHER_VOLCANIC_ASH as u32);
        }
        COORD_EVENT_WEATHER_SANDSTORM => {
            SetWeather(WEATHER_SANDSTORM as u32);
        }
        COORD_EVENT_WEATHER_SHADE => {
            SetWeather(WEATHER_SHADE as u32);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentWeather() -> u8 {
    return (*(*gWeatherPtr)).currWeather;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetRainStrengthFromSoundEffect(soundEffect: u16) {
    if (*(*gWeatherPtr)).palProcessingState != WEATHER_PAL_STATE_SCREEN_FADING_OUT {
        match soundEffect {
            SE_RAIN => {
                (*(*gWeatherPtr)).rainStrength = 0;
            }
            SE_DOWNPOUR => {
                (*(*gWeatherPtr)).rainStrength = 1;
            }
            SE_THUNDERSTORM => {
                (*(*gWeatherPtr)).rainStrength = 2;
            }
            _ => {
                return;
            }
        }
        PlaySE(soundEffect);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayRainStoppingSoundEffect() {
    if IsSpecialSEPlaying() != 0 {
        match (*(*gWeatherPtr)).rainStrength {
            0 => {
                PlaySE(SE_RAIN_STOP);
            }
            1 => {
                PlaySE(SE_DOWNPOUR_STOP);
            }
            _ => {
                PlaySE(SE_THUNDERSTORM_STOP);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsWeatherChangeComplete() -> u8 {
    return (*(*gWeatherPtr)).weatherChangeComplete;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWeatherScreenFadeOut() {
    (*(*gWeatherPtr)).palProcessingState = WEATHER_PAL_STATE_SCREEN_FADING_OUT;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWeatherPalStateIdle() {
    (*(*gWeatherPtr)).palProcessingState = WEATHER_PAL_STATE_IDLE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PreservePaletteInWeather(preservedPalIndex: u8) {
    CpuSet(
        sBasePaletteColorMapTypes.as_ptr().cast_mut() as *mut c_void,
        sFieldEffectPaletteColorMapTypes.as_mut_ptr() as *mut c_void,
        16,
    );
    sFieldEffectPaletteColorMapTypes[preservedPalIndex] = COLOR_MAP_NONE;
    sPaletteColorMapTypes = sFieldEffectPaletteColorMapTypes.as_mut_ptr();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetPreservedPalettesInWeather() {
    sPaletteColorMapTypes = sBasePaletteColorMapTypes.as_ptr().cast_mut();
}
