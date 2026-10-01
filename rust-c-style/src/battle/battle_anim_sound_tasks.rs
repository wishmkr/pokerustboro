//! Translated from `src/battle_anim_sound_tasks.c` by tools/rustport/c2rs.py.
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

unsafe extern "C" {
    static mut gAnimBattlerSpecies: CArray<u16, 4>;
    static mut gAnimCustomPanning: u8;
    static mut gBattleAnimArgs: CArray<i16, 8>;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattlerPartyIndexes: CArray<u16, 4>;
    static mut gContestResources: *mut ContestResources;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gTasks: CArray<Task, 0>;
    fn BattleAnimAdjustPanning(a0: i8) -> i8;
    fn CalculatePanIncrement(a0: i16, a1: i16, a2: i16) -> i16;
    fn DestroyAnimSoundTask(a0: u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn IsCryPlaying() -> u8;
    fn KeepPanInRange(a0: i16, a1: i32) -> i16;
    fn PlayCry_ByMode(a0: u16, a1: i8, a2: u8);
    fn PlayCry_DuckNoRestore(a0: u16, a1: i8, a2: u8);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn PlaySE1WithPanning(a0: u16, a1: i8);
    fn PlaySE2WithPanning(a0: u16, a1: i8);
    fn StopCryAndClearCrySongs();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundTask_FireBlast(taskId: u8) {
    let mut pan1: i8 = 0;
    let mut pan2: i8 = 0;
    let mut panIncrement: i8 = 0;
    gTasks[taskId].data[0] = gBattleAnimArgs[0];
    gTasks[taskId].data[1] = gBattleAnimArgs[1];
    pan1 = BattleAnimAdjustPanning(SOUND_PAN_ATTACKER);
    pan2 = BattleAnimAdjustPanning(SOUND_PAN_TARGET);
    panIncrement = CalculatePanIncrement(pan1 as i16, pan2 as i16, 2) as i8;
    gTasks[taskId].data[2] = pan1 as i16;
    gTasks[taskId].data[3] = pan2 as i16;
    gTasks[taskId].data[4] = panIncrement as i16;
    gTasks[taskId].data[10] = 10;
    gTasks[taskId].func = Some(SoundTask_FireBlast_Step1);
}
pub(crate) unsafe extern "C" fn SoundTask_FireBlast_Step1(taskId: u8) {
    let mut pan: i16 = gTasks[taskId].data[2];
    let mut panIncrement: i8 = gTasks[taskId].data[4] as i8;
    if ({
        gTasks[taskId].data[11] += 1;
        gTasks[taskId].data[11]
    }) == 111
    {
        gTasks[taskId].data[10] = 5;
        gTasks[taskId].data[11] = 0;
        gTasks[taskId].func = Some(SoundTask_FireBlast_Step2);
    } else {
        if ({
            gTasks[taskId].data[10] += 1;
            gTasks[taskId].data[10]
        }) == 11
        {
            gTasks[taskId].data[10] = 0;
            PlaySE12WithPanning(gTasks[taskId].data[0] as u16, pan as i8);
        }
        pan += panIncrement as i16;
        gTasks[taskId].data[2] = KeepPanInRange(pan, panIncrement as i32);
    }
}
pub(crate) unsafe extern "C" fn SoundTask_FireBlast_Step2(taskId: u8) {
    if ({
        gTasks[taskId].data[10] += 1;
        gTasks[taskId].data[10]
    }) == 6
    {
        let mut pan: i8 = 0;
        gTasks[taskId].data[10] = 0;
        pan = BattleAnimAdjustPanning(SOUND_PAN_TARGET);
        PlaySE12WithPanning(gTasks[taskId].data[1] as u16, pan);
        if ({
            gTasks[taskId].data[11] += 1;
            gTasks[taskId].data[11]
        }) == 2
        {
            DestroyAnimSoundTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundTask_LoopSEAdjustPanning(taskId: u8) {
    let mut songId: u16 = gBattleAnimArgs[0] as u16;
    let mut targetPan: i8 = gBattleAnimArgs[2] as i8;
    let mut panIncrement: i8 = gBattleAnimArgs[3] as i8;
    let mut r10: u8 = gBattleAnimArgs[4] as u8;
    let mut r7: u8 = gBattleAnimArgs[5] as u8;
    let mut r9: u8 = gBattleAnimArgs[6] as u8;
    let mut sourcePan: i8 = BattleAnimAdjustPanning(gBattleAnimArgs[1] as i8);
    targetPan = BattleAnimAdjustPanning(targetPan);
    panIncrement =
        CalculatePanIncrement(sourcePan as i16, targetPan as i16, panIncrement as i16) as i8;
    gTasks[taskId].data[0] = songId as i16;
    gTasks[taskId].data[1] = sourcePan as i16;
    gTasks[taskId].data[2] = targetPan as i16;
    gTasks[taskId].data[3] = panIncrement as i16;
    gTasks[taskId].data[4] = r10 as i16;
    gTasks[taskId].data[5] = r7 as i16;
    gTasks[taskId].data[6] = r9 as i16;
    gTasks[taskId].data[10] = 0;
    gTasks[taskId].data[11] = sourcePan as i16;
    gTasks[taskId].data[12] = r9 as i16;
    gTasks[taskId].func = Some(SoundTask_LoopSEAdjustPanning_Step);
    gTasks[taskId].func.unwrap_unchecked()(taskId);
}
pub(crate) unsafe extern "C" fn SoundTask_LoopSEAdjustPanning_Step(taskId: u8) {
    if ({
        let t1 = gTasks[taskId].data[12];
        gTasks[taskId].data[12] += 1;
        t1
    }) == gTasks[taskId].data[6]
    {
        gTasks[taskId].data[12] = 0;
        PlaySE12WithPanning(gTasks[taskId].data[0] as u16, gTasks[taskId].data[11] as i8);
        if ({
            gTasks[taskId].data[4] -= 1;
            gTasks[taskId].data[4]
        }) == 0
        {
            DestroyAnimSoundTask(taskId);
            return;
        }
    }
    if ({
        let t3 = gTasks[taskId].data[10];
        gTasks[taskId].data[10] += 1;
        t3
    }) == gTasks[taskId].data[5]
    {
        let mut dPan: u16 = 0;
        let mut oldPan: u16 = 0;
        gTasks[taskId].data[10] = 0;
        dPan = gTasks[taskId].data[3] as u16;
        oldPan = gTasks[taskId].data[11] as u16;
        gTasks[taskId].data[11] = dPan as i16 + oldPan as i16;
        gTasks[taskId].data[11] = KeepPanInRange(gTasks[taskId].data[11], oldPan as i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundTask_PlayCryHighPitch(taskId: u8) {
    let mut species: u16 = 0;
    let mut pan: i8 = BattleAnimAdjustPanning(SOUND_PAN_ATTACKER);
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
pub unsafe extern "C" fn SoundTask_PlayDoubleCry(taskId: u8) {
    let mut species: u16 = 0;
    let mut pan: i8 = BattleAnimAdjustPanning(SOUND_PAN_ATTACKER);
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
    gTasks[taskId].data[0] = gBattleAnimArgs[1];
    gTasks[taskId].data[1] = species as i16;
    gTasks[taskId].data[2] = pan as i16;
    if species != SPECIES_NONE {
        if gBattleAnimArgs[1] == DOUBLE_CRY_GROWL {
            PlayCry_ByMode(species, pan, CRY_MODE_GROWL_1);
        } else {
            PlayCry_ByMode(species, pan, CRY_MODE_ROAR_1);
        }
        gTasks[taskId].func = Some(SoundTask_PlayDoubleCry_Step);
    } else {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn SoundTask_PlayDoubleCry_Step(taskId: u8) {
    let mut species: u16 = gTasks[taskId].data[1] as u16;
    let mut pan: i8 = gTasks[taskId].data[2] as i8;
    if gTasks[taskId].data[9] < 2 {
        gTasks[taskId].data[9] += 1;
    } else {
        if gTasks[taskId].data[0] == DOUBLE_CRY_GROWL {
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
pub unsafe extern "C" fn SoundTask_WaitForCry(taskId: u8) {
    if gTasks[taskId].data[9] < 2 {
        gTasks[taskId].data[9] += 1;
    } else {
        if IsCryPlaying() == 0 {
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundTask_PlayCryWithEcho(taskId: u8) {
    let mut species: u16 = 0;
    let mut pan: i8 = 0;
    gTasks[taskId].data[10] = gBattleAnimArgs[0];
    pan = BattleAnimAdjustPanning(SOUND_PAN_ATTACKER);
    if IsContest() != 0 {
        species = (*(*gContestResources).moveAnim).species;
    } else {
        species = gAnimBattlerSpecies[gBattleAnimAttacker];
    }
    gTasks[taskId].data[1] = species as i16;
    gTasks[taskId].data[2] = pan as i16;
    if species != SPECIES_NONE {
        gTasks[taskId].func = Some(SoundTask_PlayCryWithEcho_Step);
    } else {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn SoundTask_PlayCryWithEcho_Step(taskId: u8) {
    let mut species: u16 = gTasks[taskId].data[1] as u16;
    let mut pan: i8 = gTasks[taskId].data[2] as i8;
    'l1: {
        let sw1: i16 = gTasks[taskId].data[9];
        let matched = sw1 == 2 || sw1 == 1 || sw1 == 3 || sw1 == 4 || sw1 == 5 || sw1 == 0;
        let mut fall = false;
        if sw1 == 2 {
            fall = true;
            PlayCry_DuckNoRestore(species, pan, CRY_MODE_ECHO_START);
            gTasks[taskId].data[9] += 1;
            break 'l1;
        }
        if sw1 == 1 || sw1 == 3 || sw1 == 4 {
            fall = true;
            gTasks[taskId].data[9] += 1;
            break 'l1;
        }
        if sw1 == 5 {
            fall = true;
            if IsCryPlaying() != 0 {
                break 'l1;
            }
        }
        if fall || sw1 == 0 {
            fall = true;
            StopCryAndClearCrySongs();
            gTasks[taskId].data[9] += 1;
            break 'l1;
        }
        if !matched {
            fall = true;
            if gTasks[taskId].data[10] == 0 {
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
pub unsafe extern "C" fn SoundTask_PlaySE1WithPanning(taskId: u8) {
    let mut songId: u16 = gBattleAnimArgs[0] as u16;
    let mut pan: i8 = BattleAnimAdjustPanning(gBattleAnimArgs[1] as i8);
    PlaySE1WithPanning(songId, pan);
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundTask_PlaySE2WithPanning(taskId: u8) {
    let mut songId: u16 = gBattleAnimArgs[0] as u16;
    let mut pan: i8 = BattleAnimAdjustPanning(gBattleAnimArgs[1] as i8);
    PlaySE2WithPanning(songId, pan);
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundTask_AdjustPanningVar(taskId: u8) {
    let mut targetPan: i8 = gBattleAnimArgs[1] as i8;
    let mut panIncrement: i8 = gBattleAnimArgs[2] as i8;
    let mut r9: u16 = gBattleAnimArgs[3] as u16;
    let mut sourcePan: i8 = BattleAnimAdjustPanning(gBattleAnimArgs[0] as i8);
    targetPan = BattleAnimAdjustPanning(targetPan);
    panIncrement =
        CalculatePanIncrement(sourcePan as i16, targetPan as i16, panIncrement as i16) as i8;
    gTasks[taskId].data[1] = sourcePan as i16;
    gTasks[taskId].data[2] = targetPan as i16;
    gTasks[taskId].data[3] = panIncrement as i16;
    gTasks[taskId].data[5] = r9 as i16;
    gTasks[taskId].data[10] = 0;
    gTasks[taskId].data[11] = sourcePan as i16;
    gTasks[taskId].func = Some(SoundTask_AdjustPanningVar_Step);
    gTasks[taskId].func.unwrap_unchecked()(taskId);
}
pub(crate) unsafe extern "C" fn SoundTask_AdjustPanningVar_Step(taskId: u8) {
    let mut panIncrement: u16 = gTasks[taskId].data[3] as u16;
    if ({
        let t1 = gTasks[taskId].data[10];
        gTasks[taskId].data[10] += 1;
        t1
    }) == gTasks[taskId].data[5]
    {
        let mut oldPan: u16 = 0;
        gTasks[taskId].data[10] = 0;
        oldPan = gTasks[taskId].data[11] as u16;
        gTasks[taskId].data[11] = panIncrement as i16 + oldPan as i16;
        gTasks[taskId].data[11] = KeepPanInRange(gTasks[taskId].data[11], oldPan as i32);
    }
    gAnimCustomPanning = gTasks[taskId].data[11] as u8;
    if gTasks[taskId].data[11] == gTasks[taskId].data[2] {
        DestroyAnimVisualTask(taskId);
    }
}
