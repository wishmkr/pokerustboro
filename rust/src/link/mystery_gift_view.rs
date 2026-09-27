//! Translated from `src/mystery_gift_view.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sCard_TextColorTable sCard_FooterTextOffsets sCard_WindowTemplates sWonderCardBgPal1 sWonderCardBgPal2 sWonderCardBgPal3 sWonderCardBgPal4 sWonderCardBgPal5 sWonderCardBgPal6 sWonderCardBgPal7 sWonderCardBgPal8 sWonderCardBgGfx1 sWonderCardBgTilemap1 sWonderCardBgGfx2 sWonderCardBgTilemap2 sWonderCardBgGfx3 sWonderCardBgTilemap3 sWonderCardBgGfx7 sWonderCardBgTilemap7 sWonderCardBgGfx8 sWonderCardBgTilemap8 sStampShadowPal1 sStampShadowPal2 sStampShadowPal3 sStampShadowPal4 sStampShadowPal5 sStampShadowPal6 sStampShadowPal7 sStampShadowPal8 sStampShadowGfx sSpriteSheet_StampShadow sSpritePalettes_StampShadow sSpriteTemplate_StampShadow sCardGraphics sNews_TextColorTable sNews_WindowTemplates sNews_ArrowsTemplate sWonderNewsPal1 sWonderNewsPal7 sWonderNewsPal8 sWonderNewsGfx1 sWonderNewsTilemap1 sWonderNewsGfx2 sWonderNewsTilemap2 sWonderNewsGfx3 sWonderNewsTilemap3 sWonderNewsGfx7 sWonderNewsTilemap7 sWonderNewsGfx8 sWonderNewsTilemap8 sNewsGraphics
#[allow(unused_imports)]
use crate::data::mystery_gift_view::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sWonderCardData: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sWonderNewsData: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gGiftIsFromEReader: u8;
    static mut gPaletteFade: u8;
    static mut gSprites: u8;
    fn AddScrollIndicatorArrowPair(a0: *mut u8, a1: *mut u16) -> u8;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AddWindow(a0: *mut u8) -> u16;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyRectToBgTilemapBufferRect(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
        a7: u8,
        a8: u8,
        a9: u8,
        a10: u8,
        a11: i16,
        a12: i16,
    );
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateMonIconNoPersonality(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut u8)>,
        a2: i16,
        a3: i16,
        a4: u8,
        a5: u32,
    ) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroySprite(a0: *mut u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAndDestroyMonIconSprite(a0: *mut u8);
    fn FreeMonIconPalettes();
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetFontAttribute(a0: u8, a1: u8) -> u8;
    fn GetIconSpeciesNoPersonality(a0: u16) -> u16;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetTextWindowPalette(a0: u8) -> *mut u16;
    fn HideBg(a0: u8);
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedSpriteSheetUsingHeap(a0: *mut u8) -> u8;
    fn LoadMonIconPalettes();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn MG_DrawCheckerboardPattern(a0: u32);
    fn PrintMysteryGiftOrEReaderHeader(a0: u8, a1: u32);
    fn PutWindowTilemap(a0: u8);
    fn RemoveScrollIndicatorArrowPair(a0: u8);
    fn RemoveWindow(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn UpdatePaletteFade() -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WonderCard_Init(card: *mut u8, metadata: *mut u8) -> u32 {
    unsafe {
        let mut card = card;
        let mut metadata = metadata;
        if (((card) as usize) == 0usize) || (((metadata) as usize) == 0usize) {
            return 0u32;
        }
        ((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(5212u32));
        if ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize
        {
            return 0u32;
        }
        (((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<crate::c::Rec4<332>>()
            .write_unaligned(card.cast::<crate::c::Rec4<332>>().read_unaligned());
        (((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(332)
            .cast::<crate::c::Rec4<36>>()
            .write_unaligned(metadata.cast::<crate::c::Rec4<36>>().read_unaligned());
        if ((crate::c::bf_read(
            (((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8),
            2,
            4,
            false,
        ) as u8) as i32)
            >= 8i32
        {
            crate::c::bf_write(
                (((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8),
                2,
                4,
                (0u8) as i32,
            );
        }
        if ((crate::c::bf_read(
            (((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8),
            0,
            2,
            false,
        ) as u8) as i32)
            >= 3i32
        {
            crate::c::bf_write(
                (((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8),
                0,
                2,
                (0u8) as i32,
            );
        }
        if ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
            .read()) as i32)
            > 7i32
        {
            ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
                .write(0u8);
        }
        ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(368)
            .cast::<*mut u8>())
        .write(
            (((&raw const sCardGraphics).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(
                ((crate::c::bf_read(
                    (((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8),
                    2,
                    4,
                    false,
                ) as u8) as i32) as isize
                    * 16,
            ),
        );
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WonderCard_Destroy() {
    unsafe {
        if ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize
        {
            ((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>())
                .read()
                .cast::<crate::c::Rec4<5212>>()
                .write_unaligned({
                    let mut __lit1 = crate::ffi::Align4([0u8; 5212]);
                    (&raw mut __lit1)
                        .cast::<crate::c::Rec4<5212>>()
                        .read_unaligned()
                });
            {
                Free(((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read());
                ((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>())
                    .write(core::ptr::null_mut());
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WonderCard_Enter() -> i32 {
    unsafe {
        if ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize
        {
            return (-1i32);
        }
        'l1: {
            let __sw1 = ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(372))
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32;
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (UpdatePaletteFade()) != 0 {
                    return 0i32;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                FillBgTilemapBufferRect_Palette0(
                    0u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                FillBgTilemapBufferRect_Palette0(
                    1u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                FillBgTilemapBufferRect_Palette0(
                    2u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                CopyBgTilemapBufferToVram(0u8);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(2u8);
                DecompressAndCopyTileDataToVram(
                    2u8,
                    (((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(368)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(4)
                    .cast::<*mut u32>())
                    .read())
                    .cast::<u8>(),
                    0u32,
                    8u16,
                    0u8,
                );
                (((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(374))
                .cast::<u16>())
                .write(AddWindow(
                    ((&raw const sCard_WindowTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                ));
                ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(374))
                .cast::<u16>())
                .wrapping_offset(1))
                .write(AddWindow(
                    (((&raw const sCard_WindowTemplates).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(8),
                ));
                ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(374))
                .cast::<u16>())
                .wrapping_offset(2))
                .write(AddWindow(
                    (((&raw const sCard_WindowTemplates).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(16),
                ));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 0i32;
                }
                LoadPalette((GetTextWindowPalette(1u8)).cast::<u8>(), 32u16, 32u16);
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (1u16) as i32,
                );
                LoadPalette(
                    (((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(368)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12)
                    .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                    16u16,
                    32u16,
                );
                LZ77UnCompWram(
                    ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(368)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(8)
                    .cast::<*mut u32>())
                    .read(),
                    ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1116))
                    .cast::<u8>(),
                );
                CopyRectToBgTilemapBufferRect(
                    2u8,
                    ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1116))
                    .cast::<u8>(),
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                    1u8,
                    8i16,
                    0i16,
                );
                CopyBgTilemapBufferToVram(2u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                BufferCardText();
                break 'l1;
            }
            if __sw1 == 5i32 {
                DrawCardWindow(0u8);
                DrawCardWindow(1u8);
                DrawCardWindow(2u8);
                CopyBgTilemapBufferToVram(1u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                LoadMonIconPalettes();
                break 'l1;
            }
            if __sw1 == 7i32 {
                ShowBg(1u8);
                ShowBg(2u8);
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                CreateCardSprites();
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                UpdatePaletteFade();
                break 'l1;
            }
            if !__matched {
                if (UpdatePaletteFade()) != 0 {
                    return 0i32;
                }
                ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(372))
                .write(0u8);
                return 1i32;
            }
        }
        let __p2 =
            (((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(372);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0i32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WonderCard_Exit(useCancel: u32) -> i32 {
    unsafe {
        let mut useCancel = useCancel;
        if ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize
        {
            return (-1i32);
        }
        'l1: {
            let __sw1 = ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(372))
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32;
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (UpdatePaletteFade()) != 0 {
                    return 0i32;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                FillBgTilemapBufferRect_Palette0(
                    0u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                FillBgTilemapBufferRect_Palette0(
                    1u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                FillBgTilemapBufferRect_Palette0(
                    2u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                CopyBgTilemapBufferToVram(0u8);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(2u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                HideBg(1u8);
                HideBg(2u8);
                RemoveWindow(
                    ((((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(374))
                    .cast::<u16>())
                    .wrapping_offset(2))
                    .read()) as u8),
                );
                RemoveWindow(
                    ((((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(374))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read()) as u8),
                );
                RemoveWindow(
                    (((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(374))
                    .cast::<u16>())
                    .read()) as u8),
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                DestroyCardSprites();
                FreeMonIconPalettes();
                break 'l1;
            }
            if __sw1 == 5i32 {
                PrintMysteryGiftOrEReaderHeader(
                    ((&raw mut gGiftIsFromEReader).cast::<u8>()).read(),
                    useCancel,
                );
                CopyBgTilemapBufferToVram(0u8);
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                break 'l1;
            }
            if !__matched {
                if (UpdatePaletteFade()) != 0 {
                    return 0i32;
                }
                ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(372))
                .write(0u8);
                return 1i32;
            }
        }
        let __p2 =
            (((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(372);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn BufferCardText() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut charsUntilStat: u16 = 0u16;
        let mut stats = crate::ffi::Align4([0u8; 6]);
        (&raw mut stats)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(0u16);
        (&raw mut stats)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<u16>()
            .write(0u16);
        (&raw mut stats)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(0u16);
        crate::c::memcpy(
            ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(395))
            .cast::<u8>(),
            ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>(),
            40u32,
        );
        ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(395))
        .cast::<u8>())
        .wrapping_offset(40))
        .write(255u8);
        crate::c::memcpy(
            ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(436))
            .cast::<u8>(),
            ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(50))
                .cast::<u8>(),
            40u32,
        );
        ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(436))
        .cast::<u8>())
        .wrapping_offset(40))
        .write(255u8);
        if ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u32>())
        .read()
            > 999999u32
        {
            ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u32>())
            .write(999999u32);
        }
        ConvertIntToDecimalStringN(
            ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(477))
            .cast::<u8>(),
            ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u32>())
            .read()) as i32),
            0i32,
            6u8,
        );
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    crate::c::memcpy(
                        ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(484))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 41))
                        .cast::<u8>(),
                        ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(90))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 40))
                        .cast::<u8>(),
                        40u32,
                    );
                    ((((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(484))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 41))
                    .cast::<u8>())
                    .wrapping_offset(40))
                    .write(255u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::memcpy(
            ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(648))
            .cast::<u8>(),
            ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(250))
            .cast::<u8>(),
            40u32,
        );
        ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(648))
        .cast::<u8>())
        .wrapping_offset(40))
        .write(255u8);
        'l3: {
            let __sw1 = ((crate::c::bf_read(
                (((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8),
                0,
                2,
                false,
            ) as u8) as i32);
            if __sw1 == 0i32 {
                crate::c::memcpy(
                    ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(689))
                    .cast::<u8>(),
                    ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(290))
                    .cast::<u8>(),
                    40u32,
                );
                ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(689))
                .cast::<u8>())
                .wrapping_offset(40))
                .write(255u8);
                break 'l3;
            }
            if __sw1 == 1i32 {
                (((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(689))
                .cast::<u8>())
                .write(255u8);
                break 'l3;
            }
            if __sw1 == 2i32 {
                (((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(689))
                .cast::<u8>())
                .write(255u8);
                ((&raw mut stats).cast::<u16>()).write(
                    ((if (((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(332))
                    .cast::<u16>())
                    .read()) as i32)
                        < 999i32
                    {
                        (((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(332))
                        .cast::<u16>())
                        .read()) as i32)
                    } else {
                        999i32
                    }) as u16),
                );
                (((&raw mut stats).cast::<u16>()).wrapping_offset(1)).write(
                    ((if (((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(332))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read()) as i32)
                        < 999i32
                    {
                        (((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(332))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32)
                    } else {
                        999i32
                    }) as u16),
                );
                (((&raw mut stats).cast::<u16>()).wrapping_offset(2)).write(
                    ((if (((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(332))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .read()) as i32)
                        < 999i32
                    {
                        (((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(332))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .read()) as i32)
                    } else {
                        999i32
                    }) as u16),
                );
                {
                    i = 0u16;
                    'l4: loop {
                        if !(((i) as u32) < crate::c::div_u32(384u32, 48u32)) {
                            break 'l4;
                        }
                        'l5: {
                            crate::c::memset(
                                (((((((&raw mut sWonderCardData)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(732))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 48))
                                .wrapping_add(42))
                                .cast::<u8>(),
                                255i32,
                                4u32,
                            );
                            crate::c::memset(
                                (((((((&raw mut sWonderCardData)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(732))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 48))
                                .wrapping_add(1))
                                .cast::<u8>(),
                                255i32,
                                41u32,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = 0u16;
                    charsUntilStat = 0u16;
                    'l6: loop {
                        if !(((i) as i32) < 40i32) {
                            break 'l6;
                        }
                        'l7: {
                            if ((((((((&raw mut sWonderCardData)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(290))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                != 247i32
                            {
                                (((((((((&raw mut sWonderCardData)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(732))
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut sWonderCardData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(373))
                                    .read()) as i32) as isize
                                        * 48,
                                ))
                                .wrapping_add(1))
                                .cast::<u8>())
                                .wrapping_offset(((charsUntilStat) as i32) as isize))
                                .write(
                                    ((((((&raw mut sWonderCardData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(290))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read(),
                                );
                                charsUntilStat = (charsUntilStat).wrapping_add(1);
                            } else {
                                let mut id: u8 = ((((((&raw mut sWonderCardData)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(290))
                                .cast::<u8>())
                                .wrapping_offset((((i) as i32).wrapping_add(1i32)) as isize))
                                .read();
                                if ((id) as u32) >= crate::c::div_u32(6u32, 2u32) {
                                    i = ((((i) as i32).wrapping_add(2i32)) as u16);
                                } else {
                                    ConvertIntToDecimalStringN(
                                        (((((((&raw mut sWonderCardData)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(732))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((((((&raw mut sWonderCardData)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(373))
                                            .read())
                                                as i32)
                                                as isize
                                                * 48,
                                        ))
                                        .wrapping_add(42))
                                        .cast::<u8>(),
                                        (((((&raw mut stats).cast::<u16>())
                                            .wrapping_offset(((id) as i32) as isize))
                                        .read()) as i32),
                                        2i32,
                                        3u8,
                                    );
                                    ((((((&raw mut sWonderCardData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(732))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((((&raw mut sWonderCardData)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(373))
                                        .read()) as i32)
                                            as isize
                                            * 48,
                                    ))
                                    .write(
                                        ((((((&raw mut sWonderCardData)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(290))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((i) as i32).wrapping_add(2i32)) as isize,
                                        ))
                                        .read(),
                                    );
                                    let __p2 = (((&raw mut sWonderCardData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(373);
                                    (__p2).write(((__p2).read()).wrapping_add(1));
                                    if ((((((&raw mut sWonderCardData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(373))
                                    .read()) as u32)
                                        >= crate::c::div_u32(384u32, 48u32)
                                    {
                                        break 'l6;
                                    }
                                    charsUntilStat = 0u16;
                                    i = ((((i) as i32).wrapping_add(2i32)) as u16);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawCardWindow(whichWindow: u8) {
    unsafe {
        let mut whichWindow = whichWindow;
        let mut i: i8 = 0i8;
        let mut windowId: i32 =
            ((((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(374))
            .cast::<u16>())
            .wrapping_offset(((whichWindow) as i32) as isize))
            .read()) as i32);
        PutWindowTilemap(((windowId) as u8));
        FillWindowPixelBuffer(((windowId) as u8), 0u8);
        'l1: {
            let __sw1 = ((whichWindow) as i32);
            if __sw1 == 0i32 {
                {
                    let mut x: i32 = 0i32;
                    AddTextPrinterParameterized3(
                        ((windowId) as u8),
                        3u8,
                        0u8,
                        1u8,
                        ((((&raw const sCard_TextColorTable).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(368)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(0),
                                0,
                                4,
                                false,
                            ) as u8) as i32) as isize
                                * 3,
                        ))
                        .cast::<u8>(),
                        0i8,
                        ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(395))
                        .cast::<u8>(),
                    );
                    x = (160i32).wrapping_sub(GetStringWidth(
                        3u8,
                        ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(436))
                        .cast::<u8>(),
                        ((GetFontAttribute(3u8, 2u8)) as i16),
                    ));
                    if x < 0i32 {
                        x = 0i32;
                    }
                    AddTextPrinterParameterized3(
                        ((windowId) as u8),
                        3u8,
                        ((x) as u8),
                        17u8,
                        ((((&raw const sCard_TextColorTable).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(368)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(0),
                                0,
                                4,
                                false,
                            ) as u8) as i32) as isize
                                * 3,
                        ))
                        .cast::<u8>(),
                        0i8,
                        ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(436))
                        .cast::<u8>(),
                    );
                    if ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<u32>())
                    .read()
                        != 0u32
                    {
                        AddTextPrinterParameterized3(
                            ((windowId) as u8),
                            1u8,
                            166u8,
                            17u8,
                            ((((&raw const sCard_TextColorTable).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::bf_read(
                                    (((((&raw mut sWonderCardData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(368)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(0),
                                    0,
                                    4,
                                    false,
                                ) as u8) as i32) as isize
                                    * 3,
                            ))
                            .cast::<u8>(),
                            0i8,
                            ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(477))
                            .cast::<u8>(),
                        );
                    }
                    break 'l1;
                }
            }
            if __sw1 == 1i32 {
                {
                    'l2: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            AddTextPrinterParameterized3(
                                ((windowId) as u8),
                                3u8,
                                0u8,
                                ((((16i32).wrapping_mul(((i) as i32))).wrapping_add(2i32)) as u8),
                                ((((&raw const sCard_TextColorTable).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((crate::c::bf_read(
                                        (((((&raw mut sWonderCardData)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(368)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(0),
                                        4,
                                        4,
                                        false,
                                    ) as u8) as i32) as isize
                                        * 3,
                                ))
                                .cast::<u8>(),
                                0i8,
                                ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(484))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 41))
                                .cast::<u8>(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                AddTextPrinterParameterized3(
                    ((windowId) as u8),
                    3u8,
                    0u8,
                    ((((&raw const sCard_FooterTextOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(8),
                                0,
                                2,
                                false,
                            ) as u8) as i32) as isize,
                        ))
                    .read(),
                    ((((&raw const sCard_TextColorTable).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(368)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(1),
                                0,
                                4,
                                false,
                            ) as u8) as i32) as isize
                                * 3,
                        ))
                    .cast::<u8>(),
                    0i8,
                    ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(648))
                    .cast::<u8>(),
                );
                if ((crate::c::bf_read(
                    (((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8),
                    0,
                    2,
                    false,
                ) as u8) as i32)
                    != 2i32
                {
                    AddTextPrinterParameterized3(
                        ((windowId) as u8),
                        3u8,
                        0u8,
                        (((16i32).wrapping_add(
                            ((((((&raw const sCard_FooterTextOffsets).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::bf_read(
                                    (((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(8),
                                    0,
                                    2,
                                    false,
                                ) as u8) as i32) as isize,
                            ))
                            .read()) as i32),
                        )) as u8),
                        ((((&raw const sCard_TextColorTable).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(368)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(1),
                                0,
                                4,
                                false,
                            ) as u8) as i32) as isize
                                * 3,
                        ))
                        .cast::<u8>(),
                        0i8,
                        ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(689))
                        .cast::<u8>(),
                    );
                } else {
                    let mut x: i32 = 0i32;
                    let mut y: i32 = ((((((&raw const sCard_FooterTextOffsets)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            (((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(8),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize,
                    ))
                    .read()) as i32)
                        .wrapping_add(16i32);
                    let mut spacing: i32 = ((GetFontAttribute(3u8, 2u8)) as i32);
                    {
                        'l4: loop {
                            if !(((i) as i32)
                                < ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(373))
                                .read()) as i32))
                            {
                                break 'l4;
                            }
                            'l5: {
                                AddTextPrinterParameterized3(
                                    ((windowId) as u8),
                                    3u8,
                                    ((x) as u8),
                                    ((y) as u8),
                                    ((((&raw const sCard_TextColorTable).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset(
                                        ((crate::c::bf_read(
                                            (((((&raw mut sWonderCardData)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(368)
                                            .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1),
                                            0,
                                            4,
                                            false,
                                        ) as u8) as i32)
                                            as isize
                                            * 3,
                                    ))
                                    .cast::<u8>(),
                                    0i8,
                                    (((((((&raw mut sWonderCardData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(732))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 48))
                                    .wrapping_add(1))
                                    .cast::<u8>(),
                                );
                                if ((((((((((&raw mut sWonderCardData)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(732))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 48))
                                .wrapping_add(42))
                                .cast::<u8>())
                                .read()) as i32)
                                    != 255i32
                                {
                                    x = (x).wrapping_add(GetStringWidth(
                                        3u8,
                                        (((((((&raw mut sWonderCardData)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(732))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 48))
                                        .wrapping_add(1))
                                        .cast::<u8>(),
                                        ((spacing) as i16),
                                    ));
                                    AddTextPrinterParameterized3(
                                        ((windowId) as u8),
                                        3u8,
                                        ((x) as u8),
                                        ((y) as u8),
                                        ((((&raw const sCard_TextColorTable)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((crate::c::bf_read(
                                                (((((&raw mut sWonderCardData)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(368)
                                                .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(1),
                                                0,
                                                4,
                                                false,
                                            ) as u8)
                                                as i32)
                                                as isize
                                                * 3,
                                        ))
                                        .cast::<u8>(),
                                        0i8,
                                        (((((((&raw mut sWonderCardData)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(732))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 48))
                                        .wrapping_add(42))
                                        .cast::<u8>(),
                                    );
                                    x = (x).wrapping_add(
                                        (GetStringWidth(
                                            3u8,
                                            (((((((&raw mut sWonderCardData)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(732))
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 48))
                                            .wrapping_add(42))
                                            .cast::<u8>(),
                                            ((spacing) as i16),
                                        ))
                                        .wrapping_add(
                                            ((((((((&raw mut sWonderCardData)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(732))
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 48))
                                            .read())
                                                as i32),
                                        ),
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                break 'l1;
            }
        }
        CopyWindowToVram(((windowId) as u8), 3u8);
    }
}
pub(crate) unsafe extern "C" fn CreateCardSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(380))
            .write(255u8);
        if (((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(332))
        .wrapping_add(6)
        .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            ((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(380))
            .write(CreateMonIconNoPersonality(
                GetIconSpeciesNoPersonality(
                    (((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(332))
                    .wrapping_add(6)
                    .cast::<u16>())
                    .read(),
                ),
                Some(SpriteCallbackDummy),
                220i16,
                20i16,
                0u8,
                0u32,
            ));
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(380))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(5),
                2,
                2,
                (2u16) as i32,
            );
        }
        if (((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(9))
        .read()) as i32)
            != 0i32)
            && (((crate::c::bf_read(
                (((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8),
                0,
                2,
                false,
            ) as u8) as i32)
                == 1i32)
        {
            LoadCompressedSpriteSheetUsingHeap(
                (&raw const sSpriteSheet_StampShadow)
                    .cast::<u8>()
                    .cast_mut(),
            );
            LoadSpritePalette(
                (((&raw const sSpritePalettes_StampShadow)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(
                    ((crate::c::bf_read(
                        (((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(368)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1),
                        4,
                        4,
                        false,
                    ) as u8) as i32) as isize
                        * 8,
                ),
            );
            {
                'l1: loop {
                    if !(((i) as i32)
                        < ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(9))
                        .read()) as i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        (((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(381))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                        .cast::<u8>())
                        .write(255u8);
                        ((((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(381))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .write(255u8);
                        (((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(381))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                        .cast::<u8>())
                        .write(CreateSprite(
                            (&raw const sSpriteTemplate_StampShadow)
                                .cast::<u8>()
                                .cast_mut(),
                            (((216i32).wrapping_sub((32i32).wrapping_mul(((i) as i32)))) as i16),
                            144i16,
                            8u8,
                        ));
                        if ((((((((((&raw mut sWonderCardData)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(332))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            != 0i32
                        {
                            ((((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(381))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 2))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .write(CreateMonIconNoPersonality(
                                GetIconSpeciesNoPersonality(
                                    ((((((((&raw mut sWonderCardData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(332))
                                    .wrapping_add(8))
                                    .cast::<u8>())
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read(),
                                ),
                                Some(SpriteCallbackDummy),
                                (((216i32).wrapping_sub((32i32).wrapping_mul(((i) as i32))))
                                    as i16),
                                136i16,
                                0u8,
                                0u32,
                            ));
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyCardSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        if ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(380))
        .read()) as i32)
            != 255i32
        {
            FreeAndDestroyMonIconSprite(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(380))
                    .read()) as i32) as isize
                        * 68,
                ),
            );
        }
        if (((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(9))
        .read()) as i32)
            != 0i32)
            && (((crate::c::bf_read(
                (((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8),
                0,
                2,
                false,
            ) as u8) as i32)
                == 1i32)
        {
            {
                'l1: loop {
                    if !(((i) as i32)
                        < ((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(9))
                        .read()) as i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        if (((((((((&raw mut sWonderCardData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(381))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                        .cast::<u8>())
                        .read()) as i32)
                            != 255i32
                        {
                            DestroySprite(
                                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((((((&raw mut sWonderCardData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(381))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 2))
                                    .cast::<u8>())
                                    .read()) as i32) as isize
                                        * 68,
                                ),
                            );
                        }
                        if ((((((((((&raw mut sWonderCardData)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(381))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            != 255i32
                        {
                            FreeAndDestroyMonIconSprite(
                                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((((&raw mut sWonderCardData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(381))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 2))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read()) as i32) as isize
                                        * 68,
                                ),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            FreeSpriteTilesByTag(32768u16);
            FreeSpritePaletteByTag(32768u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WonderNews_Init(news: *mut u8) -> u32 {
    unsafe {
        let mut news = news;
        if ((news) as usize) == 0usize {
            return 0u32;
        }
        ((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(5028u32));
        if ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize
        {
            return 0u32;
        }
        (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<crate::c::Rec4<444>>()
            .write_unaligned(news.cast::<crate::c::Rec4<444>>().read_unaligned());
        if ((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
            .read()) as i32)
            >= 8i32
        {
            ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
                .write(0u8);
        }
        ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(444)
            .cast::<*mut u8>())
        .write(
            (((&raw const sNewsGraphics).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read()) as i32) as isize
                    * 16,
            ),
        );
        ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(449))
            .write(255u8);
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WonderNews_Destroy() {
    unsafe {
        if ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize
        {
            ((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>())
                .read()
                .cast::<crate::c::Rec4<5028>>()
                .write_unaligned({
                    let mut __lit1 = crate::ffi::Align4([0u8; 5028]);
                    (&raw mut __lit1)
                        .cast::<crate::c::Rec4<5028>>()
                        .read_unaligned()
                });
            {
                Free(((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read());
                ((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>())
                    .write(core::ptr::null_mut());
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WonderNews_Enter() -> i32 {
    unsafe {
        if ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize
        {
            return (-1i32);
        }
        'l1: {
            let __sw1 = ((crate::c::bf_read(
                (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(448),
                1,
                7,
                false,
            ) as u8) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32;
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (UpdatePaletteFade()) != 0 {
                    return 0i32;
                }
                ChangeBgY(0u8, 0i32, 0u8);
                ChangeBgY(1u8, 0i32, 0u8);
                ChangeBgY(2u8, 0i32, 0u8);
                ChangeBgY(3u8, 0i32, 0u8);
                SetGpuReg(64u8, 240u16);
                SetGpuReg(68u8, 6808u16);
                SetGpuReg(72u8, 31u16);
                SetGpuReg(74u8, 27u16);
                SetGpuRegBits(0u8, 8192u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                FillBgTilemapBufferRect_Palette0(
                    0u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                FillBgTilemapBufferRect_Palette0(
                    1u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                FillBgTilemapBufferRect_Palette0(
                    2u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                FillBgTilemapBufferRect_Palette0(
                    3u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                CopyBgTilemapBufferToVram(0u8);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(2u8);
                CopyBgTilemapBufferToVram(3u8);
                DecompressAndCopyTileDataToVram(
                    3u8,
                    (((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(444)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(4)
                    .cast::<*mut u32>())
                    .read())
                    .cast::<u8>(),
                    0u32,
                    8u16,
                    0u8,
                );
                (((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(456))
                .cast::<u16>())
                .write(AddWindow(
                    ((&raw const sNews_WindowTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                ));
                ((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(456))
                .cast::<u16>())
                .wrapping_offset(1))
                .write(AddWindow(
                    (((&raw const sNews_WindowTemplates).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(8),
                ));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 0i32;
                }
                LoadPalette((GetTextWindowPalette(1u8)).cast::<u8>(), 32u16, 32u16);
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (1u16) as i32,
                );
                LoadPalette(
                    (((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(444)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12)
                    .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                    16u16,
                    32u16,
                );
                LZ77UnCompWram(
                    ((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(444)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(8)
                    .cast::<*mut u32>())
                    .read(),
                    ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(932))
                    .cast::<u8>(),
                );
                CopyRectToBgTilemapBufferRect(
                    1u8,
                    ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(932))
                    .cast::<u8>(),
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    3u8,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    3u8,
                    1u8,
                    8i16,
                    0i16,
                );
                CopyRectToBgTilemapBufferRect(
                    3u8,
                    ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(932))
                    .cast::<u8>(),
                    0u8,
                    3u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    (((3i32).wrapping_add(crate::c::div_i32(160i32, 8i32))) as u8),
                    0u8,
                    3u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    (((3i32).wrapping_add(crate::c::div_i32(160i32, 8i32))) as u8),
                    1u8,
                    8i16,
                    0i16,
                );
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(3u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                BufferNewsText();
                break 'l1;
            }
            if __sw1 == 5i32 {
                DrawNewsWindows();
                CopyBgTilemapBufferToVram(0u8);
                CopyBgTilemapBufferToVram(2u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                ShowBg(1u8);
                ShowBg(2u8);
                ShowBg(3u8);
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(449))
                .write(AddScrollIndicatorArrowPair(
                    (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(916),
                    (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(454)
                        .cast::<u16>(),
                ));
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                UpdatePaletteFade();
                break 'l1;
            }
            if !__matched {
                if (UpdatePaletteFade()) != 0 {
                    return 0i32;
                }
                crate::c::bf_write(
                    (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(448),
                    1,
                    7,
                    (0u8) as i32,
                );
                return 1i32;
            }
        }
        crate::c::bf_write(
            (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(448),
            1,
            7,
            ((crate::c::bf_read(
                (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(448),
                1,
                7,
                false,
            ) as u8)
                .wrapping_add(1)) as i32,
        );
        return 0i32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WonderNews_Exit(useCancel: u32) -> i32 {
    unsafe {
        let mut useCancel = useCancel;
        if ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize
        {
            return (-1i32);
        }
        'l1: {
            let __sw1 = ((crate::c::bf_read(
                (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(448),
                1,
                7,
                false,
            ) as u8) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32;
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (UpdatePaletteFade()) != 0 {
                    return 0i32;
                }
                ChangeBgY(2u8, 0i32, 0u8);
                SetGpuReg(64u8, 0u16);
                SetGpuReg(68u8, 0u16);
                SetGpuReg(72u8, 0u16);
                SetGpuReg(74u8, 0u16);
                ClearGpuRegBits(0u8, 8192u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                FillBgTilemapBufferRect_Palette0(
                    0u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                FillBgTilemapBufferRect_Palette0(
                    1u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                FillBgTilemapBufferRect_Palette0(
                    2u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    (((crate::c::div_i32(160i32, 8i32)).wrapping_add(4i32)) as u8),
                );
                FillBgTilemapBufferRect_Palette0(
                    3u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    (((crate::c::div_i32(160i32, 8i32)).wrapping_add(4i32)) as u8),
                );
                CopyBgTilemapBufferToVram(0u8);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(2u8);
                CopyBgTilemapBufferToVram(3u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                HideBg(1u8);
                HideBg(2u8);
                RemoveWindow(
                    ((((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(456))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read()) as u8),
                );
                RemoveWindow(
                    (((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(456))
                    .cast::<u16>())
                    .read()) as u8),
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                ChangeBgY(2u8, 0i32, 0u8);
                ChangeBgY(3u8, 0i32, 0u8);
                if ((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(449))
                .read()) as i32)
                    != 255i32
                {
                    RemoveScrollIndicatorArrowPair(
                        ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(449))
                        .read(),
                    );
                    ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(449))
                    .write(255u8);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                PrintMysteryGiftOrEReaderHeader(
                    ((&raw mut gGiftIsFromEReader).cast::<u8>()).read(),
                    useCancel,
                );
                MG_DrawCheckerboardPattern(3u32);
                CopyBgTilemapBufferToVram(0u8);
                CopyBgTilemapBufferToVram(3u8);
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                break 'l1;
            }
            if !__matched {
                if (UpdatePaletteFade()) != 0 {
                    return 0i32;
                }
                crate::c::bf_write(
                    (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(448),
                    1,
                    7,
                    (0u8) as i32,
                );
                return 1i32;
            }
        }
        crate::c::bf_write(
            (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(448),
            1,
            7,
            ((crate::c::bf_read(
                (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(448),
                1,
                7,
                false,
            ) as u8)
                .wrapping_add(1)) as i32,
        );
        return 0i32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WonderNews_RemoveScrollIndicatorArrowPair() {
    unsafe {
        if (!((crate::c::bf_read(
            (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(448),
            0,
            1,
            false,
        ) as u8)
            != 0))
            && (((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(449))
            .read()) as i32)
                != 255i32)
        {
            RemoveScrollIndicatorArrowPair(
                ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(449))
                .read(),
            );
            ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(449))
            .write(255u8);
            crate::c::bf_write(
                (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(448),
                0,
                1,
                (1u8) as i32,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WonderNews_AddScrollIndicatorArrowPair() {
    unsafe {
        if (crate::c::bf_read(
            (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(448),
            0,
            1,
            false,
        ) as u8)
            != 0
        {
            ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(449))
            .write(AddScrollIndicatorArrowPair(
                (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(916),
                (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(454)
                    .cast::<u16>(),
            ));
            crate::c::bf_write(
                (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(448),
                0,
                1,
                (0u8) as i32,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WonderNews_GetInput(input: u16) -> u32 {
    unsafe {
        let mut input = input;
        if (crate::c::bf_read(
            (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(450),
            0,
            1,
            false,
        ) as u8)
            != 0
        {
            UpdateNewsScroll();
            return 255u32;
        }
        'l1: {
            let __sw1 = ((input) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 64i32 || __sw1 == 128i32;
            if __sw1 == 1i32 {
                return 0u32;
            }
            if __sw1 == 2i32 {
                return 1u32;
            }
            if __sw1 == 64i32 {
                if ((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(454)
                    .cast::<u16>())
                .read()) as i32)
                    == 0i32
                {
                    return 255u32;
                }
                if (crate::c::bf_read(
                    (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(448),
                    0,
                    1,
                    false,
                ) as u8)
                    != 0
                {
                    return 255u32;
                }
                crate::c::bf_write(
                    (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(451),
                    0,
                    1,
                    (0u8) as i32,
                );
                break 'l1;
            }
            if __sw1 == 128i32 {
                if ((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(454)
                    .cast::<u16>())
                .read()) as i32)
                    == ((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(452)
                        .cast::<u16>())
                    .read()) as i32)
                {
                    return 255u32;
                }
                if (crate::c::bf_read(
                    (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(448),
                    0,
                    1,
                    false,
                ) as u8)
                    != 0
                {
                    return 255u32;
                }
                crate::c::bf_write(
                    (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(451),
                    0,
                    1,
                    (1u8) as i32,
                );
                break 'l1;
            }
            if !__matched {
                return 255u32;
            }
        }
        crate::c::bf_write(
            (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(450),
            0,
            1,
            (1u8) as i32,
        );
        crate::c::bf_write(
            (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(450),
            1,
            7,
            (2u8) as i32,
        );
        crate::c::bf_write(
            (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(451),
            1,
            7,
            (0u8) as i32,
        );
        if !((crate::c::bf_read(
            (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(451),
            0,
            1,
            false,
        ) as u8)
            != 0)
        {
            return 2u32;
        } else {
            return 3u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn BufferNewsText() {
    unsafe {
        let mut i: u8 = 0u8;
        crate::c::memcpy(
            ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(462))
            .cast::<u8>(),
            ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>(),
            40u32,
        );
        ((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(462))
        .cast::<u8>())
        .wrapping_offset(40))
        .write(255u8);
        {
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    crate::c::memcpy(
                        ((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(503))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 41))
                        .cast::<u8>(),
                        ((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(44))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 40))
                        .cast::<u8>(),
                        40u32,
                    );
                    ((((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(503))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 41))
                    .cast::<u8>())
                    .wrapping_offset(40))
                    .write(255u8);
                    if (((i) as i32) > 7i32)
                        && ((((((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(503))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 41))
                        .cast::<u8>())
                        .read()) as i32)
                            != 255i32)
                    {
                        let __p1 = (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(452)
                        .cast::<u16>();
                        (__p1).write(((__p1).read()).wrapping_add(1));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(916)
            .cast::<crate::c::Rec4<16>>()
            .write_unaligned(
                (&raw const sNews_ArrowsTemplate)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<16>>()
                    .read_unaligned(),
            );
        (((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(916))
            .wrapping_add(8)
            .cast::<u16>())
        .write(
            ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(452)
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DrawNewsWindows() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut x: i32 = 0i32;
        PutWindowTilemap(
            (((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(456))
            .cast::<u16>())
            .read()) as u8),
        );
        PutWindowTilemap(
            ((((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(456))
            .cast::<u16>())
            .wrapping_offset(1))
            .read()) as u8),
        );
        FillWindowPixelBuffer(
            (((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(456))
            .cast::<u16>())
            .read()) as u8),
            0u8,
        );
        FillWindowPixelBuffer(
            ((((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(456))
            .cast::<u16>())
            .wrapping_offset(1))
            .read()) as u8),
            0u8,
        );
        x = crate::c::div_i32(
            (224i32).wrapping_sub(GetStringWidth(
                3u8,
                ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(462))
                .cast::<u8>(),
                ((GetFontAttribute(3u8, 2u8)) as i16),
            )),
            2i32,
        );
        if x < 0i32 {
            x = 0i32;
        }
        AddTextPrinterParameterized3(
            (((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(456))
            .cast::<u16>())
            .read()) as u8),
            3u8,
            ((x) as u8),
            6u8,
            ((((&raw const sNews_TextColorTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((crate::c::bf_read(
                        (((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(444)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(0),
                        0,
                        4,
                        false,
                    ) as u8) as i32) as isize
                        * 3,
                ))
            .cast::<u8>(),
            0i8,
            ((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(462))
            .cast::<u8>(),
        );
        {
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    AddTextPrinterParameterized3(
                        ((((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(456))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read()) as u8),
                        3u8,
                        0u8,
                        ((((16i32).wrapping_mul(((i) as i32))).wrapping_add(2i32)) as u8),
                        ((((&raw const sNews_TextColorTable).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                (((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(444)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(0),
                                4,
                                4,
                                false,
                            ) as u8) as i32) as isize
                                * 3,
                        ))
                        .cast::<u8>(),
                        0i8,
                        ((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(503))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 41))
                        .cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyWindowToVram(
            (((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(456))
            .cast::<u16>())
            .read()) as u8),
            3u8,
        );
        CopyWindowToVram(
            ((((((((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(456))
            .cast::<u16>())
            .wrapping_offset(1))
            .read()) as u8),
            3u8,
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateNewsScroll() {
    unsafe {
        let mut bgMove: u16 = ((crate::c::bf_read(
            (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(450),
            1,
            7,
            false,
        ) as u8) as u16);
        bgMove = ((((bgMove) as i32).wrapping_mul(256i32)) as u16);
        if (crate::c::bf_read(
            (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(451),
            0,
            1,
            false,
        ) as u8)
            != 0
        {
            ChangeBgY(2u8, ((bgMove) as i32), 1u8);
            ChangeBgY(3u8, ((bgMove) as i32), 1u8);
        } else {
            ChangeBgY(2u8, ((bgMove) as i32), 2u8);
            ChangeBgY(3u8, ((bgMove) as i32), 2u8);
        }
        crate::c::bf_write(
            (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(451),
            1,
            7,
            ((((crate::c::bf_read(
                (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(451),
                1,
                7,
                false,
            ) as u8) as i32)
                .wrapping_add(
                    ((crate::c::bf_read(
                        (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(450),
                        1,
                        7,
                        false,
                    ) as u8) as i32),
                )) as u8) as i32,
        );
        if ((crate::c::bf_read(
            (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(451),
            1,
            7,
            false,
        ) as u8) as i32)
            > 15i32
        {
            if (crate::c::bf_read(
                (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(451),
                0,
                1,
                false,
            ) as u8)
                != 0
            {
                let __p1 = (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(454)
                    .cast::<u16>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            } else {
                let __p2 = (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(454)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_sub(1));
            }
            crate::c::bf_write(
                (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(450),
                0,
                1,
                (0u8) as i32,
            );
            crate::c::bf_write(
                (((&raw mut sWonderNewsData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(451),
                1,
                7,
                (0u8) as i32,
            );
        }
    }
}
