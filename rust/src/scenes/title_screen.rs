//! Translated from `src/title_screen.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sUnusedUnknownPal sTitleScreenRayquazaGfx sTitleScreenRayquazaTilemap sTitleScreenLogoShineGfx sTitleScreenCloudsGfx gTitleScreenAlphaBlend sVersionBannerLeftOamData sVersionBannerRightOamData sVersionBannerLeftAnimSequence sVersionBannerRightAnimSequence sVersionBannerLeftAnimTable sVersionBannerRightAnimTable sVersionBannerLeftSpriteTemplate sVersionBannerRightSpriteTemplate sSpriteSheet_EmeraldVersion sOamData_CopyrightBanner sAnim_PressStart_0 sAnim_PressStart_1 sAnim_PressStart_2 sAnim_PressStart_3 sAnim_PressStart_4 sAnim_Copyright_0 sAnim_Copyright_1 sAnim_Copyright_2 sAnim_Copyright_3 sAnim_Copyright_4 sStartCopyrightBannerAnimTable sStartCopyrightBannerSpriteTemplate sSpriteSheet_PressStart sSpritePalette_PressStart sPokemonLogoShineOamData sPokemonLogoShineAnimSequence sPokemonLogoShineAnimTable sPokemonLogoShineSpriteTemplate sPokemonLogoShineSpriteSheet
#[allow(unused_imports)]
use crate::data::title_screen::*;

