//! Translated from `src/battle_intro.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sBattleIntroSlideFuncs
#[allow(unused_imports)]
use crate::data::battle_intro::*;

unsafe extern "C" {
    static mut gBattleMonForms: u8;
    static mut gBattleStruct: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattle_BG0_Y: u8;
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG1_Y: u8;
    static mut gBattle_BG2_X: u8;
    static mut gBattle_BG2_Y: u8;
    static mut gBattle_WIN0V: u8;
    static mut gGameVersion: u8;
    static mut gIntroSlideFlags: u8;
    static mut gMonSpritesGfxPtr: u8;
    static mut gPartnerTrainerId: u8;
    static mut gScanlineEffect: u8;
    static mut gScanlineEffectRegBuffers: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn Cos2(a0: u16) -> i16;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetGpuReg(a0: u8) -> u16;
    fn LoadBgTilemap(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SpriteCB_VsLetterInit(a0: *mut u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleIntroSlide(environment: u8) {
    unsafe {
        let mut environment = environment;
        let mut taskId: u8 = 0u8;
        if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4194304u32) != 0)
            && (((((&raw mut gPartnerTrainerId).cast::<u16>()).read()) as i32) != 3075i32)
        {
            taskId = CreateTask(Some(BattleIntroSlidePartner), 0u8);
        } else {
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0 {
                taskId = CreateTask(Some(BattleIntroSlideLink), 0u8);
            } else {
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4129024u32) != 0 {
                    taskId = CreateTask(Some(BattleIntroSlide3), 0u8);
                } else {
                    if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4096u32) != 0)
                        && (((((&raw mut gGameVersion).cast::<u8>()).read()) as i32) != 2i32)
                    {
                        environment = 3u8;
                        taskId = CreateTask(Some(BattleIntroSlide2), 0u8);
                    } else {
                        taskId = CreateTask(
                            ((((&raw const sBattleIntroSlideFuncs)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<Option<unsafe extern "C" fn(u8)>>())
                            .cast::<Option<unsafe extern "C" fn(u8)>>())
                            .wrapping_offset(((environment) as i32) as isize))
                            .read(),
                            0u8,
                        );
                    }
                }
            }
        }
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((environment) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn BattleIntroSlideEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DestroyTask(taskId);
        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        SetGpuReg(84u8, 0u16);
        SetGpuReg(72u8, 16191u16);
        SetGpuReg(74u8, 16191u16);
    }
}
pub(crate) unsafe extern "C" fn BattleIntroSlide1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let __p1 = (&raw mut gBattle_BG1_X).cast::<u16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(6i32)) as u16));
        'l1: {
            let __sw2 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw2 == 0i32 {
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0 {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(16i16);
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                } else {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(1i16);
                    let __p4 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw2 == 1i32 {
                if (({
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    let __t6 = ((__p5).read()).wrapping_sub(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    == 0i32
                {
                    let __p7 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    SetGpuReg(72u8, 63u16);
                }
                break 'l1;
            }
            if __sw2 == 2i32 {
                let __p8 = (&raw mut gBattle_WIN0V).cast::<u16>();
                (__p8).write((((((__p8).read()) as i32).wrapping_sub(255i32)) as u16));
                if (((((&raw mut gBattle_WIN0V).cast::<u16>()).read()) as i32) & 65280i32)
                    == 12288i32
                {
                    let __p9 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(240i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(32i16);
                    let __p10 = (&raw mut gIntroSlideFlags).cast::<u16>();
                    (__p10).write((((((__p10).read()) as i32) & (-2i32)) as u16));
                }
                break 'l1;
            }
            if __sw2 == 3i32 {
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read())
                    != 0
                {
                    let __p11 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3);
                    (__p11).write(((__p11).read()).wrapping_sub(1));
                } else {
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        == 1i32
                    {
                        if ((((&raw mut gBattle_BG1_Y).cast::<u16>()).read()) as i32) != 65456i32 {
                            let __p12 = (&raw mut gBattle_BG1_Y).cast::<u16>();
                            (__p12).write((((((__p12).read()) as i32).wrapping_sub(2i32)) as u16));
                        }
                    } else {
                        if ((((&raw mut gBattle_BG1_Y).cast::<u16>()).read()) as i32) != 65480i32 {
                            let __p13 = (&raw mut gBattle_BG1_Y).cast::<u16>();
                            (__p13).write((((((__p13).read()) as i32).wrapping_sub(1i32)) as u16));
                        }
                    }
                }
                if (((((&raw mut gBattle_WIN0V).cast::<u16>()).read()) as i32) & 65280i32) != 0 {
                    let __p14 = (&raw mut gBattle_WIN0V).cast::<u16>();
                    (__p14).write((((((__p14).read()) as i32).wrapping_sub(1020i32)) as u16));
                }
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read())
                    != 0
                {
                    let __p15 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    (__p15).write((((((__p15).read()) as i32).wrapping_sub(2i32)) as i16));
                }
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < crate::c::div_i32(160i32, 2i32)) {
                            break 'l2;
                        }
                        'l3: {
                            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(20))
                                        .read()) as i32)
                                        as isize
                                        * 1920,
                                ))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .write(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    'l4: loop {
                        if !(i < 160i32) {
                            break 'l4;
                        }
                        'l5: {
                            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(20))
                                        .read()) as i32)
                                        as isize
                                        * 1920,
                                ))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .write(
                                ((((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as i32)
                                    .wrapping_neg()) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32)
                    == 0i32
                {
                    (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
                    let __p16 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p16).write(((__p16).read()).wrapping_add(1));
                    'l6: loop {
                        'l7: {
                            {
                                let mut tmp: u32 = 0u32;
                                (&raw mut tmp).write_volatile(0u32);
                                'l8: loop {
                                    'l9: {
                                        CpuSet(
                                            (&raw mut tmp).cast::<u8>(),
                                            ((100720640i32) as usize as *mut u8),
                                            ((83886080i32
                                                | (crate::c::div_i32(
                                                    2048i32,
                                                    crate::c::div_i32(32i32, 8i32),
                                                ) & 2097151i32))
                                                as u32),
                                        );
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
                    SetBgAttribute(1u8, 1u8, 0u8);
                    SetBgAttribute(2u8, 1u8, 0u8);
                    SetGpuReg(10u8, 39936u16);
                    SetGpuReg(12u8, 24064u16);
                }
                break 'l1;
            }
            if __sw2 == 4i32 {
                BattleIntroSlideEnd(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BattleIntroSlide2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32);
            if __sw1 == 2i32 || __sw1 == 4i32 {
                let __p2 = (&raw mut gBattle_BG1_X).cast::<u16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as u16));
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p3 = (&raw mut gBattle_BG1_X).cast::<u16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_add(6i32)) as u16));
                break 'l1;
            }
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            == 4i32
        {
            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(
                (((crate::c::div_i32(
                    ((Cos2(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .read()) as u16),
                    )) as i32),
                    512i32,
                ))
                .wrapping_sub(8i32)) as u16),
            );
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as i32)
                < 180i32
            {
                let __p4 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6);
                (__p4).write((((((__p4).read()) as i32).wrapping_add(4i32)) as i16));
            } else {
                let __p5 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6);
                (__p5).write((((((__p5).read()) as i32).wrapping_add(6i32)) as i16));
            }
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as i32)
                == 360i32
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(0i16);
            }
        }
        'l2: {
            let __sw6 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw6 == 0i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(16i16);
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0 {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(16i16);
                    let __p7 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                } else {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(1i16);
                    let __p8 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l2;
            }
            if __sw6 == 1i32 {
                if (({
                    let __p9 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    let __t10 = ((__p9).read()).wrapping_sub(1);
                    (__p9).write(__t10);
                    __t10
                }) as i32)
                    == 0i32
                {
                    let __p11 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p11).write(((__p11).read()).wrapping_add(1));
                    SetGpuReg(72u8, 63u16);
                }
                break 'l2;
            }
            if __sw6 == 2i32 {
                let __p12 = (&raw mut gBattle_WIN0V).cast::<u16>();
                (__p12).write((((((__p12).read()) as i32).wrapping_sub(255i32)) as u16));
                if (((((&raw mut gBattle_WIN0V).cast::<u16>()).read()) as i32) & 65280i32)
                    == 12288i32
                {
                    let __p13 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p13).write(((__p13).read()).wrapping_add(1));
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(240i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(32i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write(1i16);
                    let __p14 = (&raw mut gIntroSlideFlags).cast::<u16>();
                    (__p14).write((((((__p14).read()) as i32) & (-2i32)) as u16));
                }
                break 'l2;
            }
            if __sw6 == 3i32 {
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read())
                    != 0
                {
                    if (({
                        let __p15 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(3);
                        let __t16 = ((__p15).read()).wrapping_sub(1);
                        (__p15).write(__t16);
                        __t16
                    }) as i32)
                        == 0i32
                    {
                        SetGpuReg(80u8, 6210u16);
                        SetGpuReg(82u8, 15u16);
                        SetGpuReg(84u8, 0u16);
                    }
                } else {
                    if ((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as i32)
                        & 31i32)
                        != 0)
                        && ((({
                            let __p17 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(5);
                            let __t18 = ((__p17).read()).wrapping_sub(1);
                            (__p17).write(__t18);
                            __t18
                        }) as i32)
                            == 0i32)
                    {
                        let __p19 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4);
                        (__p19).write((((((__p19).read()) as i32).wrapping_add(255i32)) as i16));
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(4i16);
                    }
                }
                if (((((&raw mut gBattle_WIN0V).cast::<u16>()).read()) as i32) & 65280i32) != 0 {
                    let __p20 = (&raw mut gBattle_WIN0V).cast::<u16>();
                    (__p20).write((((((__p20).read()) as i32).wrapping_sub(1020i32)) as u16));
                }
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read())
                    != 0
                {
                    let __p21 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    (__p21).write((((((__p21).read()) as i32).wrapping_sub(2i32)) as i16));
                }
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i < crate::c::div_i32(160i32, 2i32)) {
                            break 'l3;
                        }
                        'l4: {
                            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(20))
                                        .read()) as i32)
                                        as isize
                                        * 1920,
                                ))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .write(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    'l5: loop {
                        if !(i < 160i32) {
                            break 'l5;
                        }
                        'l6: {
                            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(20))
                                        .read()) as i32)
                                        as isize
                                        * 1920,
                                ))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .write(
                                ((((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as i32)
                                    .wrapping_neg()) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32)
                    == 0i32
                {
                    (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
                    let __p22 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p22).write(((__p22).read()).wrapping_add(1));
                    'l7: loop {
                        'l8: {
                            {
                                let mut tmp: u32 = 0u32;
                                (&raw mut tmp).write_volatile(0u32);
                                'l9: loop {
                                    'l10: {
                                        CpuSet(
                                            (&raw mut tmp).cast::<u8>(),
                                            ((100720640i32) as usize as *mut u8),
                                            ((83886080i32
                                                | (crate::c::div_i32(
                                                    2048i32,
                                                    crate::c::div_i32(32i32, 8i32),
                                                ) & 2097151i32))
                                                as u32),
                                        );
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
                    SetBgAttribute(1u8, 1u8, 0u8);
                    SetBgAttribute(2u8, 1u8, 0u8);
                    SetGpuReg(10u8, 39936u16);
                    SetGpuReg(12u8, 24064u16);
                }
                break 'l2;
            }
            if __sw6 == 4i32 {
                BattleIntroSlideEnd(taskId);
                break 'l2;
            }
        }
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            != 4i32
        {
            SetGpuReg(
                82u8,
                ((0i32
                    | ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn BattleIntroSlide3(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let __p1 = (&raw mut gBattle_BG1_X).cast::<u16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as u16));
        'l1: {
            let __sw2 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw2 == 0i32 {
                SetGpuReg(80u8, 6210u16);
                SetGpuReg(82u8, 2056u16);
                SetGpuReg(84u8, 0u16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(2056i16);
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554434u32) != 0 {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(16i16);
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                } else {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(1i16);
                    let __p4 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw2 == 1i32 {
                if (({
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    let __t6 = ((__p5).read()).wrapping_sub(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    == 0i32
                {
                    let __p7 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    SetGpuReg(72u8, 63u16);
                }
                break 'l1;
            }
            if __sw2 == 2i32 {
                let __p8 = (&raw mut gBattle_WIN0V).cast::<u16>();
                (__p8).write((((((__p8).read()) as i32).wrapping_sub(255i32)) as u16));
                if (((((&raw mut gBattle_WIN0V).cast::<u16>()).read()) as i32) & 65280i32)
                    == 12288i32
                {
                    let __p9 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(240i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(32i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write(1i16);
                    let __p10 = (&raw mut gIntroSlideFlags).cast::<u16>();
                    (__p10).write((((((__p10).read()) as i32) & (-2i32)) as u16));
                }
                break 'l1;
            }
            if __sw2 == 3i32 {
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read())
                    != 0
                {
                    let __p11 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3);
                    (__p11).write(((__p11).read()).wrapping_sub(1));
                } else {
                    if ((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as i32)
                        & 15i32)
                        != 0)
                        && ((({
                            let __p12 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(5);
                            let __t13 = ((__p12).read()).wrapping_sub(1);
                            (__p12).write(__t13);
                            __t13
                        }) as i32)
                            == 0i32)
                    {
                        let __p14 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4);
                        (__p14).write((((((__p14).read()) as i32).wrapping_add(255i32)) as i16));
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(6i16);
                    }
                }
                if (((((&raw mut gBattle_WIN0V).cast::<u16>()).read()) as i32) & 65280i32) != 0 {
                    let __p15 = (&raw mut gBattle_WIN0V).cast::<u16>();
                    (__p15).write((((((__p15).read()) as i32).wrapping_sub(1020i32)) as u16));
                }
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read())
                    != 0
                {
                    let __p16 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    (__p16).write((((((__p16).read()) as i32).wrapping_sub(2i32)) as i16));
                }
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < crate::c::div_i32(160i32, 2i32)) {
                            break 'l2;
                        }
                        'l3: {
                            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(20))
                                        .read()) as i32)
                                        as isize
                                        * 1920,
                                ))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .write(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    'l4: loop {
                        if !(i < 160i32) {
                            break 'l4;
                        }
                        'l5: {
                            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(20))
                                        .read()) as i32)
                                        as isize
                                        * 1920,
                                ))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .write(
                                ((((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as i32)
                                    .wrapping_neg()) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32)
                    == 0i32
                {
                    (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
                    let __p17 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p17).write(((__p17).read()).wrapping_add(1));
                    'l6: loop {
                        'l7: {
                            {
                                let mut tmp: u32 = 0u32;
                                (&raw mut tmp).write_volatile(0u32);
                                'l8: loop {
                                    'l9: {
                                        CpuSet(
                                            (&raw mut tmp).cast::<u8>(),
                                            ((100720640i32) as usize as *mut u8),
                                            ((83886080i32
                                                | (crate::c::div_i32(
                                                    2048i32,
                                                    crate::c::div_i32(32i32, 8i32),
                                                ) & 2097151i32))
                                                as u32),
                                        );
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
                    SetBgAttribute(1u8, 1u8, 0u8);
                    SetBgAttribute(2u8, 1u8, 0u8);
                    SetGpuReg(10u8, 39936u16);
                    SetGpuReg(12u8, 24064u16);
                }
                break 'l1;
            }
            if __sw2 == 4i32 {
                BattleIntroSlideEnd(taskId);
                break 'l1;
            }
        }
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            != 4i32
        {
            SetGpuReg(
                82u8,
                ((0i32
                    | ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn BattleIntroSlideLink(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            > 1i32)
            && (!((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read())
                != 0))
        {
            let mut var0: u16 =
                ((((((&raw mut gBattle_BG1_X).cast::<u16>()).read()) as i32) & 32768i32) as u16);
            if ((var0) != 0) || (((((&raw mut gBattle_BG1_X).cast::<u16>()).read()) as i32) < 80i32)
            {
                let __p1 = (&raw mut gBattle_BG1_X).cast::<u16>();
                (__p1).write((((((__p1).read()) as i32).wrapping_add(3i32)) as u16));
                let __p2 = (&raw mut gBattle_BG2_X).cast::<u16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_sub(3i32)) as u16));
            } else {
                'l1: loop {
                    'l2: {
                        {
                            let mut tmp: u32 = 0u32;
                            (&raw mut tmp).write_volatile(0u32);
                            'l3: loop {
                                'l4: {
                                    CpuSet(
                                        (&raw mut tmp).cast::<u8>(),
                                        ((100720640i32) as usize as *mut u8),
                                        ((83886080i32
                                            | (crate::c::div_i32(
                                                2048i32,
                                                crate::c::div_i32(32i32, 8i32),
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
                'l5: loop {
                    'l6: {
                        {
                            let mut tmp: u32 = 0u32;
                            (&raw mut tmp).write_volatile(0u32);
                            'l7: loop {
                                'l8: {
                                    CpuSet(
                                        (&raw mut tmp).cast::<u8>(),
                                        ((100724736i32) as usize as *mut u8),
                                        ((83886080i32
                                            | (crate::c::div_i32(
                                                2048i32,
                                                crate::c::div_i32(32i32, 8i32),
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
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(1i16);
            }
        }
        'l9: {
            let __sw3 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw3 == 0i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(32i16);
                let __p4 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l9;
            }
            if __sw3 == 1i32 {
                if (({
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    let __t6 = ((__p5).read()).wrapping_sub(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    == 0i32
                {
                    let __p7 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(125))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(1),
                        2,
                        2,
                        (2u32) as i32,
                    );
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(125))
                            .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_VsLetterInit));
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(126))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(1),
                        2,
                        2,
                        (2u32) as i32,
                    );
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(126))
                            .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_VsLetterInit));
                    SetGpuReg(72u8, 63u16);
                    SetGpuReg(74u8, 16134u16);
                }
                break 'l9;
            }
            if __sw3 == 2i32 {
                let __p8 = (&raw mut gBattle_WIN0V).cast::<u16>();
                (__p8).write((((((__p8).read()) as i32).wrapping_sub(255i32)) as u16));
                if (((((&raw mut gBattle_WIN0V).cast::<u16>()).read()) as i32) & 65280i32)
                    == 12288i32
                {
                    let __p9 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(240i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(32i16);
                    let __p10 = (&raw mut gIntroSlideFlags).cast::<u16>();
                    (__p10).write((((((__p10).read()) as i32) & (-2i32)) as u16));
                }
                break 'l9;
            }
            if __sw3 == 3i32 {
                if (((((&raw mut gBattle_WIN0V).cast::<u16>()).read()) as i32) & 65280i32) != 0 {
                    let __p11 = (&raw mut gBattle_WIN0V).cast::<u16>();
                    (__p11).write((((((__p11).read()) as i32).wrapping_sub(1020i32)) as u16));
                }
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read())
                    != 0
                {
                    let __p12 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    (__p12).write((((((__p12).read()) as i32).wrapping_sub(2i32)) as i16));
                }
                {
                    i = 0i32;
                    'l10: loop {
                        if !(i < crate::c::div_i32(160i32, 2i32)) {
                            break 'l10;
                        }
                        'l11: {
                            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(20))
                                        .read()) as i32)
                                        as isize
                                        * 1920,
                                ))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .write(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    'l12: loop {
                        if !(i < 160i32) {
                            break 'l12;
                        }
                        'l13: {
                            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(20))
                                        .read()) as i32)
                                        as isize
                                        * 1920,
                                ))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .write(
                                ((((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as i32)
                                    .wrapping_neg()) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32)
                    == 0i32
                {
                    (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
                    let __p13 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p13).write(((__p13).read()).wrapping_add(1));
                    SetBgAttribute(1u8, 1u8, 0u8);
                    SetBgAttribute(2u8, 1u8, 0u8);
                    SetGpuReg(10u8, 39936u16);
                    SetGpuReg(12u8, 24064u16);
                }
                break 'l9;
            }
            if __sw3 == 4i32 {
                BattleIntroSlideEnd(taskId);
                break 'l9;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BattleIntroSlidePartner(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(1i16);
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    let __t4 = ((__p3).read()).wrapping_sub(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    == 0i32
                {
                    let __p5 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    SetGpuReg(10u8, 23562u16);
                    SetGpuReg(12u8, 24074u16);
                    SetGpuReg(
                        0u8,
                        ((((((((GetGpuReg(0u8)) as i32) | 64i32) | 4096i32) | 8192i32) | 16384i32)
                            | 32768i32) as u16),
                    );
                    SetGpuReg(72u8, 15872u16);
                    SetGpuReg(74u8, 16191u16);
                    ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(65488u16);
                    ((&raw mut gBattle_BG1_X).cast::<u16>()).write(240u16);
                    ((&raw mut gBattle_BG2_X).cast::<u16>()).write(65296u16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p6 = (&raw mut gBattle_WIN0V).cast::<u16>();
                (__p6).write((((((__p6).read()) as i32).wrapping_add(256i32)) as u16));
                if (((((&raw mut gBattle_WIN0V).cast::<u16>()).read()) as i32) & 65280i32) != 256i32
                {
                    let __p7 = (&raw mut gBattle_WIN0V).cast::<u16>();
                    (__p7).write(((__p7).read()).wrapping_sub(1));
                }
                if (((((&raw mut gBattle_WIN0V).cast::<u16>()).read()) as i32) & 65280i32)
                    == 8192i32
                {
                    let __p8 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(240i16);
                    let __p9 = (&raw mut gIntroSlideFlags).cast::<u16>();
                    (__p9).write((((((__p9).read()) as i32) & (-2i32)) as u16));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (((((&raw mut gBattle_WIN0V).cast::<u16>()).read()) as i32) & 65280i32)
                    != 19456i32
                {
                    let __p10 = (&raw mut gBattle_WIN0V).cast::<u16>();
                    (__p10).write((((((__p10).read()) as i32).wrapping_add(1020i32)) as u16));
                }
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read())
                    != 0
                {
                    let __p11 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    (__p11).write((((((__p11).read()) as i32).wrapping_sub(2i32)) as i16));
                }
                ((&raw mut gBattle_BG1_X).cast::<u16>()).write(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as u16),
                );
                ((&raw mut gBattle_BG2_X).cast::<u16>()).write(
                    ((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        .wrapping_neg()) as u16),
                );
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32)
                    == 0i32
                {
                    let __p12 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                let __p13 = (&raw mut gBattle_BG0_Y).cast::<u16>();
                (__p13).write((((((__p13).read()) as i32).wrapping_add(2i32)) as u16));
                let __p14 = (&raw mut gBattle_BG2_Y).cast::<u16>();
                (__p14).write((((((__p14).read()) as i32).wrapping_add(2i32)) as u16));
                if (((((&raw mut gBattle_WIN0V).cast::<u16>()).read()) as i32) & 65280i32)
                    != 20480i32
                {
                    let __p15 = (&raw mut gBattle_WIN0V).cast::<u16>();
                    (__p15).write((((((__p15).read()) as i32).wrapping_add(255i32)) as u16));
                }
                if !((((&raw mut gBattle_BG0_Y).cast::<u16>()).read()) != 0) {
                    'l2: loop {
                        'l3: {
                            {
                                let mut tmp: u32 = 0u32;
                                (&raw mut tmp).write_volatile(0u32);
                                'l4: loop {
                                    'l5: {
                                        CpuSet(
                                            (&raw mut tmp).cast::<u8>(),
                                            ((100720640i32) as usize as *mut u8),
                                            ((83886080i32
                                                | (crate::c::div_i32(
                                                    8192i32,
                                                    crate::c::div_i32(32i32, 8i32),
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
                    SetGpuReg(0u8, ((((GetGpuReg(0u8)) as i32) & (-16385i32)) as u16));
                    SetBgAttribute(1u8, 1u8, 0u8);
                    SetBgAttribute(2u8, 1u8, 0u8);
                    SetGpuReg(10u8, 39936u16);
                    SetGpuReg(12u8, 24064u16);
                    (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
                    let __p16 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p16).write(((__p16).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                BattleIntroSlideEnd(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawBattlerOnBg(
    bgId: i32,
    x: u8,
    y: u8,
    battlerPosition: u8,
    paletteId: u8,
    tiles: *mut u8,
    tilemap: *mut u16,
    tilesOffset: u16,
) {
    unsafe {
        let mut bgId = bgId;
        let mut x = x;
        let mut y = y;
        let mut battlerPosition = battlerPosition;
        let mut paletteId = paletteId;
        let mut tiles = tiles;
        let mut tilemap = tilemap;
        let mut tilesOffset = tilesOffset;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut battler: u8 = GetBattlerAtPosition(battlerPosition);
        let mut offset: i32 = ((tilesOffset) as i32);
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .cast::<*mut u8>())
                            .wrapping_offset(((battlerPosition) as i32) as isize))
                            .read())
                            .wrapping_offset(
                                ((2048i32).wrapping_mul(
                                    (((((&raw mut gBattleMonForms).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize))
                                    .read()) as i32),
                                )) as isize
                                    * 1,
                            ),
                            tiles,
                            ((0i32
                                | (crate::c::div_i32(2048i32, crate::c::div_i32(16i32, 8i32))
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
        LoadBgTiles(((bgId) as u8), tiles, 4096u16, tilesOffset);
        {
            i = ((y) as i32);
            'l5: loop {
                if !(i < ((y) as i32).wrapping_add(8i32)) {
                    break 'l5;
                }
                'l6: {
                    {
                        j = ((x) as i32);
                        'l7: loop {
                            if !(j < ((x) as i32).wrapping_add(8i32)) {
                                break 'l7;
                            }
                            'l8: {
                                ((tilemap).wrapping_offset(
                                    (((i).wrapping_mul(32i32)).wrapping_add(j)) as isize,
                                ))
                                .write(((offset | (((paletteId) as i32) << 12)) as u16));
                                offset = (offset).wrapping_add(1);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        LoadBgTilemap(((bgId) as u8), (tilemap).cast::<u8>(), 2048u16, 0u16);
    }
}
pub(crate) unsafe extern "C" fn DrawBattlerOnBgDMA(
    x: u8,
    y: u8,
    battlerPosition: u8,
    arg3: u8,
    paletteId: u8,
    arg5: u16,
    arg6: u8,
    arg7: u8,
) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut battlerPosition = battlerPosition;
        let mut arg3 = arg3;
        let mut paletteId = paletteId;
        let mut arg5 = arg5;
        let mut arg6 = arg6;
        let mut arg7 = arg7;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut offset: i32 = 0i32;
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        {
                            let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                            crate::c::volatile_write(
                                dmaRegs,
                                (((((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                                    .wrapping_add(4))
                                .cast::<*mut u8>())
                                .wrapping_offset(((battlerPosition) as i32) as isize))
                                .read())
                                .wrapping_offset(
                                    ((2048i32).wrapping_mul(((arg3) as i32))) as isize * 1,
                                )) as usize as u32),
                            );
                            crate::c::volatile_write(
                                (dmaRegs).wrapping_offset(1),
                                ((((100663296i32) as usize as *mut u8)
                                    .wrapping_offset(((arg5) as i32) as isize * 1))
                                    as usize as u32),
                            );
                            crate::c::volatile_write(
                                (dmaRegs).wrapping_offset(2),
                                (((-2147483648i32)
                                    | crate::c::div_i32(2048i32, crate::c::div_i32(16i32, 8i32)))
                                    as u32),
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
        offset = (((arg5) as i32) >> 5).wrapping_sub((((arg7) as i32) << 9));
        {
            i = ((y) as i32);
            'l5: loop {
                if !(i < ((y) as i32).wrapping_add(8i32)) {
                    break 'l5;
                }
                'l6: {
                    {
                        j = ((x) as i32);
                        'l7: loop {
                            if !(j < ((x) as i32).wrapping_add(8i32)) {
                                break 'l7;
                            }
                            'l8: {
                                ((((100663296i32) as usize as *mut u16)
                                    .wrapping_offset(((i).wrapping_mul(32i32)) as isize))
                                .wrapping_offset(
                                    ((j).wrapping_add((((arg6) as i32) << 10))) as isize,
                                ))
                                .write(((offset | (((paletteId) as i32) << 12)) as u16));
                                offset = (offset).wrapping_add(1);
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
