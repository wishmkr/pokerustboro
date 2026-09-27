//! Translated from `src/naming_screen.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sPCIconOff_Gfx sPCIconOn_Gfx sKeyboard_Pal sRival_Pal sTransferredToPCMessages sText_AlphabetUpperLower sBgTemplates sWindowTemplates sKeyboardChars sPageColumnCounts sPageColumnXPos sNamingScreenTemplates sSubspriteTable_PageSwapFrame sSubspriteTable_PageSwapText sSubspriteTable_Button sSubspriteTable_PCIcon sSpriteTemplate_PageSwapFrame sSpriteTemplate_PageSwapButton sSpriteTemplate_PageSwapText sSpriteTemplate_BackButton sSpriteTemplate_OkButton sSpriteTemplate_Cursor sSpriteTemplate_InputArrow sSpriteTemplate_Underscore sSpriteTemplate_PCIcon sNamingScreenKeyboardText sSpriteSheets sSpritePalettes sPageToNextGfxId sPageToNextKeyboardId sPageToKeyboardId sPageSwapAnimStateFuncs sButtonKeyRoles sPageSwapSpriteFuncs sPageSwapPalTags sPageSwapGfxTags sIconFunctions sKeyboardKeyHandlers sInputFuncs sDrawTextEntryBoxFuncs sDrawGenderIconFuncs sGenderColors sTextColorStruct sFillValues sKeyboardTextColors sNextKeyboardPageTilemaps sPlayerNamingScreenTemplate sPCBoxNamingTemplate sMonNamingScreenTemplate sWaldaWordsScreenTemplate sNamingScreenTemplates sOam_8x8 sOam_16x16 sOam_32x16 sSubsprites_PageSwapFrame sSubsprites_PageSwapText sSubsprites_Button sSubsprites_PCIcon sSubspriteTable_PageSwapFrame sSubspriteTable_PageSwapText sSubspriteTable_Button sSubspriteTable_PCIcon sImageTable_PCIcon sAnim_Loop sAnim_CursorSquish sAnim_PCIcon sAnims_Loop sAnims_Cursor sAnims_PCIcon sSpriteTemplate_PageSwapFrame sSpriteTemplate_PageSwapButton sSpriteTemplate_PageSwapText sSpriteTemplate_BackButton sSpriteTemplate_OkButton sSpriteTemplate_Cursor sSpriteTemplate_InputArrow sSpriteTemplate_Underscore sSpriteTemplate_PCIcon sNamingScreenKeyboardText sSpriteSheets sSpritePalettes
#[allow(unused_imports)]
use crate::data::naming_screen::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sNamingScreen: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gKeyRepeatStartDelay: u8;
    static mut gMain: u8;
    static mut gNamingScreenBackground_Tilemap: u8;
    static mut gNamingScreenKeyboardLower_Tilemap: u8;
    static mut gNamingScreenKeyboardUpper_Tilemap: u8;
    static mut gNamingScreenMenu_Gfx: u8;
    static mut gNamingScreenMenu_Pal: u8;
    static mut gPaletteFade: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpeciesNames: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gTextFlags: u8;
    static mut gText_ExpandedPlaceholder_Empty: u8;
    static mut gText_FemaleSymbol: u8;
    static mut gText_MaleSymbol: u8;
    static mut gText_MoveOkBack: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddTextPrinterParameterized2(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn(*mut u8, u16)>,
        a5: u8,
        a6: u8,
        a7: u8,
    ) -> u16;
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
    fn Alloc(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_ReturnToFieldWithOpenMenu();
    fn CalculatePlayerPartyCount() -> u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateMonIcon(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut u8)>,
        a2: i16,
        a3: i16,
        a4: u8,
        a5: u32,
        a6: u32,
    ) -> u8;
    fn CreateObjectGraphicsSprite(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut u8)>,
        a2: i16,
        a3: i16,
        a4: u8,
    ) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn GetBoxNamePtr(a0: u8) -> *mut u8;
    fn GetGpuReg(a0: u8) -> u16;
    fn GetPCBoxToSendMon() -> u16;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetRivalAvatarGraphicsIdByStateIdAndGender(a0: u8, a1: u8) -> u8;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn GetTextWindowPalette(a0: u8) -> *mut u16;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitStandardTextBoxWindows();
    fn InitTextBoxGfxAndPrinters();
    fn IsDestinationBoxFull() -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut u8);
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadMonIconPalettes();
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut u8);
    fn LoadSpriteSheets(a0: *mut u8);
    fn MultiplyInvertedPaletteRGBComponents(a0: u16, a1: u8, a2: u8, a3: u8);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn RunTextPrinters();
    fn SeedRngAndSetTrainerId();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetSubspriteTables(a0: *mut u8, a1: *mut u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StartTimer1();
    fn StringAppendN(a0: *mut u8, a1: *mut u8, a2: u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopyN(a0: *mut u8, a1: *mut u8, a2: u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn VarGet(a0: u16) -> u16;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoNamingScreen(
    templateNum: u8,
    destBuffer: *mut u8,
    monSpecies: u16,
    monGender: u16,
    monPersonality: u32,
    returnCallback: Option<unsafe extern "C" fn()>,
) {
    unsafe {
        let mut templateNum = templateNum;
        let mut destBuffer = destBuffer;
        let mut monSpecies = monSpecies;
        let mut monGender = monGender;
        let mut monPersonality = monPersonality;
        let mut returnCallback = returnCallback;
        ((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).write(Alloc(7744u32));
        if !(!(((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).is_null()) {
            SetMainCallback2(returnCallback);
        } else {
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7724))
                .write(templateNum);
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7732)
                .cast::<u16>())
            .write(monSpecies);
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7734)
                .cast::<u16>())
            .write(monGender);
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7736)
                .cast::<u32>())
            .write(monPersonality);
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7728)
                .cast::<*mut u8>())
            .write(destBuffer);
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7740)
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(returnCallback);
            if ((templateNum) as i32) == 0i32 {
                StartTimer1();
            }
            SetMainCallback2(Some(CB2_LoadNamingScreen));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_LoadNamingScreen() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32;
            if __sw1 == 0i32 {
                ResetVHBlank();
                NamingScreen_Init();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                NamingScreen_InitBGs();
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetPaletteFade();
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                ResetSpriteData();
                FreeAllSpritePalettes();
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                ResetTasks();
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                LoadPalettes();
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                LoadGfx();
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                CreateSprites();
                UpdatePaletteFade();
                NamingScreen_ShowBgs();
                let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                CreateHelperTasks();
                CreateNamingScreenTask();
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn NamingScreen_Init() {
    unsafe {
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7696))
            .write(0u8);
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(7704)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(7706)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(7708)
            .cast::<u16>())
        .write(1u16);
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(7710)
            .cast::<u16>())
        .write(2u16);
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7712))
            .write(0u8);
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7713))
            .write(1u8);
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(7720)
            .cast::<*mut u8>())
        .write(
            ((((&raw const sNamingScreenTemplates)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(
                ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7724))
                .read()) as i32) as isize,
            ))
            .read(),
        );
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7714))
            .write(
                ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7720)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(4))
                .read(),
            );
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(7702)
            .cast::<u16>())
        .write(
            (((crate::c::div_i32(
                (240i32).wrapping_sub(
                    ((((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7720)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1))
                    .read()) as i32)
                        .wrapping_mul(8i32),
                ),
                2i32,
            ))
            .wrapping_add(6i32)) as u16),
        );
        if ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(7724))
        .read()) as i32)
            == 4i32
        {
            let __p1 = (((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7702)
                .cast::<u16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(11i32)) as u16));
        }
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7717))
            .write(((((&raw mut gKeyRepeatStartDelay).cast::<u16>()).read()) as u8));
        crate::c::memset(
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6144))
                .cast::<u8>(),
            255i32,
            16u32,
        );
        if ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(7720)
            .cast::<*mut u8>())
        .read())
        .read())
            != 0
        {
            StringCopy(
                ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6144))
                .cast::<u8>(),
                ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7728)
                    .cast::<*mut u8>())
                .read(),
            );
        }
        ((&raw mut gKeyRepeatStartDelay).cast::<u16>()).write(16u16);
    }
}
pub(crate) unsafe extern "C" fn SetSpritesVisible() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 64i32) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 68))
                        .wrapping_add(62),
                        0,
                        1,
                        false,
                    ) as u16)
                        != 0
                    {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 68))
                            .wrapping_add(62),
                            2,
                            1,
                            (0u16) as i32,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        SetCursorInvisibility(0u8);
    }
}
pub(crate) unsafe extern "C" fn NamingScreen_InitBGs() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            let mut _dest: *mut u8 = ((100663296i32) as usize as *mut u8);
            let mut _size: u32 = 98304u32;
            'l1: loop {
                if !((1i32) != 0) {
                    break 'l1;
                }
                'l2: loop {
                    'l3: {
                        {
                            let mut tmp: u16 = 0u16;
                            (&raw mut tmp).write_volatile(0u16);
                            'l4: loop {
                                'l5: {
                                    {
                                        let mut dmaRegs: *mut u32 =
                                            ((67109076i32) as usize as *mut u32);
                                        crate::c::volatile_write(
                                            dmaRegs,
                                            ((&raw mut tmp) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(1),
                                            ((_dest) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(2),
                                            (((-2130706432i32)
                                                | crate::c::div_i32(
                                                    4096i32,
                                                    crate::c::div_i32(16i32, 8i32),
                                                ))
                                                as u32),
                                        );
                                        let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                    }
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
                _dest = (_dest).wrapping_offset(4096);
                _size = (_size).wrapping_sub(4096u32);
                if _size <= 4096u32 {
                    'l6: loop {
                        'l7: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l8: loop {
                                    'l9: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((&raw mut tmp) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2164260864u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l8;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l6;
                        }
                    }
                    break 'l1;
                }
            }
        }
        'l10: loop {
            'l11: {
                {
                    let mut _dest: *mut u32 = ((117440512i32) as usize as *mut u8).cast::<u32>();
                    let mut _size: u32 = 1024u32;
                    'l12: loop {
                        'l13: {
                            {
                                let mut tmp: u32 = 0u32;
                                (&raw mut tmp).write_volatile(0u32);
                                'l14: loop {
                                    'l15: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((&raw mut tmp) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2231369728u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l14;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l12;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l10;
            }
        }
        'l16: loop {
            'l17: {
                {
                    let mut _dest: *mut u16 = ((83886080i32) as usize as *mut u8).cast::<u16>();
                    let mut _size: u32 = 1024u32;
                    'l18: loop {
                        'l19: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l20: loop {
                                    'l21: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((&raw mut tmp) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2164260864u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l20;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l18;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l16;
            }
        }
        SetGpuReg(0u8, 0u16);
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(16u32, 4u32)) as u8),
        );
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        ChangeBgX(1u8, 0i32, 0u8);
        ChangeBgY(1u8, 0i32, 0u8);
        ChangeBgX(2u8, 0i32, 0u8);
        ChangeBgY(2u8, 0i32, 0u8);
        ChangeBgX(3u8, 0i32, 0u8);
        ChangeBgY(3u8, 0i32, 0u8);
        InitStandardTextBoxWindows();
        InitTextBoxGfxAndPrinters();
        {
            i = 0u8;
            'l22: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l22;
                }
                'l23: {
                    ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7697))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((AddWindow(
                            (((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 8),
                        )) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        SetGpuReg(0u8, 4160u16);
        SetGpuReg(80u8, 1600u16);
        SetGpuReg(82u8, 2060u16);
        SetBgTilemapBuffer(
            1u8,
            (((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>(),
        );
        SetBgTilemapBuffer(
            2u8,
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2048))
                .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            3u8,
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4096))
                .cast::<u8>(),
        );
        FillBgTilemapBufferRect_Palette0(1u8, 0u16, 0u8, 0u8, 32u8, 32u8);
        FillBgTilemapBufferRect_Palette0(2u8, 0u16, 0u8, 0u8, 32u8, 32u8);
        FillBgTilemapBufferRect_Palette0(3u8, 0u16, 0u8, 0u8, 32u8, 32u8);
    }
}
pub(crate) unsafe extern "C" fn CreateNamingScreenTask() {
    unsafe {
        CreateTask(Some(Task_NamingScreen), 2u8);
        SetMainCallback2(Some(CB2_NamingScreen));
    }
}
pub(crate) unsafe extern "C" fn Task_NamingScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7696))
            .read()) as i32);
            if __sw1 == 0i32 {
                MainState_FadeIn();
                SetSpritesVisible();
                SetVBlank();
                break 'l1;
            }
            if __sw1 == 1i32 {
                MainState_WaitFadeIn();
                break 'l1;
            }
            if __sw1 == 2i32 {
                MainState_HandleInput();
                break 'l1;
            }
            if __sw1 == 3i32 {
                MainState_MoveToOKButton();
                MainState_HandleInput();
                break 'l1;
            }
            if __sw1 == 4i32 {
                MainState_StartPageSwap();
                break 'l1;
            }
            if __sw1 == 5i32 {
                MainState_WaitPageSwap();
                break 'l1;
            }
            if __sw1 == 6i32 {
                MainState_PressedOKButton();
                break 'l1;
            }
            if __sw1 == 7i32 {
                MainState_WaitSentToPCMessage();
                break 'l1;
            }
            if __sw1 == 8i32 {
                MainState_FadeOut();
                break 'l1;
            }
            if __sw1 == 9i32 {
                MainState_Exit();
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PageToNextGfxId(page: u8) -> u8 {
    unsafe {
        let mut page = page;
        return ((((&raw const sPageToNextGfxId).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((page) as i32) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn CurrentPageToNextKeyboardId() -> u8 {
    unsafe {
        return ((((&raw const sPageToNextKeyboardId).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7714))
                .read()) as i32) as isize,
            ))
        .read();
    }
}
pub(crate) unsafe extern "C" fn CurrentPageToKeyboardId() -> u8 {
    unsafe {
        return ((((&raw const sPageToKeyboardId).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7714))
                .read()) as i32) as isize,
            ))
        .read();
    }
}
pub(crate) unsafe extern "C" fn MainState_FadeIn() -> u8 {
    unsafe {
        DrawBgTilemap(
            3u8,
            (((&raw mut gNamingScreenBackground_Tilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
        );
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7714))
            .write(1u8);
        DrawBgTilemap(
            2u8,
            (((&raw mut gNamingScreenKeyboardLower_Tilemap).cast::<u32>()).cast::<u32>())
                .cast::<u8>(),
        );
        DrawBgTilemap(
            1u8,
            (((&raw mut gNamingScreenKeyboardUpper_Tilemap).cast::<u32>()).cast::<u32>())
                .cast::<u8>(),
        );
        PrintKeyboardKeys(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7697))
            .cast::<u8>())
            .wrapping_offset(1))
            .read(),
            0u8,
        );
        PrintKeyboardKeys(
            (((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7697))
            .cast::<u8>())
            .read(),
            1u8,
        );
        NamingScreen_Dummy(2u8, 0u8);
        NamingScreen_Dummy(1u8, 1u8);
        DrawTextEntry();
        DrawTextEntryBox();
        PrintControls();
        CopyBgTilemapBufferToVram(1u8);
        CopyBgTilemapBufferToVram(2u8);
        CopyBgTilemapBufferToVram(3u8);
        BlendPalettes(4294967295u32, 16u8, 0u16);
        BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
        let __p1 =
            (((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7696);
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn MainState_WaitFadeIn() -> u8 {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            SetInputState(1u8);
            SetCursorFlashing(1u8);
            let __p1 = (((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7696);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn MainState_HandleInput() -> u8 {
    unsafe {
        return HandleKeyboardEvent();
    }
}
pub(crate) unsafe extern "C" fn MainState_MoveToOKButton() -> u8 {
    unsafe {
        if (IsCursorAnimFinished()) != 0 {
            SetInputState(1u8);
            MoveCursorToOKButton();
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7696))
                .write(2u8);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn MainState_PressedOKButton() -> u8 {
    unsafe {
        SaveInputText();
        SetInputState(0u8);
        SetCursorFlashing(0u8);
        TryStartButtonFlash(3u8, 0u8, 1u8);
        if (((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(7724))
        .read()) as i32)
            == 2i32)
            && (((CalculatePlayerPartyCount()) as i32) >= 6i32)
        {
            DisplaySentToPCMessage();
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7696))
                .write(7u8);
            return 0u8;
        } else {
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7696))
                .write(8u8);
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn MainState_FadeOut() -> u8 {
    unsafe {
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        let __p1 =
            (((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7696);
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn MainState_Exit() -> u8 {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            if ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7724))
            .read()) as i32)
                == 0i32
            {
                SeedRngAndSetTrainerId();
            }
            SetMainCallback2(
                ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7740)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            );
            DestroyTask(FindTaskIdByFunc(Some(Task_NamingScreen)));
            FreeAllWindowBuffers();
            {
                Free(((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read());
                ((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>())
                    .write(core::ptr::null_mut());
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DisplaySentToPCMessage() {
    unsafe {
        let mut stringToDisplay: u8 = 0u8;
        if !((IsDestinationBoxFull()) != 0) {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                GetBoxNamePtr(((VarGet(16438u16)) as u8)),
            );
            StringCopy(
                (&raw mut gStringVar2).cast::<u8>(),
                ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7728)
                    .cast::<*mut u8>())
                .read(),
            );
        } else {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                GetBoxNamePtr(((VarGet(16438u16)) as u8)),
            );
            StringCopy(
                (&raw mut gStringVar2).cast::<u8>(),
                ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7728)
                    .cast::<*mut u8>())
                .read(),
            );
            StringCopy(
                (&raw mut gStringVar3).cast::<u8>(),
                GetBoxNamePtr(((GetPCBoxToSendMon()) as u8)),
            );
            stringToDisplay = 2u8;
        }
        if (FlagGet(2219u16)) != 0 {
            stringToDisplay = (stringToDisplay).wrapping_add(1);
        }
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            ((((&raw const sTransferredToPCMessages)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((stringToDisplay) as i32) as isize))
            .read(),
        );
        DrawDialogueFrame(0u8, 0u8);
        crate::c::bf_write(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            0,
            1,
            (1u8) as i32,
        );
        AddTextPrinterParameterized2(
            0u8,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            GetPlayerTextSpeedDelay(),
            None,
            2u8,
            1u8,
            3u8,
        );
        CopyWindowToVram(0u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn MainState_WaitSentToPCMessage() -> u8 {
    unsafe {
        RunTextPrinters();
        if (!((IsTextPrinterActive(0u8)) != 0))
            && (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0)
        {
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7696))
                .write(8u8);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn MainState_StartPageSwap() -> u8 {
    unsafe {
        SetInputState(0u8);
        StartPageSwapButtonAnim();
        StartPageSwapAnim();
        SetCursorInvisibility(1u8);
        TryStartButtonFlash(0u8, 0u8, 1u8);
        PlaySE(6u16);
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7696))
            .write(5u8);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn MainState_WaitPageSwap() -> u8 {
    unsafe {
        let mut cursorX: i16 = 0i16;
        let mut cursorY: i16 = 0i16;
        let mut onLastColumn: u32 = 0u32;
        if (IsPageSwapAnimNotInProgress()) != 0 {
            GetCursorPos(&raw mut cursorX, &raw mut cursorY);
            onLastColumn = ((((cursorX) as i32) == ((GetCurrentPageColumnCount()) as i32)) as u32);
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7696))
                .write(2u8);
            let __p1 = (((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7714);
            (__p1).write(((__p1).read()).wrapping_add(1));
            let __p2 = (((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7714);
            (__p2).write(((crate::c::rem_i32((((__p2).read()) as i32), 3i32)) as u8));
            if (onLastColumn) != 0 {
                cursorX = ((GetCurrentPageColumnCount()) as i16);
            } else {
                if ((cursorX) as i32) >= ((GetCurrentPageColumnCount()) as i32) {
                    cursorX = ((((GetCurrentPageColumnCount()) as i32).wrapping_sub(1i32)) as i16);
                }
            }
            SetCursorPos(cursorX, cursorY);
            DrawKeyboardPageOnDeck();
            SetInputState(1u8);
            SetCursorInvisibility(0u8);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StartPageSwapAnim() {
    unsafe {
        let mut taskId: u8 = 0u8;
        taskId = CreateTask(Some(Task_HandlePageSwapAnim), 0u8);
        Task_HandlePageSwapAnim(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_HandlePageSwapAnim(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !((((((((&raw const sPageSwapAnimStateFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) as i32)
                != 0i32)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsPageSwapAnimNotInProgress() -> u8 {
    unsafe {
        if ((FindTaskIdByFunc(Some(Task_HandlePageSwapAnim))) as i32) == 255i32 {
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
pub(crate) unsafe extern "C" fn PageSwapAnimState_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(7704)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(7706)
            .cast::<u16>())
        .write(0u16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PageSwapAnimState_1(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut vOffsets = crate::ffi::Align4([0u8; 8]);
        (&raw mut vOffsets)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(
                (((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7706)
                    .cast::<u16>(),
            );
        (&raw mut vOffsets)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<*mut u16>()
            .write(
                (((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7704)
                    .cast::<u16>(),
            );
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        ((((&raw mut vOffsets).cast::<*mut u16>()).wrapping_offset(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7712))
            .read()) as i32) as isize,
        ))
        .read())
        .write(
            ((Sin(
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
                40i16,
            )) as u16),
        );
        ((((&raw mut vOffsets).cast::<*mut u16>()).wrapping_offset(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7713))
            .read()) as i32) as isize,
        ))
        .read())
        .write(
            ((Sin(
                ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    .wrapping_add(128i32)
                    & 255i32) as i16),
                40i16,
            )) as u16),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) >= 64i32
        {
            let mut temp: u8 = ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>())
                .read())
            .wrapping_add(7708)
            .cast::<u16>())
            .read()) as u8);
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7708)
                .cast::<u16>())
            .write(
                ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7710)
                    .cast::<u16>())
                .read(),
            );
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7710)
                .cast::<u16>())
            .write(((temp) as u16));
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PageSwapAnimState_2(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut vOffsets = crate::ffi::Align4([0u8; 8]);
        (&raw mut vOffsets)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(
                (((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7706)
                    .cast::<u16>(),
            );
        (&raw mut vOffsets)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<*mut u16>()
            .write(
                (((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7704)
                    .cast::<u16>(),
            );
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        ((((&raw mut vOffsets).cast::<*mut u16>()).wrapping_offset(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7712))
            .read()) as i32) as isize,
        ))
        .read())
        .write(
            ((Sin(
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
                40i16,
            )) as u16),
        );
        ((((&raw mut vOffsets).cast::<*mut u16>()).wrapping_offset(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7713))
            .read()) as i32) as isize,
        ))
        .read())
        .write(
            ((Sin(
                ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    .wrapping_add(128i32)
                    & 255i32) as i16),
                40i16,
            )) as u16),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) >= 128i32
        {
            let mut temp: u8 = ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7712))
            .read();
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7712))
                .write(
                    ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7713))
                    .read(),
                );
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7713))
                .write(temp);
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PageSwapAnimState_Done(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        DestroyTask(FindTaskIdByFunc(Some(Task_HandlePageSwapAnim)));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CreateButtonFlashTask() {
    unsafe {
        let mut taskId: u8 = 0u8;
        taskId = CreateTask(Some(Task_UpdateButtonFlash), 3u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(3i16);
    }
}
pub(crate) unsafe extern "C" fn TryStartButtonFlash(
    button: u8,
    keepFlashing: u8,
    interruptCurFlash: u8,
) {
    unsafe {
        let mut button = button;
        let mut keepFlashing = keepFlashing;
        let mut interruptCurFlash = interruptCurFlash;
        let mut task: *mut u8 = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((FindTaskIdByFunc(Some(Task_UpdateButtonFlash))) as i32) as isize * 40,
        );
        if (((button) as i32) == (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32))
            && (!((interruptCurFlash) != 0))
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                .write(((keepFlashing) as i16));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            return;
        }
        if ((((button) as i32) == 3i32)
            && (!((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) != 0)))
            && (!((interruptCurFlash) != 0))
        {
            return;
        }
        if (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) != 3i32 {
            RestoreButtonColor((((((task).wrapping_add(8)).cast::<i16>()).read()) as u8));
        }
        StartButtonFlash(task, button, keepFlashing);
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateButtonFlash(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if ((((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) == 3i32)
            || (!((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) != 0))
        {
            return;
        }
        MultiplyInvertedPaletteRGBComponents(
            GetButtonPalOffset((((((task).wrapping_add(8)).cast::<i16>()).read()) as u8)),
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) != 0)
            && (({
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) != 0)
        {
            return;
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(2i16);
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32) >= 0i32 {
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                < 14i32
            {
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
            } else {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(16i16);
                let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                (__p5).write(((__p5).read()).wrapping_add(1));
            }
        } else {
            let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
            (__p6).write(
                (((((__p6).read()) as i32).wrapping_add(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
                )) as i16),
            );
            let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
            (__p7).write(
                (((((__p7).read()) as i32).wrapping_add(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
                )) as i16),
            );
        }
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            == 16i32)
            && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                == 22i32)
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write((-4i16));
        } else {
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                == 0i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read());
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(2i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetButtonPalOffset(button: u8) -> u16 {
    unsafe {
        let mut button = button;
        let mut palOffsets = crate::ffi::Align4([0u8; 8]);
        (&raw mut palOffsets)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(
                ((((256i32)
                    .wrapping_add(((IndexOfSpritePaletteTag(4u16)) as i32).wrapping_mul(16i32)))
                .wrapping_add(14i32)) as u16),
            );
        (&raw mut palOffsets)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<u16>()
            .write(
                ((((256i32)
                    .wrapping_add(((IndexOfSpritePaletteTag(6u16)) as i32).wrapping_mul(16i32)))
                .wrapping_add(14i32)) as u16),
            );
        (&raw mut palOffsets)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(
                ((((256i32)
                    .wrapping_add(((IndexOfSpritePaletteTag(7u16)) as i32).wrapping_mul(16i32)))
                .wrapping_add(14i32)) as u16),
            );
        (&raw mut palOffsets)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<u16>()
            .write(
                ((((256i32)
                    .wrapping_add(((IndexOfSpritePaletteTag(7u16)) as i32).wrapping_mul(16i32)))
                .wrapping_add(1i32)) as u16),
            );
        return (((&raw mut palOffsets).cast::<u16>()).wrapping_offset(((button) as i32) as isize))
            .read();
    }
}
pub(crate) unsafe extern "C" fn RestoreButtonColor(button: u8) {
    unsafe {
        let mut button = button;
        let mut index: u16 = GetButtonPalOffset(button);
        ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
            .wrapping_offset(((index) as i32) as isize))
        .write(
            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(((index) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn StartButtonFlash(task: *mut u8, button: u8, keepFlashing: u8) {
    unsafe {
        let mut task = task;
        let mut button = button;
        let mut keepFlashing = keepFlashing;
        (((task).wrapping_add(8)).cast::<i16>()).write(((button) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
            .write(((keepFlashing) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(1i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(4i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(2i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(4i16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Cursor(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            StartSpriteAnim(sprite, 0u8);
        }
        crate::c::bf_write(
            (sprite).wrapping_add(62),
            2,
            1,
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                & 255i32) as u16) as i32,
        );
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            == ((GetCurrentPageColumnCount()) as i32)
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
        if ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) != 0)
            || (!((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                as i32)
                & 65280i32)
                != 0)))
            || ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
                != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)))
            || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32))
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(2i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(2i16);
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p1).write(((__p1).read()).wrapping_sub(1));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            == 0i32
        {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32),
                )) as i16),
            );
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                == 16i32)
                || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    == 0i32)
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        .wrapping_neg()) as i16),
                );
            }
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(2i16);
        }
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            & 65280i32)
            != 0
        {
            let mut gb: i8 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i8);
            let mut r: i8 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                .read()) as i32)
                >> 1) as i8);
            let mut index: u16 = ((((256i32)
                .wrapping_add(((IndexOfSpritePaletteTag(5u16)) as i32).wrapping_mul(16i32)))
            .wrapping_add(1i32)) as u16);
            MultiplyInvertedPaletteRGBComponents(index, ((r) as u8), ((gb) as u8), ((gb) as u8));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_InputArrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut x = crate::ffi::Align4([0u8; 8]);
        (&raw mut x)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i16>()
            .write(0i16);
        (&raw mut x)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<i16>()
            .write((-4i16));
        (&raw mut x)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<i16>()
            .write((-2i16));
        (&raw mut x)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<i16>()
            .write((-1i16));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32)
            || ((({
                let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32)
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(8i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write((((if ((crate::c::div_u32(8u32, 2u32) & (crate::c::div_u32(8u32, 2u32)).wrapping_sub(1u32))) != 0 { crate::c::rem_u32((((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32))).wrapping_add(1i32)) as u32)), crate::c::div_u32(8u32, 2u32)) } else { ((((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32))).wrapping_add(1i32)) as u32)) & (crate::c::div_u32(8u32, 2u32)).wrapping_sub(1u32)) })) as i16));
        }
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            (((&raw mut x).cast::<i16>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    as isize,
            ))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Underscore(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut y = crate::ffi::Align4([0u8; 8]);
        (&raw mut y)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i16>()
            .write(2i16);
        (&raw mut y)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<i16>()
            .write(3i16);
        (&raw mut y)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<i16>()
            .write(2i16);
        (&raw mut y)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<i16>()
            .write(1i16);
        let mut pos: u8 = 0u8;
        pos = GetTextEntryPosition();
        if ((pos) as i32) != ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8) as i32) {
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        } else {
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                (((&raw mut y).cast::<i16>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32) as isize,
                ))
                .read(),
            );
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > 8i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                    ((if (crate::c::div_u32(8u32, 2u32)
                        & (crate::c::div_u32(8u32, 2u32)).wrapping_sub(1u32))
                        != 0
                    {
                        crate::c::rem_u32(
                            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                .read()) as i32)
                                .wrapping_add(1i32)) as u32),
                            crate::c::div_u32(8u32, 2u32),
                        )
                    } else {
                        (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                            .read()) as i32)
                            .wrapping_add(1i32)) as u32)
                            & (crate::c::div_u32(8u32, 2u32)).wrapping_sub(1u32))
                    }) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateSprites() {
    unsafe {
        CreateCursorSprite();
        CreatePageSwapButtonSprites();
        CreateBackOkSprites();
        CreateTextEntrySprites();
        CreateInputTargetIcon();
    }
}
pub(crate) unsafe extern "C" fn CreateCursorSprite() {
    unsafe {
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7715))
            .write(CreateSprite(
                (&raw const sSpriteTemplate_Cursor).cast::<u8>().cast_mut(),
                38i16,
                88i16,
                1u8,
            ));
        SetCursorInvisibility(1u8);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7715))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7715))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(1),
            2,
            2,
            (1u32) as i32,
        );
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7715))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(1i16);
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7715))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(2i16);
        SetCursorPos(0i16, 0i16);
    }
}
pub(crate) unsafe extern "C" fn SetCursorPos(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut cursorSprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7715))
            .read()) as i32) as isize
                * 68,
        );
        if ((x) as i32)
            < ((((((&raw const sPageColumnCounts).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((CurrentPageToKeyboardId()) as i32) as isize))
            .read()) as i32)
        {
            ((cursorSprite).wrapping_add(32).cast::<i16>()).write(
                ((((((((((&raw const sPageColumnXPos).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((CurrentPageToKeyboardId()) as i32) as isize * 8))
                .cast::<u8>())
                .wrapping_offset(((x) as i32) as isize))
                .read()) as i32)
                    .wrapping_add(38i32)) as i16),
            );
        } else {
            ((cursorSprite).wrapping_add(32).cast::<i16>()).write(0i16);
        }
        ((cursorSprite).wrapping_add(34).cast::<i16>())
            .write((((((y) as i32).wrapping_mul(16i32)).wrapping_add(88i32)) as i16));
        ((((cursorSprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write((((cursorSprite).wrapping_add(46)).cast::<i16>()).read());
        ((((cursorSprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((((cursorSprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read());
        (((cursorSprite).wrapping_add(46)).cast::<i16>()).write(x);
        ((((cursorSprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(y);
    }
}
pub(crate) unsafe extern "C" fn GetCursorPos(x: *mut i16, y: *mut i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut cursorSprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7715))
            .read()) as i32) as isize
                * 68,
        );
        (x).write((((cursorSprite).wrapping_add(46)).cast::<i16>()).read());
        (y).write(((((cursorSprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read());
    }
}
pub(crate) unsafe extern "C" fn MoveCursorToOKButton() {
    unsafe {
        SetCursorPos(((GetCurrentPageColumnCount()) as i16), 2i16);
    }
}
pub(crate) unsafe extern "C" fn SetCursorInvisibility(invisible: u8) {
    unsafe {
        let mut invisible = invisible;
        let __p1 = (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7715))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4);
        (__p1).write((((((__p1).read()) as i32) & 65280i32) as i16));
        let __p2 = (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7715))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4);
        (__p2).write((((((__p2).read()) as i32) | ((invisible) as i32)) as i16));
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7715))
                .read()) as i32) as isize
                    * 68,
            ),
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn SetCursorFlashing(flashing: u8) {
    unsafe {
        let mut flashing = flashing;
        let __p1 = (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7715))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4);
        (__p1).write((((((__p1).read()) as i32) & 255i32) as i16));
        let __p2 = (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7715))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4);
        (__p2).write((((((__p2).read()) as i32) | (((flashing) as i32) << 8)) as i16));
    }
}
pub(crate) unsafe extern "C" fn SquishCursor() {
    unsafe {
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7715))
                .read()) as i32) as isize
                    * 68,
            ),
            1u8,
        );
    }
}
pub(crate) unsafe extern "C" fn IsCursorAnimFinished() -> u8 {
    unsafe {
        return ((crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7715))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(63),
            4,
            1,
            false,
        ) as u16) as u8);
    }
}
pub(crate) unsafe extern "C" fn GetKeyRoleAtCursorPos() -> u8 {
    unsafe {
        let mut cursorX: i16 = 0i16;
        let mut cursorY: i16 = 0i16;
        GetCursorPos(&raw mut cursorX, &raw mut cursorY);
        if ((cursorX) as i32) < ((GetCurrentPageColumnCount()) as i32) {
            return 0u8;
        } else {
            return ((((&raw const sButtonKeyRoles).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((cursorY) as i32) as isize))
            .read();
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetCurrentPageColumnCount() -> u8 {
    unsafe {
        return ((((&raw const sPageColumnCounts).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((CurrentPageToKeyboardId()) as i32) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn CreatePageSwapButtonSprites() {
    unsafe {
        let mut frameSpriteId: u8 = 0u8;
        let mut textSpriteId: u8 = 0u8;
        let mut buttonSpriteId: u8 = 0u8;
        frameSpriteId = CreateSprite(
            (&raw const sSpriteTemplate_PageSwapFrame)
                .cast::<u8>()
                .cast_mut(),
            204i16,
            88i16,
            0u8,
        );
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7716))
            .write(frameSpriteId);
        SetSubspriteTables(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((frameSpriteId) as i32) as isize * 68),
            ((&raw const sSubspriteTable_PageSwapFrame)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((frameSpriteId) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        textSpriteId = CreateSprite(
            (&raw const sSpriteTemplate_PageSwapText)
                .cast::<u8>()
                .cast_mut(),
            204i16,
            84i16,
            1u8,
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((frameSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((textSpriteId) as i16));
        SetSubspriteTables(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((textSpriteId) as i32) as isize * 68),
            ((&raw const sSubspriteTable_PageSwapText)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((textSpriteId) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        buttonSpriteId = CreateSprite(
            (&raw const sSpriteTemplate_PageSwapButton)
                .cast::<u8>()
                .cast_mut(),
            204i16,
            83i16,
            2u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((buttonSpriteId) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((frameSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((buttonSpriteId) as i16));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((buttonSpriteId) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn StartPageSwapButtonAnim() {
    unsafe {
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7716))
            .read()) as i32) as isize
                * 68,
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(2i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7714))
            .read()) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PageSwap(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: loop {
            if !(((((((&raw const sPageSwapSpriteFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(sprite))
                != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PageSwapSprite_Init(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        let mut text: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                as isize
                * 68,
        );
        let mut button: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                as isize
                * 68,
        );
        SetPageSwapButtonGfx(
            PageToNextGfxId(
                ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7714))
                .read(),
            ),
            text,
            button,
        );
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PageSwapSprite_Idle(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PageSwapSprite_SlideOff(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        let mut text: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                as isize
                * 68,
        );
        let mut button: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                as isize
                * 68,
        );
        let __p1 = (text).wrapping_add(38).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((text).wrapping_add(38).cast::<i16>()).read()) as i32) > 7i32 {
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
            ((text).wrapping_add(38).cast::<i16>()).write((-4i16));
            crate::c::bf_write((text).wrapping_add(62), 2, 1, (1u16) as i32);
            SetPageSwapButtonGfx(
                PageToNextGfxId(
                    ((crate::c::rem_i32(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as u8) as i32)
                            .wrapping_add(1i32),
                        3i32,
                    )) as u8),
                ),
                text,
                button,
            );
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PageSwapSprite_SlideOn(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        let mut text: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                as isize
                * 68,
        );
        crate::c::bf_write((text).wrapping_add(62), 2, 1, (0u16) as i32);
        let __p1 = (text).wrapping_add(38).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((text).wrapping_add(38).cast::<i16>()).read()) as i32) >= 0i32 {
            ((text).wrapping_add(38).cast::<i16>()).write(0i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SetPageSwapButtonGfx(page: u8, text: *mut u8, button: *mut u8) {
    unsafe {
        let mut page = page;
        let mut text = text;
        let mut button = button;
        crate::c::bf_write(
            (button).wrapping_add(5),
            4,
            4,
            ((IndexOfSpritePaletteTag(
                ((((&raw const sPageSwapPalTags)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(((page) as i32) as isize))
                .read(),
            )) as u16) as i32,
        );
        ((text).wrapping_add(64).cast::<u16>()).write(GetSpriteTileStartByTag(
            ((((&raw const sPageSwapGfxTags)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(((page) as i32) as isize))
            .read(),
        ));
        crate::c::bf_write((text).wrapping_add(66), 0, 6, (page) as i32);
    }
}
pub(crate) unsafe extern "C" fn CreateBackOkSprites() {
    unsafe {
        let mut spriteId: u8 = 0u8;
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_BackButton)
                .cast::<u8>()
                .cast_mut(),
            204i16,
            116i16,
            0u8,
        );
        SetSubspriteTables(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            ((&raw const sSubspriteTable_Button).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_OkButton)
                .cast::<u8>()
                .cast_mut(),
            204i16,
            140i16,
            0u8,
        );
        SetSubspriteTables(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            ((&raw const sSubspriteTable_Button).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn CreateTextEntrySprites() {
    unsafe {
        let mut spriteId: u8 = 0u8;
        let mut xPos: i16 = 0i16;
        let mut i: u8 = 0u8;
        xPos = ((((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(7702)
            .cast::<u16>())
        .read()) as i32)
            .wrapping_sub(5i32)) as i16);
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_InputArrow)
                .cast::<u8>()
                .cast_mut(),
            xPos,
            56i16,
            0u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            (3u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        xPos = ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(7702)
            .cast::<u16>())
        .read()) as i16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7720)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (&raw const sSpriteTemplate_Underscore)
                            .cast::<u8>()
                            .cast_mut(),
                        ((((xPos) as i32).wrapping_add(3i32)) as i16),
                        60i16,
                        0u8,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(5),
                        2,
                        2,
                        (3u16) as i32,
                    );
                    (((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(((i) as i16));
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
                xPos = ((((xPos) as i32).wrapping_add(8i32)) as i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateInputTargetIcon() {
    unsafe {
        (((((&raw const sIconFunctions)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            ((((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7720)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2))
            .read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn NamingScreen_NoIcon() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn NamingScreen_CreatePlayerIcon() {
    unsafe {
        let mut rivalGfxId: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        rivalGfxId = GetRivalAvatarGraphicsIdByStateIdAndGender(
            0u8,
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7732)
                .cast::<u16>())
            .read()) as u8),
        );
        spriteId = CreateObjectGraphicsSprite(
            ((rivalGfxId) as u16),
            Some(SpriteCallbackDummy),
            56i16,
            37i16,
            0u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            (3u16) as i32,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            4u8,
        );
    }
}
pub(crate) unsafe extern "C" fn NamingScreen_CreatePCIcon() {
    unsafe {
        let mut spriteId: u8 = 0u8;
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_PCIcon).cast::<u8>().cast_mut(),
            56i16,
            41i16,
            0u8,
        );
        SetSubspriteTables(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            ((&raw const sSubspriteTable_PCIcon).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            (3u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn NamingScreen_CreateMonIcon() {
    unsafe {
        let mut spriteId: u8 = 0u8;
        LoadMonIconPalettes();
        spriteId = CreateMonIcon(
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7732)
                .cast::<u16>())
            .read(),
            Some(SpriteCallbackDummy),
            56i16,
            40i16,
            0u8,
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7736)
                .cast::<u32>())
            .read(),
            1u32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            (3u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn NamingScreen_CreateWaldaDadIcon() {
    unsafe {
        let mut spriteId: u8 = 0u8;
        spriteId = CreateObjectGraphicsSprite(19u16, Some(SpriteCallbackDummy), 56i16, 37i16, 0u8);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            (3u16) as i32,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            4u8,
        );
    }
}
pub(crate) unsafe extern "C" fn HandleKeyboardEvent() -> u8 {
    unsafe {
        let mut input: u8 = GetInputEvent();
        let mut keyRole: u8 = GetKeyRoleAtCursorPos();
        if ((input) as i32) == 8i32 {
            return SwapKeyboardPage();
        } else {
            if ((input) as i32) == 6i32 {
                DeleteTextCharacter();
                return 0u8;
            } else {
                if ((input) as i32) == 9i32 {
                    MoveCursorToOKButton();
                    return 0u8;
                } else {
                    return (((((&raw const sKeyboardKeyHandlers)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<Option<unsafe extern "C" fn(u8) -> u8>>())
                    .cast::<Option<unsafe extern "C" fn(u8) -> u8>>())
                    .wrapping_offset(((keyRole) as i32) as isize))
                    .read())
                    .unwrap_unchecked()(input);
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn KeyboardKeyHandler_Character(input: u8) -> u8 {
    unsafe {
        let mut input = input;
        TryStartButtonFlash(3u8, 0u8, 0u8);
        if ((input) as i32) == 5i32 {
            let mut textFull: u8 = AddTextCharacter();
            SquishCursor();
            if (textFull) != 0 {
                SetInputState(2u8);
                ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7696))
                .write(3u8);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn KeyboardKeyHandler_Page(input: u8) -> u8 {
    unsafe {
        let mut input = input;
        TryStartButtonFlash(0u8, 1u8, 0u8);
        if ((input) as i32) == 5i32 {
            return SwapKeyboardPage();
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn KeyboardKeyHandler_Backspace(input: u8) -> u8 {
    unsafe {
        let mut input = input;
        TryStartButtonFlash(1u8, 1u8, 0u8);
        if ((input) as i32) == 5i32 {
            DeleteTextCharacter();
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn KeyboardKeyHandler_OK(input: u8) -> u8 {
    unsafe {
        let mut input = input;
        TryStartButtonFlash(2u8, 1u8, 0u8);
        if ((input) as i32) == 5i32 {
            PlaySE(5u16);
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7696))
                .write(6u8);
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
pub(crate) unsafe extern "C" fn SwapKeyboardPage() -> u8 {
    unsafe {
        ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7696))
            .write(4u8);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn CreateInputHandlerTask() {
    unsafe {
        CreateTask(Some(Task_HandleInput), 1u8);
    }
}
pub(crate) unsafe extern "C" fn GetInputEvent() -> u8 {
    unsafe {
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_HandleInput));
        return ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u8);
    }
}
pub(crate) unsafe extern "C" fn SetInputState(state: u8) {
    unsafe {
        let mut state = state;
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_HandleInput));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((state) as i16));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw const sInputFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .wrapping_offset(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()(
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
        );
    }
}
pub(crate) unsafe extern "C" fn Input_Disabled(task: *mut u8) {
    unsafe {
        let mut task = task;
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
    }
}
pub(crate) unsafe extern "C" fn Input_Enabled(task: *mut u8) {
    unsafe {
        let mut task = task;
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(5i16);
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(6i16);
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 4i32)
                    != 0
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(8i16);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 8i32)
                        != 0
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(9i16);
                    } else {
                        HandleDpadMovement(task);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Input_Override(task: *mut u8) {
    unsafe {
        let mut task = task;
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
    }
}
pub(crate) unsafe extern "C" fn HandleDpadMovement(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut sDpadDeltaX = crate::ffi::Align4([0u8; 10]);
        (&raw mut sDpadDeltaX)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i16>()
            .write(0i16);
        (&raw mut sDpadDeltaX)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<i16>()
            .write(0i16);
        (&raw mut sDpadDeltaX)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<i16>()
            .write(0i16);
        (&raw mut sDpadDeltaX)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<i16>()
            .write((-1i16));
        (&raw mut sDpadDeltaX)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<i16>()
            .write(1i16);
        let mut sDpadDeltaY = crate::ffi::Align4([0u8; 10]);
        (&raw mut sDpadDeltaY)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i16>()
            .write(0i16);
        (&raw mut sDpadDeltaY)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<i16>()
            .write((-1i16));
        (&raw mut sDpadDeltaY)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<i16>()
            .write(1i16);
        (&raw mut sDpadDeltaY)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<i16>()
            .write(0i16);
        (&raw mut sDpadDeltaY)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<i16>()
            .write(0i16);
        let mut sKeyRowToButtonRow = crate::ffi::Align4([0u8; 8]);
        (&raw mut sKeyRowToButtonRow)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i16>()
            .write(0i16);
        (&raw mut sKeyRowToButtonRow)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<i16>()
            .write(1i16);
        (&raw mut sKeyRowToButtonRow)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<i16>()
            .write(1i16);
        (&raw mut sKeyRowToButtonRow)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<i16>()
            .write(2i16);
        let mut sButtonRowToKeyRow = crate::ffi::Align4([0u8; 6]);
        (&raw mut sButtonRowToKeyRow)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i16>()
            .write(0i16);
        (&raw mut sButtonRowToKeyRow)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<i16>()
            .write(0i16);
        (&raw mut sButtonRowToKeyRow)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<i16>()
            .write(3i16);
        let mut cursorX: i16 = 0i16;
        let mut cursorY: i16 = 0i16;
        let mut input: u16 = 0u16;
        let mut prevCursorX: i16 = 0i16;
        GetCursorPos(&raw mut cursorX, &raw mut cursorY);
        input = 0u16;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0
        {
            input = 1u16;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0
        {
            input = 2u16;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0
        {
            input = 3u16;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0
        {
            input = 4u16;
        }
        prevCursorX = cursorX;
        cursorX = ((((cursorX) as i32).wrapping_add(
            (((((&raw mut sDpadDeltaX).cast::<i16>()).wrapping_offset(((input) as i32) as isize))
                .read()) as i32),
        )) as i16);
        cursorY = ((((cursorY) as i32).wrapping_add(
            (((((&raw mut sDpadDeltaY).cast::<i16>()).wrapping_offset(((input) as i32) as isize))
                .read()) as i32),
        )) as i16);
        if ((cursorX) as i32) < 0i32 {
            cursorX = ((GetCurrentPageColumnCount()) as i16);
        }
        if ((cursorX) as i32) > ((GetCurrentPageColumnCount()) as i32) {
            cursorX = 0i16;
        }
        if (((((&raw mut sDpadDeltaX).cast::<i16>()).wrapping_offset(((input) as i32) as isize))
            .read()) as i32)
            != 0i32
        {
            if ((cursorX) as i32) == ((GetCurrentPageColumnCount()) as i32) {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(cursorY);
                cursorY = (((&raw mut sKeyRowToButtonRow).cast::<i16>())
                    .wrapping_offset(((cursorY) as i32) as isize))
                .read();
            } else {
                if ((prevCursorX) as i32) == ((GetCurrentPageColumnCount()) as i32) {
                    if ((cursorY) as i32) == crate::c::div_i32(3i32, 2i32) {
                        cursorY =
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read();
                    } else {
                        cursorY = (((&raw mut sButtonRowToKeyRow).cast::<i16>())
                            .wrapping_offset(((cursorY) as i32) as isize))
                        .read();
                    }
                }
            }
        }
        if ((cursorX) as i32) == ((GetCurrentPageColumnCount()) as i32) {
            if ((cursorY) as i32) < 0i32 {
                cursorY = 2i16;
            }
            if ((cursorY) as i32) >= 3i32 {
                cursorY = 0i16;
            }
            if ((cursorY) as i32) == 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            } else {
                if ((cursorY) as i32) == 2i32 {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(2i16);
                }
            }
        } else {
            if ((cursorY) as i32) < 0i32 {
                cursorY = 3i16;
            }
            if ((cursorY) as i32) > 3i32 {
                cursorY = 0i16;
            }
        }
        SetCursorPos(cursorX, cursorY);
    }
}
pub(crate) unsafe extern "C" fn DrawNormalTextEntryBox() {
    unsafe {
        FillWindowPixelBuffer(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7697))
            .cast::<u8>())
            .wrapping_offset(3))
            .read(),
            17u8,
        );
        AddTextPrinterParameterized(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7697))
            .cast::<u8>())
            .wrapping_offset(3))
            .read(),
            1u8,
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7720)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<*mut u8>())
            .read(),
            8u8,
            1u8,
            0u8,
            None,
        );
        PutWindowTilemap(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7697))
            .cast::<u8>())
            .wrapping_offset(3))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DrawMonTextEntryBox() {
    unsafe {
        let mut buffer = crate::ffi::Align4([0u8; 32]);
        StringCopy(
            (&raw mut buffer).cast::<u8>(),
            (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7732)
                    .cast::<u16>())
                .read()) as i32) as isize
                    * 11,
            ))
            .cast::<u8>(),
        );
        StringAppendN(
            (&raw mut buffer).cast::<u8>(),
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7720)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<*mut u8>())
            .read(),
            15u8,
        );
        FillWindowPixelBuffer(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7697))
            .cast::<u8>())
            .wrapping_offset(3))
            .read(),
            17u8,
        );
        AddTextPrinterParameterized(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7697))
            .cast::<u8>())
            .wrapping_offset(3))
            .read(),
            1u8,
            (&raw mut buffer).cast::<u8>(),
            8u8,
            1u8,
            0u8,
            None,
        );
        PutWindowTilemap(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7697))
            .cast::<u8>())
            .wrapping_offset(3))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DrawTextEntryBox() {
    unsafe {
        (((((&raw const sDrawTextEntryBoxFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7724))
            .read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn TryDrawGenderIcon() {
    unsafe {
        (((((&raw const sDrawGenderIconFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            ((((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7720)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(3))
            .read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn DummyGenderIcon() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn DrawGenderIcon() {
    unsafe {
        let mut text = crate::ffi::Align4([0u8; 2]);
        let mut isFemale: u8 = 0u8;
        StringCopy(
            (&raw mut text).cast::<u8>(),
            (&raw mut gText_MaleSymbol).cast::<u8>(),
        );
        if ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(7734)
            .cast::<u16>())
        .read()) as i32)
            != 255i32
        {
            if ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7734)
                .cast::<u16>())
            .read()) as i32)
                == 254i32
            {
                StringCopy(
                    (&raw mut text).cast::<u8>(),
                    (&raw mut gText_FemaleSymbol).cast::<u8>(),
                );
                isFemale = 1u8;
            }
            AddTextPrinterParameterized3(
                ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7697))
                .cast::<u8>())
                .wrapping_offset(2))
                .read(),
                1u8,
                104u8,
                1u8,
                ((((&raw const sGenderColors).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((isFemale) as i32) as isize * 3))
                .cast::<u8>(),
                (-1i8),
                (&raw mut text).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn GetCharAtKeyboardPos(x: i16, y: i16) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        return ((((((((&raw const sKeyboardChars).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((CurrentPageToKeyboardId()) as i32) as isize * 32))
        .cast::<u8>())
        .wrapping_offset(((y) as i32) as isize * 8))
        .cast::<u8>())
        .wrapping_offset(((x) as i32) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn GetTextEntryPosition() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7720)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6144))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 255i32
                    {
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((((((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(7720)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1))
        .read()) as i32)
            .wrapping_sub(1i32)) as u8);
    }
}
pub(crate) unsafe extern "C" fn GetPreviousTextCaretPosition() -> u8 {
    unsafe {
        let mut i: i8 = 0i8;
        {
            i = ((((((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7720)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1))
            .read()) as i32)
                .wrapping_sub(1i32)) as i8);
            'l1: loop {
                if !(((i) as i32) > 0i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6144))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 255i32
                    {
                        return ((i) as u8);
                    }
                }
                i = (i).wrapping_sub(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DeleteTextCharacter() {
    unsafe {
        let mut index: u8 = 0u8;
        let mut keyRole: u8 = 0u8;
        index = GetPreviousTextCaretPosition();
        ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6144))
            .cast::<u8>())
        .wrapping_offset(((index) as i32) as isize))
        .write(0u8);
        DrawTextEntry();
        CopyBgTilemapBufferToVram(3u8);
        ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6144))
            .cast::<u8>())
        .wrapping_offset(((index) as i32) as isize))
        .write(255u8);
        keyRole = GetKeyRoleAtCursorPos();
        if (((keyRole) as i32) == 0i32) || (((keyRole) as i32) == 2i32) {
            TryStartButtonFlash(1u8, 0u8, 1u8);
        }
        PlaySE(23u16);
    }
}
pub(crate) unsafe extern "C" fn AddTextCharacter() -> u8 {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        GetCursorPos(&raw mut x, &raw mut y);
        BufferCharacter(GetCharAtKeyboardPos(x, y));
        DrawTextEntry();
        CopyBgTilemapBufferToVram(3u8);
        PlaySE(5u16);
        if ((GetPreviousTextCaretPosition()) as i32)
            != ((((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7720)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1))
            .read()) as i32)
                .wrapping_sub(1i32)
        {
            return 0u8;
        } else {
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn BufferCharacter(ch: u8) {
    unsafe {
        let mut ch = ch;
        let mut index: u8 = GetTextEntryPosition();
        ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6144))
            .cast::<u8>())
        .wrapping_offset(((index) as i32) as isize))
        .write(ch);
    }
}
pub(crate) unsafe extern "C" fn SaveInputText() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7720)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6144))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 0i32)
                        && (((((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(6144))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            != 255i32)
                    {
                        StringCopyN(
                            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(7728)
                                .cast::<*mut u8>())
                            .read(),
                            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(6144))
                            .cast::<u8>(),
                            ((((((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(7720)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(1))
                            .read()) as i32)
                                .wrapping_add(1i32)) as u8),
                        );
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadGfx() {
    unsafe {
        LZ77UnCompWram(
            ((&raw mut gNamingScreenMenu_Gfx).cast::<u32>()).cast::<u32>(),
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6160))
                .cast::<u8>(),
        );
        LoadBgTiles(
            1u8,
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6160))
                .cast::<u8>(),
            1536u16,
            0u16,
        );
        LoadBgTiles(
            2u8,
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6160))
                .cast::<u8>(),
            1536u16,
            0u16,
        );
        LoadBgTiles(
            3u8,
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6160))
                .cast::<u8>(),
            1536u16,
            0u16,
        );
        LoadSpriteSheets(((&raw const sSpriteSheets).cast::<u8>().cast_mut()).cast::<u8>());
        LoadSpritePalettes(((&raw const sSpritePalettes).cast::<u8>().cast_mut()).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn CreateHelperTasks() {
    unsafe {
        CreateInputHandlerTask();
        CreateButtonFlashTask();
    }
}
pub(crate) unsafe extern "C" fn LoadPalettes() {
    unsafe {
        LoadPalette((&raw mut gNamingScreenMenu_Pal).cast::<u8>(), 0u16, 192u16);
        LoadPalette(
            (((&raw const sKeyboard_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            160u16,
            32u16,
        );
        LoadPalette((GetTextWindowPalette(2u8)).cast::<u8>(), 176u16, 32u16);
    }
}
pub(crate) unsafe extern "C" fn DrawBgTilemap(bg: u8, src: *mut u8) {
    unsafe {
        let mut bg = bg;
        let mut src = src;
        CopyToBgTilemapBuffer(bg, src, 0u16, 0u16);
    }
}
pub(crate) unsafe extern "C" fn NamingScreen_Dummy(bg: u8, page: u8) {
    unsafe {
        let mut bg = bg;
        let mut page = page;
    }
}
pub(crate) unsafe extern "C" fn DrawTextEntry() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut temp = crate::ffi::Align4([0u8; 2]);
        let mut extraWidth: u16 = 0u16;
        let mut maxChars: u8 = ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(7720)
        .cast::<*mut u8>())
        .read())
        .wrapping_add(1))
        .read();
        let mut x: u16 = ((((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(7702)
            .cast::<u16>())
        .read()) as i32)
            .wrapping_sub(64i32)) as u16);
        FillWindowPixelBuffer(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7697))
            .cast::<u8>())
            .wrapping_offset(2))
            .read(),
            17u8,
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((maxChars) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((&raw mut temp).cast::<u8>()).write(
                        ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6144))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                    (((&raw mut temp).cast::<u8>()).wrapping_offset(1))
                        .write(((&raw mut gText_ExpandedPlaceholder_Empty).cast::<u8>()).read());
                    extraWidth = ((if ((IsWideLetter(((&raw mut temp).cast::<u8>()).read())) as i32)
                        == 1i32
                    {
                        2i32
                    } else {
                        0i32
                    }) as u16);
                    AddTextPrinterParameterized(
                        ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(7697))
                        .cast::<u8>())
                        .wrapping_offset(2))
                        .read(),
                        1u8,
                        (&raw mut temp).cast::<u8>(),
                        ((((((i) as i32).wrapping_mul(8i32)).wrapping_add(((x) as i32)))
                            .wrapping_add(((extraWidth) as i32))) as u8),
                        1u8,
                        255u8,
                        None,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        TryDrawGenderIcon();
        CopyWindowToVram(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7697))
            .cast::<u8>())
            .wrapping_offset(2))
            .read(),
            2u8,
        );
        PutWindowTilemap(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7697))
            .cast::<u8>())
            .wrapping_offset(2))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintKeyboardKeys(window: u8, page: u8) {
    unsafe {
        let mut window = window;
        let mut page = page;
        let mut i: u8 = 0u8;
        FillWindowPixelBuffer(
            window,
            ((((&raw const sFillValues).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((page) as i32) as isize))
            .read(),
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    AddTextPrinterParameterized3(
                        window,
                        1u8,
                        0u8,
                        (((((i) as i32).wrapping_mul(16i32)).wrapping_add(1i32)) as u8),
                        ((((&raw const sKeyboardTextColors)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(((page) as i32) as isize))
                        .read(),
                        0i8,
                        ((((((&raw const sNamingScreenKeyboardText)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((page) as i32) as isize * 16))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        PutWindowTilemap(window);
    }
}
pub(crate) unsafe extern "C" fn DrawKeyboardPageOnDeck() {
    unsafe {
        let mut bg: u8 = 0u8;
        let mut bg_: u8 = 0u8;
        let mut windowId: u8 = 0u8;
        let mut bg1Priority: u8 = ((((GetGpuReg(10u8)) as i32) & 3i32) as u8);
        let mut bg2Priority: u8 = ((((GetGpuReg(12u8)) as i32) & 3i32) as u8);
        if ((bg1Priority) as i32) > ((bg2Priority) as i32) {
            bg = 1u8;
            bg_ = 1u8;
            windowId = (((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7697))
            .cast::<u8>())
            .read();
        } else {
            bg = 2u8;
            bg_ = 2u8;
            windowId = ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7697))
            .cast::<u8>())
            .wrapping_offset(1))
            .read();
        }
        DrawBgTilemap(
            bg,
            (((((&raw const sNextKeyboardPageTilemaps)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u32>())
            .cast::<*mut u32>())
            .wrapping_offset(
                ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7714))
                .read()) as i32) as isize,
            ))
            .read())
            .cast::<u8>(),
        );
        PrintKeyboardKeys(windowId, CurrentPageToNextKeyboardId());
        NamingScreen_Dummy(bg, CurrentPageToNextKeyboardId());
        CopyBgTilemapBufferToVram(bg_);
    }
}
pub(crate) unsafe extern "C" fn PrintControls() {
    unsafe {
        let mut color = crate::ffi::Align4([0u8; 3]);
        (&raw mut color).cast::<u8>().wrapping_add(0).write(15u8);
        (&raw mut color).cast::<u8>().wrapping_add(1).write(1u8);
        (&raw mut color).cast::<u8>().wrapping_add(2).write(2u8);
        FillWindowPixelBuffer(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7697))
            .cast::<u8>())
            .wrapping_offset(4))
            .read(),
            255u8,
        );
        AddTextPrinterParameterized3(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7697))
            .cast::<u8>())
            .wrapping_offset(4))
            .read(),
            0u8,
            2u8,
            1u8,
            (&raw mut color).cast::<u8>(),
            0i8,
            (&raw mut gText_MoveOkBack).cast::<u8>(),
        );
        PutWindowTilemap(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7697))
            .cast::<u8>())
            .wrapping_offset(4))
            .read(),
        );
        CopyWindowToVram(
            ((((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7697))
            .cast::<u8>())
            .wrapping_offset(4))
            .read(),
            3u8,
        );
    }
}
pub(crate) unsafe extern "C" fn CB2_NamingScreen() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn ResetVHBlank() {
    unsafe {
        SetVBlankCallback(None);
        SetHBlankCallback(None);
    }
}
pub(crate) unsafe extern "C" fn SetVBlank() {
    unsafe {
        SetVBlankCallback(Some(VBlankCB_NamingScreen));
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_NamingScreen() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
        SetGpuReg(
            22u8,
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7704)
                .cast::<u16>())
            .read(),
        );
        SetGpuReg(
            26u8,
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7706)
                .cast::<u16>())
            .read(),
        );
        SetGpuReg(10u8, ((((GetGpuReg(10u8)) as i32) & 65532i32) as u16));
        SetGpuRegBits(
            10u8,
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7708)
                .cast::<u16>())
            .read(),
        );
        SetGpuReg(12u8, ((((GetGpuReg(12u8)) as i32) & 65532i32) as u16));
        SetGpuRegBits(
            12u8,
            ((((&raw mut sNamingScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7710)
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn NamingScreen_ShowBgs() {
    unsafe {
        ShowBg(0u8);
        ShowBg(1u8);
        ShowBg(2u8);
        ShowBg(3u8);
    }
}
pub(crate) unsafe extern "C" fn IsWideLetter(character: u8) -> u8 {
    unsafe {
        let mut character = character;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((((((&raw const sText_AlphabetUpperLower)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((i) as i32) as isize))
                .read()) as i32)
                    != 255i32)
                {
                    break 'l1;
                }
                'l2: {
                    if ((character) as i32)
                        == ((((((&raw const sText_AlphabetUpperLower)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                    {
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Debug_NamingScreenPlayer() {
    unsafe {
        DoNamingScreen(
            0u8,
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                as u16),
            0u16,
            0u32,
            Some(CB2_ReturnToFieldWithOpenMenu),
        );
    }
}
pub(crate) unsafe extern "C" fn Debug_NamingScreenBox() {
    unsafe {
        DoNamingScreen(
            1u8,
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                as u16),
            0u16,
            0u32,
            Some(CB2_ReturnToFieldWithOpenMenu),
        );
    }
}
pub(crate) unsafe extern "C" fn Debug_NamingScreenCaughtMon() {
    unsafe {
        DoNamingScreen(
            2u8,
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                as u16),
            0u16,
            0u32,
            Some(CB2_ReturnToFieldWithOpenMenu),
        );
    }
}
pub(crate) unsafe extern "C" fn Debug_NamingScreenNickname() {
    unsafe {
        DoNamingScreen(
            3u8,
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                as u16),
            0u16,
            0u32,
            Some(CB2_ReturnToFieldWithOpenMenu),
        );
    }
}
