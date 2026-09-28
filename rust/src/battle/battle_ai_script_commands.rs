//! Translated from `src/battle_ai_script_commands.c` by tools/rustport/c2rs.py.
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

static sBattleAICmdTable: Table<CArray<Option<unsafe extern "C" fn()>, 99>> =
    Table((&raw const crate::data::battle_ai_script_commands::sBattleAICmdTable).cast());
static sIgnoredPowerfulMoveEffects: Table<CArray<u16, 13>> =
    Table((&raw const crate::data::battle_ai_script_commands::sIgnoredPowerfulMoveEffects).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAIScriptPtr: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattler_AI: u8 = 0;

unsafe extern "C" {
    static mut gAbsentBattlerFlags: u8;
    static mut gActiveBattler: u8;
    static gBattleAI_ScriptsTable: CArray<*mut u8, 0>;
    static mut gBattleMons: CArray<BattlePokemon, 4>;
    static mut gBattleMoveDamage: i32;
    static gBattleMoves: CArray<BattleMove, 0>;
    static mut gBattleResources: *mut BattleResources;
    static mut gBattleResults: BattleResults;
    static mut gBattleScripting: BattleScripting;
    static mut gBattleStruct: *mut BattleStruct;
    static mut gBattleTypeFlags: u32;
    static mut gBattleWeather: u16;
    static mut gBattlerPartyIndexes: CArray<u16, 4>;
    static mut gBattlerTarget: u8;
    static gBitTable: CArray<u32, 0>;
    static mut gCritMultiplier: u8;
    static mut gCurrentMove: u16;
    static mut gDisableStructs: CArray<DisableStruct, 4>;
    static mut gDynamicBasePower: u16;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static mut gLastMoves: CArray<u16, 4>;
    static mut gMoveResultFlags: u8;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gSideStatuses: CArray<u16, 2>;
    static gSpeciesInfo: CArray<SpeciesInfo, 0>;
    static mut gStatuses3: CArray<u32, 4>;
    static mut gTrainerBattleOpponent_A: u16;
    static mut gTrainerBattleOpponent_B: u16;
    static gTrainers: CArray<Trainer, 0>;
    fn AI_CalcDmg(a0: u8, a1: u8);
    fn CheckMoveLimitations(a0: u8, a1: u8, a2: u8) -> u8;
    fn GetAiScriptsInBattleFactory() -> u32;
    fn GetAiScriptsInRecordedBattle() -> u32;
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetGenderFromSpeciesAndPersonality(a0: u16, a1: u32) -> u8;
    fn GetItemHoldEffect(a0: u16) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetWhoStrikesFirst(a0: u8, a1: u8, a2: u8) -> u8;
    fn Random() -> u16;
    fn TypeCalc(a0: u16, a1: u8, a2: u8) -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleAI_HandleItemUseBeforeAISetup(defaultScoreMoves: u8) {
    let mut i: i32 = 0;
    let mut data: *mut u8 = (*gBattleResources).battleHistory as *mut u8;
    i = 0;
    while i < 84 {
        *data.at(i) = 0;
        i += 1;
    }
    if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 && gBattleTypeFlags & 0xa7f0982 == 0 {
        i = 0;
        while i < MAX_TRAINER_ITEMS {
            if gTrainers[gTrainerBattleOpponent_A].items[i] != ITEM_NONE {
                (*(*gBattleResources).battleHistory).trainerItems
                    [(*(*gBattleResources).battleHistory).itemsNo] =
                    gTrainers[gTrainerBattleOpponent_A].items[i];
                (*(*gBattleResources).battleHistory).itemsNo += 1;
            }
            i += 1;
        }
    }
    BattleAI_SetupAIData(defaultScoreMoves);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleAI_SetupAIData(mut defaultScoreMoves: u8) {
    let mut i: i32 = 0;
    let mut data: *mut u8 = (*gBattleResources).ai as *mut u8;
    let mut moveLimitations: u8 = 0;
    i = 0;
    while i < 28 {
        *data.at(i) = 0;
        i += 1;
    }
    i = 0;
    while i < MAX_MON_MOVES {
        if defaultScoreMoves as i32 & 1 != 0 {
            (*(*gBattleResources).ai).score[i] = 100;
        } else {
            (*(*gBattleResources).ai).score[i] = 0;
        }
        defaultScoreMoves >>= 1;
        i += 1;
    }
    moveLimitations = CheckMoveLimitations(gActiveBattler, 0, MOVE_LIMITATIONS_ALL);
    i = 0;
    while i < MAX_MON_MOVES {
        if gBitTable[i] & moveLimitations as u32 != 0 {
            (*(*gBattleResources).ai).score[i] = 0;
        }
        (*(*gBattleResources).ai).simulatedRNG[i] = 100 - (Random() as i32 % 16) as u8;
        i += 1;
    }
    (*(*gBattleResources).AI_ScriptsStack).size = 0;
    sBattler_AI = gActiveBattler;
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
        gBattlerTarget = (Random() as u8 & BIT_FLANK) + (GetBattlerSide(gActiveBattler) ^ 1);
        if gAbsentBattlerFlags as u32 & gBitTable[gBattlerTarget] != 0 {
            gBattlerTarget ^= BIT_FLANK;
        }
    } else {
        gBattlerTarget = sBattler_AI ^ 1;
    }
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
        (*(*gBattleResources).ai).aiFlags = GetAiScriptsInRecordedBattle();
    } else if gBattleTypeFlags & BATTLE_TYPE_SAFARI != 0 {
        (*(*gBattleResources).ai).aiFlags = AI_SCRIPT_SAFARI;
    } else if gBattleTypeFlags & BATTLE_TYPE_ROAMER != 0 {
        (*(*gBattleResources).ai).aiFlags = AI_SCRIPT_ROAMING;
    } else if gBattleTypeFlags & BATTLE_TYPE_FIRST_BATTLE != 0 {
        (*(*gBattleResources).ai).aiFlags = 0x80000000;
    } else if gBattleTypeFlags & BATTLE_TYPE_FACTORY != 0 {
        (*(*gBattleResources).ai).aiFlags = GetAiScriptsInBattleFactory();
    } else if gBattleTypeFlags & 0xc3f0900 != 0 {
        (*(*gBattleResources).ai).aiFlags = 7;
    } else if gBattleTypeFlags & BATTLE_TYPE_TWO_OPPONENTS != 0 {
        (*(*gBattleResources).ai).aiFlags = gTrainers[gTrainerBattleOpponent_A].aiFlags
            | gTrainers[gTrainerBattleOpponent_B].aiFlags;
    } else {
        (*(*gBattleResources).ai).aiFlags = gTrainers[gTrainerBattleOpponent_A].aiFlags;
    }
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
        (*(*gBattleResources).ai).aiFlags |= AI_SCRIPT_DOUBLE_BATTLE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleAI_ChooseMoveOrAction() -> u8 {
    let mut savedCurrentMove: u16 = gCurrentMove;
    let mut ret: u8 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE == 0 {
        ret = ChooseMoveOrAction_Singles();
    } else {
        ret = ChooseMoveOrAction_Doubles();
    }
    gCurrentMove = savedCurrentMove;
    return ret;
}
pub(crate) unsafe extern "C" fn ChooseMoveOrAction_Singles() -> u8 {
    let mut currentMoveArray: CArray<u8, 4> = zeroed();
    let mut consideredMoveArray: CArray<u8, 4> = zeroed();
    let mut numOfBestMoves: u8 = 0;
    let mut i: i32 = 0;
    RecordLastUsedMoveByTarget();
    while (*(*gBattleResources).ai).aiFlags != 0 {
        if (*(*gBattleResources).ai).aiFlags & 1 != 0 {
            (*(*gBattleResources).ai).aiState = AIState_SettingUp;
            BattleAI_DoAIProcessing();
        }
        (*(*gBattleResources).ai).aiFlags >>= 1;
        (*(*gBattleResources).ai).aiLogicId += 1;
        (*(*gBattleResources).ai).movesetIndex = 0;
    }
    if (*(*gBattleResources).ai).aiAction as i32 & AI_ACTION_FLEE != 0 {
        return AI_CHOICE_FLEE;
    }
    if (*(*gBattleResources).ai).aiAction as i32 & AI_ACTION_WATCH != 0 {
        return AI_CHOICE_WATCH;
    }
    numOfBestMoves = 1;
    currentMoveArray[0] = (*(*gBattleResources).ai).score[0] as u8;
    consideredMoveArray[0] = 0;
    i = 1;
    while i < MAX_MON_MOVES {
        if gBattleMons[sBattler_AI].moves[i] != MOVE_NONE {
            if currentMoveArray[0] as i32 == (*(*gBattleResources).ai).score[i] as i32 {
                currentMoveArray[numOfBestMoves] = (*(*gBattleResources).ai).score[i] as u8;
                consideredMoveArray[{
                    let t1 = numOfBestMoves;
                    numOfBestMoves += 1;
                    t1
                }] = i as u8;
            }
            if (currentMoveArray[0] as i32) < (*(*gBattleResources).ai).score[i] as i32 {
                numOfBestMoves = 1;
                currentMoveArray[0] = (*(*gBattleResources).ai).score[i] as u8;
                consideredMoveArray[0] = i as u8;
            }
        }
        i += 1;
    }
    return consideredMoveArray[rem_i32(Random() as i32, numOfBestMoves as i32)];
}
pub(crate) unsafe extern "C" fn ChooseMoveOrAction_Doubles() -> u8 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut scriptsToRun: i32 = 0;
    let mut bestMovePointsForTarget: CArray<i16, 4> = zeroed();
    let mut mostViableTargetsArray: CArray<i8, 4> = zeroed();
    let mut actionOrMoveIndex: CArray<u8, 4> = zeroed();
    let mut mostViableMovesScores: CArray<u8, 4> = zeroed();
    let mut mostViableMovesIndices: CArray<u8, 4> = zeroed();
    let mut mostViableTargetsNo: i32 = 0;
    let mut mostViableMovesNo: i32 = 0;
    let mut mostMovePoints: i16 = 0;
    i = 0;
    while i < MAX_BATTLERS_COUNT as i32 {
        if i == sBattler_AI as i32 || gBattleMons[i].hp == 0 {
            actionOrMoveIndex[i] = 0xFF;
            bestMovePointsForTarget[i] = -1;
        } else {
            if gBattleTypeFlags & BATTLE_TYPE_PALACE != 0 {
                BattleAI_SetupAIData((*gBattleStruct).palaceFlags >> 4);
            } else {
                BattleAI_SetupAIData(ALL_MOVES_MASK);
            }
            gBattlerTarget = i as u8;
            if i & BIT_SIDE as i32 != sBattler_AI as i32 & BIT_SIDE as i32 {
                RecordLastUsedMoveByTarget();
            }
            (*(*gBattleResources).ai).aiLogicId = 0;
            (*(*gBattleResources).ai).movesetIndex = 0;
            scriptsToRun = (*(*gBattleResources).ai).aiFlags as i32;
            while scriptsToRun != 0 {
                if scriptsToRun & 1 != 0 {
                    (*(*gBattleResources).ai).aiState = AIState_SettingUp;
                    BattleAI_DoAIProcessing();
                }
                scriptsToRun >>= 1;
                (*(*gBattleResources).ai).aiLogicId += 1;
                (*(*gBattleResources).ai).movesetIndex = 0;
            }
            if (*(*gBattleResources).ai).aiAction as i32 & AI_ACTION_FLEE != 0 {
                actionOrMoveIndex[i] = AI_CHOICE_FLEE;
            } else if (*(*gBattleResources).ai).aiAction as i32 & AI_ACTION_WATCH != 0 {
                actionOrMoveIndex[i] = AI_CHOICE_WATCH;
            } else {
                mostViableMovesScores[0] = (*(*gBattleResources).ai).score[0] as u8;
                mostViableMovesIndices[0] = 0;
                mostViableMovesNo = 1;
                j = 1;
                while j < MAX_MON_MOVES {
                    if gBattleMons[sBattler_AI].moves[j] != 0 {
                        if mostViableMovesScores[0] as i32
                            == (*(*gBattleResources).ai).score[j] as i32
                        {
                            mostViableMovesScores[mostViableMovesNo] =
                                (*(*gBattleResources).ai).score[j] as u8;
                            mostViableMovesIndices[mostViableMovesNo] = j as u8;
                            mostViableMovesNo += 1;
                        }
                        if (mostViableMovesScores[0] as i32)
                            < (*(*gBattleResources).ai).score[j] as i32
                        {
                            mostViableMovesScores[0] = (*(*gBattleResources).ai).score[j] as u8;
                            mostViableMovesIndices[0] = j as u8;
                            mostViableMovesNo = 1;
                        }
                    }
                    j += 1;
                }
                actionOrMoveIndex[i] =
                    mostViableMovesIndices[rem_i32(Random() as i32, mostViableMovesNo)];
                bestMovePointsForTarget[i] = mostViableMovesScores[0] as i16;
                if i == sBattler_AI as i32 ^ 2 && bestMovePointsForTarget[i] < 100 {
                    bestMovePointsForTarget[i] = -1;
                    mostViableMovesScores[0] = mostViableMovesScores[0];
                }
            }
        }
        i += 1;
    }
    mostMovePoints = bestMovePointsForTarget[0];
    mostViableTargetsArray[0] = 0;
    mostViableTargetsNo = 1;
    i = 1;
    while i < MAX_BATTLERS_COUNT as i32 {
        if mostMovePoints == bestMovePointsForTarget[i] {
            mostViableTargetsArray[mostViableTargetsNo] = i as i8;
            mostViableTargetsNo += 1;
        }
        if mostMovePoints < bestMovePointsForTarget[i] {
            mostMovePoints = bestMovePointsForTarget[i];
            mostViableTargetsArray[0] = i as i8;
            mostViableTargetsNo = 1;
        }
        i += 1;
    }
    gBattlerTarget = mostViableTargetsArray[rem_i32(Random() as i32, mostViableTargetsNo)] as u8;
    return actionOrMoveIndex[gBattlerTarget];
}
pub(crate) unsafe extern "C" fn BattleAI_DoAIProcessing() {
    while (*(*gBattleResources).ai).aiState != AIState_FinishedProcessing {
        match (*(*gBattleResources).ai).aiState {
            AIState_DoNotProcess => {}
            AIState_SettingUp => {
                gAIScriptPtr = gBattleAI_ScriptsTable[(*(*gBattleResources).ai).aiLogicId];
                if gBattleMons[sBattler_AI].pp[(*(*gBattleResources).ai).movesetIndex] == 0 {
                    (*(*gBattleResources).ai).moveConsidered = 0;
                } else {
                    (*(*gBattleResources).ai).moveConsidered =
                        gBattleMons[sBattler_AI].moves[(*(*gBattleResources).ai).movesetIndex];
                }
                (*(*gBattleResources).ai).aiState += 1;
            }
            AIState_Processing => {
                if (*(*gBattleResources).ai).moveConsidered != 0 {
                    sBattleAICmdTable[*gAIScriptPtr].unwrap_unchecked()();
                } else {
                    (*(*gBattleResources).ai).score[(*(*gBattleResources).ai).movesetIndex] = 0;
                    (*(*gBattleResources).ai).aiAction |= AI_ACTION_DONE;
                }
                if (*(*gBattleResources).ai).aiAction as i32 & AI_ACTION_DONE as i32 != 0 {
                    (*(*gBattleResources).ai).movesetIndex += 1;
                    if (*(*gBattleResources).ai).movesetIndex < MAX_MON_MOVES as u8
                        && (*(*gBattleResources).ai).aiAction as i32 & AI_ACTION_DO_NOT_ATTACK == 0
                    {
                        (*(*gBattleResources).ai).aiState = AIState_SettingUp;
                    } else {
                        (*(*gBattleResources).ai).aiState += 1;
                    }
                    (*(*gBattleResources).ai).aiAction &= 254;
                }
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn RecordLastUsedMoveByTarget() {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_MON_MOVES {
        if (*(*gBattleResources).battleHistory).usedMoves[gBattlerTarget].moves[i]
            == gLastMoves[gBattlerTarget]
        {
            break;
        }
        if (*(*gBattleResources).battleHistory).usedMoves[gBattlerTarget].moves[i] == MOVE_NONE {
            (*(*gBattleResources).battleHistory).usedMoves[gBattlerTarget].moves[i] =
                gLastMoves[gBattlerTarget];
            break;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearBattlerMoveHistory(battler: u8) {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_MON_MOVES {
        (*(*gBattleResources).battleHistory).usedMoves[battler].moves[i] = MOVE_NONE;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordAbilityBattle(battler: u8, abilityId: u8) {
    (*(*gBattleResources).battleHistory).abilities[battler] = abilityId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearBattlerAbilityHistory(battler: u8) {
    (*(*gBattleResources).battleHistory).abilities[battler] = ABILITY_NONE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordItemEffectBattle(battler: u8, itemEffect: u8) {
    (*(*gBattleResources).battleHistory).itemEffects[battler] = itemEffect;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearBattlerItemEffectHistory(battler: u8) {
    (*(*gBattleResources).battleHistory).itemEffects[battler] = 0;
}
pub(crate) unsafe extern "C" fn Cmd_if_random_less_than() {
    let mut random: u16 = Random();
    if random as i32 % 256 < *gAIScriptPtr.at(1) as i32 {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_random_greater_than() {
    let mut random: u16 = Random();
    if random as i32 % 256 > *gAIScriptPtr.at(1) as i32 {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_random_equal() {
    let mut random: u16 = Random();
    if random as i32 % 256 == *gAIScriptPtr.at(1) as i32 {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_random_not_equal() {
    let mut random: u16 = Random();
    if random as i32 % 256 != *gAIScriptPtr.at(1) as i32 {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_score() {
    (*(*gBattleResources).ai).score[(*(*gBattleResources).ai).movesetIndex] +=
        *gAIScriptPtr.at(1) as i8;
    if (*(*gBattleResources).ai).score[(*(*gBattleResources).ai).movesetIndex] < 0 {
        (*(*gBattleResources).ai).score[(*(*gBattleResources).ai).movesetIndex] = 0;
    }
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_if_hp_less_than() {
    let mut battler: u16 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI as u16;
    } else {
        battler = gBattlerTarget as u16;
    }
    if (div_i32(
        100 * gBattleMons[battler].hp as i32,
        gBattleMons[battler].maxHP as i32,
    ) as u32)
        < *gAIScriptPtr.at(2) as u32
    {
        gAIScriptPtr = (*gAIScriptPtr.at(3) as i32
            | (*gAIScriptPtr.at(3).at(1) as i32) << 8
            | (*gAIScriptPtr.at(3).at(2) as i32) << 16
            | (*gAIScriptPtr.at(3).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(7);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_hp_more_than() {
    let mut battler: u16 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI as u16;
    } else {
        battler = gBattlerTarget as u16;
    }
    if div_i32(
        100 * gBattleMons[battler].hp as i32,
        gBattleMons[battler].maxHP as i32,
    ) as u32
        > *gAIScriptPtr.at(2) as u32
    {
        gAIScriptPtr = (*gAIScriptPtr.at(3) as i32
            | (*gAIScriptPtr.at(3).at(1) as i32) << 8
            | (*gAIScriptPtr.at(3).at(2) as i32) << 16
            | (*gAIScriptPtr.at(3).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(7);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_hp_equal() {
    let mut battler: u16 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI as u16;
    } else {
        battler = gBattlerTarget as u16;
    }
    if div_i32(
        100 * gBattleMons[battler].hp as i32,
        gBattleMons[battler].maxHP as i32,
    ) as u32
        == *gAIScriptPtr.at(2) as u32
    {
        gAIScriptPtr = (*gAIScriptPtr.at(3) as i32
            | (*gAIScriptPtr.at(3).at(1) as i32) << 8
            | (*gAIScriptPtr.at(3).at(2) as i32) << 16
            | (*gAIScriptPtr.at(3).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(7);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_hp_not_equal() {
    let mut battler: u16 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI as u16;
    } else {
        battler = gBattlerTarget as u16;
    }
    if div_i32(
        100 * gBattleMons[battler].hp as i32,
        gBattleMons[battler].maxHP as i32,
    ) as u32
        != *gAIScriptPtr.at(2) as u32
    {
        gAIScriptPtr = (*gAIScriptPtr.at(3) as i32
            | (*gAIScriptPtr.at(3).at(1) as i32) << 8
            | (*gAIScriptPtr.at(3).at(2) as i32) << 16
            | (*gAIScriptPtr.at(3).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(7);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_status() {
    let mut battler: u16 = 0;
    let mut status: u32 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI as u16;
    } else {
        battler = gBattlerTarget as u16;
    }
    status = *gAIScriptPtr.at(2) as u32
        | (*gAIScriptPtr.at(2).at(1) as u32) << 8
        | (*gAIScriptPtr.at(2).at(2) as u32) << 16
        | (*gAIScriptPtr.at(2).at(3) as u32) << 24;
    if gBattleMons[battler].status1 & status != 0 {
        gAIScriptPtr = (*gAIScriptPtr.at(6) as i32
            | (*gAIScriptPtr.at(6).at(1) as i32) << 8
            | (*gAIScriptPtr.at(6).at(2) as i32) << 16
            | (*gAIScriptPtr.at(6).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(10);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_status() {
    let mut battler: u16 = 0;
    let mut status: u32 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI as u16;
    } else {
        battler = gBattlerTarget as u16;
    }
    status = *gAIScriptPtr.at(2) as u32
        | (*gAIScriptPtr.at(2).at(1) as u32) << 8
        | (*gAIScriptPtr.at(2).at(2) as u32) << 16
        | (*gAIScriptPtr.at(2).at(3) as u32) << 24;
    if gBattleMons[battler].status1 & status == 0 {
        gAIScriptPtr = (*gAIScriptPtr.at(6) as i32
            | (*gAIScriptPtr.at(6).at(1) as i32) << 8
            | (*gAIScriptPtr.at(6).at(2) as i32) << 16
            | (*gAIScriptPtr.at(6).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(10);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_status2() {
    let mut battler: u16 = 0;
    let mut status: u32 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI as u16;
    } else {
        battler = gBattlerTarget as u16;
    }
    status = *gAIScriptPtr.at(2) as u32
        | (*gAIScriptPtr.at(2).at(1) as u32) << 8
        | (*gAIScriptPtr.at(2).at(2) as u32) << 16
        | (*gAIScriptPtr.at(2).at(3) as u32) << 24;
    if gBattleMons[battler].status2 & status != 0 {
        gAIScriptPtr = (*gAIScriptPtr.at(6) as i32
            | (*gAIScriptPtr.at(6).at(1) as i32) << 8
            | (*gAIScriptPtr.at(6).at(2) as i32) << 16
            | (*gAIScriptPtr.at(6).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(10);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_status2() {
    let mut battler: u16 = 0;
    let mut status: u32 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI as u16;
    } else {
        battler = gBattlerTarget as u16;
    }
    status = *gAIScriptPtr.at(2) as u32
        | (*gAIScriptPtr.at(2).at(1) as u32) << 8
        | (*gAIScriptPtr.at(2).at(2) as u32) << 16
        | (*gAIScriptPtr.at(2).at(3) as u32) << 24;
    if gBattleMons[battler].status2 & status == 0 {
        gAIScriptPtr = (*gAIScriptPtr.at(6) as i32
            | (*gAIScriptPtr.at(6).at(1) as i32) << 8
            | (*gAIScriptPtr.at(6).at(2) as i32) << 16
            | (*gAIScriptPtr.at(6).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(10);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_status3() {
    let mut battler: u16 = 0;
    let mut status: u32 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI as u16;
    } else {
        battler = gBattlerTarget as u16;
    }
    status = *gAIScriptPtr.at(2) as u32
        | (*gAIScriptPtr.at(2).at(1) as u32) << 8
        | (*gAIScriptPtr.at(2).at(2) as u32) << 16
        | (*gAIScriptPtr.at(2).at(3) as u32) << 24;
    if gStatuses3[battler] & status != 0 {
        gAIScriptPtr = (*gAIScriptPtr.at(6) as i32
            | (*gAIScriptPtr.at(6).at(1) as i32) << 8
            | (*gAIScriptPtr.at(6).at(2) as i32) << 16
            | (*gAIScriptPtr.at(6).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(10);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_status3() {
    let mut battler: u16 = 0;
    let mut status: u32 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI as u16;
    } else {
        battler = gBattlerTarget as u16;
    }
    status = *gAIScriptPtr.at(2) as u32
        | (*gAIScriptPtr.at(2).at(1) as u32) << 8
        | (*gAIScriptPtr.at(2).at(2) as u32) << 16
        | (*gAIScriptPtr.at(2).at(3) as u32) << 24;
    if gStatuses3[battler] & status == 0 {
        gAIScriptPtr = (*gAIScriptPtr.at(6) as i32
            | (*gAIScriptPtr.at(6).at(1) as i32) << 8
            | (*gAIScriptPtr.at(6).at(2) as i32) << 16
            | (*gAIScriptPtr.at(6).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(10);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_side_affecting() {
    let mut battler: u16 = 0;
    let mut side: u32 = 0;
    let mut status: u32 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI as u16;
    } else {
        battler = gBattlerTarget as u16;
    }
    side = GetBattlerPosition(battler as u8) as u32 & 1;
    status = *gAIScriptPtr.at(2) as u32
        | (*gAIScriptPtr.at(2).at(1) as u32) << 8
        | (*gAIScriptPtr.at(2).at(2) as u32) << 16
        | (*gAIScriptPtr.at(2).at(3) as u32) << 24;
    if gSideStatuses[side] as u32 & status != 0 {
        gAIScriptPtr = (*gAIScriptPtr.at(6) as i32
            | (*gAIScriptPtr.at(6).at(1) as i32) << 8
            | (*gAIScriptPtr.at(6).at(2) as i32) << 16
            | (*gAIScriptPtr.at(6).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(10);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_side_affecting() {
    let mut battler: u16 = 0;
    let mut side: u32 = 0;
    let mut status: u32 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI as u16;
    } else {
        battler = gBattlerTarget as u16;
    }
    side = GetBattlerPosition(battler as u8) as u32 & 1;
    status = *gAIScriptPtr.at(2) as u32
        | (*gAIScriptPtr.at(2).at(1) as u32) << 8
        | (*gAIScriptPtr.at(2).at(2) as u32) << 16
        | (*gAIScriptPtr.at(2).at(3) as u32) << 24;
    if gSideStatuses[side] as u32 & status == 0 {
        gAIScriptPtr = (*gAIScriptPtr.at(6) as i32
            | (*gAIScriptPtr.at(6).at(1) as i32) << 8
            | (*gAIScriptPtr.at(6).at(2) as i32) << 16
            | (*gAIScriptPtr.at(6).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(10);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_less_than() {
    if (*(*gBattleResources).ai).funcResult < *gAIScriptPtr.at(1) as u32 {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_more_than() {
    if (*(*gBattleResources).ai).funcResult > *gAIScriptPtr.at(1) as u32 {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_equal() {
    if (*(*gBattleResources).ai).funcResult == *gAIScriptPtr.at(1) as u32 {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_equal() {
    if (*(*gBattleResources).ai).funcResult != *gAIScriptPtr.at(1) as u32 {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_less_than_ptr() {
    let mut value: *mut u8 = (*gAIScriptPtr.at(1) as i32
        | (*gAIScriptPtr.at(1).at(1) as i32) << 8
        | (*gAIScriptPtr.at(1).at(2) as i32) << 16
        | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize
        as *mut u8;
    if (*(*gBattleResources).ai).funcResult < *value as u32 {
        gAIScriptPtr = (*gAIScriptPtr.at(5) as i32
            | (*gAIScriptPtr.at(5).at(1) as i32) << 8
            | (*gAIScriptPtr.at(5).at(2) as i32) << 16
            | (*gAIScriptPtr.at(5).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(9);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_more_than_ptr() {
    let mut value: *mut u8 = (*gAIScriptPtr.at(1) as i32
        | (*gAIScriptPtr.at(1).at(1) as i32) << 8
        | (*gAIScriptPtr.at(1).at(2) as i32) << 16
        | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize
        as *mut u8;
    if (*(*gBattleResources).ai).funcResult > *value as u32 {
        gAIScriptPtr = (*gAIScriptPtr.at(5) as i32
            | (*gAIScriptPtr.at(5).at(1) as i32) << 8
            | (*gAIScriptPtr.at(5).at(2) as i32) << 16
            | (*gAIScriptPtr.at(5).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(9);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_equal_ptr() {
    let mut value: *mut u8 = (*gAIScriptPtr.at(1) as i32
        | (*gAIScriptPtr.at(1).at(1) as i32) << 8
        | (*gAIScriptPtr.at(1).at(2) as i32) << 16
        | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize
        as *mut u8;
    if (*(*gBattleResources).ai).funcResult == *value as u32 {
        gAIScriptPtr = (*gAIScriptPtr.at(5) as i32
            | (*gAIScriptPtr.at(5).at(1) as i32) << 8
            | (*gAIScriptPtr.at(5).at(2) as i32) << 16
            | (*gAIScriptPtr.at(5).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(9);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_equal_ptr() {
    let mut value: *mut u8 = (*gAIScriptPtr.at(1) as i32
        | (*gAIScriptPtr.at(1).at(1) as i32) << 8
        | (*gAIScriptPtr.at(1).at(2) as i32) << 16
        | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize
        as *mut u8;
    if (*(*gBattleResources).ai).funcResult != *value as u32 {
        gAIScriptPtr = (*gAIScriptPtr.at(5) as i32
            | (*gAIScriptPtr.at(5).at(1) as i32) << 8
            | (*gAIScriptPtr.at(5).at(2) as i32) << 16
            | (*gAIScriptPtr.at(5).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(9);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_move() {
    let mut r#move: u16 = *gAIScriptPtr.at(1) as u16 | (*gAIScriptPtr.at(1).at(1) as u16) << 8;
    if (*(*gBattleResources).ai).moveConsidered == r#move {
        gAIScriptPtr = (*gAIScriptPtr.at(3) as i32
            | (*gAIScriptPtr.at(3).at(1) as i32) << 8
            | (*gAIScriptPtr.at(3).at(2) as i32) << 16
            | (*gAIScriptPtr.at(3).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(7);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_move() {
    let mut r#move: u16 = *gAIScriptPtr.at(1) as u16 | (*gAIScriptPtr.at(1).at(1) as u16) << 8;
    if (*(*gBattleResources).ai).moveConsidered != r#move {
        gAIScriptPtr = (*gAIScriptPtr.at(3) as i32
            | (*gAIScriptPtr.at(3).at(1) as i32) << 8
            | (*gAIScriptPtr.at(3).at(2) as i32) << 16
            | (*gAIScriptPtr.at(3).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(7);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_in_bytes() {
    let mut ptr: *mut u8 = (*gAIScriptPtr.at(1) as i32
        | (*gAIScriptPtr.at(1).at(1) as i32) << 8
        | (*gAIScriptPtr.at(1).at(2) as i32) << 16
        | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    while *ptr != 0xFF {
        if (*(*gBattleResources).ai).funcResult == *ptr as u32 {
            gAIScriptPtr = (*gAIScriptPtr.at(5) as i32
                | (*gAIScriptPtr.at(5).at(1) as i32) << 8
                | (*gAIScriptPtr.at(5).at(2) as i32) << 16
                | (*gAIScriptPtr.at(5).at(3) as i32) << 24) as usize
                as *mut u8;
            return;
        }
        ptr = ptr.at(1);
    }
    gAIScriptPtr = gAIScriptPtr.at(9);
}
pub(crate) unsafe extern "C" fn Cmd_if_not_in_bytes() {
    let mut ptr: *mut u8 = (*gAIScriptPtr.at(1) as i32
        | (*gAIScriptPtr.at(1).at(1) as i32) << 8
        | (*gAIScriptPtr.at(1).at(2) as i32) << 16
        | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    while *ptr != 0xFF {
        if (*(*gBattleResources).ai).funcResult == *ptr as u32 {
            gAIScriptPtr = gAIScriptPtr.at(9);
            return;
        }
        ptr = ptr.at(1);
    }
    gAIScriptPtr = (*gAIScriptPtr.at(5) as i32
        | (*gAIScriptPtr.at(5).at(1) as i32) << 8
        | (*gAIScriptPtr.at(5).at(2) as i32) << 16
        | (*gAIScriptPtr.at(5).at(3) as i32) << 24) as usize as *mut u8;
}
pub(crate) unsafe extern "C" fn Cmd_if_in_hwords() {
    let mut ptr: *mut u16 = (*gAIScriptPtr.at(1) as i32
        | (*gAIScriptPtr.at(1).at(1) as i32) << 8
        | (*gAIScriptPtr.at(1).at(2) as i32) << 16
        | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8
        as *mut u16;
    while *ptr != 0xFFFF {
        if (*(*gBattleResources).ai).funcResult == *ptr as u32 {
            gAIScriptPtr = (*gAIScriptPtr.at(5) as i32
                | (*gAIScriptPtr.at(5).at(1) as i32) << 8
                | (*gAIScriptPtr.at(5).at(2) as i32) << 16
                | (*gAIScriptPtr.at(5).at(3) as i32) << 24) as usize
                as *mut u8;
            return;
        }
        ptr = ptr.at(1);
    }
    gAIScriptPtr = gAIScriptPtr.at(9);
}
pub(crate) unsafe extern "C" fn Cmd_if_not_in_hwords() {
    let mut ptr: *mut u16 = (*gAIScriptPtr.at(1) as i32
        | (*gAIScriptPtr.at(1).at(1) as i32) << 8
        | (*gAIScriptPtr.at(1).at(2) as i32) << 16
        | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8
        as *mut u16;
    while *ptr != 0xFFFF {
        if (*(*gBattleResources).ai).funcResult == *ptr as u32 {
            gAIScriptPtr = gAIScriptPtr.at(9);
            return;
        }
        ptr = ptr.at(1);
    }
    gAIScriptPtr = (*gAIScriptPtr.at(5) as i32
        | (*gAIScriptPtr.at(5).at(1) as i32) << 8
        | (*gAIScriptPtr.at(5).at(2) as i32) << 16
        | (*gAIScriptPtr.at(5).at(3) as i32) << 24) as usize as *mut u8;
}
pub(crate) unsafe extern "C" fn Cmd_if_user_has_attacking_move() {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_MON_MOVES {
        if gBattleMons[sBattler_AI].moves[i] != 0
            && gBattleMoves[gBattleMons[sBattler_AI].moves[i]].power != 0
        {
            break;
        }
        i += 1;
    }
    if i == MAX_MON_MOVES {
        gAIScriptPtr = gAIScriptPtr.at(5);
    } else {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_user_has_no_attacking_moves() {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_MON_MOVES {
        if gBattleMons[sBattler_AI].moves[i] != 0
            && gBattleMoves[gBattleMons[sBattler_AI].moves[i]].power != 0
        {
            break;
        }
        i += 1;
    }
    if i != MAX_MON_MOVES {
        gAIScriptPtr = gAIScriptPtr.at(5);
    } else {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_turn_count() {
    (*(*gBattleResources).ai).funcResult = gBattleResults.battleTurnCounter as u32;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_get_type() {
    let mut typeVar: u8 = *gAIScriptPtr.at(1);
    match typeVar {
        AI_TYPE1_USER => {
            (*(*gBattleResources).ai).funcResult = gBattleMons[sBattler_AI].types[0] as u32;
        }
        AI_TYPE1_TARGET => {
            (*(*gBattleResources).ai).funcResult = gBattleMons[gBattlerTarget].types[0] as u32;
        }
        AI_TYPE2_USER => {
            (*(*gBattleResources).ai).funcResult = gBattleMons[sBattler_AI].types[1] as u32;
        }
        AI_TYPE2_TARGET => {
            (*(*gBattleResources).ai).funcResult = gBattleMons[gBattlerTarget].types[1] as u32;
        }
        AI_TYPE_MOVE => {
            (*(*gBattleResources).ai).funcResult =
                gBattleMoves[(*(*gBattleResources).ai).moveConsidered].r#type as u32;
        }
        _ => {}
    }
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn BattleAI_GetWantedBattler(wantedBattler: u8) -> u8 {
    match wantedBattler {
        AI_USER => {
            return sBattler_AI;
        }
        AI_USER_PARTNER => {
            return sBattler_AI ^ 2;
        }
        AI_TARGET_PARTNER => {
            return gBattlerTarget ^ 2;
        }
        _ => {
            return gBattlerTarget;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Cmd_is_of_type() {
    let mut battler: u8 = BattleAI_GetWantedBattler(*gAIScriptPtr.at(1));
    if gBattleMons[battler].types[0] == *gAIScriptPtr.at(2)
        || gBattleMons[battler].types[1] == *gAIScriptPtr.at(2)
    {
        (*(*gBattleResources).ai).funcResult = TRUE as u32;
    } else {
        (*(*gBattleResources).ai).funcResult = FALSE as u32;
    }
    gAIScriptPtr = gAIScriptPtr.at(3);
}
pub(crate) unsafe extern "C" fn Cmd_get_considered_move_power() {
    (*(*gBattleResources).ai).funcResult =
        gBattleMoves[(*(*gBattleResources).ai).moveConsidered].power as u32;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_get_how_powerful_move_is() {
    let mut i: i32 = 0;
    let mut checkedMove: i32 = 0;
    let mut moveDmgs: CArray<i32, 4> = zeroed();
    i = 0;
    while sIgnoredPowerfulMoveEffects[i] != IGNORED_MOVES_END {
        if gBattleMoves[(*(*gBattleResources).ai).moveConsidered].effect as u16
            == sIgnoredPowerfulMoveEffects[i]
        {
            break;
        }
        i += 1;
    }
    if gBattleMoves[(*(*gBattleResources).ai).moveConsidered].power > 1
        && sIgnoredPowerfulMoveEffects[i] == IGNORED_MOVES_END
    {
        gDynamicBasePower = 0;
        *(&raw mut (*gBattleStruct).dynamicMoveType) = 0;
        gBattleScripting.dmgMultiplier = 1;
        gMoveResultFlags = 0;
        gCritMultiplier = 1;
        checkedMove = 0;
        while checkedMove < MAX_MON_MOVES {
            i = 0;
            while sIgnoredPowerfulMoveEffects[i] != IGNORED_MOVES_END {
                if gBattleMoves[gBattleMons[sBattler_AI].moves[checkedMove]].effect as u16
                    == sIgnoredPowerfulMoveEffects[i]
                {
                    break;
                }
                i += 1;
            }
            if gBattleMons[sBattler_AI].moves[checkedMove] != MOVE_NONE
                && sIgnoredPowerfulMoveEffects[i] == IGNORED_MOVES_END
                && gBattleMoves[gBattleMons[sBattler_AI].moves[checkedMove]].power > 1
            {
                gCurrentMove = gBattleMons[sBattler_AI].moves[checkedMove];
                AI_CalcDmg(sBattler_AI, gBattlerTarget);
                TypeCalc(gCurrentMove, sBattler_AI, gBattlerTarget);
                moveDmgs[checkedMove] = gBattleMoveDamage
                    * (*(*gBattleResources).ai).simulatedRNG[checkedMove] as i32
                    / 100;
                if moveDmgs[checkedMove] == 0 {
                    moveDmgs[checkedMove] = 1;
                }
            } else {
                moveDmgs[checkedMove] = 0;
            }
            checkedMove += 1;
        }
        checkedMove = 0;
        while checkedMove < MAX_MON_MOVES {
            if moveDmgs[checkedMove] > moveDmgs[(*(*gBattleResources).ai).movesetIndex] {
                break;
            }
            checkedMove += 1;
        }
        if checkedMove == MAX_MON_MOVES {
            (*(*gBattleResources).ai).funcResult = MOVE_MOST_POWERFUL;
        } else {
            (*(*gBattleResources).ai).funcResult = MOVE_NOT_MOST_POWERFUL;
        }
    } else {
        (*(*gBattleResources).ai).funcResult = MOVE_POWER_OTHER;
    }
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_get_last_used_battler_move() {
    if *gAIScriptPtr.at(1) == 1 {
        (*(*gBattleResources).ai).funcResult = gLastMoves[sBattler_AI] as u32;
    } else {
        (*(*gBattleResources).ai).funcResult = gLastMoves[gBattlerTarget] as u32;
    }
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_if_equal_() {
    if *gAIScriptPtr.at(1) as u32 == (*(*gBattleResources).ai).funcResult {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_equal_() {
    if *gAIScriptPtr.at(1) as u32 != (*(*gBattleResources).ai).funcResult {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_user_goes() {
    if GetWhoStrikesFirst(sBattler_AI, gBattlerTarget, 1) == *gAIScriptPtr.at(1) {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_user_doesnt_go() {
    if GetWhoStrikesFirst(sBattler_AI, gBattlerTarget, 1) != *gAIScriptPtr.at(1) {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_nop_2A() {}
pub(crate) unsafe extern "C" fn Cmd_nop_2B() {}
pub(crate) unsafe extern "C" fn Cmd_count_usable_party_mons() {
    let mut battler: u8 = 0;
    let mut battlerOnField1: u8 = 0;
    let mut battlerOnField2: u8 = 0;
    let mut party: *mut Pokemon = null_mut();
    let mut i: i32 = 0;
    (*(*gBattleResources).ai).funcResult = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI;
    } else {
        battler = gBattlerTarget;
    }
    if GetBattlerSide(battler) == B_SIDE_PLAYER {
        party = gPlayerParty.as_mut_ptr();
    } else {
        party = gEnemyParty.as_mut_ptr();
    }
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
        let mut position: u32 = 0;
        battlerOnField1 = gBattlerPartyIndexes[battler] as u8;
        position = GetBattlerPosition(battler) as u32 ^ 2;
        battlerOnField2 = gBattlerPartyIndexes[GetBattlerAtPosition(position as u8)] as u8;
    } else {
        battlerOnField1 = gBattlerPartyIndexes[battler] as u8;
        battlerOnField2 = gBattlerPartyIndexes[battler] as u8;
    }
    i = 0;
    while i < PARTY_SIZE {
        if i != battlerOnField1 as i32
            && i != battlerOnField2 as i32
            && GetMonData2(party.at(i), MON_DATA_HP) != 0
            && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_NONE as u32
            && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_EGG
        {
            (*(*gBattleResources).ai).funcResult += 1;
        }
        i += 1;
    }
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_get_considered_move() {
    (*(*gBattleResources).ai).funcResult = (*(*gBattleResources).ai).moveConsidered as u32;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_get_considered_move_effect() {
    (*(*gBattleResources).ai).funcResult =
        gBattleMoves[(*(*gBattleResources).ai).moveConsidered].effect as u32;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_get_ability() {
    let mut battler: u8 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI;
    } else {
        battler = gBattlerTarget;
    }
    if gActiveBattler != battler {
        if (*(*gBattleResources).battleHistory).abilities[battler] != 0 {
            (*(*gBattleResources).ai).funcResult =
                (*(*gBattleResources).battleHistory).abilities[battler] as u32;
            gAIScriptPtr = gAIScriptPtr.at(2);
            return;
        }
        if gBattleMons[battler].ability == ABILITY_SHADOW_TAG
            || gBattleMons[battler].ability == ABILITY_MAGNET_PULL
            || gBattleMons[battler].ability == ABILITY_ARENA_TRAP
        {
            (*(*gBattleResources).ai).funcResult = gBattleMons[battler].ability as u32;
            gAIScriptPtr = gAIScriptPtr.at(2);
            return;
        }
        if gSpeciesInfo[gBattleMons[battler].species].abilities[0] != 0 {
            if gSpeciesInfo[gBattleMons[battler].species].abilities[1] != ABILITY_NONE {
                if Random() as i32 & 1 != 0 {
                    (*(*gBattleResources).ai).funcResult =
                        gSpeciesInfo[gBattleMons[battler].species].abilities[0] as u32;
                } else {
                    (*(*gBattleResources).ai).funcResult =
                        gSpeciesInfo[gBattleMons[battler].species].abilities[1] as u32;
                }
            } else {
                (*(*gBattleResources).ai).funcResult =
                    gSpeciesInfo[gBattleMons[battler].species].abilities[0] as u32;
            }
        } else {
            (*(*gBattleResources).ai).funcResult =
                gSpeciesInfo[gBattleMons[battler].species].abilities[1] as u32;
        }
    } else {
        (*(*gBattleResources).ai).funcResult = gBattleMons[battler].ability as u32;
    }
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_check_ability() {
    let mut battler: u32 = BattleAI_GetWantedBattler(*gAIScriptPtr.at(1)) as u32;
    let mut ability: u32 = *gAIScriptPtr.at(2) as u32;
    if *gAIScriptPtr.at(1) == AI_TARGET || *gAIScriptPtr.at(1) == AI_TARGET_PARTNER {
        if (*(*gBattleResources).battleHistory).abilities[battler] != ABILITY_NONE {
            ability = (*(*gBattleResources).battleHistory).abilities[battler] as u32;
            (*(*gBattleResources).ai).funcResult = ability;
        } else if gBattleMons[battler].ability == ABILITY_SHADOW_TAG
            || gBattleMons[battler].ability == ABILITY_MAGNET_PULL
            || gBattleMons[battler].ability == ABILITY_ARENA_TRAP
        {
            ability = gBattleMons[battler].ability as u32;
        } else if gSpeciesInfo[gBattleMons[battler].species].abilities[0] != 0 {
            if gSpeciesInfo[gBattleMons[battler].species].abilities[1] != ABILITY_NONE {
                let mut abilityDummyVariable: u8 = ability as u8;
                if gSpeciesInfo[gBattleMons[battler].species].abilities[0] != abilityDummyVariable
                    && gSpeciesInfo[gBattleMons[battler].species].abilities[1]
                        != abilityDummyVariable
                {
                    ability = gSpeciesInfo[gBattleMons[battler].species].abilities[0] as u32;
                } else {
                    ability = ABILITY_NONE as u32;
                }
            } else {
                ability = gSpeciesInfo[gBattleMons[battler].species].abilities[0] as u32;
            }
        } else {
            ability = gSpeciesInfo[gBattleMons[battler].species].abilities[1] as u32;
        }
    } else {
        ability = gBattleMons[battler].ability as u32;
    }
    if ability == 0 {
        (*(*gBattleResources).ai).funcResult = 2;
    } else if ability == *gAIScriptPtr.at(2) as u32 {
        (*(*gBattleResources).ai).funcResult = 1;
    } else {
        (*(*gBattleResources).ai).funcResult = 0;
    }
    gAIScriptPtr = gAIScriptPtr.at(3);
}
pub(crate) unsafe extern "C" fn Cmd_get_highest_type_effectiveness() {
    let mut i: i32 = 0;
    let mut dynamicMoveType: *mut u8 = null_mut();
    gDynamicBasePower = 0;
    dynamicMoveType = &raw mut (*gBattleStruct).dynamicMoveType;
    *dynamicMoveType = 0;
    gBattleScripting.dmgMultiplier = 1;
    gMoveResultFlags = 0;
    gCritMultiplier = 1;
    (*(*gBattleResources).ai).funcResult = 0;
    i = 0;
    while i < MAX_MON_MOVES {
        gBattleMoveDamage = 40;
        gCurrentMove = gBattleMons[sBattler_AI].moves[i];
        if gCurrentMove != MOVE_NONE {
            TypeCalc(gCurrentMove, sBattler_AI, gBattlerTarget);
            if gBattleMoveDamage == 120 {
                gBattleMoveDamage = AI_EFFECTIVENESS_x2;
            }
            if gBattleMoveDamage == 240 {
                gBattleMoveDamage = AI_EFFECTIVENESS_x4;
            }
            if gBattleMoveDamage == 30 {
                gBattleMoveDamage = AI_EFFECTIVENESS_x0_5;
            }
            if gBattleMoveDamage == 15 {
                gBattleMoveDamage = AI_EFFECTIVENESS_x0_25;
            }
            if gMoveResultFlags as i32 & MOVE_RESULT_DOESNT_AFFECT_FOE as i32 != 0 {
                gBattleMoveDamage = AI_EFFECTIVENESS_x0;
            }
            if (*(*gBattleResources).ai).funcResult < gBattleMoveDamage as u32 {
                (*(*gBattleResources).ai).funcResult = gBattleMoveDamage as u32;
            }
        }
        i += 1;
    }
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_if_type_effectiveness() {
    let mut damageVar: u8 = 0;
    gDynamicBasePower = 0;
    (*gBattleStruct).dynamicMoveType = 0;
    gBattleScripting.dmgMultiplier = 1;
    gMoveResultFlags = 0;
    gCritMultiplier = 1;
    gBattleMoveDamage = AI_EFFECTIVENESS_x1;
    gCurrentMove = (*(*gBattleResources).ai).moveConsidered;
    TypeCalc(gCurrentMove, sBattler_AI, gBattlerTarget);
    if gBattleMoveDamage == 120 {
        gBattleMoveDamage = AI_EFFECTIVENESS_x2;
    }
    if gBattleMoveDamage == 240 {
        gBattleMoveDamage = AI_EFFECTIVENESS_x4;
    }
    if gBattleMoveDamage == 30 {
        gBattleMoveDamage = AI_EFFECTIVENESS_x0_5;
    }
    if gBattleMoveDamage == 15 {
        gBattleMoveDamage = AI_EFFECTIVENESS_x0_25;
    }
    if gMoveResultFlags as i32 & MOVE_RESULT_DOESNT_AFFECT_FOE as i32 != 0 {
        gBattleMoveDamage = AI_EFFECTIVENESS_x0;
    }
    damageVar = gBattleMoveDamage as u8;
    if damageVar == *gAIScriptPtr.at(1) {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_nop_32() {}
pub(crate) unsafe extern "C" fn Cmd_nop_33() {}
pub(crate) unsafe extern "C" fn Cmd_if_status_in_party() {
    let mut party: *mut Pokemon = null_mut();
    let mut i: i32 = 0;
    let mut statusToCompareTo: u32 = 0;
    let mut battler: u8 = 0;
    match *gAIScriptPtr.at(1) {
        AI_USER => {
            battler = sBattler_AI;
        }
        _ => {
            battler = gBattlerTarget;
        }
    }
    party = if GetBattlerSide(battler) == B_SIDE_PLAYER {
        gPlayerParty.as_mut_ptr()
    } else {
        gEnemyParty.as_mut_ptr()
    };
    statusToCompareTo = *gAIScriptPtr.at(2) as u32
        | (*gAIScriptPtr.at(2).at(1) as u32) << 8
        | (*gAIScriptPtr.at(2).at(2) as u32) << 16
        | (*gAIScriptPtr.at(2).at(3) as u32) << 24;
    i = 0;
    while i < PARTY_SIZE {
        let mut species: u16 = GetMonData2(party.at(i), MON_DATA_SPECIES) as u16;
        let mut hp: u16 = GetMonData2(party.at(i), MON_DATA_HP) as u16;
        let mut status: u32 = GetMonData2(party.at(i), MON_DATA_STATUS);
        if species != 0 && species != SPECIES_EGG as u16 && hp != 0 && status == statusToCompareTo {
            gAIScriptPtr = (*gAIScriptPtr.at(6) as i32
                | (*gAIScriptPtr.at(6).at(1) as i32) << 8
                | (*gAIScriptPtr.at(6).at(2) as i32) << 16
                | (*gAIScriptPtr.at(6).at(3) as i32) << 24) as usize
                as *mut u8;
            return;
        }
        i += 1;
    }
    gAIScriptPtr = gAIScriptPtr.at(10);
}
pub(crate) unsafe extern "C" fn Cmd_if_status_not_in_party() {
    let mut party: *mut Pokemon = null_mut();
    let mut i: i32 = 0;
    let mut statusToCompareTo: u32 = 0;
    let mut battler: u8 = 0;
    match *gAIScriptPtr.at(1) {
        1 => {
            battler = sBattler_AI;
        }
        _ => {
            battler = gBattlerTarget;
        }
    }
    party = if GetBattlerSide(battler) == B_SIDE_PLAYER {
        gPlayerParty.as_mut_ptr()
    } else {
        gEnemyParty.as_mut_ptr()
    };
    statusToCompareTo = *gAIScriptPtr.at(2) as u32
        | (*gAIScriptPtr.at(2).at(1) as u32) << 8
        | (*gAIScriptPtr.at(2).at(2) as u32) << 16
        | (*gAIScriptPtr.at(2).at(3) as u32) << 24;
    i = 0;
    while i < PARTY_SIZE {
        let mut species: u16 = GetMonData2(party.at(i), MON_DATA_SPECIES) as u16;
        let mut hp: u16 = GetMonData2(party.at(i), MON_DATA_HP) as u16;
        let mut status: u32 = GetMonData2(party.at(i), MON_DATA_STATUS);
        if species != 0 && species != SPECIES_EGG as u16 && hp != 0 && status == statusToCompareTo {
            gAIScriptPtr = gAIScriptPtr.at(10);
            return;
        }
        i += 1;
    }
    gAIScriptPtr = (*gAIScriptPtr.at(6) as i32
        | (*gAIScriptPtr.at(6).at(1) as i32) << 8
        | (*gAIScriptPtr.at(6).at(2) as i32) << 16
        | (*gAIScriptPtr.at(6).at(3) as i32) << 24) as usize as *mut u8;
}
pub(crate) unsafe extern "C" fn Cmd_get_weather() {
    if gBattleWeather as i32 & B_WEATHER_RAIN != 0 {
        (*(*gBattleResources).ai).funcResult = AI_WEATHER_RAIN;
    }
    if gBattleWeather as i32 & B_WEATHER_SANDSTORM != 0 {
        (*(*gBattleResources).ai).funcResult = AI_WEATHER_SANDSTORM;
    }
    if gBattleWeather as i32 & B_WEATHER_SUN != 0 {
        (*(*gBattleResources).ai).funcResult = AI_WEATHER_SUN;
    }
    if gBattleWeather as i32 & B_WEATHER_HAIL != 0 {
        (*(*gBattleResources).ai).funcResult = AI_WEATHER_HAIL;
    }
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_if_effect() {
    if gBattleMoves[(*(*gBattleResources).ai).moveConsidered].effect == *gAIScriptPtr.at(1) {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_effect() {
    if gBattleMoves[(*(*gBattleResources).ai).moveConsidered].effect != *gAIScriptPtr.at(1) {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_stat_level_less_than() {
    let mut battler: u32 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI as u32;
    } else {
        battler = gBattlerTarget as u32;
    }
    if (gBattleMons[battler].statStages[*gAIScriptPtr.at(2)] as i32) < *gAIScriptPtr.at(3) as i32 {
        gAIScriptPtr = (*gAIScriptPtr.at(4) as i32
            | (*gAIScriptPtr.at(4).at(1) as i32) << 8
            | (*gAIScriptPtr.at(4).at(2) as i32) << 16
            | (*gAIScriptPtr.at(4).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(8);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_stat_level_more_than() {
    let mut battler: u32 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI as u32;
    } else {
        battler = gBattlerTarget as u32;
    }
    if gBattleMons[battler].statStages[*gAIScriptPtr.at(2)] as i32 > *gAIScriptPtr.at(3) as i32 {
        gAIScriptPtr = (*gAIScriptPtr.at(4) as i32
            | (*gAIScriptPtr.at(4).at(1) as i32) << 8
            | (*gAIScriptPtr.at(4).at(2) as i32) << 16
            | (*gAIScriptPtr.at(4).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(8);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_stat_level_equal() {
    let mut battler: u32 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI as u32;
    } else {
        battler = gBattlerTarget as u32;
    }
    if gBattleMons[battler].statStages[*gAIScriptPtr.at(2)] as i32 == *gAIScriptPtr.at(3) as i32 {
        gAIScriptPtr = (*gAIScriptPtr.at(4) as i32
            | (*gAIScriptPtr.at(4).at(1) as i32) << 8
            | (*gAIScriptPtr.at(4).at(2) as i32) << 16
            | (*gAIScriptPtr.at(4).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(8);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_stat_level_not_equal() {
    let mut battler: u32 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI as u32;
    } else {
        battler = gBattlerTarget as u32;
    }
    if gBattleMons[battler].statStages[*gAIScriptPtr.at(2)] as i32 != *gAIScriptPtr.at(3) as i32 {
        gAIScriptPtr = (*gAIScriptPtr.at(4) as i32
            | (*gAIScriptPtr.at(4).at(1) as i32) << 8
            | (*gAIScriptPtr.at(4).at(2) as i32) << 16
            | (*gAIScriptPtr.at(4).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(8);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_can_faint() {
    if gBattleMoves[(*(*gBattleResources).ai).moveConsidered].power < 2 {
        gAIScriptPtr = gAIScriptPtr.at(5);
        return;
    }
    gDynamicBasePower = 0;
    (*gBattleStruct).dynamicMoveType = 0;
    gBattleScripting.dmgMultiplier = 1;
    gMoveResultFlags = 0;
    gCritMultiplier = 1;
    gCurrentMove = (*(*gBattleResources).ai).moveConsidered;
    AI_CalcDmg(sBattler_AI, gBattlerTarget);
    TypeCalc(gCurrentMove, sBattler_AI, gBattlerTarget);
    gBattleMoveDamage = gBattleMoveDamage
        * (*(*gBattleResources).ai).simulatedRNG[(*(*gBattleResources).ai).movesetIndex] as i32
        / 100;
    if gBattleMoveDamage == 0 {
        gBattleMoveDamage = 1;
    }
    if gBattleMons[gBattlerTarget].hp as i32 <= gBattleMoveDamage {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_cant_faint() {
    if gBattleMoves[(*(*gBattleResources).ai).moveConsidered].power < 2 {
        gAIScriptPtr = gAIScriptPtr.at(5);
        return;
    }
    gDynamicBasePower = 0;
    (*gBattleStruct).dynamicMoveType = 0;
    gBattleScripting.dmgMultiplier = 1;
    gMoveResultFlags = 0;
    gCritMultiplier = 1;
    gCurrentMove = (*(*gBattleResources).ai).moveConsidered;
    AI_CalcDmg(sBattler_AI, gBattlerTarget);
    TypeCalc(gCurrentMove, sBattler_AI, gBattlerTarget);
    gBattleMoveDamage = gBattleMoveDamage
        * (*(*gBattleResources).ai).simulatedRNG[(*(*gBattleResources).ai).movesetIndex] as i32
        / 100;
    if gBattleMons[gBattlerTarget].hp as i32 > gBattleMoveDamage {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_has_move() {
    let mut i: i32 = 0;
    let mut movePtr: *mut u16 = gAIScriptPtr.at(2) as *mut u16;
    'l1: {
        match *gAIScriptPtr.at(1) {
            AI_USER => {
                i = 0;
                while i < MAX_MON_MOVES {
                    if gBattleMons[sBattler_AI].moves[i] == *movePtr {
                        break;
                    }
                    i += 1;
                }
                if i == MAX_MON_MOVES {
                    gAIScriptPtr = gAIScriptPtr.at(8);
                } else {
                    gAIScriptPtr = (*gAIScriptPtr.at(4) as i32
                        | (*gAIScriptPtr.at(4).at(1) as i32) << 8
                        | (*gAIScriptPtr.at(4).at(2) as i32) << 16
                        | (*gAIScriptPtr.at(4).at(3) as i32) << 24)
                        as usize as *mut u8;
                }
            }
            AI_USER_PARTNER => {
                if gBattleMons[sBattler_AI as i32 ^ 2].hp == 0 {
                    gAIScriptPtr = gAIScriptPtr.at(8);
                    break 'l1;
                } else {
                    i = 0;
                    while i < MAX_MON_MOVES {
                        if gBattleMons[sBattler_AI as i32 ^ 2].moves[i] == *movePtr {
                            break;
                        }
                        i += 1;
                    }
                }
                if i == MAX_MON_MOVES {
                    gAIScriptPtr = gAIScriptPtr.at(8);
                } else {
                    gAIScriptPtr = (*gAIScriptPtr.at(4) as i32
                        | (*gAIScriptPtr.at(4).at(1) as i32) << 8
                        | (*gAIScriptPtr.at(4).at(2) as i32) << 16
                        | (*gAIScriptPtr.at(4).at(3) as i32) << 24)
                        as usize as *mut u8;
                }
            }
            AI_TARGET | AI_TARGET_PARTNER => {
                i = 0;
                while i < MAX_MON_MOVES {
                    if (*(*gBattleResources).battleHistory).usedMoves[gBattlerTarget].moves[i]
                        == *movePtr
                    {
                        break;
                    }
                    i += 1;
                }
                if i == MAX_MON_MOVES {
                    gAIScriptPtr = gAIScriptPtr.at(8);
                } else {
                    gAIScriptPtr = (*gAIScriptPtr.at(4) as i32
                        | (*gAIScriptPtr.at(4).at(1) as i32) << 8
                        | (*gAIScriptPtr.at(4).at(2) as i32) << 16
                        | (*gAIScriptPtr.at(4).at(3) as i32) << 24)
                        as usize as *mut u8;
                }
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_doesnt_have_move() {
    let mut i: i32 = 0;
    let mut movePtr: *mut u16 = gAIScriptPtr.at(2) as *mut u16;
    match *gAIScriptPtr.at(1) {
        AI_USER | AI_USER_PARTNER => {
            i = 0;
            while i < MAX_MON_MOVES {
                if gBattleMons[sBattler_AI].moves[i] == *movePtr {
                    break;
                }
                i += 1;
            }
            if i != MAX_MON_MOVES {
                gAIScriptPtr = gAIScriptPtr.at(8);
            } else {
                gAIScriptPtr = (*gAIScriptPtr.at(4) as i32
                    | (*gAIScriptPtr.at(4).at(1) as i32) << 8
                    | (*gAIScriptPtr.at(4).at(2) as i32) << 16
                    | (*gAIScriptPtr.at(4).at(3) as i32) << 24)
                    as usize as *mut u8;
            }
        }
        AI_TARGET | AI_TARGET_PARTNER => {
            i = 0;
            while i < MAX_MON_MOVES {
                if (*(*gBattleResources).battleHistory).usedMoves[gBattlerTarget].moves[i]
                    == *movePtr
                {
                    break;
                }
                i += 1;
            }
            if i != MAX_MON_MOVES {
                gAIScriptPtr = gAIScriptPtr.at(8);
            } else {
                gAIScriptPtr = (*gAIScriptPtr.at(4) as i32
                    | (*gAIScriptPtr.at(4).at(1) as i32) << 8
                    | (*gAIScriptPtr.at(4).at(2) as i32) << 16
                    | (*gAIScriptPtr.at(4).at(3) as i32) << 24)
                    as usize as *mut u8;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_has_move_with_effect() {
    let mut i: i32 = 0;
    match *gAIScriptPtr.at(1) {
        AI_USER | AI_USER_PARTNER => {
            i = 0;
            while i < MAX_MON_MOVES {
                if gBattleMons[sBattler_AI].moves[i] != 0
                    && gBattleMoves[gBattleMons[sBattler_AI].moves[i]].effect == *gAIScriptPtr.at(2)
                {
                    break;
                }
                i += 1;
            }
            if i == MAX_MON_MOVES {
                gAIScriptPtr = gAIScriptPtr.at(7);
            } else {
                gAIScriptPtr = (*gAIScriptPtr.at(3) as i32
                    | (*gAIScriptPtr.at(3).at(1) as i32) << 8
                    | (*gAIScriptPtr.at(3).at(2) as i32) << 16
                    | (*gAIScriptPtr.at(3).at(3) as i32) << 24)
                    as usize as *mut u8;
            }
        }
        AI_TARGET | AI_TARGET_PARTNER => {
            i = 0;
            while i < MAX_MON_MOVES {
                if gBattleMons[sBattler_AI].moves[i] != 0
                    && gBattleMoves
                        [(*(*gBattleResources).battleHistory).usedMoves[gBattlerTarget].moves[i]]
                        .effect
                        == *gAIScriptPtr.at(2)
                {
                    break;
                }
                i += 1;
            }
            if i == MAX_MON_MOVES {
                gAIScriptPtr = gAIScriptPtr.at(7);
            } else {
                gAIScriptPtr = (*gAIScriptPtr.at(3) as i32
                    | (*gAIScriptPtr.at(3).at(1) as i32) << 8
                    | (*gAIScriptPtr.at(3).at(2) as i32) << 16
                    | (*gAIScriptPtr.at(3).at(3) as i32) << 24)
                    as usize as *mut u8;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_doesnt_have_move_with_effect() {
    let mut i: i32 = 0;
    match *gAIScriptPtr.at(1) {
        AI_USER | AI_USER_PARTNER => {
            i = 0;
            while i < MAX_MON_MOVES {
                if gBattleMons[sBattler_AI].moves[i] != 0
                    && gBattleMoves[gBattleMons[sBattler_AI].moves[i]].effect == *gAIScriptPtr.at(2)
                {
                    break;
                }
                i += 1;
            }
            if i != MAX_MON_MOVES {
                gAIScriptPtr = gAIScriptPtr.at(7);
            } else {
                gAIScriptPtr = (*gAIScriptPtr.at(3) as i32
                    | (*gAIScriptPtr.at(3).at(1) as i32) << 8
                    | (*gAIScriptPtr.at(3).at(2) as i32) << 16
                    | (*gAIScriptPtr.at(3).at(3) as i32) << 24)
                    as usize as *mut u8;
            }
        }
        AI_TARGET | AI_TARGET_PARTNER => {
            i = 0;
            while i < MAX_MON_MOVES {
                if (*(*gBattleResources).battleHistory).usedMoves[gBattlerTarget].moves[i] != 0
                    && gBattleMoves
                        [(*(*gBattleResources).battleHistory).usedMoves[gBattlerTarget].moves[i]]
                        .effect
                        == *gAIScriptPtr.at(2)
                {
                    break;
                }
                i += 1;
            }
            if i != MAX_MON_MOVES {
                gAIScriptPtr = gAIScriptPtr.at(7);
            } else {
                gAIScriptPtr = (*gAIScriptPtr.at(3) as i32
                    | (*gAIScriptPtr.at(3).at(1) as i32) << 8
                    | (*gAIScriptPtr.at(3).at(2) as i32) << 16
                    | (*gAIScriptPtr.at(3).at(3) as i32) << 24)
                    as usize as *mut u8;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_any_move_disabled_or_encored() {
    let mut battler: u8 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI;
    } else {
        battler = gBattlerTarget;
    }
    if *gAIScriptPtr.at(2) == 0 {
        if gDisableStructs[battler].disabledMove == MOVE_NONE {
            gAIScriptPtr = gAIScriptPtr.at(7);
        } else {
            gAIScriptPtr = (*gAIScriptPtr.at(3) as i32
                | (*gAIScriptPtr.at(3).at(1) as i32) << 8
                | (*gAIScriptPtr.at(3).at(2) as i32) << 16
                | (*gAIScriptPtr.at(3).at(3) as i32) << 24) as usize
                as *mut u8;
        }
    } else if *gAIScriptPtr.at(2) != 1 {
        gAIScriptPtr = gAIScriptPtr.at(7);
    } else {
        if gDisableStructs[battler].encoredMove != MOVE_NONE {
            gAIScriptPtr = (*gAIScriptPtr.at(3) as i32
                | (*gAIScriptPtr.at(3).at(1) as i32) << 8
                | (*gAIScriptPtr.at(3).at(2) as i32) << 16
                | (*gAIScriptPtr.at(3).at(3) as i32) << 24) as usize
                as *mut u8;
        } else {
            gAIScriptPtr = gAIScriptPtr.at(7);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_curr_move_disabled_or_encored() {
    match *gAIScriptPtr.at(1) {
        0 => {
            if gDisableStructs[gActiveBattler].disabledMove
                == (*(*gBattleResources).ai).moveConsidered
            {
                gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
                    | (*gAIScriptPtr.at(2).at(1) as i32) << 8
                    | (*gAIScriptPtr.at(2).at(2) as i32) << 16
                    | (*gAIScriptPtr.at(2).at(3) as i32) << 24)
                    as usize as *mut u8;
            } else {
                gAIScriptPtr = gAIScriptPtr.at(6);
            }
        }
        1 => {
            if gDisableStructs[gActiveBattler].encoredMove
                == (*(*gBattleResources).ai).moveConsidered
            {
                gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
                    | (*gAIScriptPtr.at(2).at(1) as i32) << 8
                    | (*gAIScriptPtr.at(2).at(2) as i32) << 16
                    | (*gAIScriptPtr.at(2).at(3) as i32) << 24)
                    as usize as *mut u8;
            } else {
                gAIScriptPtr = gAIScriptPtr.at(6);
            }
        }
        _ => {
            gAIScriptPtr = gAIScriptPtr.at(6);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_flee() {
    (*(*gBattleResources).ai).aiAction |= 11;
}
pub(crate) unsafe extern "C" fn Cmd_if_random_safari_flee() {
    let mut safariFleeRate: u8 = (*gBattleStruct).safariEscapeFactor * 5;
    if ((Random() as i32 % 100) as u8) < safariFleeRate {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_watch() {
    (*(*gBattleResources).ai).aiAction |= 13;
}
pub(crate) unsafe extern "C" fn Cmd_get_hold_effect() {
    let mut battler: u8 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI;
    } else {
        battler = gBattlerTarget;
    }
    if gActiveBattler != battler {
        (*(*gBattleResources).ai).funcResult =
            GetItemHoldEffect((*(*gBattleResources).battleHistory).itemEffects[battler] as u16)
                as u32;
    } else {
        (*(*gBattleResources).ai).funcResult = GetItemHoldEffect(gBattleMons[battler].item) as u32;
    }
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_if_holds_item() {
    let mut battler: u8 = BattleAI_GetWantedBattler(*gAIScriptPtr.at(1));
    let mut item: u16 = 0;
    let mut itemLo: u8 = 0;
    let mut itemHi: u8 = 0;
    if battler as i32 & BIT_SIDE as i32 == sBattler_AI as i32 & BIT_SIDE as i32 {
        item = gBattleMons[battler].item;
    } else {
        item = (*(*gBattleResources).battleHistory).itemEffects[battler] as u16;
    }
    itemHi = *gAIScriptPtr.at(2);
    itemLo = *gAIScriptPtr.at(3);
    if itemLo as i32 | itemHi as i32 == item as i32 {
        gAIScriptPtr = (*gAIScriptPtr.at(4) as i32
            | (*gAIScriptPtr.at(4).at(1) as i32) << 8
            | (*gAIScriptPtr.at(4).at(2) as i32) << 16
            | (*gAIScriptPtr.at(4).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(8);
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_gender() {
    let mut battler: u8 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI;
    } else {
        battler = gBattlerTarget;
    }
    (*(*gBattleResources).ai).funcResult = GetGenderFromSpeciesAndPersonality(
        gBattleMons[battler].species,
        gBattleMons[battler].personality,
    ) as u32;
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_is_first_turn_for() {
    let mut battler: u8 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI;
    } else {
        battler = gBattlerTarget;
    }
    (*(*gBattleResources).ai).funcResult = gDisableStructs[battler].isFirstTurn as u32;
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_get_stockpile_count() {
    let mut battler: u8 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI;
    } else {
        battler = gBattlerTarget;
    }
    (*(*gBattleResources).ai).funcResult = gDisableStructs[battler].stockpileCounter as u32;
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_is_double_battle() {
    (*(*gBattleResources).ai).funcResult = gBattleTypeFlags & BATTLE_TYPE_DOUBLE;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_get_used_held_item() {
    let mut battler: u8 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI;
    } else {
        battler = gBattlerTarget;
    }
    (*(*gBattleResources).ai).funcResult =
        *(&raw mut (*gBattleStruct).usedHeldItems[battler] as *mut u8) as u32;
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_get_move_type_from_result() {
    (*(*gBattleResources).ai).funcResult =
        gBattleMoves[(*(*gBattleResources).ai).funcResult].r#type as u32;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_get_move_power_from_result() {
    (*(*gBattleResources).ai).funcResult =
        gBattleMoves[(*(*gBattleResources).ai).funcResult].power as u32;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_get_move_effect_from_result() {
    (*(*gBattleResources).ai).funcResult =
        gBattleMoves[(*(*gBattleResources).ai).funcResult].effect as u32;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_get_protect_count() {
    let mut battler: u8 = 0;
    if *gAIScriptPtr.at(1) == 1 {
        battler = sBattler_AI;
    } else {
        battler = gBattlerTarget;
    }
    (*(*gBattleResources).ai).funcResult = gDisableStructs[battler].protectUses as u32;
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_nop_52() {}
pub(crate) unsafe extern "C" fn Cmd_nop_53() {}
pub(crate) unsafe extern "C" fn Cmd_nop_54() {}
pub(crate) unsafe extern "C" fn Cmd_nop_55() {}
pub(crate) unsafe extern "C" fn Cmd_nop_56() {}
pub(crate) unsafe extern "C" fn Cmd_nop_57() {}
pub(crate) unsafe extern "C" fn Cmd_call() {
    AIStackPushVar(gAIScriptPtr.at(5));
    gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
        | (*gAIScriptPtr.at(1).at(1) as i32) << 8
        | (*gAIScriptPtr.at(1).at(2) as i32) << 16
        | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
}
pub(crate) unsafe extern "C" fn Cmd_goto() {
    gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
        | (*gAIScriptPtr.at(1).at(1) as i32) << 8
        | (*gAIScriptPtr.at(1).at(2) as i32) << 16
        | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
}
pub(crate) unsafe extern "C" fn Cmd_end() {
    if AIStackPop() == 0 {
        (*(*gBattleResources).ai).aiAction |= AI_ACTION_DONE;
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_level_cond() {
    match *gAIScriptPtr.at(1) {
        0 => {
            if gBattleMons[sBattler_AI].level > gBattleMons[gBattlerTarget].level {
                gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
                    | (*gAIScriptPtr.at(2).at(1) as i32) << 8
                    | (*gAIScriptPtr.at(2).at(2) as i32) << 16
                    | (*gAIScriptPtr.at(2).at(3) as i32) << 24)
                    as usize as *mut u8;
            } else {
                gAIScriptPtr = gAIScriptPtr.at(6);
            }
        }
        1 => {
            if gBattleMons[sBattler_AI].level < gBattleMons[gBattlerTarget].level {
                gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
                    | (*gAIScriptPtr.at(2).at(1) as i32) << 8
                    | (*gAIScriptPtr.at(2).at(2) as i32) << 16
                    | (*gAIScriptPtr.at(2).at(3) as i32) << 24)
                    as usize as *mut u8;
            } else {
                gAIScriptPtr = gAIScriptPtr.at(6);
            }
        }
        2 => {
            if gBattleMons[sBattler_AI].level == gBattleMons[gBattlerTarget].level {
                gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
                    | (*gAIScriptPtr.at(2).at(1) as i32) << 8
                    | (*gAIScriptPtr.at(2).at(2) as i32) << 16
                    | (*gAIScriptPtr.at(2).at(3) as i32) << 24)
                    as usize as *mut u8;
            } else {
                gAIScriptPtr = gAIScriptPtr.at(6);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_target_taunted() {
    if gDisableStructs[gBattlerTarget].tauntTimer() != 0 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_target_not_taunted() {
    if gDisableStructs[gBattlerTarget].tauntTimer() == 0 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_target_is_ally() {
    if sBattler_AI as i32 & BIT_SIDE as i32 == gBattlerTarget as i32 & BIT_SIDE as i32 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_flash_fired() {
    let mut battler: u8 = BattleAI_GetWantedBattler(*gAIScriptPtr.at(1));
    if (*(*gBattleResources).flags).flags[battler] & RESOURCE_FLAG_FLASH_FIRE != 0 {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn AIStackPushVar(var: *mut u8) {
    (*(*gBattleResources).AI_ScriptsStack).ptr[{
        let t1 = (*(*gBattleResources).AI_ScriptsStack).size;
        (*(*gBattleResources).AI_ScriptsStack).size += 1;
        t1
    }] = var;
}
pub(crate) unsafe extern "C" fn AIStackPushVar_cursor() {
    (*(*gBattleResources).AI_ScriptsStack).ptr[{
        let t1 = (*(*gBattleResources).AI_ScriptsStack).size;
        (*(*gBattleResources).AI_ScriptsStack).size += 1;
        t1
    }] = gAIScriptPtr;
}
pub(crate) unsafe extern "C" fn AIStackPop() -> u8 {
    if (*(*gBattleResources).AI_ScriptsStack).size != 0 {
        (*(*gBattleResources).AI_ScriptsStack).size -= 1;
        gAIScriptPtr =
            (*(*gBattleResources).AI_ScriptsStack).ptr[(*(*gBattleResources).AI_ScriptsStack).size];
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
