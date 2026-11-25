//! Translated from `src/battle_ai_script_commands.c` by tools/rustport/c2rs.py.
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
    clippy::self_assignment,
    clippy::type_complexity,
    dead_code,
    unused_assignments
)]

use crate::battle_anim_mons::{GetBattlerAtPosition, GetBattlerPosition, GetBattlerSide};
use crate::battle_factory::GetAiScriptsInBattleFactory;
use crate::battle_main::{
    GetWhoStrikesFirst, gAbsentBattlerFlags, gActiveBattler, gBattleMons, gBattleMoveDamage,
    gBattleResources, gBattleResults, gBattleScripting, gBattleStruct, gBattleTypeFlags,
    gBattleWeather, gBattlerTarget, gCritMultiplier, gCurrentMove, gDisableStructs,
    gDynamicBasePower, gMoveResultFlags, gStatuses3,
};
use crate::battle_main::{gBattlerPartyIndexes, gLastMoves, gSideStatuses};
use crate::battle_script_commands::{AI_CalcDmg, TypeCalc};
use crate::battle_setup::{gTrainerBattleOpponent_A, gTrainerBattleOpponent_B};
use crate::battle_util::CheckMoveLimitations;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::item::GetItemHoldEffect;
use crate::pokemon::{GetGenderFromSpeciesAndPersonality, GetMonData2, gEnemyParty, gPlayerParty};
use crate::random::Random;
use crate::recorded_battle::GetAiScriptsInRecordedBattle;
#[allow(unused_imports)]
use crate::types::*;
use crate::util::gBitTable;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): sBattleAICmdTable sIgnoredPowerfulMoveEffects

const AIState_DoNotProcess: u8 = 3;
const AIState_FinishedProcessing: u8 = 2;
const AIState_Processing: u8 = 1;
const AIState_SettingUp: u8 = 0;
const AI_ACTION_DONE: u8 = 1;
const AI_ACTION_DO_NOT_ATTACK: i32 = 8;
const AI_ACTION_FLEE: i32 = 2;
const AI_ACTION_WATCH: i32 = 4;
const IGNORED_MOVES_END: u16 = 65535;

static sBattleAICmdTable: Table<CArray<Option<unsafe fn()>, 99>> =
    Table((&raw const crate::data::battle_ai_script_commands::sBattleAICmdTable).cast());
static sIgnoredPowerfulMoveEffects: Table<CArray<u16, 13>> =
    Table((&raw const crate::data::battle_ai_script_commands::sIgnoredPowerfulMoveEffects).cast());

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gAIScriptPtr: crate::global::Global<*mut u8> = crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sBattler_AI: crate::global::Global<u8> = crate::global::Global::new(0);

