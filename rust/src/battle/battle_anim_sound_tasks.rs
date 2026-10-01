//! Translated from `src/battle_anim_sound_tasks.c` by tools/rustport/c2rs.py.
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
    unused_assignments
)]

use crate::battle_anim::{
    BattleAnimAdjustPanning, CalculatePanIncrement, DestroyAnimSoundTask, DestroyAnimVisualTask,
    IsBattlerSpriteVisible, IsContest, KeepPanInRange, gAnimCustomPanning, gBattleAnimAttacker,
    gBattleAnimTarget,
};
use crate::battle_anim::{gAnimBattlerSpecies, gBattleAnimArgs};
use crate::battle_anim_mons::GetBattlerSide;
use crate::battle_main::gBattlerPartyIndexes;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::contest::gContestResources;
use crate::pokemon::{GetMonData2, gEnemyParty, gPlayerParty};
use crate::sound::{
    IsCryPlaying, PlayCry_ByMode, PlayCry_DuckNoRestore, PlaySE1WithPanning, PlaySE2WithPanning,
    PlaySE12WithPanning, StopCryAndClearCrySongs,
};
use crate::task::{task_func, task_get, task_set, task_set_func};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// The C's names for task and sprite data slots.
const tSpecies: usize = 1;
const tPan: usize = 2;
const tState: usize = 9;
const tLastCry: usize = 10;

