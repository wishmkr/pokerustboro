//! Translated from `src/battle_factory.c` by tools/rustport/c2rs.py.
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
    clippy::type_complexity,
    unused_assignments
)]

use crate::battle_factory_screen::{DoBattleFactorySelectScreen, DoBattleFactorySwapScreen};
use crate::battle_setup::gTrainerBattleOpponent_A;
use crate::battle_tower::gFrontierTempParty;
use crate::battle_tower::{
    GetRandomScaledFrontierTrainerId, SetBattleFacilityTrainerGfxId, SetFacilityPtrsGetLevel,
    gFacilityTrainerMons, gFacilityTrainers,
};
use crate::box_mon::GetBoxMonData3;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{VarGet, VarSet};
use crate::ffi::{gSpecialVar_0x8005, gSpecialVar_0x8006, gSpecialVar_Result};
use crate::fieldmap::gMapHeader;
use crate::frontier_util::SaveGameFrontier;
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::overworld::SetDynamicWarp;
use crate::pokemon::{
    CalculateMonStats, CreateMon, CreateMonWithEVSpreadNatureOTID, GetMonData3, SetMonData,
    SetMonMoveSlot, ZeroPlayerPartyMons, gEnemyParty, gPlayerParty,
};
use crate::random::Random;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): sRequiredMoveCounts sMoves_TotalPreparation sMoves_ImpossibleToPredict sMoves_WeakeningTheFoe sMoves_HighRiskHighReturn sMoves_Endurance sMoves_SlowAndSteady sMoves_DependsOnTheBattlesFlow sMoveStyles sBattleFactoryFunctions sWinStreakFlags sWinStreakMasks sFixedIVTable sInitialRentalMonRanges

static sBattleFactoryFunctions: Table<CArray<Option<unsafe fn()>, 17>> =
    Table((&raw const crate::data::battle_factory::sBattleFactoryFunctions).cast());
static sFixedIVTable: Table<CArray<CArray<u8, 2>, 8>> =
    Table((&raw const crate::data::battle_factory::sFixedIVTable).cast());
static sInitialRentalMonRanges: Table<CArray<CArray<u16, 2>, 16>> =
    Table((&raw const crate::data::battle_factory::sInitialRentalMonRanges).cast());
static sMoveStyles: Table<CArray<*mut u16, 7>> =
    Table((&raw const crate::data::battle_factory::sMoveStyles).cast());
static sRequiredMoveCounts: Table<CArray<u8, 7>> =
    Table((&raw const crate::data::battle_factory::sRequiredMoveCounts).cast());
static sWinStreakFlags: Table<CArray<CArray<u32, 2>, 2>> =
    Table((&raw const crate::data::battle_factory::sWinStreakFlags).cast());
static sWinStreakMasks: Table<CArray<CArray<u32, 2>, 2>> =
    Table((&raw const crate::data::battle_factory::sWinStreakMasks).cast());

pub(crate) static sPerformedRentalSwap: crate::global::Global<u8> = crate::global::Global::new(0);

