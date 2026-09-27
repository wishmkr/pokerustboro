//! Translated from `src/palette.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sDummyPaletteStructTemplate sRoundedDownGrayscaleMap
#[allow(unused_imports)]
use crate::data::palette::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlttBufferUnfaded: crate::ffi::Align4<[u8; 1024]> = crate::ffi::Align4([0; 1024]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlttBufferFaded: crate::ffi::Align4<[u8; 1024]> = crate::ffi::Align4([0; 1024]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPaletteStructs: crate::ffi::Align4<[u8; 192]> = crate::ffi::Align4([0; 192]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPaletteFade: crate::ffi::Align4<[u8; 12]> = crate::ffi::Align4([0; 12]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFiller: u32 = 0u32;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPlttBufferTransferPending: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPaletteDecompressionBuffer: crate::ffi::Align4<[u8; 1024]> =
    crate::ffi::Align4([0; 1024]);

unsafe extern "C" {
    static mut gTasks: u8;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetWordTaskArg(a0: u8, a1: u8) -> u32;
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetWordTaskArg(a0: u8, a1: u8, a2: u32);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadCompressedPalette(src: *mut u32, offset: u16, size: u16) {
    unsafe {
        let mut src = src;
        let mut offset = offset;
        let mut size = size;
        LZDecompressWram(
            src,
            ((&raw mut gPaletteDecompressionBuffer).cast::<u8>()).cast::<u8>(),
        );
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            ((&raw mut gPaletteDecompressionBuffer).cast::<u8>()).cast::<u8>(),
                            ((((&raw mut gPlttBufferUnfaded).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(((offset) as i32) as isize))
                            .cast::<u8>(),
                            ((0i32
                                | (crate::c::div_i32(
                                    ((size) as i32),
                                    crate::c::div_i32(16i32, 8i32),
                                ) & 2097151i32)) as u32),
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
        'l5: loop {
            'l6: {
                'l7: loop {
                    'l8: {
                        CpuSet(
                            ((&raw mut gPaletteDecompressionBuffer).cast::<u8>()).cast::<u8>(),
                            ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(((offset) as i32) as isize))
                            .cast::<u8>(),
                            ((0i32
                                | (crate::c::div_i32(
                                    ((size) as i32),
                                    crate::c::div_i32(16i32, 8i32),
                                ) & 2097151i32)) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l7;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadPalette(src: *mut u8, offset: u16, size: u16) {
    unsafe {
        let mut src = src;
        let mut offset = offset;
        let mut size = size;
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            src,
                            ((((&raw mut gPlttBufferUnfaded).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(((offset) as i32) as isize))
                            .cast::<u8>(),
                            ((0i32
                                | (crate::c::div_i32(
                                    ((size) as i32),
                                    crate::c::div_i32(16i32, 8i32),
                                ) & 2097151i32)) as u32),
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
        'l5: loop {
            'l6: {
                'l7: loop {
                    'l8: {
                        CpuSet(
                            src,
                            ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(((offset) as i32) as isize))
                            .cast::<u8>(),
                            ((0i32
                                | (crate::c::div_i32(
                                    ((size) as i32),
                                    crate::c::div_i32(16i32, 8i32),
                                ) & 2097151i32)) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l7;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FillPalette(value: u16, offset: u16, size: u16) {
    unsafe {
        let mut value = value;
        let mut offset = offset;
        let mut size = size;
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(value);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((((&raw mut gPlttBufferUnfaded).cast::<u8>().cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset(((offset) as i32) as isize))
                                .cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(
                                        ((size) as i32),
                                        crate::c::div_i32(16i32, 8i32),
                                    ) & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        'l5: loop {
            'l6: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(value);
                    'l7: loop {
                        'l8: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset(((offset) as i32) as isize))
                                .cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(
                                        ((size) as i32),
                                        crate::c::div_i32(16i32, 8i32),
                                    ) & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TransferPlttBuffer() {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            {
                let mut _src: *mut u8 = (((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                    .cast::<u16>())
                .cast::<u8>();
                let mut _dest: *mut u8 = ((83886080i32) as usize as *mut u8);
                let mut _size: u32 = 1024u32;
                'l1: loop {
                    'l2: {
                        'l3: loop {
                            'l4: {
                                {
                                    let mut dmaRegs: *mut u32 =
                                        ((67109076i32) as usize as *mut u32);
                                    crate::c::volatile_write(dmaRegs, ((_src) as usize as u32));
                                    crate::c::volatile_write(
                                        (dmaRegs).wrapping_offset(1),
                                        ((_dest) as usize as u32),
                                    );
                                    crate::c::volatile_write(
                                        (dmaRegs).wrapping_offset(2),
                                        (2147483648u32
                                            | crate::c::div_u32(
                                                _size,
                                                ((crate::c::div_i32(16i32, 8i32)) as u32),
                                            )),
                                    );
                                    let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                }
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
            }
            ((&raw mut sPlttBufferTransferPending)
                .cast::<u8>()
                .cast::<u32>())
            .write(0u32);
            if (((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
                0,
                2,
                false,
            ) as u16) as i32)
                == 2i32)
                && ((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
            {
                UpdateBlendRegisters();
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdatePaletteFade() -> u8 {
    unsafe {
        let mut result: u8 = 0u8;
        let mut dummy: u8 = 0u8;
        if (((&raw mut sPlttBufferTransferPending)
            .cast::<u8>()
            .cast::<u32>())
        .read())
            != 0
        {
            return 255u8;
        }
        if ((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
            0,
            2,
            false,
        ) as u16) as i32)
            == 0i32
        {
            result = UpdateNormalPaletteFade();
        } else {
            if ((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
                0,
                2,
                false,
            ) as u16) as i32)
                == 1i32
            {
                result = UpdateFastPaletteFade();
            } else {
                result = UpdateHardwarePaletteFade();
            }
        }
        ((&raw mut sPlttBufferTransferPending)
            .cast::<u8>()
            .cast::<u32>())
        .write(((((&raw mut gPaletteFade).cast::<u8>()).cast::<u32>()).read() | ((dummy) as u32)));
        return result;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetPaletteFade() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    PaletteStruct_Reset(i);
                }
                i = (i).wrapping_add(1);
            }
        }
        ResetPaletteFadeControl();
    }
}
pub(crate) unsafe extern "C" fn ReadPlttIntoBuffers() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut pltt: *mut u16 = ((83886080i32) as usize as *mut u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(1024u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gPlttBufferUnfaded).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(((pltt).wrapping_offset(((i) as i32) as isize)).read());
                    ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(((pltt).wrapping_offset(((i) as i32) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BeginNormalPaletteFade(
    selectedPalettes: u32,
    delay: i8,
    startY: u8,
    targetY: u8,
    blendColor: u16,
) -> u8 {
    unsafe {
        let mut selectedPalettes = selectedPalettes;
        let mut delay = delay;
        let mut startY = startY;
        let mut targetY = targetY;
        let mut blendColor = blendColor;
        let mut temp: u8 = 0u8;
        let mut color: u16 = blendColor;
        if (crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0
        {
            return 0u8;
        } else {
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(11),
                0,
                4,
                (2u8) as i32,
            );
            if ((delay) as i32) < 0i32 {
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(11),
                    0,
                    4,
                    ((((crate::c::bf_read(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(11),
                        0,
                        4,
                        false,
                    ) as u8) as i32)
                        .wrapping_add(((delay) as i32).wrapping_mul((-1i32))))
                        as u8) as i32,
                );
                delay = 0i8;
            }
            (((&raw mut gPaletteFade).cast::<u8>()).cast::<u32>()).write(selectedPalettes);
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                0,
                6,
                ((delay) as u8) as i32,
            );
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                0,
                6,
                ((delay) as u16) as i32,
            );
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                6,
                5,
                ((startY) as u16) as i32,
            );
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(5),
                3,
                5,
                ((targetY) as u16) as i32,
            );
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(6),
                0,
                15,
                (color) as i32,
            );
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                (1u16) as i32,
            );
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
                0,
                2,
                (0u16) as i32,
            );
            if ((startY) as i32) < ((targetY) as i32) {
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    6,
                    1,
                    (0u16) as i32,
                );
            } else {
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    6,
                    1,
                    (1u16) as i32,
                );
            }
            UpdatePaletteFade();
            temp = ((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                7,
                1,
                false,
            ) as u16) as u8);
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                7,
                1,
                (0u16) as i32,
            );
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                    .cast::<u16>())
                                .cast::<u8>(),
                                ((83886080i32) as usize as *mut u8),
                                ((67108864i32
                                    | (crate::c::div_i32(1024i32, crate::c::div_i32(32i32, 8i32))
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
            ((&raw mut sPlttBufferTransferPending)
                .cast::<u8>()
                .cast::<u32>())
            .write(0u32);
            if (((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
                0,
                2,
                false,
            ) as u16) as i32)
                == 2i32)
                && ((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
            {
                UpdateBlendRegisters();
            }
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                7,
                1,
                ((temp) as u16) as i32,
            );
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn BeginPlttFade(
    selectedPalettes: u32,
    delay: u8,
    startY: u8,
    targetY: u8,
    blendColor: u16,
) -> u8 {
    unsafe {
        let mut selectedPalettes = selectedPalettes;
        let mut delay = delay;
        let mut startY = startY;
        let mut targetY = targetY;
        let mut blendColor = blendColor;
        ReadPlttIntoBuffers();
        return BeginNormalPaletteFade(
            selectedPalettes,
            ((delay) as i8),
            startY,
            targetY,
            blendColor,
        );
    }
}
pub(crate) unsafe extern "C" fn PaletteStruct_Run(a1: u8, unkFlags: *mut u32) {
    unsafe {
        let mut a1 = a1;
        let mut unkFlags = unkFlags;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    let mut palstruct: *mut u8 = (((&raw mut sPaletteStructs).cast::<u8>())
                        .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 12);
                    if (crate::c::bf_read((palstruct).wrapping_add(4), 0, 1, false) as u32) != 0 {
                        if ((crate::c::bf_read(
                            (((palstruct).cast::<*mut u8>()).read()).wrapping_add(8),
                            0,
                            1,
                            false,
                        ) as u16) as i32)
                            == ((a1) as i32)
                        {
                            if (crate::c::bf_read((palstruct).wrapping_add(6), 5, 7, false) as u32)
                                == ((crate::c::bf_read(
                                    (((palstruct).cast::<*mut u8>()).read()).wrapping_add(11),
                                    0,
                                    5,
                                    false,
                                ) as u8) as u32)
                            {
                                PaletteStruct_TryEnd(palstruct);
                                if !((crate::c::bf_read((palstruct).wrapping_add(4), 0, 1, false)
                                    as u32)
                                    != 0)
                                {
                                    break 'l2;
                                }
                            }
                            if ((((palstruct).wrapping_add(8)).read()) as i32) == 0i32 {
                                PaletteStruct_Copy(palstruct, unkFlags);
                            } else {
                                let __p1 = (palstruct).wrapping_add(8);
                                (__p1).write(((__p1).read()).wrapping_sub(1));
                            }
                            PaletteStruct_Blend(palstruct, unkFlags);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PaletteStruct_Copy(palStruct: *mut u8, unkFlags: *mut u32) {
    unsafe {
        let mut palStruct = palStruct;
        let mut unkFlags = unkFlags;
        let mut srcIndex: i32 = 0i32;
        let mut srcCount: i32 = 0i32;
        let mut i: u8 = 0u8;
        let mut srcOffset: u16 =
            (((crate::c::bf_read((palStruct).wrapping_add(6), 5, 7, false) as u32).wrapping_mul(
                ((crate::c::bf_read(
                    (((palStruct).cast::<*mut u8>()).read()).wrapping_add(9),
                    2,
                    5,
                    false,
                ) as u16) as u32),
            )) as u16);
        if !((crate::c::bf_read(
            (((palStruct).cast::<*mut u8>()).read()).wrapping_add(8),
            0,
            1,
            false,
        ) as u16)
            != 0)
        {
            'l1: loop {
                if !(((i) as i32)
                    < ((crate::c::bf_read(
                        (((palStruct).cast::<*mut u8>()).read()).wrapping_add(9),
                        2,
                        5,
                        false,
                    ) as u16) as i32))
                {
                    break 'l1;
                }
                ((((&raw mut gPlttBufferUnfaded).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((crate::c::bf_read((palStruct).wrapping_add(5), 3, 10, false) as u32)
                            as i32) as isize,
                    ))
                .write(
                    ((((((palStruct).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(((srcOffset) as i32) as isize))
                    .read(),
                );
                ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((crate::c::bf_read((palStruct).wrapping_add(5), 3, 10, false) as u32)
                            as i32) as isize,
                    ))
                .write(
                    ((((((palStruct).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(((srcOffset) as i32) as isize))
                    .read(),
                );
                i = (i).wrapping_add(1);
                crate::c::bf_write(
                    (palStruct).wrapping_add(5),
                    3,
                    10,
                    ((crate::c::bf_read((palStruct).wrapping_add(5), 3, 10, false) as u32)
                        .wrapping_add(1)) as i32,
                );
                srcOffset = (srcOffset).wrapping_add(1);
            }
        } else {
            'l2: loop {
                if !(((i) as i32)
                    < ((crate::c::bf_read(
                        (((palStruct).cast::<*mut u8>()).read()).wrapping_add(9),
                        2,
                        5,
                        false,
                    ) as u16) as i32))
                {
                    break 'l2;
                }
                ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((crate::c::bf_read((palStruct).wrapping_add(5), 3, 10, false) as u32)
                            as i32) as isize,
                    ))
                .write(
                    ((((((palStruct).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(((srcOffset) as i32) as isize))
                    .read(),
                );
                i = (i).wrapping_add(1);
                crate::c::bf_write(
                    (palStruct).wrapping_add(5),
                    3,
                    10,
                    ((crate::c::bf_read((palStruct).wrapping_add(5), 3, 10, false) as u32)
                        .wrapping_add(1)) as i32,
                );
                srcOffset = (srcOffset).wrapping_add(1);
            }
        }
        crate::c::bf_write(
            (palStruct).wrapping_add(5),
            3,
            10,
            (crate::c::bf_read((palStruct).wrapping_add(4), 2, 9, false) as u32) as i32,
        );
        ((palStruct).wrapping_add(8))
            .write(((((palStruct).cast::<*mut u8>()).read()).wrapping_add(10)).read());
        crate::c::bf_write(
            (palStruct).wrapping_add(6),
            5,
            7,
            ((crate::c::bf_read((palStruct).wrapping_add(6), 5, 7, false) as u32).wrapping_add(1))
                as i32,
        );
        srcIndex = ((crate::c::bf_read((palStruct).wrapping_add(6), 5, 7, false) as u32) as i32);
        srcCount = ((crate::c::bf_read(
            (((palStruct).cast::<*mut u8>()).read()).wrapping_add(11),
            0,
            5,
            false,
        ) as u8) as i32);
        if srcIndex >= srcCount {
            if (((palStruct).wrapping_add(9)).read()) != 0 {
                let __p1 = (palStruct).wrapping_add(9);
                (__p1).write(((__p1).read()).wrapping_sub(1));
            }
            crate::c::bf_write((palStruct).wrapping_add(6), 5, 7, (0u32) as i32);
        }
        (unkFlags).write(
            ((unkFlags).read()
                | ((crate::c::shl_i32(
                    1i32,
                    ((crate::c::bf_read((palStruct).wrapping_add(4), 2, 9, false) as u32) >> 4),
                )) as u32)),
        );
    }
}
pub(crate) unsafe extern "C" fn PaletteStruct_Blend(palStruct: *mut u8, unkFlags: *mut u32) {
    unsafe {
        let mut palStruct = palStruct;
        let mut unkFlags = unkFlags;
        if ((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
            && ((((crate::c::shl_i32(
                1i32,
                ((crate::c::bf_read((palStruct).wrapping_add(4), 2, 9, false) as u32) >> 4),
            )) as u32)
                & (((&raw mut gPaletteFade).cast::<u8>()).cast::<u32>()).read())
                != 0)
        {
            if !((crate::c::bf_read(
                (((palStruct).cast::<*mut u8>()).read()).wrapping_add(8),
                0,
                1,
                false,
            ) as u16)
                != 0)
            {
                if ((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                    0,
                    6,
                    false,
                ) as u8) as i32)
                    != ((crate::c::bf_read(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                        0,
                        6,
                        false,
                    ) as u16) as i32)
                {
                    BlendPalette(
                        ((crate::c::bf_read((palStruct).wrapping_add(4), 2, 9, false) as u32)
                            as u16),
                        (crate::c::bf_read(
                            (((palStruct).cast::<*mut u8>()).read()).wrapping_add(9),
                            2,
                            5,
                            false,
                        ) as u16),
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
                }
            } else {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                    0,
                    6,
                    false,
                ) as u8)
                    != 0)
                {
                    if ((((palStruct).wrapping_add(8)).read()) as i32)
                        != ((((((palStruct).cast::<*mut u8>()).read()).wrapping_add(10)).read())
                            as i32)
                    {
                        let mut srcOffset: u32 =
                            (crate::c::bf_read((palStruct).wrapping_add(6), 5, 7, false) as u32)
                                .wrapping_mul(
                                    ((crate::c::bf_read(
                                        (((palStruct).cast::<*mut u8>()).read()).wrapping_add(9),
                                        2,
                                        5,
                                        false,
                                    ) as u16) as u32),
                                );
                        let mut i: u8 = 0u8;
                        {
                            i = 0u8;
                            'l1: loop {
                                if !(((i) as i32)
                                    < ((crate::c::bf_read(
                                        (((palStruct).cast::<*mut u8>()).read()).wrapping_add(9),
                                        2,
                                        5,
                                        false,
                                    ) as u16) as i32))
                                {
                                    break 'l1;
                                }
                                'l2: {
                                    ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        (((crate::c::bf_read(
                                            (palStruct).wrapping_add(4),
                                            2,
                                            9,
                                            false,
                                        ) as u32)
                                            .wrapping_add(((i) as u32)))
                                            as i32)
                                            as isize,
                                    ))
                                    .write(
                                        ((((((palStruct).cast::<*mut u8>()).read())
                                            .wrapping_add(4)
                                            .cast::<*mut u16>())
                                        .read())
                                        .wrapping_offset(
                                            (((srcOffset).wrapping_add(((i) as u32))) as i32)
                                                as isize,
                                        ))
                                        .read(),
                                    );
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PaletteStruct_TryEnd(pal: *mut u8) {
    unsafe {
        let mut pal = pal;
        if ((((pal).wrapping_add(9)).read()) as i32) == 0i32 {
            let mut state: i32 = ((crate::c::bf_read(
                (((pal).cast::<*mut u8>()).read()).wrapping_add(11),
                5,
                3,
                false,
            ) as u8) as i32);
            if state == 0i32 {
                crate::c::bf_write((pal).wrapping_add(6), 5, 7, (0u32) as i32);
                ((pal).wrapping_add(8))
                    .write(((((pal).cast::<*mut u8>()).read()).wrapping_add(10)).read());
                ((pal).wrapping_add(9))
                    .write(((((pal).cast::<*mut u8>()).read()).wrapping_add(12)).read());
                crate::c::bf_write(
                    (pal).wrapping_add(5),
                    3,
                    10,
                    (crate::c::bf_read((pal).wrapping_add(4), 2, 9, false) as u32) as i32,
                );
            } else {
                if state < 0i32 {
                    return;
                }
                if state > 2i32 {
                    return;
                }
                PaletteStruct_ResetById(((((pal).cast::<*mut u8>()).read()).cast::<u16>()).read());
            }
        } else {
            let __p1 = (pal).wrapping_add(9);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PaletteStruct_ResetById(id: u16) {
    unsafe {
        let mut id = id;
        let mut paletteNum: u8 = PaletteStruct_GetPalNum(id);
        if ((paletteNum) as i32) != 16i32 {
            PaletteStruct_Reset(paletteNum);
        }
    }
}
pub(crate) unsafe extern "C" fn PaletteStruct_Reset(paletteNum: u8) {
    unsafe {
        let mut paletteNum = paletteNum;
        (((((&raw mut sPaletteStructs).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((paletteNum) as i32) as isize * 12))
        .cast::<*mut u8>())
        .write(
            (&raw const sDummyPaletteStructTemplate)
                .cast::<u8>()
                .cast_mut(),
        );
        crate::c::bf_write(
            ((((&raw mut sPaletteStructs).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((paletteNum) as i32) as isize * 12))
            .wrapping_add(4),
            0,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut sPaletteStructs).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((paletteNum) as i32) as isize * 12))
            .wrapping_add(4),
            2,
            9,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut sPaletteStructs).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((paletteNum) as i32) as isize * 12))
            .wrapping_add(5),
            3,
            10,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut sPaletteStructs).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((paletteNum) as i32) as isize * 12))
            .wrapping_add(6),
            5,
            7,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut sPaletteStructs).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((paletteNum) as i32) as isize * 12))
            .wrapping_add(4),
            1,
            1,
            (0u32) as i32,
        );
        (((((&raw mut sPaletteStructs).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((paletteNum) as i32) as isize * 12))
        .wrapping_add(8))
        .write(0u8);
        (((((&raw mut sPaletteStructs).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((paletteNum) as i32) as isize * 12))
        .wrapping_add(9))
        .write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetPaletteFadeControl() {
    unsafe {
        (((&raw mut gPaletteFade).cast::<u8>()).cast::<u32>()).write(0u32);
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
            0,
            6,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
            0,
            6,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
            6,
            5,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(5),
            3,
            5,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(6),
            0,
            15,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
            0,
            6,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
            6,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
            7,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
            2,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
            3,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
            5,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
            0,
            5,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
            6,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(11),
            0,
            4,
            (2u8) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn PaletteStruct_SetUnusedFlag(id: u16) {
    unsafe {
        let mut id = id;
        let mut paletteNum: u8 = PaletteStruct_GetPalNum(id);
        if ((paletteNum) as i32) != 16i32 {
            crate::c::bf_write(
                ((((&raw mut sPaletteStructs).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((paletteNum) as i32) as isize * 12))
                .wrapping_add(4),
                1,
                1,
                (1u32) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PaletteStruct_ClearUnusedFlag(id: u16) {
    unsafe {
        let mut id = id;
        let mut paletteNum: u8 = PaletteStruct_GetPalNum(id);
        if ((paletteNum) as i32) != 16i32 {
            crate::c::bf_write(
                ((((&raw mut sPaletteStructs).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((paletteNum) as i32) as isize * 12))
                .wrapping_add(4),
                1,
                1,
                (0u32) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PaletteStruct_GetPalNum(id: u16) -> u8 {
    unsafe {
        let mut id = id;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut sPaletteStructs).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<*mut u8>())
                    .read())
                    .cast::<u16>())
                    .read()) as i32)
                        == ((id) as i32)
                    {
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 16u8;
    }
}
pub(crate) unsafe extern "C" fn UpdateNormalPaletteFade() -> u8 {
    unsafe {
        let mut paletteOffset: u16 = 0u16;
        let mut selectedPalettes: u16 = 0u16;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            return 0u8;
        }
        if (IsSoftwarePaletteFadeFinishing()) != 0 {
            return ((if (crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0
            {
                1i32
            } else {
                0i32
            }) as u8);
        } else {
            if !((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
                6,
                1,
                false,
            ) as u16)
                != 0)
            {
                if ((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                    0,
                    6,
                    false,
                ) as u8) as i32)
                    < ((crate::c::bf_read(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                        0,
                        6,
                        false,
                    ) as u16) as i32)
                {
                    crate::c::bf_write(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                        0,
                        6,
                        ((crate::c::bf_read(
                            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                            0,
                            6,
                            false,
                        ) as u8)
                            .wrapping_add(1)) as i32,
                    );
                    return 2u8;
                }
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                    0,
                    6,
                    (0u8) as i32,
                );
            }
            paletteOffset = 0u16;
            if !((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
                6,
                1,
                false,
            ) as u16)
                != 0)
            {
                selectedPalettes =
                    (((((&raw mut gPaletteFade).cast::<u8>()).cast::<u32>()).read()) as u16);
            } else {
                selectedPalettes =
                    (((((&raw mut gPaletteFade).cast::<u8>()).cast::<u32>()).read() >> 16) as u16);
                paletteOffset = 256u16;
            }
            'l1: loop {
                if !((selectedPalettes) != 0) {
                    break 'l1;
                }
                if (((selectedPalettes) as i32) & 1i32) != 0 {
                    BlendPalette(
                        paletteOffset,
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
                }
                selectedPalettes = ((((selectedPalettes) as i32) >> 1) as u16);
                paletteOffset = ((((paletteOffset) as i32).wrapping_add(16i32)) as u16);
            }
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
                6,
                1,
                ((((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
                    6,
                    1,
                    false,
                ) as u16) as i32)
                    ^ 1i32) as u16) as i32,
            );
            if !((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
                6,
                1,
                false,
            ) as u16)
                != 0)
            {
                if ((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                    6,
                    5,
                    false,
                ) as u16) as i32)
                    == ((crate::c::bf_read(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(5),
                        3,
                        5,
                        false,
                    ) as u16) as i32)
                {
                    (((&raw mut gPaletteFade).cast::<u8>()).cast::<u32>()).write(0u32);
                    crate::c::bf_write(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
                        5,
                        1,
                        (1u16) as i32,
                    );
                } else {
                    let mut val: i8 = 0i8;
                    if !((crate::c::bf_read(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                        6,
                        1,
                        false,
                    ) as u16)
                        != 0)
                    {
                        val = ((crate::c::bf_read(
                            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                            6,
                            5,
                            false,
                        ) as u16) as i8);
                        val = ((((val) as i32).wrapping_add(
                            ((crate::c::bf_read(
                                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(11),
                                0,
                                4,
                                false,
                            ) as u8) as i32),
                        )) as i8);
                        if ((val) as i32)
                            > ((crate::c::bf_read(
                                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(5),
                                3,
                                5,
                                false,
                            ) as u16) as i32)
                        {
                            val = ((crate::c::bf_read(
                                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(5),
                                3,
                                5,
                                false,
                            ) as u16) as i8);
                        }
                        crate::c::bf_write(
                            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                            6,
                            5,
                            ((val) as u16) as i32,
                        );
                    } else {
                        val = ((crate::c::bf_read(
                            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                            6,
                            5,
                            false,
                        ) as u16) as i8);
                        val = ((((val) as i32).wrapping_sub(
                            ((crate::c::bf_read(
                                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(11),
                                0,
                                4,
                                false,
                            ) as u8) as i32),
                        )) as i8);
                        if ((val) as i32)
                            < ((crate::c::bf_read(
                                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(5),
                                3,
                                5,
                                false,
                            ) as u16) as i32)
                        {
                            val = ((crate::c::bf_read(
                                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(5),
                                3,
                                5,
                                false,
                            ) as u16) as i8);
                        }
                        crate::c::bf_write(
                            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                            6,
                            5,
                            ((val) as u16) as i32,
                        );
                    }
                }
            }
            return ((if (crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0
            {
                1i32
            } else {
                0i32
            }) as u8);
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InvertPlttBuffer(selectedPalettes: u32) {
    unsafe {
        let mut selectedPalettes = selectedPalettes;
        let mut paletteOffset: u16 = 0u16;
        'l1: loop {
            if !((selectedPalettes) != 0) {
                break 'l1;
            }
            if (selectedPalettes & 1u32) != 0 {
                let mut i: u8 = 0u8;
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < 16i32) {
                            break 'l2;
                        }
                        'l3: {
                            ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(
                                (((paletteOffset) as i32).wrapping_add(((i) as i32))) as isize,
                            ))
                            .write(
                                ((!((((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset(
                                    (((paletteOffset) as i32).wrapping_add(((i) as i32))) as isize,
                                ))
                                .read()) as i32)) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            selectedPalettes = (selectedPalettes >> 1);
            paletteOffset = ((((paletteOffset) as i32).wrapping_add(16i32)) as u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TintPlttBuffer(selectedPalettes: u32, r: i8, g: i8, b: i8) {
    unsafe {
        let mut selectedPalettes = selectedPalettes;
        let mut r = r;
        let mut g = g;
        let mut b = b;
        let mut paletteOffset: u16 = 0u16;
        'l1: loop {
            if !((selectedPalettes) != 0) {
                break 'l1;
            }
            if (selectedPalettes & 1u32) != 0 {
                let mut i: u8 = 0u8;
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < 16i32) {
                            break 'l2;
                        }
                        'l3: {
                            let mut data: *mut u8 =
                                ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset(
                                    (((paletteOffset) as i32).wrapping_add(((i) as i32))) as isize,
                                ))
                                .cast::<u8>();
                            crate::c::bf_write(
                                (data).wrapping_add(0),
                                0,
                                5,
                                ((((crate::c::bf_read((data).wrapping_add(0), 0, 5, false) as u16)
                                    as i32)
                                    .wrapping_add(((r) as i32)))
                                    as u16) as i32,
                            );
                            crate::c::bf_write(
                                (data).wrapping_add(0),
                                5,
                                5,
                                ((((crate::c::bf_read((data).wrapping_add(0), 5, 5, false) as u16)
                                    as i32)
                                    .wrapping_add(((g) as i32)))
                                    as u16) as i32,
                            );
                            crate::c::bf_write(
                                (data).wrapping_add(1),
                                2,
                                5,
                                ((((crate::c::bf_read((data).wrapping_add(1), 2, 5, false) as u16)
                                    as i32)
                                    .wrapping_add(((b) as i32)))
                                    as u16) as i32,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            selectedPalettes = (selectedPalettes >> 1);
            paletteOffset = ((((paletteOffset) as i32).wrapping_add(16i32)) as u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UnfadePlttBuffer(selectedPalettes: u32) {
    unsafe {
        let mut selectedPalettes = selectedPalettes;
        let mut paletteOffset: u16 = 0u16;
        'l1: loop {
            if !((selectedPalettes) != 0) {
                break 'l1;
            }
            if (selectedPalettes & 1u32) != 0 {
                let mut i: u8 = 0u8;
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < 16i32) {
                            break 'l2;
                        }
                        'l3: {
                            ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(
                                (((paletteOffset) as i32).wrapping_add(((i) as i32))) as isize,
                            ))
                            .write(
                                ((((&raw mut gPlttBufferUnfaded).cast::<u8>().cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset(
                                    (((paletteOffset) as i32).wrapping_add(((i) as i32))) as isize,
                                ))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            selectedPalettes = (selectedPalettes >> 1);
            paletteOffset = ((((paletteOffset) as i32).wrapping_add(16i32)) as u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BeginFastPaletteFade(submode: u8) {
    unsafe {
        let mut submode = submode;
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(11),
            0,
            4,
            (2u8) as i32,
        );
        BeginFastPaletteFadeInternal(submode);
    }
}
pub(crate) unsafe extern "C" fn BeginFastPaletteFadeInternal(submode: u8) {
    unsafe {
        let mut submode = submode;
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
            6,
            5,
            (31u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
            0,
            6,
            ((((submode) as i32) & 63i32) as u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
            0,
            2,
            (1u16) as i32,
        );
        if ((submode) as i32) == 2i32 {
            'l1: loop {
                'l2: {
                    {
                        let mut tmp: u16 = 0u16;
                        (&raw mut tmp).write_volatile(0u16);
                        'l3: loop {
                            'l4: {
                                CpuSet(
                                    (&raw mut tmp).cast::<u8>(),
                                    (((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                        .cast::<u16>())
                                    .cast::<u8>(),
                                    ((16777216i32
                                        | (crate::c::div_i32(
                                            1024i32,
                                            crate::c::div_i32(16i32, 8i32),
                                        ) & 2097151i32))
                                        as u32),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l3;
                            }
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        }
        if ((submode) as i32) == 0i32 {
            'l5: loop {
                'l6: {
                    {
                        let mut tmp: u16 = 0u16;
                        (&raw mut tmp).write_volatile(32767u16);
                        'l7: loop {
                            'l8: {
                                CpuSet(
                                    (&raw mut tmp).cast::<u8>(),
                                    (((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                        .cast::<u16>())
                                    .cast::<u8>(),
                                    ((16777216i32
                                        | (crate::c::div_i32(
                                            1024i32,
                                            crate::c::div_i32(16i32, 8i32),
                                        ) & 2097151i32))
                                        as u32),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l7;
                            }
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l5;
                }
            }
        }
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn UpdateFastPaletteFade() -> u8 {
    unsafe {
        let mut i: u16 = 0u16;
        let mut paletteOffsetStart: u16 = 0u16;
        let mut paletteOffsetEnd: u16 = 0u16;
        let mut r0: i8 = 0i8;
        let mut g0: i8 = 0i8;
        let mut b0: i8 = 0i8;
        let mut r: i8 = 0i8;
        let mut g: i8 = 0i8;
        let mut b: i8 = 0i8;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            return 0u8;
        }
        if (IsSoftwarePaletteFadeFinishing()) != 0 {
            return ((if (crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0
            {
                1i32
            } else {
                0i32
            }) as u8);
        }
        if (crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
            6,
            1,
            false,
        ) as u16)
            != 0
        {
            paletteOffsetStart = 256u16;
            paletteOffsetEnd = ((crate::c::div_u32(1024u32, 2u32)) as u16);
        } else {
            paletteOffsetStart = 0u16;
            paletteOffsetEnd = 256u16;
        }
        'l1: {
            let __sw1 = ((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                0,
                6,
                false,
            ) as u16) as i32);
            if __sw1 == 0i32 {
                {
                    i = paletteOffsetStart;
                    'l2: loop {
                        if !(((i) as i32) < ((paletteOffsetEnd) as i32)) {
                            break 'l2;
                        }
                        'l3: {
                            let mut unfaded: *mut u8 = core::ptr::null_mut();
                            let mut faded: *mut u8 = core::ptr::null_mut();
                            unfaded =
                                ((((&raw mut gPlttBufferUnfaded).cast::<u8>().cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .cast::<u8>();
                            r0 = ((crate::c::bf_read((unfaded).wrapping_add(0), 0, 5, false) as u16)
                                as i8);
                            g0 = ((crate::c::bf_read((unfaded).wrapping_add(0), 5, 5, false) as u16)
                                as i8);
                            b0 = ((crate::c::bf_read((unfaded).wrapping_add(1), 2, 5, false) as u16)
                                as i8);
                            faded = ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .cast::<u8>();
                            r = ((((crate::c::bf_read((faded).wrapping_add(0), 0, 5, false) as u16)
                                as i32)
                                .wrapping_sub(2i32)) as i8);
                            g = ((((crate::c::bf_read((faded).wrapping_add(0), 5, 5, false) as u16)
                                as i32)
                                .wrapping_sub(2i32)) as i8);
                            b = ((((crate::c::bf_read((faded).wrapping_add(1), 2, 5, false) as u16)
                                as i32)
                                .wrapping_sub(2i32)) as i8);
                            if ((r) as i32) < ((r0) as i32) {
                                r = r0;
                            }
                            if ((g) as i32) < ((g0) as i32) {
                                g = g0;
                            }
                            if ((b) as i32) < ((b0) as i32) {
                                b = b0;
                            }
                            ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(
                                (((((r) as i32) | (((g) as i32) << 5)) | (((b) as i32) << 10))
                                    as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                {
                    i = paletteOffsetStart;
                    'l4: loop {
                        if !(((i) as i32) < ((paletteOffsetEnd) as i32)) {
                            break 'l4;
                        }
                        'l5: {
                            let mut data: *mut u8 =
                                ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .cast::<u8>();
                            r = ((((crate::c::bf_read((data).wrapping_add(0), 0, 5, false) as u16)
                                as i32)
                                .wrapping_add(2i32)) as i8);
                            g = ((((crate::c::bf_read((data).wrapping_add(0), 5, 5, false) as u16)
                                as i32)
                                .wrapping_add(2i32)) as i8);
                            b = ((((crate::c::bf_read((data).wrapping_add(1), 2, 5, false) as u16)
                                as i32)
                                .wrapping_add(2i32)) as i8);
                            if ((r) as i32) > 31i32 {
                                r = 31i8;
                            }
                            if ((g) as i32) > 31i32 {
                                g = 31i8;
                            }
                            if ((b) as i32) > 31i32 {
                                b = 31i8;
                            }
                            ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(
                                (((((r) as i32) | (((g) as i32) << 5)) | (((b) as i32) << 10))
                                    as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    i = paletteOffsetStart;
                    'l6: loop {
                        if !(((i) as i32) < ((paletteOffsetEnd) as i32)) {
                            break 'l6;
                        }
                        'l7: {
                            let mut unfaded: *mut u8 = core::ptr::null_mut();
                            let mut faded: *mut u8 = core::ptr::null_mut();
                            unfaded =
                                ((((&raw mut gPlttBufferUnfaded).cast::<u8>().cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .cast::<u8>();
                            r0 = ((crate::c::bf_read((unfaded).wrapping_add(0), 0, 5, false) as u16)
                                as i8);
                            g0 = ((crate::c::bf_read((unfaded).wrapping_add(0), 5, 5, false) as u16)
                                as i8);
                            b0 = ((crate::c::bf_read((unfaded).wrapping_add(1), 2, 5, false) as u16)
                                as i8);
                            faded = ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .cast::<u8>();
                            r = ((((crate::c::bf_read((faded).wrapping_add(0), 0, 5, false) as u16)
                                as i32)
                                .wrapping_add(2i32)) as i8);
                            g = ((((crate::c::bf_read((faded).wrapping_add(0), 5, 5, false) as u16)
                                as i32)
                                .wrapping_add(2i32)) as i8);
                            b = ((((crate::c::bf_read((faded).wrapping_add(1), 2, 5, false) as u16)
                                as i32)
                                .wrapping_add(2i32)) as i8);
                            if ((r) as i32) > ((r0) as i32) {
                                r = r0;
                            }
                            if ((g) as i32) > ((g0) as i32) {
                                g = g0;
                            }
                            if ((b) as i32) > ((b0) as i32) {
                                b = b0;
                            }
                            ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(
                                (((((r) as i32) | (((g) as i32) << 5)) | (((b) as i32) << 10))
                                    as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                {
                    i = paletteOffsetStart;
                    'l8: loop {
                        if !(((i) as i32) < ((paletteOffsetEnd) as i32)) {
                            break 'l8;
                        }
                        'l9: {
                            let mut data: *mut u8 =
                                ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .cast::<u8>();
                            r = ((((crate::c::bf_read((data).wrapping_add(0), 0, 5, false) as u16)
                                as i32)
                                .wrapping_sub(2i32)) as i8);
                            g = ((((crate::c::bf_read((data).wrapping_add(0), 5, 5, false) as u16)
                                as i32)
                                .wrapping_sub(2i32)) as i8);
                            b = ((((crate::c::bf_read((data).wrapping_add(1), 2, 5, false) as u16)
                                as i32)
                                .wrapping_sub(2i32)) as i8);
                            if ((r) as i32) < 0i32 {
                                r = 0i8;
                            }
                            if ((g) as i32) < 0i32 {
                                g = 0i8;
                            }
                            if ((b) as i32) < 0i32 {
                                b = 0i8;
                            }
                            ((((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(
                                (((((r) as i32) | (((g) as i32) << 5)) | (((b) as i32) << 10))
                                    as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
            6,
            1,
            ((((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
                6,
                1,
                false,
            ) as u16) as i32)
                ^ 1i32) as u16) as i32,
        );
        if (crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
            6,
            1,
            false,
        ) as u16)
            != 0
        {
            return ((if (crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0
            {
                1i32
            } else {
                0i32
            }) as u8);
        }
        if ((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
            6,
            5,
            false,
        ) as u16) as i32)
            .wrapping_sub(
                ((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(11),
                    0,
                    4,
                    false,
                ) as u8) as i32),
            )
            < 0i32
        {
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                6,
                5,
                (0u16) as i32,
            );
        } else {
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                6,
                5,
                ((((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                    6,
                    5,
                    false,
                ) as u16) as i32)
                    .wrapping_sub(
                        ((crate::c::bf_read(
                            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(11),
                            0,
                            4,
                            false,
                        ) as u8) as i32),
                    )) as u16) as i32,
            );
        }
        if ((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
            6,
            5,
            false,
        ) as u16) as i32)
            == 0i32
        {
            'l10: {
                let __sw2 = ((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    0,
                    6,
                    false,
                ) as u16) as i32);
                if __sw2 == 0i32 || __sw2 == 2i32 {
                    'l11: loop {
                        'l12: {
                            'l13: loop {
                                'l14: {
                                    CpuSet(
                                        (((&raw mut gPlttBufferUnfaded)
                                            .cast::<u8>()
                                            .cast::<u16>())
                                        .cast::<u16>())
                                        .cast::<u8>(),
                                        (((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                                            .cast::<u16>())
                                        .cast::<u8>(),
                                        ((67108864i32
                                            | (crate::c::div_i32(
                                                1024i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l13;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l11;
                        }
                    }
                    break 'l10;
                }
                if __sw2 == 1i32 {
                    'l15: loop {
                        'l16: {
                            {
                                let mut tmp: u32 = 0u32;
                                (&raw mut tmp).write_volatile(4294967295u32);
                                'l17: loop {
                                    'l18: {
                                        CpuSet(
                                            (&raw mut tmp).cast::<u8>(),
                                            (((&raw mut gPlttBufferFaded)
                                                .cast::<u8>()
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .cast::<u8>(),
                                            ((83886080i32
                                                | (crate::c::div_i32(
                                                    1024i32,
                                                    crate::c::div_i32(32i32, 8i32),
                                                ) & 2097151i32))
                                                as u32),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l17;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l15;
                        }
                    }
                    break 'l10;
                }
                if __sw2 == 3i32 {
                    'l19: loop {
                        'l20: {
                            {
                                let mut tmp: u32 = 0u32;
                                (&raw mut tmp).write_volatile(0u32);
                                'l21: loop {
                                    'l22: {
                                        CpuSet(
                                            (&raw mut tmp).cast::<u8>(),
                                            (((&raw mut gPlttBufferFaded)
                                                .cast::<u8>()
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .cast::<u8>(),
                                            ((83886080i32
                                                | (crate::c::div_i32(
                                                    1024i32,
                                                    crate::c::div_i32(32i32, 8i32),
                                                ) & 2097151i32))
                                                as u32),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l21;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l19;
                        }
                    }
                    break 'l10;
                }
            }
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
                0,
                2,
                (0u16) as i32,
            );
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
                5,
                1,
                (1u16) as i32,
            );
        }
        return ((if (crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0
        {
            1i32
        } else {
            0i32
        }) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BeginHardwarePaletteFade(
    blendCnt: u8,
    delay: u8,
    y: u8,
    targetY: u8,
    shouldResetBlendRegisters: u8,
) {
    unsafe {
        let mut blendCnt = blendCnt;
        let mut delay = delay;
        let mut y = y;
        let mut targetY = targetY;
        let mut shouldResetBlendRegisters = shouldResetBlendRegisters;
        (((&raw mut gPaletteFade).cast::<u8>()).cast::<u32>()).write(((blendCnt) as u32));
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
            0,
            6,
            (delay) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
            0,
            6,
            ((delay) as u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
            6,
            5,
            ((y) as u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(5),
            3,
            5,
            ((targetY) as u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
            0,
            2,
            (2u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
            2,
            1,
            ((((shouldResetBlendRegisters) as i32) & 1i32) as u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
            3,
            1,
            (0u16) as i32,
        );
        if ((y) as i32) < ((targetY) as i32) {
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                6,
                1,
                (0u16) as i32,
            );
        } else {
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                6,
                1,
                (1u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateHardwarePaletteFade() -> u8 {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            return 0u8;
        }
        if ((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
            0,
            6,
            false,
        ) as u8) as i32)
            < ((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                0,
                6,
                false,
            ) as u16) as i32)
        {
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                0,
                6,
                ((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                    0,
                    6,
                    false,
                ) as u8)
                    .wrapping_add(1)) as i32,
            );
            return 2u8;
        }
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
            0,
            6,
            (0u8) as i32,
        );
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
            6,
            1,
            false,
        ) as u16)
            != 0)
        {
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                6,
                5,
                ((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                    6,
                    5,
                    false,
                ) as u16)
                    .wrapping_add(1)) as i32,
            );
            if ((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                6,
                5,
                false,
            ) as u16) as i32)
                > ((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(5),
                    3,
                    5,
                    false,
                ) as u16) as i32)
            {
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
                    3,
                    1,
                    ((crate::c::bf_read(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
                        3,
                        1,
                        false,
                    ) as u16)
                        .wrapping_add(1)) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                    6,
                    5,
                    ((crate::c::bf_read(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                        6,
                        5,
                        false,
                    ) as u16)
                        .wrapping_sub(1)) as i32,
                );
            }
        } else {
            let mut y: i32 = (({
                let __t1 = (crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                    6,
                    5,
                    false,
                ) as u16);
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                    6,
                    5,
                    ((crate::c::bf_read(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                        6,
                        5,
                        false,
                    ) as u16)
                        .wrapping_sub(1)) as i32,
                );
                __t1
            }) as i32);
            if (y).wrapping_sub(1i32)
                < ((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(5),
                    3,
                    5,
                    false,
                ) as u16) as i32)
            {
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
                    3,
                    1,
                    ((crate::c::bf_read(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
                        3,
                        1,
                        false,
                    ) as u16)
                        .wrapping_add(1)) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                    6,
                    5,
                    ((crate::c::bf_read(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                        6,
                        5,
                        false,
                    ) as u16)
                        .wrapping_add(1)) as i32,
                );
            }
        }
        if (crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
            3,
            1,
            false,
        ) as u16)
            != 0
        {
            if (crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
                2,
                1,
                false,
            ) as u16)
                != 0
            {
                (((&raw mut gPaletteFade).cast::<u8>()).cast::<u32>()).write(0u32);
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                    6,
                    5,
                    (0u16) as i32,
                );
            }
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
                2,
                1,
                (0u16) as i32,
            );
        }
        return ((if (crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0
        {
            1i32
        } else {
            0i32
        }) as u8);
    }
}
pub(crate) unsafe extern "C" fn UpdateBlendRegisters() {
    unsafe {
        SetGpuReg(
            80u8,
            (((((&raw mut gPaletteFade).cast::<u8>()).cast::<u32>()).read()) as u16),
        );
        SetGpuReg(
            84u8,
            (crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                6,
                5,
                false,
            ) as u16),
        );
        if (crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
            3,
            1,
            false,
        ) as u16)
            != 0
        {
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
                3,
                1,
                (0u16) as i32,
            );
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(9),
                0,
                2,
                (0u16) as i32,
            );
            (((&raw mut gPaletteFade).cast::<u8>()).cast::<u32>()).write(0u32);
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                6,
                5,
                (0u16) as i32,
            );
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                (0u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn IsSoftwarePaletteFadeFinishing() -> u8 {
    unsafe {
        if (crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
            5,
            1,
            false,
        ) as u16)
            != 0
        {
            if ((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
                0,
                5,
                false,
            ) as u16) as i32)
                == 4i32
            {
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    (0u16) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
                    5,
                    1,
                    (0u16) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
                    0,
                    5,
                    (0u16) as i32,
                );
            } else {
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
                    0,
                    5,
                    ((crate::c::bf_read(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
                        0,
                        5,
                        false,
                    ) as u16)
                        .wrapping_add(1)) as i32,
                );
            }
            return 1u8;
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
pub unsafe extern "C" fn BlendPalettes(selectedPalettes: u32, coeff: u8, color: u16) {
    unsafe {
        let mut selectedPalettes = selectedPalettes;
        let mut coeff = coeff;
        let mut color = color;
        let mut paletteOffset: u16 = 0u16;
        {
            paletteOffset = 0u16;
            'l1: loop {
                if !((selectedPalettes) != 0) {
                    break 'l1;
                }
                'l2: {
                    if (selectedPalettes & 1u32) != 0 {
                        BlendPalette(paletteOffset, 16u16, coeff, color);
                    }
                    selectedPalettes = (selectedPalettes >> 1);
                }
                paletteOffset = ((((paletteOffset) as i32).wrapping_add(16i32)) as u16);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BlendPalettesUnfaded(selectedPalettes: u32, coeff: u8, color: u16) {
    unsafe {
        let mut selectedPalettes = selectedPalettes;
        let mut coeff = coeff;
        let mut color = color;
        {
            let mut _src: *mut u8 = (((&raw mut gPlttBufferUnfaded).cast::<u8>().cast::<u16>())
                .cast::<u16>())
            .cast::<u8>();
            let mut _dest: *mut u8 = (((&raw mut gPlttBufferFaded).cast::<u8>().cast::<u16>())
                .cast::<u16>())
            .cast::<u8>();
            let mut _size: u32 = 1024u32;
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((_src) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    ((_dest) as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (2214592512u32
                                        | crate::c::div_u32(
                                            _size,
                                            ((crate::c::div_i32(32i32, 8i32)) as u32),
                                        )),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
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
        }
        BlendPalettes(selectedPalettes, coeff, color);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TintPalette_GrayScale(palette: *mut u16, count: u16) {
    unsafe {
        let mut palette = palette;
        let mut count = count;
        let mut r: i32 = 0i32;
        let mut g: i32 = 0i32;
        let mut b: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut gray: u32 = 0u32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((count) as i32)) {
                    break 'l1;
                }
                'l2: {
                    r = ((((palette).read()) as i32) & 31i32);
                    g = (((((palette).read()) as i32) >> 5) & 31i32);
                    b = (((((palette).read()) as i32) >> 10) & 31i32);
                    gray =
                        (((((r).wrapping_mul(
                            (((((0.3f32) as f32) * ((256i32) as f32)) as i16) as i32),
                        ))
                        .wrapping_add((g).wrapping_mul(
                            (((((0.59f32) as f32) * ((256i32) as f32)) as i16) as i32),
                        )))
                        .wrapping_add((b).wrapping_mul(
                            (((((0.1133f32) as f32) * ((256i32) as f32)) as i16) as i32),
                        )) >> 8) as u32);
                    ({
                        let __t1 = palette;
                        palette = (palette).wrapping_offset(1);
                        __t1
                    })
                    .write(((((gray << 10) | (gray << 5)) | gray) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TintPalette_GrayScale2(palette: *mut u16, count: u16) {
    unsafe {
        let mut palette = palette;
        let mut count = count;
        let mut r: i32 = 0i32;
        let mut g: i32 = 0i32;
        let mut b: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut gray: u32 = 0u32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((count) as i32)) {
                    break 'l1;
                }
                'l2: {
                    r = ((((palette).read()) as i32) & 31i32);
                    g = (((((palette).read()) as i32) >> 5) & 31i32);
                    b = (((((palette).read()) as i32) >> 10) & 31i32);
                    gray =
                        (((((r).wrapping_mul(
                            (((((0.3f32) as f32) * ((256i32) as f32)) as i16) as i32),
                        ))
                        .wrapping_add((g).wrapping_mul(
                            (((((0.59f32) as f32) * ((256i32) as f32)) as i16) as i32),
                        )))
                        .wrapping_add((b).wrapping_mul(
                            (((((0.1133f32) as f32) * ((256i32) as f32)) as i16) as i32),
                        )) >> 8) as u32);
                    if gray > 31u32 {
                        gray = 31u32;
                    }
                    gray = ((((((&raw const sRoundedDownGrayscaleMap)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((gray) as i32) as isize))
                    .read()) as u32);
                    ({
                        let __t1 = palette;
                        palette = (palette).wrapping_offset(1);
                        __t1
                    })
                    .write(((((gray << 10) | (gray << 5)) | gray) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TintPalette_SepiaTone(palette: *mut u16, count: u16) {
    unsafe {
        let mut palette = palette;
        let mut count = count;
        let mut r: i32 = 0i32;
        let mut g: i32 = 0i32;
        let mut b: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut gray: u32 = 0u32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((count) as i32)) {
                    break 'l1;
                }
                'l2: {
                    r = ((((palette).read()) as i32) & 31i32);
                    g = (((((palette).read()) as i32) >> 5) & 31i32);
                    b = (((((palette).read()) as i32) >> 10) & 31i32);
                    gray =
                        (((((r).wrapping_mul(
                            (((((0.3f32) as f32) * ((256i32) as f32)) as i16) as i32),
                        ))
                        .wrapping_add((g).wrapping_mul(
                            (((((0.59f32) as f32) * ((256i32) as f32)) as i16) as i32),
                        )))
                        .wrapping_add((b).wrapping_mul(
                            (((((0.1133f32) as f32) * ((256i32) as f32)) as i16) as i32),
                        )) >> 8) as u32);
                    r = (((((((((1.2f32) as f32) * ((256i32) as f32)) as i16) as u32)
                        .wrapping_mul(gray)) as u16) as i32)
                        >> 8);
                    g = (((((((((1.0f32) as f32) * ((256i32) as f32)) as i16) as u32)
                        .wrapping_mul(gray)) as u16) as i32)
                        >> 8);
                    b = (((((((((0.94f32) as f32) * ((256i32) as f32)) as i16) as u32)
                        .wrapping_mul(gray)) as u16) as i32)
                        >> 8);
                    if r > 31i32 {
                        r = 31i32;
                    }
                    ({
                        let __t1 = palette;
                        palette = (palette).wrapping_offset(1);
                        __t1
                    })
                    .write(((((b << 10) | (g << 5)) | r) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TintPalette_CustomTone(
    palette: *mut u16,
    count: u16,
    rTone: u16,
    gTone: u16,
    bTone: u16,
) {
    unsafe {
        let mut palette = palette;
        let mut count = count;
        let mut rTone = rTone;
        let mut gTone = gTone;
        let mut bTone = bTone;
        let mut r: i32 = 0i32;
        let mut g: i32 = 0i32;
        let mut b: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut gray: u32 = 0u32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((count) as i32)) {
                    break 'l1;
                }
                'l2: {
                    r = ((((palette).read()) as i32) & 31i32);
                    g = (((((palette).read()) as i32) >> 5) & 31i32);
                    b = (((((palette).read()) as i32) >> 10) & 31i32);
                    gray =
                        (((((r).wrapping_mul(
                            (((((0.3f32) as f32) * ((256i32) as f32)) as i16) as i32),
                        ))
                        .wrapping_add((g).wrapping_mul(
                            (((((0.59f32) as f32) * ((256i32) as f32)) as i16) as i32),
                        )))
                        .wrapping_add((b).wrapping_mul(
                            (((((0.1133f32) as f32) * ((256i32) as f32)) as i16) as i32),
                        )) >> 8) as u32);
                    r = ((((((rTone) as u32).wrapping_mul(gray)) as u16) as i32) >> 8);
                    g = ((((((gTone) as u32).wrapping_mul(gray)) as u16) as i32) >> 8);
                    b = ((((((bTone) as u32).wrapping_mul(gray)) as u16) as i32) >> 8);
                    if r > 31i32 {
                        r = 31i32;
                    }
                    if g > 31i32 {
                        g = 31i32;
                    }
                    if b > 31i32 {
                        b = 31i32;
                    }
                    ({
                        let __t1 = palette;
                        palette = (palette).wrapping_offset(1);
                        __t1
                    })
                    .write(((((b << 10) | (g << 5)) | r) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BlendPalettesGradually(
    selectedPalettes: u32,
    delay: i8,
    coeff: u8,
    coeffTarget: u8,
    color: u16,
    priority: u8,
    id: u8,
) {
    unsafe {
        let mut selectedPalettes = selectedPalettes;
        let mut delay = delay;
        let mut coeff = coeff;
        let mut coeffTarget = coeffTarget;
        let mut color = color;
        let mut priority = priority;
        let mut id = id;
        let mut taskId: u8 = 0u8;
        taskId = CreateTask(Some(Task_BlendPalettesGradually), priority);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((coeff) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((coeffTarget) as i16));
        if ((delay) as i32) >= 0i32 {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(((delay) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(1i16);
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write((((((delay) as i32).wrapping_neg()).wrapping_add(1i32)) as i16));
        }
        if ((coeffTarget) as i32) < ((coeff) as i32) {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p1).write((((((__p1).read()) as i32).wrapping_mul((-1i32))) as i16));
        }
        SetWordTaskArg(taskId, 5u8, selectedPalettes);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((color) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write(((id) as i16));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .read())
        .unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn IsBlendPalettesGraduallyTaskActive(id: u8) -> u32 {
    unsafe {
        let mut id = id;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                        .wrapping_add(4))
                    .read()) as i32)
                        == 1i32)
                        && (core::mem::transmute::<_, usize>(
                            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                                .cast::<Option<unsafe extern "C" fn(u8)>>())
                            .read(),
                        ) == (Task_BlendPalettesGradually as *const () as usize)))
                        && (((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset((i) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(8))
                        .read()) as i32)
                            == ((id) as i32))
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn DestroyBlendPalettesGraduallyTask() {
    unsafe {
        let mut taskId: u8 = 0u8;
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            taskId = FindTaskIdByFunc(Some(Task_BlendPalettesGradually));
            if ((taskId) as i32) == 255i32 {
                break 'l1;
            }
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_BlendPalettesGradually(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut palettes: u32 = 0u32;
        let mut data: *mut i16 = core::ptr::null_mut();
        let mut target: i16 = 0i16;
        data = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        palettes = GetWordTaskArg(taskId, 5u8);
        if (({
            let __p1 = (data).wrapping_offset(4);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > ((((data).wrapping_offset(3)).read()) as i32)
        {
            ((data).wrapping_offset(4)).write(0i16);
            BlendPalettes(
                palettes,
                (((data).read()) as u8),
                ((((data).wrapping_offset(7)).read()) as u16),
            );
            target = ((data).wrapping_offset(1)).read();
            if (((data).read()) as i32) == ((target) as i32) {
                DestroyTask(taskId);
            } else {
                (data).write(
                    (((((data).read()) as i32)
                        .wrapping_add(((((data).wrapping_offset(2)).read()) as i32)))
                        as i16),
                );
                if ((((data).wrapping_offset(2)).read()) as i32) >= 0i32 {
                    if (((data).read()) as i32) < ((target) as i32) {
                        return;
                    }
                } else {
                    if (((data).read()) as i32) > ((target) as i32) {
                        return;
                    }
                }
                (data).write(target);
            }
        }
    }
}
