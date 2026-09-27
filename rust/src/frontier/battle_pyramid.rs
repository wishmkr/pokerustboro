//! Translated from `src/battle_pyramid.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sLevel50WildMons_Round1 sLevel50WildMons_Round2 sLevel50WildMons_Round3 sLevel50WildMons_Round4 sLevel50WildMons_Round5 sLevel50WildMons_Round6 sLevel50WildMons_Round7 sLevel50WildMons_Round8 sLevel50WildMons_Round9 sLevel50WildMons_Round10 sLevel50WildMons_Round11 sLevel50WildMons_Round12 sLevel50WildMons_Round13 sLevel50WildMons_Round14 sLevel50WildMons_Round15 sLevel50WildMons_Round16 sLevel50WildMons_Round17 sLevel50WildMons_Round18 sLevel50WildMons_Round19 sLevel50WildMons_Round20 sLevel50WildMonPointers sOpenLevelWildMons_Round1 sOpenLevelWildMons_Round2 sOpenLevelWildMons_Round3 sOpenLevelWildMons_Round4 sOpenLevelWildMons_Round5 sOpenLevelWildMons_Round6 sOpenLevelWildMons_Round7 sOpenLevelWildMons_Round8 sOpenLevelWildMons_Round9 sOpenLevelWildMons_Round10 sOpenLevelWildMons_Round11 sOpenLevelWildMons_Round12 sOpenLevelWildMons_Round13 sOpenLevelWildMons_Round14 sOpenLevelWildMons_Round15 sOpenLevelWildMons_Round16 sOpenLevelWildMons_Round17 sOpenLevelWildMons_Round18 sOpenLevelWildMons_Round19 sOpenLevelWildMons_Round20 sOpenLevelWildMonPointers sPyramidFloorTemplates sPyramidFloorTemplateOptions sFloorTemplateOffsets sPickupItemsLvl50 sPickupItemsLvlOpen sPickupItemSlots sPickupItemOffsets sTrainerClassEncounterMusic sTrainerTextGroups sExitDirectionHintTexts1 sRemainingItemsHintTexts1 sRemainingTrainersHintTexts1 sExitDirectionHintTexts2 sRemainingItemsHintTexts2 sRemainingTrainersHintTexts2 sExitDirectionHintTexts3 sRemainingItemsHintTexts3 sRemainingTrainersHintTexts3 sExitDirectionHintTexts4 sRemainingItemsHintTexts4 sRemainingTrainersHintTexts4 sExitDirectionHintTexts5 sRemainingItemsHintTexts5 sRemainingTrainersHintTexts5 sExitDirectionHintTexts6 sRemainingItemsHintTexts6 sRemainingTrainersHintTexts6 sPostBattleHintTexts1 sPostBattleHintTexts2 sPostBattleHintTexts3 sPostBattleHintTexts4 sPostBattleHintTexts5 sPostBattleHintTexts6 sPostBattleTexts sHintTextTypes sBattlePyramidFunctions sShortStreakRewardItems sLongStreakRewardItems sBorderedSquareIds sPickupPercentages
#[allow(unused_imports)]
use crate::data::battle_pyramid::*;