#[unsafe(no_mangle)]
pub unsafe fn CallBattleFactoryFunction() {
    sBattleFactoryFunctions[*(&raw const crate::ffi::gSpecialVar_0x8004)
        .cast::<u16>()
        .cast_mut()]
    .unwrap_unchecked()();
}
pub(crate) unsafe fn InitFactoryChallenge() {
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let battleMode: u32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u32;
    (*gSaveBlock2Ptr).frontier.challengeStatus = 0;
    (*gSaveBlock2Ptr).frontier.curChallengeBattleNum = 0;
    (*gSaveBlock2Ptr).frontier.set_challengePaused(FALSE);
    (*gSaveBlock2Ptr).frontier.set_disableRecordBattle(FALSE);
    if (*gSaveBlock2Ptr).frontier.winStreakActiveFlags & sWinStreakFlags[battleMode][lvlMode] == 0 {
        (*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode] = 0;
        (*gSaveBlock2Ptr).frontier.factoryRentsCount[battleMode][lvlMode] = 0;
    }
    sPerformedRentalSwap.set(FALSE);
    let mut i: u8 = 0;
    while i < 6 {
        (*gSaveBlock2Ptr).frontier.rentalMons[i].monId = 0xFFFF;
        i += 1;
    }
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        gFrontierTempParty[i] = 0xFFFF;
    }
    SetDynamicWarp(
        0,
        (*gSaveBlock1Ptr).location.mapGroup,
        (*gSaveBlock1Ptr).location.mapNum,
        WARP_ID_NONE,
    );
    gTrainerBattleOpponent_A = 0;
}
pub(crate) unsafe fn GetBattleFactoryData() {
    let lvlMode: i32 = (*gSaveBlock2Ptr).frontier.lvlMode() as i32;
    let battleMode: i32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as i32;
    match *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut()
    {
        FACTORY_DATA_WIN_STREAK => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode];
        }
        FACTORY_DATA_WIN_STREAK_ACTIVE => {
            gSpecialVar_Result = ((*gSaveBlock2Ptr).frontier.winStreakActiveFlags
                & sWinStreakFlags[battleMode][lvlMode]
                != 0) as u16;
        }
        FACTORY_DATA_WIN_STREAK_SWAPS => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.factoryRentsCount[battleMode][lvlMode];
        }
        _ => {}
    }
}
pub(crate) unsafe fn SetBattleFactoryData() {
    let lvlMode: i32 = (*gSaveBlock2Ptr).frontier.lvlMode() as i32;
    let battleMode: i32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as i32;
    match *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut()
    {
        FACTORY_DATA_WIN_STREAK => {
            (*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode] =
                *(&raw const crate::ffi::gSpecialVar_0x8006)
                    .cast::<u16>()
                    .cast_mut();
        }
        FACTORY_DATA_WIN_STREAK_ACTIVE => {
            if gSpecialVar_0x8006 != 0 {
                (*gSaveBlock2Ptr).frontier.winStreakActiveFlags |=
                    sWinStreakFlags[battleMode][lvlMode];
            } else {
                (*gSaveBlock2Ptr).frontier.winStreakActiveFlags &=
                    sWinStreakMasks[battleMode][lvlMode];
            }
        }
        FACTORY_DATA_WIN_STREAK_SWAPS if sPerformedRentalSwap.get() == TRUE => {
            (*gSaveBlock2Ptr).frontier.factoryRentsCount[battleMode][lvlMode] =
                *(&raw const crate::ffi::gSpecialVar_0x8006)
                    .cast::<u16>()
                    .cast_mut();
            sPerformedRentalSwap.set(FALSE);
        }
        _ => {}
    }
}
pub(crate) unsafe fn SaveFactoryChallenge() {
    (*gSaveBlock2Ptr).frontier.challengeStatus = gSpecialVar_0x8005 as u8;
    VarSet(VAR_TEMP_CHALLENGE_STATUS, 0);
    (*gSaveBlock2Ptr).frontier.set_challengePaused(TRUE);
    SaveGameFrontier();
}
pub(crate) fn FactoryDummy1() {}
pub(crate) fn FactoryDummy2() {}
pub(crate) unsafe fn SelectInitialRentalMons() {
    ZeroPlayerPartyMons();
    DoBattleFactorySelectScreen();
}
pub(crate) unsafe fn SwapRentalMons() {
    DoBattleFactorySwapScreen();
}
pub(crate) fn SetPerformedRentalSwap() {
    sPerformedRentalSwap.set(TRUE);
}
pub(crate) unsafe fn GenerateOpponentMons() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut species: CArray<u16, 3> = zeroed();
    let mut heldItems: CArray<u16, 3> = zeroed();
    let firstMonId: i32 = 0;
    let mut trainerId: u16 = 0;
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let battleMode: u32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u32;
    let winStreak: u32 = (*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode] as u32;
    let challengeNum: u32 = winStreak / 7;
    gFacilityTrainers = (*(&raw const crate::data::battle_tower::gBattleFrontierTrainers)
        .cast::<CArray<BattleFrontierTrainer, 0>>())
    .as_ptr()
    .cast_mut();
    loop {
        trainerId = GetRandomScaledFrontierTrainerId(
            challengeNum as u8,
            (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as u8,
        );
        i = 0;
        while i < (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 {
            if (*gSaveBlock2Ptr).frontier.trainerIds[i] == trainerId {
                break;
            }
            i += 1;
        }
        if i == (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 {
            break;
        }
    }
    gTrainerBattleOpponent_A = trainerId;
    if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum < 6 {
        (*gSaveBlock2Ptr).frontier.trainerIds[(*gSaveBlock2Ptr).frontier.curChallengeBattleNum] =
            trainerId;
    }
    i = 0;
    while i != FRONTIER_PARTY_SIZE {
        let monId: u16 = GetFactoryMonId(lvlMode as u8, challengeNum as u8, FALSE);
        if (*gFacilityTrainerMons.at(monId)).species == SPECIES_UNOWN {
            continue;
        }
        j = 0;
        while j < 6 {
            if (*gFacilityTrainerMons.at(monId)).species
                == (*gFacilityTrainerMons.at((*gSaveBlock2Ptr).frontier.rentalMons[j].monId))
                    .species
            {
                break;
            }
            j += 1;
        }
        if j != 6 {
            continue;
        }
        if lvlMode == FRONTIER_LVL_50 as u32 && monId > FRONTIER_MONS_HIGH_TIER {
            continue;
        }
        k = firstMonId;
        while k < firstMonId + i {
            if species[k] == (*gFacilityTrainerMons.at(monId)).species {
                break;
            }
            k += 1;
        }
        if k != firstMonId + i {
            continue;
        }
        k = firstMonId;
        while k < firstMonId + i {
            if heldItems[k] != ITEM_NONE
                && heldItems[k]
                    == (*(&raw const crate::data::battle_tower::gBattleFrontierHeldItems)
                        .cast::<CArray<u16, 0>>())[(*gFacilityTrainerMons.at(monId)).itemTableId]
            {
                break;
            }
            k += 1;
        }
        if k != firstMonId + i {
            continue;
        }
        species[i] = (*gFacilityTrainerMons.at(monId)).species;
        heldItems[i] = (*(&raw const crate::data::battle_tower::gBattleFrontierHeldItems)
            .cast::<CArray<u16, 0>>())[(*gFacilityTrainerMons.at(monId)).itemTableId];
        gFrontierTempParty[i] = monId;
        i += 1;
    }
}
pub(crate) unsafe fn SetOpponentGfxVar() {
    SetBattleFacilityTrainerGfxId(gTrainerBattleOpponent_A, 0);
}
pub(crate) unsafe fn SetRentalsToOpponentParty() {
    if (*gSaveBlock2Ptr).frontier.lvlMode() != FRONTIER_LVL_TENT {
        gFacilityTrainerMons = (*(&raw const crate::data::battle_tower::gBattleFrontierMons)
            .cast::<CArray<FacilityMon, 0>>())
        .as_ptr()
        .cast_mut();
    } else {
        gFacilityTrainerMons = (*(&raw const crate::data::battle_tower::gSlateportBattleTentMons)
            .cast::<CArray<FacilityMon, 0>>())
        .as_ptr()
        .cast_mut();
    }
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        (*gSaveBlock2Ptr).frontier.rentalMons[i as i32 + FRONTIER_PARTY_SIZE].monId =
            gFrontierTempParty[i];
        (*gSaveBlock2Ptr).frontier.rentalMons[i as i32 + FRONTIER_PARTY_SIZE].ivs =
            GetBoxMonData3(&raw mut gEnemyParty[i].r#box, MON_DATA_ATK_IV, null_mut()) as u8;
        (*gSaveBlock2Ptr).frontier.rentalMons[i as i32 + FRONTIER_PARTY_SIZE].personality =
            GetMonData3(&raw mut gEnemyParty[i], MON_DATA_PERSONALITY, null_mut());
        (*gSaveBlock2Ptr).frontier.rentalMons[i as i32 + FRONTIER_PARTY_SIZE].abilityNum =
            GetBoxMonData3(
                &raw mut gEnemyParty[i].r#box,
                MON_DATA_ABILITY_NUM,
                null_mut(),
            ) as u8;
        SetMonData(
            &raw mut gEnemyParty[i],
            MON_DATA_HELD_ITEM,
            (&raw const (*(&raw const crate::data::battle_tower::gBattleFrontierHeldItems)
                .cast::<CArray<u16, 0>>())
                [(*gFacilityTrainerMons.at(gFrontierTempParty[i])).itemTableId])
                .cast_mut() as *mut c_void,
        );
    }
}
pub(crate) unsafe fn SetPlayerAndOpponentParties() {
    let mut j: i32 = 0;
    let mut count: i32 = 0;
    let mut bits: u8 = 0;
    let mut monLevel: u8 = 0;
    let mut monId: u16 = 0;
    let mut evs: u16 = 0;
    let mut ivs: u8 = 0;
    let mut friendship: u8 = 0;
    if (*gSaveBlock2Ptr).frontier.lvlMode() == FRONTIER_LVL_TENT {
        gFacilityTrainerMons = (*(&raw const crate::data::battle_tower::gSlateportBattleTentMons)
            .cast::<CArray<FacilityMon, 0>>())
        .as_ptr()
        .cast_mut();
        monLevel = TENT_MIN_LEVEL;
    } else {
        gFacilityTrainerMons = (*(&raw const crate::data::battle_tower::gBattleFrontierMons)
            .cast::<CArray<FacilityMon, 0>>())
        .as_ptr()
        .cast_mut();
        if (*gSaveBlock2Ptr).frontier.lvlMode() != FRONTIER_LVL_50 {
            monLevel = FRONTIER_MAX_LEVEL_OPEN;
        } else {
            monLevel = FRONTIER_MAX_LEVEL_50;
        }
    }
    if gSpecialVar_0x8005 < 2 {
        ZeroPlayerPartyMons();
        for i in 0..FRONTIER_PARTY_SIZE {
            monId = (*gSaveBlock2Ptr).frontier.rentalMons[i].monId;
            ivs = (*gSaveBlock2Ptr).frontier.rentalMons[i].ivs;
            CreateMon(
                &raw mut gPlayerParty[i],
                (*gFacilityTrainerMons.at(monId)).species,
                monLevel,
                ivs,
                TRUE,
                (*gSaveBlock2Ptr).frontier.rentalMons[i].personality,
                0,
                0,
            );
            count = 0;
            bits = (*gFacilityTrainerMons.at(monId)).evSpread;
            j = 0;
            while j < NUM_STATS {
                if bits as i32 & 1 != 0 {
                    count += 1;
                }
                bits >>= 1;
                j += 1;
            }
            evs = div_i32(MAX_TOTAL_EVS, count) as u16;
            bits = 1;
            for j in 0..NUM_STATS {
                if (*gFacilityTrainerMons.at(monId)).evSpread as i32 & bits as i32 != 0 {
                    SetMonData(
                        &raw mut gPlayerParty[i],
                        MON_DATA_HP_EV + j,
                        &raw mut evs as *mut c_void,
                    );
                }
                bits <<= 1;
            }
            CalculateMonStats(&raw mut gPlayerParty[i]);
            friendship = 0;
            for k in 0..MAX_MON_MOVES {
                SetMonMoveAvoidReturn(
                    &raw mut gPlayerParty[i],
                    (*gFacilityTrainerMons.at(monId)).moves[k],
                    k as u8,
                );
            }
            SetMonData(
                &raw mut gPlayerParty[i],
                MON_DATA_FRIENDSHIP,
                &raw mut friendship as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[i],
                MON_DATA_HELD_ITEM,
                (&raw const (*(&raw const crate::data::battle_tower::gBattleFrontierHeldItems)
                    .cast::<CArray<u16, 0>>())[(*gFacilityTrainerMons.at(monId)).itemTableId])
                    .cast_mut() as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[i],
                MON_DATA_ABILITY_NUM,
                &raw mut (*gSaveBlock2Ptr).frontier.rentalMons[i].abilityNum as *mut c_void,
            );
        }
    }
    match *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut()
    {
        0 | 2 => {
            for i in 0..FRONTIER_PARTY_SIZE {
                monId = (*gSaveBlock2Ptr).frontier.rentalMons[i + FRONTIER_PARTY_SIZE].monId;
                ivs = (*gSaveBlock2Ptr).frontier.rentalMons[i + FRONTIER_PARTY_SIZE].ivs;
                CreateMon(
                    &raw mut gEnemyParty[i],
                    (*gFacilityTrainerMons.at(monId)).species,
                    monLevel,
                    ivs,
                    TRUE,
                    (*gSaveBlock2Ptr).frontier.rentalMons[i + FRONTIER_PARTY_SIZE].personality,
                    0,
                    0,
                );
                count = 0;
                bits = (*gFacilityTrainerMons.at(monId)).evSpread;
                j = 0;
                while j < NUM_STATS {
                    if bits as i32 & 1 != 0 {
                        count += 1;
                    }
                    bits >>= 1;
                    j += 1;
                }
                evs = div_i32(MAX_TOTAL_EVS, count) as u16;
                bits = 1;
                for j in 0..NUM_STATS {
                    if (*gFacilityTrainerMons.at(monId)).evSpread as i32 & bits as i32 != 0 {
                        SetMonData(
                            &raw mut gEnemyParty[i],
                            MON_DATA_HP_EV + j,
                            &raw mut evs as *mut c_void,
                        );
                    }
                    bits <<= 1;
                }
                CalculateMonStats(&raw mut gEnemyParty[i]);
                for k in 0..MAX_MON_MOVES {
                    SetMonMoveAvoidReturn(
                        &raw mut gEnemyParty[i],
                        (*gFacilityTrainerMons.at(monId)).moves[k],
                        k as u8,
                    );
                }
                SetMonData(
                    &raw mut gEnemyParty[i],
                    MON_DATA_HELD_ITEM,
                    (&raw const (*(&raw const crate::data::battle_tower::gBattleFrontierHeldItems)
                        .cast::<CArray<u16, 0>>())[(*gFacilityTrainerMons.at(monId)).itemTableId])
                        .cast_mut() as *mut c_void,
                );
                SetMonData(
                    &raw mut gEnemyParty[i],
                    MON_DATA_ABILITY_NUM,
                    &raw mut (*gSaveBlock2Ptr).frontier.rentalMons[i + FRONTIER_PARTY_SIZE]
                        .abilityNum as *mut c_void,
                );
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn GenerateInitialRentalMons() {
    let mut j: i32 = 0;
    let mut firstMonId: u8 = 0;
    let mut lvlMode: u8 = 0;
    let mut factoryLvlMode: u8 = 0;
    let mut factoryBattleMode: u8 = 0;
    let mut monId: u16 = 0;
    let mut species: CArray<u16, 6> = zeroed();
    let mut monIds: CArray<u16, 6> = zeroed();
    let mut heldItems: CArray<u16, 6> = zeroed();
    gFacilityTrainers = (*(&raw const crate::data::battle_tower::gBattleFrontierTrainers)
        .cast::<CArray<BattleFrontierTrainer, 0>>())
    .as_ptr()
    .cast_mut();
    for i in 0..PARTY_SIZE {
        species[i] = SPECIES_NONE;
        monIds[i] = 0;
        heldItems[i] = ITEM_NONE;
    }
    lvlMode = (*gSaveBlock2Ptr).frontier.lvlMode();
    let battleMode: u8 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u8;
    let challengeNum: u8 =
        ((*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode] as i32 / 7) as u8;
    if VarGet(VAR_FRONTIER_BATTLE_MODE) == FRONTIER_MODE_DOUBLES {
        factoryBattleMode = FRONTIER_MODE_DOUBLES as u8;
    } else {
        factoryBattleMode = FRONTIER_MODE_SINGLES as u8;
    }
    gFacilityTrainerMons = (*(&raw const crate::data::battle_tower::gBattleFrontierMons)
        .cast::<CArray<FacilityMon, 0>>())
    .as_ptr()
    .cast_mut();
    if (*gSaveBlock2Ptr).frontier.lvlMode() != FRONTIER_LVL_50 {
        factoryLvlMode = FRONTIER_LVL_OPEN;
        firstMonId = 0;
    } else {
        factoryLvlMode = FRONTIER_LVL_50;
        firstMonId = 0;
    }
    let rentalRank: u8 = GetNumPastRentalsRank(factoryBattleMode, factoryLvlMode);
    let mut currSpecies: u16 = SPECIES_NONE;
    let mut i: i32 = 0;
    while i != PARTY_SIZE {
        if i < rentalRank as i32 {
            monId = GetFactoryMonId(factoryLvlMode, challengeNum, TRUE);
        } else {
            monId = GetFactoryMonId(factoryLvlMode, challengeNum, FALSE);
        }
        if (*gFacilityTrainerMons.at(monId)).species == SPECIES_UNOWN {
            continue;
        }
        j = firstMonId as i32;
        while j < firstMonId as i32 + i {
            let existingMonId: u16 = monIds[j];
            if existingMonId == monId {
                break;
            }
            if species[j] == (*gFacilityTrainerMons.at(monId)).species {
                if currSpecies == SPECIES_NONE {
                    currSpecies = (*gFacilityTrainerMons.at(monId)).species;
                } else {
                    break;
                }
            }
            j += 1;
        }
        if j != firstMonId as i32 + i {
            continue;
        }
        j = firstMonId as i32;
        while j < firstMonId as i32 + i {
            if heldItems[j] != ITEM_NONE
                && heldItems[j]
                    == (*(&raw const crate::data::battle_tower::gBattleFrontierHeldItems)
                        .cast::<CArray<u16, 0>>())[(*gFacilityTrainerMons.at(monId)).itemTableId]
            {
                if (*gFacilityTrainerMons.at(monId)).species == currSpecies {
                    currSpecies = SPECIES_NONE;
                }
                break;
            }
            j += 1;
        }
        if j != firstMonId as i32 + i {
            continue;
        }
        (*gSaveBlock2Ptr).frontier.rentalMons[i].monId = monId;
        species[i] = (*gFacilityTrainerMons.at(monId)).species;
        heldItems[i] = (*(&raw const crate::data::battle_tower::gBattleFrontierHeldItems)
            .cast::<CArray<u16, 0>>())[(*gFacilityTrainerMons.at(monId)).itemTableId];
        monIds[i] = monId;
        i += 1;
    }
}
pub(crate) unsafe fn GetOpponentMostCommonMonType() {
    let mut typeCounts: CArray<u8, 18> = zeroed();
    let mut mostCommonTypes: CArray<u8, 2> = zeroed();
    gFacilityTrainerMons = (*(&raw const crate::data::battle_tower::gBattleFrontierMons)
        .cast::<CArray<FacilityMon, 0>>())
    .as_ptr()
    .cast_mut();
    for i in TYPE_NORMAL..NUMBER_OF_MON_TYPES {
        typeCounts[i] = 0;
    }
    let mut i: u8 = 0;
    while i < FRONTIER_PARTY_SIZE as u8 {
        let species: u32 = (*gFacilityTrainerMons.at(gFrontierTempParty[i])).species as u32;
        typeCounts[(*(&raw const crate::data::pokemon::gSpeciesInfo)
            .cast::<CArray<SpeciesInfo, 0>>())[species]
            .types[0]] += 1;
        if (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
            [species]
            .types[0]
            != (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [species]
                .types[1]
        {
            typeCounts[(*(&raw const crate::data::pokemon::gSpeciesInfo)
                .cast::<CArray<SpeciesInfo, 0>>())[species]
                .types[1]] += 1;
        }
        i += 1;
    }
    mostCommonTypes[0] = 0;
    mostCommonTypes[1] = 0;
    for i in 1..NUMBER_OF_MON_TYPES {
        if typeCounts[mostCommonTypes[0]] < typeCounts[i] {
            mostCommonTypes[0] = i;
        } else if typeCounts[mostCommonTypes[0]] == typeCounts[i] {
            mostCommonTypes[1] = i;
        }
    }
    if typeCounts[mostCommonTypes[0]] != 0 {
        if typeCounts[mostCommonTypes[0]] > typeCounts[mostCommonTypes[1]] {
            gSpecialVar_Result = mostCommonTypes[0] as u16;
        } else if mostCommonTypes[0] == mostCommonTypes[1] {
            gSpecialVar_Result = mostCommonTypes[0] as u16;
        } else {
            gSpecialVar_Result = NUMBER_OF_MON_TYPES as u16;
        }
    } else {
        gSpecialVar_Result = NUMBER_OF_MON_TYPES as u16;
    }
}
pub(crate) unsafe fn GetOpponentBattleStyle() {
    let mut stylePoints: CArray<u8, 8> = zeroed();
    let mut count: u8 = 0;
    gFacilityTrainerMons = (*(&raw const crate::data::battle_tower::gBattleFrontierMons)
        .cast::<CArray<FacilityMon, 0>>())
    .as_ptr()
    .cast_mut();
    for i in 0..FACTORY_NUM_STYLES {
        stylePoints[i] = 0;
    }
    let mut i: u8 = 0;
    while i < FRONTIER_PARTY_SIZE as u8 {
        let monId: u16 = gFrontierTempParty[i];
        for j in 0..(MAX_MON_MOVES as u8) {
            let battleStyle: u8 = GetMoveBattleStyle((*gFacilityTrainerMons.at(monId)).moves[j]);
            stylePoints[battleStyle] += 1;
        }
        i += 1;
    }
    gSpecialVar_Result = FACTORY_STYLE_NONE;
    for i in 1..FACTORY_NUM_STYLES {
        if stylePoints[i] >= sRequiredMoveCounts[i as i32 - 1] {
            gSpecialVar_Result = i as u16;
            count += 1;
        }
    }
    if count > 2 {
        gSpecialVar_Result = FACTORY_NUM_STYLES as u16;
    }
}
unsafe fn GetMoveBattleStyle(r#move: u16) -> u8 {
    let mut moves: *mut u16 = null_mut();
    let mut j: u8 = 0;
    for i in 0..7u8 {
        j = 0;
        moves = sMoveStyles[i];
        while *moves.at(j) != 0 {
            if *moves.at(j) == r#move {
                return i + 1;
            }
            j += 1;
        }
    }
    FACTORY_STYLE_NONE as u8
}
pub unsafe fn InBattleFactory() -> u8 {
    (gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_FACTORY_PRE_BATTLE_ROOM
        || gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_FACTORY_BATTLE_ROOM) as u8
}
pub(crate) unsafe fn RestorePlayerPartyHeldItems() {
    if (*gSaveBlock2Ptr).frontier.lvlMode() != FRONTIER_LVL_TENT {
        gFacilityTrainerMons = (*(&raw const crate::data::battle_tower::gBattleFrontierMons)
            .cast::<CArray<FacilityMon, 0>>())
        .as_ptr()
        .cast_mut();
    } else {
        gFacilityTrainerMons = (*(&raw const crate::data::battle_tower::gSlateportBattleTentMons)
            .cast::<CArray<FacilityMon, 0>>())
        .as_ptr()
        .cast_mut();
    }
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        SetMonData(
            &raw mut gPlayerParty[i],
            MON_DATA_HELD_ITEM,
            (&raw const (*(&raw const crate::data::battle_tower::gBattleFrontierHeldItems)
                .cast::<CArray<u16, 0>>())[(*gFacilityTrainerMons
                .at((*gSaveBlock2Ptr).frontier.rentalMons[i].monId))
            .itemTableId])
                .cast_mut() as *mut c_void,
        );
    }
}
pub fn GetFactoryMonFixedIV(challengeNum: u8, isLastBattle: u8) -> u8 {
    let mut ivSet: u8 = 0;
    let useHigherIV: u8 = (if isLastBattle != 0 {
        TRUE as i32
    } else {
        FALSE as i32
    }) as u8;
    if challengeNum > 8 {
        ivSet = 7;
    } else {
        ivSet = challengeNum;
    }
    sFixedIVTable[ivSet][useHigherIV]
}
pub unsafe fn FillFactoryBrainParty() {
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut species: CArray<u16, 3> = zeroed();
    let mut heldItems: CArray<u16, 3> = zeroed();
    let mut friendship: u8 = 0;
    let lvlMode: u8 = (*gSaveBlock2Ptr).frontier.lvlMode();
    let battleMode: u8 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u8;
    let challengeNum: u8 =
        ((*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode] as i32 / 7) as u8;
    let fixedIV: u8 = GetFactoryMonFixedIV(challengeNum + 2, FALSE);
    let monLevel: i32 = SetFacilityPtrsGetLevel() as i32;
    let mut i: i32 = 0;
    let otId: u32 = (*gSaveBlock2Ptr).playerTrainerId[0] as u32
        | ((*gSaveBlock2Ptr).playerTrainerId[1] as u32) << 8
        | ((*gSaveBlock2Ptr).playerTrainerId[2] as u32) << 16
        | ((*gSaveBlock2Ptr).playerTrainerId[3] as u32) << 24;
    while i != FRONTIER_PARTY_SIZE {
        let monId: u16 = GetFactoryMonId(lvlMode, challengeNum, FALSE);
        if (*gFacilityTrainerMons.at(monId)).species == SPECIES_UNOWN {
            continue;
        }
        if monLevel == FRONTIER_MAX_LEVEL_50 as i32 && monId > FRONTIER_MONS_HIGH_TIER {
            continue;
        }
        j = 0;
        while j < 6 {
            if monId == (*gSaveBlock2Ptr).frontier.rentalMons[j].monId {
                break;
            }
            j += 1;
        }
        if j != 6 {
            continue;
        }
        k = 0;
        while k < i {
            if species[k] == (*gFacilityTrainerMons.at(monId)).species {
                break;
            }
            k += 1;
        }
        if k != i {
            continue;
        }
        k = 0;
        while k < i {
            if heldItems[k] != ITEM_NONE
                && heldItems[k]
                    == (*(&raw const crate::data::battle_tower::gBattleFrontierHeldItems)
                        .cast::<CArray<u16, 0>>())[(*gFacilityTrainerMons.at(monId)).itemTableId]
            {
                break;
            }
            k += 1;
        }
        if k != i {
            continue;
        }
        species[i] = (*gFacilityTrainerMons.at(monId)).species;
        heldItems[i] = (*(&raw const crate::data::battle_tower::gBattleFrontierHeldItems)
            .cast::<CArray<u16, 0>>())[(*gFacilityTrainerMons.at(monId)).itemTableId];
        CreateMonWithEVSpreadNatureOTID(
            &raw mut gEnemyParty[i],
            (*gFacilityTrainerMons.at(monId)).species,
            monLevel as u8,
            (*gFacilityTrainerMons.at(monId)).nature,
            fixedIV,
            (*gFacilityTrainerMons.at(monId)).evSpread,
            otId,
        );
        friendship = 0;
        for k in 0..MAX_MON_MOVES {
            SetMonMoveAvoidReturn(
                &raw mut gEnemyParty[i],
                (*gFacilityTrainerMons.at(monId)).moves[k],
                k as u8,
            );
        }
        SetMonData(
            &raw mut gEnemyParty[i],
            MON_DATA_FRIENDSHIP,
            &raw mut friendship as *mut c_void,
        );
        SetMonData(
            &raw mut gEnemyParty[i],
            MON_DATA_HELD_ITEM,
            (&raw const (*(&raw const crate::data::battle_tower::gBattleFrontierHeldItems)
                .cast::<CArray<u16, 0>>())[(*gFacilityTrainerMons.at(monId)).itemTableId])
                .cast_mut() as *mut c_void,
        );
        i += 1;
    }
}
fn GetFactoryMonId(lvlMode: u8, challengeNum: u8, useBetterRange: u8) -> u16 {
    let mut numMons: u16 = 0;
    let mut monId: u16 = 0;
    let mut adder: u16 = 0;
    if lvlMode == FRONTIER_LVL_50 {
        adder = 0;
    } else {
        adder = 8;
    }
    if challengeNum < 7 {
        if useBetterRange != 0 {
            numMons = sInitialRentalMonRanges[adder as i32 + challengeNum as i32 + 1][1]
                - sInitialRentalMonRanges[adder as i32 + challengeNum as i32 + 1][0]
                + 1;
            monId = rem_i32(Random() as i32, numMons as i32) as u16;
            monId += sInitialRentalMonRanges[adder as i32 + challengeNum as i32 + 1][0];
        } else {
            numMons = sInitialRentalMonRanges[adder as i32 + challengeNum as i32][1]
                - sInitialRentalMonRanges[adder as i32 + challengeNum as i32][0]
                + 1;
            monId = rem_i32(Random() as i32, numMons as i32) as u16;
            monId += sInitialRentalMonRanges[adder as i32 + challengeNum as i32][0];
        }
    } else {
        let mut challenge: u16 = challengeNum as u16;
        if challenge != 7 {
            challenge = 7;
        }
        numMons = sInitialRentalMonRanges[adder as i32 + challenge as i32][1]
            - sInitialRentalMonRanges[adder as i32 + challenge as i32][0]
            + 1;
        monId = rem_i32(Random() as i32, numMons as i32) as u16;
        monId += sInitialRentalMonRanges[adder as i32 + challenge as i32][0];
    }
    monId
}
pub unsafe fn GetNumPastRentalsRank(battleMode: u8, lvlMode: u8) -> u8 {
    let mut ret: u8 = 0;
    let rents: u8 = (*gSaveBlock2Ptr).frontier.factoryRentsCount[battleMode][lvlMode] as u8;
    if rents < 15 {
        ret = 0;
    } else if rents < 22 {
        ret = 1;
    } else if rents < 29 {
        ret = 2;
    } else if rents < 36 {
        ret = 3;
    } else if rents < 43 {
        ret = 4;
    } else {
        ret = 5;
    }
    ret
}
pub unsafe fn GetAiScriptsInBattleFactory() -> u32 {
    let lvlMode: i32 = (*gSaveBlock2Ptr).frontier.lvlMode() as i32;
    if lvlMode == FRONTIER_LVL_TENT as i32 {
        return 0;
    } else {
        let battleMode: i32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as i32;
        let challengeNum: i32 =
            (*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode] as i32 / 7;
        if gTrainerBattleOpponent_A == TRAINER_FRONTIER_BRAIN {
            return 7;
        } else if challengeNum < 2 {
            return 0;
        } else if challengeNum < 4 {
            return AI_SCRIPT_CHECK_BAD_MOVE;
        } else {
            return 7;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn SetMonMoveAvoidReturn(mon: *mut Pokemon, moveArg: u16, moveSlot: u8) {
    let mut r#move: u16 = moveArg;
    if moveArg == MOVE_RETURN {
        r#move = MOVE_FRUSTRATION;
    }
    SetMonMoveSlot(mon, r#move, moveSlot);
}