#[unsafe(no_mangle)]
pub unsafe fn SoundTask_FireBlast(taskId: u8) {
    task_set(taskId, 0, gBattleAnimArgs[0]);
    task_set(taskId, 1, gBattleAnimArgs[1]);
    let pan1: i8 = BattleAnimAdjustPanning(SOUND_PAN_ATTACKER);
    let pan2: i8 = BattleAnimAdjustPanning(SOUND_PAN_TARGET);
    let panIncrement: i8 = CalculatePanIncrement(pan1 as i16, pan2 as i16, 2) as i8;
    task_set(taskId, 2, pan1 as i16);
    task_set(taskId, 3, pan2 as i16);
    task_set(taskId, 4, panIncrement as i16);
    task_set(taskId, 10, 10);
    task_set_func(taskId, Some(SoundTask_FireBlast_Step1));
}
pub(crate) unsafe fn SoundTask_FireBlast_Step1(taskId: u8) {
    let mut pan: i16 = task_get(taskId, 2);
    let panIncrement: i8 = task_get(taskId, 4) as i8;
    if ({
        task_set(taskId, 11, task_get(taskId, 11) + 1);
        task_get(taskId, 11)
    }) == 111
    {
        task_set(taskId, 10, 5);
        task_set(taskId, 11, 0);
        task_set_func(taskId, Some(SoundTask_FireBlast_Step2));
    } else {
        if ({
            task_set(taskId, 10, task_get(taskId, 10) + 1);
            task_get(taskId, 10)
        }) == 11
        {
            task_set(taskId, 10, 0);
            PlaySE12WithPanning(task_get(taskId, 0) as u16, pan as i8);
        }
        pan += panIncrement as i16;
        task_set(taskId, 2, KeepPanInRange(pan, panIncrement as i32));
    }
}
pub(crate) unsafe fn SoundTask_FireBlast_Step2(taskId: u8) {
    if ({
        task_set(taskId, 10, task_get(taskId, 10) + 1);
        task_get(taskId, 10)
    }) == 6
    {
        task_set(taskId, 10, 0);
        let pan: i8 = BattleAnimAdjustPanning(SOUND_PAN_TARGET);
        PlaySE12WithPanning(task_get(taskId, 1) as u16, pan);
        if ({
            task_set(taskId, 11, task_get(taskId, 11) + 1);
            task_get(taskId, 11)
        }) == 2
        {
            DestroyAnimSoundTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SoundTask_LoopSEAdjustPanning(taskId: u8) {
    let songId: u16 = gBattleAnimArgs[0] as u16;
    let mut targetPan: i8 = gBattleAnimArgs[2] as i8;
    let mut panIncrement: i8 = gBattleAnimArgs[3] as i8;
    let r10: u8 = gBattleAnimArgs[4] as u8;
    let r7: u8 = gBattleAnimArgs[5] as u8;
    let r9: u8 = gBattleAnimArgs[6] as u8;
    let sourcePan: i8 = BattleAnimAdjustPanning(gBattleAnimArgs[1] as i8);
    targetPan = BattleAnimAdjustPanning(targetPan);
    panIncrement =
        CalculatePanIncrement(sourcePan as i16, targetPan as i16, panIncrement as i16) as i8;
    task_set(taskId, 0, songId as i16);
    task_set(taskId, 1, sourcePan as i16);
    task_set(taskId, 2, targetPan as i16);
    task_set(taskId, 3, panIncrement as i16);
    task_set(taskId, 4, r10 as i16);
    task_set(taskId, 5, r7 as i16);
    task_set(taskId, 6, r9 as i16);
    task_set(taskId, 10, 0);
    task_set(taskId, 11, sourcePan as i16);
    task_set(taskId, 12, r9 as i16);
    task_set_func(taskId, Some(SoundTask_LoopSEAdjustPanning_Step));
    task_func(taskId).unwrap_unchecked()(taskId);
}
pub(crate) unsafe fn SoundTask_LoopSEAdjustPanning_Step(taskId: u8) {
    if ({
        let t1 = task_get(taskId, 12);
        task_set(taskId, 12, task_get(taskId, 12) + 1);
        t1
    }) == task_get(taskId, 6)
    {
        task_set(taskId, 12, 0);
        PlaySE12WithPanning(task_get(taskId, 0) as u16, task_get(taskId, 11) as i8);
        if ({
            task_set(taskId, 4, task_get(taskId, 4) - 1);
            task_get(taskId, 4)
        }) == 0
        {
            DestroyAnimSoundTask(taskId);
            return;
        }
    }
    if ({
        let t3 = task_get(taskId, 10);
        task_set(taskId, 10, task_get(taskId, 10) + 1);
        t3
    }) == task_get(taskId, 5)
    {
        task_set(taskId, 10, 0);
        let dPan: u16 = task_get(taskId, 3) as u16;
        let oldPan: u16 = task_get(taskId, 11) as u16;
        task_set(taskId, 11, dPan as i16 + oldPan as i16);
        task_set(
            taskId,
            11,
            KeepPanInRange(task_get(taskId, 11), oldPan as i32),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SoundTask_PlayCryHighPitch(taskId: u8) {
    let mut species: u16 = 0;
    let pan: i8 = BattleAnimAdjustPanning(SOUND_PAN_ATTACKER);
    if IsContest() != 0 {
        if gBattleAnimArgs[0] == 0 {
            species = (*(*gContestResources).moveAnim).species;
        } else {
            DestroyAnimVisualTask(taskId);
        }
    } else {
        let mut battler: u8 = 0;
        if gBattleAnimArgs[0] == 0 {
            battler = gBattleAnimAttacker;
        } else if gBattleAnimArgs[0] == ANIM_TARGET as i16 {
            battler = gBattleAnimTarget;
        } else if gBattleAnimArgs[0] == ANIM_ATK_PARTNER as i16 {
            battler = gBattleAnimAttacker ^ 2;
        } else {
            battler = gBattleAnimTarget ^ 2;
        }
        if (gBattleAnimArgs[0] == ANIM_TARGET as i16
            || gBattleAnimArgs[0] == ANIM_DEF_PARTNER as i16)
            && IsBattlerSpriteVisible(battler) == 0
        {
            DestroyAnimVisualTask(taskId);
            return;
        }
        if GetBattlerSide(battler) != B_SIDE_PLAYER {
            species = GetMonData2(
                &raw mut gEnemyParty[gBattlerPartyIndexes[battler]],
                MON_DATA_SPECIES,
            ) as u16;
        } else {
            species = GetMonData2(
                &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
                MON_DATA_SPECIES,
            ) as u16;
        }
    }
    if species != SPECIES_NONE {
        PlayCry_ByMode(species, pan, CRY_MODE_HIGH_PITCH);
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn SoundTask_PlayDoubleCry(taskId: u8) {
    let mut species: u16 = 0;
    let pan: i8 = BattleAnimAdjustPanning(SOUND_PAN_ATTACKER);
    if IsContest() != 0 {
        if gBattleAnimArgs[0] == 0 {
            species = (*(*gContestResources).moveAnim).species;
        } else {
            DestroyAnimVisualTask(taskId);
        }
    } else {
        let mut battler: u8 = 0;
        if gBattleAnimArgs[0] == 0 {
            battler = gBattleAnimAttacker;
        } else if gBattleAnimArgs[0] == ANIM_TARGET as i16 {
            battler = gBattleAnimTarget;
        } else if gBattleAnimArgs[0] == ANIM_ATK_PARTNER as i16 {
            battler = gBattleAnimAttacker ^ 2;
        } else {
            battler = gBattleAnimTarget ^ 2;
        }
        if (gBattleAnimArgs[0] == ANIM_TARGET as i16
            || gBattleAnimArgs[0] == ANIM_DEF_PARTNER as i16)
            && IsBattlerSpriteVisible(battler) == 0
        {
            DestroyAnimVisualTask(taskId);
            return;
        }
        if GetBattlerSide(battler) != B_SIDE_PLAYER {
            species = GetMonData2(
                &raw mut gEnemyParty[gBattlerPartyIndexes[battler]],
                MON_DATA_SPECIES,
            ) as u16;
        } else {
            species = GetMonData2(
                &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
                MON_DATA_SPECIES,
            ) as u16;
        }
    }
    task_set(taskId, 0, gBattleAnimArgs[1]);
    task_set(taskId, 1, species as i16);
    task_set(taskId, 2, pan as i16);
    if species != SPECIES_NONE {
        if gBattleAnimArgs[1] == DOUBLE_CRY_GROWL {
            PlayCry_ByMode(species, pan, CRY_MODE_GROWL_1);
        } else {
            PlayCry_ByMode(species, pan, CRY_MODE_ROAR_1);
        }
        task_set_func(taskId, Some(SoundTask_PlayDoubleCry_Step));
    } else {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe fn SoundTask_PlayDoubleCry_Step(taskId: u8) {
    let species: u16 = task_get(taskId, 1) as u16;
    let pan: i8 = task_get(taskId, 2) as i8;
    if task_get(taskId, 9) < 2 {
        task_set(taskId, 9, task_get(taskId, 9) + 1);
    } else {
        if task_get(taskId, 0) == DOUBLE_CRY_GROWL {
            if IsCryPlaying() == 0 {
                PlayCry_ByMode(species, pan, CRY_MODE_GROWL_2);
                DestroyAnimVisualTask(taskId);
            }
        } else {
            if IsCryPlaying() == 0 {
                PlayCry_ByMode(species, pan, CRY_MODE_ROAR_2);
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SoundTask_WaitForCry(taskId: u8) {
    if task_get(taskId, 9) < 2 {
        task_set(taskId, 9, task_get(taskId, 9) + 1);
    } else {
        if IsCryPlaying() == 0 {
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SoundTask_PlayCryWithEcho(taskId: u8) {
    let mut species: u16 = 0;
    task_set(taskId, tLastCry, gBattleAnimArgs[0]);
    let pan: i8 = BattleAnimAdjustPanning(SOUND_PAN_ATTACKER);
    if IsContest() != 0 {
        species = (*(*gContestResources).moveAnim).species;
    } else {
        species = gAnimBattlerSpecies[gBattleAnimAttacker];
    }
    task_set(taskId, tSpecies, species as i16);
    task_set(taskId, tPan, pan as i16);
    if species != SPECIES_NONE {
        task_set_func(taskId, Some(SoundTask_PlayCryWithEcho_Step));
    } else {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe fn SoundTask_PlayCryWithEcho_Step(taskId: u8) {
    let species: u16 = task_get(taskId, tSpecies) as u16;
    let pan: i8 = task_get(taskId, tPan) as i8;
    'l1: {
        let sw1: i16 = task_get(taskId, tState);
        let matched = sw1 == 2 || sw1 == 1 || sw1 == 3 || sw1 == 4 || sw1 == 5 || sw1 == 0;
        let mut fall = false;
        if sw1 == 2 {
            PlayCry_DuckNoRestore(species, pan, CRY_MODE_ECHO_START);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
            break 'l1;
        }
        if sw1 == 1 || sw1 == 3 || sw1 == 4 {
            task_set(taskId, tState, task_get(taskId, tState) + 1);
            break 'l1;
        }
        if sw1 == 5 {
            fall = true;
            if IsCryPlaying() != 0 {
                break 'l1;
            }
        }
        if fall || sw1 == 0 {
            StopCryAndClearCrySongs();
            task_set(taskId, tState, task_get(taskId, tState) + 1);
            break 'l1;
        }
        if !matched {
            if task_get(taskId, tLastCry) == 0 {
                PlayCry_DuckNoRestore(species, pan, CRY_MODE_ECHO_END);
            } else {
                PlayCry_ByMode(species, pan, CRY_MODE_ECHO_END);
            }
            DestroyAnimVisualTask(taskId);
            break 'l1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SoundTask_PlaySE1WithPanning(taskId: u8) {
    let songId: u16 = gBattleAnimArgs[0] as u16;
    let pan: i8 = BattleAnimAdjustPanning(gBattleAnimArgs[1] as i8);
    PlaySE1WithPanning(songId, pan);
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn SoundTask_PlaySE2WithPanning(taskId: u8) {
    let songId: u16 = gBattleAnimArgs[0] as u16;
    let pan: i8 = BattleAnimAdjustPanning(gBattleAnimArgs[1] as i8);
    PlaySE2WithPanning(songId, pan);
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn SoundTask_AdjustPanningVar(taskId: u8) {
    let mut targetPan: i8 = gBattleAnimArgs[1] as i8;
    let mut panIncrement: i8 = gBattleAnimArgs[2] as i8;
    let r9: u16 = gBattleAnimArgs[3] as u16;
    let sourcePan: i8 = BattleAnimAdjustPanning(gBattleAnimArgs[0] as i8);
    targetPan = BattleAnimAdjustPanning(targetPan);
    panIncrement =
        CalculatePanIncrement(sourcePan as i16, targetPan as i16, panIncrement as i16) as i8;
    task_set(taskId, 1, sourcePan as i16);
    task_set(taskId, 2, targetPan as i16);
    task_set(taskId, 3, panIncrement as i16);
    task_set(taskId, 5, r9 as i16);
    task_set(taskId, 10, 0);
    task_set(taskId, 11, sourcePan as i16);
    task_set_func(taskId, Some(SoundTask_AdjustPanningVar_Step));
    task_func(taskId).unwrap_unchecked()(taskId);
}
pub(crate) unsafe fn SoundTask_AdjustPanningVar_Step(taskId: u8) {
    let panIncrement: u16 = task_get(taskId, 3) as u16;
    if ({
        let t1 = task_get(taskId, 10);
        task_set(taskId, 10, task_get(taskId, 10) + 1);
        t1
    }) == task_get(taskId, 5)
    {
        task_set(taskId, 10, 0);
        let oldPan: u16 = task_get(taskId, 11) as u16;
        task_set(taskId, 11, panIncrement as i16 + oldPan as i16);
        task_set(
            taskId,
            11,
            KeepPanInRange(task_get(taskId, 11), oldPan as i32),
        );
    }
    gAnimCustomPanning = task_get(taskId, 11) as u8;
    if task_get(taskId, 11) == task_get(taskId, 2) {
        DestroyAnimVisualTask(taskId);
    }
}
