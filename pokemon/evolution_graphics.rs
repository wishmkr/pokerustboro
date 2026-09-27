//! Translated from `src/evolution_graphics.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sEvoSparkle_Pal sEvoSparkle_Gfx sEvoSparkleSpriteSheets sEvoSparkleSpritePals sOamData_EvoSparkle sSpriteAnim_EvoSparkle sSpriteAnimTable_EvoSparkle sEvoSparkleSpriteTemplate sEvoSparkleMatrices sUnused
#[allow(unused_imports)]
use crate::data::evolution_graphics::*;

unsafe extern "C" {
    static mut gPaletteFade: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn LoadCompressedSpriteSheetUsingHeap(a0: *mut u8) -> u8;
    fn LoadSpritePalettes(a0: *mut u8);
    fn PlaySE(a0: u16);
    fn Random() -> u16;
    fn SetOamMatrix(a0: u8, a1: u16, a2: u16, a3: u16, a4: u16);
    fn Sin(a0: i16, a1: i16) -> i16;
}

pub(crate) unsafe extern "C" fn SpriteCB_Sparkle_Dummy(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
    }
}
pub(crate) unsafe extern "C" fn SetEvoSparklesMatrices() {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(24u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    SetOamMatrix(
                        (((20i32).wrapping_add(((i) as i32))) as u8),
                        ((((&raw const sEvoSparkleMatrices)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                        0u16,
                        0u16,
                        ((((&raw const sEvoSparkleMatrices)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Sparkle_SpiralUpward(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) > 8i32 {
            let mut matrixNum: u8 = 0u8;
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                (((88i32).wrapping_sub(crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        .wrapping_mul(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as i32),
                        ),
                    80i32,
                ))) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((crate::c::div_i32(
                    ((Sin(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as u8) as i16),
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                    )) as i32),
                    4i32,
                )) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u8)
                    as i16),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
            ));
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                & 1i32)
                != 0
            {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p2).write(((__p2).read()).wrapping_sub(1));
            }
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p3).write(((__p3).read()).wrapping_add(1));
            if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) > 0i32 {
                ((sprite).wrapping_add(67)).write(1u8);
            } else {
                ((sprite).wrapping_add(67)).write(20u8);
            }
            matrixNum = (((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
                4i32,
            ))
            .wrapping_add(20i32)) as u8);
            if ((matrixNum) as i32) > 31i32 {
                matrixNum = 31u8;
            }
            crate::c::bf_write((sprite).wrapping_add(3), 1, 5, ((matrixNum) as u32) as i32);
        } else {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateSparkle_SpiralUpward(trigIdx: u8) {
    unsafe {
        let mut trigIdx = trigIdx;
        let mut spriteId: u8 = CreateSprite(
            (&raw const sEvoSparkleSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            ((crate::c::div_i32(240i32, 2i32)) as i16),
            88i16,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(48i16);
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(((trigIdx) as i16));
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(0i16);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                0,
                2,
                (1u32) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
                1,
                5,
                (31u32) as i32,
            );
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_Sparkle_SpiralUpward));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Sparkle_ArcDown(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) < 88i32 {
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                (((8i32).wrapping_add(crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        .wrapping_mul(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as i32),
                        ),
                    5i32,
                ))) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((crate::c::div_i32(
                    ((Sin(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as u8) as i16),
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                    )) as i32),
                    4i32,
                )) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u8)
                    as i16),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
            ));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                (((8i32).wrapping_add(
                    ((Sin(
                        (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                            .read()) as i32)
                            .wrapping_mul(4i32)) as u8) as i16),
                        40i16,
                    )) as i32),
                )) as i16),
            );
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateSparkle_ArcDown(trigIdx: u8) {
    unsafe {
        let mut trigIdx = trigIdx;
        let mut spriteId: u8 = CreateSprite(
            (&raw const sEvoSparkleSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            ((crate::c::div_i32(240i32, 2i32)) as i16),
            8i16,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(8i16);
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(((trigIdx) as i16));
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(0i16);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                0,
                2,
                (1u32) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
                1,
                5,
                (25u32) as i32,
            );
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(67))
            .write(1u8);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_Sparkle_ArcDown));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Sparkle_CircleInward(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32) > 8i32
        {
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u8)
                    as i16),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
            ));
            ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u8)
                    as i16),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
            ));
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32),
                )) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(4i32)) as i16));
        } else {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateSparkle_CircleInward(trigIdx: u8, speed: u8) {
    unsafe {
        let mut trigIdx = trigIdx;
        let mut speed = speed;
        let mut spriteId: u8 = CreateSprite(
            (&raw const sEvoSparkleSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            ((crate::c::div_i32(240i32, 2i32)) as i16),
            56i16,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(((speed) as i16));
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(120i16);
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(((trigIdx) as i16));
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(0i16);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                0,
                2,
                (1u32) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
                1,
                5,
                (31u32) as i32,
            );
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(67))
            .write(1u8);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_Sparkle_CircleInward));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Sparkle_Spray(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            & 3i32)
            != 0)
        {
            let __p1 = (sprite).wrapping_add(34).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
            < 128i32
        {
            let mut matrixNum: u8 = 0u8;
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as u8) as i16),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                )) as i32)
                    .wrapping_neg()) as i16),
            );
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                (((crate::c::div_i32(240i32, 2i32)).wrapping_add(crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        .wrapping_mul(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as i32),
                        ),
                    3i32,
                ))) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p2).write(((__p2).read()).wrapping_add(1));
            matrixNum = (((31i32).wrapping_sub(crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    .wrapping_mul(12i32),
                128i32,
            ))) as u8);
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                > 64i32
            {
                ((sprite).wrapping_add(67)).write(1u8);
            } else {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                ((sprite).wrapping_add(67)).write(20u8);
                if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                    as i32)
                    > 112i32)
                    && ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        & 1i32)
                        != 0)
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                }
            }
            if ((matrixNum) as i32) < 20i32 {
                matrixNum = 20u8;
            }
            crate::c::bf_write((sprite).wrapping_add(3), 1, 5, ((matrixNum) as u32) as i32);
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p3).write(((__p3).read()).wrapping_add(1));
        } else {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateSparkle_Spray(id: u8) {
    unsafe {
        let mut id = id;
        let mut spriteId: u8 = CreateSprite(
            (&raw const sEvoSparkleSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            ((crate::c::div_i32(240i32, 2i32)) as i16),
            56i16,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(3))
            .write((((3i32).wrapping_sub(crate::c::rem_i32(((Random()) as i32), 7i32))) as i16));
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(5))
            .write((((48i32).wrapping_add((((Random()) as i32) & 63i32))) as i16));
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(0i16);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                0,
                2,
                (1u32) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
                1,
                5,
                (31u32) as i32,
            );
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(67))
            .write(20u8);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_Sparkle_Spray));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadEvoSparkleSpriteAndPal() {
    unsafe {
        LoadCompressedSpriteSheetUsingHeap(
            ((&raw const sEvoSparkleSpriteSheets).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        LoadSpritePalettes(
            ((&raw const sEvoSparkleSpritePals).cast::<u8>().cast_mut()).cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EvolutionSparkles_SpiralUpward(palNum: u16) -> u8 {
    unsafe {
        let mut palNum = palNum;
        let mut taskId: u8 = CreateTask(Some(Task_Sparkles_SpiralUpward_Init), 0u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((palNum) as i16));
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_SpiralUpward_Init(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetEvoSparklesMatrices();
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(0i16);
        BeginNormalPaletteFade(
            ((crate::c::shl_i32(
                3i32,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u32),
            )) as u32),
            10i8,
            0u8,
            16u8,
            32767u16,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Sparkles_SpiralUpward));
        PlaySE(140u16);
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_SpiralUpward(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as i32)
            < 64i32
        {
            if !((((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .read()) as i32)
                & 7i32)
                != 0)
            {
                let mut i: u8 = 0u8;
                {
                    i = 0u8;
                    'l1: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l1;
                        }
                        'l2: {
                            CreateSparkle_SpiralUpward(
                                ((((((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(15))
                                .read()) as i32)
                                    & 120i32)
                                    .wrapping_mul(2i32))
                                .wrapping_add(((i) as i32).wrapping_mul(64i32)))
                                    as u8),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .write(96i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Sparkles_SpiralUpward_End));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_SpiralUpward_End(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as i32)
            != 0i32
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EvolutionSparkles_ArcDown() -> u8 {
    unsafe {
        return CreateTask(Some(Task_Sparkles_ArcDown_Init), 0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_ArcDown_Init(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetEvoSparklesMatrices();
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Sparkles_ArcDown));
        PlaySE(183u16);
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_ArcDown(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as i32)
            < 96i32
        {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .read()) as i32)
                < 6i32
            {
                let mut i: u8 = 0u8;
                {
                    i = 0u8;
                    'l1: loop {
                        if !(((i) as i32) < 9i32) {
                            break 'l1;
                        }
                        'l2: {
                            CreateSparkle_ArcDown(((((i) as i32).wrapping_mul(16i32)) as u8));
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Sparkles_ArcDown_End));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_ArcDown_End(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EvolutionSparkles_CircleInward() -> u8 {
    unsafe {
        return CreateTask(Some(Task_Sparkles_CircleInward_Init), 0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_CircleInward_Init(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetEvoSparklesMatrices();
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Sparkles_CircleInward));
        PlaySE(102u16);
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_CircleInward(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as i32)
            < 48i32
        {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .read()) as i32)
                == 0i32
            {
                let mut i: u8 = 0u8;
                {
                    i = 0u8;
                    'l1: loop {
                        if !(((i) as i32) < 16i32) {
                            break 'l1;
                        }
                        'l2: {
                            CreateSparkle_CircleInward(
                                ((((i) as i32).wrapping_mul(16i32)) as u8),
                                4u8,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .read()) as i32)
                == 32i32
            {
                let mut i: u8 = 0u8;
                {
                    i = 0u8;
                    'l3: loop {
                        if !(((i) as i32) < 16i32) {
                            break 'l3;
                        }
                        'l4: {
                            CreateSparkle_CircleInward(
                                ((((i) as i32).wrapping_mul(16i32)) as u8),
                                8u8,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Sparkles_CircleInward_End));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_CircleInward_End(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EvolutionSparkles_SprayAndFlash(species: u16) -> u8 {
    unsafe {
        let mut species = species;
        let mut taskId: u8 = CreateTask(Some(Task_Sparkles_SprayAndFlash_Init), 0u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((species) as i16));
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_SprayAndFlash_Init(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetEvoSparklesMatrices();
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(0i16);
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(32))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(32))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    96u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
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
        BeginNormalPaletteFade(4294509596u32, 0i8, 0u8, 16u8, 32767u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Sparkles_SprayAndFlash));
        PlaySE(202u16);
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_SprayAndFlash(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as i32)
            < 128i32
        {
            let mut i: u8 = 0u8;
            'l1: {
                let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as i32);
                let __matched = __sw1 == 0i32 || __sw1 == 32i32;
                if !__matched {
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15))
                    .read()) as i32)
                        < 50i32
                    {
                        CreateSparkle_Spray(((((Random()) as i32) & 7i32) as u8));
                    }
                    break 'l1;
                }
                if __sw1 == 0i32 {
                    {
                        i = 0u8;
                        'l2: loop {
                            if !(((i) as i32) < 8i32) {
                                break 'l2;
                            }
                            'l3: {
                                CreateSparkle_Spray(i);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 32i32 {
                    BeginNormalPaletteFade(4294902812u32, 16i8, 16u8, 0u8, 32767u16);
                    break 'l1;
                }
            }
            let __p2 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15);
            (__p2).write(((__p2).read()).wrapping_add(1));
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Sparkles_SprayAndFlash_End));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_SprayAndFlash_End(taskId: u8) {
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
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EvolutionSparkles_SprayAndFlash_Trade(species: u16) -> u8 {
    unsafe {
        let mut species = species;
        let mut taskId: u8 = CreateTask(Some(Task_Sparkles_SprayAndFlashTrade_Init), 0u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((species) as i16));
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_SprayAndFlashTrade_Init(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetEvoSparklesMatrices();
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(0i16);
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(32))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(32))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    96u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
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
        BeginNormalPaletteFade(4294509568u32, 0i8, 0u8, 16u8, 32767u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_Sparkles_SprayAndFlashTrade));
        PlaySE(202u16);
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_SprayAndFlashTrade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as i32)
            < 128i32
        {
            let mut i: u8 = 0u8;
            'l1: {
                let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as i32);
                let __matched = __sw1 == 0i32 || __sw1 == 32i32;
                if !__matched {
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15))
                    .read()) as i32)
                        < 50i32
                    {
                        CreateSparkle_Spray(((((Random()) as i32) & 7i32) as u8));
                    }
                    break 'l1;
                }
                if __sw1 == 0i32 {
                    {
                        i = 0u8;
                        'l2: loop {
                            if !(((i) as i32) < 8i32) {
                                break 'l2;
                            }
                            'l3: {
                                CreateSparkle_Spray(i);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 32i32 {
                    BeginNormalPaletteFade(4294902784u32, 16i8, 16u8, 0u8, 32767u16);
                    break 'l1;
                }
            }
            let __p2 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15);
            (__p2).write(((__p2).read()).wrapping_add(1));
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_Sparkles_SprayAndFlash_End));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_EvolutionMonSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CycleEvolutionMonSprite(preEvoSpriteId: u8, postEvoSpriteId: u8) -> u8 {
    unsafe {
        let mut preEvoSpriteId = preEvoSpriteId;
        let mut postEvoSpriteId = postEvoSpriteId;
        let mut i: u16 = 0u16;
        let mut monPalette = crate::ffi::Align4([0u8; 32]);
        let mut taskId: u8 = 0u8;
        let mut toDiv: i32 = 0i32;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(32u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut monPalette).cast::<u16>()).wrapping_offset(((i) as i32) as isize))
                        .write(32767u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        taskId = CreateTask(Some(Task_CycleEvolutionMonSprite_Init), 0u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((preEvoSpriteId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((postEvoSpriteId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(256i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(16i16);
        toDiv = 65536i32;
        SetOamMatrix(30u8, 256u16, 0u16, 0u16, 256u16);
        SetOamMatrix(
            31u8,
            ((crate::c::div_i32(
                toDiv,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as i32),
            )) as u16),
            0u16,
            0u16,
            ((crate::c::div_i32(
                toDiv,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as i32),
            )) as u16),
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((preEvoSpriteId) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_EvolutionMonSprite));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((preEvoSpriteId) as i32) as isize * 68))
            .wrapping_add(1),
            0,
            2,
            (1u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((preEvoSpriteId) as i32) as isize * 68))
            .wrapping_add(3),
            1,
            5,
            (30u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((preEvoSpriteId) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        'l3: loop {
            'l4: {
                CpuSet(
                    ((&raw mut monPalette).cast::<u16>()).cast::<u8>(),
                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                        ((256i32).wrapping_add(
                            ((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((preEvoSpriteId) as i32) as isize * 68))
                                .wrapping_add(5),
                                4,
                                4,
                                false,
                            ) as u16) as i32)
                                .wrapping_mul(16i32),
                        )) as isize,
                    ))
                    .cast::<u8>(),
                    16u32,
                );
            }
            if !((0i32) != 0) {
                break 'l3;
            }
        }
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((postEvoSpriteId) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_EvolutionMonSprite));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((postEvoSpriteId) as i32) as isize * 68))
            .wrapping_add(1),
            0,
            2,
            (1u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((postEvoSpriteId) as i32) as isize * 68))
            .wrapping_add(3),
            1,
            5,
            (31u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((postEvoSpriteId) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        'l5: loop {
            'l6: {
                CpuSet(
                    ((&raw mut monPalette).cast::<u16>()).cast::<u8>(),
                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                        ((256i32).wrapping_add(
                            ((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((postEvoSpriteId) as i32) as isize * 68))
                                .wrapping_add(5),
                                4,
                                4,
                                false,
                            ) as u16) as i32)
                                .wrapping_mul(16i32),
                        )) as isize,
                    ))
                    .cast::<u8>(),
                    16u32,
                );
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write(0i16);
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn Task_CycleEvolutionMonSprite_Init(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(8i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_CycleEvolutionMonSprite_TryEnd));
    }
}
pub(crate) unsafe extern "C" fn Task_CycleEvolutionMonSprite_TryEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .read())
            != 0
        {
            EndOnPreEvoMon(taskId);
        } else {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as i32)
                == 128i32
            {
                EndOnPostEvoMon(taskId);
            } else {
                let __p1 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6);
                (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
                let __p2 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5);
                (__p2).write((((((__p2).read()) as i32) ^ 1i32) as i16));
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_CycleEvolutionMonSprite_UpdateSize));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CycleEvolutionMonSprite_UpdateSize(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .read())
            != 0
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(EndOnPreEvoMon));
        } else {
            let mut oamMatrixArg: u16 = 0u16;
            let mut numSpritesFinished: u8 = 0u8;
            if !((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read())
                != 0)
            {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    < (256i32).wrapping_sub(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .read()) as i32),
                    )
                {
                    let __p1 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3);
                    (__p1).write(
                        (((((__p1).read()) as i32).wrapping_add(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6))
                            .read()) as i32),
                        )) as i16),
                    );
                } else {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(256i16);
                    numSpritesFinished = (numSpritesFinished).wrapping_add(1);
                }
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as i32)
                    > (16i32).wrapping_add(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .read()) as i32),
                    )
                {
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4);
                    (__p2).write(
                        (((((__p2).read()) as i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6))
                            .read()) as i32),
                        )) as i16),
                    );
                } else {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(16i16);
                    numSpritesFinished = (numSpritesFinished).wrapping_add(1);
                }
            } else {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as i32)
                    < (256i32).wrapping_sub(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .read()) as i32),
                    )
                {
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4);
                    (__p3).write(
                        (((((__p3).read()) as i32).wrapping_add(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6))
                            .read()) as i32),
                        )) as i16),
                    );
                } else {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(256i16);
                    numSpritesFinished = (numSpritesFinished).wrapping_add(1);
                }
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    > (16i32).wrapping_add(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .read()) as i32),
                    )
                {
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3);
                    (__p4).write(
                        (((((__p4).read()) as i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6))
                            .read()) as i32),
                        )) as i16),
                    );
                } else {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(16i16);
                    numSpritesFinished = (numSpritesFinished).wrapping_add(1);
                }
            }
            oamMatrixArg = ((crate::c::div_i32(
                65536i32,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32),
            )) as u16);
            SetOamMatrix(30u8, oamMatrixArg, 0u16, 0u16, oamMatrixArg);
            oamMatrixArg = ((crate::c::div_i32(
                65536i32,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as i32),
            )) as u16);
            SetOamMatrix(31u8, oamMatrixArg, 0u16, 0u16, oamMatrixArg);
            if ((numSpritesFinished) as i32) == 2i32 {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_CycleEvolutionMonSprite_TryEnd));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn EndOnPostEvoMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(1),
            0,
            2,
            (0u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(3),
            1,
            5,
            (0u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(1),
            0,
            2,
            (0u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(3),
            1,
            5,
            (0u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn EndOnPreEvoMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(1),
            0,
            2,
            (0u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(3),
            1,
            5,
            (0u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(1),
            0,
            2,
            (0u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(3),
            1,
            5,
            (0u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        DestroyTask(taskId);
    }
}
