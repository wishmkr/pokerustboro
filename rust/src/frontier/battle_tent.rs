//! Translated from `src/battle_tent.c` by tools/rustport/c2rs.py.
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
    clippy::modulo_one,
    clippy::type_complexity,
    unused_assignments
)]

use crate::battle_factory_screen::{DoBattleFactorySelectScreen, DoBattleFactorySwapScreen};
use crate::battle_setup::gTrainerBattleOpponent_A;
use crate::battle_tower::gFrontierTempParty;
use crate::battle_tower::{
    FrontierSpeechToString, GetFrontierTrainerName, SetBattleFacilityTrainerGfxId,
    gFacilityTrainerMons, gFacilityTrainers,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::VarSet;
use crate::ffi::{gSpecialVar_0x8005, gSpecialVar_Result};
use crate::fieldmap::gMapHeader;
use crate::frontier_util::SaveGameFrontier;
use crate::item::{AddBagItem, CopyItemName};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::overworld::SetDynamicWarp;
use crate::pokemon::ZeroPlayerPartyMons;
use crate::random::Random;
use crate::string_util::gStringVar1;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): sVerdanturfTentFuncs sVerdanturfTentRewards sFallarborTentFuncs sFallarborTentRewards sSlateportTentFuncs sSlateportTentRewards

static sFallarborTentFuncs: Table<CArray<Option<unsafe fn()>, 7>> =
    Table((&raw const crate::data::battle_tent::sFallarborTentFuncs).cast());
static sFallarborTentRewards: Table<CArray<u16, 1>> =
    Table((&raw const crate::data::battle_tent::sFallarborTentRewards).cast());
static sSlateportTentFuncs: Table<CArray<Option<unsafe fn()>, 10>> =
    Table((&raw const crate::data::battle_tent::sSlateportTentFuncs).cast());
static sSlateportTentRewards: Table<CArray<u16, 1>> =
    Table((&raw const crate::data::battle_tent::sSlateportTentRewards).cast());
static sVerdanturfTentFuncs: Table<CArray<Option<unsafe fn()>, 8>> =
    Table((&raw const crate::data::battle_tent::sVerdanturfTentFuncs).cast());
static sVerdanturfTentRewards: Table<CArray<u16, 1>> =
    Table((&raw const crate::data::battle_tent::sVerdanturfTentRewards).cast());

pub(crate) static sRandMonId: crate::global::Global<u16> = crate::global::Global::new(0);

