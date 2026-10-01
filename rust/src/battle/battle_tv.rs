//! Translated from `src/battle_tv.c` by tools/rustport/c2rs.py.
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
    unused_assignments
)]

use crate::battle_anim_mons::{GetBattlerPosition, GetBattlerSide};
use crate::battle_main::{
    gBattleMons, gBattleMoveDamage, gBattleScripting, gBattleStruct, gBattleTypeFlags,
    gBattlerAttacker, gBattlerTarget, gCurrentMove, gEffectBattler, gProtectStructs, gStatuses3,
};
use crate::battle_main::{
    gBattleTextBuff1, gBattleTextBuff2, gBattlerPartyIndexes, gMoveSelectionCursor, gSideStatuses,
};
use crate::battle_message::gBattleMsgDataPtr;
use crate::battle_script_commands::TypeCalc;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::pokemon::{
    CalculateBaseDamage, GetLinkTrainerFlankId, GetMonData3, GetOpposingLinkMultiBattlerId,
    gEnemyParty, gPlayerParty,
};
use crate::tv::{PutBattleUpdateOnTheAir, TryPutBattleSeminarOnAir};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): sVariableDmgMoves sPoints_MoveEffect sPoints_Effectiveness sPoints_SetUp sPoints_RainMoves sPoints_SunMoves sPoints_SandstormMoves sPoints_HailMoves sPoints_ElectricMoves sPoints_StatusDmg sPoints_Status sPoints_Spikes sPoints_WaterSport sPoints_MudSport sPoints_Reflect sPoints_LightScreen sPoints_Safeguard sPoints_Mist sPoints_BreakWall sPoints_CriticalHit sPoints_Faint sPoints_Flinched sPoints_StatIncrease1 sPoints_StatIncrease2 sPoints_StatDecreaseSelf sPoints_StatDecrease1 sPoints_StatDecrease2 sPoints_StatIncreaseNotSelf sPointsArray sSpecialBattleStrings

const FNT_BURN: u32 = 4;
const FNT_CONFUSION: u32 = 12;
const FNT_CURSE: u32 = 1;
const FNT_DESTINY_BOND: u32 = 11;
const FNT_DOOM_DESIRE: u32 = 9;
const FNT_EXPLOSION: u32 = 13;
const FNT_FUTURE_SIGHT: u32 = 8;
const FNT_LEECH_SEED: u32 = 2;
const FNT_NIGHTMARE: u32 = 5;
const FNT_NONE: u32 = 0;
const FNT_OTHER: u32 = 15;
const FNT_PERISH_SONG: u32 = 10;
const FNT_POISON: u32 = 3;
const FNT_RECOIL: u32 = 14;
const FNT_SPIKES: u32 = 7;
const FNT_WRAP: u32 = 6;
const PTS_BREAK_WALL: u8 = 17;
const PTS_CRITICAL_HIT: u8 = 18;
const PTS_EFFECTIVENESS: u8 = 1;
const PTS_ELECTRIC: u8 = 7;
const PTS_FAINT: u8 = 19;
const PTS_FAINT_SET_UP: u8 = 20;
const PTS_FLINCHED: u8 = 21;
const PTS_HAIL: u8 = 6;
const PTS_LIGHT_SCREEN: u8 = 14;
const PTS_MIST: u8 = 16;
const PTS_MOVE_EFFECT: u8 = 0;
const PTS_MUD_SPORT: u8 = 12;
const PTS_RAIN: u8 = 3;
const PTS_REFLECT: u8 = 13;
const PTS_SAFEGUARD: u8 = 15;
const PTS_SANDSTORM: u8 = 5;
const PTS_SET_UP: u8 = 2;
const PTS_SPIKES: u8 = 10;
const PTS_STATUS: u8 = 9;
const PTS_STATUS_DMG: u8 = 8;
const PTS_STAT_DECREASE_1: u8 = 25;
const PTS_STAT_DECREASE_2: u8 = 26;
const PTS_STAT_DECREASE_SELF: u8 = 24;
const PTS_STAT_INCREASE_1: u8 = 22;
const PTS_STAT_INCREASE_2: u8 = 23;
const PTS_STAT_INCREASE_NOT_SELF: u8 = 27;
const PTS_SUN: u8 = 4;
const PTS_WATER_SPORT: u8 = 11;
const TABLE_END: i32 = -1;

static sPointsArray: Table<CArray<*mut u16, 28>> =
    Table((&raw const crate::data::battle_tv::sPointsArray).cast());
static sSpecialBattleStrings: Table<CArray<u16, 18>> =
    Table((&raw const crate::data::battle_tv::sSpecialBattleStrings).cast());
static sVariableDmgMoves: Table<CArray<u16, 26>> =
    Table((&raw const crate::data::battle_tv::sVariableDmgMoves).cast());

