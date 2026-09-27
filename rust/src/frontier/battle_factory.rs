//! Translated from `src/battle_factory.c` by tools/rustport/c2rs.py, then reviewed.
#![allow(
    non_snake_case,
    non_upper_case_globals,
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
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons
)]

// Data tables (translate with cdata.py): sRequiredMoveCounts sMoves_TotalPreparation sMoves_ImpossibleToPredict sMoves_WeakeningTheFoe sMoves_HighRiskHighReturn sMoves_Endurance sMoves_SlowAndSteady sMoves_DependsOnTheBattlesFlow sMoveStyles sBattleFactoryFunctions sWinStreakFlags sWinStreakMasks sFixedIVTable sInitialRentalMonRanges
#[allow(unused_imports)]
use crate::data::battle_factory::*;

pub(crate) static mut sPerformedRentalSwap: u8 = 0u8;

unsafe extern "C" {
    static mut gBattleFrontierHeldItems: u8;
    static mut gBattleFrontierMons: u8;
    static mut gBattleFrontierTrainers: u8;
    static mut gEnemyParty: u8;
    static mut gFacilityTrainerMons: u8;
    static mut gFacilityTrainers: u8;
    static mut gFrontierTempParty: u8;
    static mut gMapHeader: u8;
    static mut gPlayerParty: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSlateportBattleTentMons: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesInfo: u8;
    static mut gTrainerBattleOpponent_A: u8;
    fn CalculateMonStats(a0: *mut u8);
    fn CreateMon(a0: *mut u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u32, a6: u8, a7: u32);
    fn CreateMonWithEVSpreadNatureOTID(
        a0: *mut u8,
        a1: u16,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u32,
    );
    fn DoBattleFactorySelectScreen();
    fn DoBattleFactorySwapScreen();
    fn GetBoxMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetRandomScaledFrontierTrainerId(a0: u8, a1: u8) -> u16;
    fn Random() -> u16;
    fn SaveGameFrontier();
    fn SetBattleFacilityTrainerGfxId(a0: u16, a1: u8);
    fn SetDynamicWarp(a0: i32, a1: i8, a2: i8, a3: i8);
    fn SetFacilityPtrsGetLevel() -> u8;
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetMonMoveSlot(a0: *mut u8, a1: u16, a2: u8);
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn ZeroPlayerPartyMons();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallBattleFactoryFunction() {
    unsafe {
        (((((&raw const sBattleFactoryFunctions)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn InitFactoryChallenge() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        let mut battleMode: u32 = ((VarGet(16590u16)) as u32);
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1628))
        .write(0u8);
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1638)
            .cast::<u16>())
        .write(0u16);
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            2,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            3,
            1,
            (0u8) as i32,
        );
        if !(((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1680)
            .cast::<u32>())
        .read()
            & ((((((&raw const sWinStreakFlags).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((battleMode) as i32) as isize * 8))
            .cast::<u32>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .read())
            != 0)
        {
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1942))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .write(0u16);
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1958))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .write(0u16);
        }
        ((&raw mut sPerformedRentalSwap).cast::<u8>().cast::<u8>()).write(0u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(72u32, 12u32)) {
                    break 'l1;
                }
                'l2: {
                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(2084))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<u16>())
                    .write(65535u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l3;
                }
                'l4: {
                    ((((&raw mut gFrontierTempParty).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(65535u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        SetDynamicWarp(
            0i32,
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<i8>())
                .read(),
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read(),
            (-1i8),
        );
        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(0u16);
    }
}
pub(crate) unsafe extern "C" fn GetBattleFactoryData() {
    unsafe {
        let mut lvlMode: i32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32);
        let mut battleMode: i32 = ((VarGet(16590u16)) as i32);
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 1i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1942))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1680)
                    .cast::<u32>())
                    .read()
                        & ((((((&raw const sWinStreakFlags).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((battleMode) as isize * 8))
                        .cast::<u32>())
                        .wrapping_offset((lvlMode) as isize))
                        .read())
                        != 0u32) as u16),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1958))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read(),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetBattleFactoryData() {
    unsafe {
        let mut lvlMode: i32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32);
        let mut battleMode: i32 = ((VarGet(16590u16)) as i32);
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 1i32 {
                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1942))
                .cast::<u8>())
                .wrapping_offset((battleMode) as isize * 4))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .write(((&raw mut gSpecialVar_0x8006).cast::<u16>()).read());
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) != 0 {
                    let __p2 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1680)
                    .cast::<u32>();
                    (__p2).write(
                        ((__p2).read()
                            | ((((((&raw const sWinStreakFlags).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((battleMode) as isize * 8))
                            .cast::<u32>())
                            .wrapping_offset((lvlMode) as isize))
                            .read()),
                    );
                } else {
                    let __p3 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1680)
                    .cast::<u32>();
                    (__p3).write(
                        ((__p3).read()
                            & ((((((&raw const sWinStreakMasks).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((battleMode) as isize * 8))
                            .cast::<u32>())
                            .wrapping_offset((lvlMode) as isize))
                            .read()),
                    );
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((&raw mut sPerformedRentalSwap).cast::<u8>().cast::<u8>()).read()) as i32)
                    == 1i32
                {
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1958))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .write(((&raw mut gSpecialVar_0x8006).cast::<u16>()).read());
                    ((&raw mut sPerformedRentalSwap).cast::<u8>().cast::<u8>()).write(0u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SaveFactoryChallenge() {
    unsafe {
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1628))
        .write(((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8));
        VarSet(16384u16, 0u16);
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            2,
            1,
            (1u8) as i32,
        );
        SaveGameFrontier();
    }
}
pub(crate) unsafe extern "C" fn FactoryDummy1() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn FactoryDummy2() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn SelectInitialRentalMons() {
    unsafe {
        ZeroPlayerPartyMons();
        DoBattleFactorySelectScreen();
    }
}
pub(crate) unsafe extern "C" fn SwapRentalMons() {
    unsafe {
        DoBattleFactorySwapScreen();
    }
}
pub(crate) unsafe extern "C" fn SetPerformedRentalSwap() {
    unsafe {
        ((&raw mut sPerformedRentalSwap).cast::<u8>().cast::<u8>()).write(1u8);
    }
}
pub(crate) unsafe extern "C" fn GenerateOpponentMons() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        let mut species = crate::ffi::Align4([0u8; 6]);
        let mut heldItems = crate::ffi::Align4([0u8; 6]);
        let mut firstMonId: i32 = 0i32;
        let mut trainerId: u16 = 0u16;
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        let mut battleMode: u32 = ((VarGet(16590u16)) as u32);
        let mut winStreak: u32 =
            (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1942))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .read()) as u32);
        let mut challengeNum: u32 = crate::c::div_u32(winStreak, 7u32);
        ((&raw mut gFacilityTrainers).cast::<*mut u8>())
            .write((&raw mut gBattleFrontierTrainers).cast::<u8>());
        'l1: loop {
            'l2: {
                trainerId = GetRandomScaledFrontierTrainerId(
                    ((challengeNum) as u8),
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1638)
                        .cast::<u16>())
                    .read()) as u8),
                );
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i
                            < (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1638)
                            .cast::<u16>())
                            .read()) as i32))
                        {
                            break 'l3;
                        }
                        'l4: {
                            if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1640))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                                == ((trainerId) as i32)
                            {
                                break 'l3;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            if !(i
                != (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                .read()) as i32))
            {
                break 'l1;
            }
        }
        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(trainerId);
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1638)
            .cast::<u16>())
        .read()) as i32)
            < 6i32
        {
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1640))
            .cast::<u16>())
            .wrapping_offset(
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                .read()) as i32) as isize,
            ))
            .write(trainerId);
        }
        i = 0i32;
        'l5: loop {
            if !(i != 3i32) {
                break 'l5;
            }
            let mut monId: u16 = GetFactoryMonId(((lvlMode) as u8), ((challengeNum) as u8), 0u8);
            if (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                .wrapping_offset(((monId) as i32) as isize * 16))
            .cast::<u16>())
            .read()) as i32)
                == 201i32
            {
                continue 'l5;
            }
            {
                j = 0i32;
                'l6: loop {
                    if !(j < ((crate::c::div_u32(72u32, 12u32)) as i32)) {
                        break 'l6;
                    }
                    'l7: {
                        if (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(((monId) as i32) as isize * 16))
                        .cast::<u16>())
                        .read()) as i32)
                            == (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                .wrapping_offset(
                                    ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(2084))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize * 12))
                                    .cast::<u16>())
                                    .read()) as i32) as isize
                                        * 16,
                                ))
                            .cast::<u16>())
                            .read()) as i32)
                        {
                            break 'l6;
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            if j != ((crate::c::div_u32(72u32, 12u32)) as i32) {
                continue 'l5;
            }
            if (lvlMode == 0u32) && (((monId) as i32) > 849i32) {
                continue 'l5;
            }
            {
                k = firstMonId;
                'l8: loop {
                    if !(k < (firstMonId).wrapping_add(i)) {
                        break 'l8;
                    }
                    'l9: {
                        if (((((&raw mut species).cast::<u16>()).wrapping_offset((k) as isize))
                            .read()) as i32)
                            == (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                .wrapping_offset(((monId) as i32) as isize * 16))
                            .cast::<u16>())
                            .read()) as i32)
                        {
                            break 'l8;
                        }
                    }
                    k = (k).wrapping_add(1);
                }
            }
            if k != (firstMonId).wrapping_add(i) {
                continue 'l5;
            }
            {
                k = firstMonId;
                'l10: loop {
                    if !(k < (firstMonId).wrapping_add(i)) {
                        break 'l10;
                    }
                    'l11: {
                        if ((((((&raw mut heldItems).cast::<u16>()).wrapping_offset((k) as isize))
                            .read()) as i32)
                            != 0i32)
                            && ((((((&raw mut heldItems).cast::<u16>())
                                .wrapping_offset((k) as isize))
                            .read()) as i32)
                                == ((((((&raw mut gBattleFrontierHeldItems).cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset(
                                    (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(((monId) as i32) as isize * 16))
                                    .wrapping_add(10))
                                    .read()) as i32) as isize,
                                ))
                                .read()) as i32))
                        {
                            break 'l10;
                        }
                    }
                    k = (k).wrapping_add(1);
                }
            }
            if k != (firstMonId).wrapping_add(i) {
                continue 'l5;
            }
            (((&raw mut species).cast::<u16>()).wrapping_offset((i) as isize)).write(
                (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                    .wrapping_offset(((monId) as i32) as isize * 16))
                .cast::<u16>())
                .read(),
            );
            (((&raw mut heldItems).cast::<u16>()).wrapping_offset((i) as isize)).write(
                ((((&raw mut gBattleFrontierHeldItems).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(((monId) as i32) as isize * 16))
                        .wrapping_add(10))
                        .read()) as i32) as isize,
                    ))
                .read(),
            );
            ((((&raw mut gFrontierTempParty).cast::<u16>()).cast::<u16>())
                .wrapping_offset((i) as isize))
            .write(monId);
            i = (i).wrapping_add(1);
        }
    }
}
pub(crate) unsafe extern "C" fn SetOpponentGfxVar() {
    unsafe {
        SetBattleFacilityTrainerGfxId(
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn SetRentalsToOpponentParty() {
    unsafe {
        let mut i: u8 = 0u8;
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32)
            != 2i32
        {
            ((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                .write((&raw mut gBattleFrontierMons).cast::<u8>());
        } else {
            ((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                .write((&raw mut gSlateportBattleTentMons).cast::<u8>());
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(2084))
                    .cast::<u8>())
                    .wrapping_offset((((i) as i32).wrapping_add(3i32)) as isize * 12))
                    .cast::<u16>())
                    .write(
                        ((((&raw mut gFrontierTempParty).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(2084))
                    .cast::<u8>())
                    .wrapping_offset((((i) as i32).wrapping_add(3i32)) as isize * 12))
                    .wrapping_add(8))
                    .write(
                        ((GetBoxMonData3(
                            (((&raw mut gEnemyParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100)),
                            40i32,
                            core::ptr::null_mut(),
                        )) as u8),
                    );
                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(2084))
                    .cast::<u8>())
                    .wrapping_offset((((i) as i32).wrapping_add(3i32)) as isize * 12))
                    .wrapping_add(4)
                    .cast::<u32>())
                    .write(GetMonData3(
                        ((&raw mut gEnemyParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        0i32,
                        core::ptr::null_mut(),
                    ));
                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(2084))
                    .cast::<u8>())
                    .wrapping_offset((((i) as i32).wrapping_add(3i32)) as isize * 12))
                    .wrapping_add(9))
                    .write(
                        ((GetBoxMonData3(
                            (((&raw mut gEnemyParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100)),
                            46i32,
                            core::ptr::null_mut(),
                        )) as u8),
                    );
                    SetMonData(
                        ((&raw mut gEnemyParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        12i32,
                        ((((&raw mut gBattleFrontierHeldItems).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                    .wrapping_offset(
                                        ((((((&raw mut gFrontierTempParty).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                .wrapping_add(10))
                                .read()) as i32) as isize,
                            ))
                        .cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetPlayerAndOpponentParties() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        let mut count: i32 = 0i32;
        let mut bits: u8 = 0u8;
        let mut monLevel: u8 = 0u8;
        let mut monId: u16 = 0u16;
        let mut evs: u16 = 0u16;
        let mut ivs: u8 = 0u8;
        let mut friendship: u8 = 0u8;
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32)
            == 2i32
        {
            ((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                .write((&raw mut gSlateportBattleTentMons).cast::<u8>());
            monLevel = 30u8;
        } else {
            ((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                .write((&raw mut gBattleFrontierMons).cast::<u8>());
            if ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                0,
                2,
                false,
            ) as u8) as i32)
                != 0i32
            {
                monLevel = 100u8;
            } else {
                monLevel = 50u8;
            }
        }
        if ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) < 2i32 {
            ZeroPlayerPartyMons();
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 3i32) {
                        break 'l1;
                    }
                    'l2: {
                        monId = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(2084))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 12))
                        .cast::<u16>())
                        .read();
                        ivs = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(2084))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 12))
                        .wrapping_add(8))
                        .read();
                        CreateMon(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                .wrapping_offset(((monId) as i32) as isize * 16))
                            .cast::<u16>())
                            .read(),
                            monLevel,
                            ivs,
                            1u8,
                            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(2084))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 12))
                            .wrapping_add(4)
                            .cast::<u32>())
                            .read(),
                            0u8,
                            0u32,
                        );
                        count = 0i32;
                        bits = (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(((monId) as i32) as isize * 16))
                        .wrapping_add(11))
                        .read();
                        {
                            j = 0i32;
                            'l3: loop {
                                if !(j < 6i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    if (((bits) as i32) & 1i32) != 0 {
                                        count = (count).wrapping_add(1);
                                    }
                                }
                                bits = ((((bits) as i32) >> 1) as u8);
                                j = (j).wrapping_add(1);
                            }
                        }
                        evs = ((crate::c::div_i32(510i32, count)) as u16);
                        bits = 1u8;
                        {
                            j = 0i32;
                            'l5: loop {
                                if !(j < 6i32) {
                                    break 'l5;
                                }
                                'l6: {
                                    if ((((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(((monId) as i32) as isize * 16))
                                    .wrapping_add(11))
                                    .read()) as i32)
                                        & ((bits) as i32))
                                        != 0
                                    {
                                        SetMonData(
                                            ((&raw mut gPlayerParty).cast::<u8>())
                                                .wrapping_offset((i) as isize * 100),
                                            (26i32).wrapping_add(j),
                                            (&raw mut evs).cast::<u8>(),
                                        );
                                    }
                                }
                                bits = ((((bits) as i32) << 1) as u8);
                                j = (j).wrapping_add(1);
                            }
                        }
                        CalculateMonStats(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                        );
                        friendship = 0u8;
                        {
                            k = 0i32;
                            'l7: loop {
                                if !(k < 4i32) {
                                    break 'l7;
                                }
                                'l8: {
                                    SetMonMoveAvoidReturn(
                                        ((&raw mut gPlayerParty).cast::<u8>())
                                            .wrapping_offset((i) as isize * 100),
                                        (((((((&raw mut gFacilityTrainerMons)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(((monId) as i32) as isize * 16))
                                        .wrapping_add(2))
                                        .cast::<u16>())
                                        .wrapping_offset((k) as isize))
                                        .read(),
                                        ((k) as u8),
                                    );
                                }
                                k = (k).wrapping_add(1);
                            }
                        }
                        SetMonData(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            32i32,
                            &raw mut friendship,
                        );
                        SetMonData(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            12i32,
                            ((((&raw mut gBattleFrontierHeldItems).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(((monId) as i32) as isize * 16))
                                    .wrapping_add(10))
                                    .read()) as i32) as isize,
                                ))
                            .cast::<u8>(),
                        );
                        SetMonData(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            46i32,
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(2084))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 12))
                            .wrapping_add(9),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        'l9: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 || __sw1 == 2i32 {
                {
                    i = 0i32;
                    'l10: loop {
                        if !(i < 3i32) {
                            break 'l10;
                        }
                        'l11: {
                            monId = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(1612))
                            .wrapping_add(2084))
                            .cast::<u8>())
                            .wrapping_offset(((i).wrapping_add(3i32)) as isize * 12))
                            .cast::<u16>())
                            .read();
                            ivs = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(2084))
                            .cast::<u8>())
                            .wrapping_offset(((i).wrapping_add(3i32)) as isize * 12))
                            .wrapping_add(8))
                            .read();
                            CreateMon(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                    .wrapping_offset(((monId) as i32) as isize * 16))
                                .cast::<u16>())
                                .read(),
                                monLevel,
                                ivs,
                                1u8,
                                ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(2084))
                                .cast::<u8>())
                                .wrapping_offset(((i).wrapping_add(3i32)) as isize * 12))
                                .wrapping_add(4)
                                .cast::<u32>())
                                .read(),
                                0u8,
                                0u32,
                            );
                            count = 0i32;
                            bits = (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                .wrapping_offset(((monId) as i32) as isize * 16))
                            .wrapping_add(11))
                            .read();
                            {
                                j = 0i32;
                                'l12: loop {
                                    if !(j < 6i32) {
                                        break 'l12;
                                    }
                                    'l13: {
                                        if (((bits) as i32) & 1i32) != 0 {
                                            count = (count).wrapping_add(1);
                                        }
                                    }
                                    bits = ((((bits) as i32) >> 1) as u8);
                                    j = (j).wrapping_add(1);
                                }
                            }
                            evs = ((crate::c::div_i32(510i32, count)) as u16);
                            bits = 1u8;
                            {
                                j = 0i32;
                                'l14: loop {
                                    if !(j < 6i32) {
                                        break 'l14;
                                    }
                                    'l15: {
                                        if ((((((((&raw mut gFacilityTrainerMons)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(((monId) as i32) as isize * 16))
                                        .wrapping_add(11))
                                        .read())
                                            as i32)
                                            & ((bits) as i32))
                                            != 0
                                        {
                                            SetMonData(
                                                ((&raw mut gEnemyParty).cast::<u8>())
                                                    .wrapping_offset((i) as isize * 100),
                                                (26i32).wrapping_add(j),
                                                (&raw mut evs).cast::<u8>(),
                                            );
                                        }
                                    }
                                    bits = ((((bits) as i32) << 1) as u8);
                                    j = (j).wrapping_add(1);
                                }
                            }
                            CalculateMonStats(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                            );
                            {
                                k = 0i32;
                                'l16: loop {
                                    if !(k < 4i32) {
                                        break 'l16;
                                    }
                                    'l17: {
                                        SetMonMoveAvoidReturn(
                                            ((&raw mut gEnemyParty).cast::<u8>())
                                                .wrapping_offset((i) as isize * 100),
                                            (((((((&raw mut gFacilityTrainerMons)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_offset(((monId) as i32) as isize * 16))
                                            .wrapping_add(2))
                                            .cast::<u16>())
                                            .wrapping_offset((k) as isize))
                                            .read(),
                                            ((k) as u8),
                                        );
                                    }
                                    k = (k).wrapping_add(1);
                                }
                            }
                            SetMonData(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                12i32,
                                ((((&raw mut gBattleFrontierHeldItems).cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset(
                                    (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(((monId) as i32) as isize * 16))
                                    .wrapping_add(10))
                                    .read()) as i32) as isize,
                                ))
                                .cast::<u8>(),
                            );
                            SetMonData(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                46i32,
                                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(2084))
                                .cast::<u8>())
                                .wrapping_offset(((i).wrapping_add(3i32)) as isize * 12))
                                .wrapping_add(9),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l9;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GenerateInitialRentalMons() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut firstMonId: u8 = 0u8;
        let mut battleMode: u8 = 0u8;
        let mut lvlMode: u8 = 0u8;
        let mut challengeNum: u8 = 0u8;
        let mut factoryLvlMode: u8 = 0u8;
        let mut factoryBattleMode: u8 = 0u8;
        let mut rentalRank: u8 = 0u8;
        let mut monId: u16 = 0u16;
        let mut currSpecies: u16 = 0u16;
        let mut species = crate::ffi::Align4([0u8; 12]);
        let mut monIds = crate::ffi::Align4([0u8; 12]);
        let mut heldItems = crate::ffi::Align4([0u8; 12]);
        ((&raw mut gFacilityTrainers).cast::<*mut u8>())
            .write((&raw mut gBattleFrontierTrainers).cast::<u8>());
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut species).cast::<u16>()).wrapping_offset((i) as isize)).write(0u16);
                    (((&raw mut monIds).cast::<u16>()).wrapping_offset((i) as isize)).write(0u16);
                    (((&raw mut heldItems).cast::<u16>()).wrapping_offset((i) as isize))
                        .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        lvlMode = (crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8);
        battleMode = ((VarGet(16590u16)) as u8);
        challengeNum = ((crate::c::div_i32(
            (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1942))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .read()) as i32),
            7i32,
        )) as u8);
        if ((VarGet(16590u16)) as i32) == 1i32 {
            factoryBattleMode = 1u8;
        } else {
            factoryBattleMode = 0u8;
        }
        ((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
            .write((&raw mut gBattleFrontierMons).cast::<u8>());
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32)
            != 0i32
        {
            factoryLvlMode = 1u8;
            firstMonId = 0u8;
        } else {
            factoryLvlMode = 0u8;
            firstMonId = 0u8;
        }
        rentalRank = GetNumPastRentalsRank(factoryBattleMode, factoryLvlMode);
        currSpecies = 0u16;
        i = 0i32;
        'l3: loop {
            if !(i != 6i32) {
                break 'l3;
            }
            if i < ((rentalRank) as i32) {
                monId = GetFactoryMonId(factoryLvlMode, challengeNum, 1u8);
            } else {
                monId = GetFactoryMonId(factoryLvlMode, challengeNum, 0u8);
            }
            if (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                .wrapping_offset(((monId) as i32) as isize * 16))
            .cast::<u16>())
            .read()) as i32)
                == 201i32
            {
                continue 'l3;
            }
            {
                j = ((firstMonId) as i32);
                'l4: loop {
                    if !(j < ((firstMonId) as i32).wrapping_add(i)) {
                        break 'l4;
                    }
                    'l5: {
                        let mut existingMonId: u16 = (((&raw mut monIds).cast::<u16>())
                            .wrapping_offset((j) as isize))
                        .read();
                        if ((existingMonId) as i32) == ((monId) as i32) {
                            break 'l4;
                        }
                        if (((((&raw mut species).cast::<u16>()).wrapping_offset((j) as isize))
                            .read()) as i32)
                            == (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                .wrapping_offset(((monId) as i32) as isize * 16))
                            .cast::<u16>())
                            .read()) as i32)
                        {
                            if ((currSpecies) as i32) == 0i32 {
                                currSpecies = (((((&raw mut gFacilityTrainerMons)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((monId) as i32) as isize * 16))
                                .cast::<u16>())
                                .read();
                            } else {
                                break 'l4;
                            }
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            if j != ((firstMonId) as i32).wrapping_add(i) {
                continue 'l3;
            }
            {
                j = ((firstMonId) as i32);
                'l6: loop {
                    if !(j < ((firstMonId) as i32).wrapping_add(i)) {
                        break 'l6;
                    }
                    'l7: {
                        if ((((((&raw mut heldItems).cast::<u16>()).wrapping_offset((j) as isize))
                            .read()) as i32)
                            != 0i32)
                            && ((((((&raw mut heldItems).cast::<u16>())
                                .wrapping_offset((j) as isize))
                            .read()) as i32)
                                == ((((((&raw mut gBattleFrontierHeldItems).cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset(
                                    (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(((monId) as i32) as isize * 16))
                                    .wrapping_add(10))
                                    .read()) as i32) as isize,
                                ))
                                .read()) as i32))
                        {
                            if (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                .wrapping_offset(((monId) as i32) as isize * 16))
                            .cast::<u16>())
                            .read()) as i32)
                                == ((currSpecies) as i32)
                            {
                                currSpecies = 0u16;
                            }
                            break 'l6;
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            if j != ((firstMonId) as i32).wrapping_add(i) {
                continue 'l3;
            }
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2084))
            .cast::<u8>())
            .wrapping_offset((i) as isize * 12))
            .cast::<u16>())
            .write(monId);
            (((&raw mut species).cast::<u16>()).wrapping_offset((i) as isize)).write(
                (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                    .wrapping_offset(((monId) as i32) as isize * 16))
                .cast::<u16>())
                .read(),
            );
            (((&raw mut heldItems).cast::<u16>()).wrapping_offset((i) as isize)).write(
                ((((&raw mut gBattleFrontierHeldItems).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(((monId) as i32) as isize * 16))
                        .wrapping_add(10))
                        .read()) as i32) as isize,
                    ))
                .read(),
            );
            (((&raw mut monIds).cast::<u16>()).wrapping_offset((i) as isize)).write(monId);
            i = (i).wrapping_add(1);
        }
    }
}
pub(crate) unsafe extern "C" fn GetOpponentMostCommonMonType() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut typeCounts = crate::ffi::Align4([0u8; 18]);
        let mut mostCommonTypes = crate::ffi::Align4([0u8; 2]);
        ((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
            .write((&raw mut gBattleFrontierMons).cast::<u8>());
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 18i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut typeCounts).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l3;
                }
                'l4: {
                    let mut species: u32 =
                        (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(
                                ((((((&raw mut gFrontierTempParty).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize
                                    * 16,
                            ))
                        .cast::<u16>())
                        .read()) as u32);
                    let __p1 = ((&raw mut typeCounts).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut gSpeciesInfo).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 28))
                        .wrapping_add(6))
                        .cast::<u8>())
                        .read()) as i32) as isize,
                    );
                    (__p1).write(((__p1).read()).wrapping_add(1));
                    if (((((((&raw mut gSpeciesInfo).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 28))
                    .wrapping_add(6))
                    .cast::<u8>())
                    .read()) as i32)
                        != ((((((((&raw mut gSpeciesInfo).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 28))
                        .wrapping_add(6))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32)
                    {
                        let __p2 = ((&raw mut typeCounts).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gSpeciesInfo).cast::<u8>())
                                .wrapping_offset(((species) as i32) as isize * 28))
                            .wrapping_add(6))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize,
                        );
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut mostCommonTypes).cast::<u8>()).write(0u8);
        (((&raw mut mostCommonTypes).cast::<u8>()).wrapping_offset(1)).write(0u8);
        {
            i = 1u8;
            'l5: loop {
                if !(((i) as i32) < 18i32) {
                    break 'l5;
                }
                'l6: {
                    if (((((&raw mut typeCounts).cast::<u8>()).wrapping_offset(
                        ((((&raw mut mostCommonTypes).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        < (((((&raw mut typeCounts).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                    {
                        ((&raw mut mostCommonTypes).cast::<u8>()).write(i);
                    } else {
                        if (((((&raw mut typeCounts).cast::<u8>()).wrapping_offset(
                            ((((&raw mut mostCommonTypes).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read()) as i32)
                            == (((((&raw mut typeCounts).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                        {
                            (((&raw mut mostCommonTypes).cast::<u8>()).wrapping_offset(1)).write(i);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((((&raw mut typeCounts).cast::<u8>())
            .wrapping_offset(((((&raw mut mostCommonTypes).cast::<u8>()).read()) as i32) as isize))
        .read()) as i32)
            != 0i32
        {
            if (((((&raw mut typeCounts).cast::<u8>()).wrapping_offset(
                ((((&raw mut mostCommonTypes).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32)
                > (((((&raw mut typeCounts).cast::<u8>()).wrapping_offset(
                    (((((&raw mut mostCommonTypes).cast::<u8>()).wrapping_offset(1)).read()) as i32)
                        as isize,
                ))
                .read()) as i32)
            {
                ((&raw mut gSpecialVar_Result).cast::<u16>())
                    .write(((((&raw mut mostCommonTypes).cast::<u8>()).read()) as u16));
            } else {
                if ((((&raw mut mostCommonTypes).cast::<u8>()).read()) as i32)
                    == (((((&raw mut mostCommonTypes).cast::<u8>()).wrapping_offset(1)).read())
                        as i32)
                {
                    ((&raw mut gSpecialVar_Result).cast::<u16>())
                        .write(((((&raw mut mostCommonTypes).cast::<u8>()).read()) as u16));
                } else {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(18u16);
                }
            }
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(18u16);
        }
    }
}
pub(crate) unsafe extern "C" fn GetOpponentBattleStyle() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut count: u8 = 0u8;
        let mut stylePoints = crate::ffi::Align4([0u8; 8]);
        count = 0u8;
        ((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
            .write((&raw mut gBattleFrontierMons).cast::<u8>());
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut stylePoints).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l3;
                }
                'l4: {
                    let mut monId: u16 = ((((&raw mut gFrontierTempParty).cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read();
                    {
                        j = 0u8;
                        'l5: loop {
                            if !(((j) as i32) < 4i32) {
                                break 'l5;
                            }
                            'l6: {
                                let mut battleStyle: u8 = GetMoveBattleStyle(
                                    (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(((monId) as i32) as isize * 16))
                                    .wrapping_add(2))
                                    .cast::<u16>())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .read(),
                                );
                                let __p1 = ((&raw mut stylePoints).cast::<u8>())
                                    .wrapping_offset(((battleStyle) as i32) as isize);
                                (__p1).write(((__p1).read()).wrapping_add(1));
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        {
            i = 1u8;
            'l7: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l7;
                }
                'l8: {
                    if (((((&raw mut stylePoints).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        >= ((((((&raw const sRequiredMoveCounts).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((((i) as i32).wrapping_sub(1i32)) as isize))
                        .read()) as i32)
                    {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((i) as u16));
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((count) as i32) > 2i32 {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(8u16);
        }
    }
}
pub(crate) unsafe extern "C" fn GetMoveBattleStyle(r#move: u16) -> u8 {
    unsafe {
        let mut r#move = r#move;
        let mut moves: *mut u16 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(28u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        moves = ((((&raw const sMoveStyles)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u16>())
                        .cast::<*mut u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read();
                        'l3: loop {
                            if !(((((moves).wrapping_offset(((j) as i32) as isize)).read()) as i32)
                                != 0i32)
                            {
                                break 'l3;
                            }
                            'l4: {
                                if ((((moves).wrapping_offset(((j) as i32) as isize)).read())
                                    as i32)
                                    == ((r#move) as i32)
                                {
                                    return ((((i) as i32).wrapping_add(1i32)) as u8);
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InBattleFactory() -> u8 {
    unsafe {
        return ((((((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(18)
            .cast::<u16>())
        .read()) as i32)
            == 347i32)
            || ((((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as i32)
                == 348i32)) as u8);
    }
}
pub(crate) unsafe extern "C" fn RestorePlayerPartyHeldItems() {
    unsafe {
        let mut i: u8 = 0u8;
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32)
            != 2i32
        {
            ((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                .write((&raw mut gBattleFrontierMons).cast::<u8>());
        } else {
            ((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                .write((&raw mut gSlateportBattleTentMons).cast::<u8>());
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        12i32,
                        ((((&raw mut gBattleFrontierHeldItems).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                    .wrapping_offset(
                                        ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(2084))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 12))
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                .wrapping_add(10))
                                .read()) as i32) as isize,
                            ))
                        .cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFactoryMonFixedIV(challengeNum: u8, isLastBattle: u8) -> u8 {
    unsafe {
        let mut challengeNum = challengeNum;
        let mut isLastBattle = isLastBattle;
        let mut ivSet: u8 = 0u8;
        let mut useHigherIV: u8 = ((if (isLastBattle) != 0 { 1i32 } else { 0i32 }) as u8);
        if ((challengeNum) as u32) > crate::c::div_u32(16u32, 2u32) {
            ivSet = (((crate::c::div_u32(16u32, 2u32)).wrapping_sub(1u32)) as u8);
        } else {
            ivSet = challengeNum;
        }
        return ((((((&raw const sFixedIVTable).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((ivSet) as i32) as isize * 2))
        .cast::<u8>())
        .wrapping_offset(((useHigherIV) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FillFactoryBrainParty() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        let mut species = crate::ffi::Align4([0u8; 6]);
        let mut heldItems = crate::ffi::Align4([0u8; 6]);
        let mut friendship: u8 = 0u8;
        let mut monLevel: i32 = 0i32;
        let mut fixedIV: u8 = 0u8;
        let mut otId: u32 = 0u32;
        let mut lvlMode: u8 = (crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8);
        let mut battleMode: u8 = ((VarGet(16590u16)) as u8);
        let mut challengeNum: u8 = ((crate::c::div_i32(
            (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1942))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .read()) as i32),
            7i32,
        )) as u8);
        fixedIV = GetFactoryMonFixedIV(((((challengeNum) as i32).wrapping_add(2i32)) as u8), 0u8);
        monLevel = ((SetFacilityPtrsGetLevel()) as i32);
        i = 0i32;
        otId = (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
            .cast::<u8>())
        .read()) as i32)
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as u32);
        'l1: loop {
            if !(i != 3i32) {
                break 'l1;
            }
            let mut monId: u16 = GetFactoryMonId(lvlMode, challengeNum, 0u8);
            if (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                .wrapping_offset(((monId) as i32) as isize * 16))
            .cast::<u16>())
            .read()) as i32)
                == 201i32
            {
                continue 'l1;
            }
            if (monLevel == 50i32) && (((monId) as i32) > 849i32) {
                continue 'l1;
            }
            {
                j = 0i32;
                'l2: loop {
                    if !(j < ((crate::c::div_u32(72u32, 12u32)) as i32)) {
                        break 'l2;
                    }
                    'l3: {
                        if ((monId) as i32)
                            == ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(2084))
                            .cast::<u8>())
                            .wrapping_offset((j) as isize * 12))
                            .cast::<u16>())
                            .read()) as i32)
                        {
                            break 'l2;
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            if j != ((crate::c::div_u32(72u32, 12u32)) as i32) {
                continue 'l1;
            }
            {
                k = 0i32;
                'l4: loop {
                    if !(k < i) {
                        break 'l4;
                    }
                    'l5: {
                        if (((((&raw mut species).cast::<u16>()).wrapping_offset((k) as isize))
                            .read()) as i32)
                            == (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                .wrapping_offset(((monId) as i32) as isize * 16))
                            .cast::<u16>())
                            .read()) as i32)
                        {
                            break 'l4;
                        }
                    }
                    k = (k).wrapping_add(1);
                }
            }
            if k != i {
                continue 'l1;
            }
            {
                k = 0i32;
                'l6: loop {
                    if !(k < i) {
                        break 'l6;
                    }
                    'l7: {
                        if ((((((&raw mut heldItems).cast::<u16>()).wrapping_offset((k) as isize))
                            .read()) as i32)
                            != 0i32)
                            && ((((((&raw mut heldItems).cast::<u16>())
                                .wrapping_offset((k) as isize))
                            .read()) as i32)
                                == ((((((&raw mut gBattleFrontierHeldItems).cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset(
                                    (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(((monId) as i32) as isize * 16))
                                    .wrapping_add(10))
                                    .read()) as i32) as isize,
                                ))
                                .read()) as i32))
                        {
                            break 'l6;
                        }
                    }
                    k = (k).wrapping_add(1);
                }
            }
            if k != i {
                continue 'l1;
            }
            (((&raw mut species).cast::<u16>()).wrapping_offset((i) as isize)).write(
                (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                    .wrapping_offset(((monId) as i32) as isize * 16))
                .cast::<u16>())
                .read(),
            );
            (((&raw mut heldItems).cast::<u16>()).wrapping_offset((i) as isize)).write(
                ((((&raw mut gBattleFrontierHeldItems).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(((monId) as i32) as isize * 16))
                        .wrapping_add(10))
                        .read()) as i32) as isize,
                    ))
                .read(),
            );
            CreateMonWithEVSpreadNatureOTID(
                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                    .wrapping_offset(((monId) as i32) as isize * 16))
                .cast::<u16>())
                .read(),
                ((monLevel) as u8),
                (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                    .wrapping_offset(((monId) as i32) as isize * 16))
                .wrapping_add(12))
                .read(),
                fixedIV,
                (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                    .wrapping_offset(((monId) as i32) as isize * 16))
                .wrapping_add(11))
                .read(),
                otId,
            );
            friendship = 0u8;
            {
                k = 0i32;
                'l8: loop {
                    if !(k < 4i32) {
                        break 'l8;
                    }
                    'l9: {
                        SetMonMoveAvoidReturn(
                            ((&raw mut gEnemyParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                .wrapping_offset(((monId) as i32) as isize * 16))
                            .wrapping_add(2))
                            .cast::<u16>())
                            .wrapping_offset((k) as isize))
                            .read(),
                            ((k) as u8),
                        );
                    }
                    k = (k).wrapping_add(1);
                }
            }
            SetMonData(
                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                32i32,
                &raw mut friendship,
            );
            SetMonData(
                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                12i32,
                ((((&raw mut gBattleFrontierHeldItems).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(((monId) as i32) as isize * 16))
                        .wrapping_add(10))
                        .read()) as i32) as isize,
                    ))
                .cast::<u8>(),
            );
            i = (i).wrapping_add(1);
        }
    }
}
pub(crate) unsafe extern "C" fn GetFactoryMonId(
    lvlMode: u8,
    challengeNum: u8,
    useBetterRange: u8,
) -> u16 {
    unsafe {
        let mut lvlMode = lvlMode;
        let mut challengeNum = challengeNum;
        let mut useBetterRange = useBetterRange;
        let mut numMons: u16 = 0u16;
        let mut monId: u16 = 0u16;
        let mut adder: u16 = 0u16;
        if ((lvlMode) as i32) == 0i32 {
            adder = 0u16;
        } else {
            adder = 8u16;
        }
        if ((challengeNum) as i32) < 7i32 {
            if (useBetterRange) != 0 {
                numMons =
                    (((((((((((&raw const sInitialRentalMonRanges).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((adder) as i32).wrapping_add(((challengeNum) as i32)))
                            .wrapping_add(1i32)) as isize
                            * 4,
                    ))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_sub(
                            (((((((&raw const sInitialRentalMonRanges).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((((adder) as i32).wrapping_add(((challengeNum) as i32)))
                                    .wrapping_add(1i32)) as isize
                                    * 4,
                            ))
                            .cast::<u16>())
                            .read()) as i32),
                        ))
                    .wrapping_add(1i32)) as u16);
                monId = ((crate::c::rem_i32(((Random()) as i32), ((numMons) as i32))) as u16);
                monId = ((((monId) as i32).wrapping_add(
                    (((((((&raw const sInitialRentalMonRanges).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((adder) as i32).wrapping_add(((challengeNum) as i32)))
                            .wrapping_add(1i32)) as isize
                            * 4,
                    ))
                    .cast::<u16>())
                    .read()) as i32),
                )) as u16);
            } else {
                numMons = (((((((((((&raw const sInitialRentalMonRanges)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(
                    (((adder) as i32).wrapping_add(((challengeNum) as i32))) as isize * 4,
                ))
                .cast::<u16>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_sub(
                        (((((((&raw const sInitialRentalMonRanges).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            (((adder) as i32).wrapping_add(((challengeNum) as i32))) as isize * 4,
                        ))
                        .cast::<u16>())
                        .read()) as i32),
                    ))
                .wrapping_add(1i32)) as u16);
                monId = ((crate::c::rem_i32(((Random()) as i32), ((numMons) as i32))) as u16);
                monId = ((((monId) as i32).wrapping_add(
                    (((((((&raw const sInitialRentalMonRanges).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        (((adder) as i32).wrapping_add(((challengeNum) as i32))) as isize * 4,
                    ))
                    .cast::<u16>())
                    .read()) as i32),
                )) as u16);
            }
        } else {
            let mut challenge: u16 = ((challengeNum) as u16);
            if ((challenge) as i32) != 7i32 {
                challenge = 7u16;
            }
            numMons = (((((((((((&raw const sInitialRentalMonRanges).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset((((adder) as i32).wrapping_add(((challenge) as i32))) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(1))
            .read()) as i32)
                .wrapping_sub(
                    (((((((&raw const sInitialRentalMonRanges).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        (((adder) as i32).wrapping_add(((challenge) as i32))) as isize * 4,
                    ))
                    .cast::<u16>())
                    .read()) as i32),
                ))
            .wrapping_add(1i32)) as u16);
            monId = ((crate::c::rem_i32(((Random()) as i32), ((numMons) as i32))) as u16);
            monId = ((((monId) as i32).wrapping_add(
                (((((((&raw const sInitialRentalMonRanges).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((adder) as i32).wrapping_add(((challenge) as i32))) as isize * 4,
                    ))
                .cast::<u16>())
                .read()) as i32),
            )) as u16);
        }
        return monId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNumPastRentalsRank(battleMode: u8, lvlMode: u8) -> u8 {
    unsafe {
        let mut battleMode = battleMode;
        let mut lvlMode = lvlMode;
        let mut ret: u8 = 0u8;
        let mut rents: u8 = (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1958))
        .cast::<u8>())
        .wrapping_offset(((battleMode) as i32) as isize * 4))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read()) as u8);
        if ((rents) as i32) < 15i32 {
            ret = 0u8;
        } else {
            if ((rents) as i32) < 22i32 {
                ret = 1u8;
            } else {
                if ((rents) as i32) < 29i32 {
                    ret = 2u8;
                } else {
                    if ((rents) as i32) < 36i32 {
                        ret = 3u8;
                    } else {
                        if ((rents) as i32) < 43i32 {
                            ret = 4u8;
                        } else {
                            ret = 5u8;
                        }
                    }
                }
            }
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetAiScriptsInBattleFactory() -> u32 {
    unsafe {
        let mut lvlMode: i32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32);
        if lvlMode == 2i32 {
            return 0u32;
        } else {
            let mut battleMode: i32 = ((VarGet(16590u16)) as i32);
            let mut challengeNum: i32 = crate::c::div_i32(
                (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1942))
                .cast::<u8>())
                .wrapping_offset((battleMode) as isize * 4))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as i32),
                7i32,
            );
            if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) == 1022i32 {
                return 7u32;
            } else {
                if challengeNum < 2i32 {
                    return 0u32;
                } else {
                    if challengeNum < 4i32 {
                        return 1u32;
                    } else {
                        return 7u32;
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMonMoveAvoidReturn(mon: *mut u8, moveArg: u16, moveSlot: u8) {
    unsafe {
        let mut mon = mon;
        let mut moveArg = moveArg;
        let mut moveSlot = moveSlot;
        let mut r#move: u16 = moveArg;
        if ((moveArg) as i32) == 216i32 {
            r#move = 218u16;
        }
        SetMonMoveSlot(mon, r#move, moveSlot);
    }
}
