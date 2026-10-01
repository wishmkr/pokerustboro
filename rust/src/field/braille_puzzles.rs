//! Translated from `src/braille_puzzles.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs,
    overflowing_literals,
    clippy::if_same_then_else,
    clippy::missing_transmute_annotations,
    dead_code
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagClear, FlagGet, FlagSet, VarGet, VarSet};
use crate::field_camera::{
    DrawWholeMapView, InstallCameraPanAheadCallback, SetCameraPanning, SetCameraPanningCallback,
};
use crate::field_effect::{FieldEffectActiveListRemove, FieldEffectStart, gFieldEffectArguments};
use crate::fieldmap::MapGridSetMetatileIdAt;
use crate::fldeff_rocksmash::CreateFieldMoveTask;
use crate::load_save::gSaveBlock1Ptr;
use crate::party_menu::GetCursorSelectionMonId;
use crate::pokemon::{CalculatePlayerPartyCount, GetMonData3, gPlayerParty, gPlayerPartyCount};
use crate::script::{ScriptContext_Enable, UnlockPlayerFieldControls};
use crate::sound::PlaySE;
use crate::task::DestroyTask;
use crate::task::gTasks;
use crate::task::task_set;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
// The C's names for task and sprite data slots.
const tDelayCounter: usize = 1;
const tShakeCounter: usize = 2;
const tVerticalPan: usize = 4;
const tDelay: usize = 5;
const tNumShakes: usize = 6;
// Data tables (translate with cdata.py): sRegicePathCoords

static sRegicePathCoords: Table<CArray<CArray<u8, 2>, 36>> =
    Table((&raw const crate::data::braille_puzzles::sRegicePathCoords).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static sIsRegisteelPuzzle: crate::global::Global<u8> = crate::global::Global::new(0);

#[unsafe(no_mangle)]
pub unsafe fn ShouldDoBrailleDigEffect() -> u8 {
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
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn DoBrailleDigEffect() {
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
pub unsafe fn CheckRelicanthWailord() -> u8 {
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
    FALSE
}
#[unsafe(no_mangle)]
pub fn ShouldDoBrailleRegirockEffectOld() {}
#[unsafe(no_mangle)]
pub unsafe fn DoSealedChamberShakingEffect_Long() {
    let taskId: u8 = CreateTask(Some(Task_SealedChamberShakingEffect), 9);
    task_set(taskId, tDelayCounter, 0);
    task_set(taskId, tShakeCounter, 0);
    task_set(taskId, tVerticalPan, 2);
    task_set(taskId, tDelay, 5);
    task_set(taskId, tNumShakes, 50);
    SetCameraPanningCallback(None);
}
#[unsafe(no_mangle)]
pub unsafe fn DoSealedChamberShakingEffect_Short() {
    let taskId: u8 = CreateTask(Some(Task_SealedChamberShakingEffect), 9);
    task_set(taskId, tDelayCounter, 0);
    task_set(taskId, tShakeCounter, 0);
    task_set(taskId, tVerticalPan, 3);
    task_set(taskId, tDelay, 5);
    task_set(taskId, tNumShakes, 2);
    SetCameraPanningCallback(None);
}
pub(crate) unsafe fn Task_SealedChamberShakingEffect(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[tDelayCounter] += 1;
    if rem_i32(
        (*task).data[tDelayCounter] as i32,
        (*task).data[tDelay] as i32,
    ) == 0
    {
        (*task).data[tDelayCounter] = 0;
        (*task).data[tShakeCounter] += 1;
        (*task).data[tVerticalPan] = -(*task).data[tVerticalPan];
        SetCameraPanning(0, (*task).data[tVerticalPan]);
        if (*task).data[tShakeCounter] == (*task).data[tNumShakes] {
            DestroyTask(taskId);
            ScriptContext_Enable();
            InstallCameraPanAheadCallback();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ShouldDoBrailleRegirockEffect() -> u8 {
    if FlagGet(FLAG_SYS_REGIROCK_PUZZLE_COMPLETED) == 0
        && (*gSaveBlock1Ptr).location.mapGroup == 24
        && (*gSaveBlock1Ptr).location.mapNum == 6
    {
        if (*gSaveBlock1Ptr).pos.x == 6 && (*gSaveBlock1Ptr).pos.y == 23 {
            sIsRegisteelPuzzle.set(FALSE);
            return TRUE;
        } else if (*gSaveBlock1Ptr).pos.x == 5 && (*gSaveBlock1Ptr).pos.y == 23 {
            sIsRegisteelPuzzle.set(FALSE);
            return TRUE;
        } else if (*gSaveBlock1Ptr).pos.x == 7 && (*gSaveBlock1Ptr).pos.y == 23 {
            sIsRegisteelPuzzle.set(FALSE);
            return TRUE;
        }
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn SetUpPuzzleEffectRegirock() {
    gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32;
    FieldEffectStart(FLDEFF_USE_TOMB_PUZZLE_EFFECT);
}
pub unsafe fn UseRegirockHm_Callback() {
    FieldEffectActiveListRemove(FLDEFF_USE_TOMB_PUZZLE_EFFECT);
    DoBrailleRegirockEffect();
}
unsafe fn DoBrailleRegirockEffect() {
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
pub unsafe fn ShouldDoBrailleRegisteelEffect() -> u8 {
    if FlagGet(FLAG_SYS_REGISTEEL_PUZZLE_COMPLETED) == 0
        && ((*gSaveBlock1Ptr).location.mapGroup == 24 && (*gSaveBlock1Ptr).location.mapNum == 68)
        && (*gSaveBlock1Ptr).pos.x == 8
        && (*gSaveBlock1Ptr).pos.y == 25
    {
        sIsRegisteelPuzzle.set(TRUE);
        return TRUE;
    }
    FALSE
}
pub unsafe fn SetUpPuzzleEffectRegisteel() {
    gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32;
    FieldEffectStart(FLDEFF_USE_TOMB_PUZZLE_EFFECT);
}
pub unsafe fn UseRegisteelHm_Callback() {
    FieldEffectActiveListRemove(FLDEFF_USE_TOMB_PUZZLE_EFFECT);
    DoBrailleRegisteelEffect();
}
unsafe fn DoBrailleRegisteelEffect() {
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
fn DoBrailleWait() {}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_UsePuzzleEffect() -> u8 {
    let taskId: u8 = CreateFieldMoveTask();
    if sIsRegisteelPuzzle.get() == TRUE {
        task_set(
            taskId,
            8,
            (UseRegisteelHm_Callback as *const () as usize as u32 >> 16) as i16,
        );
        task_set(
            taskId,
            9,
            UseRegisteelHm_Callback as *const () as usize as u32 as i16,
        );
    } else {
        task_set(
            taskId,
            8,
            (UseRegirockHm_Callback as *const () as usize as u32 >> 16) as i16,
        );
        task_set(
            taskId,
            9,
            UseRegirockHm_Callback as *const () as usize as u32 as i16,
        );
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ShouldDoBrailleRegicePuzzle() -> u8 {
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
        for i in 0..36u8 {
            let xPos: u8 = sRegicePathCoords[i][0];
            let yPos: u8 = sRegicePathCoords[i][1];
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
        }
        FlagSet(FLAG_TEMP_REGICE_PUZZLE_FAILED);
        FlagClear(FLAG_TEMP_REGICE_PUZZLE_STARTED);
    }
    FALSE
}
