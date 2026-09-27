//! Translated from `src/braille_puzzles.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sRegicePathCoords
#[allow(unused_imports)]
use crate::data::braille_puzzles::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sIsRegisteelPuzzle: u8 = 0u8;

unsafe extern "C" {
    static mut gFieldEffectArguments: u8;
    static mut gPlayerParty: u8;
    static mut gPlayerPartyCount: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gTasks: u8;
    fn CalculatePlayerPartyCount() -> u8;
    fn CreateFieldMoveTask() -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn DrawWholeMapView();
    fn FieldEffectActiveListRemove(a0: u8);
    fn FieldEffectStart(a0: u8) -> u32;
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn GetCursorSelectionMonId() -> u8;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn InstallCameraPanAheadCallback();
    fn MapGridSetMetatileIdAt(a0: i32, a1: i32, a2: u16);
    fn PlaySE(a0: u16);
    fn ScriptContext_Enable();
    fn SetCameraPanning(a0: i16, a1: i16);
    fn SetCameraPanningCallback(a0: Option<unsafe extern "C" fn()>);
    fn UnlockPlayerFieldControls();
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoBrailleDigEffect() -> u8 {
    unsafe {
        if (!((FlagGet(2223u16)) != 0))
            && (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as i32)
                == 24i32)
                && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as i32)
                    == 71i32))
        {
            if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as i32)
                == 10i32)
                && (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32)
                    == 3i32)
            {
                return 1u8;
            }
            if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as i32)
                == 9i32)
                && (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32)
                    == 3i32)
            {
                return 1u8;
            }
            if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as i32)
                == 11i32)
                && (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32)
                    == 3i32)
            {
                return 1u8;
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoBrailleDigEffect() {
    unsafe {
        MapGridSetMetatileIdAt(16i32, 8i32, 554u16);
        MapGridSetMetatileIdAt(17i32, 8i32, 555u16);
        MapGridSetMetatileIdAt(18i32, 8i32, 556u16);
        MapGridSetMetatileIdAt(16i32, 9i32, 3634u16);
        MapGridSetMetatileIdAt(17i32, 9i32, 563u16);
        MapGridSetMetatileIdAt(18i32, 9i32, 3636u16);
        DrawWholeMapView();
        PlaySE(20u16);
        FlagSet(2223u16);
        UnlockPlayerFieldControls();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckRelicanthWailord() -> u8 {
    unsafe {
        if GetMonData3(
            (&raw mut gPlayerParty).cast::<u8>(),
            65i32,
            core::ptr::null_mut(),
        ) == 314u32
        {
            CalculatePlayerPartyCount();
            if GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32)
                        .wrapping_sub(1i32)) as isize
                        * 100,
                ),
                65i32,
                core::ptr::null_mut(),
            ) == 381u32
            {
                return 1u8;
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoBrailleRegirockEffectOld() {
    unsafe {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoSealedChamberShakingEffect_Long() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_SealedChamberShakingEffect), 9u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(2i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(5i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(50i16);
        SetCameraPanningCallback(None);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoSealedChamberShakingEffect_Short() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_SealedChamberShakingEffect), 9u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(3i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(5i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(2i16);
        SetCameraPanningCallback(None);
    }
}
pub(crate) unsafe extern "C" fn Task_SealedChamberShakingEffect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if crate::c::rem_i32(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
        ) == 0i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(((__p2).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
                ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                    .wrapping_neg()) as i16),
            );
            SetCameraPanning(
                0i16,
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
            );
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                == ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
            {
                DestroyTask(taskId);
                ScriptContext_Enable();
                InstallCameraPanAheadCallback();
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoBrailleRegirockEffect() -> u8 {
    unsafe {
        if ((!((FlagGet(2224u16)) != 0))
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as i32)
                == 24i32))
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == 6i32)
        {
            if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as i32)
                == 6i32)
                && (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32)
                    == 23i32)
            {
                ((&raw mut sIsRegisteelPuzzle).cast::<u8>().cast::<u8>()).write(0u8);
                return 1u8;
            } else {
                if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                    as i32)
                    == 5i32)
                    && (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(2)
                        .cast::<i16>())
                    .read()) as i32)
                        == 23i32)
                {
                    ((&raw mut sIsRegisteelPuzzle).cast::<u8>().cast::<u8>()).write(0u8);
                    return 1u8;
                } else {
                    if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>())
                        .read()) as i32)
                        == 7i32)
                        && (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(2)
                            .cast::<i16>())
                        .read()) as i32)
                            == 23i32)
                    {
                        ((&raw mut sIsRegisteelPuzzle).cast::<u8>().cast::<u8>()).write(0u8);
                        return 1u8;
                    }
                }
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpPuzzleEffectRegirock() {
    unsafe {
        (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
            .write(((GetCursorSelectionMonId()) as i32));
        FieldEffectStart(60u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UseRegirockHm_Callback() {
    unsafe {
        FieldEffectActiveListRemove(60u8);
        DoBrailleRegirockEffect();
    }
}
pub(crate) unsafe extern "C" fn DoBrailleRegirockEffect() {
    unsafe {
        MapGridSetMetatileIdAt(14i32, 26i32, 554u16);
        MapGridSetMetatileIdAt(15i32, 26i32, 555u16);
        MapGridSetMetatileIdAt(16i32, 26i32, 556u16);
        MapGridSetMetatileIdAt(14i32, 27i32, 3634u16);
        MapGridSetMetatileIdAt(15i32, 27i32, 563u16);
        MapGridSetMetatileIdAt(16i32, 27i32, 3636u16);
        DrawWholeMapView();
        PlaySE(20u16);
        FlagSet(2224u16);
        UnlockPlayerFieldControls();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoBrailleRegisteelEffect() -> u8 {
    unsafe {
        if (!((FlagGet(2226u16)) != 0))
            && (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as i32)
                == 24i32)
                && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as i32)
                    == 68i32))
        {
            if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as i32)
                == 8i32)
                && (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32)
                    == 25i32)
            {
                ((&raw mut sIsRegisteelPuzzle).cast::<u8>().cast::<u8>()).write(1u8);
                return 1u8;
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpPuzzleEffectRegisteel() {
    unsafe {
        (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
            .write(((GetCursorSelectionMonId()) as i32));
        FieldEffectStart(60u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UseRegisteelHm_Callback() {
    unsafe {
        FieldEffectActiveListRemove(60u8);
        DoBrailleRegisteelEffect();
    }
}
pub(crate) unsafe extern "C" fn DoBrailleRegisteelEffect() {
    unsafe {
        MapGridSetMetatileIdAt(14i32, 26i32, 554u16);
        MapGridSetMetatileIdAt(15i32, 26i32, 555u16);
        MapGridSetMetatileIdAt(16i32, 26i32, 556u16);
        MapGridSetMetatileIdAt(14i32, 27i32, 3634u16);
        MapGridSetMetatileIdAt(15i32, 27i32, 563u16);
        MapGridSetMetatileIdAt(16i32, 27i32, 3636u16);
        DrawWholeMapView();
        PlaySE(20u16);
        FlagSet(2226u16);
        UnlockPlayerFieldControls();
    }
}
pub(crate) unsafe extern "C" fn DoBrailleWait() {
    unsafe {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UsePuzzleEffect() -> u8 {
    unsafe {
        let mut taskId: u8 = CreateFieldMoveTask();
        if ((((&raw mut sIsRegisteelPuzzle).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8))
            .write((((UseRegisteelHm_Callback as *const () as usize as u32) >> 16) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(9))
            .write(((UseRegisteelHm_Callback as *const () as usize as u32) as i16));
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8))
            .write((((UseRegirockHm_Callback as *const () as usize as u32) >> 16) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(9))
            .write(((UseRegirockHm_Callback as *const () as usize as u32) as i16));
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoBrailleRegicePuzzle() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            == 24i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == 67i32)
        {
            if (FlagGet(2225u16)) != 0 {
                return 0u8;
            }
            if ((FlagGet(2u16)) as i32) == 0i32 {
                return 0u8;
            }
            if ((FlagGet(3u16)) as i32) == 1i32 {
                return 0u8;
            }
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as u32) < crate::c::div_u32(72u32, 2u32)) {
                        break 'l1;
                    }
                    'l2: {
                        let mut xPos: u8 =
                            (((((&raw const sRegicePathCoords).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 2))
                            .cast::<u8>())
                            .read();
                        let mut yPos: u8 =
                            ((((((&raw const sRegicePathCoords).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 2))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read();
                        if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .cast::<i16>())
                        .read()) as i32)
                            == ((xPos) as i32))
                            && (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(2)
                                .cast::<i16>())
                            .read()) as i32)
                                == ((yPos) as i32))
                        {
                            if ((i) as i32) < 16i32 {
                                let mut val: u16 = VarGet(16443u16);
                                val = ((((val) as i32) | crate::c::shl_i32(1i32, ((i) as u32)))
                                    as u16);
                                VarSet(16443u16, val);
                            } else {
                                if ((i) as i32) < 32i32 {
                                    let mut val: u16 = VarGet(16444u16);
                                    val = ((((val) as i32)
                                        | crate::c::shl_i32(
                                            1i32,
                                            ((((i) as i32).wrapping_sub(16i32)) as u32),
                                        )) as u16);
                                    VarSet(16444u16, val);
                                } else {
                                    let mut val: u16 = VarGet(16445u16);
                                    val = ((((val) as i32)
                                        | crate::c::shl_i32(
                                            1i32,
                                            ((((i) as i32).wrapping_sub(32i32)) as u32),
                                        )) as u16);
                                    VarSet(16445u16, val);
                                }
                            }
                            if ((((VarGet(16443u16)) as i32) != 65535i32)
                                || (((VarGet(16444u16)) as i32) != 65535i32))
                                || (((VarGet(16445u16)) as i32) != 15i32)
                            {
                                return 0u8;
                            }
                            if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .cast::<i16>())
                            .read()) as i32)
                                == 8i32)
                                && (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(2)
                                    .cast::<i16>())
                                .read()) as i32)
                                    == 21i32)
                            {
                                return 1u8;
                            } else {
                                return 0u8;
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            FlagSet(3u16);
            FlagClear(2u16);
        }
        return 0u8;
    }
}
