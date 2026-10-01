//! Translated from `src/image_processing_effects.c` by tools/rustport/c2rs.py.
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
    clippy::manual_clamp,
    unused_assignments,
    unused_variables
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
// Data tables (translate with cdata.py): sPointillismPoints

/// `struct PointillismPoint`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct PointillismPoint {
    pub column: u8,
    pub row: u8,
    pub delta: u16,
}

unsafe impl Sync for PointillismPoint {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PointillismPoint>() == 4);
    assert!(offset_of!(PointillismPoint, column) == 0);
    assert!(offset_of!(PointillismPoint, row) == 1);
    assert!(offset_of!(PointillismPoint, delta) == 2);
};

const MAX_DIMENSION: u8 = 64;

static sPointillismPoints: Table<CArray<CArray<u8, 3>, 3200>> =
    Table((&raw const crate::data::image_processing_effects::sPointillismPoints).cast());

#[unsafe(link_section = "common_data")]
pub static gCanvasColumnStart: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static mut gCanvasPixels: *mut u16 = null_mut();
#[unsafe(link_section = "common_data")]
pub static gCanvasRowEnd: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static gCanvasHeight: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static gCanvasColumnEnd: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static gCanvasRowStart: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static gCanvasMonPersonality: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static gCanvasWidth: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static mut gCanvasPalette: *mut u16 = null_mut();
#[unsafe(link_section = "common_data")]
pub static gCanvasPaletteStart: crate::global::Global<u16> = crate::global::Global::new(0);

