//! Translated from `src/battle_tent.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sVerdanturfTentFuncs sVerdanturfTentRewards sFallarborTentFuncs sFallarborTentRewards sSlateportTentFuncs sSlateportTentRewards

static sFallarborTentFuncs: Table<CArray<Option<unsafe extern "C" fn()>, 7>> =
    Table((&raw const crate::data::battle_tent::sFallarborTentFuncs).cast());
static sFallarborTentRewards: Table<CArray<u16, 1>> =
    Table((&raw const crate::data::battle_tent::sFallarborTentRewards).cast());
static sSlateportTentFuncs: Table<CArray<Option<unsafe extern "C" fn()>, 10>> =
    Table((&raw const crate::data::battle_tent::sSlateportTentFuncs).cast());
static sSlateportTentRewards: Table<CArray<u16, 1>> =
    Table((&raw const crate::data::battle_tent::sSlateportTentRewards).cast());
static sVerdanturfTentFuncs: Table<CArray<Option<unsafe extern "C" fn()>, 8>> =
    Table((&raw const crate::data::battle_tent::sVerdanturfTentFuncs).cast());
static sVerdanturfTentRewards: Table<CArray<u16, 1>> =
    Table((&raw const crate::data::battle_tent::sVerdanturfTentRewards).cast());

pub(crate) static mut sRandMonId: u16 = 0;