pub unsafe fn BattleTv_SetDataBasedOnString(stringId: u16) {
    let mut atkMon: *mut Pokemon = null_mut();
    let mut defMon: *mut Pokemon = null_mut();
    if gBattleTypeFlags & BATTLE_TYPE_LINK == 0
        && stringId != STRINGID_ITDOESNTAFFECT as u16
        && stringId != STRINGID_NOTVERYEFFECTIVE
    {
        return;
    }
    let tvPtr: *mut BattleTv = &raw mut (*gBattleStruct).tv;
    let atkSide: u32 = GetBattlerSide(gBattlerAttacker) as u32;
    let defSide: u32 = GetBattlerSide(gBattlerTarget) as u32;
    let effSide: u32 = GetBattlerSide(gEffectBattler) as u32;
    let scriptingSide: u32 = GetBattlerSide((*gBattleMsgDataPtr).scrActive) as u32;
    if atkSide == B_SIDE_PLAYER as u32 {
        atkMon = &raw mut gPlayerParty[gBattlerPartyIndexes[gBattlerAttacker]];
    } else {
        atkMon = &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerAttacker]];
    }
    if defSide == B_SIDE_PLAYER as u32 {
        defMon = &raw mut gPlayerParty[gBattlerPartyIndexes[gBattlerTarget]];
    } else {
        defMon = &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerTarget]];
    }
    let moveSlot: u8 = GetBattlerMoveSlotId(gBattlerAttacker, (*gBattleMsgDataPtr).currentMove);
    if moveSlot >= MAX_MON_MOVES as u8
        && IsNotSpecialBattleString(stringId) != 0
        && stringId > BATTLESTRINGS_TABLE_START as u16
    {
        (*tvPtr).side[atkSide].set_faintCause(FNT_OTHER);
        return;
    }
    let perishCount: *mut u8 = gBattleTextBuff1.as_mut_ptr().at(4);
    let statStringId: *mut u16 = gBattleTextBuff2.as_mut_ptr().at(2) as *mut u16;
    let finishedMoveId: *mut u16 = gBattleTextBuff1.as_mut_ptr().at(2) as *mut u16;
    let atkFlank: u32 = (GetBattlerPosition(gBattlerAttacker) as i32 / 2) as u32;
    let defFlank: u32 = (GetBattlerPosition(gBattlerTarget) as i32 / 2) as u32;
    let effFlank: u32 = (GetBattlerPosition(gEffectBattler) as i32 / 2) as u32;
    'l1: {
        let sw1: u16 = stringId;
        let mut fall = false;
        if sw1 == 27 {
            AddMovePoints(PTS_EFFECTIVENESS, moveSlot as u16, 2, 0);
            if gBattleTypeFlags & BATTLE_TYPE_LINK == 0 {
                TrySetBattleSeminarShow();
            }
            break 'l1;
        }
        if sw1 == STRINGID_NOTVERYEFFECTIVE {
            AddMovePoints(PTS_EFFECTIVENESS, moveSlot as u16, 1, 0);
            if gBattleTypeFlags & BATTLE_TYPE_LINK == 0
                && GetMonData3(defMon, MON_DATA_HP, null_mut()) != 0
            {
                TrySetBattleSeminarShow();
            }
            break 'l1;
        }
        if sw1 == 222 {
            AddMovePoints(PTS_EFFECTIVENESS, moveSlot as u16, 0, 0);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNFORESAWATTACK {
            (*tvPtr).side[atkSide]
                .set_futureSightMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).side[atkSide].set_futureSightMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNCHOSEXASDESTINY {
            (*tvPtr).side[atkSide]
                .set_doomDesireMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).side[atkSide].set_doomDesireMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_FAINTINTHREE {
            (*tvPtr).side[atkSide]
                .set_perishSongMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).side[atkSide].set_perishSongMoveSlot(moveSlot as u32);
            (*tvPtr).side[atkSide].set_perishSong(1);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNPERISHCOUNTFELL {
            if *perishCount == 0 {
                (*tvPtr).side[atkSide].set_faintCause(FNT_PERISH_SONG);
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNWISHCAMETRUE {
            if (*tvPtr).side[defSide].wishMonId() != 0 {
                AddMovePoints(
                    PTS_SET_UP,
                    3,
                    defSide as u8,
                    ((*tvPtr).side[defSide].wishMonId() as u8 - 1) * 4
                        + (*tvPtr).side[defSide].wishMoveSlot() as u8,
                );
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNWANTSGRUDGE {
            (*tvPtr).side[atkSide]
                .set_grudgeMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).side[atkSide].set_grudgeMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNLOSTPPGRUDGE {
            if (*tvPtr).side[defSide].grudgeMonId() != 0 {
                AddMovePoints(
                    PTS_SET_UP,
                    4,
                    defSide as u8,
                    ((*tvPtr).side[defSide].grudgeMonId() as u8 - 1) * 4
                        + (*tvPtr).side[defSide].grudgeMoveSlot() as u8,
                );
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNTRYINGTOTAKEFOE {
            (*tvPtr).side[atkSide]
                .set_destinyBondMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).side[atkSide].set_destinyBondMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNTOOKFOE {
            if (*tvPtr).side[defSide].destinyBondMonId() != 0 {
                (*tvPtr).side[atkSide].set_faintCause(FNT_DESTINY_BOND);
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNPLANTEDROOTS {
            (*tvPtr).pos[atkSide][atkFlank]
                .set_ingrainMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).pos[atkSide][atkFlank].set_ingrainMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNABSORBEDNUTRIENTS {
            if (*tvPtr).pos[atkSide][atkFlank].ingrainMonId() != 0 {
                AddMovePoints(
                    PTS_SET_UP,
                    6,
                    atkSide as u8,
                    ((*tvPtr).pos[atkSide][atkFlank].ingrainMonId() as u8 - 1) * 4
                        + (*tvPtr).pos[atkSide][atkFlank].ingrainMoveSlot() as u8,
                );
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNANCHOREDITSELF {
            if (*tvPtr).pos[defSide][defFlank].ingrainMonId() != 0 {
                AddMovePoints(
                    PTS_SET_UP,
                    6,
                    defSide as u8,
                    ((*tvPtr).pos[defSide][defFlank].ingrainMonId() as u8 - 1) * 4
                        + (*tvPtr).pos[defSide][defFlank].ingrainMoveSlot() as u8,
                );
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNTRANSFORMEDINTO {
            (*gBattleStruct).anyMonHasTransformed = TRUE;
            break 'l1;
        }
        if sw1 == STRINGID_CRITICALHIT {
            AddMovePoints(PTS_CRITICAL_HIT, moveSlot as u16, 0, 0);
            break 'l1;
        }
        if sw1 == STRINGID_ATTACKERSSTATROSE {
            if gBattleTextBuff1[2] != 0 {
                if *statStringId == STRINGID_STATSHARPLY as u16 {
                    AddMovePoints(
                        PTS_STAT_INCREASE_2,
                        moveSlot as u16,
                        gBattleTextBuff1[2] - 1,
                        0,
                    );
                } else {
                    AddMovePoints(
                        PTS_STAT_INCREASE_1,
                        moveSlot as u16,
                        gBattleTextBuff1[2] - 1,
                        0,
                    );
                }
            }
            break 'l1;
        }
        if sw1 == STRINGID_DEFENDERSSTATROSE {
            if gBattleTextBuff1[2] != 0 {
                if gBattlerAttacker == gBattlerTarget {
                    if *statStringId == STRINGID_STATSHARPLY as u16 {
                        AddMovePoints(
                            PTS_STAT_INCREASE_2,
                            moveSlot as u16,
                            gBattleTextBuff1[2] - 1,
                            0,
                        );
                    } else {
                        AddMovePoints(
                            PTS_STAT_INCREASE_1,
                            moveSlot as u16,
                            gBattleTextBuff1[2] - 1,
                            0,
                        );
                    }
                } else {
                    AddMovePoints(
                        PTS_STAT_INCREASE_NOT_SELF,
                        moveSlot as u16,
                        gBattleTextBuff1[2] - 1,
                        0,
                    );
                }
            }
            break 'l1;
        }
        if sw1 == STRINGID_ATTACKERSSTATFELL {
            if gBattleTextBuff1[2] != 0 {
                AddMovePoints(
                    PTS_STAT_DECREASE_SELF,
                    moveSlot as u16,
                    gBattleTextBuff1[2] - 1,
                    0,
                );
            }
            break 'l1;
        }
        if sw1 == STRINGID_DEFENDERSSTATFELL {
            if gBattleTextBuff1[2] != 0 {
                if *statStringId == STRINGID_STATHARSHLY as u16 {
                    AddMovePoints(
                        PTS_STAT_DECREASE_2,
                        moveSlot as u16,
                        gBattleTextBuff1[2] - 1,
                        0,
                    );
                } else {
                    AddMovePoints(
                        PTS_STAT_DECREASE_1,
                        moveSlot as u16,
                        gBattleTextBuff1[2] - 1,
                        0,
                    );
                }
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNLAIDCURSE {
            (*tvPtr).pos[defSide][defFlank]
                .set_curseMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).pos[defSide][defFlank].set_curseMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNAFFLICTEDBYCURSE {
            if GetMonData3(atkMon, MON_DATA_HP, null_mut()) != 0
                && (*tvPtr).pos[atkSide][atkFlank].curseMonId() != 0
            {
                AddMovePoints(
                    PTS_STATUS_DMG,
                    0,
                    (*tvPtr).pos[atkSide][atkFlank].curseMonId() as u8 - 1,
                    (*tvPtr).pos[atkSide][atkFlank].curseMoveSlot() as u8,
                );
                (*tvPtr).side[atkSide].set_faintCause(FNT_CURSE);
                (*tvPtr).side[atkSide].set_faintCauseMonId(atkFlank);
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNSEEDED {
            (*tvPtr).pos[defSide][defFlank]
                .set_leechSeedMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).pos[defSide][defFlank].set_leechSeedMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNSAPPEDBYLEECHSEED {
            if (*tvPtr).pos[atkSide][atkFlank].leechSeedMonId() != 0 {
                AddMovePoints(
                    PTS_STATUS_DMG,
                    1,
                    (*tvPtr).pos[atkSide][atkFlank].leechSeedMonId() as u8 - 1,
                    (*tvPtr).pos[atkSide][atkFlank].leechSeedMoveSlot() as u8,
                );
                (*tvPtr).side[atkSide].set_faintCause(FNT_LEECH_SEED);
                (*tvPtr).side[atkSide].set_faintCauseMonId(atkFlank);
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNFELLINTONIGHTMARE {
            (*tvPtr).pos[defSide][defFlank]
                .set_nightmareMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).pos[defSide][defFlank].set_nightmareMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNLOCKEDINNIGHTMARE {
            if GetMonData3(atkMon, MON_DATA_HP, null_mut()) != 0
                && (*tvPtr).pos[atkSide][atkFlank].nightmareMonId() != 0
            {
                AddMovePoints(
                    PTS_STATUS_DMG,
                    5,
                    (*tvPtr).pos[atkSide][atkFlank].nightmareMonId() as u8 - 1,
                    (*tvPtr).pos[atkSide][atkFlank].nightmareMoveSlot() as u8,
                );
                (*tvPtr).side[atkSide].set_faintCause(FNT_NIGHTMARE);
                (*tvPtr).side[atkSide].set_faintCauseMonId(atkFlank);
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNSQUEEZEDBYBIND
            || sw1 == STRINGID_PKMNTRAPPEDINVORTEX
            || sw1 == STRINGID_PKMNWRAPPEDBY
            || sw1 == STRINGID_PKMNCLAMPED
            || sw1 == STRINGID_PKMNTRAPPEDBYSANDTOMB
        {
            (*tvPtr).pos[defSide][defFlank]
                .set_wrapMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).pos[defSide][defFlank].set_wrapMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNHURTBY {
            if GetMonData3(atkMon, MON_DATA_HP, null_mut()) != 0
                && (*tvPtr).pos[atkSide][atkFlank].wrapMonId() != 0
            {
                AddMovePoints(
                    PTS_STATUS_DMG,
                    6,
                    (*tvPtr).pos[atkSide][atkFlank].wrapMonId() as u8 - 1,
                    (*tvPtr).pos[atkSide][atkFlank].wrapMoveSlot() as u8,
                );
                (*tvPtr).side[atkSide].set_faintCause(FNT_WRAP);
                (*tvPtr).side[atkSide].set_faintCauseMonId(atkFlank);
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNWASBURNED {
            (*tvPtr).mon[effSide][gBattlerPartyIndexes[gEffectBattler]]
                .set_brnMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).mon[effSide][gBattlerPartyIndexes[gEffectBattler]]
                .set_brnMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNHURTBYBURN {
            if GetMonData3(atkMon, MON_DATA_HP, null_mut()) != 0 {
                if (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]].brnMonId() != 0 {
                    AddMovePoints(
                        PTS_STATUS_DMG,
                        4,
                        (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]].brnMonId()
                            as u8
                            - 1,
                        (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]].brnMoveSlot()
                            as u8,
                    );
                }
                (*tvPtr).side[atkSide].set_faintCause(FNT_BURN);
                (*tvPtr).side[atkSide]
                    .set_faintCauseMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32);
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNWASPOISONED {
            (*tvPtr).mon[effSide][gBattlerPartyIndexes[gEffectBattler]]
                .set_psnMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).mon[effSide][gBattlerPartyIndexes[gEffectBattler]]
                .set_psnMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNBADLYPOISONED {
            (*tvPtr).mon[effSide][gBattlerPartyIndexes[gEffectBattler]]
                .set_badPsnMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).mon[effSide][gBattlerPartyIndexes[gEffectBattler]]
                .set_badPsnMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNHURTBYPOISON {
            if GetMonData3(atkMon, MON_DATA_HP, null_mut()) != 0 {
                if (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]].psnMonId() != 0 {
                    AddMovePoints(
                        PTS_STATUS_DMG,
                        2,
                        (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]].psnMonId()
                            as u8
                            - 1,
                        (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]].psnMoveSlot()
                            as u8,
                    );
                }
                if (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]].badPsnMonId() != 0
                {
                    AddMovePoints(
                        PTS_STATUS_DMG,
                        3,
                        (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]].badPsnMonId()
                            as u8
                            - 1,
                        (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]]
                            .badPsnMoveSlot() as u8,
                    );
                }
                (*tvPtr).side[atkSide].set_faintCause(FNT_POISON);
                (*tvPtr).side[atkSide]
                    .set_faintCauseMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32);
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNFELLINLOVE {
            (*tvPtr).pos[defSide][defFlank]
                .set_attractMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).pos[defSide][defFlank].set_attractMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNIMMOBILIZEDBYLOVE {
            if (*tvPtr).pos[atkSide][atkFlank].attractMonId() != 0 {
                AddMovePoints(
                    PTS_STATUS,
                    0,
                    (*tvPtr).pos[atkSide][atkFlank].attractMonId() as u8 - 1,
                    (*tvPtr).pos[atkSide][atkFlank].attractMoveSlot() as u8,
                );
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNWASPARALYZED {
            (*tvPtr).mon[effSide][gBattlerPartyIndexes[gEffectBattler]]
                .set_prlzMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).mon[effSide][gBattlerPartyIndexes[gEffectBattler]]
                .set_prlzMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNISPARALYZED {
            if (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]].prlzMonId() != 0 {
                AddMovePoints(
                    PTS_STATUS,
                    2,
                    (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]].prlzMonId() as u8
                        - 1,
                    (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]].prlzMoveSlot()
                        as u8,
                );
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNFELLASLEEP {
            (*tvPtr).mon[effSide][gBattlerPartyIndexes[gEffectBattler]]
                .set_slpMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).mon[effSide][gBattlerPartyIndexes[gEffectBattler]]
                .set_slpMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNFASTASLEEP {
            if (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]].slpMonId() != 0
                && (*gBattleMsgDataPtr).currentMove != MOVE_SNORE
                && (*gBattleMsgDataPtr).currentMove != MOVE_SLEEP_TALK
            {
                AddMovePoints(
                    PTS_STATUS,
                    3,
                    (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]].slpMonId() as u8
                        - 1,
                    (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]].slpMoveSlot()
                        as u8,
                );
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNWASFROZEN {
            (*tvPtr).mon[effSide][gBattlerPartyIndexes[gEffectBattler]]
                .set_frzMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).mon[effSide][gBattlerPartyIndexes[gEffectBattler]]
                .set_frzMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNISFROZEN {
            if (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]].frzMonId() != 0 {
                AddMovePoints(
                    PTS_STATUS,
                    4,
                    (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]].frzMonId() as u8
                        - 1,
                    (*tvPtr).mon[atkSide][gBattlerPartyIndexes[gBattlerAttacker]].frzMoveSlot()
                        as u8,
                );
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNWASCONFUSED {
            (*tvPtr).pos[effSide][effFlank]
                .set_confusionMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).pos[effSide][effFlank].set_confusionMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_ITHURTCONFUSION {
            if (*tvPtr).pos[atkSide][atkFlank].confusionMonId() != 0 {
                AddMovePoints(
                    PTS_STATUS,
                    1,
                    (*tvPtr).pos[atkSide][atkFlank].confusionMonId() as u8 - 1,
                    (*tvPtr).pos[atkSide][atkFlank].confusionMoveSlot() as u8,
                );
            }
            (*tvPtr).side[atkSide].set_faintCause(FNT_CONFUSION);
            break 'l1;
        }
        if sw1 == STRINGID_SPIKESSCATTERED {
            (*tvPtr).side[defSide]
                .set_spikesMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).side[defSide].set_spikesMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNHURTBYSPIKES {
            if (*tvPtr).side[scriptingSide].spikesMonId() != 0 {
                AddMovePoints(
                    PTS_SPIKES,
                    scriptingSide as u16 ^ 1,
                    (*tvPtr).side[scriptingSide].spikesMonId() as u8 - 1,
                    (*tvPtr).side[scriptingSide].spikesMoveSlot() as u8,
                );
                (*tvPtr).side[scriptingSide].set_faintCause(FNT_SPIKES);
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNBLEWAWAYSPIKES {
            (*tvPtr).side[atkSide].set_spikesMonId(0);
            (*tvPtr).side[atkSide].set_spikesMoveSlot(0);
            break 'l1;
        }
        if sw1 == STRINGID_FIREWEAKENED {
            (*tvPtr).pos[atkSide][atkFlank]
                .set_waterSportMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).pos[atkSide][atkFlank].set_waterSportMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_ELECTRICITYWEAKENED {
            (*tvPtr).pos[atkSide][atkFlank]
                .set_mudSportMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).pos[atkSide][atkFlank].set_mudSportMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_ATTACKERFAINTED {
            fall = true;
            AddPointsOnFainting(FALSE);
        }
        if fall || sw1 == STRINGID_RETURNMON {
            if (*tvPtr).pos[atkSide][atkFlank].waterSportMonId() != 0 {
                (*tvPtr).pos[atkSide][atkFlank].set_waterSportMonId(0);
                (*tvPtr).pos[atkSide][atkFlank].set_waterSportMoveSlot(0);
            }
            if (*tvPtr).pos[atkSide][atkFlank].mudSportMonId() != 0 {
                (*tvPtr).pos[atkSide][atkFlank].set_mudSportMonId(0);
                (*tvPtr).pos[atkSide][atkFlank].set_mudSportMoveSlot(0);
            }
            break 'l1;
        }
        if sw1 == STRINGID_TARGETFAINTED {
            AddPointsOnFainting(TRUE);
            if (*tvPtr).pos[atkSide][defFlank].waterSportMonId() != 0 {
                (*tvPtr).pos[atkSide][defFlank].set_waterSportMonId(0);
                (*tvPtr).pos[atkSide][defFlank].set_waterSportMoveSlot(0);
            }
            if (*tvPtr).pos[atkSide][defFlank].mudSportMonId() != 0 {
                (*tvPtr).pos[atkSide][defFlank].set_mudSportMonId(0);
                (*tvPtr).pos[atkSide][defFlank].set_mudSportMoveSlot(0);
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNRAISEDDEF || sw1 == STRINGID_PKMNRAISEDDEFALITTLE {
            (*tvPtr).side[atkSide]
                .set_reflectMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).side[atkSide].set_reflectMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNRAISEDSPDEF || sw1 == STRINGID_PKMNRAISEDSPDEFALITTLE {
            (*tvPtr).side[atkSide]
                .set_lightScreenMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).side[atkSide].set_lightScreenMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNSXWOREOFF {
            if *finishedMoveId == MOVE_REFLECT {
                (*tvPtr).side[atkSide].set_reflectMonId(0);
                (*tvPtr).side[atkSide].set_reflectMoveSlot(0);
            }
            if *finishedMoveId == MOVE_LIGHT_SCREEN {
                (*tvPtr).side[atkSide].set_lightScreenMonId(0);
                (*tvPtr).side[atkSide].set_lightScreenMoveSlot(0);
            }
            if *finishedMoveId == MOVE_MIST {
                (*tvPtr).side[atkSide].set_mistMonId(0);
                (*tvPtr).side[atkSide].set_mistMoveSlot(0);
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNCOVEREDBYVEIL {
            (*tvPtr).side[atkSide]
                .set_safeguardMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).side[atkSide].set_safeguardMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNUSEDSAFEGUARD {
            if (*tvPtr).side[defSide].safeguardMonId() != 0 {
                AddMovePoints(
                    PTS_SAFEGUARD,
                    0,
                    (*tvPtr).side[defSide].safeguardMonId() as u8 - 1,
                    (*tvPtr).side[defSide].safeguardMoveSlot() as u8,
                );
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNSAFEGUARDEXPIRED {
            (*tvPtr).side[atkSide].set_safeguardMonId(0);
            (*tvPtr).side[atkSide].set_safeguardMoveSlot(0);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNSHROUDEDINMIST {
            (*tvPtr).side[atkSide].set_mistMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
            (*tvPtr).side[atkSide].set_mistMoveSlot(moveSlot as u32);
            break 'l1;
        }
        if sw1 == STRINGID_PKMNPROTECTEDBYMIST {
            if (*tvPtr).side[defSide].mistMonId() != 0 {
                AddMovePoints(
                    PTS_MIST,
                    0,
                    (*tvPtr).side[defSide].mistMonId() as u8 - 1,
                    (*tvPtr).side[defSide].mistMoveSlot() as u8,
                );
            }
            break 'l1;
        }
        if sw1 == STRINGID_THEWALLSHATTERED {
            (*tvPtr).side[defSide].set_reflectMonId(0);
            (*tvPtr).side[defSide].set_reflectMoveSlot(0);
            (*tvPtr).side[defSide].set_lightScreenMonId(0);
            (*tvPtr).side[defSide].set_lightScreenMoveSlot(0);
            AddMovePoints(
                PTS_BREAK_WALL,
                0,
                gBattlerPartyIndexes[gBattlerAttacker] as u8,
                moveSlot,
            );
            break 'l1;
        }
        if sw1 == STRINGID_PKMNFLINCHED {
            if (*tvPtr).pos[atkSide][0].attackedByMonId() != 0 {
                AddMovePoints(
                    PTS_FLINCHED,
                    0,
                    (*tvPtr).pos[atkSide][0].attackedByMonId() as u8 - 1,
                    (*tvPtr).pos[atkSide][0].attackedByMoveSlot() as u8,
                );
            }
            if (*tvPtr).pos[atkSide][1].attackedByMonId() != 0 {
                AddMovePoints(
                    PTS_FLINCHED,
                    0,
                    (*tvPtr).pos[atkSide][1].attackedByMonId() as u8 - 1,
                    (*tvPtr).pos[atkSide][1].attackedByMoveSlot() as u8,
                );
            }
            break 'l1;
        }
        if sw1 == STRINGID_PKMNCRASHED || sw1 == STRINGID_PKMNHITWITHRECOIL {
            (*tvPtr).side[atkSide].set_faintCause(FNT_RECOIL);
            break 'l1;
        }
    }
}
fn IsNotSpecialBattleString(stringId: u16) -> u8 {
    let mut i: i32 = 0;
    loop {
        if sSpecialBattleStrings[i] == stringId {
            break;
        }
        i += 1;
        if sSpecialBattleStrings[i] == TABLE_END as u16 {
            break;
        }
    }
    if sSpecialBattleStrings[i] == TABLE_END as u16 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn BattleTv_SetDataBasedOnMove(
    r#move: u16,
    weatherFlags: u16,
    disableStructPtr: *mut DisableStruct,
) {
    if gBattleTypeFlags & BATTLE_TYPE_LINK == 0 {
        return;
    }
    let tvPtr: *mut BattleTv = &raw mut (*gBattleStruct).tv;
    let atkSide: u32 = GetBattlerSide(gBattlerAttacker) as u32;
    let defSide: u32 = GetBattlerSide(gBattlerTarget) as u32;
    let moveSlot: u8 = GetBattlerMoveSlotId(gBattlerAttacker, r#move);
    if moveSlot >= MAX_MON_MOVES as u8 {
        (*tvPtr).side[atkSide].set_faintCause(FNT_OTHER);
        return;
    }
    (*tvPtr).pos[defSide][GetBattlerPosition(gBattlerAttacker) as i32 / 2]
        .set_attackedByMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
    (*tvPtr).pos[defSide][GetBattlerPosition(gBattlerAttacker) as i32 / 2]
        .set_attackedByMoveSlot(moveSlot as u32);
    (*tvPtr).side[atkSide].set_usedMoveSlot(moveSlot as u32);
    AddMovePoints(
        PTS_MOVE_EFFECT,
        moveSlot as u16,
        (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[r#move]
            .effect,
        0,
    );
    AddPointsBasedOnWeather(weatherFlags, r#move, moveSlot);
    if (*disableStructPtr).chargeTimer() != 0 {
        AddMovePoints(PTS_ELECTRIC, r#move, moveSlot, 0);
    }
    if r#move == MOVE_WISH {
        (*tvPtr).side[atkSide].set_wishMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
        (*tvPtr).side[atkSide].set_wishMoveSlot(moveSlot as u32);
    }
    if r#move == MOVE_SELF_DESTRUCT || r#move == MOVE_EXPLOSION {
        (*tvPtr).side[atkSide ^ 1]
            .set_explosionMonId(gBattlerPartyIndexes[gBattlerAttacker] as u32 + 1);
        (*tvPtr).side[atkSide ^ BIT_SIDE as u32].set_explosionMoveSlot(moveSlot as u32);
        (*tvPtr).side[atkSide ^ BIT_SIDE as u32].set_faintCause(FNT_EXPLOSION);
        (*tvPtr).side[atkSide ^ 1].set_explosion(1);
    }
    AddMovePoints(
        PTS_REFLECT,
        (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[r#move]
            .r#type as u16,
        (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[r#move]
            .power,
        0,
    );
    AddMovePoints(
        PTS_LIGHT_SCREEN,
        (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[r#move]
            .r#type as u16,
        (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[r#move]
            .power,
        0,
    );
    AddMovePoints(
        PTS_WATER_SPORT,
        (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[r#move]
            .r#type as u16,
        0,
        0,
    );
    AddMovePoints(
        PTS_MUD_SPORT,
        (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[r#move]
            .r#type as u16,
        0,
        0,
    );
}
pub unsafe fn BattleTv_SetDataBasedOnAnimation(animationId: u8) {
    if gBattleTypeFlags & BATTLE_TYPE_LINK == 0 {
        return;
    }
    let tvPtr: *mut BattleTv = &raw mut (*gBattleStruct).tv;
    let atkSide: u32 = GetBattlerSide(gBattlerAttacker) as u32;
    match animationId {
        B_ANIM_FUTURE_SIGHT_HIT => {
            if (*tvPtr).side[atkSide].futureSightMonId() != 0 {
                AddMovePoints(
                    PTS_SET_UP,
                    0,
                    atkSide as u8,
                    ((*tvPtr).side[atkSide].futureSightMonId() as u8 - 1) * 4
                        + (*tvPtr).side[atkSide].futureSightMoveSlot() as u8,
                );
                (*tvPtr).side[atkSide].set_faintCause(FNT_FUTURE_SIGHT);
            }
        }
        B_ANIM_DOOM_DESIRE_HIT if (*tvPtr).side[atkSide].doomDesireMonId() != 0 => {
            AddMovePoints(
                PTS_SET_UP,
                1,
                atkSide as u8,
                ((*tvPtr).side[atkSide].doomDesireMonId() as u8 - 1) * 4
                    + (*tvPtr).side[atkSide].doomDesireMoveSlot() as u8,
            );
            (*tvPtr).side[atkSide].set_faintCause(FNT_DOOM_DESIRE);
        }
        _ => {}
    }
}
pub unsafe fn TryPutLinkBattleTvShowOnAir() {
    let mut playerBestSpecies: u16 = 0;
    let mut opponentBestSpecies: u16 = 0;
    let mut playerBestSum: i16 = 0;
    let mut opponentBestSum: i16 = 32767;
    let mut playerBestMonId: u8 = 0;
    let mut opponentBestMonId: u8 = 0;
    let mut countPlayer: u8 = 0;
    let mut countOpponent: u8 = 0;
    let mut sum: i16 = 0;
    let mut species: u16 = 0;
    let mut j: i32 = 0;
    let zero: i32 = 0;
    let one: i32 = 1;
    if (*gBattleStruct).anyMonHasTransformed != 0 {
        return;
    }
    let movePoints: *mut BattleTvMovePoints = &raw mut (*gBattleStruct).tvMovePoints;
    for i in 0..PARTY_SIZE {
        if GetMonData3(&raw mut gPlayerParty[i], MON_DATA_SPECIES, null_mut())
            != SPECIES_NONE as u32
        {
            countPlayer += 1;
        }
        if GetMonData3(&raw mut gEnemyParty[i], MON_DATA_SPECIES, null_mut()) != SPECIES_NONE as u32
        {
            countOpponent += 1;
        }
    }
    if gBattleTypeFlags & BATTLE_TYPE_LINK == 0 || countPlayer != countOpponent {
        return;
    }
    for i in 0..PARTY_SIZE {
        species = GetMonData3(&raw mut gPlayerParty[i], MON_DATA_SPECIES, null_mut()) as u16;
        if species != SPECIES_NONE
            && GetMonData3(&raw mut gPlayerParty[i], MON_DATA_IS_EGG, null_mut()) == 0
        {
            sum = 0;
            for j in 0..MAX_MON_MOVES {
                sum += (*movePoints).points[zero][i * 4 + j];
            }
            if playerBestSum < sum {
                playerBestMonId = i as u8;
                playerBestSum = sum;
                playerBestSpecies = species;
            }
        }
        species = GetMonData3(&raw mut gEnemyParty[i], MON_DATA_SPECIES, null_mut()) as u16;
        if species != SPECIES_NONE
            && GetMonData3(&raw mut gEnemyParty[i], MON_DATA_IS_EGG, null_mut()) == 0
        {
            sum = 0;
            for j in 0..MAX_MON_MOVES {
                sum += (*movePoints).points[one][i * 4 + j];
            }
            if opponentBestSum == sum {
                if GetMonData3(&raw mut gEnemyParty[i], MON_DATA_EXP, null_mut())
                    > GetMonData3(
                        &raw mut gEnemyParty[opponentBestMonId],
                        MON_DATA_EXP,
                        null_mut(),
                    )
                {
                    opponentBestMonId = i as u8;
                    opponentBestSum = sum;
                    opponentBestSpecies = species;
                }
            } else if opponentBestSum > sum {
                opponentBestMonId = i as u8;
                opponentBestSum = sum;
                opponentBestSpecies = species;
            }
        }
    }
    sum = 0;
    let mut i: i32 = 0;
    j = 0;
    while j < MAX_MON_MOVES {
        if sum < (*movePoints).points[zero][playerBestMonId as i32 * 4 + j] {
            sum = (*movePoints).points[zero][playerBestMonId as i32 * 4 + j];
            i = j;
        }
        j += 1;
    }
    let r#move: u16 = GetMonData3(
        &raw mut gPlayerParty[playerBestMonId],
        MON_DATA_MOVE1 + i,
        null_mut(),
    ) as u16;
    if playerBestSum == 0 || r#move == 0 {
        return;
    }
    if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        if playerBestMonId < MULTI_PARTY_SIZE as u8
            && GetLinkTrainerFlankId(gBattleScripting.multiplayerId) == 0
            || playerBestMonId >= MULTI_PARTY_SIZE as u8
                && GetLinkTrainerFlankId(gBattleScripting.multiplayerId) != 0
        {
            j = if opponentBestMonId < MULTI_PARTY_SIZE as u8 {
                FALSE as i32
            } else {
                TRUE as i32
            };
            PutBattleUpdateOnTheAir(
                GetOpposingLinkMultiBattlerId(j as u8, gBattleScripting.multiplayerId),
                r#move,
                playerBestSpecies,
                opponentBestSpecies,
            );
        }
    } else {
        PutBattleUpdateOnTheAir(
            gBattleScripting.multiplayerId ^ 1,
            r#move,
            playerBestSpecies,
            opponentBestSpecies,
        );
    }
}
unsafe fn AddMovePoints(caseId: u8, arg1: u16, arg2: u8, arg3: u8) {
    let movePoints: *mut BattleTvMovePoints = &raw mut (*gBattleStruct).tvMovePoints;
    let tvPtr: *mut BattleTv = &raw mut (*gBattleStruct).tv;
    let atkSide: u32 = GetBattlerSide(gBattlerAttacker) as u32;
    let defSide: u32 = GetBattlerSide(gBattlerTarget) as u32;
    let mut ptr: *mut u16 = null_mut();
    let mut i: i32 = 0;
    'l1: {
        let sw1: u8 = caseId;
        let mut fall = false;
        if sw1 == PTS_MOVE_EFFECT
            || sw1 == PTS_EFFECTIVENESS
            || sw1 == PTS_CRITICAL_HIT
            || sw1 == PTS_STAT_INCREASE_1
            || sw1 == PTS_STAT_INCREASE_2
            || sw1 == PTS_STAT_DECREASE_SELF
            || sw1 == PTS_STAT_DECREASE_1
            || sw1 == PTS_STAT_DECREASE_2
            || sw1 == PTS_STAT_INCREASE_NOT_SELF
        {
            (*movePoints).points[atkSide]
                [gBattlerPartyIndexes[gBattlerAttacker] as i32 * 4 + arg1 as i32] +=
                *sPointsArray[caseId].at(arg2) as i16;
            break 'l1;
        }
        if sw1 == PTS_RAIN
            || sw1 == PTS_SUN
            || sw1 == PTS_SANDSTORM
            || sw1 == PTS_HAIL
            || sw1 == PTS_ELECTRIC
        {
            i = 0;
            ptr = sPointsArray[caseId];
            loop {
                if arg1 == *ptr.at(i) {
                    (*movePoints).points[atkSide]
                        [gBattlerPartyIndexes[gBattlerAttacker] as i32 * 4 + arg2 as i32] +=
                        *ptr.at(i + 1) as i16;
                    break;
                }
                i += 2;
                if *ptr.at(i) == TABLE_END as u16 {
                    break;
                }
            }
            break 'l1;
        }
        if sw1 == PTS_FAINT {
            (*tvPtr).side[arg2 as i32 ^ 1].set_faintCause(FNT_NONE);
            (*movePoints).points[arg2][arg3 as i32] += *sPointsArray[caseId].at(arg1) as i16;
            break 'l1;
        }
        if sw1 == PTS_FAINT_SET_UP {
            fall = true;
            (*tvPtr).side[arg2].set_faintCause(FNT_NONE);
        }
        if fall || sw1 == PTS_SET_UP {
            (*movePoints).points[arg2][arg3 as i32] += *sPointsArray[caseId].at(arg1) as i16;
            break 'l1;
        }
        if sw1 == PTS_BREAK_WALL {
            (*movePoints).points[atkSide][arg2 as i32 * 4 + arg3 as i32] +=
                *sPointsArray[caseId].at(arg1) as i16;
            break 'l1;
        }
        if sw1 == PTS_STATUS_DMG
            || sw1 == PTS_STATUS
            || sw1 == PTS_SAFEGUARD
            || sw1 == PTS_MIST
            || sw1 == PTS_FLINCHED
        {
            (*movePoints).points[atkSide ^ BIT_SIDE as u32][arg2 as i32 * 4 + arg3 as i32] +=
                *sPointsArray[caseId].at(arg1) as i16;
            break 'l1;
        }
        if sw1 == PTS_SPIKES {
            (*movePoints).points[arg1][arg2 as i32 * 4 + arg3 as i32] +=
                *sPointsArray[caseId] as i16;
            break 'l1;
        }
        if sw1 == PTS_WATER_SPORT {
            if (*tvPtr).pos[defSide][0].waterSportMonId()
                != (*tvPtr).pos[defSide][1].waterSportMonId().wrapping_neg()
                && arg1 == TYPE_FIRE as u16
            {
                if (*tvPtr).pos[defSide][0].waterSportMonId() != 0 {
                    let id: u32 = ((*tvPtr).pos[defSide][0].waterSportMonId() - 1) * 4;
                    (*movePoints).points[defSide]
                        [id + (*tvPtr).pos[defSide][0].waterSportMoveSlot()] +=
                        *sPointsArray[caseId] as i16;
                }
                if (*tvPtr).pos[defSide][1].waterSportMonId() != 0 {
                    let id: u32 = ((*tvPtr).pos[defSide][1].waterSportMonId() - 1) * 4;
                    (*movePoints).points[defSide]
                        [id + (*tvPtr).pos[defSide][1].waterSportMoveSlot()] +=
                        *sPointsArray[caseId] as i16;
                }
            }
            break 'l1;
        }
        if sw1 == PTS_MUD_SPORT {
            if (*tvPtr).pos[defSide][0].mudSportMonId()
                != (*tvPtr).pos[defSide][1].mudSportMonId().wrapping_neg()
                && arg1 == TYPE_ELECTRIC as u16
            {
                if (*tvPtr).pos[defSide][0].mudSportMonId() != 0 {
                    let id: u32 = ((*tvPtr).pos[defSide][0].mudSportMonId() - 1) * 4;
                    (*movePoints).points[defSide]
                        [id + (*tvPtr).pos[defSide][0].mudSportMoveSlot()] +=
                        *sPointsArray[caseId] as i16;
                }
                if (*tvPtr).pos[defSide][1].mudSportMonId() != 0 {
                    let id: u32 = ((*tvPtr).pos[defSide][1].mudSportMonId() - 1) * 4;
                    (*movePoints).points[defSide]
                        [id + (*tvPtr).pos[defSide][1].mudSportMoveSlot()] +=
                        *sPointsArray[caseId] as i16;
                }
            }
            break 'l1;
        }
        if sw1 == PTS_REFLECT {
            if arg1 < 9 && arg2 != 0 && (*tvPtr).side[defSide].reflectMonId() != 0 {
                let id: u32 = ((*tvPtr).side[defSide].reflectMonId() - 1) * 4;
                (*movePoints).points[defSide][id + (*tvPtr).side[defSide].reflectMoveSlot()] +=
                    *sPointsArray[caseId] as i16;
            }
            break 'l1;
        }
        if sw1 == PTS_LIGHT_SCREEN {
            if arg1 >= 9 && arg2 != 0 && (*tvPtr).side[defSide].lightScreenMonId() != 0 {
                let id: u32 = ((*tvPtr).side[defSide].lightScreenMonId() - 1) * 4;
                (*movePoints).points[defSide][id + (*tvPtr).side[defSide].lightScreenMoveSlot()] +=
                    *sPointsArray[caseId] as i16;
            }
            break 'l1;
        }
    }
}
unsafe fn AddPointsOnFainting(targetFainted: u8) {
    let tvPtr: *mut BattleTv = &raw mut (*gBattleStruct).tv;
    let atkSide: u32 = GetBattlerSide(gBattlerAttacker) as u32;
    let defSide: u32 = GetBattlerSide(gBattlerTarget) as u32;
    let atkArrId: u32 = (*tvPtr).side[atkSide].faintCauseMonId();
    if (*tvPtr).side[atkSide].faintCause() != FNT_NONE {
        match (*tvPtr).side[atkSide].faintCause() {
            FNT_CURSE => {
                if (*tvPtr).pos[atkSide][atkArrId].curseMonId() != 0 {
                    AddMovePoints(
                        PTS_FAINT,
                        0,
                        atkSide as u8 ^ BIT_SIDE,
                        ((*tvPtr).pos[atkSide][atkArrId].curseMonId() as u8 - 1) * 4
                            + (*tvPtr).pos[atkSide][atkArrId].curseMoveSlot() as u8,
                    );
                }
            }
            FNT_LEECH_SEED => {
                if (*tvPtr).pos[atkSide][atkArrId].leechSeedMonId() != 0 {
                    AddMovePoints(
                        PTS_FAINT,
                        0,
                        atkSide as u8 ^ BIT_SIDE,
                        ((*tvPtr).pos[atkSide][atkArrId].leechSeedMonId() as u8 - 1) * 4
                            + (*tvPtr).pos[atkSide][atkArrId].leechSeedMoveSlot() as u8,
                    );
                }
            }
            FNT_POISON => {
                if (*tvPtr).mon[atkSide][atkArrId].psnMonId() != 0 {
                    AddMovePoints(
                        PTS_FAINT,
                        0,
                        atkSide as u8 ^ BIT_SIDE,
                        ((*tvPtr).mon[atkSide][atkArrId].psnMonId() as u8 - 1) * 4
                            + (*tvPtr).mon[atkSide][atkArrId].psnMoveSlot() as u8,
                    );
                }
                if (*tvPtr).mon[atkSide][atkArrId].badPsnMonId() != 0 {
                    AddMovePoints(
                        PTS_FAINT,
                        0,
                        atkSide as u8 ^ BIT_SIDE,
                        ((*tvPtr).mon[atkSide][atkArrId].badPsnMonId() as u8 - 1) * 4
                            + (*tvPtr).mon[atkSide][atkArrId].badPsnMoveSlot() as u8,
                    );
                }
            }
            FNT_BURN => {
                if (*tvPtr).mon[atkSide][atkArrId].brnMonId() != 0 {
                    AddMovePoints(
                        PTS_FAINT,
                        0,
                        atkSide as u8 ^ BIT_SIDE,
                        ((*tvPtr).mon[atkSide][atkArrId].brnMonId() as u8 - 1) * 4
                            + (*tvPtr).mon[atkSide][atkArrId].brnMoveSlot() as u8,
                    );
                }
            }
            FNT_NIGHTMARE => {
                if (*tvPtr).pos[atkSide][atkArrId].nightmareMonId() != 0 {
                    AddMovePoints(
                        PTS_FAINT,
                        0,
                        atkSide as u8 ^ BIT_SIDE,
                        ((*tvPtr).pos[atkSide][atkArrId].nightmareMonId() as u8 - 1) * 4
                            + (*tvPtr).pos[atkSide][atkArrId].nightmareMoveSlot() as u8,
                    );
                }
            }
            FNT_WRAP => {
                if (*tvPtr).pos[atkSide][atkArrId].wrapMonId() != 0 {
                    AddMovePoints(
                        PTS_FAINT,
                        0,
                        atkSide as u8 ^ BIT_SIDE,
                        ((*tvPtr).pos[atkSide][atkArrId].wrapMonId() as u8 - 1) * 4
                            + (*tvPtr).pos[atkSide][atkArrId].wrapMoveSlot() as u8,
                    );
                }
            }
            FNT_SPIKES => {
                if (*tvPtr).side[atkSide].spikesMonId() != 0 {
                    AddMovePoints(
                        PTS_FAINT,
                        0,
                        atkSide as u8 ^ BIT_SIDE,
                        ((*tvPtr).side[atkSide].spikesMonId() as u8 - 1) * 4
                            + (*tvPtr).side[atkSide].spikesMoveSlot() as u8,
                    );
                }
            }
            FNT_FUTURE_SIGHT => {
                if (*tvPtr).side[atkSide].futureSightMonId() != 0 {
                    AddMovePoints(
                        PTS_FAINT_SET_UP,
                        0,
                        atkSide as u8,
                        ((*tvPtr).side[atkSide].futureSightMonId() as u8 - 1) * 4
                            + (*tvPtr).side[atkSide].futureSightMoveSlot() as u8,
                    );
                }
            }
            FNT_DOOM_DESIRE => {
                if (*tvPtr).side[atkSide].doomDesireMonId() != 0 {
                    AddMovePoints(
                        PTS_FAINT_SET_UP,
                        0,
                        atkSide as u8,
                        ((*tvPtr).side[atkSide].doomDesireMonId() as u8 - 1) * 4
                            + (*tvPtr).side[atkSide].doomDesireMoveSlot() as u8,
                    );
                }
            }
            FNT_PERISH_SONG => {
                if (*tvPtr).side[atkSide].perishSong() != 0
                    && (*tvPtr).side[atkSide].perishSongMonId() - 1
                        != gBattlerPartyIndexes[gBattlerAttacker] as u32
                {
                    AddMovePoints(
                        PTS_FAINT,
                        0,
                        atkSide as u8,
                        ((*tvPtr).side[atkSide].perishSongMonId() as u8 - 1) * 4
                            + (*tvPtr).side[atkSide].perishSongMoveSlot() as u8,
                    );
                }
                if (*tvPtr).side[atkSide ^ BIT_SIDE as u32].perishSong() != 0 {
                    AddMovePoints(
                        PTS_FAINT,
                        0,
                        atkSide as u8 ^ BIT_SIDE,
                        ((*tvPtr).side[atkSide ^ 1].perishSongMonId() as u8 - 1) * 4
                            + (*tvPtr).side[atkSide ^ 1].perishSongMoveSlot() as u8,
                    );
                }
            }
            FNT_DESTINY_BOND => {
                if (*tvPtr).side[atkSide ^ BIT_SIDE as u32].destinyBondMonId() != 0 {
                    AddMovePoints(
                        PTS_FAINT,
                        0,
                        atkSide as u8 ^ BIT_SIDE,
                        ((*tvPtr).side[atkSide ^ 1].destinyBondMonId() as u8 - 1) * 4
                            + (*tvPtr).side[atkSide ^ 1].destinyBondMoveSlot() as u8,
                    );
                }
            }
            FNT_CONFUSION => {
                for i in 0..2i32 {
                    if (*tvPtr).pos[atkSide][i].confusionMonId() != 0 {
                        AddMovePoints(
                            PTS_FAINT,
                            0,
                            atkSide as u8 ^ BIT_SIDE,
                            ((*tvPtr).pos[atkSide][i].confusionMonId() as u8 - 1) * 4
                                + (*tvPtr).pos[atkSide][i].confusionMoveSlot() as u8,
                        );
                    }
                }
            }
            FNT_EXPLOSION => {
                if (*tvPtr).side[atkSide].explosion() != 0 {
                    AddMovePoints(
                        PTS_FAINT,
                        0,
                        atkSide as u8,
                        ((*tvPtr).side[atkSide].explosionMonId() as u8 - 1) * 4
                            + (*tvPtr).side[atkSide].explosionMoveSlot() as u8,
                    );
                }
                if (*tvPtr).side[atkSide ^ BIT_SIDE as u32].explosion() != 0 {
                    AddMovePoints(
                        PTS_FAINT,
                        0,
                        atkSide as u8 ^ BIT_SIDE,
                        ((*tvPtr).side[atkSide ^ 1].explosionMonId() as u8 - 1) * 4
                            + (*tvPtr).side[atkSide ^ 1].explosionMoveSlot() as u8,
                    );
                }
            }
            FNT_RECOIL => {
                if targetFainted == TRUE {
                    AddMovePoints(
                        PTS_FAINT_SET_UP,
                        0,
                        atkSide as u8,
                        gBattlerPartyIndexes[gBattlerAttacker] as u8 * 4
                            + (*tvPtr).side[atkSide].usedMoveSlot() as u8,
                    );
                }
            }
            FNT_OTHER => {}
            _ => {}
        }
    } else {
        if (*tvPtr).side[defSide].faintCause() == FNT_SPIKES {
            if (*tvPtr).side[defSide].spikesMonId() != 0 {
                AddMovePoints(
                    PTS_FAINT,
                    0,
                    defSide as u8 ^ BIT_SIDE,
                    ((*tvPtr).side[defSide].spikesMonId() as u8 - 1) * 4
                        + (*tvPtr).side[defSide].spikesMoveSlot() as u8,
                );
            }
        } else {
            AddMovePoints(
                PTS_FAINT_SET_UP,
                0,
                atkSide as u8,
                gBattlerPartyIndexes[gBattlerAttacker] as u8 * 4
                    + (*tvPtr).side[atkSide].usedMoveSlot() as u8,
            );
        }
    }
}
unsafe fn TrySetBattleSeminarShow() {
    let mut dmgByMove: CArray<i32, 4> = zeroed();
    let mut powerOverride: u16 = 0;
    if gBattleTypeFlags & 0x2000003 != 0 {
        return;
    } else if GetBattlerSide(gBattlerAttacker) == B_SIDE_OPPONENT {
        return;
    } else if gBattleMons[gBattlerAttacker].statStages[6] < 6 {
        return;
    } else if gBattleMons[gBattlerTarget].statStages[7] > DEFAULT_STAT_STAGE {
        return;
    } else if gCurrentMove == MOVE_HIDDEN_POWER || gCurrentMove == MOVE_WEATHER_BALL {
        return;
    } else if gBattleTypeFlags & 0x320000 != 0 {
        return;
    } else if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [gBattleMons[gBattlerAttacker].moves[gMoveSelectionCursor[gBattlerAttacker]]]
        .power
        == 0
    {
        return;
    }
    let mut i: i32 = 0;
    let mut currMoveSaved: u16 =
        gBattleMons[gBattlerAttacker].moves[gMoveSelectionCursor[gBattlerAttacker]];
    loop {
        if currMoveSaved == sVariableDmgMoves[i] {
            break;
        }
        i += 1;
        if sVariableDmgMoves[i] == TABLE_END as u16 {
            break;
        }
    }
    if sVariableDmgMoves[i] != TABLE_END as u16 {
        return;
    }
    dmgByMove[gMoveSelectionCursor[gBattlerAttacker]] = gBattleMoveDamage;
    currMoveSaved = gCurrentMove;
    i = 0;
    while i < MAX_MON_MOVES {
        gCurrentMove = gBattleMons[gBattlerAttacker].moves[i];
        powerOverride = 0;
        if ShouldCalculateDamage(gCurrentMove, &raw mut dmgByMove[i], &raw mut powerOverride) != 0 {
            let sideStatus: u16 = gSideStatuses[GetBattlerPosition(gBattlerTarget) as i32 & 1];
            gBattleMoveDamage = CalculateBaseDamage(
                &raw mut gBattleMons[gBattlerAttacker],
                &raw mut gBattleMons[gBattlerTarget],
                gCurrentMove as u32,
                sideStatus,
                powerOverride,
                0,
                gBattlerAttacker,
                gBattlerTarget,
            );
            if gStatuses3[gBattlerAttacker] & STATUS3_CHARGED_UP != 0
                && (*(&raw const crate::data::pokemon::gBattleMoves)
                    .cast::<CArray<BattleMove, 0>>())[gCurrentMove]
                    .r#type
                    == TYPE_ELECTRIC
            {
                gBattleMoveDamage *= 2;
            }
            if gProtectStructs[gBattlerAttacker].helpingHand() != 0 {
                gBattleMoveDamage = gBattleMoveDamage * 15 / 10;
            }
            let moveResultFlags: u8 = TypeCalc(gCurrentMove, gBattlerAttacker, gBattlerTarget);
            dmgByMove[i] = gBattleMoveDamage;
            if dmgByMove[i] == 0 && moveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0 {
                dmgByMove[i] = 1;
            }
        }
        i += 1;
    }
    for i in 0..MAX_MON_MOVES {
        if i != gMoveSelectionCursor[gBattlerAttacker] as i32
            && dmgByMove[i] > dmgByMove[gMoveSelectionCursor[gBattlerAttacker]]
        {
            let mut bestMoveId: i32 = 0;
            if gMoveSelectionCursor[gBattlerAttacker] != 0 {
                bestMoveId = 0;
            } else {
                bestMoveId = 1;
            }
            for i in 0..MAX_MON_MOVES {
                if i != gMoveSelectionCursor[gBattlerAttacker] as i32
                    && dmgByMove[i] > dmgByMove[bestMoveId]
                {
                    bestMoveId = i;
                }
            }
            let opponentSpecies: u16 = GetMonData3(
                &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerTarget]],
                MON_DATA_SPECIES,
                null_mut(),
            ) as u16;
            let playerSpecies: u16 = GetMonData3(
                &raw mut gPlayerParty[gBattlerPartyIndexes[gBattlerAttacker]],
                MON_DATA_SPECIES,
                null_mut(),
            ) as u16;
            TryPutBattleSeminarOnAir(
                opponentSpecies,
                playerSpecies,
                gMoveSelectionCursor[gBattlerAttacker],
                gBattleMons[gBattlerAttacker].moves.as_mut_ptr(),
                gBattleMons[gBattlerAttacker].moves[bestMoveId],
            );
            break;
        }
    }
    gBattleMoveDamage = dmgByMove[gMoveSelectionCursor[gBattlerAttacker]];
    gCurrentMove = currMoveSaved;
}
unsafe fn ShouldCalculateDamage(r#move: u16, dmg: *mut i32, powerOverride: *mut u16) -> u8 {
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[r#move]
        .power
        == 0
    {
        *dmg = 0;
        return FALSE;
    } else {
        let mut i: i32 = 0;
        loop {
            if r#move == sVariableDmgMoves[i] {
                break;
            }
            i += 1;
            if sVariableDmgMoves[i] == TABLE_END as u16 {
                break;
            }
        }
        if sVariableDmgMoves[i] != TABLE_END as u16 {
            *dmg = 0;
            return FALSE;
        } else if r#move == MOVE_PSYWAVE {
            *dmg = gBattleMons[gBattlerAttacker].level as i32;
            *dmg /= 2;
            return FALSE;
        } else if r#move == MOVE_MAGNITUDE {
            *powerOverride = 10;
            return TRUE;
        } else {
            return TRUE;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn BattleTv_ClearExplosionFaintCause() {
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        let tvPtr: *mut BattleTv = &raw mut (*gBattleStruct).tv;
        (*tvPtr).side[0].set_faintCause(FNT_NONE);
        (*tvPtr).side[1].set_faintCause(FNT_NONE);
        (*tvPtr).side[0].set_faintCauseMonId(0);
        (*tvPtr).side[1].set_faintCauseMonId(0);
        (*tvPtr).side[0].set_explosionMonId(0);
        (*tvPtr).side[1].set_explosionMonId(0);
        (*tvPtr).side[0].set_explosionMoveSlot(0);
        (*tvPtr).side[1].set_explosionMoveSlot(0);
        (*tvPtr).side[0].set_explosion(0);
        (*tvPtr).side[1].set_explosion(FALSE as u32);
    }
}
pub unsafe fn GetBattlerMoveSlotId(battler: u8, r#move: u16) -> u8 {
    let mut party: *mut Pokemon = null_mut();
    if GetBattlerSide(battler) == B_SIDE_PLAYER {
        party = gPlayerParty.as_mut_ptr();
    } else {
        party = gEnemyParty.as_mut_ptr();
    }
    let mut i: i32 = 0;
    loop {
        if i >= MAX_MON_MOVES {
            break;
        }
        if GetMonData3(
            party.at(gBattlerPartyIndexes[battler]),
            MON_DATA_MOVE1 + i,
            null_mut(),
        ) == r#move as u32
        {
            break;
        }
        i += 1;
    }
    i as u8
}
unsafe fn AddPointsBasedOnWeather(weatherFlags: u16, r#move: u16, moveSlot: u8) {
    if weatherFlags as i32 & B_WEATHER_RAIN != 0 {
        AddMovePoints(PTS_RAIN, r#move, moveSlot, 0);
    } else if weatherFlags as i32 & B_WEATHER_SUN != 0 {
        AddMovePoints(PTS_SUN, r#move, moveSlot, 0);
    } else if weatherFlags as i32 & B_WEATHER_SANDSTORM != 0 {
        AddMovePoints(PTS_SANDSTORM, r#move, moveSlot, 0);
    } else if weatherFlags as i32 & B_WEATHER_HAIL != 0 {
        AddMovePoints(PTS_HAIL, r#move, moveSlot, 0);
    }
}
