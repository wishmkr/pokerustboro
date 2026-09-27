//! Translated from `src/battle_pike.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sLvl50_Mons1 sLvl50_Mons2 sLvl50_Mons3 sLvl50_Mons4 sLvl50Mons sLvlOpen_Mons1 sLvlOpen_Mons2 sLvlOpen_Mons3 sLvlOpen_Mons4 sLvlOpenMons sWildMons sNPCTable sNPCSpeeches sFrontierBrainStreakAppearances sBattlePikeFunctions sRoomTypeHints sNumMonsToHealBeforePikeQueen sStatusInflictionScreenFlashFuncs sWinStreakFlags
#[allow(unused_imports)]
use crate::data::battle_pike::*;

pub(crate) static mut sRoomType: u8 = 0u8;
pub(crate) static mut sStatusMon: u8 = 0u8;
pub(crate) static mut sInWildMonRoom: u8 = 0u8;
pub(crate) static mut sStatusFlags: u32 = 0u32;
pub(crate) static mut sNpcId: u8 = 0u8;

unsafe extern "C" {
    static mut gBattleFrontierTrainers: u8;
    static mut gBattleOutcome: u8;
    static mut gEnemyParty: u8;
    static mut gExperienceTables: u8;
    static mut gFacilityTrainers: u8;
    static mut gMapHeader: u8;
    static mut gPlayerParty: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_0x8007: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesInfo: u8;
    static mut gTasks: u8;
    static mut gTrainerBattleOpponent_A: u8;
    static mut gTrainerBattleOpponent_B: u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn CalculateMonStats(a0: *mut u8);
    fn CalculatePPWithBonus(a0: u16, a1: u8, a2: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn Free(a0: *mut u8);
    fn FrontierSpeechToString(a0: *mut u16);
    fn GetAilmentFromStatus(a0: u32) -> u8;
    fn GetHighestLevelInPlayerParty() -> i32;
    fn GetMonAbility(a0: *mut u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetPlayerSymbolCountForFacility(a0: u8) -> u8;
    fn GetRandomScaledFrontierTrainerId(a0: u8, a1: u8) -> u16;
    fn Random() -> u16;
    fn SaveMapView();
    fn ScriptContext_Enable();
    fn SetBattleFacilityTrainerGfxId(a0: u16, a1: u8);
    fn SetFrontierBrainObjEventGfx(a0: u8);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetMonMoveSlot(a0: *mut u8, a1: u16, a2: u8);
    fn TrySavingData(a0: u8) -> u8;
    fn VarSet(a0: u16, a1: u16) -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallBattlePikeFunction() {
    unsafe {
        (((((&raw const sBattlePikeFunctions)
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
pub(crate) unsafe extern "C" fn SetRoomType() {
    unsafe {
        let mut roomType: u8 = GetNextRoomType();
        ((&raw mut sRoomType).cast::<u8>().cast::<u8>()).write(roomType);
    }
}
pub(crate) unsafe extern "C" fn SetupRoomObjectEvents() {
    unsafe {
        let mut setObjGfx1: u32 = 0u32;
        let mut setObjGfx2: u32 = 0u32;
        let mut objGfx1: u32 = 0u32;
        let mut objGfx2: u16 = 0u16;
        VarSet(16400u16, 28u16);
        VarSet(16401u16, 226u16);
        setObjGfx1 = 1u32;
        setObjGfx2 = 0u32;
        objGfx1 = 0u32;
        objGfx2 = 0u16;
        'l1: {
            let __sw1 = ((((&raw mut sRoomType).cast::<u8>().cast::<u8>()).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32;
            if __sw1 == 0i32 {
                PrepareOneTrainer(0u8);
                setObjGfx1 = 0u32;
                break 'l1;
            }
            if __sw1 == 1i32 {
                objGfx1 = 28u32;
                break 'l1;
            }
            if __sw1 == 2i32 {
                objGfx1 = (((GetNPCRoomGraphicsId()) as u8) as u32);
                break 'l1;
            }
            if __sw1 == 3i32 {
                objGfx1 = 48u32;
                if ((((&raw mut sStatusMon).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
                    objGfx2 = 226u16;
                } else {
                    objGfx2 = 225u16;
                }
                setObjGfx2 = 1u32;
                break 'l1;
            }
            if __sw1 == 4i32 {
                objGfx1 = 48u32;
                break 'l1;
            }
            if __sw1 == 5i32 {
                setObjGfx1 = 0u32;
                break 'l1;
            }
            if __sw1 == 6i32 {
                PrepareOneTrainer(1u8);
                objGfx2 = 28u16;
                setObjGfx1 = 0u32;
                setObjGfx2 = 1u32;
                break 'l1;
            }
            if __sw1 == 7i32 {
                PrepareTwoTrainers();
                setObjGfx1 = 0u32;
                break 'l1;
            }
            if __sw1 == 8i32 {
                SetFrontierBrainObjEventGfx(5u8);
                objGfx2 = 28u16;
                setObjGfx1 = 0u32;
                setObjGfx2 = 1u32;
                break 'l1;
            }
            if !__matched {
                return;
            }
        }
        if setObjGfx1 == 1u32 {
            VarSet(16400u16, ((objGfx1) as u16));
        }
        if setObjGfx2 == 1u32 {
            VarSet(16401u16, objGfx2);
        }
    }
}
pub(crate) unsafe extern "C" fn GetBattlePikeData() {
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
                        .wrapping_add(1974)
                        .cast::<u16>())
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1976))
                    .cast::<u16>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1629),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize,
                    ))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1980))
                    .cast::<u16>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1629),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize,
                    ))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1984))
                    .cast::<u16>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1629),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize,
                    ))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                if lvlMode != 0u32 {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>())
                        .read()
                            & 2048u32) as u16),
                    );
                } else {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>())
                        .read()
                            & 1024u32) as u16),
                    );
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetBattlePikeData() {
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
                    .wrapping_add(1974)
                    .cast::<u16>())
                .write(((&raw mut gSpecialVar_0x8006).cast::<u16>()).read());
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) <= 9999i32 {
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1976))
                    .cast::<u16>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1629),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize,
                    ))
                    .write(((&raw mut gSpecialVar_0x8006).cast::<u16>()).read());
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) <= 9999i32)
                    && ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1980))
                    .cast::<u16>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1629),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize,
                    ))
                    .read()) as i32)
                        < ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32))
                {
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1980))
                    .cast::<u16>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1629),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize,
                    ))
                    .write(((&raw mut gSpecialVar_0x8006).cast::<u16>()).read());
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) <= 9999i32 {
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1984))
                    .cast::<u16>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1629),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize,
                    ))
                    .write(((&raw mut gSpecialVar_0x8006).cast::<u16>()).read());
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if lvlMode != 0u32 {
                    if (((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) != 0 {
                        let __p2 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>();
                        (__p2).write(((__p2).read() | 2048u32));
                    } else {
                        let __p3 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>();
                        (__p3).write(((__p3).read() & 4294965247u32));
                    }
                } else {
                    if (((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) != 0 {
                        let __p4 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>();
                        (__p4).write(((__p4).read() | 1024u32));
                    } else {
                        let __p5 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>();
                        (__p5).write(((__p5).read() & 4294966271u32));
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsNextRoomFinal() {
    unsafe {
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1638)
            .cast::<u16>())
        .read()) as i32)
            > 14i32
        {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn GetRoomType() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((((&raw mut sRoomType).cast::<u8>().cast::<u8>()).read()) as u16));
    }
}
pub(crate) unsafe extern "C" fn SetInWildMonRoom() {
    unsafe {
        ((&raw mut sInWildMonRoom).cast::<u8>().cast::<u8>()).write(1u8);
    }
}
pub(crate) unsafe extern "C" fn ClearInWildMonRoom() {
    unsafe {
        ((&raw mut sInWildMonRoom).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn SavePikeChallenge() {
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
pub(crate) unsafe extern "C" fn PikeDummy1() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn PikeDummy2() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn GetRoomInflictedStatus() {
    unsafe {
        'l1: {
            let __sw1 = ((&raw mut sStatusFlags).cast::<u8>().cast::<u32>()).read();
            if __sw1 == 32u32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                break 'l1;
            }
            if __sw1 == 16u32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                break 'l1;
            }
            if __sw1 == 128u32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(2u16);
                break 'l1;
            }
            if __sw1 == 64u32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(3u16);
                break 'l1;
            }
            if __sw1 == 7u32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(4u16);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetRoomInflictedStatusMon() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((((&raw mut sStatusMon).cast::<u8>().cast::<u8>()).read()) as u16));
    }
}
pub(crate) unsafe extern "C" fn HealOneOrTwoMons() {
    unsafe {
        let mut toHeal: u16 =
            (((crate::c::rem_i32(((Random()) as i32), 2i32)).wrapping_add(1i32)) as u16);
        TryHealMons(((toHeal) as u8));
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(toHeal);
    }
}
pub(crate) unsafe extern "C" fn BufferNPCMessage() {
    unsafe {
        let mut speechId: i32 = 0i32;
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1638)
            .cast::<u16>())
        .read()) as i32)
            <= 4i32
        {
            speechId = (((((((&raw const sNPCTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut sNpcId).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 8,
                ))
            .wrapping_add(2))
            .read()) as i32);
        } else {
            if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1638)
                .cast::<u16>())
            .read()) as i32)
                <= 10i32
            {
                speechId = (((((((&raw const sNPCTable).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut sNpcId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                            * 8,
                    ))
                .wrapping_add(3))
                .read()) as i32);
            } else {
                speechId = (((((((&raw const sNPCTable).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut sNpcId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                            * 8,
                    ))
                .wrapping_add(4))
                .read()) as i32);
            }
        }
        FrontierSpeechToString(
            ((((&raw const sNPCSpeeches).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((speechId) as isize * 12))
            .cast::<u16>(),
        );
    }
}
pub(crate) unsafe extern "C" fn StatusInflictionScreenFlash() {
    unsafe {
        CreateTask(Some(Task_DoStatusInflictionScreenFlash), 2u8);
    }
}
pub(crate) unsafe extern "C" fn HealMon(mon: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut i: u8 = 0u8;
        let mut hp: u16 = 0u16;
        let mut ppBonuses: u8 = 0u8;
        let mut data = crate::ffi::Align4([0u8; 4]);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut data).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        hp = ((GetMonData2(mon, 58i32)) as u16);
        ((&raw mut data).cast::<u8>()).write(((hp) as u8));
        (((&raw mut data).cast::<u8>()).wrapping_offset(1)).write(((((hp) as i32) >> 8) as u8));
        SetMonData(mon, 57i32, (&raw mut data).cast::<u8>());
        ppBonuses = ((GetMonData2(mon, 21i32)) as u8);
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l3;
                }
                'l4: {
                    let mut r#move: u16 =
                        ((GetMonData2(mon, (13i32).wrapping_add(((i) as i32)))) as u16);
                    ((&raw mut data).cast::<u8>())
                        .write(CalculatePPWithBonus(r#move, ppBonuses, i));
                    SetMonData(
                        mon,
                        (17i32).wrapping_add(((i) as i32)),
                        (&raw mut data).cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut data).cast::<u8>()).write(0u8);
        (((&raw mut data).cast::<u8>()).wrapping_offset(1)).write(0u8);
        (((&raw mut data).cast::<u8>()).wrapping_offset(2)).write(0u8);
        (((&raw mut data).cast::<u8>()).wrapping_offset(3)).write(0u8);
        SetMonData(mon, 55i32, (&raw mut data).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn DoesAbilityPreventStatus(mon: *mut u8, status: u32) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut status = status;
        let mut ability: u8 = GetMonAbility(mon);
        let mut ret: u8 = 0u8;
        'l1: {
            let __sw1 = status;
            if __sw1 == 32u32 {
                if ((ability) as i32) == 40i32 {
                    ret = 1u8;
                }
                break 'l1;
            }
            if __sw1 == 16u32 {
                if ((ability) as i32) == 41i32 {
                    ret = 1u8;
                }
                break 'l1;
            }
            if __sw1 == 64u32 {
                if ((ability) as i32) == 7i32 {
                    ret = 1u8;
                }
                break 'l1;
            }
            if __sw1 == 7u32 {
                if (((ability) as i32) == 15i32) || (((ability) as i32) == 72i32) {
                    ret = 1u8;
                }
                break 'l1;
            }
            if __sw1 == 128u32 {
                if ((ability) as i32) == 17i32 {
                    ret = 1u8;
                }
                break 'l1;
            }
        }
        return ret;
    }
}
pub(crate) unsafe extern "C" fn DoesTypePreventStatus(species: u16, status: u32) -> u8 {
    unsafe {
        let mut species = species;
        let mut status = status;
        let mut ret: u8 = 0u8;
        'l1: {
            let __sw1 = status;
            if __sw1 == 128u32 {
                if ((((((((((&raw mut gSpeciesInfo).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 28))
                .wrapping_add(6))
                .cast::<u8>())
                .read()) as i32)
                    == 8i32)
                    || ((((((((&raw mut gSpeciesInfo).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 28))
                    .wrapping_add(6))
                    .cast::<u8>())
                    .read()) as i32)
                        == 3i32))
                    || (((((((((&raw mut gSpeciesInfo).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 28))
                    .wrapping_add(6))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        == 8i32))
                    || (((((((((&raw mut gSpeciesInfo).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 28))
                    .wrapping_add(6))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        == 3i32)
                {
                    ret = 1u8;
                }
                break 'l1;
            }
            if __sw1 == 32u32 {
                if ((((((((&raw mut gSpeciesInfo).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 28))
                .wrapping_add(6))
                .cast::<u8>())
                .read()) as i32)
                    == 15i32)
                    || (((((((((&raw mut gSpeciesInfo).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 28))
                    .wrapping_add(6))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        == 15i32)
                {
                    ret = 1u8;
                }
                break 'l1;
            }
            if __sw1 == 64u32 {
                if ((((((((((&raw mut gSpeciesInfo).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 28))
                .wrapping_add(6))
                .cast::<u8>())
                .read()) as i32)
                    == 4i32)
                    || ((((((((&raw mut gSpeciesInfo).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 28))
                    .wrapping_add(6))
                    .cast::<u8>())
                    .read()) as i32)
                        == 13i32))
                    || (((((((((&raw mut gSpeciesInfo).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 28))
                    .wrapping_add(6))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        == 4i32))
                    || (((((((((&raw mut gSpeciesInfo).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 28))
                    .wrapping_add(6))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        == 13i32)
                {
                    ret = 1u8;
                }
                break 'l1;
            }
            if __sw1 == 16u32 {
                if ((((((((&raw mut gSpeciesInfo).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 28))
                .wrapping_add(6))
                .cast::<u8>())
                .read()) as i32)
                    == 10i32)
                    || (((((((((&raw mut gSpeciesInfo).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 28))
                    .wrapping_add(6))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        == 10i32)
                {
                    ret = 1u8;
                }
                break 'l1;
            }
            if __sw1 == 7u32 {
                break 'l1;
            }
        }
        return ret;
    }
}
pub(crate) unsafe extern "C" fn TryInflictRandomStatus() -> u8 {
    unsafe {
        let mut j: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut count: u8 = 0u8;
        let mut indices = crate::ffi::Align4([0u8; 3]);
        let mut status: u32 = 0u32;
        let mut species: u16 = 0u16;
        let mut statusChosen: u8 = 0u8;
        let mut mon: *mut u8 = core::ptr::null_mut();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut indices).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(i);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            j = 0u8;
            'l3: loop {
                if !(((j) as i32) < 10i32) {
                    break 'l3;
                }
                'l4: {
                    let mut temp: u8 = 0u8;
                    let mut id: u8 = 0u8;
                    i = ((crate::c::rem_i32(((Random()) as i32), 3i32)) as u8);
                    id = ((crate::c::rem_i32(((Random()) as i32), 3i32)) as u8);
                    {
                        temp = (((&raw mut indices).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read();
                        (((&raw mut indices).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                            .write(
                                (((&raw mut indices).cast::<u8>())
                                    .wrapping_offset(((id) as i32) as isize))
                                .read(),
                            );
                        (((&raw mut indices).cast::<u8>()).wrapping_offset(((id) as i32) as isize))
                            .write(temp);
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1638)
            .cast::<u16>())
        .read()) as i32)
            <= 4i32
        {
            count = 1u8;
        } else {
            if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1638)
                .cast::<u16>())
            .read()) as i32)
                <= 9i32
            {
                count = 2u8;
            } else {
                count = 3u8;
            }
        }
        status = 0u32;
        'l5: loop {
            'l6: {
                let mut rand: u8 = 0u8;
                statusChosen = 0u8;
                rand = ((crate::c::rem_i32(((Random()) as i32), 100i32)) as u8);
                if ((rand) as i32) < 35i32 {
                    ((&raw mut sStatusFlags).cast::<u8>().cast::<u32>()).write(128u32);
                } else {
                    if ((rand) as i32) < 60i32 {
                        ((&raw mut sStatusFlags).cast::<u8>().cast::<u32>()).write(32u32);
                    } else {
                        if ((rand) as i32) < 80i32 {
                            ((&raw mut sStatusFlags).cast::<u8>().cast::<u32>()).write(64u32);
                        } else {
                            if ((rand) as i32) < 90i32 {
                                ((&raw mut sStatusFlags).cast::<u8>().cast::<u32>()).write(7u32);
                            } else {
                                ((&raw mut sStatusFlags).cast::<u8>().cast::<u32>()).write(16u32);
                            }
                        }
                    }
                }
                if status != ((&raw mut sStatusFlags).cast::<u8>().cast::<u32>()).read() {
                    status = ((&raw mut sStatusFlags).cast::<u8>().cast::<u32>()).read();
                    j = 0u8;
                    {
                        i = 0u8;
                        'l7: loop {
                            if !(((i) as i32) < 3i32) {
                                break 'l7;
                            }
                            'l8: {
                                mon = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut indices).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 100,
                                );
                                if (((GetAilmentFromStatus(GetMonData2(mon, 55i32))) as i32)
                                    == 0i32)
                                    && (GetMonData2(mon, 57i32) != 0u32)
                                {
                                    j = (j).wrapping_add(1);
                                    species = ((GetMonData2(mon, 11i32)) as u16);
                                    if !((DoesTypePreventStatus(
                                        species,
                                        ((&raw mut sStatusFlags).cast::<u8>().cast::<u32>()).read(),
                                    )) != 0)
                                    {
                                        statusChosen = 1u8;
                                        break 'l7;
                                    }
                                }
                                if ((j) as i32) == ((count) as i32) {
                                    break 'l7;
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    if ((j) as i32) == 0i32 {
                        return 0u8;
                    }
                }
            }
            if !(!((statusChosen) != 0)) {
                break 'l5;
            }
        }
        'l9: {
            let __sw1 = ((&raw mut sStatusFlags).cast::<u8>().cast::<u32>()).read();
            let __matched = __sw1 == 32u32
                || __sw1 == 16u32
                || __sw1 == 64u32
                || __sw1 == 7u32
                || __sw1 == 128u32;
            if __sw1 == 32u32 {
                ((&raw mut sStatusMon).cast::<u8>().cast::<u8>()).write(1u8);
                break 'l9;
            }
            if __sw1 == 16u32 {
                if crate::c::rem_i32(((Random()) as i32), 2i32) != 0i32 {
                    ((&raw mut sStatusMon).cast::<u8>().cast::<u8>()).write(1u8);
                } else {
                    ((&raw mut sStatusMon).cast::<u8>().cast::<u8>()).write(0u8);
                }
                break 'l9;
            }
            if __sw1 == 64u32 || __sw1 == 7u32 || __sw1 == 128u32 || !__matched {
                ((&raw mut sStatusMon).cast::<u8>().cast::<u8>()).write(0u8);
                break 'l9;
            }
        }
        j = 0u8;
        {
            i = 0u8;
            'l10: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l10;
                }
                'l11: {
                    mon = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        (((((&raw mut indices).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32) as isize
                            * 100,
                    );
                    if (((GetAilmentFromStatus(GetMonData2(mon, 55i32))) as i32) == 0i32)
                        && (GetMonData2(mon, 57i32) != 0u32)
                    {
                        j = (j).wrapping_add(1);
                        species = ((GetMonData2(mon, 11i32)) as u16);
                        if (!((DoesAbilityPreventStatus(
                            mon,
                            ((&raw mut sStatusFlags).cast::<u8>().cast::<u32>()).read(),
                        )) != 0))
                            && (!((DoesTypePreventStatus(
                                species,
                                ((&raw mut sStatusFlags).cast::<u8>().cast::<u32>()).read(),
                            )) != 0))
                        {
                            SetMonData(
                                mon,
                                55i32,
                                ((&raw mut sStatusFlags).cast::<u8>().cast::<u32>()).cast::<u8>(),
                            );
                        }
                    }
                    if ((j) as i32) == ((count) as i32) {
                        break 'l10;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn AtLeastOneHealthyMon() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut healthyMonsCount: u8 = 0u8;
        let mut count: u8 = 0u8;
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1638)
            .cast::<u16>())
        .read()) as i32)
            <= 4i32
        {
            count = 1u8;
        } else {
            if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1638)
                .cast::<u16>())
            .read()) as i32)
                <= 9i32
            {
                count = 2u8;
            } else {
                count = 3u8;
            }
        }
        healthyMonsCount = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 100);
                    if (((GetAilmentFromStatus(GetMonData2(mon, 55i32))) as i32) == 0i32)
                        && (GetMonData2(mon, 57i32) != 0u32)
                    {
                        healthyMonsCount = (healthyMonsCount).wrapping_add(1);
                    }
                    if ((healthyMonsCount) as i32) == ((count) as i32) {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((healthyMonsCount) as i32) == 0i32 {
            return 0u8;
        } else {
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetNextRoomType() -> u8 {
    unsafe {
        let mut roomTypesDisabled = crate::ffi::Align4([0u8; 8]);
        let mut i: u8 = 0u8;
        let mut nextRoomType: u8 = 0u8;
        let mut roomHint: u8 = 0u8;
        let mut numRoomCandidates: u8 = 0u8;
        let mut roomCandidates: *mut u8 = core::ptr::null_mut();
        let mut id: u8 = 0u8;
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1988),
            3,
            4,
            false,
        ) as u8) as i32)
            == 8i32
        {
            return (crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1988),
                3,
                4,
                false,
            ) as u8);
        }
        if ((((&raw mut gSpecialVar_0x8007).cast::<u16>()).read()) as i32)
            == ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1988),
                0,
                3,
                false,
            ) as u8) as i32)
        {
            if ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1988),
                3,
                4,
                false,
            ) as u8) as i32)
                == 3i32
            {
                TryInflictRandomStatus();
            }
            return (crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1988),
                3,
                4,
                false,
            ) as u8);
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut roomTypesDisabled).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        numRoomCandidates = 8u8;
        roomHint = ((((&raw const sRoomTypeHints).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1988),
                    3,
                    4,
                    false,
                ) as u8) as i32) as isize,
            ))
        .read();
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 1u32)) {
                    break 'l3;
                }
                'l4: {
                    if ((((((&raw const sRoomTypeHints).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((roomHint) as i32)
                    {
                        (((&raw mut roomTypesDisabled).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(1u8);
                        numRoomCandidates = (numRoomCandidates).wrapping_sub(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((((&raw mut roomTypesDisabled).cast::<u8>()).wrapping_offset(7)).read()) as i32)
            != 1i32)
            && (!((AtLeastTwoAliveMons()) != 0))
        {
            (((&raw mut roomTypesDisabled).cast::<u8>()).wrapping_offset(7)).write(1u8);
            numRoomCandidates = (numRoomCandidates).wrapping_sub(1);
        }
        if ((((((&raw mut roomTypesDisabled).cast::<u8>()).wrapping_offset(3)).read()) as i32)
            != 1i32)
            && (!((AtLeastOneHealthyMon()) != 0))
        {
            (((&raw mut roomTypesDisabled).cast::<u8>()).wrapping_offset(3)).write(1u8);
            numRoomCandidates = (numRoomCandidates).wrapping_sub(1);
        }
        if (crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1988),
            7,
            1,
            false,
        ) as u8)
            != 0
        {
            if (((((&raw mut roomTypesDisabled).cast::<u8>()).wrapping_offset(1)).read()) as i32)
                != 1i32
            {
                (((&raw mut roomTypesDisabled).cast::<u8>()).wrapping_offset(1)).write(1u8);
                numRoomCandidates = (numRoomCandidates).wrapping_sub(1);
            }
            if (((((&raw mut roomTypesDisabled).cast::<u8>()).wrapping_offset(4)).read()) as i32)
                != 1i32
            {
                (((&raw mut roomTypesDisabled).cast::<u8>()).wrapping_offset(4)).write(1u8);
                numRoomCandidates = (numRoomCandidates).wrapping_sub(1);
            }
        }
        roomCandidates = AllocZeroed(((numRoomCandidates) as u32));
        id = 0u8;
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 1u32)) {
                    break 'l5;
                }
                'l6: {
                    if (((((&raw mut roomTypesDisabled).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        ((roomCandidates).wrapping_offset(
                            (({
                                let __t1 = id;
                                id = (id).wrapping_add(1);
                                __t1
                            }) as i32) as isize,
                        ))
                        .write(i);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        nextRoomType = ((roomCandidates).wrapping_offset(
            (crate::c::rem_i32(((Random()) as i32), ((numRoomCandidates) as i32))) as isize,
        ))
        .read();
        Free(roomCandidates);
        if ((nextRoomType) as i32) == 3i32 {
            TryInflictRandomStatus();
        }
        return nextRoomType;
    }
}
pub(crate) unsafe extern "C" fn GetNPCRoomGraphicsId() -> u16 {
    unsafe {
        ((&raw mut sNpcId).cast::<u8>().cast::<u8>()).write(
            ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(200u32, 8u32))) as u8),
        );
        return (((((&raw const sNPCTable).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sNpcId).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 8,
            ))
        .cast::<u16>())
        .read();
    }
}
pub(crate) unsafe extern "C" fn GetInWildMonRoom() -> u8 {
    unsafe {
        return ((&raw mut sInWildMonRoom).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryGenerateBattlePikeWildMon(checkKeenEyeIntimidate: u8) -> u32 {
    unsafe {
        let mut checkKeenEyeIntimidate = checkKeenEyeIntimidate;
        let mut i: i32 = 0i32;
        let mut monLevel: i32 = 0i32;
        let mut headerId: u8 = GetBattlePikeWildMonHeaderId();
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        let mut wildMons: *mut *mut u8 = ((((&raw const sWildMons)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut *mut u8>())
        .cast::<*mut *mut u8>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        let mut abilityNum: u32 = 0u32;
        let mut pikeMonId: i32 = ((GetMonData3(
            (&raw mut gEnemyParty).cast::<u8>(),
            11i32,
            core::ptr::null_mut(),
        )) as i32);
        pikeMonId = ((SpeciesToPikeMonId(((pikeMonId) as u16))) as i32);
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32)
            != 0i32
        {
            monLevel = GetHighestLevelInPlayerParty();
            if monLevel < 60i32 {
                monLevel = 60i32;
            } else {
                monLevel = (monLevel).wrapping_sub(
                    (((((((wildMons).wrapping_offset(((headerId) as i32) as isize)).read())
                        .wrapping_offset((pikeMonId) as isize * 12))
                    .wrapping_add(2))
                    .read()) as i32),
                );
                if monLevel < 60i32 {
                    monLevel = 60i32;
                }
            }
        } else {
            monLevel = (50i32).wrapping_sub(
                (((((((wildMons).wrapping_offset(((headerId) as i32) as isize)).read())
                    .wrapping_offset((pikeMonId) as isize * 12))
                .wrapping_add(2))
                .read()) as i32),
            );
        }
        if (((checkKeenEyeIntimidate) as i32) == 1i32)
            && (!((CanEncounterWildMon(((monLevel) as u8))) != 0))
        {
            return 0u32;
        }
        SetMonData(
            (&raw mut gEnemyParty).cast::<u8>(),
            25i32,
            (((((&raw mut gExperienceTables).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                    (((((((wildMons).wrapping_offset(((headerId) as i32) as isize)).read())
                        .wrapping_offset((pikeMonId) as isize * 12))
                    .cast::<u16>())
                    .read()) as i32) as isize
                        * 28,
                ))
                .wrapping_add(19))
                .read()) as i32) as isize
                    * 404,
            ))
            .cast::<u32>())
            .wrapping_offset((monLevel) as isize))
            .cast::<u8>(),
        );
        if (((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
            (((((((wildMons).wrapping_offset(((headerId) as i32) as isize)).read())
                .wrapping_offset((pikeMonId) as isize * 12))
            .cast::<u16>())
            .read()) as i32) as isize
                * 28,
        ))
        .wrapping_add(22))
        .cast::<u8>())
        .wrapping_offset(1))
        .read())
            != 0
        {
            abilityNum = ((crate::c::rem_i32(((Random()) as i32), 2i32)) as u32);
        } else {
            abilityNum = 0u32;
        }
        SetMonData(
            (&raw mut gEnemyParty).cast::<u8>(),
            46i32,
            (&raw mut abilityNum).cast::<u8>(),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    SetMonMoveSlot(
                        (&raw mut gEnemyParty).cast::<u8>(),
                        (((((((wildMons).wrapping_offset(((headerId) as i32) as isize)).read())
                            .wrapping_offset((pikeMonId) as isize * 12))
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
        CalculateMonStats((&raw mut gEnemyParty).cast::<u8>());
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlePikeWildMonHeaderId() -> u8 {
    unsafe {
        let mut headerId: u8 = 0u8;
        let mut lvlMode: u8 = (crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8);
        let mut winStreak: u16 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1976))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        if ((winStreak) as i32) <= 280i32 {
            headerId = 0u8;
        } else {
            if ((winStreak) as i32) <= 560i32 {
                headerId = 1u8;
            } else {
                if ((winStreak) as i32) <= 840i32 {
                    headerId = 2u8;
                } else {
                    headerId = 3u8;
                }
            }
        }
        return headerId;
    }
}
pub(crate) unsafe extern "C" fn DoStatusInflictionScreenFlash(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sStatusInflictionScreenFlashFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StatusInflictionFadeOut(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32) == 0i32)
            || ((({
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32)
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read());
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
                )) as i16),
            );
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                > 16i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(16i16);
            }
            BlendPalettes(
                4294967295u32,
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as u8),
                11627u16,
            );
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32) >= 16i32
        {
            let __p4 = ((task).wrapping_add(8)).cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read());
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StatusInflictionFadeIn(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32) == 0i32)
            || ((({
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32)
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read());
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_sub(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
                )) as i16),
            );
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                < 0i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            }
            BlendPalettes(
                4294967295u32,
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as u8),
                11627u16,
            );
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32) == 0i32 {
            if (({
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                let __t5 = ((__p4).read()).wrapping_sub(1);
                (__p4).write(__t5);
                __t5
            }) as i32)
                == 0i32
            {
                DestroyTask(FindTaskIdByFunc(Some(DoStatusInflictionScreenFlash)));
            } else {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read());
                (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StartStatusInflictionScreenFlash(
    fadeOutDelay: i16,
    fadeInDelay: i16,
    numFades: i16,
    fadeOutSpeed: i16,
    fadeInSpped: i16,
) {
    unsafe {
        let mut fadeOutDelay = fadeOutDelay;
        let mut fadeInDelay = fadeInDelay;
        let mut numFades = numFades;
        let mut fadeOutSpeed = fadeOutSpeed;
        let mut fadeInSpped = fadeInSpped;
        let mut taskId: u8 = CreateTask(Some(DoStatusInflictionScreenFlash), 3u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(fadeOutDelay);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(fadeInDelay);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(numFades);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(fadeOutSpeed);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(fadeInSpped);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(fadeOutDelay);
    }
}
pub(crate) unsafe extern "C" fn IsStatusInflictionScreenFlashTaskFinished() -> u8 {
    unsafe {
        if ((FindTaskIdByFunc(Some(DoStatusInflictionScreenFlash))) as i32) == 255i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DoStatusInflictionScreenFlash(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            StartStatusInflictionScreenFlash(0i16, 0i16, 3i16, 2i16, 2i16);
        } else {
            if (IsStatusInflictionScreenFlashTaskFinished()) != 0 {
                ScriptContext_Enable();
                DestroyTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryHealMons(healCount: u8) {
    unsafe {
        let mut healCount = healCount;
        let mut j: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut k: u8 = 0u8;
        let mut indices = crate::ffi::Align4([0u8; 3]);
        if ((healCount) as i32) == 0i32 {
            return;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut indices).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(i);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            k = 0u8;
            'l3: loop {
                if !(((k) as i32) < 10i32) {
                    break 'l3;
                }
                'l4: {
                    let mut temp: u8 = 0u8;
                    i = ((crate::c::rem_i32(((Random()) as i32), 3i32)) as u8);
                    j = ((crate::c::rem_i32(((Random()) as i32), 3i32)) as u8);
                    {
                        temp = (((&raw mut indices).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read();
                        (((&raw mut indices).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                            .write(
                                (((&raw mut indices).cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize))
                                .read(),
                            );
                        (((&raw mut indices).cast::<u8>()).wrapping_offset(((j) as i32) as isize))
                            .write(temp);
                    }
                }
                k = (k).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l5;
                }
                'l6: {
                    let mut canBeHealed: u32 = 0u32;
                    let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        (((((&raw mut indices).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32) as isize
                            * 100,
                    );
                    let mut curr: u16 = ((GetMonData2(mon, 57i32)) as u16);
                    let mut max: u16 = ((GetMonData2(mon, 58i32)) as u16);
                    if ((curr) as i32) < ((max) as i32) {
                        canBeHealed = 1u32;
                    } else {
                        if ((GetAilmentFromStatus(GetMonData2(mon, 55i32))) as i32) != 0i32 {
                            canBeHealed = 1u32;
                        } else {
                            let mut ppBonuses: u8 = ((GetMonData2(mon, 21i32)) as u8);
                            {
                                j = 0u8;
                                'l7: loop {
                                    if !(((j) as i32) < 4i32) {
                                        break 'l7;
                                    }
                                    'l8: {
                                        let mut r#move: u16 =
                                            ((GetMonData2(mon, (13i32).wrapping_add(((j) as i32))))
                                                as u16);
                                        max = ((CalculatePPWithBonus(r#move, ppBonuses, j)) as u16);
                                        curr =
                                            ((GetMonData2(mon, (17i32).wrapping_add(((j) as i32))))
                                                as u16);
                                        if ((curr) as i32) < ((max) as i32) {
                                            canBeHealed = 1u32;
                                            break 'l7;
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                        }
                    }
                    if canBeHealed == 1u32 {
                        HealMon(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                (((((&raw mut indices).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                        );
                        if (({
                            let __t1 = (healCount).wrapping_sub(1);
                            healCount = __t1;
                            __t1
                        }) as i32)
                            == 0i32
                        {
                            break 'l5;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetInBattlePike() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((InBattlePike()) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InBattlePike() -> u8 {
    unsafe {
        return ((((((((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(18)
            .cast::<u16>())
        .read()) as i32)
            == 351i32)
            || ((((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as i32)
                == 352i32))
            || ((((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as i32)
                == 358i32))
            || ((((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as i32)
                == 359i32)) as u8);
    }
}
pub(crate) unsafe extern "C" fn SetHintedRoom() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut count: u8 = 0u8;
        let mut id: u8 = 0u8;
        let mut roomCandidates: *mut u8 = core::ptr::null_mut();
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        if (GetPikeQueenFightType(1u8)) != 0 {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
            crate::c::bf_write(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1988),
                0,
                3,
                ((crate::c::rem_i32(((Random()) as i32), 6i32)) as u8) as i32,
            );
            crate::c::bf_write(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1988),
                3,
                4,
                (8u8) as i32,
            );
        } else {
            crate::c::bf_write(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1988),
                0,
                3,
                ((crate::c::rem_i32(((Random()) as i32), 3i32)) as u8) as i32,
            );
            if (crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1988),
                7,
                1,
                false,
            ) as u8)
                != 0
            {
                count = 6u8;
            } else {
                count = 8u8;
            }
            roomCandidates = AllocZeroed(((count) as u32));
            {
                i = 0u8;
                id = 0u8;
                'l1: loop {
                    if !(((i) as i32) < ((count) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        if (crate::c::bf_read(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1988),
                            7,
                            1,
                            false,
                        ) as u8)
                            != 0
                        {
                            if (((i) as i32) != 1i32) && (((i) as i32) != 4i32) {
                                ((roomCandidates).wrapping_offset(
                                    (({
                                        let __t1 = id;
                                        id = (id).wrapping_add(1);
                                        __t1
                                    }) as i32) as isize,
                                ))
                                .write(i);
                            }
                        } else {
                            ((roomCandidates).wrapping_offset(((i) as i32) as isize)).write(i);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            crate::c::bf_write(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1988),
                3,
                4,
                (((roomCandidates).wrapping_offset(
                    (crate::c::rem_i32(((Random()) as i32), ((count) as i32))) as isize,
                ))
                .read()) as i32,
            );
            Free(roomCandidates);
            if (((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1988),
                3,
                4,
                false,
            ) as u8) as i32)
                == 3i32)
                && (!((AtLeastOneHealthyMon()) != 0))
            {
                crate::c::bf_write(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1988),
                    3,
                    4,
                    (2u8) as i32,
                );
            }
            if (((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1988),
                3,
                4,
                false,
            ) as u8) as i32)
                == 7i32)
                && (!((AtLeastTwoAliveMons()) != 0))
            {
                crate::c::bf_write(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1988),
                    3,
                    4,
                    (2u8) as i32,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetHintedRoomIndex() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
            ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1988),
                0,
                3,
                false,
            ) as u8) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn GetRoomTypeHint() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
            ((((((&raw const sRoomTypeHints).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                            .wrapping_add(1988),
                        3,
                        4,
                        false,
                    ) as u8) as i32) as isize,
                ))
            .read()) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn PrepareOneTrainer(difficult: u8) {
    unsafe {
        let mut difficult = difficult;
        let mut i: i32 = 0i32;
        let mut lvlMode: u8 = 0u8;
        let mut battleNum: u8 = 0u8;
        let mut challengeNum: u16 = 0u16;
        let mut trainerId: u16 = 0u16;
        if !((difficult) != 0) {
            battleNum = 1u8;
        } else {
            battleNum = 6u8;
        }
        lvlMode = (crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8);
        challengeNum = ((crate::c::div_i32(
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1976))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .read()) as i32),
            14i32,
        )) as u16);
        'l1: loop {
            'l2: {
                trainerId = GetRandomScaledFrontierTrainerId(((challengeNum) as u8), battleNum);
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i
                            < (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1638)
                            .cast::<u16>())
                            .read()) as i32)
                                .wrapping_sub(1i32))
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
                .read()) as i32)
                    .wrapping_sub(1i32))
            {
                break 'l1;
            }
        }
        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(trainerId);
        ((&raw mut gFacilityTrainers).cast::<*mut u8>())
            .write((&raw mut gBattleFrontierTrainers).cast::<u8>());
        SetBattleFacilityTrainerGfxId(
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
            0u8,
        );
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1638)
            .cast::<u16>())
        .read()) as i32)
            < 14i32
        {
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1640))
            .cast::<u16>())
            .wrapping_offset(
                ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_sub(1i32)) as isize,
            ))
            .write(((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read());
        }
    }
}
pub(crate) unsafe extern "C" fn PrepareTwoTrainers() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut trainerId: u16 = 0u16;
        let mut lvlMode: u8 = (crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8);
        let mut challengeNum: u16 = ((crate::c::div_i32(
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1976))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .read()) as i32),
            14i32,
        )) as u16);
        ((&raw mut gFacilityTrainers).cast::<*mut u8>())
            .write((&raw mut gBattleFrontierTrainers).cast::<u8>());
        'l1: loop {
            'l2: {
                trainerId = GetRandomScaledFrontierTrainerId(((challengeNum) as u8), 1u8);
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i
                            < (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1638)
                            .cast::<u16>())
                            .read()) as i32)
                                .wrapping_sub(1i32))
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
                .read()) as i32)
                    .wrapping_sub(1i32))
            {
                break 'l1;
            }
        }
        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(trainerId);
        SetBattleFacilityTrainerGfxId(
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
            0u8,
        );
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1638)
            .cast::<u16>())
        .read()) as i32)
            <= 14i32
        {
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1640))
            .cast::<u16>())
            .wrapping_offset(
                ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_sub(1i32)) as isize,
            ))
            .write(((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read());
        }
        'l5: loop {
            'l6: {
                trainerId = GetRandomScaledFrontierTrainerId(((challengeNum) as u8), 1u8);
                {
                    i = 0i32;
                    'l7: loop {
                        if !(i
                            < (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1638)
                            .cast::<u16>())
                            .read()) as i32))
                        {
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
            if !(i
                != (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                .read()) as i32))
            {
                break 'l5;
            }
        }
        ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).write(trainerId);
        SetBattleFacilityTrainerGfxId(
            ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read(),
            1u8,
        );
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1638)
            .cast::<u16>())
        .read()) as i32)
            < 14i32
        {
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1640))
            .cast::<u16>())
            .wrapping_offset(
                ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_sub(2i32)) as isize,
            ))
            .write(((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read());
        }
    }
}
pub(crate) unsafe extern "C" fn ClearPikeTrainerIds() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 14i32) {
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
    }
}
pub(crate) unsafe extern "C" fn BufferTrainerIntro() {
    unsafe {
        if ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) == 0i32 {
            if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) < 300i32 {
                FrontierSpeechToString(
                    (((((&raw mut gFacilityTrainers).cast::<*mut u8>()).read()).wrapping_offset(
                        ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32)
                            as isize
                            * 52,
                    ))
                    .wrapping_add(12))
                    .cast::<u16>(),
                );
            }
        } else {
            if ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) == 1i32 {
                if ((((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read()) as i32) < 300i32 {
                    FrontierSpeechToString(
                        (((((&raw mut gFacilityTrainers).cast::<*mut u8>()).read())
                            .wrapping_offset(
                                ((((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read())
                                    as i32) as isize
                                    * 52,
                            ))
                        .wrapping_add(12))
                        .cast::<u16>(),
                    );
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AtLeastTwoAliveMons() -> u8 {
    unsafe {
        let mut mon: *mut u8 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        let mut countDead: u8 = 0u8;
        mon = (&raw mut gPlayerParty).cast::<u8>();
        countDead = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if GetMonData2(mon, 57i32) == 0u32 {
                        countDead = (countDead).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
                mon = (mon).wrapping_offset(100);
            }
        }
        if ((countDead) as i32) >= 2i32 {
            return 0u8;
        } else {
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetPikeQueenFightType(nextRoom: u8) -> u8 {
    unsafe {
        let mut nextRoom = nextRoom;
        let mut numPikeSymbols: u8 = 0u8;
        let mut facility: u8 = 5u8;
        let mut ret: u8 = 0u8;
        let mut lvlMode: u8 = (crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8);
        let mut winStreak: u16 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1976))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        winStreak = ((((winStreak) as i32).wrapping_add(((nextRoom) as i32))) as u16);
        numPikeSymbols = GetPlayerSymbolCountForFacility(5u8);
        'l1: {
            let __sw1 = ((numPikeSymbols) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 || __sw1 == 1i32 {
                if ((winStreak) as i32)
                    == ((((((((&raw const sFrontierBrainStreakAppearances)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((facility) as i32) as isize * 4))
                    .cast::<u8>())
                    .wrapping_offset(((numPikeSymbols) as i32) as isize))
                    .read()) as i32)
                        .wrapping_sub(
                            ((((((((&raw const sFrontierBrainStreakAppearances)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((facility) as i32) as isize * 4))
                            .cast::<u8>())
                            .wrapping_offset(3))
                            .read()) as i32),
                        )
                {
                    ret = ((((numPikeSymbols) as i32).wrapping_add(1i32)) as u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 || !__matched {
                if ((winStreak) as i32)
                    == (((((((&raw const sFrontierBrainStreakAppearances)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((facility) as i32) as isize * 4))
                    .cast::<u8>())
                    .read()) as i32)
                        .wrapping_sub(
                            ((((((((&raw const sFrontierBrainStreakAppearances)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((facility) as i32) as isize * 4))
                            .cast::<u8>())
                            .wrapping_offset(3))
                            .read()) as i32),
                        )
                {
                    ret = 3u8;
                } else {
                    if (((winStreak) as i32)
                        == ((((((((&raw const sFrontierBrainStreakAppearances)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((facility) as i32) as isize * 4))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            .wrapping_sub(
                                ((((((((&raw const sFrontierBrainStreakAppearances)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((facility) as i32) as isize * 4))
                                .cast::<u8>())
                                .wrapping_offset(3))
                                .read()) as i32),
                            ))
                        || ((((winStreak) as i32)
                            > ((((((((&raw const sFrontierBrainStreakAppearances)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((facility) as i32) as isize * 4))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32))
                            && (crate::c::rem_i32(
                                (((winStreak) as i32).wrapping_sub(
                                    ((((((((&raw const sFrontierBrainStreakAppearances)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((facility) as i32) as isize * 4))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read()) as i32),
                                ))
                                .wrapping_add(
                                    ((((((((&raw const sFrontierBrainStreakAppearances)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((facility) as i32) as isize * 4))
                                    .cast::<u8>())
                                    .wrapping_offset(3))
                                    .read()) as i32),
                                ),
                                ((((((((&raw const sFrontierBrainStreakAppearances)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((facility) as i32) as isize * 4))
                                .cast::<u8>())
                                .wrapping_offset(2))
                                .read()) as i32),
                            ) == 0i32))
                    {
                        ret = 4u8;
                    }
                }
                break 'l1;
            }
        }
        return ret;
    }
}
pub(crate) unsafe extern "C" fn GetCurrentRoomPikeQueenFightType() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((GetPikeQueenFightType(0u8)) as u16));
    }
}
pub(crate) unsafe extern "C" fn HealSomeMonsBeforePikeQueen() {
    unsafe {
        let mut toHealCount: u8 = ((((((&raw const sNumMonsToHealBeforePikeQueen)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(
            ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1988),
                0,
                3,
                false,
            ) as u8) as i32) as isize
                * 3,
        ))
        .cast::<u8>())
        .wrapping_offset(((((&raw mut gSpecialVar_0x8007).cast::<u16>()).read()) as i32) as isize))
        .read();
        TryHealMons(toHealCount);
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((toHealCount) as u16));
    }
}
pub(crate) unsafe extern "C" fn SetHealingroomTypesDisabled() {
    unsafe {
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1988),
            7,
            1,
            ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn IsPartyFullHealed() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    let mut canBeHealed: u32 = 0u32;
                    let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 100);
                    let mut curr: u16 = ((GetMonData2(mon, 57i32)) as u16);
                    let mut max: u16 = ((GetMonData2(mon, 58i32)) as u16);
                    if (((curr) as i32) >= ((max) as i32))
                        && (((GetAilmentFromStatus(GetMonData2(mon, 55i32))) as i32) == 0i32)
                    {
                        let mut ppBonuses: u8 = ((GetMonData2(mon, 21i32)) as u8);
                        {
                            j = 0u8;
                            'l3: loop {
                                if !(((j) as i32) < 4i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    let mut r#move: u16 =
                                        ((GetMonData2(mon, (13i32).wrapping_add(((j) as i32))))
                                            as u16);
                                    max = ((CalculatePPWithBonus(r#move, ppBonuses, j)) as u16);
                                    curr = ((GetMonData2(mon, (17i32).wrapping_add(((j) as i32))))
                                        as u16);
                                    if ((curr) as i32) < ((max) as i32) {
                                        canBeHealed = 1u32;
                                        break 'l3;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    } else {
                        canBeHealed = 1u32;
                    }
                    if canBeHealed == 1u32 {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SaveMonHeldItems() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    let mut heldItem: i32 = ((GetMonData2(
                        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(568))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1630))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                .wrapping_sub(1i32)) as isize
                                * 100,
                        ),
                        12i32,
                    )) as i32);
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1990))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(((heldItem) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RestoreMonHeldItems() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1630))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                .wrapping_sub(1i32)) as isize
                                * 100,
                        ),
                        12i32,
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1990))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitPikeChallenge() {
    unsafe {
        let mut lvlMode: u8 = (crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8);
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
        if !(((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1680)
            .cast::<u32>())
        .read()
            & ((((&raw const sWinStreakFlags)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .read())
            != 0)
        {
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1976))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .write(0u16);
        }
        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(0u16);
        ((&raw mut gBattleOutcome).cast::<u8>()).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn CanEncounterWildMon(enemyMonLevel: u8) -> u8 {
    unsafe {
        let mut enemyMonLevel = enemyMonLevel;
        if !((GetMonData2((&raw mut gPlayerParty).cast::<u8>(), 6i32)) != 0) {
            let mut monAbility: u8 = GetMonAbility((&raw mut gPlayerParty).cast::<u8>());
            if (((monAbility) as i32) == 51i32) || (((monAbility) as i32) == 22i32) {
                let mut playerMonLevel: u8 =
                    ((GetMonData2((&raw mut gPlayerParty).cast::<u8>(), 56i32)) as u8);
                if ((((playerMonLevel) as i32) > 5i32)
                    && (((enemyMonLevel) as i32) <= ((playerMonLevel) as i32).wrapping_sub(5i32)))
                    && (crate::c::rem_i32(((Random()) as i32), 2i32) == 0i32)
                {
                    return 0u8;
                }
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn SpeciesToPikeMonId(species: u16) -> u8 {
    unsafe {
        let mut species = species;
        let mut ret: u8 = 0u8;
        if ((species) as i32) == 379i32 {
            ret = 0u8;
        } else {
            if ((species) as i32) == 329i32 {
                ret = 1u8;
            } else {
                ret = 2u8;
            }
        }
        return ret;
    }
}
