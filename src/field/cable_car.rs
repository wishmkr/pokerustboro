//! Translated from `src/cable_car.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sBgTemplates sGround_Tilemap sTrees_Tilemap sBgMountains_Tilemap sPylonTop_Tilemap sPylonPole_Tilemap sSpriteSheets sSpritePalettes sOam_CableCar sOam_CableCarDoor sOam_Cable sSpriteTemplates_CableCar sSpriteTemplate_Cable
#[allow(unused_imports)]
use crate::data::cable_car::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCableCar: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sGroundX_Up: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sGroundY_Up: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sGroundSegmentY_Up: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sGroundX_Down: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sGroundY_Down: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sGroundSegmentY_Down: u8 = 0u8;

unsafe extern "C" {
    static mut gCableCarBg_Gfx: u8;
    static mut gCableCarBg_Pal: u8;
    static mut gFieldCallback: u8;
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpriteCoordOffsetX: u8;
    static mut gSpriteCoordOffsetY: u8;
    static mut gSprites: u8;
    static mut gWeatherPtr: u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_LoadMap();
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CreateObjectGraphicsSprite(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut u8)>,
        a2: i16,
        a3: i16,
        a4: u8,
    ) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn FadeInNewBGM(a0: u16, a1: u8);
    fn FadeOutBGM(a0: u8);
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn HideBg(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitMapMusic();
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut u8);
    fn LockPlayerFieldControls();
    fn MapMusicMain();
    fn ProcessSpriteCopyRequests();
    fn Random() -> u16;
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetMapMusic();
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn RunTasks();
    fn ScanlineEffect_Stop();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetCurrentAndNextWeatherNoDelay(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetNextWeather(a0: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StartWeather();
    fn TransferPlttBuffer();
    fn UnsetBgTilemapBuffer(a0: u8);
    fn UpdatePaletteFade() -> u8;
    fn WarpIntoMap();
    fn malloc_and_decompress(a0: *mut u8, a1: *mut u32) -> *mut u8;
}

pub(crate) unsafe extern "C" fn Task_LoadCableCar(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            SetMainCallback2(Some(CB2_LoadCableCar));
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CableCar() {
    unsafe {
        LockPlayerFieldControls();
        CreateTask(Some(Task_LoadCableCar), 1u8);
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn CB2_LoadCableCar() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut sizeOut: u32 = 0u32;
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
                || __sw1 == 9i32;
            if __sw1 == 0i32 || !__matched {
                SetVBlankCallback(None);
                SetBgRegs(0u8);
                ScanlineEffect_Stop();
                {
                    let mut _dest: *mut u8 = ((100663296i32) as usize as *mut u8);
                    let mut _size: u32 = 98304u32;
                    'l2: loop {
                        if !((1i32) != 0) {
                            break 'l2;
                        }
                        'l3: loop {
                            'l4: {
                                {
                                    let mut tmp: u16 = 0u16;
                                    (&raw mut tmp).write_volatile(0u16);
                                    'l5: loop {
                                        'l6: {
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
                                                let _ =
                                                    ((dmaRegs).wrapping_offset(2)).read_volatile();
                                            }
                                        }
                                        if !((0i32) != 0) {
                                            break 'l5;
                                        }
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l3;
                            }
                        }
                        _dest = (_dest).wrapping_offset(4096);
                        _size = (_size).wrapping_sub(4096u32);
                        if _size <= 4096u32 {
                            'l7: loop {
                                'l8: {
                                    {
                                        let mut tmp: u16 = 0u16;
                                        (&raw mut tmp).write_volatile(0u16);
                                        'l9: loop {
                                            'l10: {
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
                                                                ((crate::c::div_i32(16i32, 8i32))
                                                                    as u32),
                                                            )),
                                                    );
                                                    let _ = ((dmaRegs).wrapping_offset(2))
                                                        .read_volatile();
                                                }
                                            }
                                            if !((0i32) != 0) {
                                                break 'l9;
                                            }
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l7;
                                }
                            }
                            break 'l2;
                        }
                    }
                }
                {
                    let mut _dest: *mut u8 = ((117440512i32) as usize as *mut u8);
                    let mut _size: u32 = 1024u32;
                    'l11: loop {
                        'l12: {
                            {
                                let mut tmp: u32 = 0u32;
                                (&raw mut tmp).write_volatile(0u32);
                                'l13: loop {
                                    'l14: {
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
                                        break 'l13;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l11;
                        }
                    }
                }
                {
                    let mut _dest: *mut u8 = ((83886080i32) as usize as *mut u8);
                    let mut _size: u32 = 1024u32;
                    'l15: loop {
                        'l16: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l17: loop {
                                    'l18: {
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
                                        break 'l17;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l15;
                        }
                    }
                }
                ((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(16656u32));
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ResetSpriteData();
                ResetTasks();
                FreeAllSpritePalettes();
                ResetPaletteFade();
                ResetTempTileDataBuffers();
                StartWeather();
                {
                    i = 0u8;
                    'l19: loop {
                        if !(((i) as i32) < 20i32) {
                            break 'l19;
                        }
                        'l20: {
                            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(240))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(core::ptr::null_mut());
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                InitMapMusic();
                ResetMapMusic();
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(16u32, 4u32)) as u8),
                );
                SetBgTilemapBuffer(
                    0u8,
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(252))
                    .cast::<u8>())
                    .cast::<u16>())
                    .cast::<u8>(),
                );
                SetBgTilemapBuffer(
                    1u8,
                    (((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(252))
                    .cast::<u8>())
                    .wrapping_offset(4096))
                    .cast::<u16>())
                    .cast::<u8>(),
                );
                SetBgTilemapBuffer(
                    2u8,
                    (((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(252))
                    .cast::<u8>())
                    .wrapping_offset(8192))
                    .cast::<u16>())
                    .cast::<u8>(),
                );
                SetBgTilemapBuffer(
                    3u8,
                    (((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(252))
                    .cast::<u8>())
                    .wrapping_offset(12288))
                    .cast::<u16>())
                    .cast::<u8>(),
                );
                ((&raw mut gSpriteCoordOffsetX).cast::<i16>()).write({
                    let __v3 = 0i16;
                    ((&raw mut gSpriteCoordOffsetY).cast::<i16>()).write(__v3);
                    __v3
                });
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    i = 0u8;
                    'l21: loop {
                        if !(((i) as u32) < (crate::c::div_u32(32u32, 8u32)).wrapping_sub(1u32)) {
                            break 'l21;
                        }
                        'l22: {
                            LoadCompressedSpriteSheet(
                                (((&raw const sSpriteSheets).cast::<u8>().cast_mut()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 8),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                LoadSpritePalettes(
                    ((&raw const sSpritePalettes).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16636)
                    .cast::<*mut u16>())
                .write(
                    (malloc_and_decompress(
                        (((&raw const sGround_Tilemap)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .cast::<u8>(),
                        &raw mut sizeOut,
                    ))
                    .cast::<u16>(),
                );
                ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16640)
                    .cast::<*mut u16>())
                .write(
                    (malloc_and_decompress(
                        (((&raw const sTrees_Tilemap)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .cast::<u8>(),
                        &raw mut sizeOut,
                    ))
                    .cast::<u16>(),
                );
                ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16644)
                    .cast::<*mut u16>())
                .write(
                    (malloc_and_decompress(
                        (((&raw const sBgMountains_Tilemap)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .cast::<u8>(),
                        &raw mut sizeOut,
                    ))
                    .cast::<u16>(),
                );
                ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16652)
                    .cast::<*mut u16>())
                .write(
                    (malloc_and_decompress(
                        (((&raw const sPylonPole_Tilemap)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .cast::<u8>(),
                        &raw mut sizeOut,
                    ))
                    .cast::<u16>(),
                );
                ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16648)
                    .cast::<*mut u16>())
                .write(
                    ((&raw const sPylonTop_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>(),
                );
                DecompressAndCopyTileDataToVram(
                    0u8,
                    (((&raw mut gCableCarBg_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((FreeTempTileDataBuffersIfPossible()) != 0) {
                    LoadPalette(
                        (((&raw mut gCableCarBg_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                        0u16,
                        128u16,
                    );
                    let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                CreateCableCarSprites();
                RunTasks();
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .read()) as i32)
                    == 7i32
                {
                    let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                } else {
                    if !((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(240))
                        .cast::<*mut u8>())
                    .read())
                    .is_null()
                    {
                        {
                            i = 0u8;
                            'l23: loop {
                                if !(((i) as i32) < 20i32) {
                                    break 'l23;
                                }
                                'l24: {
                                    if !(((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                        .wrapping_add(240))
                                    .cast::<*mut u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read())
                                    .is_null()
                                    {
                                        crate::c::bf_write(
                                            (((((((&raw mut gWeatherPtr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(240))
                                            .cast::<*mut u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                            .wrapping_add(5),
                                            2,
                                            2,
                                            (0u16) as i32,
                                        );
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                        (__p9).write(((__p9).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                CopyToBgTilemapBufferRect_ChangePalette(
                    1u8,
                    (((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16640)
                        .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                    0u8,
                    17u8,
                    32u8,
                    15u8,
                    17u8,
                );
                CopyToBgTilemapBufferRect_ChangePalette(
                    2u8,
                    (((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16644)
                        .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                    0u8,
                    0u8,
                    30u8,
                    20u8,
                    17u8,
                );
                CopyToBgTilemapBufferRect_ChangePalette(
                    3u8,
                    (((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16648)
                        .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                    0u8,
                    0u8,
                    5u8,
                    2u8,
                    17u8,
                );
                CopyToBgTilemapBufferRect_ChangePalette(
                    3u8,
                    (((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16652)
                        .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                    0u8,
                    2u8,
                    2u8,
                    20u8,
                    17u8,
                );
                let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                InitGroundTilemapData(
                    ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8),
                );
                CopyToBgTilemapBufferRect_ChangePalette(
                    0u8,
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16636)
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(72))
                    .cast::<u8>(),
                    0u8,
                    14u8,
                    12u8,
                    3u8,
                    17u8,
                );
                CopyToBgTilemapBufferRect_ChangePalette(
                    0u8,
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16636)
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(108))
                    .cast::<u8>(),
                    12u8,
                    17u8,
                    12u8,
                    3u8,
                    17u8,
                );
                CopyToBgTilemapBufferRect_ChangePalette(
                    0u8,
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16636)
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(144))
                    .cast::<u8>(),
                    24u8,
                    20u8,
                    12u8,
                    3u8,
                    17u8,
                );
                CopyToBgTilemapBufferRect_ChangePalette(
                    0u8,
                    (((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16636)
                        .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                    0u8,
                    17u8,
                    12u8,
                    3u8,
                    17u8,
                );
                CopyToBgTilemapBufferRect_ChangePalette(
                    0u8,
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16636)
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(36))
                    .cast::<u8>(),
                    0u8,
                    20u8,
                    12u8,
                    3u8,
                    17u8,
                );
                CopyToBgTilemapBufferRect_ChangePalette(
                    0u8,
                    (((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16636)
                        .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                    12u8,
                    20u8,
                    12u8,
                    3u8,
                    17u8,
                );
                CopyToBgTilemapBufferRect_ChangePalette(
                    0u8,
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16636)
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(36))
                    .cast::<u8>(),
                    12u8,
                    23u8,
                    12u8,
                    3u8,
                    17u8,
                );
                CopyToBgTilemapBufferRect_ChangePalette(
                    0u8,
                    (((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16636)
                        .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                    24u8,
                    23u8,
                    12u8,
                    3u8,
                    17u8,
                );
                let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                BeginNormalPaletteFade(4294967295u32, 3i8, 16u8, 0u8, 0u16);
                FadeInNewBGM(425u16, 1u8);
                SetBgRegs(1u8);
                let __p12 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                {
                    let mut imeTemp: u16 = 0u16;
                    imeTemp = ((67109384i32) as usize as *mut u16).read_volatile();
                    crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
                    let __p13 = ((67109376i32) as usize as *mut u16);
                    crate::c::volatile_write(
                        __p13,
                        (((((__p13).read_volatile()) as i32) | 1i32) as u16),
                    );
                    crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeTemp);
                }
                SetVBlankCallback(Some(VBlankCB_CableCar));
                SetMainCallback2(Some(CB2_CableCar));
                CreateTask(Some(Task_CableCar), 0u8);
                if !((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) != 0) {
                    (((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .write(CreateTask(Some(Task_AnimateBgGoingUp), 1u8));
                } else {
                    (((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .write(CreateTask(Some(Task_AnimateBgGoingDown), 1u8));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_CableCar() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
        MapMusicMain();
    }
}
pub(crate) unsafe extern "C" fn CB2_EndCableCar() {
    unsafe {
        let mut i: u8 = 0u8;
        HideBg(0u8);
        HideBg(1u8);
        HideBg(2u8);
        HideBg(3u8);
        SetBgRegs(0u8);
        ((&raw mut gSpriteCoordOffsetX).cast::<i16>()).write(0i16);
        SetCurrentAndNextWeatherNoDelay(0u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(240))
                        .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(core::ptr::null_mut());
                }
                i = (i).wrapping_add(1);
            }
        }
        ResetTasks();
        ResetSpriteData();
        ResetPaletteFade();
        UnsetBgTilemapBuffer(0u8);
        UnsetBgTilemapBuffer(1u8);
        UnsetBgTilemapBuffer(2u8);
        UnsetBgTilemapBuffer(3u8);
        ResetBgsAndClearDma3BusyFlags(0u32);
        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16648)
            .cast::<*mut u16>())
        .write(core::ptr::null_mut());
        {
            Free(
                (((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16652)
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
            );
            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16652)
                .cast::<*mut u16>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                (((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16644)
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
            );
            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16644)
                .cast::<*mut u16>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                (((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16640)
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
            );
            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16640)
                .cast::<*mut u16>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                (((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16636)
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
            );
            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16636)
                .cast::<*mut u16>())
            .write(core::ptr::null_mut());
        }
        {
            Free(((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        }
        {
            let mut _dest: *mut u8 = ((100663296i32) as usize as *mut u8);
            let mut _size: u32 = 98304u32;
            'l3: loop {
                if !((1i32) != 0) {
                    break 'l3;
                }
                'l4: loop {
                    'l5: {
                        {
                            let mut tmp: u16 = 0u16;
                            (&raw mut tmp).write_volatile(0u16);
                            'l6: loop {
                                'l7: {
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
                                    break 'l6;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l4;
                    }
                }
                _dest = (_dest).wrapping_offset(4096);
                _size = (_size).wrapping_sub(4096u32);
                if _size <= 4096u32 {
                    'l8: loop {
                        'l9: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l10: loop {
                                    'l11: {
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
                                        break 'l10;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l8;
                        }
                    }
                    break 'l3;
                }
            }
        }
        {
            let mut _dest: *mut u8 = ((117440512i32) as usize as *mut u8);
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
        {
            let mut _dest: *mut u8 = ((83886080i32) as usize as *mut u8);
            let mut _size: u32 = 1024u32;
            'l16: loop {
                'l17: {
                    {
                        let mut tmp: u16 = 0u16;
                        (&raw mut tmp).write_volatile(0u16);
                        'l18: loop {
                            'l19: {
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
                                break 'l18;
                            }
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l16;
                }
            }
        }
        WarpIntoMap();
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>()).write(None);
        SetMainCallback2(Some(CB2_LoadMap));
    }
}
pub(crate) unsafe extern "C" fn Task_CableCar(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let __p1 = (((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6)
            .cast::<u16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        'l1: {
            let __sw2 = ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32);
            if __sw2 == 0i32 {
                if ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6)
                    .cast::<u16>())
                .read()) as i32)
                    == ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<u16>())
                    .read()) as i32)
                {
                    SetNextWeather(
                        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2))
                        .read(),
                    );
                    ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1))
                    .write(1u8);
                }
                break 'l1;
            }
            if __sw2 == 1i32 {
                'l2: {
                    let __sw3 = ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2))
                    .read()) as i32);
                    if __sw3 == 7i32 {
                        if ((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                            .wrapping_add(240))
                        .cast::<*mut u8>())
                        .read()) as usize)
                            != 0usize)
                            && (((crate::c::bf_read(
                                ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                    .wrapping_add(240))
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(5),
                                2,
                                2,
                                false,
                            ) as u16) as i32)
                                != 0i32)
                        {
                            {
                                'l3: loop {
                                    if !(((i) as i32) < 20i32) {
                                        break 'l3;
                                    }
                                    'l4: {
                                        if !(((((((&raw mut gWeatherPtr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(240))
                                        .cast::<*mut u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .read())
                                        .is_null()
                                        {
                                            crate::c::bf_write(
                                                (((((((&raw mut gWeatherPtr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(240))
                                                .cast::<*mut u8>())
                                                .wrapping_offset(((i) as i32) as isize))
                                                .read())
                                                .wrapping_add(5),
                                                2,
                                                2,
                                                (0u16) as i32,
                                            );
                                        }
                                    }
                                    i = (i).wrapping_add(1);
                                }
                            }
                            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1))
                            .write(2u8);
                        }
                        break 'l2;
                    }
                    if __sw3 == 2i32 {
                        if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                            .wrapping_add(1744))
                        .read()) as i32)
                            == 2i32
                        {
                            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1))
                            .write(2u8);
                        } else {
                            if ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(6)
                                .cast::<u16>())
                            .read()) as i32)
                                >= ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4)
                                .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_add(8i32)
                            {
                                {
                                    'l5: loop {
                                        if !(((i) as i32) < 20i32) {
                                            break 'l5;
                                        }
                                        'l6: {
                                            if !(((((((&raw mut gWeatherPtr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(240))
                                            .cast::<*mut u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                            .is_null()
                                            {
                                                crate::c::bf_write(
                                                    (((((((&raw mut gWeatherPtr)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(240))
                                                    .cast::<*mut u8>())
                                                    .wrapping_offset(((i) as i32) as isize))
                                                    .read())
                                                    .wrapping_add(62),
                                                    2,
                                                    1,
                                                    ((((crate::c::bf_read(
                                                        (((((((&raw mut gWeatherPtr)
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(240))
                                                        .cast::<*mut u8>())
                                                        .wrapping_offset(((i) as i32) as isize))
                                                        .read())
                                                        .wrapping_add(62),
                                                        2,
                                                        1,
                                                        false,
                                                    )
                                                        as u16)
                                                        as i32)
                                                        ^ 1i32)
                                                        as u16)
                                                        as i32,
                                                );
                                            }
                                        }
                                        i = (i).wrapping_add(1);
                                    }
                                }
                            }
                        }
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw2 == 2i32 {
                if ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6)
                    .cast::<u16>())
                .read()) as i32)
                    == 570i32
                {
                    ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1))
                    .write(3u8);
                    BeginNormalPaletteFade(4294967295u32, 3i8, 0u8, 16u8, 0u16);
                    FadeOutBGM(4u8);
                }
                break 'l1;
            }
            if __sw2 == 3i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1))
                    .write(255u8);
                }
                break 'l1;
            }
            if __sw2 == 255i32 {
                SetVBlankCallback(None);
                DestroyTask(taskId);
                DestroyTask((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).read());
                SetMainCallback2(Some(CB2_EndCableCar));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_AnimateBgGoingUp(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .read()) as i32)
            != 255i32
        {
            let __p1 =
                (((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            if crate::c::rem_i32(
                ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6)
                    .cast::<u16>())
                .read()) as i32),
                2i32,
            ) == 0i32
            {
                let __p2 =
                    (((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21);
                (__p2).write(((__p2).read()).wrapping_sub(1));
            }
            if crate::c::rem_i32(
                ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6)
                    .cast::<u16>())
                .read()) as i32),
                8i32,
            ) == 0i32
            {
                let __p3 =
                    (((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12);
                (__p3).write(((__p3).read()).wrapping_sub(1));
                let __p4 =
                    (((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13);
                (__p4).write(((__p4).read()).wrapping_sub(1));
            }
            'l1: {
                let __sw5 = ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20))
                .read()) as i32);
                if __sw5 == 175i32 {
                    FillBgTilemapBufferRect(3u8, 0u16, 0u8, 22u8, 2u8, 10u8, 17u8);
                    break 'l1;
                }
                if __sw5 == 40i32 {
                    FillBgTilemapBufferRect(3u8, 0u16, 3u8, 0u8, 2u8, 2u8, 17u8);
                    break 'l1;
                }
                if __sw5 == 32i32 {
                    FillBgTilemapBufferRect(3u8, 0u16, 2u8, 0u8, 1u8, 2u8, 17u8);
                    break 'l1;
                }
                if __sw5 == 16i32 {
                    CopyToBgTilemapBufferRect_ChangePalette(
                        3u8,
                        (((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16648)
                            .cast::<*mut u16>())
                        .read())
                        .cast::<u8>(),
                        0u8,
                        0u8,
                        5u8,
                        2u8,
                        17u8,
                    );
                    CopyToBgTilemapBufferRect_ChangePalette(
                        3u8,
                        (((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16652)
                            .cast::<*mut u16>())
                        .read())
                        .cast::<u8>(),
                        0u8,
                        2u8,
                        2u8,
                        30u8,
                        17u8,
                    );
                    ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(21))
                    .write(64u8);
                    break 'l1;
                }
            }
        }
        AnimateGroundGoingUp();
        ((&raw mut gSpriteCoordOffsetX).cast::<i16>()).write(
            ((crate::c::rem_i32(
                ((((&raw mut gSpriteCoordOffsetX).cast::<i16>()).read()) as i32).wrapping_add(1i32),
                128i32,
            )) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_AnimateBgGoingDown(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .read()) as i32)
            != 255i32
        {
            let __p1 =
                (((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20);
            (__p1).write(((__p1).read()).wrapping_add(1));
            if crate::c::rem_i32(
                ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6)
                    .cast::<u16>())
                .read()) as i32),
                2i32,
            ) == 0i32
            {
                let __p2 =
                    (((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21);
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if crate::c::rem_i32(
                ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6)
                    .cast::<u16>())
                .read()) as i32),
                8i32,
            ) == 0i32
            {
                let __p3 =
                    (((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12);
                (__p3).write(((__p3).read()).wrapping_add(1));
                let __p4 =
                    (((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13);
                (__p4).write(((__p4).read()).wrapping_add(1));
            }
            'l1: {
                let __sw5 = ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20))
                .read()) as i32);
                if __sw5 == 176i32 {
                    CopyToBgTilemapBufferRect_ChangePalette(
                        3u8,
                        (((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16652)
                            .cast::<*mut u16>())
                        .read())
                        .cast::<u8>(),
                        0u8,
                        2u8,
                        2u8,
                        30u8,
                        17u8,
                    );
                    break 'l1;
                }
                if __sw5 == 16i32 {
                    FillBgTilemapBufferRect(3u8, 0u16, 2u8, 0u8, 3u8, 2u8, 17u8);
                    FillBgTilemapBufferRect(3u8, 0u16, 0u8, 22u8, 2u8, 10u8, 17u8);
                    ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(21))
                    .write(192u8);
                    break 'l1;
                }
                if __sw5 == 32i32 {
                    FillBgTilemapBufferRect(
                        3u8,
                        ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16648)
                            .cast::<*mut u16>())
                        .read())
                        .wrapping_offset(2))
                        .read(),
                        2u8,
                        0u8,
                        1u8,
                        1u8,
                        17u8,
                    );
                    FillBgTilemapBufferRect(
                        3u8,
                        ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16648)
                            .cast::<*mut u16>())
                        .read())
                        .wrapping_offset(3))
                        .read(),
                        3u8,
                        0u8,
                        1u8,
                        1u8,
                        17u8,
                    );
                    FillBgTilemapBufferRect(
                        3u8,
                        ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16648)
                            .cast::<*mut u16>())
                        .read())
                        .wrapping_offset(7))
                        .read(),
                        2u8,
                        1u8,
                        1u8,
                        1u8,
                        17u8,
                    );
                    FillBgTilemapBufferRect(
                        3u8,
                        ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16648)
                            .cast::<*mut u16>())
                        .read())
                        .wrapping_offset(8))
                        .read(),
                        3u8,
                        1u8,
                        1u8,
                        1u8,
                        17u8,
                    );
                    break 'l1;
                }
                if __sw5 == 40i32 {
                    FillBgTilemapBufferRect(
                        3u8,
                        ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16648)
                            .cast::<*mut u16>())
                        .read())
                        .wrapping_offset(4))
                        .read(),
                        4u8,
                        0u8,
                        1u8,
                        1u8,
                        17u8,
                    );
                    FillBgTilemapBufferRect(
                        3u8,
                        ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16648)
                            .cast::<*mut u16>())
                        .read())
                        .wrapping_offset(9))
                        .read(),
                        4u8,
                        1u8,
                        1u8,
                        1u8,
                        17u8,
                    );
                    break 'l1;
                }
            }
        }
        AnimateGroundGoingDown();
        if ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6)
            .cast::<u16>())
        .read()) as i32)
            < ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32)
        {
            ((&raw mut gSpriteCoordOffsetX).cast::<i16>()).write(
                ((crate::c::rem_i32(
                    ((((&raw mut gSpriteCoordOffsetX).cast::<i16>()).read()) as i32)
                        .wrapping_add(247i32),
                    248i32,
                )) as i16),
            );
        } else {
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1788)
                .cast::<u16>())
            .write(
                ((crate::c::rem_i32(
                    ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1788)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_add(247i32),
                    248i32,
                )) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_CableCar() {
    unsafe {
        CopyBgTilemapBufferToVram(0u8);
        CopyBgTilemapBufferToVram(3u8);
        SetGpuReg(
            28u8,
            ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20))
                .read()) as u16),
        );
        SetGpuReg(
            30u8,
            ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21))
                .read()) as u16),
        );
        SetGpuReg(
            20u8,
            ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
                .read()) as u16),
        );
        SetGpuReg(
            22u8,
            ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13))
                .read()) as u16),
        );
        SetGpuReg(
            16u8,
            ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                .read()) as u16),
        );
        SetGpuReg(
            18u8,
            ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
                .read()) as u16),
        );
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Cable(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CableCar(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .read()) as i32)
            != 255i32
        {
            if !((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) != 0) {
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_sub(
                        (((((0.14f32) as f32)
                            * (({
                                let __v1 =
                                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(6)
                                    .cast::<u16>())
                                    .read()) as i16);
                                let mut f = __v1 as f32;
                                if __v1 < 0 {
                                    f += 65536.0;
                                }
                                f
                            }) as f32)) as u8) as i32),
                    )) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        .wrapping_sub(
                            (((((0.067f32) as f32)
                                * (({
                                    let __v2 =
                                        ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(6)
                                        .cast::<u16>())
                                        .read()) as i16);
                                    let mut f = __v2 as f32;
                                    if __v2 < 0 {
                                        f += 65536.0;
                                    }
                                    f
                                }) as f32)) as u8) as i32),
                        )) as i16),
                );
            } else {
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(
                        (((((0.14f32) as f32)
                            * (({
                                let __v3 =
                                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(6)
                                    .cast::<u16>())
                                    .read()) as i16);
                                let mut f = __v3 as f32;
                                if __v3 < 0 {
                                    f += 65536.0;
                                }
                                f
                            }) as f32)) as u8) as i32),
                    )) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        .wrapping_add(
                            (((((0.067f32) as f32)
                                * (({
                                    let __v4 =
                                        ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(6)
                                        .cast::<u16>())
                                        .read()) as i16);
                                    let mut f = __v4 as f32;
                                    if __v4 < 0 {
                                        f += 65536.0;
                                    }
                                    f
                                }) as f32)) as u8) as i32),
                        )) as i16),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Player(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .read()) as i32)
            != 255i32
        {
            if !((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) != 0) {
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_sub(
                        (((((0.14f32) as f32)
                            * (({
                                let __v1 =
                                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(6)
                                    .cast::<u16>())
                                    .read()) as i16);
                                let mut f = __v1 as f32;
                                if __v1 < 0 {
                                    f += 65536.0;
                                }
                                f
                            }) as f32)) as u8) as i32),
                    )) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        .wrapping_sub(
                            (((((0.067f32) as f32)
                                * (({
                                    let __v2 =
                                        ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(6)
                                        .cast::<u16>())
                                        .read()) as i16);
                                    let mut f = __v2 as f32;
                                    if __v2 < 0 {
                                        f += 65536.0;
                                    }
                                    f
                                }) as f32)) as u8) as i32),
                        )) as i16),
                );
            } else {
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(
                        (((((0.14f32) as f32)
                            * (({
                                let __v3 =
                                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(6)
                                    .cast::<u16>())
                                    .read()) as i16);
                                let mut f = __v3 as f32;
                                if __v3 < 0 {
                                    f += 65536.0;
                                }
                                f
                            }) as f32)) as u8) as i32),
                    )) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        .wrapping_add(
                            (((((0.067f32) as f32)
                                * (({
                                    let __v4 =
                                        ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(6)
                                        .cast::<u16>())
                                        .read()) as i16);
                                    let mut f = __v4 as f32;
                                    if __v4 < 0 {
                                        f += 65536.0;
                                    }
                                    f
                                }) as f32)) as u8) as i32),
                        )) as i16),
                );
            }
            'l1: {
                let __sw5 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32);
                let __matched = __sw5 == 0i32;
                if __sw5 == 0i32 {
                    ((sprite).wrapping_add(38).cast::<i16>()).write(17i16);
                    if (({
                        let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                        let __t7 = (__p6).read();
                        (__p6).write(((__p6).read()).wrapping_add(1));
                        __t7
                    }) as i32)
                        > 9i32
                    {
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                            .write(0i16);
                        let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                        (__p8).write(((__p8).read()).wrapping_add(1));
                    }
                    break 'l1;
                }
                if !__matched {
                    ((sprite).wrapping_add(38).cast::<i16>()).write(16i16);
                    if (({
                        let __p9 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                        let __t10 = (__p9).read();
                        (__p9).write(((__p9).read()).wrapping_add(1));
                        __t10
                    }) as i32)
                        > 9i32
                    {
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                            .write(0i16);
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                            .write(0i16);
                    }
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_HikerGoingUp(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    (2i32).wrapping_mul(((((sprite).wrapping_add(40).cast::<i8>()).read()) as i32)),
                )) as i16),
            );
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    (16i32)
                        .wrapping_add(((((sprite).wrapping_add(41).cast::<i8>()).read()) as i32)),
                )) as i16),
            );
        }
        if (({
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            >= ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
        {
            'l1: {
                let __sw5 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32);
                if __sw5 == 0i32 {
                    let __p6 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                    if crate::c::rem_i32(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                        4i32,
                    ) == 0i32
                    {
                        let __p7 = (sprite).wrapping_add(34).cast::<i16>();
                        (__p7).write(((__p7).read()).wrapping_add(1));
                    }
                    break 'l1;
                }
                if __sw5 == 1i32 {
                    if crate::c::rem_i32(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                        2i32,
                    ) != 0i32
                    {
                        let __p8 = (sprite).wrapping_add(32).cast::<i16>();
                        (__p8).write(((__p8).read()).wrapping_add(1));
                        if crate::c::rem_i32(
                            ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32),
                            4i32,
                        ) == 0i32
                        {
                            let __p9 = (sprite).wrapping_add(34).cast::<i16>();
                            (__p9).write(((__p9).read()).wrapping_add(1));
                        }
                    }
                    break 'l1;
                }
            }
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) > 160i32 {
                DestroySprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_HikerGoingDown(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            let __p1 = (sprite).wrapping_add(34).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    (16i32)
                        .wrapping_add(((((sprite).wrapping_add(41).cast::<i8>()).read()) as i32)),
                )) as i16),
            );
        }
        if (({
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t3 = ((__p2).read()).wrapping_add(1);
            (__p2).write(__t3);
            __t3
        }) as i32)
            >= ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
        {
            'l1: {
                let __sw4 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32);
                if __sw4 == 0i32 {
                    let __p5 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_sub(1));
                    if crate::c::rem_i32(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                        4i32,
                    ) == 0i32
                    {
                        let __p6 = (sprite).wrapping_add(34).cast::<i16>();
                        (__p6).write(((__p6).read()).wrapping_sub(1));
                    }
                    break 'l1;
                }
                if __sw4 == 1i32 {
                    if crate::c::rem_i32(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                        2i32,
                    ) != 0i32
                    {
                        let __p7 = (sprite).wrapping_add(32).cast::<i16>();
                        (__p7).write(((__p7).read()).wrapping_sub(1));
                        if crate::c::rem_i32(
                            ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32),
                            4i32,
                        ) == 0i32
                        {
                            let __p8 = (sprite).wrapping_add(34).cast::<i16>();
                            (__p8).write(((__p8).read()).wrapping_sub(1));
                        }
                    }
                    break 'l1;
                }
            }
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) < 80i32 {
                DestroySprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetBgRegs(active: u8) {
    unsafe {
        let mut active = active;
        'l1: {
            let __sw1 = ((active) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 || !__matched {
                SetGpuReg(72u8, 0u16);
                SetGpuReg(74u8, 0u16);
                SetGpuReg(64u8, 0u16);
                SetGpuReg(66u8, 0u16);
                SetGpuReg(68u8, 0u16);
                SetGpuReg(70u8, 0u16);
                SetGpuReg(0u8, 0u16);
                SetGpuReg(14u8, 0u16);
                SetGpuReg(12u8, 0u16);
                SetGpuReg(10u8, 0u16);
                SetGpuReg(8u8, 0u16);
                SetGpuReg(28u8, 0u16);
                SetGpuReg(30u8, 0u16);
                SetGpuReg(24u8, 0u16);
                SetGpuReg(26u8, 0u16);
                SetGpuReg(20u8, 0u16);
                SetGpuReg(22u8, 0u16);
                SetGpuReg(16u8, 0u16);
                SetGpuReg(18u8, 0u16);
                SetGpuReg(80u8, 0u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetGpuReg(72u8, 0u16);
                SetGpuReg(74u8, 0u16);
                SetGpuReg(64u8, 0u16);
                SetGpuReg(66u8, 0u16);
                SetGpuReg(68u8, 0u16);
                SetGpuReg(70u8, 0u16);
                if !((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) != 0) {
                    ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20))
                    .write(176u8);
                    ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(21))
                    .write(16u8);
                    ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .write(0u8);
                    ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(13))
                    .write(80u8);
                    ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(9))
                    .write(0u8);
                    ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(9))
                    .write(0u8);
                } else {
                    ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20))
                    .write(96u8);
                    ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(21))
                    .write(232u8);
                    ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .write(0u8);
                    ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(13))
                    .write(4u8);
                    ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(9))
                    .write(0u8);
                    ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(9))
                    .write(0u8);
                }
                SetGpuReg(
                    28u8,
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20))
                    .read()) as u16),
                );
                SetGpuReg(
                    30u8,
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(21))
                    .read()) as u16),
                );
                SetGpuReg(24u8, 0u16);
                SetGpuReg(26u8, 0u16);
                SetGpuReg(
                    20u8,
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .read()) as u16),
                );
                SetGpuReg(
                    22u8,
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(13))
                    .read()) as u16),
                );
                SetGpuReg(
                    16u8,
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8))
                    .read()) as u16),
                );
                SetGpuReg(
                    18u8,
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(9))
                    .read()) as u16),
                );
                SetGpuReg(0u8, 4160u16);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(2u8);
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                ShowBg(3u8);
                SetGpuReg(80u8, 16128u16);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateCableCarSprites() {
    unsafe {
        let mut spriteId: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut playerGraphicsIds = crate::ffi::Align4([0u8; 2]);
        (&raw mut playerGraphicsIds)
            .cast::<u8>()
            .wrapping_add(0)
            .write(100u8);
        (&raw mut playerGraphicsIds)
            .cast::<u8>()
            .wrapping_add(1)
            .write(105u8);
        let mut rval: u16 = Random();
        let mut hikerGraphicsIds = crate::ffi::Align4([0u8; 4]);
        (&raw mut hikerGraphicsIds)
            .cast::<u8>()
            .wrapping_add(0)
            .write(55u8);
        (&raw mut hikerGraphicsIds)
            .cast::<u8>()
            .wrapping_add(1)
            .write(31u8);
        (&raw mut hikerGraphicsIds)
            .cast::<u8>()
            .wrapping_add(2)
            .write(32u8);
        (&raw mut hikerGraphicsIds)
            .cast::<u8>()
            .wrapping_add(3)
            .write(98u8);
        let mut hikerCoords = crate::ffi::Align4([0u8; 8]);
        (&raw mut hikerCoords)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(0)
            .cast::<i16>()
            .write(0i16);
        (&raw mut hikerCoords)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(2)
            .cast::<i16>()
            .write(80i16);
        (&raw mut hikerCoords)
            .cast::<u8>()
            .wrapping_add(4)
            .wrapping_add(0)
            .cast::<i16>()
            .write(240i16);
        (&raw mut hikerCoords)
            .cast::<u8>()
            .wrapping_add(4)
            .wrapping_add(2)
            .cast::<i16>()
            .write(146i16);
        let mut hikerMovementDelayTable = crate::ffi::Align4([0u8; 4]);
        (&raw mut hikerMovementDelayTable)
            .cast::<u8>()
            .wrapping_add(0)
            .write(0u8);
        (&raw mut hikerMovementDelayTable)
            .cast::<u8>()
            .wrapping_add(1)
            .write(60u8);
        (&raw mut hikerMovementDelayTable)
            .cast::<u8>()
            .wrapping_add(2)
            .write(120u8);
        (&raw mut hikerMovementDelayTable)
            .cast::<u8>()
            .wrapping_add(3)
            .write(170u8);
        let mut hikerCallbacks = crate::ffi::Align4([0u8; 8]);
        (&raw mut hikerCallbacks)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>()
            .write(Some(SpriteCB_HikerGoingUp));
        (&raw mut hikerCallbacks)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>()
            .write(Some(SpriteCB_HikerGoingDown));
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 || !__matched {
                spriteId = CreateObjectGraphicsSprite(
                    (((((&raw mut playerGraphicsIds).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8))
                            .read()) as i32) as isize,
                    ))
                    .read()) as u16),
                    Some(SpriteCB_Player),
                    200i16,
                    73i16,
                    102u8,
                );
                if ((spriteId) as i32) != 64i32 {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(5),
                        2,
                        2,
                        (2u16) as i32,
                    );
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(8i16);
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(38)
                    .cast::<i16>())
                    .write(16i16);
                    (((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(200i16);
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(73i16);
                }
                spriteId = CreateSprite(
                    ((&raw const sSpriteTemplates_CableCar)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    176i16,
                    43i16,
                    103u8,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write({
                    let __v2 = 32i16;
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(38)
                    .cast::<i16>())
                    .write(__v2);
                    __v2
                });
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(176i16);
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(43i16);
                spriteId = CreateSprite(
                    (((&raw const sSpriteTemplates_CableCar)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(24),
                    200i16,
                    99i16,
                    101u8,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(8i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(4i16);
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(200i16);
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(99i16);
                ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                    .write(7u8);
                ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>())
                .write(350u16);
                SetCurrentAndNextWeatherNoDelay(2u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                CopyToBgTilemapBufferRect_ChangePalette(
                    0u8,
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16636)
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(36))
                    .cast::<u8>(),
                    24u8,
                    26u8,
                    12u8,
                    3u8,
                    17u8,
                );
                spriteId = CreateObjectGraphicsSprite(
                    (((((&raw mut playerGraphicsIds).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8))
                            .read()) as i32) as isize,
                    ))
                    .read()) as u16),
                    Some(SpriteCB_Player),
                    128i16,
                    39i16,
                    102u8,
                );
                if ((spriteId) as i32) != 64i32 {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(5),
                        2,
                        2,
                        (2u16) as i32,
                    );
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(8i16);
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(38)
                    .cast::<i16>())
                    .write(16i16);
                    (((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(128i16);
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(39i16);
                }
                spriteId = CreateSprite(
                    ((&raw const sSpriteTemplates_CableCar)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    104i16,
                    9i16,
                    103u8,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write({
                    let __v3 = 32i16;
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(38)
                    .cast::<i16>())
                    .write(__v3);
                    __v3
                });
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(104i16);
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(9i16);
                spriteId = CreateSprite(
                    (((&raw const sSpriteTemplates_CableCar)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(24),
                    128i16,
                    65i16,
                    101u8,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(8i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(4i16);
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(128i16);
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(65i16);
                ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                    .write(2u8);
                ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>())
                .write(265u16);
                SetCurrentAndNextWeatherNoDelay(7u8);
                break 'l1;
            }
        }
        {
            i = 0u8;
            'l2: loop {
                if !(((i) as i32) < 9i32) {
                    break 'l2;
                }
                'l3: {
                    spriteId = CreateSprite(
                        (&raw const sSpriteTemplate_Cable).cast::<u8>().cast_mut(),
                        ((((16i32).wrapping_mul(((i) as i32))).wrapping_add(96i32)) as i16),
                        ((((8i32).wrapping_mul(((i) as i32))).wrapping_sub(8i32)) as i16),
                        104u8,
                    );
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(8i16);
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(38)
                    .cast::<i16>())
                    .write(8i16);
                }
                i = (i).wrapping_add(1);
            }
        }
        if crate::c::rem_i32(((rval) as i32), 64i32) == 0i32 {
            spriteId = CreateObjectGraphicsSprite(
                (((((&raw mut hikerGraphicsIds).cast::<u8>()).wrapping_offset(
                    ((crate::c::rem_u32(
                        ((rval) as u32),
                        (crate::c::div_u32(4u32, 1u32)).wrapping_sub(1u32),
                    )) as i32) as isize,
                ))
                .read()) as u16),
                (((&raw mut hikerCallbacks).cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .wrapping_offset(
                        ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize,
                    ))
                .read(),
                ((((&raw mut hikerCoords).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 4,
                ))
                .cast::<i16>())
                .read(),
                (((((&raw mut hikerCoords).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 4,
                ))
                .cast::<i16>())
                .wrapping_offset(1))
                .read(),
                106u8,
            );
            if ((spriteId) as i32) != 64i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(5),
                    2,
                    2,
                    (2u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(
                    ((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(40)
                    .cast::<i8>())
                    .read()) as i32)
                        .wrapping_neg()) as i16),
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(
                    ((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(41)
                    .cast::<i8>())
                    .read()) as i32)
                        .wrapping_neg()) as i16),
                );
                if !((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) != 0) {
                    if (crate::c::rem_i32(((rval) as i32), 2i32)) != 0 {
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                            6u8,
                        );
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(1i16);
                        let __p4 = (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(34)
                        .cast::<i16>();
                        (__p4).write((((((__p4).read()) as i32).wrapping_add(2i32)) as i16));
                    } else {
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                            7u8,
                        );
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(0i16);
                    }
                } else {
                    if (crate::c::rem_i32(((rval) as i32), 2i32)) != 0 {
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                            7u8,
                        );
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(1i16);
                        let __p5 = (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(34)
                        .cast::<i16>();
                        (__p5).write((((((__p5).read()) as i32).wrapping_add(2i32)) as i16));
                    } else {
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                            6u8,
                        );
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(0i16);
                    }
                }
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(
                    (((((&raw mut hikerMovementDelayTable).cast::<u8>()).wrapping_offset(
                        ((crate::c::rem_u32(((rval) as u32), crate::c::div_u32(4u32, 1u32))) as i32)
                            as isize,
                    ))
                    .read()) as i16),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BufferNextGroundSegment() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut k: u8 = 0u8;
        let mut offset: u8 = 0u8;
        {
            i = 0u8;
            k = 0u8;
            offset = (((36i32).wrapping_mul(
                ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(27))
                    .read()) as i32)
                    .wrapping_add(2i32),
            )) as u8);
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as u32) < crate::c::div_u32(24u32, 2u32)) {
                                break 'l3;
                            }
                            'l4: {
                                ((((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(34))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 24))
                                .cast::<u16>())
                                .wrapping_offset(((j) as i32) as isize))
                                .write(
                                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(16636)
                                    .cast::<*mut u16>())
                                    .read())
                                    .wrapping_offset(
                                        (({
                                            let __t1 = offset;
                                            offset = (offset).wrapping_add(1);
                                            __t1
                                        }) as i32) as isize,
                                    ))
                                    .read(),
                                );
                                ((((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(34))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((i) as i32).wrapping_add(3i32)) as isize * 24,
                                ))
                                .cast::<u16>())
                                .wrapping_offset(((j) as i32) as isize))
                                .write(
                                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(16636)
                                    .cast::<*mut u16>())
                                    .read())
                                    .wrapping_offset(((k) as i32) as isize))
                                    .read(),
                                );
                                ((((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(34))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((i) as i32).wrapping_add(6i32)) as isize * 24,
                                ))
                                .cast::<u16>())
                                .wrapping_offset(((j) as i32) as isize))
                                .write(
                                    (((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(16636)
                                    .cast::<*mut u16>())
                                    .read())
                                    .wrapping_offset(36))
                                    .wrapping_offset(((k) as i32) as isize))
                                    .read(),
                                );
                                k = (k).wrapping_add(1);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(27)).write(
            ((crate::c::rem_i32(
                ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(27))
                    .read()) as i32)
                    .wrapping_add(1i32),
                3i32,
            )) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimateGroundGoingUp() {
    unsafe {
        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(28)).write(
            ((crate::c::rem_i32(
                ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(28))
                    .read()) as i32)
                    .wrapping_add(1i32),
                96i32,
            )) as u8),
        );
        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8)).write(
            ((((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31))
                .read()) as i32)
                .wrapping_sub(
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(29))
                    .read()) as i32),
                )) as u8),
        );
        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9)).write(
            ((((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32))
                .read()) as i32)
                .wrapping_sub(
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(30))
                    .read()) as i32),
                )) as u8),
        );
        let __p1 = (((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(29);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if crate::c::rem_i32(
            ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(29))
                .read()) as i32),
            4i32,
        ) == 0i32
        {
            let __p2 =
                (((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(30);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        if ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(29))
            .read()) as i32)
            > 16i32
        {
            DrawNextGroundSegmentGoingUp();
        }
    }
}
pub(crate) unsafe extern "C" fn AnimateGroundGoingDown() {
    unsafe {
        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(28)).write(
            ((crate::c::rem_i32(
                ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(28))
                    .read()) as i32)
                    .wrapping_add(1i32),
                96i32,
            )) as u8),
        );
        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8)).write(
            ((((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31))
                .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(29))
                    .read()) as i32),
                )) as u8),
        );
        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9)).write(
            ((((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32))
                .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(30))
                    .read()) as i32),
                )) as u8),
        );
        let __p1 = (((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(29);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if crate::c::rem_i32(
            ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(29))
                .read()) as i32),
            4i32,
        ) == 0i32
        {
            let __p2 =
                (((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(30);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        if ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(29))
            .read()) as i32)
            > 16i32
        {
            DrawNextGroundSegmentGoingDown();
        }
    }
}
pub(crate) unsafe extern "C" fn DrawNextGroundSegmentGoingUp() {
    unsafe {
        let mut i: u8 = 0u8;
        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(29)).write({
            let __v1 = 0u8;
            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(30))
                .write(__v1);
            __v1
        });
        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31)).write(
            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8)).read(),
        );
        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32)).write(
            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9)).read(),
        );
        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25)).write(
            ((crate::c::rem_i32(
                ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                    .read()) as i32)
                    .wrapping_add(30i32),
                32i32,
            )) as u8),
        );
        let __p2 = (((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(24);
        (__p2).write((((((__p2).read()) as i32).wrapping_sub(2i32)) as u8));
        ((&raw mut sGroundSegmentY_Up).cast::<u8>().cast::<u8>()).write(
            ((crate::c::rem_i32(
                ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26))
                    .read()) as i32)
                    .wrapping_add(23i32),
                32i32,
            )) as u8),
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(216u32, 24u32)) {
                    break 'l1;
                }
                'l2: {
                    ((&raw mut sGroundX_Up).cast::<u8>().cast::<u8>()).write(
                        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(25))
                        .read(),
                    );
                    ((&raw mut sGroundY_Up).cast::<u8>().cast::<u8>()).write(
                        ((crate::c::rem_i32(
                            ((((&raw mut sGroundSegmentY_Up).cast::<u8>().cast::<u8>()).read())
                                as i32)
                                .wrapping_add(((i) as i32)),
                            32i32,
                        )) as u8),
                    );
                    FillBgTilemapBufferRect(
                        0u8,
                        ((((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(34))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 24))
                        .cast::<u16>())
                        .wrapping_offset(
                            ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(24))
                            .read()) as i32) as isize,
                        ))
                        .read(),
                        ((&raw mut sGroundX_Up).cast::<u8>().cast::<u8>()).read(),
                        ((&raw mut sGroundY_Up).cast::<u8>().cast::<u8>()).read(),
                        1u8,
                        1u8,
                        17u8,
                    );
                    ((&raw mut sGroundX_Up).cast::<u8>().cast::<u8>()).write(
                        ((crate::c::rem_i32(
                            ((((&raw mut sGroundX_Up).cast::<u8>().cast::<u8>()).read()) as i32)
                                .wrapping_add(1i32),
                            32i32,
                        )) as u8),
                    );
                    FillBgTilemapBufferRect(
                        0u8,
                        ((((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(34))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 24))
                        .cast::<u16>())
                        .wrapping_offset(
                            (((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(24))
                            .read()) as i32)
                                .wrapping_add(1i32)) as isize,
                        ))
                        .read(),
                        ((&raw mut sGroundX_Up).cast::<u8>().cast::<u8>()).read(),
                        ((&raw mut sGroundY_Up).cast::<u8>().cast::<u8>()).read(),
                        1u8,
                        1u8,
                        17u8,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut sGroundX_Up).cast::<u8>().cast::<u8>()).write(
            ((crate::c::rem_i32(
                ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                    .read()) as i32)
                    .wrapping_add(30i32),
                32i32,
            )) as u8),
        );
        FillBgTilemapBufferRect(
            0u8,
            0u16,
            ((&raw mut sGroundX_Up).cast::<u8>().cast::<u8>()).read(),
            0u8,
            2u8,
            32u8,
            17u8,
        );
        if ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(24))
            .read()) as i32)
            == 0i32
        {
            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26))
                .write(
                    ((crate::c::rem_i32(
                        ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(26))
                        .read()) as i32)
                            .wrapping_add(29i32),
                        32i32,
                    )) as u8),
                );
            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(24))
                .write(12u8);
            BufferNextGroundSegment();
            ((&raw mut sGroundX_Up).cast::<u8>().cast::<u8>()).write(
                ((crate::c::rem_i32(
                    ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(26))
                    .read()) as i32)
                        .wrapping_add(1i32),
                    32i32,
                )) as u8),
            );
            FillBgTilemapBufferRect(
                0u8,
                0u16,
                0u8,
                ((&raw mut sGroundX_Up).cast::<u8>().cast::<u8>()).read(),
                32u8,
                9u8,
                17u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DrawNextGroundSegmentGoingDown() {
    unsafe {
        let mut i: u8 = 0u8;
        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(29)).write({
            let __v1 = 0u8;
            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(30))
                .write(__v1);
            __v1
        });
        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31)).write(
            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8)).read(),
        );
        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32)).write(
            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9)).read(),
        );
        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25)).write(
            ((crate::c::rem_i32(
                ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                    .read()) as i32)
                    .wrapping_add(2i32),
                32i32,
            )) as u8),
        );
        let __p2 = (((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(24);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(2i32)) as u8));
        ((&raw mut sGroundSegmentY_Down).cast::<u8>().cast::<u8>()).write(
            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26))
                .read(),
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(216u32, 24u32)) {
                    break 'l1;
                }
                'l2: {
                    ((&raw mut sGroundX_Down).cast::<u8>().cast::<u8>()).write(
                        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(25))
                        .read(),
                    );
                    ((&raw mut sGroundY_Down).cast::<u8>().cast::<u8>()).write(
                        ((crate::c::rem_i32(
                            ((((&raw mut sGroundSegmentY_Down).cast::<u8>().cast::<u8>()).read())
                                as i32)
                                .wrapping_add(((i) as i32)),
                            32i32,
                        )) as u8),
                    );
                    FillBgTilemapBufferRect(
                        0u8,
                        ((((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(34))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 24))
                        .cast::<u16>())
                        .wrapping_offset(
                            ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(24))
                            .read()) as i32) as isize,
                        ))
                        .read(),
                        ((&raw mut sGroundX_Down).cast::<u8>().cast::<u8>()).read(),
                        ((&raw mut sGroundY_Down).cast::<u8>().cast::<u8>()).read(),
                        1u8,
                        1u8,
                        17u8,
                    );
                    ((&raw mut sGroundX_Down).cast::<u8>().cast::<u8>()).write(
                        ((crate::c::rem_i32(
                            ((((&raw mut sGroundX_Down).cast::<u8>().cast::<u8>()).read()) as i32)
                                .wrapping_add(1i32),
                            32i32,
                        )) as u8),
                    );
                    FillBgTilemapBufferRect(
                        0u8,
                        ((((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(34))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 24))
                        .cast::<u16>())
                        .wrapping_offset(
                            (((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(24))
                            .read()) as i32)
                                .wrapping_add(1i32)) as isize,
                        ))
                        .read(),
                        ((&raw mut sGroundX_Down).cast::<u8>().cast::<u8>()).read(),
                        ((&raw mut sGroundY_Down).cast::<u8>().cast::<u8>()).read(),
                        1u8,
                        1u8,
                        17u8,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut sGroundY_Down).cast::<u8>().cast::<u8>()).write(
            ((crate::c::rem_i32(
                ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26))
                    .read()) as i32)
                    .wrapping_add(23i32),
                32i32,
            )) as u8),
        );
        FillBgTilemapBufferRect(
            0u8,
            0u16,
            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                .read(),
            ((&raw mut sGroundY_Down).cast::<u8>().cast::<u8>()).read(),
            2u8,
            9u8,
            17u8,
        );
        if ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(24))
            .read()) as i32)
            == 10i32
        {
            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26))
                .write(
                    ((crate::c::rem_i32(
                        ((((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(26))
                        .read()) as i32)
                            .wrapping_add(3i32),
                        32i32,
                    )) as u8),
                );
            ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(24))
                .write(254u8);
            BufferNextGroundSegment();
        }
    }
}
pub(crate) unsafe extern "C" fn InitGroundTilemapData(goingDown: u8) {
    unsafe {
        let mut goingDown = goingDown;
        'l1: {
            let __sw1 = ((goingDown) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 || !__matched {
                ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(27))
                    .write(2u8);
                ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                    .write(0u8);
                ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26))
                    .write(20u8);
                ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(24))
                    .write(12u8);
                BufferNextGroundSegment();
                DrawNextGroundSegmentGoingUp();
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(27))
                    .write(2u8);
                ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                    .write(28u8);
                ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26))
                    .write(20u8);
                ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(24))
                    .write(4u8);
                BufferNextGroundSegment();
                DrawNextGroundSegmentGoingDown();
                break 'l1;
            }
        }
        ((((&raw mut sCableCar).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(28))
            .write(0u8);
    }
}
