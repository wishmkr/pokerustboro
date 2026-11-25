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
    gBattlerAttacker.set({
        let gl_i0 = gCurrentTurnActionNumber.get();
        crate::gl!(gBattlerByTurnOrder[gl_i0])
    });
    if (*crate::gl!(gBattleStruct)).absentBattlerFlags as u32
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gBattlerAttacker.get()]
        != 0
    {
        gCurrentActionFuncId.set(B_ACTION_FINISHED);
        return;
    }
    gCritMultiplier.set(1);
    crate::gl!(gBattleScripting.dmgMultiplier = 1);
    (*crate::gl!(gBattleStruct)).atkCancelerTracker = 0;
    gMoveResultFlags.set(0);
    gMultiHitCounter.set(0);
    crate::gl!(gBattleCommunication[6] = 0);
    gCurrMovePos.set({
        gChosenMovePos.set(
            *(*crate::gl!(gBattleStruct))
                .chosenMovePositions
                .as_mut_ptr()
                .at(gBattlerAttacker.get()),
        );
        gChosenMovePos.get()
    });
    if (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].noValidMoves() != 0 {
        (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].set_noValidMoves(FALSE as u32);
        gCurrentMove.set({
            gChosenMove.set(MOVE_STRUGGLE);
            gChosenMove.get()
        });
        gHitMarker.set(gHitMarker.get() | (HITMARKER_NO_PPDEDUCT));
        *(*crate::gl!(gBattleStruct))
            .moveTarget
            .as_mut_ptr()
            .at(gBattlerAttacker.get()) = GetMoveTarget(MOVE_STRUGGLE, NO_TARGET_OVERRIDE);
    } else if ({
        let gl_i0 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i0].status2)
    }) & STATUS2_MULTIPLETURNS
        != 0
        || ({
            let gl_i1 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i1].status2)
        }) & STATUS2_RECHARGE
            != 0
    {
        gCurrentMove.set({
            gChosenMove.set({
                let gl_i1 = gBattlerAttacker.get();
                crate::gl!(gLockedMoves[gl_i1])
            });
            gChosenMove.get()
        });
    } else if ({
        let gl_i2 = gBattlerAttacker.get();
        crate::gl!(gDisableStructs[gl_i2].encoredMove)
    }) != MOVE_NONE
        && ({
            let gl_i3 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i3].encoredMove)
        }) == (*gBattleMons.as_ptr())[gBattlerAttacker.get()].moves[{
            let gl_i5 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i5].encoredMovePos)
        }]
    {
        gCurrentMove.set({
            gChosenMove.set({
                let gl_i6 = gBattlerAttacker.get();
                crate::gl!(gDisableStructs[gl_i6].encoredMove)
            });
            gChosenMove.get()
        });
        gCurrMovePos.set({
            gChosenMovePos.set({
                let gl_i7 = gBattlerAttacker.get();
                crate::gl!(gDisableStructs[gl_i7].encoredMovePos)
            });
            gChosenMovePos.get()
        });
        *(*crate::gl!(gBattleStruct))
            .moveTarget
            .as_mut_ptr()
            .at(gBattlerAttacker.get()) = GetMoveTarget(gCurrentMove.get(), NO_TARGET_OVERRIDE);
    } else if ({
        let gl_i8 = gBattlerAttacker.get();
        crate::gl!(gDisableStructs[gl_i8].encoredMove)
    }) != MOVE_NONE
        && ({
            let gl_i9 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i9].encoredMove)
        }) != (*gBattleMons.as_ptr())[gBattlerAttacker.get()].moves[{
            let gl_i11 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i11].encoredMovePos)
        }]
    {
        gCurrMovePos.set({
            gChosenMovePos.set({
                let gl_i12 = gBattlerAttacker.get();
                crate::gl!(gDisableStructs[gl_i12].encoredMovePos)
            });
            gChosenMovePos.get()
        });
        gCurrentMove.set({
            gChosenMove.set({
                let gl_i13 = gBattlerAttacker.get();
                let gl_i14 = gCurrMovePos.get();
                crate::gl!(gBattleMons[gl_i13].moves[gl_i14])
            });
            gChosenMove.get()
        });
        {
            let gl_i15 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i15].encoredMove = MOVE_NONE)
        };
        {
            let gl_i16 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i16].encoredMovePos = 0)
        };
        (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].set_encoreTimer(0);
        *(*crate::gl!(gBattleStruct))
            .moveTarget
            .as_mut_ptr()
            .at(gBattlerAttacker.get()) = GetMoveTarget(gCurrentMove.get(), NO_TARGET_OVERRIDE);
    } else if ({
        let gl_i17 = gBattlerAttacker.get();
        let gl_i18 = gCurrMovePos.get();
        crate::gl!(gBattleMons[gl_i17].moves[gl_i18])
    }) != {
        let gl_i2 = gBattlerAttacker.get();
        crate::gl!(gChosenMoveByBattler[gl_i2])
    } {
        gCurrentMove.set({
            gChosenMove.set({
                let gl_i19 = gBattlerAttacker.get();
                let gl_i20 = gCurrMovePos.get();
                crate::gl!(gBattleMons[gl_i19].moves[gl_i20])
            });
            gChosenMove.get()
        });
        *(*crate::gl!(gBattleStruct))
            .moveTarget
            .as_mut_ptr()
            .at(gBattlerAttacker.get()) = GetMoveTarget(gCurrentMove.get(), NO_TARGET_OVERRIDE);
    } else {
        gCurrentMove.set({
            gChosenMove.set({
                let gl_i21 = gBattlerAttacker.get();
                let gl_i22 = gCurrMovePos.get();
                crate::gl!(gBattleMons[gl_i21].moves[gl_i22])
            });
            gChosenMove.get()
        });
    }
    if ({
        let gl_i23 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i23].hp)
    }) != 0
    {
        if GetBattlerSide(gBattlerAttacker.get()) == B_SIDE_PLAYER {
            crate::gl!(gBattleResults.lastUsedMovePlayer = gCurrentMove.get());
        } else {
            crate::gl!(gBattleResults.lastUsedMoveOpponent = gCurrentMove.get());
        }
    }
    let mut side: u8 = GetBattlerSide(gBattlerAttacker.get()) ^ 1;
    if crate::gl!(gSideTimers[side].followmeTimer) != 0
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .target
            == MOVE_TARGET_SELECTED
        && GetBattlerSide(gBattlerAttacker.get())
            != GetBattlerSide(crate::gl!(gSideTimers[side].followmeTarget))
        && ({
            let gl_i24 = crate::gl!(gSideTimers[side].followmeTarget);
            crate::gl!(gBattleMons[gl_i24].hp)
        }) != 0
    {
        gBattlerTarget.set(crate::gl!(gSideTimers[side].followmeTarget));
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0
        && crate::gl!(gSideTimers[side].followmeTimer) == 0
        && ((*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .power
            != 0
            || (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [gCurrentMove.get()]
            .target
                != MOVE_TARGET_USER)
        && (*gBattleMons.as_ptr())[*(*crate::gl!(gBattleStruct))
            .moveTarget
            .as_mut_ptr()
            .at(gBattlerAttacker.get())]
        .ability
            != ABILITY_LIGHTNING_ROD
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .r#type
            == TYPE_ELECTRIC
    {
        side = GetBattlerSide(gBattlerAttacker.get());
        gActiveBattler.set(0);
        while gActiveBattler.get() < gBattlersCount.get() {
            if side != GetBattlerSide(gActiveBattler.get())
                && *(*crate::gl!(gBattleStruct))
                    .moveTarget
                    .as_mut_ptr()
                    .at(gBattlerAttacker.get())
                    != gActiveBattler.get()
                && ({
                    let gl_i25 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i25].ability)
                }) == ABILITY_LIGHTNING_ROD
                && GetBattlerTurnOrderNum(gActiveBattler.get()) < var
            {
                var = GetBattlerTurnOrderNum(gActiveBattler.get());
            }
            gActiveBattler.set(gActiveBattler.get() + (1));
        }
        if var == 4 {
            if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [gChosenMove.get()]
            .target as i32
                & MOVE_TARGET_RANDOM
                != 0
            {
                if GetBattlerSide(gBattlerAttacker.get()) == B_SIDE_PLAYER {
                    if Random() as i32 & 1 != 0 {
                        gBattlerTarget.set(GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT));
                    } else {
                        gBattlerTarget.set(GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT));
                    }
                } else {
                    if Random() as i32 & 1 != 0 {
                        gBattlerTarget.set(GetBattlerAtPosition(B_POSITION_PLAYER_LEFT));
                    } else {
                        gBattlerTarget.set(GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT));
                    }
                }
            } else {
                gBattlerTarget.set(
                    *(*crate::gl!(gBattleStruct))
                        .moveTarget
                        .as_mut_ptr()
                        .at(gBattlerAttacker.get()),
                );
            }
            if gAbsentBattlerFlags.get() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                    [gBattlerTarget.get()]
                != 0
            {
                if GetBattlerSide(gBattlerAttacker.get()) != GetBattlerSide(gBattlerTarget.get()) {
                    gBattlerTarget.set(GetBattlerAtPosition(
                        GetBattlerPosition(gBattlerTarget.get()) ^ 2,
                    ));
                } else {
                    gBattlerTarget.set(GetBattlerAtPosition(
                        GetBattlerPosition(gBattlerAttacker.get()) ^ 1,
                    ));
                    if gAbsentBattlerFlags.get() as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                            [gBattlerTarget.get()]
                        != 0
                    {
                        gBattlerTarget.set(GetBattlerAtPosition(
                            GetBattlerPosition(gBattlerTarget.get()) ^ 2,
                        ));
                    }
                }
            }
        } else {
            gActiveBattler.set(crate::gl!(gBattlerByTurnOrder[var]));
            RecordAbilityBattle(gActiveBattler.get(), {
                let gl_i26 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i26].ability)
            });
            (*gSpecialStatuses.as_ptr())[gActiveBattler.get()].set_lightningRodRedirected(1);
            gBattlerTarget.set(gActiveBattler.get());
        }
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gChosenMove.get()]
        .target as i32
            & MOVE_TARGET_RANDOM
            != 0
    {
        if GetBattlerSide(gBattlerAttacker.get()) == B_SIDE_PLAYER {
            if Random() as i32 & 1 != 0 {
                gBattlerTarget.set(GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT));
            } else {
                gBattlerTarget.set(GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT));
            }
        } else {
            if Random() as i32 & 1 != 0 {
                gBattlerTarget.set(GetBattlerAtPosition(B_POSITION_PLAYER_LEFT));
            } else {
                gBattlerTarget.set(GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT));
            }
        }
        if gAbsentBattlerFlags.get() as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gBattlerTarget.get()]
            != 0
            && GetBattlerSide(gBattlerAttacker.get()) != GetBattlerSide(gBattlerTarget.get())
        {
            gBattlerTarget.set(GetBattlerAtPosition(
                GetBattlerPosition(gBattlerTarget.get()) ^ 2,
            ));
        }
    } else {
        gBattlerTarget.set(
            *(*crate::gl!(gBattleStruct))
                .moveTarget
                .as_mut_ptr()
                .at(gBattlerAttacker.get()),
        );
        if gAbsentBattlerFlags.get() as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gBattlerTarget.get()]
            != 0
        {
            if GetBattlerSide(gBattlerAttacker.get()) != GetBattlerSide(gBattlerTarget.get()) {
                gBattlerTarget.set(GetBattlerAtPosition(
                    GetBattlerPosition(gBattlerTarget.get()) ^ 2,
                ));
            } else {
                gBattlerTarget.set(GetBattlerAtPosition(
                    GetBattlerPosition(gBattlerAttacker.get()) ^ 1,
                ));
                if gAbsentBattlerFlags.get() as u32
                    & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                        [gBattlerTarget.get()]
                    != 0
                {
                    gBattlerTarget.set(GetBattlerAtPosition(
                        GetBattlerPosition(gBattlerTarget.get()) ^ 2,
                    ));
                }
            }
        }
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_PALACE != 0
        && (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].palaceUnableToUseMove() != 0
    {
        if ({
            let gl_i27 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i27].hp)
        }) == 0
        {
            gCurrentActionFuncId.set(B_ACTION_FINISHED);
            return;
        } else if !(*gPalaceSelectionBattleScripts.as_ptr())[gBattlerAttacker.get()].is_null() {
            crate::gl!(gBattleCommunication[5] = B_MSG_INCAPABLE_OF_POWER);
            crate::gl!(
                gBattlescriptCurrInstr = {
                    let gl_i3 = gBattlerAttacker.get();
                    crate::gl!(gPalaceSelectionBattleScripts[gl_i3])
                }
            );
            {
                let gl_v = null_mut();
                let gl_i4 = gBattlerAttacker.get();
                crate::gl!(gPalaceSelectionBattleScripts[gl_i4] = gl_v)
            };
        } else {
            crate::gl!(gBattleCommunication[5] = B_MSG_INCAPABLE_OF_POWER);
            crate::gl!(
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_MoveUsedLoafingAround
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            );
        }
    } else {
        crate::gl!(
            gBattlescriptCurrInstr =
                (*crate::asmdata::gBattleScriptsForMoveEffects.cast::<CArray<*mut u8, 0>>())
                    [(*(&raw const crate::data::pokemon::gBattleMoves)
                        .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
                    .effect]
        );
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_ARENA != 0 {
        BattleArena_AddMindPoints(gBattlerAttacker.get());
    }
    gCurrentActionFuncId.set(B_ACTION_EXEC_SCRIPT);
}
pub unsafe fn HandleAction_Switch() {
    gBattlerAttacker.set({
        let gl_i5 = gCurrentTurnActionNumber.get();
        crate::gl!(gBattlerByTurnOrder[gl_i5])
    });
    gBattle_BG0_X.set(0);
    gBattle_BG0_Y.set(0);
    {
        let gl_i28 = gBattlerAttacker.get();
        crate::gl!(gActionSelectionCursor[gl_i28] = 0)
    };
    {
        let gl_i29 = gBattlerAttacker.get();
        crate::gl!(gMoveSelectionCursor[gl_i29] = 0)
    };
    crate::gl!(gBattleTextBuff1[0] = 0xFD);
    crate::gl!(gBattleTextBuff1[1] = 7);
    crate::gl!(gBattleTextBuff1[2] = gBattlerAttacker.get());
    crate::gl!(
        gBattleTextBuff1[3] = *(*crate::gl!(gBattleStruct))
            .battlerPartyIndexes
            .as_mut_ptr()
            .at(gBattlerAttacker.get())
    );
    crate::gl!(gBattleTextBuff1[4] = 0xFF);
    crate::gl!(gBattleScripting.battler = gBattlerAttacker.get());
    crate::gl!(
        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_ActionSwitch
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut()
    );
    gCurrentActionFuncId.set(B_ACTION_EXEC_SCRIPT);
    crate::gl!(
        gBattleResults.playerSwitchesCounter = (*gBattleResults.as_ptr())
            .playerSwitchesCounter
            .saturating_add(1)
    );
}
pub unsafe fn HandleAction_UseItem() {
    gBattlerAttacker.set({
        gBattlerTarget.set({
            let gl_i6 = gCurrentTurnActionNumber.get();
            crate::gl!(gBattlerByTurnOrder[gl_i6])
        });
        gBattlerTarget.get()
    });
    gBattle_BG0_X.set(0);
    gBattle_BG0_Y.set(0);
    ClearFuryCutterDestinyBondGrudge(gBattlerAttacker.get());
    gLastUsedItem.set(
        {
            let gl_i7 = gBattlerAttacker.get();
            crate::gl!(gBattleBufferB[gl_i7][1])
        } as u16
            | ({
                let gl_i8 = gBattlerAttacker.get();
                crate::gl!(gBattleBufferB[gl_i8][2])
            } as u16)
                << 8,
    );
    if gLastUsedItem.get() <= ITEM_PREMIER_BALL {
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::gBattlescriptsForBallThrow
                .cast::<CArray<*mut u8, 0>>())[gLastUsedItem.get()]
        );
    } else if gLastUsedItem.get() == ITEM_POKE_DOLL || gLastUsedItem.get() == ITEM_FLUFFY_TAIL {
        crate::gl!(
            gBattlescriptCurrInstr =
                (*crate::asmdata::gBattlescriptsForRunningByItem.cast::<CArray<*mut u8, 0>>())[0]
        );
    } else if GetBattlerSide(gBattlerAttacker.get()) == B_SIDE_PLAYER {
        crate::gl!(
            gBattlescriptCurrInstr =
                (*crate::asmdata::gBattlescriptsForUsingItem.cast::<CArray<*mut u8, 0>>())[0]
        );
    } else {
        crate::gl!(gBattleScripting.battler = gBattlerAttacker.get());
        match *(*crate::gl!(gBattleStruct))
            .AI_itemType
            .as_mut_ptr()
            .at(gBattlerAttacker.get() >> 1)
        {
            AI_ITEM_FULL_RESTORE | AI_ITEM_HEAL_HP => {}
            AI_ITEM_CURE_CONDITION => {
                crate::gl!(gBattleCommunication[5] = AI_HEAL_CONFUSION);
                if *(*crate::gl!(gBattleStruct))
                    .AI_itemFlags
                    .as_mut_ptr()
                    .at(gBattlerAttacker.get() as i32 / 2) as i32
                    & 1
                    != 0
                {
                    if *(*crate::gl!(gBattleStruct))
                        .AI_itemFlags
                        .as_mut_ptr()
                        .at(gBattlerAttacker.get() as i32 / 2) as i32
                        & 62
                        != 0
                    {
                        crate::gl!(gBattleCommunication[5] = AI_HEAL_SLEEP);
                    }
                } else {
                    while *(*crate::gl!(gBattleStruct))
                        .AI_itemFlags
                        .as_mut_ptr()
                        .at(gBattlerAttacker.get() as i32 / 2) as i32
                        & 1
                        == 0
                    {
                        *(*crate::gl!(gBattleStruct))
                            .AI_itemFlags
                            .as_mut_ptr()
                            .at(gBattlerAttacker.get() as i32 / 2) >>= 1;
                        crate::gl!(gBattleCommunication[5] += 1);
                    }
                }
            }
            AI_ITEM_X_STAT => {
                crate::gl!(gBattleCommunication[5] = B_MSG_STAT_ROSE_ITEM);
                if *(*crate::gl!(gBattleStruct))
                    .AI_itemFlags
                    .as_mut_ptr()
                    .at(gBattlerAttacker.get() >> 1) as i32
                    & 128
                    != 0
                {
                    crate::gl!(gBattleCommunication[5] = 5);
                } else {
                    crate::gl!(gBattleTextBuff1[0] = 0xFD);
                    crate::gl!(gBattleTextBuff1[1] = 5);
                    crate::gl!(gBattleTextBuff1[2] = STAT_ATK);
                    crate::gl!(gBattleTextBuff1[3] = 0xFF);
                    crate::gl!(gBattleTextBuff2[0] = 0xFD);
                    crate::gl!(gBattleTextBuff2[1] = 0);
                    crate::gl!(gBattleTextBuff2[2] = CHAR_X);
                    crate::gl!(gBattleTextBuff2[3] = 0);
                    crate::gl!(gBattleTextBuff2[4] = 0xFF);
                    while *(*crate::gl!(gBattleStruct))
                        .AI_itemFlags
                        .as_mut_ptr()
                        .at(gBattlerAttacker.get() >> 1) as i32
                        & 1
                        == 0
                    {
                        *(*crate::gl!(gBattleStruct))
                            .AI_itemFlags
                            .as_mut_ptr()
                            .at(gBattlerAttacker.get() as i32 / 2) >>= 1;
                        crate::gl!(gBattleTextBuff1[2] += 1);
                    }
                    crate::gl!(
                        gBattleScripting.animArg1 =
                            crate::gl!(gBattleTextBuff1[2]) + STAT_ANIM_PLUS1 as u8
                    );
                    crate::gl!(gBattleScripting.animArg2 = 0);
                }
            }
            AI_ITEM_GUARD_SPEC => {
                crate::gl!(gBattleCommunication[5] = B_MSG_SET_MIST);
            }
            _ => {}
        }
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::gBattlescriptsForUsingItem
                .cast::<CArray<*mut u8, 0>>())[*(*crate::gl!(gBattleStruct))
                .AI_itemType
                .as_mut_ptr()
                .at(gBattlerAttacker.get() as i32 / 2)]
        );
    }
    gCurrentActionFuncId.set(B_ACTION_EXEC_SCRIPT);
}
pub unsafe fn TryRunFromBattle(battler: u8) -> u8 {
    let mut effect: u8 = FALSE;
    let mut holdEffect: u8 = 0;
    let mut pyramidMultiplier: u8 = 0;
    let mut speedVar: u8 = 0;
    if crate::gl!(gBattleMons[battler].item) == ITEM_ENIGMA_BERRY {
        holdEffect = crate::gl!(gEnigmaBerries[battler].holdEffect);
    } else {
        holdEffect = GetItemHoldEffect(crate::gl!(gBattleMons[battler].item));
    }
    gPotentialItemEffectBattler.set(battler);
    if holdEffect == HOLD_EFFECT_CAN_ALWAYS_RUN {
        gLastUsedItem.set(crate::gl!(gBattleMons[battler].item));
        (*gProtectStructs.as_ptr())[battler].set_fleeType(FLEE_ITEM);
        effect += 1;
    } else if crate::gl!(gBattleMons[battler].ability) == ABILITY_RUN_AWAY {
        if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
            (*crate::gl!(gBattleStruct)).runTries += 1;
            pyramidMultiplier = GetPyramidRunMultiplier();
            speedVar = div_i32(
                crate::gl!(gBattleMons[battler].speed) as i32 * pyramidMultiplier as i32,
                ({
                    let gl_i30 = battler as i32 ^ 1;
                    crate::gl!(gBattleMons[gl_i30].speed)
                }) as i32,
            ) as u8
                + (*crate::gl!(gBattleStruct)).runTries * 30;
            if speedVar as i32 > Random() as i32 & 0xFF {
                gLastUsedAbility.set(ABILITY_RUN_AWAY);
                (*gProtectStructs.as_ptr())[battler].set_fleeType(FLEE_ABILITY);
                effect += 1;
            }
        } else {
            gLastUsedAbility.set(ABILITY_RUN_AWAY);
            (*gProtectStructs.as_ptr())[battler].set_fleeType(FLEE_ABILITY);
            effect += 1;
        }
    } else if gBattleTypeFlags.get() & 0x43f0100 != 0
        && gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER != 0
    {
        effect += 1;
    } else {
        if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE == 0 {
            if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
                pyramidMultiplier = GetPyramidRunMultiplier();
                speedVar = div_i32(
                    crate::gl!(gBattleMons[battler].speed) as i32 * pyramidMultiplier as i32,
                    ({
                        let gl_i31 = battler as i32 ^ 1;
                        crate::gl!(gBattleMons[gl_i31].speed)
                    }) as i32,
                ) as u8
                    + (*crate::gl!(gBattleStruct)).runTries * 30;
                if speedVar as i32 > Random() as i32 & 0xFF {
                    effect += 1;
                }
            } else if crate::gl!(gBattleMons[battler].speed)
                < ({
                    let gl_i32 = battler as i32 ^ 1;
                    crate::gl!(gBattleMons[gl_i32].speed)
                })
            {
                speedVar = div_i32(
                    crate::gl!(gBattleMons[battler].speed) as i32 * 128,
                    ({
                        let gl_i33 = battler as i32 ^ 1;
                        crate::gl!(gBattleMons[gl_i33].speed)
                    }) as i32,
                ) as u8
                    + (*crate::gl!(gBattleStruct)).runTries * 30;
                if speedVar as i32 > Random() as i32 & 0xFF {
                    effect += 1;
                }
            } else {
                effect += 1;
            }
        }
        (*crate::gl!(gBattleStruct)).runTries += 1;
    }
    if effect != 0 {
        gCurrentTurnActionNumber.set(gBattlersCount.get());
        gBattleOutcome.set(B_OUTCOME_RAN);
    }
    effect
}
pub unsafe fn HandleAction_Run() {
    gBattlerAttacker.set({
        let gl_i9 = gCurrentTurnActionNumber.get();
        crate::gl!(gBattlerByTurnOrder[gl_i9])
    });
    if gBattleTypeFlags.get() & 0x2000002 != 0 {
        gCurrentTurnActionNumber.set(gBattlersCount.get());
        gActiveBattler.set(0);
        while gActiveBattler.get() < gBattlersCount.get() {
            if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
                if {
                    let gl_i10 = gActiveBattler.get();
                    crate::gl!(gChosenActionByBattler[gl_i10])
                } == B_ACTION_RUN
                {
                    gBattleOutcome.set(gBattleOutcome.get() | (B_OUTCOME_LOST));
                }
            } else {
                if {
                    let gl_i11 = gActiveBattler.get();
                    crate::gl!(gChosenActionByBattler[gl_i11])
                } == B_ACTION_RUN
                {
                    gBattleOutcome.set(gBattleOutcome.get() | (B_OUTCOME_WON));
                }
            }
            gActiveBattler.set(gActiveBattler.get() + (1));
        }
        gBattleOutcome.set(gBattleOutcome.get() | (B_OUTCOME_LINK_BATTLE_RAN as u8));
        (*crate::gl!(gSaveBlock2Ptr))
            .frontier
            .set_disableRecordBattle(TRUE);
    } else {
        if GetBattlerSide(gBattlerAttacker.get()) == B_SIDE_PLAYER {
            if TryRunFromBattle(gBattlerAttacker.get()) == 0 {
                ClearFuryCutterDestinyBondGrudge(gBattlerAttacker.get());
                crate::gl!(gBattleCommunication[5] = B_MSG_CANT_ESCAPE_2);
                crate::gl!(
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_PrintFailedToRunString
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                );
                gCurrentActionFuncId.set(B_ACTION_EXEC_SCRIPT);
            }
        } else {
            if ({
                let gl_i34 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i34].status2)
            }) & 0x400e000
                != 0
            {
                crate::gl!(gBattleCommunication[5] = B_MSG_ATTACKER_CANT_ESCAPE);
                crate::gl!(
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_PrintFailedToRunString
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                );
                gCurrentActionFuncId.set(B_ACTION_EXEC_SCRIPT);
            } else {
                gCurrentTurnActionNumber.set(gBattlersCount.get());
                gBattleOutcome.set(B_OUTCOME_MON_FLED);
            }
        }
    }
}
pub unsafe fn HandleAction_WatchesCarefully() {
    gBattlerAttacker.set({
        let gl_i12 = gCurrentTurnActionNumber.get();
        crate::gl!(gBattlerByTurnOrder[gl_i12])
    });
    gBattle_BG0_X.set(0);
    gBattle_BG0_Y.set(0);
    crate::gl!(
        gBattlescriptCurrInstr =
            (*crate::asmdata::gBattlescriptsForSafariActions.cast::<CArray<*mut u8, 0>>())[0]
    );
    gCurrentActionFuncId.set(B_ACTION_EXEC_SCRIPT);
}
pub unsafe fn HandleAction_SafariZoneBallThrow() {
    gBattlerAttacker.set({
        let gl_i13 = gCurrentTurnActionNumber.get();
        crate::gl!(gBattlerByTurnOrder[gl_i13])
    });
    gBattle_BG0_X.set(0);
    gBattle_BG0_Y.set(0);
    gNumSafariBalls.set(gNumSafariBalls.get() - (1));
    gLastUsedItem.set(ITEM_SAFARI_BALL);
    crate::gl!(
        gBattlescriptCurrInstr =
            (*crate::asmdata::gBattlescriptsForBallThrow.cast::<CArray<*mut u8, 0>>())[5]
    );
    gCurrentActionFuncId.set(B_ACTION_EXEC_SCRIPT);
}
pub unsafe fn HandleAction_ThrowPokeblock() {
    gBattlerAttacker.set({
        let gl_i14 = gCurrentTurnActionNumber.get();
        crate::gl!(gBattlerByTurnOrder[gl_i14])
    });
    gBattle_BG0_X.set(0);
    gBattle_BG0_Y.set(0);
    crate::gl!(
        gBattleCommunication[5] = {
            let gl_i15 = gBattlerAttacker.get();
            crate::gl!(gBattleBufferB[gl_i15][1])
        } - 1
    );
    gLastUsedItem.set({
        let gl_i16 = gBattlerAttacker.get();
        crate::gl!(gBattleBufferB[gl_i16][2])
    } as u16);
    crate::gl!(
        gBattleResults.pokeblockThrows =
            (*gBattleResults.as_ptr()).pokeblockThrows.saturating_add(1)
    );
    if (*crate::gl!(gBattleStruct)).safariPkblThrowCounter < 3 {
        (*crate::gl!(gBattleStruct)).safariPkblThrowCounter += 1;
    }
    if (*crate::gl!(gBattleStruct)).safariEscapeFactor > 1 {
        if (*crate::gl!(gBattleStruct)).safariEscapeFactor
            < sPkblToEscapeFactor[(*crate::gl!(gBattleStruct)).safariPkblThrowCounter]
                [crate::gl!(gBattleCommunication[5])]
        {
            (*crate::gl!(gBattleStruct)).safariEscapeFactor = 1;
        } else {
            (*crate::gl!(gBattleStruct)).safariEscapeFactor -= sPkblToEscapeFactor
                [(*crate::gl!(gBattleStruct)).safariPkblThrowCounter]
                [crate::gl!(gBattleCommunication[5])];
        }
    }
    crate::gl!(
        gBattlescriptCurrInstr =
            (*crate::asmdata::gBattlescriptsForSafariActions.cast::<CArray<*mut u8, 0>>())[2]
    );
    gCurrentActionFuncId.set(B_ACTION_EXEC_SCRIPT);
}
pub unsafe fn HandleAction_GoNear() {
    gBattlerAttacker.set({
        let gl_i17 = gCurrentTurnActionNumber.get();
        crate::gl!(gBattlerByTurnOrder[gl_i17])
    });
    gBattle_BG0_X.set(0);
    gBattle_BG0_Y.set(0);
    (*crate::gl!(gBattleStruct)).safariCatchFactor +=
        sGoNearCounterToCatchFactor[(*crate::gl!(gBattleStruct)).safariGoNearCounter];
    if (*crate::gl!(gBattleStruct)).safariCatchFactor > 20 {
        (*crate::gl!(gBattleStruct)).safariCatchFactor = 20;
    }
    (*crate::gl!(gBattleStruct)).safariEscapeFactor +=
        sGoNearCounterToEscapeFactor[(*crate::gl!(gBattleStruct)).safariGoNearCounter];
    if (*crate::gl!(gBattleStruct)).safariEscapeFactor > 20 {
        (*crate::gl!(gBattleStruct)).safariEscapeFactor = 20;
    }
    if (*crate::gl!(gBattleStruct)).safariGoNearCounter < 3 {
        (*crate::gl!(gBattleStruct)).safariGoNearCounter += 1;
        crate::gl!(gBattleCommunication[5] = B_MSG_CREPT_CLOSER);
    } else {
        crate::gl!(gBattleCommunication[5] = B_MSG_CANT_GET_CLOSER);
    }
    crate::gl!(
        gBattlescriptCurrInstr =
            (*crate::asmdata::gBattlescriptsForSafariActions.cast::<CArray<*mut u8, 0>>())[1]
    );
    gCurrentActionFuncId.set(B_ACTION_EXEC_SCRIPT);
}
pub unsafe fn HandleAction_SafariZoneRun() {
    gBattlerAttacker.set({
        let gl_i18 = gCurrentTurnActionNumber.get();
        crate::gl!(gBattlerByTurnOrder[gl_i18])
    });
    PlaySE(SE_FLEE);
    gCurrentTurnActionNumber.set(gBattlersCount.get());
    gBattleOutcome.set(B_OUTCOME_RAN);
}
pub unsafe fn HandleAction_WallyBallThrow() {
    gBattlerAttacker.set({
        let gl_i19 = gCurrentTurnActionNumber.get();
        crate::gl!(gBattlerByTurnOrder[gl_i19])
    });
    gBattle_BG0_X.set(0);
    gBattle_BG0_Y.set(0);
    crate::gl!(gBattleTextBuff1[0] = 0xFD);
    crate::gl!(gBattleTextBuff1[1] = 7);
    crate::gl!(gBattleTextBuff1[2] = gBattlerAttacker.get());
    crate::gl!(
        gBattleTextBuff1[3] = ({
            let gl_i35 = gBattlerAttacker.get();
            crate::gl!(gBattlerPartyIndexes[gl_i35])
        }) as u8
    );
    crate::gl!(gBattleTextBuff1[4] = 0xFF);
    crate::gl!(
        gBattlescriptCurrInstr =
            (*crate::asmdata::gBattlescriptsForSafariActions.cast::<CArray<*mut u8, 0>>())[3]
    );
    gCurrentActionFuncId.set(B_ACTION_EXEC_SCRIPT);
    crate::gl!(gActionsByTurnOrder[1] = B_ACTION_FINISHED);
}
pub unsafe fn HandleAction_TryFinish() {
    if HandleFaintedMonActions() == 0 {
        (*crate::gl!(gBattleStruct)).faintedActionsState = 0;
        gCurrentActionFuncId.set(B_ACTION_FINISHED);
    }
}
pub fn HandleAction_NothingIsFainted() {
    gCurrentTurnActionNumber.set(gCurrentTurnActionNumber.get() + (1));
    gCurrentActionFuncId.set({
        let gl_i20 = gCurrentTurnActionNumber.get();
        crate::gl!(gActionsByTurnOrder[gl_i20])
    });
    gHitMarker.set(gHitMarker.get() & (0xf1e892af));
}
pub unsafe fn HandleAction_ActionFinished() {
    *(*crate::gl!(gBattleStruct))
        .monToSwitchIntoId
        .as_mut_ptr()
        .at({
            let gl_i21 = gCurrentTurnActionNumber.get();
            crate::gl!(gBattlerByTurnOrder[gl_i21])
        }) = PARTY_SIZE as u8;
    gCurrentTurnActionNumber.set(gCurrentTurnActionNumber.get() + (1));
    gCurrentActionFuncId.set({
        let gl_i22 = gCurrentTurnActionNumber.get();
        crate::gl!(gActionsByTurnOrder[gl_i22])
    });
    SpecialStatusesClear();
    gHitMarker.set(gHitMarker.get() & (0xf1e892af));
    gCurrentMove.set(0);
    gBattleMoveDamage.set(0);
    gMoveResultFlags.set(0);
    crate::gl!(gBattleScripting.animTurn = 0);
    crate::gl!(gBattleScripting.animTargetsHit = 0);
    {
        let gl_v = 0;
        let gl_i23 = gBattlerAttacker.get();
        crate::gl!(gLastLandedMoves[gl_i23] = gl_v)
    };
    {
        let gl_v = 0;
        let gl_i24 = gBattlerAttacker.get();
        crate::gl!(gLastHitByType[gl_i24] = gl_v)
    };
    (*crate::gl!(gBattleStruct)).dynamicMoveType = 0;
    gDynamicBasePower.set(0);
    crate::gl!(gBattleScripting.moveendState = 0);
    crate::gl!(gBattleCommunication[3] = 0);
    crate::gl!(gBattleCommunication[4] = 0);
    crate::gl!(gBattleScripting.multihitMoveEffect = 0);
    (*(*crate::gl!(gBattleResources)).battleScriptsStack).size = 0;
}
pub unsafe fn GetBattlerForBattleScript(caseId: u8) -> u8 {
    let mut ret: u8 = 0;
    match caseId {
        BS_TARGET => {
            ret = gBattlerTarget.get();
        }
        BS_ATTACKER => {
            ret = gBattlerAttacker.get();
        }
        BS_EFFECT_BATTLER => {
            ret = gEffectBattler.get();
        }
        BS_BATTLER_0 => {
            ret = 0;
        }
        BS_SCRIPTING => {
            ret = crate::gl!(gBattleScripting.battler);
        }
        BS_FAINTED => {
            ret = gBattlerFainted.get();
        }
        BS_FAINTED_LINK_MULTIPLE_1 => {
            ret = gBattlerFainted.get();
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
    if crate::gl!(gBattleMons[target].ability) != ABILITY_PRESSURE {
        return;
    }
    let mut moveIndex: i32 = 0;
    while moveIndex < MAX_MON_MOVES {
        if crate::gl!(gBattleMons[attacker].moves[moveIndex]) == r#move {
            break;
        }
        moveIndex += 1;
    }
    if moveIndex == MAX_MON_MOVES {
        return;
    }
    if crate::gl!(gBattleMons[attacker].pp[moveIndex]) != 0 {
        crate::gl!(gBattleMons[attacker].pp[moveIndex] -= 1);
    }
    if crate::gl!(gBattleMons[attacker].status2) & 0x200000 == 0
        && (*gDisableStructs.as_ptr())[attacker].mimickedMoves() as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[moveIndex]
            == 0
    {
        gActiveBattler.set(attacker);
        BtlController_EmitSetMonData(
            B_COMM_TO_CONTROLLER,
            REQUEST_PPMOVE1_BATTLE + moveIndex as u8,
            0,
            1,
            ({
                let gl_i36 = gActiveBattler.get();
                crate::gl!(&raw gBattleMons[gl_i36].pp[moveIndex])
            }) as *mut c_void,
        );
        MarkBattlerForControllerExec(gActiveBattler.get());
    }
}
pub unsafe fn PressurePPLoseOnUsingImprison(attacker: u8) {
    let mut j: i32 = 0;
    let mut imprisonPos: i32 = MAX_MON_MOVES;
    let atkSide: u8 = GetBattlerSide(attacker);
    let mut i: i32 = 0;
    while i < gBattlersCount.get() as i32 {
        if atkSide != GetBattlerSide(i as u8)
            && crate::gl!(gBattleMons[i].ability) == ABILITY_PRESSURE
        {
            j = 0;
            while j < MAX_MON_MOVES {
                if crate::gl!(gBattleMons[attacker].moves[j]) == MOVE_IMPRISON {
                    break;
                }
                j += 1;
            }
            if j != MAX_MON_MOVES {
                imprisonPos = j;
                if crate::gl!(gBattleMons[attacker].pp[j]) != 0 {
                    crate::gl!(gBattleMons[attacker].pp[j] -= 1);
                }
            }
        }
        i += 1;
    }
    if imprisonPos != MAX_MON_MOVES
        && (crate::gl!(gBattleMons[attacker].status2) & 0x200000 == 0
            && (*gDisableStructs.as_ptr())[attacker].mimickedMoves() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[imprisonPos]
                == 0)
    {
        gActiveBattler.set(attacker);
        BtlController_EmitSetMonData(
            B_COMM_TO_CONTROLLER,
            REQUEST_PPMOVE1_BATTLE + imprisonPos as u8,
            0,
            1,
            ({
                let gl_i37 = gActiveBattler.get();
                crate::gl!(&raw gBattleMons[gl_i37].pp[imprisonPos])
            }) as *mut c_void,
        );
        MarkBattlerForControllerExec(gActiveBattler.get());
    }
}
pub unsafe fn PressurePPLoseOnUsingPerishSong(attacker: u8) {
    let mut j: i32 = 0;
    let mut perishSongPos: i32 = MAX_MON_MOVES;
    for i in 0..(gBattlersCount.get() as i32) {
        if crate::gl!(gBattleMons[i].ability) == ABILITY_PRESSURE && i != attacker as i32 {
            j = 0;
            while j < MAX_MON_MOVES {
                if crate::gl!(gBattleMons[attacker].moves[j]) == MOVE_PERISH_SONG {
                    break;
                }
                j += 1;
            }
            if j != MAX_MON_MOVES {
                perishSongPos = j;
                if crate::gl!(gBattleMons[attacker].pp[j]) != 0 {
                    crate::gl!(gBattleMons[attacker].pp[j] -= 1);
                }
            }
        }
    }
    if perishSongPos != MAX_MON_MOVES
        && (crate::gl!(gBattleMons[attacker].status2) & 0x200000 == 0
            && (*gDisableStructs.as_ptr())[attacker].mimickedMoves() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[perishSongPos]
                == 0)
    {
        gActiveBattler.set(attacker);
        BtlController_EmitSetMonData(
            B_COMM_TO_CONTROLLER,
            REQUEST_PPMOVE1_BATTLE + perishSongPos as u8,
            0,
            1,
            ({
                let gl_i38 = gActiveBattler.get();
                crate::gl!(&raw gBattleMons[gl_i38].pp[perishSongPos])
            }) as *mut c_void,
        );
        MarkBattlerForControllerExec(gActiveBattler.get());
    }
}
fn MarkAllBattlersForControllerExec() {
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
        for i in 0..(gBattlersCount.get() as i32) {
            gBattleControllerExecFlags.set(gBattleControllerExecFlags.get() | (gBitTable[i] << 28));
        }
    } else {
        for i in 0..(gBattlersCount.get() as i32) {
            gBattleControllerExecFlags.set(gBattleControllerExecFlags.get() | (gBitTable[i]));
        }
    }
}
#[unsafe(no_mangle)]
pub fn MarkBattlerForControllerExec(battler: u8) {
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
        gBattleControllerExecFlags
            .set(gBattleControllerExecFlags.get() | (gBitTable[battler] << 28));
    } else {
        gBattleControllerExecFlags.set(gBattleControllerExecFlags.get() | (gBitTable[battler]));
    }
}
pub unsafe fn MarkBattlerReceivedLinkData(battler: u8) {
    let mut i: i32 = 0;
    while i < GetLinkPlayerCount() as i32 {
        {
            let rhs = shl_u32(gBitTable[battler], (i as u32) << 2);
            gBattleControllerExecFlags.set(gBattleControllerExecFlags.get() | rhs)
        };
        i += 1;
    }
    {
        let rhs = !(shl_i32(0x10000000, battler as u32) as u32);
        gBattleControllerExecFlags.set(gBattleControllerExecFlags.get() & rhs)
    };
}
pub unsafe fn CancelMultiTurnMoves(battler: u8) {
    crate::gl!(gBattleMons[battler].status2 &= 0xffffefff);
    crate::gl!(gBattleMons[battler].status2 &= 0xfffff3ff);
    crate::gl!(gBattleMons[battler].status2 &= 0xffffff8f);
    crate::gl!(gBattleMons[battler].status2 &= 0xfffffcff);
    crate::gl!(gStatuses3[battler] &= 0xfffbff3f);
    (*gDisableStructs.as_ptr())[battler].set_rolloutTimer(0);
    crate::gl!(gDisableStructs[battler].furyCutterCounter = 0);
}
pub unsafe fn WasUnableToUseMove(battler: u8) -> u8 {
    if (*gProtectStructs.as_ptr())[battler].prlzImmobility() != 0
        || (*gProtectStructs.as_ptr())[battler].targetNotAffected() != 0
        || (*gProtectStructs.as_ptr())[battler].usedImprisonedMove() != 0
        || (*gProtectStructs.as_ptr())[battler].loveImmobility() != 0
        || (*gProtectStructs.as_ptr())[battler].usedDisabledMove() != 0
        || (*gProtectStructs.as_ptr())[battler].usedTauntedMove() != 0
        || (*gProtectStructs.as_ptr())[battler].flag2Unknown() != 0
        || (*gProtectStructs.as_ptr())[battler].flinchImmobility() != 0
        || (*gProtectStructs.as_ptr())[battler].confusionSelfDmg() != 0
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
    gActiveBattler.set(battler);
    BtlController_EmitPrintString(B_COMM_TO_CONTROLLER, stringId);
    MarkBattlerForControllerExec(gActiveBattler.get());
}
pub fn ResetSentPokesToOpponentValue() {
    let mut bits: u32 = 0;
    crate::gl!(gSentPokesToOpponent[0] = 0);
    crate::gl!(gSentPokesToOpponent[1] = 0);
    let mut i: i32 = 0;
    while i < gBattlersCount.get() as i32 {
        bits |= gBitTable[crate::gl!(gBattlerPartyIndexes[i])];
        i += 2;
    }
    i = 1;
    while i < gBattlersCount.get() as i32 {
        {
            let gl_v = bits as u8;
            let gl_i25 = (i & BIT_FLANK as i32) >> 1;
            crate::gl!(gSentPokesToOpponent[gl_i25] = gl_v)
        };
        i += 2;
    }
}
pub unsafe fn OpponentSwitchInResetSentPokesToOpponentValue(battler: u8) {
    let mut i: i32 = 0;
    let mut bits: u32 = 0;
    if GetBattlerSide(battler) == B_SIDE_OPPONENT {
        let flank: u8 = ((battler as i32 & BIT_FLANK as i32) >> 1) as u8;
        crate::gl!(gSentPokesToOpponent[flank] = 0);
        i = 0;
        while i < gBattlersCount.get() as i32 {
            if gAbsentBattlerFlags.get() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                == 0
            {
                bits |= gBitTable[crate::gl!(gBattlerPartyIndexes[i])];
            }
            i += 2;
        }
        crate::gl!(gSentPokesToOpponent[flank] = bits as u8);
    }
}
pub unsafe fn UpdateSentPokesToOpponentValue(battler: u8) {
    if GetBattlerSide(battler) == B_SIDE_OPPONENT {
        OpponentSwitchInResetSentPokesToOpponentValue(battler);
    } else {
        for i in 1..(gBattlersCount.get() as i32) {
            {
                let gl_v = gBitTable[crate::gl!(gBattlerPartyIndexes[battler])] as u8;
                let gl_i26 = (i & BIT_FLANK as i32) >> 1;
                crate::gl!(gSentPokesToOpponent[gl_i26] |= gl_v)
            };
        }
    }
}
pub unsafe fn BattleScriptPush(bsPtr: *mut u8) {
    (*(*crate::gl!(gBattleResources)).battleScriptsStack).ptr[{
        let t1 = (*(*crate::gl!(gBattleResources)).battleScriptsStack).size;
        (*(*crate::gl!(gBattleResources)).battleScriptsStack).size += 1;
        t1
    }] = bsPtr;
}
#[unsafe(no_mangle)]
pub unsafe fn BattleScriptPushCursor() {
    (*(*crate::gl!(gBattleResources)).battleScriptsStack).ptr[{
        let t1 = (*(*crate::gl!(gBattleResources)).battleScriptsStack).size;
        (*(*crate::gl!(gBattleResources)).battleScriptsStack).size += 1;
        t1
    }] = crate::gl!(gBattlescriptCurrInstr);
}
pub unsafe fn BattleScriptPop() {
    crate::gl!(
        gBattlescriptCurrInstr = (*(*crate::gl!(gBattleResources)).battleScriptsStack).ptr[{
            (*(*crate::gl!(gBattleResources)).battleScriptsStack).size -= 1;
            (*(*crate::gl!(gBattleResources)).battleScriptsStack).size
        }]
    );
}
pub unsafe fn TrySetCantSelectMoveBattleScript() -> u8 {
    let mut limitations: u8 = 0;
    let r#move: u16 = {
        let gl_i39 = gActiveBattler.get();
        let gl_i40 = {
            let gl_i27 = gActiveBattler.get();
            crate::gl!(gBattleBufferB[gl_i27][2])
        };
        crate::gl!(gBattleMons[gl_i39].moves[gl_i40])
    };
    let mut holdEffect: u8 = 0;
    let choicedMove: *mut u16 =
        &raw mut (*crate::gl!(gBattleStruct)).choicedMove[gActiveBattler.get()];
    if ({
        let gl_i41 = gActiveBattler.get();
        crate::gl!(gDisableStructs[gl_i41].disabledMove)
    }) == r#move
        && r#move != MOVE_NONE
    {
        crate::gl!(gBattleScripting.battler = gActiveBattler.get());
        gCurrentMove.set(r#move);
        if gBattleTypeFlags.get() & BATTLE_TYPE_PALACE != 0 {
            {
                let gl_v = (*crate::asmdata::BattleScript_SelectingDisabledMoveInPalace
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
                let gl_i28 = gActiveBattler.get();
                crate::gl!(gPalaceSelectionBattleScripts[gl_i28] = gl_v)
            };
            (*gProtectStructs.as_ptr())[gActiveBattler.get()]
                .set_palaceUnableToUseMove(TRUE as u32);
        } else {
            {
                let gl_v = (*crate::asmdata::BattleScript_SelectingDisabledMove
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
                let gl_i29 = gActiveBattler.get();
                crate::gl!(gSelectionBattleScripts[gl_i29] = gl_v)
            };
            limitations = 1;
        }
    }
    if r#move == {
        let gl_i30 = gActiveBattler.get();
        crate::gl!(gLastMoves[gl_i30])
    } && r#move != MOVE_STRUGGLE
        && ({
            let gl_i42 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i42].status2)
        }) & 0x80000000
            != 0
    {
        CancelMultiTurnMoves(gActiveBattler.get());
        if gBattleTypeFlags.get() & BATTLE_TYPE_PALACE != 0 {
            {
                let gl_v = (*crate::asmdata::BattleScript_SelectingTormentedMoveInPalace
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
                let gl_i31 = gActiveBattler.get();
                crate::gl!(gPalaceSelectionBattleScripts[gl_i31] = gl_v)
            };
            (*gProtectStructs.as_ptr())[gActiveBattler.get()]
                .set_palaceUnableToUseMove(TRUE as u32);
        } else {
            {
                let gl_v = (*crate::asmdata::BattleScript_SelectingTormentedMove
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
                let gl_i32 = gActiveBattler.get();
                crate::gl!(gSelectionBattleScripts[gl_i32] = gl_v)
            };
            limitations += 1;
        }
    }
    if (*gDisableStructs.as_ptr())[gActiveBattler.get()].tauntTimer() != 0
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .power
            == 0
    {
        gCurrentMove.set(r#move);
        if gBattleTypeFlags.get() & BATTLE_TYPE_PALACE != 0 {
            {
                let gl_v = (*crate::asmdata::BattleScript_SelectingNotAllowedMoveTauntInPalace
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
                let gl_i33 = gActiveBattler.get();
                crate::gl!(gPalaceSelectionBattleScripts[gl_i33] = gl_v)
            };
            (*gProtectStructs.as_ptr())[gActiveBattler.get()]
                .set_palaceUnableToUseMove(TRUE as u32);
        } else {
            {
                let gl_v = (*crate::asmdata::BattleScript_SelectingNotAllowedMoveTaunt
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
                let gl_i34 = gActiveBattler.get();
                crate::gl!(gSelectionBattleScripts[gl_i34] = gl_v)
            };
            limitations += 1;
        }
    }
    if GetImprisonedMovesCount(gActiveBattler.get(), r#move) != 0 {
        gCurrentMove.set(r#move);
        if gBattleTypeFlags.get() & BATTLE_TYPE_PALACE != 0 {
            {
                let gl_v = (*crate::asmdata::BattleScript_SelectingImprisonedMoveInPalace
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
                let gl_i35 = gActiveBattler.get();
                crate::gl!(gPalaceSelectionBattleScripts[gl_i35] = gl_v)
            };
            (*gProtectStructs.as_ptr())[gActiveBattler.get()]
                .set_palaceUnableToUseMove(TRUE as u32);
        } else {
            {
                let gl_v = (*crate::asmdata::BattleScript_SelectingImprisonedMove
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
                let gl_i36 = gActiveBattler.get();
                crate::gl!(gSelectionBattleScripts[gl_i36] = gl_v)
            };
            limitations += 1;
        }
    }
    if ({
        let gl_i43 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i43].item)
    }) == ITEM_ENIGMA_BERRY
    {
        holdEffect = {
            let gl_i37 = gActiveBattler.get();
            crate::gl!(gEnigmaBerries[gl_i37].holdEffect)
        };
    } else {
        holdEffect = GetItemHoldEffect({
            let gl_i44 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i44].item)
        });
    }
    gPotentialItemEffectBattler.set(gActiveBattler.get());
    if holdEffect == HOLD_EFFECT_CHOICE_BAND
        && *choicedMove != MOVE_NONE
        && *choicedMove != MOVE_UNAVAILABLE
        && *choicedMove != r#move
    {
        gCurrentMove.set(*choicedMove);
        gLastUsedItem.set({
            let gl_i45 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i45].item)
        });
        if gBattleTypeFlags.get() & BATTLE_TYPE_PALACE != 0 {
            (*gProtectStructs.as_ptr())[gActiveBattler.get()]
                .set_palaceUnableToUseMove(TRUE as u32);
        } else {
            {
                let gl_v = (*crate::asmdata::BattleScript_SelectingNotAllowedMoveChoiceItem
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
                let gl_i38 = gActiveBattler.get();
                crate::gl!(gSelectionBattleScripts[gl_i38] = gl_v)
            };
            limitations += 1;
        }
    }
    if ({
        let gl_i46 = gActiveBattler.get();
        let gl_i47 = {
            let gl_i39 = gActiveBattler.get();
            crate::gl!(gBattleBufferB[gl_i39][2])
        };
        crate::gl!(gBattleMons[gl_i46].pp[gl_i47])
    }) == 0
    {
        if gBattleTypeFlags.get() & BATTLE_TYPE_PALACE != 0 {
            (*gProtectStructs.as_ptr())[gActiveBattler.get()]
                .set_palaceUnableToUseMove(TRUE as u32);
        } else {
            {
                let gl_v = (*crate::asmdata::BattleScript_SelectingMoveWithNoPP
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
                let gl_i40 = gActiveBattler.get();
                crate::gl!(gSelectionBattleScripts[gl_i40] = gl_v)
            };
            limitations += 1;
        }
    }
    limitations
}
pub unsafe fn CheckMoveLimitations(battler: u8, mut unusableMoves: u8, check: u8) -> u8 {
    let mut holdEffect: u8 = 0;
    let choicedMove: *mut u16 = &raw mut (*crate::gl!(gBattleStruct)).choicedMove[battler];
    if crate::gl!(gBattleMons[battler].item) == ITEM_ENIGMA_BERRY {
        holdEffect = crate::gl!(gEnigmaBerries[battler].holdEffect);
    } else {
        holdEffect = GetItemHoldEffect(crate::gl!(gBattleMons[battler].item));
    }
    gPotentialItemEffectBattler.set(battler);
    for i in 0..MAX_MON_MOVES {
        if crate::gl!(gBattleMons[battler].moves[i]) == MOVE_NONE
            && check as i32 & MOVE_LIMITATION_ZEROMOVE != 0
        {
            unusableMoves |= gBitTable[i] as u8;
        }
        if crate::gl!(gBattleMons[battler].pp[i]) == 0 && check as i32 & MOVE_LIMITATION_PP != 0 {
            unusableMoves |= gBitTable[i] as u8;
        }
        if crate::gl!(gBattleMons[battler].moves[i])
            == crate::gl!(gDisableStructs[battler].disabledMove)
            && check as i32 & MOVE_LIMITATION_DISABLED != 0
        {
            unusableMoves |= gBitTable[i] as u8;
        }
        if crate::gl!(gBattleMons[battler].moves[i]) == crate::gl!(gLastMoves[battler])
            && check as i32 & MOVE_LIMITATION_TORMENTED != 0
            && crate::gl!(gBattleMons[battler].status2) & 0x80000000 != 0
        {
            unusableMoves |= gBitTable[i] as u8;
        }
        if (*gDisableStructs.as_ptr())[battler].tauntTimer() != 0
            && check as i32 & MOVE_LIMITATION_TAUNT != 0
            && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [crate::gl!(gBattleMons[battler].moves[i])]
            .power
                == 0
        {
            unusableMoves |= gBitTable[i] as u8;
        }
        if GetImprisonedMovesCount(battler, crate::gl!(gBattleMons[battler].moves[i])) != 0
            && check as i32 & MOVE_LIMITATION_IMPRISON != 0
        {
            unusableMoves |= gBitTable[i] as u8;
        }
        if (*gDisableStructs.as_ptr())[battler].encoreTimer() != 0
            && crate::gl!(gDisableStructs[battler].encoredMove)
                != crate::gl!(gBattleMons[battler].moves[i])
        {
            unusableMoves |= gBitTable[i] as u8;
        }
        if holdEffect == HOLD_EFFECT_CHOICE_BAND
            && *choicedMove != MOVE_NONE
            && *choicedMove != MOVE_UNAVAILABLE
            && *choicedMove != crate::gl!(gBattleMons[battler].moves[i])
        {
            unusableMoves |= gBitTable[i] as u8;
        }
    }
    unusableMoves
}
pub unsafe fn AreAllMovesUnusable() -> u8 {
    let unusable: u8 = CheckMoveLimitations(gActiveBattler.get(), 0, MOVE_LIMITATIONS_ALL);
    if unusable == ALL_MOVES_MASK {
        (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_noValidMoves(TRUE as u32);
        {
            let gl_v = (*crate::asmdata::BattleScript_NoMovesLeft.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
            let gl_i41 = gActiveBattler.get();
            crate::gl!(gSelectionBattleScripts[gl_i41] = gl_v)
        };
    } else {
        (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_noValidMoves(FALSE as u32);
    }
    (unusable == ALL_MOVES_MASK) as u8
}
pub unsafe fn GetImprisonedMovesCount(battler: u8, r#move: u16) -> u8 {
    let mut imprisonedMoves: u8 = 0;
    let battlerSide: u8 = GetBattlerSide(battler);
    let mut i: i32 = 0;
    while i < gBattlersCount.get() as i32 {
        if battlerSide != GetBattlerSide(i as u8)
            && crate::gl!(gStatuses3[i]) & STATUS3_IMPRISONED_OTHERS != 0
        {
            let mut j: i32 = 0;
            while j < MAX_MON_MOVES {
                if r#move == crate::gl!(gBattleMons[i].moves[j]) {
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
    gBattlerAttacker.set(0);
    while gBattlerAttacker.get() < gBattlersCount.get()
        && gAbsentBattlerFlags.get() as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                [gBattlerAttacker.get()]
            != 0
    {
        gBattlerAttacker.set(gBattlerAttacker.get() + (1));
    }
    gBattlerTarget.set(0);
    while gBattlerTarget.get() < gBattlersCount.get()
        && gAbsentBattlerFlags.get() as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gBattlerTarget.get()]
            != 0
    {
        gBattlerTarget.set(gBattlerTarget.get() + (1));
    }
    loop {
        let mut side: u8 = 0;
        'l4: {
            let sw1: u8 = (*crate::gl!(gBattleStruct)).turnCountersTracker;
            let mut fall = false;
            if sw1 == ENDTURN_ORDER {
                fall = true;
                for i in 0..(gBattlersCount.get() as i32) {
                    crate::gl!(gBattlerByTurnOrder[i] = i as u8);
                }
                i = 0;
                while i < gBattlersCount.get() as i32 - 1 {
                    let mut j: i32 = i + 1;
                    while j < gBattlersCount.get() as i32 {
                        if GetWhoStrikesFirst(
                            crate::gl!(gBattlerByTurnOrder[i]),
                            crate::gl!(gBattlerByTurnOrder[j]),
                            FALSE,
                        ) != 0
                        {
                            SwapTurnOrder(i as u8, j as u8);
                        }
                        j += 1;
                    }
                    i += 1;
                }
                (*crate::gl!(gBattleStruct)).turnCountersTracker += 1;
                (*crate::gl!(gBattleStruct)).turnSideTracker = 0;
            }
            if fall || sw1 == ENDTURN_REFLECT {
                while (*crate::gl!(gBattleStruct)).turnSideTracker < 2 {
                    side = (*crate::gl!(gBattleStruct)).turnSideTracker;
                    gActiveBattler.set({
                        gBattlerAttacker.set(crate::gl!(gSideTimers[side].reflectBattlerId));
                        gBattlerAttacker.get()
                    });
                    if crate::gl!(gSideStatuses[side]) as i32 & SIDE_STATUS_REFLECT != 0
                        && ({
                            crate::gl!(gSideTimers[side].reflectTimer -= 1);
                            crate::gl!(gSideTimers[side].reflectTimer)
                        }) == 0
                    {
                        crate::gl!(gSideStatuses[side] &= 65534);
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_SideStatusWoreOff
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        );
                        crate::gl!(gBattleTextBuff1[0] = 0xFD);
                        crate::gl!(gBattleTextBuff1[1] = 2);
                        crate::gl!(gBattleTextBuff1[2] = MOVE_REFLECT as u8);
                        crate::gl!(gBattleTextBuff1[3] = 0);
                        crate::gl!(gBattleTextBuff1[4] = 0xFF);
                        effect += 1;
                    }
                    (*crate::gl!(gBattleStruct)).turnSideTracker += 1;
                    if effect != 0 {
                        break;
                    }
                }
                if effect == 0 {
                    (*crate::gl!(gBattleStruct)).turnCountersTracker += 1;
                    (*crate::gl!(gBattleStruct)).turnSideTracker = 0;
                }
                break 'l4;
            }
            if sw1 == ENDTURN_LIGHT_SCREEN {
                while (*crate::gl!(gBattleStruct)).turnSideTracker < 2 {
                    side = (*crate::gl!(gBattleStruct)).turnSideTracker;
                    gActiveBattler.set({
                        gBattlerAttacker.set(crate::gl!(gSideTimers[side].lightscreenBattlerId));
                        gBattlerAttacker.get()
                    });
                    if crate::gl!(gSideStatuses[side]) as i32 & SIDE_STATUS_LIGHTSCREEN != 0
                        && ({
                            crate::gl!(gSideTimers[side].lightscreenTimer -= 1);
                            crate::gl!(gSideTimers[side].lightscreenTimer)
                        }) == 0
                    {
                        crate::gl!(gSideStatuses[side] &= 65533);
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_SideStatusWoreOff
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        );
                        crate::gl!(gBattleCommunication[5] = side);
                        crate::gl!(gBattleTextBuff1[0] = 0xFD);
                        crate::gl!(gBattleTextBuff1[1] = 2);
                        crate::gl!(gBattleTextBuff1[2] = MOVE_LIGHT_SCREEN as u8);
                        crate::gl!(gBattleTextBuff1[3] = 0);
                        crate::gl!(gBattleTextBuff1[4] = 0xFF);
                        effect += 1;
                    }
                    (*crate::gl!(gBattleStruct)).turnSideTracker += 1;
                    if effect != 0 {
                        break;
                    }
                }
                if effect == 0 {
                    (*crate::gl!(gBattleStruct)).turnCountersTracker += 1;
                    (*crate::gl!(gBattleStruct)).turnSideTracker = 0;
                }
                break 'l4;
            }
            if sw1 == ENDTURN_MIST {
                while (*crate::gl!(gBattleStruct)).turnSideTracker < 2 {
                    side = (*crate::gl!(gBattleStruct)).turnSideTracker;
                    gActiveBattler.set({
                        gBattlerAttacker.set(crate::gl!(gSideTimers[side].mistBattlerId));
                        gBattlerAttacker.get()
                    });
                    if crate::gl!(gSideTimers[side].mistTimer) != 0
                        && ({
                            crate::gl!(gSideTimers[side].mistTimer -= 1);
                            crate::gl!(gSideTimers[side].mistTimer)
                        }) == 0
                    {
                        crate::gl!(gSideStatuses[side] &= 65279);
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_SideStatusWoreOff
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        );
                        crate::gl!(gBattleCommunication[5] = side);
                        crate::gl!(gBattleTextBuff1[0] = 0xFD);
                        crate::gl!(gBattleTextBuff1[1] = 2);
                        crate::gl!(gBattleTextBuff1[2] = MOVE_MIST as u8);
                        crate::gl!(gBattleTextBuff1[3] = 0);
                        crate::gl!(gBattleTextBuff1[4] = 0xFF);
                        effect += 1;
                    }
                    (*crate::gl!(gBattleStruct)).turnSideTracker += 1;
                    if effect != 0 {
                        break;
                    }
                }
                if effect == 0 {
                    (*crate::gl!(gBattleStruct)).turnCountersTracker += 1;
                    (*crate::gl!(gBattleStruct)).turnSideTracker = 0;
                }
                break 'l4;
            }
            if sw1 == ENDTURN_SAFEGUARD {
                while (*crate::gl!(gBattleStruct)).turnSideTracker < 2 {
                    side = (*crate::gl!(gBattleStruct)).turnSideTracker;
                    gActiveBattler.set({
                        gBattlerAttacker.set(crate::gl!(gSideTimers[side].safeguardBattlerId));
                        gBattlerAttacker.get()
                    });
                    if crate::gl!(gSideStatuses[side]) as i32 & SIDE_STATUS_SAFEGUARD != 0
                        && ({
                            crate::gl!(gSideTimers[side].safeguardTimer -= 1);
                            crate::gl!(gSideTimers[side].safeguardTimer)
                        }) == 0
                    {
                        crate::gl!(gSideStatuses[side] &= 65503);
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_SafeguardEnds.cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                        );
                        effect += 1;
                    }
                    (*crate::gl!(gBattleStruct)).turnSideTracker += 1;
                    if effect != 0 {
                        break;
                    }
                }
                if effect == 0 {
                    (*crate::gl!(gBattleStruct)).turnCountersTracker += 1;
                    (*crate::gl!(gBattleStruct)).turnSideTracker = 0;
                }
                break 'l4;
            }
            if sw1 == ENDTURN_WISH {
                while (*crate::gl!(gBattleStruct)).turnSideTracker < gBattlersCount.get() {
                    gActiveBattler.set({
                        let gl_i42 = (*crate::gl!(gBattleStruct)).turnSideTracker;
                        crate::gl!(gBattlerByTurnOrder[gl_i42])
                    });
                    if {
                        let gl_i43 = gActiveBattler.get();
                        crate::gl!(gWishFutureKnock.wishCounter[gl_i43])
                    } != 0
                        && ({
                            {
                                let gl_v = 1;
                                let gl_i44 = gActiveBattler.get();
                                crate::gl!(gWishFutureKnock.wishCounter[gl_i44] -= gl_v)
                            };
                            {
                                let gl_i45 = gActiveBattler.get();
                                crate::gl!(gWishFutureKnock.wishCounter[gl_i45])
                            }
                        }) == 0
                        && ({
                            let gl_i48 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i48].hp)
                        }) != 0
                    {
                        gBattlerTarget.set(gActiveBattler.get());
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_WishComesTrue.cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                        );
                        effect += 1;
                    }
                    (*crate::gl!(gBattleStruct)).turnSideTracker += 1;
                    if effect != 0 {
                        break;
                    }
                }
                if effect == 0 {
                    (*crate::gl!(gBattleStruct)).turnCountersTracker += 1;
                }
                break 'l4;
            }
            if sw1 == ENDTURN_RAIN {
                if gBattleWeather.get() as i32 & B_WEATHER_RAIN != 0 {
                    if gBattleWeather.get() as i32 & B_WEATHER_RAIN_PERMANENT == 0 {
                        if ({
                            crate::gl!(gWishFutureKnock.weatherDuration -= 1);
                            crate::gl!(gWishFutureKnock.weatherDuration)
                        }) == 0
                        {
                            gBattleWeather.set(gBattleWeather.get() & (65534));
                            gBattleWeather.set(gBattleWeather.get() & (65533));
                            crate::gl!(gBattleCommunication[5] = B_MSG_RAIN_STOPPED);
                        } else if gBattleWeather.get() as i32 & B_WEATHER_RAIN_DOWNPOUR != 0 {
                            crate::gl!(gBattleCommunication[5] = B_MSG_DOWNPOUR_CONTINUES);
                        } else {
                            crate::gl!(gBattleCommunication[5] = B_MSG_RAIN_CONTINUES);
                        }
                    } else if gBattleWeather.get() as i32 & B_WEATHER_RAIN_DOWNPOUR != 0 {
                        crate::gl!(gBattleCommunication[5] = B_MSG_DOWNPOUR_CONTINUES);
                    } else {
                        crate::gl!(gBattleCommunication[5] = B_MSG_RAIN_CONTINUES);
                    }
                    BattleScriptExecute(
                        (*crate::asmdata::BattleScript_RainContinuesOrEnds.cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                    );
                    effect += 1;
                }
                (*crate::gl!(gBattleStruct)).turnCountersTracker += 1;
                break 'l4;
            }
            if sw1 == ENDTURN_SANDSTORM {
                if gBattleWeather.get() as i32 & B_WEATHER_SANDSTORM != 0 {
                    if gBattleWeather.get() as i32 & B_WEATHER_SANDSTORM_PERMANENT == 0
                        && ({
                            crate::gl!(gWishFutureKnock.weatherDuration -= 1);
                            crate::gl!(gWishFutureKnock.weatherDuration)
                        }) == 0
                    {
                        gBattleWeather.set(gBattleWeather.get() & (65527));
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_SandStormHailEnds
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                    } else {
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_DamagingWeatherContinues
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                    }
                    crate::gl!(gBattleScripting.animArg1 = B_ANIM_SANDSTORM_CONTINUES);
                    crate::gl!(gBattleCommunication[5] = B_MSG_SANDSTORM);
                    BattleScriptExecute(crate::gl!(gBattlescriptCurrInstr));
                    effect += 1;
                }
                (*crate::gl!(gBattleStruct)).turnCountersTracker += 1;
                break 'l4;
            }
            if sw1 == ENDTURN_SUN {
                if gBattleWeather.get() as i32 & B_WEATHER_SUN != 0 {
                    if gBattleWeather.get() as i32 & B_WEATHER_SUN_PERMANENT == 0
                        && ({
                            crate::gl!(gWishFutureKnock.weatherDuration -= 1);
                            crate::gl!(gWishFutureKnock.weatherDuration)
                        }) == 0
                    {
                        gBattleWeather.set(gBattleWeather.get() & (65503));
                        crate::gl!(
                            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SunlightFaded
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut()
                        );
                    } else {
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_SunlightContinues
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                    }
                    BattleScriptExecute(crate::gl!(gBattlescriptCurrInstr));
                    effect += 1;
                }
                (*crate::gl!(gBattleStruct)).turnCountersTracker += 1;
                break 'l4;
            }
            if sw1 == ENDTURN_HAIL {
                if gBattleWeather.get() as i32 & B_WEATHER_HAIL != 0 {
                    if ({
                        crate::gl!(gWishFutureKnock.weatherDuration -= 1);
                        crate::gl!(gWishFutureKnock.weatherDuration)
                    }) == 0
                    {
                        gBattleWeather.set(gBattleWeather.get() & (65407));
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_SandStormHailEnds
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                    } else {
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_DamagingWeatherContinues
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                    }
                    crate::gl!(gBattleScripting.animArg1 = B_ANIM_HAIL_CONTINUES);
                    crate::gl!(gBattleCommunication[5] = B_MSG_HAIL);
                    BattleScriptExecute(crate::gl!(gBattlescriptCurrInstr));
                    effect += 1;
                }
                (*crate::gl!(gBattleStruct)).turnCountersTracker += 1;
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
    (crate::gl!(gBattleMainFunc) != Some(BattleTurnPassed as unsafe fn())) as u8
}
pub unsafe fn DoBattlerEndTurnEffects() -> u8 {
    let mut effect: u8 = 0;
    gHitMarker.set(gHitMarker.get() | (0x1000020));
    while (*crate::gl!(gBattleStruct)).turnEffectsBattlerId < gBattlersCount.get()
        && (*crate::gl!(gBattleStruct)).turnEffectsTracker <= ENDTURN_BATTLER_COUNT
    {
        gActiveBattler.set({
            gBattlerAttacker.set({
                let gl_i46 = (*crate::gl!(gBattleStruct)).turnEffectsBattlerId;
                crate::gl!(gBattlerByTurnOrder[gl_i46])
            });
            gBattlerAttacker.get()
        });
        if gAbsentBattlerFlags.get() as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler.get()]
            != 0
        {
            (*crate::gl!(gBattleStruct)).turnEffectsBattlerId += 1;
        } else {
            'l2: {
                match (*crate::gl!(gBattleStruct)).turnEffectsTracker {
                    ENDTURN_INGRAIN => {
                        if {
                            let gl_i47 = gActiveBattler.get();
                            crate::gl!(gStatuses3[gl_i47])
                        } & STATUS3_ROOTED
                            != 0
                            && ({
                                let gl_i49 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i49].hp)
                            }) != ({
                                let gl_i50 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i50].maxHP)
                            })
                            && ({
                                let gl_i51 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i51].hp)
                            }) != 0
                        {
                            gBattleMoveDamage.set(
                                ({
                                    let gl_i52 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i52].maxHP)
                                }) as i32
                                    / 16,
                            );
                            if gBattleMoveDamage.get() == 0 {
                                gBattleMoveDamage.set(1);
                            }
                            gBattleMoveDamage.set(-gBattleMoveDamage.get());
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_IngrainTurnHeal
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect += 1;
                        }
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_ABILITIES => {
                        if AbilityBattleEffects(
                            ABILITYEFFECT_ENDTURN,
                            gActiveBattler.get(),
                            0,
                            0,
                            0,
                        ) != 0
                        {
                            effect += 1;
                        }
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_ITEMS1 => {
                        if ItemBattleEffects(ITEMEFFECT_NORMAL, gActiveBattler.get(), FALSE) != 0 {
                            effect += 1;
                        }
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_ITEMS2 => {
                        if ItemBattleEffects(1, gActiveBattler.get(), 1) != 0 {
                            effect += 1;
                        }
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_LEECH_SEED => {
                        if {
                            let gl_i48 = gActiveBattler.get();
                            crate::gl!(gStatuses3[gl_i48])
                        } & STATUS3_LEECHSEED
                            != 0
                            && ({
                                let gl_i53 = {
                                    let gl_i49 = gActiveBattler.get();
                                    crate::gl!(gStatuses3[gl_i49])
                                } & STATUS3_LEECHSEED_BATTLER;
                                crate::gl!(gBattleMons[gl_i53].hp)
                            }) != 0
                            && ({
                                let gl_i54 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i54].hp)
                            }) != 0
                        {
                            gBattlerTarget.set(
                                {
                                    let gl_i50 = gActiveBattler.get();
                                    crate::gl!(gStatuses3[gl_i50])
                                } as u8
                                    & STATUS3_LEECHSEED_BATTLER as u8,
                            );
                            gBattleMoveDamage.set(
                                ({
                                    let gl_i55 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i55].maxHP)
                                }) as i32
                                    / 8,
                            );
                            if gBattleMoveDamage.get() == 0 {
                                gBattleMoveDamage.set(1);
                            }
                            crate::gl!(gBattleScripting.animArg1 = gBattlerTarget.get());
                            crate::gl!(gBattleScripting.animArg2 = gBattlerAttacker.get());
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_LeechSeedTurnDrain
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect += 1;
                        }
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_POISON => {
                        if ({
                            let gl_i56 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i56].status1)
                        }) & STATUS1_POISON
                            != 0
                            && ({
                                let gl_i57 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i57].hp)
                            }) != 0
                        {
                            gBattleMoveDamage.set(
                                ({
                                    let gl_i58 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i58].maxHP)
                                }) as i32
                                    / 8,
                            );
                            if gBattleMoveDamage.get() == 0 {
                                gBattleMoveDamage.set(1);
                            }
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_PoisonTurnDmg
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect += 1;
                        }
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_BAD_POISON => {
                        if ({
                            let gl_i59 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i59].status1)
                        }) & STATUS1_TOXIC_POISON
                            != 0
                            && ({
                                let gl_i60 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i60].hp)
                            }) != 0
                        {
                            gBattleMoveDamage.set(
                                ({
                                    let gl_i61 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i61].maxHP)
                                }) as i32
                                    / 16,
                            );
                            if gBattleMoveDamage.get() == 0 {
                                gBattleMoveDamage.set(1);
                            }
                            if ({
                                let gl_i62 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i62].status1)
                            }) & STATUS1_TOXIC_COUNTER
                                != STATUS1_TOXIC_COUNTER
                            {
                                {
                                    let gl_i63 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i63].status1 += 256)
                                };
                            }
                            gBattleMoveDamage.set(
                                gBattleMoveDamage.get()
                                    * (((({
                                        let gl_i64 = gActiveBattler.get();
                                        crate::gl!(gBattleMons[gl_i64].status1)
                                    }) & STATUS1_TOXIC_COUNTER)
                                        >> 8) as i32),
                            );
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_PoisonTurnDmg
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect += 1;
                        }
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_BURN => {
                        if ({
                            let gl_i65 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i65].status1)
                        }) & STATUS1_BURN
                            != 0
                            && ({
                                let gl_i66 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i66].hp)
                            }) != 0
                        {
                            gBattleMoveDamage.set(
                                ({
                                    let gl_i67 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i67].maxHP)
                                }) as i32
                                    / 8,
                            );
                            if gBattleMoveDamage.get() == 0 {
                                gBattleMoveDamage.set(1);
                            }
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_BurnTurnDmg.cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                            );
                            effect += 1;
                        }
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_NIGHTMARES => {
                        if ({
                            let gl_i68 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i68].status2)
                        }) & STATUS2_NIGHTMARE
                            != 0
                            && ({
                                let gl_i69 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i69].hp)
                            }) != 0
                        {
                            if ({
                                let gl_i70 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i70].status1)
                            }) & STATUS1_SLEEP
                                != 0
                            {
                                gBattleMoveDamage.set(
                                    ({
                                        let gl_i71 = gActiveBattler.get();
                                        crate::gl!(gBattleMons[gl_i71].maxHP)
                                    }) as i32
                                        / 4,
                                );
                                if gBattleMoveDamage.get() == 0 {
                                    gBattleMoveDamage.set(1);
                                }
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_NightmareTurnDmg
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                effect += 1;
                            } else {
                                {
                                    let gl_i72 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i72].status2 &= 0xf7ffffff)
                                };
                            }
                        }
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_CURSE => {
                        if ({
                            let gl_i73 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i73].status2)
                        }) & STATUS2_CURSED
                            != 0
                            && ({
                                let gl_i74 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i74].hp)
                            }) != 0
                        {
                            gBattleMoveDamage.set(
                                ({
                                    let gl_i75 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i75].maxHP)
                                }) as i32
                                    / 4,
                            );
                            if gBattleMoveDamage.get() == 0 {
                                gBattleMoveDamage.set(1);
                            }
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_CurseTurnDmg
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            effect += 1;
                        }
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_WRAP => {
                        if ({
                            let gl_i76 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i76].status2)
                        }) & STATUS2_WRAPPED
                            != 0
                            && ({
                                let gl_i77 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i77].hp)
                            }) != 0
                        {
                            {
                                let gl_i78 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i78].status2 -= 8192)
                            };
                            if ({
                                let gl_i79 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i79].status2)
                            }) & STATUS2_WRAPPED
                                != 0
                            {
                                crate::gl!(
                                    gBattleScripting.animArg1 = *(*crate::gl!(gBattleStruct))
                                        .wrappedMove
                                        .as_mut_ptr()
                                        .at(gActiveBattler.get() as i32 * 2)
                                );
                                crate::gl!(
                                    gBattleScripting.animArg2 = *(*crate::gl!(gBattleStruct))
                                        .wrappedMove
                                        .as_mut_ptr()
                                        .at(gActiveBattler.get() as i32 * 2)
                                        .at(1)
                                );
                                crate::gl!(gBattleTextBuff1[0] = B_BUFF_PLACEHOLDER_BEGIN);
                                crate::gl!(gBattleTextBuff1[1] = B_BUFF_MOVE);
                                crate::gl!(
                                    gBattleTextBuff1[2] = *(*crate::gl!(gBattleStruct))
                                        .wrappedMove
                                        .as_mut_ptr()
                                        .at(gActiveBattler.get() as i32 * 2)
                                );
                                crate::gl!(
                                    gBattleTextBuff1[3] = *(*crate::gl!(gBattleStruct))
                                        .wrappedMove
                                        .as_mut_ptr()
                                        .at(gActiveBattler.get() as i32 * 2)
                                        .at(1)
                                );
                                crate::gl!(gBattleTextBuff1[4] = EOS);
                                crate::gl!(
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_WrapTurnDmg
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut()
                                );
                                gBattleMoveDamage.set(
                                    ({
                                        let gl_i80 = gActiveBattler.get();
                                        crate::gl!(gBattleMons[gl_i80].maxHP)
                                    }) as i32
                                        / 16,
                                );
                                if gBattleMoveDamage.get() == 0 {
                                    gBattleMoveDamage.set(1);
                                }
                            } else {
                                crate::gl!(gBattleTextBuff1[0] = B_BUFF_PLACEHOLDER_BEGIN);
                                crate::gl!(gBattleTextBuff1[1] = B_BUFF_MOVE);
                                crate::gl!(
                                    gBattleTextBuff1[2] = *(*crate::gl!(gBattleStruct))
                                        .wrappedMove
                                        .as_mut_ptr()
                                        .at(gActiveBattler.get() as i32 * 2)
                                );
                                crate::gl!(
                                    gBattleTextBuff1[3] = *(*crate::gl!(gBattleStruct))
                                        .wrappedMove
                                        .as_mut_ptr()
                                        .at(gActiveBattler.get() as i32 * 2)
                                        .at(1)
                                );
                                crate::gl!(gBattleTextBuff1[4] = EOS);
                                crate::gl!(
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_WrapEnds
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut()
                                );
                            }
                            BattleScriptExecute(crate::gl!(gBattlescriptCurrInstr));
                            effect += 1;
                        }
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_UPROAR => {
                        if ({
                            let gl_i81 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i81].status2)
                        }) & STATUS2_UPROAR
                            != 0
                        {
                            gBattlerAttacker.set(0);
                            while gBattlerAttacker.get() < gBattlersCount.get() {
                                if ({
                                    let gl_i82 = gBattlerAttacker.get();
                                    crate::gl!(gBattleMons[gl_i82].status1)
                                }) & STATUS1_SLEEP
                                    != 0
                                    && ({
                                        let gl_i83 = gBattlerAttacker.get();
                                        crate::gl!(gBattleMons[gl_i83].ability)
                                    }) != ABILITY_SOUNDPROOF
                                {
                                    {
                                        let gl_i84 = gBattlerAttacker.get();
                                        crate::gl!(gBattleMons[gl_i84].status1 &= 0xfffffff8)
                                    };
                                    {
                                        let gl_i85 = gBattlerAttacker.get();
                                        crate::gl!(gBattleMons[gl_i85].status2 &= 0xf7ffffff)
                                    };
                                    crate::gl!(gBattleCommunication[5] = 1);
                                    BattleScriptExecute(
                                        (*crate::asmdata::BattleScript_MonWokeUpInUproar
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut(),
                                    );
                                    gActiveBattler.set(gBattlerAttacker.get());
                                    BtlController_EmitSetMonData(
                                        B_COMM_TO_CONTROLLER,
                                        REQUEST_STATUS_BATTLE,
                                        0,
                                        4,
                                        ({
                                            let gl_i86 = gActiveBattler.get();
                                            crate::gl!(&raw gBattleMons[gl_i86].status1)
                                        }) as *mut c_void,
                                    );
                                    MarkBattlerForControllerExec(gActiveBattler.get());
                                    break;
                                }
                                gBattlerAttacker.set(gBattlerAttacker.get() + (1));
                            }
                            if gBattlerAttacker.get() != gBattlersCount.get() {
                                effect = 2;
                                break 'l2;
                            } else {
                                gBattlerAttacker.set(gActiveBattler.get());
                                {
                                    let gl_i87 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i87].status2 -= 16)
                                };
                                if WasUnableToUseMove(gActiveBattler.get()) != 0 {
                                    CancelMultiTurnMoves(gActiveBattler.get());
                                    crate::gl!(gBattleCommunication[5] = B_MSG_UPROAR_ENDS);
                                } else if ({
                                    let gl_i88 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i88].status2)
                                }) & STATUS2_UPROAR
                                    != 0
                                {
                                    crate::gl!(gBattleCommunication[5] = B_MSG_UPROAR_CONTINUES);
                                    {
                                        let gl_i89 = gActiveBattler.get();
                                        crate::gl!(
                                            gBattleMons[gl_i89].status2 |= STATUS2_MULTIPLETURNS
                                        )
                                    };
                                } else {
                                    crate::gl!(gBattleCommunication[5] = B_MSG_UPROAR_ENDS);
                                    CancelMultiTurnMoves(gActiveBattler.get());
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
                            (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                        }
                    }
                    ENDTURN_THRASH => {
                        if ({
                            let gl_i90 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i90].status2)
                        }) & STATUS2_LOCK_CONFUSE
                            != 0
                        {
                            {
                                let gl_i91 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i91].status2 -= 1024)
                            };
                            if WasUnableToUseMove(gActiveBattler.get()) != 0 {
                                CancelMultiTurnMoves(gActiveBattler.get());
                            } else if ({
                                let gl_i92 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i92].status2)
                            }) & STATUS2_LOCK_CONFUSE
                                == 0
                                && ({
                                    let gl_i93 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i93].status2)
                                }) & STATUS2_MULTIPLETURNS
                                    != 0
                            {
                                {
                                    let gl_i94 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i94].status2 &= 0xffffefff)
                                };
                                if ({
                                    let gl_i95 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i95].status2)
                                }) & STATUS2_CONFUSION
                                    == 0
                                {
                                    crate::gl!(gBattleCommunication[3] = 71);
                                    SetMoveEffect(TRUE, 0);
                                    if ({
                                        let gl_i96 = gActiveBattler.get();
                                        crate::gl!(gBattleMons[gl_i96].status2)
                                    }) & STATUS2_CONFUSION
                                        != 0
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
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_DISABLE => {
                        if (*gDisableStructs.as_ptr())[gActiveBattler.get()].disableTimer() != 0 {
                            let mut i: i32 = 0;
                            while i < MAX_MON_MOVES {
                                if ({
                                    let gl_i97 = gActiveBattler.get();
                                    crate::gl!(gDisableStructs[gl_i97].disabledMove)
                                }) == ({
                                    let gl_i98 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i98].moves[i])
                                }) {
                                    break;
                                }
                                i += 1;
                            }
                            if i == MAX_MON_MOVES {
                                {
                                    let gl_i99 = gActiveBattler.get();
                                    crate::gl!(gDisableStructs[gl_i99].disabledMove = MOVE_NONE)
                                };
                                (*gDisableStructs.as_ptr())[gActiveBattler.get()]
                                    .set_disableTimer(0);
                            } else if ({
                                (*gDisableStructs.as_ptr())[gActiveBattler.get()].set_disableTimer(
                                    (*gDisableStructs.as_ptr())[gActiveBattler.get()]
                                        .disableTimer()
                                        - 1,
                                );
                                (*gDisableStructs.as_ptr())[gActiveBattler.get()].disableTimer()
                            }) == 0
                            {
                                {
                                    let gl_i100 = gActiveBattler.get();
                                    crate::gl!(gDisableStructs[gl_i100].disabledMove = MOVE_NONE)
                                };
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_DisabledNoMore
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                effect += 1;
                            }
                        }
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_ENCORE => {
                        if (*gDisableStructs.as_ptr())[gActiveBattler.get()].encoreTimer() != 0 {
                            if (*gBattleMons.as_ptr())[gActiveBattler.get()].moves[{
                                let gl_i102 = gActiveBattler.get();
                                crate::gl!(gDisableStructs[gl_i102].encoredMovePos)
                            }] != ({
                                let gl_i103 = gActiveBattler.get();
                                crate::gl!(gDisableStructs[gl_i103].encoredMove)
                            }) {
                                {
                                    let gl_i104 = gActiveBattler.get();
                                    crate::gl!(gDisableStructs[gl_i104].encoredMove = MOVE_NONE)
                                };
                                (*gDisableStructs.as_ptr())[gActiveBattler.get()]
                                    .set_encoreTimer(0);
                            } else if ({
                                (*gDisableStructs.as_ptr())[gActiveBattler.get()].set_encoreTimer(
                                    (*gDisableStructs.as_ptr())[gActiveBattler.get()].encoreTimer()
                                        - 1,
                                );
                                (*gDisableStructs.as_ptr())[gActiveBattler.get()].encoreTimer()
                            }) == 0
                                || (*gBattleMons.as_ptr())[gActiveBattler.get()].pp[{
                                    let gl_i106 = gActiveBattler.get();
                                    crate::gl!(gDisableStructs[gl_i106].encoredMovePos)
                                }] == 0
                            {
                                {
                                    let gl_i107 = gActiveBattler.get();
                                    crate::gl!(gDisableStructs[gl_i107].encoredMove = MOVE_NONE)
                                };
                                (*gDisableStructs.as_ptr())[gActiveBattler.get()]
                                    .set_encoreTimer(0);
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_EncoredNoMore
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                effect += 1;
                            }
                        }
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_LOCK_ON => {
                        if {
                            let gl_i51 = gActiveBattler.get();
                            crate::gl!(gStatuses3[gl_i51])
                        } & STATUS3_ALWAYS_HITS
                            != 0
                        {
                            {
                                let gl_v = 8;
                                let gl_i52 = gActiveBattler.get();
                                crate::gl!(gStatuses3[gl_i52] -= gl_v)
                            };
                        }
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_CHARGE => {
                        if (*gDisableStructs.as_ptr())[gActiveBattler.get()].chargeTimer() != 0
                            && ({
                                (*gDisableStructs.as_ptr())[gActiveBattler.get()].set_chargeTimer(
                                    (*gDisableStructs.as_ptr())[gActiveBattler.get()].chargeTimer()
                                        - 1,
                                );
                                (*gDisableStructs.as_ptr())[gActiveBattler.get()].chargeTimer()
                            }) == 0
                        {
                            {
                                let gl_v = 0xfffffdff;
                                let gl_i53 = gActiveBattler.get();
                                crate::gl!(gStatuses3[gl_i53] &= gl_v)
                            };
                        }
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_TAUNT => {
                        if (*gDisableStructs.as_ptr())[gActiveBattler.get()].tauntTimer() != 0 {
                            (*gDisableStructs.as_ptr())[gActiveBattler.get()].set_tauntTimer(
                                (*gDisableStructs.as_ptr())[gActiveBattler.get()].tauntTimer() - 1,
                            );
                        }
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_YAWN => {
                        if {
                            let gl_i54 = gActiveBattler.get();
                            crate::gl!(gStatuses3[gl_i54])
                        } & STATUS3_YAWN
                            != 0
                        {
                            {
                                let gl_v = 2048;
                                let gl_i55 = gActiveBattler.get();
                                crate::gl!(gStatuses3[gl_i55] -= gl_v)
                            };
                            if {
                                let gl_i56 = gActiveBattler.get();
                                crate::gl!(gStatuses3[gl_i56])
                            } & STATUS3_YAWN
                                == 0
                                && ({
                                    let gl_i108 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i108].status1)
                                }) & STATUS1_ANY
                                    == 0
                                && ({
                                    let gl_i109 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i109].ability)
                                }) != ABILITY_VITAL_SPIRIT
                                && ({
                                    let gl_i110 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i110].ability)
                                }) != ABILITY_INSOMNIA
                                && UproarWakeUpCheck(gActiveBattler.get()) == 0
                            {
                                CancelMultiTurnMoves(gActiveBattler.get());
                                {
                                    let gl_v = (Random() as u32 & 3) + 2;
                                    let gl_i111 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i111].status1 |= gl_v)
                                };
                                BtlController_EmitSetMonData(
                                    B_COMM_TO_CONTROLLER,
                                    REQUEST_STATUS_BATTLE,
                                    0,
                                    4,
                                    ({
                                        let gl_i112 = gActiveBattler.get();
                                        crate::gl!(&raw gBattleMons[gl_i112].status1)
                                    }) as *mut c_void,
                                );
                                MarkBattlerForControllerExec(gActiveBattler.get());
                                gEffectBattler.set(gActiveBattler.get());
                                BattleScriptExecute(
                                    (*crate::asmdata::BattleScript_YawnMakesAsleep
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                effect += 1;
                            }
                        }
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker += 1;
                    }
                    ENDTURN_BATTLER_COUNT => {
                        (*crate::gl!(gBattleStruct)).turnEffectsTracker = 0;
                        (*crate::gl!(gBattleStruct)).turnEffectsBattlerId += 1;
                    }
                    _ => {}
                }
            }
            if effect != 0 {
                return effect;
            }
        }
    }
    gHitMarker.set(gHitMarker.get() & (0xfeffffdf));
    0
}
pub unsafe fn HandleWishPerishSongOnTurnEnd() -> u8 {
    gHitMarker.set(gHitMarker.get() | (0x1000020));
    'l1: {
        let sw1: u8 = (*crate::gl!(gBattleStruct)).wishPerishSongState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            while (*crate::gl!(gBattleStruct)).wishPerishSongBattlerId < gBattlersCount.get() {
                gActiveBattler.set((*crate::gl!(gBattleStruct)).wishPerishSongBattlerId);
                if gAbsentBattlerFlags.get() as u32
                    & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                        [gActiveBattler.get()]
                    != 0
                {
                    (*crate::gl!(gBattleStruct)).wishPerishSongBattlerId += 1;
                    continue;
                }
                (*crate::gl!(gBattleStruct)).wishPerishSongBattlerId += 1;
                if {
                    let gl_i57 = gActiveBattler.get();
                    crate::gl!(gWishFutureKnock.futureSightCounter[gl_i57])
                } != 0
                    && ({
                        {
                            let gl_v = 1;
                            let gl_i58 = gActiveBattler.get();
                            crate::gl!(gWishFutureKnock.futureSightCounter[gl_i58] -= gl_v)
                        };
                        {
                            let gl_i59 = gActiveBattler.get();
                            crate::gl!(gWishFutureKnock.futureSightCounter[gl_i59])
                        }
                    }) == 0
                    && ({
                        let gl_i113 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i113].hp)
                    }) != 0
                {
                    if {
                        let gl_i60 = gActiveBattler.get();
                        crate::gl!(gWishFutureKnock.futureSightMove[gl_i60])
                    } == MOVE_FUTURE_SIGHT
                    {
                        crate::gl!(gBattleCommunication[5] = B_MSG_FUTURE_SIGHT);
                    } else {
                        crate::gl!(gBattleCommunication[5] = B_MSG_DOOM_DESIRE);
                    }
                    crate::gl!(gBattleTextBuff1[0] = 0xFD);
                    crate::gl!(gBattleTextBuff1[1] = 2);
                    crate::gl!(
                        gBattleTextBuff1[2] = {
                            let gl_i61 = gActiveBattler.get();
                            crate::gl!(gWishFutureKnock.futureSightMove[gl_i61])
                        } as u8
                    );
                    crate::gl!(
                        gBattleTextBuff1[3] = (({
                            let gl_i62 = gActiveBattler.get();
                            crate::gl!(gWishFutureKnock.futureSightMove[gl_i62])
                        } as i32
                            & 0xFF00)
                            >> 8) as u8
                    );
                    crate::gl!(gBattleTextBuff1[4] = 0xFF);
                    gBattlerTarget.set(gActiveBattler.get());
                    gBattlerAttacker.set({
                        let gl_i63 = gActiveBattler.get();
                        crate::gl!(gWishFutureKnock.futureSightAttacker[gl_i63])
                    });
                    gBattleMoveDamage.set({
                        let gl_i64 = gActiveBattler.get();
                        crate::gl!(gWishFutureKnock.futureSightDmg[gl_i64])
                    });
                    {
                        let gl_v = IGNORE_SHELL_BELL;
                        let gl_i65 = gBattlerTarget.get();
                        crate::gl!(gSpecialStatuses[gl_i65].shellBellDmg = gl_v)
                    };
                    BattleScriptExecute(
                        (*crate::asmdata::BattleScript_MonTookFutureAttack.cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                    );
                    if {
                        let gl_i66 = gActiveBattler.get();
                        crate::gl!(gWishFutureKnock.futureSightCounter[gl_i66])
                    } == 0
                        && {
                            let gl_i67 = gActiveBattler.get() as i32 ^ 2;
                            crate::gl!(gWishFutureKnock.futureSightCounter[gl_i67])
                        } == 0
                    {
                        {
                            let gl_v = 65471;
                            let gl_i68 = GetBattlerPosition(gBattlerTarget.get()) as i32 & 1;
                            crate::gl!(gSideStatuses[gl_i68] &= gl_v)
                        };
                    }
                    return TRUE;
                }
            }
            (*crate::gl!(gBattleStruct)).wishPerishSongState = 1;
            (*crate::gl!(gBattleStruct)).wishPerishSongBattlerId = 0;
        }
        if fall || sw1 == 1 {
            fall = true;
            while (*crate::gl!(gBattleStruct)).wishPerishSongBattlerId < gBattlersCount.get() {
                gActiveBattler.set({
                    gBattlerAttacker.set({
                        let gl_i69 = (*crate::gl!(gBattleStruct)).wishPerishSongBattlerId;
                        crate::gl!(gBattlerByTurnOrder[gl_i69])
                    });
                    gBattlerAttacker.get()
                });
                if gAbsentBattlerFlags.get() as u32
                    & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                        [gActiveBattler.get()]
                    != 0
                {
                    (*crate::gl!(gBattleStruct)).wishPerishSongBattlerId += 1;
                    continue;
                }
                (*crate::gl!(gBattleStruct)).wishPerishSongBattlerId += 1;
                if {
                    let gl_i70 = gActiveBattler.get();
                    crate::gl!(gStatuses3[gl_i70])
                } & STATUS3_PERISH_SONG
                    != 0
                {
                    crate::gl!(gBattleTextBuff1[0] = 0xFD);
                    crate::gl!(gBattleTextBuff1[1] = 1);
                    crate::gl!(gBattleTextBuff1[2] = 1);
                    crate::gl!(gBattleTextBuff1[3] = 1);
                    crate::gl!(
                        gBattleTextBuff1[4] =
                            (*gDisableStructs.as_ptr())[gActiveBattler.get()].perishSongTimer()
                    );
                    crate::gl!(gBattleTextBuff1[5] = 0xFF);
                    if (*gDisableStructs.as_ptr())[gActiveBattler.get()].perishSongTimer() == 0 {
                        {
                            let gl_v = 0xffffffdf;
                            let gl_i71 = gActiveBattler.get();
                            crate::gl!(gStatuses3[gl_i71] &= gl_v)
                        };
                        gBattleMoveDamage.set(
                            ({
                                let gl_i114 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i114].hp)
                            }) as i32,
                        );
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_PerishSongTakesLife
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                    } else {
                        (*gDisableStructs.as_ptr())[gActiveBattler.get()].set_perishSongTimer(
                            (*gDisableStructs.as_ptr())[gActiveBattler.get()].perishSongTimer() - 1,
                        );
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_PerishSongCountGoesDown
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                    }
                    BattleScriptExecute(crate::gl!(gBattlescriptCurrInstr));
                    return TRUE;
                }
            }
            (*crate::gl!(gBattleStruct)).wishPerishSongState = 2;
            (*crate::gl!(gBattleStruct)).wishPerishSongBattlerId = 0;
        }
        if fall || sw1 == 2 {
            if gBattleTypeFlags.get() & BATTLE_TYPE_ARENA != 0
                && (*crate::gl!(gBattleStruct)).arenaTurnCounter == 2
                && crate::gl!(gBattleMons[0].hp) != 0
                && crate::gl!(gBattleMons[1].hp) != 0
            {
                for i in 0..2i32 {
                    CancelMultiTurnMoves(i as u8);
                }
                crate::gl!(
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_ArenaDoJudgment
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                );
                BattleScriptExecute(
                    (*crate::asmdata::BattleScript_ArenaDoJudgment.cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
                (*crate::gl!(gBattleStruct)).wishPerishSongState += 1;
                return TRUE;
            }
            break 'l1;
        }
    }
    gHitMarker.set(gHitMarker.get() & (0xfeffffdf));
    FALSE
}
pub unsafe fn HandleFaintedMonActions() -> u8 {
    if gBattleTypeFlags.get() & BATTLE_TYPE_SAFARI != 0 {
        return FALSE;
    }
    loop {
        let mut i: i32 = 0;
        'l2: {
            let sw1: u8 = (*crate::gl!(gBattleStruct)).faintedActionsState;
            let mut fall = false;
            if sw1 == 0 {
                fall = true;
                (*crate::gl!(gBattleStruct)).faintedActionsBattlerId = 0;
                (*crate::gl!(gBattleStruct)).faintedActionsState += 1;
                i = 0;
                while i < gBattlersCount.get() as i32 {
                    if gAbsentBattlerFlags.get() as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                        != 0
                        && HasNoMonsToSwitch(i as u8, PARTY_SIZE as u8, PARTY_SIZE as u8) == 0
                    {
                        gAbsentBattlerFlags
                            .set(gAbsentBattlerFlags.get() & (!(gBitTable[i] as u8)));
                    }
                    i += 1;
                }
            }
            if fall || sw1 == 1 {
                loop {
                    gBattlerFainted.set({
                        gBattlerTarget.set((*crate::gl!(gBattleStruct)).faintedActionsBattlerId);
                        gBattlerTarget.get()
                    });
                    if (*gBattleMons.as_ptr())[(*crate::gl!(gBattleStruct)).faintedActionsBattlerId]
                        .hp
                        == 0
                        && (*crate::gl!(gBattleStruct)).givenExpMons as u32
                            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                                [(*gBattlerPartyIndexes.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).faintedActionsBattlerId]]
                            == 0
                        && gAbsentBattlerFlags.get() as u32
                            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                                [(*crate::gl!(gBattleStruct)).faintedActionsBattlerId]
                            == 0
                    {
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_GiveExp.cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                        );
                        (*crate::gl!(gBattleStruct)).faintedActionsState = 2;
                        return TRUE;
                    }
                    if ({
                        (*crate::gl!(gBattleStruct)).faintedActionsBattlerId += 1;
                        (*crate::gl!(gBattleStruct)).faintedActionsBattlerId
                    }) == gBattlersCount.get()
                    {
                        break;
                    }
                }
                (*crate::gl!(gBattleStruct)).faintedActionsState = 3;
                break 'l2;
            }
            if sw1 == 2 {
                OpponentSwitchInResetSentPokesToOpponentValue(gBattlerFainted.get());
                if ({
                    (*crate::gl!(gBattleStruct)).faintedActionsBattlerId += 1;
                    (*crate::gl!(gBattleStruct)).faintedActionsBattlerId
                }) == gBattlersCount.get()
                {
                    (*crate::gl!(gBattleStruct)).faintedActionsState = 3;
                } else {
                    (*crate::gl!(gBattleStruct)).faintedActionsState = 1;
                }
                break 'l2;
            }
            if sw1 == 3 {
                fall = true;
                (*crate::gl!(gBattleStruct)).faintedActionsBattlerId = 0;
                (*crate::gl!(gBattleStruct)).faintedActionsState += 1;
            }
            if fall || sw1 == 4 {
                loop {
                    gBattlerFainted.set({
                        gBattlerTarget.set((*crate::gl!(gBattleStruct)).faintedActionsBattlerId);
                        gBattlerTarget.get()
                    });
                    if (*gBattleMons.as_ptr())[(*crate::gl!(gBattleStruct)).faintedActionsBattlerId]
                        .hp
                        == 0
                        && gAbsentBattlerFlags.get() as u32
                            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                                [(*crate::gl!(gBattleStruct)).faintedActionsBattlerId]
                            == 0
                    {
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_HandleFaintedMon
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        );
                        (*crate::gl!(gBattleStruct)).faintedActionsState = 5;
                        return TRUE;
                    }
                    if ({
                        (*crate::gl!(gBattleStruct)).faintedActionsBattlerId += 1;
                        (*crate::gl!(gBattleStruct)).faintedActionsBattlerId
                    }) == gBattlersCount.get()
                    {
                        break;
                    }
                }
                (*crate::gl!(gBattleStruct)).faintedActionsState = 6;
                break 'l2;
            }
            if sw1 == 5 {
                if ({
                    (*crate::gl!(gBattleStruct)).faintedActionsBattlerId += 1;
                    (*crate::gl!(gBattleStruct)).faintedActionsBattlerId
                }) == gBattlersCount.get()
                {
                    (*crate::gl!(gBattleStruct)).faintedActionsState = 6;
                } else {
                    (*crate::gl!(gBattleStruct)).faintedActionsState = 4;
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
                (*crate::gl!(gBattleStruct)).faintedActionsState += 1;
                break 'l2;
            }
            if sw1 == FAINTED_ACTIONS_MAX_CASE {
                break 'l2;
            }
        }
        if (*crate::gl!(gBattleStruct)).faintedActionsState == FAINTED_ACTIONS_MAX_CASE {
            break;
        }
    }
    FALSE
}
pub fn TryClearRageStatuses() {
    for i in 0..(gBattlersCount.get() as i32) {
        if crate::gl!(gBattleMons[i].status2) & STATUS2_RAGE != 0
            && crate::gl!(gChosenMoveByBattler[i]) != MOVE_RAGE
        {
            crate::gl!(gBattleMons[i].status2 &= 0xff7fffff);
        }
    }
}
pub unsafe fn AtkCanceler_UnableToUseMove() -> u8 {
    let mut effect: u8 = 0;
    let bideDmg: *mut i32 = crate::gl!(&raw gBattleScripting.bideDmg);
    loop {
        'l2: {
            match (*crate::gl!(gBattleStruct)).atkCancelerTracker {
                CANCELER_FLAGS => {
                    {
                        let gl_i115 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i115].status2 &= 0xfdffffff)
                    };
                    {
                        let gl_v = 0xffffbfff;
                        let gl_i72 = gBattlerAttacker.get();
                        crate::gl!(gStatuses3[gl_i72] &= gl_v)
                    };
                    (*crate::gl!(gBattleStruct)).atkCancelerTracker += 1;
                }
                CANCELER_ASLEEP => {
                    if ({
                        let gl_i116 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i116].status1)
                    }) & STATUS1_SLEEP
                        != 0
                    {
                        if UproarWakeUpCheck(gBattlerAttacker.get()) != 0 {
                            {
                                let gl_i117 = gBattlerAttacker.get();
                                crate::gl!(gBattleMons[gl_i117].status1 &= 0xfffffff8)
                            };
                            {
                                let gl_i118 = gBattlerAttacker.get();
                                crate::gl!(gBattleMons[gl_i118].status2 &= 0xf7ffffff)
                            };
                            BattleScriptPushCursor();
                            crate::gl!(gBattleCommunication[5] = B_MSG_WOKE_UP_UPROAR);
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_MoveUsedWokeUp
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                            effect = 2;
                        } else {
                            let mut toSub: u8 = 0;
                            if ({
                                let gl_i119 = gBattlerAttacker.get();
                                crate::gl!(gBattleMons[gl_i119].ability)
                            }) == ABILITY_EARLY_BIRD
                            {
                                toSub = 2;
                            } else {
                                toSub = 1;
                            }
                            if ({
                                let gl_i120 = gBattlerAttacker.get();
                                crate::gl!(gBattleMons[gl_i120].status1)
                            }) & STATUS1_SLEEP
                                < toSub as u32
                            {
                                {
                                    let gl_i121 = gBattlerAttacker.get();
                                    crate::gl!(gBattleMons[gl_i121].status1 &= 0xfffffff8)
                                };
                            } else {
                                {
                                    let gl_i122 = gBattlerAttacker.get();
                                    crate::gl!(gBattleMons[gl_i122].status1 -= toSub as u32)
                                };
                            }
                            if ({
                                let gl_i123 = gBattlerAttacker.get();
                                crate::gl!(gBattleMons[gl_i123].status1)
                            }) & STATUS1_SLEEP
                                != 0
                            {
                                if gCurrentMove.get() != MOVE_SNORE
                                    && gCurrentMove.get() != MOVE_SLEEP_TALK
                                {
                                    crate::gl!(
                                        gBattlescriptCurrInstr =
                                            (*crate::asmdata::BattleScript_MoveUsedIsAsleep
                                                .cast::<CArray<u8, 0>>())
                                            .as_ptr()
                                            .cast_mut()
                                    );
                                    gHitMarker
                                        .set(gHitMarker.get() | (HITMARKER_UNABLE_TO_USE_MOVE));
                                    effect = 2;
                                }
                            } else {
                                {
                                    let gl_i124 = gBattlerAttacker.get();
                                    crate::gl!(gBattleMons[gl_i124].status2 &= 0xf7ffffff)
                                };
                                BattleScriptPushCursor();
                                crate::gl!(gBattleCommunication[5] = B_MSG_WOKE_UP);
                                crate::gl!(
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_MoveUsedWokeUp
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut()
                                );
                                effect = 2;
                            }
                        }
                    }
                    (*crate::gl!(gBattleStruct)).atkCancelerTracker += 1;
                }
                CANCELER_FROZEN => {
                    if ({
                        let gl_i125 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i125].status1)
                    }) & STATUS1_FREEZE
                        != 0
                    {
                        if Random() as i32 % 5 != 0 {
                            if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<
                                BattleMove,
                                0,
                            >>(
                            ))[gCurrentMove.get()]
                            .effect
                                != EFFECT_THAW_HIT
                            {
                                crate::gl!(
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_MoveUsedIsFrozen
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut()
                                );
                                gHitMarker.set(gHitMarker.get() | (HITMARKER_NO_ATTACKSTRING));
                            } else {
                                (*crate::gl!(gBattleStruct)).atkCancelerTracker += 1;
                                break 'l2;
                            }
                        } else {
                            {
                                let gl_i126 = gBattlerAttacker.get();
                                crate::gl!(gBattleMons[gl_i126].status1 &= 0xffffffdf)
                            };
                            BattleScriptPushCursor();
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_MoveUsedUnfroze
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                            crate::gl!(gBattleCommunication[5] = B_MSG_DEFROSTED);
                        }
                        effect = 2;
                    }
                    (*crate::gl!(gBattleStruct)).atkCancelerTracker += 1;
                }
                CANCELER_TRUANT => {
                    if ({
                        let gl_i127 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i127].ability)
                    }) == ABILITY_TRUANT
                        && (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].truantCounter() != 0
                    {
                        CancelMultiTurnMoves(gBattlerAttacker.get());
                        gHitMarker.set(gHitMarker.get() | (HITMARKER_UNABLE_TO_USE_MOVE));
                        crate::gl!(gBattleCommunication[5] = B_MSG_LOAFING);
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_MoveUsedLoafingAround
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
                        effect = 1;
                    }
                    (*crate::gl!(gBattleStruct)).atkCancelerTracker += 1;
                }
                CANCELER_RECHARGE => {
                    if ({
                        let gl_i128 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i128].status2)
                    }) & STATUS2_RECHARGE
                        != 0
                    {
                        {
                            let gl_i129 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i129].status2 &= 0xffbfffff)
                        };
                        {
                            let gl_i130 = gBattlerAttacker.get();
                            crate::gl!(gDisableStructs[gl_i130].rechargeTimer = 0)
                        };
                        CancelMultiTurnMoves(gBattlerAttacker.get());
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_MoveUsedMustRecharge
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        gHitMarker.set(gHitMarker.get() | (HITMARKER_UNABLE_TO_USE_MOVE));
                        effect = 1;
                    }
                    (*crate::gl!(gBattleStruct)).atkCancelerTracker += 1;
                }
                CANCELER_FLINCH => {
                    if ({
                        let gl_i131 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i131].status2)
                    }) & STATUS2_FLINCHED
                        != 0
                    {
                        {
                            let gl_i132 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i132].status2 &= 0xfffffff7)
                        };
                        (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].set_flinchImmobility(1);
                        CancelMultiTurnMoves(gBattlerAttacker.get());
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_MoveUsedFlinched
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        gHitMarker.set(gHitMarker.get() | (HITMARKER_UNABLE_TO_USE_MOVE));
                        effect = 1;
                    }
                    (*crate::gl!(gBattleStruct)).atkCancelerTracker += 1;
                }
                CANCELER_DISABLED => {
                    if ({
                        let gl_i133 = gBattlerAttacker.get();
                        crate::gl!(gDisableStructs[gl_i133].disabledMove)
                    }) == gCurrentMove.get()
                        && ({
                            let gl_i134 = gBattlerAttacker.get();
                            crate::gl!(gDisableStructs[gl_i134].disabledMove)
                        }) != MOVE_NONE
                    {
                        (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].set_usedDisabledMove(1);
                        crate::gl!(gBattleScripting.battler = gBattlerAttacker.get());
                        CancelMultiTurnMoves(gBattlerAttacker.get());
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_MoveUsedIsDisabled
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        gHitMarker.set(gHitMarker.get() | (HITMARKER_UNABLE_TO_USE_MOVE));
                        effect = 1;
                    }
                    (*crate::gl!(gBattleStruct)).atkCancelerTracker += 1;
                }
                CANCELER_TAUNTED => {
                    if (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].tauntTimer() != 0
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
                        .power
                            == 0
                    {
                        (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].set_usedTauntedMove(1);
                        CancelMultiTurnMoves(gBattlerAttacker.get());
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_MoveUsedIsTaunted
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        gHitMarker.set(gHitMarker.get() | (HITMARKER_UNABLE_TO_USE_MOVE));
                        effect = 1;
                    }
                    (*crate::gl!(gBattleStruct)).atkCancelerTracker += 1;
                }
                CANCELER_IMPRISONED => {
                    if GetImprisonedMovesCount(gBattlerAttacker.get(), gCurrentMove.get()) != 0 {
                        (*gProtectStructs.as_ptr())[gBattlerAttacker.get()]
                            .set_usedImprisonedMove(1);
                        CancelMultiTurnMoves(gBattlerAttacker.get());
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_MoveUsedIsImprisoned
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        gHitMarker.set(gHitMarker.get() | (HITMARKER_UNABLE_TO_USE_MOVE));
                        effect = 1;
                    }
                    (*crate::gl!(gBattleStruct)).atkCancelerTracker += 1;
                }
                CANCELER_CONFUSED => {
                    if ({
                        let gl_i135 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i135].status2)
                    }) & STATUS2_CONFUSION
                        != 0
                    {
                        {
                            let gl_i136 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i136].status2 -= 1)
                        };
                        if ({
                            let gl_i137 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i137].status2)
                        }) & STATUS2_CONFUSION
                            != 0
                        {
                            if Random() as i32 & 1 != 0 {
                                crate::gl!(gBattleCommunication[5] = FALSE);
                                BattleScriptPushCursor();
                            } else {
                                crate::gl!(gBattleCommunication[5] = TRUE);
                                gBattlerTarget.set(gBattlerAttacker.get());
                                gBattleMoveDamage.set(CalculateBaseDamage(
                                    {
                                        let gl_i138 = gBattlerAttacker.get();
                                        crate::gl!(&raw gBattleMons[gl_i138])
                                    },
                                    {
                                        let gl_i139 = gBattlerAttacker.get();
                                        crate::gl!(&raw gBattleMons[gl_i139])
                                    },
                                    MOVE_POUND,
                                    0,
                                    40,
                                    0,
                                    gBattlerAttacker.get(),
                                    gBattlerAttacker.get(),
                                ));
                                (*gProtectStructs.as_ptr())[gBattlerAttacker.get()]
                                    .set_confusionSelfDmg(1);
                                gHitMarker.set(gHitMarker.get() | (HITMARKER_UNABLE_TO_USE_MOVE));
                            }
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_MoveUsedIsConfused
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                        } else {
                            BattleScriptPushCursor();
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_MoveUsedIsConfusedNoMore
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                        }
                        effect = 1;
                    }
                    (*crate::gl!(gBattleStruct)).atkCancelerTracker += 1;
                }
                CANCELER_PARALYZED => {
                    if ({
                        let gl_i140 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i140].status1)
                    }) & STATUS1_PARALYSIS
                        != 0
                        && Random() as i32 % 4 == 0
                    {
                        (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].set_prlzImmobility(1);
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_MoveUsedIsParalyzed
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        gHitMarker.set(gHitMarker.get() | (HITMARKER_UNABLE_TO_USE_MOVE));
                        effect = 1;
                    }
                    (*crate::gl!(gBattleStruct)).atkCancelerTracker += 1;
                }
                CANCELER_IN_LOVE => {
                    if ({
                        let gl_i141 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i141].status2)
                    }) & STATUS2_INFATUATION
                        != 0
                    {
                        crate::gl!(
                            gBattleScripting.battler = CountTrailingZeroBits(
                                (({
                                    let gl_i142 = gBattlerAttacker.get();
                                    crate::gl!(gBattleMons[gl_i142].status2)
                                }) & STATUS2_INFATUATION)
                                    >> 16,
                            ) as u8
                        );
                        if Random() as i32 & 1 != 0 {
                            BattleScriptPushCursor();
                        } else {
                            BattleScriptPush(
                                (*crate::asmdata::BattleScript_MoveUsedIsInLoveCantAttack
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            gHitMarker.set(gHitMarker.get() | (HITMARKER_UNABLE_TO_USE_MOVE));
                            (*gProtectStructs.as_ptr())[gBattlerAttacker.get()]
                                .set_loveImmobility(1);
                            CancelMultiTurnMoves(gBattlerAttacker.get());
                        }
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_MoveUsedIsInLove
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        effect = 1;
                    }
                    (*crate::gl!(gBattleStruct)).atkCancelerTracker += 1;
                }
                CANCELER_BIDE => {
                    if ({
                        let gl_i143 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i143].status2)
                    }) & STATUS2_BIDE
                        != 0
                    {
                        {
                            let gl_i144 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i144].status2 -= 256)
                        };
                        if ({
                            let gl_i145 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i145].status2)
                        }) & STATUS2_BIDE
                            != 0
                        {
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_BideStoringEnergy
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                        } else {
                            if {
                                let gl_i73 = gBattlerAttacker.get();
                                crate::gl!(gBideDmg[gl_i73])
                            } != 0
                            {
                                gCurrentMove.set(MOVE_BIDE);
                                *bideDmg = {
                                    let gl_i74 = gBattlerAttacker.get();
                                    crate::gl!(gBideDmg[gl_i74])
                                } * 2;
                                gBattlerTarget.set({
                                    let gl_i75 = gBattlerAttacker.get();
                                    crate::gl!(gBideTarget[gl_i75])
                                });
                                if gAbsentBattlerFlags.get() as u32
                                    & (*(&raw const crate::util::gBitTable)
                                        .cast::<CArray<u32, 0>>())[gBattlerTarget.get()]
                                    != 0
                                {
                                    gBattlerTarget.set(GetMoveTarget(MOVE_BIDE, 1));
                                }
                                crate::gl!(
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_BideAttack
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut()
                                );
                            } else {
                                crate::gl!(
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_BideNoEnergyToAttack
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut()
                                );
                            }
                        }
                        effect = 1;
                    }
                    (*crate::gl!(gBattleStruct)).atkCancelerTracker += 1;
                }
                CANCELER_THAW => {
                    if ({
                        let gl_i146 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i146].status1)
                    }) & STATUS1_FREEZE
                        != 0
                    {
                        if (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
                        .effect
                            == EFFECT_THAW_HIT
                        {
                            {
                                let gl_i147 = gBattlerAttacker.get();
                                crate::gl!(gBattleMons[gl_i147].status1 &= 0xffffffdf)
                            };
                            BattleScriptPushCursor();
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_MoveUsedUnfroze
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                            crate::gl!(gBattleCommunication[5] = B_MSG_DEFROSTED_BY_MOVE);
                        }
                        effect = 2;
                    }
                    (*crate::gl!(gBattleStruct)).atkCancelerTracker += 1;
                }
                CANCELER_END => {}
                _ => {}
            }
        }
        if !((*crate::gl!(gBattleStruct)).atkCancelerTracker != CANCELER_END && effect == 0) {
            break;
        }
    }
    if effect == 2 {
        gActiveBattler.set(gBattlerAttacker.get());
        BtlController_EmitSetMonData(
            B_COMM_TO_CONTROLLER,
            REQUEST_STATUS_BATTLE,
            0,
            4,
            ({
                let gl_i148 = gActiveBattler.get();
                crate::gl!(&raw gBattleMons[gl_i148].status1)
            }) as *mut c_void,
        );
        MarkBattlerForControllerExec(gActiveBattler.get());
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
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE == 0 {
        return FALSE;
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER != 0 {
        if GetBattlerSide(battler) == B_SIDE_PLAYER {
            party = (*gPlayerParty.as_ptr()).as_mut_ptr();
        } else {
            party = (*gEnemyParty.as_ptr()).as_mut_ptr();
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
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
        if gBattleTypeFlags.get() & BATTLE_TYPE_TOWER_LINK_MULTI != 0 {
            if GetBattlerSide(battler) == B_SIDE_PLAYER {
                party = (*gPlayerParty.as_ptr()).as_mut_ptr();
                flankId = GetBattlerMultiplayerId(battler as u16) as u8;
                playerId = GetLinkTrainerFlankId(flankId) as u8;
            } else {
                party = (*gEnemyParty.as_ptr()).as_mut_ptr();
                if battler == 1 {
                    playerId = 0;
                } else {
                    playerId = 1;
                }
            }
        } else {
            flankId = GetBattlerMultiplayerId(battler as u16) as u8;
            if GetBattlerSide(battler) == B_SIDE_PLAYER {
                party = (*gPlayerParty.as_ptr()).as_mut_ptr();
            } else {
                party = (*gEnemyParty.as_ptr()).as_mut_ptr();
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
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_TWO_OPPONENTS != 0
        && GetBattlerSide(battler) == B_SIDE_OPPONENT
    {
        party = (*gEnemyParty.as_ptr()).as_mut_ptr();
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
            party = (*gEnemyParty.as_ptr()).as_mut_ptr();
        } else {
            flankId = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
            playerId = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT);
            party = (*gPlayerParty.as_ptr()).as_mut_ptr();
        }
        if partyIdBattlerOn1 == PARTY_SIZE as u8 {
            partyIdBattlerOn1 = crate::gl!(gBattlerPartyIndexes[flankId]) as u8;
        }
        if partyIdBattlerOn2 == PARTY_SIZE as u8 {
            partyIdBattlerOn2 = crate::gl!(gBattlerPartyIndexes[playerId]) as u8;
        }
        i = 0;
        while i < PARTY_SIZE {
            if GetMonData2(party.at(i), MON_DATA_HP) != 0
                && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_NONE as u32
                && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_EGG
                && i != partyIdBattlerOn1 as i32
                && i != partyIdBattlerOn2 as i32
                && i != *(*crate::gl!(gBattleStruct))
                    .monToSwitchIntoId
                    .as_mut_ptr()
                    .at(flankId) as i32
                && i != *(*crate::gl!(gBattleStruct))
                    .monToSwitchIntoId
                    .as_mut_ptr()
                    .at(playerId) as i32
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
    if crate::gl!(gBattleMons[battler].species) != SPECIES_CASTFORM
        || crate::gl!(gBattleMons[battler].ability) != ABILITY_FORECAST
        || crate::gl!(gBattleMons[battler].hp) == 0
    {
        return 0;
    }
    if !(AbilityBattleEffects(19, TYPE_NORMAL, 13, TYPE_NORMAL, TYPE_NORMAL as u16) == 0
        && AbilityBattleEffects(19, TYPE_NORMAL, 77, TYPE_NORMAL, TYPE_NORMAL as u16) == 0)
        && !(crate::gl!(gBattleMons[battler].types[0]) == TYPE_NORMAL
            || crate::gl!(gBattleMons[battler].types[1]) == TYPE_NORMAL)
    {
        crate::gl!(gBattleMons[battler].types[0] = TYPE_NORMAL);
        crate::gl!(gBattleMons[battler].types[1] = TYPE_NORMAL);
        return 1;
    }
    if !(AbilityBattleEffects(19, 0, 13, 0, 0) == 0 && AbilityBattleEffects(19, 0, 77, 0, 0) == 0) {
        return 0;
    }
    if gBattleWeather.get() as i32 & 231 == 0
        && !(crate::gl!(gBattleMons[battler].types[0]) == TYPE_NORMAL
            || crate::gl!(gBattleMons[battler].types[1]) == TYPE_NORMAL)
    {
        crate::gl!(gBattleMons[battler].types[0] = TYPE_NORMAL);
        crate::gl!(gBattleMons[battler].types[1] = TYPE_NORMAL);
        formChange = 1;
    }
    if gBattleWeather.get() as i32 & B_WEATHER_SUN != 0
        && !(crate::gl!(gBattleMons[battler].types[0]) == TYPE_FIRE
            || crate::gl!(gBattleMons[battler].types[1]) == TYPE_FIRE)
    {
        crate::gl!(gBattleMons[battler].types[0] = TYPE_FIRE);
        crate::gl!(gBattleMons[battler].types[1] = TYPE_FIRE);
        formChange = 2;
    }
    if gBattleWeather.get() as i32 & B_WEATHER_RAIN != 0
        && !(crate::gl!(gBattleMons[battler].types[0]) == TYPE_WATER
            || crate::gl!(gBattleMons[battler].types[1]) == TYPE_WATER)
    {
        crate::gl!(gBattleMons[battler].types[0] = TYPE_WATER);
        crate::gl!(gBattleMons[battler].types[1] = TYPE_WATER);
        formChange = 3;
    }
    if gBattleWeather.get() as i32 & B_WEATHER_HAIL != 0
        && !(crate::gl!(gBattleMons[battler].types[0]) == TYPE_ICE
            || crate::gl!(gBattleMons[battler].types[1]) == TYPE_ICE)
    {
        crate::gl!(gBattleMons[battler].types[0] = TYPE_ICE);
        crate::gl!(gBattleMons[battler].types[1] = TYPE_ICE);
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
    if gBattlerAttacker.get() >= gBattlersCount.get() {
        gBattlerAttacker.set(battler);
    }
    if GetBattlerSide(gBattlerAttacker.get()) == B_SIDE_PLAYER {
        pokeAtk = &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i149 = gBattlerAttacker.get();
            crate::gl!(gBattlerPartyIndexes[gl_i149])
        }];
    } else {
        pokeAtk = &raw mut (*gEnemyParty.as_ptr())[{
            let gl_i150 = gBattlerAttacker.get();
            crate::gl!(gBattlerPartyIndexes[gl_i150])
        }];
    }
    if gBattlerTarget.get() >= gBattlersCount.get() {
        gBattlerTarget.set(battler);
    }
    if GetBattlerSide(gBattlerTarget.get()) == B_SIDE_PLAYER {
        pokeDef = &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i151 = gBattlerTarget.get();
            crate::gl!(gBattlerPartyIndexes[gl_i151])
        }];
    } else {
        pokeDef = &raw mut (*gEnemyParty.as_ptr())[{
            let gl_i152 = gBattlerTarget.get();
            crate::gl!(gBattlerPartyIndexes[gl_i152])
        }];
    }
    let speciesAtk: u16 = GetMonData2(pokeAtk, MON_DATA_SPECIES) as u16;
    let pidAtk: u32 = GetMonData2(pokeAtk, MON_DATA_PERSONALITY);
    let speciesDef: u16 = GetMonData2(pokeDef, MON_DATA_SPECIES) as u16;
    let pidDef: u32 = GetMonData2(pokeDef, MON_DATA_PERSONALITY);
    if gBattleTypeFlags.get() & BATTLE_TYPE_SAFARI == 0 {
        let mut moveType: u8 = 0;
        let mut i: i32 = 0;
        let mut r#move: u16 = 0;
        let mut side: u8 = 0;
        let mut target1: u8 = 0;
        if special != 0 {
            gLastUsedAbility.set(special);
        } else {
            gLastUsedAbility.set(crate::gl!(gBattleMons[battler].ability));
        }
        if moveArg != 0 {
            r#move = moveArg;
        } else {
            r#move = gCurrentMove.get();
        }
        if (*crate::gl!(gBattleStruct)).dynamicMoveType != 0 {
            moveType = (*crate::gl!(gBattleStruct)).dynamicMoveType & 63;
        } else {
            moveType = (*(&raw const crate::data::pokemon::gBattleMoves)
                .cast::<CArray<BattleMove, 0>>())[r#move]
                .r#type;
        }
        match caseID {
            0 => {
                if gBattlerAttacker.get() >= gBattlersCount.get() {
                    gBattlerAttacker.set(battler);
                }
                match gLastUsedAbility.get() {
                    ABILITYEFFECT_SWITCH_IN_WEATHER => {
                        if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED == 0 {
                            match GetCurrentWeather() {
                                WEATHER_RAIN | WEATHER_RAIN_THUNDERSTORM | WEATHER_DOWNPOUR => {
                                    if gBattleWeather.get() as i32 & B_WEATHER_RAIN == 0 {
                                        gBattleWeather.set(5);
                                        crate::gl!(
                                            gBattleScripting.animArg1 = B_ANIM_RAIN_CONTINUES
                                        );
                                        crate::gl!(gBattleScripting.battler = battler);
                                        effect += 1;
                                    }
                                }
                                WEATHER_SANDSTORM => {
                                    if gBattleWeather.get() as i32 & B_WEATHER_SANDSTORM == 0 {
                                        gBattleWeather.set(B_WEATHER_SANDSTORM as u16);
                                        crate::gl!(
                                            gBattleScripting.animArg1 = B_ANIM_SANDSTORM_CONTINUES
                                        );
                                        crate::gl!(gBattleScripting.battler = battler);
                                        effect += 1;
                                    }
                                }
                                WEATHER_DROUGHT
                                    if gBattleWeather.get() as i32 & B_WEATHER_SUN == 0 =>
                                {
                                    gBattleWeather.set(B_WEATHER_SUN as u16);
                                    crate::gl!(gBattleScripting.animArg1 = B_ANIM_SUN_CONTINUES);
                                    crate::gl!(gBattleScripting.battler = battler);
                                    effect += 1;
                                }
                                _ => {}
                            }
                        }
                        if effect != 0 {
                            crate::gl!(gBattleCommunication[5] = GetCurrentWeather());
                            BattleScriptPushCursorAndCallback(
                                (*crate::asmdata::BattleScript_OverworldWeatherStarts
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                        }
                    }
                    ABILITY_DRIZZLE => {
                        if gBattleWeather.get() as i32 & B_WEATHER_RAIN_PERMANENT == 0 {
                            gBattleWeather.set(5);
                            BattleScriptPushCursorAndCallback(
                                (*crate::asmdata::BattleScript_DrizzleActivates
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            crate::gl!(gBattleScripting.battler = battler);
                            effect += 1;
                        }
                    }
                    ABILITY_SAND_STREAM => {
                        if gBattleWeather.get() as i32 & B_WEATHER_SANDSTORM_PERMANENT == 0 {
                            gBattleWeather.set(B_WEATHER_SANDSTORM as u16);
                            BattleScriptPushCursorAndCallback(
                                (*crate::asmdata::BattleScript_SandstreamActivates
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            crate::gl!(gBattleScripting.battler = battler);
                            effect += 1;
                        }
                    }
                    ABILITY_DROUGHT => {
                        if gBattleWeather.get() as i32 & B_WEATHER_SUN_PERMANENT == 0 {
                            gBattleWeather.set(B_WEATHER_SUN as u16);
                            BattleScriptPushCursorAndCallback(
                                (*crate::asmdata::BattleScript_DroughtActivates
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            crate::gl!(gBattleScripting.battler = battler);
                            effect += 1;
                        }
                    }
                    ABILITY_INTIMIDATE => {
                        if (*gSpecialStatuses.as_ptr())[battler].intimidatedMon() == 0 {
                            crate::gl!(gStatuses3[battler] |= STATUS3_INTIMIDATE_POKES);
                            (*gSpecialStatuses.as_ptr())[battler].set_intimidatedMon(1);
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
                            crate::gl!(gBattleScripting.battler = battler);
                            (*crate::gl!(gBattleStruct)).formToChangeInto = effect - 1;
                        }
                    }
                    ABILITY_TRACE => {
                        if (*gSpecialStatuses.as_ptr())[battler].traced() == 0 {
                            crate::gl!(gStatuses3[battler] |= STATUS3_TRACE);
                            (*gSpecialStatuses.as_ptr())[battler].set_traced(1);
                        }
                    }
                    ABILITY_CLOUD_NINE | ABILITY_AIR_LOCK => {
                        target1 = 0;
                        while target1 < gBattlersCount.get() {
                            effect = CastformDataTypeChange(target1);
                            if effect != 0 {
                                BattleScriptPushCursorAndCallback(
                                    (*crate::asmdata::BattleScript_CastformChange
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                crate::gl!(gBattleScripting.battler = target1);
                                (*crate::gl!(gBattleStruct)).formToChangeInto = effect - 1;
                                break;
                            }
                            target1 += 1;
                        }
                    }
                    _ => {}
                }
            }
            1 => {
                if crate::gl!(gBattleMons[battler].hp) != 0 {
                    gBattlerAttacker.set(battler);
                    match gLastUsedAbility.get() {
                        ABILITY_RAIN_DISH => {
                            if AbilityBattleEffects(19, 0, 13, 0, 0) == 0
                                && AbilityBattleEffects(19, 0, 77, 0, 0) == 0
                                && gBattleWeather.get() as i32 & B_WEATHER_RAIN != 0
                                && crate::gl!(gBattleMons[battler].maxHP)
                                    > crate::gl!(gBattleMons[battler].hp)
                            {
                                gLastUsedAbility.set(ABILITY_RAIN_DISH);
                                BattleScriptPushCursorAndCallback(
                                    (*crate::asmdata::BattleScript_RainDishActivates
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                gBattleMoveDamage
                                    .set(crate::gl!(gBattleMons[battler].maxHP) as i32 / 16);
                                if gBattleMoveDamage.get() == 0 {
                                    gBattleMoveDamage.set(1);
                                }
                                gBattleMoveDamage.set(-gBattleMoveDamage.get());
                                effect += 1;
                            }
                        }
                        ABILITY_SHED_SKIN => {
                            if crate::gl!(gBattleMons[battler].status1) & STATUS1_ANY != 0
                                && Random() as i32 % 3 == 0
                            {
                                if crate::gl!(gBattleMons[battler].status1) & 136 != 0 {
                                    StringCopy(
                                        (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                        (*(&raw const crate::data::battle_main::gStatusConditionString_PoisonJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                    );
                                }
                                if crate::gl!(gBattleMons[battler].status1) & STATUS1_SLEEP != 0 {
                                    StringCopy(
                                        (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                        (*(&raw const crate::data::battle_main::gStatusConditionString_SleepJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                    );
                                }
                                if crate::gl!(gBattleMons[battler].status1) & STATUS1_PARALYSIS != 0
                                {
                                    StringCopy(
                                        (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                        (*(&raw const crate::data::battle_main::gStatusConditionString_ParalysisJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                    );
                                }
                                if crate::gl!(gBattleMons[battler].status1) & STATUS1_BURN != 0 {
                                    StringCopy(
                                        (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                        (*(&raw const crate::data::battle_main::gStatusConditionString_BurnJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                    );
                                }
                                if crate::gl!(gBattleMons[battler].status1) & STATUS1_FREEZE != 0 {
                                    StringCopy(
                                        (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                        (*(&raw const crate::data::battle_main::gStatusConditionString_IceJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                    );
                                }
                                crate::gl!(gBattleMons[battler].status1 = 0);
                                crate::gl!(gBattleMons[battler].status2 &= 0xf7ffffff);
                                crate::gl!(
                                    gBattleScripting.battler = {
                                        gActiveBattler.set(battler);
                                        gActiveBattler.get()
                                    }
                                );
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
                                    crate::gl!(&raw gBattleMons[battler].status1) as *mut c_void,
                                );
                                MarkBattlerForControllerExec(gActiveBattler.get());
                                effect += 1;
                            }
                        }
                        ABILITY_SPEED_BOOST => {
                            if crate::gl!(gBattleMons[battler].statStages[3]) < MAX_STAT_STAGE
                                && crate::gl!(gDisableStructs[battler].isFirstTurn) != 2
                            {
                                crate::gl!(gBattleMons[battler].statStages[3] += 1);
                                crate::gl!(gBattleScripting.animArg1 = 17);
                                crate::gl!(gBattleScripting.animArg2 = 0);
                                BattleScriptPushCursorAndCallback(
                                    (*crate::asmdata::BattleScript_SpeedBoostActivates
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut(),
                                );
                                crate::gl!(gBattleScripting.battler = battler);
                                effect += 1;
                            }
                        }
                        ABILITY_TRUANT => {
                            (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].set_truantCounter(
                                (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].truantCounter()
                                    ^ 1,
                            );
                        }
                        _ => {}
                    }
                }
            }
            2 => {
                if gLastUsedAbility.get() == ABILITY_SOUNDPROOF {
                    i = 0;
                    while sSoundMovesTable[i] != SOUND_MOVES_END {
                        if sSoundMovesTable[i] == r#move {
                            break;
                        }
                        i += 1;
                    }
                    if sSoundMovesTable[i] != SOUND_MOVES_END {
                        if ({
                            let gl_i153 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i153].status2)
                        }) & STATUS2_MULTIPLETURNS
                            != 0
                        {
                            gHitMarker.set(gHitMarker.get() | (HITMARKER_NO_PPDEDUCT));
                        }
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_SoundproofProtected
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        effect = 1;
                    }
                }
            }
            3 => {
                if r#move != 0 {
                    match gLastUsedAbility.get() {
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
                                if (*gProtectStructs.as_ptr())[gBattlerAttacker.get()]
                                    .notFirstStrike()
                                    != 0
                                {
                                    crate::gl!(
                                        gBattlescriptCurrInstr =
                                            (*crate::asmdata::BattleScript_MoveHPDrain
                                                .cast::<CArray<u8, 0>>())
                                            .as_ptr()
                                            .cast_mut()
                                    );
                                } else {
                                    crate::gl!(
                                        gBattlescriptCurrInstr =
                                            (*crate::asmdata::BattleScript_MoveHPDrain_PPLoss
                                                .cast::<CArray<u8, 0>>())
                                            .as_ptr()
                                            .cast_mut()
                                    );
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
                                if (*gProtectStructs.as_ptr())[gBattlerAttacker.get()]
                                    .notFirstStrike()
                                    != 0
                                {
                                    crate::gl!(
                                        gBattlescriptCurrInstr =
                                            (*crate::asmdata::BattleScript_MoveHPDrain
                                                .cast::<CArray<u8, 0>>())
                                            .as_ptr()
                                            .cast_mut()
                                    );
                                } else {
                                    crate::gl!(
                                        gBattlescriptCurrInstr =
                                            (*crate::asmdata::BattleScript_MoveHPDrain_PPLoss
                                                .cast::<CArray<u8, 0>>())
                                            .as_ptr()
                                            .cast_mut()
                                    );
                                }
                                effect = 1;
                            }
                        }
                        ABILITY_FLASH_FIRE
                            if moveType == TYPE_FIRE
                                && crate::gl!(gBattleMons[battler].status1) & STATUS1_FREEZE
                                    == 0 =>
                        {
                            if (*(*crate::gl!(gBattleResources)).flags).flags[battler]
                                & RESOURCE_FLAG_FLASH_FIRE
                                == 0
                            {
                                crate::gl!(gBattleCommunication[5] = B_MSG_FLASH_FIRE_BOOST);
                                if (*gProtectStructs.as_ptr())[gBattlerAttacker.get()]
                                    .notFirstStrike()
                                    != 0
                                {
                                    crate::gl!(
                                        gBattlescriptCurrInstr =
                                            (*crate::asmdata::BattleScript_FlashFireBoost
                                                .cast::<CArray<u8, 0>>())
                                            .as_ptr()
                                            .cast_mut()
                                    );
                                } else {
                                    crate::gl!(
                                        gBattlescriptCurrInstr =
                                            (*crate::asmdata::BattleScript_FlashFireBoost_PPLoss
                                                .cast::<CArray<u8, 0>>())
                                            .as_ptr()
                                            .cast_mut()
                                    );
                                }
                                (*(*crate::gl!(gBattleResources)).flags).flags[battler] |=
                                    RESOURCE_FLAG_FLASH_FIRE;
                                effect = 2;
                            } else {
                                crate::gl!(gBattleCommunication[5] = B_MSG_FLASH_FIRE_NO_BOOST);
                                if (*gProtectStructs.as_ptr())[gBattlerAttacker.get()]
                                    .notFirstStrike()
                                    != 0
                                {
                                    crate::gl!(
                                        gBattlescriptCurrInstr =
                                            (*crate::asmdata::BattleScript_FlashFireBoost
                                                .cast::<CArray<u8, 0>>())
                                            .as_ptr()
                                            .cast_mut()
                                    );
                                } else {
                                    crate::gl!(
                                        gBattlescriptCurrInstr =
                                            (*crate::asmdata::BattleScript_FlashFireBoost_PPLoss
                                                .cast::<CArray<u8, 0>>())
                                            .as_ptr()
                                            .cast_mut()
                                    );
                                }
                                effect = 2;
                            }
                        }
                        _ => {}
                    }
                    if effect == 1 {
                        if crate::gl!(gBattleMons[battler].maxHP)
                            == crate::gl!(gBattleMons[battler].hp)
                        {
                            if (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].notFirstStrike()
                                != 0
                            {
                                crate::gl!(
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_MonMadeMoveUseless
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut()
                                );
                            } else {
                                crate::gl!(
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_MonMadeMoveUseless_PPLoss
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut()
                                );
                            }
                        } else {
                            gBattleMoveDamage
                                .set(crate::gl!(gBattleMons[battler].maxHP) as i32 / 4);
                            if gBattleMoveDamage.get() == 0 {
                                gBattleMoveDamage.set(1);
                            }
                            gBattleMoveDamage.set(-gBattleMoveDamage.get());
                        }
                    }
                }
            }
            ABILITYEFFECT_ON_DAMAGE => match gLastUsedAbility.get() {
                ABILITY_COLOR_CHANGE => {
                    if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && r#move != MOVE_STRUGGLE
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[r#move]
                            .power
                            != 0
                        && ({
                            let gl_i76 = gBattlerTarget.get();
                            crate::gl!(gSpecialStatuses[gl_i76].physicalDmg)
                        } != 0
                            || {
                                let gl_i77 = gBattlerTarget.get();
                                crate::gl!(gSpecialStatuses[gl_i77].specialDmg)
                            } != 0)
                        && !(crate::gl!(gBattleMons[battler].types[0]) == moveType
                            || crate::gl!(gBattleMons[battler].types[1]) == moveType)
                        && crate::gl!(gBattleMons[battler].hp) != 0
                    {
                        crate::gl!(gBattleMons[battler].types[0] = moveType);
                        crate::gl!(gBattleMons[battler].types[1] = moveType);
                        crate::gl!(gBattleTextBuff1[0] = 0xFD);
                        crate::gl!(gBattleTextBuff1[1] = 3);
                        crate::gl!(gBattleTextBuff1[2] = moveType);
                        crate::gl!(gBattleTextBuff1[3] = 0xFF);
                        BattleScriptPushCursor();
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_ColorChangeActivates
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        effect += 1;
                    }
                }
                ABILITY_ROUGH_SKIN => {
                    if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && ({
                            let gl_i154 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i154].hp)
                        }) != 0
                        && (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].confusionSelfDmg()
                            == 0
                        && ({
                            let gl_i78 = gBattlerTarget.get();
                            crate::gl!(gSpecialStatuses[gl_i78].physicalDmg)
                        } != 0
                            || {
                                let gl_i79 = gBattlerTarget.get();
                                crate::gl!(gSpecialStatuses[gl_i79].specialDmg)
                            } != 0)
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[r#move]
                            .flags as i32
                            & FLAG_MAKES_CONTACT
                            != 0
                    {
                        gBattleMoveDamage.set(
                            ({
                                let gl_i155 = gBattlerAttacker.get();
                                crate::gl!(gBattleMons[gl_i155].maxHP)
                            }) as i32
                                / 16,
                        );
                        if gBattleMoveDamage.get() == 0 {
                            gBattleMoveDamage.set(1);
                        }
                        BattleScriptPushCursor();
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_RoughSkinActivates
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        effect += 1;
                    }
                }
                ABILITY_EFFECT_SPORE => {
                    if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && ({
                            let gl_i156 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i156].hp)
                        }) != 0
                        && (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].confusionSelfDmg()
                            == 0
                        && ({
                            let gl_i80 = gBattlerTarget.get();
                            crate::gl!(gSpecialStatuses[gl_i80].physicalDmg)
                        } != 0
                            || {
                                let gl_i81 = gBattlerTarget.get();
                                crate::gl!(gSpecialStatuses[gl_i81].specialDmg)
                            } != 0)
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[r#move]
                            .flags as i32
                            & FLAG_MAKES_CONTACT
                            != 0
                        && Random() as i32 % 10 == 0
                    {
                        loop {
                            crate::gl!(gBattleCommunication[3] = Random() as u8 & 3);
                            if crate::gl!(gBattleCommunication[3]) != 0 {
                                break;
                            }
                        }
                        if crate::gl!(gBattleCommunication[3]) == 3 {
                            crate::gl!(gBattleCommunication[3] += 2);
                        }
                        crate::gl!(gBattleCommunication[3] += MOVE_EFFECT_AFFECTS_USER);
                        BattleScriptPushCursor();
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_ApplySecondaryEffect
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        gHitMarker.set(gHitMarker.get() | (HITMARKER_STATUS_ABILITY_EFFECT));
                        effect += 1;
                    }
                }
                ABILITY_POISON_POINT => {
                    if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && ({
                            let gl_i157 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i157].hp)
                        }) != 0
                        && (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].confusionSelfDmg()
                            == 0
                        && ({
                            let gl_i82 = gBattlerTarget.get();
                            crate::gl!(gSpecialStatuses[gl_i82].physicalDmg)
                        } != 0
                            || {
                                let gl_i83 = gBattlerTarget.get();
                                crate::gl!(gSpecialStatuses[gl_i83].specialDmg)
                            } != 0)
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[r#move]
                            .flags as i32
                            & FLAG_MAKES_CONTACT
                            != 0
                        && Random() as i32 % 3 == 0
                    {
                        crate::gl!(gBattleCommunication[3] = 66);
                        BattleScriptPushCursor();
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_ApplySecondaryEffect
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        gHitMarker.set(gHitMarker.get() | (HITMARKER_STATUS_ABILITY_EFFECT));
                        effect += 1;
                    }
                }
                ABILITY_STATIC => {
                    if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && ({
                            let gl_i158 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i158].hp)
                        }) != 0
                        && (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].confusionSelfDmg()
                            == 0
                        && ({
                            let gl_i84 = gBattlerTarget.get();
                            crate::gl!(gSpecialStatuses[gl_i84].physicalDmg)
                        } != 0
                            || {
                                let gl_i85 = gBattlerTarget.get();
                                crate::gl!(gSpecialStatuses[gl_i85].specialDmg)
                            } != 0)
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[r#move]
                            .flags as i32
                            & FLAG_MAKES_CONTACT
                            != 0
                        && Random() as i32 % 3 == 0
                    {
                        crate::gl!(gBattleCommunication[3] = 69);
                        BattleScriptPushCursor();
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_ApplySecondaryEffect
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        gHitMarker.set(gHitMarker.get() | (HITMARKER_STATUS_ABILITY_EFFECT));
                        effect += 1;
                    }
                }
                ABILITY_FLAME_BODY => {
                    if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && ({
                            let gl_i159 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i159].hp)
                        }) != 0
                        && (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].confusionSelfDmg()
                            == 0
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[r#move]
                            .flags as i32
                            & FLAG_MAKES_CONTACT
                            != 0
                        && ({
                            let gl_i86 = gBattlerTarget.get();
                            crate::gl!(gSpecialStatuses[gl_i86].physicalDmg)
                        } != 0
                            || {
                                let gl_i87 = gBattlerTarget.get();
                                crate::gl!(gSpecialStatuses[gl_i87].specialDmg)
                            } != 0)
                        && Random() as i32 % 3 == 0
                    {
                        crate::gl!(gBattleCommunication[3] = 67);
                        BattleScriptPushCursor();
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_ApplySecondaryEffect
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        gHitMarker.set(gHitMarker.get() | (HITMARKER_STATUS_ABILITY_EFFECT));
                        effect += 1;
                    }
                }
                ABILITY_CUTE_CHARM
                    if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && ({
                            let gl_i160 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i160].hp)
                        }) != 0
                        && (*gProtectStructs.as_ptr())[gBattlerAttacker.get()]
                            .confusionSelfDmg()
                            == 0
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[r#move]
                            .flags as i32
                            & FLAG_MAKES_CONTACT
                            != 0
                        && ({
                            let gl_i88 = gBattlerTarget.get();
                            crate::gl!(gSpecialStatuses[gl_i88].physicalDmg)
                        } != 0
                            || {
                                let gl_i89 = gBattlerTarget.get();
                                crate::gl!(gSpecialStatuses[gl_i89].specialDmg)
                            } != 0)
                        && ({
                            let gl_i161 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i161].hp)
                        }) != 0
                        && Random() as i32 % 3 == 0
                        && ({
                            let gl_i162 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i162].ability)
                        }) != ABILITY_OBLIVIOUS
                        && GetGenderFromSpeciesAndPersonality(speciesAtk, pidAtk)
                            != GetGenderFromSpeciesAndPersonality(speciesDef, pidDef)
                        && ({
                            let gl_i163 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i163].status2)
                        }) & STATUS2_INFATUATION
                            == 0
                        && GetGenderFromSpeciesAndPersonality(speciesAtk, pidAtk)
                            != MON_GENDERLESS
                        && GetGenderFromSpeciesAndPersonality(speciesDef, pidDef)
                            != MON_GENDERLESS =>
                {
                    {
                        let gl_i164 = gBattlerAttacker.get();
                        crate::gl!(
                            gBattleMons[gl_i164].status2 |= gBitTable[gBattlerTarget.get()] << 16
                        )
                    };
                    BattleScriptPushCursor();
                    crate::gl!(
                        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_CuteCharmActivates
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut()
                    );
                    effect += 1;
                }
                _ => {}
            },
            5 => {
                battler = 0;
                while battler < gBattlersCount.get() {
                    match crate::gl!(gBattleMons[battler].ability) {
                        ABILITY_IMMUNITY => {
                            if crate::gl!(gBattleMons[battler].status1) & 3976 != 0 {
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_PoisonJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                effect = 1;
                            }
                        }
                        ABILITY_OWN_TEMPO => {
                            if crate::gl!(gBattleMons[battler].status2) & STATUS2_CONFUSION != 0 {
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_ConfusionJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                effect = 2;
                            }
                        }
                        ABILITY_LIMBER => {
                            if crate::gl!(gBattleMons[battler].status1) & STATUS1_PARALYSIS != 0 {
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_ParalysisJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                effect = 1;
                            }
                        }
                        ABILITY_INSOMNIA | ABILITY_VITAL_SPIRIT => {
                            if crate::gl!(gBattleMons[battler].status1) & STATUS1_SLEEP != 0 {
                                crate::gl!(gBattleMons[battler].status2 &= 0xf7ffffff);
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_SleepJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                effect = 1;
                            }
                        }
                        ABILITY_WATER_VEIL => {
                            if crate::gl!(gBattleMons[battler].status1) & STATUS1_BURN != 0 {
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_BurnJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                effect = 1;
                            }
                        }
                        ABILITY_MAGMA_ARMOR => {
                            if crate::gl!(gBattleMons[battler].status1) & STATUS1_FREEZE != 0 {
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_IceJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                effect = 1;
                            }
                        }
                        ABILITY_OBLIVIOUS
                            if crate::gl!(gBattleMons[battler].status2) & STATUS2_INFATUATION
                                != 0 =>
                        {
                            StringCopy(
                                (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                (*(&raw const crate::data::battle_main::gStatusConditionString_LoveJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                            );
                            effect = 3;
                        }
                        _ => {}
                    }
                    if effect != 0 {
                        match effect {
                            1 => {
                                crate::gl!(gBattleMons[battler].status1 = 0);
                            }
                            2 => {
                                crate::gl!(gBattleMons[battler].status2 &= 0xfffffff8);
                            }
                            3 => {
                                crate::gl!(gBattleMons[battler].status2 &= 0xfff0ffff);
                            }
                            _ => {}
                        }
                        BattleScriptPushCursor();
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_AbilityCuredStatus
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        crate::gl!(gBattleScripting.battler = battler);
                        gActiveBattler.set(battler);
                        BtlController_EmitSetMonData(
                            B_COMM_TO_CONTROLLER,
                            REQUEST_STATUS_BATTLE,
                            0,
                            4,
                            ({
                                let gl_i165 = gActiveBattler.get();
                                crate::gl!(&raw gBattleMons[gl_i165].status1)
                            }) as *mut c_void,
                        );
                        MarkBattlerForControllerExec(gActiveBattler.get());
                        return effect;
                    }
                    battler += 1;
                }
            }
            6 => {
                battler = 0;
                while battler < gBattlersCount.get() {
                    if crate::gl!(gBattleMons[battler].ability) == ABILITY_FORECAST {
                        effect = CastformDataTypeChange(battler);
                        if effect != 0 {
                            BattleScriptPushCursorAndCallback(
                                (*crate::asmdata::BattleScript_CastformChange
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            crate::gl!(gBattleScripting.battler = battler);
                            (*crate::gl!(gBattleStruct)).formToChangeInto = effect - 1;
                            return effect;
                        }
                    }
                    battler += 1;
                }
            }
            7 => {
                if gLastUsedAbility.get() == ABILITY_SYNCHRONIZE
                    && gHitMarker.get() & HITMARKER_SYNCHRONIZE_EFFECT != 0
                {
                    gHitMarker.set(gHitMarker.get() & (0xffffbfff));
                    (*crate::gl!(gBattleStruct)).synchronizeMoveEffect &= 63;
                    if (*crate::gl!(gBattleStruct)).synchronizeMoveEffect == MOVE_EFFECT_TOXIC {
                        (*crate::gl!(gBattleStruct)).synchronizeMoveEffect = MOVE_EFFECT_POISON;
                    }
                    crate::gl!(
                        gBattleCommunication[3] = (*crate::gl!(gBattleStruct))
                            .synchronizeMoveEffect
                            + MOVE_EFFECT_AFFECTS_USER
                    );
                    crate::gl!(gBattleScripting.battler = gBattlerTarget.get());
                    BattleScriptPushCursor();
                    crate::gl!(
                        gBattlescriptCurrInstr =
                            (*crate::asmdata::BattleScript_SynchronizeActivates
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut()
                    );
                    gHitMarker.set(gHitMarker.get() | (HITMARKER_STATUS_ABILITY_EFFECT));
                    effect += 1;
                }
            }
            8 => {
                if gLastUsedAbility.get() == ABILITY_SYNCHRONIZE
                    && gHitMarker.get() & HITMARKER_SYNCHRONIZE_EFFECT != 0
                {
                    gHitMarker.set(gHitMarker.get() & (0xffffbfff));
                    (*crate::gl!(gBattleStruct)).synchronizeMoveEffect &= 63;
                    if (*crate::gl!(gBattleStruct)).synchronizeMoveEffect == MOVE_EFFECT_TOXIC {
                        (*crate::gl!(gBattleStruct)).synchronizeMoveEffect = MOVE_EFFECT_POISON;
                    }
                    crate::gl!(
                        gBattleCommunication[3] =
                            (*crate::gl!(gBattleStruct)).synchronizeMoveEffect
                    );
                    crate::gl!(gBattleScripting.battler = gBattlerAttacker.get());
                    BattleScriptPushCursor();
                    crate::gl!(
                        gBattlescriptCurrInstr =
                            (*crate::asmdata::BattleScript_SynchronizeActivates
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut()
                    );
                    gHitMarker.set(gHitMarker.get() | (HITMARKER_STATUS_ABILITY_EFFECT));
                    effect += 1;
                }
            }
            9 => {
                i = 0;
                while i < gBattlersCount.get() as i32 {
                    if crate::gl!(gBattleMons[i].ability) == ABILITY_INTIMIDATE
                        && crate::gl!(gStatuses3[i]) & STATUS3_INTIMIDATE_POKES != 0
                    {
                        gLastUsedAbility.set(ABILITY_INTIMIDATE);
                        crate::gl!(gStatuses3[i] &= 0xfff7ffff);
                        BattleScriptPushCursorAndCallback(
                            (*crate::asmdata::BattleScript_IntimidateActivatesEnd3
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        );
                        (*crate::gl!(gBattleStruct)).intimidateBattler = i as u8;
                        effect += 1;
                        break;
                    }
                    i += 1;
                }
            }
            11 => {
                i = 0;
                while i < gBattlersCount.get() as i32 {
                    if crate::gl!(gBattleMons[i].ability) == ABILITY_TRACE
                        && crate::gl!(gStatuses3[i]) & STATUS3_TRACE != 0
                    {
                        side = (GetBattlerPosition(i as u8) ^ BIT_SIDE) & BIT_SIDE;
                        target1 = GetBattlerAtPosition(side);
                        let target2: u8 = GetBattlerAtPosition(side + BIT_FLANK);
                        if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
                            if crate::gl!(gBattleMons[target1].ability) != 0
                                && crate::gl!(gBattleMons[target1].hp) != 0
                                && crate::gl!(gBattleMons[target2].ability) != 0
                                && crate::gl!(gBattleMons[target2].hp) != 0
                            {
                                gActiveBattler
                                    .set(GetBattlerAtPosition(((Random() as u8 & 1) * 2) | side));
                                crate::gl!(
                                    gBattleMons[i].ability = ({
                                        let gl_i166 = gActiveBattler.get();
                                        crate::gl!(gBattleMons[gl_i166].ability)
                                    })
                                );
                                gLastUsedAbility.set({
                                    let gl_i167 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i167].ability)
                                });
                                effect += 1;
                            } else if crate::gl!(gBattleMons[target1].ability) != 0
                                && crate::gl!(gBattleMons[target1].hp) != 0
                            {
                                gActiveBattler.set(target1);
                                crate::gl!(
                                    gBattleMons[i].ability = ({
                                        let gl_i168 = gActiveBattler.get();
                                        crate::gl!(gBattleMons[gl_i168].ability)
                                    })
                                );
                                gLastUsedAbility.set({
                                    let gl_i169 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i169].ability)
                                });
                                effect += 1;
                            } else if crate::gl!(gBattleMons[target2].ability) != 0
                                && crate::gl!(gBattleMons[target2].hp) != 0
                            {
                                gActiveBattler.set(target2);
                                crate::gl!(
                                    gBattleMons[i].ability = ({
                                        let gl_i170 = gActiveBattler.get();
                                        crate::gl!(gBattleMons[gl_i170].ability)
                                    })
                                );
                                gLastUsedAbility.set({
                                    let gl_i171 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i171].ability)
                                });
                                effect += 1;
                            }
                        } else {
                            gActiveBattler.set(target1);
                            if crate::gl!(gBattleMons[target1].ability) != 0
                                && crate::gl!(gBattleMons[target1].hp) != 0
                            {
                                crate::gl!(
                                    gBattleMons[i].ability =
                                        crate::gl!(gBattleMons[target1].ability)
                                );
                                gLastUsedAbility.set(crate::gl!(gBattleMons[target1].ability));
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
                            crate::gl!(gStatuses3[i] &= 0xffefffff);
                            crate::gl!(gBattleScripting.battler = i as u8);
                            crate::gl!(gBattleTextBuff1[0] = 0xFD);
                            crate::gl!(gBattleTextBuff1[1] = 4);
                            crate::gl!(gBattleTextBuff1[2] = gActiveBattler.get());
                            crate::gl!(
                                gBattleTextBuff1[3] = ({
                                    let gl_i172 = gActiveBattler.get();
                                    crate::gl!(gBattlerPartyIndexes[gl_i172])
                                }) as u8
                            );
                            crate::gl!(gBattleTextBuff1[4] = 0xFF);
                            crate::gl!(gBattleTextBuff2[0] = 0xFD);
                            crate::gl!(gBattleTextBuff2[1] = 9);
                            crate::gl!(gBattleTextBuff2[2] = gLastUsedAbility.get());
                            crate::gl!(gBattleTextBuff2[3] = 0xFF);
                            break;
                        }
                    }
                    i += 1;
                }
            }
            10 => {
                i = 0;
                while i < gBattlersCount.get() as i32 {
                    if crate::gl!(gBattleMons[i].ability) == ABILITY_INTIMIDATE
                        && crate::gl!(gStatuses3[i]) & STATUS3_INTIMIDATE_POKES != 0
                    {
                        gLastUsedAbility.set(ABILITY_INTIMIDATE);
                        crate::gl!(gStatuses3[i] &= 0xfff7ffff);
                        BattleScriptPushCursor();
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_IntimidateActivates
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        (*crate::gl!(gBattleStruct)).intimidateBattler = i as u8;
                        effect += 1;
                        break;
                    }
                    i += 1;
                }
            }
            12 => {
                side = GetBattlerSide(battler);
                i = 0;
                while i < gBattlersCount.get() as i32 {
                    if GetBattlerSide(i as u8) != side
                        && crate::gl!(gBattleMons[i].ability) == ability
                    {
                        gLastUsedAbility.set(ability);
                        effect = i as u8 + 1;
                    }
                    i += 1;
                }
            }
            13 => {
                side = GetBattlerSide(battler);
                i = 0;
                while i < gBattlersCount.get() as i32 {
                    if GetBattlerSide(i as u8) == side
                        && crate::gl!(gBattleMons[i].ability) == ability
                    {
                        gLastUsedAbility.set(ability);
                        effect = i as u8 + 1;
                    }
                    i += 1;
                }
            }
            14 => match gLastUsedAbility.get() {
                ABILITYEFFECT_MUD_SPORT => {
                    for i in 0..(gBattlersCount.get() as i32) {
                        if crate::gl!(gStatuses3[i]) & STATUS3_MUDSPORT != 0 {
                            effect = i as u8 + 1;
                        }
                    }
                }
                ABILITYEFFECT_WATER_SPORT => {
                    for i in 0..(gBattlersCount.get() as i32) {
                        if crate::gl!(gStatuses3[i]) & STATUS3_WATERSPORT != 0 {
                            effect = i as u8 + 1;
                        }
                    }
                }
                _ => {
                    for i in 0..(gBattlersCount.get() as i32) {
                        if crate::gl!(gBattleMons[i].ability) == ability {
                            gLastUsedAbility.set(ability);
                            effect = i as u8 + 1;
                        }
                    }
                }
            },
            19 => {
                for i in 0..(gBattlersCount.get() as i32) {
                    if crate::gl!(gBattleMons[i].ability) == ability
                        && crate::gl!(gBattleMons[i].hp) != 0
                    {
                        gLastUsedAbility.set(ability);
                        effect = i as u8 + 1;
                    }
                }
            }
            15 => {
                for i in 0..(gBattlersCount.get() as i32) {
                    if crate::gl!(gBattleMons[i].ability) == ability && i != battler as i32 {
                        gLastUsedAbility.set(ability);
                        effect = i as u8 + 1;
                    }
                }
            }
            16 => {
                side = GetBattlerSide(battler);
                i = 0;
                while i < gBattlersCount.get() as i32 {
                    if GetBattlerSide(i as u8) != side
                        && crate::gl!(gBattleMons[i].ability) == ability
                    {
                        gLastUsedAbility.set(ability);
                        effect += 1;
                    }
                    i += 1;
                }
            }
            17 => {
                side = GetBattlerSide(battler);
                i = 0;
                while i < gBattlersCount.get() as i32 {
                    if GetBattlerSide(i as u8) == side
                        && crate::gl!(gBattleMons[i].ability) == ability
                    {
                        gLastUsedAbility.set(ability);
                        effect += 1;
                    }
                    i += 1;
                }
            }
            18 => {
                for i in 0..(gBattlersCount.get() as i32) {
                    if crate::gl!(gBattleMons[i].ability) == ability && i != battler as i32 {
                        gLastUsedAbility.set(ability);
                        effect += 1;
                    }
                }
            }
            _ => {}
        }
        if effect != 0 && caseID < ABILITYEFFECT_CHECK_OTHER_SIDE && gLastUsedAbility.get() != 0xFF
        {
            RecordAbilityBattle(battler, gLastUsedAbility.get());
        }
    }
    effect
}
pub unsafe fn BattleScriptExecute(BS_ptr: *mut u8) {
    crate::gl!(gBattlescriptCurrInstr = BS_ptr);
    (*(*crate::gl!(gBattleResources)).battleCallbackStack).function[{
        let t1 = (*(*crate::gl!(gBattleResources)).battleCallbackStack).size;
        (*(*crate::gl!(gBattleResources)).battleCallbackStack).size += 1;
        t1
    }] = crate::gl!(gBattleMainFunc);
    crate::gl!(gBattleMainFunc = Some(RunBattleScriptCommands_PopCallbacksStack));
    gCurrentActionFuncId.set(0);
}
pub unsafe fn BattleScriptPushCursorAndCallback(BS_ptr: *mut u8) {
    BattleScriptPushCursor();
    crate::gl!(gBattlescriptCurrInstr = BS_ptr);
    (*(*crate::gl!(gBattleResources)).battleCallbackStack).function[{
        let t1 = (*(*crate::gl!(gBattleResources)).battleCallbackStack).size;
        (*(*crate::gl!(gBattleResources)).battleCallbackStack).size += 1;
        t1
    }] = crate::gl!(gBattleMainFunc);
    crate::gl!(gBattleMainFunc = Some(RunBattleScriptCommands));
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
    gLastUsedItem.set(crate::gl!(gBattleMons[battler].item));
    if gLastUsedItem.get() == ITEM_ENIGMA_BERRY {
        battlerHoldEffect = crate::gl!(gEnigmaBerries[battler].holdEffect);
        battlerHoldEffectParam = crate::gl!(gEnigmaBerries[battler].holdEffectParam);
    } else {
        battlerHoldEffect = GetItemHoldEffect(gLastUsedItem.get());
        battlerHoldEffectParam = GetItemHoldEffectParam(gLastUsedItem.get());
    }
    let atkItem: u16 = {
        let gl_i173 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i173].item)
    };
    if atkItem == ITEM_ENIGMA_BERRY {
        atkHoldEffect = {
            let gl_i90 = gBattlerAttacker.get();
            crate::gl!(gEnigmaBerries[gl_i90].holdEffect)
        };
        atkHoldEffectParam = {
            let gl_i91 = gBattlerAttacker.get();
            crate::gl!(gEnigmaBerries[gl_i91].holdEffectParam)
        };
    } else {
        atkHoldEffect = GetItemHoldEffect(atkItem);
        atkHoldEffectParam = GetItemHoldEffectParam(atkItem);
    }
    let defItem: u16 = {
        let gl_i174 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i174].item)
    };
    if defItem == ITEM_ENIGMA_BERRY {
        defHoldEffect = {
            let gl_i92 = gBattlerTarget.get();
            crate::gl!(gEnigmaBerries[gl_i92].holdEffect)
        };
        defHoldEffectParam = {
            let gl_i93 = gBattlerTarget.get();
            crate::gl!(gEnigmaBerries[gl_i93].holdEffectParam)
        };
    } else {
        defHoldEffect = GetItemHoldEffect(defItem);
        defHoldEffectParam = GetItemHoldEffectParam(defItem);
    }
    match caseID {
        ITEMEFFECT_ON_SWITCH_IN => match battlerHoldEffect {
            HOLD_EFFECT_DOUBLE_PRIZE => {
                if GetBattlerSide(battler) == B_SIDE_PLAYER {
                    (*crate::gl!(gBattleStruct)).moneyMultiplier = 2;
                }
            }
            HOLD_EFFECT_RESTORE_STATS => {
                for i in 0..NUM_BATTLE_STATS {
                    if crate::gl!(gBattleMons[battler].statStages[i]) < DEFAULT_STAT_STAGE {
                        crate::gl!(gBattleMons[battler].statStages[i] = DEFAULT_STAT_STAGE);
                        effect = ITEM_STATS_CHANGE;
                    }
                }
                if effect != 0 {
                    crate::gl!(gBattleScripting.battler = battler);
                    gPotentialItemEffectBattler.set(battler);
                    gActiveBattler.set({
                        gBattlerAttacker.set(battler);
                        gBattlerAttacker.get()
                    });
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
            if crate::gl!(gBattleMons[battler].hp) != 0 {
                match battlerHoldEffect {
                    HOLD_EFFECT_RESTORE_HP => {
                        if crate::gl!(gBattleMons[battler].hp) as i32
                            <= crate::gl!(gBattleMons[battler].maxHP) as i32 / 2
                            && moveTurn == 0
                        {
                            gBattleMoveDamage.set(battlerHoldEffectParam as i32);
                            if crate::gl!(gBattleMons[battler].hp) as i32
                                + battlerHoldEffectParam as i32
                                > crate::gl!(gBattleMons[battler].maxHP) as i32
                            {
                                gBattleMoveDamage.set(
                                    crate::gl!(gBattleMons[battler].maxHP) as i32
                                        - crate::gl!(gBattleMons[battler].hp) as i32,
                                );
                            }
                            gBattleMoveDamage.set(-gBattleMoveDamage.get());
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
                                mon = &raw mut (*gPlayerParty.as_ptr())
                                    [crate::gl!(gBattlerPartyIndexes[battler])];
                            } else {
                                mon = &raw mut (*gEnemyParty.as_ptr())
                                    [crate::gl!(gBattlerPartyIndexes[battler])];
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
                                crate::gl!(gBattleTextBuff1[0] = 0xFD);
                                crate::gl!(gBattleTextBuff1[1] = 2);
                                crate::gl!(gBattleTextBuff1[2] = r#move as u8);
                                crate::gl!(
                                    gBattleTextBuff1[3] = ((r#move as i32 & 0xFF00) >> 8) as u8
                                );
                                crate::gl!(gBattleTextBuff1[4] = 0xFF);
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
                                MarkBattlerForControllerExec(gActiveBattler.get());
                                effect = ITEM_PP_CHANGE;
                            }
                        }
                    }
                    HOLD_EFFECT_RESTORE_STATS => {
                        for i in 0..NUM_BATTLE_STATS {
                            if crate::gl!(gBattleMons[battler].statStages[i]) < DEFAULT_STAT_STAGE {
                                crate::gl!(gBattleMons[battler].statStages[i] = DEFAULT_STAT_STAGE);
                                effect = ITEM_STATS_CHANGE;
                            }
                        }
                        if effect != 0 {
                            crate::gl!(gBattleScripting.battler = battler);
                            gPotentialItemEffectBattler.set(battler);
                            gActiveBattler.set({
                                gBattlerAttacker.set(battler);
                                gBattlerAttacker.get()
                            });
                            BattleScriptExecute(
                                (*crate::asmdata::BattleScript_WhiteHerbEnd2
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                        }
                    }
                    HOLD_EFFECT_LEFTOVERS => {
                        if crate::gl!(gBattleMons[battler].hp)
                            < crate::gl!(gBattleMons[battler].maxHP)
                            && moveTurn == 0
                        {
                            gBattleMoveDamage
                                .set(crate::gl!(gBattleMons[battler].maxHP) as i32 / 16);
                            if gBattleMoveDamage.get() == 0 {
                                gBattleMoveDamage.set(1);
                            }
                            if crate::gl!(gBattleMons[battler].hp) as i32 + gBattleMoveDamage.get()
                                > crate::gl!(gBattleMons[battler].maxHP) as i32
                            {
                                gBattleMoveDamage.set(
                                    crate::gl!(gBattleMons[battler].maxHP) as i32
                                        - crate::gl!(gBattleMons[battler].hp) as i32,
                                );
                            }
                            gBattleMoveDamage.set(-gBattleMoveDamage.get());
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
                        if crate::gl!(gBattleMons[battler].hp) as i32
                            <= crate::gl!(gBattleMons[battler].maxHP) as i32 / 2
                            && moveTurn == 0
                        {
                            crate::gl!(gBattleTextBuff1[0] = 0xFD);
                            crate::gl!(gBattleTextBuff1[1] = 8);
                            crate::gl!(gBattleTextBuff1[2] = FLAVOR_SPICY);
                            crate::gl!(gBattleTextBuff1[3] = 0xFF);
                            gBattleMoveDamage.set(div_i32(
                                crate::gl!(gBattleMons[battler].maxHP) as i32,
                                battlerHoldEffectParam as i32,
                            ));
                            if gBattleMoveDamage.get() == FLAVOR_SPICY as i32 {
                                gBattleMoveDamage.set(1);
                            }
                            if crate::gl!(gBattleMons[battler].hp) as i32 + gBattleMoveDamage.get()
                                > crate::gl!(gBattleMons[battler].maxHP) as i32
                            {
                                gBattleMoveDamage.set(
                                    crate::gl!(gBattleMons[battler].maxHP) as i32
                                        - crate::gl!(gBattleMons[battler].hp) as i32,
                                );
                            }
                            gBattleMoveDamage.set(-gBattleMoveDamage.get());
                            if GetFlavorRelationByPersonality(
                                crate::gl!(gBattleMons[battler].personality),
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
                        if crate::gl!(gBattleMons[battler].hp) as i32
                            <= crate::gl!(gBattleMons[battler].maxHP) as i32 / 2
                            && moveTurn == 0
                        {
                            crate::gl!(gBattleTextBuff1[0] = 0xFD);
                            crate::gl!(gBattleTextBuff1[1] = 8);
                            crate::gl!(gBattleTextBuff1[2] = FLAVOR_DRY);
                            crate::gl!(gBattleTextBuff1[3] = 0xFF);
                            gBattleMoveDamage.set(div_i32(
                                crate::gl!(gBattleMons[battler].maxHP) as i32,
                                battlerHoldEffectParam as i32,
                            ));
                            if gBattleMoveDamage.get() == 0 {
                                gBattleMoveDamage.set(FLAVOR_DRY as i32);
                            }
                            if crate::gl!(gBattleMons[battler].hp) as i32 + gBattleMoveDamage.get()
                                > crate::gl!(gBattleMons[battler].maxHP) as i32
                            {
                                gBattleMoveDamage.set(
                                    crate::gl!(gBattleMons[battler].maxHP) as i32
                                        - crate::gl!(gBattleMons[battler].hp) as i32,
                                );
                            }
                            gBattleMoveDamage.set(-gBattleMoveDamage.get());
                            if GetFlavorRelationByPersonality(
                                crate::gl!(gBattleMons[battler].personality),
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
                        if crate::gl!(gBattleMons[battler].hp) as i32
                            <= crate::gl!(gBattleMons[battler].maxHP) as i32 / 2
                            && moveTurn == 0
                        {
                            crate::gl!(gBattleTextBuff1[0] = 0xFD);
                            crate::gl!(gBattleTextBuff1[1] = 8);
                            crate::gl!(gBattleTextBuff1[2] = FLAVOR_SWEET);
                            crate::gl!(gBattleTextBuff1[3] = 0xFF);
                            gBattleMoveDamage.set(div_i32(
                                crate::gl!(gBattleMons[battler].maxHP) as i32,
                                battlerHoldEffectParam as i32,
                            ));
                            if gBattleMoveDamage.get() == 0 {
                                gBattleMoveDamage.set(1);
                            }
                            if crate::gl!(gBattleMons[battler].hp) as i32 + gBattleMoveDamage.get()
                                > crate::gl!(gBattleMons[battler].maxHP) as i32
                            {
                                gBattleMoveDamage.set(
                                    crate::gl!(gBattleMons[battler].maxHP) as i32
                                        - crate::gl!(gBattleMons[battler].hp) as i32,
                                );
                            }
                            gBattleMoveDamage.set(-gBattleMoveDamage.get());
                            if GetFlavorRelationByPersonality(
                                crate::gl!(gBattleMons[battler].personality),
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
                        if crate::gl!(gBattleMons[battler].hp) as i32
                            <= crate::gl!(gBattleMons[battler].maxHP) as i32 / 2
                            && moveTurn == 0
                        {
                            crate::gl!(gBattleTextBuff1[0] = 0xFD);
                            crate::gl!(gBattleTextBuff1[1] = 8);
                            crate::gl!(gBattleTextBuff1[2] = FLAVOR_BITTER);
                            crate::gl!(gBattleTextBuff1[3] = 0xFF);
                            gBattleMoveDamage.set(div_i32(
                                crate::gl!(gBattleMons[battler].maxHP) as i32,
                                battlerHoldEffectParam as i32,
                            ));
                            if gBattleMoveDamage.get() == 0 {
                                gBattleMoveDamage.set(1);
                            }
                            if crate::gl!(gBattleMons[battler].hp) as i32 + gBattleMoveDamage.get()
                                > crate::gl!(gBattleMons[battler].maxHP) as i32
                            {
                                gBattleMoveDamage.set(
                                    crate::gl!(gBattleMons[battler].maxHP) as i32
                                        - crate::gl!(gBattleMons[battler].hp) as i32,
                                );
                            }
                            gBattleMoveDamage.set(-gBattleMoveDamage.get());
                            if GetFlavorRelationByPersonality(
                                crate::gl!(gBattleMons[battler].personality),
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
                        if crate::gl!(gBattleMons[battler].hp) as i32
                            <= crate::gl!(gBattleMons[battler].maxHP) as i32 / 2
                            && moveTurn == 0
                        {
                            crate::gl!(gBattleTextBuff1[0] = 0xFD);
                            crate::gl!(gBattleTextBuff1[1] = 8);
                            crate::gl!(gBattleTextBuff1[2] = FLAVOR_SOUR);
                            crate::gl!(gBattleTextBuff1[3] = 0xFF);
                            gBattleMoveDamage.set(div_i32(
                                crate::gl!(gBattleMons[battler].maxHP) as i32,
                                battlerHoldEffectParam as i32,
                            ));
                            if gBattleMoveDamage.get() == 0 {
                                gBattleMoveDamage.set(1);
                            }
                            if crate::gl!(gBattleMons[battler].hp) as i32 + gBattleMoveDamage.get()
                                > crate::gl!(gBattleMons[battler].maxHP) as i32
                            {
                                gBattleMoveDamage.set(
                                    crate::gl!(gBattleMons[battler].maxHP) as i32
                                        - crate::gl!(gBattleMons[battler].hp) as i32,
                                );
                            }
                            gBattleMoveDamage.set(-gBattleMoveDamage.get());
                            if GetFlavorRelationByPersonality(
                                crate::gl!(gBattleMons[battler].personality),
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
                        if crate::gl!(gBattleMons[battler].hp) as i32
                            <= div_i32(
                                crate::gl!(gBattleMons[battler].maxHP) as i32,
                                battlerHoldEffectParam as i32,
                            )
                            && moveTurn == 0
                            && crate::gl!(gBattleMons[battler].statStages[1]) < MAX_STAT_STAGE
                        {
                            crate::gl!(gBattleTextBuff1[0] = 0xFD);
                            crate::gl!(gBattleTextBuff1[1] = 5);
                            crate::gl!(gBattleTextBuff1[2] = STAT_ATK);
                            crate::gl!(gBattleTextBuff1[3] = 0xFF);
                            crate::gl!(gBattleTextBuff2[0] = 0xFD);
                            crate::gl!(gBattleTextBuff2[1] = 0);
                            crate::gl!(gBattleTextBuff2[2] = STRINGID_STATROSE);
                            crate::gl!(gBattleTextBuff2[3] = 0);
                            crate::gl!(gBattleTextBuff2[4] = 0xFF);
                            gEffectBattler.set(battler);
                            crate::gl!(gBattleScripting.statChanger = 17);
                            crate::gl!(gBattleScripting.animArg1 = 15);
                            crate::gl!(gBattleScripting.animArg2 = 0);
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
                        if crate::gl!(gBattleMons[battler].hp) as i32
                            <= div_i32(
                                crate::gl!(gBattleMons[battler].maxHP) as i32,
                                battlerHoldEffectParam as i32,
                            )
                            && moveTurn == 0
                            && crate::gl!(gBattleMons[battler].statStages[2]) < 12
                        {
                            crate::gl!(gBattleTextBuff1[0] = 0xFD);
                            crate::gl!(gBattleTextBuff1[1] = 5);
                            crate::gl!(gBattleTextBuff1[2] = STAT_DEF as u8);
                            crate::gl!(gBattleTextBuff1[3] = 0xFF);
                            gEffectBattler.set(battler);
                            crate::gl!(gBattleScripting.statChanger = 18);
                            crate::gl!(gBattleScripting.animArg1 = 16);
                            crate::gl!(gBattleScripting.animArg2 = 0);
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
                        if crate::gl!(gBattleMons[battler].hp) as i32
                            <= div_i32(
                                crate::gl!(gBattleMons[battler].maxHP) as i32,
                                battlerHoldEffectParam as i32,
                            )
                            && moveTurn == 0
                            && crate::gl!(gBattleMons[battler].statStages[3]) < 12
                        {
                            crate::gl!(gBattleTextBuff1[0] = 0xFD);
                            crate::gl!(gBattleTextBuff1[1] = 5);
                            crate::gl!(gBattleTextBuff1[2] = STAT_SPEED);
                            crate::gl!(gBattleTextBuff1[3] = 0xFF);
                            gEffectBattler.set(battler);
                            crate::gl!(gBattleScripting.statChanger = 19);
                            crate::gl!(gBattleScripting.animArg1 = 17);
                            crate::gl!(gBattleScripting.animArg2 = 0);
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
                        if crate::gl!(gBattleMons[battler].hp) as i32
                            <= div_i32(
                                crate::gl!(gBattleMons[battler].maxHP) as i32,
                                battlerHoldEffectParam as i32,
                            )
                            && moveTurn == 0
                            && crate::gl!(gBattleMons[battler].statStages[4]) < 12
                        {
                            crate::gl!(gBattleTextBuff1[0] = 0xFD);
                            crate::gl!(gBattleTextBuff1[1] = 5);
                            crate::gl!(gBattleTextBuff1[2] = STAT_SPATK);
                            crate::gl!(gBattleTextBuff1[3] = 0xFF);
                            gEffectBattler.set(battler);
                            crate::gl!(gBattleScripting.statChanger = 20);
                            crate::gl!(gBattleScripting.animArg1 = 18);
                            crate::gl!(gBattleScripting.animArg2 = 0);
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
                        if crate::gl!(gBattleMons[battler].hp) as i32
                            <= div_i32(
                                crate::gl!(gBattleMons[battler].maxHP) as i32,
                                battlerHoldEffectParam as i32,
                            )
                            && moveTurn == 0
                            && crate::gl!(gBattleMons[battler].statStages[5]) < 12
                        {
                            crate::gl!(gBattleTextBuff1[0] = 0xFD);
                            crate::gl!(gBattleTextBuff1[1] = STAT_SPDEF as u8);
                            crate::gl!(gBattleTextBuff1[2] = STAT_SPDEF as u8);
                            crate::gl!(gBattleTextBuff1[3] = 0xFF);
                            gEffectBattler.set(battler);
                            crate::gl!(gBattleScripting.statChanger = 21);
                            crate::gl!(gBattleScripting.animArg1 = 19);
                            crate::gl!(gBattleScripting.animArg2 = 0);
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
                        if crate::gl!(gBattleMons[battler].hp) as i32
                            <= div_i32(
                                crate::gl!(gBattleMons[battler].maxHP) as i32,
                                battlerHoldEffectParam as i32,
                            )
                            && moveTurn == 0
                            && crate::gl!(gBattleMons[battler].status2) & STATUS2_FOCUS_ENERGY == 0
                        {
                            crate::gl!(gBattleMons[battler].status2 |= STATUS2_FOCUS_ENERGY);
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
                            && crate::gl!(gBattleMons[battler].hp) as i32
                                <= div_i32(
                                    crate::gl!(gBattleMons[battler].maxHP) as i32,
                                    battlerHoldEffectParam as i32,
                                )
                        {
                            i = 0;
                            while i < 5 {
                                if ({
                                    let gl_i175 = STAT_ATK as i32 + i;
                                    crate::gl!(gBattleMons[battler].statStages[gl_i175])
                                }) < MAX_STAT_STAGE
                                {
                                    break;
                                }
                                i += 1;
                            }
                            if i != 5 {
                                loop {
                                    i = Random() as i32 % 5;
                                    if ({
                                        let gl_i176 = STAT_ATK as i32 + i;
                                        crate::gl!(gBattleMons[battler].statStages[gl_i176])
                                    }) != MAX_STAT_STAGE
                                    {
                                        break;
                                    }
                                }
                                crate::gl!(gBattleTextBuff1[0] = 0xFD);
                                crate::gl!(gBattleTextBuff1[1] = 5);
                                crate::gl!(gBattleTextBuff1[2] = i as u8 + 1);
                                crate::gl!(gBattleTextBuff1[3] = 0xFF);
                                crate::gl!(gBattleTextBuff2[0] = B_BUFF_PLACEHOLDER_BEGIN);
                                crate::gl!(gBattleTextBuff2[1] = B_BUFF_STRING);
                                crate::gl!(gBattleTextBuff2[2] = STRINGID_STATSHARPLY);
                                crate::gl!(gBattleTextBuff2[3] = 0);
                                crate::gl!(gBattleTextBuff2[4] = B_BUFF_STRING);
                                crate::gl!(gBattleTextBuff2[5] = STRINGID_STATROSE);
                                crate::gl!(gBattleTextBuff2[6] = 0);
                                crate::gl!(gBattleTextBuff2[7] = EOS);
                                gEffectBattler.set(battler);
                                crate::gl!(gBattleScripting.statChanger = i as u8 + 1 + 32 + FALSE);
                                crate::gl!(
                                    gBattleScripting.animArg1 =
                                        STAT_ANIM_PLUS2 as u8 + (i as u8 + 1)
                                );
                                crate::gl!(gBattleScripting.animArg2 = 0);
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
                        if crate::gl!(gBattleMons[battler].status1) & STATUS1_PARALYSIS != 0 {
                            crate::gl!(gBattleMons[battler].status1 &= 0xffffffbf);
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
                        if crate::gl!(gBattleMons[battler].status1) & STATUS1_PSN_ANY != 0 {
                            crate::gl!(gBattleMons[battler].status1 &= 0xfffff077);
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
                        if crate::gl!(gBattleMons[battler].status1) & STATUS1_BURN != 0 {
                            crate::gl!(gBattleMons[battler].status1 &= 0xffffffef);
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
                        if crate::gl!(gBattleMons[battler].status1) & STATUS1_FREEZE != 0 {
                            crate::gl!(gBattleMons[battler].status1 &= 0xffffffdf);
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
                        if crate::gl!(gBattleMons[battler].status1) & STATUS1_SLEEP != 0 {
                            crate::gl!(gBattleMons[battler].status1 &= 0xfffffff8);
                            crate::gl!(gBattleMons[battler].status2 &= 0xf7ffffff);
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
                        if crate::gl!(gBattleMons[battler].status2) & STATUS2_CONFUSION != 0 {
                            crate::gl!(gBattleMons[battler].status2 &= 0xfffffff8);
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
                        if crate::gl!(gBattleMons[battler].status1) & STATUS1_ANY != 0
                            || crate::gl!(gBattleMons[battler].status2) & STATUS2_CONFUSION != 0
                        {
                            i = 0;
                            if crate::gl!(gBattleMons[battler].status1) & STATUS1_PSN_ANY != 0 {
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_PoisonJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                i += 1;
                            }
                            if crate::gl!(gBattleMons[battler].status1) & STATUS1_SLEEP != 0 {
                                crate::gl!(gBattleMons[battler].status2 &= 0xf7ffffff);
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_SleepJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                i += 1;
                            }
                            if crate::gl!(gBattleMons[battler].status1) & STATUS1_PARALYSIS != 0 {
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_ParalysisJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                i += 1;
                            }
                            if crate::gl!(gBattleMons[battler].status1) & STATUS1_BURN != 0 {
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_BurnJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                i += 1;
                            }
                            if crate::gl!(gBattleMons[battler].status1) & STATUS1_FREEZE != 0 {
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_IceJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                i += 1;
                            }
                            if crate::gl!(gBattleMons[battler].status2) & STATUS2_CONFUSION != 0 {
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_ConfusionJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                                i += 1;
                            }
                            if i <= 1 {
                                crate::gl!(gBattleCommunication[5] = B_MSG_CURED_PROBLEM);
                            } else {
                                crate::gl!(gBattleCommunication[5] = B_MSG_NORMALIZED_STATUS);
                            }
                            crate::gl!(gBattleMons[battler].status1 = 0);
                            crate::gl!(gBattleMons[battler].status2 &= 0xfffffff8);
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
                        if crate::gl!(gBattleMons[battler].status2) & STATUS2_INFATUATION != 0 =>
                    {
                        crate::gl!(gBattleMons[battler].status2 &= 0xfff0ffff);
                        StringCopy(
                            (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                            (*(&raw const crate::data::battle_main::gStatusConditionString_LoveJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                        );
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_BerryCureChosenStatusEnd2
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        );
                        crate::gl!(gBattleCommunication[5] = B_MSG_CURED_PROBLEM);
                        effect = ITEM_EFFECT_OTHER;
                    }
                    _ => {}
                }
                if effect != 0 {
                    crate::gl!(gBattleScripting.battler = battler);
                    gPotentialItemEffectBattler.set(battler);
                    gActiveBattler.set({
                        gBattlerAttacker.set(battler);
                        gBattlerAttacker.get()
                    });
                    match effect {
                        ITEM_STATUS_CHANGE => {
                            BtlController_EmitSetMonData(
                                B_COMM_TO_CONTROLLER,
                                REQUEST_STATUS_BATTLE,
                                0,
                                4,
                                crate::gl!(&raw gBattleMons[battler].status1) as *mut c_void,
                            );
                            MarkBattlerForControllerExec(gActiveBattler.get());
                        }
                        ITEM_PP_CHANGE
                            if crate::gl!(gBattleMons[battler].status2) & 0x200000 == 0
                                && (*gDisableStructs.as_ptr())[battler].mimickedMoves() as u32
                                    & (*(&raw const crate::util::gBitTable)
                                        .cast::<CArray<u32, 0>>())[i]
                                    == 0 =>
                        {
                            crate::gl!(gBattleMons[battler].pp[i] = changedPP);
                        }
                        _ => {}
                    }
                }
            }
        }
        ITEMEFFECT_DUMMY => {}
        ITEMEFFECT_MOVE_END => {
            battler = 0;
            while battler < gBattlersCount.get() {
                gLastUsedItem.set(crate::gl!(gBattleMons[battler].item));
                if crate::gl!(gBattleMons[battler].item) == ITEM_ENIGMA_BERRY {
                    battlerHoldEffect = crate::gl!(gEnigmaBerries[battler].holdEffect);
                    battlerHoldEffectParam = crate::gl!(gEnigmaBerries[battler].holdEffectParam);
                } else {
                    battlerHoldEffect = GetItemHoldEffect(gLastUsedItem.get());
                    battlerHoldEffectParam = GetItemHoldEffectParam(gLastUsedItem.get());
                }
                match battlerHoldEffect {
                    HOLD_EFFECT_CURE_PAR => {
                        if crate::gl!(gBattleMons[battler].status1) & STATUS1_PARALYSIS != 0 {
                            crate::gl!(gBattleMons[battler].status1 &= 0xffffffbf);
                            BattleScriptPushCursor();
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_BerryCureParRet
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CURE_PSN => {
                        if crate::gl!(gBattleMons[battler].status1) & STATUS1_PSN_ANY != 0 {
                            crate::gl!(gBattleMons[battler].status1 &= 0xfffff077);
                            BattleScriptPushCursor();
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_BerryCurePsnRet
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CURE_BRN => {
                        if crate::gl!(gBattleMons[battler].status1) & STATUS1_BURN != 0 {
                            crate::gl!(gBattleMons[battler].status1 &= 0xffffffef);
                            BattleScriptPushCursor();
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_BerryCureBrnRet
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CURE_FRZ => {
                        if crate::gl!(gBattleMons[battler].status1) & STATUS1_FREEZE != 0 {
                            crate::gl!(gBattleMons[battler].status1 &= 0xffffffdf);
                            BattleScriptPushCursor();
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_BerryCureFrzRet
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CURE_SLP => {
                        if crate::gl!(gBattleMons[battler].status1) & STATUS1_SLEEP != 0 {
                            crate::gl!(gBattleMons[battler].status1 &= 0xfffffff8);
                            crate::gl!(gBattleMons[battler].status2 &= 0xf7ffffff);
                            BattleScriptPushCursor();
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_BerryCureSlpRet
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_CURE_CONFUSION => {
                        if crate::gl!(gBattleMons[battler].status2) & STATUS2_CONFUSION != 0 {
                            crate::gl!(gBattleMons[battler].status2 &= 0xfffffff8);
                            BattleScriptPushCursor();
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_BerryCureConfusionRet
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                            effect = ITEM_EFFECT_OTHER;
                        }
                    }
                    HOLD_EFFECT_CURE_ATTRACT => {
                        if crate::gl!(gBattleMons[battler].status2) & STATUS2_INFATUATION != 0 {
                            crate::gl!(gBattleMons[battler].status2 &= 0xfff0ffff);
                            StringCopy(
                                (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                (*(&raw const crate::data::battle_main::gStatusConditionString_LoveJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                            );
                            BattleScriptPushCursor();
                            crate::gl!(gBattleCommunication[5] = B_MSG_CURED_PROBLEM);
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_BerryCureChosenStatusRet
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                            effect = ITEM_EFFECT_OTHER;
                        }
                    }
                    HOLD_EFFECT_CURE_STATUS => {
                        if crate::gl!(gBattleMons[battler].status1) & STATUS1_ANY != 0
                            || crate::gl!(gBattleMons[battler].status2) & STATUS2_CONFUSION != 0
                        {
                            if crate::gl!(gBattleMons[battler].status1) & STATUS1_PSN_ANY != 0 {
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_PoisonJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                            }
                            if crate::gl!(gBattleMons[battler].status1) & STATUS1_SLEEP != 0 {
                                crate::gl!(gBattleMons[battler].status2 &= 0xf7ffffff);
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_SleepJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                            }
                            if crate::gl!(gBattleMons[battler].status1) & STATUS1_PARALYSIS != 0 {
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_ParalysisJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                            }
                            if crate::gl!(gBattleMons[battler].status1) & STATUS1_BURN != 0 {
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_BurnJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                            }
                            if crate::gl!(gBattleMons[battler].status1) & STATUS1_FREEZE != 0 {
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_IceJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                            }
                            if crate::gl!(gBattleMons[battler].status2) & STATUS2_CONFUSION != 0 {
                                StringCopy(
                                    (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
                                    (*(&raw const crate::data::battle_main::gStatusConditionString_ConfusionJpn).cast::<CArray<u8, 8>>()).as_ptr().cast_mut(),
                                );
                            }
                            crate::gl!(gBattleMons[battler].status1 = 0);
                            crate::gl!(gBattleMons[battler].status2 &= 0xfffffff8);
                            BattleScriptPushCursor();
                            crate::gl!(gBattleCommunication[5] = B_MSG_CURED_PROBLEM);
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_BerryCureChosenStatusRet
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                            effect = ITEM_STATUS_CHANGE;
                        }
                    }
                    HOLD_EFFECT_RESTORE_STATS => {
                        for i in 0..NUM_BATTLE_STATS {
                            if crate::gl!(gBattleMons[battler].statStages[i]) < DEFAULT_STAT_STAGE {
                                crate::gl!(gBattleMons[battler].statStages[i] = DEFAULT_STAT_STAGE);
                                effect = ITEM_STATS_CHANGE;
                            }
                        }
                        if effect != 0 {
                            crate::gl!(gBattleScripting.battler = battler);
                            gPotentialItemEffectBattler.set(battler);
                            BattleScriptPushCursor();
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_WhiteHerbRet
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                            return effect;
                        }
                    }
                    _ => {}
                }
                if effect != 0 {
                    crate::gl!(gBattleScripting.battler = battler);
                    gPotentialItemEffectBattler.set(battler);
                    gActiveBattler.set(battler);
                    BtlController_EmitSetMonData(
                        B_COMM_TO_CONTROLLER,
                        REQUEST_STATUS_BATTLE,
                        0,
                        4,
                        ({
                            let gl_i177 = gActiveBattler.get();
                            crate::gl!(&raw gBattleMons[gl_i177].status1)
                        }) as *mut c_void,
                    );
                    MarkBattlerForControllerExec(gActiveBattler.get());
                    break;
                }
                battler += 1;
            }
        }
        ITEMEFFECT_KINGSROCK_SHELLBELL if gBattleMoveDamage.get() != 0 => match atkHoldEffect {
            HOLD_EFFECT_FLINCH => {
                if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0
                    && ({
                        let gl_i94 = gBattlerTarget.get();
                        crate::gl!(gSpecialStatuses[gl_i94].physicalDmg)
                    } != 0
                        || {
                            let gl_i95 = gBattlerTarget.get();
                            crate::gl!(gSpecialStatuses[gl_i95].specialDmg)
                        } != 0)
                    && Random() as i32 % 100 < atkHoldEffectParam as i32
                    && (*(&raw const crate::data::pokemon::gBattleMoves)
                        .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
                    .flags as i32
                        & FLAG_KINGS_ROCK_AFFECTED
                        != 0
                    && ({
                        let gl_i178 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i178].hp)
                    }) != 0
                {
                    crate::gl!(gBattleCommunication[3] = MOVE_EFFECT_FLINCH);
                    BattleScriptPushCursor();
                    SetMoveEffect(0, 0);
                    BattleScriptPop();
                }
            }
            HOLD_EFFECT_SHELL_BELL
                if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0
                    && {
                        let gl_i96 = gBattlerTarget.get();
                        crate::gl!(gSpecialStatuses[gl_i96].shellBellDmg)
                    } != 0
                    && {
                        let gl_i97 = gBattlerTarget.get();
                        crate::gl!(gSpecialStatuses[gl_i97].shellBellDmg)
                    } != IGNORE_SHELL_BELL
                    && gBattlerAttacker.get() != gBattlerTarget.get()
                    && ({
                        let gl_i179 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i179].hp)
                    }) != ({
                        let gl_i180 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i180].maxHP)
                    })
                    && ({
                        let gl_i181 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i181].hp)
                    }) != 0 =>
            {
                gLastUsedItem.set(atkItem);
                gPotentialItemEffectBattler.set(gBattlerAttacker.get());
                crate::gl!(gBattleScripting.battler = gBattlerAttacker.get());
                gBattleMoveDamage.set(-div_i32(
                    {
                        let gl_i98 = gBattlerTarget.get();
                        crate::gl!(gSpecialStatuses[gl_i98].shellBellDmg)
                    },
                    atkHoldEffectParam as i32,
                ));
                if gBattleMoveDamage.get() == 0 {
                    gBattleMoveDamage.set(-1);
                }
                {
                    let gl_v = 0;
                    let gl_i99 = gBattlerTarget.get();
                    crate::gl!(gSpecialStatuses[gl_i99].shellBellDmg = gl_v)
                };
                BattleScriptPushCursor();
                crate::gl!(
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_ItemHealHP_Ret
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                );
                effect += 1;
            }
            _ => {}
        },
        _ => {}
    }
    effect
}
pub fn ClearFuryCutterDestinyBondGrudge(battler: u8) {
    crate::gl!(gDisableStructs[battler].furyCutterCounter = 0);
    crate::gl!(gBattleMons[battler].status2 &= 0xfdffffff);
    crate::gl!(gStatuses3[battler] &= 0xffffbfff);
}
pub unsafe fn HandleAction_RunBattleScript() {
    if gBattleControllerExecFlags.get() == 0 {
        (*(&raw const crate::data::battle_script_commands::gBattleScriptingCommandsTable)
            .cast::<CArray<Option<unsafe fn()>, 0>>())[*crate::gl!(gBattlescriptCurrInstr)]
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
            side = GetBattlerSide(gBattlerAttacker.get()) ^ 1;
            if crate::gl!(gSideTimers[side].followmeTimer) != 0
                && ({
                    let gl_i182 = crate::gl!(gSideTimers[side].followmeTarget);
                    crate::gl!(gBattleMons[gl_i182].hp)
                }) != 0
            {
                targetBattler = crate::gl!(gSideTimers[side].followmeTarget);
            } else {
                side = GetBattlerSide(gBattlerAttacker.get());
                loop {
                    targetBattler = rem_i32(Random() as i32, gBattlersCount.get() as i32) as u8;
                    if !(targetBattler == gBattlerAttacker.get()
                        || side == GetBattlerSide(targetBattler)
                        || gAbsentBattlerFlags.get() as u32
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
                        gBattlerAttacker.get(),
                        ABILITY_LIGHTNING_ROD,
                        0,
                        0,
                    ) != 0
                    && crate::gl!(gBattleMons[targetBattler].ability) != ABILITY_LIGHTNING_ROD
                {
                    targetBattler ^= BIT_FLANK;
                    RecordAbilityBattle(
                        targetBattler,
                        crate::gl!(gBattleMons[targetBattler].ability),
                    );
                    (*gSpecialStatuses.as_ptr())[targetBattler].set_lightningRodRedirected(1);
                }
            }
        }
        MOVE_TARGET_DEPENDS
        | MOVE_TARGET_BOTH
        | MOVE_TARGET_FOES_AND_ALLY
        | MOVE_TARGET_OPPONENTS_FIELD => {
            targetBattler =
                GetBattlerAtPosition(GetBattlerPosition(gBattlerAttacker.get()) & 1 ^ 1);
            if gAbsentBattlerFlags.get() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[targetBattler]
                != 0
            {
                targetBattler ^= BIT_FLANK;
            }
        }
        4 => {
            side = GetBattlerSide(gBattlerAttacker.get()) ^ 1;
            if crate::gl!(gSideTimers[side].followmeTimer) != 0
                && ({
                    let gl_i183 = crate::gl!(gSideTimers[side].followmeTarget);
                    crate::gl!(gBattleMons[gl_i183].hp)
                }) != 0
            {
                targetBattler = crate::gl!(gSideTimers[side].followmeTarget);
            } else if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0
                && moveTarget as i32 & MOVE_TARGET_RANDOM != 0
            {
                if GetBattlerSide(gBattlerAttacker.get()) == B_SIDE_PLAYER {
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
                if gAbsentBattlerFlags.get() as u32
                    & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[targetBattler]
                    != 0
                {
                    targetBattler ^= BIT_FLANK;
                }
            } else {
                targetBattler =
                    GetBattlerAtPosition(GetBattlerPosition(gBattlerAttacker.get()) & 1 ^ 1);
            }
        }
        MOVE_TARGET_USER_OR_SELECTED | MOVE_TARGET_USER => {
            targetBattler = gBattlerAttacker.get();
        }
        _ => {}
    }
    *(*crate::gl!(gBattleStruct))
        .moveTarget
        .as_mut_ptr()
        .at(gBattlerAttacker.get()) = targetBattler;
    targetBattler
}
unsafe fn IsBattlerModernFatefulEncounter(battler: u8) -> u32 {
    if GetBattlerSide(battler) == B_SIDE_OPPONENT {
        return TRUE as u32;
    }
    if GetMonData3(
        &raw mut (*gPlayerParty.as_ptr())[crate::gl!(gBattlerPartyIndexes[battler])],
        MON_DATA_SPECIES,
        null_mut(),
    ) != SPECIES_DEOXYS
        && GetMonData3(
            &raw mut (*gPlayerParty.as_ptr())[crate::gl!(gBattlerPartyIndexes[battler])],
            MON_DATA_SPECIES,
            null_mut(),
        ) != SPECIES_MEW
    {
        return TRUE as u32;
    }
    GetMonData3(
        &raw mut (*gPlayerParty.as_ptr())[crate::gl!(gBattlerPartyIndexes[battler])],
        MON_DATA_MODERN_FATEFUL_ENCOUNTER,
        null_mut(),
    )
}
pub unsafe fn IsMonDisobedient() -> u8 {
    let mut obedienceLevel: u8 = 0;
    if gBattleTypeFlags.get() & 0x2000002 != 0 {
        return DISOBEDIENCE_OBEDIENT;
    }
    if GetBattlerSide(gBattlerAttacker.get()) == B_SIDE_OPPONENT {
        return DISOBEDIENCE_OBEDIENT;
    }
    if IsBattlerModernFatefulEncounter(gBattlerAttacker.get()) != 0 {
        if gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER != 0
            && GetBattlerPosition(gBattlerAttacker.get()) == 2
        {
            return DISOBEDIENCE_OBEDIENT;
        }
        if gBattleTypeFlags.get() & BATTLE_TYPE_FRONTIER != 0 {
            return DISOBEDIENCE_OBEDIENT;
        }
        if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED != 0 {
            return DISOBEDIENCE_OBEDIENT;
        }
        if IsOtherTrainer(
            {
                let gl_i184 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i184].otId)
            },
            (*gBattleMons.as_ptr())[gBattlerAttacker.get()]
                .otName
                .as_mut_ptr(),
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
    if ({
        let gl_i185 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i185].level)
    }) <= obedienceLevel
    {
        return DISOBEDIENCE_OBEDIENT;
    }
    let mut rnd: i32 = Random() as i32 & 255;
    let mut calc: i32 = ((({
        let gl_i186 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i186].level)
    }) as i32
        + obedienceLevel as i32)
        * rnd)
        >> 8;
    if calc < obedienceLevel as i32 {
        return DISOBEDIENCE_OBEDIENT;
    }
    if gCurrentMove.get() == MOVE_RAGE {
        {
            let gl_i187 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i187].status2 &= 0xff7fffff)
        };
    }
    if ({
        let gl_i188 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i188].status1)
    }) & STATUS1_SLEEP
        != 0
        && (gCurrentMove.get() == MOVE_SNORE || gCurrentMove.get() == MOVE_SLEEP_TALK)
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_IgnoresWhileAsleep
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
        return DISOBEDIENCE_IGNORED;
    }
    rnd = Random() as i32 & 255;
    calc = ((({
        let gl_i189 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i189].level)
    }) as i32
        + obedienceLevel as i32)
        * rnd)
        >> 8;
    if calc < obedienceLevel as i32 {
        calc = CheckMoveLimitations(
            gBattlerAttacker.get(),
            gBitTable[gCurrMovePos.get()] as u8,
            MOVE_LIMITATIONS_ALL,
        ) as i32;
        if calc == ALL_MOVES_MASK as i32 {
            crate::gl!(
                gBattleCommunication[5] = (if 0 != 0 {
                    Random() as i32 % 4
                } else {
                    Random() as i32 & 3
                }) as u8
            );
            crate::gl!(
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_MoveUsedLoafingAround
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            );
            return DISOBEDIENCE_IGNORED;
        } else {
            loop {
                gCurrMovePos.set({
                    gChosenMovePos.set(
                        (if 0 != 0 {
                            Random() as i32 % 4
                        } else {
                            Random() as i32 & 3
                        }) as u8,
                    );
                    gChosenMovePos.get()
                });
                if gBitTable[gCurrMovePos.get()] & calc as u32 == 0 {
                    break;
                }
            }
            gCalledMove.set({
                let gl_i190 = gBattlerAttacker.get();
                let gl_i191 = gCurrMovePos.get();
                crate::gl!(gBattleMons[gl_i190].moves[gl_i191])
            });
            crate::gl!(
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_IgnoresAndUsesRandomMove
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            );
            gBattlerTarget.set(GetMoveTarget(gCalledMove.get(), NO_TARGET_OVERRIDE));
            gHitMarker.set(gHitMarker.get() | (HITMARKER_DISOBEDIENT_MOVE));
            return DISOBEDIENCE_OTHER;
        }
    } else {
        obedienceLevel = ({
            let gl_i192 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i192].level)
        }) - obedienceLevel;
        calc = Random() as i32 & 255;
        if calc < obedienceLevel as i32
            && ({
                let gl_i193 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i193].status1)
            }) & STATUS1_ANY
                == 0
            && ({
                let gl_i194 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i194].ability)
            }) != ABILITY_VITAL_SPIRIT
            && ({
                let gl_i195 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i195].ability)
            }) != ABILITY_INSOMNIA
        {
            let mut i: i32 = 0;
            while i < gBattlersCount.get() as i32 {
                if crate::gl!(gBattleMons[i].status2) & STATUS2_UPROAR != 0 {
                    break;
                }
                i += 1;
            }
            if i == gBattlersCount.get() as i32 {
                crate::gl!(
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_IgnoresAndFallsAsleep
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                );
                return DISOBEDIENCE_IGNORED;
            }
        }
        calc -= obedienceLevel as i32;
        if calc < obedienceLevel as i32 {
            gBattleMoveDamage.set(CalculateBaseDamage(
                {
                    let gl_i196 = gBattlerAttacker.get();
                    crate::gl!(&raw gBattleMons[gl_i196])
                },
                {
                    let gl_i197 = gBattlerAttacker.get();
                    crate::gl!(&raw gBattleMons[gl_i197])
                },
                MOVE_POUND,
                0,
                40,
                0,
                gBattlerAttacker.get(),
                gBattlerAttacker.get(),
            ));
            gBattlerTarget.set(gBattlerAttacker.get());
            crate::gl!(
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_IgnoresAndHitsItself
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            );
            gHitMarker.set(gHitMarker.get() | (HITMARKER_UNABLE_TO_USE_MOVE));
            return DISOBEDIENCE_OTHER;
        } else {
            crate::gl!(
                gBattleCommunication[5] = (if 0 != 0 {
                    Random() as i32 % 4
                } else {
                    Random() as i32 & 3
                }) as u8
            );
            crate::gl!(
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_MoveUsedLoafingAround
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            );
            return DISOBEDIENCE_IGNORED;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
