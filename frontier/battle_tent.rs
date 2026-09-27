//! Translated from `src/battle_tent.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sVerdanturfTentFuncs sVerdanturfTentRewards sFallarborTentFuncs sFallarborTentRewards sSlateportTentFuncs sSlateportTentRewards
#[allow(unused_imports)]
use crate::data::battle_tent::*;

pub(crate) static mut sRandMonId: u16 = 0u16;

unsafe extern "C" {
    static mut gBattleFrontierHeldItems: u8;
    static mut gFacilityTrainerMons: u8;
    static mut gFacilityTrainers: u8;
    static mut gFrontierTempParty: u8;
    static mut gMapHeader: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSlateportBattleTentMons: u8;
    static mut gSlateportBattleTentTrainers: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_Result: u8;
    static mut gStringVar1: u8;
    static mut gTrainerBattleOpponent_A: u8;
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
    unsafe {
        (((((&raw const sVerdanturfTentFuncs)
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
pub(crate) unsafe extern "C" fn InitVerdanturfTentChallenge() {
    unsafe {
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
    }
}
pub(crate) unsafe extern "C" fn GetVerdanturfTentPrize() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2078)
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn SetVerdanturfTentPrize() {
    unsafe {
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2078)
            .cast::<u16>())
        .write(((&raw mut gSpecialVar_0x8006).cast::<u16>()).read());
    }
}
pub(crate) unsafe extern "C" fn SetVerdanturfTentTrainerGfx() {
    unsafe {
        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(
            ((crate::c::div_u32(
                (((crate::c::rem_i32(((Random()) as i32), 255i32)).wrapping_mul(5i32)) as u32),
                64u32,
            )) as u16),
        );
        SetBattleFacilityTrainerGfxId(
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn BufferVerdanturfTentTrainerIntro() {
    unsafe {
        if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) < 300i32 {
            FrontierSpeechToString(
                (((((&raw mut gFacilityTrainers).cast::<*mut u8>()).read()).wrapping_offset(
                    ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) as isize
                        * 52,
                ))
                .wrapping_add(12))
                .cast::<u16>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SaveVerdanturfTentChallenge() {
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
pub(crate) unsafe extern "C" fn SetRandomVerdanturfTentPrize() {
    unsafe {
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2078)
            .cast::<u16>())
        .write(
            ((((&raw const sVerdanturfTentRewards)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(
                ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(2u32, 2u32))) as i32)
                    as isize,
            ))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn GiveVerdanturfTentPrize() {
    unsafe {
        if ((AddBagItem(
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2078)
                .cast::<u16>())
            .read(),
            1u16,
        )) as i32)
            == 1i32
        {
            CopyItemName(
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(2078)
                    .cast::<u16>())
                .read(),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2078)
                .cast::<u16>())
            .write(0u16);
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallFallarborTentFunction() {
    unsafe {
        (((((&raw const sFallarborTentFuncs)
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
pub(crate) unsafe extern "C" fn InitFallarborTentChallenge() {
    unsafe {
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
    }
}
pub(crate) unsafe extern "C" fn GetFallarborTentPrize() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2080)
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn SetFallarborTentPrize() {
    unsafe {
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2080)
            .cast::<u16>())
        .write(((&raw mut gSpecialVar_0x8006).cast::<u16>()).read());
    }
}
pub(crate) unsafe extern "C" fn SaveFallarborTentChallenge() {
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
pub(crate) unsafe extern "C" fn SetRandomFallarborTentPrize() {
    unsafe {
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2080)
            .cast::<u16>())
        .write(
            ((((&raw const sFallarborTentRewards)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(
                ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(2u32, 2u32))) as i32)
                    as isize,
            ))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn GiveFallarborTentPrize() {
    unsafe {
        if ((AddBagItem(
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2080)
                .cast::<u16>())
            .read(),
            1u16,
        )) as i32)
            == 1i32
        {
            CopyItemName(
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(2080)
                    .cast::<u16>())
                .read(),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2080)
                .cast::<u16>())
            .write(0u16);
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn BufferFallarborTentTrainerName() {
    unsafe {
        GetFrontierTrainerName(
            (&raw mut gStringVar1).cast::<u8>(),
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallSlateportTentFunction() {
    unsafe {
        (((((&raw const sSlateportTentFuncs)
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
pub(crate) unsafe extern "C" fn InitSlateportTentChallenge() {
    unsafe {
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
    }
}
pub(crate) unsafe extern "C" fn GetSlateportTentPrize() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2082)
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn SetSlateportTentPrize() {
    unsafe {
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2082)
            .cast::<u16>())
        .write(((&raw mut gSpecialVar_0x8006).cast::<u16>()).read());
    }
}
pub(crate) unsafe extern "C" fn SaveSlateportTentChallenge() {
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
pub(crate) unsafe extern "C" fn SetRandomSlateportTentPrize() {
    unsafe {
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2082)
            .cast::<u16>())
        .write(
            ((((&raw const sSlateportTentRewards)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(
                ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(2u32, 2u32))) as i32)
                    as isize,
            ))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn GiveSlateportTentPrize() {
    unsafe {
        if ((AddBagItem(
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2082)
                .cast::<u16>())
            .read(),
            1u16,
        )) as i32)
            == 1i32
        {
            CopyItemName(
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(2082)
                    .cast::<u16>())
                .read(),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2082)
                .cast::<u16>())
            .write(0u16);
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        }
    }
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InSlateportBattleTent() -> u8 {
    unsafe {
        return ((((((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as i32)
            == 8i32)
            && (((((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as i32)
                == 385i32)
                || ((((((&raw mut gMapHeader).cast::<u8>())
                    .wrapping_add(18)
                    .cast::<u16>())
                .read()) as i32)
                    == 386i32))) as u8);
    }
}
pub(crate) unsafe extern "C" fn GenerateInitialRentalMons() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut firstMonId: u8 = 0u8;
        let mut monSetId: u16 = 0u16;
        let mut currSpecies: u16 = 0u16;
        let mut species = crate::ffi::Align4([0u8; 12]);
        let mut monIds = crate::ffi::Align4([0u8; 12]);
        let mut heldItems = crate::ffi::Align4([0u8; 12]);
        firstMonId = 0u8;
        ((&raw mut gFacilityTrainers).cast::<*mut u8>())
            .write((&raw mut gSlateportBattleTentTrainers).cast::<u8>());
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
        ((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
            .write((&raw mut gSlateportBattleTentMons).cast::<u8>());
        currSpecies = 0u16;
        i = 0i32;
        'l3: loop {
            if !(i != 6i32) {
                break 'l3;
            }
            monSetId = ((crate::c::rem_i32(((Random()) as i32), 70i32)) as u16);
            {
                j = ((firstMonId) as i32);
                'l4: loop {
                    if !(j < ((firstMonId) as i32).wrapping_add(i)) {
                        break 'l4;
                    }
                    'l5: {
                        if (((((&raw mut monIds).cast::<u16>()).wrapping_offset((j) as isize))
                            .read()) as i32)
                            == ((monSetId) as i32)
                        {
                            break 'l4;
                        }
                        if (((((&raw mut species).cast::<u16>()).wrapping_offset((j) as isize))
                            .read()) as i32)
                            == (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                .wrapping_offset(((monSetId) as i32) as isize * 16))
                            .cast::<u16>())
                            .read()) as i32)
                        {
                            if ((currSpecies) as i32) == 0i32 {
                                currSpecies = (((((&raw mut gFacilityTrainerMons)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((monSetId) as i32) as isize * 16))
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
            if j != (i).wrapping_add(((firstMonId) as i32)) {
                continue 'l3;
            }
            {
                j = ((firstMonId) as i32);
                'l6: loop {
                    if !(j < (i).wrapping_add(((firstMonId) as i32))) {
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
                                    .wrapping_offset(((monSetId) as i32) as isize * 16))
                                    .wrapping_add(10))
                                    .read()) as i32) as isize,
                                ))
                                .read()) as i32))
                        {
                            if (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                .wrapping_offset(((monSetId) as i32) as isize * 16))
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
            if j != (i).wrapping_add(((firstMonId) as i32)) {
                continue 'l3;
            }
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2084))
            .cast::<u8>())
            .wrapping_offset((i) as isize * 12))
            .cast::<u16>())
            .write(monSetId);
            (((&raw mut species).cast::<u16>()).wrapping_offset((i) as isize)).write(
                (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                    .wrapping_offset(((monSetId) as i32) as isize * 16))
                .cast::<u16>())
                .read(),
            );
            (((&raw mut heldItems).cast::<u16>()).wrapping_offset((i) as isize)).write(
                ((((&raw mut gBattleFrontierHeldItems).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(((monSetId) as i32) as isize * 16))
                        .wrapping_add(10))
                        .read()) as i32) as isize,
                    ))
                .read(),
            );
            (((&raw mut monIds).cast::<u16>()).wrapping_offset((i) as isize)).write(monSetId);
            i = (i).wrapping_add(1);
        }
    }
}
pub(crate) unsafe extern "C" fn GenerateOpponentMons() {
    unsafe {
        let mut trainerId: u16 = 0u16;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        let mut monSet: *mut u16 = core::ptr::null_mut();
        let mut species = crate::ffi::Align4([0u8; 6]);
        let mut heldItems = crate::ffi::Align4([0u8; 6]);
        let mut numMons: i32 = 0i32;
        ((&raw mut gFacilityTrainers).cast::<*mut u8>())
            .write((&raw mut gSlateportBattleTentTrainers).cast::<u8>());
        ((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
            .write((&raw mut gSlateportBattleTentMons).cast::<u8>());
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            'l2: loop {
                'l3: {
                    trainerId = ((crate::c::rem_i32(((Random()) as i32), 30i32)) as u16);
                    {
                        i = 0i32;
                        'l4: loop {
                            if !(i
                                < (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1638)
                                .cast::<u16>())
                                .read()) as i32))
                            {
                                break 'l4;
                            }
                            'l5: {
                                if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1640))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)
                                    == ((trainerId) as i32)
                                {
                                    break 'l4;
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                if !(i
                    != (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                    .read()) as i32))
                {
                    break 'l2;
                }
            }
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(trainerId);
            monSet = (((((&raw mut gFacilityTrainers).cast::<*mut u8>()).read()).wrapping_offset(
                ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) as isize * 52,
            ))
            .wrapping_add(48)
            .cast::<*mut u16>())
            .read();
            'l6: loop {
                if !(((((monSet).wrapping_offset((numMons) as isize)).read()) as i32) != 65535i32) {
                    break 'l6;
                }
                numMons = (numMons).wrapping_add(1);
            }
            if numMons > 8i32 {
                break 'l1;
            }
            numMons = 0i32;
        }
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1638)
            .cast::<u16>())
        .read()) as i32)
            < 2i32
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
            .write(((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read());
        }
        monSet = (((((&raw mut gFacilityTrainers).cast::<*mut u8>()).read()).wrapping_offset(
            ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) as isize * 52,
        ))
        .wrapping_add(48)
        .cast::<*mut u16>())
        .read();
        i = 0i32;
        'l7: loop {
            if !(i != 3i32) {
                break 'l7;
            }
            ((&raw mut sRandMonId).cast::<u8>().cast::<u16>()).write(
                ((monSet)
                    .wrapping_offset((crate::c::rem_i32(((Random()) as i32), numMons)) as isize))
                .read(),
            );
            {
                j = 0i32;
                'l8: loop {
                    if !(j < ((crate::c::div_u32(72u32, 12u32)) as i32)) {
                        break 'l8;
                    }
                    'l9: {
                        if (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(
                                ((((&raw mut sRandMonId).cast::<u8>().cast::<u16>()).read()) as i32)
                                    as isize
                                    * 16,
                            ))
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
                            break 'l8;
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            if j != ((crate::c::div_u32(72u32, 12u32)) as i32) {
                continue 'l7;
            }
            {
                k = 0i32;
                'l10: loop {
                    if !(k < i) {
                        break 'l10;
                    }
                    'l11: {
                        if (((((&raw mut species).cast::<u16>()).wrapping_offset((k) as isize))
                            .read()) as i32)
                            == (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                .wrapping_offset(
                                    ((((&raw mut sRandMonId).cast::<u8>().cast::<u16>()).read())
                                        as i32) as isize
                                        * 16,
                                ))
                            .cast::<u16>())
                            .read()) as i32)
                        {
                            break 'l10;
                        }
                    }
                    k = (k).wrapping_add(1);
                }
            }
            if k != i {
                continue 'l7;
            }
            {
                k = 0i32;
                'l12: loop {
                    if !(k < i) {
                        break 'l12;
                    }
                    'l13: {
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
                                    .wrapping_offset(
                                        ((((&raw mut sRandMonId).cast::<u8>().cast::<u16>()).read())
                                            as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .wrapping_add(10))
                                    .read()) as i32) as isize,
                                ))
                                .read()) as i32))
                        {
                            break 'l12;
                        }
                    }
                    k = (k).wrapping_add(1);
                }
            }
            if k != i {
                continue 'l7;
            }
            (((&raw mut species).cast::<u16>()).wrapping_offset((i) as isize)).write(
                (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset(
                    ((((&raw mut sRandMonId).cast::<u8>().cast::<u16>()).read()) as i32) as isize
                        * 16,
                ))
                .cast::<u16>())
                .read(),
            );
            (((&raw mut heldItems).cast::<u16>()).wrapping_offset((i) as isize)).write(
                ((((&raw mut gBattleFrontierHeldItems).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(
                                ((((&raw mut sRandMonId).cast::<u8>().cast::<u16>()).read()) as i32)
                                    as isize
                                    * 16,
                            ))
                        .wrapping_add(10))
                        .read()) as i32) as isize,
                    ))
                .read(),
            );
            ((((&raw mut gFrontierTempParty).cast::<u16>()).cast::<u16>())
                .wrapping_offset((i) as isize))
            .write(((&raw mut sRandMonId).cast::<u8>().cast::<u16>()).read());
            i = (i).wrapping_add(1);
        }
    }
}