unsafe extern "C" {
    static mut BattlePyramid_FindItemBall: u8;
    static mut BattlePyramid_Retire: u8;
    static mut BattlePyramid_TrainerBattle: u8;
    static mut gBackupMapLayout: u8;
    static mut gBattleFrontierTrainers: u8;
    static mut gBattleOutcome: u8;
    static mut gBattlePyramidFloor_Pal: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBitTable: u8;
    static mut gEnemyParty: u8;
    static mut gExperienceTables: u8;
    static mut gFacilityClassToTrainerClass: u8;
    static mut gFacilityTrainers: u8;
    static mut gMapHeader: u8;
    static mut gMapLayouts: u8;
    static mut gObjectEvents: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSelectedObjectEvent: u8;
    static mut gSelectedOrderFromParty: u8;
    static mut gSpecialVar_0x8000: u8;
    static mut gSpecialVar_0x8001: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_0x8007: u8;
    static mut gSpecialVar_LastTalked: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesInfo: u8;
    static mut gStringVar1: u8;
    static mut gTrainerBattleOpponent_A: u8;
    static mut gTrainerBattleOpponent_B: u8;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn AddPyramidBagItem(a0: u16, a1: u16) -> u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn CalculateMonStats(a0: *mut u8);
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn DoSoftReset();
    fn Free(a0: *mut u8);
    fn FrontierSpeechToString(a0: *mut u16);
    fn GetBattleFacilityTrainerGfxId(a0: u16) -> u8;
    fn GetChosenApproachingTrainerObjectEventId(a0: u8) -> u8;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetRandomScaledFrontierTrainerId(a0: u8, a1: u8) -> u16;
    fn GetSpeciesName(a0: *mut u8, a1: u16);
    fn InitBattlePyramidBagCursorPosition();
    fn LoadPlayerParty();
    fn Overworld_GetMapHeaderByGroupAndId(a0: u16, a1: u16) -> *mut u8;
    fn PlaySE(a0: u16);
    fn Random() -> u16;
    fn Random2() -> u16;
    fn RunOnLoadMapScript();
    fn SaveMapView();
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn SeedRng2(a0: u16);
    fn SetFacilityPtrsGetLevel() -> u8;
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetMonMoveSlot(a0: *mut u8, a1: u16, a2: u8);
    fn ShowBattlePyramidStartMenu();
    fn ShowFieldMessage(a0: *mut u8) -> u8;
    fn TrySavingData(a0: u8) -> u8;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn WriteBattlePyramidViewScanlineEffectBuffer();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallBattlePyramidFunction() {
    unsafe {
        (((((&raw const sBattlePyramidFunctions)
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
pub(crate) unsafe extern "C" fn InitPyramidChallenge() {
    unsafe {
        let mut isCurrent: u32 = 0u32;
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
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
        if lvlMode != 0u32 {
            isCurrent = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1612))
            .wrapping_add(1680)
            .cast::<u32>())
            .read()
                & 8192u32);
        } else {
            isCurrent = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1612))
            .wrapping_add(1680)
            .cast::<u32>())
            .read()
                & 4096u32);
        }
        if !((isCurrent) != 0) {
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1998))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .write(0u16);
            InitPyramidBagItems(((lvlMode) as u8));
        }
        InitBattlePyramidBagCursorPosition();
        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(0u16);
        ((&raw mut gBattleOutcome).cast::<u8>()).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn GetBattlePyramidData() {
    unsafe {
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1996)
                        .cast::<u16>())
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1998))
                    .cast::<u16>())
                    .wrapping_offset(((lvlMode) as i32) as isize))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                if lvlMode != 0u32 {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>())
                        .read()
                            & 8192u32) as u16),
                    );
                } else {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>())
                        .read()
                            & 4096u32) as u16),
                    );
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1998))
                    .cast::<u16>())
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1998))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>())
                    .read()
                        & 4096u32) as u16),
                );
                break 'l1;
            }
            if __sw1 == 6i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>())
                    .read()
                        & 8192u32) as u16),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetBattlePyramidData() {
    unsafe {
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1996)
                    .cast::<u16>())
                .write(((&raw mut gSpecialVar_0x8006).cast::<u16>()).read());
                break 'l1;
            }
            if __sw1 == 1i32 {
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1998))
                .cast::<u16>())
                .wrapping_offset(((lvlMode) as i32) as isize))
                .write(((&raw mut gSpecialVar_0x8006).cast::<u16>()).read());
                break 'l1;
            }
            if __sw1 == 2i32 {
                if lvlMode != 0u32 {
                    if (((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) != 0 {
                        let __p2 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>();
                        (__p2).write(((__p2).read() | 8192u32));
                    } else {
                        let __p3 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>();
                        (__p3).write(((__p3).read() & 4294959103u32));
                    }
                } else {
                    if (((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) != 0 {
                        let __p4 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>();
                        (__p4).write(((__p4).read() | 4096u32));
                    } else {
                        let __p5 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>();
                        (__p5).write(((__p5).read() & 4294963199u32));
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(2014))
                .write(((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SavePyramidChallenge() {
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
        SaveMapView();
        TrySavingData(1u8);
    }
}
pub(crate) unsafe extern "C" fn SetBattlePyramidPrize() {
    unsafe {
        if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1998))
        .cast::<u16>())
        .wrapping_offset(
            ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                0,
                2,
                false,
            ) as u8) as i32) as isize,
        ))
        .read()) as i32)
            > 41i32
        {
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1996)
                .cast::<u16>())
            .write(
                ((((&raw const sLongStreakRewardItems)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(
                    ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(18u32, 2u32)))
                        as i32) as isize,
                ))
                .read(),
            );
        } else {
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1996)
                .cast::<u16>())
            .write(
                ((((&raw const sShortStreakRewardItems)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(
                    ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(12u32, 2u32)))
                        as i32) as isize,
                ))
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn GiveBattlePyramidPrize() {
    unsafe {
        if ((AddBagItem(
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1996)
                .cast::<u16>())
            .read(),
            1u16,
        )) as i32)
            == 1i32
        {
            CopyItemName(
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1996)
                    .cast::<u16>())
                .read(),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1996)
                .cast::<u16>())
            .write(0u16);
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn SeedPyramidFloor() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(8u32, 2u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(2006))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(Random());
                }
                i = (i).wrapping_add(1);
            }
        }
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2014))
        .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn SetPickupItem() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut itemIndex: i32 = 0i32;
        let mut rand: i32 = 0i32;
        let mut id: u8 = 0u8;
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        let mut floor: u32 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1638)
        .cast::<u16>())
        .read()) as u32);
        let mut round: u32 = ((crate::c::rem_i32(
            crate::c::div_i32(
                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1998))
                .cast::<u16>())
                .wrapping_offset(((lvlMode) as i32) as isize))
                .read()) as i32),
                7i32,
            ),
            20i32,
        )) as u32);
        if round >= 20u32 {
            round = 19u32;
        }
        id = GetPyramidFloorTemplateId();
        itemIndex = (((((&raw mut gSpecialVar_LastTalked).cast::<u16>()).read()) as i32)
            .wrapping_sub(
                (((((((&raw const sPyramidFloorTemplates).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 16))
                .wrapping_add(1))
                .read()) as i32),
            ))
        .wrapping_sub(1i32);
        rand = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2006))
        .cast::<u16>())
        .wrapping_offset((crate::c::div_i32(itemIndex, 2i32)) as isize))
        .read()) as i32);
        SeedRng2(((rand) as u16));
        {
            i = 0i32;
            'l1: loop {
                if !(i < (itemIndex).wrapping_add(1i32)) {
                    break 'l1;
                }
                'l2: {
                    rand = crate::c::rem_i32(((Random2()) as i32), 100i32);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = ((((((&raw const sPickupItemOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((floor) as i32) as isize))
            .read()) as i32);
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(126u32, 2u32)) {
                    break 'l3;
                }
                'l4: {
                    if rand
                        < (((((((&raw const sPickupItemSlots).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize * 2))
                        .cast::<u8>())
                        .read()) as i32)
                    {
                        break 'l3;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if lvlMode != 0u32 {
            ((&raw mut gSpecialVar_0x8000).cast::<u16>()).write(
                ((((((&raw const sPickupItemsLvlOpen).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((round) as i32) as isize * 20))
                .cast::<u16>())
                .wrapping_offset(
                    ((((((((&raw const sPickupItemSlots).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset((i) as isize * 2))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize,
                ))
                .read(),
            );
        } else {
            ((&raw mut gSpecialVar_0x8000).cast::<u16>()).write(
                ((((((&raw const sPickupItemsLvl50).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((round) as i32) as isize * 20))
                .cast::<u16>())
                .wrapping_offset(
                    ((((((((&raw const sPickupItemSlots).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset((i) as isize * 2))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize,
                ))
                .read(),
            );
        }
        ((&raw mut gSpecialVar_0x8001).cast::<u16>()).write(1u16);
    }
}
pub(crate) unsafe extern "C" fn HidePyramidItem() {
    unsafe {
        let mut events: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(3184))
        .cast::<u8>();
        let mut i: i32 = 0i32;
        'l1: loop {
            'l2: {
                if ((((events).wrapping_offset((i) as isize * 24)).read()) as i32)
                    == ((((&raw mut gSpecialVar_LastTalked).cast::<u16>()).read()) as i32)
                {
                    (((events).wrapping_offset((i) as isize * 24))
                        .wrapping_add(4)
                        .cast::<i16>())
                    .write(32767i16);
                    (((events).wrapping_offset((i) as isize * 24))
                        .wrapping_add(6)
                        .cast::<i16>())
                    .write(32767i16);
                    break 'l1;
                }
                i = (i).wrapping_add(1);
            }
            if !(((((events).wrapping_offset((i) as isize * 24)).read()) as i32) != 0i32) {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetPyramidFacilityTrainers() {
    unsafe {
        ((&raw mut gFacilityTrainers).cast::<*mut u8>())
            .write((&raw mut gBattleFrontierTrainers).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn ShowPostBattleHintText() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut hintType: i32 = 0i32;
        let mut id: u8 = 0u8;
        let mut textGroup: i32 = 0i32;
        let mut textIndex: i32 = 0i32;
        let mut events: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(3184))
        .cast::<u8>();
        let mut trainerId: u16 = LocalIdToPyramidTrainerId(
            ((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSelectedObjectEvent).cast::<u8>()).read()) as i32) as isize * 36,
            ))
            .wrapping_add(8))
            .read(),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(100u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw const sTrainerTextGroups).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset((i) as isize * 2))
                    .cast::<u8>())
                    .read()) as i32)
                        == ((((((&raw mut gFacilityTrainers).cast::<*mut u8>()).read())
                            .wrapping_offset(((trainerId) as i32) as isize * 52))
                        .read()) as i32)
                    {
                        textGroup =
                            ((((((((&raw const sTrainerTextGroups).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize * 2))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32);
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        hintType = ((((((&raw const sHintTextTypes).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                (((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gSelectedObjectEvent).cast::<u8>()).read()) as i32) as isize * 36,
                ))
                .wrapping_add(8))
                .read()) as i32)
                    .wrapping_sub(1i32)) as isize,
            ))
        .read()) as i32);
        i = 0i32;
        'l3: loop {
            if !(!((i) != 0)) {
                break 'l3;
            }
            'l4: {
                let __sw1 = hintType;
                if __sw1 == 0i32 {
                    textIndex =
                        ((GetPostBattleDirectionHintTextIndex(&raw mut hintType, 8u8, 0u8)) as i32);
                    i = 1i32;
                    break 'l4;
                }
                if __sw1 == 1i32 {
                    {
                        i = 0i32;
                        'l5: loop {
                            if !(i < ((GetNumBattlePyramidObjectEvents()) as i32)) {
                                break 'l5;
                            }
                            'l6: {
                                if (((((((events).wrapping_offset((i) as isize * 24))
                                    .wrapping_add(1))
                                .read()) as i32)
                                    == 59i32)
                                    && ((((((events).wrapping_offset((i) as isize * 24))
                                        .wrapping_add(4)
                                        .cast::<i16>())
                                    .read()) as i32)
                                        != 32767i32))
                                    && ((((((events).wrapping_offset((i) as isize * 24))
                                        .wrapping_add(6)
                                        .cast::<i16>())
                                    .read()) as i32)
                                        != 32767i32)
                                {
                                    textIndex = (textIndex).wrapping_add(1);
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    i = 1i32;
                    break 'l4;
                }
                if __sw1 == 2i32 {
                    id = GetPyramidFloorTemplateId();
                    textIndex =
                        (((((((&raw const sPyramidFloorTemplates).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 16))
                        .wrapping_add(1))
                        .read()) as i32);
                    {
                        i = 0i32;
                        'l7: loop {
                            if !(i < 8i32) {
                                break 'l7;
                            }
                            'l8: {
                                if (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset((i) as isize))
                                .read()
                                    & (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(2014))
                                    .read()) as u32))
                                    != 0
                                {
                                    textIndex = (textIndex).wrapping_sub(1);
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    i = 1i32;
                    break 'l4;
                }
                if __sw1 == 3i32 {
                    GetPostBattleDirectionHintTextIndex(&raw mut hintType, 8u8, 2u8);
                    break 'l4;
                }
                if __sw1 == 4i32 {
                    GetPostBattleDirectionHintTextIndex(&raw mut hintType, 8u8, 1u8);
                    break 'l4;
                }
                if __sw1 == 5i32 {
                    GetPostBattleDirectionHintTextIndex(&raw mut hintType, 16u8, 2u8);
                    break 'l4;
                }
                if __sw1 == 6i32 {
                    GetPostBattleDirectionHintTextIndex(&raw mut hintType, 16u8, 1u8);
                    break 'l4;
                }
                if __sw1 == 7i32 {
                    GetPostBattleDirectionHintTextIndex(&raw mut hintType, 24u8, 2u8);
                    break 'l4;
                }
                if __sw1 == 8i32 {
                    GetPostBattleDirectionHintTextIndex(&raw mut hintType, 24u8, 1u8);
                    break 'l4;
                }
            }
        }
        ShowFieldMessage(
            ((((((((&raw const sPostBattleTexts)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut *mut *mut u8>())
            .cast::<*mut *mut *mut u8>())
            .wrapping_offset((textGroup) as isize))
            .read())
            .wrapping_offset((hintType) as isize))
            .read())
            .wrapping_offset((textIndex) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn UpdatePyramidWinStreak() {
    unsafe {
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1998))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read()) as i32)
            < 999i32
        {
            let __p1 = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1612))
            .wrapping_add(1998))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1998))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read()) as i32)
            > (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2002))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .read()) as i32)
        {
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2002))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .write(
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1998))
                .cast::<u16>())
                .wrapping_offset(((lvlMode) as i32) as isize))
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn GetCurrentBattlePyramidLocation() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((CurrentBattlePyramidLocation()) as u16));
    }
}
pub(crate) unsafe extern "C" fn UpdatePyramidLightRadius() {
    unsafe {
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(2076))
                .write(((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8));
                break 'l1;
            }
            if __sw1 == 1i32 {
                'l2: {
                    let __sw2 = ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32);
                    let __matched = __sw2 == 0i32 || __sw2 == 1i32 || __sw2 == 2i32;
                    if __sw2 == 0i32 {
                        if !((crate::c::bf_read(
                            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                            7,
                            1,
                            false,
                        ) as u16)
                            != 0)
                        {
                            if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(2076))
                            .read()) as i32)
                                >= 120i32
                            {
                                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(2076))
                                .write(120u8);
                            } else {
                                PlaySE(((&raw mut gSpecialVar_0x8007).cast::<u16>()).read());
                            }
                            let __p3 = (&raw mut gSpecialVar_Result).cast::<u16>();
                            (__p3).write(((__p3).read()).wrapping_add(1));
                        }
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        if ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) != 0i32 {
                            let __p4 = (&raw mut gSpecialVar_0x8005).cast::<u16>();
                            (__p4).write(((__p4).read()).wrapping_sub(1));
                            let __p5 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(2076);
                            (__p5).write(((__p5).read()).wrapping_add(1));
                            if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(2076))
                            .read()) as i32)
                                > 120i32
                            {
                                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(2076))
                                .write(120u8);
                                let __p6 = (&raw mut gSpecialVar_Result).cast::<u16>();
                                (__p6).write(((__p6).read()).wrapping_add(1));
                            }
                            WriteBattlePyramidViewScanlineEffectBuffer();
                        } else {
                            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(2u16);
                        }
                        break 'l2;
                    }
                    if __sw2 == 2i32 || !__matched {
                        break 'l2;
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ClearPyramidPartyHeldItems() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut item: u16 = 0u16;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j
                                < (if 3i32 >= (if 4i32 >= 2i32 { 4i32 } else { 2i32 }) {
                                    3i32
                                } else {
                                    (if 4i32 >= 2i32 { 4i32 } else { 2i32 })
                                }))
                            {
                                break 'l3;
                            }
                            'l4: {
                                if ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1612))
                                .wrapping_add(1630))
                                .cast::<u16>())
                                .wrapping_offset((j) as isize))
                                .read()) as i32)
                                    != 0i32)
                                    && ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(1630))
                                    .cast::<u16>())
                                    .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                        .wrapping_sub(1i32)
                                        == i)
                                {
                                    SetMonData(
                                        ((&raw mut gPlayerParty).cast::<u8>())
                                            .wrapping_offset((i) as isize * 100),
                                        12i32,
                                        (&raw mut item).cast::<u8>(),
                                    );
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetPyramidFloorPalette() {
    unsafe {
        CreateTask(Some(Task_SetPyramidFloorPalette), 0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_SetPyramidFloorPalette(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0
        {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                ((((&raw mut gBattlePyramidFloor_Pal).cast::<u8>())
                                    .wrapping_offset(
                                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1638)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize
                                            * 32,
                                    ))
                                .cast::<u16>())
                                .cast::<u8>(),
                                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(96))
                                .cast::<u8>(),
                                (0u32
                                    | (crate::c::div_u32(
                                        32u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn BattlePyramidStartMenu() {
    unsafe {
        ShowBattlePyramidStartMenu();
    }
}
pub(crate) unsafe extern "C" fn RestorePyramidPlayerParty() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        let mut l: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    let mut partyIndex: i32 =
                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1630))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                            .wrapping_sub(1i32);
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 3i32) {
                                break 'l3;
                            }
                            'l4: {
                                if GetMonData3(
                                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(568))
                                    .cast::<u8>())
                                    .wrapping_offset((partyIndex) as isize * 100),
                                    11i32,
                                    core::ptr::null_mut(),
                                ) == GetMonData3(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset((j) as isize * 100),
                                    11i32,
                                    core::ptr::null_mut(),
                                ) {
                                    {
                                        k = 0i32;
                                        'l5: loop {
                                            if !(k < 4i32) {
                                                break 'l5;
                                            }
                                            'l6: {
                                                {
                                                    l = 0i32;
                                                    'l7: loop {
                                                        if !(l < 4i32) {
                                                            break 'l7;
                                                        }
                                                        'l8: {
                                                            if GetMonData3(
                                                                (((((&raw mut gSaveBlock1Ptr)
                                                                    .cast::<*mut u8>())
                                                                .read())
                                                                .wrapping_add(568))
                                                                .cast::<u8>())
                                                                .wrapping_offset(
                                                                    (partyIndex) as isize * 100,
                                                                ),
                                                                (13i32).wrapping_add(l),
                                                                core::ptr::null_mut(),
                                                            ) == GetMonData3(
                                                                ((&raw mut gPlayerParty)
                                                                    .cast::<u8>())
                                                                .wrapping_offset(
                                                                    (j) as isize * 100,
                                                                ),
                                                                (13i32).wrapping_add(k),
                                                                core::ptr::null_mut(),
                                                            ) {
                                                                break 'l7;
                                                            }
                                                        }
                                                        l = (l).wrapping_add(1);
                                                    }
                                                }
                                                if l == 4i32 {
                                                    SetMonMoveSlot(
                                                        ((&raw mut gPlayerParty).cast::<u8>())
                                                            .wrapping_offset((j) as isize * 100),
                                                        166u16,
                                                        ((k) as u8),
                                                    );
                                                }
                                            }
                                            k = (k).wrapping_add(1);
                                        }
                                    }
                                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(568))
                                    .cast::<u8>())
                                    .wrapping_offset((partyIndex) as isize * 100)
                                    .cast::<crate::c::Rec4<100>>()
                                    .write_unaligned(
                                        ((&raw mut gPlayerParty).cast::<u8>())
                                            .wrapping_offset((j) as isize * 100)
                                            .cast::<crate::c::Rec4<100>>()
                                            .read_unaligned(),
                                    );
                                    (((&raw mut gSelectedOrderFromParty).cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                    .write((((partyIndex).wrapping_add(1i32)) as u8));
                                    break 'l3;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l9: loop {
                if !(i < 3i32) {
                    break 'l9;
                }
                'l10: {
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1630))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(
                        (((((&raw mut gSelectedOrderFromParty).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetPostBattleDirectionHintTextIndex(
    hintType: *mut i32,
    minDistanceForExitHint: u8,
    defaultHintType: u8,
) -> u8 {
    unsafe {
        let mut hintType = hintType;
        let mut minDistanceForExitHint = minDistanceForExitHint;
        let mut defaultHintType = defaultHintType;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        let mut textIndex: u8 = 0u8;
        let mut map: *mut u16 = (((&raw mut gBackupMapLayout).cast::<u8>())
            .wrapping_add(8)
            .cast::<*mut u16>())
        .read();
        map = (map).wrapping_offset(
            ((((((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read())
                .wrapping_mul(7i32))
            .wrapping_add(7i32)) as isize,
        );
        {
            y = 0i32;
            'l1: loop {
                if !(y < 32i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        x = 0i32;
                        'l3: loop {
                            if !(x < 32i32) {
                                break 'l3;
                            }
                            'l4: {
                                if (((((map).wrapping_offset((x) as isize)).read()) as i32)
                                    & 1023i32)
                                    == 654i32
                                {
                                    x = (x).wrapping_sub(
                                        (((((((&raw mut gObjectEvents).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gSelectedObjectEvent).cast::<u8>())
                                                    .read())
                                                    as i32)
                                                    as isize
                                                    * 36,
                                            ))
                                        .wrapping_add(12))
                                        .cast::<i16>())
                                        .read()) as i32)
                                            .wrapping_sub(7i32),
                                    );
                                    y = (y).wrapping_sub(
                                        (((((((&raw mut gObjectEvents).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gSelectedObjectEvent).cast::<u8>())
                                                    .read())
                                                    as i32)
                                                    as isize
                                                    * 36,
                                            ))
                                        .wrapping_add(12))
                                        .wrapping_add(2)
                                        .cast::<i16>())
                                        .read()) as i32)
                                            .wrapping_sub(7i32),
                                    );
                                    if ((((x >= ((minDistanceForExitHint) as i32))
                                        || (x
                                            <= ((minDistanceForExitHint) as i32).wrapping_neg()))
                                        || (y >= ((minDistanceForExitHint) as i32)))
                                        || (y <= ((minDistanceForExitHint) as i32).wrapping_neg()))
                                        || (((defaultHintType) as i32) == 0i32)
                                    {
                                        if (x > 0i32) && (y > 0i32) {
                                            if x >= y {
                                                textIndex = 2u8;
                                            } else {
                                                textIndex = 3u8;
                                            }
                                        } else {
                                            if (x < 0i32) && (y < 0i32) {
                                                if x > y {
                                                    textIndex = 0u8;
                                                } else {
                                                    textIndex = 1u8;
                                                }
                                            } else {
                                                if x == 0i32 {
                                                    if y > 0i32 {
                                                        textIndex = 3u8;
                                                    } else {
                                                        textIndex = 0u8;
                                                    }
                                                } else {
                                                    if y == 0i32 {
                                                        if x > 0i32 {
                                                            textIndex = 2u8;
                                                        } else {
                                                            textIndex = 1u8;
                                                        }
                                                    } else {
                                                        if x < 0i32 {
                                                            if (x).wrapping_add(y) > 0i32 {
                                                                textIndex = 3u8;
                                                            } else {
                                                                textIndex = 1u8;
                                                            }
                                                        } else {
                                                            if (x).wrapping_add(y) >= 0i32 {
                                                                textIndex = 2u8;
                                                            } else {
                                                                textIndex = 0u8;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        (hintType).write(0i32);
                                    } else {
                                        (hintType).write(((defaultHintType) as i32));
                                    }
                                    return textIndex;
                                }
                            }
                            x = (x).wrapping_add(1);
                        }
                    }
                    map = (map).wrapping_offset(47);
                }
                y = (y).wrapping_add(1);
            }
        }
        return textIndex;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LocalIdToPyramidTrainerId(localId: u8) -> u16 {
    unsafe {
        let mut localId = localId;
        return (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1640))
        .cast::<u16>())
        .wrapping_offset((((localId) as i32).wrapping_sub(1i32)) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlePyramidTrainerFlag(eventId: u8) -> u8 {
    unsafe {
        let mut eventId = eventId;
        return (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2014))
        .read()) as u32)
            & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                (((((((&raw mut gObjectEvents).cast::<u8>())
                    .wrapping_offset(((eventId) as i32) as isize * 36))
                .wrapping_add(8))
                .read()) as i32)
                    .wrapping_sub(1i32)) as isize,
            ))
            .read()) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MarkApproachingPyramidTrainersAsBattled() {
    unsafe {
        MarkPyramidTrainerAsBattled(((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read());
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 32768u32) != 0 {
            ((&raw mut gSelectedObjectEvent).cast::<u8>())
                .write(GetChosenApproachingTrainerObjectEventId(1u8));
            MarkPyramidTrainerAsBattled(((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read());
        }
    }
}
pub(crate) unsafe extern "C" fn MarkPyramidTrainerAsBattled(trainerId: u16) {
    unsafe {
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1640))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == ((trainerId) as i32)
                    {
                        let __p1 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(2014);
                        (__p1).write(
                            (((((__p1).read()) as u32)
                                | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset((i) as isize))
                                .read()) as u8),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            ((((&raw mut gSelectedObjectEvent).cast::<u8>()).read()) as i32) as isize * 36,
        ))
        .wrapping_add(6))
        .write(2u8);
        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(3184))
            .cast::<u8>())
        .wrapping_offset(
            (((((&raw mut gSpecialVar_LastTalked).cast::<u16>()).read()) as i32).wrapping_sub(1i32))
                as isize
                * 24,
        ))
        .wrapping_add(9))
        .write(2u8);
        (((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            ((((&raw mut gSelectedObjectEvent).cast::<u8>()).read()) as i32) as isize * 36,
        ))
        .wrapping_add(12))
        .cast::<i16>())
        .write(
            (((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSelectedObjectEvent).cast::<u8>()).read()) as i32) as isize * 36,
            ))
            .wrapping_add(16))
            .cast::<i16>())
            .read(),
        );
        (((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            ((((&raw mut gSelectedObjectEvent).cast::<u8>()).read()) as i32) as isize * 36,
        ))
        .wrapping_add(12))
        .wrapping_add(2)
        .cast::<i16>())
        .write(
            (((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSelectedObjectEvent).cast::<u8>()).read()) as i32) as isize * 36,
            ))
            .wrapping_add(16))
            .wrapping_add(2)
            .cast::<i16>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GenerateBattlePyramidWildMon() {
    unsafe {
        let mut name = crate::ffi::Align4([0u8; 11]);
        let mut i: i32 = 0i32;
        let mut wildMons: *mut u8 = core::ptr::null_mut();
        let mut id: u32 = 0u32;
        let mut lvl: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        let mut round: u16 = ((crate::c::rem_i32(
            crate::c::div_i32(
                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1998))
                .cast::<u16>())
                .wrapping_offset(((lvl) as i32) as isize))
                .read()) as i32),
                7i32,
            ),
            20i32,
        )) as u16);
        if ((round) as i32) >= 20i32 {
            round = 19u16;
        }
        if lvl != 0u32 {
            wildMons = ((((&raw const sOpenLevelWildMonPointers)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((round) as i32) as isize))
            .read();
        } else {
            wildMons = ((((&raw const sLevel50WildMonPointers)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((round) as i32) as isize))
            .read();
        }
        id = (GetMonData3(
            (&raw mut gEnemyParty).cast::<u8>(),
            11i32,
            core::ptr::null_mut(),
        ))
        .wrapping_sub(1u32);
        SetMonData(
            (&raw mut gEnemyParty).cast::<u8>(),
            11i32,
            (((wildMons).wrapping_offset(((id) as i32) as isize * 12)).cast::<u16>()).cast::<u8>(),
        );
        GetSpeciesName(
            (&raw mut name).cast::<u8>(),
            (((wildMons).wrapping_offset(((id) as i32) as isize * 12)).cast::<u16>()).read(),
        );
        SetMonData(
            (&raw mut gEnemyParty).cast::<u8>(),
            2i32,
            ((&raw mut name).cast::<u8>()).cast::<u8>(),
        );
        if lvl != 0u32 {
            lvl = ((SetFacilityPtrsGetLevel()) as u32);
            lvl = (lvl).wrapping_sub(
                (((((wildMons).wrapping_offset(((id) as i32) as isize * 12)).wrapping_add(2))
                    .read()) as u32),
            );
            lvl = ((lvl).wrapping_sub(5u32))
                .wrapping_add(((crate::c::rem_i32(((Random()) as i32), 11i32)) as u32));
        } else {
            lvl = ((((((((wildMons).wrapping_offset(((id) as i32) as isize * 12)).wrapping_add(2))
                .read()) as i32)
                .wrapping_sub(5i32))
            .wrapping_add(crate::c::rem_i32(((Random()) as i32), 11i32)))
                as u32);
        }
        SetMonData(
            (&raw mut gEnemyParty).cast::<u8>(),
            25i32,
            (((((&raw mut gExperienceTables).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                    (((((wildMons).wrapping_offset(((id) as i32) as isize * 12)).cast::<u16>())
                        .read()) as i32) as isize
                        * 28,
                ))
                .wrapping_add(19))
                .read()) as i32) as isize
                    * 404,
            ))
            .cast::<u32>())
            .wrapping_offset(((lvl) as i32) as isize))
            .cast::<u8>(),
        );
        'l1: {
            let __sw1 = (((((wildMons).wrapping_offset(((id) as i32) as isize * 12))
                .wrapping_add(3))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 || __sw1 == 1i32 {
                SetMonData(
                    (&raw mut gEnemyParty).cast::<u8>(),
                    46i32,
                    ((wildMons).wrapping_offset(((id) as i32) as isize * 12)).wrapping_add(3),
                );
                break 'l1;
            }
            if __sw1 == 2i32 || !__matched {
                if (((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                    (((((wildMons).wrapping_offset(((id) as i32) as isize * 12)).cast::<u16>())
                        .read()) as i32) as isize
                        * 28,
                ))
                .wrapping_add(22))
                .cast::<u8>())
                .wrapping_offset(1))
                .read())
                    != 0
                {
                    i = ((crate::c::rem_u32(
                        GetMonData3(
                            (&raw mut gEnemyParty).cast::<u8>(),
                            0i32,
                            core::ptr::null_mut(),
                        ),
                        2u32,
                    )) as i32);
                    SetMonData(
                        (&raw mut gEnemyParty).cast::<u8>(),
                        46i32,
                        (&raw mut i).cast::<u8>(),
                    );
                } else {
                    i = 0i32;
                    SetMonData(
                        (&raw mut gEnemyParty).cast::<u8>(),
                        46i32,
                        (&raw mut i).cast::<u8>(),
                    );
                }
                break 'l1;
            }
        }
        {
            i = 0i32;
            'l2: loop {
                if !(i < 4i32) {
                    break 'l2;
                }
                'l3: {
                    SetMonMoveSlot(
                        (&raw mut gEnemyParty).cast::<u8>(),
                        (((((wildMons).wrapping_offset(((id) as i32) as isize * 12))
                            .wrapping_add(4))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                        ((i) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1998))
        .cast::<u16>())
        .wrapping_offset(
            ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                0,
                2,
                false,
            ) as u8) as i32) as isize,
        ))
        .read()) as i32)
            >= 140i32
        {
            id = (((crate::c::rem_i32(((Random()) as i32), 17i32)).wrapping_add(15i32)) as u32);
            {
                i = 0i32;
                'l4: loop {
                    if !(i < 6i32) {
                        break 'l4;
                    }
                    'l5: {
                        SetMonData(
                            (&raw mut gEnemyParty).cast::<u8>(),
                            (39i32).wrapping_add(i),
                            (&raw mut id).cast::<u8>(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        CalculateMonStats((&raw mut gEnemyParty).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPyramidRunMultiplier() -> u8 {
    unsafe {
        let mut id: u8 = GetPyramidFloorTemplateId();
        return (((((&raw const sPyramidFloorTemplates).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 16))
        .wrapping_add(4))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CurrentBattlePyramidLocation() -> u8 {
    unsafe {
        if (((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(18)
            .cast::<u16>())
        .read()) as i32)
            == 361i32
        {
            return 1u8;
        } else {
            if (((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as i32)
                == 378i32
            {
                return 2u8;
            } else {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InBattlePyramid_() -> u8 {
    unsafe {
        return ((((((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(18)
            .cast::<u16>())
        .read()) as i32)
            == 361i32)
            || ((((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as i32)
                == 378i32)) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PausePyramidChallenge() {
    unsafe {
        if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
            RestorePyramidPlayerParty();
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1628))
            .write(2u8);
            VarSet(16398u16, 0u16);
            LoadPlayerParty();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoftResetInBattlePyramid() {
    unsafe {
        if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
            DoSoftReset();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyPyramidTrainerSpeechBefore(trainerId: u16) {
    unsafe {
        let mut trainerId = trainerId;
        FrontierSpeechToString(
            (((((&raw mut gFacilityTrainers).cast::<*mut u8>()).read())
                .wrapping_offset(((trainerId) as i32) as isize * 52))
            .wrapping_add(12))
            .cast::<u16>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyPyramidTrainerWinSpeech(trainerId: u16) {
    unsafe {
        let mut trainerId = trainerId;
        FrontierSpeechToString(
            (((((&raw mut gFacilityTrainers).cast::<*mut u8>()).read())
                .wrapping_offset(((trainerId) as i32) as isize * 52))
            .wrapping_add(24))
            .cast::<u16>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyPyramidTrainerLoseSpeech(trainerId: u16) {
    unsafe {
        let mut trainerId = trainerId;
        FrontierSpeechToString(
            (((((&raw mut gFacilityTrainers).cast::<*mut u8>()).read())
                .wrapping_offset(((trainerId) as i32) as isize * 52))
            .wrapping_add(36))
            .cast::<u16>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerEncounterMusicIdInBattlePyramid(trainerId: u16) -> u8 {
    unsafe {
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(216u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sTrainerClassEncounterMusic)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 4))
                    .read()) as i32)
                        == (((((&raw mut gFacilityClassToTrainerClass).cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gFacilityTrainers).cast::<*mut u8>()).read())
                                    .wrapping_offset(((trainerId) as i32) as isize * 52))
                                .read()) as i32) as isize,
                            ))
                        .read()) as i32)
                    {
                        return (((((&raw const sTrainerClassEncounterMusic)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                        .wrapping_add(1))
                        .read();
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn BattlePyramidRetireChallenge() {
    unsafe {
        ScriptContext_SetupScript((&raw mut BattlePyramid_Retire).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn GetUniqueTrainerId(objectEventId: u8) -> u16 {
    unsafe {
        let mut objectEventId = objectEventId;
        let mut i: i32 = 0i32;
        let mut trainerId: u16 = 0u16;
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        let mut challengeNum: u32 = ((crate::c::div_i32(
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1998))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .read()) as i32),
            7i32,
        )) as u32);
        let mut floor: u32 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1638)
        .cast::<u16>())
        .read()) as u32);
        if floor == 7u32 {
            'l1: loop {
                'l2: {
                    trainerId = GetRandomScaledFrontierTrainerId(
                        (((challengeNum).wrapping_add(1u32)) as u8),
                        ((floor) as u8),
                    );
                    {
                        i = 0i32;
                        'l3: loop {
                            if !(i < ((objectEventId) as i32)) {
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
                if !(i != ((objectEventId) as i32)) {
                    break 'l1;
                }
            }
        } else {
            'l5: loop {
                'l6: {
                    trainerId =
                        GetRandomScaledFrontierTrainerId(((challengeNum) as u8), ((floor) as u8));
                    {
                        i = 0i32;
                        'l7: loop {
                            if !(i < ((objectEventId) as i32)) {
                                break 'l7;
                            }
                            'l8: {
                                if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1640))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)
                                    == ((trainerId) as i32)
                                {
                                    break 'l7;
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                if !(i != ((objectEventId) as i32)) {
                    break 'l5;
                }
            }
        }
        return trainerId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GenerateBattlePyramidFloorLayout(
    backupMapData: *mut u16,
    setPlayerPosition: u8,
) {
    unsafe {
        let mut backupMapData = backupMapData;
        let mut setPlayerPosition = setPlayerPosition;
        let mut y: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut entranceSquareId: u8 = 0u8;
        let mut exitSquareId: u8 = 0u8;
        let mut floorLayoutOffsets: *mut u8 = AllocZeroed(16u32);
        GetPyramidFloorLayoutOffsets(floorLayoutOffsets);
        GetPyramidEntranceAndExitSquareIds(&raw mut entranceSquareId, &raw mut exitSquareId);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 16i32) {
                    break 'l1;
                }
                'l2: {
                    let mut map: *mut u16 = core::ptr::null_mut();
                    let mut mapLayout: *mut u8 = ((((&raw mut gMapLayouts).cast::<*mut u8>())
                        .cast::<*mut u8>())
                    .wrapping_offset(
                        (((((floorLayoutOffsets).wrapping_offset((i) as isize)).read()) as i32)
                            .wrapping_add(361i32)) as isize,
                    ))
                    .read();
                    let mut layoutMap: *mut u16 =
                        ((mapLayout).wrapping_add(12).cast::<*mut u16>()).read();
                    (((&raw mut gBackupMapLayout).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<*mut u16>())
                    .write(backupMapData);
                    (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).write(
                        ((((mapLayout).cast::<i32>()).read()).wrapping_mul(4i32))
                            .wrapping_add(15i32),
                    );
                    (((&raw mut gBackupMapLayout).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<i32>())
                    .write(
                        ((((mapLayout).wrapping_add(4).cast::<i32>()).read()).wrapping_mul(4i32))
                            .wrapping_add(14i32),
                    );
                    map = (((&raw mut gBackupMapLayout).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<*mut u16>())
                    .read();
                    map = (map).wrapping_offset(
                        (((((((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read())
                            .wrapping_mul((7i32).wrapping_add(
                                (crate::c::div_i32(i, 4i32)).wrapping_mul(
                                    ((mapLayout).wrapping_add(4).cast::<i32>()).read(),
                                ),
                            )))
                        .wrapping_add(7i32))
                        .wrapping_add(
                            (crate::c::rem_i32(i, 4i32))
                                .wrapping_mul(((mapLayout).cast::<i32>()).read()),
                        )) as isize,
                    );
                    {
                        y = 0i32;
                        'l3: loop {
                            if !(y < ((mapLayout).wrapping_add(4).cast::<i32>()).read()) {
                                break 'l3;
                            }
                            'l4: {
                                {
                                    x = 0i32;
                                    'l5: loop {
                                        if !(x < ((mapLayout).cast::<i32>()).read()) {
                                            break 'l5;
                                        }
                                        'l6: {
                                            if (((((layoutMap).wrapping_offset((x) as isize))
                                                .read())
                                                as i32)
                                                & 1023i32)
                                                != 654i32
                                            {
                                                ((map).wrapping_offset((x) as isize)).write(
                                                    ((layoutMap).wrapping_offset((x) as isize))
                                                        .read(),
                                                );
                                            } else {
                                                if i != ((exitSquareId) as i32) {
                                                    if (i == ((entranceSquareId) as i32))
                                                        && (((setPlayerPosition) as i32) == 0i32)
                                                    {
                                                        ((((&raw mut gSaveBlock1Ptr)
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .cast::<i16>())
                                                        .write(
                                                            ((((((mapLayout).cast::<i32>())
                                                                .read())
                                                            .wrapping_mul(crate::c::rem_i32(
                                                                i, 4i32,
                                                            )))
                                                            .wrapping_add(x))
                                                                as i16),
                                                        );
                                                        ((((&raw mut gSaveBlock1Ptr)
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(2)
                                                        .cast::<i16>())
                                                        .write(
                                                            ((((((mapLayout)
                                                                .wrapping_add(4)
                                                                .cast::<i32>())
                                                            .read())
                                                            .wrapping_mul(crate::c::div_i32(
                                                                i, 4i32,
                                                            )))
                                                            .wrapping_add(y))
                                                                as i16),
                                                        );
                                                    }
                                                    ((map).wrapping_offset((x) as isize)).write(
                                                        (((((((layoutMap)
                                                            .wrapping_offset((x) as isize))
                                                        .read())
                                                            as i32)
                                                            & 64512i32)
                                                            | 653i32)
                                                            as u16),
                                                    );
                                                } else {
                                                    ((map).wrapping_offset((x) as isize)).write(
                                                        ((layoutMap).wrapping_offset((x) as isize))
                                                            .read(),
                                                    );
                                                }
                                            }
                                        }
                                        x = (x).wrapping_add(1);
                                    }
                                }
                                map = (map).wrapping_offset(
                                    ((15i32).wrapping_add(
                                        (((mapLayout).cast::<i32>()).read()).wrapping_mul(4i32),
                                    )) as isize,
                                );
                                layoutMap = (layoutMap)
                                    .wrapping_offset((((mapLayout).cast::<i32>()).read()) as isize);
                            }
                            y = (y).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        RunOnLoadMapScript();
        Free(floorLayoutOffsets);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadBattlePyramidObjectEventTemplates() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut id: u8 = 0u8;
        let mut entranceSquareId: u8 = 0u8;
        let mut exitSquareId: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1640))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(65535u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        id = GetPyramidFloorTemplateId();
        GetPyramidEntranceAndExitSquareIds(&raw mut entranceSquareId, &raw mut exitSquareId);
        'l3: loop {
            'l4: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l5: loop {
                        'l6: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(3184))
                                .cast::<u8>(),
                                (83886080u32
                                    | (crate::c::div_u32(
                                        1536u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l5;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l3;
            }
        }
        {
            i = 0i32;
            'l7: loop {
                if !(i < 2i32) {
                    break 'l7;
                }
                'l8: {
                    let mut objectPositionsType: u8 = 0u8;
                    if i == 0i32 {
                        objectPositionsType =
                            (((((&raw const sPyramidFloorTemplates).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((id) as i32) as isize * 16))
                            .wrapping_add(3))
                            .read();
                    } else {
                        objectPositionsType =
                            (((((&raw const sPyramidFloorTemplates).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((id) as i32) as isize * 16))
                            .wrapping_add(2))
                            .read();
                    }
                    'l9: {
                        let __sw1 = ((objectPositionsType) as i32);
                        if __sw1 == 0i32 {
                            SetPyramidObjectPositionsUniformly(((i) as u8));
                            break 'l9;
                        }
                        if __sw1 == 1i32 {
                            if (SetPyramidObjectPositionsInAndNearSquare(
                                ((i) as u8),
                                entranceSquareId,
                            )) != 0
                            {
                                SetPyramidObjectPositionsUniformly(((i) as u8));
                            }
                            break 'l9;
                        }
                        if __sw1 == 2i32 {
                            if (SetPyramidObjectPositionsInAndNearSquare(((i) as u8), exitSquareId))
                                != 0
                            {
                                SetPyramidObjectPositionsUniformly(((i) as u8));
                            }
                            break 'l9;
                        }
                        if __sw1 == 3i32 {
                            if (SetPyramidObjectPositionsNearSquare(((i) as u8), entranceSquareId))
                                != 0
                            {
                                SetPyramidObjectPositionsUniformly(((i) as u8));
                            }
                            break 'l9;
                        }
                        if __sw1 == 4i32 {
                            if (SetPyramidObjectPositionsNearSquare(((i) as u8), exitSquareId)) != 0
                            {
                                SetPyramidObjectPositionsUniformly(((i) as u8));
                            }
                            break 'l9;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadBattlePyramidFloorObjectEventScripts() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut events: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(3184))
        .cast::<u8>();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 64i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((events).wrapping_offset((i) as isize * 24)).wrapping_add(1)).read())
                        as i32)
                        != 59i32
                    {
                        (((events).wrapping_offset((i) as isize * 24))
                            .wrapping_add(16)
                            .cast::<*mut u8>())
                        .write((&raw mut BattlePyramid_TrainerBattle).cast::<u8>());
                    } else {
                        (((events).wrapping_offset((i) as isize * 24))
                            .wrapping_add(16)
                            .cast::<*mut u8>())
                        .write((&raw mut BattlePyramid_FindItemBall).cast::<u8>());
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetPyramidEntranceAndExitSquareIds(
    entranceSquareId: *mut u8,
    exitSquareId: *mut u8,
) {
    unsafe {
        let mut entranceSquareId = entranceSquareId;
        let mut exitSquareId = exitSquareId;
        (entranceSquareId).write(
            ((crate::c::rem_i32(
                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(2006))
                .cast::<u16>())
                .wrapping_offset(3))
                .read()) as i32),
                16i32,
            )) as u8),
        );
        (exitSquareId).write(
            ((crate::c::rem_i32(
                ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(2006))
                .cast::<u16>())
                .read()) as i32),
                16i32,
            )) as u8),
        );
        if (((entranceSquareId).read()) as i32) == (((exitSquareId).read()) as i32) {
            (entranceSquareId).write(
                ((crate::c::rem_i32(
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(2006))
                    .cast::<u16>())
                    .wrapping_offset(3))
                    .read()) as i32)
                        .wrapping_add(1i32),
                    16i32,
                )) as u8),
            );
            (exitSquareId).write(
                ((crate::c::rem_i32(
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(2006))
                    .cast::<u16>())
                    .read()) as i32)
                        .wrapping_add(16i32))
                    .wrapping_sub(1i32),
                    16i32,
                )) as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SetPyramidObjectPositionsUniformly(objType: u8) {
    unsafe {
        let mut objType = objType;
        let mut i: i32 = 0i32;
        let mut numObjects: i32 = 0i32;
        let mut objectStartIndex: i32 = 0i32;
        let mut squareId: i32 = 0i32;
        let mut bits: u32 = 0u32;
        let mut id: u8 = GetPyramidFloorTemplateId();
        let mut floorLayoutOffsets: *mut u8 = AllocZeroed(16u32);
        GetPyramidFloorLayoutOffsets(floorLayoutOffsets);
        squareId = crate::c::rem_i32(
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2006))
            .cast::<u16>())
            .wrapping_offset(2))
            .read()) as i32),
            16i32,
        );
        if ((objType) as i32) == 0i32 {
            numObjects = (((((((&raw const sPyramidFloorTemplates).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 16))
            .wrapping_add(1))
            .read()) as i32);
            objectStartIndex = 0i32;
        } else {
            numObjects = ((((((&raw const sPyramidFloorTemplates).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 16))
            .read()) as i32);
            objectStartIndex =
                (((((((&raw const sPyramidFloorTemplates).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 16))
                .wrapping_add(1))
                .read()) as i32);
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < numObjects) {
                    break 'l1;
                }
                'l2: {
                    'l3: loop {
                        'l4: {
                            'l5: loop {
                                'l6: {
                                    if (bits & 1u32) != 0 {
                                        if !((((((&raw mut gBitTable).cast::<u32>())
                                            .cast::<u32>())
                                        .wrapping_offset((squareId) as isize))
                                        .read()
                                            & (((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(2006))
                                            .cast::<u16>())
                                            .wrapping_offset(3))
                                            .read())
                                                as u32))
                                            != 0)
                                        {
                                            bits = (bits | 2u32);
                                        }
                                    } else {
                                        if (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                            .wrapping_offset((squareId) as isize))
                                        .read()
                                            & (((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(2006))
                                            .cast::<u16>())
                                            .wrapping_offset(3))
                                            .read())
                                                as u32))
                                            != 0
                                        {
                                            bits = (bits | 2u32);
                                        }
                                    }
                                    if {
                                        let __t1 = (squareId).wrapping_add(1);
                                        squareId = __t1;
                                        __t1
                                    } >= 16i32
                                    {
                                        squareId = 0i32;
                                    }
                                    if squareId
                                        == crate::c::rem_i32(
                                            (((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(2006))
                                            .cast::<u16>())
                                            .wrapping_offset(2))
                                            .read())
                                                as i32),
                                            16i32,
                                        )
                                    {
                                        if (bits & 1u32) != 0 {
                                            bits = (bits | 6u32);
                                        } else {
                                            bits = (bits | 1u32);
                                        }
                                    }
                                }
                                if !(!((bits & 2u32) != 0)) {
                                    break 'l5;
                                }
                            }
                        }
                        if !((!((bits & 4u32) != 0))
                            && ((TrySetPyramidObjectEventPositionInSquare(
                                objType,
                                floorLayoutOffsets,
                                ((squareId) as u8),
                                (((objectStartIndex).wrapping_add(i)) as u8),
                            )) != 0))
                        {
                            break 'l3;
                        }
                    }
                    bits = (bits & 1u32);
                }
                i = (i).wrapping_add(1);
            }
        }
        Free(floorLayoutOffsets);
    }
}
pub(crate) unsafe extern "C" fn SetPyramidObjectPositionsInAndNearSquare(
    objType: u8,
    squareId: u8,
) -> u8 {
    unsafe {
        let mut objType = objType;
        let mut squareId = squareId;
        let mut i: i32 = 0i32;
        let mut objectStartIndex: i32 = 0i32;
        let mut borderedIndex: i32 = 0i32;
        let mut r7: i32 = 0i32;
        let mut numPlacedObjects: i32 = 0i32;
        let mut numObjects: i32 = 0i32;
        let mut id: u8 = GetPyramidFloorTemplateId();
        let mut floorLayoutOffsets: *mut u8 = AllocZeroed(16u32);
        GetPyramidFloorLayoutOffsets(floorLayoutOffsets);
        if ((objType) as i32) == 0i32 {
            numObjects = (((((((&raw const sPyramidFloorTemplates).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 16))
            .wrapping_add(1))
            .read()) as i32);
            objectStartIndex = 0i32;
        } else {
            numObjects = ((((((&raw const sPyramidFloorTemplates).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 16))
            .read()) as i32);
            objectStartIndex =
                (((((((&raw const sPyramidFloorTemplates).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 16))
                .wrapping_add(1))
                .read()) as i32);
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < numObjects) {
                    break 'l1;
                }
                'l2: {
                    if r7 == 0i32 {
                        if (TrySetPyramidObjectEventPositionInSquare(
                            objType,
                            floorLayoutOffsets,
                            squareId,
                            (((objectStartIndex).wrapping_add(i)) as u8),
                        )) != 0
                        {
                            r7 = 1i32;
                        } else {
                            numPlacedObjects = (numPlacedObjects).wrapping_add(1);
                        }
                    }
                    if (r7 & 1i32) != 0 {
                        if (TrySetPyramidObjectEventPositionInSquare(
                            objType,
                            floorLayoutOffsets,
                            ((((((&raw const sBorderedSquareIds).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((squareId) as i32) as isize * 4))
                            .cast::<u8>())
                            .wrapping_offset((borderedIndex) as isize))
                            .read(),
                            (((objectStartIndex).wrapping_add(i)) as u8),
                        )) != 0
                        {
                            'l3: loop {
                                'l4: {
                                    borderedIndex = (borderedIndex).wrapping_add(1);
                                    if (((((((((&raw const sBorderedSquareIds)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((squareId) as i32) as isize * 4))
                                    .cast::<u8>())
                                    .wrapping_offset((borderedIndex) as isize))
                                    .read()) as i32)
                                        == 255i32)
                                        || (borderedIndex >= 4i32)
                                    {
                                        borderedIndex = 0i32;
                                    }
                                    r7 = (r7).wrapping_add(2i32);
                                }
                                if !(((r7 >> 1) != 4i32)
                                    && ((TrySetPyramidObjectEventPositionInSquare(
                                        objType,
                                        floorLayoutOffsets,
                                        ((((((&raw const sBorderedSquareIds)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((squareId) as i32) as isize * 4))
                                        .cast::<u8>())
                                        .wrapping_offset((borderedIndex) as isize))
                                        .read(),
                                        (((objectStartIndex).wrapping_add(i)) as u8),
                                    )) != 0))
                                {
                                    break 'l3;
                                }
                            }
                            numPlacedObjects = (numPlacedObjects).wrapping_add(1);
                        } else {
                            borderedIndex = (borderedIndex).wrapping_add(1);
                            if (((((((((&raw const sBorderedSquareIds).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((squareId) as i32) as isize * 4))
                            .cast::<u8>())
                            .wrapping_offset((borderedIndex) as isize))
                            .read()) as i32)
                                == 255i32)
                                || (borderedIndex >= 4i32)
                            {
                                borderedIndex = 0i32;
                            }
                            numPlacedObjects = (numPlacedObjects).wrapping_add(1);
                        }
                    }
                    if (r7 >> 1) == 4i32 {
                        break 'l1;
                    }
                    r7 = (r7 & 1i32);
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((crate::c::div_i32(numObjects, 2i32) > numPlacedObjects) as u8);
    }
}
pub(crate) unsafe extern "C" fn SetPyramidObjectPositionsNearSquare(
    objType: u8,
    squareId: u8,
) -> u8 {
    unsafe {
        let mut objType = objType;
        let mut squareId = squareId;
        let mut i: i32 = 0i32;
        let mut objectStartIndex: i32 = 0i32;
        let mut borderOffset: i32 = 0i32;
        let mut numPlacedObjects: i32 = 0i32;
        let mut r8: i32 = 0i32;
        let mut numObjects: i32 = 0i32;
        let mut id: u8 = GetPyramidFloorTemplateId();
        let mut floorLayoutOffsets: *mut u8 = AllocZeroed(16u32);
        GetPyramidFloorLayoutOffsets(floorLayoutOffsets);
        if ((objType) as i32) == 0i32 {
            numObjects = (((((((&raw const sPyramidFloorTemplates).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 16))
            .wrapping_add(1))
            .read()) as i32);
            objectStartIndex = 0i32;
        } else {
            numObjects = ((((((&raw const sPyramidFloorTemplates).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 16))
            .read()) as i32);
            objectStartIndex =
                (((((((&raw const sPyramidFloorTemplates).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 16))
                .wrapping_add(1))
                .read()) as i32);
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < numObjects) {
                    break 'l1;
                }
                'l2: {
                    if (TrySetPyramidObjectEventPositionInSquare(
                        objType,
                        floorLayoutOffsets,
                        ((((((&raw const sBorderedSquareIds).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((squareId) as i32) as isize * 4))
                        .cast::<u8>())
                        .wrapping_offset((borderOffset) as isize))
                        .read(),
                        (((objectStartIndex).wrapping_add(i)) as u8),
                    )) != 0
                    {
                        'l3: loop {
                            'l4: {
                                borderOffset = (borderOffset).wrapping_add(1);
                                if (((((((((&raw const sBorderedSquareIds)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((squareId) as i32) as isize * 4))
                                .cast::<u8>())
                                .wrapping_offset((borderOffset) as isize))
                                .read()) as i32)
                                    == 255i32)
                                    || (borderOffset >= 4i32)
                                {
                                    borderOffset = 0i32;
                                }
                                r8 = (r8).wrapping_add(1);
                            }
                            if !((r8 != 4i32)
                                && ((TrySetPyramidObjectEventPositionInSquare(
                                    objType,
                                    floorLayoutOffsets,
                                    ((((((&raw const sBorderedSquareIds)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((squareId) as i32) as isize * 4))
                                    .cast::<u8>())
                                    .wrapping_offset((borderOffset) as isize))
                                    .read(),
                                    (((objectStartIndex).wrapping_add(i)) as u8),
                                )) != 0))
                            {
                                break 'l3;
                            }
                        }
                        numPlacedObjects = (numPlacedObjects).wrapping_add(1);
                    } else {
                        borderOffset = (borderOffset).wrapping_add(1);
                        if (((((((((&raw const sBorderedSquareIds).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((squareId) as i32) as isize * 4))
                        .cast::<u8>())
                        .wrapping_offset((borderOffset) as isize))
                        .read()) as i32)
                            == 255i32)
                            || (borderOffset >= 4i32)
                        {
                            borderOffset = 0i32;
                        }
                        numPlacedObjects = (numPlacedObjects).wrapping_add(1);
                    }
                    if r8 == 4i32 {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((crate::c::div_i32(numObjects, 2i32) > numPlacedObjects) as u8);
    }
}
pub(crate) unsafe extern "C" fn TrySetPyramidObjectEventPositionInSquare(
    objType: u8,
    floorLayoutOffsets: *mut u8,
    squareId: u8,
    objectEventId: u8,
) -> u8 {
    unsafe {
        let mut objType = objType;
        let mut floorLayoutOffsets = floorLayoutOffsets;
        let mut squareId = squareId;
        let mut objectEventId = objectEventId;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2006))
        .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            {
                y = 7i32;
                'l1: loop {
                    if !(y > (-1i32)) {
                        break 'l1;
                    }
                    'l2: {
                        {
                            x = 7i32;
                            'l3: loop {
                                if !(x > (-1i32)) {
                                    break 'l3;
                                }
                                'l4: {
                                    if !((TrySetPyramidObjectEventPositionAtCoords(
                                        objType,
                                        ((x) as u8),
                                        ((y) as u8),
                                        floorLayoutOffsets,
                                        squareId,
                                        objectEventId,
                                    )) != 0)
                                    {
                                        return 0u8;
                                    }
                                }
                                x = (x).wrapping_sub(1);
                            }
                        }
                    }
                    y = (y).wrapping_sub(1);
                }
            }
        } else {
            {
                y = 0i32;
                'l5: loop {
                    if !(y < 8i32) {
                        break 'l5;
                    }
                    'l6: {
                        {
                            x = 0i32;
                            'l7: loop {
                                if !(x < 8i32) {
                                    break 'l7;
                                }
                                'l8: {
                                    if !((TrySetPyramidObjectEventPositionAtCoords(
                                        objType,
                                        ((x) as u8),
                                        ((y) as u8),
                                        floorLayoutOffsets,
                                        squareId,
                                        objectEventId,
                                    )) != 0)
                                    {
                                        return 0u8;
                                    }
                                }
                                x = (x).wrapping_add(1);
                            }
                        }
                    }
                    y = (y).wrapping_add(1);
                }
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn TrySetPyramidObjectEventPositionAtCoords(
    objType: u8,
    x: u8,
    y: u8,
    floorLayoutOffsets: *mut u8,
    squareId: u8,
    objectEventId: u8,
) -> u8 {
    unsafe {
        let mut objType = objType;
        let mut x = x;
        let mut y = y;
        let mut floorLayoutOffsets = floorLayoutOffsets;
        let mut squareId = squareId;
        let mut objectEventId = objectEventId;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut mapHeader: *mut u8 = core::ptr::null_mut();
        let mut floorEvents: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(3184))
        .cast::<u8>();
        mapHeader = Overworld_GetMapHeaderByGroupAndId(
            25u16,
            ((((((floorLayoutOffsets).wrapping_offset(((squareId) as i32) as isize)).read())
                as i32)
                .wrapping_add(44i32)) as u16),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < (((((mapHeader).wrapping_add(4).cast::<*mut u8>()).read()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((((((mapHeader).wrapping_add(4).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 24))
                    .wrapping_add(4)
                    .cast::<i16>())
                    .read()) as i32)
                        != ((x) as i32))
                        || ((((((((((mapHeader).wrapping_add(4).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 24))
                        .wrapping_add(6)
                        .cast::<i16>())
                        .read()) as i32)
                            != ((y) as i32))
                    {
                        break 'l2;
                    }
                    if (((objType) as i32) != 0i32)
                        || ((((((((((mapHeader).wrapping_add(4).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 24))
                        .wrapping_add(1))
                        .read()) as i32)
                            == 59i32)
                    {
                        if (((objType) as i32) != 1i32)
                            || ((((((((((mapHeader).wrapping_add(4).cast::<*mut u8>()).read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 24))
                            .wrapping_add(1))
                            .read()) as i32)
                                != 59i32)
                        {
                            break 'l2;
                        }
                    }
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < ((objectEventId) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                if ((((((floorEvents).wrapping_offset((j) as isize * 24))
                                    .wrapping_add(4)
                                    .cast::<i16>())
                                .read()) as i32)
                                    == ((x) as i32).wrapping_add(
                                        (crate::c::rem_i32(((squareId) as i32), 4i32))
                                            .wrapping_mul(8i32),
                                    ))
                                    && ((((((floorEvents).wrapping_offset((j) as isize * 24))
                                        .wrapping_add(6)
                                        .cast::<i16>())
                                    .read()) as i32)
                                        == ((y) as i32).wrapping_add(
                                            (crate::c::div_i32(((squareId) as i32), 4i32))
                                                .wrapping_mul(8i32),
                                        ))
                                {
                                    break 'l3;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if j == ((objectEventId) as i32) {
                        (floorEvents)
                            .wrapping_offset(((objectEventId) as i32) as isize * 24)
                            .cast::<crate::c::Rec4<24>>()
                            .write_unaligned(
                                (((((mapHeader).wrapping_add(4).cast::<*mut u8>()).read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 24)
                                .cast::<crate::c::Rec4<24>>()
                                .read_unaligned(),
                            );
                        let __p1 = ((floorEvents)
                            .wrapping_offset(((objectEventId) as i32) as isize * 24))
                        .wrapping_add(4)
                        .cast::<i16>();
                        (__p1).write(
                            (((((__p1).read()) as i32).wrapping_add(
                                (crate::c::rem_i32(((squareId) as i32), 4i32)).wrapping_mul(8i32),
                            )) as i16),
                        );
                        let __p2 = ((floorEvents)
                            .wrapping_offset(((objectEventId) as i32) as isize * 24))
                        .wrapping_add(6)
                        .cast::<i16>();
                        (__p2).write(
                            (((((__p2).read()) as i32).wrapping_add(
                                (crate::c::div_i32(((squareId) as i32), 4i32)).wrapping_mul(8i32),
                            )) as i16),
                        );
                        ((floorEvents).wrapping_offset(((objectEventId) as i32) as isize * 24))
                            .write(((((objectEventId) as i32).wrapping_add(1i32)) as u8));
                        if (((((floorEvents)
                            .wrapping_offset(((objectEventId) as i32) as isize * 24))
                        .wrapping_add(1))
                        .read()) as i32)
                            != 59i32
                        {
                            i = ((GetUniqueTrainerId(objectEventId)) as i32);
                            (((floorEvents)
                                .wrapping_offset(((objectEventId) as i32) as isize * 24))
                            .wrapping_add(1))
                            .write(GetBattleFacilityTrainerGfxId(((i) as u16)));
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1640))
                            .cast::<u16>())
                            .wrapping_offset(((objectEventId) as i32) as isize))
                            .write(((i) as u16));
                        }
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn GetPyramidFloorLayoutOffsets(layoutOffsets: *mut u8) {
    unsafe {
        let mut layoutOffsets = layoutOffsets;
        let mut i: i32 = 0i32;
        let mut rand: i32 = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(2006))
        .cast::<u16>())
        .read()) as i32)
            | ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2006))
            .cast::<u16>())
            .wrapping_offset(1))
            .read()) as i32)
                << 16));
        let mut id: u8 = GetPyramidFloorTemplateId();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 16i32) {
                    break 'l1;
                }
                'l2: {
                    ((layoutOffsets).wrapping_offset((i) as isize)).write(
                        (((((((&raw const sPyramidFloorTemplates).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 16))
                        .wrapping_add(5))
                        .cast::<u8>())
                        .wrapping_offset(
                            (if (0i32) != 0 {
                                crate::c::rem_i32(rand, 8i32)
                            } else {
                                (rand & 7i32)
                            }) as isize,
                        ))
                        .read(),
                    );
                    rand = (rand >> 3);
                    if i == 7i32 {
                        rand = ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(2006))
                        .cast::<u16>())
                        .wrapping_offset(2))
                        .read()) as i32)
                            | ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(2006))
                            .cast::<u16>())
                            .wrapping_offset(3))
                            .read()) as i32)
                                << 16));
                        rand = (rand >> 8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetPyramidFloorTemplateId() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut rand: i32 = crate::c::rem_i32(
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2006))
            .cast::<u16>())
            .wrapping_offset(3))
            .read()) as i32),
            100i32,
        );
        let mut floor: i32 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1638)
        .cast::<u16>())
        .read()) as i32);
        {
            i = ((((((&raw const sFloorTemplateOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((floor) as isize))
            .read()) as i32);
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(68u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    if rand
                        < (((((((&raw const sPyramidFloorTemplateOptions)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 2))
                        .cast::<u8>())
                        .read()) as i32)
                    {
                        return ((((((&raw const sPyramidFloorTemplateOptions)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 2))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read();
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNumBattlePyramidObjectEvents() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut events: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(3184))
        .cast::<u8>();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((events).wrapping_offset(((i) as i32) as isize * 24)).read()) as i32)
                        == 0i32
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return i;
    }
}
pub(crate) unsafe extern "C" fn InitPyramidBagItems(lvlMode: u8) {
    unsafe {
        let mut lvlMode = lvlMode;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 10i32) {
                    break 'l1;
                }
                'l2: {
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(2016))
                    .cast::<u8>())
                    .wrapping_offset(((lvlMode) as i32) as isize * 20))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(0u16);
                    ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(2016))
                    .wrapping_add(40))
                    .cast::<u8>())
                    .wrapping_offset(((lvlMode) as i32) as isize * 10))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        AddPyramidBagItem(21u16, 1u16);
        AddPyramidBagItem(34u16, 1u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlePyramidPickupItemId() -> u16 {
    unsafe {
        let mut rand: i32 = 0i32;
        let mut i: u32 = 0u32;
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        let mut round: i32 = crate::c::div_i32(
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1998))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .read()) as i32),
            7i32,
        );
        if round >= 20i32 {
            round = 19i32;
        }
        rand = crate::c::rem_i32(((Random()) as i32), 100i32);
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(10u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sPickupPercentages).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        > rand
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if i >= 10u32 {
            i = 9u32;
        }
        if lvlMode != 0u32 {
            return ((((((&raw const sPickupItemsLvlOpen).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((round) as isize * 20))
            .cast::<u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read();
        } else {
            return ((((((&raw const sPickupItemsLvl50).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((round) as isize * 20))
            .cast::<u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read();
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
