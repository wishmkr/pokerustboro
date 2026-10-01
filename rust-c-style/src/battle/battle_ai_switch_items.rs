//! Translated from `src/battle_ai_switch_items.c` by tools/rustport/c2rs.py.
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
    static mut gAbsentBattlerFlags: u8;
    static mut gActiveBattler: u8;
    static mut gBattleMons: CArray<BattlePokemon, 4>;
    static mut gBattleMoveDamage: i32;
    static gBattleMoves: CArray<BattleMove, 0>;
    static mut gBattleResources: *mut BattleResources;
    static mut gBattleScripting: BattleScripting;
    static mut gBattleStruct: *mut BattleStruct;
    static mut gBattleTypeFlags: u32;
    static mut gBattlerPartyIndexes: CArray<u16, 4>;
    static gBitTable: CArray<u32, 0>;
    static mut gCritMultiplier: u8;
    static mut gDisableStructs: CArray<DisableStruct, 4>;
    static mut gDynamicBasePower: u16;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static gItemEffectTable: CArray<*mut u8, 0>;
    static mut gLastHitBy: CArray<u8, 4>;
    static mut gLastLandedMoves: CArray<u16, 4>;
    static mut gMoveResultFlags: u8;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSideTimers: CArray<SideTimer, 2>;
    static gSpeciesInfo: CArray<SpeciesInfo, 0>;
    static mut gStatuses3: CArray<u32, 4>;
    static gTypeEffectiveness: CArray<u8, 336>;
    fn AI_CalcDmg(a0: u8, a1: u8);
    fn AI_TypeCalc(a0: u16, a1: u16, a2: u8) -> u8;
    fn AbilityBattleEffects(a0: u8, a1: u8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BtlController_EmitTwoReturnValues(a0: u8, a1: u8, a2: u16);
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetItemEffectParamOffset(a0: u16, a1: u8, a2: u8) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn Random() -> u16;
    fn TypeCalc(a0: u16, a1: u8, a2: u8) -> u8;
}