pub unsafe fn BattleAI_HandleItemUseBeforeAISetup(defaultScoreMoves: u8) {
    let data: *mut u8 = (*crate::gl!(gBattleResources)).battleHistory as *mut u8;
    let mut i: i32 = 0;
    while i < 84 {
        *data.at(i) = 0;
        i += 1;
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER != 0 && gBattleTypeFlags.get() & 0xa7f0982 == 0
    {
        for i in 0..MAX_TRAINER_ITEMS {
            if (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
                [gTrainerBattleOpponent_A.get()]
            .items[i]
                != ITEM_NONE
            {
                (*(*crate::gl!(gBattleResources)).battleHistory).trainerItems
                    [(*(*crate::gl!(gBattleResources)).battleHistory).itemsNo] =
                    (*(&raw const crate::data::data_tables::gTrainers)
                        .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_A.get()]
                    .items[i];
                (*(*crate::gl!(gBattleResources)).battleHistory).itemsNo += 1;
            }
        }
    }
    BattleAI_SetupAIData(defaultScoreMoves);
}
pub unsafe fn BattleAI_SetupAIData(mut defaultScoreMoves: u8) {
    let data: *mut u8 = (*crate::gl!(gBattleResources)).ai as *mut u8;
    for i in 0..28i32 {
        *data.at(i) = 0;
    }
    let mut i: i32 = 0;
    while i < MAX_MON_MOVES {
        if defaultScoreMoves as i32 & 1 != 0 {
            (*(*crate::gl!(gBattleResources)).ai).score[i] = 100;
        } else {
            (*(*crate::gl!(gBattleResources)).ai).score[i] = 0;
        }
        defaultScoreMoves >>= 1;
        i += 1;
    }
    let moveLimitations: u8 = CheckMoveLimitations(gActiveBattler.get(), 0, MOVE_LIMITATIONS_ALL);
    for i in 0..MAX_MON_MOVES {
        if gBitTable[i] & moveLimitations as u32 != 0 {
            (*(*crate::gl!(gBattleResources)).ai).score[i] = 0;
        }
        (*(*crate::gl!(gBattleResources)).ai).simulatedRNG[i] = 100 - (Random() as i32 % 16) as u8;
    }
    (*(*crate::gl!(gBattleResources)).AI_ScriptsStack).size = 0;
    sBattler_AI.set(gActiveBattler.get());
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
        gBattlerTarget
            .set((Random() as u8 & BIT_FLANK) + (GetBattlerSide(gActiveBattler.get()) ^ 1));
        if gAbsentBattlerFlags.get() as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gBattlerTarget.get()]
            != 0
        {
            gBattlerTarget.set(gBattlerTarget.get() ^ (BIT_FLANK));
        }
    } else {
        gBattlerTarget.set(sBattler_AI.get() ^ 1);
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED != 0 {
        (*(*crate::gl!(gBattleResources)).ai).aiFlags = GetAiScriptsInRecordedBattle();
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_SAFARI != 0 {
        (*(*crate::gl!(gBattleResources)).ai).aiFlags = AI_SCRIPT_SAFARI;
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_ROAMER != 0 {
        (*(*crate::gl!(gBattleResources)).ai).aiFlags = AI_SCRIPT_ROAMING;
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_FIRST_BATTLE != 0 {
        (*(*crate::gl!(gBattleResources)).ai).aiFlags = 0x80000000;
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_FACTORY != 0 {
        (*(*crate::gl!(gBattleResources)).ai).aiFlags = GetAiScriptsInBattleFactory();
    } else if gBattleTypeFlags.get() & 0xc3f0900 != 0 {
        (*(*crate::gl!(gBattleResources)).ai).aiFlags = 7;
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_TWO_OPPONENTS != 0 {
        (*(*crate::gl!(gBattleResources)).ai).aiFlags =
            (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
                [gTrainerBattleOpponent_A.get()]
            .aiFlags
                | (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
                    [gTrainerBattleOpponent_B.get()]
                .aiFlags;
    } else {
        (*(*crate::gl!(gBattleResources)).ai).aiFlags =
            (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
                [gTrainerBattleOpponent_A.get()]
            .aiFlags;
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
        (*(*crate::gl!(gBattleResources)).ai).aiFlags |= AI_SCRIPT_DOUBLE_BATTLE;
    }
}
pub unsafe fn BattleAI_ChooseMoveOrAction() -> u8 {
    let savedCurrentMove: u16 = gCurrentMove.get();
    let mut ret: u8 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE == 0 {
        ret = ChooseMoveOrAction_Singles();
    } else {
        ret = ChooseMoveOrAction_Doubles();
    }
    gCurrentMove.set(savedCurrentMove);
    ret
}
unsafe fn ChooseMoveOrAction_Singles() -> u8 {
    let mut currentMoveArray: CArray<u8, 4> = zeroed();
    let mut consideredMoveArray: CArray<u8, 4> = zeroed();
    RecordLastUsedMoveByTarget();
    while (*(*crate::gl!(gBattleResources)).ai).aiFlags != 0 {
        if (*(*crate::gl!(gBattleResources)).ai).aiFlags & 1 != 0 {
            (*(*crate::gl!(gBattleResources)).ai).aiState = AIState_SettingUp;
            BattleAI_DoAIProcessing();
        }
        (*(*crate::gl!(gBattleResources)).ai).aiFlags >>= 1;
        (*(*crate::gl!(gBattleResources)).ai).aiLogicId += 1;
        (*(*crate::gl!(gBattleResources)).ai).movesetIndex = 0;
    }
    if (*(*crate::gl!(gBattleResources)).ai).aiAction as i32 & AI_ACTION_FLEE != 0 {
        return AI_CHOICE_FLEE;
    }
    if (*(*crate::gl!(gBattleResources)).ai).aiAction as i32 & AI_ACTION_WATCH != 0 {
        return AI_CHOICE_WATCH;
    }
    let mut numOfBestMoves: u8 = 1;
    currentMoveArray[0] = (*(*crate::gl!(gBattleResources)).ai).score[0] as u8;
    consideredMoveArray[0] = 0;
    for i in 1..MAX_MON_MOVES {
        if ({
            let gl_i0 = sBattler_AI.get();
            crate::gl!(gBattleMons[gl_i0].moves[i])
        }) != MOVE_NONE
        {
            if currentMoveArray[0] as i32 == (*(*crate::gl!(gBattleResources)).ai).score[i] as i32 {
                currentMoveArray[numOfBestMoves] =
                    (*(*crate::gl!(gBattleResources)).ai).score[i] as u8;
                consideredMoveArray[{
                    let t1 = numOfBestMoves;
                    numOfBestMoves += 1;
                    t1
                }] = i as u8;
            }
            if (currentMoveArray[0] as i32) < (*(*crate::gl!(gBattleResources)).ai).score[i] as i32
            {
                numOfBestMoves = 1;
                currentMoveArray[0] = (*(*crate::gl!(gBattleResources)).ai).score[i] as u8;
                consideredMoveArray[0] = i as u8;
            }
        }
    }
    consideredMoveArray[rem_i32(Random() as i32, numOfBestMoves as i32)]
}
unsafe fn ChooseMoveOrAction_Doubles() -> u8 {
    let mut scriptsToRun: i32 = 0;
    let mut bestMovePointsForTarget: CArray<i16, 4> = zeroed();
    let mut mostViableTargetsArray: CArray<i8, 4> = zeroed();
    let mut actionOrMoveIndex: CArray<u8, 4> = zeroed();
    let mut mostViableMovesScores: CArray<u8, 4> = zeroed();
    let mut mostViableMovesIndices: CArray<u8, 4> = zeroed();
    let mut mostViableMovesNo: i32 = 0;
    let mut i: i32 = 0;
    while i < MAX_BATTLERS_COUNT as i32 {
        if i == sBattler_AI.get() as i32 || crate::gl!(gBattleMons[i].hp) == 0 {
            actionOrMoveIndex[i] = 0xFF;
            bestMovePointsForTarget[i] = -1;
        } else {
            if gBattleTypeFlags.get() & BATTLE_TYPE_PALACE != 0 {
                BattleAI_SetupAIData((*crate::gl!(gBattleStruct)).palaceFlags >> 4);
            } else {
                BattleAI_SetupAIData(ALL_MOVES_MASK);
            }
            gBattlerTarget.set(i as u8);
            if i & BIT_SIDE as i32 != sBattler_AI.get() as i32 & BIT_SIDE as i32 {
                RecordLastUsedMoveByTarget();
            }
            (*(*crate::gl!(gBattleResources)).ai).aiLogicId = 0;
            (*(*crate::gl!(gBattleResources)).ai).movesetIndex = 0;
            scriptsToRun = (*(*crate::gl!(gBattleResources)).ai).aiFlags as i32;
            while scriptsToRun != 0 {
                if scriptsToRun & 1 != 0 {
                    (*(*crate::gl!(gBattleResources)).ai).aiState = AIState_SettingUp;
                    BattleAI_DoAIProcessing();
                }
                scriptsToRun >>= 1;
                (*(*crate::gl!(gBattleResources)).ai).aiLogicId += 1;
                (*(*crate::gl!(gBattleResources)).ai).movesetIndex = 0;
            }
            if (*(*crate::gl!(gBattleResources)).ai).aiAction as i32 & AI_ACTION_FLEE != 0 {
                actionOrMoveIndex[i] = AI_CHOICE_FLEE;
            } else if (*(*crate::gl!(gBattleResources)).ai).aiAction as i32 & AI_ACTION_WATCH != 0 {
                actionOrMoveIndex[i] = AI_CHOICE_WATCH;
            } else {
                mostViableMovesScores[0] = (*(*crate::gl!(gBattleResources)).ai).score[0] as u8;
                mostViableMovesIndices[0] = 0;
                mostViableMovesNo = 1;
                for j in 1..MAX_MON_MOVES {
                    if ({
                        let gl_i1 = sBattler_AI.get();
                        crate::gl!(gBattleMons[gl_i1].moves[j])
                    }) != 0
                    {
                        if mostViableMovesScores[0] as i32
                            == (*(*crate::gl!(gBattleResources)).ai).score[j] as i32
                        {
                            mostViableMovesScores[mostViableMovesNo] =
                                (*(*crate::gl!(gBattleResources)).ai).score[j] as u8;
                            mostViableMovesIndices[mostViableMovesNo] = j as u8;
                            mostViableMovesNo += 1;
                        }
                        if (mostViableMovesScores[0] as i32)
                            < (*(*crate::gl!(gBattleResources)).ai).score[j] as i32
                        {
                            mostViableMovesScores[0] =
                                (*(*crate::gl!(gBattleResources)).ai).score[j] as u8;
                            mostViableMovesIndices[0] = j as u8;
                            mostViableMovesNo = 1;
                        }
                    }
                }
                actionOrMoveIndex[i] =
                    mostViableMovesIndices[rem_i32(Random() as i32, mostViableMovesNo)];
                bestMovePointsForTarget[i] = mostViableMovesScores[0] as i16;
                if i == sBattler_AI.get() as i32 ^ 2 && bestMovePointsForTarget[i] < 100 {
                    bestMovePointsForTarget[i] = -1;
                    mostViableMovesScores[0] = mostViableMovesScores[0];
                }
            }
        }
        i += 1;
    }
    let mut mostMovePoints: i16 = bestMovePointsForTarget[0];
    mostViableTargetsArray[0] = 0;
    let mut mostViableTargetsNo: i32 = 1;
    for i in 1..(MAX_BATTLERS_COUNT as i32) {
        if mostMovePoints == bestMovePointsForTarget[i] {
            mostViableTargetsArray[mostViableTargetsNo] = i as i8;
            mostViableTargetsNo += 1;
        }
        if mostMovePoints < bestMovePointsForTarget[i] {
            mostMovePoints = bestMovePointsForTarget[i];
            mostViableTargetsArray[0] = i as i8;
            mostViableTargetsNo = 1;
        }
    }
    gBattlerTarget.set(mostViableTargetsArray[rem_i32(Random() as i32, mostViableTargetsNo)] as u8);
    actionOrMoveIndex[gBattlerTarget.get()]
}
unsafe fn BattleAI_DoAIProcessing() {
    while (*(*crate::gl!(gBattleResources)).ai).aiState != AIState_FinishedProcessing {
        match (*(*crate::gl!(gBattleResources)).ai).aiState {
            AIState_DoNotProcess => {}
            AIState_SettingUp => {
                crate::gl!(
                    gAIScriptPtr = (*crate::asmdata::gBattleAI_ScriptsTable
                        .cast::<CArray<*mut u8, 0>>())
                        [(*(*crate::gl!(gBattleResources)).ai).aiLogicId]
                );
                if ({
                    let gl_i2 = sBattler_AI.get();
                    let gl_i3 = (*(*crate::gl!(gBattleResources)).ai).movesetIndex;
                    crate::gl!(gBattleMons[gl_i2].pp[gl_i3])
                }) == 0
                {
                    (*(*crate::gl!(gBattleResources)).ai).moveConsidered = 0;
                } else {
                    (*(*crate::gl!(gBattleResources)).ai).moveConsidered = {
                        let gl_i4 = sBattler_AI.get();
                        let gl_i5 = (*(*crate::gl!(gBattleResources)).ai).movesetIndex;
                        crate::gl!(gBattleMons[gl_i4].moves[gl_i5])
                    };
                }
                (*(*crate::gl!(gBattleResources)).ai).aiState += 1;
            }
            AIState_Processing => {
                if (*(*crate::gl!(gBattleResources)).ai).moveConsidered != 0 {
                    sBattleAICmdTable[*crate::gl!(gAIScriptPtr)].unwrap_unchecked()();
                } else {
                    (*(*crate::gl!(gBattleResources)).ai).score
                        [(*(*crate::gl!(gBattleResources)).ai).movesetIndex] = 0;
                    (*(*crate::gl!(gBattleResources)).ai).aiAction |= AI_ACTION_DONE;
                }
                if (*(*crate::gl!(gBattleResources)).ai).aiAction as i32 & AI_ACTION_DONE as i32
                    != 0
                {
                    (*(*crate::gl!(gBattleResources)).ai).movesetIndex += 1;
                    if (*(*crate::gl!(gBattleResources)).ai).movesetIndex < MAX_MON_MOVES as u8
                        && (*(*crate::gl!(gBattleResources)).ai).aiAction as i32
                            & AI_ACTION_DO_NOT_ATTACK
                            == 0
                    {
                        (*(*crate::gl!(gBattleResources)).ai).aiState = AIState_SettingUp;
                    } else {
                        (*(*crate::gl!(gBattleResources)).ai).aiState += 1;
                    }
                    (*(*crate::gl!(gBattleResources)).ai).aiAction &= 254;
                }
            }
            _ => {}
        }
    }
}
unsafe fn RecordLastUsedMoveByTarget() {
    for i in 0..MAX_MON_MOVES {
        if (*(*crate::gl!(gBattleResources)).battleHistory).usedMoves[gBattlerTarget.get()].moves[i]
            == {
                let gl_i0 = gBattlerTarget.get();
                crate::gl!(gLastMoves[gl_i0])
            }
        {
            break;
        }
        if (*(*crate::gl!(gBattleResources)).battleHistory).usedMoves[gBattlerTarget.get()].moves[i]
            == MOVE_NONE
        {
            (*(*crate::gl!(gBattleResources)).battleHistory).usedMoves[gBattlerTarget.get()]
                .moves[i] = {
                let gl_i1 = gBattlerTarget.get();
                crate::gl!(gLastMoves[gl_i1])
            };
            break;
        }
    }
}
pub unsafe fn ClearBattlerMoveHistory(battler: u8) {
    for i in 0..MAX_MON_MOVES {
        (*(*crate::gl!(gBattleResources)).battleHistory).usedMoves[battler].moves[i] = MOVE_NONE;
    }
}
pub unsafe fn RecordAbilityBattle(battler: u8, abilityId: u8) {
    (*(*crate::gl!(gBattleResources)).battleHistory).abilities[battler] = abilityId;
}
pub unsafe fn ClearBattlerAbilityHistory(battler: u8) {
    (*(*crate::gl!(gBattleResources)).battleHistory).abilities[battler] = ABILITY_NONE;
}
pub unsafe fn RecordItemEffectBattle(battler: u8, itemEffect: u8) {
    (*(*crate::gl!(gBattleResources)).battleHistory).itemEffects[battler] = itemEffect;
}
pub unsafe fn ClearBattlerItemEffectHistory(battler: u8) {
    (*(*crate::gl!(gBattleResources)).battleHistory).itemEffects[battler] = 0;
}
pub(crate) unsafe fn Cmd_if_random_less_than() {
    let random: u16 = Random();
    if random as i32 % 256 < *(*gAIScriptPtr.as_ptr()).at(1) as i32 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_if_random_greater_than() {
    let random: u16 = Random();
    if random as i32 % 256 > *(*gAIScriptPtr.as_ptr()).at(1) as i32 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_if_random_equal() {
    let random: u16 = Random();
    if random as i32 % 256 == *(*gAIScriptPtr.as_ptr()).at(1) as i32 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_if_random_not_equal() {
    let random: u16 = Random();
    if random as i32 % 256 != *(*gAIScriptPtr.as_ptr()).at(1) as i32 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_score() {
    (*(*crate::gl!(gBattleResources)).ai).score
        [(*(*crate::gl!(gBattleResources)).ai).movesetIndex] +=
        *(*gAIScriptPtr.as_ptr()).at(1) as i8;
    if (*(*crate::gl!(gBattleResources)).ai).score
        [(*(*crate::gl!(gBattleResources)).ai).movesetIndex]
        < 0
    {
        (*(*crate::gl!(gBattleResources)).ai).score
            [(*(*crate::gl!(gBattleResources)).ai).movesetIndex] = 0;
    }
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_if_hp_less_than() {
    let mut battler: u16 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get() as u16;
    } else {
        battler = gBattlerTarget.get() as u16;
    }
    if (div_i32(
        100 * crate::gl!(gBattleMons[battler].hp) as i32,
        crate::gl!(gBattleMons[battler].maxHP) as i32,
    ) as u32)
        < *(*gAIScriptPtr.as_ptr()).at(2) as u32
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(3) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(7));
    }
}
pub(crate) unsafe fn Cmd_if_hp_more_than() {
    let mut battler: u16 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get() as u16;
    } else {
        battler = gBattlerTarget.get() as u16;
    }
    if div_i32(
        100 * crate::gl!(gBattleMons[battler].hp) as i32,
        crate::gl!(gBattleMons[battler].maxHP) as i32,
    ) as u32
        > *(*gAIScriptPtr.as_ptr()).at(2) as u32
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(3) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(7));
    }
}
pub(crate) unsafe fn Cmd_if_hp_equal() {
    let mut battler: u16 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get() as u16;
    } else {
        battler = gBattlerTarget.get() as u16;
    }
    if div_i32(
        100 * crate::gl!(gBattleMons[battler].hp) as i32,
        crate::gl!(gBattleMons[battler].maxHP) as i32,
    ) as u32
        == *(*gAIScriptPtr.as_ptr()).at(2) as u32
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(3) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(7));
    }
}
pub(crate) unsafe fn Cmd_if_hp_not_equal() {
    let mut battler: u16 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get() as u16;
    } else {
        battler = gBattlerTarget.get() as u16;
    }
    if div_i32(
        100 * crate::gl!(gBattleMons[battler].hp) as i32,
        crate::gl!(gBattleMons[battler].maxHP) as i32,
    ) as u32
        != *(*gAIScriptPtr.as_ptr()).at(2) as u32
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(3) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(7));
    }
}
pub(crate) unsafe fn Cmd_if_status() {
    let mut battler: u16 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get() as u16;
    } else {
        battler = gBattlerTarget.get() as u16;
    }
    let status: u32 = *(*gAIScriptPtr.as_ptr()).at(2) as u32
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as u32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as u32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as u32) << 24;
    if crate::gl!(gBattleMons[battler].status1) & status != 0 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(6) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(10));
    }
}
pub(crate) unsafe fn Cmd_if_not_status() {
    let mut battler: u16 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get() as u16;
    } else {
        battler = gBattlerTarget.get() as u16;
    }
    let status: u32 = *(*gAIScriptPtr.as_ptr()).at(2) as u32
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as u32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as u32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as u32) << 24;
    if crate::gl!(gBattleMons[battler].status1) & status == 0 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(6) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(10));
    }
}
pub(crate) unsafe fn Cmd_if_status2() {
    let mut battler: u16 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get() as u16;
    } else {
        battler = gBattlerTarget.get() as u16;
    }
    let status: u32 = *(*gAIScriptPtr.as_ptr()).at(2) as u32
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as u32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as u32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as u32) << 24;
    if crate::gl!(gBattleMons[battler].status2) & status != 0 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(6) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(10));
    }
}
pub(crate) unsafe fn Cmd_if_not_status2() {
    let mut battler: u16 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get() as u16;
    } else {
        battler = gBattlerTarget.get() as u16;
    }
    let status: u32 = *(*gAIScriptPtr.as_ptr()).at(2) as u32
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as u32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as u32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as u32) << 24;
    if crate::gl!(gBattleMons[battler].status2) & status == 0 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(6) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(10));
    }
}
pub(crate) unsafe fn Cmd_if_status3() {
    let mut battler: u16 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get() as u16;
    } else {
        battler = gBattlerTarget.get() as u16;
    }
    let status: u32 = *(*gAIScriptPtr.as_ptr()).at(2) as u32
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as u32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as u32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as u32) << 24;
    if crate::gl!(gStatuses3[battler]) & status != 0 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(6) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(10));
    }
}
pub(crate) unsafe fn Cmd_if_not_status3() {
    let mut battler: u16 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get() as u16;
    } else {
        battler = gBattlerTarget.get() as u16;
    }
    let status: u32 = *(*gAIScriptPtr.as_ptr()).at(2) as u32
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as u32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as u32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as u32) << 24;
    if crate::gl!(gStatuses3[battler]) & status == 0 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(6) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(10));
    }
}
pub(crate) unsafe fn Cmd_if_side_affecting() {
    let mut battler: u16 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get() as u16;
    } else {
        battler = gBattlerTarget.get() as u16;
    }
    let side: u32 = GetBattlerPosition(battler as u8) as u32 & 1;
    let status: u32 = *(*gAIScriptPtr.as_ptr()).at(2) as u32
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as u32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as u32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as u32) << 24;
    if crate::gl!(gSideStatuses[side]) as u32 & status != 0 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(6) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(10));
    }
}
pub(crate) unsafe fn Cmd_if_not_side_affecting() {
    let mut battler: u16 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get() as u16;
    } else {
        battler = gBattlerTarget.get() as u16;
    }
    let side: u32 = GetBattlerPosition(battler as u8) as u32 & 1;
    let status: u32 = *(*gAIScriptPtr.as_ptr()).at(2) as u32
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as u32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as u32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as u32) << 24;
    if crate::gl!(gSideStatuses[side]) as u32 & status == 0 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(6) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(6).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(10));
    }
}
pub(crate) unsafe fn Cmd_if_less_than() {
    if (*(*crate::gl!(gBattleResources)).ai).funcResult < *(*gAIScriptPtr.as_ptr()).at(1) as u32 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_if_more_than() {
    if (*(*crate::gl!(gBattleResources)).ai).funcResult > *(*gAIScriptPtr.as_ptr()).at(1) as u32 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_if_equal() {
    if (*(*crate::gl!(gBattleResources)).ai).funcResult == *(*gAIScriptPtr.as_ptr()).at(1) as u32 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_if_not_equal() {
    if (*(*crate::gl!(gBattleResources)).ai).funcResult != *(*gAIScriptPtr.as_ptr()).at(1) as u32 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_if_less_than_ptr() {
    let value: *mut u8 = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24) as usize
        as *mut u8;
    if (*(*crate::gl!(gBattleResources)).ai).funcResult < *value as u32 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(5) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(5).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(5).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(5).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(9));
    }
}
pub(crate) unsafe fn Cmd_if_more_than_ptr() {
    let value: *mut u8 = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24) as usize
        as *mut u8;
    if (*(*crate::gl!(gBattleResources)).ai).funcResult > *value as u32 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(5) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(5).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(5).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(5).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(9));
    }
}
pub(crate) unsafe fn Cmd_if_equal_ptr() {
    let value: *mut u8 = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24) as usize
        as *mut u8;
    if (*(*crate::gl!(gBattleResources)).ai).funcResult == *value as u32 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(5) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(5).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(5).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(5).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(9));
    }
}
pub(crate) unsafe fn Cmd_if_not_equal_ptr() {
    let value: *mut u8 = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24) as usize
        as *mut u8;
    if (*(*crate::gl!(gBattleResources)).ai).funcResult != *value as u32 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(5) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(5).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(5).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(5).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(9));
    }
}
pub(crate) unsafe fn Cmd_if_move() {
    let r#move: u16 = *(*gAIScriptPtr.as_ptr()).at(1) as u16
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as u16) << 8;
    if (*(*crate::gl!(gBattleResources)).ai).moveConsidered == r#move {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(3) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(7));
    }
}
pub(crate) unsafe fn Cmd_if_not_move() {
    let r#move: u16 = *(*gAIScriptPtr.as_ptr()).at(1) as u16
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as u16) << 8;
    if (*(*crate::gl!(gBattleResources)).ai).moveConsidered != r#move {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(3) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(7));
    }
}
pub(crate) unsafe fn Cmd_if_in_bytes() {
    let mut ptr: *mut u8 = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24) as usize
        as *mut u8;
    while *ptr != 0xFF {
        if (*(*crate::gl!(gBattleResources)).ai).funcResult == *ptr as u32 {
            crate::gl!(
                gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(5) as i32
                    | (*(*gAIScriptPtr.as_ptr()).at(5).at(1) as i32) << 8
                    | (*(*gAIScriptPtr.as_ptr()).at(5).at(2) as i32) << 16
                    | (*(*gAIScriptPtr.as_ptr()).at(5).at(3) as i32) << 24)
                    as usize as *mut u8
            );
            return;
        }
        ptr = ptr.at(1);
    }
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(9));
}
pub(crate) unsafe fn Cmd_if_not_in_bytes() {
    let mut ptr: *mut u8 = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24) as usize
        as *mut u8;
    while *ptr != 0xFF {
        if (*(*crate::gl!(gBattleResources)).ai).funcResult == *ptr as u32 {
            crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(9));
            return;
        }
        ptr = ptr.at(1);
    }
    crate::gl!(
        gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(5) as i32
            | (*(*gAIScriptPtr.as_ptr()).at(5).at(1) as i32) << 8
            | (*(*gAIScriptPtr.as_ptr()).at(5).at(2) as i32) << 16
            | (*(*gAIScriptPtr.as_ptr()).at(5).at(3) as i32) << 24) as usize
            as *mut u8
    );
}
pub(crate) unsafe fn Cmd_if_in_hwords() {
    let mut ptr: *mut u16 = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24) as usize
        as *mut u8 as *mut u16;
    while *ptr != 0xFFFF {
        if (*(*crate::gl!(gBattleResources)).ai).funcResult == *ptr as u32 {
            crate::gl!(
                gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(5) as i32
                    | (*(*gAIScriptPtr.as_ptr()).at(5).at(1) as i32) << 8
                    | (*(*gAIScriptPtr.as_ptr()).at(5).at(2) as i32) << 16
                    | (*(*gAIScriptPtr.as_ptr()).at(5).at(3) as i32) << 24)
                    as usize as *mut u8
            );
            return;
        }
        ptr = ptr.at(1);
    }
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(9));
}
pub(crate) unsafe fn Cmd_if_not_in_hwords() {
    let mut ptr: *mut u16 = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24) as usize
        as *mut u8 as *mut u16;
    while *ptr != 0xFFFF {
        if (*(*crate::gl!(gBattleResources)).ai).funcResult == *ptr as u32 {
            crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(9));
            return;
        }
        ptr = ptr.at(1);
    }
    crate::gl!(
        gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(5) as i32
            | (*(*gAIScriptPtr.as_ptr()).at(5).at(1) as i32) << 8
            | (*(*gAIScriptPtr.as_ptr()).at(5).at(2) as i32) << 16
            | (*(*gAIScriptPtr.as_ptr()).at(5).at(3) as i32) << 24) as usize
            as *mut u8
    );
}
pub(crate) unsafe fn Cmd_if_user_has_attacking_move() {
    let mut i: i32 = 0;
    while i < MAX_MON_MOVES {
        if ({
            let gl_i6 = sBattler_AI.get();
            crate::gl!(gBattleMons[gl_i6].moves[i])
        }) != 0
            && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[{
                let gl_i7 = sBattler_AI.get();
                crate::gl!(gBattleMons[gl_i7].moves[i])
            }]
            .power
                != 0
        {
            break;
        }
        i += 1;
    }
    if i == MAX_MON_MOVES {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    } else {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_if_user_has_no_attacking_moves() {
    let mut i: i32 = 0;
    while i < MAX_MON_MOVES {
        if ({
            let gl_i8 = sBattler_AI.get();
            crate::gl!(gBattleMons[gl_i8].moves[i])
        }) != 0
            && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[{
                let gl_i9 = sBattler_AI.get();
                crate::gl!(gBattleMons[gl_i9].moves[i])
            }]
            .power
                != 0
        {
            break;
        }
        i += 1;
    }
    if i != MAX_MON_MOVES {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    } else {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_get_turn_count() {
    (*(*crate::gl!(gBattleResources)).ai).funcResult =
        crate::gl!(gBattleResults.battleTurnCounter) as u32;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_get_type() {
    let typeVar: u8 = *(*gAIScriptPtr.as_ptr()).at(1);
    match typeVar {
        AI_TYPE1_USER => {
            (*(*crate::gl!(gBattleResources)).ai).funcResult = ({
                let gl_i10 = sBattler_AI.get();
                crate::gl!(gBattleMons[gl_i10].types[0])
            }) as u32;
        }
        AI_TYPE1_TARGET => {
            (*(*crate::gl!(gBattleResources)).ai).funcResult = ({
                let gl_i11 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i11].types[0])
            }) as u32;
        }
        AI_TYPE2_USER => {
            (*(*crate::gl!(gBattleResources)).ai).funcResult = ({
                let gl_i12 = sBattler_AI.get();
                crate::gl!(gBattleMons[gl_i12].types[1])
            }) as u32;
        }
        AI_TYPE2_TARGET => {
            (*(*crate::gl!(gBattleResources)).ai).funcResult = ({
                let gl_i13 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i13].types[1])
            }) as u32;
        }
        AI_TYPE_MOVE => {
            (*(*crate::gl!(gBattleResources)).ai).funcResult =
                (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                    [(*(*crate::gl!(gBattleResources)).ai).moveConsidered]
                    .r#type as u32;
        }
        _ => {}
    }
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
unsafe fn BattleAI_GetWantedBattler(wantedBattler: u8) -> u8 {
    match wantedBattler {
        AI_USER => {
            return sBattler_AI.get();
        }
        AI_USER_PARTNER => {
            return sBattler_AI.get() ^ 2;
        }
        AI_TARGET_PARTNER => {
            return gBattlerTarget.get() ^ 2;
        }
        _ => {
            return gBattlerTarget.get();
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn Cmd_is_of_type() {
    let battler: u8 = BattleAI_GetWantedBattler(*(*gAIScriptPtr.as_ptr()).at(1));
    if crate::gl!(gBattleMons[battler].types[0]) == *(*gAIScriptPtr.as_ptr()).at(2)
        || crate::gl!(gBattleMons[battler].types[1]) == *(*gAIScriptPtr.as_ptr()).at(2)
    {
        (*(*crate::gl!(gBattleResources)).ai).funcResult = TRUE as u32;
    } else {
        (*(*crate::gl!(gBattleResources)).ai).funcResult = FALSE as u32;
    }
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(3));
}
pub(crate) unsafe fn Cmd_get_considered_move_power() {
    (*(*crate::gl!(gBattleResources)).ai).funcResult =
        (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [(*(*crate::gl!(gBattleResources)).ai).moveConsidered]
            .power as u32;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_get_how_powerful_move_is() {
    let mut checkedMove: i32 = 0;
    let mut moveDmgs: CArray<i32, 4> = zeroed();
    let mut i: i32 = 0;
    while sIgnoredPowerfulMoveEffects[i] != IGNORED_MOVES_END {
        if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [(*(*crate::gl!(gBattleResources)).ai).moveConsidered]
            .effect as u16
            == sIgnoredPowerfulMoveEffects[i]
        {
            break;
        }
        i += 1;
    }
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [(*(*crate::gl!(gBattleResources)).ai).moveConsidered]
        .power
        > 1
        && sIgnoredPowerfulMoveEffects[i] == IGNORED_MOVES_END
    {
        gDynamicBasePower.set(0);
        (*crate::gl!(gBattleStruct)).dynamicMoveType = 0;
        crate::gl!(gBattleScripting.dmgMultiplier = 1);
        gMoveResultFlags.set(0);
        gCritMultiplier.set(1);
        for checkedMove in 0..MAX_MON_MOVES {
            i = 0;
            while sIgnoredPowerfulMoveEffects[i] != IGNORED_MOVES_END {
                if (*(&raw const crate::data::pokemon::gBattleMoves)
                    .cast::<CArray<BattleMove, 0>>())[{
                    let gl_i14 = sBattler_AI.get();
                    crate::gl!(gBattleMons[gl_i14].moves[checkedMove])
                }]
                .effect as u16
                    == sIgnoredPowerfulMoveEffects[i]
                {
                    break;
                }
                i += 1;
            }
            if ({
                let gl_i15 = sBattler_AI.get();
                crate::gl!(gBattleMons[gl_i15].moves[checkedMove])
            }) != MOVE_NONE
                && sIgnoredPowerfulMoveEffects[i] == IGNORED_MOVES_END
                && (*(&raw const crate::data::pokemon::gBattleMoves)
                    .cast::<CArray<BattleMove, 0>>())[{
                    let gl_i16 = sBattler_AI.get();
                    crate::gl!(gBattleMons[gl_i16].moves[checkedMove])
                }]
                .power
                    > 1
            {
                gCurrentMove.set({
                    let gl_i17 = sBattler_AI.get();
                    crate::gl!(gBattleMons[gl_i17].moves[checkedMove])
                });
                AI_CalcDmg(sBattler_AI.get(), gBattlerTarget.get());
                TypeCalc(gCurrentMove.get(), sBattler_AI.get(), gBattlerTarget.get());
                moveDmgs[checkedMove] = gBattleMoveDamage.get()
                    * (*(*crate::gl!(gBattleResources)).ai).simulatedRNG[checkedMove] as i32
                    / 100;
                if moveDmgs[checkedMove] == 0 {
                    moveDmgs[checkedMove] = 1;
                }
            } else {
                moveDmgs[checkedMove] = 0;
            }
        }
        checkedMove = 0;
        while checkedMove < MAX_MON_MOVES {
            if moveDmgs[checkedMove] > moveDmgs[(*(*crate::gl!(gBattleResources)).ai).movesetIndex]
            {
                break;
            }
            checkedMove += 1;
        }
        if checkedMove == MAX_MON_MOVES {
            (*(*crate::gl!(gBattleResources)).ai).funcResult = MOVE_MOST_POWERFUL;
        } else {
            (*(*crate::gl!(gBattleResources)).ai).funcResult = MOVE_NOT_MOST_POWERFUL;
        }
    } else {
        (*(*crate::gl!(gBattleResources)).ai).funcResult = MOVE_POWER_OTHER;
    }
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_get_last_used_battler_move() {
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        (*(*crate::gl!(gBattleResources)).ai).funcResult = {
            let gl_i2 = sBattler_AI.get();
            crate::gl!(gLastMoves[gl_i2])
        } as u32;
    } else {
        (*(*crate::gl!(gBattleResources)).ai).funcResult = {
            let gl_i3 = gBattlerTarget.get();
            crate::gl!(gLastMoves[gl_i3])
        } as u32;
    }
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_if_equal_() {
    if *(*gAIScriptPtr.as_ptr()).at(1) as u32 == (*(*crate::gl!(gBattleResources)).ai).funcResult {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_if_not_equal_() {
    if *(*gAIScriptPtr.as_ptr()).at(1) as u32 != (*(*crate::gl!(gBattleResources)).ai).funcResult {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_if_user_goes() {
    if GetWhoStrikesFirst(sBattler_AI.get(), gBattlerTarget.get(), 1)
        == *(*gAIScriptPtr.as_ptr()).at(1)
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_if_user_doesnt_go() {
    if GetWhoStrikesFirst(sBattler_AI.get(), gBattlerTarget.get(), 1)
        != *(*gAIScriptPtr.as_ptr()).at(1)
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) fn Cmd_nop_2A() {}
pub(crate) fn Cmd_nop_2B() {}
pub(crate) unsafe fn Cmd_count_usable_party_mons() {
    let mut battler: u8 = 0;
    let mut battlerOnField1: u8 = 0;
    let mut battlerOnField2: u8 = 0;
    let mut party: *mut Pokemon = null_mut();
    (*(*crate::gl!(gBattleResources)).ai).funcResult = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get();
    } else {
        battler = gBattlerTarget.get();
    }
    if GetBattlerSide(battler) == B_SIDE_PLAYER {
        party = (*gPlayerParty.as_ptr()).as_mut_ptr();
    } else {
        party = (*gEnemyParty.as_ptr()).as_mut_ptr();
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
        battlerOnField1 = crate::gl!(gBattlerPartyIndexes[battler]) as u8;
        let position: u32 = GetBattlerPosition(battler) as u32 ^ 2;
        battlerOnField2 = ({
            let gl_i18 = GetBattlerAtPosition(position as u8);
            crate::gl!(gBattlerPartyIndexes[gl_i18])
        }) as u8;
    } else {
        battlerOnField1 = crate::gl!(gBattlerPartyIndexes[battler]) as u8;
        battlerOnField2 = crate::gl!(gBattlerPartyIndexes[battler]) as u8;
    }
    for i in 0..PARTY_SIZE {
        if i != battlerOnField1 as i32
            && i != battlerOnField2 as i32
            && GetMonData2(party.at(i), MON_DATA_HP) != 0
            && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_NONE as u32
            && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_EGG
        {
            (*(*crate::gl!(gBattleResources)).ai).funcResult += 1;
        }
    }
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_get_considered_move() {
    (*(*crate::gl!(gBattleResources)).ai).funcResult =
        (*(*crate::gl!(gBattleResources)).ai).moveConsidered as u32;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_get_considered_move_effect() {
    (*(*crate::gl!(gBattleResources)).ai).funcResult =
        (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [(*(*crate::gl!(gBattleResources)).ai).moveConsidered]
            .effect as u32;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_get_ability() {
    let mut battler: u8 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get();
    } else {
        battler = gBattlerTarget.get();
    }
    if gActiveBattler.get() != battler {
        if (*(*crate::gl!(gBattleResources)).battleHistory).abilities[battler] != 0 {
            (*(*crate::gl!(gBattleResources)).ai).funcResult =
                (*(*crate::gl!(gBattleResources)).battleHistory).abilities[battler] as u32;
            crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
            return;
        }
        if crate::gl!(gBattleMons[battler].ability) == ABILITY_SHADOW_TAG
            || crate::gl!(gBattleMons[battler].ability) == ABILITY_MAGNET_PULL
            || crate::gl!(gBattleMons[battler].ability) == ABILITY_ARENA_TRAP
        {
            (*(*crate::gl!(gBattleResources)).ai).funcResult =
                crate::gl!(gBattleMons[battler].ability) as u32;
            crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
            return;
        }
        if (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
            [crate::gl!(gBattleMons[battler].species)]
        .abilities[0]
            != 0
        {
            if (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [crate::gl!(gBattleMons[battler].species)]
            .abilities[1]
                != ABILITY_NONE
            {
                if Random() as i32 & 1 != 0 {
                    (*(*crate::gl!(gBattleResources)).ai).funcResult =
                        (*(&raw const crate::data::pokemon::gSpeciesInfo)
                            .cast::<CArray<SpeciesInfo, 0>>())
                            [crate::gl!(gBattleMons[battler].species)]
                        .abilities[0] as u32;
                } else {
                    (*(*crate::gl!(gBattleResources)).ai).funcResult =
                        (*(&raw const crate::data::pokemon::gSpeciesInfo)
                            .cast::<CArray<SpeciesInfo, 0>>())
                            [crate::gl!(gBattleMons[battler].species)]
                        .abilities[1] as u32;
                }
            } else {
                (*(*crate::gl!(gBattleResources)).ai).funcResult =
                    (*(&raw const crate::data::pokemon::gSpeciesInfo)
                        .cast::<CArray<SpeciesInfo, 0>>())[crate::gl!(gBattleMons[battler].species)]
                    .abilities[0] as u32;
            }
        } else {
            (*(*crate::gl!(gBattleResources)).ai).funcResult =
                (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                    [crate::gl!(gBattleMons[battler].species)]
                .abilities[1] as u32;
        }
    } else {
        (*(*crate::gl!(gBattleResources)).ai).funcResult =
            crate::gl!(gBattleMons[battler].ability) as u32;
    }
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_check_ability() {
    let battler: u32 = BattleAI_GetWantedBattler(*(*gAIScriptPtr.as_ptr()).at(1)) as u32;
    let mut ability: u32 = *(*gAIScriptPtr.as_ptr()).at(2) as u32;
    if *(*gAIScriptPtr.as_ptr()).at(1) == AI_TARGET
        || *(*gAIScriptPtr.as_ptr()).at(1) == AI_TARGET_PARTNER
    {
        if (*(*crate::gl!(gBattleResources)).battleHistory).abilities[battler] != ABILITY_NONE {
            ability = (*(*crate::gl!(gBattleResources)).battleHistory).abilities[battler] as u32;
            (*(*crate::gl!(gBattleResources)).ai).funcResult = ability;
        } else if crate::gl!(gBattleMons[battler].ability) == ABILITY_SHADOW_TAG
            || crate::gl!(gBattleMons[battler].ability) == ABILITY_MAGNET_PULL
            || crate::gl!(gBattleMons[battler].ability) == ABILITY_ARENA_TRAP
        {
            ability = crate::gl!(gBattleMons[battler].ability) as u32;
        } else if (*(&raw const crate::data::pokemon::gSpeciesInfo)
            .cast::<CArray<SpeciesInfo, 0>>())[crate::gl!(gBattleMons[battler].species)]
        .abilities[0]
            != 0
        {
            if (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [crate::gl!(gBattleMons[battler].species)]
            .abilities[1]
                != ABILITY_NONE
            {
                let abilityDummyVariable: u8 = ability as u8;
                if (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[crate::gl!(gBattleMons[battler].species)]
                .abilities[0]
                    != abilityDummyVariable
                    && (*(&raw const crate::data::pokemon::gSpeciesInfo)
                        .cast::<CArray<SpeciesInfo, 0>>())[crate::gl!(gBattleMons[battler].species)]
                    .abilities[1]
                        != abilityDummyVariable
                {
                    ability =
                        (*(&raw const crate::data::pokemon::gSpeciesInfo)
                            .cast::<CArray<SpeciesInfo, 0>>())
                            [crate::gl!(gBattleMons[battler].species)]
                        .abilities[0] as u32;
                } else {
                    ability = ABILITY_NONE as u32;
                }
            } else {
                ability = (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[crate::gl!(gBattleMons[battler].species)]
                .abilities[0] as u32;
            }
        } else {
            ability = (*(&raw const crate::data::pokemon::gSpeciesInfo)
                .cast::<CArray<SpeciesInfo, 0>>())[crate::gl!(gBattleMons[battler].species)]
            .abilities[1] as u32;
        }
    } else {
        ability = crate::gl!(gBattleMons[battler].ability) as u32;
    }
    if ability == 0 {
        (*(*crate::gl!(gBattleResources)).ai).funcResult = 2;
    } else if ability == *(*gAIScriptPtr.as_ptr()).at(2) as u32 {
        (*(*crate::gl!(gBattleResources)).ai).funcResult = 1;
    } else {
        (*(*crate::gl!(gBattleResources)).ai).funcResult = 0;
    }
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(3));
}
pub(crate) unsafe fn Cmd_get_highest_type_effectiveness() {
    let mut dynamicMoveType: *mut u8 = null_mut();
    gDynamicBasePower.set(0);
    dynamicMoveType = &raw mut (*crate::gl!(gBattleStruct)).dynamicMoveType;
    *dynamicMoveType = 0;
    crate::gl!(gBattleScripting.dmgMultiplier = 1);
    gMoveResultFlags.set(0);
    gCritMultiplier.set(1);
    (*(*crate::gl!(gBattleResources)).ai).funcResult = 0;
    for i in 0..MAX_MON_MOVES {
        gBattleMoveDamage.set(40);
        gCurrentMove.set({
            let gl_i19 = sBattler_AI.get();
            crate::gl!(gBattleMons[gl_i19].moves[i])
        });
        if gCurrentMove.get() != MOVE_NONE {
            TypeCalc(gCurrentMove.get(), sBattler_AI.get(), gBattlerTarget.get());
            if gBattleMoveDamage.get() == 120 {
                gBattleMoveDamage.set(AI_EFFECTIVENESS_x2);
            }
            if gBattleMoveDamage.get() == 240 {
                gBattleMoveDamage.set(AI_EFFECTIVENESS_x4);
            }
            if gBattleMoveDamage.get() == 30 {
                gBattleMoveDamage.set(AI_EFFECTIVENESS_x0_5);
            }
            if gBattleMoveDamage.get() == 15 {
                gBattleMoveDamage.set(AI_EFFECTIVENESS_x0_25);
            }
            if gMoveResultFlags.get() as i32 & MOVE_RESULT_DOESNT_AFFECT_FOE as i32 != 0 {
                gBattleMoveDamage.set(AI_EFFECTIVENESS_x0);
            }
            if (*(*crate::gl!(gBattleResources)).ai).funcResult < gBattleMoveDamage.get() as u32 {
                (*(*crate::gl!(gBattleResources)).ai).funcResult = gBattleMoveDamage.get() as u32;
            }
        }
    }
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_if_type_effectiveness() {
    gDynamicBasePower.set(0);
    (*crate::gl!(gBattleStruct)).dynamicMoveType = 0;
    crate::gl!(gBattleScripting.dmgMultiplier = 1);
    gMoveResultFlags.set(0);
    gCritMultiplier.set(1);
    gBattleMoveDamage.set(AI_EFFECTIVENESS_x1);
    gCurrentMove.set((*(*crate::gl!(gBattleResources)).ai).moveConsidered);
    TypeCalc(gCurrentMove.get(), sBattler_AI.get(), gBattlerTarget.get());
    if gBattleMoveDamage.get() == 120 {
        gBattleMoveDamage.set(AI_EFFECTIVENESS_x2);
    }
    if gBattleMoveDamage.get() == 240 {
        gBattleMoveDamage.set(AI_EFFECTIVENESS_x4);
    }
    if gBattleMoveDamage.get() == 30 {
        gBattleMoveDamage.set(AI_EFFECTIVENESS_x0_5);
    }
    if gBattleMoveDamage.get() == 15 {
        gBattleMoveDamage.set(AI_EFFECTIVENESS_x0_25);
    }
    if gMoveResultFlags.get() as i32 & MOVE_RESULT_DOESNT_AFFECT_FOE as i32 != 0 {
        gBattleMoveDamage.set(AI_EFFECTIVENESS_x0);
    }
    let damageVar: u8 = gBattleMoveDamage.get() as u8;
    if damageVar == *(*gAIScriptPtr.as_ptr()).at(1) {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) fn Cmd_nop_32() {}
pub(crate) fn Cmd_nop_33() {}
pub(crate) unsafe fn Cmd_if_status_in_party() {
    let mut battler: u8 = 0;
    match *(*gAIScriptPtr.as_ptr()).at(1) {
        AI_USER => {
            battler = sBattler_AI.get();
        }
        _ => {
            battler = gBattlerTarget.get();
        }
    }
    let party: *mut Pokemon = if GetBattlerSide(battler) == B_SIDE_PLAYER {
        (*gPlayerParty.as_ptr()).as_mut_ptr()
    } else {
        (*gEnemyParty.as_ptr()).as_mut_ptr()
    };
    let statusToCompareTo: u32 = *(*gAIScriptPtr.as_ptr()).at(2) as u32
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as u32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as u32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as u32) << 24;
    for i in 0..PARTY_SIZE {
        let species: u16 = GetMonData2(party.at(i), MON_DATA_SPECIES) as u16;
        let hp: u16 = GetMonData2(party.at(i), MON_DATA_HP) as u16;
        let status: u32 = GetMonData2(party.at(i), MON_DATA_STATUS);
        if species != 0 && species != SPECIES_EGG as u16 && hp != 0 && status == statusToCompareTo {
            crate::gl!(
                gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(6) as i32
                    | (*(*gAIScriptPtr.as_ptr()).at(6).at(1) as i32) << 8
                    | (*(*gAIScriptPtr.as_ptr()).at(6).at(2) as i32) << 16
                    | (*(*gAIScriptPtr.as_ptr()).at(6).at(3) as i32) << 24)
                    as usize as *mut u8
            );
            return;
        }
    }
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(10));
}
pub(crate) unsafe fn Cmd_if_status_not_in_party() {
    let mut battler: u8 = 0;
    match *(*gAIScriptPtr.as_ptr()).at(1) {
        1 => {
            battler = sBattler_AI.get();
        }
        _ => {
            battler = gBattlerTarget.get();
        }
    }
    let party: *mut Pokemon = if GetBattlerSide(battler) == B_SIDE_PLAYER {
        (*gPlayerParty.as_ptr()).as_mut_ptr()
    } else {
        (*gEnemyParty.as_ptr()).as_mut_ptr()
    };
    let statusToCompareTo: u32 = *(*gAIScriptPtr.as_ptr()).at(2) as u32
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as u32) << 8
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as u32) << 16
        | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as u32) << 24;
    for i in 0..PARTY_SIZE {
        let species: u16 = GetMonData2(party.at(i), MON_DATA_SPECIES) as u16;
        let hp: u16 = GetMonData2(party.at(i), MON_DATA_HP) as u16;
        let status: u32 = GetMonData2(party.at(i), MON_DATA_STATUS);
        if species != 0 && species != SPECIES_EGG as u16 && hp != 0 && status == statusToCompareTo {
            crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(10));
            return;
        }
    }
    crate::gl!(
        gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(6) as i32
            | (*(*gAIScriptPtr.as_ptr()).at(6).at(1) as i32) << 8
            | (*(*gAIScriptPtr.as_ptr()).at(6).at(2) as i32) << 16
            | (*(*gAIScriptPtr.as_ptr()).at(6).at(3) as i32) << 24) as usize
            as *mut u8
    );
}
pub(crate) unsafe fn Cmd_get_weather() {
    if gBattleWeather.get() as i32 & B_WEATHER_RAIN != 0 {
        (*(*crate::gl!(gBattleResources)).ai).funcResult = AI_WEATHER_RAIN;
    }
    if gBattleWeather.get() as i32 & B_WEATHER_SANDSTORM != 0 {
        (*(*crate::gl!(gBattleResources)).ai).funcResult = AI_WEATHER_SANDSTORM;
    }
    if gBattleWeather.get() as i32 & B_WEATHER_SUN != 0 {
        (*(*crate::gl!(gBattleResources)).ai).funcResult = AI_WEATHER_SUN;
    }
    if gBattleWeather.get() as i32 & B_WEATHER_HAIL != 0 {
        (*(*crate::gl!(gBattleResources)).ai).funcResult = AI_WEATHER_HAIL;
    }
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_if_effect() {
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [(*(*crate::gl!(gBattleResources)).ai).moveConsidered]
        .effect
        == *(*gAIScriptPtr.as_ptr()).at(1)
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_if_not_effect() {
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [(*(*crate::gl!(gBattleResources)).ai).moveConsidered]
        .effect
        != *(*gAIScriptPtr.as_ptr()).at(1)
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_if_stat_level_less_than() {
    let mut battler: u32 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get() as u32;
    } else {
        battler = gBattlerTarget.get() as u32;
    }
    if (({
        let gl_i20 = *(*gAIScriptPtr.as_ptr()).at(2);
        crate::gl!(gBattleMons[battler].statStages[gl_i20])
    }) as i32)
        < *(*gAIScriptPtr.as_ptr()).at(3) as i32
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(4) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(8));
    }
}
pub(crate) unsafe fn Cmd_if_stat_level_more_than() {
    let mut battler: u32 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get() as u32;
    } else {
        battler = gBattlerTarget.get() as u32;
    }
    if ({
        let gl_i21 = *(*gAIScriptPtr.as_ptr()).at(2);
        crate::gl!(gBattleMons[battler].statStages[gl_i21])
    }) as i32
        > *(*gAIScriptPtr.as_ptr()).at(3) as i32
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(4) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(8));
    }
}
pub(crate) unsafe fn Cmd_if_stat_level_equal() {
    let mut battler: u32 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get() as u32;
    } else {
        battler = gBattlerTarget.get() as u32;
    }
    if ({
        let gl_i22 = *(*gAIScriptPtr.as_ptr()).at(2);
        crate::gl!(gBattleMons[battler].statStages[gl_i22])
    }) as i32
        == *(*gAIScriptPtr.as_ptr()).at(3) as i32
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(4) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(8));
    }
}
pub(crate) unsafe fn Cmd_if_stat_level_not_equal() {
    let mut battler: u32 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get() as u32;
    } else {
        battler = gBattlerTarget.get() as u32;
    }
    if ({
        let gl_i23 = *(*gAIScriptPtr.as_ptr()).at(2);
        crate::gl!(gBattleMons[battler].statStages[gl_i23])
    }) as i32
        != *(*gAIScriptPtr.as_ptr()).at(3) as i32
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(4) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(8));
    }
}
pub(crate) unsafe fn Cmd_if_can_faint() {
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [(*(*crate::gl!(gBattleResources)).ai).moveConsidered]
        .power
        < 2
    {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
        return;
    }
    gDynamicBasePower.set(0);
    (*crate::gl!(gBattleStruct)).dynamicMoveType = 0;
    crate::gl!(gBattleScripting.dmgMultiplier = 1);
    gMoveResultFlags.set(0);
    gCritMultiplier.set(1);
    gCurrentMove.set((*(*crate::gl!(gBattleResources)).ai).moveConsidered);
    AI_CalcDmg(sBattler_AI.get(), gBattlerTarget.get());
    TypeCalc(gCurrentMove.get(), sBattler_AI.get(), gBattlerTarget.get());
    gBattleMoveDamage.set(
        gBattleMoveDamage.get()
            * (*(*crate::gl!(gBattleResources)).ai).simulatedRNG
                [(*(*crate::gl!(gBattleResources)).ai).movesetIndex] as i32
            / 100,
    );
    if gBattleMoveDamage.get() == 0 {
        gBattleMoveDamage.set(1);
    }
    if ({
        let gl_i24 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i24].hp)
    }) as i32
        <= gBattleMoveDamage.get()
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_if_cant_faint() {
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [(*(*crate::gl!(gBattleResources)).ai).moveConsidered]
        .power
        < 2
    {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
        return;
    }
    gDynamicBasePower.set(0);
    (*crate::gl!(gBattleStruct)).dynamicMoveType = 0;
    crate::gl!(gBattleScripting.dmgMultiplier = 1);
    gMoveResultFlags.set(0);
    gCritMultiplier.set(1);
    gCurrentMove.set((*(*crate::gl!(gBattleResources)).ai).moveConsidered);
    AI_CalcDmg(sBattler_AI.get(), gBattlerTarget.get());
    TypeCalc(gCurrentMove.get(), sBattler_AI.get(), gBattlerTarget.get());
    gBattleMoveDamage.set(
        gBattleMoveDamage.get()
            * (*(*crate::gl!(gBattleResources)).ai).simulatedRNG
                [(*(*crate::gl!(gBattleResources)).ai).movesetIndex] as i32
            / 100,
    );
    if ({
        let gl_i25 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i25].hp)
    }) as i32
        > gBattleMoveDamage.get()
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_if_has_move() {
    let mut i: i32 = 0;
    let movePtr: *mut u16 = (*gAIScriptPtr.as_ptr()).at(2) as *mut u16;
    'l1: {
        match *(*gAIScriptPtr.as_ptr()).at(1) {
            AI_USER => {
                i = 0;
                while i < MAX_MON_MOVES {
                    if ({
                        let gl_i26 = sBattler_AI.get();
                        crate::gl!(gBattleMons[gl_i26].moves[i])
                    }) == *movePtr
                    {
                        break;
                    }
                    i += 1;
                }
                if i == MAX_MON_MOVES {
                    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(8));
                } else {
                    crate::gl!(
                        gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(4) as i32
                            | (*(*gAIScriptPtr.as_ptr()).at(4).at(1) as i32) << 8
                            | (*(*gAIScriptPtr.as_ptr()).at(4).at(2) as i32) << 16
                            | (*(*gAIScriptPtr.as_ptr()).at(4).at(3) as i32) << 24)
                            as usize as *mut u8
                    );
                }
            }
            AI_USER_PARTNER => {
                if ({
                    let gl_i27 = sBattler_AI.get() as i32 ^ 2;
                    crate::gl!(gBattleMons[gl_i27].hp)
                }) == 0
                {
                    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(8));
                    break 'l1;
                } else {
                    for i in 0..MAX_MON_MOVES {
                        if ({
                            let gl_i28 = sBattler_AI.get() as i32 ^ 2;
                            crate::gl!(gBattleMons[gl_i28].moves[i])
                        }) == *movePtr
                        {
                            break;
                        }
                    }
                }
                if i == MAX_MON_MOVES {
                    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(8));
                } else {
                    crate::gl!(
                        gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(4) as i32
                            | (*(*gAIScriptPtr.as_ptr()).at(4).at(1) as i32) << 8
                            | (*(*gAIScriptPtr.as_ptr()).at(4).at(2) as i32) << 16
                            | (*(*gAIScriptPtr.as_ptr()).at(4).at(3) as i32) << 24)
                            as usize as *mut u8
                    );
                }
            }
            AI_TARGET | AI_TARGET_PARTNER => {
                i = 0;
                while i < MAX_MON_MOVES {
                    if (*(*crate::gl!(gBattleResources)).battleHistory).usedMoves
                        [gBattlerTarget.get()]
                    .moves[i]
                        == *movePtr
                    {
                        break;
                    }
                    i += 1;
                }
                if i == MAX_MON_MOVES {
                    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(8));
                } else {
                    crate::gl!(
                        gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(4) as i32
                            | (*(*gAIScriptPtr.as_ptr()).at(4).at(1) as i32) << 8
                            | (*(*gAIScriptPtr.as_ptr()).at(4).at(2) as i32) << 16
                            | (*(*gAIScriptPtr.as_ptr()).at(4).at(3) as i32) << 24)
                            as usize as *mut u8
                    );
                }
            }
            _ => {}
        }
    }
}
pub(crate) unsafe fn Cmd_if_doesnt_have_move() {
    let mut i: i32 = 0;
    let movePtr: *mut u16 = (*gAIScriptPtr.as_ptr()).at(2) as *mut u16;
    match *(*gAIScriptPtr.as_ptr()).at(1) {
        AI_USER | AI_USER_PARTNER => {
            i = 0;
            while i < MAX_MON_MOVES {
                if ({
                    let gl_i29 = sBattler_AI.get();
                    crate::gl!(gBattleMons[gl_i29].moves[i])
                }) == *movePtr
                {
                    break;
                }
                i += 1;
            }
            if i != MAX_MON_MOVES {
                crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(8));
            } else {
                crate::gl!(
                    gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(4) as i32
                        | (*(*gAIScriptPtr.as_ptr()).at(4).at(1) as i32) << 8
                        | (*(*gAIScriptPtr.as_ptr()).at(4).at(2) as i32) << 16
                        | (*(*gAIScriptPtr.as_ptr()).at(4).at(3) as i32) << 24)
                        as usize as *mut u8
                );
            }
        }
        AI_TARGET | AI_TARGET_PARTNER => {
            i = 0;
            while i < MAX_MON_MOVES {
                if (*(*crate::gl!(gBattleResources)).battleHistory).usedMoves[gBattlerTarget.get()]
                    .moves[i]
                    == *movePtr
                {
                    break;
                }
                i += 1;
            }
            if i != MAX_MON_MOVES {
                crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(8));
            } else {
                crate::gl!(
                    gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(4) as i32
                        | (*(*gAIScriptPtr.as_ptr()).at(4).at(1) as i32) << 8
                        | (*(*gAIScriptPtr.as_ptr()).at(4).at(2) as i32) << 16
                        | (*(*gAIScriptPtr.as_ptr()).at(4).at(3) as i32) << 24)
                        as usize as *mut u8
                );
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Cmd_if_has_move_with_effect() {
    let mut i: i32 = 0;
    match *(*gAIScriptPtr.as_ptr()).at(1) {
        AI_USER | AI_USER_PARTNER => {
            i = 0;
            while i < MAX_MON_MOVES {
                if ({
                    let gl_i30 = sBattler_AI.get();
                    crate::gl!(gBattleMons[gl_i30].moves[i])
                }) != 0
                    && (*(&raw const crate::data::pokemon::gBattleMoves)
                        .cast::<CArray<BattleMove, 0>>())[{
                        let gl_i31 = sBattler_AI.get();
                        crate::gl!(gBattleMons[gl_i31].moves[i])
                    }]
                    .effect
                        == *(*gAIScriptPtr.as_ptr()).at(2)
                {
                    break;
                }
                i += 1;
            }
            if i == MAX_MON_MOVES {
                crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(7));
            } else {
                crate::gl!(
                    gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(3) as i32
                        | (*(*gAIScriptPtr.as_ptr()).at(3).at(1) as i32) << 8
                        | (*(*gAIScriptPtr.as_ptr()).at(3).at(2) as i32) << 16
                        | (*(*gAIScriptPtr.as_ptr()).at(3).at(3) as i32) << 24)
                        as usize as *mut u8
                );
            }
        }
        AI_TARGET | AI_TARGET_PARTNER => {
            i = 0;
            while i < MAX_MON_MOVES {
                if ({
                    let gl_i32 = sBattler_AI.get();
                    crate::gl!(gBattleMons[gl_i32].moves[i])
                }) != 0
                    && (*(&raw const crate::data::pokemon::gBattleMoves)
                        .cast::<CArray<BattleMove, 0>>())[(*(*crate::gl!(gBattleResources))
                        .battleHistory)
                        .usedMoves[gBattlerTarget.get()]
                    .moves[i]]
                        .effect
                        == *(*gAIScriptPtr.as_ptr()).at(2)
                {
                    break;
                }
                i += 1;
            }
            if i == MAX_MON_MOVES {
                crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(7));
            } else {
                crate::gl!(
                    gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(3) as i32
                        | (*(*gAIScriptPtr.as_ptr()).at(3).at(1) as i32) << 8
                        | (*(*gAIScriptPtr.as_ptr()).at(3).at(2) as i32) << 16
                        | (*(*gAIScriptPtr.as_ptr()).at(3).at(3) as i32) << 24)
                        as usize as *mut u8
                );
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Cmd_if_doesnt_have_move_with_effect() {
    let mut i: i32 = 0;
    match *(*gAIScriptPtr.as_ptr()).at(1) {
        AI_USER | AI_USER_PARTNER => {
            i = 0;
            while i < MAX_MON_MOVES {
                if ({
                    let gl_i33 = sBattler_AI.get();
                    crate::gl!(gBattleMons[gl_i33].moves[i])
                }) != 0
                    && (*(&raw const crate::data::pokemon::gBattleMoves)
                        .cast::<CArray<BattleMove, 0>>())[{
                        let gl_i34 = sBattler_AI.get();
                        crate::gl!(gBattleMons[gl_i34].moves[i])
                    }]
                    .effect
                        == *(*gAIScriptPtr.as_ptr()).at(2)
                {
                    break;
                }
                i += 1;
            }
            if i != MAX_MON_MOVES {
                crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(7));
            } else {
                crate::gl!(
                    gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(3) as i32
                        | (*(*gAIScriptPtr.as_ptr()).at(3).at(1) as i32) << 8
                        | (*(*gAIScriptPtr.as_ptr()).at(3).at(2) as i32) << 16
                        | (*(*gAIScriptPtr.as_ptr()).at(3).at(3) as i32) << 24)
                        as usize as *mut u8
                );
            }
        }
        AI_TARGET | AI_TARGET_PARTNER => {
            i = 0;
            while i < MAX_MON_MOVES {
                if (*(*crate::gl!(gBattleResources)).battleHistory).usedMoves[gBattlerTarget.get()]
                    .moves[i]
                    != 0
                    && (*(&raw const crate::data::pokemon::gBattleMoves)
                        .cast::<CArray<BattleMove, 0>>())[(*(*crate::gl!(gBattleResources))
                        .battleHistory)
                        .usedMoves[gBattlerTarget.get()]
                    .moves[i]]
                        .effect
                        == *(*gAIScriptPtr.as_ptr()).at(2)
                {
                    break;
                }
                i += 1;
            }
            if i != MAX_MON_MOVES {
                crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(7));
            } else {
                crate::gl!(
                    gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(3) as i32
                        | (*(*gAIScriptPtr.as_ptr()).at(3).at(1) as i32) << 8
                        | (*(*gAIScriptPtr.as_ptr()).at(3).at(2) as i32) << 16
                        | (*(*gAIScriptPtr.as_ptr()).at(3).at(3) as i32) << 24)
                        as usize as *mut u8
                );
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Cmd_if_any_move_disabled_or_encored() {
    let mut battler: u8 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get();
    } else {
        battler = gBattlerTarget.get();
    }
    if *(*gAIScriptPtr.as_ptr()).at(2) == 0 {
        if crate::gl!(gDisableStructs[battler].disabledMove) == MOVE_NONE {
            crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(7));
        } else {
            crate::gl!(
                gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(3) as i32
                    | (*(*gAIScriptPtr.as_ptr()).at(3).at(1) as i32) << 8
                    | (*(*gAIScriptPtr.as_ptr()).at(3).at(2) as i32) << 16
                    | (*(*gAIScriptPtr.as_ptr()).at(3).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        }
    } else if *(*gAIScriptPtr.as_ptr()).at(2) != 1 {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(7));
    } else {
        if crate::gl!(gDisableStructs[battler].encoredMove) != MOVE_NONE {
            crate::gl!(
                gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(3) as i32
                    | (*(*gAIScriptPtr.as_ptr()).at(3).at(1) as i32) << 8
                    | (*(*gAIScriptPtr.as_ptr()).at(3).at(2) as i32) << 16
                    | (*(*gAIScriptPtr.as_ptr()).at(3).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        } else {
            crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(7));
        }
    }
}
pub(crate) unsafe fn Cmd_if_curr_move_disabled_or_encored() {
    match *(*gAIScriptPtr.as_ptr()).at(1) {
        0 => {
            if ({
                let gl_i35 = gActiveBattler.get();
                crate::gl!(gDisableStructs[gl_i35].disabledMove)
            }) == (*(*crate::gl!(gBattleResources)).ai).moveConsidered
            {
                crate::gl!(
                    gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                        | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                        | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                        | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                        as usize as *mut u8
                );
            } else {
                crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
            }
        }
        1 if ({
            let gl_i36 = gActiveBattler.get();
            crate::gl!(gDisableStructs[gl_i36].encoredMove)
        }) == (*(*crate::gl!(gBattleResources)).ai).moveConsidered =>
        {
            crate::gl!(
                gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                    | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                    | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                    | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        }
        _ => {
            crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
        }
    }
}
pub(crate) unsafe fn Cmd_flee() {
    (*(*crate::gl!(gBattleResources)).ai).aiAction |= 11;
}
pub(crate) unsafe fn Cmd_if_random_safari_flee() {
    let safariFleeRate: u8 = (*crate::gl!(gBattleStruct)).safariEscapeFactor * 5;
    if ((Random() as i32 % 100) as u8) < safariFleeRate {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_watch() {
    (*(*crate::gl!(gBattleResources)).ai).aiAction |= 13;
}
pub(crate) unsafe fn Cmd_get_hold_effect() {
    let mut battler: u8 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get();
    } else {
        battler = gBattlerTarget.get();
    }
    if gActiveBattler.get() != battler {
        (*(*crate::gl!(gBattleResources)).ai).funcResult = GetItemHoldEffect(
            (*(*crate::gl!(gBattleResources)).battleHistory).itemEffects[battler] as u16,
        ) as u32;
    } else {
        (*(*crate::gl!(gBattleResources)).ai).funcResult =
            GetItemHoldEffect(crate::gl!(gBattleMons[battler].item)) as u32;
    }
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_if_holds_item() {
    let battler: u8 = BattleAI_GetWantedBattler(*(*gAIScriptPtr.as_ptr()).at(1));
    let mut item: u16 = 0;
    if battler as i32 & BIT_SIDE as i32 == sBattler_AI.get() as i32 & BIT_SIDE as i32 {
        item = crate::gl!(gBattleMons[battler].item);
    } else {
        item = (*(*crate::gl!(gBattleResources)).battleHistory).itemEffects[battler] as u16;
    }
    let itemHi: u8 = *(*gAIScriptPtr.as_ptr()).at(2);
    let itemLo: u8 = *(*gAIScriptPtr.as_ptr()).at(3);
    if itemLo as i32 | itemHi as i32 == item as i32 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(4) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(8));
    }
}
pub(crate) unsafe fn Cmd_get_gender() {
    let mut battler: u8 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get();
    } else {
        battler = gBattlerTarget.get();
    }
    (*(*crate::gl!(gBattleResources)).ai).funcResult = GetGenderFromSpeciesAndPersonality(
        crate::gl!(gBattleMons[battler].species),
        crate::gl!(gBattleMons[battler].personality),
    ) as u32;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_is_first_turn_for() {
    let mut battler: u8 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get();
    } else {
        battler = gBattlerTarget.get();
    }
    (*(*crate::gl!(gBattleResources)).ai).funcResult =
        crate::gl!(gDisableStructs[battler].isFirstTurn) as u32;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_get_stockpile_count() {
    let mut battler: u8 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get();
    } else {
        battler = gBattlerTarget.get();
    }
    (*(*crate::gl!(gBattleResources)).ai).funcResult =
        crate::gl!(gDisableStructs[battler].stockpileCounter) as u32;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_is_double_battle() {
    (*(*crate::gl!(gBattleResources)).ai).funcResult = gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_get_used_held_item() {
    let mut battler: u8 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get();
    } else {
        battler = gBattlerTarget.get();
    }
    (*(*crate::gl!(gBattleResources)).ai).funcResult =
        *(&raw mut (*crate::gl!(gBattleStruct)).usedHeldItems[battler] as *mut u8) as u32;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_get_move_type_from_result() {
    (*(*crate::gl!(gBattleResources)).ai).funcResult =
        (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [(*(*crate::gl!(gBattleResources)).ai).funcResult]
            .r#type as u32;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_get_move_power_from_result() {
    (*(*crate::gl!(gBattleResources)).ai).funcResult =
        (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [(*(*crate::gl!(gBattleResources)).ai).funcResult]
            .power as u32;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_get_move_effect_from_result() {
    (*(*crate::gl!(gBattleResources)).ai).funcResult =
        (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [(*(*crate::gl!(gBattleResources)).ai).funcResult]
            .effect as u32;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_get_protect_count() {
    let mut battler: u8 = 0;
    if *(*gAIScriptPtr.as_ptr()).at(1) == 1 {
        battler = sBattler_AI.get();
    } else {
        battler = gBattlerTarget.get();
    }
    (*(*crate::gl!(gBattleResources)).ai).funcResult =
        crate::gl!(gDisableStructs[battler].protectUses) as u32;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) fn Cmd_nop_52() {}
pub(crate) fn Cmd_nop_53() {}
pub(crate) fn Cmd_nop_54() {}
pub(crate) fn Cmd_nop_55() {}
pub(crate) fn Cmd_nop_56() {}
pub(crate) fn Cmd_nop_57() {}
pub(crate) unsafe fn Cmd_call() {
    AIStackPushVar((*gAIScriptPtr.as_ptr()).at(5));
    crate::gl!(
        gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
            | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
            | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
            | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24) as usize
            as *mut u8
    );
}
pub(crate) unsafe fn Cmd_goto() {
    crate::gl!(
        gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
            | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
            | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
            | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24) as usize
            as *mut u8
    );
}
pub(crate) unsafe fn Cmd_end() {
    if AIStackPop() == 0 {
        (*(*crate::gl!(gBattleResources)).ai).aiAction |= AI_ACTION_DONE;
    }
}
pub(crate) unsafe fn Cmd_if_level_cond() {
    match *(*gAIScriptPtr.as_ptr()).at(1) {
        0 => {
            if ({
                let gl_i37 = sBattler_AI.get();
                crate::gl!(gBattleMons[gl_i37].level)
            }) > ({
                let gl_i38 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i38].level)
            }) {
                crate::gl!(
                    gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                        | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                        | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                        | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                        as usize as *mut u8
                );
            } else {
                crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
            }
        }
        1 => {
            if ({
                let gl_i39 = sBattler_AI.get();
                crate::gl!(gBattleMons[gl_i39].level)
            }) < ({
                let gl_i40 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i40].level)
            }) {
                crate::gl!(
                    gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                        | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                        | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                        | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                        as usize as *mut u8
                );
            } else {
                crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
            }
        }
        2 => {
            if ({
                let gl_i41 = sBattler_AI.get();
                crate::gl!(gBattleMons[gl_i41].level)
            }) == ({
                let gl_i42 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i42].level)
            }) {
                crate::gl!(
                    gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                        | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                        | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                        | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                        as usize as *mut u8
                );
            } else {
                crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Cmd_if_target_taunted() {
    if (*gDisableStructs.as_ptr())[gBattlerTarget.get()].tauntTimer() != 0 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_if_target_not_taunted() {
    if (*gDisableStructs.as_ptr())[gBattlerTarget.get()].tauntTimer() == 0 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_if_target_is_ally() {
    if sBattler_AI.get() as i32 & BIT_SIDE as i32 == gBattlerTarget.get() as i32 & BIT_SIDE as i32 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_if_flash_fired() {
    let battler: u8 = BattleAI_GetWantedBattler(*(*gAIScriptPtr.as_ptr()).at(1));
    if (*(*crate::gl!(gBattleResources)).flags).flags[battler] & RESOURCE_FLAG_FLASH_FIRE != 0 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn AIStackPushVar(var: *mut u8) {
    (*(*crate::gl!(gBattleResources)).AI_ScriptsStack).ptr[{
        let t1 = (*(*crate::gl!(gBattleResources)).AI_ScriptsStack).size;
        (*(*crate::gl!(gBattleResources)).AI_ScriptsStack).size += 1;
        t1
    }] = var;
}
unsafe fn AIStackPushVar_cursor() {
    (*(*crate::gl!(gBattleResources)).AI_ScriptsStack).ptr[{
        let t1 = (*(*crate::gl!(gBattleResources)).AI_ScriptsStack).size;
        (*(*crate::gl!(gBattleResources)).AI_ScriptsStack).size += 1;
        t1
    }] = crate::gl!(gAIScriptPtr);
}
pub(crate) unsafe fn AIStackPop() -> u8 {
    if (*(*crate::gl!(gBattleResources)).AI_ScriptsStack).size != 0 {
        (*(*crate::gl!(gBattleResources)).AI_ScriptsStack).size -= 1;
        crate::gl!(
            gAIScriptPtr = (*(*crate::gl!(gBattleResources)).AI_ScriptsStack).ptr
                [(*(*crate::gl!(gBattleResources)).AI_ScriptsStack).size]
        );
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