unsafe extern "C" {
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG1_Y: u8;
    static mut gMPlayInfo_BGM: u8;
    static mut gMain: u8;
    static mut gPlttBufferFaded: u8;
    static mut gReservedSpritePaletteCount: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gTitleScreenBgPalettes: u8;
    static mut gTitleScreenCloudsTilemap: u8;
    static mut gTitleScreenEmeraldVersionPal: u8;
    static mut gTitleScreenPokemonLogoGfx: u8;
    static mut gTitleScreenPokemonLogoTilemap: u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_InitBerryFixProgram();
    fn CB2_InitClearSaveDataScreen();
    fn CB2_InitCopyrightScreenAfterTitleScreen();
    fn CB2_InitMainMenu();
    fn CB2_InitResetRtcScreen();
    fn CanResetRTC() -> u32;
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn EnableInterrupts(a0: u16);
    fn FadeOutBGM(a0: u8);
    fn FreeAllSpritePalettes();
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn PanFadeAndZoomScreen(a0: u16, a1: u16, a2: u16, a3: u16);
    fn ProcessSpriteCopyRequests();
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn ScanlineEffect_InitWave(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u8;
    fn ScanlineEffect_Stop();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn m4aMPlayAllStop();
    fn m4aSongNumStart(a0: u16);
}

pub(crate) unsafe extern "C" fn SpriteCB_VersionBannerLeft(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read())
            != 0
        {
            crate::c::bf_write((sprite).wrapping_add(1), 2, 2, (0u32) as i32);
            ((sprite).wrapping_add(34).cast::<i16>()).write(66i16);
        } else {
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) != 66i32 {
                let __p1 = (sprite).wrapping_add(34).cast::<i16>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_sub(1));
            }
            SetGpuReg(
                82u8,
                ((((&raw const gTitleScreenAlphaBlend)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize,
                ))
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_VersionBannerRight(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read())
            != 0
        {
            crate::c::bf_write((sprite).wrapping_add(1), 2, 2, (0u32) as i32);
            ((sprite).wrapping_add(34).cast::<i16>()).write(66i16);
        } else {
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) != 66i32 {
                let __p1 = (sprite).wrapping_add(34).cast::<i16>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PressStartCopyrightBanner(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 1i32 {
            if ((({
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                & 16i32)
                != 0
            {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            } else {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            }
        } else {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        }
    }
}
pub(crate) unsafe extern "C" fn CreatePressStartBanner(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut i: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        x = ((((x) as i32).wrapping_sub(64i32)) as i16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (&raw const sStartCopyrightBannerSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        x,
                        y,
                        0u8,
                    );
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        i,
                    );
                    (((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(1i16);
                }
                i = (i).wrapping_add(1);
                x = ((((x) as i32).wrapping_add(32i32)) as i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateCopyrightBanner(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut i: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        x = ((((x) as i32).wrapping_sub(64i32)) as i16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (&raw const sStartCopyrightBannerSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        x,
                        y,
                        0u8,
                    );
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        ((((i) as i32).wrapping_add(5i32)) as u8),
                    );
                }
                i = (i).wrapping_add(1);
                x = ((((x) as i32).wrapping_add(32i32)) as i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PokemonLogoShine(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < 272i32 {
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
                let mut backgroundColor: u16 = 0u16;
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    < crate::c::div_i32(240i32, 2i32)
                {
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        < 31i32
                    {
                        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                        (__p1).write(((__p1).read()).wrapping_add(1));
                    }
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        < 31i32
                    {
                        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                } else {
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        != 0i32
                    {
                        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                        (__p3).write(((__p3).read()).wrapping_sub(1));
                    }
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        != 0i32
                    {
                        let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                        (__p4).write(((__p4).read()).wrapping_sub(1));
                    }
                }
                backgroundColor = (((((((((((sprite).wrapping_add(46)).cast::<i16>())
                    .wrapping_offset(1))
                .read()) as i32)
                    & 31i32)
                    << 10)
                    .wrapping_add(
                        ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32)
                            & 31i32)
                            << 5),
                    ))
                .wrapping_add(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        & 31i32),
                )) as u16);
                if (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    == (crate::c::div_i32(240i32, 2i32)).wrapping_add(12i32))
                    || (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                        == (crate::c::div_i32(240i32, 2i32)).wrapping_add(16i32)))
                    || (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                        == (crate::c::div_i32(240i32, 2i32)).wrapping_add(20i32)))
                    || (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                        == (crate::c::div_i32(240i32, 2i32)).wrapping_add(24i32))
                {
                    (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).write(13304u16);
                } else {
                    (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                        .write(backgroundColor);
                }
            }
            let __p5 = (sprite).wrapping_add(32).cast::<i16>();
            (__p5).write((((((__p5).read()) as i32).wrapping_add(4i32)) as i16));
        } else {
            (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).write(0u16);
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PokemonLogoShine_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < 272i32 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
        } else {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn StartPokemonLogoShine(mode: u8) {
    unsafe {
        let mut mode = mode;
        let mut spriteId: u8 = 0u8;
        'l1: {
            let __sw1 = ((mode) as i32);
            if __sw1 == 0i32 || __sw1 == 2i32 {
                spriteId = CreateSprite(
                    (&raw const sPokemonLogoShineSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    0i16,
                    68i16,
                    0u8,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(1),
                    2,
                    2,
                    (2u32) as i32,
                );
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(((mode) as i16));
                break 'l1;
            }
            if __sw1 == 1i32 {
                spriteId = CreateSprite(
                    (&raw const sPokemonLogoShineSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    0i16,
                    68i16,
                    0u8,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(1),
                    2,
                    2,
                    (2u32) as i32,
                );
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(((mode) as i16));
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                spriteId = CreateSprite(
                    (&raw const sPokemonLogoShineSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    0i16,
                    68i16,
                    0u8,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_PokemonLogoShine_Fast));
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(1),
                    2,
                    2,
                    (2u32) as i32,
                );
                spriteId = CreateSprite(
                    (&raw const sPokemonLogoShineSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    (-80i16),
                    68i16,
                    0u8,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_PokemonLogoShine_Fast));
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(1),
                    2,
                    2,
                    (2u32) as i32,
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB() {
    unsafe {
        ScanlineEffect_InitHBlankDmaTransfer();
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
        SetGpuReg(22u8, ((&raw mut gBattle_BG1_Y).cast::<u16>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitTitleScreen() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32;
            if __sw1 == 0i32 || !__matched {
                SetVBlankCallback(None);
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetGpuReg(84u8, 0u16);
                ((83886080i32) as usize as *mut u16).write(32767u16);
                SetGpuReg(0u8, 0u16);
                SetGpuReg(12u8, 0u16);
                SetGpuReg(10u8, 0u16);
                SetGpuReg(8u8, 0u16);
                SetGpuReg(24u8, 0u16);
                SetGpuReg(26u8, 0u16);
                SetGpuReg(20u8, 0u16);
                SetGpuReg(22u8, 0u16);
                SetGpuReg(16u8, 0u16);
                SetGpuReg(18u8, 0u16);
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
                                            (((100663296i32) as usize as *mut u8) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(2),
                                            (((-2130706432i32)
                                                | crate::c::div_i32(
                                                    98304i32,
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
                'l6: loop {
                    'l7: {
                        {
                            let mut tmp: u32 = 0u32;
                            (&raw mut tmp).write_volatile(0u32);
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
                                            (((117440512i32) as usize as *mut u8) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(2),
                                            (((-2063597568i32)
                                                | crate::c::div_i32(
                                                    1024i32,
                                                    crate::c::div_i32(32i32, 8i32),
                                                ))
                                                as u32),
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
                'l10: loop {
                    'l11: {
                        {
                            let mut tmp: u16 = 0u16;
                            (&raw mut tmp).write_volatile(0u16);
                            'l12: loop {
                                'l13: {
                                    {
                                        let mut dmaRegs: *mut u32 =
                                            ((67109076i32) as usize as *mut u32);
                                        crate::c::volatile_write(
                                            dmaRegs,
                                            ((&raw mut tmp) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(1),
                                            (((83886082i32) as usize as *mut u8) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(2),
                                            (((-2130706432i32)
                                                | crate::c::div_i32(
                                                    1022i32,
                                                    crate::c::div_i32(16i32, 8i32),
                                                ))
                                                as u32),
                                        );
                                        let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
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
                ResetPaletteFade();
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                LZ77UnCompVram(
                    ((&raw mut gTitleScreenPokemonLogoGfx).cast::<u32>()).cast::<u32>(),
                    ((100663296i32) as usize as *mut u8),
                );
                LZ77UnCompVram(
                    ((&raw mut gTitleScreenPokemonLogoTilemap).cast::<u32>()).cast::<u32>(),
                    ((100681728i32) as usize as *mut u8),
                );
                LoadPalette(
                    (((&raw mut gTitleScreenBgPalettes).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                    0u16,
                    480u16,
                );
                LZ77UnCompVram(
                    ((&raw const sTitleScreenRayquazaGfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100696064i32) as usize as *mut u8),
                );
                LZ77UnCompVram(
                    ((&raw const sTitleScreenRayquazaTilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100716544i32) as usize as *mut u8),
                );
                LZ77UnCompVram(
                    ((&raw const sTitleScreenCloudsGfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100712448i32) as usize as *mut u8),
                );
                LZ77UnCompVram(
                    ((&raw mut gTitleScreenCloudsTilemap).cast::<u32>()).cast::<u32>(),
                    ((100718592i32) as usize as *mut u8),
                );
                ScanlineEffect_Stop();
                ResetTasks();
                ResetSpriteData();
                FreeAllSpritePalettes();
                ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(9u8);
                LoadCompressedSpriteSheet(
                    ((&raw const sSpriteSheet_EmeraldVersion)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                LoadCompressedSpriteSheet(
                    ((&raw const sSpriteSheet_PressStart).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                LoadCompressedSpriteSheet(
                    ((&raw const sPokemonLogoShineSpriteSheet)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                LoadPalette(
                    (((&raw mut gTitleScreenEmeraldVersionPal).cast::<u16>()).cast::<u16>())
                        .cast::<u8>(),
                    256u16,
                    32u16,
                );
                LoadSpritePalette(
                    ((&raw const sSpritePalette_PressStart)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(2u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    let mut taskId: u8 = CreateTask(Some(Task_TitleScreenPhase1), 0u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(256i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write((-16i16));
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write((-32i16));
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(3u8);
                    break 'l1;
                }
            }
            if __sw1 == 3i32 {
                BeginNormalPaletteFade(4294967295u32, 1i8, 16u8, 0u8, 65535u16);
                SetVBlankCallback(Some(VBlankCB));
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(4u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                PanFadeAndZoomScreen(
                    ((crate::c::div_i32(240i32, 2i32)) as u16),
                    ((crate::c::div_i32(160i32, 2i32)) as u16),
                    256u16,
                    0u16,
                );
                SetGpuReg(40u8, 58112u16);
                SetGpuReg(42u8, 65535u16);
                SetGpuReg(44u8, 57344u16);
                SetGpuReg(46u8, 65535u16);
                SetGpuReg(64u8, 0u16);
                SetGpuReg(68u8, 0u16);
                SetGpuReg(66u8, 0u16);
                SetGpuReg(70u8, 0u16);
                SetGpuReg(72u8, 7967u16);
                SetGpuReg(74u8, 16159u16);
                SetGpuReg(80u8, 132u16);
                SetGpuReg(82u8, 0u16);
                SetGpuReg(84u8, 12u16);
                SetGpuReg(8u8, 6667u16);
                SetGpuReg(10u8, 6926u16);
                SetGpuReg(12u8, 18817u16);
                EnableInterrupts(1u16);
                SetGpuReg(0u8, 46145u16);
                m4aSongNumStart(413u16);
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(5u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((UpdatePaletteFade()) != 0) {
                    StartPokemonLogoShine(0u8);
                    ScanlineEffect_InitWave(0u8, 160u8, 4u8, 4u8, 0u8, 4u8, 1u8);
                    SetMainCallback2(Some(MainCB2));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn MainCB2() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn Task_TitleScreenPhase1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 15i32)
            != 0)
            || ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read())
                != 0)
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(1i16);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
        }
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            != 0i32
        {
            let mut frameNum: u16 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as u16);
            if ((frameNum) as i32) == 176i32 {
                StartPokemonLogoShine(1u8);
            } else {
                if ((frameNum) as i32) == 64i32 {
                    StartPokemonLogoShine(2u8);
                }
            }
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            let mut spriteId: u8 = 0u8;
            SetGpuReg(0u8, 5185u16);
            SetGpuReg(72u8, 0u16);
            SetGpuReg(74u8, 0u16);
            SetGpuReg(80u8, 16208u16);
            SetGpuReg(82u8, 16u16);
            SetGpuReg(84u8, 0u16);
            spriteId = CreateSprite(
                (&raw const sVersionBannerLeftSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                98i16,
                2i16,
                0u8,
            );
            (((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .write(((crate::c::div_u32(128u32, 2u32)) as i16));
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((taskId) as i16));
            spriteId = CreateSprite(
                (&raw const sVersionBannerRightSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                162i16,
                2i16,
                0u8,
            );
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((taskId) as i16));
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(144i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_TitleScreenPhase2));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TitleScreenPhase2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut yPos: u32 = 0u32;
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 15i32)
            != 0)
            || ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read())
                != 0)
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(1i16);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
        }
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            != 0i32
        {
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(1i16);
            SetGpuReg(80u8, 8514u16);
            SetGpuReg(82u8, 3846u16);
            SetGpuReg(84u8, 0u16);
            SetGpuReg(0u8, 5953u16);
            CreatePressStartBanner(128i16, 108i16);
            CreateCopyrightBanner(128i16, 148i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_TitleScreenPhase3));
        }
        if (!(((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            & 3i32)
            != 0))
            && (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
                != 0i32)
        {
            let __p2 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        if (!(((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            & 1i32)
            != 0))
            && (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as i32)
                != 0i32)
        {
            let __p3 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        yPos = ((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            .wrapping_mul(256i32)) as u32);
        SetGpuReg(44u8, ((yPos) as u16));
        SetGpuReg(46u8, ((crate::c::div_u32(yPos, 65536u32)) as u16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(15i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(6i16);
    }
}
pub(crate) unsafe extern "C" fn Task_TitleScreenPhase3(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0)
            || (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 8i32)
                != 0)
        {
            FadeOutBGM(4u8);
            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 65535u16);
            SetMainCallback2(Some(CB2_GoToMainMenu));
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(44)
                .cast::<u16>())
            .read()) as i32)
                & 70i32)
                == 70i32
            {
                SetMainCallback2(Some(CB2_GoToClearSaveDataScreen));
            } else {
                if (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(44)
                    .cast::<u16>())
                .read()) as i32)
                    & 38i32)
                    == 38i32)
                    && (CanResetRTC() == 1u32)
                {
                    FadeOutBGM(4u8);
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                    SetMainCallback2(Some(CB2_GoToResetRtcScreen));
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(44)
                        .cast::<u16>())
                    .read()) as i32)
                        & 6i32)
                        == 6i32
                    {
                        FadeOutBGM(4u8);
                        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                        SetMainCallback2(Some(CB2_GoToBerryFixScreen));
                    } else {
                        SetGpuReg(44u8, 0u16);
                        SetGpuReg(46u8, 0u16);
                        if ((({
                            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>();
                            let __t2 = ((__p1).read()).wrapping_add(1);
                            (__p1).write(__t2);
                            __t2
                        }) as i32)
                            & 1i32)
                            != 0
                        {
                            let __p3 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(4);
                            (__p3).write(((__p3).read()).wrapping_add(1));
                            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(
                                ((crate::c::div_i32(
                                    ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(4))
                                    .read()) as i32),
                                    2i32,
                                )) as u16),
                            );
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                        }
                        UpdateLegendaryMarkingColor(
                            (((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .read()) as u8),
                        );
                        if ((((&raw mut gMPlayInfo_BGM).cast::<u8>())
                            .wrapping_add(4)
                            .cast::<u32>())
                        .read()
                            & 65535u32)
                            == 0u32
                        {
                            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 65535u16);
                            SetMainCallback2(Some(CB2_GoToCopyrightScreen));
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_GoToMainMenu() {
    unsafe {
        if !((UpdatePaletteFade()) != 0) {
            SetMainCallback2(Some(CB2_InitMainMenu));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_GoToCopyrightScreen() {
    unsafe {
        if !((UpdatePaletteFade()) != 0) {
            SetMainCallback2(Some(CB2_InitCopyrightScreenAfterTitleScreen));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_GoToClearSaveDataScreen() {
    unsafe {
        if !((UpdatePaletteFade()) != 0) {
            SetMainCallback2(Some(CB2_InitClearSaveDataScreen));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_GoToResetRtcScreen() {
    unsafe {
        if !((UpdatePaletteFade()) != 0) {
            SetMainCallback2(Some(CB2_InitResetRtcScreen));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_GoToBerryFixScreen() {
    unsafe {
        if !((UpdatePaletteFade()) != 0) {
            m4aMPlayAllStop();
            SetMainCallback2(Some(CB2_InitBerryFixProgram));
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateLegendaryMarkingColor(frameNum: u8) {
    unsafe {
        let mut frameNum = frameNum;
        if crate::c::rem_i32(((frameNum) as i32), 4i32) == 0i32 {
            let mut intensity: i32 = ((Cos(
                ((frameNum) as i16),
                ((((0.5f32) as f32) * ((256i32) as f32)) as i16),
            )) as i32)
                .wrapping_add((((((0.5f32) as f32) * ((256i32) as f32)) as i16) as i32));
            let mut r: u32 = (((31i32)
                .wrapping_sub(crate::c::div_i32((intensity).wrapping_mul(31i32), 256i32)))
                as u32);
            let mut g: u32 = (((31i32)
                .wrapping_sub(crate::c::div_i32((intensity).wrapping_mul(22i32), 256i32)))
                as u32);
            let mut b: u32 = 12u32;
            let mut color: u16 = (((r | (g << 5)) | (b << 10)) as u16);
            LoadPalette((&raw mut color).cast::<u8>(), 239u16, 2u16);
        }
    }
}
