//! Translated from `src/battle_util.c` by tools/rustport/c2rs.py.
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
    clippy::eq_op,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::battle_ai_script_commands::{RecordAbilityBattle, RecordItemEffectBattle};
use crate::battle_anim_mons::{GetBattlerAtPosition, GetBattlerPosition, GetBattlerSide};
use crate::battle_arena::BattleArena_AddMindPoints;
use crate::battle_controllers::{BtlController_EmitPrintString, BtlController_EmitSetMonData};
use crate::battle_main::{
    BattleTurnPassed, GetWhoStrikesFirst, RunBattleScriptCommands,
    RunBattleScriptCommands_PopCallbacksStack, SpecialStatusesClear, SwapTurnOrder,
    gAbsentBattlerFlags, gActiveBattler, gBattle_BG0_X, gBattle_BG0_Y, gBattleControllerExecFlags,
    gBattleMainFunc, gBattleMons, gBattleMoveDamage, gBattleOutcome, gBattleResources,
    gBattleResults, gBattleScripting, gBattleStruct, gBattleTypeFlags, gBattleWeather,
    gBattlerAttacker, gBattlerFainted, gBattlerTarget, gBattlersCount, gBattlescriptCurrInstr,
    gBideDmg, gCalledMove, gChosenMove, gChosenMovePos, gCritMultiplier, gCurrMovePos,
    gCurrentActionFuncId, gCurrentMove, gCurrentTurnActionNumber, gDisableStructs,
    gDynamicBasePower, gEffectBattler, gEnigmaBerries, gHitMarker, gLastUsedAbility, gLastUsedItem,
    gMoveResultFlags, gMultiHitCounter, gPalaceSelectionBattleScripts, gPotentialItemEffectBattler,
    gProtectStructs, gSelectionBattleScripts, gSideTimers, gSpecialStatuses, gStatuses3,
    gWishFutureKnock,
};
use crate::battle_main::{
    gActionSelectionCursor, gActionsByTurnOrder, gBattleBufferB, gBattleCommunication,
    gBattleTextBuff1, gBattleTextBuff2, gBattlerByTurnOrder, gBattlerPartyIndexes, gBideTarget,
    gChosenActionByBattler, gChosenMoveByBattler, gLastHitByType, gLastLandedMoves, gLastMoves,
    gLockedMoves, gMoveSelectionCursor, gSentPokesToOpponent, gSideStatuses,
};
use crate::battle_pyramid::{CurrentBattlePyramidLocation, GetPyramidRunMultiplier};
use crate::battle_script_commands::{GetBattlerTurnOrderNum, SetMoveEffect, UproarWakeUpCheck};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::FlagGet;
use crate::field_weather::GetCurrentWeather;
use crate::item::{GetItemHoldEffect, GetItemHoldEffectParam};
use crate::link::GetLinkPlayerCount;
use crate::load_save::gSaveBlock2Ptr;
use crate::pokemon::{
    CalculateBaseDamage, CalculatePPWithBonus, GetBattlerMultiplayerId,
    GetFlavorRelationByPersonality, GetGenderFromSpeciesAndPersonality, GetLinkTrainerFlankId,
    GetMonData2, GetMonData3, IsOtherTrainer, gEnemyParty, gPlayerParty,
};
use crate::random::Random;
use crate::safari_zone::gNumSafariBalls;
use crate::sound::PlaySE;
use crate::string_util::StringCopy;
#[allow(unused_imports)]
use crate::types::*;
use crate::util::CountTrailingZeroBits;
use crate::util::gBitTable;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): sPkblToEscapeFactor sGoNearCounterToCatchFactor sGoNearCounterToEscapeFactor sSoundMovesTable

const CANCELER_ASLEEP: u8 = 1;
const CANCELER_BIDE: u8 = 12;
const CANCELER_CONFUSED: u8 = 9;
const CANCELER_DISABLED: u8 = 6;
const CANCELER_END: u8 = 14;
const CANCELER_FLAGS: u8 = 0;
const CANCELER_FLINCH: u8 = 5;
const CANCELER_FROZEN: u8 = 2;
const CANCELER_IMPRISONED: u8 = 8;
const CANCELER_IN_LOVE: u8 = 11;
const CANCELER_PARALYZED: u8 = 10;
const CANCELER_RECHARGE: u8 = 4;
const CANCELER_TAUNTED: u8 = 7;
const CANCELER_THAW: u8 = 13;
const CANCELER_TRUANT: u8 = 3;
const ENDTURN_ABILITIES: u8 = 1;
const ENDTURN_BAD_POISON: u8 = 5;
const ENDTURN_BATTLER_COUNT: u8 = 19;
const ENDTURN_BURN: u8 = 6;
const ENDTURN_CHARGE: u8 = 15;
const ENDTURN_CURSE: u8 = 8;
const ENDTURN_DISABLE: u8 = 12;
const ENDTURN_ENCORE: u8 = 13;
const ENDTURN_FIELD_COUNT: u8 = 10;
const ENDTURN_HAIL: u8 = 9;
const ENDTURN_INGRAIN: u8 = 0;
const ENDTURN_ITEMS1: u8 = 2;
const ENDTURN_ITEMS2: u8 = 18;
const ENDTURN_LEECH_SEED: u8 = 3;
const ENDTURN_LIGHT_SCREEN: u8 = 2;
const ENDTURN_LOCK_ON: u8 = 14;
const ENDTURN_MIST: u8 = 3;
const ENDTURN_NIGHTMARES: u8 = 7;
const ENDTURN_ORDER: u8 = 0;
const ENDTURN_POISON: u8 = 4;
const ENDTURN_RAIN: u8 = 6;
const ENDTURN_REFLECT: u8 = 1;
const ENDTURN_SAFEGUARD: u8 = 4;
const ENDTURN_SANDSTORM: u8 = 7;
const ENDTURN_SUN: u8 = 8;
const ENDTURN_TAUNT: u8 = 16;
const ENDTURN_THRASH: u8 = 11;
const ENDTURN_UPROAR: u8 = 10;
const ENDTURN_WISH: u8 = 5;
const ENDTURN_WRAP: u8 = 9;
const ENDTURN_YAWN: u8 = 17;
const FAINTED_ACTIONS_MAX_CASE: u8 = 7;
const ITEM_EFFECT_OTHER: u8 = 2;
const ITEM_HP_CHANGE: u8 = 4;
const ITEM_NO_EFFECT: u8 = 0;
const ITEM_PP_CHANGE: u8 = 3;
const ITEM_STATS_CHANGE: u8 = 5;
const ITEM_STATUS_CHANGE: u8 = 1;
const SOUND_MOVES_END: u16 = 65535;

static sGoNearCounterToCatchFactor: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_util::sGoNearCounterToCatchFactor).cast());
static sGoNearCounterToEscapeFactor: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_util::sGoNearCounterToEscapeFactor).cast());
static sPkblToEscapeFactor: Table<CArray<CArray<u8, 3>, 5>> =
    Table((&raw const crate::data::battle_util::sPkblToEscapeFactor).cast());
static sSoundMovesTable: Table<CArray<u16, 11>> =
    Table((&raw const crate::data::battle_util::sSoundMovesTable).cast());