pub unsafe fn ApplyImageProcessingEffects(context: *mut ImageProcessingContext) {
    gCanvasPixels = (*context).canvasPixels as *mut u16;
    gCanvasMonPersonality.set((*context).personality);
    gCanvasColumnStart.set((*context).columnStart);
    gCanvasRowStart.set((*context).rowStart);
    gCanvasColumnEnd.set((*context).columnEnd);
    gCanvasRowEnd.set((*context).rowEnd);
    gCanvasWidth.set((*context).canvasWidth);
    gCanvasHeight.set((*context).canvasHeight);
    'l1: {
        let sw1: u8 = (*context).effect;
        let mut fall = false;
        if sw1 == IMAGE_EFFECT_POINTILLISM {
            ApplyImageEffect_Pointillism();
            break 'l1;
        }
        if sw1 == IMAGE_EFFECT_BLUR {
            ApplyImageEffect_Blur();
            break 'l1;
        }
        if sw1 == IMAGE_EFFECT_OUTLINE_COLORED {
            ApplyImageEffect_BlackOutline();
            ApplyImageEffect_PersonalityColor(gCanvasMonPersonality.get());
            break 'l1;
        }
        if sw1 == IMAGE_EFFECT_INVERT_BLACK_WHITE {
            fall = true;
            ApplyImageEffect_BlackOutline();
            ApplyImageEffect_Invert();
            ApplyImageEffect_BlackAndWhite();
        }
        if fall || sw1 == IMAGE_EFFECT_INVERT {
            ApplyImageEffect_Invert();
            break 'l1;
        }
        if sw1 == IMAGE_EFFECT_THICK_BLACK_WHITE {
            ApplyImageEffect_BlackOutline();
            ApplyImageEffect_BlurRight();
            ApplyImageEffect_BlurRight();
            ApplyImageEffect_BlurDown();
            ApplyImageEffect_BlackAndWhite();
            break 'l1;
        }
        if sw1 == IMAGE_EFFECT_SHIMMER {
            ApplyImageEffect_Shimmer();
            break 'l1;
        }
        if sw1 == IMAGE_EFFECT_OUTLINE {
            ApplyImageEffect_BlackOutline();
            break 'l1;
        }
        if sw1 == IMAGE_EFFECT_BLUR_RIGHT {
            ApplyImageEffect_BlurRight();
            break 'l1;
        }
        if sw1 == IMAGE_EFFECT_BLUR_DOWN {
            ApplyImageEffect_BlurDown();
            break 'l1;
        }
        if sw1 == IMAGE_EFFECT_GRAYSCALE_LIGHT {
            ApplyImageEffect_Grayscale();
            ApplyImageEffect_RedChannelGrayscale(3);
            break 'l1;
        }
        if sw1 == IMAGE_EFFECT_CHARCOAL {
            ApplyImageEffect_BlackOutline();
            ApplyImageEffect_BlurRight();
            ApplyImageEffect_BlurDown();
            ApplyImageEffect_BlackAndWhite();
            ApplyImageEffect_Blur();
            ApplyImageEffect_Blur();
            ApplyImageEffect_RedChannelGrayscale(2);
            ApplyImageEffect_RedChannelGrayscaleHighlight(4);
            break 'l1;
        }
    }
}
unsafe fn ApplyImageEffect_RedChannelGrayscale(delta: u8) {
    let mut i: u8 = 0;
    for j in 0..gCanvasRowEnd.get() {
        let pixelRow: *mut u16 =
            gCanvasPixels.at((gCanvasRowStart.get() as i32 + j as i32) * gCanvasWidth.get() as i32);
        let mut pixel: *mut u16 = pixelRow.at(gCanvasColumnStart.get());
        i = 0;
        while i < gCanvasColumnEnd.get() {
            if *pixel as i32 & 32768 == 0 {
                let mut grayValue: u8 = *pixel as u8 & 31;
                grayValue += delta;
                if grayValue > 31 {
                    grayValue = 31;
                }
                *pixel = (grayValue as u16) << 10 | (grayValue as u16) << 5 | grayValue as u16;
            }
            i += 1;
            pixel = pixel.at(1);
        }
    }
}
unsafe fn ApplyImageEffect_RedChannelGrayscaleHighlight(highlight: u8) {
    let mut i: u8 = 0;
    for j in 0..gCanvasRowEnd.get() {
        let pixelRow: *mut u16 =
            gCanvasPixels.at((gCanvasRowStart.get() as i32 + j as i32) * gCanvasWidth.get() as i32);
        let mut pixel: *mut u16 = pixelRow.at(gCanvasColumnStart.get());
        i = 0;
        while i < gCanvasColumnEnd.get() {
            if *pixel as i32 & 32768 == 0 {
                let mut grayValue: u8 = *pixel as u8 & 31;
                if grayValue as i32 > 31 - highlight as i32 {
                    grayValue = 31 - (highlight >> 1);
                }
                *pixel = (grayValue as u16) << 10 | (grayValue as u16) << 5 | grayValue as u16;
            }
            i += 1;
            pixel = pixel.at(1);
        }
    }
}
unsafe fn ApplyImageEffect_Pointillism() {
    for i in 0..3200u32 {
        AddPointillismPoints(i as u16);
    }
}
unsafe fn ApplyImageEffect_Grayscale() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    while j < gCanvasRowEnd.get() {
        let pixelRow: *mut u16 =
            gCanvasPixels.at((gCanvasRowStart.get() as i32 + j as i32) * gCanvasWidth.get() as i32);
        let mut pixel: *mut u16 = pixelRow.at(gCanvasColumnStart.get());
        i = 0;
        while i < gCanvasColumnEnd.get() {
            if *pixel as i32 & 32768 == 0 {
                *pixel = ConvertColorToGrayscale(pixel);
            }
            i += 1;
            pixel = pixel.at(1);
        }
        j += 1;
    }
}
unsafe fn ApplyImageEffect_Blur() {
    let mut j: u8 = 0;
    let mut i: u8 = 0;
    while i < gCanvasColumnEnd.get() {
        let pixelRow: *mut u16 =
            gCanvasPixels.at(gCanvasRowStart.get() as i32 * gCanvasWidth.get() as i32);
        let mut pixel: *mut u16 = pixelRow.at(gCanvasColumnStart.get() as i32 + i as i32);
        let mut prevPixel: u16 = *pixel;
        j = 1;
        pixel = pixel.at(gCanvasWidth.get());
        while (j as i32) < gCanvasRowEnd.get() as i32 - 1 {
            if *pixel as i32 & 32768 == 0 {
                *pixel =
                    QuantizePixel_Blur(&raw mut prevPixel, pixel, pixel.at(gCanvasWidth.get()));
                prevPixel = *pixel;
            }
            j += 1;
            pixel = pixel.at(gCanvasWidth.get());
        }
        i += 1;
    }
}
unsafe fn ApplyImageEffect_PersonalityColor(personality: u8) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    while j < gCanvasRowEnd.get() {
        let pixelRow: *mut u16 =
            gCanvasPixels.at((gCanvasRowStart.get() as i32 + j as i32) * gCanvasWidth.get() as i32);
        let mut pixel: *mut u16 = pixelRow.at(gCanvasColumnStart.get());
        i = 0;
        while i < gCanvasColumnEnd.get() {
            if *pixel as i32 & 32768 == 0 {
                *pixel = QuantizePixel_PersonalityColor(pixel, personality);
            }
            i += 1;
            pixel = pixel.at(1);
        }
        j += 1;
    }
}
unsafe fn ApplyImageEffect_BlackAndWhite() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    while j < gCanvasRowEnd.get() {
        let pixelRow: *mut u16 =
            gCanvasPixels.at((gCanvasRowStart.get() as i32 + j as i32) * gCanvasWidth.get() as i32);
        let mut pixel: *mut u16 = pixelRow.at(gCanvasColumnStart.get());
        i = 0;
        while i < gCanvasColumnEnd.get() {
            if *pixel as i32 & 32768 == 0 {
                *pixel = QuantizePixel_BlackAndWhite(pixel);
            }
            i += 1;
            pixel = pixel.at(1);
        }
        j += 1;
    }
}
unsafe fn ApplyImageEffect_BlackOutline() {
    let mut i: u8 = 0;
    let mut pixel: *mut u16 = null_mut();
    let mut j: u8 = 0;
    while j < gCanvasRowEnd.get() {
        let pixelRow: *mut u16 =
            gCanvasPixels.at((gCanvasRowStart.get() as i32 + j as i32) * gCanvasWidth.get() as i32);
        pixel = pixelRow.at(gCanvasColumnStart.get());
        *pixel = QuantizePixel_BlackOutline(pixel, pixel.at(1));
        i = 1;
        pixel = pixel.at(1);
        while (i as i32) < gCanvasColumnEnd.get() as i32 - 1 {
            *pixel = QuantizePixel_BlackOutline(pixel, pixel.at(1));
            *pixel = QuantizePixel_BlackOutline(pixel, pixel.at(-1));
            i += 1;
            pixel = pixel.at(1);
        }
        *pixel = QuantizePixel_BlackOutline(pixel, pixel.at(-1));
        j += 1;
    }
    i = 0;
    while i < gCanvasColumnEnd.get() {
        let pixelRow: *mut u16 =
            gCanvasPixels.at(gCanvasRowStart.get() as i32 * gCanvasWidth.get() as i32);
        pixel = pixelRow.at(gCanvasColumnStart.get() as i32 + i as i32);
        *pixel = QuantizePixel_BlackOutline(pixel, pixel.at(gCanvasWidth.get()));
        j = 1;
        pixel = pixel.at(gCanvasWidth.get());
        while (j as i32) < gCanvasRowEnd.get() as i32 - 1 {
            *pixel = QuantizePixel_BlackOutline(pixel, pixel.at(gCanvasWidth.get()));
            *pixel = QuantizePixel_BlackOutline(pixel, pixel.at(-(gCanvasWidth.get() as i32)));
            j += 1;
            pixel = pixel.at(gCanvasWidth.get());
        }
        *pixel = QuantizePixel_BlackOutline(pixel, pixel.at(-(gCanvasWidth.get() as i32)));
        i += 1;
    }
}
unsafe fn ApplyImageEffect_Invert() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    while j < gCanvasRowEnd.get() {
        let pixelRow: *mut u16 =
            gCanvasPixels.at((gCanvasRowStart.get() as i32 + j as i32) * gCanvasWidth.get() as i32);
        let mut pixel: *mut u16 = pixelRow.at(gCanvasColumnStart.get());
        i = 0;
        while i < gCanvasColumnEnd.get() {
            if *pixel as i32 & 32768 == 0 {
                *pixel = QuantizePixel_Invert(pixel);
            }
            i += 1;
            pixel = pixel.at(1);
        }
        j += 1;
    }
}
unsafe fn ApplyImageEffect_Shimmer() {
    let mut j: u8 = 0;
    let mut prevPixel: u16 = 0;
    let mut pixel: *mut u16 = gCanvasPixels;
    let mut i: u8 = 0;
    while i < MAX_DIMENSION {
        j = 0;
        while j < MAX_DIMENSION {
            if *pixel as i32 & 32768 == 0 {
                *pixel = QuantizePixel_Invert(pixel);
            }
            j += 1;
            pixel = pixel.at(1);
        }
        i += 1;
    }
    j = 0;
    while j < MAX_DIMENSION {
        pixel = gCanvasPixels.at(j);
        prevPixel = *pixel;
        *pixel = RGB_ALPHA;
        i = 1;
        pixel = pixel.at(64);
        while i < 63 {
            if *pixel as i32 & 32768 == 0 {
                *pixel = QuantizePixel_BlurHard(&raw mut prevPixel, pixel, pixel.at(64));
                prevPixel = *pixel;
            }
            i += 1;
            pixel = pixel.at(64);
        }
        *pixel = RGB_ALPHA;
        pixel = gCanvasPixels.at(j);
        prevPixel = *pixel;
        *pixel = RGB_ALPHA;
        i = 1;
        pixel = pixel.at(64);
        while i < 63 {
            if *pixel as i32 & 32768 == 0 {
                *pixel = QuantizePixel_BlurHard(&raw mut prevPixel, pixel, pixel.at(64));
                prevPixel = *pixel;
            }
            i += 1;
            pixel = pixel.at(64);
        }
        *pixel = RGB_ALPHA;
        j += 1;
    }
    pixel = gCanvasPixels;
    for i in 0..MAX_DIMENSION {
        j = 0;
        while j < MAX_DIMENSION {
            if *pixel as i32 & 32768 == 0 {
                *pixel = QuantizePixel_Invert(pixel);
            }
            j += 1;
            pixel = pixel.at(1);
        }
    }
}
unsafe fn ApplyImageEffect_BlurRight() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    while j < gCanvasRowEnd.get() {
        let pixelRow: *mut u16 =
            gCanvasPixels.at((gCanvasRowStart.get() as i32 + j as i32) * gCanvasWidth.get() as i32);
        let mut pixel: *mut u16 = pixelRow.at(gCanvasColumnStart.get());
        let mut prevPixel: u16 = *pixel;
        i = 1;
        pixel = pixel.at(1);
        while (i as i32) < gCanvasColumnEnd.get() as i32 - 1 {
            if *pixel as i32 & 32768 == 0 {
                *pixel = QuantizePixel_MotionBlur(&raw mut prevPixel, pixel);
                prevPixel = *pixel;
            }
            i += 1;
            pixel = pixel.at(1);
        }
        j += 1;
    }
}
unsafe fn ApplyImageEffect_BlurDown() {
    let mut j: u8 = 0;
    let mut i: u8 = 0;
    while i < gCanvasColumnEnd.get() {
        let pixelRow: *mut u16 =
            gCanvasPixels.at(gCanvasRowStart.get() as i32 * gCanvasWidth.get() as i32);
        let mut pixel: *mut u16 = pixelRow.at(gCanvasColumnStart.get() as i32 + i as i32);
        let mut prevPixel: u16 = *pixel;
        j = 1;
        pixel = pixel.at(gCanvasWidth.get());
        while (j as i32) < gCanvasRowEnd.get() as i32 - 1 {
            if *pixel as i32 & 32768 == 0 {
                *pixel = QuantizePixel_MotionBlur(&raw mut prevPixel, pixel);
                prevPixel = *pixel;
            }
            j += 1;
            pixel = pixel.at(gCanvasWidth.get());
        }
        i += 1;
    }
}
unsafe fn AddPointillismPoints(point: u16) {
    let mut points: CArray<PointillismPoint, 6> = zeroed();
    points[0].column = sPointillismPoints[point][0];
    points[0].row = sPointillismPoints[point][1];
    points[0].delta = (sPointillismPoints[point][2] >> 3) as u16 & 7;
    let colorType: u8 = sPointillismPoints[point][2] >> 1 & 3;
    let offsetDownLeft: u8 = sPointillismPoints[point][2] & 1;
    let mut i: u8 = 1;
    while (i as u16) < points[0].delta {
        if offsetDownLeft == 0 {
            points[i].column = points[0].column - i;
            points[i].row = points[0].row + i;
        } else {
            points[i].column = points[0].column + 1;
            points[i].row = points[0].row - 1;
        }
        if points[i].column >= MAX_DIMENSION || points[i].row >= MAX_DIMENSION {
            points[0].delta = i as u16 - 1;
            break;
        }
        points[i].delta = points[0].delta - i as u16;
        i += 1;
    }
    i = 0;
    while (i as u16) < points[0].delta {
        let pixel: *mut u16 = gCanvasPixels
            .at(points[i].row as i32 * MAX_DIMENSION as i32)
            .at(points[i].column);
        if *pixel as i32 & 32768 == 0 {
            let mut red: u16 = *pixel & 0x1F;
            let mut green: u16 = *pixel >> 5 & 0x1F;
            let mut blue: u16 = *pixel >> 10 & 0x1F;
            match colorType {
                0 | 1 => match ((sPointillismPoints[point][2] >> 3) as i32 & 7) % 3 {
                    0 => {
                        if red >= points[i].delta {
                            red -= points[i].delta;
                        } else {
                            red = 0;
                        }
                    }
                    1 => {
                        if green >= points[i].delta {
                            green -= points[i].delta;
                        } else {
                            green = 0;
                        }
                    }
                    2 => {
                        if blue >= points[i].delta {
                            blue -= points[i].delta;
                        } else {
                            blue = 0;
                        }
                    }
                    _ => {}
                },
                2 | 3 => {
                    red += points[i].delta;
                    green += points[i].delta;
                    blue += points[i].delta;
                    if red > 31 {
                        red = 31;
                    }
                    if green > 31 {
                        green = 31;
                    }
                    if blue > 31 {
                        blue = 31;
                    }
                }
                _ => {}
            }
            *pixel = blue << 10 | green << 5 | red;
        }
        i += 1;
    }
}
unsafe fn ConvertColorToGrayscale(color: *mut u16) -> u16 {
    let clr: i32 = *color as i32;
    let r: i32 = clr & 0x1F;
    let g: i32 = clr >> 5 & 0x1F;
    let b: i32 = clr >> 10 & 0x1F;
    let gray: i32 = (r * (0_f32 * 256_f32) as i16 as i32
        + g * (0_f32 * 256_f32) as i16 as i32
        + b * (0_f32 * 256_f32) as i16 as i32)
        >> 8;
    (gray as u16) << 10 | (gray as u16) << 5 | gray as u16
}
unsafe fn QuantizePixel_PersonalityColor(color: *mut u16, personality: u8) -> u16 {
    let red: u16 = *color & 0x1F;
    let green: u16 = *color >> 5 & 0x1F;
    let blue: u16 = *color >> 10 & 0x1F;
    if red < 17 && green < 17 && blue < 17 {
        return GetColorFromPersonality(personality);
    } else {
        return 32767;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
fn GetColorFromPersonality(personality: u8) -> u16 {
    let mut red: u16 = 0;
    let mut green: u16 = 0;
    let mut blue: u16 = 0;
    let strength: u8 = (personality as i32 / 6 % 3) as u8;
    let colorType: u8 = (personality as i32 % 6) as u8;
    match colorType {
        0 => {
            green = 21 - strength as u16;
            blue = green;
            red = 0;
        }
        1 => {
            blue = 0;
            red = 21 - strength as u16;
            green = red;
        }
        2 => {
            blue = 21 - strength as u16;
            green = 0;
            red = blue;
        }
        3 => {
            blue = 0;
            green = 0;
            red = 23 - strength as u16;
        }
        4 => {
            blue = 23 - strength as u16;
            green = 0;
            red = 0;
        }
        5 => {
            blue = 0;
            green = 23 - strength as u16;
            red = 0;
        }
        _ => {}
    }
    blue << 10 | green << 5 | red
}
unsafe fn QuantizePixel_BlackAndWhite(color: *mut u16) -> u16 {
    let red: u16 = *color & 0x1F;
    let green: u16 = *color >> 5 & 0x1F;
    let blue: u16 = *color >> 10 & 0x1F;
    if red < 17 && green < 17 && blue < 17 {
        return 0;
    } else {
        return 32767;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn QuantizePixel_BlackOutline(pixelA: *mut u16, pixelB: *mut u16) -> u16 {
    if *pixelA != 0 {
        if *pixelA as i32 & 32768 != 0 {
            return RGB_ALPHA;
        }
        if *pixelB as i32 & 32768 != 0 {
            return 0;
        }
        return *pixelA;
    }
    0
}
unsafe fn QuantizePixel_Invert(color: *mut u16) -> u16 {
    let mut red: u16 = *color & 0x1F;
    let mut green: u16 = *color >> 5 & 0x1F;
    let mut blue: u16 = *color >> 10 & 0x1F;
    red = 31 - red;
    green = 31 - green;
    blue = 31 - blue;
    blue << 10 | green << 5 | red
}
unsafe fn QuantizePixel_MotionBlur(prevPixel: *mut u16, curPixel: *mut u16) -> u16 {
    let mut pixelChannels: CArray<CArray<u16, 3>, 2> = zeroed();
    let mut diffs: CArray<u16, 3> = zeroed();
    let mut largestDiff: u16 = 0;
    if *prevPixel == *curPixel {
        return *curPixel;
    }
    pixelChannels[0][0] = *prevPixel & 0x1F;
    pixelChannels[0][1] = *prevPixel >> 5 & 0x1F;
    pixelChannels[0][2] = *prevPixel >> 10 & 0x1F;
    pixelChannels[1][0] = *curPixel & 0x1F;
    pixelChannels[1][1] = *curPixel >> 5 & 0x1F;
    pixelChannels[1][2] = *curPixel >> 10 & 0x1F;
    if pixelChannels[0][0] > 25 && pixelChannels[0][1] > 25 && pixelChannels[0][2] > 25 {
        return *curPixel;
    }
    if pixelChannels[1][0] > 25 && pixelChannels[1][1] > 25 && pixelChannels[1][2] > 25 {
        return *curPixel;
    }
    for i in 0..3u8 {
        if pixelChannels[0][i] > pixelChannels[1][i] {
            diffs[i] = pixelChannels[0][i] - pixelChannels[1][i];
        } else {
            diffs[i] = pixelChannels[1][i] - pixelChannels[0][i];
        }
    }
    if diffs[0] >= diffs[1] {
        if diffs[0] >= diffs[2] {
            largestDiff = diffs[0];
        } else if diffs[1] >= diffs[2] {
            largestDiff = diffs[1];
        } else {
            largestDiff = diffs[2];
        }
    } else {
        if diffs[1] >= diffs[2] {
            largestDiff = diffs[1];
        } else if diffs[2] >= diffs[0] {
            largestDiff = diffs[2];
        } else {
            largestDiff = diffs[0];
        }
    }
    let red: u16 = (pixelChannels[1][0] as i32 * (31 - largestDiff as i32 / 2) / 31) as u16;
    let green: u16 = (pixelChannels[1][1] as i32 * (31 - largestDiff as i32 / 2) / 31) as u16;
    let blue: u16 = (pixelChannels[1][2] as i32 * (31 - largestDiff as i32 / 2) / 31) as u16;
    blue << 10 | green << 5 | red
}
unsafe fn QuantizePixel_Blur(prevPixel: *mut u16, curPixel: *mut u16, nextPixel: *mut u16) -> u16 {
    let mut prevDiff: u16 = 0;
    let mut nextDiff: u16 = 0;
    let mut diff: u32 = 0;
    if *prevPixel == *curPixel && *nextPixel == *curPixel {
        return *curPixel;
    }
    let mut red: u16 = *curPixel & 0x1F;
    let mut green: u16 = *curPixel >> 5 & 0x1F;
    let mut blue: u16 = *curPixel >> 10 & 0x1F;
    let prevAvg: u16 = (((*prevPixel as i32 & 0x1F)
        + ((*prevPixel >> 5) as i32 & 0x1F)
        + ((*prevPixel >> 10) as i32 & 0x1F))
        / 3) as u16;
    let curAvg: u16 = (((*curPixel as i32 & 0x1F)
        + ((*curPixel >> 5) as i32 & 0x1F)
        + ((*curPixel >> 10) as i32 & 0x1F))
        / 3) as u16;
    let nextAvg: u16 = (((*nextPixel as i32 & 0x1F)
        + ((*nextPixel >> 5) as i32 & 0x1F)
        + ((*nextPixel >> 10) as i32 & 0x1F))
        / 3) as u16;
    if prevAvg == curAvg && nextAvg == curAvg {
        return *curPixel;
    }
    if prevAvg > curAvg {
        prevDiff = prevAvg - curAvg;
    } else {
        prevDiff = curAvg - prevAvg;
    }
    if nextAvg > curAvg {
        nextDiff = nextAvg - curAvg;
    } else {
        nextDiff = curAvg - nextAvg;
    }
    if prevDiff >= nextDiff {
        diff = prevDiff as u32;
    } else {
        diff = nextDiff as u32;
    }
    let factor: u16 = 31 - (diff / 2) as u16;
    red = (red as i32 * factor as i32 / 31) as u16;
    green = (green as i32 * factor as i32 / 31) as u16;
    blue = (blue as i32 * factor as i32 / 31) as u16;
    blue << 10 | green << 5 | red
}
unsafe fn QuantizePixel_BlurHard(
    prevPixel: *mut u16,
    curPixel: *mut u16,
    nextPixel: *mut u16,
) -> u16 {
    let mut prevDiff: u16 = 0;
    let mut nextDiff: u16 = 0;
    let mut diff: u32 = 0;
    if *prevPixel == *curPixel && *nextPixel == *curPixel {
        return *curPixel;
    }
    let mut red: u16 = *curPixel & 0x1F;
    let mut green: u16 = *curPixel >> 5 & 0x1F;
    let mut blue: u16 = *curPixel >> 10 & 0x1F;
    let prevAvg: u16 = (((*prevPixel as i32 & 0x1F)
        + ((*prevPixel >> 5) as i32 & 0x1F)
        + ((*prevPixel >> 10) as i32 & 0x1F))
        / 3) as u16;
    let curAvg: u16 = (((*curPixel as i32 & 0x1F)
        + ((*curPixel >> 5) as i32 & 0x1F)
        + ((*curPixel >> 10) as i32 & 0x1F))
        / 3) as u16;
    let nextAvg: u16 = (((*nextPixel as i32 & 0x1F)
        + ((*nextPixel >> 5) as i32 & 0x1F)
        + ((*nextPixel >> 10) as i32 & 0x1F))
        / 3) as u16;
    if prevAvg == curAvg && nextAvg == curAvg {
        return *curPixel;
    }
    if prevAvg > curAvg {
        prevDiff = prevAvg - curAvg;
    } else {
        prevDiff = curAvg - prevAvg;
    }
    if nextAvg > curAvg {
        nextDiff = nextAvg - curAvg;
    } else {
        nextDiff = curAvg - nextAvg;
    }
    if prevDiff >= nextDiff {
        diff = prevDiff as u32;
    } else {
        diff = nextDiff as u32;
    }
    let factor: u16 = 31 - diff as u16;
    red = (red as i32 * factor as i32 / 31) as u16;
    green = (green as i32 * factor as i32 / 31) as u16;
    blue = (blue as i32 * factor as i32 / 31) as u16;
    blue << 10 | green << 5 | red
}
pub unsafe fn ConvertImageProcessingToGBA(context: *mut ImageProcessingContext) {
    let mut src: *mut u16 = null_mut();
    let mut dest: *mut u16 = null_mut();
    let width: u16 = ((*context).canvasWidth >> 3) as u16;
    let height: u16 = ((*context).canvasHeight >> 3) as u16;
    let src_: *mut u16 = (*context).canvasPixels as *mut u16;
    let dest_: *mut u16 = (*context).dest as *mut u16;
    if (*context).var_16 == 2 {
        for i in 0..height {
            for j in 0..width {
                for k in 0..8u16 {
                    dest = dest_
                        .at((i as i32 * width as i32 + j as i32) << 5)
                        .at((k as i32) << 2);
                    src = src_
                        .at(((((i as i32) << 3) + k as i32) << 3) * width as i32)
                        .at((j as i32) << 3);
                    *dest = *src | *src.at(1) << 8;
                    *dest.at(1) = *src.at(2) | *src.at(3) << 8;
                    *dest.at(2) = *src.at(4) | *src.at(5) << 8;
                    *dest.at(3) = *src.at(6) | *src.at(7) << 8;
                }
            }
        }
    } else {
        for i in 0..height {
            for j in 0..width {
                for k in 0..8u16 {
                    dest = dest_
                        .at((i as i32 * width as i32 + j as i32) << 4)
                        .at((k as i32) << 1);
                    src = src_
                        .at(((((i as i32) << 3) + k as i32) << 3) * width as i32)
                        .at((j as i32) << 3);
                    *dest = *src | *src.at(1) << 4 | *src.at(2) << 8 | *src.at(3) << 12;
                    *dest.at(1) = *src.at(4) | *src.at(5) << 4 | *src.at(6) << 8 | *src.at(7) << 12;
                }
            }
        }
    }
}
pub unsafe fn ApplyImageProcessingQuantization(context: *mut ImageProcessingContext) {
    gCanvasPaletteStart.set((*context).paletteStart as u16 * 16);
    gCanvasPalette = (*context).canvasPalette.at(gCanvasPaletteStart.get());
    gCanvasPixels = (*context).canvasPixels as *mut u16;
    gCanvasColumnStart.set((*context).columnStart);
    gCanvasRowStart.set((*context).rowStart);
    gCanvasColumnEnd.set((*context).columnEnd);
    gCanvasRowEnd.set((*context).rowEnd);
    gCanvasWidth.set((*context).canvasWidth);
    gCanvasHeight.set((*context).canvasHeight);
    match (*context).quantizeEffect {
        QUANTIZE_EFFECT_STANDARD => {
            QuantizePalette_Standard(FALSE);
        }
        QUANTIZE_EFFECT_STANDARD_LIMITED_COLORS => {
            QuantizePalette_Standard(TRUE);
        }
        QUANTIZE_EFFECT_PRIMARY_COLORS => {
            SetPresetPalette_PrimaryColors();
            QuantizePalette_PrimaryColors();
        }
        QUANTIZE_EFFECT_GRAYSCALE => {
            SetPresetPalette_Grayscale();
            QuantizePalette_Grayscale();
        }
        QUANTIZE_EFFECT_GRAYSCALE_SMALL => {
            SetPresetPalette_GrayscaleSmall();
            QuantizePalette_GrayscaleSmall();
        }
        QUANTIZE_EFFECT_BLACK_WHITE => {
            SetPresetPalette_BlackAndWhite();
            QuantizePalette_BlackAndWhite();
        }
        _ => {}
    }
}
unsafe fn SetPresetPalette_PrimaryColors() {
    *gCanvasPalette = 0;
    *gCanvasPalette.at(1) = 6342;
    *gCanvasPalette.at(2) = 30653;
    *gCanvasPalette.at(3) = 11627;
    *gCanvasPalette.at(4) = 6365;
    *gCanvasPalette.at(5) = 7078;
    *gCanvasPalette.at(6) = 29894;
    *gCanvasPalette.at(7) = 7101;
    *gCanvasPalette.at(8) = 29917;
    *gCanvasPalette.at(9) = 30630;
    *gCanvasPalette.at(10) = 6525;
    *gCanvasPalette.at(11) = 7083;
    *gCanvasPalette.at(12) = 30054;
    *gCanvasPalette.at(13) = 11485;
    *gCanvasPalette.at(14) = 12198;
    *gCanvasPalette.at(15) = 29899;
}
unsafe fn SetPresetPalette_BlackAndWhite() {
    *gCanvasPalette = 0;
    *gCanvasPalette.at(1) = 0;
    *gCanvasPalette.at(2) = 32767;
}
unsafe fn SetPresetPalette_GrayscaleSmall() {
    *gCanvasPalette = 0;
    *gCanvasPalette.at(1) = 0;
    for i in 0..14u8 {
        *gCanvasPalette.at(i as i32 + 2) =
            (2 * (i as u16 + 2)) << 10 | (2 * (i as u16 + 2)) << 5 | (2 * (i as u16 + 2));
    }
}
unsafe fn SetPresetPalette_Grayscale() {
    *gCanvasPalette = 0;
    for i in 0..32u8 {
        *gCanvasPalette.at(i as i32 + 1) = (i as u16) << 10 | (i as u16) << 5 | i as u16;
    }
}
unsafe fn QuantizePalette_Standard(useLimitedPalette: u8) {
    let mut maxIndex: u16 = 0xDF;
    if useLimitedPalette == 0 {
        maxIndex = 0xFF;
    }
    let mut i: u8 = 0;
    while (i as u16) < maxIndex {
        *gCanvasPalette.at(i) = 0;
        i += 1;
    }
    *gCanvasPalette.at(maxIndex) = 15855;
    let mut j: u8 = 0;
    while j < gCanvasRowEnd.get() {
        let pixelRow: *mut u16 =
            gCanvasPixels.at((gCanvasRowStart.get() as i32 + j as i32) * gCanvasWidth.get() as i32);
        let mut pixel: *mut u16 = pixelRow.at(gCanvasColumnStart.get());
        i = 0;
        while i < gCanvasColumnEnd.get() {
            if *pixel as i32 & 32768 != 0 {
                *pixel = gCanvasPaletteStart.get();
            } else {
                let quantizedColor: u16 = QuantizePixel_Standard(pixel);
                let mut curIndex: u8 = 1;
                if (curIndex as u16) < maxIndex {
                    if *gCanvasPalette.at(curIndex) == 0 {
                        *gCanvasPalette.at(curIndex) = quantizedColor;
                        *pixel = gCanvasPaletteStart.get() + curIndex as u16;
                    } else {
                        while (curIndex as u16) < maxIndex {
                            if *gCanvasPalette.at(curIndex) == 0 {
                                *gCanvasPalette.at(curIndex) = quantizedColor;
                                *pixel = gCanvasPaletteStart.get() + curIndex as u16;
                                break;
                            }
                            if *gCanvasPalette.at(curIndex) == quantizedColor {
                                *pixel = gCanvasPaletteStart.get() + curIndex as u16;
                                break;
                            }
                            curIndex += 1;
                        }
                    }
                }
                if curIndex as u16 == maxIndex {
                    curIndex = maxIndex as u8;
                    *pixel = curIndex as u16;
                }
            }
            i += 1;
            pixel = pixel.at(1);
        }
        j += 1;
    }
}
unsafe fn QuantizePalette_BlackAndWhite() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    while j < gCanvasRowEnd.get() {
        let pixelRow: *mut u16 =
            gCanvasPixels.at((gCanvasRowStart.get() as i32 + j as i32) * gCanvasWidth.get() as i32);
        let mut pixel: *mut u16 = pixelRow.at(gCanvasColumnStart.get());
        i = 0;
        while i < gCanvasColumnEnd.get() {
            if *pixel as i32 & 32768 != 0 {
                *pixel = gCanvasPaletteStart.get();
            } else {
                if QuantizePixel_BlackAndWhite(pixel) == 0 {
                    *pixel = gCanvasPaletteStart.get() + 1;
                } else {
                    *pixel = gCanvasPaletteStart.get() + 2;
                }
            }
            i += 1;
            pixel = pixel.at(1);
        }
        j += 1;
    }
}
unsafe fn QuantizePalette_GrayscaleSmall() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    while j < gCanvasRowEnd.get() {
        let pixelRow: *mut u16 =
            gCanvasPixels.at((gCanvasRowStart.get() as i32 + j as i32) * gCanvasWidth.get() as i32);
        let mut pixel: *mut u16 = pixelRow.at(gCanvasColumnStart.get());
        i = 0;
        while i < gCanvasColumnEnd.get() {
            if *pixel as i32 & 32768 != 0 {
                *pixel = gCanvasPaletteStart.get();
            } else {
                *pixel = QuantizePixel_GrayscaleSmall(pixel) + gCanvasPaletteStart.get();
            }
            i += 1;
            pixel = pixel.at(1);
        }
        j += 1;
    }
}
unsafe fn QuantizePalette_Grayscale() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    while j < gCanvasRowEnd.get() {
        let pixelRow: *mut u16 =
            gCanvasPixels.at((gCanvasRowStart.get() as i32 + j as i32) * gCanvasWidth.get() as i32);
        let mut pixel: *mut u16 = pixelRow.at(gCanvasColumnStart.get());
        i = 0;
        while i < gCanvasColumnEnd.get() {
            if *pixel as i32 & 32768 != 0 {
                *pixel = gCanvasPaletteStart.get();
            } else {
                *pixel = QuantizePixel_Grayscale(pixel) + gCanvasPaletteStart.get();
            }
            i += 1;
            pixel = pixel.at(1);
        }
        j += 1;
    }
}
unsafe fn QuantizePalette_PrimaryColors() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    while j < gCanvasRowEnd.get() {
        let pixelRow: *mut u16 =
            gCanvasPixels.at((gCanvasRowStart.get() as i32 + j as i32) * gCanvasWidth.get() as i32);
        let mut pixel: *mut u16 = pixelRow.at(gCanvasColumnStart.get());
        i = 0;
        while i < gCanvasColumnEnd.get() {
            if *pixel as i32 & 32768 != 0 {
                *pixel = gCanvasPaletteStart.get();
            } else {
                *pixel = QuantizePixel_PrimaryColors(pixel) + gCanvasPaletteStart.get();
            }
            i += 1;
            pixel = pixel.at(1);
        }
        j += 1;
    }
}
unsafe fn QuantizePixel_Standard(pixel: *mut u16) -> u16 {
    let mut red: u16 = *pixel & 0x1F;
    let mut green: u16 = *pixel >> 5 & 0x1F;
    let mut blue: u16 = *pixel >> 10 & 0x1F;
    if red as i32 & 3 != 0 {
        red = (red & 0x1C) + 4;
    }
    if green as i32 & 3 != 0 {
        green = (green & 0x1C) + 4;
    }
    if blue as i32 & 3 != 0 {
        blue = (blue & 0x1C) + 4;
    }
    if red < 6 {
        red = 6;
    }
    if red > 30 {
        red = 30;
    }
    if green < 6 {
        green = 6;
    }
    if green > 30 {
        green = 30;
    }
    if blue < 6 {
        blue = 6;
    }
    if blue > 30 {
        blue = 30;
    }
    blue << 10 | green << 5 | red
}
unsafe fn QuantizePixel_PrimaryColors(color: *mut u16) -> u16 {
    let red: u16 = *color & 0x1F;
    let green: u16 = *color >> 5 & 0x1F;
    let blue: u16 = *color >> 10 & 0x1F;
    if red < 12 && green < 11 && blue < 11 {
        return 1;
    }
    if red > 19 && green > 19 && blue > 19 {
        return 2;
    }
    if red > 19 {
        if green > 19 {
            if blue > 14 {
                return 2;
            } else {
                return 7;
            }
        } else if blue > 19 {
            if green > 14 {
                return 2;
            } else {
                return 8;
            }
        }
    }
    if green > 19 && blue > 19 {
        if red > 14 {
            return 2;
        } else {
            return 9;
        }
    }
    if red > 19 {
        if green > 11 {
            if blue > 11 {
                if green < blue {
                    return 8;
                } else {
                    return 7;
                }
            } else {
                return 10;
            }
        } else if blue > 11 {
            return 13;
        } else {
            return 4;
        }
    }
    if green > 19 {
        if red > 11 {
            if blue > 11 {
                if red < blue {
                    return 9;
                } else {
                    return 7;
                }
            } else {
                return 11;
            }
        } else {
            if blue > 11 {
                return 14;
            } else {
                return 5;
            }
        }
    }
    if blue > 19 {
        if red > 11 {
            if green > 11 {
                if red < green {
                    return 9;
                } else {
                    return 8;
                }
            }
        } else if green > 11 {
            return 12;
        }
        if blue > 11 {
            return 15;
        } else {
            return 6;
        }
    }
    3
}
unsafe fn QuantizePixel_GrayscaleSmall(color: *mut u16) -> u16 {
    let red: u16 = *color & 0x1F;
    let green: u16 = *color >> 5 & 0x1F;
    let blue: u16 = *color >> 10 & 0x1F;
    let average: u16 = ((red as i32 + green as i32 + blue as i32) / 3) as u16 & 0x1E;
    if average == 0 {
        return 1;
    } else {
        return (average as i32 / 2) as u16;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn QuantizePixel_Grayscale(color: *mut u16) -> u16 {
    let red: u16 = *color & 0x1F;
    let green: u16 = *color >> 5 & 0x1F;
    let blue: u16 = *color >> 10 & 0x1F;
    let average: u16 = ((red as i32 + green as i32 + blue as i32) / 3) as u16;
    average + 1
}
