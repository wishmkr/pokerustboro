//! Translated from `src/trainer_hill.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sChallenge_JPDefault sFloors_JPDefault sChallenge_Normal sFloors_Normal sChallenge_Variety sFloors_Variety sChallenge_Unique sFloors_Unique sChallenge_Expert sFloors_Expert sTrainerClassesAndMusic sPrizeListRareCandy1 sPrizeListLuxuryBall1 sPrizeListMaxRevive1 sPrizeListMaxEther1 sPrizeListElixir1 sPrizeListRoar sPrizeListSludgeBomb sPrizeListToxic sPrizeListSunnyDay sPrizeListEarthQuake sPrizeListRareCandy2 sPrizeListLuxuryBall2 sPrizeListMaxRevive2 sPrizeListMaxEther2 sPrizeListElixir2 sPrizeListBrickBreak sPrizeListTorment sPrizeListSkillSwap sPrizeListGigaDrain sPrizeListAttract sPrizeLists1 sPrizeLists2 sPrizeListSets sEReader_Pal sRecordWinColors sChallengeData sFloorStrings sHillFunctions sModeStrings sTrainerObjectEventTemplate sNextFloorMapNum sTrainerPartySlots
#[allow(unused_imports)]
use crate::data::trainer_hill::*;

pub(crate) static mut sHillData: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFloorTrainers: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gTrainerHillVBlankCounter: *mut u32 = core::ptr::null_mut();

