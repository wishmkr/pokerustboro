//! Translated from `src/image_processing_effects.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sPointillismPoints
#[allow(unused_imports)]
use crate::data::image_processing_effects::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gCanvasColumnStart: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gCanvasPixels: *mut u16 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gCanvasRowEnd: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gCanvasHeight: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gCanvasColumnEnd: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gCanvasRowStart: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gCanvasMonPersonality: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gCanvasWidth: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gCanvasPalette: *mut u16 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gCanvasPaletteStart: u16 = 0u16;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApplyImageProcessingEffects(context: *mut u8) {
    unsafe {
        let mut context = context;
        ((&raw mut gCanvasPixels).cast::<u8>().cast::<*mut u16>())
            .write((((context).wrapping_add(4).cast::<*mut u8>()).read()).cast::<u16>());
        ((&raw mut gCanvasMonPersonality).cast::<u8>().cast::<u8>())
            .write(((context).wrapping_add(31)).read());
        ((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>())
            .write(((context).wrapping_add(25)).read());
        ((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>())
            .write(((context).wrapping_add(26)).read());
        ((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>())
            .write(((context).wrapping_add(27)).read());
        ((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>())
            .write(((context).wrapping_add(28)).read());
        ((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>())
            .write(((context).wrapping_add(29)).read());
        ((&raw mut gCanvasHeight).cast::<u8>().cast::<u8>())
            .write(((context).wrapping_add(30)).read());
        'l1: {
            let __sw1 = (((context).read()) as i32);
            let mut __fall = false;
            if __sw1 == 2i32 {
                __fall = true;
                ApplyImageEffect_Pointillism();
                break 'l1;
            }
            if __sw1 == 8i32 {
                __fall = true;
                ApplyImageEffect_Blur();
                break 'l1;
            }
            if __sw1 == 9i32 {
                __fall = true;
                ApplyImageEffect_BlackOutline();
                ApplyImageEffect_PersonalityColor(
                    ((&raw mut gCanvasMonPersonality).cast::<u8>().cast::<u8>()).read(),
                );
                break 'l1;
            }
            if __sw1 == 10i32 {
                __fall = true;
                ApplyImageEffect_BlackOutline();
                ApplyImageEffect_Invert();
                ApplyImageEffect_BlackAndWhite();
            }
            if __fall || __sw1 == 31i32 {
                __fall = true;
                ApplyImageEffect_Invert();
                break 'l1;
            }
            if __sw1 == 11i32 {
                __fall = true;
                ApplyImageEffect_BlackOutline();
                ApplyImageEffect_BlurRight();
                ApplyImageEffect_BlurRight();
                ApplyImageEffect_BlurDown();
                ApplyImageEffect_BlackAndWhite();
                break 'l1;
            }
            if __sw1 == 13i32 {
                __fall = true;
                ApplyImageEffect_Shimmer();
                break 'l1;
            }
            if __sw1 == 30i32 {
                __fall = true;
                ApplyImageEffect_BlackOutline();
                break 'l1;
            }
            if __sw1 == 32i32 {
                __fall = true;
                ApplyImageEffect_BlurRight();
                break 'l1;
            }
            if __sw1 == 33i32 {
                __fall = true;
                ApplyImageEffect_BlurDown();
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                ApplyImageEffect_Grayscale();
                ApplyImageEffect_RedChannelGrayscale(3u8);
                break 'l1;
            }
            if __sw1 == 36i32 {
                __fall = true;
                ApplyImageEffect_BlackOutline();
                ApplyImageEffect_BlurRight();
                ApplyImageEffect_BlurDown();
                ApplyImageEffect_BlackAndWhite();
                ApplyImageEffect_Blur();
                ApplyImageEffect_Blur();
                ApplyImageEffect_RedChannelGrayscale(2u8);
                ApplyImageEffect_RedChannelGrayscaleHighlight(4u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyImageEffect_RedChannelGrayscale(delta: u8) {
    unsafe {
        let mut delta = delta;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            j = 0u8;
            'l1: loop {
                if !(((j) as i32)
                    < ((((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut pixelRow: *mut u16 = (((&raw mut gCanvasPixels)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(
                        ((((((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            .wrapping_add(((j) as i32)))
                        .wrapping_mul(
                            ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32),
                        )) as isize,
                    );
                    let mut pixel: *mut u16 = (pixelRow).wrapping_offset(
                        ((((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    );
                    {
                        i = 0u8;
                        'l3: loop {
                            if !(((i) as i32)
                                < ((((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>()).read())
                                    as i32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                if !(((((pixel).read()) as i32) & 32768i32) != 0) {
                                    let mut grayValue: u8 =
                                        (((((pixel).read()) as i32) & 31i32) as u8);
                                    grayValue = ((((grayValue) as i32)
                                        .wrapping_add(((delta) as i32)))
                                        as u8);
                                    if ((grayValue) as i32) > 31i32 {
                                        grayValue = 31u8;
                                    }
                                    (pixel).write(
                                        ((((((grayValue) as i32) << 10)
                                            | (((grayValue) as i32) << 5))
                                            | ((grayValue) as i32))
                                            as u16),
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyImageEffect_RedChannelGrayscaleHighlight(highlight: u8) {
    unsafe {
        let mut highlight = highlight;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            j = 0u8;
            'l1: loop {
                if !(((j) as i32)
                    < ((((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut pixelRow: *mut u16 = (((&raw mut gCanvasPixels)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(
                        ((((((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            .wrapping_add(((j) as i32)))
                        .wrapping_mul(
                            ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32),
                        )) as isize,
                    );
                    let mut pixel: *mut u16 = (pixelRow).wrapping_offset(
                        ((((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    );
                    {
                        i = 0u8;
                        'l3: loop {
                            if !(((i) as i32)
                                < ((((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>()).read())
                                    as i32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                if !(((((pixel).read()) as i32) & 32768i32) != 0) {
                                    let mut grayValue: u8 =
                                        (((((pixel).read()) as i32) & 31i32) as u8);
                                    if ((grayValue) as i32)
                                        > (31i32).wrapping_sub(((highlight) as i32))
                                    {
                                        grayValue = (((31i32)
                                            .wrapping_sub((((highlight) as i32) >> 1)))
                                            as u8);
                                    }
                                    (pixel).write(
                                        ((((((grayValue) as i32) << 10)
                                            | (((grayValue) as i32) << 5))
                                            | ((grayValue) as i32))
                                            as u16),
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyImageEffect_Pointillism() {
    unsafe {
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(9600u32, 3u32)) {
                    break 'l1;
                }
                'l2: {
                    AddPointillismPoints(((i) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyImageEffect_Grayscale() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            j = 0u8;
            'l1: loop {
                if !(((j) as i32)
                    < ((((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut pixelRow: *mut u16 = (((&raw mut gCanvasPixels)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(
                        ((((((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            .wrapping_add(((j) as i32)))
                        .wrapping_mul(
                            ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32),
                        )) as isize,
                    );
                    let mut pixel: *mut u16 = (pixelRow).wrapping_offset(
                        ((((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    );
                    {
                        i = 0u8;
                        'l3: loop {
                            if !(((i) as i32)
                                < ((((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>()).read())
                                    as i32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                if !(((((pixel).read()) as i32) & 32768i32) != 0) {
                                    (pixel).write(ConvertColorToGrayscale(pixel));
                                }
                            }
                            i = (i).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyImageEffect_Blur() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut pixelRow: *mut u16 = (((&raw mut gCanvasPixels)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(
                        (((((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            .wrapping_mul(
                                ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read())
                                    as i32),
                            )) as isize,
                    );
                    let mut pixel: *mut u16 = (pixelRow).wrapping_offset(
                        (((((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>()).read())
                            as i32)
                            .wrapping_add(((i) as i32))) as isize,
                    );
                    let mut prevPixel: u16 = (pixel).read();
                    j = 1u8;
                    pixel = (pixel).wrapping_offset(
                        ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    );
                    'l3: loop {
                        if !(((j) as i32)
                            < ((((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>()).read())
                                as i32)
                                .wrapping_sub(1i32))
                        {
                            break 'l3;
                        }
                        if !(((((pixel).read()) as i32) & 32768i32) != 0) {
                            (pixel).write(QuantizePixel_Blur(
                                &raw mut prevPixel,
                                pixel,
                                (pixel).wrapping_offset(
                                    ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize,
                                ),
                            ));
                            prevPixel = (pixel).read();
                        }
                        j = (j).wrapping_add(1);
                        pixel = (pixel).wrapping_offset(
                            ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyImageEffect_PersonalityColor(personality: u8) {
    unsafe {
        let mut personality = personality;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            j = 0u8;
            'l1: loop {
                if !(((j) as i32)
                    < ((((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut pixelRow: *mut u16 = (((&raw mut gCanvasPixels)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(
                        ((((((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            .wrapping_add(((j) as i32)))
                        .wrapping_mul(
                            ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32),
                        )) as isize,
                    );
                    let mut pixel: *mut u16 = (pixelRow).wrapping_offset(
                        ((((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    );
                    {
                        i = 0u8;
                        'l3: loop {
                            if !(((i) as i32)
                                < ((((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>()).read())
                                    as i32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                if !(((((pixel).read()) as i32) & 32768i32) != 0) {
                                    (pixel)
                                        .write(QuantizePixel_PersonalityColor(pixel, personality));
                                }
                            }
                            i = (i).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyImageEffect_BlackAndWhite() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            j = 0u8;
            'l1: loop {
                if !(((j) as i32)
                    < ((((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut pixelRow: *mut u16 = (((&raw mut gCanvasPixels)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(
                        ((((((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            .wrapping_add(((j) as i32)))
                        .wrapping_mul(
                            ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32),
                        )) as isize,
                    );
                    let mut pixel: *mut u16 = (pixelRow).wrapping_offset(
                        ((((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    );
                    {
                        i = 0u8;
                        'l3: loop {
                            if !(((i) as i32)
                                < ((((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>()).read())
                                    as i32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                if !(((((pixel).read()) as i32) & 32768i32) != 0) {
                                    (pixel).write(QuantizePixel_BlackAndWhite(pixel));
                                }
                            }
                            i = (i).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyImageEffect_BlackOutline() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut pixel: *mut u16 = core::ptr::null_mut();
        {
            j = 0u8;
            'l1: loop {
                if !(((j) as i32)
                    < ((((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut pixelRow: *mut u16 = (((&raw mut gCanvasPixels)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(
                        ((((((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            .wrapping_add(((j) as i32)))
                        .wrapping_mul(
                            ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32),
                        )) as isize,
                    );
                    pixel = (pixelRow).wrapping_offset(
                        ((((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    );
                    (pixel).write(QuantizePixel_BlackOutline(
                        pixel,
                        (pixel).wrapping_offset(1),
                    ));
                    {
                        i = 1u8;
                        pixel = (pixel).wrapping_offset(1);
                        'l3: loop {
                            if !(((i) as i32)
                                < ((((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>()).read())
                                    as i32)
                                    .wrapping_sub(1i32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                (pixel).write(QuantizePixel_BlackOutline(
                                    pixel,
                                    (pixel).wrapping_offset(1),
                                ));
                                (pixel).write(QuantizePixel_BlackOutline(
                                    pixel,
                                    (pixel).wrapping_offset(-1),
                                ));
                            }
                            i = (i).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(1);
                        }
                    }
                    (pixel).write(QuantizePixel_BlackOutline(
                        pixel,
                        (pixel).wrapping_offset(-1),
                    ));
                }
                j = (j).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32)
                    < ((((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l5;
                }
                'l6: {
                    let mut pixelRow: *mut u16 = (((&raw mut gCanvasPixels)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(
                        (((((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            .wrapping_mul(
                                ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read())
                                    as i32),
                            )) as isize,
                    );
                    pixel = (pixelRow).wrapping_offset(
                        (((((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>()).read())
                            as i32)
                            .wrapping_add(((i) as i32))) as isize,
                    );
                    (pixel).write(QuantizePixel_BlackOutline(
                        pixel,
                        (pixel).wrapping_offset(
                            ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize,
                        ),
                    ));
                    {
                        j = 1u8;
                        pixel = (pixel).wrapping_offset(
                            ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize,
                        );
                        'l7: loop {
                            if !(((j) as i32)
                                < ((((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>()).read())
                                    as i32)
                                    .wrapping_sub(1i32))
                            {
                                break 'l7;
                            }
                            'l8: {
                                (pixel).write(QuantizePixel_BlackOutline(
                                    pixel,
                                    (pixel).wrapping_offset(
                                        ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ),
                                ));
                                (pixel).write(QuantizePixel_BlackOutline(
                                    pixel,
                                    (pixel).wrapping_offset(
                                        (((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            .wrapping_neg())
                                            as isize,
                                    ),
                                ));
                            }
                            j = (j).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(
                                ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read())
                                    as i32) as isize,
                            );
                        }
                    }
                    (pixel).write(QuantizePixel_BlackOutline(
                        pixel,
                        (pixel).wrapping_offset(
                            (((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32)
                                .wrapping_neg()) as isize,
                        ),
                    ));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyImageEffect_Invert() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            j = 0u8;
            'l1: loop {
                if !(((j) as i32)
                    < ((((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut pixelRow: *mut u16 = (((&raw mut gCanvasPixels)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(
                        ((((((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            .wrapping_add(((j) as i32)))
                        .wrapping_mul(
                            ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32),
                        )) as isize,
                    );
                    let mut pixel: *mut u16 = (pixelRow).wrapping_offset(
                        ((((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    );
                    {
                        i = 0u8;
                        'l3: loop {
                            if !(((i) as i32)
                                < ((((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>()).read())
                                    as i32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                if !(((((pixel).read()) as i32) & 32768i32) != 0) {
                                    (pixel).write(QuantizePixel_Invert(pixel));
                                }
                            }
                            i = (i).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyImageEffect_Shimmer() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut pixel: *mut u16 = core::ptr::null_mut();
        let mut prevPixel: u16 = 0u16;
        pixel = ((&raw mut gCanvasPixels).cast::<u8>().cast::<*mut u16>()).read();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 64i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 64i32) {
                                break 'l3;
                            }
                            'l4: {
                                if !(((((pixel).read()) as i32) & 32768i32) != 0) {
                                    (pixel).write(QuantizePixel_Invert(pixel));
                                }
                            }
                            j = (j).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            j = 0u8;
            'l5: loop {
                if !(((j) as i32) < 64i32) {
                    break 'l5;
                }
                'l6: {
                    pixel = (((&raw mut gCanvasPixels).cast::<u8>().cast::<*mut u16>()).read())
                        .wrapping_offset(((j) as i32) as isize);
                    prevPixel = (pixel).read();
                    (pixel).write(32768u16);
                    {
                        i = 1u8;
                        pixel = (pixel).wrapping_offset(64);
                        'l7: loop {
                            if !(((i) as i32) < 63i32) {
                                break 'l7;
                            }
                            'l8: {
                                if !(((((pixel).read()) as i32) & 32768i32) != 0) {
                                    (pixel).write(QuantizePixel_BlurHard(
                                        &raw mut prevPixel,
                                        pixel,
                                        (pixel).wrapping_offset(64),
                                    ));
                                    prevPixel = (pixel).read();
                                }
                            }
                            i = (i).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(64);
                        }
                    }
                    (pixel).write(32768u16);
                    pixel = (((&raw mut gCanvasPixels).cast::<u8>().cast::<*mut u16>()).read())
                        .wrapping_offset(((j) as i32) as isize);
                    prevPixel = (pixel).read();
                    (pixel).write(32768u16);
                    {
                        i = 1u8;
                        pixel = (pixel).wrapping_offset(64);
                        'l9: loop {
                            if !(((i) as i32) < 63i32) {
                                break 'l9;
                            }
                            'l10: {
                                if !(((((pixel).read()) as i32) & 32768i32) != 0) {
                                    (pixel).write(QuantizePixel_BlurHard(
                                        &raw mut prevPixel,
                                        pixel,
                                        (pixel).wrapping_offset(64),
                                    ));
                                    prevPixel = (pixel).read();
                                }
                            }
                            i = (i).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(64);
                        }
                    }
                    (pixel).write(32768u16);
                }
                j = (j).wrapping_add(1);
            }
        }
        pixel = ((&raw mut gCanvasPixels).cast::<u8>().cast::<*mut u16>()).read();
        {
            i = 0u8;
            'l11: loop {
                if !(((i) as i32) < 64i32) {
                    break 'l11;
                }
                'l12: {
                    {
                        j = 0u8;
                        'l13: loop {
                            if !(((j) as i32) < 64i32) {
                                break 'l13;
                            }
                            'l14: {
                                if !(((((pixel).read()) as i32) & 32768i32) != 0) {
                                    (pixel).write(QuantizePixel_Invert(pixel));
                                }
                            }
                            j = (j).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyImageEffect_BlurRight() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            j = 0u8;
            'l1: loop {
                if !(((j) as i32)
                    < ((((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut pixelRow: *mut u16 = (((&raw mut gCanvasPixels)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(
                        ((((((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            .wrapping_add(((j) as i32)))
                        .wrapping_mul(
                            ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32),
                        )) as isize,
                    );
                    let mut pixel: *mut u16 = (pixelRow).wrapping_offset(
                        ((((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    );
                    let mut prevPixel: u16 = (pixel).read();
                    {
                        i = 1u8;
                        pixel = (pixel).wrapping_offset(1);
                        'l3: loop {
                            if !(((i) as i32)
                                < ((((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>()).read())
                                    as i32)
                                    .wrapping_sub(1i32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                if !(((((pixel).read()) as i32) & 32768i32) != 0) {
                                    (pixel)
                                        .write(QuantizePixel_MotionBlur(&raw mut prevPixel, pixel));
                                    prevPixel = (pixel).read();
                                }
                            }
                            i = (i).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyImageEffect_BlurDown() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut pixelRow: *mut u16 = (((&raw mut gCanvasPixels)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(
                        (((((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            .wrapping_mul(
                                ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read())
                                    as i32),
                            )) as isize,
                    );
                    let mut pixel: *mut u16 = (pixelRow).wrapping_offset(
                        (((((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>()).read())
                            as i32)
                            .wrapping_add(((i) as i32))) as isize,
                    );
                    let mut prevPixel: u16 = (pixel).read();
                    {
                        j = 1u8;
                        pixel = (pixel).wrapping_offset(
                            ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize,
                        );
                        'l3: loop {
                            if !(((j) as i32)
                                < ((((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>()).read())
                                    as i32)
                                    .wrapping_sub(1i32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                if !(((((pixel).read()) as i32) & 32768i32) != 0) {
                                    (pixel)
                                        .write(QuantizePixel_MotionBlur(&raw mut prevPixel, pixel));
                                    prevPixel = (pixel).read();
                                }
                            }
                            j = (j).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(
                                ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read())
                                    as i32) as isize,
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AddPointillismPoints(point: u16) {
    unsafe {
        let mut point = point;
        let mut i: u8 = 0u8;
        let mut offsetDownLeft: u8 = 0u8;
        let mut colorType: u8 = 0u8;
        let mut points = crate::ffi::Align4([0u8; 24]);
        ((&raw mut points).cast::<u8>()).write(
            (((((&raw const sPointillismPoints).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((point) as i32) as isize * 3))
            .cast::<u8>())
            .read(),
        );
        (((&raw mut points).cast::<u8>()).wrapping_add(1)).write(
            ((((((&raw const sPointillismPoints).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((point) as i32) as isize * 3))
            .cast::<u8>())
            .wrapping_offset(1))
            .read(),
        );
        (((&raw mut points).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(
            (((((((((((&raw const sPointillismPoints).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((point) as i32) as isize * 3))
            .cast::<u8>())
            .wrapping_offset(2))
            .read()) as i32)
                >> 3)
                & 7i32) as u16),
        );
        colorType = (((((((((((&raw const sPointillismPoints).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((point) as i32) as isize * 3))
        .cast::<u8>())
        .wrapping_offset(2))
        .read()) as i32)
            >> 1)
            & 3i32) as u8);
        offsetDownLeft = (((((((((((&raw const sPointillismPoints).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((point) as i32) as isize * 3))
        .cast::<u8>())
        .wrapping_offset(2))
        .read()) as i32)
            >> 0)
            & 1i32) as u8);
        {
            i = 1u8;
            'l1: loop {
                if !(((i) as i32)
                    < (((((&raw mut points).cast::<u8>())
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if !((offsetDownLeft) != 0) {
                        (((&raw mut points).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                        .write(
                            ((((((&raw mut points).cast::<u8>()).read()) as i32)
                                .wrapping_sub(((i) as i32))) as u8),
                        );
                        ((((&raw mut points).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(1))
                        .write(
                            (((((((&raw mut points).cast::<u8>()).wrapping_add(1)).read()) as i32)
                                .wrapping_add(((i) as i32))) as u8),
                        );
                    } else {
                        (((&raw mut points).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                        .write(
                            ((((((&raw mut points).cast::<u8>()).read()) as i32).wrapping_add(1i32))
                                as u8),
                        );
                        ((((&raw mut points).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(1))
                        .write(
                            (((((((&raw mut points).cast::<u8>()).wrapping_add(1)).read()) as i32)
                                .wrapping_sub(1i32)) as u8),
                        );
                    }
                    if ((((((&raw mut points).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                    .read()) as i32)
                        >= 64i32)
                        || (((((((&raw mut points).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(1))
                        .read()) as i32)
                            >= 64i32)
                    {
                        (((&raw mut points).cast::<u8>())
                            .wrapping_add(2)
                            .cast::<u16>())
                        .write(((((i) as i32).wrapping_sub(1i32)) as u16));
                        break 'l1;
                    }
                    ((((&raw mut points).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                    .write(
                        (((((((&raw mut points).cast::<u8>())
                            .wrapping_add(2)
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_sub(((i) as i32))) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32)
                    < (((((&raw mut points).cast::<u8>())
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read()) as i32))
                {
                    break 'l3;
                }
                'l4: {
                    let mut pixel: *mut u16 =
                        ((((&raw mut gCanvasPixels).cast::<u8>().cast::<*mut u16>()).read())
                            .wrapping_offset(
                                (((((((&raw mut points).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                .wrapping_add(1))
                                .read()) as i32)
                                    .wrapping_mul(64i32)) as isize,
                            ))
                        .wrapping_offset(
                            (((((&raw mut points).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                            .read()) as i32) as isize,
                        );
                    if !(((((pixel).read()) as i32) & 32768i32) != 0) {
                        let mut red: u16 = (((((pixel).read()) as i32) & 31i32) as u16);
                        let mut green: u16 = ((((((pixel).read()) as i32) >> 5) & 31i32) as u16);
                        let mut blue: u16 = ((((((pixel).read()) as i32) >> 10) & 31i32) as u16);
                        'l5: {
                            let __sw1 = ((colorType) as i32);
                            if __sw1 == 0i32 || __sw1 == 1i32 {
                                'l6: {
                                    let __sw2 = crate::c::rem_i32(
                                        ((((((((((&raw const sPointillismPoints)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((point) as i32) as isize * 3))
                                        .cast::<u8>())
                                        .wrapping_offset(2))
                                        .read())
                                            as i32)
                                            >> 3)
                                            & 7i32),
                                        3i32,
                                    );
                                    if __sw2 == 0i32 {
                                        if ((red) as i32)
                                            >= ((((((&raw mut points).cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize * 4))
                                            .wrapping_add(2)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                        {
                                            red = ((((red) as i32).wrapping_sub(
                                                ((((((&raw mut points).cast::<u8>())
                                                    .wrapping_offset(((i) as i32) as isize * 4))
                                                .wrapping_add(2)
                                                .cast::<u16>())
                                                .read())
                                                    as i32),
                                            ))
                                                as u16);
                                        } else {
                                            red = 0u16;
                                        }
                                        break 'l6;
                                    }
                                    if __sw2 == 1i32 {
                                        if ((green) as i32)
                                            >= ((((((&raw mut points).cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize * 4))
                                            .wrapping_add(2)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                        {
                                            green = ((((green) as i32).wrapping_sub(
                                                ((((((&raw mut points).cast::<u8>())
                                                    .wrapping_offset(((i) as i32) as isize * 4))
                                                .wrapping_add(2)
                                                .cast::<u16>())
                                                .read())
                                                    as i32),
                                            ))
                                                as u16);
                                        } else {
                                            green = 0u16;
                                        }
                                        break 'l6;
                                    }
                                    if __sw2 == 2i32 {
                                        if ((blue) as i32)
                                            >= ((((((&raw mut points).cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize * 4))
                                            .wrapping_add(2)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                        {
                                            blue = ((((blue) as i32).wrapping_sub(
                                                ((((((&raw mut points).cast::<u8>())
                                                    .wrapping_offset(((i) as i32) as isize * 4))
                                                .wrapping_add(2)
                                                .cast::<u16>())
                                                .read())
                                                    as i32),
                                            ))
                                                as u16);
                                        } else {
                                            blue = 0u16;
                                        }
                                        break 'l6;
                                    }
                                }
                                break 'l5;
                            }
                            if __sw1 == 2i32 || __sw1 == 3i32 {
                                red = ((((red) as i32).wrapping_add(
                                    ((((((&raw mut points).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as i32),
                                )) as u16);
                                green = ((((green) as i32).wrapping_add(
                                    ((((((&raw mut points).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as i32),
                                )) as u16);
                                blue = ((((blue) as i32).wrapping_add(
                                    ((((((&raw mut points).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as i32),
                                )) as u16);
                                if ((red) as i32) > 31i32 {
                                    red = 31u16;
                                }
                                if ((green) as i32) > 31i32 {
                                    green = 31u16;
                                }
                                if ((blue) as i32) > 31i32 {
                                    blue = 31u16;
                                }
                                break 'l5;
                            }
                        }
                        (pixel).write(
                            ((((((blue) as i32) << 10) | (((green) as i32) << 5)) | ((red) as i32))
                                as u16),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ConvertColorToGrayscale(color: *mut u16) -> u16 {
    unsafe {
        let mut color = color;
        let mut clr: i32 = (((color).read()) as i32);
        let mut r: i32 = (clr & 31i32);
        let mut g: i32 = ((clr >> 5) & 31i32);
        let mut b: i32 = ((clr >> 10) & 31i32);
        let mut gray: i32 = ((((r)
            .wrapping_mul((((((0.3f32) as f32) * ((256i32) as f32)) as i16) as i32)))
        .wrapping_add(
            (g).wrapping_mul((((((0.59f32) as f32) * ((256i32) as f32)) as i16) as i32)),
        ))
        .wrapping_add(
            (b).wrapping_mul((((((0.1133f32) as f32) * ((256i32) as f32)) as i16) as i32)),
        ) >> 8);
        return ((((gray << 10) | (gray << 5)) | gray) as u16);
    }
}
pub(crate) unsafe extern "C" fn QuantizePixel_PersonalityColor(
    color: *mut u16,
    personality: u8,
) -> u16 {
    unsafe {
        let mut color = color;
        let mut personality = personality;
        let mut red: u16 = (((((color).read()) as i32) & 31i32) as u16);
        let mut green: u16 = ((((((color).read()) as i32) >> 5) & 31i32) as u16);
        let mut blue: u16 = ((((((color).read()) as i32) >> 10) & 31i32) as u16);
        if ((((red) as i32) < 17i32) && (((green) as i32) < 17i32)) && (((blue) as i32) < 17i32) {
            return GetColorFromPersonality(personality);
        } else {
            return 32767u16;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn GetColorFromPersonality(personality: u8) -> u16 {
    unsafe {
        let mut personality = personality;
        let mut red: u16 = 0u16;
        let mut green: u16 = 0u16;
        let mut blue: u16 = 0u16;
        let mut strength: u8 =
            ((crate::c::rem_i32(crate::c::div_i32(((personality) as i32), 6i32), 3i32)) as u8);
        let mut colorType: u8 = ((crate::c::rem_i32(((personality) as i32), 6i32)) as u8);
        'l1: {
            let __sw1 = ((colorType) as i32);
            if __sw1 == 0i32 {
                green = (((21i32).wrapping_sub(((strength) as i32))) as u16);
                blue = green;
                red = 0u16;
                break 'l1;
            }
            if __sw1 == 1i32 {
                blue = 0u16;
                red = (((21i32).wrapping_sub(((strength) as i32))) as u16);
                green = red;
                break 'l1;
            }
            if __sw1 == 2i32 {
                blue = (((21i32).wrapping_sub(((strength) as i32))) as u16);
                green = 0u16;
                red = blue;
                break 'l1;
            }
            if __sw1 == 3i32 {
                blue = 0u16;
                green = 0u16;
                red = (((23i32).wrapping_sub(((strength) as i32))) as u16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                blue = (((23i32).wrapping_sub(((strength) as i32))) as u16);
                green = 0u16;
                red = 0u16;
                break 'l1;
            }
            if __sw1 == 5i32 {
                blue = 0u16;
                green = (((23i32).wrapping_sub(((strength) as i32))) as u16);
                red = 0u16;
                break 'l1;
            }
        }
        return ((((((blue) as i32) << 10) | (((green) as i32) << 5)) | ((red) as i32)) as u16);
    }
}
pub(crate) unsafe extern "C" fn QuantizePixel_BlackAndWhite(color: *mut u16) -> u16 {
    unsafe {
        let mut color = color;
        let mut red: u16 = (((((color).read()) as i32) & 31i32) as u16);
        let mut green: u16 = ((((((color).read()) as i32) >> 5) & 31i32) as u16);
        let mut blue: u16 = ((((((color).read()) as i32) >> 10) & 31i32) as u16);
        if ((((red) as i32) < 17i32) && (((green) as i32) < 17i32)) && (((blue) as i32) < 17i32) {
            return 0u16;
        } else {
            return 32767u16;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn QuantizePixel_BlackOutline(
    pixelA: *mut u16,
    pixelB: *mut u16,
) -> u16 {
    unsafe {
        let mut pixelA = pixelA;
        let mut pixelB = pixelB;
        if (((pixelA).read()) as i32) != 0i32 {
            if ((((pixelA).read()) as i32) & 32768i32) != 0 {
                return 32768u16;
            }
            if ((((pixelB).read()) as i32) & 32768i32) != 0 {
                return 0u16;
            }
            return (pixelA).read();
        }
        return 0u16;
    }
}
pub(crate) unsafe extern "C" fn QuantizePixel_Invert(color: *mut u16) -> u16 {
    unsafe {
        let mut color = color;
        let mut red: u16 = (((((color).read()) as i32) & 31i32) as u16);
        let mut green: u16 = ((((((color).read()) as i32) >> 5) & 31i32) as u16);
        let mut blue: u16 = ((((((color).read()) as i32) >> 10) & 31i32) as u16);
        red = (((31i32).wrapping_sub(((red) as i32))) as u16);
        green = (((31i32).wrapping_sub(((green) as i32))) as u16);
        blue = (((31i32).wrapping_sub(((blue) as i32))) as u16);
        return ((((((blue) as i32) << 10) | (((green) as i32) << 5)) | ((red) as i32)) as u16);
    }
}
pub(crate) unsafe extern "C" fn QuantizePixel_MotionBlur(
    prevPixel: *mut u16,
    curPixel: *mut u16,
) -> u16 {
    unsafe {
        let mut prevPixel = prevPixel;
        let mut curPixel = curPixel;
        let mut pixelChannels = crate::ffi::Align4([0u8; 12]);
        let mut diffs = crate::ffi::Align4([0u8; 6]);
        let mut i: u8 = 0u8;
        let mut largestDiff: u16 = 0u16;
        let mut red: u16 = 0u16;
        let mut green: u16 = 0u16;
        let mut blue: u16 = 0u16;
        if (((prevPixel).read()) as i32) == (((curPixel).read()) as i32) {
            return (curPixel).read();
        }
        (((&raw mut pixelChannels).cast::<u8>()).cast::<u16>())
            .write((((((prevPixel).read()) as i32) & 31i32) as u16));
        ((((&raw mut pixelChannels).cast::<u8>()).cast::<u16>()).wrapping_offset(1))
            .write(((((((prevPixel).read()) as i32) >> 5) & 31i32) as u16));
        ((((&raw mut pixelChannels).cast::<u8>()).cast::<u16>()).wrapping_offset(2))
            .write(((((((prevPixel).read()) as i32) >> 10) & 31i32) as u16));
        ((((&raw mut pixelChannels).cast::<u8>()).wrapping_offset(6)).cast::<u16>())
            .write((((((curPixel).read()) as i32) & 31i32) as u16));
        (((((&raw mut pixelChannels).cast::<u8>()).wrapping_offset(6)).cast::<u16>())
            .wrapping_offset(1))
        .write(((((((curPixel).read()) as i32) >> 5) & 31i32) as u16));
        (((((&raw mut pixelChannels).cast::<u8>()).wrapping_offset(6)).cast::<u16>())
            .wrapping_offset(2))
        .write(((((((curPixel).read()) as i32) >> 10) & 31i32) as u16));
        if (((((((&raw mut pixelChannels).cast::<u8>()).cast::<u16>()).read()) as i32) > 25i32)
            && (((((((&raw mut pixelChannels).cast::<u8>()).cast::<u16>()).wrapping_offset(1))
                .read()) as i32)
                > 25i32))
            && (((((((&raw mut pixelChannels).cast::<u8>()).cast::<u16>()).wrapping_offset(2))
                .read()) as i32)
                > 25i32)
        {
            return (curPixel).read();
        }
        if ((((((((&raw mut pixelChannels).cast::<u8>()).wrapping_offset(6)).cast::<u16>()).read())
            as i32)
            > 25i32)
            && ((((((((&raw mut pixelChannels).cast::<u8>()).wrapping_offset(6)).cast::<u16>())
                .wrapping_offset(1))
            .read()) as i32)
                > 25i32))
            && ((((((((&raw mut pixelChannels).cast::<u8>()).wrapping_offset(6)).cast::<u16>())
                .wrapping_offset(2))
            .read()) as i32)
                > 25i32)
        {
            return (curPixel).read();
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut pixelChannels).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        > (((((((&raw mut pixelChannels).cast::<u8>()).wrapping_offset(6))
                            .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                    {
                        (((&raw mut diffs).cast::<u16>()).wrapping_offset(((i) as i32) as isize))
                            .write(
                                ((((((((&raw mut pixelChannels).cast::<u8>()).cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    .wrapping_sub(
                                        (((((((&raw mut pixelChannels).cast::<u8>())
                                            .wrapping_offset(6))
                                        .cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .read()) as i32),
                                    )) as u16),
                            );
                    } else {
                        (((&raw mut diffs).cast::<u16>()).wrapping_offset(((i) as i32) as isize))
                            .write(
                                (((((((((&raw mut pixelChannels).cast::<u8>())
                                    .wrapping_offset(6))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    .wrapping_sub(
                                        ((((((&raw mut pixelChannels).cast::<u8>()).cast::<u16>())
                                            .wrapping_offset(((i) as i32) as isize))
                                        .read()) as i32),
                                    )) as u16),
                            );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((&raw mut diffs).cast::<u16>()).read()) as i32)
            >= (((((&raw mut diffs).cast::<u16>()).wrapping_offset(1)).read()) as i32)
        {
            if ((((&raw mut diffs).cast::<u16>()).read()) as i32)
                >= (((((&raw mut diffs).cast::<u16>()).wrapping_offset(2)).read()) as i32)
            {
                largestDiff = ((&raw mut diffs).cast::<u16>()).read();
            } else {
                if (((((&raw mut diffs).cast::<u16>()).wrapping_offset(1)).read()) as i32)
                    >= (((((&raw mut diffs).cast::<u16>()).wrapping_offset(2)).read()) as i32)
                {
                    largestDiff = (((&raw mut diffs).cast::<u16>()).wrapping_offset(1)).read();
                } else {
                    largestDiff = (((&raw mut diffs).cast::<u16>()).wrapping_offset(2)).read();
                }
            }
        } else {
            if (((((&raw mut diffs).cast::<u16>()).wrapping_offset(1)).read()) as i32)
                >= (((((&raw mut diffs).cast::<u16>()).wrapping_offset(2)).read()) as i32)
            {
                largestDiff = (((&raw mut diffs).cast::<u16>()).wrapping_offset(1)).read();
            } else {
                if (((((&raw mut diffs).cast::<u16>()).wrapping_offset(2)).read()) as i32)
                    >= ((((&raw mut diffs).cast::<u16>()).read()) as i32)
                {
                    largestDiff = (((&raw mut diffs).cast::<u16>()).wrapping_offset(2)).read();
                } else {
                    largestDiff = ((&raw mut diffs).cast::<u16>()).read();
                }
            }
        }
        red = ((crate::c::div_i32(
            ((((((&raw mut pixelChannels).cast::<u8>()).wrapping_offset(6)).cast::<u16>()).read())
                as i32)
                .wrapping_mul(
                    (31i32).wrapping_sub(crate::c::div_i32(((largestDiff) as i32), 2i32)),
                ),
            31i32,
        )) as u16);
        green = ((crate::c::div_i32(
            (((((((&raw mut pixelChannels).cast::<u8>()).wrapping_offset(6)).cast::<u16>())
                .wrapping_offset(1))
            .read()) as i32)
                .wrapping_mul(
                    (31i32).wrapping_sub(crate::c::div_i32(((largestDiff) as i32), 2i32)),
                ),
            31i32,
        )) as u16);
        blue = ((crate::c::div_i32(
            (((((((&raw mut pixelChannels).cast::<u8>()).wrapping_offset(6)).cast::<u16>())
                .wrapping_offset(2))
            .read()) as i32)
                .wrapping_mul(
                    (31i32).wrapping_sub(crate::c::div_i32(((largestDiff) as i32), 2i32)),
                ),
            31i32,
        )) as u16);
        return ((((((blue) as i32) << 10) | (((green) as i32) << 5)) | ((red) as i32)) as u16);
    }
}
pub(crate) unsafe extern "C" fn QuantizePixel_Blur(
    prevPixel: *mut u16,
    curPixel: *mut u16,
    nextPixel: *mut u16,
) -> u16 {
    unsafe {
        let mut prevPixel = prevPixel;
        let mut curPixel = curPixel;
        let mut nextPixel = nextPixel;
        let mut red: u16 = 0u16;
        let mut green: u16 = 0u16;
        let mut blue: u16 = 0u16;
        let mut prevAvg: u16 = 0u16;
        let mut curAvg: u16 = 0u16;
        let mut nextAvg: u16 = 0u16;
        let mut prevDiff: u16 = 0u16;
        let mut nextDiff: u16 = 0u16;
        let mut diff: u32 = 0u32;
        let mut factor: u16 = 0u16;
        if ((((prevPixel).read()) as i32) == (((curPixel).read()) as i32))
            && ((((nextPixel).read()) as i32) == (((curPixel).read()) as i32))
        {
            return (curPixel).read();
        }
        red = (((((curPixel).read()) as i32) & 31i32) as u16);
        green = ((((((curPixel).read()) as i32) >> 5) & 31i32) as u16);
        blue = ((((((curPixel).read()) as i32) >> 10) & 31i32) as u16);
        prevAvg = ((crate::c::div_i32(
            (((((prevPixel).read()) as i32) & 31i32)
                .wrapping_add((((((prevPixel).read()) as i32) >> 5) & 31i32)))
            .wrapping_add((((((prevPixel).read()) as i32) >> 10) & 31i32)),
            3i32,
        )) as u16);
        curAvg = ((crate::c::div_i32(
            (((((curPixel).read()) as i32) & 31i32)
                .wrapping_add((((((curPixel).read()) as i32) >> 5) & 31i32)))
            .wrapping_add((((((curPixel).read()) as i32) >> 10) & 31i32)),
            3i32,
        )) as u16);
        nextAvg = ((crate::c::div_i32(
            (((((nextPixel).read()) as i32) & 31i32)
                .wrapping_add((((((nextPixel).read()) as i32) >> 5) & 31i32)))
            .wrapping_add((((((nextPixel).read()) as i32) >> 10) & 31i32)),
            3i32,
        )) as u16);
        if (((prevAvg) as i32) == ((curAvg) as i32)) && (((nextAvg) as i32) == ((curAvg) as i32)) {
            return (curPixel).read();
        }
        if ((prevAvg) as i32) > ((curAvg) as i32) {
            prevDiff = ((((prevAvg) as i32).wrapping_sub(((curAvg) as i32))) as u16);
        } else {
            prevDiff = ((((curAvg) as i32).wrapping_sub(((prevAvg) as i32))) as u16);
        }
        if ((nextAvg) as i32) > ((curAvg) as i32) {
            nextDiff = ((((nextAvg) as i32).wrapping_sub(((curAvg) as i32))) as u16);
        } else {
            nextDiff = ((((curAvg) as i32).wrapping_sub(((nextAvg) as i32))) as u16);
        }
        if ((prevDiff) as i32) >= ((nextDiff) as i32) {
            diff = ((prevDiff) as u32);
        } else {
            diff = ((nextDiff) as u32);
        }
        factor = (((31u32).wrapping_sub(crate::c::div_u32(diff, 2u32))) as u16);
        red = ((crate::c::div_i32(((red) as i32).wrapping_mul(((factor) as i32)), 31i32)) as u16);
        green =
            ((crate::c::div_i32(((green) as i32).wrapping_mul(((factor) as i32)), 31i32)) as u16);
        blue = ((crate::c::div_i32(((blue) as i32).wrapping_mul(((factor) as i32)), 31i32)) as u16);
        return ((((((blue) as i32) << 10) | (((green) as i32) << 5)) | ((red) as i32)) as u16);
    }
}
pub(crate) unsafe extern "C" fn QuantizePixel_BlurHard(
    prevPixel: *mut u16,
    curPixel: *mut u16,
    nextPixel: *mut u16,
) -> u16 {
    unsafe {
        let mut prevPixel = prevPixel;
        let mut curPixel = curPixel;
        let mut nextPixel = nextPixel;
        let mut red: u16 = 0u16;
        let mut green: u16 = 0u16;
        let mut blue: u16 = 0u16;
        let mut prevAvg: u16 = 0u16;
        let mut curAvg: u16 = 0u16;
        let mut nextAvg: u16 = 0u16;
        let mut prevDiff: u16 = 0u16;
        let mut nextDiff: u16 = 0u16;
        let mut diff: u32 = 0u32;
        let mut factor: u16 = 0u16;
        if ((((prevPixel).read()) as i32) == (((curPixel).read()) as i32))
            && ((((nextPixel).read()) as i32) == (((curPixel).read()) as i32))
        {
            return (curPixel).read();
        }
        red = (((((curPixel).read()) as i32) & 31i32) as u16);
        green = ((((((curPixel).read()) as i32) >> 5) & 31i32) as u16);
        blue = ((((((curPixel).read()) as i32) >> 10) & 31i32) as u16);
        prevAvg = ((crate::c::div_i32(
            (((((prevPixel).read()) as i32) & 31i32)
                .wrapping_add((((((prevPixel).read()) as i32) >> 5) & 31i32)))
            .wrapping_add((((((prevPixel).read()) as i32) >> 10) & 31i32)),
            3i32,
        )) as u16);
        curAvg = ((crate::c::div_i32(
            (((((curPixel).read()) as i32) & 31i32)
                .wrapping_add((((((curPixel).read()) as i32) >> 5) & 31i32)))
            .wrapping_add((((((curPixel).read()) as i32) >> 10) & 31i32)),
            3i32,
        )) as u16);
        nextAvg = ((crate::c::div_i32(
            (((((nextPixel).read()) as i32) & 31i32)
                .wrapping_add((((((nextPixel).read()) as i32) >> 5) & 31i32)))
            .wrapping_add((((((nextPixel).read()) as i32) >> 10) & 31i32)),
            3i32,
        )) as u16);
        if (((prevAvg) as i32) == ((curAvg) as i32)) && (((nextAvg) as i32) == ((curAvg) as i32)) {
            return (curPixel).read();
        }
        if ((prevAvg) as i32) > ((curAvg) as i32) {
            prevDiff = ((((prevAvg) as i32).wrapping_sub(((curAvg) as i32))) as u16);
        } else {
            prevDiff = ((((curAvg) as i32).wrapping_sub(((prevAvg) as i32))) as u16);
        }
        if ((nextAvg) as i32) > ((curAvg) as i32) {
            nextDiff = ((((nextAvg) as i32).wrapping_sub(((curAvg) as i32))) as u16);
        } else {
            nextDiff = ((((curAvg) as i32).wrapping_sub(((nextAvg) as i32))) as u16);
        }
        if ((prevDiff) as i32) >= ((nextDiff) as i32) {
            diff = ((prevDiff) as u32);
        } else {
            diff = ((nextDiff) as u32);
        }
        factor = (((31u32).wrapping_sub(diff)) as u16);
        red = ((crate::c::div_i32(((red) as i32).wrapping_mul(((factor) as i32)), 31i32)) as u16);
        green =
            ((crate::c::div_i32(((green) as i32).wrapping_mul(((factor) as i32)), 31i32)) as u16);
        blue = ((crate::c::div_i32(((blue) as i32).wrapping_mul(((factor) as i32)), 31i32)) as u16);
        return ((((((blue) as i32) << 10) | (((green) as i32) << 5)) | ((red) as i32)) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertImageProcessingToGBA(context: *mut u8) {
    unsafe {
        let mut context = context;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        let mut k: u16 = 0u16;
        let mut src: *mut u16 = core::ptr::null_mut();
        let mut dest: *mut u16 = core::ptr::null_mut();
        let mut src_: *mut u16 = core::ptr::null_mut();
        let mut dest_: *mut u16 = core::ptr::null_mut();
        let mut width: u16 = 0u16;
        let mut height: u16 = 0u16;
        width = ((((((context).wrapping_add(29)).read()) as i32) >> 3) as u16);
        height = ((((((context).wrapping_add(30)).read()) as i32) >> 3) as u16);
        src_ = (((context).wrapping_add(4).cast::<*mut u8>()).read()).cast::<u16>();
        dest_ = (((context).wrapping_add(16).cast::<*mut u8>()).read()).cast::<u16>();
        if ((((context).wrapping_add(22).cast::<u16>()).read()) as i32) == 2i32 {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < ((height) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        {
                            j = 0u16;
                            'l3: loop {
                                if !(((j) as i32) < ((width) as i32)) {
                                    break 'l3;
                                }
                                'l4: {
                                    {
                                        k = 0u16;
                                        'l5: loop {
                                            if !(((k) as i32) < 8i32) {
                                                break 'l5;
                                            }
                                            'l6: {
                                                dest = ((dest_).wrapping_offset(
                                                    ((((i) as i32).wrapping_mul(((width) as i32)))
                                                        .wrapping_add(((j) as i32))
                                                        << 5)
                                                        as isize,
                                                ))
                                                .wrapping_offset((((k) as i32) << 2) as isize);
                                                src = ((src_).wrapping_offset(
                                                    (((((i) as i32) << 3)
                                                        .wrapping_add(((k) as i32))
                                                        << 3)
                                                        .wrapping_mul(((width) as i32)))
                                                        as isize,
                                                ))
                                                .wrapping_offset((((j) as i32) << 3) as isize);
                                                (dest).write(
                                                    (((((src).read()) as i32)
                                                        | (((((src).wrapping_offset(1)).read())
                                                            as i32)
                                                            << 8))
                                                        as u16),
                                                );
                                                ((dest).wrapping_offset(1)).write(
                                                    ((((((src).wrapping_offset(2)).read()) as i32)
                                                        | (((((src).wrapping_offset(3)).read())
                                                            as i32)
                                                            << 8))
                                                        as u16),
                                                );
                                                ((dest).wrapping_offset(2)).write(
                                                    ((((((src).wrapping_offset(4)).read()) as i32)
                                                        | (((((src).wrapping_offset(5)).read())
                                                            as i32)
                                                            << 8))
                                                        as u16),
                                                );
                                                ((dest).wrapping_offset(3)).write(
                                                    ((((((src).wrapping_offset(6)).read()) as i32)
                                                        | (((((src).wrapping_offset(7)).read())
                                                            as i32)
                                                            << 8))
                                                        as u16),
                                                );
                                            }
                                            k = (k).wrapping_add(1);
                                        }
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0u16;
                'l7: loop {
                    if !(((i) as i32) < ((height) as i32)) {
                        break 'l7;
                    }
                    'l8: {
                        {
                            j = 0u16;
                            'l9: loop {
                                if !(((j) as i32) < ((width) as i32)) {
                                    break 'l9;
                                }
                                'l10: {
                                    {
                                        k = 0u16;
                                        'l11: loop {
                                            if !(((k) as i32) < 8i32) {
                                                break 'l11;
                                            }
                                            'l12: {
                                                dest = ((dest_).wrapping_offset(
                                                    ((((i) as i32).wrapping_mul(((width) as i32)))
                                                        .wrapping_add(((j) as i32))
                                                        << 4)
                                                        as isize,
                                                ))
                                                .wrapping_offset((((k) as i32) << 1) as isize);
                                                src = ((src_).wrapping_offset(
                                                    (((((i) as i32) << 3)
                                                        .wrapping_add(((k) as i32))
                                                        << 3)
                                                        .wrapping_mul(((width) as i32)))
                                                        as isize,
                                                ))
                                                .wrapping_offset((((j) as i32) << 3) as isize);
                                                (dest).write(
                                                    (((((((src).read()) as i32)
                                                        | (((((src).wrapping_offset(1)).read())
                                                            as i32)
                                                            << 4))
                                                        | (((((src).wrapping_offset(2)).read())
                                                            as i32)
                                                            << 8))
                                                        | (((((src).wrapping_offset(3)).read())
                                                            as i32)
                                                            << 12))
                                                        as u16),
                                                );
                                                ((dest).wrapping_offset(1)).write(
                                                    ((((((((src).wrapping_offset(4)).read())
                                                        as i32)
                                                        | (((((src).wrapping_offset(5)).read())
                                                            as i32)
                                                            << 4))
                                                        | (((((src).wrapping_offset(6)).read())
                                                            as i32)
                                                            << 8))
                                                        | (((((src).wrapping_offset(7)).read())
                                                            as i32)
                                                            << 12))
                                                        as u16),
                                                );
                                            }
                                            k = (k).wrapping_add(1);
                                        }
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApplyImageProcessingQuantization(context: *mut u8) {
    unsafe {
        let mut context = context;
        ((&raw mut gCanvasPaletteStart).cast::<u8>().cast::<u16>())
            .write(((((((context).wrapping_add(24)).read()) as i32).wrapping_mul(16i32)) as u16));
        ((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).write(
            (((context).wrapping_add(8).cast::<*mut u16>()).read()).wrapping_offset(
                ((((&raw mut gCanvasPaletteStart).cast::<u8>().cast::<u16>()).read()) as i32)
                    as isize,
            ),
        );
        ((&raw mut gCanvasPixels).cast::<u8>().cast::<*mut u16>())
            .write((((context).wrapping_add(4).cast::<*mut u8>()).read()).cast::<u16>());
        ((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>())
            .write(((context).wrapping_add(25)).read());
        ((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>())
            .write(((context).wrapping_add(26)).read());
        ((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>())
            .write(((context).wrapping_add(27)).read());
        ((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>())
            .write(((context).wrapping_add(28)).read());
        ((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>())
            .write(((context).wrapping_add(29)).read());
        ((&raw mut gCanvasHeight).cast::<u8>().cast::<u8>())
            .write(((context).wrapping_add(30)).read());
        'l1: {
            let __sw1 = ((((context).wrapping_add(20).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                QuantizePalette_Standard(0u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                QuantizePalette_Standard(1u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetPresetPalette_PrimaryColors();
                QuantizePalette_PrimaryColors();
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetPresetPalette_Grayscale();
                QuantizePalette_Grayscale();
                break 'l1;
            }
            if __sw1 == 4i32 {
                SetPresetPalette_GrayscaleSmall();
                QuantizePalette_GrayscaleSmall();
                break 'l1;
            }
            if __sw1 == 5i32 {
                SetPresetPalette_BlackAndWhite();
                QuantizePalette_BlackAndWhite();
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetPresetPalette_PrimaryColors() {
    unsafe {
        (((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).write(0u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(1))
            .write(6342u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(2))
            .write(30653u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(3))
            .write(11627u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(4))
            .write(6365u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(5))
            .write(7078u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(6))
            .write(29894u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(7))
            .write(7101u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(8))
            .write(29917u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(9))
            .write(30630u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(10))
            .write(6525u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(11))
            .write(7083u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(12))
            .write(30054u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(13))
            .write(11485u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(14))
            .write(12198u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(15))
            .write(29899u16);
    }
}
pub(crate) unsafe extern "C" fn SetPresetPalette_BlackAndWhite() {
    unsafe {
        (((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).write(0u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(1))
            .write(0u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(2))
            .write(32767u16);
    }
}
pub(crate) unsafe extern "C" fn SetPresetPalette_GrayscaleSmall() {
    unsafe {
        let mut i: u8 = 0u8;
        (((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).write(0u16);
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(1))
            .write(0u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 14i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read())
                        .wrapping_offset((((i) as i32).wrapping_add(2i32)) as isize))
                    .write(
                        (((((2i32).wrapping_mul(((i) as i32).wrapping_add(2i32)) << 10)
                            | ((2i32).wrapping_mul(((i) as i32).wrapping_add(2i32)) << 5))
                            | (2i32).wrapping_mul(((i) as i32).wrapping_add(2i32)))
                            as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetPresetPalette_Grayscale() {
    unsafe {
        let mut i: u8 = 0u8;
        (((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read()).write(0u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 32i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read())
                        .wrapping_offset((((i) as i32).wrapping_add(1i32)) as isize))
                    .write(((((((i) as i32) << 10) | (((i) as i32) << 5)) | ((i) as i32)) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn QuantizePalette_Standard(useLimitedPalette: u8) {
    unsafe {
        let mut useLimitedPalette = useLimitedPalette;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut maxIndex: u16 = 0u16;
        maxIndex = 223u16;
        if !((useLimitedPalette) != 0) {
            maxIndex = 255u16;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((maxIndex) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gCanvasPalette).cast::<u8>().cast::<*mut u16>()).read())
            .wrapping_offset(((maxIndex) as i32) as isize))
        .write(15855u16);
        {
            j = 0u8;
            'l3: loop {
                if !(((j) as i32)
                    < ((((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l3;
                }
                'l4: {
                    let mut pixelRow: *mut u16 = (((&raw mut gCanvasPixels)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(
                        ((((((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            .wrapping_add(((j) as i32)))
                        .wrapping_mul(
                            ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32),
                        )) as isize,
                    );
                    let mut pixel: *mut u16 = (pixelRow).wrapping_offset(
                        ((((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    );
                    {
                        i = 0u8;
                        'l5: loop {
                            if !(((i) as i32)
                                < ((((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>()).read())
                                    as i32))
                            {
                                break 'l5;
                            }
                            'l6: {
                                if ((((pixel).read()) as i32) & 32768i32) != 0 {
                                    (pixel).write(
                                        ((&raw mut gCanvasPaletteStart).cast::<u8>().cast::<u16>())
                                            .read(),
                                    );
                                } else {
                                    let mut quantizedColor: u16 = QuantizePixel_Standard(pixel);
                                    let mut curIndex: u8 = 1u8;
                                    if ((curIndex) as i32) < ((maxIndex) as i32) {
                                        if ((((((&raw mut gCanvasPalette)
                                            .cast::<u8>()
                                            .cast::<*mut u16>())
                                        .read())
                                        .wrapping_offset(((curIndex) as i32) as isize))
                                        .read()) as i32)
                                            == 0i32
                                        {
                                            ((((&raw mut gCanvasPalette)
                                                .cast::<u8>()
                                                .cast::<*mut u16>())
                                            .read())
                                            .wrapping_offset(((curIndex) as i32) as isize))
                                            .write(quantizedColor);
                                            (pixel).write(
                                                ((((((&raw mut gCanvasPaletteStart)
                                                    .cast::<u8>()
                                                    .cast::<u16>())
                                                .read())
                                                    as i32)
                                                    .wrapping_add(((curIndex) as i32)))
                                                    as u16),
                                            );
                                        } else {
                                            'l7: loop {
                                                if !(((curIndex) as i32) < ((maxIndex) as i32)) {
                                                    break 'l7;
                                                }
                                                if ((((((&raw mut gCanvasPalette)
                                                    .cast::<u8>()
                                                    .cast::<*mut u16>())
                                                .read())
                                                .wrapping_offset(((curIndex) as i32) as isize))
                                                .read())
                                                    as i32)
                                                    == 0i32
                                                {
                                                    ((((&raw mut gCanvasPalette)
                                                        .cast::<u8>()
                                                        .cast::<*mut u16>())
                                                    .read())
                                                    .wrapping_offset(((curIndex) as i32) as isize))
                                                    .write(quantizedColor);
                                                    (pixel).write(
                                                        ((((((&raw mut gCanvasPaletteStart)
                                                            .cast::<u8>()
                                                            .cast::<u16>())
                                                        .read())
                                                            as i32)
                                                            .wrapping_add(((curIndex) as i32)))
                                                            as u16),
                                                    );
                                                    break 'l7;
                                                }
                                                if ((((((&raw mut gCanvasPalette)
                                                    .cast::<u8>()
                                                    .cast::<*mut u16>())
                                                .read())
                                                .wrapping_offset(((curIndex) as i32) as isize))
                                                .read())
                                                    as i32)
                                                    == ((quantizedColor) as i32)
                                                {
                                                    (pixel).write(
                                                        ((((((&raw mut gCanvasPaletteStart)
                                                            .cast::<u8>()
                                                            .cast::<u16>())
                                                        .read())
                                                            as i32)
                                                            .wrapping_add(((curIndex) as i32)))
                                                            as u16),
                                                    );
                                                    break 'l7;
                                                }
                                                curIndex = (curIndex).wrapping_add(1);
                                            }
                                        }
                                    }
                                    if ((curIndex) as i32) == ((maxIndex) as i32) {
                                        curIndex = ((maxIndex) as u8);
                                        (pixel).write(((curIndex) as u16));
                                    }
                                }
                            }
                            i = (i).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn QuantizePalette_BlackAndWhite() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            j = 0u8;
            'l1: loop {
                if !(((j) as i32)
                    < ((((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut pixelRow: *mut u16 = (((&raw mut gCanvasPixels)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(
                        ((((((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            .wrapping_add(((j) as i32)))
                        .wrapping_mul(
                            ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32),
                        )) as isize,
                    );
                    let mut pixel: *mut u16 = (pixelRow).wrapping_offset(
                        ((((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    );
                    {
                        i = 0u8;
                        'l3: loop {
                            if !(((i) as i32)
                                < ((((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>()).read())
                                    as i32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                if ((((pixel).read()) as i32) & 32768i32) != 0 {
                                    (pixel).write(
                                        ((&raw mut gCanvasPaletteStart).cast::<u8>().cast::<u16>())
                                            .read(),
                                    );
                                } else {
                                    if ((QuantizePixel_BlackAndWhite(pixel)) as i32) == 0i32 {
                                        (pixel).write(
                                            ((((((&raw mut gCanvasPaletteStart)
                                                .cast::<u8>()
                                                .cast::<u16>())
                                            .read())
                                                as i32)
                                                .wrapping_add(1i32))
                                                as u16),
                                        );
                                    } else {
                                        (pixel).write(
                                            ((((((&raw mut gCanvasPaletteStart)
                                                .cast::<u8>()
                                                .cast::<u16>())
                                            .read())
                                                as i32)
                                                .wrapping_add(2i32))
                                                as u16),
                                        );
                                    }
                                }
                            }
                            i = (i).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn QuantizePalette_GrayscaleSmall() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            j = 0u8;
            'l1: loop {
                if !(((j) as i32)
                    < ((((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut pixelRow: *mut u16 = (((&raw mut gCanvasPixels)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(
                        ((((((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            .wrapping_add(((j) as i32)))
                        .wrapping_mul(
                            ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32),
                        )) as isize,
                    );
                    let mut pixel: *mut u16 = (pixelRow).wrapping_offset(
                        ((((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    );
                    {
                        i = 0u8;
                        'l3: loop {
                            if !(((i) as i32)
                                < ((((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>()).read())
                                    as i32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                if ((((pixel).read()) as i32) & 32768i32) != 0 {
                                    (pixel).write(
                                        ((&raw mut gCanvasPaletteStart).cast::<u8>().cast::<u16>())
                                            .read(),
                                    );
                                } else {
                                    (pixel).write(
                                        ((((QuantizePixel_GrayscaleSmall(pixel)) as i32)
                                            .wrapping_add(
                                                ((((&raw mut gCanvasPaletteStart)
                                                    .cast::<u8>()
                                                    .cast::<u16>())
                                                .read())
                                                    as i32),
                                            )) as u16),
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn QuantizePalette_Grayscale() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            j = 0u8;
            'l1: loop {
                if !(((j) as i32)
                    < ((((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut pixelRow: *mut u16 = (((&raw mut gCanvasPixels)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(
                        ((((((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            .wrapping_add(((j) as i32)))
                        .wrapping_mul(
                            ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32),
                        )) as isize,
                    );
                    let mut pixel: *mut u16 = (pixelRow).wrapping_offset(
                        ((((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    );
                    {
                        i = 0u8;
                        'l3: loop {
                            if !(((i) as i32)
                                < ((((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>()).read())
                                    as i32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                if ((((pixel).read()) as i32) & 32768i32) != 0 {
                                    (pixel).write(
                                        ((&raw mut gCanvasPaletteStart).cast::<u8>().cast::<u16>())
                                            .read(),
                                    );
                                } else {
                                    (pixel).write(
                                        ((((QuantizePixel_Grayscale(pixel)) as i32).wrapping_add(
                                            ((((&raw mut gCanvasPaletteStart)
                                                .cast::<u8>()
                                                .cast::<u16>())
                                            .read())
                                                as i32),
                                        )) as u16),
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn QuantizePalette_PrimaryColors() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            j = 0u8;
            'l1: loop {
                if !(((j) as i32)
                    < ((((&raw mut gCanvasRowEnd).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut pixelRow: *mut u16 = (((&raw mut gCanvasPixels)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(
                        ((((((&raw mut gCanvasRowStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            .wrapping_add(((j) as i32)))
                        .wrapping_mul(
                            ((((&raw mut gCanvasWidth).cast::<u8>().cast::<u8>()).read()) as i32),
                        )) as isize,
                    );
                    let mut pixel: *mut u16 = (pixelRow).wrapping_offset(
                        ((((&raw mut gCanvasColumnStart).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    );
                    {
                        i = 0u8;
                        'l3: loop {
                            if !(((i) as i32)
                                < ((((&raw mut gCanvasColumnEnd).cast::<u8>().cast::<u8>()).read())
                                    as i32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                if ((((pixel).read()) as i32) & 32768i32) != 0 {
                                    (pixel).write(
                                        ((&raw mut gCanvasPaletteStart).cast::<u8>().cast::<u16>())
                                            .read(),
                                    );
                                } else {
                                    (pixel).write(
                                        ((((QuantizePixel_PrimaryColors(pixel)) as i32)
                                            .wrapping_add(
                                                ((((&raw mut gCanvasPaletteStart)
                                                    .cast::<u8>()
                                                    .cast::<u16>())
                                                .read())
                                                    as i32),
                                            )) as u16),
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                            pixel = (pixel).wrapping_offset(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn QuantizePixel_Standard(pixel: *mut u16) -> u16 {
    unsafe {
        let mut pixel = pixel;
        let mut red: u16 = (((((pixel).read()) as i32) & 31i32) as u16);
        let mut green: u16 = ((((((pixel).read()) as i32) >> 5) & 31i32) as u16);
        let mut blue: u16 = ((((((pixel).read()) as i32) >> 10) & 31i32) as u16);
        if (((red) as i32) & 3i32) != 0 {
            red = (((((red) as i32) & 28i32).wrapping_add(4i32)) as u16);
        }
        if (((green) as i32) & 3i32) != 0 {
            green = (((((green) as i32) & 28i32).wrapping_add(4i32)) as u16);
        }
        if (((blue) as i32) & 3i32) != 0 {
            blue = (((((blue) as i32) & 28i32).wrapping_add(4i32)) as u16);
        }
        if ((red) as i32) < 6i32 {
            red = 6u16;
        }
        if ((red) as i32) > 30i32 {
            red = 30u16;
        }
        if ((green) as i32) < 6i32 {
            green = 6u16;
        }
        if ((green) as i32) > 30i32 {
            green = 30u16;
        }
        if ((blue) as i32) < 6i32 {
            blue = 6u16;
        }
        if ((blue) as i32) > 30i32 {
            blue = 30u16;
        }
        return ((((((blue) as i32) << 10) | (((green) as i32) << 5)) | ((red) as i32)) as u16);
    }
}
pub(crate) unsafe extern "C" fn QuantizePixel_PrimaryColors(color: *mut u16) -> u16 {
    unsafe {
        let mut color = color;
        let mut red: u16 = (((((color).read()) as i32) & 31i32) as u16);
        let mut green: u16 = ((((((color).read()) as i32) >> 5) & 31i32) as u16);
        let mut blue: u16 = ((((((color).read()) as i32) >> 10) & 31i32) as u16);
        if ((((red) as i32) < 12i32) && (((green) as i32) < 11i32)) && (((blue) as i32) < 11i32) {
            return 1u16;
        }
        if ((((red) as i32) > 19i32) && (((green) as i32) > 19i32)) && (((blue) as i32) > 19i32) {
            return 2u16;
        }
        if ((red) as i32) > 19i32 {
            if ((green) as i32) > 19i32 {
                if ((blue) as i32) > 14i32 {
                    return 2u16;
                } else {
                    return 7u16;
                }
            } else {
                if ((blue) as i32) > 19i32 {
                    if ((green) as i32) > 14i32 {
                        return 2u16;
                    } else {
                        return 8u16;
                    }
                }
            }
        }
        if (((green) as i32) > 19i32) && (((blue) as i32) > 19i32) {
            if ((red) as i32) > 14i32 {
                return 2u16;
            } else {
                return 9u16;
            }
        }
        if ((red) as i32) > 19i32 {
            if ((green) as i32) > 11i32 {
                if ((blue) as i32) > 11i32 {
                    if ((green) as i32) < ((blue) as i32) {
                        return 8u16;
                    } else {
                        return 7u16;
                    }
                } else {
                    return 10u16;
                }
            } else {
                if ((blue) as i32) > 11i32 {
                    return 13u16;
                } else {
                    return 4u16;
                }
            }
        }
        if ((green) as i32) > 19i32 {
            if ((red) as i32) > 11i32 {
                if ((blue) as i32) > 11i32 {
                    if ((red) as i32) < ((blue) as i32) {
                        return 9u16;
                    } else {
                        return 7u16;
                    }
                } else {
                    return 11u16;
                }
            } else {
                if ((blue) as i32) > 11i32 {
                    return 14u16;
                } else {
                    return 5u16;
                }
            }
        }
        if ((blue) as i32) > 19i32 {
            if ((red) as i32) > 11i32 {
                if ((green) as i32) > 11i32 {
                    if ((red) as i32) < ((green) as i32) {
                        return 9u16;
                    } else {
                        return 8u16;
                    }
                }
            } else {
                if ((green) as i32) > 11i32 {
                    return 12u16;
                }
            }
            if ((blue) as i32) > 11i32 {
                return 15u16;
            } else {
                return 6u16;
            }
        }
        return 3u16;
    }
}
pub(crate) unsafe extern "C" fn QuantizePixel_GrayscaleSmall(color: *mut u16) -> u16 {
    unsafe {
        let mut color = color;
        let mut red: u16 = (((((color).read()) as i32) & 31i32) as u16);
        let mut green: u16 = ((((((color).read()) as i32) >> 5) & 31i32) as u16);
        let mut blue: u16 = ((((((color).read()) as i32) >> 10) & 31i32) as u16);
        let mut average: u16 = ((crate::c::div_i32(
            (((red) as i32).wrapping_add(((green) as i32))).wrapping_add(((blue) as i32)),
            3i32,
        ) & 30i32) as u16);
        if ((average) as i32) == 0i32 {
            return 1u16;
        } else {
            return ((crate::c::div_i32(((average) as i32), 2i32)) as u16);
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn QuantizePixel_Grayscale(color: *mut u16) -> u16 {
    unsafe {
        let mut color = color;
        let mut red: u16 = (((((color).read()) as i32) & 31i32) as u16);
        let mut green: u16 = ((((((color).read()) as i32) >> 5) & 31i32) as u16);
        let mut blue: u16 = ((((((color).read()) as i32) >> 10) & 31i32) as u16);
        let mut average: u16 = ((crate::c::div_i32(
            (((red) as i32).wrapping_add(((green) as i32))).wrapping_add(((blue) as i32)),
            3i32,
        )) as u16);
        return ((((average) as i32).wrapping_add(1i32)) as u16);
    }
}
