//! Translated from `src/field_weather_effect.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gCloudsWeatherPalette gSandstormWeatherPalette gWeatherFogDiagonalTiles gWeatherFogHorizontalTiles gWeatherCloudTiles gWeatherSnow1Tiles gWeatherSnow2Tiles gWeatherBubbleTiles gWeatherAshTiles gWeatherRainTiles gWeatherSandstormTiles sCloudSpriteMapCoords sCloudSpriteSheet sCloudSpriteOamData sCloudSpriteAnimCmd sCloudSpriteAnimCmds sCloudSpriteTemplate sRainSpriteCoords sRainSpriteOamData sRainSpriteFallAnimCmd sRainSpriteSplashAnimCmd sRainSpriteHeavySplashAnimCmd sRainSpriteAnimCmds sRainSpriteTemplate sRainSpriteMovement sRainSpriteFallingDurations sRainSpriteSheet sSnowflakeSpriteOamData sSnowflakeSpriteImages sSnowflakeAnimCmd0 sSnowflakeAnimCmd1 sSnowflakeAnimCmds sSnowflakeSpriteTemplate sUnusedData sOamData_FogH sAnim_FogH_0 sAnim_FogH_1 sAnim_FogH_2 sAnim_FogH_3 sAnim_FogH_4 sAnim_FogH_5 sAnims_FogH sAffineAnim_FogH sAffineAnims_FogH sFogHorizontalSpriteTemplate sAshSpriteSheet sAshSpriteOamData sAshSpriteAnimCmd0 sAshSpriteAnimCmds sAshSpriteTemplate sFogDiagonalSpriteSheet sFogDiagonalSpriteOamData sFogDiagonalSpriteAnimCmd0 sFogDiagonalSpriteAnimCmds sFogDiagonalSpriteTemplate sSandstormSpriteOamData sSandstormSpriteAnimCmd0 sSandstormSpriteAnimCmd1 sSandstormSpriteAnimCmds sSandstormSpriteTemplate sSandstormSpriteSheet sSwirlEntranceDelays sBubbleStartDelays sWeatherBubbleSpriteSheet sBubbleStartCoords sBubbleSpriteAnimCmd0 sBubbleSpriteAnimCmds sBubbleSpriteTemplate sWeatherCycleRoute119 sWeatherCycleRoute123
#[allow(unused_imports)]
use crate::data::field_weather_effect::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCurrentAbnormalWeather: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnusedWeatherRelated: u16 = 0u16;