unsafe extern "C" {
    static mut TrainerHill_EventScript_TrainerBattle: u8;
    static mut gBackupMapLayout: u8;
    static mut gBattleOutcome: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBitTable: u8;
    static mut gEnemyParty: u8;
    static mut gExperienceTables: u8;
    static mut gFacilityClassToPicIndex: u8;
    static mut gFacilityClassToTrainerClass: u8;
    static mut gMapHeader: u8;
    static mut gObjectEvents: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_LastTalked: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesInfo: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gText_TimeBoard: u8;
    static mut gText_TimeCleared: u8;
    static mut gText_XMinYDotZSec: u8;
    static mut gTrainerBattleOpponent_A: u8;
    static mut gTrainerBattleOpponent_B: u8;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn CalculateMonStats(a0: *mut u8);
    fn ClearTrainerHillVBlankCounter();
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateBattleTowerMon(a0: *mut u8, a1: *mut u8);
    fn FacilityClassToGraphicsId(a0: u8) -> u8;
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FrontierSpeechToString(a0: *mut u16);
    fn GetHighestLevelInPlayerParty() -> i32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn InitMapFromSavedGame();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn Overworld_GetMapHeaderByGroupAndId(a0: u16, a1: u16) -> *mut u8;
    fn PutWindowTilemap(a0: u8);
    fn ReadTrainerHillAndValidate() -> u32;
    fn RunOnLoadMapScript();
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetTrainerHillVBlankCounter(a0: *mut u32);
    fn ShowFieldMessageFromBuffer() -> u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn VarGet(a0: u16) -> u16;
    fn ZeroEnemyPartyMons();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallTrainerHillFunction() {
    unsafe {
        SetUpDataStruct();
        (((((&raw const sHillFunctions)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()();
        FreeDataStruct();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetTrainerHillResults() {
    unsafe {
        let mut i: i32 = 0i32;
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2221),
            7,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2221),
            0,
            7,
            (0u8) as i32,
        );
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
            .wrapping_add(4)
            .cast::<u32>())
        .write(0u32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    SetTimerValue(
                        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(14104))
                        .cast::<u32>())
                        .wrapping_offset((i) as isize),
                        215999u32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetFloorId() -> u8 {
    unsafe {
        return (((((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(18)
            .cast::<u16>())
        .read()) as i32)
            .wrapping_sub(415i32)) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerHillOpponentClass(trainerId: u16) -> u8 {
    unsafe {
        let mut trainerId = trainerId;
        let mut id: u8 = ((((trainerId) as i32).wrapping_sub(1i32)) as u8);
        return (((&raw mut gFacilityClassToTrainerClass).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sFloorTrainers).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(22))
            .cast::<u8>())
            .wrapping_offset(((id) as i32) as isize))
            .read()) as i32) as isize,
        ))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerHillTrainerName(dst: *mut u8, trainerId: u16) {
    unsafe {
        let mut dst = dst;
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        let mut id: u8 = ((((trainerId) as i32).wrapping_sub(1i32)) as u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 11i32) {
                    break 'l1;
                }
                'l2: {
                    ((dst).wrapping_offset((i) as isize)).write(
                        (((((((&raw mut sFloorTrainers).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 11))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerHillTrainerFrontSpriteId(trainerId: u16) -> u8 {
    unsafe {
        let mut trainerId = trainerId;
        let mut id: u8 = 0u8;
        let mut facilityClass: u8 = 0u8;
        SetUpDataStruct();
        id = ((((trainerId) as i32).wrapping_sub(1i32)) as u8);
        facilityClass = ((((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(12))
        .cast::<u8>())
        .wrapping_offset(
            (((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
                as isize
                * 952,
        ))
        .wrapping_add(4))
        .cast::<u8>())
        .wrapping_offset(((id) as i32) as isize * 328))
        .wrapping_add(11))
        .read();
        FreeDataStruct();
        return (((&raw mut gFacilityClassToPicIndex).cast::<u8>())
            .wrapping_offset(((facilityClass) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTrainerHillBattleStruct() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        SetUpDataStruct();
        ((&raw mut sFloorTrainers).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(24u32));
        {
            i = 0i32;
            'l1: loop {
                if !(i < 2i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 11i32) {
                                break 'l3;
                            }
                            'l4: {
                                (((((((&raw mut sFloorTrainers)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 11))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize))
                                .write(
                                    (((((((((((&raw mut sHillData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(12))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .read()) as i32)
                                            as isize
                                            * 952,
                                    ))
                                    .wrapping_add(4))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 328))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize))
                                    .read(),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    ((((((&raw mut sFloorTrainers).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(22))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(
                        ((((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read()).read())
                                as i32) as isize
                                * 952,
                        ))
                        .wrapping_add(4))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 328))
                        .wrapping_add(11))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        SetTrainerHillVBlankCounter(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .cast::<u32>(),
        );
        FreeDataStruct();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeTrainerHillBattleStruct() {
    unsafe {
        if ((((&raw mut sFloorTrainers).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize
        {
            Free(((&raw mut sFloorTrainers).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sFloorTrainers).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn SetUpDataStruct() {
    unsafe {
        if ((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize {
            ((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(3820u32));
            (((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read()).write(
                (((((((&raw mut gMapHeader).cast::<u8>())
                    .wrapping_add(18)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_sub(415i32)) as u8),
            );
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                ((((&raw const sChallengeData)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<*mut u8>())
                                .cast::<*mut u8>())
                                .wrapping_offset(
                                    ((crate::c::bf_read(
                                        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                            .wrapping_add(15716))
                                        .wrapping_add(10),
                                        6,
                                        2,
                                        false,
                                    ) as u16) as i32) as isize,
                                ))
                                .read(),
                                (((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(4),
                                (67108864u32
                                    | (crate::c::div_u32(
                                        3816u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
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
            TrainerHillDummy();
        }
    }
}
pub(crate) unsafe extern "C" fn FreeDataStruct() {
    unsafe {
        if ((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize {
            Free(((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyTrainerHillTrainerText(which: u8, localId: u16) {
    unsafe {
        let mut which = which;
        let mut localId = localId;
        let mut id: u8 = 0u8;
        let mut floorId: u8 = 0u8;
        SetUpDataStruct();
        floorId = GetFloorId();
        id = ((((localId) as i32).wrapping_sub(1i32)) as u8);
        'l1: {
            let __sw1 = ((which) as i32);
            if __sw1 == 2i32 {
                FrontierSpeechToString(
                    ((((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .cast::<u8>())
                    .wrapping_offset(((floorId) as i32) as isize * 952))
                    .wrapping_add(4))
                    .cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 328))
                    .wrapping_add(16))
                    .cast::<u16>(),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                FrontierSpeechToString(
                    ((((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .cast::<u8>())
                    .wrapping_offset(((floorId) as i32) as isize * 952))
                    .wrapping_add(4))
                    .cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 328))
                    .wrapping_add(28))
                    .cast::<u16>(),
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                FrontierSpeechToString(
                    ((((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .cast::<u8>())
                    .wrapping_offset(((floorId) as i32) as isize * 952))
                    .wrapping_add(4))
                    .cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 328))
                    .wrapping_add(40))
                    .cast::<u16>(),
                );
                break 'l1;
            }
            if __sw1 == 5i32 {
                FrontierSpeechToString(
                    ((((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .cast::<u8>())
                    .wrapping_offset(((floorId) as i32) as isize * 952))
                    .wrapping_add(4))
                    .cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 328))
                    .wrapping_add(52))
                    .cast::<u16>(),
                );
                break 'l1;
            }
        }
        FreeDataStruct();
    }
}
pub(crate) unsafe extern "C" fn TrainerHillStartChallenge() {
    unsafe {
        TrainerHillDummy();
        if !((ReadTrainerHillAndValidate()) != 0) {
            crate::c::bf_write(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                    .wrapping_add(10),
                5,
                1,
                (1u16) as i32,
            );
        } else {
            crate::c::bf_write(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                    .wrapping_add(10),
                5,
                1,
                (0u16) as i32,
            );
        }
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
            .wrapping_add(8))
        .write(0u8);
        SetTrainerHillVBlankCounter(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .cast::<u32>(),
        );
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
            .cast::<u32>())
        .write(0u32);
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .wrapping_add(10),
            2,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .wrapping_add(10),
            1,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .wrapping_add(10),
            4,
            1,
            (0u16) as i32,
        );
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2196))
        .write(0u8);
        ((&raw mut gBattleOutcome).cast::<u8>()).write(0u8);
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .wrapping_add(10),
            0,
            1,
            (0u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn GetOwnerState() {
    unsafe {
        ClearTrainerHillVBlankCounter();
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        if (crate::c::bf_read(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .wrapping_add(10),
            2,
            1,
            false,
        ) as u16)
            != 0
        {
            let __p1 = (&raw mut gSpecialVar_Result).cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .wrapping_add(10),
            0,
            1,
            false,
        ) as u16)
            != 0)
            && ((crate::c::bf_read(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                    .wrapping_add(10),
                1,
                1,
                false,
            ) as u16)
                != 0)
        {
            let __p2 = (&raw mut gSpecialVar_Result).cast::<u16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .wrapping_add(10),
            2,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn GiveChallengePrize() {
    unsafe {
        let mut itemId: u16 = GetPrizeItemId();
        if ((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .wrapping_add(2))
        .read()) as i32)
            != 4i32)
            || ((crate::c::bf_read(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                    .wrapping_add(10),
                0,
                1,
                false,
            ) as u16)
                != 0)
        {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(2u16);
        } else {
            if ((AddBagItem(itemId, 1u16)) as i32) == 1i32 {
                CopyItemName(itemId, (&raw mut gStringVar2).cast::<u8>());
                crate::c::bf_write(
                    ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                        .wrapping_add(10),
                    0,
                    1,
                    (1u16) as i32,
                );
                crate::c::bf_write(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(2221),
                    0,
                    7,
                    (0u8) as i32,
                );
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
            } else {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CheckFinalTime() {
    unsafe {
        if (crate::c::bf_read(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .wrapping_add(10),
            1,
            1,
            false,
        ) as u16)
            != 0
        {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(2u16);
        } else {
            if GetTimerValue(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                    .wrapping_add(4)
                    .cast::<u32>(),
            ) > (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .cast::<u32>())
            .read()
            {
                SetTimerValue(
                    ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                        .wrapping_add(4)
                        .cast::<u32>(),
                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                        .cast::<u32>())
                    .read(),
                );
                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(14104))
                    .cast::<u32>())
                .wrapping_offset(
                    ((crate::c::bf_read(
                        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(15716))
                        .wrapping_add(10),
                        6,
                        2,
                        false,
                    ) as u16) as i32) as isize,
                ))
                .write(
                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                        .wrapping_add(4)
                        .cast::<u32>())
                    .read(),
                );
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
            } else {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
            }
        }
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .wrapping_add(10),
            1,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn TrainerHillResumeTimer() {
    unsafe {
        if !((crate::c::bf_read(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .wrapping_add(10),
            2,
            1,
            false,
        ) as u16)
            != 0)
        {
            if (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .cast::<u32>())
            .read()
                >= 215999u32
            {
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                    .cast::<u32>())
                .write(215999u32);
            } else {
                SetTrainerHillVBlankCounter(
                    ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                        .cast::<u32>(),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TrainerHillSetPlayerLost() {
    unsafe {
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .wrapping_add(10),
            3,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn TrainerHillGetChallengeStatus() {
    unsafe {
        if (crate::c::bf_read(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .wrapping_add(10),
            3,
            1,
            false,
        ) as u16)
            != 0
        {
            crate::c::bf_write(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                    .wrapping_add(10),
                3,
                1,
                (0u16) as i32,
            );
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        } else {
            if (crate::c::bf_read(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                    .wrapping_add(10),
                4,
                1,
                false,
            ) as u16)
                != 0
            {
                crate::c::bf_write(
                    ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                        .wrapping_add(10),
                    4,
                    1,
                    (0u16) as i32,
                );
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
            } else {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(2u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BufferChallengeTime() {
    unsafe {
        let mut total: i32 = 0i32;
        let mut minutes: i32 = 0i32;
        let mut secondsWhole: i32 = 0i32;
        let mut secondsFraction: i32 = 0i32;
        total = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
            .cast::<u32>())
        .read()) as i32);
        if total >= 215999i32 {
            total = 215999i32;
        }
        minutes = crate::c::div_i32(total, 3600i32);
        total = crate::c::rem_i32(total, 3600i32);
        secondsWhole = crate::c::div_i32(total, 60i32);
        total = crate::c::rem_i32(total, 60i32);
        secondsFraction = crate::c::div_i32((total).wrapping_mul(168i32), 100i32);
        ConvertIntToDecimalStringN((&raw mut gStringVar1).cast::<u8>(), minutes, 1i32, 2u8);
        ConvertIntToDecimalStringN((&raw mut gStringVar2).cast::<u8>(), secondsWhole, 1i32, 2u8);
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar3).cast::<u8>(),
            secondsFraction,
            2i32,
            2u8,
        );
    }
}
pub(crate) unsafe extern "C" fn GetAllFloorsUsed() {
    unsafe {
        SetUpDataStruct();
        if (((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .wrapping_add(2))
        .read()) as i32)
            != 4i32
        {
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                (((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .wrapping_add(2))
                .read()) as i32),
                0i32,
                1u8,
            );
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        }
        FreeDataStruct();
    }
}
pub(crate) unsafe extern "C" fn GetInEReaderMode() {
    unsafe {
        SetUpDataStruct();
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        FreeDataStruct();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InTrainerHillChallenge() -> u8 {
    unsafe {
        if ((VarGet(16598u16)) as i32) == 0i32 {
            return 0u8;
        } else {
            if (crate::c::bf_read(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                    .wrapping_add(10),
                2,
                1,
                false,
            ) as u16)
                != 0
            {
                return 0u8;
            } else {
                if ((GetCurrentTrainerHillMapId()) as i32) != 0i32 {
                    return 1u8;
                } else {
                    return 0u8;
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn IsTrainerHillChallengeActive() {
    unsafe {
        if !((InTrainerHillChallenge()) != 0) {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        }
    }
}
pub(crate) unsafe extern "C" fn TrainerHillDummy_Unused() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn TrainerHillDummy() {
    unsafe {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintOnTrainerHillRecordsWindow() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        let mut total: u32 = 0u32;
        let mut minutes: u32 = 0u32;
        let mut secondsWhole: u32 = 0u32;
        let mut secondsFraction: u32 = 0u32;
        SetUpDataStruct();
        FillWindowPixelBuffer(0u8, 0u8);
        x = GetStringCenterAlignXOffset(1i32, (&raw mut gText_TimeBoard).cast::<u8>(), 208i32);
        AddTextPrinterParameterized3(
            0u8,
            1u8,
            ((x) as u8),
            2u8,
            ((&raw const sRecordWinColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            (&raw mut gText_TimeBoard).cast::<u8>(),
        );
        y = 18i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    AddTextPrinterParameterized3(
                        0u8,
                        1u8,
                        0u8,
                        ((y) as u8),
                        ((&raw const sRecordWinColors).cast::<u8>().cast_mut()).cast::<u8>(),
                        (-1i8),
                        ((((&raw const sModeStrings)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                    y = (y).wrapping_add(15i32);
                    total = GetTimerValue(
                        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(14104))
                        .cast::<u32>())
                        .wrapping_offset((i) as isize),
                    );
                    minutes = crate::c::div_u32(total, 3600u32);
                    total = crate::c::rem_u32(total, 3600u32);
                    ConvertIntToDecimalStringN(
                        (&raw mut gStringVar1).cast::<u8>(),
                        ((minutes) as i32),
                        1i32,
                        2u8,
                    );
                    secondsWhole = crate::c::div_u32(total, 60u32);
                    total = crate::c::rem_u32(total, 60u32);
                    ConvertIntToDecimalStringN(
                        (&raw mut gStringVar2).cast::<u8>(),
                        ((secondsWhole) as i32),
                        1i32,
                        2u8,
                    );
                    secondsFraction = crate::c::div_u32((total).wrapping_mul(168u32), 100u32);
                    ConvertIntToDecimalStringN(
                        (&raw mut gStringVar3).cast::<u8>(),
                        ((secondsFraction) as i32),
                        2i32,
                        2u8,
                    );
                    StringExpandPlaceholders(
                        StringCopy(
                            (&raw mut gStringVar4).cast::<u8>(),
                            (&raw mut gText_TimeCleared).cast::<u8>(),
                        ),
                        (&raw mut gText_XMinYDotZSec).cast::<u8>(),
                    );
                    x = GetStringRightAlignXOffset(
                        1i32,
                        (&raw mut gStringVar4).cast::<u8>(),
                        208i32,
                    );
                    AddTextPrinterParameterized3(
                        0u8,
                        1u8,
                        ((x) as u8),
                        ((y) as u8),
                        ((&raw const sRecordWinColors).cast::<u8>().cast_mut()).cast::<u8>(),
                        (-1i8),
                        (&raw mut gStringVar4).cast::<u8>(),
                    );
                    y = (y).wrapping_add(17i32);
                }
                i = (i).wrapping_add(1);
            }
        }
        PutWindowTilemap(0u8);
        CopyWindowToVram(0u8, 3u8);
        FreeDataStruct();
    }
}
pub(crate) unsafe extern "C" fn GetTimerValue(src: *mut u32) -> u32 {
    unsafe {
        let mut src = src;
        return (src).read();
    }
}
pub(crate) unsafe extern "C" fn SetTimerValue(dst: *mut u32, val: u32) {
    unsafe {
        let mut dst = dst;
        let mut val = val;
        (dst).write(val);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadTrainerHillObjectEventTemplates() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut floorId: u8 = 0u8;
        let mut eventTemplates: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(3184))
        .cast::<u8>();
        if !((LoadTrainerHillFloorObjectEventScripts()) != 0) {
            return;
        }
        SetUpDataStruct();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1640))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(65535u16);
                }
                i = (i).wrapping_add(1);
            }
        }
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
        floorId = GetFloorId();
        {
            i = 0u8;
            'l7: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l7;
                }
                'l8: {
                    let mut bits: u8 = 0u8;
                    (eventTemplates)
                        .wrapping_offset(((i) as i32) as isize * 24)
                        .cast::<crate::c::Rec4<24>>()
                        .write_unaligned(
                            (&raw const sTrainerObjectEventTemplate)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<crate::c::Rec4<24>>()
                                .read_unaligned(),
                        );
                    ((eventTemplates).wrapping_offset(((i) as i32) as isize * 24))
                        .write(((((i) as i32).wrapping_add(1i32)) as u8));
                    (((eventTemplates).wrapping_offset(((i) as i32) as isize * 24))
                        .wrapping_add(1))
                    .write(FacilityClassToGraphicsId(
                        ((((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(((floorId) as i32) as isize * 952))
                        .wrapping_add(4))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 328))
                        .wrapping_add(11))
                        .read(),
                    ));
                    (((eventTemplates).wrapping_offset(((i) as i32) as isize * 24))
                        .wrapping_add(4)
                        .cast::<i16>())
                    .write(
                        ((((((((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(((floorId) as i32) as isize * 952))
                        .wrapping_add(660))
                        .wrapping_add(288))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            & 15i32) as i16),
                    );
                    (((eventTemplates).wrapping_offset(((i) as i32) as isize * 24))
                        .wrapping_add(6)
                        .cast::<i16>())
                    .write(
                        ((((((((((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(((floorId) as i32) as isize * 952))
                        .wrapping_add(660))
                        .wrapping_add(288))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            >> 4)
                            & 15i32)
                            .wrapping_add(5i32)) as i16),
                    );
                    bits = ((((i) as i32) << 2) as u8);
                    (((eventTemplates).wrapping_offset(((i) as i32) as isize * 24))
                        .wrapping_add(9))
                    .write(
                        (((crate::c::shr_i32(
                            ((((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(12))
                            .cast::<u8>())
                            .wrapping_offset(((floorId) as i32) as isize * 952))
                            .wrapping_add(660))
                            .wrapping_add(290))
                            .read()) as i32),
                            ((bits) as u32),
                        ) & 15i32)
                            .wrapping_add(7i32)) as u8),
                    );
                    (((eventTemplates).wrapping_offset(((i) as i32) as isize * 24))
                        .wrapping_add(14)
                        .cast::<u16>())
                    .write(
                        ((crate::c::shr_i32(
                            ((((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(12))
                            .cast::<u8>())
                            .wrapping_offset(((floorId) as i32) as isize * 952))
                            .wrapping_add(660))
                            .wrapping_add(291))
                            .read()) as i32),
                            ((bits) as u32),
                        ) & 15i32) as u16),
                    );
                    (((eventTemplates).wrapping_offset(((i) as i32) as isize * 24))
                        .wrapping_add(16)
                        .cast::<*mut u8>())
                    .write((&raw mut TrainerHill_EventScript_TrainerBattle).cast::<u8>());
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1640))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(((((i) as i32).wrapping_add(1i32)) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
        FreeDataStruct();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadTrainerHillFloorObjectEventScripts() -> u32 {
    unsafe {
        SetUpDataStruct();
        FreeDataStruct();
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn GetMapDataForFloor(
    floorId: u8,
    x: u32,
    y: u32,
    floorWidth: u32,
) -> u16 {
    unsafe {
        let mut floorId = floorId;
        let mut x = x;
        let mut y = y;
        let mut floorWidth = floorWidth;
        let mut impassable: u8 = 0u8;
        let mut metatileId: u16 = 0u16;
        let mut elevation: u16 = 0u16;
        impassable = ((crate::c::shr_i32(
            ((((((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(((floorId) as i32) as isize * 952))
            .wrapping_add(660))
            .wrapping_add(256))
            .cast::<u16>())
            .wrapping_offset(((y) as i32) as isize))
            .read()) as i32),
            (15u32).wrapping_sub(x),
        ) & 1i32) as u8);
        metatileId = (((((((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12))
        .cast::<u8>())
        .wrapping_offset(((floorId) as i32) as isize * 952))
        .wrapping_add(660))
        .cast::<u8>())
        .wrapping_offset(((((floorWidth).wrapping_mul(y)).wrapping_add(x)) as i32) as isize))
        .read()) as i32)
            .wrapping_add(512i32)) as u16);
        elevation = 12288u16;
        return (((((((impassable) as i32) << 10) & 3072i32) | ((elevation) as i32))
            | ((((metatileId) as i32) << 0) & 1023i32)) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GenerateTrainerHillFloorLayout(mapArg: *mut u16) {
    unsafe {
        let mut mapArg = mapArg;
        let mut y: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut src: *mut u16 = core::ptr::null_mut();
        let mut dst: *mut u16 = core::ptr::null_mut();
        let mut mapId: u8 = GetCurrentTrainerHillMapId();
        if ((mapId) as i32) == 6i32 {
            InitMapFromSavedGame();
            return;
        }
        SetUpDataStruct();
        if ((mapId) as i32) == 5i32 {
            InitMapFromSavedGame();
            FreeDataStruct();
            return;
        }
        mapId = GetFloorId();
        src = (((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u16>())
        .read();
        (((&raw mut gBackupMapLayout).cast::<u8>())
            .wrapping_add(8)
            .cast::<*mut u16>())
        .write(mapArg);
        (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).write(31i32);
        (((&raw mut gBackupMapLayout).cast::<u8>())
            .wrapping_add(4)
            .cast::<i32>())
        .write(35i32);
        dst = (mapArg).wrapping_offset(224);
        {
            y = 0i32;
            'l1: loop {
                if !(y < 5i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        x = 0i32;
                        'l3: loop {
                            if !(x < 16i32) {
                                break 'l3;
                            }
                            'l4: {
                                ((dst).wrapping_offset((x) as isize))
                                    .write(((src).wrapping_offset((x) as isize)).read());
                            }
                            x = (x).wrapping_add(1);
                        }
                    }
                    dst = (dst).wrapping_offset(31);
                    src = (src).wrapping_offset(16);
                }
                y = (y).wrapping_add(1);
            }
        }
        {
            y = 0i32;
            'l5: loop {
                if !(y < 16i32) {
                    break 'l5;
                }
                'l6: {
                    {
                        x = 0i32;
                        'l7: loop {
                            if !(x < 16i32) {
                                break 'l7;
                            }
                            'l8: {
                                ((dst).wrapping_offset((x) as isize)).write(GetMapDataForFloor(
                                    mapId,
                                    ((x) as u32),
                                    ((y) as u32),
                                    16u32,
                                ));
                            }
                            x = (x).wrapping_add(1);
                        }
                    }
                    dst = (dst).wrapping_offset(31);
                }
                y = (y).wrapping_add(1);
            }
        }
        RunOnLoadMapScript();
        FreeDataStruct();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InTrainerHill() -> u32 {
    unsafe {
        let mut ret: u32 = 0u32;
        if ((((((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(18)
            .cast::<u16>())
        .read()) as i32)
            == 415i32)
            || ((((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as i32)
                == 416i32))
            || ((((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as i32)
                == 417i32))
            || ((((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as i32)
                == 418i32)
        {
            ret = 1u32;
        } else {
            ret = 0u32;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentTrainerHillMapId() -> u8 {
    unsafe {
        let mut mapId: u8 = 0u8;
        if (((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(18)
            .cast::<u16>())
        .read()) as i32)
            == 415i32
        {
            mapId = 1u8;
        } else {
            if (((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as i32)
                == 416i32
            {
                mapId = 2u8;
            } else {
                if (((((&raw mut gMapHeader).cast::<u8>())
                    .wrapping_add(18)
                    .cast::<u16>())
                .read()) as i32)
                    == 417i32
                {
                    mapId = 3u8;
                } else {
                    if (((((&raw mut gMapHeader).cast::<u8>())
                        .wrapping_add(18)
                        .cast::<u16>())
                    .read()) as i32)
                        == 418i32
                    {
                        mapId = 4u8;
                    } else {
                        if (((((&raw mut gMapHeader).cast::<u8>())
                            .wrapping_add(18)
                            .cast::<u16>())
                        .read()) as i32)
                            == 419i32
                        {
                            mapId = 5u8;
                        } else {
                            if (((((&raw mut gMapHeader).cast::<u8>())
                                .wrapping_add(18)
                                .cast::<u16>())
                            .read()) as i32)
                                == 414i32
                            {
                                mapId = 6u8;
                            } else {
                                mapId = 0u8;
                            }
                        }
                    }
                }
            }
        }
        return mapId;
    }
}
pub(crate) unsafe extern "C" fn OnTrainerHillRoof() -> u32 {
    unsafe {
        let mut onRoof: u32 = 0u32;
        if (((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(18)
            .cast::<u16>())
        .read()) as i32)
            == 419i32
        {
            onRoof = 1u32;
        } else {
            onRoof = 0u32;
        }
        return onRoof;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWarpDestinationTrainerHill4F() -> *mut u8 {
    unsafe {
        let mut header: *mut u8 = Overworld_GetMapHeaderByGroupAndId(26u16, 64u16);
        return (((((header).wrapping_add(4).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWarpDestinationTrainerHillFinalFloor(warpEventId: u8) -> *mut u8 {
    unsafe {
        let mut warpEventId = warpEventId;
        let mut numFloors: u8 = 0u8;
        let mut header: *mut u8 = core::ptr::null_mut();
        if ((warpEventId) as i32) == 1i32 {
            return ((((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<*mut u8>())
            .read())
            .wrapping_offset(8);
        }
        numFloors = GetNumFloorsInTrainerHillChallenge();
        if (((numFloors) as i32) == 0i32) || (((numFloors) as i32) > 4i32) {
            numFloors = 4u8;
        }
        header = Overworld_GetMapHeaderByGroupAndId(
            26u16,
            ((((((&raw const sNextFloorMapNum)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .wrapping_offset((((numFloors) as i32).wrapping_sub(1i32)) as isize))
            .read()) as u16),
        );
        return ((((header).wrapping_add(4).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LocalIdToHillTrainerId(localId: u8) -> u16 {
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
pub unsafe extern "C" fn GetHillTrainerFlag(objectEventId: u8) -> u8 {
    unsafe {
        let mut objectEventId = objectEventId;
        let mut trainerIndexStart: u32 = ((((GetFloorId()) as i32).wrapping_mul(2i32)) as u32);
        let mut bitId: u8 = ((((((((((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objectEventId) as i32) as isize * 36))
        .wrapping_add(8))
        .read()) as i32)
            .wrapping_sub(1i32)) as u32)
            .wrapping_add(trainerIndexStart)) as u8);
        return (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2196))
        .read()) as u32)
            & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                .wrapping_offset(((bitId) as i32) as isize))
            .read()) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetHillTrainerFlag() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut trainerIndexStart: u8 = ((((GetFloorId()) as i32).wrapping_mul(2i32)) as u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1640))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32)
                    {
                        let __p1 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(2196);
                        (__p1).write(
                            (((((__p1).read()) as u32)
                                | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(
                                        (((trainerIndexStart) as i32).wrapping_add(((i) as i32)))
                                            as isize,
                                    ))
                                .read()) as u8),
                        );
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 32768u32) != 0 {
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < 2i32) {
                        break 'l3;
                    }
                    'l4: {
                        if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1640))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            == ((((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read()) as i32)
                        {
                            let __p2 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(2196);
                            (__p2).write(
                                (((((__p2).read()) as u32)
                                    | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            (((trainerIndexStart) as i32)
                                                .wrapping_add(((i) as i32)))
                                                as isize,
                                        ))
                                    .read()) as u8),
                            );
                            break 'l3;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerHillTrainerScript() -> *mut u8 {
    unsafe {
        return (&raw mut TrainerHill_EventScript_TrainerBattle).cast::<u8>();
    }
}
pub(crate) unsafe extern "C" fn ShowTrainerHillPostBattleText() {
    unsafe {
        CopyTrainerHillTrainerText(
            5u8,
            ((&raw mut gSpecialVar_LastTalked).cast::<u16>()).read(),
        );
        ShowFieldMessageFromBuffer();
    }
}
pub(crate) unsafe extern "C" fn CreateNPCTrainerHillParty(trainerId: u16, firstMonId: u8) {
    unsafe {
        let mut trainerId = trainerId;
        let mut firstMonId = firstMonId;
        let mut trId: u8 = 0u8;
        let mut level: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut floorId: i32 = 0i32;
        let mut partySlot: i32 = 0i32;
        if (((trainerId) as i32) == 0i32) || (((trainerId) as i32) > 2i32) {
            return;
        }
        trId = ((((trainerId) as i32).wrapping_sub(1i32)) as u8);
        SetUpDataStruct();
        level = ((GetHighestLevelInPlayerParty()) as u8);
        floorId = ((GetFloorId()) as i32);
        {
            i = ((firstMonId) as i32);
            partySlot = 0i32;
            'l1: loop {
                if !(i < ((firstMonId) as i32).wrapping_add(crate::c::div_i32(6i32, 2i32))) {
                    break 'l1;
                }
                'l2: {
                    let mut id: u8 =
                        ((((((&raw const sTrainerPartySlots).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((trId) as i32) as isize * 3))
                        .cast::<u8>())
                        .wrapping_offset((partySlot) as isize))
                        .read();
                    let mut mon: *mut u8 =
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset((i) as isize * 100);
                    CreateBattleTowerMon(
                        mon,
                        (((((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset((floorId) as isize * 952))
                        .wrapping_add(4))
                        .cast::<u8>())
                        .wrapping_offset(((trId) as i32) as isize * 328))
                        .wrapping_add(64))
                        .cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 44),
                    );
                    SetTrainerHillMonLevel(mon, level);
                }
                i = (i).wrapping_add(1);
                partySlot = (partySlot).wrapping_add(1);
            }
        }
        FreeDataStruct();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FillHillTrainerParty() {
    unsafe {
        ZeroEnemyPartyMons();
        CreateNPCTrainerHillParty(
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
            0u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FillHillTrainersParties() {
    unsafe {
        ZeroEnemyPartyMons();
        CreateNPCTrainerHillParty(
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
            0u8,
        );
        CreateNPCTrainerHillParty(
            ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read(),
            ((crate::c::div_i32(6i32, 2i32)) as u8),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerHillAIFlags() -> u32 {
    unsafe {
        return 7u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerEncounterMusicIdInTrainerHill(trainerId: u16) -> u8 {
    unsafe {
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        let mut trId: u8 = 0u8;
        let mut facilityClass: u8 = 0u8;
        SetUpDataStruct();
        trId = ((((trainerId) as i32).wrapping_sub(1i32)) as u8);
        facilityClass = ((((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(12))
        .cast::<u8>())
        .wrapping_offset(
            (((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
                as isize
                * 952,
        ))
        .wrapping_add(4))
        .cast::<u8>())
        .wrapping_offset(((trId) as i32) as isize * 328))
        .wrapping_add(11))
        .read();
        FreeDataStruct();
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(216u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sTrainerClassesAndMusic).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset((i) as isize * 4))
                    .read()) as i32)
                        == (((((&raw mut gFacilityClassToTrainerClass).cast::<u8>())
                            .wrapping_offset(((facilityClass) as i32) as isize))
                        .read()) as i32)
                    {
                        return (((((&raw const sTrainerClassesAndMusic)
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
pub(crate) unsafe extern "C" fn SetTrainerHillMonLevel(mon: *mut u8, level: u8) {
    unsafe {
        let mut mon = mon;
        let mut level = level;
        let mut species: u16 = ((GetMonData3(mon, 11i32, core::ptr::null_mut())) as u16);
        let mut exp: u32 = (((((&raw mut gExperienceTables).cast::<u8>()).wrapping_offset(
            ((((((&raw mut gSpeciesInfo).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(19))
            .read()) as i32) as isize
                * 404,
        ))
        .cast::<u32>())
        .wrapping_offset(((level) as i32) as isize))
        .read();
        SetMonData(mon, 25i32, (&raw mut exp).cast::<u8>());
        SetMonData(mon, 56i32, &raw mut level);
        CalculateMonStats(mon);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNumFloorsInTrainerHillChallenge() -> u8 {
    unsafe {
        let mut floors: u8 = 0u8;
        SetUpDataStruct();
        floors = (((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .wrapping_add(2))
        .read();
        FreeDataStruct();
        return floors;
    }
}
pub(crate) unsafe extern "C" fn SetAllTrainerFlags() {
    unsafe {
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2196))
        .write(255u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryLoadTrainerHillEReaderPalette() {
    unsafe {
        if OnTrainerHillEReaderChallengeFloor() == 1u32 {
            LoadPalette(
                (((&raw const sEReader_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .cast::<u8>(),
                112u16,
                32u16,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn GetGameSaved() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
            ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(2221),
                7,
                1,
                false,
            ) as u8) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn SetGameSaved() {
    unsafe {
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2221),
            7,
            1,
            (1u8) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn ClearGameSaved() {
    unsafe {
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2221),
            7,
            1,
            (0u8) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OnTrainerHillEReaderChallengeFloor() -> u32 {
    unsafe {
        if (!((InTrainerHillChallenge()) != 0)) || (((GetCurrentTrainerHillMapId()) as i32) == 6i32)
        {
            return 0u32;
        }
        GetInEReaderMode();
        if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 0i32 {
            return 0u32;
        } else {
            return 1u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn GetChallengeWon() {
    unsafe {
        if (crate::c::bf_read(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .wrapping_add(10),
            3,
            1,
            false,
        ) as u16)
            != 0
        {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        }
    }
}
pub(crate) unsafe extern "C" fn TrainerHillSetMode() {
    unsafe {
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .wrapping_add(10),
            6,
            2,
            (((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32,
        );
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
            .wrapping_add(4)
            .cast::<u32>())
        .write(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(14104))
                .cast::<u32>())
            .wrapping_offset(
                ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) as isize,
            ))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetPrizeListId(allowTMs: u8) -> u8 {
    unsafe {
        let mut allowTMs = allowTMs;
        let mut prizeListId: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut modBy: u8 = 0u8;
        prizeListId = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    prizeListId = ((((prizeListId) as i32)
                        ^ (((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 952))
                        .read()) as i32)
                            & 31i32)) as u8);
                    prizeListId = ((((prizeListId) as i32)
                        ^ ((((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 952))
                        .wrapping_add(1))
                        .read()) as i32)
                            & 31i32)) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        if (allowTMs) != 0 {
            modBy = 10u8;
        } else {
            modBy = ((crate::c::div_i32(10i32, 2i32)) as u8);
        }
        prizeListId = ((crate::c::rem_i32(((prizeListId) as i32), ((modBy) as i32))) as u8);
        return prizeListId;
    }
}
pub(crate) unsafe extern "C" fn GetPrizeItemId() -> u16 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut prizeList: *mut u16 = core::ptr::null_mut();
        let mut trainerNumSum: i32 = 0i32;
        let mut prizeListSetId: i32 = 0i32;
        let mut minutes: i32 = 0i32;
        let mut id: i32 = 0i32;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    trainerNumSum = (trainerNumSum).wrapping_add(
                        ((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 952))
                        .read()) as i32),
                    );
                    trainerNumSum = (trainerNumSum).wrapping_add(
                        (((((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 952))
                        .wrapping_add(1))
                        .read()) as i32),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        prizeListSetId = crate::c::div_i32(trainerNumSum, 256i32);
        prizeListSetId =
            crate::c::rem_i32(prizeListSetId, ((crate::c::div_u32(8u32, 4u32)) as i32));
        if ((FlagGet(2148u16)) != 0)
            && (((((((&raw mut sHillData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .read()) as i32)
                == 8i32)
        {
            i = GetPrizeListId(1u8);
        } else {
            i = GetPrizeListId(0u8);
        }
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .wrapping_add(10),
            6,
            2,
            false,
        ) as u16) as i32)
            == 3i32
        {
            i = ((crate::c::rem_i32(((i) as i32).wrapping_add(1i32), 10i32)) as u8);
        }
        prizeList = ((((((&raw const sPrizeListSets)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut *mut u16>())
        .cast::<*mut *mut u16>())
        .wrapping_offset((prizeListSetId) as isize))
        .read())
        .wrapping_offset(((i) as i32) as isize))
        .read();
        minutes = crate::c::div_i32(
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .cast::<u32>())
            .read()) as i32),
            3600i32,
        );
        if minutes < 12i32 {
            id = 0i32;
        } else {
            if minutes < 13i32 {
                id = 1i32;
            } else {
                if minutes < 14i32 {
                    id = 2i32;
                } else {
                    if minutes < 16i32 {
                        id = 3i32;
                    } else {
                        if minutes < 18i32 {
                            id = 4i32;
                        } else {
                            id = 5i32;
                        }
                    }
                }
            }
        }
        return ((prizeList).wrapping_offset((id) as isize)).read();
    }
}