pub unsafe fn HandleAction_UseMove() {
    let mut var: u8 = 4;
    gBattlerAttacker = gBattlerByTurnOrder[gCurrentTurnActionNumber];
    if (*gBattleStruct).absentBattlerFlags as u32
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gBattlerAttacker]
        != 0
    {
        gCurrentActionFuncId = B_ACTION_FINISHED;
        return;
    }
    gCritMultiplier = 1;
    gBattleScripting.dmgMultiplier = 1;
    (*gBattleStruct).atkCancelerTracker = 0;
    gMoveResultFlags = 0;
    gMultiHitCounter = 0;
    gBattleCommunication[6] = 0;
    gCurrMovePos = {
        gChosenMovePos = *(*gBattleStruct)
            .chosenMovePositions
            .as_mut_ptr()
            .at(gBattlerAttacker);
        gChosenMovePos
    };
    if gProtectStructs[gBattlerAttacker].noValidMoves() != 0 {
        gProtectStructs[gBattlerAttacker].set_noValidMoves(FALSE as u32);
        gCurrentMove = {
            gChosenMove = MOVE_STRUGGLE;
            gChosenMove
        };
        gHitMarker |= HITMARKER_NO_PPDEDUCT;
        *(*gBattleStruct)
            .moveTarget
            .as_mut_ptr()
            .at(gBattlerAttacker) = GetMoveTarget(MOVE_STRUGGLE, NO_TARGET_OVERRIDE);
    } else if gBattleMons[gBattlerAttacker].status2 & STATUS2_MULTIPLETURNS != 0
        || gBattleMons[gBattlerAttacker].status2 & STATUS2_RECHARGE != 0
    {
        gCurrentMove = {
            gChosenMove = gLockedMoves[gBattlerAttacker];
            gChosenMove
        };
    } else if gDisableStructs[gBattlerAttacker].encoredMove != MOVE_NONE
        && gDisableStructs[gBattlerAttacker].encoredMove
            == gBattleMons[gBattlerAttacker].moves[gDisableStructs[gBattlerAttacker].encoredMovePos]
    {
        gCurrentMove = {
            gChosenMove = gDisableStructs[gBattlerAttacker].encoredMove;
            gChosenMove
        };
        gCurrMovePos = {
            gChosenMovePos = gDisableStructs[gBattlerAttacker].encoredMovePos;
            gChosenMovePos
        };
        *(*gBattleStruct)
            .moveTarget
            .as_mut_ptr()
            .at(gBattlerAttacker) = GetMoveTarget(gCurrentMove, NO_TARGET_OVERRIDE);
    } else if gDisableStructs[gBattlerAttacker].encoredMove != MOVE_NONE
        && gDisableStructs[gBattlerAttacker].encoredMove
            != gBattleMons[gBattlerAttacker].moves[gDisableStructs[gBattlerAttacker].encoredMovePos]
    {
        gCurrMovePos = {
            gChosenMovePos = gDisableStructs[gBattlerAttacker].encoredMovePos;
            gChosenMovePos
        };
        gCurrentMove = {
            gChosenMove = gBattleMons[gBattlerAttacker].moves[gCurrMovePos];
            gChosenMove
        };
        gDisableStructs[gBattlerAttacker].encoredMove = MOVE_NONE;
        gDisableStructs[gBattlerAttacker].encoredMovePos = 0;
        gDisableStructs[gBattlerAttacker].set_encoreTimer(0);
        *(*gBattleStruct)
            .moveTarget
            .as_mut_ptr()
            .at(gBattlerAttacker) = GetMoveTarget(gCurrentMove, NO_TARGET_OVERRIDE);
    } else if gBattleMons[gBattlerAttacker].moves[gCurrMovePos]
        != gChosenMoveByBattler[gBattlerAttacker]
    {
        gCurrentMove = {
            gChosenMove = gBattleMons[gBattlerAttacker].moves[gCurrMovePos];
            gChosenMove
        };
        *(*gBattleStruct)
            .moveTarget
            .as_mut_ptr()
            .at(gBattlerAttacker) = GetMoveTarget(gCurrentMove, NO_TARGET_OVERRIDE);
    } else {
        gCurrentMove = {
            gChosenMove = gBattleMons[gBattlerAttacker].moves[gCurrMovePos];
            gChosenMove
        };
    }
    if gBattleMons[gBattlerAttacker].hp != 0 {
        if GetBattlerSide(gBattlerAttacker) == B_SIDE_PLAYER {
            gBattleResults.lastUsedMovePlayer = gCurrentMove;
        } else {
            gBattleResults.lastUsedMoveOpponent = gCurrentMove;
        }
    }
    let mut side: u8 = GetBattlerSide(gBattlerAttacker) ^ 1;
    if gSideTimers[side].followmeTimer != 0
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove]
            .target
            == MOVE_TARGET_SELECTED
        && GetBattlerSide(gBattlerAttacker) != GetBattlerSide(gSideTimers[side].followmeTarget)
        && gBattleMons[gSideTimers[side].followmeTarget].hp != 0
    {
        gBattlerTarget = gSideTimers[side].followmeTarget;
    } else if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
        && gSideTimers[side].followmeTimer == 0
        && ((*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove]
            .power
            != 0
            || (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [gCurrentMove]
                .target
                != MOVE_TARGET_USER)
        && gBattleMons[*(*gBattleStruct)
            .moveTarget
            .as_mut_ptr()
            .at(gBattlerAttacker)]
        .ability
            != ABILITY_LIGHTNING_ROD
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove]
            .r#type
            == TYPE_ELECTRIC
    {
        side = GetBattlerSide(gBattlerAttacker);
        gActiveBattler = 0;
        while gActiveBattler < gBattlersCount {
            if side != GetBattlerSide(gActiveBattler)
                && *(*gBattleStruct)
                    .moveTarget
                    .as_mut_ptr()
                    .at(gBattlerAttacker)
                    != gActiveBattler
                && gBattleMons[gActiveBattler].ability == ABILITY_LIGHTNING_ROD
                && GetBattlerTurnOrderNum(gActiveBattler) < var
            {
                var = GetBattlerTurnOrderNum(gActiveBattler);
            }
            gActiveBattler += 1;
        }
        if var == 4 {
            if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [gChosenMove]
                .target as i32
                & MOVE_TARGET_RANDOM
                != 0
            {
                if GetBattlerSide(gBattlerAttacker) == B_SIDE_PLAYER {
                    if Random() as i32 & 1 != 0 {
                        gBattlerTarget = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
                    } else {
                        gBattlerTarget = GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT);
                    }
                } else {
                    if Random() as i32 & 1 != 0 {
                        gBattlerTarget = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
                    } else {
                        gBattlerTarget = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT);
                    }
                }
            } else {
                gBattlerTarget = *(*gBattleStruct)
                    .moveTarget
                    .as_mut_ptr()
                    .at(gBattlerAttacker);
            }
            if gAbsentBattlerFlags as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gBattlerTarget]
                != 0
            {
                if GetBattlerSide(gBattlerAttacker) != GetBattlerSide(gBattlerTarget) {
                    gBattlerTarget = GetBattlerAtPosition(GetBattlerPosition(gBattlerTarget) ^ 2);
                } else {
                    gBattlerTarget = GetBattlerAtPosition(GetBattlerPosition(gBattlerAttacker) ^ 1);
                    if gAbsentBattlerFlags as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                            [gBattlerTarget]
                        != 0
                    {
                        gBattlerTarget =
                            GetBattlerAtPosition(GetBattlerPosition(gBattlerTarget) ^ 2);
                    }
                }
            }
        } else {
            gActiveBattler = gBattlerByTurnOrder[var];
            RecordAbilityBattle(gActiveBattler, gBattleMons[gActiveBattler].ability);
            gSpecialStatuses[gActiveBattler].set_lightningRodRedirected(1);
            gBattlerTarget = gActiveBattler;
        }
    } else if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gChosenMove]
            .target as i32
            & MOVE_TARGET_RANDOM
            != 0
    {
        if GetBattlerSide(gBattlerAttacker) == B_SIDE_PLAYER {
            if Random() as i32 & 1 != 0 {
                gBattlerTarget = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
            } else {
                gBattlerTarget = GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT);
            }
        } else {
            if Random() as i32 & 1 != 0 {
                gBattlerTarget = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
            } else {
                gBattlerTarget = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT);
            }
        }
        if gAbsentBattlerFlags as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gBattlerTarget]
            != 0
            && GetBattlerSide(gBattlerAttacker) != GetBattlerSide(gBattlerTarget)
        {
            gBattlerTarget = GetBattlerAtPosition(GetBattlerPosition(gBattlerTarget) ^ 2);
        }
    } else {
        gBattlerTarget = *(*gBattleStruct)
            .moveTarget
            .as_mut_ptr()
            .at(gBattlerAttacker);
        if gAbsentBattlerFlags as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gBattlerTarget]
            != 0
        {
            if GetBattlerSide(gBattlerAttacker) != GetBattlerSide(gBattlerTarget) {
                gBattlerTarget = GetBattlerAtPosition(GetBattlerPosition(gBattlerTarget) ^ 2);
            } else {
                gBattlerTarget = GetBattlerAtPosition(GetBattlerPosition(gBattlerAttacker) ^ 1);
                if gAbsentBattlerFlags as u32
                    & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                        [gBattlerTarget]
                    != 0
                {
                    gBattlerTarget = GetBattlerAtPosition(GetBattlerPosition(gBattlerTarget) ^ 2);
                }
            }
        }
    }
    if gBattleTypeFlags & BATTLE_TYPE_PALACE != 0
        && gProtectStructs[gBattlerAttacker].palaceUnableToUseMove() != 0
    {
        if gBattleMons[gBattlerAttacker].hp == 0 {
            gCurrentActionFuncId = B_ACTION_FINISHED;
            return;
        } else if !gPalaceSelectionBattleScripts[gBattlerAttacker].is_null() {
            gBattleCommunication[5] = B_MSG_INCAPABLE_OF_POWER;
            gBattlescriptCurrInstr = gPalaceSelectionBattleScripts[gBattlerAttacker];
            gPalaceSelectionBattleScripts[gBattlerAttacker] = null_mut();
        } else {
            gBattleCommunication[5] = B_MSG_INCAPABLE_OF_POWER;
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_MoveUsedLoafingAround
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
    } else {
        gBattlescriptCurrInstr = (*crate::asmdata::gBattleScriptsForMoveEffects
            .cast::<CArray<*mut u8, 0>>())[(*(&raw const crate::data::pokemon::gBattleMoves)
            .cast::<CArray<BattleMove, 0>>())[gCurrentMove]
            .effect];
    }
    if gBattleTypeFlags & BATTLE_TYPE_ARENA != 0 {
        BattleArena_AddMindPoints(gBattlerAttacker);
    }
    gCurrentActionFuncId = B_ACTION_EXEC_SCRIPT;
}
pub unsafe fn HandleAction_Switch() {
    gBattlerAttacker = gBattlerByTurnOrder[gCurrentTurnActionNumber];
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    gActionSelectionCursor[gBattlerAttacker] = 0;
    gMoveSelectionCursor[gBattlerAttacker] = 0;
    gBattleTextBuff1[0] = 0xFD;
    gBattleTextBuff1[1] = 7;
    gBattleTextBuff1[2] = gBattlerAttacker;
    gBattleTextBuff1[3] = *(*gBattleStruct)
        .battlerPartyIndexes
        .as_mut_ptr()
        .at(gBattlerAttacker);
    gBattleTextBuff1[4] = 0xFF;
    gBattleScripting.battler = gBattlerAttacker;
    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_ActionSwitch.cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    gCurrentActionFuncId = B_ACTION_EXEC_SCRIPT;
    gBattleResults.playerSwitchesCounter = gBattleResults.playerSwitchesCounter.saturating_add(1);
}
pub unsafe fn HandleAction_UseItem() {
    gBattlerAttacker = {
        gBattlerTarget = gBattlerByTurnOrder[gCurrentTurnActionNumber];
        gBattlerTarget
    };
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    ClearFuryCutterDestinyBondGrudge(gBattlerAttacker);
    gLastUsedItem = gBattleBufferB[gBattlerAttacker][1] as u16
        | (gBattleBufferB[gBattlerAttacker][2] as u16) << 8;
    if gLastUsedItem <= ITEM_PREMIER_BALL {
        gBattlescriptCurrInstr = (*crate::asmdata::gBattlescriptsForBallThrow
            .cast::<CArray<*mut u8, 0>>())[gLastUsedItem];
    } else if gLastUsedItem == ITEM_POKE_DOLL || gLastUsedItem == ITEM_FLUFFY_TAIL {
        gBattlescriptCurrInstr =
            (*crate::asmdata::gBattlescriptsForRunningByItem.cast::<CArray<*mut u8, 0>>())[0];
    } else if GetBattlerSide(gBattlerAttacker) == B_SIDE_PLAYER {
        gBattlescriptCurrInstr =
            (*crate::asmdata::gBattlescriptsForUsingItem.cast::<CArray<*mut u8, 0>>())[0];
    } else {
        gBattleScripting.battler = gBattlerAttacker;
        match *(*gBattleStruct)
            .AI_itemType
            .as_mut_ptr()
            .at(gBattlerAttacker >> 1)
        {
            AI_ITEM_FULL_RESTORE | AI_ITEM_HEAL_HP => {}
            AI_ITEM_CURE_CONDITION => {
                gBattleCommunication[5] = AI_HEAL_CONFUSION;
                if *(*gBattleStruct)
                    .AI_itemFlags
                    .as_mut_ptr()
                    .at(gBattlerAttacker as i32 / 2) as i32
                    & 1
                    != 0
                {
                    if *(*gBattleStruct)
                        .AI_itemFlags
                        .as_mut_ptr()
                        .at(gBattlerAttacker as i32 / 2) as i32
                        & 62
                        != 0
                    {
                        gBattleCommunication[5] = AI_HEAL_SLEEP;
                    }
                } else {
                    while *(*gBattleStruct)
                        .AI_itemFlags
                        .as_mut_ptr()
                        .at(gBattlerAttacker as i32 / 2) as i32
                        & 1
                        == 0
                    {
                        *(*gBattleStruct)
                            .AI_itemFlags
                            .as_mut_ptr()
                            .at(gBattlerAttacker as i32 / 2) >>= 1;
                        gBattleCommunication[5] += 1;
                    }
                }
            }
            AI_ITEM_X_STAT => {
                gBattleCommunication[5] = B_MSG_STAT_ROSE_ITEM;
                if *(*gBattleStruct)
                    .AI_itemFlags
                    .as_mut_ptr()
                    .at(gBattlerAttacker >> 1) as i32
                    & 128
                    != 0
                {
                    gBattleCommunication[5] = 5;
                } else {
                    gBattleTextBuff1[0] = 0xFD;
                    gBattleTextBuff1[1] = 5;
                    gBattleTextBuff1[2] = STAT_ATK;
                    gBattleTextBuff1[3] = 0xFF;
                    gBattleTextBuff2[0] = 0xFD;
                    gBattleTextBuff2[1] = 0;
                    gBattleTextBuff2[2] = CHAR_X;
                    gBattleTextBuff2[3] = 0;
                    gBattleTextBuff2[4] = 0xFF;
                    while *(*gBattleStruct)
                        .AI_itemFlags
                        .as_mut_ptr()
                        .at(gBattlerAttacker >> 1) as i32
                        & 1
                        == 0
                    {
                        *(*gBattleStruct)
                            .AI_itemFlags
                            .as_mut_ptr()
                            .at(gBattlerAttacker as i32 / 2) >>= 1;
                        gBattleTextBuff1[2] += 1;
                    }
                    gBattleScripting.animArg1 = gBattleTextBuff1[2] + STAT_ANIM_PLUS1 as u8;
                    gBattleScripting.animArg2 = 0;
                }
            }
            AI_ITEM_GUARD_SPEC => {
                gBattleCommunication[5] = B_MSG_SET_MIST;
            }
            _ => {}
        }
        gBattlescriptCurrInstr = (*crate::asmdata::gBattlescriptsForUsingItem
            .cast::<CArray<*mut u8, 0>>())[*(*gBattleStruct)
            .AI_itemType
            .as_mut_ptr()
            .at(gBattlerAttacker as i32 / 2)];
    }
    gCurrentActionFuncId = B_ACTION_EXEC_SCRIPT;
}
pub unsafe fn TryRunFromBattle(battler: u8) -> u8 {
    let mut effect: u8 = FALSE;
    let mut holdEffect: u8 = 0;
    let mut pyramidMultiplier: u8 = 0;
    let mut speedVar: u8 = 0;
    if gBattleMons[battler].item == ITEM_ENIGMA_BERRY {
        holdEffect = gEnigmaBerries[battler].holdEffect;
    } else {
        holdEffect = GetItemHoldEffect(gBattleMons[battler].item);
    }
    gPotentialItemEffectBattler = battler;
    if holdEffect == HOLD_EFFECT_CAN_ALWAYS_RUN {
        gLastUsedItem = gBattleMons[battler].item;
        gProtectStructs[battler].set_fleeType(FLEE_ITEM);
        effect += 1;
    } else if gBattleMons[battler].ability == ABILITY_RUN_AWAY {
        if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
            (*gBattleStruct).runTries += 1;
            pyramidMultiplier = GetPyramidRunMultiplier();
            speedVar = div_i32(
                gBattleMons[battler].speed as i32 * pyramidMultiplier as i32,
                gBattleMons[battler as i32 ^ 1].speed as i32,
            ) as u8
                + (*gBattleStruct).runTries * 30;
            if speedVar as i32 > Random() as i32 & 0xFF {
                gLastUsedAbility = ABILITY_RUN_AWAY;
                gProtectStructs[battler].set_fleeType(FLEE_ABILITY);
                effect += 1;
            }
        } else {
            gLastUsedAbility = ABILITY_RUN_AWAY;
            gProtectStructs[battler].set_fleeType(FLEE_ABILITY);
            effect += 1;
        }
    } else if gBattleTypeFlags & 0x43f0100 != 0 && gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
        effect += 1;
    } else {
        if gBattleTypeFlags & BATTLE_TYPE_DOUBLE == 0 {
            if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
                pyramidMultiplier = GetPyramidRunMultiplier();
                speedVar = div_i32(
                    gBattleMons[battler].speed as i32 * pyramidMultiplier as i32,
                    gBattleMons[battler as i32 ^ 1].speed as i32,
                ) as u8
                    + (*gBattleStruct).runTries * 30;
                if speedVar as i32 > Random() as i32 & 0xFF {
                    effect += 1;
                }
            } else if gBattleMons[battler].speed < gBattleMons[battler as i32 ^ 1].speed {
                speedVar = div_i32(
                    gBattleMons[battler].speed as i32 * 128,
                    gBattleMons[battler as i32 ^ 1].speed as i32,
                ) as u8
                    + (*gBattleStruct).runTries * 30;
                if speedVar as i32 > Random() as i32 & 0xFF {
                    effect += 1;
                }
            } else {
                effect += 1;
            }
        }
        (*gBattleStruct).runTries += 1;
    }
    if effect != 0 {
        gCurrentTurnActionNumber = gBattlersCount;
        gBattleOutcome = B_OUTCOME_RAN;
    }
    effect
}
pub unsafe fn HandleAction_Run() {
    gBattlerAttacker = gBattlerByTurnOrder[gCurrentTurnActionNumber];
    if gBattleTypeFlags & 0x2000002 != 0 {
        gCurrentTurnActionNumber = gBattlersCount;
        gActiveBattler = 0;
        while gActiveBattler < gBattlersCount {
            if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
                if gChosenActionByBattler[gActiveBattler] == B_ACTION_RUN {
                    gBattleOutcome |= B_OUTCOME_LOST;
                }
            } else {
                if gChosenActionByBattler[gActiveBattler] == B_ACTION_RUN {
                    gBattleOutcome |= B_OUTCOME_WON;
                }
            }
            gActiveBattler += 1;
        }
        gBattleOutcome |= B_OUTCOME_LINK_BATTLE_RAN as u8;
        (*gSaveBlock2Ptr).frontier.set_disableRecordBattle(TRUE);
    } else {
        if GetBattlerSide(gBattlerAttacker) == B_SIDE_PLAYER {
            if TryRunFromBattle(gBattlerAttacker) == 0 {
                ClearFuryCutterDestinyBondGrudge(gBattlerAttacker);
                gBattleCommunication[5] = B_MSG_CANT_ESCAPE_2;
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_PrintFailedToRunString
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
                gCurrentActionFuncId = B_ACTION_EXEC_SCRIPT;
            }
        } else {
            if gBattleMons[gBattlerAttacker].status2 & 0x400e000 != 0 {
                gBattleCommunication[5] = B_MSG_ATTACKER_CANT_ESCAPE;
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_PrintFailedToRunString
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
                gCurrentActionFuncId = B_ACTION_EXEC_SCRIPT;
            } else {
                gCurrentTurnActionNumber = gBattlersCount;
                gBattleOutcome = B_OUTCOME_MON_FLED;
            }
        }
    }
}
pub unsafe fn HandleAction_WatchesCarefully() {
    gBattlerAttacker = gBattlerByTurnOrder[gCurrentTurnActionNumber];
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    gBattlescriptCurrInstr =
        (*crate::asmdata::gBattlescriptsForSafariActions.cast::<CArray<*mut u8, 0>>())[0];
    gCurrentActionFuncId = B_ACTION_EXEC_SCRIPT;
}
pub unsafe fn HandleAction_SafariZoneBallThrow() {
    gBattlerAttacker = gBattlerByTurnOrder[gCurrentTurnActionNumber];
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    gNumSafariBalls -= 1;
    gLastUsedItem = ITEM_SAFARI_BALL;
    gBattlescriptCurrInstr =
        (*crate::asmdata::gBattlescriptsForBallThrow.cast::<CArray<*mut u8, 0>>())[5];
    gCurrentActionFuncId = B_ACTION_EXEC_SCRIPT;
}
pub unsafe fn HandleAction_ThrowPokeblock() {
    gBattlerAttacker = gBattlerByTurnOrder[gCurrentTurnActionNumber];
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    gBattleCommunication[5] = gBattleBufferB[gBattlerAttacker][1] - 1;
    gLastUsedItem = gBattleBufferB[gBattlerAttacker][2] as u16;
    gBattleResults.pokeblockThrows = gBattleResults.pokeblockThrows.saturating_add(1);
    if (*gBattleStruct).safariPkblThrowCounter < 3 {
        (*gBattleStruct).safariPkblThrowCounter += 1;
    }
    if (*gBattleStruct).safariEscapeFactor > 1 {
        if (*gBattleStruct).safariEscapeFactor
            < sPkblToEscapeFactor[(*gBattleStruct).safariPkblThrowCounter][gBattleCommunication[5]]
        {
            (*gBattleStruct).safariEscapeFactor = 1;
        } else {
            (*gBattleStruct).safariEscapeFactor -= sPkblToEscapeFactor
                [(*gBattleStruct).safariPkblThrowCounter][gBattleCommunication[5]];
        }
    }
    gBattlescriptCurrInstr =
        (*crate::asmdata::gBattlescriptsForSafariActions.cast::<CArray<*mut u8, 0>>())[2];
    gCurrentActionFuncId = B_ACTION_EXEC_SCRIPT;
}
pub unsafe fn HandleAction_GoNear() {
    gBattlerAttacker = gBattlerByTurnOrder[gCurrentTurnActionNumber];
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    (*gBattleStruct).safariCatchFactor +=
        sGoNearCounterToCatchFactor[(*gBattleStruct).safariGoNearCounter];
    if (*gBattleStruct).safariCatchFactor > 20 {
        (*gBattleStruct).safariCatchFactor = 20;
    }
    (*gBattleStruct).safariEscapeFactor +=
        sGoNearCounterToEscapeFactor[(*gBattleStruct).safariGoNearCounter];
    if (*gBattleStruct).safariEscapeFactor > 20 {
        (*gBattleStruct).safariEscapeFactor = 20;
    }
    if (*gBattleStruct).safariGoNearCounter < 3 {
        (*gBattleStruct).safariGoNearCounter += 1;
        gBattleCommunication[5] = B_MSG_CREPT_CLOSER;
    } else {
        gBattleCommunication[5] = B_MSG_CANT_GET_CLOSER;
    }
    gBattlescriptCurrInstr =
        (*crate::asmdata::gBattlescriptsForSafariActions.cast::<CArray<*mut u8, 0>>())[1];
    gCurrentActionFuncId = B_ACTION_EXEC_SCRIPT;
}
pub unsafe fn HandleAction_SafariZoneRun() {
    gBattlerAttacker = gBattlerByTurnOrder[gCurrentTurnActionNumber];
    PlaySE(SE_FLEE);
    gCurrentTurnActionNumber = gBattlersCount;
    gBattleOutcome = B_OUTCOME_RAN;
}
pub unsafe fn HandleAction_WallyBallThrow() {
    gBattlerAttacker = gBattlerByTurnOrder[gCurrentTurnActionNumber];
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    gBattleTextBuff1[0] = 0xFD;
    gBattleTextBuff1[1] = 7;
    gBattleTextBuff1[2] = gBattlerAttacker;
    gBattleTextBuff1[3] = gBattlerPartyIndexes[gBattlerAttacker] as u8;
    gBattleTextBuff1[4] = 0xFF;
    gBattlescriptCurrInstr =
        (*crate::asmdata::gBattlescriptsForSafariActions.cast::<CArray<*mut u8, 0>>())[3];
    gCurrentActionFuncId = B_ACTION_EXEC_SCRIPT;
    gActionsByTurnOrder[1] = B_ACTION_FINISHED;
}
pub unsafe fn HandleAction_TryFinish() {
    if HandleFaintedMonActions() == 0 {
        (*gBattleStruct).faintedActionsState = 0;
        gCurrentActionFuncId = B_ACTION_FINISHED;
    }
}
pub unsafe fn HandleAction_NothingIsFainted() {
    gCurrentTurnActionNumber += 1;
    gCurrentActionFuncId = gActionsByTurnOrder[gCurrentTurnActionNumber];
    gHitMarker &= 0xf1e892af;
}
pub unsafe fn HandleAction_ActionFinished() {
    *(*gBattleStruct)
        .monToSwitchIntoId
        .as_mut_ptr()
        .at(gBattlerByTurnOrder[gCurrentTurnActionNumber]) = PARTY_SIZE as u8;
    gCurrentTurnActionNumber += 1;
    gCurrentActionFuncId = gActionsByTurnOrder[gCurrentTurnActionNumber];
    SpecialStatusesClear();
    gHitMarker &= 0xf1e892af;
    gCurrentMove = 0;
    gBattleMoveDamage = 0;
    gMoveResultFlags = 0;
    gBattleScripting.animTurn = 0;
    gBattleScripting.animTargetsHit = 0;
    gLastLandedMoves[gBattlerAttacker] = 0;
    gLastHitByType[gBattlerAttacker] = 0;
    (*gBattleStruct).dynamicMoveType = 0;
    gDynamicBasePower = 0;
    gBattleScripting.moveendState = 0;
    gBattleCommunication[3] = 0;
    gBattleCommunication[4] = 0;
    gBattleScripting.multihitMoveEffect = 0;
    (*(*gBattleResources).battleScriptsStack).size = 0;
}
pub unsafe fn GetBattlerForBattleScript(caseId: u8) -> u8 {
    let mut ret: u8 = 0;
    match caseId {
        BS_TARGET => {
            ret = gBattlerTarget;
        }
        BS_ATTACKER => {
            ret = gBattlerAttacker;
        }
        BS_EFFECT_BATTLER => {
            ret = gEffectBattler;
        }
        BS_BATTLER_0 => {
            ret = 0;
        }
        BS_SCRIPTING => {
            ret = gBattleScripting.battler;
        }
        BS_FAINTED => {
            ret = gBattlerFainted;
        }
        BS_FAINTED_LINK_MULTIPLE_1 => {
            ret = gBattlerFainted;
        }
        BS_ATTACKER_WITH_PARTNER
        | BS_FAINTED_LINK_MULTIPLE_2
        | BS_ATTACKER_SIDE
        | BS_NOT_ATTACKER_SIDE
        | BS_PLAYER1 => {
            ret = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
        }
        BS_OPPONENT1 => {
            ret = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
        }
        BS_PLAYER2 => {
            ret = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT);
        }
        BS_OPPONENT2 => {
            ret = GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT);
        }
        _ => {}
    }
    ret
}
pub unsafe fn PressurePPLose(target: u8, attacker: u8, r#move: u16) {
    if gBattleMons[target].ability != ABILITY_PRESSURE {
        return;
    }
    let mut moveIndex: i32 = 0;
    while moveIndex < MAX_MON_MOVES {
        if gBattleMons[attacker].moves[moveIndex] == r#move {
            break;
        }
        moveIndex += 1;
    }
    if moveIndex == MAX_MON_MOVES {
        return;
    }
    if gBattleMons[attacker].pp[moveIndex] != 0 {
        gBattleMons[attacker].pp[moveIndex] -= 1;
    }
    if gBattleMons[attacker].status2 & 0x200000 == 0
        && gDisableStructs[attacker].mimickedMoves() as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[moveIndex]
            == 0
    {
        gActiveBattler = attacker;
        BtlController_EmitSetMonData(
            B_COMM_TO_CONTROLLER,
            REQUEST_PPMOVE1_BATTLE + moveIndex as u8,
            0,
            1,
            &raw mut gBattleMons[gActiveBattler].pp[moveIndex] as *mut c_void,
        );
        MarkBattlerForControllerExec(gActiveBattler);
    }
}
pub unsafe fn PressurePPLoseOnUsingImprison(attacker: u8) {
    let mut j: i32 = 0;
    let mut imprisonPos: i32 = MAX_MON_MOVES;
    let atkSide: u8 = GetBattlerSide(attacker);
    let mut i: i32 = 0;
    while i < gBattlersCount as i32 {
        if atkSide != GetBattlerSide(i as u8) && gBattleMons[i].ability == ABILITY_PRESSURE {
            j = 0;
            while j < MAX_MON_MOVES {
                if gBattleMons[attacker].moves[j] == MOVE_IMPRISON {
                    break;
                }
                j += 1;
            }
            if j != MAX_MON_MOVES {
                imprisonPos = j;
                if gBattleMons[attacker].pp[j] != 0 {
                    gBattleMons[attacker].pp[j] -= 1;
                }
            }
        }
        i += 1;
    }
    if imprisonPos != MAX_MON_MOVES
        && (gBattleMons[attacker].status2 & 0x200000 == 0
            && gDisableStructs[attacker].mimickedMoves() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[imprisonPos]
                == 0)
    {
        gActiveBattler = attacker;
        BtlController_EmitSetMonData(
            B_COMM_TO_CONTROLLER,
            REQUEST_PPMOVE1_BATTLE + imprisonPos as u8,
            0,
            1,
            &raw mut gBattleMons[gActiveBattler].pp[imprisonPos] as *mut c_void,
        );
        MarkBattlerForControllerExec(gActiveBattler);
    }
}
pub unsafe fn PressurePPLoseOnUsingPerishSong(attacker: u8) {
    let mut j: i32 = 0;
    let mut perishSongPos: i32 = MAX_MON_MOVES;
    for i in 0..(gBattlersCount as i32) {
        if gBattleMons[i].ability == ABILITY_PRESSURE && i != attacker as i32 {
            j = 0;
            while j < MAX_MON_MOVES {
                if gBattleMons[attacker].moves[j] == MOVE_PERISH_SONG {
                    break;
                }
                j += 1;
            }
            if j != MAX_MON_MOVES {
                perishSongPos = j;
                if gBattleMons[attacker].pp[j] != 0 {
                    gBattleMons[attacker].pp[j] -= 1;
                }
            }
        }
    }
    if perishSongPos != MAX_MON_MOVES
        && (gBattleMons[attacker].status2 & 0x200000 == 0
            && gDisableStructs[attacker].mimickedMoves() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[perishSongPos]
                == 0)
    {
        gActiveBattler = attacker;
        BtlController_EmitSetMonData(
            B_COMM_TO_CONTROLLER,
            REQUEST_PPMOVE1_BATTLE + perishSongPos as u8,
            0,
            1,
            &raw mut gBattleMons[gActiveBattler].pp[perishSongPos] as *mut c_void,
        );
        MarkBattlerForControllerExec(gActiveBattler);
    }
}
unsafe fn MarkAllBattlersForControllerExec() {
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        for i in 0..(gBattlersCount as i32) {
            gBattleControllerExecFlags |= gBitTable[i] << 28;
        }
    } else {
        for i in 0..(gBattlersCount as i32) {
            gBattleControllerExecFlags |= gBitTable[i];
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn MarkBattlerForControllerExec(battler: u8) {
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        gBattleControllerExecFlags |= gBitTable[battler] << 28;
    } else {
        gBattleControllerExecFlags |= gBitTable[battler];
    }
}
pub unsafe fn MarkBattlerReceivedLinkData(battler: u8) {
    let mut i: i32 = 0;
    while i < GetLinkPlayerCount() as i32 {
        gBattleControllerExecFlags |= shl_u32(gBitTable[battler], (i as u32) << 2);
        i += 1;
    }
    gBattleControllerExecFlags &= !(shl_i32(0x10000000, battler as u32) as u32);
}
pub unsafe fn CancelMultiTurnMoves(battler: u8) {
    gBattleMons[battler].status2 &= 0xffffefff;
    gBattleMons[battler].status2 &= 0xfffff3ff;
    gBattleMons[battler].status2 &= 0xffffff8f;
    gBattleMons[battler].status2 &= 0xfffffcff;
    gStatuses3[battler] &= 0xfffbff3f;
    gDisableStructs[battler].set_rolloutTimer(0);
    gDisableStructs[battler].furyCutterCounter = 0;
}
pub unsafe fn WasUnableToUseMove(battler: u8) -> u8 {
    if gProtectStructs[battler].prlzImmobility() != 0
        || gProtectStructs[battler].targetNotAffected() != 0
        || gProtectStructs[battler].usedImprisonedMove() != 0
        || gProtectStructs[battler].loveImmobility() != 0
        || gProtectStructs[battler].usedDisabledMove() != 0
        || gProtectStructs[battler].usedTauntedMove() != 0
        || gProtectStructs[battler].flag2Unknown() != 0
        || gProtectStructs[battler].flinchImmobility() != 0
        || gProtectStructs[battler].confusionSelfDmg() != 0
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn PrepareStringBattle(stringId: u16, battler: u8) {
    gActiveBattler = battler;
    BtlController_EmitPrintString(B_COMM_TO_CONTROLLER, stringId);
    MarkBattlerForControllerExec(gActiveBattler);
}
pub unsafe fn ResetSentPokesToOpponentValue() {
    let mut bits: u32 = 0;
    gSentPokesToOpponent[0] = 0;
    gSentPokesToOpponent[1] = 0;
    let mut i: i32 = 0;
    while i < gBattlersCount as i32 {
        bits |= gBitTable[gBattlerPartyIndexes[i]];
        i += 2;
    }
    i = 1;
    while i < gBattlersCount as i32 {
        gSentPokesToOpponent[(i & BIT_FLANK as i32) >> 1] = bits as u8;
        i += 2;
    }
}
pub unsafe fn OpponentSwitchInResetSentPokesToOpponentValue(battler: u8) {
    let mut i: i32 = 0;
    let mut bits: u32 = 0;
    if GetBattlerSide(battler) == B_SIDE_OPPONENT {
        let flank: u8 = ((battler as i32 & BIT_FLANK as i32) >> 1) as u8;
        gSentPokesToOpponent[flank] = 0;
        i = 0;
        while i < gBattlersCount as i32 {
            if gAbsentBattlerFlags as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                == 0
            {
                bits |= gBitTable[gBattlerPartyIndexes[i]];
            }
            i += 2;
        }
        gSentPokesToOpponent[flank] = bits as u8;
    }
}
pub unsafe fn UpdateSentPokesToOpponentValue(battler: u8) {
    if GetBattlerSide(battler) == B_SIDE_OPPONENT {
        OpponentSwitchInResetSentPokesToOpponentValue(battler);
    } else {
        for i in 1..(gBattlersCount as i32) {
            gSentPokesToOpponent[(i & BIT_FLANK as i32) >> 1] |=
                gBitTable[gBattlerPartyIndexes[battler]] as u8;
        }
    }
}
pub unsafe fn BattleScriptPush(bsPtr: *mut u8) {
    (*(*gBattleResources).battleScriptsStack).ptr[{
        let t1 = (*(*gBattleResources).battleScriptsStack).size;
        (*(*gBattleResources).battleScriptsStack).size += 1;
        t1
    }] = bsPtr;
}
#[unsafe(no_mangle)]
pub unsafe fn BattleScriptPushCursor() {
    (*(*gBattleResources).battleScriptsStack).ptr[{
        let t1 = (*(*gBattleResources).battleScriptsStack).size;
        (*(*gBattleResources).battleScriptsStack).size += 1;
        t1
    }] = gBattlescriptCurrInstr;
}
pub unsafe fn BattleScriptPop() {
    gBattlescriptCurrInstr = (*(*gBattleResources).battleScriptsStack).ptr[{
        (*(*gBattleResources).battleScriptsStack).size -= 1;
        (*(*gBattleResources).battleScriptsStack).size
    }];
}
pub unsafe fn TrySetCantSelectMoveBattleScript() -> u8 {
    let mut limitations: u8 = 0;
    let r#move: u16 = gBattleMons[gActiveBattler].moves[gBattleBufferB[gActiveBattler][2]];
    let mut holdEffect: u8 = 0;
    let choicedMove: *mut u16 = &raw mut (*gBattleStruct).choicedMove[gActiveBattler];
    if gDisableStructs[gActiveBattler].disabledMove == r#move && r#move != MOVE_NONE {
        gBattleScripting.battler = gActiveBattler;
        gCurrentMove = r#move;
        if gBattleTypeFlags & BATTLE_TYPE_PALACE != 0 {
            gPalaceSelectionBattleScripts[gActiveBattler] =
                (*crate::asmdata::BattleScript_SelectingDisabledMoveInPalace
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
            gProtectStructs[gActiveBattler].set_palaceUnableToUseMove(TRUE as u32);
        } else {
            gSelectionBattleScripts[gActiveBattler] =
                (*crate::asmdata::BattleScript_SelectingDisabledMove.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut();
            limitations = 1;
        }
    }
    if r#move == gLastMoves[gActiveBattler]
        && r#move != MOVE_STRUGGLE
        && gBattleMons[gActiveBattler].status2 & 0x80000000 != 0
    {
        CancelMultiTurnMoves(gActiveBattler);
        if gBattleTypeFlags & BATTLE_TYPE_PALACE != 0 {
            gPalaceSelectionBattleScripts[gActiveBattler] =
                (*crate::asmdata::BattleScript_SelectingTormentedMoveInPalace
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
            gProtectStructs[gActiveBattler].set_palaceUnableToUseMove(TRUE as u32);
        } else {
            gSelectionBattleScripts[gActiveBattler] =
                (*crate::asmdata::BattleScript_SelectingTormentedMove.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut();
            limitations += 1;
        }
    }
    if gDisableStructs[gActiveBattler].tauntTimer() != 0
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .power
            == 0
    {
        gCurrentMove = r#move;
        if gBattleTypeFlags & BATTLE_TYPE_PALACE != 0 {
            gPalaceSelectionBattleScripts[gActiveBattler] =
                (*crate::asmdata::BattleScript_SelectingNotAllowedMoveTauntInPalace
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
            gProtectStructs[gActiveBattler].set_palaceUnableToUseMove(TRUE as u32);
        } else {
            gSelectionBattleScripts[gActiveBattler] =
                (*crate::asmdata::BattleScript_SelectingNotAllowedMoveTaunt
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
            limitations += 1;
        }
    }
    if GetImprisonedMovesCount(gActiveBattler, r#move) != 0 {
        gCurrentMove = r#move;
        if gBattleTypeFlags & BATTLE_TYPE_PALACE != 0 {
            gPalaceSelectionBattleScripts[gActiveBattler] =
                (*crate::asmdata::BattleScript_SelectingImprisonedMoveInPalace
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
            gProtectStructs[gActiveBattler].set_palaceUnableToUseMove(TRUE as u32);
        } else {
            gSelectionBattleScripts[gActiveBattler] =
                (*crate::asmdata::BattleScript_SelectingImprisonedMove.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut();
            limitations += 1;
        }
    }
    if gBattleMons[gActiveBattler].item == ITEM_ENIGMA_BERRY {
        holdEffect = gEnigmaBerries[gActiveBattler].holdEffect;
    } else {
        holdEffect = GetItemHoldEffect(gBattleMons[gActiveBattler].item);
    }
    gPotentialItemEffectBattler = gActiveBattler;
    if holdEffect == HOLD_EFFECT_CHOICE_BAND
        && *choicedMove != MOVE_NONE
        && *choicedMove != MOVE_UNAVAILABLE
        && *choicedMove != r#move
    {
        gCurrentMove = *choicedMove;
        gLastUsedItem = gBattleMons[gActiveBattler].item;
        if gBattleTypeFlags & BATTLE_TYPE_PALACE != 0 {
            gProtectStructs[gActiveBattler].set_palaceUnableToUseMove(TRUE as u32);
        } else {
            gSelectionBattleScripts[gActiveBattler] =
                (*crate::asmdata::BattleScript_SelectingNotAllowedMoveChoiceItem
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
            limitations += 1;
        }
    }
    if gBattleMons[gActiveBattler].pp[gBattleBufferB[gActiveBattler][2]] == 0 {
        if gBattleTypeFlags & BATTLE_TYPE_PALACE != 0 {
            gProtectStructs[gActiveBattler].set_palaceUnableToUseMove(TRUE as u32);
        } else {
            gSelectionBattleScripts[gActiveBattler] =
                (*crate::asmdata::BattleScript_SelectingMoveWithNoPP.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut();
            limitations += 1;
        }
    }
    limitations
}
pub unsafe fn CheckMoveLimitations(battler: u8, mut unusableMoves: u8, check: u8) -> u8 {
    let mut holdEffect: u8 = 0;
    let choicedMove: *mut u16 = &raw mut (*gBattleStruct).choicedMove[battler];
    if gBattleMons[battler].item == ITEM_ENIGMA_BERRY {
        holdEffect = gEnigmaBerries[battler].holdEffect;
    } else {
        holdEffect = GetItemHoldEffect(gBattleMons[battler].item);
    }
    gPotentialItemEffectBattler = battler;
    for i in 0..MAX_MON_MOVES {
        if gBattleMons[battler].moves[i] == MOVE_NONE
            && check as i32 & MOVE_LIMITATION_ZEROMOVE != 0
        {
            unusableMoves |= gBitTable[i] as u8;
        }
        if gBattleMons[battler].pp[i] == 0 && check as i32 & MOVE_LIMITATION_PP != 0 {
            unusableMoves |= gBitTable[i] as u8;
        }
        if gBattleMons[battler].moves[i] == gDisableStructs[battler].disabledMove
            && check as i32 & MOVE_LIMITATION_DISABLED != 0
        {
            unusableMoves |= gBitTable[i] as u8;
        }
        if gBattleMons[battler].moves[i] == gLastMoves[battler]
            && check as i32 & MOVE_LIMITATION_TORMENTED != 0
            && gBattleMons[battler].status2 & 0x80000000 != 0
        {
            unusableMoves |= gBitTable[i] as u8;
        }
        if gDisableStructs[battler].tauntTimer() != 0
            && check as i32 & MOVE_LIMITATION_TAUNT != 0
            && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [gBattleMons[battler].moves[i]]
                .power
                == 0
        {
            unusableMoves |= gBitTable[i] as u8;
        }
        if GetImprisonedMovesCount(battler, gBattleMons[battler].moves[i]) != 0
            && check as i32 & MOVE_LIMITATION_IMPRISON != 0
        {
            unusableMoves |= gBitTable[i] as u8;
        }
        if gDisableStructs[battler].encoreTimer() != 0
            && gDisableStructs[battler].encoredMove != gBattleMons[battler].moves[i]
        {
            unusableMoves |= gBitTable[i] as u8;
        }
        if holdEffect == HOLD_EFFECT_CHOICE_BAND
            && *choicedMove != MOVE_NONE
            && *choicedMove != MOVE_UNAVAILABLE
            && *choicedMove != gBattleMons[battler].moves[i]
        {
            unusableMoves |= gBitTable[i] as u8;
        }
    }
    unusableMoves
}
pub unsafe fn AreAllMovesUnusable() -> u8 {
    let unusable: u8 = CheckMoveLimitations(gActiveBattler, 0, MOVE_LIMITATIONS_ALL);
    if unusable == ALL_MOVES_MASK {
        gProtectStructs[gActiveBattler].set_noValidMoves(TRUE as u32);
        gSelectionBattleScripts[gActiveBattler] = (*crate::asmdata::BattleScript_NoMovesLeft
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    } else {
        gProtectStructs[gActiveBattler].set_noValidMoves(FALSE as u32);
    }
    (unusable == ALL_MOVES_MASK) as u8
}
pub unsafe fn GetImprisonedMovesCount(battler: u8, r#move: u16) -> u8 {
    let mut imprisonedMoves: u8 = 0;
    let battlerSide: u8 = GetBattlerSide(battler);
    let mut i: i32 = 0;
    while i < gBattlersCount as i32 {
        if battlerSide != GetBattlerSide(i as u8) && gStatuses3[i] & STATUS3_IMPRISONED_OTHERS != 0
        {
            let mut j: i32 = 0;
            while j < MAX_MON_MOVES {
                if r#move == gBattleMons[i].moves[j] {
                    break;
                }
                j += 1;
            }
            if j < MAX_MON_MOVES {
                imprisonedMoves += 1;
            }
        }
        i += 1;
    }
    imprisonedMoves
}
pub unsafe fn DoFieldEndTurnEffects() -> u8 {
    let mut effect: u8 = 0;
    let mut i: i32 = 0;
    gBattlerAttacker = 0;
    while gBattlerAttacker < gBattlersCount
        && gAbsentBattlerFlags as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gBattlerAttacker]
            != 0
    {
        gBattlerAttacker += 1;
    }
    gBattlerTarget = 0;
    while gBattlerTarget < gBattlersCount
        && gAbsentBattlerFlags as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gBattlerTarget]
            != 0
    {
        gBattlerTarget += 1;
    }
    loop {
        let mut side: u8 = 0;
        'l4: {
            let sw1: u8 = (*gBattleStruct).turnCountersTracker;
            let mut fall = false;
            if sw1 == ENDTURN_ORDER {
                fall = true;
                for i in 0..(gBattlersCount as i32) {
                    gBattlerByTurnOrder[i] = i as u8;
                }
                i = 0;
                while i < gBattlersCount as i32 - 1 {
                    let mut j: i32 = i + 1;
                    while j < gBattlersCount as i32 {
                        if GetWhoStrikesFirst(gBattlerByTurnOrder[i], gBattlerByTurnOrder[j], FALSE)
                            != 0
                        {
                            SwapTurnOrder(i as u8, j as u8);
                        }
                        j += 1;
                    }
                    i += 1;
                }
                (*gBattleStruct).turnCountersTracker += 1;
                (*gBattleStruct).turnSideTracker = 0;
            }
            if fall || sw1 == ENDTURN_REFLECT {
                while (*gBattleStruct).turnSideTracker < 2 {
                    side = (*gBattleStruct).turnSideTracker;
                    gActiveBattler = {
                        gBattlerAttacker = gSideTimers[side].reflectBattlerId;
                        gBattlerAttacker
                    };
                    if gSideStatuses[side] as i32 & SIDE_STATUS_REFLECT != 0
                        && ({
                            gSideTimers[side].reflectTimer -= 1;
                            gSideTimers[side].reflectTimer
                        }) == 0
                    {
                        gSideStatuses[side] &= 65534;
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_SideStatusWoreOff
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        );
                        gBattleTextBuff1[0] = 0xFD;
                        gBattleTextBuff1[1] = 2;
                        gBattleTextBuff1[2] = MOVE_REFLECT as u8;
                        gBattleTextBuff1[3] = 0;
                        gBattleTextBuff1[4] = 0xFF;
                        effect += 1;
                    }
                    (*gBattleStruct).turnSideTracker += 1;
                    if effect != 0 {
                        break;
                    }
                }
                if effect == 0 {
                    (*gBattleStruct).turnCountersTracker += 1;
                    (*gBattleStruct).turnSideTracker = 0;
                }
                break 'l4;
            }
            if sw1 == ENDTURN_LIGHT_SCREEN {
                while (*gBattleStruct).turnSideTracker < 2 {
                    side = (*gBattleStruct).turnSideTracker;
                    gActiveBattler = {
                        gBattlerAttacker = gSideTimers[side].lightscreenBattlerId;
                        gBattlerAttacker
                    };
                    if gSideStatuses[side] as i32 & SIDE_STATUS_LIGHTSCREEN != 0
                        && ({
                            gSideTimers[side].lightscreenTimer -= 1;
                            gSideTimers[side].lightscreenTimer
                        }) == 0
                    {
                        gSideStatuses[side] &= 65533;
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_SideStatusWoreOff
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        );
                        gBattleCommunication[5] = side;
                        gBattleTextBuff1[0] = 0xFD;
                        gBattleTextBuff1[1] = 2;
                        gBattleTextBuff1[2] = MOVE_LIGHT_SCREEN as u8;
                        gBattleTextBuff1[3] = 0;
                        gBattleTextBuff1[4] = 0xFF;
                        effect += 1;
                    }
                    (*gBattleStruct).turnSideTracker += 1;
                    if effect != 0 {
                        break;
                    }
                }
                if effect == 0 {
                    (*gBattleStruct).turnCountersTracker += 1;
                    (*gBattleStruct).turnSideTracker = 0;
                }
                break 'l4;
            }
            if sw1 == ENDTURN_MIST {
                while (*gBattleStruct).turnSideTracker < 2 {
                    side = (*gBattleStruct).turnSideTracker;
                    gActiveBattler = {
                        gBattlerAttacker = gSideTimers[side].mistBattlerId;
                        gBattlerAttacker
                    };
                    if gSideTimers[side].mistTimer != 0
                        && ({
                            gSideTimers[side].mistTimer -= 1;
                            gSideTimers[side].mistTimer
                        }) == 0
                    {
                        gSideStatuses[side] &= 65279;
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_SideStatusWoreOff
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        );
                        gBattleCommunication[5] = side;
                        gBattleTextBuff1[0] = 0xFD;
                        gBattleTextBuff1[1] = 2;
                        gBattleTextBuff1[2] = MOVE_MIST as u8;
                        gBattleTextBuff1[3] = 0;
                        gBattleTextBuff1[4] = 0xFF;
                        effect += 1;
                    }
                    (*gBattleStruct).turnSideTracker += 1;
                    if effect != 0 {
                        break;
                    }
                }
                if effect == 0 {
                    (*gBattleStruct).turnCountersTracker += 1;
                    (*gBattleStruct).turnSideTracker = 0;
                }
                break 'l4;
            }
            if sw1 == ENDTURN_SAFEGUARD {
                while (*gBattleStruct).turnSideTracker < 2 {
                    side = (*gBattleStruct).turnSideTracker;
                    gActiveBattler = {
                        gBattlerAttacker = gSideTimers[side].safeguardBattlerId;
                        gBattlerAttacker
                    };
                    if gSideStatuses[side] as i32 & SIDE_STATUS_SAFEGUARD != 0
                        && ({
                            gSideTimers[side].safeguardTimer -= 1;
                            gSideTimers[side].safeguardTimer
                        }) == 0
                    {
                        gSideStatuses[side] &= 65503;
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_SafeguardEnds.cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                        );
                        effect += 1;
                    }
                    (*gBattleStruct).turnSideTracker += 1;
                    if effect != 0 {
                        break;
                    }
                }
                if effect == 0 {
                    (*gBattleStruct).turnCountersTracker += 1;
                    (*gBattleStruct).turnSideTracker = 0;
                }
                break 'l4;
            }
            if sw1 == ENDTURN_WISH {
                while (*gBattleStruct).turnSideTracker < gBattlersCount {
                    gActiveBattler = gBattlerByTurnOrder[(*gBattleStruct).turnSideTracker];
                    if gWishFutureKnock.wishCounter[gActiveBattler] != 0
                        && ({
                            gWishFutureKnock.wishCounter[gActiveBattler] -= 1;
                            gWishFutureKnock.wishCounter[gActiveBattler]
                        }) == 0
                        && gBattleMons[gActiveBattler].hp != 0
                    {
                        gBattlerTarget = gActiveBattler;
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_WishComesTrue.cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                        );
                        effect += 1;
                    }
                    (*gBattleStruct).turnSideTracker += 1;
                    if effect != 0 {
                        break;
                    }
                }
                if effect == 0 {
                    (*gBattleStruct).turnCountersTracker += 1;
                }
                break 'l4;
            }
            if sw1 == ENDTURN_RAIN {
                if gBattleWeather as i32 & B_WEATHER_RAIN != 0 {
                    if gBattleWeather as i32 & B_WEATHER_RAIN_PERMANENT == 0 {
                        if ({
                            gWishFutureKnock.weatherDuration -= 1;
                            gWishFutureKnock.weatherDuration
                        }) == 0
                        {
                            gBattleWeather &= 65534;
                            gBattleWeather &= 65533;
                            gBattleCommunication[5] = B_MSG_RAIN_STOPPED;
                        } else if gBattleWeather as i32 & B_WEATHER_RAIN_DOWNPOUR != 0 {
                            gBattleCommunication[5] = B_MSG_DOWNPOUR_CONTINUES;
                        } else {
                            gBattleCommunication[5] = B_MSG_RAIN_CONTINUES;
                        }
                    } else if gBattleWeather as i32 & B_WEATHER_RAIN_DOWNPOUR != 0 {
                        gBattleCommunication[5] = B_MSG_DOWNPOUR_CONTINUES;
                    } else {
                        gBattleCommunication[5] = B_MSG_RAIN_CONTINUES;
                    }
                    BattleScriptExecute(
                        (*crate::asmdata::BattleScript_RainContinuesOrEnds.cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                    );
                    effect += 1;
                }
                (*gBattleStruct).turnCountersTracker += 1;
                break 'l4;
            }
            if sw1 == ENDTURN_SANDSTORM {
                if gBattleWeather as i32 & B_WEATHER_SANDSTORM != 0 {
                    if gBattleWeather as i32 & B_WEATHER_SANDSTORM_PERMANENT == 0
                        && ({
                            gWishFutureKnock.weatherDuration -= 1;
                            gWishFutureKnock.weatherDuration
                        }) == 0
                    {
                        gBattleWeather &= 65527;
                        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SandStormHailEnds
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut();
                    } else {
                        gBattlescriptCurrInstr =
                            (*crate::asmdata::BattleScript_DamagingWeatherContinues
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                    }
                    gBattleScripting.animArg1 = B_ANIM_SANDSTORM_CONTINUES;
                    gBattleCommunication[5] = B_MSG_SANDSTORM;
                    BattleScriptExecute(gBattlescriptCurrInstr);
                    effect += 1;
                }
                (*gBattleStruct).turnCountersTracker += 1;
                break 'l4;
            }
            if sw1 == ENDTURN_SUN {
                if gBattleWeather as i32 & B_WEATHER_SUN != 0 {
                    if gBattleWeather as i32 & B_WEATHER_SUN_PERMANENT == 0
                        && ({
                            gWishFutureKnock.weatherDuration -= 1;
                            gWishFutureKnock.weatherDuration
                        }) == 0
                    {
                        gBattleWeather &= 65503;
                        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SunlightFaded
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut();
                    } else {
                        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SunlightContinues
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut();
                    }
                    BattleScriptExecute(gBattlescriptCurrInstr);
                    effect += 1;
                }
                (*gBattleStruct).turnCountersTracker += 1;
                break 'l4;
            }
            if sw1 == ENDTURN_HAIL {
                if gBattleWeather as i32 & B_WEATHER_HAIL != 0 {
                    if ({
                        gWishFutureKnock.weatherDuration -= 1;
                        gWishFutureKnock.weatherDuration
                    }) == 0
                    {
                        gBattleWeather &= 65407;
                        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SandStormHailEnds
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut();
                    } else {
                        gBattlescriptCurrInstr =
                            (*crate::asmdata::BattleScript_DamagingWeatherContinues
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                    }
                    gBattleScripting.animArg1 = B_ANIM_HAIL_CONTINUES;
                    gBattleCommunication[5] = B_MSG_HAIL;
                    BattleScriptExecute(gBattlescriptCurrInstr);
                    effect += 1;
                }
                (*gBattleStruct).turnCountersTracker += 1;
                break 'l4;
            }
            if sw1 == ENDTURN_FIELD_COUNT {
                effect += 1;
                break 'l4;
            }
        }
        if effect != 0 {
            break;
        }
    }
    (gBattleMainFunc != Some(BattleTurnPassed as unsafe fn())) as u8
}
pub unsafe fn DoBattlerEndTurnEffects() -> u8 {
    let mut effect: u8 = 0;
    gHitMarker |= 0x1000020;
    while (*gBattleStruct).turnEffectsBattlerId < gBattlersCount
        && (*gBattleStruct).turnEffectsTracker <= ENDTURN_BATTLER_COUNT
    {
        gActiveBattler = {
            gBattlerAttacker = gBattlerByTurnOrder[(*gBattleStruct).turnEffectsBattlerId];
            gBattlerAttacker
        };
        if gAbsentBattlerFlags as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler]
            != 0
        {
            (*gBattleStruct).turnEffectsBattlerId += 1;
        } else {
            'l2: {
                match (*gBattleStruct).turnEffectsTracker {
                    ENDTURN_INGRAIN => {
                        if gStatuses3[gActiveBattler] & STATUS3_ROOTED != 0
                            && gBattleMons[gActiveBattler].hp != gBattleMons[gActiveBattler].maxHP
                            && gBattleMons[gActiveBattler].hp != 0
                        {
                            gBattleMoveDamage = gBattleMons[gActiveBattler].maxHP as i32 / 16;
                            if gBattleMoveDamage == 0 {
                                gBattleMoveDamage = 1;
                            }
                            gBattleMoveDamage *= -1;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_IngrainTurnHeal
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect += 1;
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_ABILITIES => {
                        if AbilityBattleEffects(ABILITYEFFECT_ENDTURN, gActiveBattler, 0, 0, 0) != 0
                        {
                            effect += 1;
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_ITEMS1 => {
                        if ItemBattleEffects(ITEMEFFECT_NORMAL, gActiveBattler, FALSE) != 0 {
                            effect += 1;
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_ITEMS2 => {
                        if ItemBattleEffects(1, gActiveBattler, 1) != 0 {
                            effect += 1;
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_LEECH_SEED => {
                        if gStatuses3[gActiveBattler] & STATUS3_LEECHSEED != 0
                            && gBattleMons[gStatuses3[gActiveBattler] & STATUS3_LEECHSEED_BATTLER]
                                .hp
                                != 0
                            && gBattleMons[gActiveBattler].hp != 0
                        {
                            gBattlerTarget =
                                gStatuses3[gActiveBattler] as u8 & STATUS3_LEECHSEED_BATTLER as u8;
                            gBattleMoveDamage = gBattleMons[gActiveBattler].maxHP as i32 / 8;
                            if gBattleMoveDamage == 0 {
                                gBattleMoveDamage = 1;
                            }
                            gBattleScripting.animArg1 = gBattlerTarget;
                            gBattleScripting.animArg2 = gBattlerAttacker;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_LeechSeedTurnDrain
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect += 1;
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_POISON => {
                        if gBattleMons[gActiveBattler].status1 & STATUS1_POISON != 0
                            && gBattleMons[gActiveBattler].hp != 0
                        {
                            gBattleMoveDamage = gBattleMons[gActiveBattler].maxHP as i32 / 8;
                            if gBattleMoveDamage == 0 {
                                gBattleMoveDamage = 1;
                            }
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_PoisonTurnDmg
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect += 1;
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_BAD_POISON => {
                        if gBattleMons[gActiveBattler].status1 & STATUS1_TOXIC_POISON != 0
                            && gBattleMons[gActiveBattler].hp != 0
                        {
                            gBattleMoveDamage = gBattleMons[gActiveBattler].maxHP as i32 / 16;
                            if gBattleMoveDamage == 0 {
                                gBattleMoveDamage = 1;
                            }
                            if gBattleMons[gActiveBattler].status1 & STATUS1_TOXIC_COUNTER
                                != STATUS1_TOXIC_COUNTER
                            {
                                gBattleMons[gActiveBattler].status1 += 256;
                            }
                            gBattleMoveDamage *= ((gBattleMons[gActiveBattler].status1
                                & STATUS1_TOXIC_COUNTER)
                                >> 8) as i32;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_PoisonTurnDmg
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect += 1;
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_BURN => {
                        if gBattleMons[gActiveBattler].status1 & STATUS1_BURN != 0
                            && gBattleMons[gActiveBattler].hp != 0
                        {
                            gBattleMoveDamage = gBattleMons[gActiveBattler].maxHP as i32 / 8;
                            if gBattleMoveDamage == 0 {
                                gBattleMoveDamage = 1;
                            }
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_BurnTurnDmg.cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                            );
                            effect += 1;
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_NIGHTMARES => {
                        if gBattleMons[gActiveBattler].status2 & STATUS2_NIGHTMARE != 0
                            && gBattleMons[gActiveBattler].hp != 0
                        {
                            if gBattleMons[gActiveBattler].status1 & STATUS1_SLEEP != 0 {
                                gBattleMoveDamage = gBattleMons[gActiveBattler].maxHP as i32 / 4;
                                if gBattleMoveDamage == 0 {
                                    gBattleMoveDamage = 1;
                                }
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_NightmareTurnDmg
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                effect += 1;
                            } else {
                                gBattleMons[gActiveBattler].status2 &= 0xf7ffffff;
                            }
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_CURSE => {
                        if gBattleMons[gActiveBattler].status2 & STATUS2_CURSED != 0
                            && gBattleMons[gActiveBattler].hp != 0
                        {
                            gBattleMoveDamage = gBattleMons[gActiveBattler].maxHP as i32 / 4;
                            if gBattleMoveDamage == 0 {
                                gBattleMoveDamage = 1;
                            }
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_CurseTurnDmg
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect += 1;
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_WRAP => {
                        if gBattleMons[gActiveBattler].status2 & STATUS2_WRAPPED != 0
                            && gBattleMons[gActiveBattler].hp != 0
                        {
                            gBattleMons[gActiveBattler].status2 -= 8192;
                            if gBattleMons[gActiveBattler].status2 & STATUS2_WRAPPED != 0 {
                                gBattleScripting.animArg1 = *(*gBattleStruct)
                                    .wrappedMove
                                    .as_mut_ptr()
                                    .at(gActiveBattler as i32 * 2);
                                gBattleScripting.animArg2 = *(*gBattleStruct)
                                    .wrappedMove
                                    .as_mut_ptr()
                                    .at(gActiveBattler as i32 * 2)
                                    .at(1);
                                gBattleTextBuff1[0] = B_BUFF_PLACEHOLDER_BEGIN;
                                gBattleTextBuff1[1] = B_BUFF_MOVE;
                                gBattleTextBuff1[2] = *(*gBattleStruct)
                                    .wrappedMove
                                    .as_mut_ptr()
                                    .at(gActiveBattler as i32 * 2);
                                gBattleTextBuff1[3] = *(*gBattleStruct)
                                    .wrappedMove
                                    .as_mut_ptr()
                                    .at(gActiveBattler as i32 * 2)
                                    .at(1);
                                gBattleTextBuff1[4] = EOS;
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_WrapTurnDmg
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut();
                                gBattleMoveDamage = gBattleMons[gActiveBattler].maxHP as i32 / 16;
                                if gBattleMoveDamage == 0 {
                                    gBattleMoveDamage = 1;
                                }
                            } else {
                                gBattleTextBuff1[0] = B_BUFF_PLACEHOLDER_BEGIN;
                                gBattleTextBuff1[1] = B_BUFF_MOVE;
                                gBattleTextBuff1[2] = *(*gBattleStruct)
                                    .wrappedMove
                                    .as_mut_ptr()
                                    .at(gActiveBattler as i32 * 2);
                                gBattleTextBuff1[3] = *(*gBattleStruct)
                                    .wrappedMove
                                    .as_mut_ptr()
                                    .at(gActiveBattler as i32 * 2)
                                    .at(1);
                                gBattleTextBuff1[4] = EOS;
                                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_WrapEnds
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut();
                            }
                            BattleScriptExecute(gBattlescriptCurrInstr);
                            effect += 1;
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_UPROAR => {
                        if gBattleMons[gActiveBattler].status2 & STATUS2_UPROAR != 0 {
                            gBattlerAttacker = 0;
                            while gBattlerAttacker < gBattlersCount {
                                if gBattleMons[gBattlerAttacker].status1 & STATUS1_SLEEP != 0
                                    && gBattleMons[gBattlerAttacker].ability != ABILITY_SOUNDPROOF
                                {
                                    gBattleMons[gBattlerAttacker].status1 &= 0xfffffff8;
                                    gBattleMons[gBattlerAttacker].status2 &= 0xf7ffffff;
                                    gBattleCommunication[5] = 1;
                                    BattleScriptExecute(
                                        (*crate::asmdata::BattleScript_MonWokeUpInUproar
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut(),
                                    );
                                    gActiveBattler = gBattlerAttacker;
                                    BtlController_EmitSetMonData(
                                        B_COMM_TO_CONTROLLER,
                                        REQUEST_STATUS_BATTLE,
                                        0,
                                        4,
                                        &raw mut gBattleMons[gActiveBattler].status1 as *mut c_void,
                                    );
                                    MarkBattlerForControllerExec(gActiveBattler);
                                    break;
                                }
                                gBattlerAttacker += 1;
                            }
                            if gBattlerAttacker != gBattlersCount {
                                effect = 2;
                                break 'l2;
                            } else {
                                gBattlerAttacker = gActiveBattler;
                                gBattleMons[gActiveBattler].status2 -= 16;
                                if WasUnableToUseMove(gActiveBattler) != 0 {
                                    CancelMultiTurnMoves(gActiveBattler);
                                    gBattleCommunication[5] = B_MSG_UPROAR_ENDS;
                                } else if gBattleMons[gActiveBattler].status2 & STATUS2_UPROAR != 0
                                {
                                    gBattleCommunication[5] = B_MSG_UPROAR_CONTINUES;
                                    gBattleMons[gActiveBattler].status2 |= STATUS2_MULTIPLETURNS;
                                } else {
                                    gBattleCommunication[5] = B_MSG_UPROAR_ENDS;
                                    CancelMultiTurnMoves(gActiveBattler);
                                }
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_PrintUproarOverTurns
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                effect = 1;
                            }
                        }
                        if effect != 2 {
                            (*gBattleStruct).turnEffectsTracker += 1;
                        }
                    }
                    ENDTURN_THRASH => {
                        if gBattleMons[gActiveBattler].status2 & STATUS2_LOCK_CONFUSE != 0 {
                            gBattleMons[gActiveBattler].status2 -= 1024;
                            if WasUnableToUseMove(gActiveBattler) != 0 {
                                CancelMultiTurnMoves(gActiveBattler);
                            } else if gBattleMons[gActiveBattler].status2 & STATUS2_LOCK_CONFUSE
                                == 0
                                && gBattleMons[gActiveBattler].status2 & STATUS2_MULTIPLETURNS != 0
                            {
                                gBattleMons[gActiveBattler].status2 &= 0xffffefff;
                                if gBattleMons[gActiveBattler].status2 & STATUS2_CONFUSION == 0 {
                                    gBattleCommunication[3] = 71;
                                    SetMoveEffect(TRUE, 0);
                                    if gBattleMons[gActiveBattler].status2 & STATUS2_CONFUSION != 0
                                    {
                                        BattleScriptExecute(
                                            (*crate::asmdata::BattleScript_ThrashConfuses
                                                .cast::<CArray<u8, 0>>())
                                            .as_ptr()
                                            .cast_mut(),
                                        );
                                    }
                                    effect += 1;
                                }
                            }
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_DISABLE => {
                        if gDisableStructs[gActiveBattler].disableTimer() != 0 {
                            let mut i: i32 = 0;
                            while i < MAX_MON_MOVES {
                                if gDisableStructs[gActiveBattler].disabledMove
                                    == gBattleMons[gActiveBattler].moves[i]
                                {
                                    break;
                                }
                                i += 1;
                            }
                            if i == MAX_MON_MOVES {
                                gDisableStructs[gActiveBattler].disabledMove = MOVE_NONE;
                                gDisableStructs[gActiveBattler].set_disableTimer(0);
                            } else if ({
                                gDisableStructs[gActiveBattler].set_disableTimer(
                                    gDisableStructs[gActiveBattler].disableTimer() - 1,
                                );
                                gDisableStructs[gActiveBattler].disableTimer()
                            }) == 0
                            {
                                gDisableStructs[gActiveBattler].disabledMove = MOVE_NONE;
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_DisabledNoMore
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                effect += 1;
                            }
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_ENCORE => {
                        if gDisableStructs[gActiveBattler].encoreTimer() != 0 {
                            if gBattleMons[gActiveBattler].moves
                                [gDisableStructs[gActiveBattler].encoredMovePos]
                                != gDisableStructs[gActiveBattler].encoredMove
                            {
                                gDisableStructs[gActiveBattler].encoredMove = MOVE_NONE;
                                gDisableStructs[gActiveBattler].set_encoreTimer(0);
                            } else if ({
                                gDisableStructs[gActiveBattler].set_encoreTimer(
                                    gDisableStructs[gActiveBattler].encoreTimer() - 1,
                                );
                                gDisableStructs[gActiveBattler].encoreTimer()
                            }) == 0
                                || gBattleMons[gActiveBattler].pp
                                    [gDisableStructs[gActiveBattler].encoredMovePos]
                                    == 0
                            {
                                gDisableStructs[gActiveBattler].encoredMove = MOVE_NONE;
                                gDisableStructs[gActiveBattler].set_encoreTimer(0);
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_EncoredNoMore
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                effect += 1;
                            }
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_LOCK_ON => {
                        if gStatuses3[gActiveBattler] & STATUS3_ALWAYS_HITS != 0 {
                            gStatuses3[gActiveBattler] -= 8;
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_CHARGE => {
                        if gDisableStructs[gActiveBattler].chargeTimer() != 0
                            && ({
                                gDisableStructs[gActiveBattler].set_chargeTimer(
                                    gDisableStructs[gActiveBattler].chargeTimer() - 1,
                                );
                                gDisableStructs[gActiveBattler].chargeTimer()
                            }) == 0
                        {
                            gStatuses3[gActiveBattler] &= 0xfffffdff;
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_TAUNT => {
                        if gDisableStructs[gActiveBattler].tauntTimer() != 0 {
                            gDisableStructs[gActiveBattler]
                                .set_tauntTimer(gDisableStructs[gActiveBattler].tauntTimer() - 1);
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_YAWN => {
                        if gStatuses3[gActiveBattler] & STATUS3_YAWN != 0 {
                            gStatuses3[gActiveBattler] -= 2048;
                            if gStatuses3[gActiveBattler] & STATUS3_YAWN == 0
                                && gBattleMons[gActiveBattler].status1 & STATUS1_ANY == 0
                                && gBattleMons[gActiveBattler].ability != ABILITY_VITAL_SPIRIT
                                && gBattleMons[gActiveBattler].ability != ABILITY_INSOMNIA
                                && UproarWakeUpCheck(gActiveBattler) == 0
                            {
                                CancelMultiTurnMoves(gActiveBattler);
                                gBattleMons[gActiveBattler].status1 |= (Random() as u32 & 3) + 2;
                                BtlController_EmitSetMonData(
                                    B_COMM_TO_CONTROLLER,
                                    REQUEST_STATUS_BATTLE,
                                    0,
                                    4,
                                    &raw mut gBattleMons[gActiveBattler].status1 as *mut c_void,
                                );
                                MarkBattlerForControllerExec(gActiveBattler);
                                gEffectBattler = gActiveBattler;
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_YawnMakesAsleep
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                effect += 1;
                            }
                        }
                        (*gBattleStruct).turnEffectsTracker += 1;
                    }
                    ENDTURN_BATTLER_COUNT => {
                        (*gBattleStruct).turnEffectsTracker = 0;
                        (*gBattleStruct).turnEffectsBattlerId += 1;
                    }
                    _ => {}
                }
            }
            if effect != 0 {
                return effect;
            }
        }
    }
    gHitMarker &= 0xfeffffdf;
    0
}
pub unsafe fn HandleWishPerishSongOnTurnEnd() -> u8 {
    gHitMarker |= 0x1000020;
    'l1: {
        let sw1: u8 = (*gBattleStruct).wishPerishSongState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            while (*gBattleStruct).wishPerishSongBattlerId < gBattlersCount {
                gActiveBattler = (*gBattleStruct).wishPerishSongBattlerId;
                if gAbsentBattlerFlags as u32
                    & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                        [gActiveBattler]
                    != 0
                {
                    (*gBattleStruct).wishPerishSongBattlerId += 1;
                    continue;
                }
                (*gBattleStruct).wishPerishSongBattlerId += 1;
                if gWishFutureKnock.futureSightCounter[gActiveBattler] != 0
                    && ({
                        gWishFutureKnock.futureSightCounter[gActiveBattler] -= 1;
                        gWishFutureKnock.futureSightCounter[gActiveBattler]
                    }) == 0
                    && gBattleMons[gActiveBattler].hp != 0
                {
                    if gWishFutureKnock.futureSightMove[gActiveBattler] == MOVE_FUTURE_SIGHT {
                        gBattleCommunication[5] = B_MSG_FUTURE_SIGHT;
                    } else {
                        gBattleCommunication[5] = B_MSG_DOOM_DESIRE;
                    }
                    gBattleTextBuff1[0] = 0xFD;
                    gBattleTextBuff1[1] = 2;
                    gBattleTextBuff1[2] = gWishFutureKnock.futureSightMove[gActiveBattler] as u8;
                    gBattleTextBuff1[3] =
                        ((gWishFutureKnock.futureSightMove[gActiveBattler] as i32 & 0xFF00) >> 8)
                            as u8;
                    gBattleTextBuff1[4] = 0xFF;
                    gBattlerTarget = gActiveBattler;
                    gBattlerAttacker = gWishFutureKnock.futureSightAttacker[gActiveBattler];
                    gBattleMoveDamage = gWishFutureKnock.futureSightDmg[gActiveBattler];
                    gSpecialStatuses[gBattlerTarget].shellBellDmg = IGNORE_SHELL_BELL;
                    BattleScriptExecute(
                        (*crate::asmdata::BattleScript_MonTookFutureAttack.cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                    );
                    if gWishFutureKnock.futureSightCounter[gActiveBattler] == 0
                        && gWishFutureKnock.futureSightCounter[gActiveBattler as i32 ^ 2] == 0
                    {
                        gSideStatuses[GetBattlerPosition(gBattlerTarget) as i32 & 1] &= 65471;
                    }
                    return TRUE;
                }
            }
            (*gBattleStruct).wishPerishSongState = 1;
            (*gBattleStruct).wishPerishSongBattlerId = 0;
        }
        if fall || sw1 == 1 {
            fall = true;
            while (*gBattleStruct).wishPerishSongBattlerId < gBattlersCount {
                gActiveBattler = {
                    gBattlerAttacker =
                        gBattlerByTurnOrder[(*gBattleStruct).wishPerishSongBattlerId];
                    gBattlerAttacker
                };
                if gAbsentBattlerFlags as u32
                    & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                        [gActiveBattler]
                    != 0
                {
                    (*gBattleStruct).wishPerishSongBattlerId += 1;
                    continue;
                }
                (*gBattleStruct).wishPerishSongBattlerId += 1;
                if gStatuses3[gActiveBattler] & STATUS3_PERISH_SONG != 0 {
                    gBattleTextBuff1[0] = 0xFD;
                    gBattleTextBuff1[1] = 1;
                    gBattleTextBuff1[2] = 1;
                    gBattleTextBuff1[3] = 1;
                    gBattleTextBuff1[4] = gDisableStructs[gActiveBattler].perishSongTimer();
                    gBattleTextBuff1[5] = 0xFF;
                    if gDisableStructs[gActiveBattler].perishSongTimer() == 0 {
                        gStatuses3[gActiveBattler] &= 0xffffffdf;
                        gBattleMoveDamage = gBattleMons[gActiveBattler].hp as i32;
                        gBattlescriptCurrInstr =
                            (*crate::asmdata::BattleScript_PerishSongTakesLife
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                    } else {
                        gDisableStructs[gActiveBattler].set_perishSongTimer(
                            gDisableStructs[gActiveBattler].perishSongTimer() - 1,
                        );
                        gBattlescriptCurrInstr =
                            (*crate::asmdata::BattleScript_PerishSongCountGoesDown
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                    }
                    BattleScriptExecute(gBattlescriptCurrInstr);
                    return TRUE;
                }
            }
            (*gBattleStruct).wishPerishSongState = 2;
            (*gBattleStruct).wishPerishSongBattlerId = 0;
        }
        if fall || sw1 == 2 {
            if gBattleTypeFlags & BATTLE_TYPE_ARENA != 0
                && (*gBattleStruct).arenaTurnCounter == 2
                && gBattleMons[0].hp != 0
                && gBattleMons[1].hp != 0
            {
                for i in 0..2i32 {
                    CancelMultiTurnMoves(i as u8);
                }
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_ArenaDoJudgment
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
                BattleScriptExecute(
                    (*crate::asmdata::BattleScript_ArenaDoJudgment.cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
                (*gBattleStruct).wishPerishSongState += 1;
                return TRUE;
            }
            break 'l1;
        }
    }
    gHitMarker &= 0xfeffffdf;
    FALSE
}
pub unsafe fn HandleFaintedMonActions() -> u8 {
    if gBattleTypeFlags & BATTLE_TYPE_SAFARI != 0 {
        return FALSE;
    }
    loop {
        let mut i: i32 = 0;
        'l2: {
            let sw1: u8 = (*gBattleStruct).faintedActionsState;
            let mut fall = false;
            if sw1 == 0 {
                fall = true;
                (*gBattleStruct).faintedActionsBattlerId = 0;
                (*gBattleStruct).faintedActionsState += 1;
                i = 0;
                while i < gBattlersCount as i32 {
                    if gAbsentBattlerFlags as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                        != 0
                        && HasNoMonsToSwitch(i as u8, PARTY_SIZE as u8, PARTY_SIZE as u8) == 0
                    {
                        gAbsentBattlerFlags &= !(gBitTable[i] as u8);
                    }
                    i += 1;
                }
            }
            if fall || sw1 == 1 {
                loop {
                    gBattlerFainted = {
                        gBattlerTarget = (*gBattleStruct).faintedActionsBattlerId;
                        gBattlerTarget
                    };
                    if gBattleMons[(*gBattleStruct).faintedActionsBattlerId].hp == 0
                        && (*gBattleStruct).givenExpMons as u32
                            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                                [gBattlerPartyIndexes[(*gBattleStruct).faintedActionsBattlerId]]
                            == 0
                        && gAbsentBattlerFlags as u32
                            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                                [(*gBattleStruct).faintedActionsBattlerId]
                            == 0
                    {
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_GiveExp.cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                        );
                        (*gBattleStruct).faintedActionsState = 2;
                        return TRUE;
                    }
                    if ({
                        (*gBattleStruct).faintedActionsBattlerId += 1;
                        (*gBattleStruct).faintedActionsBattlerId
                    }) == gBattlersCount
                    {
                        break;
                    }
                }
                (*gBattleStruct).faintedActionsState = 3;
                break 'l2;
            }
            if sw1 == 2 {
                OpponentSwitchInResetSentPokesToOpponentValue(gBattlerFainted);
                if ({
                    (*gBattleStruct).faintedActionsBattlerId += 1;
                    (*gBattleStruct).faintedActionsBattlerId
                }) == gBattlersCount
                {
                    (*gBattleStruct).faintedActionsState = 3;
                } else {
                    (*gBattleStruct).faintedActionsState = 1;
                }
                break 'l2;
            }
            if sw1 == 3 {
                fall = true;
                (*gBattleStruct).faintedActionsBattlerId = 0;
                (*gBattleStruct).faintedActionsState += 1;
            }
            if fall || sw1 == 4 {
                loop {
                    gBattlerFainted = {
                        gBattlerTarget = (*gBattleStruct).faintedActionsBattlerId;
                        gBattlerTarget
                    };
                    if gBattleMons[(*gBattleStruct).faintedActionsBattlerId].hp == 0
                        && gAbsentBattlerFlags as u32
                            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                                [(*gBattleStruct).faintedActionsBattlerId]
                            == 0
                    {
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_HandleFaintedMon
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        );
                        (*gBattleStruct).faintedActionsState = 5;
                        return TRUE;
                    }
                    if ({
                        (*gBattleStruct).faintedActionsBattlerId += 1;
                        (*gBattleStruct).faintedActionsBattlerId
                    }) == gBattlersCount
                    {
                        break;
                    }
                }
                (*gBattleStruct).faintedActionsState = 6;
                break 'l2;
            }
            if sw1 == 5 {
                if ({
                    (*gBattleStruct).faintedActionsBattlerId += 1;
                    (*gBattleStruct).faintedActionsBattlerId
                }) == gBattlersCount
                {
                    (*gBattleStruct).faintedActionsState = 6;
                } else {
                    (*gBattleStruct).faintedActionsState = 4;
                }
                break 'l2;
            }
            if sw1 == 6 {
                if AbilityBattleEffects(ABILITYEFFECT_INTIMIDATE1, 0, 0, 0, 0) != 0
                    || AbilityBattleEffects(ABILITYEFFECT_TRACE, 0, 0, 0, 0) != 0
                    || ItemBattleEffects(1, 0, 1) != 0
                    || AbilityBattleEffects(ABILITYEFFECT_FORECAST, 0, 0, 0, 0) != 0
                {
                    return TRUE;
                }
                (*gBattleStruct).faintedActionsState += 1;
                break 'l2;
            }
            if sw1 == FAINTED_ACTIONS_MAX_CASE {
                break 'l2;
            }
        }
        if (*gBattleStruct).faintedActionsState == FAINTED_ACTIONS_MAX_CASE {
            break;
        }
    }
    FALSE
}
pub unsafe fn TryClearRageStatuses() {
    for i in 0..(gBattlersCount as i32) {
        if gBattleMons[i].status2 & STATUS2_RAGE != 0 && gChosenMoveByBattler[i] != MOVE_RAGE {
            gBattleMons[i].status2 &= 0xff7fffff;
        }
    }
}
pub unsafe fn AtkCanceler_UnableToUseMove() -> u8 {
    let mut effect: u8 = 0;
    let bideDmg: *mut i32 = &raw mut gBattleScripting.bideDmg;
    loop {
        'l2: {
            match (*gBattleStruct).atkCancelerTracker {
                CANCELER_FLAGS => {
                    gBattleMons[gBattlerAttacker].status2 &= 0xfdffffff;
                    gStatuses3[gBattlerAttacker] &= 0xffffbfff;
                    (*gBattleStruct).atkCancelerTracker += 1;
                }
                CANCELER_ASLEEP => {
                    if gBattleMons[gBattlerAttacker].status1 & STATUS1_SLEEP != 0 {
                        if UproarWakeUpCheck(gBattlerAttacker) != 0 {
                            gBattleMons[gBattlerAttacker].status1 &= 0xfffffff8;
                            gBattleMons[gBattlerAttacker].status2 &= 0xf7ffffff;
                            BattleScriptPushCursor();
                            gBattleCommunication[5] = B_MSG_WOKE_UP_UPROAR;
                            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_MoveUsedWokeUp
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                            effect = 2;
                        } else {
                            let mut toSub: u8 = 0;
                            if gBattleMons[gBattlerAttacker].ability == ABILITY_EARLY_BIRD {
                                toSub = 2;
                            } else {
                                toSub = 1;
                            }
                            if gBattleMons[gBattlerAttacker].status1 & STATUS1_SLEEP < toSub as u32
                            {
                                gBattleMons[gBattlerAttacker].status1 &= 0xfffffff8;
                            } else {
                                gBattleMons[gBattlerAttacker].status1 -= toSub as u32;
                            }
                            if gBattleMons[gBattlerAttacker].status1 & STATUS1_SLEEP != 0 {
                                if gCurrentMove != MOVE_SNORE && gCurrentMove != MOVE_SLEEP_TALK {
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_MoveUsedIsAsleep
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut();
                                    gHitMarker |= HITMARKER_UNABLE_TO_USE_MOVE;
                                    effect = 2;
                                }
                            } else {
                                gBattleMons[gBattlerAttacker].status2 &= 0xf7ffffff;
                                BattleScriptPushCursor();
                                gBattleCommunication[5] = B_MSG_WOKE_UP;
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_MoveUsedWokeUp
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut();
                                effect = 2;
                            }
                        }
                    }
                    (*gBattleStruct).atkCancelerTracker += 1;
                }
                CANCELER_FROZEN => {
                    if gBattleMons[gBattlerAttacker].status1 & STATUS1_FREEZE != 0 {
                        if Random() as i32 % 5 != 0 {
                            if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<
                                BattleMove,
                                0,
                            >>(
                            ))[gCurrentMove]
                                .effect
                                != EFFECT_THAW_HIT
                            {
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_MoveUsedIsFrozen
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut();
                                gHitMarker |= HITMARKER_NO_ATTACKSTRING;
                            } else {
                                (*gBattleStruct).atkCancelerTracker += 1;
                                break 'l2;
                            }
                        } else {
                            gBattleMons[gBattlerAttacker].status1 &= 0xffffffdf;
                            BattleScriptPushCursor();
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_MoveUsedUnfroze
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut();
                            gBattleCommunication[5] = B_MSG_DEFROSTED;
                        }
                        effect = 2;
                    }
                    (*gBattleStruct).atkCancelerTracker += 1;
                }
                CANCELER_TRUANT => {
                    if gBattleMons[gBattlerAttacker].ability == ABILITY_TRUANT
                        && gDisableStructs[gBattlerAttacker].truantCounter() != 0
                    {
                        CancelMultiTurnMoves(gBattlerAttacker);
                        gHitMarker |= HITMARKER_UNABLE_TO_USE_MOVE;
                        gBattleCommunication[5] = B_MSG_LOAFING;
                        gBattlescriptCurrInstr =
                            (*crate::asmdata::BattleScript_MoveUsedLoafingAround
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                        gMoveResultFlags |= MOVE_RESULT_MISSED;
                        effect = 1;
                    }
                    (*gBattleStruct).atkCancelerTracker += 1;
                }
                CANCELER_RECHARGE => {
                    if gBattleMons[gBattlerAttacker].status2 & STATUS2_RECHARGE != 0 {
                        gBattleMons[gBattlerAttacker].status2 &= 0xffbfffff;
                        gDisableStructs[gBattlerAttacker].rechargeTimer = 0;
                        CancelMultiTurnMoves(gBattlerAttacker);
                        gBattlescriptCurrInstr =
                            (*crate::asmdata::BattleScript_MoveUsedMustRecharge
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                        gHitMarker |= HITMARKER_UNABLE_TO_USE_MOVE;
                        effect = 1;
                    }
                    (*gBattleStruct).atkCancelerTracker += 1;
                }
                CANCELER_FLINCH => {
                    if gBattleMons[gBattlerAttacker].status2 & STATUS2_FLINCHED != 0 {
                        gBattleMons[gBattlerAttacker].status2 &= 0xfffffff7;
                        gProtectStructs[gBattlerAttacker].set_flinchImmobility(1);
                        CancelMultiTurnMoves(gBattlerAttacker);
                        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_MoveUsedFlinched
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut();
                        gHitMarker |= HITMARKER_UNABLE_TO_USE_MOVE;
                        effect = 1;
                    }
                    (*gBattleStruct).atkCancelerTracker += 1;
                }
                CANCELER_DISABLED => {
                    if gDisableStructs[gBattlerAttacker].disabledMove == gCurrentMove
                        && gDisableStructs[gBattlerAttacker].disabledMove != MOVE_NONE
                    {
                        gProtectStructs[gBattlerAttacker].set_usedDisabledMove(1);
                        gBattleScripting.battler = gBattlerAttacker;
                        CancelMultiTurnMoves(gBattlerAttacker);
                        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_MoveUsedIsDisabled
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut();
                        gHitMarker |= HITMARKER_UNABLE_TO_USE_MOVE;
                        effect = 1;
                    }
                    (*gBattleStruct).atkCancelerTracker += 1;
                }
                CANCELER_TAUNTED => {
                    if gDisableStructs[gBattlerAttacker].tauntTimer() != 0
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[gCurrentMove]
                            .power
                            == 0
                    {
                        gProtectStructs[gBattlerAttacker].set_usedTauntedMove(1);
                        CancelMultiTurnMoves(gBattlerAttacker);
                        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_MoveUsedIsTaunted
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut();
                        gHitMarker |= HITMARKER_UNABLE_TO_USE_MOVE;
                        effect = 1;
                    }
                    (*gBattleStruct).atkCancelerTracker += 1;
                }
                CANCELER_IMPRISONED => {
                    if GetImprisonedMovesCount(gBattlerAttacker, gCurrentMove) != 0 {
                        gProtectStructs[gBattlerAttacker].set_usedImprisonedMove(1);
                        CancelMultiTurnMoves(gBattlerAttacker);
                        gBattlescriptCurrInstr =
                            (*crate::asmdata::BattleScript_MoveUsedIsImprisoned
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                        gHitMarker |= HITMARKER_UNABLE_TO_USE_MOVE;
                        effect = 1;
                    }
                    (*gBattleStruct).atkCancelerTracker += 1;
                }
                CANCELER_CONFUSED => {
                    if gBattleMons[gBattlerAttacker].status2 & STATUS2_CONFUSION != 0 {
                        gBattleMons[gBattlerAttacker].status2 -= 1;
                        if gBattleMons[gBattlerAttacker].status2 & STATUS2_CONFUSION != 0 {
                            if Random() as i32 & 1 != 0 {
                                gBattleCommunication[5] = FALSE;
                                BattleScriptPushCursor();
                            } else {
                                gBattleCommunication[5] = TRUE;
                                gBattlerTarget = gBattlerAttacker;
                                gBattleMoveDamage = CalculateBaseDamage(
                                    &raw mut gBattleMons[gBattlerAttacker],
                                    &raw mut gBattleMons[gBattlerAttacker],
                                    MOVE_POUND,
                                    0,
                                    40,
                                    0,
                                    gBattlerAttacker,
                                    gBattlerAttacker,
                                );
                                gProtectStructs[gBattlerAttacker].set_confusionSelfDmg(1);
                                gHitMarker |= HITMARKER_UNABLE_TO_USE_MOVE;
                            }
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_MoveUsedIsConfused
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut();
                        } else {
                            BattleScriptPushCursor();
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_MoveUsedIsConfusedNoMore
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut();
                        }
                        effect = 1;
                    }
                    (*gBattleStruct).atkCancelerTracker += 1;
                }
                CANCELER_PARALYZED => {
                    if gBattleMons[gBattlerAttacker].status1 & STATUS1_PARALYSIS != 0
                        && Random() as i32 % 4 == 0
                    {
                        gProtectStructs[gBattlerAttacker].set_prlzImmobility(1);
                        gBattlescriptCurrInstr =
                            (*crate::asmdata::BattleScript_MoveUsedIsParalyzed
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                        gHitMarker |= HITMARKER_UNABLE_TO_USE_MOVE;
                        effect = 1;
                    }
                    (*gBattleStruct).atkCancelerTracker += 1;
                }
                CANCELER_IN_LOVE => {
                    if gBattleMons[gBattlerAttacker].status2 & STATUS2_INFATUATION != 0 {
                        gBattleScripting.battler = CountTrailingZeroBits(
                            (gBattleMons[gBattlerAttacker].status2 & STATUS2_INFATUATION) >> 16,
                        ) as u8;
                        if Random() as i32 & 1 != 0 {
                            BattleScriptPushCursor();
                        } else {
                            BattleScriptPush(
                                (*crate::asmdata::BattleScript_MoveUsedIsInLoveCantAttack
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            gHitMarker |= HITMARKER_UNABLE_TO_USE_MOVE;
                            gProtectStructs[gBattlerAttacker].set_loveImmobility(1);
                            CancelMultiTurnMoves(gBattlerAttacker);
                        }
                        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_MoveUsedIsInLove
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut();
                        effect = 1;
                    }
                    (*gBattleStruct).atkCancelerTracker += 1;
                }
                CANCELER_BIDE => {
                    if gBattleMons[gBattlerAttacker].status2 & STATUS2_BIDE != 0 {
                        gBattleMons[gBattlerAttacker].status2 -= 256;
                        if gBattleMons[gBattlerAttacker].status2 & STATUS2_BIDE != 0 {
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_BideStoringEnergy
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut();
                        } else {
                            if gBideDmg[gBattlerAttacker] != 0 {
                                gCurrentMove = MOVE_BIDE;
                                *bideDmg = gBideDmg[gBattlerAttacker] * 2;
                                gBattlerTarget = gBideTarget[gBattlerAttacker];
                                if gAbsentBattlerFlags as u32
                                    & (*(&raw const crate::util::gBitTable)
                                        .cast::<CArray<u32, 0>>())[gBattlerTarget]
                                    != 0
                                {
                                    gBattlerTarget = GetMoveTarget(MOVE_BIDE, 1);
                                }
                                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_BideAttack
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut();
                            } else {
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_BideNoEnergyToAttack
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut();
                            }
                        }
                        effect = 1;
                    }
                    (*gBattleStruct).atkCancelerTracker += 1;
                }
                CANCELER_THAW => {
                    if gBattleMons[gBattlerAttacker].status1 & STATUS1_FREEZE != 0 {
                        if (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[gCurrentMove]
                            .effect
                            == EFFECT_THAW_HIT
                        {
                            gBattleMons[gBattlerAttacker].status1 &= 0xffffffdf;
                            BattleScriptPushCursor();
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_MoveUsedUnfroze
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut();
                            gBattleCommunication[5] = B_MSG_DEFROSTED_BY_MOVE;
                        }
                        effect = 2;
                    }
                    (*gBattleStruct).atkCancelerTracker += 1;
                }
                CANCELER_END => {}
                _ => {}
            }
        }
        if !((*gBattleStruct).atkCancelerTracker != CANCELER_END && effect == 0) {
            break;
        }
    }
    if effect == 2 {
        gActiveBattler = gBattlerAttacker;
        BtlController_EmitSetMonData(
            B_COMM_TO_CONTROLLER,
            REQUEST_STATUS_BATTLE,
            0,
            4,
            &raw mut gBattleMons[gActiveBattler].status1 as *mut c_void,
        );
        MarkBattlerForControllerExec(gActiveBattler);
    }
    effect
}
pub unsafe fn HasNoMonsToSwitch(
    battler: u8,
    mut partyIdBattlerOn1: u8,
    mut partyIdBattlerOn2: u8,
) -> u8 {
    let mut playerId: u8 = 0;
    let mut flankId: u8 = 0;
    let mut party: *mut Pokemon = null_mut();
    let mut i: i32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE == 0 {
        return FALSE;
    }
    if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0 {
        if GetBattlerSide(battler) == B_SIDE_PLAYER {
            party = gPlayerParty.as_mut_ptr();
        } else {
            party = gEnemyParty.as_mut_ptr();
        }
        playerId = ((battler as i32 & 2) / 2) as u8;
        i = playerId as i32 * MULTI_PARTY_SIZE;
        while i < playerId as i32 * MULTI_PARTY_SIZE + MULTI_PARTY_SIZE {
            if GetMonData2(party.at(i), MON_DATA_HP) != 0
                && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_NONE as u32
                && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_EGG
            {
                break;
            }
            i += 1;
        }
        return (i == playerId as i32 * MULTI_PARTY_SIZE + MULTI_PARTY_SIZE) as u8;
    } else if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        if gBattleTypeFlags & BATTLE_TYPE_TOWER_LINK_MULTI != 0 {
            if GetBattlerSide(battler) == B_SIDE_PLAYER {
                party = gPlayerParty.as_mut_ptr();
                flankId = GetBattlerMultiplayerId(battler as u16) as u8;
                playerId = GetLinkTrainerFlankId(flankId) as u8;
            } else {
                party = gEnemyParty.as_mut_ptr();
                if battler == 1 {
                    playerId = 0;
                } else {
                    playerId = 1;
                }
            }
        } else {
            flankId = GetBattlerMultiplayerId(battler as u16) as u8;
            if GetBattlerSide(battler) == B_SIDE_PLAYER {
                party = gPlayerParty.as_mut_ptr();
            } else {
                party = gEnemyParty.as_mut_ptr();
            }
            playerId = GetLinkTrainerFlankId(flankId) as u8;
        }
        i = playerId as i32 * MULTI_PARTY_SIZE;
        while i < playerId as i32 * MULTI_PARTY_SIZE + MULTI_PARTY_SIZE {
            if GetMonData2(party.at(i), MON_DATA_HP) != 0
                && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_NONE as u32
                && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_EGG
            {
                break;
            }
            i += 1;
        }
        return (i == playerId as i32 * MULTI_PARTY_SIZE + MULTI_PARTY_SIZE) as u8;
    } else if gBattleTypeFlags & BATTLE_TYPE_TWO_OPPONENTS != 0
        && GetBattlerSide(battler) == B_SIDE_OPPONENT
    {
        party = gEnemyParty.as_mut_ptr();
        if battler == 1 {
            playerId = 0;
        } else {
            playerId = MULTI_PARTY_SIZE as u8;
        }
        i = playerId as i32;
        while i < playerId as i32 + MULTI_PARTY_SIZE {
            if GetMonData2(party.at(i), MON_DATA_HP) != 0
                && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_NONE as u32
                && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_EGG
            {
                break;
            }
            i += 1;
        }
        return (i == playerId as i32 + 3) as u8;
    } else {
        if GetBattlerSide(battler) == B_SIDE_OPPONENT {
            flankId = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
            playerId = GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT);
            party = gEnemyParty.as_mut_ptr();
        } else {
            flankId = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
            playerId = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT);
            party = gPlayerParty.as_mut_ptr();
        }
        if partyIdBattlerOn1 == PARTY_SIZE as u8 {
            partyIdBattlerOn1 = gBattlerPartyIndexes[flankId] as u8;
        }
        if partyIdBattlerOn2 == PARTY_SIZE as u8 {
            partyIdBattlerOn2 = gBattlerPartyIndexes[playerId] as u8;
        }
        i = 0;
        while i < PARTY_SIZE {
            if GetMonData2(party.at(i), MON_DATA_HP) != 0
                && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_NONE as u32
                && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_EGG
                && i != partyIdBattlerOn1 as i32
                && i != partyIdBattlerOn2 as i32
                && i != *(*gBattleStruct).monToSwitchIntoId.as_mut_ptr().at(flankId) as i32
                && i != *(*gBattleStruct).monToSwitchIntoId.as_mut_ptr().at(playerId) as i32
            {
                break;
            }
            i += 1;
        }
        return (i == PARTY_SIZE) as u8;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn CastformDataTypeChange(battler: u8) -> u8 {
    let mut formChange: u8 = 0;
    if gBattleMons[battler].species != SPECIES_CASTFORM
        || gBattleMons[battler].ability != ABILITY_FORECAST
        || gBattleMons[battler].hp == 0
    {
        return 0;
    }
    if !(AbilityBattleEffects(19, TYPE_NORMAL, 13, TYPE_NORMAL, TYPE_NORMAL as u16) == 0
        && AbilityBattleEffects(19, TYPE_NORMAL, 77, TYPE_NORMAL, TYPE_NORMAL as u16) == 0)
        && !(gBattleMons[battler].types[0] == TYPE_NORMAL
            || gBattleMons[battler].types[1] == TYPE_NORMAL)
    {
        gBattleMons[battler].types[0] = TYPE_NORMAL;
        gBattleMons[battler].types[1] = TYPE_NORMAL;
        return 1;
    }
    if !(AbilityBattleEffects(19, 0, 13, 0, 0) == 0 && AbilityBattleEffects(19, 0, 77, 0, 0) == 0) {
        return 0;
    }
    if gBattleWeather as i32 & 231 == 0
        && !(gBattleMons[battler].types[0] == TYPE_NORMAL
            || gBattleMons[battler].types[1] == TYPE_NORMAL)
    {
        gBattleMons[battler].types[0] = TYPE_NORMAL;
        gBattleMons[battler].types[1] = TYPE_NORMAL;
        formChange = 1;
    }
    if gBattleWeather as i32 & B_WEATHER_SUN != 0
        && !(gBattleMons[battler].types[0] == TYPE_FIRE
            || gBattleMons[battler].types[1] == TYPE_FIRE)
    {
        gBattleMons[battler].types[0] = TYPE_FIRE;
        gBattleMons[battler].types[1] = TYPE_FIRE;
        formChange = 2;
    }
    if gBattleWeather as i32 & B_WEATHER_RAIN != 0
        && !(gBattleMons[battler].types[0] == TYPE_WATER
            || gBattleMons[battler].types[1] == TYPE_WATER)
    {
        gBattleMons[battler].types[0] = TYPE_WATER;
        gBattleMons[battler].types[1] = TYPE_WATER;
        formChange = 3;
    }
    if gBattleWeather as i32 & B_WEATHER_HAIL != 0
        && !(gBattleMons[battler].types[0] == TYPE_ICE || gBattleMons[battler].types[1] == TYPE_ICE)
    {
        gBattleMons[battler].types[0] = TYPE_ICE;
        gBattleMons[battler].types[1] = TYPE_ICE;
        formChange = 4;
    }
    formChange
}
pub unsafe fn AbilityBattleEffects(
    caseID: u8,
    mut battler: u8,
    ability: u8,
    special: u8,
    moveArg: u16,
) -> u8 {
    let mut effect: u8 = 0;
    let mut pokeAtk: *mut Pokemon = null_mut();
    let mut pokeDef: *mut Pokemon = null_mut();
    if gBattlerAttacker >= gBattlersCount {
        gBattlerAttacker = battler;
    }
    if GetBattlerSide(gBattlerAttacker) == B_SIDE_PLAYER {
        pokeAtk = &raw mut gPlayerParty[gBattlerPartyIndexes[gBattlerAttacker]];
    } else {
        pokeAtk = &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerAttacker]];
    }
    if gBattlerTarget >= gBattlersCount {
        gBattlerTarget = battler;
    }
    if GetBattlerSide(gBattlerTarget) == B_SIDE_PLAYER {
        pokeDef = &raw mut gPlayerParty[gBattlerPartyIndexes[gBattlerTarget]];
    } else {
        pokeDef = &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerTarget]];
    }
    let speciesAtk: u16 = GetMonData2(pokeAtk, MON_DATA_SPECIES) as u16;
    let pidAtk: u32 = GetMonData2(pokeAtk, MON_DATA_PERSONALITY);
    let speciesDef: u16 = GetMonData2(pokeDef, MON_DATA_SPECIES) as u16;
    let pidDef: u32 = GetMonData2(pokeDef, MON_DATA_PERSONALITY);
    if gBattleTypeFlags & BATTLE_TYPE_SAFARI == 0 {
        let mut moveType: u8 = 0;
        let mut i: i32 = 0;
        let mut r#move: u16 = 0;
        let mut side: u8 = 0;
        let mut target1: u8 = 0;
        if special != 0 {
            gLastUsedAbility = special;
        } else {
            gLastUsedAbility = gBattleMons[battler].ability;
        }
        if moveArg != 0 {
            r#move = moveArg;
        } else {
            r#move = gCurrentMove;
        }
        if (*gBattleStruct).dynamicMoveType != 0 {
            moveType = (*gBattleStruct).dynamicMoveType & 63;
        } else {
            moveType = (*(&raw const crate::data::pokemon::gBattleMoves)
                .cast::<CArray<BattleMove, 0>>())[r#move]
                .r#type;
        }
        match caseID {
            0 => {
                if gBattlerAttacker >= gBattlersCount {
                    gBattlerAttacker = battler;
                }
                match gLastUsedAbility {
                    ABILITYEFFECT_SWITCH_IN_WEATHER => {
                        if gBattleTypeFlags & BATTLE_TYPE_RECORDED == 0 {
                            match GetCurrentWeather() {
                                WEATHER_RAIN | WEATHER_RAIN_THUNDERSTORM | WEATHER_DOWNPOUR => {
                                    if gBattleWeather as i32 & B_WEATHER_RAIN == 0 {
                                        gBattleWeather = 5;
                                        gBattleScripting.animArg1 = B_ANIM_RAIN_CONTINUES;
                                        gBattleScripting.battler = battler;
                                        effect += 1;
                                    }
                                }
                                WEATHER_SANDSTORM => {
                                    if gBattleWeather as i32 & B_WEATHER_SANDSTORM == 0 {
                                        gBattleWeather = B_WEATHER_SANDSTORM as u16;
                                        gBattleScripting.animArg1 = B_ANIM_SANDSTORM_CONTINUES;
                                        gBattleScripting.battler = battler;
                                        effect += 1;
                                    }
                                }
                                WEATHER_DROUGHT if gBattleWeather as i32 & B_WEATHER_SUN == 0 => {
                                    gBattleWeather = B_WEATHER_SUN as u16;
                                    gBattleScripting.animArg1 = B_ANIM_SUN_CONTINUES;
                                    gBattleScripting.battler = battler;
                                    effect += 1;
                                }
                                _ => {}
                            }
                        }
                        if effect != 0 {
                            gBattleCommunication[5] = GetCurrentWeather();
                            BattleScriptPushCursorAndCallback(
                                (*crate::asmdata::BattleScript_OverworldWeatherStarts
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                        }
                    }
                    ABILITY_DRIZZLE => {
                        if gBattleWeather as i32 & B_WEATHER_RAIN_PERMANENT == 0 {
                            gBattleWeather = 5;
                            BattleScriptPushCursorAndCallback(
                                (*crate::asmdata::BattleScript_DrizzleActivates
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            gBattleScripting.battler = battler;
                            effect += 1;
                        }
                    }
                    ABILITY_SAND_STREAM => {
                        if gBattleWeather as i32 & B_WEATHER_SANDSTORM_PERMANENT == 0 {
                            gBattleWeather = B_WEATHER_SANDSTORM as u16;
                            BattleScriptPushCursorAndCallback(
                                (*crate::asmdata::BattleScript_SandstreamActivates
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            gBattleScripting.battler = battler;
                            effect += 1;
                        }
                    }
                    ABILITY_DROUGHT => {
                        if gBattleWeather as i32 & B_WEATHER_SUN_PERMANENT == 0 {
                            gBattleWeather = B_WEATHER_SUN as u16;
                            BattleScriptPushCursorAndCallback(
                                (*crate::asmdata::BattleScript_DroughtActivates
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            gBattleScripting.battler = battler;
                            effect += 1;
                        }
                    }
                    ABILITY_INTIMIDATE => {
                        if gSpecialStatuses[battler].intimidatedMon() == 0 {
                            gStatuses3[battler] |= STATUS3_INTIMIDATE_POKES;
                            gSpecialStatuses[battler].set_intimidatedMon(1);
                        }
                    }
                    ABILITY_FORECAST => {
                        effect = CastformDataTypeChange(battler);
                        if effect != 0 {
                            BattleScriptPushCursorAndCallback(
                                (*crate::asmdata::BattleScript_CastformChange
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            gBattleScripting.battler = battler;
                            (*gBattleStruct).formToChangeInto = effect - 1;
                        }
                    }
                    ABILITY_TRACE => {
                        if gSpecialStatuses[battler].traced() == 0 {
                            gStatuses3[battler] |= STATUS3_TRACE;
                            gSpecialStatuses[battler].set_traced(1);
                        }
                    }
                    ABILITY_CLOUD_NINE | ABILITY_AIR_LOCK => {
                        target1 = 0;
                        while target1 < gBattlersCount {
                            effect = CastformDataTypeChange(target1);
                            if effect != 0 {
                                BattleScriptPushCursorAndCallback(
                                    (*crate::asmdata::BattleScript_CastformChange
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                gBattleScripting.battler = target1;
                                (*gBattleStruct).formToChangeInto = effect - 1;
                                break;
                            }
                            target1 += 1;
                        }
                    }
                    _ => {}
                }
            }
            1 => {
                if gBattleMons[battler].hp != 0 {
                    gBattlerAttacker = battler;
                    match gLastUsedAbility {
                        ABILITY_RAIN_DISH => {
                            if AbilityBattleEffects(19, 0, 13, 0, 0) == 0
                                && AbilityBattleEffects(19, 0, 77, 0, 0) == 0
                                && gBattleWeather as i32 & B_WEATHER_RAIN != 0
                                && gBattleMons[battler].maxHP > gBattleMons[battler].hp
                            {
                                gLastUsedAbility = ABILITY_RAIN_DISH;
                                BattleScriptPushCursorAndCallback(
                                    (*crate::asmdata::BattleScript_RainDishActivates
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                gBattleMoveDamage = gBattleMons[battler].maxHP as i32 / 16;
                                if gBattleMoveDamage == 0 {
                                    gBattleMoveDamage = 1;
                                }
                                gBattleMoveDamage *= -1;
                                effect += 1;
                            }
                        }
                        ABILITY_SHED_SKIN => {
                            if gBattleMons[battler].status1 & STATUS1_ANY != 0
                                && Random() as i32 % 3 == 0
                            {
                                if gBattleMons[battler].status1 & 136 != 0 {
                                    StringCopy(
                                        gBattleTextBuff1.as_mut_ptr(),
                                        (*(&raw const crate::data::battle_main::gStatusConditionString_PoisonJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                    );
                                }
                                if gBattleMons[battler].status1 & STATUS1_SLEEP != 0 {
                                    StringCopy(
                                        gBattleTextBuff1.as_mut_ptr(),
                                        (*(&raw const crate::data::battle_main::gStatusConditionString_SleepJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                    );
                                }
                                if gBattleMons[battler].status1 & STATUS1_PARALYSIS != 0 {
                                    StringCopy(
                                        gBattleTextBuff1.as_mut_ptr(),
                                        (*(&raw const crate::data::battle_main::gStatusConditionString_ParalysisJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                    );
                                }
                                if gBattleMons[battler].status1 & STATUS1_BURN != 0 {
                                    StringCopy(
                                        gBattleTextBuff1.as_mut_ptr(),
                                        (*(&raw const crate::data::battle_main::gStatusConditionString_BurnJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                    );
                                }
                                if gBattleMons[battler].status1 & STATUS1_FREEZE != 0 {
                                    StringCopy(
                                        gBattleTextBuff1.as_mut_ptr(),
                                        (*(&raw const crate::data::battle_main::gStatusConditionString_IceJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                    );
                                }
                                gBattleMons[battler].status1 = 0;
                                gBattleMons[battler].status2 &= 0xf7ffffff;
                                gBattleScripting.battler = {
                                    gActiveBattler = battler;
                                    gActiveBattler
                                };
                                BattleScriptPushCursorAndCallback(
                                    (*crate::asmdata::BattleScript_ShedSkinActivates
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                BtlController_EmitSetMonData(
                                    B_COMM_TO_CONTROLLER,
                                    REQUEST_STATUS_BATTLE,
                                    0,
                                    4,
                                    &raw mut gBattleMons[battler].status1 as *mut c_void,
                                );
                                MarkBattlerForControllerExec(gActiveBattler);
                                effect += 1;
                            }
                        }
                        ABILITY_SPEED_BOOST => {
                            if gBattleMons[battler].statStages[3] < MAX_STAT_STAGE
                                && gDisableStructs[battler].isFirstTurn != 2
                            {
                                gBattleMons[battler].statStages[3] += 1;
                                gBattleScripting.animArg1 = 17;
                                gBattleScripting.animArg2 = 0;
                                BattleScriptPushCursorAndCallback(
                                    (*crate::asmdata::BattleScript_SpeedBoostActivates
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                gBattleScripting.battler = battler;
                                effect += 1;
                            }
                        }
                        ABILITY_TRUANT => {
                            gDisableStructs[gBattlerAttacker].set_truantCounter(
                                gDisableStructs[gBattlerAttacker].truantCounter() ^ 1,
                            );
                        }
                        _ => {}
                    }
                }
            }
            2 => {
                if gLastUsedAbility == ABILITY_SOUNDPROOF {
                    i = 0;
                    while sSoundMovesTable[i] != SOUND_MOVES_END {
                        if sSoundMovesTable[i] == r#move {
                            break;
                        }
                        i += 1;
                    }
                    if sSoundMovesTable[i] != SOUND_MOVES_END {
                        if gBattleMons[gBattlerAttacker].status2 & STATUS2_MULTIPLETURNS != 0 {
                            gHitMarker |= HITMARKER_NO_PPDEDUCT;
                        }
                        gBattlescriptCurrInstr =
                            (*crate::asmdata::BattleScript_SoundproofProtected
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                        effect = 1;
                    }
                }
            }
            3 => {
                if r#move != 0 {
                    match gLastUsedAbility {
                        ABILITY_VOLT_ABSORB => {
                            if moveType == TYPE_ELECTRIC
                                && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<
                                    BattleMove,
                                    0,
                                >>(
                                ))[r#move]
                                    .power
                                    != 0
                            {
                                if gProtectStructs[gBattlerAttacker].notFirstStrike() != 0 {
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_MoveHPDrain
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut();
                                } else {
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_MoveHPDrain_PPLoss
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut();
                                }
                                effect = 1;
                            }
                        }
                        ABILITY_WATER_ABSORB => {
                            if moveType == TYPE_WATER
                                && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<
                                    BattleMove,
                                    0,
                                >>(
                                ))[r#move]
                                    .power
                                    != 0
                            {
                                if gProtectStructs[gBattlerAttacker].notFirstStrike() != 0 {
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_MoveHPDrain
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut();
                                } else {
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_MoveHPDrain_PPLoss
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut();
                                }
                                effect = 1;
                            }
                        }
                        ABILITY_FLASH_FIRE
                            if moveType == TYPE_FIRE
                                && gBattleMons[battler].status1 & STATUS1_FREEZE == 0 =>
                        {
                            if (*(*gBattleResources).flags).flags[battler]
                                & RESOURCE_FLAG_FLASH_FIRE
                                == 0
                            {
                                gBattleCommunication[5] = B_MSG_FLASH_FIRE_BOOST;
                                if gProtectStructs[gBattlerAttacker].notFirstStrike() != 0 {
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_FlashFireBoost
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut();
                                } else {
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_FlashFireBoost_PPLoss
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut();
                                }
                                (*(*gBattleResources).flags).flags[battler] |=
                                    RESOURCE_FLAG_FLASH_FIRE;
                                effect = 2;
                            } else {
                                gBattleCommunication[5] = B_MSG_FLASH_FIRE_NO_BOOST;
                                if gProtectStructs[gBattlerAttacker].notFirstStrike() != 0 {
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_FlashFireBoost
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut();
                                } else {
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_FlashFireBoost_PPLoss
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut();
                                }
                                effect = 2;
                            }
                        }
                        _ => {}
                    }
                    if effect == 1 {
                        if gBattleMons[battler].maxHP == gBattleMons[battler].hp {
                            if gProtectStructs[gBattlerAttacker].notFirstStrike() != 0 {
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_MonMadeMoveUseless
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut();
                            } else {
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_MonMadeMoveUseless_PPLoss
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut();
                            }
                        } else {
                            gBattleMoveDamage = gBattleMons[battler].maxHP as i32 / 4;
                            if gBattleMoveDamage == 0 {
                                gBattleMoveDamage = 1;
                            }
                            gBattleMoveDamage *= -1;
                        }
                    }
                }
            }
            ABILITYEFFECT_ON_DAMAGE => match gLastUsedAbility {
                ABILITY_COLOR_CHANGE => {
                    if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && r#move != MOVE_STRUGGLE
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[r#move]
                            .power
                            != 0
                        && (gSpecialStatuses[gBattlerTarget].physicalDmg != 0
                            || gSpecialStatuses[gBattlerTarget].specialDmg != 0)
                        && !(gBattleMons[battler].types[0] == moveType
                            || gBattleMons[battler].types[1] == moveType)
                        && gBattleMons[battler].hp != 0
                    {
                        gBattleMons[battler].types[0] = moveType;
                        gBattleMons[battler].types[1] = moveType;
                        gBattleTextBuff1[0] = 0xFD;
                        gBattleTextBuff1[1] = 3;
                        gBattleTextBuff1[2] = moveType;
                        gBattleTextBuff1[3] = 0xFF;
                        BattleScriptPushCursor();
                        gBattlescriptCurrInstr =
                            (*crate::asmdata::BattleScript_ColorChangeActivates
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                        effect += 1;
                    }
                }
                ABILITY_ROUGH_SKIN => {
                    if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && gBattleMons[gBattlerAttacker].hp != 0
                        && gProtectStructs[gBattlerAttacker].confusionSelfDmg() == 0
                        && (gSpecialStatuses[gBattlerTarget].physicalDmg != 0
                            || gSpecialStatuses[gBattlerTarget].specialDmg != 0)
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[r#move]
                            .flags as i32
                            & FLAG_MAKES_CONTACT
                            != 0
                    {
                        gBattleMoveDamage = gBattleMons[gBattlerAttacker].maxHP as i32 / 16;
                        if gBattleMoveDamage == 0 {
                            gBattleMoveDamage = 1;
                        }
                        BattleScriptPushCursor();
                        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_RoughSkinActivates
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut();
                        effect += 1;
                    }
                }
                ABILITY_EFFECT_SPORE => {
                    if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && gBattleMons[gBattlerAttacker].hp != 0
                        && gProtectStructs[gBattlerAttacker].confusionSelfDmg() == 0
                        && (gSpecialStatuses[gBattlerTarget].physicalDmg != 0
                            || gSpecialStatuses[gBattlerTarget].specialDmg != 0)
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[r#move]
                            .flags as i32
                            & FLAG_MAKES_CONTACT
                            != 0
                        && Random() as i32 % 10 == 0
                    {
                        loop {
                            gBattleCommunication[3] = Random() as u8 & 3;
                            if gBattleCommunication[3] != 0 {
                                break;
                            }
                        }
                        if gBattleCommunication[3] == 3 {
                            gBattleCommunication[3] += 2;
                        }
                        gBattleCommunication[3] += MOVE_EFFECT_AFFECTS_USER;
                        BattleScriptPushCursor();
                        gBattlescriptCurrInstr =
                            (*crate::asmdata::BattleScript_ApplySecondaryEffect
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                        gHitMarker |= HITMARKER_STATUS_ABILITY_EFFECT;
                        effect += 1;
                    }
                }
                ABILITY_POISON_POINT => {
                    if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && gBattleMons[gBattlerAttacker].hp != 0
                        && gProtectStructs[gBattlerAttacker].confusionSelfDmg() == 0
                        && (gSpecialStatuses[gBattlerTarget].physicalDmg != 0
                            || gSpecialStatuses[gBattlerTarget].specialDmg != 0)
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[r#move]
                            .flags as i32
                            & FLAG_MAKES_CONTACT
                            != 0
                        && Random() as i32 % 3 == 0
                    {
                        gBattleCommunication[3] = 66;
                        BattleScriptPushCursor();
                        gBattlescriptCurrInstr =
                            (*crate::asmdata::BattleScript_ApplySecondaryEffect
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                        gHitMarker |= HITMARKER_STATUS_ABILITY_EFFECT;
                        effect += 1;
                    }
                }
                ABILITY_STATIC => {
                    if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && gBattleMons[gBattlerAttacker].hp != 0
                        && gProtectStructs[gBattlerAttacker].confusionSelfDmg() == 0
                        && (gSpecialStatuses[gBattlerTarget].physicalDmg != 0
                            || gSpecialStatuses[gBattlerTarget].specialDmg != 0)
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[r#move]
                            .flags as i32
                            & FLAG_MAKES_CONTACT
                            != 0
                        && Random() as i32 % 3 == 0
                    {
                        gBattleCommunication[3] = 69;
                        BattleScriptPushCursor();
                        gBattlescriptCurrInstr =
                            (*crate::asmdata::BattleScript_ApplySecondaryEffect
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                        gHitMarker |= HITMARKER_STATUS_ABILITY_EFFECT;
                        effect += 1;
                    }
                }
                ABILITY_FLAME_BODY => {
                    if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && gBattleMons[gBattlerAttacker].hp != 0
                        && gProtectStructs[gBattlerAttacker].confusionSelfDmg() == 0
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[r#move]
                            .flags as i32
                            & FLAG_MAKES_CONTACT
                            != 0
                        && (gSpecialStatuses[gBattlerTarget].physicalDmg != 0
                            || gSpecialStatuses[gBattlerTarget].specialDmg != 0)
                        && Random() as i32 % 3 == 0
                    {
                        gBattleCommunication[3] = 67;
                        BattleScriptPushCursor();
                        gBattlescriptCurrInstr =
                            (*crate::asmdata::BattleScript_ApplySecondaryEffect
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                        gHitMarker |= HITMARKER_STATUS_ABILITY_EFFECT;
                        effect += 1;
                    }
                }
                ABILITY_CUTE_CHARM
                    if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && gBattleMons[gBattlerAttacker].hp != 0
                        && gProtectStructs[gBattlerAttacker].confusionSelfDmg() == 0
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[r#move]
                            .flags as i32
                            & FLAG_MAKES_CONTACT
                            != 0
                        && (gSpecialStatuses[gBattlerTarget].physicalDmg != 0
                            || gSpecialStatuses[gBattlerTarget].specialDmg != 0)
                        && gBattleMons[gBattlerTarget].hp != 0
                        && Random() as i32 % 3 == 0
                        && gBattleMons[gBattlerAttacker].ability != ABILITY_OBLIVIOUS
                        && GetGenderFromSpeciesAndPersonality(speciesAtk, pidAtk)
                            != GetGenderFromSpeciesAndPersonality(speciesDef, pidDef)
                        && gBattleMons[gBattlerAttacker].status2 & STATUS2_INFATUATION == 0
                        && GetGenderFromSpeciesAndPersonality(speciesAtk, pidAtk)
                            != MON_GENDERLESS
                        && GetGenderFromSpeciesAndPersonality(speciesDef, pidDef)
                            != MON_GENDERLESS =>
                {
                    gBattleMons[gBattlerAttacker].status2 |= gBitTable[gBattlerTarget] << 16;
                    BattleScriptPushCursor();
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_CuteCharmActivates
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut();
                    effect += 1;
                }
                _ => {}
            },
            5 => {
                battler = 0;
                while battler < gBattlersCount {
                    match gBattleMons[battler].ability {
                        ABILITY_IMMUNITY => {
                            if gBattleMons[battler].status1 & 3976 != 0 {
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_PoisonJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                effect = 1;
                            }
                        }
                        ABILITY_OWN_TEMPO => {
                            if gBattleMons[battler].status2 & STATUS2_CONFUSION != 0 {
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_ConfusionJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                effect = 2;
                            }
                        }
                        ABILITY_LIMBER => {
                            if gBattleMons[battler].status1 & STATUS1_PARALYSIS != 0 {
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_ParalysisJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                effect = 1;
                            }
                        }
                        ABILITY_INSOMNIA | ABILITY_VITAL_SPIRIT => {
                            if gBattleMons[battler].status1 & STATUS1_SLEEP != 0 {
                                gBattleMons[battler].status2 &= 0xf7ffffff;
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_SleepJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                effect = 1;
                            }
                        }
                        ABILITY_WATER_VEIL => {
                            if gBattleMons[battler].status1 & STATUS1_BURN != 0 {
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_BurnJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                effect = 1;
                            }
                        }
                        ABILITY_MAGMA_ARMOR => {
                            if gBattleMons[battler].status1 & STATUS1_FREEZE != 0 {
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_IceJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                effect = 1;
                            }
                        }
                        ABILITY_OBLIVIOUS
                            if gBattleMons[battler].status2 & STATUS2_INFATUATION != 0 =>
                        {
                            StringCopy(
                                gBattleTextBuff1.as_mut_ptr(),
                                (*(&raw const crate::data::battle_main::gStatusConditionString_LoveJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                            );
                            effect = 3;
                        }
                        _ => {}
                    }
                    if effect != 0 {
                        match effect {
                            1 => {
                                gBattleMons[battler].status1 = 0;
                            }
                            2 => {
                                gBattleMons[battler].status2 &= 0xfffffff8;
                            }
                            3 => {
                                gBattleMons[battler].status2 &= 0xfff0ffff;
                            }
                            _ => {}
                        }
                        BattleScriptPushCursor();
                        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_AbilityCuredStatus
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut();
                        gBattleScripting.battler = battler;
                        gActiveBattler = battler;
                        BtlController_EmitSetMonData(
                            B_COMM_TO_CONTROLLER,
                            REQUEST_STATUS_BATTLE,
                            0,
                            4,
                            &raw mut gBattleMons[gActiveBattler].status1 as *mut c_void,
                        );
                        MarkBattlerForControllerExec(gActiveBattler);
                        return effect;
                    }
                    battler += 1;
                }
            }
            6 => {
                battler = 0;
                while battler < gBattlersCount {
                    if gBattleMons[battler].ability == ABILITY_FORECAST {
                        effect = CastformDataTypeChange(battler);
                        if effect != 0 {
                            BattleScriptPushCursorAndCallback(
                                (*crate::asmdata::BattleScript_CastformChange
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            gBattleScripting.battler = battler;
                            (*gBattleStruct).formToChangeInto = effect - 1;
                            return effect;
                        }
                    }
                    battler += 1;
                }
            }
            7 => {
                if gLastUsedAbility == ABILITY_SYNCHRONIZE
                    && gHitMarker & HITMARKER_SYNCHRONIZE_EFFECT != 0
                {
                    gHitMarker &= 0xffffbfff;
                    (*gBattleStruct).synchronizeMoveEffect &= 63;
                    if (*gBattleStruct).synchronizeMoveEffect == MOVE_EFFECT_TOXIC {
                        (*gBattleStruct).synchronizeMoveEffect = MOVE_EFFECT_POISON;
                    }
                    gBattleCommunication[3] =
                        (*gBattleStruct).synchronizeMoveEffect + MOVE_EFFECT_AFFECTS_USER;
                    gBattleScripting.battler = gBattlerTarget;
                    BattleScriptPushCursor();
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SynchronizeActivates
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut();
                    gHitMarker |= HITMARKER_STATUS_ABILITY_EFFECT;
                    effect += 1;
                }
            }
            8 => {
                if gLastUsedAbility == ABILITY_SYNCHRONIZE
                    && gHitMarker & HITMARKER_SYNCHRONIZE_EFFECT != 0
                {
                    gHitMarker &= 0xffffbfff;
                    (*gBattleStruct).synchronizeMoveEffect &= 63;
                    if (*gBattleStruct).synchronizeMoveEffect == MOVE_EFFECT_TOXIC {
                        (*gBattleStruct).synchronizeMoveEffect = MOVE_EFFECT_POISON;
                    }
                    gBattleCommunication[3] = (*gBattleStruct).synchronizeMoveEffect;
                    gBattleScripting.battler = gBattlerAttacker;
                    BattleScriptPushCursor();
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SynchronizeActivates
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut();
                    gHitMarker |= HITMARKER_STATUS_ABILITY_EFFECT;
                    effect += 1;
                }
            }
            9 => {
                i = 0;
                while i < gBattlersCount as i32 {
                    if gBattleMons[i].ability == ABILITY_INTIMIDATE
                        && gStatuses3[i] & STATUS3_INTIMIDATE_POKES != 0
                    {
                        gLastUsedAbility = ABILITY_INTIMIDATE;
                        gStatuses3[i] &= 0xfff7ffff;
                        BattleScriptPushCursorAndCallback(
                            (*crate::asmdata::BattleScript_IntimidateActivatesEnd3
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        );
                        (*gBattleStruct).intimidateBattler = i as u8;
                        effect += 1;
                        break;
                    }
                    i += 1;
                }
            }
            11 => {
                i = 0;
                while i < gBattlersCount as i32 {
                    if gBattleMons[i].ability == ABILITY_TRACE && gStatuses3[i] & STATUS3_TRACE != 0
                    {
                        side = (GetBattlerPosition(i as u8) ^ BIT_SIDE) & BIT_SIDE;
                        target1 = GetBattlerAtPosition(side);
                        let target2: u8 = GetBattlerAtPosition(side + BIT_FLANK);
                        if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
                            if gBattleMons[target1].ability != 0
                                && gBattleMons[target1].hp != 0
                                && gBattleMons[target2].ability != 0
                                && gBattleMons[target2].hp != 0
                            {
                                gActiveBattler =
                                    GetBattlerAtPosition(((Random() as u8 & 1) * 2) | side);
                                gBattleMons[i].ability = gBattleMons[gActiveBattler].ability;
                                gLastUsedAbility = gBattleMons[gActiveBattler].ability;
                                effect += 1;
                            } else if gBattleMons[target1].ability != 0
                                && gBattleMons[target1].hp != 0
                            {
                                gActiveBattler = target1;
                                gBattleMons[i].ability = gBattleMons[gActiveBattler].ability;
                                gLastUsedAbility = gBattleMons[gActiveBattler].ability;
                                effect += 1;
                            } else if gBattleMons[target2].ability != 0
                                && gBattleMons[target2].hp != 0
                            {
                                gActiveBattler = target2;
                                gBattleMons[i].ability = gBattleMons[gActiveBattler].ability;
                                gLastUsedAbility = gBattleMons[gActiveBattler].ability;
                                effect += 1;
                            }
                        } else {
                            gActiveBattler = target1;
                            if gBattleMons[target1].ability != 0 && gBattleMons[target1].hp != 0 {
                                gBattleMons[i].ability = gBattleMons[target1].ability;
                                gLastUsedAbility = gBattleMons[target1].ability;
                                effect += 1;
                            }
                        }
                        if effect != 0 {
                            BattleScriptPushCursorAndCallback(
                                (*crate::asmdata::BattleScript_TraceActivates
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            gStatuses3[i] &= 0xffefffff;
                            gBattleScripting.battler = i as u8;
                            gBattleTextBuff1[0] = 0xFD;
                            gBattleTextBuff1[1] = 4;
                            gBattleTextBuff1[2] = gActiveBattler;
                            gBattleTextBuff1[3] = gBattlerPartyIndexes[gActiveBattler] as u8;
                            gBattleTextBuff1[4] = 0xFF;
                            gBattleTextBuff2[0] = 0xFD;
                            gBattleTextBuff2[1] = 9;
                            gBattleTextBuff2[2] = gLastUsedAbility;
                            gBattleTextBuff2[3] = 0xFF;
                            break;
                        }
                    }
                    i += 1;
                }
            }
            10 => {
                i = 0;
                while i < gBattlersCount as i32 {
                    if gBattleMons[i].ability == ABILITY_INTIMIDATE
                        && gStatuses3[i] & STATUS3_INTIMIDATE_POKES != 0
                    {
                        gLastUsedAbility = ABILITY_INTIMIDATE;
                        gStatuses3[i] &= 0xfff7ffff;
                        BattleScriptPushCursor();
                        gBattlescriptCurrInstr =
                            (*crate::asmdata::BattleScript_IntimidateActivates
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                        (*gBattleStruct).intimidateBattler = i as u8;
                        effect += 1;
                        break;
                    }
                    i += 1;
                }
            }
            12 => {
                side = GetBattlerSide(battler);
                i = 0;
                while i < gBattlersCount as i32 {
                    if GetBattlerSide(i as u8) != side && gBattleMons[i].ability == ability {
                        gLastUsedAbility = ability;
                        effect = i as u8 + 1;
                    }
                    i += 1;
                }
            }
            13 => {
                side = GetBattlerSide(battler);
                i = 0;
                while i < gBattlersCount as i32 {
                    if GetBattlerSide(i as u8) == side && gBattleMons[i].ability == ability {
                        gLastUsedAbility = ability;
                        effect = i as u8 + 1;
                    }
                    i += 1;
                }
            }
            14 => match gLastUsedAbility {
                ABILITYEFFECT_MUD_SPORT => {
                    for i in 0..(gBattlersCount as i32) {
                        if gStatuses3[i] & STATUS3_MUDSPORT != 0 {
                            effect = i as u8 + 1;
                        }
                    }
                }
                ABILITYEFFECT_WATER_SPORT => {
                    for i in 0..(gBattlersCount as i32) {
                        if gStatuses3[i] & STATUS3_WATERSPORT != 0 {
                            effect = i as u8 + 1;
                        }
                    }
                }
                _ => {
                    for i in 0..(gBattlersCount as i32) {
                        if gBattleMons[i].ability == ability {
                            gLastUsedAbility = ability;
                            effect = i as u8 + 1;
                        }
                    }
                }
            },
            19 => {
                for i in 0..(gBattlersCount as i32) {
                    if gBattleMons[i].ability == ability && gBattleMons[i].hp != 0 {
                        gLastUsedAbility = ability;
                        effect = i as u8 + 1;
                    }
                }
            }
            15 => {
                for i in 0..(gBattlersCount as i32) {
                    if gBattleMons[i].ability == ability && i != battler as i32 {
                        gLastUsedAbility = ability;
                        effect = i as u8 + 1;
                    }
                }
            }
            16 => {
                side = GetBattlerSide(battler);
                i = 0;
                while i < gBattlersCount as i32 {
                    if GetBattlerSide(i as u8) != side && gBattleMons[i].ability == ability {
                        gLastUsedAbility = ability;
                        effect += 1;
                    }
                    i += 1;
                }
            }
            17 => {
                side = GetBattlerSide(battler);
                i = 0;
                while i < gBattlersCount as i32 {
                    if GetBattlerSide(i as u8) == side && gBattleMons[i].ability == ability {
                        gLastUsedAbility = ability;
                        effect += 1;
                    }
                    i += 1;
                }
            }
            18 => {
                for i in 0..(gBattlersCount as i32) {
                    if gBattleMons[i].ability == ability && i != battler as i32 {
                        gLastUsedAbility = ability;
                        effect += 1;
                    }
                }
            }
            _ => {}
        }
        if effect != 0 && caseID < ABILITYEFFECT_CHECK_OTHER_SIDE && gLastUsedAbility != 0xFF {
            RecordAbilityBattle(battler, gLastUsedAbility);
        }
    }
    effect
}
pub unsafe fn BattleScriptExecute(BS_ptr: *mut u8) {
    gBattlescriptCurrInstr = BS_ptr;
    (*(*gBattleResources).battleCallbackStack).function[{
        let t1 = (*(*gBattleResources).battleCallbackStack).size;
        (*(*gBattleResources).battleCallbackStack).size += 1;
        t1
    }] = gBattleMainFunc;
    gBattleMainFunc = Some(RunBattleScriptCommands_PopCallbacksStack);
    gCurrentActionFuncId = 0;
}
pub unsafe fn BattleScriptPushCursorAndCallback(BS_ptr: *mut u8) {
    BattleScriptPushCursor();
    gBattlescriptCurrInstr = BS_ptr;
    (*(*gBattleResources).battleCallbackStack).function[{
        let t1 = (*(*gBattleResources).battleCallbackStack).size;
        (*(*gBattleResources).battleCallbackStack).size += 1;
        t1
    }] = gBattleMainFunc;
    gBattleMainFunc = Some(RunBattleScriptCommands);
}
pub unsafe fn ItemBattleEffects(caseID: u8, mut battler: u8, moveTurn: u8) -> u8 {
    let mut i: i32 = 0;
    let mut effect: u8 = ITEM_NO_EFFECT;
    let mut changedPP: u8 = 0;
    let mut battlerHoldEffect: u8 = 0;
    let mut atkHoldEffect: u8 = 0;
    let mut defHoldEffect: u8 = 0;
    let mut battlerHoldEffectParam: u8 = 0;
    let mut atkHoldEffectParam: u8 = 0;
    let mut defHoldEffectParam: u8 = 0;
    gLastUsedItem = gBattleMons[battler].item;
    if gLastUsedItem == ITEM_ENIGMA_BERRY {
        battlerHoldEffect = gEnigmaBerries[battler].holdEffect;
        battlerHoldEffectParam = gEnigmaBerries[battler].holdEffectParam;
    } else {
        battlerHoldEffect = GetItemHoldEffect(gLastUsedItem);
        battlerHoldEffectParam = GetItemHoldEffectParam(gLastUsedItem);
    }
    let atkItem: u16 = gBattleMons[gBattlerAttacker].item;
    if atkItem == ITEM_ENIGMA_BERRY {
        atkHoldEffect = gEnigmaBerries[gBattlerAttacker].holdEffect;
        atkHoldEffectParam = gEnigmaBerries[gBattlerAttacker].holdEffectParam;
    } else {
        atkHoldEffect = GetItemHoldEffect(atkItem);
        atkHoldEffectParam = GetItemHoldEffectParam(atkItem);
    }
    let defItem: u16 = gBattleMons[gBattlerTarget].item;
    if defItem == ITEM_ENIGMA_BERRY {
        defHoldEffect = gEnigmaBerries[gBattlerTarget].holdEffect;
        defHoldEffectParam = gEnigmaBerries[gBattlerTarget].holdEffectParam;
    } else {
        defHoldEffect = GetItemHoldEffect(defItem);
        defHoldEffectParam = GetItemHoldEffectParam(defItem);
    }
    match caseID {
        ITEMEFFECT_ON_SWITCH_IN => match battlerHoldEffect {
            HOLD_EFFECT_DOUBLE_PRIZE => {
                if GetBattlerSide(battler) == B_SIDE_PLAYER {
                    (*gBattleStruct).moneyMultiplier = 2;
                }
            }
            HOLD_EFFECT_RESTORE_STATS => {
                for i in 0..NUM_BATTLE_STATS {
                    if gBattleMons[battler].statStages[i] < DEFAULT_STAT_STAGE {
                        gBattleMons[battler].statStages[i] = DEFAULT_STAT_STAGE;
                        effect = ITEM_STATS_CHANGE;
                    }
                }
                if effect != 0 {
                    gBattleScripting.battler = battler;
                    gPotentialItemEffectBattler = battler;
                    gActiveBattler = {
                        gBattlerAttacker = battler;
                        gBattlerAttacker
                    };
                    BattleScriptExecute(
                        (*crate::asmdata::BattleScript_WhiteHerbEnd2.cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                    );
                }
            }
            _ => {}
        },
        ITEMEFFECT_NORMAL => {
            if gBattleMons[battler].hp != 0 {
                match battlerHoldEffect {
                    HOLD_EFFECT_RESTORE_HP => {
                        if gBattleMons[battler].hp as i32 <= gBattleMons[battler].maxHP as i32 / 2
                            && moveTurn == 0
                        {
                            gBattleMoveDamage = battlerHoldEffectParam as i32;
                            if gBattleMons[battler].hp as i32 + battlerHoldEffectParam as i32
                                > gBattleMons[battler].maxHP as i32
                            {
                                gBattleMoveDamage = gBattleMons[battler].maxHP as i32
                                    - gBattleMons[battler].hp as i32;
                            }
                            gBattleMoveDamage *= -1;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_ItemHealHP_RemoveItem
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect = ITEM_HP_CHANGE;
                        }
                    }
                    HOLD_EFFECT_RESTORE_PP => {
                        if moveTurn == 0 {
                            let mut mon: *mut Pokemon = null_mut();
                            let mut ppBonuses: u8 = 0;
                            let mut r#move: u16 = 0;
                            if GetBattlerSide(battler) == B_SIDE_PLAYER {
                                mon = &raw mut gPlayerParty[gBattlerPartyIndexes[battler]];
                            } else {
                                mon = &raw mut gEnemyParty[gBattlerPartyIndexes[battler]];
                            }
                            i = 0;
                            while i < MAX_MON_MOVES {
                                r#move = GetMonData2(mon, MON_DATA_MOVE1 + i) as u16;
                                changedPP = GetMonData2(mon, MON_DATA_PP1 + i) as u8;
                                ppBonuses = GetMonData2(mon, MON_DATA_PP_BONUSES) as u8;
                                if r#move != 0 && changedPP == 0 {
                                    break;
                                }
                                i += 1;
                            }
                            if i != MAX_MON_MOVES {
                                let maxPP: u8 = CalculatePPWithBonus(r#move, ppBonuses, i as u8);
                                if changedPP as i32 + battlerHoldEffectParam as i32 > maxPP as i32 {
                                    changedPP = maxPP;
                                } else {
                                    changedPP += battlerHoldEffectParam;
                                }
                                gBattleTextBuff1[0] = 0xFD;
                                gBattleTextBuff1[1] = 2;
                                gBattleTextBuff1[2] = r#move as u8;
                                gBattleTextBuff1[3] = ((r#move as i32 & 0xFF00) >> 8) as u8;
                                gBattleTextBuff1[4] = 0xFF;
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_BerryPPHealEnd2
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                BtlController_EmitSetMonData(
                                    B_COMM_TO_CONTROLLER,
                                    i as u8 + REQUEST_PPMOVE1_BATTLE,
                                    0,
                                    1,
                                    &raw mut changedPP as *mut c_void,
                                );
                                MarkBattlerForControllerExec(gActiveBattler);
                                effect = ITEM_PP_CHANGE;
                            }
                        }
                    }
                    HOLD_EFFECT_RESTORE_STATS => {
                        for i in 0..NUM_BATTLE_STATS {
                            if gBattleMons[battler].statStages[i] < DEFAULT_STAT_STAGE {
                                gBattleMons[battler].statStages[i] = DEFAULT_STAT_STAGE;
                                effect = ITEM_STATS_CHANGE;
                            }
                        }
                        if effect != 0 {
                            gBattleScripting.battler = battler;
                            gPotentialItemEffectBattler = battler;
                            gActiveBattler = {
                                gBattlerAttacker = battler;
                                gBattlerAttacker
                            };
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_WhiteHerbEnd2
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                        }
                    }
                    HOLD_EFFECT_LEFTOVERS => {
                        if gBattleMons[battler].hp < gBattleMons[battler].maxHP && moveTurn == 0 {
                            gBattleMoveDamage = gBattleMons[battler].maxHP as i32 / 16;
                            if gBattleMoveDamage == 0 {
                                gBattleMoveDamage = 1;
                            }
                            if gBattleMons[battler].hp as i32 + gBattleMoveDamage
                                > gBattleMons[battler].maxHP as i32
                            {
                                gBattleMoveDamage = gBattleMons[battler].maxHP as i32
                                    - gBattleMons[battler].hp as i32;
                            }
                            gBattleMoveDamage *= -1;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_ItemHealHP_End2
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect = ITEM_HP_CHANGE;
                            RecordItemEffectBattle(battler, battlerHoldEffect);
                        }
                    }
                    HOLD_EFFECT_CONFUSE_SPICY => {
                        if gBattleMons[battler].hp as i32 <= gBattleMons[battler].maxHP as i32 / 2
                            && moveTurn == 0
                        {
                            gBattleTextBuff1[0] = 0xFD;
                            gBattleTextBuff1[1] = 8;
                            gBattleTextBuff1[2] = FLAVOR_SPICY;
                            gBattleTextBuff1[3] = 0xFF;
                            gBattleMoveDamage = div_i32(
                                gBattleMons[battler].maxHP as i32,
                                battlerHoldEffectParam as i32,
                            );
                            if gBattleMoveDamage == FLAVOR_SPICY as i32 {
                                gBattleMoveDamage = 1;
                            }
                            if gBattleMons[battler].hp as i32 + gBattleMoveDamage
                                > gBattleMons[battler].maxHP as i32
                            {
                                gBattleMoveDamage = gBattleMons[battler].maxHP as i32
                                    - gBattleMons[battler].hp as i32;
                            }
                            gBattleMoveDamage *= -1;
                            if GetFlavorRelationByPersonality(
                                gBattleMons[battler].personality,
                                FLAVOR_SPICY,
                            ) < FLAVOR_SPICY as i8
                            {
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_BerryConfuseHealEnd2
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                            } else {
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_ItemHealHP_RemoveItem
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                            }
                            effect = ITEM_HP_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CONFUSE_DRY => {
                        if gBattleMons[battler].hp as i32 <= gBattleMons[battler].maxHP as i32 / 2
                            && moveTurn == 0
                        {
                            gBattleTextBuff1[0] = 0xFD;
                            gBattleTextBuff1[1] = 8;
                            gBattleTextBuff1[2] = FLAVOR_DRY;
                            gBattleTextBuff1[3] = 0xFF;
                            gBattleMoveDamage = div_i32(
                                gBattleMons[battler].maxHP as i32,
                                battlerHoldEffectParam as i32,
                            );
                            if gBattleMoveDamage == 0 {
                                gBattleMoveDamage = FLAVOR_DRY as i32;
                            }
                            if gBattleMons[battler].hp as i32 + gBattleMoveDamage
                                > gBattleMons[battler].maxHP as i32
                            {
                                gBattleMoveDamage = gBattleMons[battler].maxHP as i32
                                    - gBattleMons[battler].hp as i32;
                            }
                            gBattleMoveDamage *= -1;
                            if GetFlavorRelationByPersonality(
                                gBattleMons[battler].personality,
                                FLAVOR_DRY,
                            ) < 0
                            {
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_BerryConfuseHealEnd2
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                            } else {
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_ItemHealHP_RemoveItem
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                            }
                            effect = ITEM_HP_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CONFUSE_SWEET => {
                        if gBattleMons[battler].hp as i32 <= gBattleMons[battler].maxHP as i32 / 2
                            && moveTurn == 0
                        {
                            gBattleTextBuff1[0] = 0xFD;
                            gBattleTextBuff1[1] = 8;
                            gBattleTextBuff1[2] = FLAVOR_SWEET;
                            gBattleTextBuff1[3] = 0xFF;
                            gBattleMoveDamage = div_i32(
                                gBattleMons[battler].maxHP as i32,
                                battlerHoldEffectParam as i32,
                            );
                            if gBattleMoveDamage == 0 {
                                gBattleMoveDamage = 1;
                            }
                            if gBattleMons[battler].hp as i32 + gBattleMoveDamage
                                > gBattleMons[battler].maxHP as i32
                            {
                                gBattleMoveDamage = gBattleMons[battler].maxHP as i32
                                    - gBattleMons[battler].hp as i32;
                            }
                            gBattleMoveDamage *= -1;
                            if GetFlavorRelationByPersonality(
                                gBattleMons[battler].personality,
                                FLAVOR_SWEET,
                            ) < 0
                            {
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_BerryConfuseHealEnd2
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                            } else {
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_ItemHealHP_RemoveItem
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                            }
                            effect = ITEM_HP_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CONFUSE_BITTER => {
                        if gBattleMons[battler].hp as i32 <= gBattleMons[battler].maxHP as i32 / 2
                            && moveTurn == 0
                        {
                            gBattleTextBuff1[0] = 0xFD;
                            gBattleTextBuff1[1] = 8;
                            gBattleTextBuff1[2] = FLAVOR_BITTER;
                            gBattleTextBuff1[3] = 0xFF;
                            gBattleMoveDamage = div_i32(
                                gBattleMons[battler].maxHP as i32,
                                battlerHoldEffectParam as i32,
                            );
                            if gBattleMoveDamage == 0 {
                                gBattleMoveDamage = 1;
                            }
                            if gBattleMons[battler].hp as i32 + gBattleMoveDamage
                                > gBattleMons[battler].maxHP as i32
                            {
                                gBattleMoveDamage = gBattleMons[battler].maxHP as i32
                                    - gBattleMons[battler].hp as i32;
                            }
                            gBattleMoveDamage *= -1;
                            if GetFlavorRelationByPersonality(
                                gBattleMons[battler].personality,
                                FLAVOR_BITTER,
                            ) < 0
                            {
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_BerryConfuseHealEnd2
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                            } else {
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_ItemHealHP_RemoveItem
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                            }
                            effect = ITEM_HP_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CONFUSE_SOUR => {
                        if gBattleMons[battler].hp as i32 <= gBattleMons[battler].maxHP as i32 / 2
                            && moveTurn == 0
                        {
                            gBattleTextBuff1[0] = 0xFD;
                            gBattleTextBuff1[1] = 8;
                            gBattleTextBuff1[2] = FLAVOR_SOUR;
                            gBattleTextBuff1[3] = 0xFF;
                            gBattleMoveDamage = div_i32(
                                gBattleMons[battler].maxHP as i32,
                                battlerHoldEffectParam as i32,
                            );
                            if gBattleMoveDamage == 0 {
                                gBattleMoveDamage = 1;
                            }
                            if gBattleMons[battler].hp as i32 + gBattleMoveDamage
                                > gBattleMons[battler].maxHP as i32
                            {
                                gBattleMoveDamage = gBattleMons[battler].maxHP as i32
                                    - gBattleMons[battler].hp as i32;
                            }
                            gBattleMoveDamage *= -1;
                            if GetFlavorRelationByPersonality(
                                gBattleMons[battler].personality,
                                FLAVOR_SOUR,
                            ) < 0
                            {
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_BerryConfuseHealEnd2
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                            } else {
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_ItemHealHP_RemoveItem
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                            }
                            effect = ITEM_HP_CHANGE;
                        }
                    }
                    HOLD_EFFECT_ATTACK_UP => {
                        if gBattleMons[battler].hp as i32
                            <= div_i32(
                                gBattleMons[battler].maxHP as i32,
                                battlerHoldEffectParam as i32,
                            )
                            && moveTurn == 0
                            && gBattleMons[battler].statStages[1] < MAX_STAT_STAGE
                        {
                            gBattleTextBuff1[0] = 0xFD;
                            gBattleTextBuff1[1] = 5;
                            gBattleTextBuff1[2] = STAT_ATK;
                            gBattleTextBuff1[3] = 0xFF;
                            gBattleTextBuff2[0] = 0xFD;
                            gBattleTextBuff2[1] = 0;
                            gBattleTextBuff2[2] = STRINGID_STATROSE;
                            gBattleTextBuff2[3] = 0;
                            gBattleTextBuff2[4] = 0xFF;
                            gEffectBattler = battler;
                            gBattleScripting.statChanger = 17;
                            gBattleScripting.animArg1 = 15;
                            gBattleScripting.animArg2 = 0;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_BerryStatRaiseEnd2
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect = ITEM_STATS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_DEFENSE_UP => {
                        if gBattleMons[battler].hp as i32
                            <= div_i32(
                                gBattleMons[battler].maxHP as i32,
                                battlerHoldEffectParam as i32,
                            )
                            && moveTurn == 0
                            && gBattleMons[battler].statStages[2] < 12
                        {
                            gBattleTextBuff1[0] = 0xFD;
                            gBattleTextBuff1[1] = 5;
                            gBattleTextBuff1[2] = STAT_DEF as u8;
                            gBattleTextBuff1[3] = 0xFF;
                            gEffectBattler = battler;
                            gBattleScripting.statChanger = 18;
                            gBattleScripting.animArg1 = 16;
                            gBattleScripting.animArg2 = 0;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_BerryStatRaiseEnd2
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect = ITEM_STATS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_SPEED_UP => {
                        if gBattleMons[battler].hp as i32
                            <= div_i32(
                                gBattleMons[battler].maxHP as i32,
                                battlerHoldEffectParam as i32,
                            )
                            && moveTurn == 0
                            && gBattleMons[battler].statStages[3] < 12
                        {
                            gBattleTextBuff1[0] = 0xFD;
                            gBattleTextBuff1[1] = 5;
                            gBattleTextBuff1[2] = STAT_SPEED;
                            gBattleTextBuff1[3] = 0xFF;
                            gEffectBattler = battler;
                            gBattleScripting.statChanger = 19;
                            gBattleScripting.animArg1 = 17;
                            gBattleScripting.animArg2 = 0;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_BerryStatRaiseEnd2
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect = ITEM_STATS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_SP_ATTACK_UP => {
                        if gBattleMons[battler].hp as i32
                            <= div_i32(
                                gBattleMons[battler].maxHP as i32,
                                battlerHoldEffectParam as i32,
                            )
                            && moveTurn == 0
                            && gBattleMons[battler].statStages[4] < 12
                        {
                            gBattleTextBuff1[0] = 0xFD;
                            gBattleTextBuff1[1] = 5;
                            gBattleTextBuff1[2] = STAT_SPATK;
                            gBattleTextBuff1[3] = 0xFF;
                            gEffectBattler = battler;
                            gBattleScripting.statChanger = 20;
                            gBattleScripting.animArg1 = 18;
                            gBattleScripting.animArg2 = 0;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_BerryStatRaiseEnd2
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect = ITEM_STATS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_SP_DEFENSE_UP => {
                        if gBattleMons[battler].hp as i32
                            <= div_i32(
                                gBattleMons[battler].maxHP as i32,
                                battlerHoldEffectParam as i32,
                            )
                            && moveTurn == 0
                            && gBattleMons[battler].statStages[5] < 12
                        {
                            gBattleTextBuff1[0] = 0xFD;
                            gBattleTextBuff1[1] = STAT_SPDEF as u8;
                            gBattleTextBuff1[2] = STAT_SPDEF as u8;
                            gBattleTextBuff1[3] = 0xFF;
                            gEffectBattler = battler;
                            gBattleScripting.statChanger = 21;
                            gBattleScripting.animArg1 = 19;
                            gBattleScripting.animArg2 = 0;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_BerryStatRaiseEnd2
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect = ITEM_STATS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CRITICAL_UP => {
                        if gBattleMons[battler].hp as i32
                            <= div_i32(
                                gBattleMons[battler].maxHP as i32,
                                battlerHoldEffectParam as i32,
                            )
                            && moveTurn == 0
                            && gBattleMons[battler].status2 & STATUS2_FOCUS_ENERGY == 0
                        {
                            gBattleMons[battler].status2 |= STATUS2_FOCUS_ENERGY;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_BerryFocusEnergyEnd2
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect = ITEM_EFFECT_OTHER;
                        }
                    }
                    HOLD_EFFECT_RANDOM_STAT_UP => {
                        if moveTurn == 0
                            && gBattleMons[battler].hp as i32
                                <= div_i32(
                                    gBattleMons[battler].maxHP as i32,
                                    battlerHoldEffectParam as i32,
                                )
                        {
                            i = 0;
                            while i < 5 {
                                if gBattleMons[battler].statStages[STAT_ATK as i32 + i]
                                    < MAX_STAT_STAGE
                                {
                                    break;
                                }
                                i += 1;
                            }
                            if i != 5 {
                                loop {
                                    i = Random() as i32 % 5;
                                    if gBattleMons[battler].statStages[STAT_ATK as i32 + i]
                                        != MAX_STAT_STAGE
                                    {
                                        break;
                                    }
                                }
                                gBattleTextBuff1[0] = 0xFD;
                                gBattleTextBuff1[1] = 5;
                                gBattleTextBuff1[2] = i as u8 + 1;
                                gBattleTextBuff1[3] = 0xFF;
                                gBattleTextBuff2[0] = B_BUFF_PLACEHOLDER_BEGIN;
                                gBattleTextBuff2[1] = B_BUFF_STRING;
                                gBattleTextBuff2[2] = STRINGID_STATSHARPLY;
                                gBattleTextBuff2[3] = 0;
                                gBattleTextBuff2[4] = B_BUFF_STRING;
                                gBattleTextBuff2[5] = STRINGID_STATROSE;
                                gBattleTextBuff2[6] = 0;
                                gBattleTextBuff2[7] = EOS;
                                gEffectBattler = battler;
                                gBattleScripting.statChanger = i as u8 + 1 + 32 + FALSE;
                                gBattleScripting.animArg1 = STAT_ANIM_PLUS2 as u8 + (i as u8 + 1);
                                gBattleScripting.animArg2 = 0;
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_BerryStatRaiseEnd2
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                effect = ITEM_STATS_CHANGE;
                            }
                        }
                    }
                    HOLD_EFFECT_CURE_PAR => {
                        if gBattleMons[battler].status1 & STATUS1_PARALYSIS != 0 {
                            gBattleMons[battler].status1 &= 0xffffffbf;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_BerryCurePrlzEnd2
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CURE_PSN => {
                        if gBattleMons[battler].status1 & STATUS1_PSN_ANY != 0 {
                            gBattleMons[battler].status1 &= 0xfffff077;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_BerryCurePsnEnd2
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CURE_BRN => {
                        if gBattleMons[battler].status1 & STATUS1_BURN != 0 {
                            gBattleMons[battler].status1 &= 0xffffffef;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_BerryCureBrnEnd2
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CURE_FRZ => {
                        if gBattleMons[battler].status1 & STATUS1_FREEZE != 0 {
                            gBattleMons[battler].status1 &= 0xffffffdf;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_BerryCureFrzEnd2
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CURE_SLP => {
                        if gBattleMons[battler].status1 & STATUS1_SLEEP != 0 {
                            gBattleMons[battler].status1 &= 0xfffffff8;
                            gBattleMons[battler].status2 &= 0xf7ffffff;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_BerryCureSlpEnd2
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CURE_CONFUSION => {
                        if gBattleMons[battler].status2 & STATUS2_CONFUSION != 0 {
                            gBattleMons[battler].status2 &= 0xfffffff8;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_BerryCureConfusionEnd2
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect = ITEM_EFFECT_OTHER;
                        }
                    }
                    HOLD_EFFECT_CURE_STATUS => {
                        if gBattleMons[battler].status1 & STATUS1_ANY != 0
                            || gBattleMons[battler].status2 & STATUS2_CONFUSION != 0
                        {
                            i = 0;
                            if gBattleMons[battler].status1 & STATUS1_PSN_ANY != 0 {
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_PoisonJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                i += 1;
                            }
                            if gBattleMons[battler].status1 & STATUS1_SLEEP != 0 {
                                gBattleMons[battler].status2 &= 0xf7ffffff;
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_SleepJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                i += 1;
                            }
                            if gBattleMons[battler].status1 & STATUS1_PARALYSIS != 0 {
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_ParalysisJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                i += 1;
                            }
                            if gBattleMons[battler].status1 & STATUS1_BURN != 0 {
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_BurnJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                i += 1;
                            }
                            if gBattleMons[battler].status1 & STATUS1_FREEZE != 0 {
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_IceJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                i += 1;
                            }
                            if gBattleMons[battler].status2 & STATUS2_CONFUSION != 0 {
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_ConfusionJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                i += 1;
                            }
                            if i <= 1 {
                                gBattleCommunication[5] = B_MSG_CURED_PROBLEM;
                            } else {
                                gBattleCommunication[5] = B_MSG_NORMALIZED_STATUS;
                            }
                            gBattleMons[battler].status1 = 0;
                            gBattleMons[battler].status2 &= 0xfffffff8;
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_BerryCureChosenStatusEnd2
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CURE_ATTRACT
                        if gBattleMons[battler].status2 & STATUS2_INFATUATION != 0 =>
                    {
                        gBattleMons[battler].status2 &= 0xfff0ffff;
                        StringCopy(
                            gBattleTextBuff1.as_mut_ptr(),
                            (*(&raw const crate::data::battle_main::gStatusConditionString_LoveJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                        );
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_BerryCureChosenStatusEnd2
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        );
                        gBattleCommunication[5] = B_MSG_CURED_PROBLEM;
                        effect = ITEM_EFFECT_OTHER;
                    }
                    _ => {}
                }
                if effect != 0 {
                    gBattleScripting.battler = battler;
                    gPotentialItemEffectBattler = battler;
                    gActiveBattler = {
                        gBattlerAttacker = battler;
                        gBattlerAttacker
                    };
                    match effect {
                        ITEM_STATUS_CHANGE => {
                            BtlController_EmitSetMonData(
                                B_COMM_TO_CONTROLLER,
                                REQUEST_STATUS_BATTLE,
                                0,
                                4,
                                &raw mut gBattleMons[battler].status1 as *mut c_void,
                            );
                            MarkBattlerForControllerExec(gActiveBattler);
                        }
                        ITEM_PP_CHANGE
                            if gBattleMons[battler].status2 & 0x200000 == 0
                                && gDisableStructs[battler].mimickedMoves() as u32
                                    & (*(&raw const crate::util::gBitTable)
                                        .cast::<CArray<u32, 0>>())[i]
                                    == 0 =>
                        {
                            gBattleMons[battler].pp[i] = changedPP;
                        }
                        _ => {}
                    }
                }
            }
        }
        ITEMEFFECT_DUMMY => {}
        ITEMEFFECT_MOVE_END => {
            battler = 0;
            while battler < gBattlersCount {
                gLastUsedItem = gBattleMons[battler].item;
                if gBattleMons[battler].item == ITEM_ENIGMA_BERRY {
                    battlerHoldEffect = gEnigmaBerries[battler].holdEffect;
                    battlerHoldEffectParam = gEnigmaBerries[battler].holdEffectParam;
                } else {
                    battlerHoldEffect = GetItemHoldEffect(gLastUsedItem);
                    battlerHoldEffectParam = GetItemHoldEffectParam(gLastUsedItem);
                }
                match battlerHoldEffect {
                    HOLD_EFFECT_CURE_PAR => {
                        if gBattleMons[battler].status1 & STATUS1_PARALYSIS != 0 {
                            gBattleMons[battler].status1 &= 0xffffffbf;
                            BattleScriptPushCursor();
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_BerryCureParRet
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut();
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CURE_PSN => {
                        if gBattleMons[battler].status1 & STATUS1_PSN_ANY != 0 {
                            gBattleMons[battler].status1 &= 0xfffff077;
                            BattleScriptPushCursor();
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_BerryCurePsnRet
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut();
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CURE_BRN => {
                        if gBattleMons[battler].status1 & STATUS1_BURN != 0 {
                            gBattleMons[battler].status1 &= 0xffffffef;
                            BattleScriptPushCursor();
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_BerryCureBrnRet
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut();
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CURE_FRZ => {
                        if gBattleMons[battler].status1 & STATUS1_FREEZE != 0 {
                            gBattleMons[battler].status1 &= 0xffffffdf;
                            BattleScriptPushCursor();
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_BerryCureFrzRet
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut();
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CURE_SLP => {
                        if gBattleMons[battler].status1 & STATUS1_SLEEP != 0 {
                            gBattleMons[battler].status1 &= 0xfffffff8;
                            gBattleMons[battler].status2 &= 0xf7ffffff;
                            BattleScriptPushCursor();
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_BerryCureSlpRet
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut();
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CURE_CONFUSION => {
                        if gBattleMons[battler].status2 & STATUS2_CONFUSION != 0 {
                            gBattleMons[battler].status2 &= 0xfffffff8;
                            BattleScriptPushCursor();
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_BerryCureConfusionRet
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut();
                            effect = ITEM_EFFECT_OTHER;
                        }
                    }
                    HOLD_EFFECT_CURE_ATTRACT => {
                        if gBattleMons[battler].status2 & STATUS2_INFATUATION != 0 {
                            gBattleMons[battler].status2 &= 0xfff0ffff;
                            StringCopy(
                                gBattleTextBuff1.as_mut_ptr(),
                                (*(&raw const crate::data::battle_main::gStatusConditionString_LoveJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                            );
                            BattleScriptPushCursor();
                            gBattleCommunication[5] = B_MSG_CURED_PROBLEM;
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_BerryCureChosenStatusRet
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut();
                            effect = ITEM_EFFECT_OTHER;
                        }
                    }
                    HOLD_EFFECT_CURE_STATUS => {
                        if gBattleMons[battler].status1 & STATUS1_ANY != 0
                            || gBattleMons[battler].status2 & STATUS2_CONFUSION != 0
                        {
                            if gBattleMons[battler].status1 & STATUS1_PSN_ANY != 0 {
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_PoisonJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                            }
                            if gBattleMons[battler].status1 & STATUS1_SLEEP != 0 {
                                gBattleMons[battler].status2 &= 0xf7ffffff;
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_SleepJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                            }
                            if gBattleMons[battler].status1 & STATUS1_PARALYSIS != 0 {
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_ParalysisJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                            }
                            if gBattleMons[battler].status1 & STATUS1_BURN != 0 {
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_BurnJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                            }
                            if gBattleMons[battler].status1 & STATUS1_FREEZE != 0 {
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_IceJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                            }
                            if gBattleMons[battler].status2 & STATUS2_CONFUSION != 0 {
                                StringCopy(
                                    gBattleTextBuff1.as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_ConfusionJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                            }
                            gBattleMons[battler].status1 = 0;
                            gBattleMons[battler].status2 &= 0xfffffff8;
                            BattleScriptPushCursor();
                            gBattleCommunication[5] = B_MSG_CURED_PROBLEM;
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_BerryCureChosenStatusRet
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut();
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_RESTORE_STATS => {
                        for i in 0..NUM_BATTLE_STATS {
                            if gBattleMons[battler].statStages[i] < DEFAULT_STAT_STAGE {
                                gBattleMons[battler].statStages[i] = DEFAULT_STAT_STAGE;
                                effect = ITEM_STATS_CHANGE;
                            }
                        }
                        if effect != 0 {
                            gBattleScripting.battler = battler;
                            gPotentialItemEffectBattler = battler;
                            BattleScriptPushCursor();
                            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_WhiteHerbRet
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                            return effect;
                        }
                    }
                    _ => {}
                }
                if effect != 0 {
                    gBattleScripting.battler = battler;
                    gPotentialItemEffectBattler = battler;
                    gActiveBattler = battler;
                    BtlController_EmitSetMonData(
                        B_COMM_TO_CONTROLLER,
                        REQUEST_STATUS_BATTLE,
                        0,
                        4,
                        &raw mut gBattleMons[gActiveBattler].status1 as *mut c_void,
                    );
                    MarkBattlerForControllerExec(gActiveBattler);
                    break;
                }
                battler += 1;
            }
        }
        ITEMEFFECT_KINGSROCK_SHELLBELL if gBattleMoveDamage != 0 => match atkHoldEffect {
            HOLD_EFFECT_FLINCH => {
                if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0
                    && (gSpecialStatuses[gBattlerTarget].physicalDmg != 0
                        || gSpecialStatuses[gBattlerTarget].specialDmg != 0)
                    && Random() as i32 % 100 < atkHoldEffectParam as i32
                    && (*(&raw const crate::data::pokemon::gBattleMoves)
                        .cast::<CArray<BattleMove, 0>>())[gCurrentMove]
                        .flags as i32
                        & FLAG_KINGS_ROCK_AFFECTED
                        != 0
                    && gBattleMons[gBattlerTarget].hp != 0
                {
                    gBattleCommunication[3] = MOVE_EFFECT_FLINCH;
                    BattleScriptPushCursor();
                    SetMoveEffect(0, 0);
                    BattleScriptPop();
                }
            }
            HOLD_EFFECT_SHELL_BELL
                if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0
                    && gSpecialStatuses[gBattlerTarget].shellBellDmg != 0
                    && gSpecialStatuses[gBattlerTarget].shellBellDmg != IGNORE_SHELL_BELL
                    && gBattlerAttacker != gBattlerTarget
                    && gBattleMons[gBattlerAttacker].hp != gBattleMons[gBattlerAttacker].maxHP
                    && gBattleMons[gBattlerAttacker].hp != 0 =>
            {
                gLastUsedItem = atkItem;
                gPotentialItemEffectBattler = gBattlerAttacker;
                gBattleScripting.battler = gBattlerAttacker;
                gBattleMoveDamage = -div_i32(
                    gSpecialStatuses[gBattlerTarget].shellBellDmg,
                    atkHoldEffectParam as i32,
                );
                if gBattleMoveDamage == 0 {
                    gBattleMoveDamage = -1;
                }
                gSpecialStatuses[gBattlerTarget].shellBellDmg = 0;
                BattleScriptPushCursor();
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_ItemHealHP_Ret
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
                effect += 1;
            }
            _ => {}
        },
        _ => {}
    }
    effect
}
pub unsafe fn ClearFuryCutterDestinyBondGrudge(battler: u8) {
    gDisableStructs[battler].furyCutterCounter = 0;
    gBattleMons[battler].status2 &= 0xfdffffff;
    gStatuses3[battler] &= 0xffffbfff;
}
pub unsafe fn HandleAction_RunBattleScript() {
    if gBattleControllerExecFlags == 0 {
        (*(&raw const crate::data::battle_script_commands::gBattleScriptingCommandsTable)
            .cast::<CArray<Option<unsafe fn()>, 0>>())[*gBattlescriptCurrInstr]
            .unwrap_unchecked()();
    }
}
pub unsafe fn GetMoveTarget(r#move: u16, setTarget: u8) -> u8 {
    let mut targetBattler: u8 = 0;
    let mut moveTarget: u8 = 0;
    let mut side: u8 = 0;
    if setTarget != NO_TARGET_OVERRIDE {
        moveTarget = setTarget - 1;
    } else {
        moveTarget = (*(&raw const crate::data::pokemon::gBattleMoves)
            .cast::<CArray<BattleMove, 0>>())[r#move]
            .target;
    }
    match moveTarget {
        MOVE_TARGET_SELECTED => {
            side = GetBattlerSide(gBattlerAttacker) ^ 1;
            if gSideTimers[side].followmeTimer != 0
                && gBattleMons[gSideTimers[side].followmeTarget].hp != 0
            {
                targetBattler = gSideTimers[side].followmeTarget;
            } else {
                side = GetBattlerSide(gBattlerAttacker);
                loop {
                    targetBattler = rem_i32(Random() as i32, gBattlersCount as i32) as u8;
                    if !(targetBattler == gBattlerAttacker
                        || side == GetBattlerSide(targetBattler)
                        || gAbsentBattlerFlags as u32
                            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                                [targetBattler]
                            != 0)
                    {
                        break;
                    }
                }
                if (*(&raw const crate::data::pokemon::gBattleMoves)
                    .cast::<CArray<BattleMove, 0>>())[r#move]
                    .r#type
                    == TYPE_ELECTRIC
                    && AbilityBattleEffects(
                        ABILITYEFFECT_COUNT_OTHER_SIDE,
                        gBattlerAttacker,
                        ABILITY_LIGHTNING_ROD,
                        0,
                        0,
                    ) != 0
                    && gBattleMons[targetBattler].ability != ABILITY_LIGHTNING_ROD
                {
                    targetBattler ^= BIT_FLANK;
                    RecordAbilityBattle(targetBattler, gBattleMons[targetBattler].ability);
                    gSpecialStatuses[targetBattler].set_lightningRodRedirected(1);
                }
            }
        }
        MOVE_TARGET_DEPENDS
        | MOVE_TARGET_BOTH
        | MOVE_TARGET_FOES_AND_ALLY
        | MOVE_TARGET_OPPONENTS_FIELD => {
            targetBattler = GetBattlerAtPosition(GetBattlerPosition(gBattlerAttacker) & 1 ^ 1);
            if gAbsentBattlerFlags as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[targetBattler]
                != 0
            {
                targetBattler ^= BIT_FLANK;
            }
        }
        4 => {
            side = GetBattlerSide(gBattlerAttacker) ^ 1;
            if gSideTimers[side].followmeTimer != 0
                && gBattleMons[gSideTimers[side].followmeTarget].hp != 0
            {
                targetBattler = gSideTimers[side].followmeTarget;
            } else if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
                && moveTarget as i32 & MOVE_TARGET_RANDOM != 0
            {
                if GetBattlerSide(gBattlerAttacker) == B_SIDE_PLAYER {
                    if Random() as i32 & 1 != 0 {
                        targetBattler = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
                    } else {
                        targetBattler = GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT);
                    }
                } else {
                    if Random() as i32 & 1 != 0 {
                        targetBattler = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
                    } else {
                        targetBattler = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT);
                    }
                }
                if gAbsentBattlerFlags as u32
                    & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[targetBattler]
                    != 0
                {
                    targetBattler ^= BIT_FLANK;
                }
            } else {
                targetBattler = GetBattlerAtPosition(GetBattlerPosition(gBattlerAttacker) & 1 ^ 1);
            }
        }
        MOVE_TARGET_USER_OR_SELECTED | MOVE_TARGET_USER => {
            targetBattler = gBattlerAttacker;
        }
        _ => {}
    }
    *(*gBattleStruct)
        .moveTarget
        .as_mut_ptr()
        .at(gBattlerAttacker) = targetBattler;
    targetBattler
}
unsafe fn IsBattlerModernFatefulEncounter(battler: u8) -> u32 {
    if GetBattlerSide(battler) == B_SIDE_OPPONENT {
        return TRUE as u32;
    }
    if GetMonData3(
        &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
        MON_DATA_SPECIES,
        null_mut(),
    ) != SPECIES_DEOXYS
        && GetMonData3(
            &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
            MON_DATA_SPECIES,
            null_mut(),
        ) != SPECIES_MEW
    {
        return TRUE as u32;
    }
    GetMonData3(
        &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
        MON_DATA_MODERN_FATEFUL_ENCOUNTER,
        null_mut(),
    )
}
pub unsafe fn IsMonDisobedient() -> u8 {
    let mut obedienceLevel: u8 = 0;
    if gBattleTypeFlags & 0x2000002 != 0 {
        return DISOBEDIENCE_OBEDIENT;
    }
    if GetBattlerSide(gBattlerAttacker) == B_SIDE_OPPONENT {
        return DISOBEDIENCE_OBEDIENT;
    }
    if IsBattlerModernFatefulEncounter(gBattlerAttacker) != 0 {
        if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0
            && GetBattlerPosition(gBattlerAttacker) == 2
        {
            return DISOBEDIENCE_OBEDIENT;
        }
        if gBattleTypeFlags & BATTLE_TYPE_FRONTIER != 0 {
            return DISOBEDIENCE_OBEDIENT;
        }
        if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
            return DISOBEDIENCE_OBEDIENT;
        }
        if IsOtherTrainer(
            gBattleMons[gBattlerAttacker].otId,
            gBattleMons[gBattlerAttacker].otName.as_mut_ptr(),
        ) == 0
        {
            return DISOBEDIENCE_OBEDIENT;
        }
        if FlagGet(FLAG_BADGE08_GET) != 0 {
            return DISOBEDIENCE_OBEDIENT;
        }
        obedienceLevel = 10;
        if FlagGet(FLAG_BADGE02_GET) != 0 {
            obedienceLevel = 30;
        }
        if FlagGet(FLAG_BADGE04_GET) != 0 {
            obedienceLevel = 50;
        }
        if FlagGet(FLAG_BADGE06_GET) != 0 {
            obedienceLevel = 70;
        }
    }
    if gBattleMons[gBattlerAttacker].level <= obedienceLevel {
        return DISOBEDIENCE_OBEDIENT;
    }
    let mut rnd: i32 = Random() as i32 & 255;
    let mut calc: i32 =
        ((gBattleMons[gBattlerAttacker].level as i32 + obedienceLevel as i32) * rnd) >> 8;
    if calc < obedienceLevel as i32 {
        return DISOBEDIENCE_OBEDIENT;
    }
    if gCurrentMove == MOVE_RAGE {
        gBattleMons[gBattlerAttacker].status2 &= 0xff7fffff;
    }
    if gBattleMons[gBattlerAttacker].status1 & STATUS1_SLEEP != 0
        && (gCurrentMove == MOVE_SNORE || gCurrentMove == MOVE_SLEEP_TALK)
    {
        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_IgnoresWhileAsleep
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
        return DISOBEDIENCE_IGNORED;
    }
    rnd = Random() as i32 & 255;
    calc = ((gBattleMons[gBattlerAttacker].level as i32 + obedienceLevel as i32) * rnd) >> 8;
    if calc < obedienceLevel as i32 {
        calc = CheckMoveLimitations(
            gBattlerAttacker,
            gBitTable[gCurrMovePos] as u8,
            MOVE_LIMITATIONS_ALL,
        ) as i32;
        if calc == ALL_MOVES_MASK as i32 {
            gBattleCommunication[5] = (if 0 != 0 {
                Random() as i32 % 4
            } else {
                Random() as i32 & 3
            }) as u8;
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_MoveUsedLoafingAround
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
            return DISOBEDIENCE_IGNORED;
        } else {
            loop {
                gCurrMovePos = {
                    gChosenMovePos = (if 0 != 0 {
                        Random() as i32 % 4
                    } else {
                        Random() as i32 & 3
                    }) as u8;
                    gChosenMovePos
                };
                if gBitTable[gCurrMovePos] & calc as u32 == 0 {
                    break;
                }
            }
            gCalledMove = gBattleMons[gBattlerAttacker].moves[gCurrMovePos];
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_IgnoresAndUsesRandomMove
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
            gBattlerTarget = GetMoveTarget(gCalledMove, NO_TARGET_OVERRIDE);
            gHitMarker |= HITMARKER_DISOBEDIENT_MOVE;
            return DISOBEDIENCE_OTHER;
        }
    } else {
        obedienceLevel = gBattleMons[gBattlerAttacker].level - obedienceLevel;
        calc = Random() as i32 & 255;
        if calc < obedienceLevel as i32
            && gBattleMons[gBattlerAttacker].status1 & STATUS1_ANY == 0
            && gBattleMons[gBattlerAttacker].ability != ABILITY_VITAL_SPIRIT
            && gBattleMons[gBattlerAttacker].ability != ABILITY_INSOMNIA
        {
            let mut i: i32 = 0;
            while i < gBattlersCount as i32 {
                if gBattleMons[i].status2 & STATUS2_UPROAR != 0 {
                    break;
                }
                i += 1;
            }
            if i == gBattlersCount as i32 {
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_IgnoresAndFallsAsleep
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
                return DISOBEDIENCE_IGNORED;
            }
        }
        calc -= obedienceLevel as i32;
        if calc < obedienceLevel as i32 {
            gBattleMoveDamage = CalculateBaseDamage(
                &raw mut gBattleMons[gBattlerAttacker],
                &raw mut gBattleMons[gBattlerAttacker],
                MOVE_POUND,
                0,
                40,
                0,
                gBattlerAttacker,
                gBattlerAttacker,
            );
            gBattlerTarget = gBattlerAttacker;
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_IgnoresAndHitsItself
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
            gHitMarker |= HITMARKER_UNABLE_TO_USE_MOVE;
            return DISOBEDIENCE_OTHER;
        } else {
            gBattleCommunication[5] = (if 0 != 0 {
                Random() as i32 % 4
            } else {
                Random() as i32 & 3
            }) as u8;
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_MoveUsedLoafingAround
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
            return DISOBEDIENCE_IGNORED;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