unsafe extern "C" {
    static gBattleFrontierHeldItems: CArray<u16, 0>;
    static mut gFacilityTrainerMons: *mut FacilityMon;
    static mut gFacilityTrainers: *mut BattleFrontierTrainer;
    static mut gFrontierTempParty: CArray<u16, 0>;
    static mut gMapHeader: MapHeader;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static gSlateportBattleTentMons: CArray<FacilityMon, 0>;
    static gSlateportBattleTentTrainers: CArray<BattleFrontierTrainer, 0>;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSpecialVar_0x8006: u16;
    static mut gSpecialVar_Result: u16;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gTrainerBattleOpponent_A: u16;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn DoBattleFactorySelectScreen();
    fn DoBattleFactorySwapScreen();
    fn FrontierSpeechToString(a0: *mut u16);
    fn GetFrontierTrainerName(a0: *mut u8, a1: u16);
    fn Random() -> u16;
    fn SaveGameFrontier();
    fn SetBattleFacilityTrainerGfxId(a0: u16, a1: u8);
    fn SetDynamicWarp(a0: i32, a1: i8, a2: i8, a3: i8);
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn ZeroPlayerPartyMons();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallVerdanturfTentFunction() {
    sVerdanturfTentFuncs[gSpecialVar_0x8004].unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn InitVerdanturfTentChallenge() {
    (*gSaveBlock2Ptr).frontier.challengeStatus = 0;
    (*gSaveBlock2Ptr).frontier.curChallengeBattleNum = 0;
    (*gSaveBlock2Ptr).frontier.set_challengePaused(FALSE);
    SetDynamicWarp(
        0,
        (*gSaveBlock1Ptr).location.mapGroup,
        (*gSaveBlock1Ptr).location.mapNum,
        WARP_ID_NONE,
    );
}
pub(crate) unsafe extern "C" fn GetVerdanturfTentPrize() {
    gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.verdanturfTentPrize;
}
pub(crate) unsafe extern "C" fn SetVerdanturfTentPrize() {
    (*gSaveBlock2Ptr).frontier.verdanturfTentPrize = gSpecialVar_0x8006;
}
pub(crate) unsafe extern "C" fn SetVerdanturfTentTrainerGfx() {
    gTrainerBattleOpponent_A = ((Random() as i32 % 255) as u32 * 5 / 64) as u16;
    SetBattleFacilityTrainerGfxId(gTrainerBattleOpponent_A, 0);
}
pub(crate) unsafe extern "C" fn BufferVerdanturfTentTrainerIntro() {
    if gTrainerBattleOpponent_A < FRONTIER_TRAINERS_COUNT {
        FrontierSpeechToString(
            (*gFacilityTrainers.at(gTrainerBattleOpponent_A))
                .speechBefore
                .as_mut_ptr(),
        );
    }
}
pub(crate) unsafe extern "C" fn SaveVerdanturfTentChallenge() {
    (*gSaveBlock2Ptr).frontier.challengeStatus = gSpecialVar_0x8005 as u8;
    VarSet(VAR_TEMP_CHALLENGE_STATUS, 0);
    (*gSaveBlock2Ptr).frontier.set_challengePaused(TRUE);
    SaveGameFrontier();
}
pub(crate) unsafe extern "C" fn SetRandomVerdanturfTentPrize() {
    (*gSaveBlock2Ptr).frontier.verdanturfTentPrize = sVerdanturfTentRewards[Random() % 1];
}
pub(crate) unsafe extern "C" fn GiveVerdanturfTentPrize() {
    if AddBagItem((*gSaveBlock2Ptr).frontier.verdanturfTentPrize, 1) == 1 {
        CopyItemName(
            (*gSaveBlock2Ptr).frontier.verdanturfTentPrize,
            gStringVar1.as_mut_ptr(),
        );
        (*gSaveBlock2Ptr).frontier.verdanturfTentPrize = ITEM_NONE;
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallFallarborTentFunction() {
    sFallarborTentFuncs[gSpecialVar_0x8004].unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn InitFallarborTentChallenge() {
    (*gSaveBlock2Ptr).frontier.challengeStatus = 0;
    (*gSaveBlock2Ptr).frontier.curChallengeBattleNum = 0;
    (*gSaveBlock2Ptr).frontier.set_challengePaused(FALSE);
    SetDynamicWarp(
        0,
        (*gSaveBlock1Ptr).location.mapGroup,
        (*gSaveBlock1Ptr).location.mapNum,
        WARP_ID_NONE,
    );
}
pub(crate) unsafe extern "C" fn GetFallarborTentPrize() {
    gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.fallarborTentPrize;
}
pub(crate) unsafe extern "C" fn SetFallarborTentPrize() {
    (*gSaveBlock2Ptr).frontier.fallarborTentPrize = gSpecialVar_0x8006;
}
pub(crate) unsafe extern "C" fn SaveFallarborTentChallenge() {
    (*gSaveBlock2Ptr).frontier.challengeStatus = gSpecialVar_0x8005 as u8;
    VarSet(VAR_TEMP_CHALLENGE_STATUS, 0);
    (*gSaveBlock2Ptr).frontier.set_challengePaused(TRUE);
    SaveGameFrontier();
}
pub(crate) unsafe extern "C" fn SetRandomFallarborTentPrize() {
    (*gSaveBlock2Ptr).frontier.fallarborTentPrize = sFallarborTentRewards[Random() % 1];
}
pub(crate) unsafe extern "C" fn GiveFallarborTentPrize() {
    if AddBagItem((*gSaveBlock2Ptr).frontier.fallarborTentPrize, 1) == 1 {
        CopyItemName(
            (*gSaveBlock2Ptr).frontier.fallarborTentPrize,
            gStringVar1.as_mut_ptr(),
        );
        (*gSaveBlock2Ptr).frontier.fallarborTentPrize = ITEM_NONE;
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
pub(crate) unsafe extern "C" fn BufferFallarborTentTrainerName() {
    GetFrontierTrainerName(gStringVar1.as_mut_ptr(), gTrainerBattleOpponent_A);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallSlateportTentFunction() {
    sSlateportTentFuncs[gSpecialVar_0x8004].unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn InitSlateportTentChallenge() {
    (*gSaveBlock2Ptr).frontier.challengeStatus = 0;
    (*gSaveBlock2Ptr).frontier.curChallengeBattleNum = 0;
    (*gSaveBlock2Ptr).frontier.set_challengePaused(FALSE);
    SetDynamicWarp(
        0,
        (*gSaveBlock1Ptr).location.mapGroup,
        (*gSaveBlock1Ptr).location.mapNum,
        WARP_ID_NONE,
    );
}
pub(crate) unsafe extern "C" fn GetSlateportTentPrize() {
    gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.slateportTentPrize;
}
pub(crate) unsafe extern "C" fn SetSlateportTentPrize() {
    (*gSaveBlock2Ptr).frontier.slateportTentPrize = gSpecialVar_0x8006;
}
pub(crate) unsafe extern "C" fn SaveSlateportTentChallenge() {
    (*gSaveBlock2Ptr).frontier.challengeStatus = gSpecialVar_0x8005 as u8;
    VarSet(VAR_TEMP_CHALLENGE_STATUS, 0);
    (*gSaveBlock2Ptr).frontier.set_challengePaused(TRUE);
    SaveGameFrontier();
}
pub(crate) unsafe extern "C" fn SetRandomSlateportTentPrize() {
    (*gSaveBlock2Ptr).frontier.slateportTentPrize = sSlateportTentRewards[Random() % 1];
}
pub(crate) unsafe extern "C" fn GiveSlateportTentPrize() {
    if AddBagItem((*gSaveBlock2Ptr).frontier.slateportTentPrize, 1) == 1 {
        CopyItemName(
            (*gSaveBlock2Ptr).frontier.slateportTentPrize,
            gStringVar1.as_mut_ptr(),
        );
        (*gSaveBlock2Ptr).frontier.slateportTentPrize = ITEM_NONE;
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
pub(crate) unsafe extern "C" fn SelectInitialRentalMons() {
    ZeroPlayerPartyMons();
    DoBattleFactorySelectScreen();
}
pub(crate) unsafe extern "C" fn SwapRentalMons() {
    DoBattleFactorySwapScreen();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InSlateportBattleTent() -> u8 {
    return (gMapHeader.regionMapSectionId == MAPSEC_SLATEPORT_CITY as u8
        && (gMapHeader.mapLayoutId == LAYOUT_BATTLE_TENT_CORRIDOR
            || gMapHeader.mapLayoutId == LAYOUT_BATTLE_TENT_BATTLE_ROOM)) as u8;
}
pub(crate) unsafe extern "C" fn GenerateInitialRentalMons() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut firstMonId: u8 = 0;
    let mut monSetId: u16 = 0;
    let mut currSpecies: u16 = 0;
    let mut species: CArray<u16, 6> = zeroed();
    let mut monIds: CArray<u16, 6> = zeroed();
    let mut heldItems: CArray<u16, 6> = zeroed();
    firstMonId = 0;
    gFacilityTrainers = gSlateportBattleTentTrainers.as_ptr().cast_mut();
    i = 0;
    while i < PARTY_SIZE {
        species[i] = 0;
        monIds[i] = 0;
        heldItems[i] = 0;
        i += 1;
    }
    gFacilityTrainerMons = gSlateportBattleTentMons.as_ptr().cast_mut();
    currSpecies = SPECIES_NONE;
    i = 0;
    while i != PARTY_SIZE {
        monSetId = (Random() as i32 % 70) as u16;
        j = firstMonId as i32;
        while j < firstMonId as i32 + i {
            if monIds[j] == monSetId {
                break;
            }
            if species[j] == (*gFacilityTrainerMons.at(monSetId)).species {
                if currSpecies == SPECIES_NONE {
                    currSpecies = (*gFacilityTrainerMons.at(monSetId)).species;
                } else {
                    break;
                }
            }
            j += 1;
        }
        if j != i + firstMonId as i32 {
            continue;
        }
        j = firstMonId as i32;
        while j < i + firstMonId as i32 {
            if heldItems[j] != 0
                && heldItems[j]
                    == gBattleFrontierHeldItems[(*gFacilityTrainerMons.at(monSetId)).itemTableId]
            {
                if (*gFacilityTrainerMons.at(monSetId)).species == currSpecies {
                    currSpecies = SPECIES_NONE;
                }
                break;
            }
            j += 1;
        }
        if j != i + firstMonId as i32 {
            continue;
        }
        (*gSaveBlock2Ptr).frontier.rentalMons[i].monId = monSetId;
        species[i] = (*gFacilityTrainerMons.at(monSetId)).species;
        heldItems[i] = gBattleFrontierHeldItems[(*gFacilityTrainerMons.at(monSetId)).itemTableId];
        monIds[i] = monSetId;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GenerateOpponentMons() {
    let mut trainerId: u16 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut monSet: *mut u16 = null_mut();
    let mut species: CArray<u16, 3> = zeroed();
    let mut heldItems: CArray<u16, 3> = zeroed();
    let mut numMons: i32 = 0;
    gFacilityTrainers = gSlateportBattleTentTrainers.as_ptr().cast_mut();
    gFacilityTrainerMons = gSlateportBattleTentMons.as_ptr().cast_mut();
    loop {
        loop {
            trainerId = (Random() as i32 % 30) as u16;
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
        monSet = (*gFacilityTrainers.at(gTrainerBattleOpponent_A)).monSet;
        while *monSet.at(numMons) != 0xFFFF {
            numMons += 1;
        }
        if numMons > 8 {
            break;
        }
        numMons = 0;
    }
    if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum < 2 {
        (*gSaveBlock2Ptr).frontier.trainerIds[(*gSaveBlock2Ptr).frontier.curChallengeBattleNum] =
            gTrainerBattleOpponent_A;
    }
    monSet = (*gFacilityTrainers.at(gTrainerBattleOpponent_A)).monSet;
    i = 0;
    while i != FRONTIER_PARTY_SIZE {
        sRandMonId = *monSet.at(rem_i32(Random() as i32, numMons));
        j = 0;
        while j < 6 {
            if (*gFacilityTrainerMons.at(sRandMonId)).species
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
        k = 0;
        while k < i {
            if species[k] == (*gFacilityTrainerMons.at(sRandMonId)).species {
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
                    == gBattleFrontierHeldItems[(*gFacilityTrainerMons.at(sRandMonId)).itemTableId]
            {
                break;
            }
            k += 1;
        }
        if k != i {
            continue;
        }
        species[i] = (*gFacilityTrainerMons.at(sRandMonId)).species;
        heldItems[i] = gBattleFrontierHeldItems[(*gFacilityTrainerMons.at(sRandMonId)).itemTableId];
        gFrontierTempParty[i] = sRandMonId;
        i += 1;
    }
}