#[unsafe(no_mangle)]
pub unsafe fn CallVerdanturfTentFunction() {
    sVerdanturfTentFuncs[*(&raw const crate::ffi::gSpecialVar_0x8004)
        .cast::<u16>()
        .cast_mut()]
    .unwrap_unchecked()();
}
pub(crate) unsafe fn InitVerdanturfTentChallenge() {
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
pub(crate) unsafe fn GetVerdanturfTentPrize() {
    gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.verdanturfTentPrize;
}
pub(crate) unsafe fn SetVerdanturfTentPrize() {
    (*gSaveBlock2Ptr).frontier.verdanturfTentPrize = *(&raw const crate::ffi::gSpecialVar_0x8006)
        .cast::<u16>()
        .cast_mut();
}
pub(crate) unsafe fn SetVerdanturfTentTrainerGfx() {
    gTrainerBattleOpponent_A = ((Random() as i32 % 255) as u32 * 5 / 64) as u16;
    SetBattleFacilityTrainerGfxId(gTrainerBattleOpponent_A, 0);
}
pub(crate) unsafe fn BufferVerdanturfTentTrainerIntro() {
    if gTrainerBattleOpponent_A < FRONTIER_TRAINERS_COUNT {
        FrontierSpeechToString(
            (*gFacilityTrainers.at(gTrainerBattleOpponent_A))
                .speechBefore
                .as_mut_ptr(),
        );
    }
}
pub(crate) unsafe fn SaveVerdanturfTentChallenge() {
    (*gSaveBlock2Ptr).frontier.challengeStatus = gSpecialVar_0x8005 as u8;
    VarSet(VAR_TEMP_CHALLENGE_STATUS, 0);
    (*gSaveBlock2Ptr).frontier.set_challengePaused(TRUE);
    SaveGameFrontier();
}
pub(crate) unsafe fn SetRandomVerdanturfTentPrize() {
    (*gSaveBlock2Ptr).frontier.verdanturfTentPrize = sVerdanturfTentRewards[Random() % 1];
}
pub(crate) unsafe fn GiveVerdanturfTentPrize() {
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
pub unsafe fn CallFallarborTentFunction() {
    sFallarborTentFuncs[*(&raw const crate::ffi::gSpecialVar_0x8004)
        .cast::<u16>()
        .cast_mut()]
    .unwrap_unchecked()();
}
pub(crate) unsafe fn InitFallarborTentChallenge() {
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
pub(crate) unsafe fn GetFallarborTentPrize() {
    gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.fallarborTentPrize;
}
pub(crate) unsafe fn SetFallarborTentPrize() {
    (*gSaveBlock2Ptr).frontier.fallarborTentPrize = *(&raw const crate::ffi::gSpecialVar_0x8006)
        .cast::<u16>()
        .cast_mut();
}
pub(crate) unsafe fn SaveFallarborTentChallenge() {
    (*gSaveBlock2Ptr).frontier.challengeStatus = gSpecialVar_0x8005 as u8;
    VarSet(VAR_TEMP_CHALLENGE_STATUS, 0);
    (*gSaveBlock2Ptr).frontier.set_challengePaused(TRUE);
    SaveGameFrontier();
}
pub(crate) unsafe fn SetRandomFallarborTentPrize() {
    (*gSaveBlock2Ptr).frontier.fallarborTentPrize = sFallarborTentRewards[Random() % 1];
}
pub(crate) unsafe fn GiveFallarborTentPrize() {
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
pub(crate) unsafe fn BufferFallarborTentTrainerName() {
    GetFrontierTrainerName(gStringVar1.as_mut_ptr(), gTrainerBattleOpponent_A);
}
#[unsafe(no_mangle)]
pub unsafe fn CallSlateportTentFunction() {
    sSlateportTentFuncs[*(&raw const crate::ffi::gSpecialVar_0x8004)
        .cast::<u16>()
        .cast_mut()]
    .unwrap_unchecked()();
}
pub(crate) unsafe fn InitSlateportTentChallenge() {
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
pub(crate) unsafe fn GetSlateportTentPrize() {
    gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.slateportTentPrize;
}
pub(crate) unsafe fn SetSlateportTentPrize() {
    (*gSaveBlock2Ptr).frontier.slateportTentPrize = *(&raw const crate::ffi::gSpecialVar_0x8006)
        .cast::<u16>()
        .cast_mut();
}
pub(crate) unsafe fn SaveSlateportTentChallenge() {
    (*gSaveBlock2Ptr).frontier.challengeStatus = gSpecialVar_0x8005 as u8;
    VarSet(VAR_TEMP_CHALLENGE_STATUS, 0);
    (*gSaveBlock2Ptr).frontier.set_challengePaused(TRUE);
    SaveGameFrontier();
}
pub(crate) unsafe fn SetRandomSlateportTentPrize() {
    (*gSaveBlock2Ptr).frontier.slateportTentPrize = sSlateportTentRewards[Random() % 1];
}
pub(crate) unsafe fn GiveSlateportTentPrize() {
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
pub(crate) unsafe fn SelectInitialRentalMons() {
    ZeroPlayerPartyMons();
    DoBattleFactorySelectScreen();
}
pub(crate) unsafe fn SwapRentalMons() {
    DoBattleFactorySwapScreen();
}
pub unsafe fn InSlateportBattleTent() -> u8 {
    (gMapHeader.regionMapSectionId == MAPSEC_SLATEPORT_CITY as u8
        && (gMapHeader.mapLayoutId == LAYOUT_BATTLE_TENT_CORRIDOR
            || gMapHeader.mapLayoutId == LAYOUT_BATTLE_TENT_BATTLE_ROOM)) as u8
}
pub(crate) unsafe fn GenerateInitialRentalMons() {
    let mut j: i32 = 0;
    let mut monSetId: u16 = 0;
    let mut species: CArray<u16, 6> = zeroed();
    let mut monIds: CArray<u16, 6> = zeroed();
    let mut heldItems: CArray<u16, 6> = zeroed();
    let firstMonId: u8 = 0;
    gFacilityTrainers = (*(&raw const crate::data::battle_tower::gSlateportBattleTentTrainers)
        .cast::<CArray<BattleFrontierTrainer, 0>>())
    .as_ptr()
    .cast_mut();
    for i in 0..PARTY_SIZE {
        species[i] = 0;
        monIds[i] = 0;
        heldItems[i] = 0;
    }
    gFacilityTrainerMons = (*(&raw const crate::data::battle_tower::gSlateportBattleTentMons)
        .cast::<CArray<FacilityMon, 0>>())
    .as_ptr()
    .cast_mut();
    let mut currSpecies: u16 = SPECIES_NONE;
    let mut i: i32 = 0;
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
                    == (*(&raw const crate::data::battle_tower::gBattleFrontierHeldItems)
                        .cast::<CArray<u16, 0>>())[(*gFacilityTrainerMons.at(monSetId)).itemTableId]
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
        heldItems[i] = (*(&raw const crate::data::battle_tower::gBattleFrontierHeldItems)
            .cast::<CArray<u16, 0>>())[(*gFacilityTrainerMons.at(monSetId)).itemTableId];
        monIds[i] = monSetId;
        i += 1;
    }
}
pub(crate) unsafe fn GenerateOpponentMons() {
    let mut trainerId: u16 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut monSet: *mut u16 = null_mut();
    let mut species: CArray<u16, 3> = zeroed();
    let mut heldItems: CArray<u16, 3> = zeroed();
    let mut numMons: i32 = 0;
    gFacilityTrainers = (*(&raw const crate::data::battle_tower::gSlateportBattleTentTrainers)
        .cast::<CArray<BattleFrontierTrainer, 0>>())
    .as_ptr()
    .cast_mut();
    gFacilityTrainerMons = (*(&raw const crate::data::battle_tower::gSlateportBattleTentMons)
        .cast::<CArray<FacilityMon, 0>>())
    .as_ptr()
    .cast_mut();
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
        sRandMonId.set(*monSet.at(rem_i32(Random() as i32, numMons)));
        j = 0;
        while j < 6 {
            if (*gFacilityTrainerMons.at(sRandMonId.get())).species
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
            if species[k] == (*gFacilityTrainerMons.at(sRandMonId.get())).species {
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
                        .cast::<CArray<u16, 0>>())
                        [(*gFacilityTrainerMons.at(sRandMonId.get())).itemTableId]
            {
                break;
            }
            k += 1;
        }
        if k != i {
            continue;
        }
        species[i] = (*gFacilityTrainerMons.at(sRandMonId.get())).species;
        heldItems[i] = (*(&raw const crate::data::battle_tower::gBattleFrontierHeldItems)
            .cast::<CArray<u16, 0>>())[(*gFacilityTrainerMons.at(sRandMonId.get())).itemTableId];
        gFrontierTempParty[i] = sRandMonId.get();
        i += 1;
    }
}
