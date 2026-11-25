//! Translated from `src/battle_ai_switch_items.c` by tools/rustport/c2rs.py.
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

use crate::battle_anim_mons::{GetBattlerAtPosition, GetBattlerPosition, GetBattlerSide};
use crate::battle_controllers::BtlController_EmitTwoReturnValues;
use crate::battle_main::{
    gAbsentBattlerFlags, gActiveBattler, gBattleMons, gBattleMoveDamage, gBattleResources,
    gBattleScripting, gBattleStruct, gBattleTypeFlags, gCritMultiplier, gDisableStructs,
    gDynamicBasePower, gMoveResultFlags, gSideTimers, gStatuses3,
};
use crate::battle_main::{gBattlerPartyIndexes, gLastHitBy, gLastLandedMoves};
use crate::battle_script_commands::{AI_CalcDmg, AI_TypeCalc, TypeCalc};
use crate::battle_util::AbilityBattleEffects;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::load_save::gSaveBlock1Ptr;
use crate::pokemon::{GetItemEffectParamOffset, GetMonData2, gEnemyParty, gPlayerParty};
use crate::random::Random;
#[allow(unused_imports)]
use crate::types::*;
use crate::util::gBitTable;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;

unsafe fn ShouldSwitchIfPerishSong() -> u8 {
    if {
        let gl_i0 = gActiveBattler.get();
        crate::gl!(gStatuses3[gl_i0])
    } & STATUS3_PERISH_SONG
        != 0
        && (*gDisableStructs.as_ptr())[gActiveBattler.get()].perishSongTimer() == 0
    {
        *(*crate::gl!(gBattleStruct))
            .AI_monToSwitchIntoId
            .as_mut_ptr()
            .at(gActiveBattler.get()) = PARTY_SIZE as u8;
        BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SWITCH, 0);
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn ShouldSwitchIfWonderGuard() -> u8 {
    let mut moveFlags: u8 = 0;
    let mut firstId: i32 = 0;
    let mut lastId: i32 = 0;
    let mut party: *mut Pokemon = null_mut();
    let mut r#move: u16 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
        return FALSE;
    }
    let opposingPosition: u8 = GetBattlerPosition(gActiveBattler.get()) ^ 1;
    if ({
        let gl_i0 = GetBattlerAtPosition(opposingPosition);
        crate::gl!(gBattleMons[gl_i0].ability)
    }) != ABILITY_WONDER_GUARD
    {
        return FALSE;
    }
    let mut opposingBattler: u8 = GetBattlerAtPosition(opposingPosition);
    let mut i: i32 = 0;
    while i < MAX_MON_MOVES {
        'l1: {
            r#move = {
                let gl_i1 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i1].moves[i])
            };
            if r#move == MOVE_NONE {
                break 'l1;
            }
            moveFlags = AI_TypeCalc(
                r#move,
                crate::gl!(gBattleMons[opposingBattler].species),
                crate::gl!(gBattleMons[opposingBattler].ability),
            );
            if moveFlags as i32 & MOVE_RESULT_SUPER_EFFECTIVE != 0 {
                return FALSE;
            }
        }
        i += 1;
    }
    if gBattleTypeFlags.get() & 0x808000 != 0 {
        if gActiveBattler.get() as i32 & BIT_FLANK as i32 == B_FLANK_LEFT {
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
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
        party = (*gPlayerParty.as_ptr()).as_mut_ptr();
    } else {
        party = (*gEnemyParty.as_ptr()).as_mut_ptr();
    }
    for i in firstId..lastId {
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
            if i == ({
                let gl_i2 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i2])
            }) as i32
            {
                break 'l3;
            }
            GetMonData2(party.at(i), MON_DATA_SPECIES);
            GetMonData2(party.at(i), MON_DATA_ABILITY_NUM);
            opposingBattler = GetBattlerAtPosition(opposingPosition);
            for j in 0..MAX_MON_MOVES {
                'l5: {
                    r#move = GetMonData2(party.at(i), MON_DATA_MOVE1 + j) as u16;
                    if r#move == MOVE_NONE {
                        break 'l5;
                    }
                    moveFlags = AI_TypeCalc(
                        r#move,
                        crate::gl!(gBattleMons[opposingBattler].species),
                        crate::gl!(gBattleMons[opposingBattler].ability),
                    );
                    if moveFlags as i32 & 2 != 0 && Random() as i32 % 3 < 2 {
                        *(*crate::gl!(gBattleStruct))
                            .AI_monToSwitchIntoId
                            .as_mut_ptr()
                            .at(gActiveBattler.get()) = i as u8;
                        BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SWITCH, 0);
                        return TRUE;
                    }
                }
            }
        }
    }
    FALSE
}
unsafe fn FindMonThatAbsorbsOpponentsMove() -> u8 {
    let mut battlerIn1: u8 = 0;
    let mut battlerIn2: u8 = 0;
    let mut absorbingTypeAbility: u8 = 0;
    let mut firstId: i32 = 0;
    let mut lastId: i32 = 0;
    let mut party: *mut Pokemon = null_mut();
    if HasSuperEffectiveMoveAgainstOpponents(TRUE) != 0 && Random() as i32 % 3 != 0 {
        return FALSE;
    }
    if {
        let gl_i1 = gActiveBattler.get();
        crate::gl!(gLastLandedMoves[gl_i1])
    } == MOVE_NONE
    {
        return FALSE;
    }
    if {
        let gl_i2 = gActiveBattler.get();
        crate::gl!(gLastLandedMoves[gl_i2])
    } == MOVE_UNAVAILABLE
    {
        return FALSE;
    }
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[{
        let gl_i3 = gActiveBattler.get();
        crate::gl!(gLastLandedMoves[gl_i3])
    }]
    .power
        == 0
    {
        return FALSE;
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
        battlerIn1 = gActiveBattler.get();
        if gAbsentBattlerFlags.get() as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                [GetBattlerAtPosition(GetBattlerPosition(gActiveBattler.get()) ^ 2)]
            != 0
        {
            battlerIn2 = gActiveBattler.get();
        } else {
            battlerIn2 = GetBattlerAtPosition(GetBattlerPosition(gActiveBattler.get()) ^ 2);
        }
    } else {
        battlerIn1 = gActiveBattler.get();
        battlerIn2 = gActiveBattler.get();
    }
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[{
        let gl_i4 = gActiveBattler.get();
        crate::gl!(gLastLandedMoves[gl_i4])
    }]
    .r#type
        == TYPE_FIRE
    {
        absorbingTypeAbility = ABILITY_FLASH_FIRE;
    } else if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[{
        let gl_i5 = gActiveBattler.get();
        crate::gl!(gLastLandedMoves[gl_i5])
    }]
    .r#type
        == TYPE_WATER
    {
        absorbingTypeAbility = ABILITY_WATER_ABSORB;
    } else if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[{
        let gl_i6 = gActiveBattler.get();
        crate::gl!(gLastLandedMoves[gl_i6])
    }]
    .r#type
        == TYPE_ELECTRIC
    {
        absorbingTypeAbility = ABILITY_VOLT_ABSORB;
    } else {
        return FALSE;
    }
    if ({
        let gl_i3 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i3].ability)
    }) == absorbingTypeAbility
    {
        return FALSE;
    }
    if gBattleTypeFlags.get() & 0x808000 != 0 {
        if gActiveBattler.get() as i32 & BIT_FLANK as i32 == B_FLANK_LEFT {
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
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
        party = (*gPlayerParty.as_ptr()).as_mut_ptr();
    } else {
        party = (*gEnemyParty.as_ptr()).as_mut_ptr();
    }
    for i in firstId..lastId {
        'l1: {
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
            if i == crate::gl!(gBattlerPartyIndexes[battlerIn1]) as i32 {
                break 'l1;
            }
            if i == crate::gl!(gBattlerPartyIndexes[battlerIn2]) as i32 {
                break 'l1;
            }
            if i == *(*crate::gl!(gBattleStruct))
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(battlerIn1) as i32
            {
                break 'l1;
            }
            if i == *(*crate::gl!(gBattleStruct))
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(battlerIn2) as i32
            {
                break 'l1;
            }
            let species: u16 = GetMonData2(party.at(i), MON_DATA_SPECIES) as u16;
            if GetMonData2(party.at(i), MON_DATA_ABILITY_NUM) != 0 {
                monAbility = (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .abilities[1];
            } else {
                monAbility = (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .abilities[0];
            }
            if absorbingTypeAbility == monAbility && Random() as i32 & 1 != 0 {
                *(*crate::gl!(gBattleStruct))
                    .AI_monToSwitchIntoId
                    .as_mut_ptr()
                    .at(gActiveBattler.get()) = i as u8;
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SWITCH, 0);
                return TRUE;
            }
        }
    }
    FALSE
}
unsafe fn ShouldSwitchIfNaturalCure() -> u8 {
    if ({
        let gl_i4 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i4].status1)
    }) & STATUS1_SLEEP
        == 0
    {
        return FALSE;
    }
    if ({
        let gl_i5 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i5].ability)
    }) != ABILITY_NATURAL_CURE
    {
        return FALSE;
    }
    if (({
        let gl_i6 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i6].hp)
    }) as i32)
        < ({
            let gl_i7 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i7].maxHP)
        }) as i32
            / 2
    {
        return FALSE;
    }
    if ({
        let gl_i7 = gActiveBattler.get();
        crate::gl!(gLastLandedMoves[gl_i7])
    } == MOVE_NONE
        || {
            let gl_i8 = gActiveBattler.get();
            crate::gl!(gLastLandedMoves[gl_i8])
        } == MOVE_UNAVAILABLE)
        && Random() as i32 & 1 != 0
    {
        *(*crate::gl!(gBattleStruct))
            .AI_monToSwitchIntoId
            .as_mut_ptr()
            .at(gActiveBattler.get()) = PARTY_SIZE as u8;
        BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SWITCH, 0);
        return TRUE;
    } else if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[{
        let gl_i9 = gActiveBattler.get();
        crate::gl!(gLastLandedMoves[gl_i9])
    }]
    .power
        == 0
        && Random() as i32 & 1 != 0
    {
        *(*crate::gl!(gBattleStruct))
            .AI_monToSwitchIntoId
            .as_mut_ptr()
            .at(gActiveBattler.get()) = PARTY_SIZE as u8;
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
        *(*crate::gl!(gBattleStruct))
            .AI_monToSwitchIntoId
            .as_mut_ptr()
            .at(gActiveBattler.get()) = PARTY_SIZE as u8;
        BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SWITCH, 0);
        return TRUE;
    }
    FALSE
}
unsafe fn HasSuperEffectiveMoveAgainstOpponents(noRng: u8) -> u8 {
    let mut moveFlags: u8 = 0;
    let mut r#move: u16 = 0;
    let opposingPosition: u8 = GetBattlerPosition(gActiveBattler.get()) ^ 1;
    let mut opposingBattler: u8 = GetBattlerAtPosition(opposingPosition);
    if gAbsentBattlerFlags.get() as u32
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[opposingBattler]
        == 0
    {
        for i in 0..MAX_MON_MOVES {
            'l1: {
                r#move = {
                    let gl_i8 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i8].moves[i])
                };
                if r#move == MOVE_NONE {
                    break 'l1;
                }
                moveFlags = AI_TypeCalc(
                    r#move,
                    crate::gl!(gBattleMons[opposingBattler].species),
                    crate::gl!(gBattleMons[opposingBattler].ability),
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
        }
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE == 0 {
        return FALSE;
    }
    opposingBattler = GetBattlerAtPosition(opposingPosition ^ 2);
    if gAbsentBattlerFlags.get() as u32
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[opposingBattler]
        == 0
    {
        for i in 0..MAX_MON_MOVES {
            'l3: {
                r#move = {
                    let gl_i9 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i9].moves[i])
                };
                if r#move == MOVE_NONE {
                    break 'l3;
                }
                moveFlags = AI_TypeCalc(
                    r#move,
                    crate::gl!(gBattleMons[opposingBattler].species),
                    crate::gl!(gBattleMons[opposingBattler].ability),
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
        }
    }
    FALSE
}
fn AreStatsRaised() -> u8 {
    let mut buffedStatsValue: u8 = 0;
    for i in 0..NUM_BATTLE_STATS {
        if ({
            let gl_i10 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i10].statStages[i])
        }) > DEFAULT_STAT_STAGE
        {
            buffedStatsValue += ({
                let gl_i11 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i11].statStages[i])
            }) as u8
                - DEFAULT_STAT_STAGE as u8;
        }
    }
    (buffedStatsValue > 3) as u8
}
unsafe fn FindMonWithFlagsAndSuperEffective(flags: u8, moduloPercent: u8) -> u8 {
    let mut battlerIn1: u8 = 0;
    let mut battlerIn2: u8 = 0;
    let mut firstId: i32 = 0;
    let mut lastId: i32 = 0;
    let mut party: *mut Pokemon = null_mut();
    let mut r#move: u16 = 0;
    let mut moveFlags: u8 = 0;
    if {
        let gl_i10 = gActiveBattler.get();
        crate::gl!(gLastLandedMoves[gl_i10])
    } == MOVE_NONE
    {
        return FALSE;
    }
    if {
        let gl_i11 = gActiveBattler.get();
        crate::gl!(gLastLandedMoves[gl_i11])
    } == MOVE_UNAVAILABLE
    {
        return FALSE;
    }
    if {
        let gl_i12 = gActiveBattler.get();
        crate::gl!(gLastHitBy[gl_i12])
    } == 0xFF
    {
        return FALSE;
    }
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[{
        let gl_i13 = gActiveBattler.get();
        crate::gl!(gLastLandedMoves[gl_i13])
    }]
    .power
        == 0
    {
        return FALSE;
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
        battlerIn1 = gActiveBattler.get();
        if gAbsentBattlerFlags.get() as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                [GetBattlerAtPosition(GetBattlerPosition(gActiveBattler.get()) ^ 2)]
            != 0
        {
            battlerIn2 = gActiveBattler.get();
        } else {
            battlerIn2 = GetBattlerAtPosition(GetBattlerPosition(gActiveBattler.get()) ^ 2);
        }
    } else {
        battlerIn1 = gActiveBattler.get();
        battlerIn2 = gActiveBattler.get();
    }
    if gBattleTypeFlags.get() & 0x808000 != 0 {
        if gActiveBattler.get() as i32 & BIT_FLANK as i32 == 0 {
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
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
        party = (*gPlayerParty.as_ptr()).as_mut_ptr();
    } else {
        party = (*gEnemyParty.as_ptr()).as_mut_ptr();
    }
    for i in firstId..lastId {
        'l1: {
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
            if i == crate::gl!(gBattlerPartyIndexes[battlerIn1]) as i32 {
                break 'l1;
            }
            if i == crate::gl!(gBattlerPartyIndexes[battlerIn2]) as i32 {
                break 'l1;
            }
            if i == *(*crate::gl!(gBattleStruct))
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(battlerIn1) as i32
            {
                break 'l1;
            }
            if i == *(*crate::gl!(gBattleStruct))
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(battlerIn2) as i32
            {
                break 'l1;
            }
            let species: u16 = GetMonData2(party.at(i), MON_DATA_SPECIES) as u16;
            if GetMonData2(party.at(i), MON_DATA_ABILITY_NUM) != 0 {
                monAbility = (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .abilities[1];
            } else {
                monAbility = (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .abilities[0];
            }
            moveFlags = AI_TypeCalc(
                {
                    let gl_i14 = gActiveBattler.get();
                    crate::gl!(gLastLandedMoves[gl_i14])
                },
                species,
                monAbility,
            );
            if moveFlags as i32 & flags as i32 != 0 {
                battlerIn1 = {
                    let gl_i15 = gActiveBattler.get();
                    crate::gl!(gLastHitBy[gl_i15])
                };
                for j in 0..MAX_MON_MOVES {
                    'l3: {
                        r#move = GetMonData2(party.at(i), MON_DATA_MOVE1 + j) as u16;
                        if r#move == 0 {
                            break 'l3;
                        }
                        moveFlags = AI_TypeCalc(
                            r#move,
                            crate::gl!(gBattleMons[battlerIn1].species),
                            crate::gl!(gBattleMons[battlerIn1].ability),
                        );
                        if moveFlags as i32 & MOVE_RESULT_SUPER_EFFECTIVE != 0
                            && rem_i32(Random() as i32, moduloPercent as i32) == 0
                        {
                            *(*crate::gl!(gBattleStruct))
                                .AI_monToSwitchIntoId
                                .as_mut_ptr()
                                .at(gActiveBattler.get()) = i as u8;
                            BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SWITCH, 0);
                            return TRUE;
                        }
                    }
                }
            }
        }
    }
    FALSE
}
unsafe fn ShouldSwitch() -> u8 {
    let mut battlerIn1: u8 = 0;
    let mut battlerIn2: u8 = 0;
    let mut activeBattlerPtr: *mut u8 = null_mut();
    let mut firstId: i32 = 0;
    let mut lastId: i32 = 0;
    let mut party: *mut Pokemon = null_mut();
    if ({
        let gl_i12 = *({
            activeBattlerPtr = gActiveBattler.as_ptr();
            activeBattlerPtr
        });
        crate::gl!(gBattleMons[gl_i12].status2)
    }) & 0x400e000
        != 0
    {
        return FALSE;
    }
    if {
        let gl_i16 = gActiveBattler.get();
        crate::gl!(gStatuses3[gl_i16])
    } & STATUS3_ROOTED
        != 0
    {
        return FALSE;
    }
    if AbilityBattleEffects(12, gActiveBattler.get(), ABILITY_SHADOW_TAG, 0, 0) != 0 {
        return FALSE;
    }
    if AbilityBattleEffects(12, gActiveBattler.get(), ABILITY_ARENA_TRAP, 0, 0) != 0 {
        return FALSE;
    }
    if AbilityBattleEffects(14, 0, ABILITY_MAGNET_PULL, 0, 0) != 0
        && (({
            let gl_i13 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i13].types[0])
        }) == TYPE_STEEL
            || ({
                let gl_i14 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i14].types[1])
            }) == TYPE_STEEL)
    {
        return FALSE;
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_ARENA != 0 {
        return FALSE;
    }
    let mut availableToSwitch: i32 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
        battlerIn1 = *activeBattlerPtr;
        if gAbsentBattlerFlags.get() as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                [GetBattlerAtPosition(GetBattlerPosition(*activeBattlerPtr) ^ 2)]
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
    if gBattleTypeFlags.get() & 0x808000 != 0 {
        if gActiveBattler.get() as i32 & BIT_FLANK as i32 == B_FLANK_LEFT {
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
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
        party = (*gPlayerParty.as_ptr()).as_mut_ptr();
    } else {
        party = (*gEnemyParty.as_ptr()).as_mut_ptr();
    }
    for i in firstId..lastId {
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
            if i == crate::gl!(gBattlerPartyIndexes[battlerIn1]) as i32 {
                break 'l1;
            }
            if i == crate::gl!(gBattlerPartyIndexes[battlerIn2]) as i32 {
                break 'l1;
            }
            if i == *(*crate::gl!(gBattleStruct))
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(battlerIn1) as i32
            {
                break 'l1;
            }
            if i == *(*crate::gl!(gBattleStruct))
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(battlerIn2) as i32
            {
                break 'l1;
            }
            availableToSwitch += 1;
        }
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
    FALSE
}
pub unsafe fn AI_TrySwitchOrUseItem() {
    let mut party: *mut Pokemon = null_mut();
    let mut battlerIn1: u8 = 0;
    let mut battlerIn2: u8 = 0;
    let mut firstId: i32 = 0;
    let mut lastId: i32 = 0;
    let battlerIdentity: u8 = GetBattlerPosition(gActiveBattler.get());
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
        party = (*gPlayerParty.as_ptr()).as_mut_ptr();
    } else {
        party = (*gEnemyParty.as_ptr()).as_mut_ptr();
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER != 0 {
        if ShouldSwitch() != 0 {
            if *(*crate::gl!(gBattleStruct))
                .AI_monToSwitchIntoId
                .as_mut_ptr()
                .at(gActiveBattler.get())
                == PARTY_SIZE as u8
            {
                let monToSwitchId: i32 = GetMostSuitableMonToSwitchInto() as i32;
                if monToSwitchId == PARTY_SIZE {
                    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE == 0 {
                        battlerIn1 = GetBattlerAtPosition(battlerIdentity);
                        battlerIn2 = battlerIn1;
                    } else {
                        battlerIn1 = GetBattlerAtPosition(battlerIdentity);
                        battlerIn2 = GetBattlerAtPosition(battlerIdentity ^ 2);
                    }
                    if gBattleTypeFlags.get() & 0x808000 != 0 {
                        if gActiveBattler.get() as i32 & BIT_FLANK as i32 == B_FLANK_LEFT {
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
                    'l2: for monToSwitchId in firstId..lastId {
                        'l1: {
                            if GetMonData2(party.at(monToSwitchId), MON_DATA_HP) == 0 {
                                break 'l1;
                            }
                            if monToSwitchId == crate::gl!(gBattlerPartyIndexes[battlerIn1]) as i32
                            {
                                break 'l1;
                            }
                            if monToSwitchId == crate::gl!(gBattlerPartyIndexes[battlerIn2]) as i32
                            {
                                break 'l1;
                            }
                            if monToSwitchId
                                == *(*crate::gl!(gBattleStruct))
                                    .monToSwitchIntoId
                                    .as_mut_ptr()
                                    .at(battlerIn1) as i32
                            {
                                break 'l1;
                            }
                            if monToSwitchId
                                == *(*crate::gl!(gBattleStruct))
                                    .monToSwitchIntoId
                                    .as_mut_ptr()
                                    .at(battlerIn2) as i32
                            {
                                break 'l1;
                            }
                            break 'l2;
                        }
                    }
                }
                *(*crate::gl!(gBattleStruct))
                    .AI_monToSwitchIntoId
                    .as_mut_ptr()
                    .at(gActiveBattler.get()) = monToSwitchId as u8;
            }
            *(*crate::gl!(gBattleStruct))
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(gActiveBattler.get()) = *(*crate::gl!(gBattleStruct))
                .AI_monToSwitchIntoId
                .as_mut_ptr()
                .at(gActiveBattler.get());
            return;
        } else if ShouldUseItem() != 0 {
            return;
        }
    }
    BtlController_EmitTwoReturnValues(
        B_COMM_TO_ENGINE,
        B_ACTION_USE_MOVE,
        (gActiveBattler.get() as u16 ^ 1) << 8,
    );
}
unsafe fn ModulateByTypeEffectiveness(atkType: u8, defType1: u8, defType2: u8, var: *mut u8) {
    let mut i: i32 = 0;
    while (*(&raw const crate::data::battle_main::gTypeEffectiveness).cast::<CArray<u8, 336>>())[i]
        != TYPE_ENDTABLE
    {
        if (*(&raw const crate::data::battle_main::gTypeEffectiveness).cast::<CArray<u8, 336>>())[i]
            == TYPE_FORESIGHT
        {
            i += 3;
            continue;
        } else if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
            .cast::<CArray<u8, 336>>())[i]
            == atkType
        {
            if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[i + 1]
                == defType1
            {
                *var = (*var as i32
                    * (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                        .cast::<CArray<u8, 336>>())[i + 2] as i32
                    / 10) as u8;
            }
            if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[i + 1]
                == defType2
                && defType1 != defType2
            {
                *var = (*var as i32
                    * (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                        .cast::<CArray<u8, 336>>())[i + 2] as i32
                    / 10) as u8;
            }
        }
        i += 3;
    }
}
pub unsafe fn GetMostSuitableMonToSwitchInto() -> u8 {
    let mut opposingBattler: u8 = 0;
    let mut bestDmg: u8 = 0;
    let mut bestMonId: u8 = 0;
    let mut battlerIn1: u8 = 0;
    let mut battlerIn2: u8 = 0;
    let mut firstId: i32 = 0;
    let mut lastId: i32 = 0;
    let mut party: *mut Pokemon = null_mut();
    let mut i: i32 = 0;
    let mut r#move: u16 = 0;
    if *(*crate::gl!(gBattleStruct))
        .monToSwitchIntoId
        .as_mut_ptr()
        .at(gActiveBattler.get())
        != PARTY_SIZE as u8
    {
        return *(*crate::gl!(gBattleStruct))
            .monToSwitchIntoId
            .as_mut_ptr()
            .at(gActiveBattler.get());
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_ARENA != 0 {
        return ({
            let gl_i15 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i15])
        }) as u8
            + 1;
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
        battlerIn1 = gActiveBattler.get();
        if gAbsentBattlerFlags.get() as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                [GetBattlerAtPosition(GetBattlerPosition(gActiveBattler.get()) ^ 2)]
            != 0
        {
            battlerIn2 = gActiveBattler.get();
        } else {
            battlerIn2 = GetBattlerAtPosition(GetBattlerPosition(gActiveBattler.get()) ^ 2);
        }
        opposingBattler = Random() as u8 & BIT_FLANK;
        if gAbsentBattlerFlags.get() as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[opposingBattler]
            != 0
        {
            opposingBattler ^= BIT_FLANK;
        }
    } else {
        opposingBattler = GetBattlerAtPosition(GetBattlerPosition(gActiveBattler.get()) ^ 1);
        battlerIn1 = gActiveBattler.get();
        battlerIn2 = gActiveBattler.get();
    }
    if gBattleTypeFlags.get() & 0x808000 != 0 {
        if gActiveBattler.get() as i32 & BIT_FLANK as i32 == B_FLANK_LEFT {
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
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
        party = (*gPlayerParty.as_ptr()).as_mut_ptr();
    } else {
        party = (*gEnemyParty.as_ptr()).as_mut_ptr();
    }
    let mut invalidMons: u8 = 0;
    while invalidMons != 63 {
        bestDmg = TYPE_MUL_NO_EFFECT;
        bestMonId = PARTY_SIZE as u8;
        i = firstId;
        while i < lastId {
            let species: u16 = GetMonData2(party.at(i), MON_DATA_SPECIES) as u16;
            if species != SPECIES_NONE
                && GetMonData2(party.at(i), MON_DATA_HP) != 0
                && (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                    & invalidMons as u32
                    == 0
                && crate::gl!(gBattlerPartyIndexes[battlerIn1]) as i32 != i
                && crate::gl!(gBattlerPartyIndexes[battlerIn2]) as i32 != i
                && i != *(*crate::gl!(gBattleStruct))
                    .monToSwitchIntoId
                    .as_mut_ptr()
                    .at(battlerIn1) as i32
                && i != *(*crate::gl!(gBattleStruct))
                    .monToSwitchIntoId
                    .as_mut_ptr()
                    .at(battlerIn2) as i32
            {
                let type1: u8 =
                    (*(&raw const crate::data::pokemon::gSpeciesInfo)
                        .cast::<CArray<SpeciesInfo, 0>>())[species]
                        .types[0];
                let type2: u8 =
                    (*(&raw const crate::data::pokemon::gSpeciesInfo)
                        .cast::<CArray<SpeciesInfo, 0>>())[species]
                        .types[1];
                let mut typeDmg: u8 = TYPE_MUL_NORMAL;
                ModulateByTypeEffectiveness(
                    crate::gl!(gBattleMons[opposingBattler].types[0]),
                    type1,
                    type2,
                    &raw mut typeDmg,
                );
                ModulateByTypeEffectiveness(
                    crate::gl!(gBattleMons[opposingBattler].types[1]),
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
                    && TypeCalc(r#move, gActiveBattler.get(), opposingBattler) as i32
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
    gDynamicBasePower.set(0);
    (*crate::gl!(gBattleStruct)).dynamicMoveType = 0;
    crate::gl!(gBattleScripting.dmgMultiplier = 1);
    gMoveResultFlags.set(0);
    gCritMultiplier.set(1);
    bestDmg = 0;
    bestMonId = PARTY_SIZE as u8;
    for i in firstId..lastId {
        'l4: {
            if GetMonData2(party.at(i), MON_DATA_SPECIES) as u16 == SPECIES_NONE {
                break 'l4;
            }
            if GetMonData2(party.at(i), MON_DATA_HP) == 0 {
                break 'l4;
            }
            if crate::gl!(gBattlerPartyIndexes[battlerIn1]) as i32 == i {
                break 'l4;
            }
            if crate::gl!(gBattlerPartyIndexes[battlerIn2]) as i32 == i {
                break 'l4;
            }
            if i == *(*crate::gl!(gBattleStruct))
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(battlerIn1) as i32
            {
                break 'l4;
            }
            if i == *(*crate::gl!(gBattleStruct))
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(battlerIn2) as i32
            {
                break 'l4;
            }
            for j in 0..MAX_MON_MOVES {
                r#move = GetMonData2(party.at(i), MON_DATA_MOVE1 + j) as u16;
                gBattleMoveDamage.set(0);
                if r#move != MOVE_NONE
                    && (*(&raw const crate::data::pokemon::gBattleMoves)
                        .cast::<CArray<BattleMove, 0>>())[r#move]
                        .power
                        != 1
                {
                    AI_CalcDmg(gActiveBattler.get(), opposingBattler);
                    TypeCalc(r#move, gActiveBattler.get(), opposingBattler);
                }
                if (bestDmg as i32) < gBattleMoveDamage.get() {
                    bestDmg = gBattleMoveDamage.get() as u8;
                    bestMonId = i as u8;
                }
            }
        }
    }
    bestMonId
}
unsafe fn GetAI_ItemType(itemId: u8, itemEffect: *mut u8) -> u8 {
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
        0
    }
}
unsafe fn ShouldUseItem() -> u8 {
    let mut party: *mut Pokemon = null_mut();
    let mut validMons: u8 = 0;
    let mut shouldUse: u8 = FALSE;
    if gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER != 0
        && GetBattlerPosition(gActiveBattler.get()) == B_POSITION_PLAYER_RIGHT
    {
        return FALSE;
    }
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
        party = (*gPlayerParty.as_ptr()).as_mut_ptr();
    } else {
        party = (*gEnemyParty.as_ptr()).as_mut_ptr();
    }
    let mut i: i32 = 0;
    while i < PARTY_SIZE {
        if GetMonData2(party.at(i), MON_DATA_HP) != 0
            && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_NONE as u32
            && GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) != SPECIES_EGG
        {
            validMons += 1;
        }
        i += 1;
    }
    for i in 0..MAX_TRAINER_ITEMS {
        'l2: {
            let mut itemEffects: *mut u8 = null_mut();
            let mut paramOffset: u8 = 0;
            let mut battlerSide: u8 = 0;
            if i != 0
                && validMons as i32
                    > (*(*crate::gl!(gBattleResources)).battleHistory).itemsNo as i32 - i + 1
            {
                break 'l2;
            }
            let item: u16 = (*(*crate::gl!(gBattleResources)).battleHistory).trainerItems[i];
            if item == ITEM_NONE {
                break 'l2;
            }
            if (*(&raw const crate::data::pokemon::gItemEffectTable).cast::<CArray<*mut u8, 0>>())
                [item as i32 - ITEM_POTION]
                .is_null()
            {
                break 'l2;
            }
            if item == ITEM_ENIGMA_BERRY {
                itemEffects = (*crate::gl!(gSaveBlock1Ptr))
                    .enigmaBerry
                    .itemEffect
                    .as_mut_ptr();
            } else {
                itemEffects =
                    (*(&raw const crate::data::pokemon::gItemEffectTable)
                        .cast::<CArray<*mut u8, 0>>())[item as i32 - ITEM_POTION];
            }
            *(*crate::gl!(gBattleStruct))
                .AI_itemType
                .as_mut_ptr()
                .at(gActiveBattler.get() as i32 / 2) = GetAI_ItemType(item as u8, itemEffects);
            'l4: {
                match *(*crate::gl!(gBattleStruct))
                    .AI_itemType
                    .as_mut_ptr()
                    .at(gActiveBattler.get() as i32 / 2)
                {
                    AI_ITEM_FULL_RESTORE => {
                        if ({
                            let gl_i16 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i16].hp)
                        }) as i32
                            >= ({
                                let gl_i17 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i17].maxHP)
                            }) as i32
                                / 4
                        {
                            break 'l4;
                        }
                        if ({
                            let gl_i18 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i18].hp)
                        }) == 0
                        {
                            break 'l4;
                        }
                        shouldUse = TRUE;
                    }
                    AI_ITEM_HEAL_HP => {
                        paramOffset = GetItemEffectParamOffset(item, 4, 0x4);
                        if paramOffset == 0 {
                            break 'l4;
                        }
                        if ({
                            let gl_i19 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i19].hp)
                        }) == 0
                        {
                            break 'l4;
                        }
                        if (({
                            let gl_i20 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i20].hp)
                        }) as i32)
                            < ({
                                let gl_i21 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i21].maxHP)
                            }) as i32
                                / 4
                            || ({
                                let gl_i22 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i22].maxHP)
                            }) as i32
                                - ({
                                    let gl_i23 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i23].hp)
                                }) as i32
                                > *itemEffects.at(paramOffset) as i32
                        {
                            shouldUse = TRUE;
                        }
                    }
                    AI_ITEM_CURE_CONDITION => {
                        *(*crate::gl!(gBattleStruct))
                            .AI_itemFlags
                            .as_mut_ptr()
                            .at(gActiveBattler.get() as i32 / 2) = 0;
                        if *itemEffects.at(3) as i32 & ITEM3_SLEEP != 0
                            && ({
                                let gl_i24 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i24].status1)
                            }) & STATUS1_SLEEP
                                != 0
                        {
                            *(*crate::gl!(gBattleStruct))
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler.get() as i32 / 2) |= 32;
                            shouldUse = TRUE;
                        }
                        if *itemEffects.at(3) as i32 & ITEM3_POISON != 0
                            && (({
                                let gl_i25 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i25].status1)
                            }) & STATUS1_POISON
                                != 0
                                || ({
                                    let gl_i26 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i26].status1)
                                }) & STATUS1_TOXIC_POISON
                                    != 0)
                        {
                            *(*crate::gl!(gBattleStruct))
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler.get() as i32 / 2) |= 16;
                            shouldUse = TRUE;
                        }
                        if *itemEffects.at(3) as i32 & ITEM3_BURN != 0
                            && ({
                                let gl_i27 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i27].status1)
                            }) & STATUS1_BURN
                                != 0
                        {
                            *(*crate::gl!(gBattleStruct))
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler.get() as i32 / 2) |= 8;
                            shouldUse = TRUE;
                        }
                        if *itemEffects.at(3) as i32 & ITEM3_FREEZE != 0
                            && ({
                                let gl_i28 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i28].status1)
                            }) & STATUS1_FREEZE
                                != 0
                        {
                            *(*crate::gl!(gBattleStruct))
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler.get() as i32 / 2) |= 4;
                            shouldUse = TRUE;
                        }
                        if *itemEffects.at(3) as i32 & ITEM3_PARALYSIS != 0
                            && ({
                                let gl_i29 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i29].status1)
                            }) & STATUS1_PARALYSIS
                                != 0
                        {
                            *(*crate::gl!(gBattleStruct))
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler.get() as i32 / 2) |= 2;
                            shouldUse = TRUE;
                        }
                        if *itemEffects.at(3) as i32 & ITEM3_CONFUSION != 0
                            && ({
                                let gl_i30 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i30].status2)
                            }) & STATUS2_CONFUSION
                                != 0
                        {
                            *(*crate::gl!(gBattleStruct))
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler.get() as i32 / 2) |= 1;
                            shouldUse = TRUE;
                        }
                    }
                    AI_ITEM_X_STAT => {
                        *(*crate::gl!(gBattleStruct))
                            .AI_itemFlags
                            .as_mut_ptr()
                            .at(gActiveBattler.get() as i32 / 2) = 0;
                        if ({
                            let gl_i31 = gActiveBattler.get();
                            crate::gl!(gDisableStructs[gl_i31].isFirstTurn)
                        }) == 0
                        {
                            break 'l4;
                        }
                        if *itemEffects as i32 & ITEM0_X_ATTACK != 0 {
                            *(*crate::gl!(gBattleStruct))
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler.get() as i32 / 2) |= 1;
                        }
                        if *itemEffects.at(1) as i32 & ITEM1_X_DEFEND != 0 {
                            *(*crate::gl!(gBattleStruct))
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler.get() as i32 / 2) |= 2;
                        }
                        if *itemEffects.at(1) as i32 & ITEM1_X_SPEED != 0 {
                            *(*crate::gl!(gBattleStruct))
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler.get() as i32 / 2) |= 4;
                        }
                        if *itemEffects.at(2) as i32 & ITEM2_X_SPATK != 0 {
                            *(*crate::gl!(gBattleStruct))
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler.get() as i32 / 2) |= 8;
                        }
                        if *itemEffects.at(2) as i32 & ITEM2_X_ACCURACY != 0 {
                            *(*crate::gl!(gBattleStruct))
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler.get() as i32 / 2) |= 32;
                        }
                        if *itemEffects as i32 & ITEM0_DIRE_HIT != 0 {
                            *(*crate::gl!(gBattleStruct))
                                .AI_itemFlags
                                .as_mut_ptr()
                                .at(gActiveBattler.get() as i32 / 2) |= 128;
                        }
                        shouldUse = TRUE;
                    }
                    AI_ITEM_GUARD_SPEC => {
                        battlerSide = GetBattlerSide(gActiveBattler.get());
                        if ({
                            let gl_i32 = gActiveBattler.get();
                            crate::gl!(gDisableStructs[gl_i32].isFirstTurn)
                        }) != 0
                            && crate::gl!(gSideTimers[battlerSide].mistTimer) == 0
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
                *(*crate::gl!(gBattleStruct))
                    .chosenItem
                    .as_mut_ptr()
                    .at(gActiveBattler.get() as i32 / 2 * 2) = item as u8;
                (*(*crate::gl!(gBattleResources)).battleHistory).trainerItems[i] = ITEM_NONE;
                return shouldUse;
            }
        }
    }
    FALSE
}
