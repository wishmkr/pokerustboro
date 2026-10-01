//! Translated from `src/braille_puzzles.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
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
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): sRegicePathCoords

static sRegicePathCoords: Table<CArray<CArray<u8, 2>, 36>> =
    Table((&raw const crate::data::braille_puzzles::sRegicePathCoords).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sIsRegisteelPuzzle: u8 = 0;

unsafe extern "C" {
    static mut gFieldEffectArguments: CArray<i32, 8>;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlayerPartyCount: u8;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gTasks: CArray<Task, 0>;
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
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
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
    if FlagGet(FLAG_SYS_BRAILLE_DIG) == 0
        && ((*gSaveBlock1Ptr).location.mapGroup == 24 && (*gSaveBlock1Ptr).location.mapNum == 71)
    {
        if (*gSaveBlock1Ptr).pos.x == 10 && (*gSaveBlock1Ptr).pos.y == 3 {
            return TRUE;
        }
        if (*gSaveBlock1Ptr).pos.x == 9 && (*gSaveBlock1Ptr).pos.y == 3 {
            return TRUE;
        }
        if (*gSaveBlock1Ptr).pos.x == 11 && (*gSaveBlock1Ptr).pos.y == 3 {
            return TRUE;
        }
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoBrailleDigEffect() {
    MapGridSetMetatileIdAt(16, 8, METATILE_Cave_SealedChamberEntrance_TopLeft);
    MapGridSetMetatileIdAt(17, 8, METATILE_Cave_SealedChamberEntrance_TopMid);
    MapGridSetMetatileIdAt(18, 8, METATILE_Cave_SealedChamberEntrance_TopRight);
    MapGridSetMetatileIdAt(16, 9, 3634);
    MapGridSetMetatileIdAt(17, 9, METATILE_Cave_SealedChamberEntrance_BottomMid);
    MapGridSetMetatileIdAt(18, 9, 3636);
    DrawWholeMapView();
    PlaySE(SE_BANG);
    FlagSet(FLAG_SYS_BRAILLE_DIG);
    UnlockPlayerFieldControls();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckRelicanthWailord() -> u8 {
    if GetMonData3(
        &raw mut gPlayerParty[0],
        MON_DATA_SPECIES_OR_EGG,
        null_mut(),
    ) == SPECIES_WAILORD
    {
        CalculatePlayerPartyCount();
        if GetMonData3(
            &raw mut gPlayerParty[gPlayerPartyCount as i32 - 1],
            MON_DATA_SPECIES_OR_EGG,
            null_mut(),
        ) == SPECIES_RELICANTH
        {
            return TRUE;
        }
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoBrailleRegirockEffectOld() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoSealedChamberShakingEffect_Long() {
    let mut taskId: u8 = CreateTask(Some(Task_SealedChamberShakingEffect), 9);
    gTasks[taskId].data[1] = 0;
    gTasks[taskId].data[2] = 0;
    gTasks[taskId].data[4] = 2;
    gTasks[taskId].data[5] = 5;
    gTasks[taskId].data[6] = 50;
    SetCameraPanningCallback(None);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoSealedChamberShakingEffect_Short() {
    let mut taskId: u8 = CreateTask(Some(Task_SealedChamberShakingEffect), 9);
    gTasks[taskId].data[1] = 0;
    gTasks[taskId].data[2] = 0;
    gTasks[taskId].data[4] = 3;
    gTasks[taskId].data[5] = 5;
    gTasks[taskId].data[6] = 2;
    SetCameraPanningCallback(None);
}
pub(crate) unsafe extern "C" fn Task_SealedChamberShakingEffect(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    (*task).data[1] += 1;
    if rem_i32((*task).data[1] as i32, (*task).data[5] as i32) == 0 {
        (*task).data[1] = 0;
        (*task).data[2] += 1;
        (*task).data[4] = -(*task).data[4];
        SetCameraPanning(0, (*task).data[4]);
        if (*task).data[2] == (*task).data[6] {
            DestroyTask(taskId);
            ScriptContext_Enable();
            InstallCameraPanAheadCallback();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoBrailleRegirockEffect() -> u8 {
    if FlagGet(FLAG_SYS_REGIROCK_PUZZLE_COMPLETED) == 0
        && (*gSaveBlock1Ptr).location.mapGroup == 24
        && (*gSaveBlock1Ptr).location.mapNum == 6
    {
        if (*gSaveBlock1Ptr).pos.x == 6 && (*gSaveBlock1Ptr).pos.y == 23 {
            sIsRegisteelPuzzle = FALSE;
            return TRUE;
        } else if (*gSaveBlock1Ptr).pos.x == 5 && (*gSaveBlock1Ptr).pos.y == 23 {
            sIsRegisteelPuzzle = FALSE;
            return TRUE;
        } else if (*gSaveBlock1Ptr).pos.x == 7 && (*gSaveBlock1Ptr).pos.y == 23 {
            sIsRegisteelPuzzle = FALSE;
            return TRUE;
        }
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpPuzzleEffectRegirock() {
    gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32;
    FieldEffectStart(FLDEFF_USE_TOMB_PUZZLE_EFFECT);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UseRegirockHm_Callback() {
    FieldEffectActiveListRemove(FLDEFF_USE_TOMB_PUZZLE_EFFECT);
    DoBrailleRegirockEffect();
}
pub(crate) unsafe extern "C" fn DoBrailleRegirockEffect() {
    MapGridSetMetatileIdAt(14, 26, METATILE_Cave_SealedChamberEntrance_TopLeft);
    MapGridSetMetatileIdAt(15, 26, METATILE_Cave_SealedChamberEntrance_TopMid);
    MapGridSetMetatileIdAt(16, 26, METATILE_Cave_SealedChamberEntrance_TopRight);
    MapGridSetMetatileIdAt(14, 27, 3634);
    MapGridSetMetatileIdAt(15, 27, METATILE_Cave_SealedChamberEntrance_BottomMid);
    MapGridSetMetatileIdAt(16, 27, 3636);
    DrawWholeMapView();
    PlaySE(SE_BANG);
    FlagSet(FLAG_SYS_REGIROCK_PUZZLE_COMPLETED);
    UnlockPlayerFieldControls();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoBrailleRegisteelEffect() -> u8 {
    if FlagGet(FLAG_SYS_REGISTEEL_PUZZLE_COMPLETED) == 0
        && ((*gSaveBlock1Ptr).location.mapGroup == 24 && (*gSaveBlock1Ptr).location.mapNum == 68)
    {
        if (*gSaveBlock1Ptr).pos.x == 8 && (*gSaveBlock1Ptr).pos.y == 25 {
            sIsRegisteelPuzzle = TRUE;
            return TRUE;
        }
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpPuzzleEffectRegisteel() {
    gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32;
    FieldEffectStart(FLDEFF_USE_TOMB_PUZZLE_EFFECT);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UseRegisteelHm_Callback() {
    FieldEffectActiveListRemove(FLDEFF_USE_TOMB_PUZZLE_EFFECT);
    DoBrailleRegisteelEffect();
}
pub(crate) unsafe extern "C" fn DoBrailleRegisteelEffect() {
    MapGridSetMetatileIdAt(14, 26, METATILE_Cave_SealedChamberEntrance_TopLeft);
    MapGridSetMetatileIdAt(15, 26, METATILE_Cave_SealedChamberEntrance_TopMid);
    MapGridSetMetatileIdAt(16, 26, METATILE_Cave_SealedChamberEntrance_TopRight);
    MapGridSetMetatileIdAt(14, 27, 3634);
    MapGridSetMetatileIdAt(15, 27, METATILE_Cave_SealedChamberEntrance_BottomMid);
    MapGridSetMetatileIdAt(16, 27, 3636);
    DrawWholeMapView();
    PlaySE(SE_BANG);
    FlagSet(FLAG_SYS_REGISTEEL_PUZZLE_COMPLETED);
    UnlockPlayerFieldControls();
}
pub(crate) unsafe extern "C" fn DoBrailleWait() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UsePuzzleEffect() -> u8 {
    let mut taskId: u8 = CreateFieldMoveTask();
    if sIsRegisteelPuzzle == TRUE {
        gTasks[taskId].data[8] =
            (UseRegisteelHm_Callback as *const () as usize as u32 >> 16) as i16;
        gTasks[taskId].data[9] = UseRegisteelHm_Callback as *const () as usize as u32 as i16;
    } else {
        gTasks[taskId].data[8] = (UseRegirockHm_Callback as *const () as usize as u32 >> 16) as i16;
        gTasks[taskId].data[9] = UseRegirockHm_Callback as *const () as usize as u32 as i16;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoBrailleRegicePuzzle() -> u8 {
    let mut i: u8 = 0;
    if (*gSaveBlock1Ptr).location.mapGroup == 24 && (*gSaveBlock1Ptr).location.mapNum == 67 {
        if FlagGet(FLAG_SYS_BRAILLE_REGICE_COMPLETED) != 0 {
            return FALSE;
        }
        if FlagGet(FLAG_TEMP_REGICE_PUZZLE_STARTED) == FALSE {
            return FALSE;
        }
        if FlagGet(FLAG_TEMP_REGICE_PUZZLE_FAILED) == TRUE {
            return FALSE;
        }
        i = 0;
        while i < 36 {
            let mut xPos: u8 = sRegicePathCoords[i][0];
            let mut yPos: u8 = sRegicePathCoords[i][1];
            if (*gSaveBlock1Ptr).pos.x == xPos as i16 && (*gSaveBlock1Ptr).pos.y == yPos as i16 {
                if i < 16 {
                    let mut val: u16 = VarGet(VAR_REGICE_STEPS_1);
                    val |= shl_i32(1, i as u32) as u16;
                    VarSet(VAR_REGICE_STEPS_1, val);
                } else if i < 32 {
                    let mut val: u16 = VarGet(VAR_REGICE_STEPS_2);
                    val |= shl_i32(1, i as u32 - 16) as u16;
                    VarSet(VAR_REGICE_STEPS_2, val);
                } else {
                    let mut val: u16 = VarGet(VAR_REGICE_STEPS_3);
                    val |= shl_i32(1, i as u32 - 32) as u16;
                    VarSet(VAR_REGICE_STEPS_3, val);
                }
                if VarGet(VAR_REGICE_STEPS_1) != 0xFFFF
                    || VarGet(VAR_REGICE_STEPS_2) != 0xFFFF
                    || VarGet(VAR_REGICE_STEPS_3) != 0xF
                {
                    return FALSE;
                }
                if (*gSaveBlock1Ptr).pos.x == 8 && (*gSaveBlock1Ptr).pos.y == 21 {
                    return TRUE;
                } else {
                    return FALSE;
                }
            }
            i += 1;
        }
        FlagSet(FLAG_TEMP_REGICE_PUZZLE_FAILED);
        FlagClear(FLAG_TEMP_REGICE_PUZZLE_STARTED);
    }
    return FALSE;
}