unsafe extern "C" {
    static mut gMapHeader: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSineTable: u8;
    static mut gSpriteCoordOffsetX: u8;
    static mut gSpriteCoordOffsetY: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gWeatherPtr: u8;
    fn ApplyWeatherColorMapIfIdle(a0: i8);
    fn ApplyWeatherColorMapIfIdle_Gradual(a0: u8, a1: u8, a2: u8);
    fn CalcCenterToCornerVec(a0: *mut u8, a1: u8, a2: u8, a3: u8);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateSpriteAtEnd(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DroughtStateInit();
    fn DroughtStateRun();
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FreeSpriteTilesByTag(a0: u16);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn IncrementGameStat(a0: u8);
    fn IsSEPlaying() -> u8;
    fn LoadCustomWeatherSpritePalette(a0: *mut u16);
    fn LoadDroughtWeatherPalettes() -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn PlaySE(a0: u16);
    fn Random() -> u16;
    fn ResetDroughtWeatherPaletteLoading();
    fn ScriptContext_Enable();
    fn SetCurrentAndNextWeather(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetNextWeather(a0: u8);
    fn SetRainStrengthFromSoundEffect(a0: u16);
    fn SetSpritePosToMapCoords(a0: i16, a1: i16, a2: *mut i16, a3: *mut i16);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn Weather_SetBlendCoeffs(a0: u8, a1: u8);
    fn Weather_SetTargetBlendCoeffs(a0: u8, a1: u8, a2: i32);
    fn Weather_UpdateBlend() -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Clouds_InitVars() {
    unsafe {
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1729)
            .cast::<i8>())
        .write(0i8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1730)).write(20u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).write(0u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1740)
            .cast::<u16>())
        .write(0u16);
        if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1758)).read())
            as i32)
            == 0i32
        {
            Weather_SetBlendCoeffs(0u8, 16u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Clouds_InitAll() {
    unsafe {
        Clouds_InitVars();
        'l1: loop {
            if !(((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).read())
                as i32)
                == 0i32)
            {
                break 'l1;
            }
            Clouds_Main();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Clouds_Main() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1740)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                CreateCloudSprites();
                let __p2 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                Weather_SetTargetBlendCoeffs(12u8, 8u8, 1i32);
                let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (Weather_UpdateBlend()) != 0 {
                    ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746))
                        .write(1u8);
                    let __p4 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1740)
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Clouds_Finish() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1742)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                Weather_SetTargetBlendCoeffs(0u8, 16u8, 1i32);
                let __p2 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1742)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                return 1u8;
            }
            if __sw1 == 1i32 {
                if (Weather_UpdateBlend()) != 0 {
                    DestroyCloudSprites();
                    let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1742)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                return 1u8;
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sunny_InitVars() {
    unsafe {
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1729)
            .cast::<i8>())
        .write(0i8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1730)).write(20u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sunny_InitAll() {
    unsafe {
        Sunny_InitVars();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sunny_Main() {
    unsafe {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sunny_Finish() -> u8 {
    unsafe {
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CreateCloudSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1758)).read())
            as i32)
            == 1i32
        {
            return;
        }
        LoadSpriteSheet((&raw const sCloudSpriteSheet).cast::<u8>().cast_mut());
        LoadCustomWeatherSpritePalette(
            ((&raw const gCloudsWeatherPalette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>(),
        );
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (&raw const sCloudSpriteTemplate).cast::<u8>().cast_mut(),
                        0i16,
                        0i16,
                        255u8,
                    );
                    if ((spriteId) as i32) != 64i32 {
                        ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(500))
                            .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                        );
                        sprite = ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                            .wrapping_add(500))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read();
                        SetSpritePosToMapCoords(
                            (((((((((&raw const sCloudSpriteMapCoords).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .cast::<i16>())
                            .read()) as i32)
                                .wrapping_add(7i32)) as i16),
                            (((((((((&raw const sCloudSpriteMapCoords).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<i16>())
                            .read()) as i32)
                                .wrapping_add(7i32)) as i16),
                            (sprite).wrapping_add(32).cast::<i16>(),
                            (sprite).wrapping_add(34).cast::<i16>(),
                        );
                        crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
                    } else {
                        ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(500))
                            .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(core::ptr::null_mut());
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1758)).write(1u8);
    }
}
pub(crate) unsafe extern "C" fn DestroyCloudSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        if !((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1758)).read()) != 0)
        {
            return;
        }
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(500))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as usize)
                        != 0usize
                    {
                        DestroySprite(
                            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(500))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        FreeSpriteTilesByTag(4608u16);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1758)).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn UpdateCloudSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(1i32)
                & 1i32) as i16),
        );
        if ((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Drought_InitVars() {
    unsafe {
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1740)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).write(0u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1729)
            .cast::<i8>())
        .write(0i8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1730)).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Drought_InitAll() {
    unsafe {
        Drought_InitVars();
        'l1: loop {
            if !(((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).read())
                as i32)
                == 0i32)
            {
                break 'l1;
            }
            Drought_Main();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Drought_Main() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1740)
                .cast::<u16>())
            .read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 {
                if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1734))
                    .read()) as i32)
                    != 0i32
                {
                    let __p2 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1740)
                        .cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ResetDroughtWeatherPaletteLoading();
                let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((LoadDroughtWeatherPalettes()) as i32) == 0i32 {
                    let __p4 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1740)
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                DroughtStateInit();
                let __p5 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                DroughtStateRun();
                if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1852)
                    .cast::<i16>())
                .read()) as i32)
                    == 6i32
                {
                    ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746))
                        .write(1u8);
                    let __p6 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1740)
                        .cast::<u16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                DroughtStateRun();
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Drought_Finish() -> u8 {
    unsafe {
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartDroughtWeatherBlend() {
    unsafe {
        CreateTask(Some(UpdateDroughtBlend), 80u8);
    }
}
pub(crate) unsafe extern "C" fn UpdateDroughtBlend(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
                    .write(((((67108936i32) as usize as *mut u16).read_volatile()) as i16));
                SetGpuReg(72u8, 16191u16);
                SetGpuReg(80u8, 158u16);
                SetGpuReg(84u8, 0u16);
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(3i32)) as i16));
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    > 16i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(16i16);
                }
                SetGpuReg(
                    84u8,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    >= 16i32
                {
                    let __p4 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p5).write(((__p5).read()).wrapping_add(1));
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    > 9i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    (__p6).write(((__p6).read()).wrapping_sub(1));
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        <= 0i32
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                        let __p7 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p7).write(((__p7).read()).wrapping_add(1));
                    }
                    SetGpuReg(
                        84u8,
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as u16),
                    );
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                SetGpuReg(80u8, 0u16);
                SetGpuReg(84u8, 0u16);
                SetGpuReg(
                    72u8,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as u16),
                );
                let __p8 = ((task).wrapping_add(8)).cast::<i16>();
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                ScriptContext_Enable();
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rain_InitVars() {
    unsafe {
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1740)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).write(0u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1750)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1755)).write(8u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1756)).write(0u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1753)).write(10u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1729)
            .cast::<i8>())
        .write(3i8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1730)).write(20u8);
        SetRainStrengthFromSoundEffect(85u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rain_InitAll() {
    unsafe {
        Rain_InitVars();
        'l1: loop {
            if !(!((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746))
                .read())
                != 0))
            {
                break 'l1;
            }
            Rain_Main();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rain_Main() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1740)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                LoadRainSpriteSheet();
                let __p2 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((CreateRainSprite()) != 0) {
                    let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1740)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((UpdateVisibleRainSprites()) != 0) {
                    ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746))
                        .write(1u8);
                    let __p4 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1740)
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rain_Finish() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1742)
                .cast::<u16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if ((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1745))
                    .read()) as i32)
                    == 3i32)
                    || (((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1745))
                        .read()) as i32)
                        == 5i32))
                    || (((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1745))
                        .read()) as i32)
                        == 13i32)
                {
                    ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1742)
                        .cast::<u16>())
                    .write(255u16);
                    return 0u8;
                } else {
                    ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1753))
                        .write(0u8);
                    let __p2 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1742)
                        .cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if !((UpdateVisibleRainSprites()) != 0) {
                    DestroyRainSprites();
                    let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1742)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    return 0u8;
                }
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StartRainSpriteFall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut rand: u32 = 0u32;
        let mut numFallingFrames: u16 = 0u16;
        let mut tileX: i32 = 0i32;
        let mut tileY: i32 = 0i32;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(361i16);
        }
        rand = ((((1103515245i32).wrapping_mul(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
        ))
        .wrapping_add(12345i32)) as u32);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((crate::c::rem_u32(((rand & 2147418112u32) >> 16), 600u32)) as i16));
        numFallingFrames = (((((&raw const sRainSpriteFallingDurations)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1756)).read())
                as i32) as isize
                * 4,
        ))
        .cast::<u16>())
        .read();
        tileX = crate::c::rem_i32(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            30i32,
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write((((tileX).wrapping_mul(8i32)) as i16));
        tileY = crate::c::div_i32(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            30i32,
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write((((tileY).wrapping_mul(8i32)) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(((tileX) as i16));
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write((((((__p1).read()) as i32) << 7) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(((tileY) as i16));
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p2).write((((((__p2).read()) as i32) << 7) as i16));
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_sub(
                (((((((&raw const sRainSpriteMovement).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1756))
                            .read()) as i32) as isize
                            * 4,
                    ))
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_mul(((numFallingFrames) as i32)),
            )) as i16),
        );
        let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p4).write(
            (((((__p4).read()) as i32).wrapping_sub(
                ((((((((&raw const sRainSpriteMovement).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1756))
                            .read()) as i32) as isize
                            * 4,
                    ))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_mul(((numFallingFrames) as i32)),
            )) as i16),
        );
        StartSpriteAnim(sprite, 0u8);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (0u16) as i32);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(((numFallingFrames) as i16));
    }
}
pub(crate) unsafe extern "C" fn UpdateRainSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            == 0i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    (((((((&raw const sRainSpriteMovement).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(1756))
                            .read()) as i32) as isize
                                * 4,
                        ))
                    .cast::<i16>())
                    .read()) as i32),
                )) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((((&raw const sRainSpriteMovement).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1756))
                            .read()) as i32) as isize
                            * 4,
                    ))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32),
                )) as i16),
            );
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    >> 4) as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    >> 4) as i16),
            );
            if ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) != 0)
                && ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) >= (-8i32))
                    && (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) <= 248i32)))
                && (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) >= (-16i32)))
                && (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) <= 176i32)
            {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            } else {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            }
            if (({
                let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                let __t4 = ((__p3).read()).wrapping_sub(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                == 0i32
            {
                StartSpriteAnim(
                    sprite,
                    ((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1756))
                        .read()) as i32)
                        .wrapping_add(1i32)) as u8),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
                let __p5 = (sprite).wrapping_add(32).cast::<i16>();
                (__p5).write(
                    (((((__p5).read()) as i32).wrapping_sub(
                        ((((&raw mut gSpriteCoordOffsetX).cast::<i16>()).read()) as i32),
                    )) as i16),
                );
                let __p6 = (sprite).wrapping_add(34).cast::<i16>();
                (__p6).write(
                    (((((__p6).read()) as i32).wrapping_sub(
                        ((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as i32),
                    )) as i16),
                );
                crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            }
        } else {
            if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                StartRainSpriteFall(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn WaitRainSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            StartRainSpriteFall(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(UpdateRainSprite));
        } else {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn InitRainSpriteMovement(sprite: *mut u8, val: u16) {
    unsafe {
        let mut sprite = sprite;
        let mut val = val;
        let mut numFallingFrames: u16 = (((((&raw const sRainSpriteFallingDurations)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1756)).read())
                as i32) as isize
                * 4,
        ))
        .cast::<u16>())
        .read();
        let mut numAdvanceRng: u16 = ((crate::c::div_i32(
            ((val) as i32),
            ((((((((&raw const sRainSpriteFallingDurations)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1756)).read())
                    as i32) as isize
                    * 4,
            ))
            .cast::<u16>())
            .wrapping_offset(1))
            .read()) as i32)
                .wrapping_add(((numFallingFrames) as i32)),
        )) as u16);
        let mut frameVal: u16 = ((crate::c::rem_i32(
            ((val) as i32),
            ((((((((&raw const sRainSpriteFallingDurations)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1756)).read())
                    as i32) as isize
                    * 4,
            ))
            .cast::<u16>())
            .wrapping_offset(1))
            .read()) as i32)
                .wrapping_add(((numFallingFrames) as i32)),
        )) as u16);
        'l1: loop {
            if !((({
                let __t1 = (numAdvanceRng).wrapping_sub(1);
                numAdvanceRng = __t1;
                __t1
            }) as i32)
                != 65535i32)
            {
                break 'l1;
            }
            StartRainSpriteFall(sprite);
        }
        if ((frameVal) as i32) < ((numFallingFrames) as i32) {
            'l2: loop {
                if !((({
                    let __t2 = (frameVal).wrapping_sub(1);
                    frameVal = __t2;
                    __t2
                }) as i32)
                    != 65535i32)
                {
                    break 'l2;
                }
                UpdateRainSprite(sprite);
            }
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        } else {
            (((sprite).wrapping_add(46)).cast::<i16>())
                .write(((((frameVal) as i32).wrapping_sub(((numFallingFrames) as i32))) as i16));
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(1i16);
        }
    }
}
pub(crate) unsafe extern "C" fn LoadRainSpriteSheet() {
    unsafe {
        LoadSpriteSheet((&raw const sRainSpriteSheet).cast::<u8>().cast_mut());
    }
}
pub(crate) unsafe extern "C" fn CreateRainSprite() -> u8 {
    unsafe {
        let mut spriteIndex: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1754)).read())
            as i32)
            == 24i32
        {
            return 0u8;
        }
        spriteIndex =
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1754)).read();
        spriteId = CreateSpriteAtEnd(
            (&raw const sRainSpriteTemplate).cast::<u8>().cast_mut(),
            (((((&raw const sRainSpriteCoords).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((spriteIndex) as i32) as isize * 4))
            .cast::<i16>())
            .read(),
            (((((&raw const sRainSpriteCoords).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((spriteIndex) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<i16>())
            .read(),
            78u8,
        );
        if ((spriteId) as i32) != 64i32 {
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(0i16);
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((((spriteIndex) as i32).wrapping_mul(145i32)) as i16));
            'l1: loop {
                if !(((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    >= 600i32)
                {
                    break 'l1;
                }
                let __p1 = (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p1).write((((((__p1).read()) as i32).wrapping_sub(600i32)) as i16));
            }
            StartRainSpriteFall(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
            InitRainSpriteMovement(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                ((((spriteIndex) as i32).wrapping_mul(9i32)) as u16),
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            (((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                .wrapping_offset(((spriteIndex) as i32) as isize))
            .write(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
        } else {
            (((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                .wrapping_offset(((spriteIndex) as i32) as isize))
            .write(core::ptr::null_mut());
        }
        if (({
            let __p2 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1754);
            let __t3 = ((__p2).read()).wrapping_add(1);
            (__p2).write(__t3);
            __t3
        }) as i32)
            == 24i32
        {
            let mut i: u16 = 0u16;
            {
                i = 0u16;
                'l2: loop {
                    if !(((i) as i32) < 24i32) {
                        break 'l2;
                    }
                    'l3: {
                        if !((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .is_null()
                        {
                            if !(((((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(6))
                            .read())
                                != 0)
                            {
                                (((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                    .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read())
                                .wrapping_add(28)
                                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                .write(Some(UpdateRainSprite));
                            } else {
                                (((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                    .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read())
                                .wrapping_add(28)
                                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                .write(Some(WaitRainSprite));
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn UpdateVisibleRainSprites() -> u8 {
    unsafe {
        if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1752)).read())
            as i32)
            == ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1753)).read())
                as i32)
        {
            return 0u8;
        }
        if (({
            let __p1 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1750)
                .cast::<u16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1755)).read())
                as i32)
        {
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1750)
                .cast::<u16>())
            .write(0u16);
            if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1752)).read())
                as i32)
                < ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1753)).read())
                    as i32)
            {
                (((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                    .wrapping_offset(
                        (({
                            let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(1752);
                            let __t4 = (__p3).read();
                            (__p3).write(((__p3).read()).wrapping_add(1));
                            __t4
                        }) as i32) as isize,
                    ))
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(1i16);
            } else {
                let __p5 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1752);
                (__p5).write(((__p5).read()).wrapping_sub(1));
                (((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                    .wrapping_offset(
                        ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1752))
                            .read()) as i32) as isize,
                    ))
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(0i16);
                crate::c::bf_write(
                    ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                        .wrapping_offset(
                            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(1752))
                            .read()) as i32) as isize,
                        ))
                    .read())
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn DestroyRainSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1754))
                        .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as usize)
                        != 0usize
                    {
                        DestroySprite(
                            (((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1754)).write(0u8);
        FreeSpriteTilesByTag(4614u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Snow_InitVars() {
    unsafe {
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1740)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).write(0u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1729)
            .cast::<i8>())
        .write(3i8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1730)).write(20u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1765)).write(16u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1760)
            .cast::<u16>())
        .write(0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Snow_InitAll() {
    unsafe {
        let mut i: u16 = 0u16;
        Snow_InitVars();
        'l1: loop {
            if !(((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).read())
                as i32)
                == 0i32)
            {
                break 'l1;
            }
            Snow_Main();
            {
                i = 0u16;
                'l2: loop {
                    if !(((i) as i32)
                        < ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                            .wrapping_add(1764))
                        .read()) as i32))
                    {
                        break 'l2;
                    }
                    'l3: {
                        UpdateSnowflakeSprite(
                            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(96))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Snow_Main() {
    unsafe {
        if (((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1740)
            .cast::<u16>())
        .read()) as i32)
            == 0i32)
            && (!((UpdateVisibleSnowflakeSprites()) != 0))
        {
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).write(1u8);
            let __p1 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1740)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Snow_Finish() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1742)
                .cast::<u16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1765)).write(0u8);
                ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1760)
                    .cast::<u16>())
                .write(0u16);
                let __p2 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1742)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if !((UpdateVisibleSnowflakeSprites()) != 0) {
                    let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1742)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    return 0u8;
                }
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn UpdateVisibleSnowflakeSprites() -> u8 {
    unsafe {
        if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1764)).read())
            as i32)
            == ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1765)).read())
                as i32)
        {
            return 0u8;
        }
        if (({
            let __p1 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1760)
                .cast::<u16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 36i32
        {
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1760)
                .cast::<u16>())
            .write(0u16);
            if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1764)).read())
                as i32)
                < ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1765)).read())
                    as i32)
            {
                CreateSnowflakeSprite();
            } else {
                DestroySnowflakeSprite();
            }
        }
        return ((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1764)).read())
            as i32)
            != ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1765)).read())
                as i32)) as u8);
    }
}
pub(crate) unsafe extern "C" fn CreateSnowflakeSprite() -> u8 {
    unsafe {
        let mut spriteId: u8 = CreateSpriteAtEnd(
            (&raw const sSnowflakeSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            0i16,
            0i16,
            78u8,
        );
        if ((spriteId) as i32) == 64i32 {
            return 0u8;
        }
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(
            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1764)).read())
                as i16),
        );
        InitSnowflakeSpriteMovement(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            1,
            1,
            (1u16) as i32,
        );
        ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(96))
            .cast::<*mut u8>())
        .wrapping_offset(
            (({
                let __p1 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1764);
                let __t2 = (__p1).read();
                (__p1).write(((__p1).read()).wrapping_add(1));
                __t2
            }) as i32) as isize,
        ))
        .write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn DestroySnowflakeSprite() -> u8 {
    unsafe {
        if (((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1764)).read()) != 0 {
            DestroySprite(
                ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(96))
                    .cast::<*mut u8>())
                .wrapping_offset(
                    (({
                        let __p1 =
                            (((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1764);
                        let __t2 = ((__p1).read()).wrapping_sub(1);
                        (__p1).write(__t2);
                        __t2
                    }) as i32) as isize,
                ))
                .read(),
            );
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn InitSnowflakeSpriteMovement(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut rand: u16 = 0u16;
        let mut x: u16 = ((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .read()) as i32)
            .wrapping_mul(5i32)
            & 7i32)
            .wrapping_mul(30i32))
        .wrapping_add(crate::c::rem_i32(((Random()) as i32), 30i32)))
            as u16);
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            (((-3i32).wrapping_sub(
                ((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(41).cast::<i8>()).read()) as i32)),
            )) as i16),
        );
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((x) as i32).wrapping_sub(
                ((((&raw mut gSpriteCoordOffsetX).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(40).cast::<i8>()).read()) as i32)),
            )) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_mul(128i32))
                as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        rand = Random();
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((((((rand) as i32) & 3i32).wrapping_mul(5i32)).wrapping_add(64i32)) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read());
        StartSpriteAnim(
            sprite,
            ((if (((rand) as i32) & 1i32) != 0 {
                0i32
            } else {
                1i32
            }) as u8),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((if (((rand) as i32) & 3i32) == 0i32 {
                2i32
            } else {
                1i32
            }) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .write((((((rand) as i32) & 31i32).wrapping_add(210i32)) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
    }
}
pub(crate) unsafe extern "C" fn WaitSnowflakeSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1762)
            .cast::<u16>())
        .read()) as i32)
            > 18i32
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(UpdateSnowflakeSprite));
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                (((250i32).wrapping_sub(
                    ((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(41).cast::<i8>()).read()) as i32)),
                )) as i16),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_mul(128i32))
                    as i16),
            );
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1762)
                .cast::<u16>())
            .write(0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateSnowflakeSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>())
            .write((((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >> 7) as i16));
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16),
        );
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p3).write((((((__p3).read()) as i32) & 255i32) as i16));
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32) as isize,
                ))
                .read()) as i32),
                64i32,
            )) as i16),
        );
        x = (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(40).cast::<i8>()).read()) as i32)))
        .wrapping_add(((((&raw mut gSpriteCoordOffsetX).cast::<i16>()).read()) as i32))
            & 511i32) as i16);
        if (((x) as i32) & 256i32) != 0 {
            x = ((((x) as i32) | (-256i32)) as i16);
        }
        if ((x) as i32) < (-3i32) {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                (((242i32).wrapping_sub(
                    ((((&raw mut gSpriteCoordOffsetX).cast::<i16>()).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(40).cast::<i8>()).read()) as i32)),
                )) as i16),
            );
        } else {
            if ((x) as i32) > 242i32 {
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    (((-3i32).wrapping_sub(
                        ((((&raw mut gSpriteCoordOffsetX).cast::<i16>()).read()) as i32)
                            .wrapping_add(
                                ((((sprite).wrapping_add(40).cast::<i8>()).read()) as i32),
                            ),
                    )) as i16),
                );
            }
        }
        y = (((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(41).cast::<i8>()).read()) as i32)))
        .wrapping_add(((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as i32))
            & 255i32) as i16);
        if (((y) as i32) > 163i32) && (((y) as i32) < 171i32) {
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                (((250i32).wrapping_sub(
                    ((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(41).cast::<i8>()).read()) as i32)),
                )) as i16),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_mul(128i32))
                    as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(220i16);
        } else {
            if (((y) as i32) > 242i32) && (((y) as i32) < 250i32) {
                ((sprite).wrapping_add(34).cast::<i16>()).write(163i16);
                (((sprite).wrapping_add(46)).cast::<i16>()).write(
                    ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                        .wrapping_mul(128i32)) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(220i16);
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitSnowflakeSprite));
            }
        }
        if (({
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            let __t5 = ((__p4).read()).wrapping_add(1);
            (__p4).write(__t5);
            __t5
        }) as i32)
            == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
        {
            InitSnowflakeSpriteMovement(sprite);
            ((sprite).wrapping_add(34).cast::<i16>()).write(250i16);
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitSnowflakeSprite));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Thunderstorm_InitVars() {
    unsafe {
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1740)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).write(0u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1750)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1755)).write(4u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1756)).write(0u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1753)).write(16u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1729)
            .cast::<i8>())
        .write(3i8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1730)).write(20u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).write(0u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1773)).write(0u8);
        SetRainStrengthFromSoundEffect(81u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Thunderstorm_InitAll() {
    unsafe {
        Thunderstorm_InitVars();
        'l1: loop {
            if !(((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).read())
                as i32)
                == 0i32)
            {
                break 'l1;
            }
            Thunderstorm_Main();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Downpour_InitVars() {
    unsafe {
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1740)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).write(0u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1750)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1755)).write(4u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1756)).write(1u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1753)).write(24u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1729)
            .cast::<i8>())
        .write(3i8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1730)).write(20u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).write(0u8);
        SetRainStrengthFromSoundEffect(83u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Downpour_InitAll() {
    unsafe {
        Downpour_InitVars();
        'l1: loop {
            if !(((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).read())
                as i32)
                == 0i32)
            {
                break 'l1;
            }
            Thunderstorm_Main();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Thunderstorm_Main() {
    unsafe {
        UpdateThunderSound();
        'l1: {
            let __sw1 = ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1740)
                .cast::<u16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                LoadRainSpriteSheet();
                let __p2 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if !((CreateRainSprite()) != 0) {
                    let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1740)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if !((UpdateVisibleRainSprites()) != 0) {
                    ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746))
                        .write(1u8);
                    let __p4 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1740)
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1734))
                    .read()) as i32)
                    != 0i32
                {
                    ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1740)
                        .cast::<u16>())
                    .write(6u16);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1770)).write(1u8);
                ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1766)
                    .cast::<u16>())
                .write(
                    (((crate::c::rem_i32(((Random()) as i32), 360i32)).wrapping_add(360i32))
                        as u16),
                );
                let __p5 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 5i32 {
                __fall = true;
                if (({
                    let __p6 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1766)
                        .cast::<u16>();
                    let __t7 = ((__p6).read()).wrapping_sub(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    == 0i32
                {
                    let __p8 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1740)
                        .cast::<u16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1770)).write(1u8);
                ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1771))
                    .write(((crate::c::rem_i32(((Random()) as i32), 2i32)) as u8));
                let __p9 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                __fall = true;
                ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1772))
                    .write((((((Random()) as i32) & 1i32).wrapping_add(1i32)) as u8));
                let __p10 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p10).write(((__p10).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 8i32 {
                __fall = true;
                ApplyWeatherColorMapIfIdle(19i8);
                if (!((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1771))
                    .read())
                    != 0))
                    && (((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1772))
                        .read()) as i32)
                        == 1i32)
                {
                    EnqueueThunder(20u16);
                }
                ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1766)
                    .cast::<u16>())
                .write(
                    (((crate::c::rem_i32(((Random()) as i32), 3i32)).wrapping_add(6i32)) as u16),
                );
                let __p11 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                __fall = true;
                if (({
                    let __p12 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1766)
                        .cast::<u16>();
                    let __t13 = ((__p12).read()).wrapping_sub(1);
                    (__p12).write(__t13);
                    __t13
                }) as i32)
                    == 0i32
                {
                    ApplyWeatherColorMapIfIdle(3i8);
                    ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1770))
                        .write(1u8);
                    if (({
                        let __p14 =
                            (((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1772);
                        let __t15 = ((__p14).read()).wrapping_sub(1);
                        (__p14).write(__t15);
                        __t15
                    }) as i32)
                        != 0i32
                    {
                        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                            .wrapping_add(1766)
                            .cast::<u16>())
                        .write(
                            (((crate::c::rem_i32(((Random()) as i32), 16i32)).wrapping_add(60i32))
                                as u16),
                        );
                        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                            .wrapping_add(1740)
                            .cast::<u16>())
                        .write(10u16);
                    } else {
                        if !((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                            .wrapping_add(1771))
                        .read())
                            != 0)
                        {
                            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(1740)
                                .cast::<u16>())
                            .write(4u16);
                        } else {
                            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(1740)
                                .cast::<u16>())
                            .write(11u16);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                __fall = true;
                if (({
                    let __p16 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1766)
                        .cast::<u16>();
                    let __t17 = ((__p16).read()).wrapping_sub(1);
                    (__p16).write(__t17);
                    __t17
                }) as i32)
                    == 0i32
                {
                    ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1740)
                        .cast::<u16>())
                    .write(8u16);
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                __fall = true;
                ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1766)
                    .cast::<u16>())
                .write(
                    (((crate::c::rem_i32(((Random()) as i32), 16i32)).wrapping_add(60i32)) as u16),
                );
                let __p18 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p18).write(((__p18).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                __fall = true;
                if (({
                    let __p19 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1766)
                        .cast::<u16>();
                    let __t20 = ((__p19).read()).wrapping_sub(1);
                    (__p19).write(__t20);
                    __t20
                }) as i32)
                    == 0i32
                {
                    EnqueueThunder(100u16);
                    ApplyWeatherColorMapIfIdle(19i8);
                    ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1766)
                        .cast::<u16>())
                    .write((((((Random()) as i32) & 15i32).wrapping_add(30i32)) as u16));
                    let __p21 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1740)
                        .cast::<u16>();
                    (__p21).write(((__p21).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                __fall = true;
                if (({
                    let __p22 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1766)
                        .cast::<u16>();
                    let __t23 = ((__p22).read()).wrapping_sub(1);
                    (__p22).write(__t23);
                    __t23
                }) as i32)
                    == 0i32
                {
                    ApplyWeatherColorMapIfIdle_Gradual(19u8, 3u8, 5u8);
                    let __p24 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1740)
                        .cast::<u16>();
                    (__p24).write(((__p24).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 14i32 {
                __fall = true;
                if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1734))
                    .read()) as i32)
                    == 3i32
                {
                    ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1770))
                        .write(1u8);
                    ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1740)
                        .cast::<u16>())
                    .write(4u16);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Thunderstorm_Finish() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1742)
                .cast::<u16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1770)).write(0u8);
                let __p2 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1742)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                Thunderstorm_Main();
                if (((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1770)).read())
                    != 0
                {
                    if ((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1745))
                        .read()) as i32)
                        == 3i32)
                        || (((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                            .wrapping_add(1745))
                        .read()) as i32)
                            == 5i32))
                        || (((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                            .wrapping_add(1745))
                        .read()) as i32)
                            == 13i32)
                    {
                        return 0u8;
                    }
                    ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1753))
                        .write(0u8);
                    let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1742)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if !((UpdateVisibleRainSprites()) != 0) {
                    DestroyRainSprites();
                    ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1773))
                        .write(0u8);
                    let __p4 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1742)
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    return 0u8;
                }
                break 'l1;
            }
            if !__matched {
                __fall = true;
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn EnqueueThunder(waitFrames: u16) {
    unsafe {
        let mut waitFrames = waitFrames;
        if !((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1773)).read()) != 0)
        {
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1768)
                .cast::<u16>())
            .write(((crate::c::rem_i32(((Random()) as i32), ((waitFrames) as i32))) as u16));
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1773)).write(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateThunderSound() {
    unsafe {
        if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1773)).read())
            as i32)
            == 1i32
        {
            if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1768)
                .cast::<u16>())
            .read()) as i32)
                == 0i32
            {
                if (IsSEPlaying()) != 0 {
                    return;
                }
                if (((Random()) as i32) & 1i32) != 0 {
                    PlaySE(87u16);
                } else {
                    PlaySE(88u16);
                }
                ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1773)).write(0u8);
            } else {
                let __p1 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1768)
                    .cast::<u16>();
                (__p1).write(((__p1).read()).wrapping_sub(1));
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FogHorizontal_InitVars() {
    unsafe {
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1740)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).write(0u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1729)
            .cast::<i8>())
        .write(0i8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1730)).write(20u8);
        if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1787)).read())
            as i32)
            == 0i32
        {
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1776)
                .cast::<u16>())
            .write(0u16);
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1778)
                .cast::<u16>())
            .write(0u16);
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1774)
                .cast::<u16>())
            .write(0u16);
            Weather_SetBlendCoeffs(0u8, 16u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FogHorizontal_InitAll() {
    unsafe {
        FogHorizontal_InitVars();
        'l1: loop {
            if !(((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).read())
                as i32)
                == 0i32)
            {
                break 'l1;
            }
            FogHorizontal_Main();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FogHorizontal_Main() {
    unsafe {
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1774)
            .cast::<u16>())
        .write(
            ((((((&raw mut gSpriteCoordOffsetX).cast::<i16>()).read()) as i32).wrapping_sub(
                ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1778)
                    .cast::<u16>())
                .read()) as i32),
            ) & 255i32) as u16),
        );
        if (({
            let __p1 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1776)
                .cast::<u16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 3i32
        {
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1776)
                .cast::<u16>())
            .write(0u16);
            let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1778)
                .cast::<u16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        'l1: {
            let __sw4 = ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1740)
                .cast::<u16>())
            .read()) as i32);
            if __sw4 == 0i32 {
                CreateFogHorizontalSprites();
                if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1744))
                    .read()) as i32)
                    == 6i32
                {
                    Weather_SetTargetBlendCoeffs(12u8, 8u8, 3i32);
                } else {
                    Weather_SetTargetBlendCoeffs(4u8, 16u8, 0i32);
                }
                let __p5 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw4 == 1i32 {
                if (Weather_UpdateBlend()) != 0 {
                    ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746))
                        .write(1u8);
                    let __p6 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1740)
                        .cast::<u16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FogHorizontal_Finish() -> u8 {
    unsafe {
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1774)
            .cast::<u16>())
        .write(
            ((((((&raw mut gSpriteCoordOffsetX).cast::<i16>()).read()) as i32).wrapping_sub(
                ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1778)
                    .cast::<u16>())
                .read()) as i32),
            ) & 255i32) as u16),
        );
        if (({
            let __p1 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1776)
                .cast::<u16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 3i32
        {
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1776)
                .cast::<u16>())
            .write(0u16);
            let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1778)
                .cast::<u16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        'l1: {
            let __sw4 = ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1742)
                .cast::<u16>())
            .read()) as i32);
            let __matched = __sw4 == 0i32 || __sw4 == 1i32 || __sw4 == 2i32;
            if __sw4 == 0i32 {
                Weather_SetTargetBlendCoeffs(0u8, 16u8, 3i32);
                let __p5 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1742)
                    .cast::<u16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw4 == 1i32 {
                if (Weather_UpdateBlend()) != 0 {
                    let __p6 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1742)
                        .cast::<u16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw4 == 2i32 {
                DestroyFogHorizontalSprites();
                let __p7 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1742)
                    .cast::<u16>();
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn FogHorizontalSpriteCallback(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(38).cast::<i16>())
            .write((((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as u8) as i16));
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            (((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1774)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(32i32))
            .wrapping_add(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(64i32),
            )) as i16),
        );
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) >= 272i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((480i32).wrapping_add(
                    ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1774)
                        .cast::<u16>())
                    .read()) as i32),
                ))
                .wrapping_sub(
                    ((4i32).wrapping_sub(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                    ))
                    .wrapping_mul(64i32),
                )) as i16),
            );
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32) & 511i32) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn CreateFogHorizontalSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        if !((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1787)).read()) != 0)
        {
            let mut fogHorizontalSpriteSheet = crate::ffi::Align4([0u8; 8]);
            (&raw mut fogHorizontalSpriteSheet)
                .cast::<u8>()
                .wrapping_add(0)
                .cast::<*mut u8>()
                .write(
                    ((&raw const gWeatherFogHorizontalTiles)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
            (&raw mut fogHorizontalSpriteSheet)
                .cast::<u8>()
                .wrapping_add(4)
                .cast::<u16>()
                .write(2048u16);
            (&raw mut fogHorizontalSpriteSheet)
                .cast::<u8>()
                .wrapping_add(6)
                .cast::<u16>()
                .write(4609u16);
            LoadSpriteSheet((&raw mut fogHorizontalSpriteSheet).cast::<u8>());
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 20i32) {
                        break 'l1;
                    }
                    'l2: {
                        spriteId = CreateSpriteAtEnd(
                            (&raw const sFogHorizontalSpriteTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                            0i16,
                            0i16,
                            255u8,
                        );
                        if ((spriteId) as i32) != 64i32 {
                            sprite = ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68);
                            (((sprite).wrapping_add(46)).cast::<i16>())
                                .write(((crate::c::rem_i32(((i) as i32), 5i32)) as i16));
                            ((sprite).wrapping_add(32).cast::<i16>()).write(
                                ((((crate::c::rem_i32(((i) as i32), 5i32)).wrapping_mul(64i32))
                                    .wrapping_add(32i32)) as i16),
                            );
                            ((sprite).wrapping_add(34).cast::<i16>()).write(
                                ((((crate::c::div_i32(((i) as i32), 5i32)).wrapping_mul(64i32))
                                    .wrapping_add(32i32)) as i16),
                            );
                            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(160))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(sprite);
                        } else {
                            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(160))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(core::ptr::null_mut());
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1787)).write(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyFogHorizontalSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        if (((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1787)).read()) != 0 {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 20i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                            .wrapping_add(160))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as usize)
                            != 0usize
                        {
                            DestroySprite(
                                ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                    .wrapping_add(160))
                                .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            FreeSpriteTilesByTag(4609u16);
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1787)).write(0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Ash_InitVars() {
    unsafe {
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1740)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).write(0u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1729)
            .cast::<i8>())
        .write(0i8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1730)).write(20u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1790)
            .cast::<u16>())
        .write(20u16);
        if !((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1792)).read()) != 0)
        {
            Weather_SetBlendCoeffs(0u8, 16u8);
            SetGpuReg(82u8, 16192u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Ash_InitAll() {
    unsafe {
        Ash_InitVars();
        'l1: loop {
            if !(((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).read())
                as i32)
                == 0i32)
            {
                break 'l1;
            }
            Ash_Main();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Ash_Main() {
    unsafe {
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1788)
            .cast::<u16>())
        .write(
            ((((((&raw mut gSpriteCoordOffsetX).cast::<i16>()).read()) as i32) & 511i32) as u16),
        );
        'l1: loop {
            if !(((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1788)
                .cast::<u16>())
            .read()) as i32)
                >= 240i32)
            {
                break 'l1;
            }
            let __p1 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1788)
                .cast::<u16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(240i32)) as u16));
        }
        'l2: {
            let __sw2 = ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1740)
                .cast::<u16>())
            .read()) as i32);
            let __matched = __sw2 == 0i32 || __sw2 == 1i32 || __sw2 == 2i32;
            if __sw2 == 0i32 {
                LoadAshSpriteSheet();
                let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l2;
            }
            if __sw2 == 1i32 {
                if !((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1792))
                    .read())
                    != 0)
                {
                    CreateAshSprites();
                }
                Weather_SetTargetBlendCoeffs(16u8, 0u8, 1i32);
                let __p4 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l2;
            }
            if __sw2 == 2i32 {
                if (Weather_UpdateBlend()) != 0 {
                    ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746))
                        .write(1u8);
                    let __p5 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1740)
                        .cast::<u16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l2;
            }
            if !__matched {
                Weather_UpdateBlend();
                break 'l2;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Ash_Finish() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1742)
                .cast::<u16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                Weather_SetTargetBlendCoeffs(0u8, 16u8, 1i32);
                let __p2 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1742)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (Weather_UpdateBlend()) != 0 {
                    DestroyAshSprites();
                    let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1742)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetGpuReg(82u8, 0u16);
                let __p4 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1742)
                    .cast::<u16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                return 0u8;
            }
            if !__matched {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn LoadAshSpriteSheet() {
    unsafe {
        LoadSpriteSheet((&raw const sAshSpriteSheet).cast::<u8>().cast_mut());
    }
}
pub(crate) unsafe extern "C" fn CreateAshSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        if !((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1792)).read()) != 0)
        {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 20i32) {
                        break 'l1;
                    }
                    'l2: {
                        spriteId = CreateSpriteAtEnd(
                            (&raw const sAshSpriteTemplate).cast::<u8>().cast_mut(),
                            0i16,
                            0i16,
                            78u8,
                        );
                        if ((spriteId) as i32) != 64i32 {
                            sprite = ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68);
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                .write(0i16);
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                .write((((crate::c::rem_i32(((i) as i32), 5i32)) as u8) as i16));
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                                .write((((crate::c::div_i32(((i) as i32), 5i32)) as u8) as i16));
                            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                                (((((((((sprite).wrapping_add(46)).cast::<i16>())
                                    .wrapping_offset(3))
                                .read()) as i32)
                                    .wrapping_mul(64i32))
                                .wrapping_add(32i32)) as i16),
                            );
                            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(240))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(sprite);
                        } else {
                            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(240))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(core::ptr::null_mut());
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1792)).write(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyAshSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        if (((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1792)).read()) != 0 {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 20i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                            .wrapping_add(240))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as usize)
                            != 0usize
                        {
                            DestroySprite(
                                ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                    .wrapping_add(240))
                                .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            FreeSpriteTilesByTag(4610u16);
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1792)).write(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateAshSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 5i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as i32)
                .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            (((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1788)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(32i32))
            .wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    .wrapping_mul(64i32),
            )) as i16),
        );
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) >= 272i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                (((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1788)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(480i32))
                .wrapping_sub(
                    ((4i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                    ))
                    .wrapping_mul(64i32),
                )) as i16),
            );
            let __p4 = (sprite).wrapping_add(32).cast::<i16>();
            (__p4).write((((((__p4).read()) as i32) & 511i32) as i16));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FogDiagonal_InitVars() {
    unsafe {
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1740)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).write(0u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1729)
            .cast::<i8>())
        .write(0i8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1730)).write(20u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1776)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1778)
            .cast::<u16>())
        .write(1u16);
        if !((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1828)).read()) != 0)
        {
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1820)
                .cast::<u16>())
            .write(0u16);
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1822)
                .cast::<u16>())
            .write(0u16);
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1824)
                .cast::<u16>())
            .write(0u16);
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1826)
                .cast::<u16>())
            .write(0u16);
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1816)
                .cast::<u16>())
            .write(0u16);
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1818)
                .cast::<u16>())
            .write(0u16);
            Weather_SetBlendCoeffs(0u8, 16u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FogDiagonal_InitAll() {
    unsafe {
        FogDiagonal_InitVars();
        'l1: loop {
            if !(((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).read())
                as i32)
                == 0i32)
            {
                break 'l1;
            }
            FogDiagonal_Main();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FogDiagonal_Main() {
    unsafe {
        UpdateFogDiagonalMovement();
        'l1: {
            let __sw1 = ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1740)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                CreateFogDiagonalSprites();
                let __p2 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                Weather_SetTargetBlendCoeffs(12u8, 8u8, 8i32);
                let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((Weather_UpdateBlend()) != 0) {
                    break 'l1;
                }
                ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).write(1u8);
                let __p4 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FogDiagonal_Finish() -> u8 {
    unsafe {
        UpdateFogDiagonalMovement();
        'l1: {
            let __sw1 = ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1742)
                .cast::<u16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                Weather_SetTargetBlendCoeffs(0u8, 16u8, 1i32);
                let __p2 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1742)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((Weather_UpdateBlend()) != 0) {
                    break 'l1;
                }
                let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1742)
                    .cast::<u16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                DestroyFogDiagonalSprites();
                let __p4 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1742)
                    .cast::<u16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn UpdateFogDiagonalMovement() {
    unsafe {
        if (({
            let __p1 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1820)
                .cast::<u16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 2i32
        {
            let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1824)
                .cast::<u16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1820)
                .cast::<u16>())
            .write(0u16);
        }
        if (({
            let __p4 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1822)
                .cast::<u16>();
            let __t5 = ((__p4).read()).wrapping_add(1);
            (__p4).write(__t5);
            __t5
        }) as i32)
            > 4i32
        {
            let __p6 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1826)
                .cast::<u16>();
            (__p6).write(((__p6).read()).wrapping_add(1));
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1822)
                .cast::<u16>())
            .write(0u16);
        }
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1816)
            .cast::<u16>())
        .write(
            ((((((&raw mut gSpriteCoordOffsetX).cast::<i16>()).read()) as i32).wrapping_sub(
                ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1824)
                    .cast::<u16>())
                .read()) as i32),
            ) & 255i32) as u16),
        );
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1818)
            .cast::<u16>())
        .write(
            ((((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as i32).wrapping_add(
                ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1826)
                    .cast::<u16>())
                .read()) as i32),
            )) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateFogDiagonalSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut fogDiagonalSpriteSheet = crate::ffi::Align4([0u8; 8]);
        let mut spriteId: u8 = 0u8;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        if !((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1828)).read()) != 0)
        {
            (&raw mut fogDiagonalSpriteSheet)
                .cast::<u8>()
                .cast::<crate::c::Rec4<8>>()
                .write_unaligned(
                    (&raw const sFogDiagonalSpriteSheet)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<crate::c::Rec4<8>>()
                        .read_unaligned(),
                );
            LoadSpriteSheet((&raw mut fogDiagonalSpriteSheet).cast::<u8>());
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 20i32) {
                        break 'l1;
                    }
                    'l2: {
                        spriteId = CreateSpriteAtEnd(
                            (&raw const sFogDiagonalSpriteTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                            0i16,
                            (((crate::c::div_i32(((i) as i32), 5i32)).wrapping_mul(64i32)) as i16),
                            255u8,
                        );
                        if ((spriteId) as i32) != 64i32 {
                            sprite = ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68);
                            (((sprite).wrapping_add(46)).cast::<i16>())
                                .write(((crate::c::rem_i32(((i) as i32), 5i32)) as i16));
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                .write(((crate::c::div_i32(((i) as i32), 5i32)) as i16));
                            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(320))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(sprite);
                        } else {
                            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(320))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(core::ptr::null_mut());
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1828)).write(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyFogDiagonalSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        if (((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1828)).read()) != 0 {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 20i32) {
                        break 'l1;
                    }
                    'l2: {
                        if !(((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                            .wrapping_add(320))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .is_null()
                        {
                            DestroySprite(
                                ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                    .wrapping_add(320))
                                .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            FreeSpriteTilesByTag(4611u16);
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1828)).write(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateFogDiagonalSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1818)
                .cast::<u16>())
            .read()) as i16),
        );
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            (((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1816)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(32i32))
            .wrapping_add(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(64i32),
            )) as i16),
        );
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) >= 272i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                (((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1816)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(480i32))
                .wrapping_sub(
                    ((4i32).wrapping_sub(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                    ))
                    .wrapping_mul(64i32),
                )) as i16),
            );
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32) & 511i32) as i16));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sandstorm_InitVars() {
    unsafe {
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1740)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746)).write(0u8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1729)
            .cast::<i8>())
        .write(0i8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1730)).write(20u8);
        if !((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1814)).read()) != 0)
        {
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1796)
                .cast::<u32>())
            .write({
                let __v1 = 0u32;
                ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1800)
                    .cast::<u32>())
                .write(__v1);
                __v1
            });
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1810)
                .cast::<u16>())
            .write(8u16);
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1812)
                .cast::<u16>())
            .write(0u16);
            if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1810)
                .cast::<u16>())
            .read()) as i32)
                >= 96i32
            {
                ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1810)
                    .cast::<u16>())
                .write(
                    (((128i32).wrapping_sub(
                        ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                            .wrapping_add(1810)
                            .cast::<u16>())
                        .read()) as i32),
                    )) as u16),
                );
            }
            Weather_SetBlendCoeffs(0u8, 16u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sandstorm_InitAll() {
    unsafe {
        Sandstorm_InitVars();
        'l1: loop {
            if !(!((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746))
                .read())
                != 0))
            {
                break 'l1;
            }
            Sandstorm_Main();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sandstorm_Main() {
    unsafe {
        UpdateSandstormMovement();
        UpdateSandstormWaveIndex();
        if ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1810)
            .cast::<u16>())
        .read()) as i32)
            >= 96i32
        {
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1810)
                .cast::<u16>())
            .write(32u16);
        }
        'l1: {
            let __sw1 = ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1740)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                CreateSandstormSprites();
                CreateSwirlSandstormSprites();
                let __p2 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                Weather_SetTargetBlendCoeffs(16u8, 0u8, 0i32);
                let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1740)
                    .cast::<u16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (Weather_UpdateBlend()) != 0 {
                    ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746))
                        .write(1u8);
                    let __p4 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1740)
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sandstorm_Finish() -> u8 {
    unsafe {
        UpdateSandstormMovement();
        UpdateSandstormWaveIndex();
        'l1: {
            let __sw1 = ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1742)
                .cast::<u16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                Weather_SetTargetBlendCoeffs(0u8, 16u8, 0i32);
                let __p2 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1742)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (Weather_UpdateBlend()) != 0 {
                    let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1742)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                DestroySandstormSprites();
                let __p4 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1742)
                    .cast::<u16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn UpdateSandstormWaveIndex() {
    unsafe {
        if (({
            let __p1 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1812)
                .cast::<u16>();
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            > 4i32
        {
            let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1810)
                .cast::<u16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1812)
                .cast::<u16>())
            .write(0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateSandstormMovement() {
    unsafe {
        let __p1 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1796)
            .cast::<u32>();
        (__p1).write(
            ((__p1).read()).wrapping_sub(
                ((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                    ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1810)
                        .cast::<u16>())
                    .read()) as i32) as isize,
                ))
                .read()) as i32)
                    .wrapping_mul(4i32)) as u32),
            ),
        );
        let __p2 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1800)
            .cast::<u32>();
        (__p2).write(
            ((__p2).read()).wrapping_sub(
                ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                    ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1810)
                        .cast::<u16>())
                    .read()) as i32) as isize,
                ))
                .read()) as u32),
            ),
        );
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1806)
            .cast::<u16>())
        .write(
            ((((((&raw mut gSpriteCoordOffsetX).cast::<i16>()).read()) as u32).wrapping_add(
                (((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1796)
                    .cast::<u32>())
                .read()
                    >> 8),
            ) & 255u32) as u16),
        );
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1808)
            .cast::<u16>())
        .write(
            ((((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as u32).wrapping_add(
                (((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1800)
                    .cast::<u32>())
                .read()
                    >> 8),
            )) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn DestroySandstormSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        if (((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1814)).read()) != 0 {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 20i32) {
                        break 'l1;
                    }
                    'l2: {
                        if !(((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                            .wrapping_add(400))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .is_null()
                        {
                            DestroySprite(
                                ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                    .wrapping_add(400))
                                .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1814)).write(0u8);
            FreeSpriteTilesByTag(4612u16);
        }
        if (((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1815)).read()) != 0 {
            {
                i = 0u16;
                'l3: loop {
                    if !(((i) as i32) < 5i32) {
                        break 'l3;
                    }
                    'l4: {
                        if ((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                            .wrapping_add(480))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as usize)
                            != 0usize
                        {
                            DestroySprite(
                                ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                    .wrapping_add(480))
                                .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1815)).write(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateSandstormSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        if !((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1814)).read()) != 0)
        {
            LoadSpriteSheet((&raw const sSandstormSpriteSheet).cast::<u8>().cast_mut());
            LoadCustomWeatherSpritePalette(
                ((&raw const gSandstormWeatherPalette)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>(),
            );
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 20i32) {
                        break 'l1;
                    }
                    'l2: {
                        spriteId = CreateSpriteAtEnd(
                            (&raw const sSandstormSpriteTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                            0i16,
                            (((crate::c::div_i32(((i) as i32), 5i32)).wrapping_mul(64i32)) as i16),
                            1u8,
                        );
                        if ((spriteId) as i32) != 64i32 {
                            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(400))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(
                                ((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                            );
                            (((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(400))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(46))
                            .cast::<i16>())
                            .write(((crate::c::rem_i32(((i) as i32), 5i32)) as i16));
                            ((((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(400))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .write(((crate::c::div_i32(((i) as i32), 5i32)) as i16));
                        } else {
                            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(400))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(core::ptr::null_mut());
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1814)).write(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateSwirlSandstormSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        if !((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1815)).read()) != 0)
        {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 5i32) {
                        break 'l1;
                    }
                    'l2: {
                        spriteId = CreateSpriteAtEnd(
                            (&raw const sSandstormSpriteTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                            (((((i) as i32).wrapping_mul(48i32)).wrapping_add(24i32)) as i16),
                            208i16,
                            1u8,
                        );
                        if ((spriteId) as i32) != 64i32 {
                            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(480))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(
                                ((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                            );
                            crate::c::bf_write(
                                (((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                    .wrapping_add(480))
                                .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read())
                                .wrapping_add(3),
                                6,
                                2,
                                (2u32) as i32,
                            );
                            ((((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(480))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .write(((((i) as i32).wrapping_mul(51i32)) as i16));
                            (((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(480))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(46))
                            .cast::<i16>())
                            .write(8i16);
                            ((((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(480))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(2))
                            .write(0i16);
                            ((((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(480))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(4))
                            .write(26416i16);
                            ((((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(480))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(3))
                            .write(
                                ((((((&raw const sSwirlEntranceDelays)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<u16>())
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i16),
                            );
                            StartSpriteAnim(
                                ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                    .wrapping_add(480))
                                .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                                1u8,
                            );
                            CalcCenterToCornerVec(
                                ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                    .wrapping_add(480))
                                .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                                0u8,
                                2u8,
                                0u8,
                            );
                            ((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(480))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                            .write(Some(WaitSandSwirlSpriteEntrance));
                        } else {
                            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                                .wrapping_add(480))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(core::ptr::null_mut());
                        }
                        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1815))
                            .write(1u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateSandstormSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1808)
                .cast::<u16>())
            .read()) as i16),
        );
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            (((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1806)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(32i32))
            .wrapping_add(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(64i32),
            )) as i16),
        );
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) >= 272i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                (((((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1806)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(480i32))
                .wrapping_sub(
                    ((4i32).wrapping_sub(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                    ))
                    .wrapping_mul(64i32),
                )) as i16),
            );
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32) & 511i32) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn WaitSandSwirlSpriteEntrance(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == (-1i32)
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(UpdateSandstormSwirlSprite));
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateSandstormSwirlSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut x: u32 = 0u32;
        let mut y: u32 = 0u32;
        if (({
            let __p1 = (sprite).wrapping_add(34).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            < (-48i32)
        {
            ((sprite).wrapping_add(34).cast::<i16>()).write(208i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(4i16);
        }
        x = (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(
            ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    as isize,
            ))
            .read()) as i32),
        )) as u32);
        y = (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(
            ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    .wrapping_add(64i32)) as isize,
            ))
            .read()) as i32),
        )) as u32);
        ((sprite).wrapping_add(36).cast::<i16>()).write(((x >> 8) as i16));
        ((sprite).wrapping_add(38).cast::<i16>()).write(((y >> 8) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_add(10i32)
                & 255i32) as i16),
        );
        if (({
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            > 8i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p5).write(((__p5).read()).wrapping_add(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Shade_InitVars() {
    unsafe {
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1740)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1729)
            .cast::<i8>())
        .write(3i8);
        ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1730)).write(20u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Shade_InitAll() {
    unsafe {
        Shade_InitVars();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Shade_Main() {
    unsafe {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Shade_Finish() -> u8 {
    unsafe {
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Bubbles_InitVars() {
    unsafe {
        FogHorizontal_InitVars();
        if !((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1838)).read()) != 0)
        {
            LoadSpriteSheet(
                (&raw const sWeatherBubbleSpriteSheet)
                    .cast::<u8>()
                    .cast_mut(),
            );
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1832)
                .cast::<u16>())
            .write(0u16);
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1830)
                .cast::<u16>())
            .write(
                (((((&raw const sBubbleStartDelays).cast::<u8>().cast_mut()).cast::<u8>()).read())
                    as u16),
            );
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1834)
                .cast::<u16>())
            .write(0u16);
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1836)
                .cast::<u16>())
            .write(0u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Bubbles_InitAll() {
    unsafe {
        Bubbles_InitVars();
        'l1: loop {
            if !(!((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read()).wrapping_add(1746))
                .read())
                != 0))
            {
                break 'l1;
            }
            Bubbles_Main();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Bubbles_Main() {
    unsafe {
        FogHorizontal_Main();
        if (({
            let __p1 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1830)
                .cast::<u16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > ((((((&raw const sBubbleStartDelays).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                        .wrapping_add(1832)
                        .cast::<u16>())
                    .read()) as i32) as isize,
                ))
            .read()) as i32)
        {
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1830)
                .cast::<u16>())
            .write(0u16);
            if (({
                let __p3 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1832)
                    .cast::<u16>();
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as u32)
                > (crate::c::div_u32(8u32, 1u32)).wrapping_sub(1u32)
            {
                ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1832)
                    .cast::<u16>())
                .write(0u16);
            }
            CreateBubbleSprite(
                ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1834)
                    .cast::<u16>())
                .read(),
            );
            if (({
                let __p5 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1834)
                    .cast::<u16>();
                let __t6 = ((__p5).read()).wrapping_add(1);
                (__p5).write(__t6);
                __t6
            }) as u32)
                > (crate::c::div_u32(52u32, 4u32)).wrapping_sub(1u32)
            {
                ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                    .wrapping_add(1834)
                    .cast::<u16>())
                .write(0u16);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Bubbles_Finish() -> u8 {
    unsafe {
        if !((FogHorizontal_Finish()) != 0) {
            DestroyBubbleSprites();
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn CreateBubbleSprite(coordsIndex: u16) {
    unsafe {
        let mut coordsIndex = coordsIndex;
        let mut x: i16 = (((((&raw const sBubbleStartCoords).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((coordsIndex) as i32) as isize * 4))
        .cast::<i16>())
        .read();
        let mut y: i16 = ((((((((((&raw const sBubbleStartCoords).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((coordsIndex) as i32) as isize * 4))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            .wrapping_sub(((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as i32)))
            as i16);
        let mut spriteId: u8 = CreateSpriteAtEnd(
            (&raw const sBubbleSpriteTemplate).cast::<u8>().cast_mut(),
            x,
            y,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                2,
                2,
                (1u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                1,
                1,
                (1u16) as i32,
            );
            (((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .write(0i16);
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(0i16);
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(0i16);
            let __p1 = (((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1836)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyBubbleSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        if (((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
            .wrapping_add(1836)
            .cast::<u16>())
        .read())
            != 0
        {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 64i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 68))
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                        .read()) as usize)
                            == (((&raw const sBubbleSpriteTemplate).cast::<u8>().cast_mut())
                                as usize)
                        {
                            DestroySprite(
                                ((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 68),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            FreeSpriteTilesByTag(4613u16);
            ((((&raw mut gWeatherPtr).cast::<*mut u8>()).read())
                .wrapping_add(1836)
                .cast::<u16>())
            .write(0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateBubbleSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (({
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t3 = ((__p2).read()).wrapping_add(1);
            (__p2).write(__t3);
            __t3
        }) as i32)
            > 8i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                == 0i32
            {
                if (({
                    let __p4 = (sprite).wrapping_add(36).cast::<i16>();
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 4i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(1i16);
                }
            } else {
                if (({
                    let __p6 = (sprite).wrapping_add(36).cast::<i16>();
                    let __t7 = ((__p6).read()).wrapping_sub(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    <= 0i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                }
            }
        }
        let __p8 = (sprite).wrapping_add(34).cast::<i16>();
        (__p8).write((((((__p8).read()) as i32).wrapping_sub(3i32)) as i16));
        if (({
            let __p9 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            let __t10 = ((__p9).read()).wrapping_add(1);
            (__p9).write(__t10);
            __t10
        }) as i32)
            >= 120i32
        {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn UnusedSetCurrentAbnormalWeather(weather: u32, unknown: u32) {
    unsafe {
        let mut weather = weather;
        let mut unknown = unknown;
        ((&raw mut sCurrentAbnormalWeather).cast::<u8>().cast::<u8>()).write(((weather) as u8));
        ((&raw mut sUnusedWeatherRelated).cast::<u8>().cast::<u16>()).write(((unknown) as u16));
    }
}
pub(crate) unsafe extern "C" fn Task_DoAbnormalWeather(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (data).wrapping_offset(15);
                    let __t3 = (__p2).read();
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                    __t3
                }) as i32)
                    <= 0i32
                {
                    SetNextWeather(((((data).wrapping_offset(1)).read()) as u8));
                    ((&raw mut sCurrentAbnormalWeather).cast::<u8>().cast::<u8>())
                        .write(((((data).wrapping_offset(1)).read()) as u8));
                    ((data).wrapping_offset(15)).write(600i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p4 = (data).wrapping_offset(15);
                    let __t5 = (__p4).read();
                    (__p4).write(((__p4).read()).wrapping_sub(1));
                    __t5
                }) as i32)
                    <= 0i32
                {
                    SetNextWeather(((((data).wrapping_offset(2)).read()) as u8));
                    ((&raw mut sCurrentAbnormalWeather).cast::<u8>().cast::<u8>())
                        .write(((((data).wrapping_offset(2)).read()) as u8));
                    ((data).wrapping_offset(15)).write(600i16);
                    (data).write(0i16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateAbnormalWeatherTask() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_DoAbnormalWeather), 0u8);
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ((data).wrapping_offset(15)).write(600i16);
        if ((((&raw mut sCurrentAbnormalWeather).cast::<u8>().cast::<u8>()).read()) as i32) == 13i32
        {
            ((data).wrapping_offset(1)).write(12i16);
            ((data).wrapping_offset(2)).write(13i16);
        } else {
            if ((((&raw mut sCurrentAbnormalWeather).cast::<u8>().cast::<u8>()).read()) as i32)
                == 12i32
            {
                ((data).wrapping_offset(1)).write(13i16);
                ((data).wrapping_offset(2)).write(12i16);
            } else {
                ((&raw mut sCurrentAbnormalWeather).cast::<u8>().cast::<u8>()).write(13u8);
                ((data).wrapping_offset(1)).write(12i16);
                ((data).wrapping_offset(2)).write(13i16);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSavedWeather(weather: u32) {
    unsafe {
        let mut weather = weather;
        let mut oldWeather: u8 =
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(46)).read();
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(46))
            .write(TranslateWeatherNum(((weather) as u8)));
        UpdateRainCounter(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(46)).read(),
            oldWeather,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSavedWeather() -> u8 {
    unsafe {
        return ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(46)).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSavedWeatherFromCurrMapHeader() {
    unsafe {
        let mut oldWeather: u8 =
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(46)).read();
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(46)).write(
            TranslateWeatherNum((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(22)).read()),
        );
        UpdateRainCounter(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(46)).read(),
            oldWeather,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWeather(weather: u32) {
    unsafe {
        let mut weather = weather;
        SetSavedWeather(weather);
        SetNextWeather(GetSavedWeather());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWeather_Unused(weather: u32) {
    unsafe {
        let mut weather = weather;
        SetSavedWeather(weather);
        SetCurrentAndNextWeather(GetSavedWeather());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoCurrentWeather() {
    unsafe {
        let mut weather: u8 = GetSavedWeather();
        if ((weather) as i32) == 15i32 {
            if !((FuncIsActiveTask(Some(Task_DoAbnormalWeather))) != 0) {
                CreateAbnormalWeatherTask();
            }
            weather = ((&raw mut sCurrentAbnormalWeather).cast::<u8>().cast::<u8>()).read();
        } else {
            if (FuncIsActiveTask(Some(Task_DoAbnormalWeather))) != 0 {
                DestroyTask(FindTaskIdByFunc(Some(Task_DoAbnormalWeather)));
            }
            ((&raw mut sCurrentAbnormalWeather).cast::<u8>().cast::<u8>()).write(13u8);
        }
        SetNextWeather(weather);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResumePausedWeather() {
    unsafe {
        let mut weather: u8 = GetSavedWeather();
        if ((weather) as i32) == 15i32 {
            if !((FuncIsActiveTask(Some(Task_DoAbnormalWeather))) != 0) {
                CreateAbnormalWeatherTask();
            }
            weather = ((&raw mut sCurrentAbnormalWeather).cast::<u8>().cast::<u8>()).read();
        } else {
            if (FuncIsActiveTask(Some(Task_DoAbnormalWeather))) != 0 {
                DestroyTask(FindTaskIdByFunc(Some(Task_DoAbnormalWeather)));
            }
            ((&raw mut sCurrentAbnormalWeather).cast::<u8>().cast::<u8>()).write(13u8);
        }
        SetCurrentAndNextWeather(weather);
    }
}
pub(crate) unsafe extern "C" fn TranslateWeatherNum(weather: u8) -> u8 {
    unsafe {
        let mut weather = weather;
        'l1: {
            let __sw1 = ((weather) as i32);
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
                || __sw1 == 20i32
                || __sw1 == 21i32;
            if __sw1 == 0i32 {
                return 0u8;
            }
            if __sw1 == 1i32 {
                return 1u8;
            }
            if __sw1 == 2i32 {
                return 2u8;
            }
            if __sw1 == 3i32 {
                return 3u8;
            }
            if __sw1 == 4i32 {
                return 4u8;
            }
            if __sw1 == 5i32 {
                return 5u8;
            }
            if __sw1 == 6i32 {
                return 6u8;
            }
            if __sw1 == 7i32 {
                return 7u8;
            }
            if __sw1 == 8i32 {
                return 8u8;
            }
            if __sw1 == 9i32 {
                return 9u8;
            }
            if __sw1 == 10i32 {
                return 10u8;
            }
            if __sw1 == 11i32 {
                return 11u8;
            }
            if __sw1 == 12i32 {
                return 12u8;
            }
            if __sw1 == 13i32 {
                return 13u8;
            }
            if __sw1 == 14i32 {
                return 14u8;
            }
            if __sw1 == 15i32 {
                return 15u8;
            }
            if __sw1 == 20i32 {
                return ((((&raw const sWeatherCycleRoute119).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(47))
                        .read()) as i32) as isize,
                ))
                .read();
            }
            if __sw1 == 21i32 {
                return ((((&raw const sWeatherCycleRoute123).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(47))
                        .read()) as i32) as isize,
                ))
                .read();
            }
            if !__matched {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateWeatherPerDay(increment: u16) {
    unsafe {
        let mut increment = increment;
        let mut weatherStage: u16 = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(47))
        .read()) as i32)
            .wrapping_add(((increment) as i32))) as u16);
        weatherStage = ((crate::c::rem_i32(((weatherStage) as i32), 4i32)) as u16);
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(47))
            .write(((weatherStage) as u8));
    }
}
pub(crate) unsafe extern "C" fn UpdateRainCounter(newWeather: u8, oldWeather: u8) {
    unsafe {
        let mut newWeather = newWeather;
        let mut oldWeather = oldWeather;
        if (((newWeather) as i32) != ((oldWeather) as i32))
            && ((((newWeather) as i32) == 3i32) || (((newWeather) as i32) == 5i32))
        {
            IncrementGameStat(40u8);
        }
    }
}
