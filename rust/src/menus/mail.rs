//! Translated from `src/mail.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sBgTemplates sWindowTemplates sTextColors sBgColors sMailGraphics sLineLayouts_Wide sMailLayouts_Wide sLineLayouts_Tall sMailLayouts_Tall
#[allow(unused_imports)]
use crate::data::mail::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMailRead: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSprites: u8;
    static mut gText_FromSpace: u8;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn ConvertEasyChatWordsToString(a0: *mut u8, a1: *mut u16, a2: u16, a3: u16) -> *mut u8;
    fn ConvertInternationalPlayerName(a0: *mut u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyEasyChatWord(a0: *mut u8, a1: u16) -> *mut u8;
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateMonIconNoPersonality(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut u8)>,
        a2: i16,
        a3: i16,
        a4: u8,
        a5: u32,
    ) -> u8;
    fn DeactivateAllTextPrinters();
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeAndDestroyMonIconSprite(a0: *mut u8);
    fn FreeMonIconPalette(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetIconSpeciesNoPersonality(a0: u16) -> u16;
    fn GetOverworldTextboxPalettePtr() -> *mut u16;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn LoadMonIconPalette(a0: u16);
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn MailSpeciesToSpecies(a0: u16, a1: *mut u16) -> u16;
    fn MenuHelpers_IsLinkActive() -> u8;
    fn Overworld_IsRecvQueueAtMax() -> u32;
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn RunTextPrinters();
    fn ScanlineEffect_Stop();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringLength(a0: *mut u8) -> u16;
    fn TransferPlttBuffer();
    fn UnsetBgTilemapBuffer(a0: u8);
    fn UpdatePaletteFade() -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReadMail(
    mail: *mut u8,
    exitCallback: Option<unsafe extern "C" fn()>,
    hasText: u8,
) {
    unsafe {
        let mut mail = mail;
        let mut exitCallback = exitCallback;
        let mut hasText = hasText;
        let mut buffer = crate::ffi::Align4([0u8; 4]);
        let mut species: u16 = 0u16;
        ((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(8748u32));
        ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(541))
            .write(2u8);
        ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(542))
            .write(1u8);
        ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(544)
            .cast::<Option<unsafe extern "C" fn(*mut u8, u16) -> *mut u8>>())
        .write(Some(CopyEasyChatWord));
        ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(548)
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u16, u16, u16) -> *mut u8>>())
        .write(Some(ConvertEasyChatWordsToString));
        if (((((((((((((((mail).wrapping_add(32).cast::<u16>()).read()) as i32) == 121i32)
            || (((((mail).wrapping_add(32).cast::<u16>()).read()) as i32) == 122i32))
            || (((((mail).wrapping_add(32).cast::<u16>()).read()) as i32) == 123i32))
            || (((((mail).wrapping_add(32).cast::<u16>()).read()) as i32) == 124i32))
            || (((((mail).wrapping_add(32).cast::<u16>()).read()) as i32) == 125i32))
            || (((((mail).wrapping_add(32).cast::<u16>()).read()) as i32) == 126i32))
            || (((((mail).wrapping_add(32).cast::<u16>()).read()) as i32) == 127i32))
            || (((((mail).wrapping_add(32).cast::<u16>()).read()) as i32) == 128i32))
            || (((((mail).wrapping_add(32).cast::<u16>()).read()) as i32) == 129i32))
            || (((((mail).wrapping_add(32).cast::<u16>()).read()) as i32) == 130i32))
            || (((((mail).wrapping_add(32).cast::<u16>()).read()) as i32) == 131i32))
            || (((((mail).wrapping_add(32).cast::<u16>()).read()) as i32) == 132i32)
        {
            ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(538))
                .write(
                    ((((((mail).wrapping_add(32).cast::<u16>()).read()) as i32)
                        .wrapping_sub(121i32)) as u8),
                );
        } else {
            ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(538))
                .write(0u8);
            hasText = 0u8;
        }
        'l1: {
            let __sw1 = ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(542))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 || !__matched {
                ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(552)
                    .cast::<*mut u8>())
                .write(
                    (((&raw const sMailLayouts_Wide).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(538))
                            .read()) as i32) as isize
                                * 12,
                        ),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(552)
                    .cast::<*mut u8>())
                .write(
                    (((&raw const sMailLayouts_Tall).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(538))
                            .read()) as i32) as isize
                                * 12,
                        ),
                );
                break 'l1;
            }
        }
        species = MailSpeciesToSpecies(
            ((mail).wrapping_add(30).cast::<u16>()).read(),
            (&raw mut buffer).cast::<u16>(),
        );
        if (((species) as i32) > 0i32) && (((species) as i32) < 412i32) {
            'l2: {
                let __sw2 = ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(538))
                .read()) as i32);
                let __matched = __sw2 == 6i32 || __sw2 == 9i32;
                if !__matched {
                    ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(539))
                    .write(0u8);
                    break 'l2;
                }
                if __sw2 == 6i32 {
                    ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(539))
                    .write(1u8);
                    break 'l2;
                }
                if __sw2 == 9i32 {
                    ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(539))
                    .write(2u8);
                    break 'l2;
                }
            }
        } else {
            ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(539))
                .write(0u8);
        }
        ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(532)
            .cast::<*mut u8>())
        .write(mail);
        ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(524)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(exitCallback);
        ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(536))
            .write(hasText);
        SetMainCallback2(Some(CB2_InitMailRead));
    }
}
pub(crate) unsafe extern "C" fn MailReadBuildGraphics() -> u8 {
    unsafe {
        let mut icon: u16 = 0u16;
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 13i32
                || __sw1 == 14i32
                || __sw1 == 15i32
                || __sw1 == 16i32
                || __sw1 == 17i32
                || __sw1 == 18i32;
            if __sw1 == 0i32 {
                SetVBlankCallback(None);
                ScanlineEffect_Stop();
                SetGpuReg(0u8, 0u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                'l2: loop {
                    'l3: {
                        {
                            let mut tmp: u16 = 0u16;
                            (&raw mut tmp).write_volatile(0u16);
                            'l4: loop {
                                'l5: {
                                    CpuSet(
                                        (&raw mut tmp).cast::<u8>(),
                                        ((117440512i32) as usize as *mut u8),
                                        ((16777216i32
                                            | (crate::c::div_i32(
                                                1024i32,
                                                crate::c::div_i32(16i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetPaletteFade();
                break 'l1;
            }
            if __sw1 == 3i32 {
                ResetTasks();
                break 'l1;
            }
            if __sw1 == 4i32 {
                ResetSpriteData();
                break 'l1;
            }
            if __sw1 == 5i32 {
                FreeAllSpritePalettes();
                ResetTempTileDataBuffers();
                SetGpuReg(16u8, 0u16);
                SetGpuReg(18u8, 0u16);
                SetGpuReg(20u8, 0u16);
                SetGpuReg(22u8, 0u16);
                SetGpuReg(26u8, 0u16);
                SetGpuReg(24u8, 0u16);
                SetGpuReg(28u8, 0u16);
                SetGpuReg(30u8, 0u16);
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                break 'l1;
            }
            if __sw1 == 6i32 {
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(12u32, 4u32)) as u8),
                );
                SetBgTilemapBuffer(
                    1u8,
                    ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(556))
                    .cast::<u8>(),
                );
                SetBgTilemapBuffer(
                    2u8,
                    ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4652))
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 7i32 {
                InitWindows(((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>());
                DeactivateAllTextPrinters();
                break 'l1;
            }
            if __sw1 == 8i32 {
                DecompressAndCopyTileDataToVram(
                    1u8,
                    ((((((&raw const sMailGraphics).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(538))
                            .read()) as i32) as isize
                                * 20,
                        ))
                    .wrapping_add(4)
                    .cast::<*mut u32>())
                    .read())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                break 'l1;
            }
            if __sw1 == 9i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 0u8;
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                FillBgTilemapBufferRect_Palette0(
                    0u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                FillBgTilemapBufferRect_Palette0(
                    2u8,
                    1u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                CopyToBgTilemapBuffer(
                    1u8,
                    ((((((&raw const sMailGraphics).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(538))
                            .read()) as i32) as isize
                                * 20,
                        ))
                    .wrapping_add(8)
                    .cast::<*mut u32>())
                    .read())
                    .cast::<u8>(),
                    0u16,
                    0u16,
                );
                break 'l1;
            }
            if __sw1 == 11i32 {
                CopyBgTilemapBufferToVram(0u8);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(2u8);
                break 'l1;
            }
            if __sw1 == 12i32 {
                LoadPalette(
                    (GetOverworldTextboxPalettePtr()).cast::<u8>(),
                    240u16,
                    32u16,
                );
                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(250))
                .write(
                    (((((&raw const sMailGraphics).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(538))
                            .read()) as i32) as isize
                                * 20,
                        ))
                    .wrapping_add(16)
                    .cast::<u16>())
                    .read(),
                );
                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(250))
                    .write(
                        (((((&raw const sMailGraphics).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(538))
                                .read()) as i32) as isize
                                    * 20,
                            ))
                        .wrapping_add(16)
                        .cast::<u16>())
                        .read(),
                    );
                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(251))
                .write(
                    (((((&raw const sMailGraphics).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(538))
                            .read()) as i32) as isize
                                * 20,
                        ))
                    .wrapping_add(18)
                    .cast::<u16>())
                    .read(),
                );
                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(251))
                    .write(
                        (((((&raw const sMailGraphics).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(538))
                                .read()) as i32) as isize
                                    * 20,
                            ))
                        .wrapping_add(18)
                        .cast::<u16>())
                        .read(),
                    );
                LoadPalette(
                    ((((((&raw const sMailGraphics).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(538))
                            .read()) as i32) as isize
                                * 20,
                        ))
                    .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                    0u16,
                    32u16,
                );
                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).wrapping_offset(10))
                    .write(
                        (((((&raw const sBgColors).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(8))
                                .read()) as i32) as isize
                                    * 4,
                            ))
                        .cast::<u16>())
                        .read(),
                    );
                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(10))
                    .write(
                        (((((&raw const sBgColors).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(8))
                                .read()) as i32) as isize
                                    * 4,
                            ))
                        .cast::<u16>())
                        .read(),
                    );
                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).wrapping_offset(11))
                    .write(
                        ((((((&raw const sBgColors).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(8))
                                .read()) as i32) as isize
                                    * 4,
                            ))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read(),
                    );
                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(11))
                    .write(
                        ((((((&raw const sBgColors).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(8))
                                .read()) as i32) as isize
                                    * 4,
                            ))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read(),
                    );
                break 'l1;
            }
            if __sw1 == 13i32 {
                if (((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(536))
                .read())
                    != 0
                {
                    BufferMailText();
                }
                break 'l1;
            }
            if __sw1 == 14i32 {
                if (((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(536))
                .read())
                    != 0
                {
                    PrintMailText();
                    RunTextPrinters();
                }
                break 'l1;
            }
            if __sw1 == 15i32 {
                if Overworld_IsRecvQueueAtMax() == 1u32 {
                    return 0u8;
                }
                break 'l1;
            }
            if __sw1 == 16i32 {
                SetVBlankCallback(Some(VBlankCB_MailRead));
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (1u16) as i32,
                );
                break 'l1;
            }
            if __sw1 == 17i32 {
                icon = GetIconSpeciesNoPersonality(
                    ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(532)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(30)
                    .cast::<u16>())
                    .read(),
                );
                'l6: {
                    let __sw2 = ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(539))
                    .read()) as i32);
                    if __sw2 == 1i32 {
                        LoadMonIconPalette(icon);
                        ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(540))
                        .write(CreateMonIconNoPersonality(
                            icon,
                            Some(SpriteCallbackDummy),
                            96i16,
                            128i16,
                            0u8,
                            0u32,
                        ));
                        break 'l6;
                    }
                    if __sw2 == 2i32 {
                        LoadMonIconPalette(icon);
                        ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(540))
                        .write(CreateMonIconNoPersonality(
                            icon,
                            Some(SpriteCallbackDummy),
                            40i16,
                            128i16,
                            0u8,
                            0u32,
                        ));
                        break 'l6;
                    }
                }
                break 'l1;
            }
            if __sw1 == 18i32 {
                SetGpuReg(0u8, 4160u16);
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(528)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(CB2_WaitForPaletteExitOnKeyPress));
                return 1u8;
            }
            if !__matched {
                return 0u8;
            }
        }
        let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
        (__p3).write(((__p3).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CB2_InitMailRead() {
    unsafe {
        'l1: loop {
            'l2: {
                if ((MailReadBuildGraphics()) as i32) == 1i32 {
                    SetMainCallback2(Some(CB2_MailRead));
                    break 'l1;
                }
            }
            if !(((MenuHelpers_IsLinkActive()) as i32) != 1i32) {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BufferMailText() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut numWords: u8 = 0u8;
        let mut ptr: *mut u8 = core::ptr::null_mut();
        numWords = 0u8;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < (((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(552)
                        .cast::<*mut u8>())
                    .read())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    ConvertEasyChatWordsToString(
                        (((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 64))
                        .cast::<u8>(),
                        ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(532)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u16>())
                        .wrapping_offset(((numWords) as i32) as isize),
                        ((crate::c::bf_read(
                            ((((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(552)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(0),
                            0,
                            2,
                            false,
                        ) as u8) as u16),
                        1u16,
                    );
                    numWords = ((((numWords) as i32).wrapping_add(
                        ((crate::c::bf_read(
                            ((((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(552)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(0),
                            0,
                            2,
                            false,
                        ) as u8) as i32),
                    )) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ptr = StringCopy(
            ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(512))
                .cast::<u8>(),
            ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(532)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(18))
            .cast::<u8>(),
        );
        if !((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(542))
            .read())
            != 0)
        {
            StringCopy(ptr, (&raw mut gText_FromSpace).cast::<u8>());
            ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(537))
                .write(
                    ((((((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(552)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(2))
                    .read()) as i32)
                        .wrapping_sub(
                            (((StringLength(
                                ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(512))
                                .cast::<u8>(),
                            )) as i32)
                                .wrapping_mul(8i32))
                            .wrapping_sub(96i32),
                        )) as u8),
                );
        } else {
            ConvertInternationalPlayerName(
                ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(512))
                    .cast::<u8>(),
            );
            ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(537))
                .write(
                    ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(552)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(2))
                    .read(),
                );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintMailText() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut signature = crate::ffi::Align4([0u8; 32]);
        let mut y: u8 = 0u8;
        let mut bufptr: *mut u8 = core::ptr::null_mut();
        let mut box_x: i32 = 0i32;
        let mut box_y: i32 = 0i32;
        y = 0u8;
        PutWindowTilemap(0u8);
        PutWindowTilemap(1u8);
        FillWindowPixelBuffer(0u8, 0u8);
        FillWindowPixelBuffer(1u8, 0u8);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < (((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(552)
                        .cast::<*mut u8>())
                    .read())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 64))
                    .cast::<u8>())
                    .read()) as i32)
                        == 255i32)
                        || (((((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 64))
                        .cast::<u8>())
                        .read()) as i32)
                            == 0i32)
                    {
                        break 'l2;
                    }
                    AddTextPrinterParameterized3(
                        0u8,
                        1u8,
                        ((((crate::c::bf_read(
                            ((((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(552)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(0),
                            2,
                            6,
                            false,
                        ) as u8) as i32)
                            .wrapping_add(
                                ((((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(552)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4))
                                .read()) as i32),
                            )) as u8),
                        ((((y) as i32).wrapping_add(
                            ((((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(552)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(3))
                            .read()) as i32),
                        )) as u8),
                        ((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                        0i8,
                        (((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 64))
                        .cast::<u8>(),
                    );
                    y = ((((y) as i32).wrapping_add(
                        (((((((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(552)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(1))
                        .read()) as i32),
                    )) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        bufptr = StringCopy(
            (&raw mut signature).cast::<u8>(),
            (&raw mut gText_FromSpace).cast::<u8>(),
        );
        StringCopy(
            bufptr,
            ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(512))
                .cast::<u8>(),
        );
        box_x = (GetStringCenterAlignXOffset(
            1i32,
            (&raw mut signature).cast::<u8>(),
            ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(537))
                .read()) as i32),
        ))
        .wrapping_add(104i32);
        box_y = ((((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(552)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1))
        .read()) as i32)
            .wrapping_add(88i32);
        AddTextPrinterParameterized3(
            0u8,
            1u8,
            ((box_x) as u8),
            ((box_y) as u8),
            ((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            (&raw mut signature).cast::<u8>(),
        );
        CopyWindowToVram(0u8, 3u8);
        CopyWindowToVram(1u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_MailRead() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn CB2_MailRead() {
    unsafe {
        if ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(539))
            .read()) as i32)
            != 0i32
        {
            AnimateSprites();
            BuildOamBuffer();
        }
        (((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(528)
            .cast::<Option<unsafe extern "C" fn()>>())
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn CB2_WaitForPaletteExitOnKeyPress() {
    unsafe {
        if !((UpdatePaletteFade()) != 0) {
            ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(528)
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(CB2_ExitOnKeyPress));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_ExitOnKeyPress() {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
            ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(528)
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(CB2_ExitMailReadFreeVars));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_ExitMailReadFreeVars() {
    unsafe {
        if !((UpdatePaletteFade()) != 0) {
            SetMainCallback2(
                ((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(524)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            );
            'l1: {
                let __sw1 = ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(539))
                .read()) as i32);
                if __sw1 == 1i32 || __sw1 == 2i32 {
                    FreeMonIconPalette(GetIconSpeciesNoPersonality(
                        ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(532)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(30)
                        .cast::<u16>())
                        .read(),
                    ));
                    FreeAndDestroyMonIconSprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(540))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                }
            }
            crate::c::memset(
                ((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read(),
                0i32,
                8748u32,
            );
            ResetPaletteFade();
            UnsetBgTilemapBuffer(0u8);
            UnsetBgTilemapBuffer(1u8);
            ResetBgsAndClearDma3BusyFlags(0u32);
            FreeAllWindowBuffers();
            {
                Free(((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).read());
                ((&raw mut sMailRead).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
            }
        }
    }
}
