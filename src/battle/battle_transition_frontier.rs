//! Translated from `src/battle_transition_frontier.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sLogoCenter_Gfx sLogoCenter_Tilemap sLogoCircles_Gfx sLogo_Pal sFiller sOamData_LogoCircles sSpriteSheet_LogoCircles sSpritePalette_LogoCircles sAnim_LogoCircle_Top sAnim_LogoCircle_Left sAnim_LogoCircle_Right sAnimTable_LogoCircles sSpriteTemplate_LogoCircles sFrontierCirclesMeet_Funcs sFrontierCirclesCross_Funcs sFrontierCirclesAsymmetricSpiral_Funcs sFrontierCirclesSymmetricSpiral_Funcs sFrontierCirclesMeetInSeq_Funcs sFrontierCirclesCrossInSeq_Funcs sFrontierCirclesAsymmetricSpiralInSeq_Funcs sFrontierCirclesSymmetricSpiralInSeq_Funcs
#[allow(unused_imports)]
use crate::data::battle_transition_frontier::*;

unsafe extern "C" {
    static mut gPaletteFade: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn Cos2(a0: u16) -> i16;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetBg0TilesDst(a0: *mut *mut u16, a1: *mut *mut u16);
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn Sin2(a0: u16) -> i16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
}

pub(crate) unsafe extern "C" fn LoadLogoGfx() {
    unsafe {
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        LZ77UnCompVram(
            ((&raw const sLogoCenter_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (tileset).cast::<u8>(),
        );
        LZ77UnCompVram(
            ((&raw const sLogoCenter_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (tilemap).cast::<u8>(),
        );
        LoadPalette(
            (((&raw const sLogo_Pal).cast::<u8>().cast_mut().cast::<u16>()).cast::<u16>())
                .cast::<u8>(),
            240u16,
            32u16,
        );
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_LogoCircles)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadSpritePalette(
            (&raw const sSpritePalette_LogoCircles)
                .cast::<u8>()
                .cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateSlidingLogoCircleSprite(
    x: i16,
    y: i16,
    delayX: u8,
    delayY: u8,
    speedX: i8,
    speedY: i8,
    spriteAnimNum: u8,
) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut delayX = delayX;
        let mut delayY = delayY;
        let mut speedX = speedX;
        let mut speedY = speedY;
        let mut spriteAnimNum = spriteAnimNum;
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_LogoCircles)
                .cast::<u8>()
                .cast_mut(),
            x,
            y,
            0u8,
        );
        'l1: {
            let __sw1 = ((spriteAnimNum) as i32);
            if __sw1 == 0i32 {
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(120i16);
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(45i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(89i16);
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(97i16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(151i16);
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(97i16);
                break 'l1;
            }
        }
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((speedX) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((speedY) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((delayX) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((delayY) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(0i16);
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            spriteAnimNum,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_LogoCircleSlide));
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_LogoCircleSlide(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut data: *mut i16 = ((sprite).wrapping_add(46)).cast::<i16>();
        if (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) == (((data).read()) as i32))
            && (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                == ((((data).wrapping_offset(1)).read()) as i32))
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        } else {
            if ((((data).wrapping_offset(4)).read()) as i32)
                == ((((data).wrapping_offset(6)).read()) as i32)
            {
                let __p1 = (sprite).wrapping_add(32).cast::<i16>();
                (__p1).write(
                    (((((__p1).read()) as i32)
                        .wrapping_add(((((data).wrapping_offset(2)).read()) as i32)))
                        as i16),
                );
                ((data).wrapping_offset(4)).write(0i16);
            } else {
                let __p2 = (data).wrapping_offset(4);
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if ((((data).wrapping_offset(5)).read()) as i32)
                == ((((data).wrapping_offset(7)).read()) as i32)
            {
                let __p3 = (sprite).wrapping_add(34).cast::<i16>();
                (__p3).write(
                    (((((__p3).read()) as i32)
                        .wrapping_add(((((data).wrapping_offset(3)).read()) as i32)))
                        as i16),
                );
                ((data).wrapping_offset(5)).write(0i16);
            } else {
                let __p4 = (data).wrapping_offset(5);
                (__p4).write(((__p4).read()).wrapping_add(1));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateSpiralingLogoCircleSprite(
    x: i16,
    y: i16,
    angle: i16,
    rotateSpeed: i16,
    radiusStart: i16,
    radiusEnd: i16,
    radiusDelta: i16,
    spriteAnimNum: u8,
) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut angle = angle;
        let mut rotateSpeed = rotateSpeed;
        let mut radiusStart = radiusStart;
        let mut radiusEnd = radiusEnd;
        let mut radiusDelta = radiusDelta;
        let mut spriteAnimNum = spriteAnimNum;
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_LogoCircles)
                .cast::<u8>()
                .cast_mut(),
            x,
            y,
            0u8,
        );
        'l1: {
            let __sw1 = ((spriteAnimNum) as i32);
            if __sw1 == 0i32 {
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(120i16);
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(45i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(89i16);
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(97i16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(151i16);
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(97i16);
                break 'l1;
            }
        }
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(angle);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(rotateSpeed);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(radiusStart);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(radiusEnd);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(radiusDelta);
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            spriteAnimNum,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_LogoCircleSpiral));
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_LogoCircleSpiral(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((Sin2(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u16),
            )) as i32)
                .wrapping_mul(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32),
                )
                >> 12) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((Cos2(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u16),
            )) as i32)
                .wrapping_mul(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32),
                )
                >> 12) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((crate::c::rem_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    .wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                    ),
                360i32,
            )) as i16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32),
                )) as i16),
            );
        } else {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyLogoCirclesGfx(task: *mut u8) {
    unsafe {
        let mut task = task;
        FreeSpriteTilesByTag(11920u16);
        FreeSpritePaletteByTag(11920u16);
        DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                as isize
                * 68,
        ));
        DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                as isize
                * 68,
        ));
        DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                as isize
                * 68,
        ));
    }
}
pub(crate) unsafe extern "C" fn IsLogoCirclesAnimFinished(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if ((core::mem::transmute::<_, usize>(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read(),
        ) == (SpriteCallbackDummy as *const () as usize))
            && (core::mem::transmute::<_, usize>(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .read(),
            ) == (SpriteCallbackDummy as *const () as usize)))
            && (core::mem::transmute::<_, usize>(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .read(),
            ) == (SpriteCallbackDummy as *const () as usize))
        {
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
pub(crate) unsafe extern "C" fn Circles_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 0i32 {
            ClearGpuRegBits(0u8, 8192u16);
            ClearGpuRegBits(0u8, 16384u16);
            ClearGpuRegBits(0u8, 256u16);
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_add(1));
            return 0u8;
        } else {
            LoadLogoGfx();
            SetGpuReg(80u8, 16193u16);
            SetGpuReg(82u8, 4096u16);
            ChangeBgX(0u8, 0i32, 0u8);
            ChangeBgY(0u8, 0i32, 0u8);
            ChangeBgY(0u8, 1280i32, 2u8);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn FadeInCenterLogoCircle(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) == 0i32 {
            SetGpuRegBits(0u8, 256u16);
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) == 16i32
        {
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                == 31i32
            {
                BeginNormalPaletteFade(4294967295u32, (-1i8), 0u8, 16u8, 0u16);
                let __p1 = ((task).wrapping_add(8)).cast::<i16>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            } else {
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
        } else {
            let mut blnd: u16 = 0u16;
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            (__p3).write(((__p3).read()).wrapping_add(1));
            blnd = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u16);
            SetGpuReg(
                82u8,
                ((((16i32).wrapping_sub(((blnd) as i32)) << 8) | ((blnd) as i32)) as u16),
            );
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WaitForLogoCirclesAnim(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if ((IsLogoCirclesAnimFinished(task)) as i32) == 1i32 {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_FrontierCirclesMeet(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sFrontierCirclesMeet_Funcs)
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
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CirclesMeet_CreateSprites(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
            ((CreateSlidingLogoCircleSprite(120i16, (-51i16), 0u8, 0u8, 0i8, 2i8, 0u8)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
            ((CreateSlidingLogoCircleSprite((-7i16), 193i16, 0u8, 0u8, 2i8, (-2i8), 1u8)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
            ((CreateSlidingLogoCircleSprite(247i16, 193i16, 0u8, 0u8, (-2i8), (-2i8), 2u8)) as i16),
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CirclesMeet_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            DestroyLogoCirclesGfx(task);
            DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesMeet)));
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_FrontierCirclesCross(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sFrontierCirclesCross_Funcs)
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
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CirclesCross_CreateSprites(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
            ((CreateSlidingLogoCircleSprite(120i16, 197i16, 0u8, 0u8, 0i8, (-4i8), 0u8)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
            ((CreateSlidingLogoCircleSprite(241i16, 59i16, 0u8, 1u8, (-4i8), 2i8, 1u8)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
            ((CreateSlidingLogoCircleSprite((-1i16), 59i16, 0u8, 1u8, 4i8, 2i8, 2u8)) as i16),
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CirclesCross_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            DestroyLogoCirclesGfx(task);
            DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesCross)));
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_FrontierCirclesAsymmetricSpiral(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sFrontierCirclesAsymmetricSpiral_Funcs)
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
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CirclesAsymmetricSpiral_CreateSprites(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
            ((CreateSpiralingLogoCircleSprite(
                120i16,
                45i16,
                12i16,
                4i16,
                128i16,
                0i16,
                (-4i16),
                0u8,
            )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
            ((CreateSpiralingLogoCircleSprite(
                89i16,
                97i16,
                252i16,
                4i16,
                128i16,
                0i16,
                (-4i16),
                1u8,
            )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
            ((CreateSpiralingLogoCircleSprite(
                151i16,
                97i16,
                132i16,
                4i16,
                128i16,
                0i16,
                (-4i16),
                2u8,
            )) as i16),
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CirclesAsymmetricSpiral_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            DestroyLogoCirclesGfx(task);
            DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesAsymmetricSpiral)));
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_FrontierCirclesSymmetricSpiral(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sFrontierCirclesSymmetricSpiral_Funcs)
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
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CirclesSymmetricSpiral_CreateSprites(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
            ((CreateSpiralingLogoCircleSprite(
                120i16,
                80i16,
                284i16,
                8i16,
                131i16,
                35i16,
                (-3i16),
                0u8,
            )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
            ((CreateSpiralingLogoCircleSprite(
                120i16,
                80i16,
                44i16,
                8i16,
                131i16,
                35i16,
                (-3i16),
                1u8,
            )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
            ((CreateSpiralingLogoCircleSprite(
                121i16,
                80i16,
                164i16,
                8i16,
                131i16,
                35i16,
                (-3i16),
                2u8,
            )) as i16),
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CirclesSymmetricSpiral_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            DestroyLogoCirclesGfx(task);
            DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesSymmetricSpiral)));
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_FrontierCirclesMeetInSeq(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sFrontierCirclesMeetInSeq_Funcs)
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
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CirclesMeetInSeq_CreateSprites(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 0i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
                ((CreateSlidingLogoCircleSprite(120i16, (-51i16), 0u8, 0u8, 0i8, 4i8, 0u8)) as i16),
            );
        } else {
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                == 16i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
                    ((CreateSlidingLogoCircleSprite((-7i16), 193i16, 0u8, 0u8, 4i8, (-4i8), 1u8))
                        as i16),
                );
            } else {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    == 32i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
                        ((CreateSlidingLogoCircleSprite(
                            247i16,
                            193i16,
                            0u8,
                            0u8,
                            (-4i8),
                            (-4i8),
                            2u8,
                        )) as i16),
                    );
                    let __p1 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p1).write(((__p1).read()).wrapping_add(1));
                }
            }
        }
        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CirclesMeetInSeq_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            DestroyLogoCirclesGfx(task);
            DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesMeetInSeq)));
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_FrontierCirclesCrossInSeq(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sFrontierCirclesCrossInSeq_Funcs)
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
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CirclesCrossInSeq_CreateSprites(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 0i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
                ((CreateSlidingLogoCircleSprite(120i16, 197i16, 0u8, 0u8, 0i8, (-8i8), 0u8))
                    as i16),
            );
        } else {
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                == 16i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
                    ((CreateSlidingLogoCircleSprite(241i16, 78i16, 0u8, 0u8, (-8i8), 1i8, 1u8))
                        as i16),
                );
            } else {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    == 32i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
                        ((CreateSlidingLogoCircleSprite((-1i16), 78i16, 0u8, 0u8, 8i8, 1i8, 2u8))
                            as i16),
                    );
                    let __p1 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p1).write(((__p1).read()).wrapping_add(1));
                }
            }
        }
        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CirclesCrossInSeq_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            DestroyLogoCirclesGfx(task);
            DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesCrossInSeq)));
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_FrontierCirclesAsymmetricSpiralInSeq(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sFrontierCirclesAsymmetricSpiralInSeq_Funcs)
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
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CirclesAsymmetricSpiralInSeq_CreateSprites(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 0i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
                ((CreateSpiralingLogoCircleSprite(
                    120i16,
                    45i16,
                    12i16,
                    4i16,
                    128i16,
                    0i16,
                    (-4i16),
                    0u8,
                )) as i16),
            );
        } else {
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                == 16i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
                    ((CreateSpiralingLogoCircleSprite(
                        89i16,
                        97i16,
                        252i16,
                        4i16,
                        128i16,
                        0i16,
                        (-4i16),
                        1u8,
                    )) as i16),
                );
            } else {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    == 32i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
                        ((CreateSpiralingLogoCircleSprite(
                            151i16,
                            97i16,
                            132i16,
                            4i16,
                            128i16,
                            0i16,
                            (-4i16),
                            2u8,
                        )) as i16),
                    );
                    let __p1 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p1).write(((__p1).read()).wrapping_add(1));
                }
            }
        }
        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CirclesAsymmetricSpiralInSeq_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            DestroyLogoCirclesGfx(task);
            DestroyTask(FindTaskIdByFunc(Some(
                Task_FrontierCirclesAsymmetricSpiralInSeq,
            )));
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_FrontierCirclesSymmetricSpiralInSeq(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sFrontierCirclesSymmetricSpiralInSeq_Funcs)
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
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CirclesSymmetricSpiralInSeq_CreateSprites(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 0i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
                ((CreateSpiralingLogoCircleSprite(
                    120i16,
                    80i16,
                    284i16,
                    8i16,
                    131i16,
                    35i16,
                    (-3i16),
                    0u8,
                )) as i16),
            );
        } else {
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                == 16i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
                    ((CreateSpiralingLogoCircleSprite(
                        120i16,
                        80i16,
                        44i16,
                        8i16,
                        131i16,
                        35i16,
                        (-3i16),
                        1u8,
                    )) as i16),
                );
            } else {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    == 32i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
                        ((CreateSpiralingLogoCircleSprite(
                            121i16,
                            80i16,
                            164i16,
                            8i16,
                            131i16,
                            35i16,
                            (-3i16),
                            2u8,
                        )) as i16),
                    );
                    let __p1 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p1).write(((__p1).read()).wrapping_add(1));
                }
            }
        }
        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CirclesSymmetricSpiralInSeq_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            DestroyLogoCirclesGfx(task);
            DestroyTask(FindTaskIdByFunc(Some(
                Task_FrontierCirclesSymmetricSpiralInSeq,
            )));
        }
        return 0u8;
    }
}