pub(crate) unsafe extern "C" fn ShouldSwitchIfPerishSong() -> u8 {
    if gStatuses3[gActiveBattler] & STATUS3_PERISH_SONG != 0
        && gDisableStructs[gActiveBattler].perishSongTimer() == 0
    {
        *(*gBattleStruct)
            .AI_monToSwitchIntoId
            .as_mut_ptr()
            .at(gActiveBattler) = PARTY_SIZE as u8;
        BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SWITCH, 0);
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ShouldSwitchIfWonderGuard() -> u8 {
    let mut opposingPosition: u8 = 0;
    let mut opposingBattler: u8 = 0;
    let mut moveFlags: u8 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut firstId: i32 = 0;
    let mut lastId: i32 = 0;
    let mut party: *mut Pokemon = null_mut();
    let mut r#move: u16 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
        return FALSE;
    }
    opposingPosition = GetBattlerPosition(gActiveBattler) ^ 1;
    if gBattleMons[GetBattlerAtPosition(opposingPosition)].ability != ABILITY_WONDER_GUARD {
        return FALSE;
    }
    opposingBattler = GetBattlerAtPosition(opposingPosition);
    i = 0;
    while i < MAX_MON_MOVES {
        'l1: {
            r#move = gBattleMons[gActiveBattler].moves[i];
            if r#move == MOVE_NONE {
                break 'l1;
            }
            moveFlags = AI_TypeCalc(
                r#move,
                gBattleMons[opposingBattler].species,
                gBattleMons[opposingBattler].ability,
            );
            if moveFlags as i32 & MOVE_RESULT_SUPER_EFFECTIVE != 0 {
                return FALSE;
            }
        }
        i += 1;
    }
    if gBattleTypeFlags & 0x808000 != 0 {
        if gActiveBattler as i32 & BIT_FLANK as i32 == B_FLANK_LEFT {
            firstId = 0;
            lastId = 3;
        } else {
            firstId = 3;
            lastId = PARTY_SIZE;
        }
    } else {
        firstId = 0;
        lastId = PARTY_SIZE;
    }
    if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
        party = gPlayerParty.as_mut_ptr();
    } else {
        party = gEnemyParty.as_mut_ptr();
    }
    i = firstId;
    while i < lastId {
        'l3: {
            if GetMonData2(party.at(i), MON_DATA_HP) == 0 {
                break 'l3;
            }
            if GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) == SPECIES_NONE as u32 {
                break 'l3;
            }
            if GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) == SPECIES_EGG {
                break 'l3;
            }
            if i == gBattlerPartyIndexes[gActiveBattler] as i32 {
                break 'l3;
            }
            GetMonData2(party.at(i), MON_DATA_SPECIES);
            GetMonData2(party.at(i), MON_DATA_ABILITY_NUM);
            opposingBattler = GetBattlerAtPosition(opposingPosition);
            j = 0;
            while j < MAX_MON_MOVES {
                'l5: {
                    r#move = GetMonData2(party.at(i), MON_DATA_MOVE1 + j) as u16;
                    if r#move == MOVE_NONE {
                        break 'l5;
                    }
                    moveFlags = AI_TypeCalc(
                        r#move,
                        gBattleMons[opposingBattler].species,
                        gBattleMons[opposingBattler].ability,
                    );
                    if moveFlags as i32 & 2 != 0 && Random() as i32 % 3 < 2 {
                        *(*gBattleStruct)
                            .AI_monToSwitchIntoId
                            .as_mut_ptr()
                            .at(gActiveBattler) = i as u8;
                        BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SWITCH, 0);
                        return TRUE;
                    }
                }
                j += 1;
            }
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn FindMonThatAbsorbsOpponentsMove() -> u8 {
    let mut battlerIn1: u8 = 0;
    let mut battlerIn2: u8 = 0;
    let mut absorbingTypeAbility: u8 = 0;
    let mut firstId: i32 = 0;
    let mut lastId: i32 = 0;
    let mut party: *mut Pokemon = null_mut();
    let mut i: i32 = 0;
    if HasSuperEffectiveMoveAgainstOpponents(TRUE) != 0 && Random() as i32 % 3 != 0 {
        return FALSE;
    }
    if gLastLandedMoves[gActiveBattler] == MOVE_NONE {
        return FALSE;
    }
    if gLastLandedMoves[gActiveBattler] == MOVE_UNAVAILABLE {
        return FALSE;
    }
    if gBattleMoves[gLastLandedMoves[gActiveBattler]].power == 0 {
        return FALSE;
    }
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
        battlerIn1 = gActiveBattler;
        if gAbsentBattlerFlags as u32
            & gBitTable[GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2)]
            != 0
        {
            battlerIn2 = gActiveBattler;
        } else {
            battlerIn2 = GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2);
        }
    } else {
        battlerIn1 = gActiveBattler;
        battlerIn2 = gActiveBattler;
    }
    if gBattleMoves[gLastLandedMoves[gActiveBattler]].r#type == TYPE_FIRE {
        absorbingTypeAbility = ABILITY_FLASH_FIRE;
    } else if gBattleMoves[gLastLandedMoves[gActiveBattler]].r#type == TYPE_WATER {
        absorbingTypeAbility = ABILITY_WATER_ABSORB;
    } else if gBattleMoves[gLastLandedMoves[gActiveBattler]].r#type == TYPE_ELECTRIC {
        absorbingTypeAbility = ABILITY_VOLT_ABSORB;
    } else {
        return FALSE;
    }
    if gBattleMons[gActiveBattler].ability == absorbingTypeAbility {
        return FALSE;
    }
    if gBattleTypeFlags & 0x808000 != 0 {
        if gActiveBattler as i32 & BIT_FLANK as i32 == B_FLANK_LEFT {
            firstId = 0;
            lastId = 3;
        } else {
            firstId = 3;
            lastId = PARTY_SIZE;
        }
    } else {
        firstId = 0;
        lastId = PARTY_SIZE;
    }
    if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
        party = gPlayerParty.as_mut_ptr();
    } else {
        party = gEnemyParty.as_mut_ptr();
    }
    i = firstId;
    while i < lastId {
        'l1: {
            let mut species: u16 = 0;
            let mut monAbility: u8 = 0;
            if GetMonData2(party.at(i), MON_DATA_HP) == 0 {
                break 'l1;
            }
            if GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) == SPECIES_NONE as u32 {
                break 'l1;
            }
            if GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) == SPECIES_EGG {
                break 'l1;
            }
            if i == gBattlerPartyIndexes[battlerIn1] as i32 {
                break 'l1;
            }
            if i == gBattlerPartyIndexes[battlerIn2] as i32 {
                break 'l1;
            }
            if i == *(*gBattleStruct)
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(battlerIn1) as i32
            {
                break 'l1;
            }
            if i == *(*gBattleStruct)
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(battlerIn2) as i32
            {
                break 'l1;
            }
            species = GetMonData2(party.at(i), MON_DATA_SPECIES) as u16;
            if GetMonData2(party.at(i), MON_DATA_ABILITY_NUM) != 0 {
                monAbility = gSpeciesInfo[species].abilities[1];
            } else {
                monAbility = gSpeciesInfo[species].abilities[0];
            }
            if absorbingTypeAbility == monAbility && Random() as i32 & 1 != 0 {
                *(*gBattleStruct)
                    .AI_monToSwitchIntoId
                    .as_mut_ptr()
                    .at(gActiveBattler) = i as u8;
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SWITCH, 0);
                return TRUE;
            }
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ShouldSwitchIfNaturalCure() -> u8 {
    if gBattleMons[gActiveBattler].status1 & STATUS1_SLEEP == 0 {
        return FALSE;
    }
    if gBattleMons[gActiveBattler].ability != ABILITY_NATURAL_CURE {
        return FALSE;
    }
    if (gBattleMons[gActiveBattler].hp as i32) < gBattleMons[gActiveBattler].maxHP as i32 / 2 {
        return FALSE;
    }
    if (gLastLandedMoves[gActiveBattler] == MOVE_NONE
        || gLastLandedMoves[gActiveBattler] == MOVE_UNAVAILABLE)
        && Random() as i32 & 1 != 0
    {
        *(*gBattleStruct)
            .AI_monToSwitchIntoId
            .as_mut_ptr()
            .at(gActiveBattler) = PARTY_SIZE as u8;
        BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SWITCH, 0);
        return TRUE;
    } else if gBattleMoves[gLastLandedMoves[gActiveBattler]].power == 0 && Random() as i32 & 1 != 0
    {
        *(*gBattleStruct)
            .AI_monToSwitchIntoId
            .as_mut_ptr()
            .at(gActiveBattler) = PARTY_SIZE as u8;
        BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SWITCH, 0);
        return TRUE;
    }
    if FindMonWithFlagsAndSuperEffective(MOVE_RESULT_DOESNT_AFFECT_FOE, 1) != 0 {
        return TRUE;
    }
    if FindMonWithFlagsAndSuperEffective(MOVE_RESULT_NOT_VERY_EFFECTIVE as u8, 1) != 0 {
        return TRUE;
    }
    if Random() as i32 & 1 != 0 {
        *(*gBattleStruct)
            .AI_monToSwitchIntoId
            .as_mut_ptr()
            .at(gActiveBattler) = PARTY_SIZE as u8;
        BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SWITCH, 0);
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn HasSuperEffectiveMoveAgainstOpponents(noRng: u8) -> u8 {
    let mut opposingPosition: u8 = 0;
    let mut opposingBattler: u8 = 0;
    let mut i: i32 = 0;
    let mut moveFlags: u8 = 0;
    let mut r#move: u16 = 0;
    opposingPosition = GetBattlerPosition(gActiveBattler) ^ 1;
    opposingBattler = GetBattlerAtPosition(opposingPosition);
    if gAbsentBattlerFlags as u32 & gBitTable[opposingBattler] == 0 {
        i = 0;
        while i < MAX_MON_MOVES {
            'l1: {
                r#move = gBattleMons[gActiveBattler].moves[i];
                if r#move == MOVE_NONE {
                    break 'l1;
                }
                moveFlags = AI_TypeCalc(
                    r#move,
                    gBattleMons[opposingBattler].species,
                    gBattleMons[opposingBattler].ability,
                );
                if moveFlags as i32 & MOVE_RESULT_SUPER_EFFECTIVE != 0 {
                    if noRng != 0 {
                        return TRUE;
                    }
                    if Random() as i32 % 10 != 0 {
                        return TRUE;
                    }
                }
            }
            i += 1;
        }
    }
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE == 0 {
        return FALSE;
    }
    opposingBattler = GetBattlerAtPosition(opposingPosition ^ 2);
    if gAbsentBattlerFlags as u32 & gBitTable[opposingBattler] == 0 {
        i = 0;
        while i < MAX_MON_MOVES {
            'l3: {
                r#move = gBattleMons[gActiveBattler].moves[i];
                if r#move == MOVE_NONE {
                    break 'l3;
                }
                moveFlags = AI_TypeCalc(
                    r#move,
                    gBattleMons[opposingBattler].species,
                    gBattleMons[opposingBattler].ability,
                );
                if moveFlags as i32 & MOVE_RESULT_SUPER_EFFECTIVE != 0 {
                    if noRng != 0 {
                        return TRUE;
                    }
                    if Random() as i32 % 10 != 0 {
                        return TRUE;
                    }
                }
            }
            i += 1;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn AreStatsRaised() -> u8 {
    let mut buffedStatsValue: u8 = 0;
    let mut i: i32 = 0;
    i = 0;
    while i < NUM_BATTLE_STATS {
        if gBattleMons[gActiveBattler].statStages[i] > DEFAULT_STAT_STAGE {
            buffedStatsValue +=
                gBattleMons[gActiveBattler].statStages[i] as u8 - DEFAULT_STAT_STAGE as u8;
        }
        i += 1;
    }
    return (buffedStatsValue > 3) as u8;
}
pub(crate) unsafe extern "C" fn FindMonWithFlagsAndSuperEffective(
    flags: u8,
    moduloPercent: u8,
) -> u8 {
    let mut battlerIn1: u8 = 0;
    let mut battlerIn2: u8 = 0;
    let mut firstId: i32 = 0;
    let mut lastId: i32 = 0;
    let mut party: *mut Pokemon = null_mut();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut r#move: u16 = 0;
    let mut moveFlags: u8 = 0;
    if gLastLandedMoves[gActiveBattler] == MOVE_NONE {
        return FALSE;
    }
    if gLastLandedMoves[gActiveBattler] == MOVE_UNAVAILABLE {
        return FALSE;
    }
    if gLastHitBy[gActiveBattler] == 0xFF {
        return FALSE;
    }
    if gBattleMoves[gLastLandedMoves[gActiveBattler]].power == 0 {
        return FALSE;
    }
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
        battlerIn1 = gActiveBattler;
        if gAbsentBattlerFlags as u32
            & gBitTable[GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2)]
            != 0
        {
            battlerIn2 = gActiveBattler;
        } else {
            battlerIn2 = GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2);
        }
    } else {
        battlerIn1 = gActiveBattler;
        battlerIn2 = gActiveBattler;
    }
    if gBattleTypeFlags & 0x808000 != 0 {
        if gActiveBattler as i32 & BIT_FLANK as i32 == 0 {
            firstId = 0;
            lastId = 3;
        } else {
            firstId = 3;
            lastId = PARTY_SIZE;
        }
    } else {
        firstId = 0;
        lastId = PARTY_SIZE;
    }
    if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
        party = gPlayerParty.as_mut_ptr();
    } else {
        party = gEnemyParty.as_mut_ptr();
    }
    i = firstId;
    while i < lastId {
        'l1: {
            let mut species: u16 = 0;
            let mut monAbility: u8 = 0;
            if GetMonData2(party.at(i), MON_DATA_HP) == 0 {
                break 'l1;
            }
            if GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) == SPECIES_NONE as u32 {
                break 'l1;
            }
            if GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) == SPECIES_EGG {
                break 'l1;
            }
            if i == gBattlerPartyIndexes[battlerIn1] as i32 {
                break 'l1;
            }
            if i == gBattlerPartyIndexes[battlerIn2] as i32 {
                break 'l1;
            }
            if i == *(*gBattleStruct)
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(battlerIn1) as i32
            {
                break 'l1;
            }
            if i == *(*gBattleStruct)
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(battlerIn2) as i32
            {
                break 'l1;
            }
            species = GetMonData2(party.at(i), MON_DATA_SPECIES) as u16;
            if GetMonData2(party.at(i), MON_DATA_ABILITY_NUM) != 0 {
                monAbility = gSpeciesInfo[species].abilities[1];
            } else {
                monAbility = gSpeciesInfo[species].abilities[0];
            }
            moveFlags = AI_TypeCalc(gLastLandedMoves[gActiveBattler], species, monAbility);
            if moveFlags as i32 & flags as i32 != 0 {
                battlerIn1 = gLastHitBy[gActiveBattler];
                j = 0;
                while j < MAX_MON_MOVES {
                    'l3: {
                        r#move = GetMonData2(party.at(i), MON_DATA_MOVE1 + j) as u16;
                        if r#move == 0 {
                            break 'l3;
                        }
                        moveFlags = AI_TypeCalc(
                            r#move,
                            gBattleMons[battlerIn1].species,
                            gBattleMons[battlerIn1].ability,
                        );
                        if moveFlags as i32 & MOVE_RESULT_SUPER_EFFECTIVE != 0
                            && rem_i32(Random() as i32, moduloPercent as i32) == 0
                        {
                            *(*gBattleStruct)
                                .AI_monToSwitchIntoId
                                .as_mut_ptr()
                                .at(gActiveBattler) = i as u8;
                            BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SWITCH, 0);
                            return TRUE;
                        }
                    }
                    j += 1;
                }
            }
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ShouldSwitch() -> u8 {
    let mut battlerIn1: u8 = 0;
    let mut battlerIn2: u8 = 0;
    let mut activeBattlerPtr: *mut u8 = null_mut();
    let mut firstId: i32 = 0;
    let mut lastId: i32 = 0;
    let mut party: *mut Pokemon = null_mut();
    let mut i: i32 = 0;
    let mut availableToSwitch: i32 = 0;
    if gBattleMons[*({
        activeBattlerPtr = &raw mut gActiveBattler;
        activeBattlerPtr
    })]
    .status2
        & 0x400e000
        != 0
    {
        return FALSE;
    }
    if gStatuses3[gActiveBattler] & STATUS3_ROOTED != 0 {
        return FALSE;
    }
    if AbilityBattleEffects(12, gActiveBattler, ABILITY_SHADOW_TAG, 0, 0) != 0 {
        return FALSE;
    }
    if AbilityBattleEffects(12, gActiveBattler, ABILITY_ARENA_TRAP, 0, 0) != 0 {
        return FALSE;
    }
    if AbilityBattleEffects(14, 0, ABILITY_MAGNET_PULL, 0, 0) != 0 {
        if gBattleMons[gActiveBattler].types[0] == TYPE_STEEL
            || gBattleMons[gActiveBattler].types[1] == TYPE_STEEL
        {
            return FALSE;
        }
    }
    if gBattleTypeFlags & BATTLE_TYPE_ARENA != 0 {
        return FALSE;
    }
    availableToSwitch = 0;
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
        battlerIn1 = *activeBattlerPtr;
        if gAbsentBattlerFlags as u32
            & gBitTable[GetBattlerAtPosition(GetBattlerPosition(*activeBattlerPtr) ^ 2)]
            != 0
        {
            battlerIn2 = *activeBattlerPtr;
        } else {
            battlerIn2 = GetBattlerAtPosition(GetBattlerPosition(*activeBattlerPtr) ^ 2);
        }
    } else {
        battlerIn1 = *activeBattlerPtr;
        battlerIn2 = *activeBattlerPtr;
    }
    if gBattleTypeFlags & 0x808000 != 0 {
        if gActiveBattler as i32 & BIT_FLANK as i32 == B_FLANK_LEFT {
            firstId = 0;
            lastId = 3;
        } else {
            firstId = 3;
            lastId = PARTY_SIZE;
        }
    } else {
        firstId = 0;
        lastId = PARTY_SIZE;
    }
    if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
        party = gPlayerParty.as_mut_ptr();
    } else {
        party = gEnemyParty.as_mut_ptr();
    }
    i = firstId;
    while i < lastId {
        'l1: {
            if GetMonData2(party.at(i), MON_DATA_HP) == 0 {
                break 'l1;
            }
            if GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) == SPECIES_NONE as u32 {
                break 'l1;
            }
            if GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) == SPECIES_EGG {
                break 'l1;
            }
            if i == gBattlerPartyIndexes[battlerIn1] as i32 {
                break 'l1;
            }
            if i == gBattlerPartyIndexes[battlerIn2] as i32 {
                break 'l1;
            }
            if i == *(*gBattleStruct)
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(battlerIn1) as i32
            {
                break 'l1;
            }
            if i == *(*gBattleStruct)
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(battlerIn2) as i32
            {
                break 'l1;
            }
            availableToSwitch += 1;
        }
        i += 1;
    }
    if availableToSwitch == 0 {
        return FALSE;
    }
    if ShouldSwitchIfPerishSong() != 0 {
        return TRUE;
    }
    if ShouldSwitchIfWonderGuard() != 0 {
        return TRUE;
    }
    if FindMonThatAbsorbsOpponentsMove() != 0 {
        return TRUE;
    }
    if ShouldSwitchIfNaturalCure() != 0 {
        return TRUE;
    }
    if HasSuperEffectiveMoveAgainstOpponents(FALSE) != 0 {
        return FALSE;
    }
    if AreStatsRaised() != 0 {
        return FALSE;
    }
    if FindMonWithFlagsAndSuperEffective(MOVE_RESULT_DOESNT_AFFECT_FOE, 2) != 0
        || FindMonWithFlagsAndSuperEffective(MOVE_RESULT_NOT_VERY_EFFECTIVE as u8, 3) != 0
    {
        return TRUE;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AI_TrySwitchOrUseItem() {
    let mut party: *mut Pokemon = null_mut();
    let mut battlerIn1: u8 = 0;
    let mut battlerIn2: u8 = 0;
    let mut firstId: i32 = 0;
    let mut lastId: i32 = 0;
    let mut battlerIdentity: u8 = GetBattlerPosition(gActiveBattler);
    if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
        party = gPlayerParty.as_mut_ptr();
    } else {
        party = gEnemyParty.as_mut_ptr();
    }
    if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
        if ShouldSwitch() != 0 {
            if *(*gBattleStruct)
                .AI_monToSwitchIntoId
                .as_mut_ptr()
                .at(gActiveBattler)
                == PARTY_SIZE as u8
            {
                let mut monToSwitchId: i32 = GetMostSuitableMonToSwitchInto() as i32;
                if monToSwitchId == PARTY_SIZE {
                    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE == 0 {
                        battlerIn1 = GetBattlerAtPosition(battlerIdentity);
                        battlerIn2 = battlerIn1;
                    } else {
                        battlerIn1 = GetBattlerAtPosition(battlerIdentity);
                        battlerIn2 = GetBattlerAtPosition(battlerIdentity ^ 2);
                    }
                    if gBattleTypeFlags & 0x808000 != 0 {
                        if gActiveBattler as i32 & BIT_FLANK as i32 == B_FLANK_LEFT {
                            firstId = 0;
                            lastId = 3;
                        } else {
                            firstId = 3;
                            lastId = PARTY_SIZE;
                        }
                    } else {
                        firstId = 0;
                        lastId = PARTY_SIZE;
                    }
                    monToSwitchId = firstId;
                    'l2: while monToSwitchId < lastId {
                        'l1: {
                            if GetMonData2(party.at(monToSwitchId), MON_DATA_HP) == 0 {
                                break 'l1;
                            }
                            if monToSwitchId == gBattlerPartyIndexes[battlerIn1] as i32 {
                                break 'l1;
                            }
                            if monToSwitchId == gBattlerPartyIndexes[battlerIn2] as i32 {
                                break 'l1;
                            }
                            if monToSwitchId
                                == *(*gBattleStruct)
                                    .monToSwitchIntoId
                                    .as_mut_ptr()
                                    .at(battlerIn1) as i32
                            {
                                break 'l1;
                            }
                            if monToSwitchId
                                == *(*gBattleStruct)
                                    .monToSwitchIntoId
                                    .as_mut_ptr()
                                    .at(battlerIn2) as i32
                            {
                                break 'l1;
                            }
                            break 'l2;
                        }
                        monToSwitchId += 1;
                    }
                }
                *(*gBattleStruct)
                    .AI_monToSwitchIntoId
                    .as_mut_ptr()
                    .at(gActiveBattler) = monToSwitchId as u8;
            }
            *(*gBattleStruct)
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(gActiveBattler) = *(*gBattleStruct)
                .AI_monToSwitchIntoId
                .as_mut_ptr()
                .at(gActiveBattler);
            return;
        } else if ShouldUseItem() != 0 {
            return;
        }
    }
    BtlController_EmitTwoReturnValues(
        B_COMM_TO_ENGINE,
        B_ACTION_USE_MOVE,
        (gActiveBattler as u16 ^ 1) << 8,
    );
}
pub(crate) unsafe extern "C" fn ModulateByTypeEffectiveness(
    atkType: u8,
    defType1: u8,
    defType2: u8,
    var: *mut u8,
) {
    let mut i: i32 = 0;
    while gTypeEffectiveness[i + 0] != TYPE_ENDTABLE {
        if gTypeEffectiveness[i + 0] == TYPE_FORESIGHT {
            i += 3;
            continue;
        } else if gTypeEffectiveness[i + 0] == atkType {
            if gTypeEffectiveness[i + 1] == defType1 {
                *var = (*var as i32 * gTypeEffectiveness[i + 2] as i32 / 10) as u8;
            }
            if gTypeEffectiveness[i + 1] == defType2 && defType1 != defType2 {
                *var = (*var as i32 * gTypeEffectiveness[i + 2] as i32 / 10) as u8;
            }
        }
        i += 3;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMostSuitableMonToSwitchInto() -> u8 {
    let mut opposingBattler: u8 = 0;
    let mut bestDmg: u8 = 0;
    let mut bestMonId: u8 = 0;
    let mut battlerIn1: u8 = 0;
    let mut battlerIn2: u8 = 0;
    let mut firstId: i32 = 0;
    let mut lastId: i32 = 0;
    let mut party: *mut Pokemon = null_mut();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut invalidMons: u8 = 0;
    let mut r#move: u16 = 0;
    if *(*gBattleStruct)
        .monToSwitchIntoId
        .as_mut_ptr()
        .at(gActiveBattler)
        != PARTY_SIZE as u8
    {
        return *(*gBattleStruct)
            .monToSwitchIntoId
            .as_mut_ptr()
            .at(gActiveBattler);
    }
    if gBattleTypeFlags & BATTLE_TYPE_ARENA != 0 {
        return gBattlerPartyIndexes[gActiveBattler] as u8 + 1;
    }
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
        battlerIn1 = gActiveBattler;
        if gAbsentBattlerFlags as u32
            & gBitTable[GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2)]
            != 0
        {
            battlerIn2 = gActiveBattler;
        } else {
            battlerIn2 = GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2);
        }
        opposingBattler = Random() as u8 & BIT_FLANK;
        if gAbsentBattlerFlags as u32 & gBitTable[opposingBattler] != 0 {
            opposingBattler ^= BIT_FLANK;
        }
    } else {
        opposingBattler = GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 1);
        battlerIn1 = gActiveBattler;
        battlerIn2 = gActiveBattler;
    }
    if gBattleTypeFlags & 0x808000 != 0 {
        if gActiveBattler as i32 & BIT_FLANK as i32 == B_FLANK_LEFT {
            firstId = 0;
            lastId = 3;
        } else {
            firstId = 3;
            lastId = PARTY_SIZE;
        }
    } else {
        firstId = 0;
        lastId = PARTY_SIZE;
    }
    if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
        party = gPlayerParty.as_mut_ptr();
    } else {
        party = gEnemyParty.as_mut_ptr();
    }
    invalidMons = 0;
    while invalidMons != 63 {
        bestDmg = TYPE_MUL_NO_EFFECT;
        bestMonId = PARTY_SIZE as u8;
        i = firstId;
        while i < lastId {
            let mut species: u16 = GetMonData2(party.at(i), MON_DATA_SPECIES) as u16;
            if species != SPECIES_NONE
                && GetMonData2(party.at(i), MON_DATA_HP) != 0
                && gBitTable[i] & invalidMons as u32 == 0
                && gBattlerPartyIndexes[battlerIn1] as i32 != i
                && gBattlerPartyIndexes[battlerIn2] as i32 != i
                && i != *(*gBattleStruct)
                    .monToSwitchIntoId
                    .as_mut_ptr()
                    .at(battlerIn1) as i32
                && i != *(*gBattleStruct)
                    .monToSwitchIntoId
                    .as_mut_ptr()
                    .at(battlerIn2) as i32
            {
                let mut type1: u8 = gSpeciesInfo[species].types[0];
                let mut type2: u8 = gSpeciesInfo[species].types[1];
                let mut typeDmg: u8 = TYPE_MUL_NORMAL;
                ModulateByTypeEffectiveness(
                    gBattleMons[opposingBattler].types[0],
                    type1,
                    type2,
                    &raw mut typeDmg,
                );
                ModulateByTypeEffectiveness(
                    gBattleMons[opposingBattler].types[1],
                    type1,
                    type2,
                    &raw mut typeDmg,
                );
                if bestDmg < typeDmg {
                    bestDmg = typeDmg;
                    bestMonId = i as u8;
                }
            } else {
                invalidMons |= gBitTable[i] as u8;
            }
            i += 1;
        }
        if bestMonId != PARTY_SIZE as u8 {
            i = 0;
            while i < MAX_MON_MOVES {
                r#move = GetMonData2(party.at(bestMonId), MON_DATA_MOVE1 + i) as u16;
                if r#move != MOVE_NONE
                    && TypeCalc(r#move, gActiveBattler, opposingBattler) as i32
                        & MOVE_RESULT_SUPER_EFFECTIVE
                        != 0
                {
                    break;
                }
                i += 1;
            }
            if i != MAX_MON_MOVES {
                return bestMonId;
            }
            invalidMons |= gBitTable[bestMonId] as u8;
        } else {
            invalidMons = 63;
        }
    }
    gDynamicBasePower = 0;
    (*gBattleStruct).dynamicMoveType = 0;
    gBattleScripting.dmgMultiplier = 1;
    gMoveResultFlags = 0;
    gCritMultiplier = 1;
    bestDmg = 0;
    bestMonId = PARTY_SIZE as u8;
    i = firstId;
    while i < lastId {
        'l4: {
            if GetMonData2(party.at(i), MON_DATA_SPECIES) as u16 == SPECIES_NONE {
                break 'l4;
            }
            if GetMonData2(party.at(i), MON_DATA_HP) == 0 {
                break 'l4;
            }
            if gBattlerPartyIndexes[battlerIn1] as i32 == i {
                break 'l4;
            }
            if gBattlerPartyIndexes[battlerIn2] as i32 == i {
                break 'l4;
            }
            if i == *(*gBattleStruct)
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(battlerIn1) as i32
            {
                break 'l4;
            }
            if i == *(*gBattleStruct)
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(battlerIn2) as i32
            {
                break 'l4;
            }
            j = 0;
            while j < MAX_MON_MOVES {
                r#move = GetMonData2(party.at(i), MON_DATA_MOVE1 + j) as u16;
                gBattleMoveDamage = 0;
                if r#move != MOVE_NONE && gBattleMoves[r#move].power != 1 {
                    AI_CalcDmg(gActiveBattler, opposingBattler);
                    TypeCalc(r#move, gActiveBattler, opposingBattler);
                }
                if (bestDmg as i32) < gBattleMoveDamage {
                    bestDmg = gBattleMoveDamage as u8;
                    bestMonId = i as u8;
                }
                j += 1;
            }
        }
        i += 1;
    }
    return bestMonId;
}
pub(crate) unsafe extern "C" fn GetAI_ItemType(itemId: u8, itemEffect: *mut u8) -> u8 {
    if itemId == ITEM_FULL_RESTORE {
        return AI_ITEM_FULL_RESTORE;
    } else if *itemEffect.at(4) as i32 & 0x4 != 0 {
        return AI_ITEM_HEAL_HP;
    } else if *itemEffect.at(3) as i32 & ITEM3_STATUS_ALL != 0 {
        return AI_ITEM_CURE_CONDITION;
    } else if *itemEffect as i32 & 63 != 0 || *itemEffect.at(1) != 0 || *itemEffect.at(2) != 0 {
        return AI_ITEM_X_STAT;
    } else if *itemEffect.at(3) as i32 & ITEM3_GUARD_SPEC != 0 {
        return AI_ITEM_GUARD_SPEC;
    } else {
        return AI_ITEM_NOT_RECOGNIZABLE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ShouldUseItem() -> u8 {
    let mut party: *mut Pokemon = null_mut();
    let mut i: i32 = 0;
    let mut validMons: u8 = 0;
    let mut shouldUse: u8 = FALSE;
    if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0
        && GetBattlerPosition(gActiveBattler) == B_POSITION_PLAYER_RIGHT
    {
        return FALSE;
    }
    if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
        party = gPlayerParty.as_mut_ptr();
    } else {
        party = gEnemyParty.as_mut_ptr();
    }
    i = 0;
    while i < PARTY_SIZE {
        if GetMonData2(party.at(i), MON_DATA_HP) != 0
            && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_NONE as u32
            && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_EGG
        {
            validMons += 1;
        }
        i += 1;
    }
    i = 0;
    while i < MAX_TRAINER_ITEMS {
        'l2: {
            let mut item: u16 = 0;
            let mut itemEffects: *mut u8 = null_mut();
            let mut paramOffset: u8 = 0;
            let mut battlerSide: u8 = 0;
            if i != 0
                && validMons as i32 > (*(*gBattleResources).battleHistory).itemsNo as i32 - i + 1
            {
                break 'l2;
            }
            item = (*(*gBattleResources).battleHistory).trainerItems[i];
            if item == ITEM_NONE {
                break 'l2;
            }
            if gItemEffectTable[item as i32 - ITEM_POTION].is_null() {
                break 'l2;
            }
            if item == ITEM_ENIGMA_BERRY {
                itemEffects = (*gSaveBlock1Ptr).enigmaBerry.itemEffect.as_mut_ptr();
            } else {
                itemEffects = gItemEffectTable[item as i32 - ITEM_POTION];
            }
            *(*gBattleStruct)
                .AI_itemType
                .as_mut_ptr()
                .at(gActiveBattler as i32 / 2) = GetAI_ItemType(item as u8, itemEffects);
            'l4: {
                match *(*gBattleStruct)
                    .AI_itemType
                    .as_mut_ptr()
                    .at(gActiveBattler as i32 / 2)
                {
                    AI_ITEM_FULL_RESTORE => {
                        if gBattleMons[gActiveBattler].hp as i32
                            >= gBattleMons[gActiveBattler].maxHP as i32 / 4
                        {
                            break 'l4;
                        }
                        if gBattleMons[gActiveBattler].hp == 0 {
                            break 'l4;
                        }
                        shouldUse = TRUE;
                    }
                    AI_ITEM_HEAL_HP => {
                        paramOffset = GetItemEffectParamOffset(item, 4, 0x4);
                        if paramOffset == 0 {
                            break 'l4;
                        }
                        if gBattleMons[gActiveBattler].hp == 0 {
                            break 'l4;
                        }
                        if (gBattleMons[gActiveBattler].hp as i32)
                            < gBattleMons[gActiveBattler].maxHP as i32 / 4
                            || gBattleMons[gActiveBattler].maxHP as i32
                                - gBattleMons[gActiveBattler].hp as i32
                                > *itemEffects.at(paramOffset) as i32
                        {
                            shouldUse = TRUE;
                        }
                    }
                    AI_ITEM_CURE_CONDITION => {
                        *(*gBattleStruct)
                            .AI_itemFlags
                            .as_mut_ptr()
                            .at(gActiveBattler as i32 / 2) = 0;
                        if *itemEffects.at(3) as i32 & ITEM3_SLEEP != 0
                            && gBattleMons[gActiveBattler].status1 & STATUS1_SLEEP != 0
                        {
                            *(*gBattleStruct)
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler as i32 / 2) |= 32;
                            shouldUse = TRUE;
                        }
                        if *itemEffects.at(3) as i32 & ITEM3_POISON != 0
                            && (gBattleMons[gActiveBattler].status1 & STATUS1_POISON != 0
                                || gBattleMons[gActiveBattler].status1 & STATUS1_TOXIC_POISON != 0)
                        {
                            *(*gBattleStruct)
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler as i32 / 2) |= 16;
                            shouldUse = TRUE;
                        }
                        if *itemEffects.at(3) as i32 & ITEM3_BURN != 0
                            && gBattleMons[gActiveBattler].status1 & STATUS1_BURN != 0
                        {
                            *(*gBattleStruct)
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler as i32 / 2) |= 8;
                            shouldUse = TRUE;
                        }
                        if *itemEffects.at(3) as i32 & ITEM3_FREEZE != 0
                            && gBattleMons[gActiveBattler].status1 & STATUS1_FREEZE != 0
                        {
                            *(*gBattleStruct)
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler as i32 / 2) |= 4;
                            shouldUse = TRUE;
                        }
                        if *itemEffects.at(3) as i32 & ITEM3_PARALYSIS != 0
                            && gBattleMons[gActiveBattler].status1 & STATUS1_PARALYSIS != 0
                        {
                            *(*gBattleStruct)
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler as i32 / 2) |= 2;
                            shouldUse = TRUE;
                        }
                        if *itemEffects.at(3) as i32 & ITEM3_CONFUSION != 0
                            && gBattleMons[gActiveBattler].status2 & STATUS2_CONFUSION != 0
                        {
                            *(*gBattleStruct)
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler as i32 / 2) |= 1;
                            shouldUse = TRUE;
                        }
                    }
                    AI_ITEM_X_STAT => {
                        *(*gBattleStruct)
                            .AI_itemFlags
                            .as_mut_ptr()
                            .at(gActiveBattler as i32 / 2) = 0;
                        if gDisableStructs[gActiveBattler].isFirstTurn == 0 {
                            break 'l4;
                        }
                        if *itemEffects as i32 & ITEM0_X_ATTACK != 0 {
                            *(*gBattleStruct)
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler as i32 / 2) |= 1;
                        }
                        if *itemEffects.at(1) as i32 & ITEM1_X_DEFEND != 0 {
                            *(*gBattleStruct)
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler as i32 / 2) |= 2;
                        }
                        if *itemEffects.at(1) as i32 & ITEM1_X_SPEED != 0 {
                            *(*gBattleStruct)
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler as i32 / 2) |= 4;
                        }
                        if *itemEffects.at(2) as i32 & ITEM2_X_SPATK != 0 {
                            *(*gBattleStruct)
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler as i32 / 2) |= 8;
                        }
                        if *itemEffects.at(2) as i32 & ITEM2_X_ACCURACY != 0 {
                            *(*gBattleStruct)
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler as i32 / 2) |= 32;
                        }
                        if *itemEffects as i32 & ITEM0_DIRE_HIT != 0 {
                            *(*gBattleStruct)
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler as i32 / 2) |= 128;
                        }
                        shouldUse = TRUE;
                    }
                    AI_ITEM_GUARD_SPEC => {
                        battlerSide = GetBattlerSide(gActiveBattler);
                        if gDisableStructs[gActiveBattler].isFirstTurn != 0
                            && gSideTimers[battlerSide].mistTimer == 0
                        {
                            shouldUse = TRUE;
                        }
                    }
                    AI_ITEM_NOT_RECOGNIZABLE => {
                        return FALSE;
                    }
                    _ => {}
                }
            }
            if shouldUse != 0 {
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_USE_ITEM, 0);
                *(*gBattleStruct)
                    .chosenItem
                    .as_mut_ptr()
                    .at(gActiveBattler as i32 / 2 * 2) = item as u8;
                (*(*gBattleResources).battleHistory).trainerItems[i] = ITEM_NONE;
                return shouldUse;
            }
        }
        i += 1;
    }
    return FALSE;
}
