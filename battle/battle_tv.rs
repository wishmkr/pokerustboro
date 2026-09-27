//! Translated from `src/battle_tv.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sVariableDmgMoves sPoints_MoveEffect sPoints_Effectiveness sPoints_SetUp sPoints_RainMoves sPoints_SunMoves sPoints_SandstormMoves sPoints_HailMoves sPoints_ElectricMoves sPoints_StatusDmg sPoints_Status sPoints_Spikes sPoints_WaterSport sPoints_MudSport sPoints_Reflect sPoints_LightScreen sPoints_Safeguard sPoints_Mist sPoints_BreakWall sPoints_CriticalHit sPoints_Faint sPoints_Flinched sPoints_StatIncrease1 sPoints_StatIncrease2 sPoints_StatDecreaseSelf sPoints_StatDecrease1 sPoints_StatDecrease2 sPoints_StatIncreaseNotSelf sPointsArray sSpecialBattleStrings
#[allow(unused_imports)]
use crate::data::battle_tv::*;

unsafe extern "C" {
    static mut gBattleMons: u8;
    static mut gBattleMoveDamage: u8;
    static mut gBattleMoves: u8;
    static mut gBattleMsgDataPtr: u8;
    static mut gBattleScripting: u8;
    static mut gBattleStruct: u8;
    static mut gBattleTextBuff1: u8;
    static mut gBattleTextBuff2: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattlerAttacker: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlerTarget: u8;
    static mut gCurrentMove: u8;
    static mut gEffectBattler: u8;
    static mut gEnemyParty: u8;
    static mut gMoveSelectionCursor: u8;
    static mut gPlayerParty: u8;
    static mut gProtectStructs: u8;
    static mut gSideStatuses: u8;
    static mut gStatuses3: u8;
    fn CalculateBaseDamage(
        a0: *mut u8,
        a1: *mut u8,
        a2: u32,
        a3: u16,
        a4: u16,
        a5: u8,
        a6: u8,
        a7: u8,
    ) -> i32;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetLinkTrainerFlankId(a0: u8) -> u16;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetOpposingLinkMultiBattlerId(a0: u8, a1: u8) -> u8;
    fn PutBattleUpdateOnTheAir(a0: u8, a1: u16, a2: u16, a3: u16);
    fn TryPutBattleSeminarOnAir(a0: u16, a1: u16, a2: u8, a3: *mut u16, a4: u16);
    fn TypeCalc(a0: u16, a1: u8, a2: u8) -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleTv_SetDataBasedOnString(stringId: u16) {
    unsafe {
        let mut stringId = stringId;
        let mut tvPtr: *mut u8 = core::ptr::null_mut();
        let mut atkSide: u32 = 0u32;
        let mut defSide: u32 = 0u32;
        let mut effSide: u32 = 0u32;
        let mut scriptingSide: u32 = 0u32;
        let mut atkMon: *mut u8 = core::ptr::null_mut();
        let mut defMon: *mut u8 = core::ptr::null_mut();
        let mut moveSlot: u8 = 0u8;
        let mut atkFlank: u32 = 0u32;
        let mut defFlank: u32 = 0u32;
        let mut effFlank: u32 = 0u32;
        let mut perishCount: *mut u8 = core::ptr::null_mut();
        let mut statStringId: *mut u16 = core::ptr::null_mut();
        let mut finishedMoveId: *mut u16 = core::ptr::null_mut();
        if ((!((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0))
            && (((stringId) as i32) != 27i32))
            && (((stringId) as i32) != 221i32)
        {
            return;
        }
        tvPtr = (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(516);
        atkSide = ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read())) as u32);
        defSide = ((GetBattlerSide(((&raw mut gBattlerTarget).cast::<u8>()).read())) as u32);
        effSide = ((GetBattlerSide(((&raw mut gEffectBattler).cast::<u8>()).read())) as u32);
        scriptingSide = ((GetBattlerSide(
            ((((&raw mut gBattleMsgDataPtr).cast::<*mut u8>()).read()).wrapping_add(7)).read(),
        )) as u32);
        if atkSide == 0u32 {
            atkMon = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as i32) as isize
                    * 100,
            );
        } else {
            atkMon = ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as i32) as isize
                    * 100,
            );
        }
        if defSide == 0u32 {
            defMon = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as i32) as isize
                    * 100,
            );
        } else {
            defMon = ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as i32) as isize
                    * 100,
            );
        }
        moveSlot = GetBattlerMoveSlotId(
            ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
            ((((&raw mut gBattleMsgDataPtr).cast::<*mut u8>()).read()).cast::<u16>()).read(),
        );
        if ((((moveSlot) as i32) >= 4i32) && ((IsNotSpecialBattleString(stringId)) != 0))
            && (((stringId) as i32) > 12i32)
        {
            crate::c::bf_write(
                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                    .wrapping_offset(((atkSide) as i32) as isize * 12))
                .wrapping_add(7),
                1,
                4,
                (15u32) as i32,
            );
            return;
        }
        perishCount = ((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(4);
        statStringId =
            (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(2)).cast::<u16>();
        finishedMoveId =
            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2)).cast::<u16>();
        atkFlank = ((crate::c::div_i32(
            ((GetBattlerPosition(((&raw mut gBattlerAttacker).cast::<u8>()).read())) as i32),
            2i32,
        )) as u32);
        defFlank = ((crate::c::div_i32(
            ((GetBattlerPosition(((&raw mut gBattlerTarget).cast::<u8>()).read())) as i32),
            2i32,
        )) as u32);
        effFlank = ((crate::c::div_i32(
            ((GetBattlerPosition(((&raw mut gEffectBattler).cast::<u8>()).read())) as i32),
            2i32,
        )) as u32);
        'l1: {
            let __sw1 = ((stringId) as i32);
            let mut __fall = false;
            if __sw1 == 27i32 {
                __fall = true;
                AddMovePoints(1u8, ((moveSlot) as u16), 2u8, 0u8);
                if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0) {
                    TrySetBattleSeminarShow();
                }
                break 'l1;
            }
            if __sw1 == 221i32 {
                __fall = true;
                AddMovePoints(1u8, ((moveSlot) as u16), 1u8, 0u8);
                if (!((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0))
                    && (GetMonData3(defMon, 57i32, core::ptr::null_mut()) != 0u32)
                {
                    TrySetBattleSeminarShow();
                }
                break 'l1;
            }
            if __sw1 == 222i32 {
                __fall = true;
                AddMovePoints(1u8, ((moveSlot) as u16), 0u8, 0u8);
                break 'l1;
            }
            if __sw1 == 161i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(1),
                    7,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(5),
                    2,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 335i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(2),
                    2,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(5),
                    4,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 254i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(2),
                    5,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(5),
                    6,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(8),
                    6,
                    1,
                    (1u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 151i32 {
                __fall = true;
                if (((perishCount).read()) as i32) == 0i32 {
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(7),
                        1,
                        4,
                        (10u32) as i32,
                    );
                }
                break 'l1;
            }
            if __sw1 == 178i32 {
                __fall = true;
                if (crate::c::bf_read(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 12))
                    .wrapping_add(3),
                    0,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    AddMovePoints(
                        2u8,
                        3u16,
                        ((defSide) as u8),
                        (((((crate::c::bf_read(
                            ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                .wrapping_offset(((defSide) as i32) as isize * 12))
                            .wrapping_add(3),
                            0,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32))
                        .wrapping_mul(4u32))
                        .wrapping_add(
                            (crate::c::bf_read(
                                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                    .wrapping_offset(((defSide) as i32) as isize * 12))
                                .wrapping_add(6),
                                0,
                                2,
                                false,
                            ) as u32),
                        )) as u8),
                    );
                }
                break 'l1;
            }
            if __sw1 == 187i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(3),
                    3,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(6),
                    2,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 188i32 {
                __fall = true;
                if (crate::c::bf_read(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 12))
                    .wrapping_add(3),
                    3,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    AddMovePoints(
                        2u8,
                        4u16,
                        ((defSide) as u8),
                        (((((crate::c::bf_read(
                            ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                .wrapping_offset(((defSide) as i32) as isize * 12))
                            .wrapping_add(3),
                            3,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32))
                        .wrapping_mul(4u32))
                        .wrapping_add(
                            (crate::c::bf_read(
                                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                    .wrapping_offset(((defSide) as i32) as isize * 12))
                                .wrapping_add(6),
                                2,
                                2,
                                false,
                            ) as u32),
                        )) as u8),
                    );
                }
                break 'l1;
            }
            if __sw1 == 139i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(6),
                    4,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(6),
                    7,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 140i32 {
                __fall = true;
                if (crate::c::bf_read(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 12))
                    .wrapping_add(6),
                    4,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(7),
                        1,
                        4,
                        (11u32) as i32,
                    );
                }
                break 'l1;
            }
            if __sw1 == 179i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((atkFlank) as i32) as isize * 8))
                    .wrapping_add(5),
                    0,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((atkFlank) as i32) as isize * 8))
                    .wrapping_add(5),
                    3,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 180i32 {
                __fall = true;
                if (crate::c::bf_read(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((atkFlank) as i32) as isize * 8))
                    .wrapping_add(5),
                    0,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    AddMovePoints(
                        2u8,
                        6u16,
                        ((atkSide) as u8),
                        (((((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(((atkFlank) as i32) as isize * 8))
                            .wrapping_add(5),
                            0,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32))
                        .wrapping_mul(4u32))
                        .wrapping_add(
                            (crate::c::bf_read(
                                ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 16))
                                .cast::<u8>())
                                .wrapping_offset(((atkFlank) as i32) as isize * 8))
                                .wrapping_add(5),
                                3,
                                2,
                                false,
                            ) as u32),
                        )) as u8),
                    );
                }
                break 'l1;
            }
            if __sw1 == 181i32 {
                __fall = true;
                if (crate::c::bf_read(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((defFlank) as i32) as isize * 8))
                    .wrapping_add(5),
                    0,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    AddMovePoints(
                        2u8,
                        6u16,
                        ((defSide) as u8),
                        (((((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((defSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(((defFlank) as i32) as isize * 8))
                            .wrapping_add(5),
                            0,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32))
                        .wrapping_mul(4u32))
                        .wrapping_add(
                            (crate::c::bf_read(
                                ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                    .wrapping_offset(((defSide) as i32) as isize * 16))
                                .cast::<u8>())
                                .wrapping_offset(((defFlank) as i32) as isize * 8))
                                .wrapping_add(5),
                                3,
                                2,
                                false,
                            ) as u32),
                        )) as u8),
                    );
                }
                break 'l1;
            }
            if __sw1 == 125i32 {
                __fall = true;
                ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(179))
                    .write(1u8);
                break 'l1;
            }
            if __sw1 == 217i32 {
                __fall = true;
                AddMovePoints(18u8, ((moveSlot) as u16), 0u8, 0u8);
                break 'l1;
            }
            if __sw1 == 213i32 {
                __fall = true;
                if (((((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2)).read()) as i32)
                    != 0i32
                {
                    if (((statStringId).read()) as i32) == 209i32 {
                        AddMovePoints(
                            23u8,
                            ((moveSlot) as u16),
                            (((((((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                .read()) as i32)
                                .wrapping_sub(1i32)) as u8),
                            0u8,
                        );
                    } else {
                        AddMovePoints(
                            22u8,
                            ((moveSlot) as u16),
                            (((((((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                .read()) as i32)
                                .wrapping_sub(1i32)) as u8),
                            0u8,
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 214i32 {
                __fall = true;
                if (((((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2)).read()) as i32)
                    != 0i32
                {
                    if ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                        == ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                    {
                        if (((statStringId).read()) as i32) == 209i32 {
                            AddMovePoints(
                                23u8,
                                ((moveSlot) as u16),
                                (((((((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                    .read()) as i32)
                                    .wrapping_sub(1i32)) as u8),
                                0u8,
                            );
                        } else {
                            AddMovePoints(
                                22u8,
                                ((moveSlot) as u16),
                                (((((((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                    .read()) as i32)
                                    .wrapping_sub(1i32)) as u8),
                                0u8,
                            );
                        }
                    } else {
                        AddMovePoints(
                            27u8,
                            ((moveSlot) as u16),
                            (((((((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                .read()) as i32)
                                .wrapping_sub(1i32)) as u8),
                            0u8,
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 215i32 {
                __fall = true;
                if (((((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2)).read()) as i32)
                    != 0i32
                {
                    AddMovePoints(
                        24u8,
                        ((moveSlot) as u16),
                        (((((((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2)).read())
                            as i32)
                            .wrapping_sub(1i32)) as u8),
                        0u8,
                    );
                }
                break 'l1;
            }
            if __sw1 == 216i32 {
                __fall = true;
                if (((((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2)).read()) as i32)
                    != 0i32
                {
                    if (((statStringId).read()) as i32) == 211i32 {
                        AddMovePoints(
                            26u8,
                            ((moveSlot) as u16),
                            (((((((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                .read()) as i32)
                                .wrapping_sub(1i32)) as u8),
                            0u8,
                        );
                    } else {
                        AddMovePoints(
                            25u8,
                            ((moveSlot) as u16),
                            (((((((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                .read()) as i32)
                                .wrapping_sub(1i32)) as u8),
                            0u8,
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 146i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((defFlank) as i32) as isize * 8))
                    .wrapping_add(0),
                    0,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((defFlank) as i32) as isize * 8))
                    .wrapping_add(2),
                    2,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 147i32 {
                __fall = true;
                if ((GetMonData3(atkMon, 57i32, core::ptr::null_mut())) != 0)
                    && ((crate::c::bf_read(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(((atkFlank) as i32) as isize * 8))
                        .wrapping_add(0),
                        0,
                        3,
                        false,
                    ) as u32)
                        != 0u32)
                {
                    AddMovePoints(
                        8u8,
                        0u16,
                        (((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(((atkFlank) as i32) as isize * 8))
                            .wrapping_add(0),
                            0,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32)) as u8),
                        ((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(((atkFlank) as i32) as isize * 8))
                            .wrapping_add(2),
                            2,
                            2,
                            false,
                        ) as u32) as u8),
                    );
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(7),
                        1,
                        4,
                        (1u32) as i32,
                    );
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(7),
                        5,
                        3,
                        (atkFlank) as i32,
                    );
                }
                break 'l1;
            }
            if __sw1 == 104i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((defFlank) as i32) as isize * 8))
                    .wrapping_add(0),
                    3,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((defFlank) as i32) as isize * 8))
                    .wrapping_add(2),
                    4,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 106i32 {
                __fall = true;
                if (crate::c::bf_read(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((atkFlank) as i32) as isize * 8))
                    .wrapping_add(0),
                    3,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    AddMovePoints(
                        8u8,
                        1u16,
                        (((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(((atkFlank) as i32) as isize * 8))
                            .wrapping_add(0),
                            3,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32)) as u8),
                        ((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(((atkFlank) as i32) as isize * 8))
                            .wrapping_add(2),
                            4,
                            2,
                            false,
                        ) as u32) as u8),
                    );
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(7),
                        1,
                        4,
                        (2u32) as i32,
                    );
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(7),
                        5,
                        3,
                        (atkFlank) as i32,
                    );
                }
                break 'l1;
            }
            if __sw1 == 144i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((defFlank) as i32) as isize * 8))
                    .wrapping_add(0),
                    6,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((defFlank) as i32) as isize * 8))
                    .wrapping_add(2),
                    6,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 145i32 {
                __fall = true;
                if (GetMonData3(atkMon, 57i32, core::ptr::null_mut()) != 0u32)
                    && ((crate::c::bf_read(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(((atkFlank) as i32) as isize * 8))
                        .wrapping_add(0),
                        6,
                        3,
                        false,
                    ) as u32)
                        != 0u32)
                {
                    AddMovePoints(
                        8u8,
                        5u16,
                        (((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(((atkFlank) as i32) as isize * 8))
                            .wrapping_add(0),
                            6,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32)) as u8),
                        ((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(((atkFlank) as i32) as isize * 8))
                            .wrapping_add(2),
                            6,
                            2,
                            false,
                        ) as u32) as u8),
                    );
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(7),
                        1,
                        4,
                        (5u32) as i32,
                    );
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(7),
                        5,
                        3,
                        (atkFlank) as i32,
                    );
                }
                break 'l1;
            }
            if __sw1 == 90i32
                || __sw1 == 91i32
                || __sw1 == 92i32
                || __sw1 == 93i32
                || __sw1 == 328i32
            {
                __fall = true;
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((defFlank) as i32) as isize * 8))
                    .wrapping_add(1),
                    1,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((defFlank) as i32) as isize * 8))
                    .wrapping_add(3),
                    0,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 94i32 {
                __fall = true;
                if (GetMonData3(atkMon, 57i32, core::ptr::null_mut()) != 0u32)
                    && ((crate::c::bf_read(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(((atkFlank) as i32) as isize * 8))
                        .wrapping_add(1),
                        1,
                        3,
                        false,
                    ) as u32)
                        != 0u32)
                {
                    AddMovePoints(
                        8u8,
                        6u16,
                        (((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(((atkFlank) as i32) as isize * 8))
                            .wrapping_add(1),
                            1,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32)) as u8),
                        ((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(((atkFlank) as i32) as isize * 8))
                            .wrapping_add(3),
                            0,
                            2,
                            false,
                        ) as u32) as u8),
                    );
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(7),
                        1,
                        4,
                        (6u32) as i32,
                    );
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(7),
                        5,
                        3,
                        (atkFlank) as i32,
                    );
                }
                break 'l1;
            }
            if __sw1 == 46i32 {
                __fall = true;
                crate::c::bf_write(
                    (((((tvPtr).cast::<u8>()).wrapping_offset(((effSide) as i32) as isize * 24))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gEffectBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(0),
                    6,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    (((((tvPtr).cast::<u8>()).wrapping_offset(((effSide) as i32) as isize * 24))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gEffectBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(2),
                    6,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 48i32 {
                __fall = true;
                if GetMonData3(atkMon, 57i32, core::ptr::null_mut()) != 0u32 {
                    if (crate::c::bf_read(
                        (((((tvPtr).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 24))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                            .read()) as i32) as isize
                                * 4,
                        ))
                        .wrapping_add(0),
                        6,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        AddMovePoints(
                            8u8,
                            4u16,
                            (((crate::c::bf_read(
                                (((((tvPtr).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 24))
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 4,
                                ))
                                .wrapping_add(0),
                                6,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32)) as u8),
                            ((crate::c::bf_read(
                                (((((tvPtr).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 24))
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 4,
                                ))
                                .wrapping_add(2),
                                6,
                                2,
                                false,
                            ) as u32) as u8),
                        );
                    }
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(7),
                        1,
                        4,
                        (4u32) as i32,
                    );
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(7),
                        5,
                        3,
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                        .read()) as u32) as i32,
                    );
                }
                break 'l1;
            }
            if __sw1 == 40i32 {
                __fall = true;
                crate::c::bf_write(
                    (((((tvPtr).cast::<u8>()).wrapping_offset(((effSide) as i32) as isize * 24))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gEffectBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(0),
                    0,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    (((((tvPtr).cast::<u8>()).wrapping_offset(((effSide) as i32) as isize * 24))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gEffectBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(2),
                    2,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 44i32 {
                __fall = true;
                crate::c::bf_write(
                    (((((tvPtr).cast::<u8>()).wrapping_offset(((effSide) as i32) as isize * 24))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gEffectBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(0),
                    3,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    (((((tvPtr).cast::<u8>()).wrapping_offset(((effSide) as i32) as isize * 24))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gEffectBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(2),
                    4,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 42i32 {
                __fall = true;
                if GetMonData3(atkMon, 57i32, core::ptr::null_mut()) != 0u32 {
                    if (crate::c::bf_read(
                        (((((tvPtr).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 24))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                            .read()) as i32) as isize
                                * 4,
                        ))
                        .wrapping_add(0),
                        0,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        AddMovePoints(
                            8u8,
                            2u16,
                            (((crate::c::bf_read(
                                (((((tvPtr).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 24))
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 4,
                                ))
                                .wrapping_add(0),
                                0,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32)) as u8),
                            ((crate::c::bf_read(
                                (((((tvPtr).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 24))
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 4,
                                ))
                                .wrapping_add(2),
                                2,
                                2,
                                false,
                            ) as u32) as u8),
                        );
                    }
                    if (crate::c::bf_read(
                        (((((tvPtr).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 24))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                            .read()) as i32) as isize
                                * 4,
                        ))
                        .wrapping_add(0),
                        3,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        AddMovePoints(
                            8u8,
                            3u16,
                            (((crate::c::bf_read(
                                (((((tvPtr).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 24))
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 4,
                                ))
                                .wrapping_add(0),
                                3,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32)) as u8),
                            ((crate::c::bf_read(
                                (((((tvPtr).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 24))
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 4,
                                ))
                                .wrapping_add(2),
                                4,
                                2,
                                false,
                            ) as u32) as u8),
                        );
                    }
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(7),
                        1,
                        4,
                        (3u32) as i32,
                    );
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(7),
                        5,
                        3,
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                        .read()) as u32) as i32,
                    );
                }
                break 'l1;
            }
            if __sw1 == 69i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((defFlank) as i32) as isize * 8))
                    .wrapping_add(1),
                    4,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((defFlank) as i32) as isize * 8))
                    .wrapping_add(3),
                    2,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 71i32 {
                __fall = true;
                if (crate::c::bf_read(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((atkFlank) as i32) as isize * 8))
                    .wrapping_add(1),
                    4,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    AddMovePoints(
                        9u8,
                        0u16,
                        (((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(((atkFlank) as i32) as isize * 8))
                            .wrapping_add(1),
                            4,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32)) as u8),
                        ((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(((atkFlank) as i32) as isize * 8))
                            .wrapping_add(3),
                            2,
                            2,
                            false,
                        ) as u32) as u8),
                    );
                }
                break 'l1;
            }
            if __sw1 == 55i32 {
                __fall = true;
                crate::c::bf_write(
                    (((((tvPtr).cast::<u8>()).wrapping_offset(((effSide) as i32) as isize * 24))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gEffectBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(1),
                    1,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    (((((tvPtr).cast::<u8>()).wrapping_offset(((effSide) as i32) as isize * 24))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gEffectBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(3),
                    0,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 57i32 {
                __fall = true;
                if (crate::c::bf_read(
                    (((((tvPtr).cast::<u8>()).wrapping_offset(((atkSide) as i32) as isize * 24))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(1),
                    1,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    AddMovePoints(
                        9u8,
                        2u16,
                        (((crate::c::bf_read(
                            (((((tvPtr).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 24))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 4,
                            ))
                            .wrapping_add(1),
                            1,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32)) as u8),
                        ((crate::c::bf_read(
                            (((((tvPtr).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 24))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 4,
                            ))
                            .wrapping_add(3),
                            0,
                            2,
                            false,
                        ) as u32) as u8),
                    );
                }
                break 'l1;
            }
            if __sw1 == 35i32 {
                __fall = true;
                crate::c::bf_write(
                    (((((tvPtr).cast::<u8>()).wrapping_offset(((effSide) as i32) as isize * 24))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gEffectBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(1),
                    4,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    (((((tvPtr).cast::<u8>()).wrapping_offset(((effSide) as i32) as isize * 24))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gEffectBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(3),
                    2,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 107i32 {
                __fall = true;
                if (((crate::c::bf_read(
                    (((((tvPtr).cast::<u8>()).wrapping_offset(((atkSide) as i32) as isize * 24))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(1),
                    4,
                    3,
                    false,
                ) as u32)
                    != 0u32)
                    && (((((((&raw mut gBattleMsgDataPtr).cast::<*mut u8>()).read()).cast::<u16>())
                        .read()) as i32)
                        != 173i32))
                    && (((((((&raw mut gBattleMsgDataPtr).cast::<*mut u8>()).read()).cast::<u16>())
                        .read()) as i32)
                        != 214i32)
                {
                    AddMovePoints(
                        9u8,
                        3u16,
                        (((crate::c::bf_read(
                            (((((tvPtr).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 24))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 4,
                            ))
                            .wrapping_add(1),
                            4,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32)) as u8),
                        ((crate::c::bf_read(
                            (((((tvPtr).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 24))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 4,
                            ))
                            .wrapping_add(3),
                            2,
                            2,
                            false,
                        ) as u32) as u8),
                    );
                }
                break 'l1;
            }
            if __sw1 == 49i32 {
                __fall = true;
                crate::c::bf_write(
                    (((((tvPtr).cast::<u8>()).wrapping_offset(((effSide) as i32) as isize * 24))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gEffectBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(1),
                    7,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    (((((tvPtr).cast::<u8>()).wrapping_offset(((effSide) as i32) as isize * 24))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gEffectBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(3),
                    4,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 51i32 {
                __fall = true;
                if (crate::c::bf_read(
                    (((((tvPtr).cast::<u8>()).wrapping_offset(((atkSide) as i32) as isize * 24))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(1),
                    7,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    AddMovePoints(
                        9u8,
                        4u16,
                        (((crate::c::bf_read(
                            (((((tvPtr).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 24))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 4,
                            ))
                            .wrapping_add(1),
                            7,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32)) as u8),
                        ((crate::c::bf_read(
                            (((((tvPtr).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 24))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 4,
                            ))
                            .wrapping_add(3),
                            4,
                            2,
                            false,
                        ) as u32) as u8),
                    );
                }
                break 'l1;
            }
            if __sw1 == 67i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((effSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((effFlank) as i32) as isize * 8))
                    .wrapping_add(1),
                    7,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((effSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((effFlank) as i32) as isize * 8))
                    .wrapping_add(3),
                    4,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 230i32 {
                __fall = true;
                if (crate::c::bf_read(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((atkFlank) as i32) as isize * 8))
                    .wrapping_add(1),
                    7,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    AddMovePoints(
                        9u8,
                        1u16,
                        (((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(((atkFlank) as i32) as isize * 8))
                            .wrapping_add(1),
                            7,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32)) as u8),
                        ((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(((atkFlank) as i32) as isize * 8))
                            .wrapping_add(3),
                            4,
                            2,
                            false,
                        ) as u32) as u8),
                    );
                }
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(7),
                    1,
                    4,
                    (12u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 148i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 12))
                    .wrapping_add(0),
                    0,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 12))
                    .wrapping_add(4),
                    0,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 149i32 {
                __fall = true;
                if (crate::c::bf_read(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((scriptingSide) as i32) as isize * 12))
                    .wrapping_add(0),
                    0,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    AddMovePoints(
                        10u8,
                        ((scriptingSide ^ 1u32) as u16),
                        (((crate::c::bf_read(
                            ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                .wrapping_offset(((scriptingSide) as i32) as isize * 12))
                            .wrapping_add(0),
                            0,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32)) as u8),
                        ((crate::c::bf_read(
                            ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                .wrapping_offset(((scriptingSide) as i32) as isize * 12))
                            .wrapping_add(4),
                            0,
                            2,
                            false,
                        ) as u32) as u8),
                    );
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((scriptingSide) as i32) as isize * 12))
                        .wrapping_add(7),
                        1,
                        4,
                        (7u32) as i32,
                    );
                }
                break 'l1;
            }
            if __sw1 == 159i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(0),
                    0,
                    3,
                    (0u32) as i32,
                );
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(4),
                    0,
                    2,
                    (0u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 316i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((atkFlank) as i32) as isize * 8))
                    .wrapping_add(4),
                    0,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((atkFlank) as i32) as isize * 8))
                    .wrapping_add(3),
                    6,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 315i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((atkFlank) as i32) as isize * 8))
                    .wrapping_add(4),
                    3,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((atkFlank) as i32) as isize * 8))
                    .wrapping_add(4),
                    6,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 28i32 {
                __fall = true;
                AddPointsOnFainting(0u8);
            }
            if __fall || __sw1 == 2i32 {
                __fall = true;
                if (crate::c::bf_read(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((atkFlank) as i32) as isize * 8))
                    .wrapping_add(4),
                    0,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    crate::c::bf_write(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(((atkFlank) as i32) as isize * 8))
                        .wrapping_add(4),
                        0,
                        3,
                        (0u32) as i32,
                    );
                    crate::c::bf_write(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(((atkFlank) as i32) as isize * 8))
                        .wrapping_add(3),
                        6,
                        2,
                        (0u32) as i32,
                    );
                }
                if (crate::c::bf_read(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((atkFlank) as i32) as isize * 8))
                    .wrapping_add(4),
                    3,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    crate::c::bf_write(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(((atkFlank) as i32) as isize * 8))
                        .wrapping_add(4),
                        3,
                        3,
                        (0u32) as i32,
                    );
                    crate::c::bf_write(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(((atkFlank) as i32) as isize * 8))
                        .wrapping_add(4),
                        6,
                        2,
                        (0u32) as i32,
                    );
                }
                break 'l1;
            }
            if __sw1 == 29i32 {
                __fall = true;
                AddPointsOnFainting(1u8);
                if (crate::c::bf_read(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((defFlank) as i32) as isize * 8))
                    .wrapping_add(4),
                    0,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    crate::c::bf_write(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(((defFlank) as i32) as isize * 8))
                        .wrapping_add(4),
                        0,
                        3,
                        (0u32) as i32,
                    );
                    crate::c::bf_write(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(((defFlank) as i32) as isize * 8))
                        .wrapping_add(3),
                        6,
                        2,
                        (0u32) as i32,
                    );
                }
                if (crate::c::bf_read(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(((defFlank) as i32) as isize * 8))
                    .wrapping_add(4),
                    3,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    crate::c::bf_write(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(((defFlank) as i32) as isize * 8))
                        .wrapping_add(4),
                        3,
                        3,
                        (0u32) as i32,
                    );
                    crate::c::bf_write(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(((defFlank) as i32) as isize * 8))
                        .wrapping_add(4),
                        6,
                        2,
                        (0u32) as i32,
                    );
                }
                break 'l1;
            }
            if __sw1 == 78i32 || __sw1 == 352i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(0),
                    3,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(4),
                    2,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 77i32 || __sw1 == 353i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(0),
                    6,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(4),
                    4,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 351i32 {
                __fall = true;
                if (((finishedMoveId).read()) as i32) == 115i32 {
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(0),
                        3,
                        3,
                        (0u32) as i32,
                    );
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(4),
                        2,
                        2,
                        (0u32) as i32,
                    );
                }
                if (((finishedMoveId).read()) as i32) == 113i32 {
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(0),
                        6,
                        3,
                        (0u32) as i32,
                    );
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(4),
                        4,
                        2,
                        (0u32) as i32,
                    );
                }
                if (((finishedMoveId).read()) as i32) == 54i32 {
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(1),
                        4,
                        3,
                        (0u32) as i32,
                    );
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(5),
                        0,
                        2,
                        (0u32) as i32,
                    );
                }
                break 'l1;
            }
            if __sw1 == 79i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(1),
                    1,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(4),
                    6,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 80i32 {
                __fall = true;
                if (crate::c::bf_read(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 12))
                    .wrapping_add(1),
                    1,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    AddMovePoints(
                        15u8,
                        0u16,
                        (((crate::c::bf_read(
                            ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                .wrapping_offset(((defSide) as i32) as isize * 12))
                            .wrapping_add(1),
                            1,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32)) as u8),
                        ((crate::c::bf_read(
                            ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                .wrapping_offset(((defSide) as i32) as isize * 12))
                            .wrapping_add(4),
                            6,
                            2,
                            false,
                        ) as u32) as u8),
                    );
                }
                break 'l1;
            }
            if __sw1 == 81i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(1),
                    1,
                    3,
                    (0u32) as i32,
                );
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(4),
                    6,
                    2,
                    (0u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 97i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(1),
                    4,
                    3,
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u32) as i32,
                );
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(5),
                    0,
                    2,
                    ((moveSlot) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 98i32 {
                __fall = true;
                if (crate::c::bf_read(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 12))
                    .wrapping_add(1),
                    4,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    AddMovePoints(
                        16u8,
                        0u16,
                        (((crate::c::bf_read(
                            ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                .wrapping_offset(((defSide) as i32) as isize * 12))
                            .wrapping_add(1),
                            4,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32)) as u8),
                        ((crate::c::bf_read(
                            ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                .wrapping_offset(((defSide) as i32) as isize * 12))
                            .wrapping_add(5),
                            0,
                            2,
                            false,
                        ) as u32) as u8),
                    );
                }
                break 'l1;
            }
            if __sw1 == 354i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 12))
                    .wrapping_add(0),
                    3,
                    3,
                    (0u32) as i32,
                );
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 12))
                    .wrapping_add(4),
                    2,
                    2,
                    (0u32) as i32,
                );
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 12))
                    .wrapping_add(0),
                    6,
                    3,
                    (0u32) as i32,
                );
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 12))
                    .wrapping_add(4),
                    4,
                    2,
                    (0u32) as i32,
                );
                AddMovePoints(
                    17u8,
                    0u16,
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as u8),
                    moveSlot,
                );
                break 'l1;
            }
            if __sw1 == 74i32 {
                __fall = true;
                if (crate::c::bf_read(
                    (((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_add(5),
                    5,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    AddMovePoints(
                        21u8,
                        0u16,
                        (((crate::c::bf_read(
                            (((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_add(5),
                            5,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32)) as u8),
                        ((crate::c::bf_read(
                            (((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_add(6),
                            0,
                            2,
                            false,
                        ) as u32) as u8),
                    );
                }
                if (crate::c::bf_read(
                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_offset(8))
                    .wrapping_add(5),
                    5,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    AddMovePoints(
                        21u8,
                        0u16,
                        (((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(8))
                            .wrapping_add(5),
                            5,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32)) as u8),
                        ((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(8))
                            .wrapping_add(6),
                            0,
                            2,
                            false,
                        ) as u32) as u8),
                    );
                }
                break 'l1;
            }
            if __sw1 == 96i32 || __sw1 == 100i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(7),
                    1,
                    4,
                    (14u32) as i32,
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsNotSpecialBattleString(stringId: u16) -> u8 {
    unsafe {
        let mut stringId = stringId;
        let mut i: i32 = 0i32;
        'l1: loop {
            'l2: {
                if ((((((&raw const sSpecialBattleStrings)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset((i) as isize))
                .read()) as i32)
                    == ((stringId) as i32)
                {
                    break 'l1;
                }
                i = (i).wrapping_add(1);
            }
            if !(((((((&raw const sSpecialBattleStrings)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset((i) as isize))
            .read()) as i32)
                != 65535i32)
            {
                break 'l1;
            }
        }
        if ((((((&raw const sSpecialBattleStrings)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset((i) as isize))
        .read()) as i32)
            == 65535i32
        {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleTv_SetDataBasedOnMove(
    r#move: u16,
    weatherFlags: u16,
    disableStructPtr: *mut u8,
) {
    unsafe {
        let mut r#move = r#move;
        let mut weatherFlags = weatherFlags;
        let mut disableStructPtr = disableStructPtr;
        let mut tvPtr: *mut u8 = core::ptr::null_mut();
        let mut atkSide: u32 = 0u32;
        let mut defSide: u32 = 0u32;
        let mut moveSlot: u8 = 0u8;
        if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0) {
            return;
        }
        tvPtr = (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(516);
        atkSide = ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read())) as u32);
        defSide = ((GetBattlerSide(((&raw mut gBattlerTarget).cast::<u8>()).read())) as u32);
        moveSlot = GetBattlerMoveSlotId(((&raw mut gBattlerAttacker).cast::<u8>()).read(), r#move);
        if ((moveSlot) as i32) >= 4i32 {
            crate::c::bf_write(
                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                    .wrapping_offset(((atkSide) as i32) as isize * 12))
                .wrapping_add(7),
                1,
                4,
                (15u32) as i32,
            );
            return;
        }
        crate::c::bf_write(
            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                .wrapping_offset(((defSide) as i32) as isize * 16))
            .cast::<u8>())
            .wrapping_offset(
                (crate::c::div_i32(
                    ((GetBattlerPosition(((&raw mut gBattlerAttacker).cast::<u8>()).read()))
                        as i32),
                    2i32,
                )) as isize
                    * 8,
            ))
            .wrapping_add(5),
            5,
            3,
            ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>()).wrapping_offset(
                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32)
                .wrapping_add(1i32)) as u32) as i32,
        );
        crate::c::bf_write(
            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                .wrapping_offset(((defSide) as i32) as isize * 16))
            .cast::<u8>())
            .wrapping_offset(
                (crate::c::div_i32(
                    ((GetBattlerPosition(((&raw mut gBattlerAttacker).cast::<u8>()).read()))
                        as i32),
                    2i32,
                )) as isize
                    * 8,
            ))
            .wrapping_add(6),
            0,
            2,
            ((moveSlot) as u32) as i32,
        );
        crate::c::bf_write(
            ((((tvPtr).wrapping_add(80)).cast::<u8>())
                .wrapping_offset(((atkSide) as i32) as isize * 12))
            .wrapping_add(3),
            6,
            2,
            ((moveSlot) as u32) as i32,
        );
        AddMovePoints(
            0u8,
            ((moveSlot) as u16),
            (((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .read(),
            0u8,
        );
        AddPointsBasedOnWeather(weatherFlags, r#move, moveSlot);
        if ((crate::c::bf_read((disableStructPtr).wrapping_add(18), 0, 4, false) as u8) as i32)
            != 0i32
        {
            AddMovePoints(7u8, r#move, moveSlot, 0u8);
        }
        if ((r#move) as i32) == 273i32 {
            crate::c::bf_write(
                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                    .wrapping_offset(((atkSide) as i32) as isize * 12))
                .wrapping_add(3),
                0,
                3,
                ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as i32)
                    .wrapping_add(1i32)) as u32) as i32,
            );
            crate::c::bf_write(
                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                    .wrapping_offset(((atkSide) as i32) as isize * 12))
                .wrapping_add(6),
                0,
                2,
                ((moveSlot) as u32) as i32,
            );
        }
        if (((r#move) as i32) == 120i32) || (((r#move) as i32) == 153i32) {
            crate::c::bf_write(
                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                    .wrapping_offset(((atkSide ^ 1u32) as i32) as isize * 12))
                .wrapping_add(8),
                3,
                3,
                ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as i32)
                    .wrapping_add(1i32)) as u32) as i32,
            );
            crate::c::bf_write(
                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                    .wrapping_offset(((atkSide ^ 1u32) as i32) as isize * 12))
                .wrapping_add(8),
                1,
                2,
                ((moveSlot) as u32) as i32,
            );
            crate::c::bf_write(
                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                    .wrapping_offset(((atkSide ^ 1u32) as i32) as isize * 12))
                .wrapping_add(7),
                1,
                4,
                (13u32) as i32,
            );
            crate::c::bf_write(
                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                    .wrapping_offset(((atkSide ^ 1u32) as i32) as isize * 12))
                .wrapping_add(8),
                0,
                1,
                (1u32) as i32,
            );
        }
        AddMovePoints(
            13u8,
            ((((((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .wrapping_add(2))
            .read()) as u16),
            ((((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .wrapping_add(1))
            .read(),
            0u8,
        );
        AddMovePoints(
            14u8,
            ((((((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .wrapping_add(2))
            .read()) as u16),
            ((((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .wrapping_add(1))
            .read(),
            0u8,
        );
        AddMovePoints(
            11u8,
            ((((((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .wrapping_add(2))
            .read()) as u16),
            0u8,
            0u8,
        );
        AddMovePoints(
            12u8,
            ((((((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .wrapping_add(2))
            .read()) as u16),
            0u8,
            0u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleTv_SetDataBasedOnAnimation(animationId: u8) {
    unsafe {
        let mut animationId = animationId;
        let mut tvPtr: *mut u8 = core::ptr::null_mut();
        let mut atkSide: u32 = 0u32;
        if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0) {
            return;
        }
        tvPtr = (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(516);
        atkSide = ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read())) as u32);
        'l1: {
            let __sw1 = ((animationId) as i32);
            if __sw1 == 18i32 {
                if (crate::c::bf_read(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(1),
                    7,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    AddMovePoints(
                        2u8,
                        0u16,
                        ((atkSide) as u8),
                        (((((crate::c::bf_read(
                            ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 12))
                            .wrapping_add(1),
                            7,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32))
                        .wrapping_mul(4u32))
                        .wrapping_add(
                            (crate::c::bf_read(
                                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 12))
                                .wrapping_add(5),
                                2,
                                2,
                                false,
                            ) as u32),
                        )) as u8),
                    );
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(7),
                        1,
                        4,
                        (8u32) as i32,
                    );
                }
                break 'l1;
            }
            if __sw1 == 19i32 {
                if (crate::c::bf_read(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(2),
                    2,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    AddMovePoints(
                        2u8,
                        1u16,
                        ((atkSide) as u8),
                        (((((crate::c::bf_read(
                            ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 12))
                            .wrapping_add(2),
                            2,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32))
                        .wrapping_mul(4u32))
                        .wrapping_add(
                            (crate::c::bf_read(
                                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 12))
                                .wrapping_add(5),
                                4,
                                2,
                                false,
                            ) as u32),
                        )) as u8),
                    );
                    crate::c::bf_write(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(7),
                        1,
                        4,
                        (9u32) as i32,
                    );
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutLinkBattleTvShowOnAir() {
    unsafe {
        let mut playerBestSpecies: u16 = 0u16;
        let mut opponentBestSpecies: u16 = 0u16;
        let mut playerBestSum: i16 = 0i16;
        let mut opponentBestSum: i16 = 32767i16;
        let mut playerBestMonId: u8 = 0u8;
        let mut opponentBestMonId: u8 = 0u8;
        let mut movePoints: *mut u8 = core::ptr::null_mut();
        let mut countPlayer: u8 = 0u8;
        let mut countOpponent: u8 = 0u8;
        let mut sum: i16 = 0i16;
        let mut species: u16 = 0u16;
        let mut r#move: u16 = 0u16;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut zero: i32 = 0i32;
        let mut one: i32 = 1i32;
        if (((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(179)).read()) != 0 {
            return;
        }
        movePoints = (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(420);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if GetMonData3(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        11i32,
                        core::ptr::null_mut(),
                    ) != 0u32
                    {
                        countPlayer = (countPlayer).wrapping_add(1);
                    }
                    if GetMonData3(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        11i32,
                        core::ptr::null_mut(),
                    ) != 0u32
                    {
                        countOpponent = (countOpponent).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (!((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0))
            || (((countPlayer) as i32) != ((countOpponent) as i32))
        {
            return;
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 6i32) {
                    break 'l3;
                }
                'l4: {
                    species = ((GetMonData3(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        11i32,
                        core::ptr::null_mut(),
                    )) as u16);
                    if (((species) as i32) != 0i32)
                        && (!((GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            45i32,
                            core::ptr::null_mut(),
                        )) != 0))
                    {
                        {
                            sum = 0i16;
                            j = 0i32;
                            'l5: loop {
                                if !(j < 4i32) {
                                    break 'l5;
                                }
                                'l6: {
                                    sum = ((((sum) as i32).wrapping_add(
                                        (((((((movePoints).cast::<u8>())
                                            .wrapping_offset((zero) as isize * 48))
                                        .cast::<i16>())
                                        .wrapping_offset(
                                            (((i).wrapping_mul(4i32)).wrapping_add(j)) as isize,
                                        ))
                                        .read()) as i32),
                                    )) as i16);
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        if ((playerBestSum) as i32) < ((sum) as i32) {
                            playerBestMonId = ((i) as u8);
                            playerBestSum = sum;
                            playerBestSpecies = species;
                        }
                    }
                    species = ((GetMonData3(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        11i32,
                        core::ptr::null_mut(),
                    )) as u16);
                    if (((species) as i32) != 0i32)
                        && (!((GetMonData3(
                            ((&raw mut gEnemyParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            45i32,
                            core::ptr::null_mut(),
                        )) != 0))
                    {
                        {
                            sum = 0i16;
                            j = 0i32;
                            'l7: loop {
                                if !(j < 4i32) {
                                    break 'l7;
                                }
                                'l8: {
                                    sum = ((((sum) as i32).wrapping_add(
                                        (((((((movePoints).cast::<u8>())
                                            .wrapping_offset((one) as isize * 48))
                                        .cast::<i16>())
                                        .wrapping_offset(
                                            (((i).wrapping_mul(4i32)).wrapping_add(j)) as isize,
                                        ))
                                        .read()) as i32),
                                    )) as i16);
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        if ((opponentBestSum) as i32) == ((sum) as i32) {
                            if GetMonData3(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                25i32,
                                core::ptr::null_mut(),
                            ) > GetMonData3(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset(((opponentBestMonId) as i32) as isize * 100),
                                25i32,
                                core::ptr::null_mut(),
                            ) {
                                opponentBestMonId = ((i) as u8);
                                opponentBestSum = sum;
                                opponentBestSpecies = species;
                            }
                        } else {
                            if ((opponentBestSum) as i32) > ((sum) as i32) {
                                opponentBestMonId = ((i) as u8);
                                opponentBestSum = sum;
                                opponentBestSpecies = species;
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            sum = 0i16;
            i = 0i32;
            j = 0i32;
            'l9: loop {
                if !(j < 4i32) {
                    break 'l9;
                }
                'l10: {
                    if ((sum) as i32)
                        < (((((((movePoints).cast::<u8>()).wrapping_offset((zero) as isize * 48))
                            .cast::<i16>())
                        .wrapping_offset(
                            ((((playerBestMonId) as i32).wrapping_mul(4i32)).wrapping_add(j))
                                as isize,
                        ))
                        .read()) as i32)
                    {
                        sum = (((((movePoints).cast::<u8>())
                            .wrapping_offset((zero) as isize * 48))
                        .cast::<i16>())
                        .wrapping_offset(
                            ((((playerBestMonId) as i32).wrapping_mul(4i32)).wrapping_add(j))
                                as isize,
                        ))
                        .read();
                        i = j;
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
        r#move = ((GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((playerBestMonId) as i32) as isize * 100),
            (13i32).wrapping_add(i),
            core::ptr::null_mut(),
        )) as u16);
        if (((playerBestSum) as i32) == 0i32) || (((r#move) as i32) == 0i32) {
            return;
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0 {
            if ((((playerBestMonId) as i32) < crate::c::div_i32(6i32, 2i32))
                && (!((GetLinkTrainerFlankId(
                    (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37)).read(),
                )) != 0)))
                || ((((playerBestMonId) as i32) >= crate::c::div_i32(6i32, 2i32))
                    && ((GetLinkTrainerFlankId(
                        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37)).read(),
                    )) != 0))
            {
                j = (if ((opponentBestMonId) as i32) < crate::c::div_i32(6i32, 2i32) {
                    0i32
                } else {
                    1i32
                });
                PutBattleUpdateOnTheAir(
                    GetOpposingLinkMultiBattlerId(
                        ((j) as u8),
                        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37)).read(),
                    ),
                    r#move,
                    playerBestSpecies,
                    opponentBestSpecies,
                );
            }
        } else {
            PutBattleUpdateOnTheAir(
                (((((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37)).read()) as i32)
                    ^ 1i32) as u8),
                r#move,
                playerBestSpecies,
                opponentBestSpecies,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn AddMovePoints(caseId: u8, arg1: u16, arg2: u8, arg3: u8) {
    unsafe {
        let mut caseId = caseId;
        let mut arg1 = arg1;
        let mut arg2 = arg2;
        let mut arg3 = arg3;
        let mut movePoints: *mut u8 =
            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(420);
        let mut tvPtr: *mut u8 =
            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(516);
        let mut atkSide: u32 =
            ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read())) as u32);
        let mut defSide: u32 =
            ((GetBattlerSide(((&raw mut gBattlerTarget).cast::<u8>()).read())) as u32);
        let mut ptr: *mut u16 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 = ((caseId) as i32);
            let mut __fall = false;
            if __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 18i32
                || __sw1 == 22i32
                || __sw1 == 23i32
                || __sw1 == 24i32
                || __sw1 == 25i32
                || __sw1 == 26i32
                || __sw1 == 27i32
            {
                __fall = true;
                let __p2 = ((((movePoints).cast::<u8>())
                    .wrapping_offset(((atkSide) as i32) as isize * 48))
                .cast::<i16>())
                .wrapping_offset(
                    ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_mul(4i32))
                    .wrapping_add(((arg1) as i32))) as isize,
                );
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        ((((((((&raw const sPointsArray)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u16>())
                        .cast::<*mut u16>())
                        .wrapping_offset(((caseId) as i32) as isize))
                        .read())
                        .wrapping_offset(((arg2) as i32) as isize))
                        .read()) as i32),
                    )) as i16),
                );
                break 'l1;
            }
            if __sw1 == 3i32 || __sw1 == 4i32 || __sw1 == 5i32 || __sw1 == 6i32 || __sw1 == 7i32 {
                __fall = true;
                i = 0i32;
                ptr = ((((&raw const sPointsArray)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .cast::<*mut u16>())
                .wrapping_offset(((caseId) as i32) as isize))
                .read();
                'l2: loop {
                    'l3: {
                        if ((arg1) as i32)
                            == ((((ptr).wrapping_offset((i) as isize)).read()) as i32)
                        {
                            let __p3 = ((((movePoints).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 48))
                            .cast::<i16>())
                            .wrapping_offset(
                                ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                                .read()) as i32)
                                    .wrapping_mul(4i32))
                                .wrapping_add(((arg2) as i32)))
                                    as isize,
                            );
                            (__p3).write(
                                (((((__p3).read()) as i32).wrapping_add(
                                    ((((ptr).wrapping_offset(((i).wrapping_add(1i32)) as isize))
                                        .read()) as i32),
                                )) as i16),
                            );
                            break 'l2;
                        }
                        i = (i).wrapping_add(2i32);
                    }
                    if !(((((ptr).wrapping_offset((i) as isize)).read()) as i32) != 65535i32) {
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 19i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset((((arg2) as i32) ^ 1i32) as isize * 12))
                    .wrapping_add(7),
                    1,
                    4,
                    (0u32) as i32,
                );
                let __p4 = ((((movePoints).cast::<u8>())
                    .wrapping_offset(((arg2) as i32) as isize * 48))
                .cast::<i16>())
                .wrapping_offset(((0i32).wrapping_add(((arg3) as i32))) as isize);
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_add(
                        ((((((((&raw const sPointsArray)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u16>())
                        .cast::<*mut u16>())
                        .wrapping_offset(((caseId) as i32) as isize))
                        .read())
                        .wrapping_offset(((arg1) as i32) as isize))
                        .read()) as i32),
                    )) as i16),
                );
                break 'l1;
            }
            if __sw1 == 20i32 {
                __fall = true;
                crate::c::bf_write(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((arg2) as i32) as isize * 12))
                    .wrapping_add(7),
                    1,
                    4,
                    (0u32) as i32,
                );
            }
            if __fall || __sw1 == 2i32 {
                __fall = true;
                let __p5 = ((((movePoints).cast::<u8>())
                    .wrapping_offset(((arg2) as i32) as isize * 48))
                .cast::<i16>())
                .wrapping_offset(((0i32).wrapping_add(((arg3) as i32))) as isize);
                (__p5).write(
                    (((((__p5).read()) as i32).wrapping_add(
                        ((((((((&raw const sPointsArray)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u16>())
                        .cast::<*mut u16>())
                        .wrapping_offset(((caseId) as i32) as isize))
                        .read())
                        .wrapping_offset(((arg1) as i32) as isize))
                        .read()) as i32),
                    )) as i16),
                );
                break 'l1;
            }
            if __sw1 == 17i32 {
                __fall = true;
                let __p6 = ((((movePoints).cast::<u8>())
                    .wrapping_offset(((atkSide) as i32) as isize * 48))
                .cast::<i16>())
                .wrapping_offset(
                    ((((arg2) as i32).wrapping_mul(4i32)).wrapping_add(((arg3) as i32))) as isize,
                );
                (__p6).write(
                    (((((__p6).read()) as i32).wrapping_add(
                        ((((((((&raw const sPointsArray)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u16>())
                        .cast::<*mut u16>())
                        .wrapping_offset(((caseId) as i32) as isize))
                        .read())
                        .wrapping_offset(((arg1) as i32) as isize))
                        .read()) as i32),
                    )) as i16),
                );
                break 'l1;
            }
            if __sw1 == 8i32 || __sw1 == 9i32 || __sw1 == 15i32 || __sw1 == 16i32 || __sw1 == 21i32
            {
                __fall = true;
                let __p7 = ((((movePoints).cast::<u8>())
                    .wrapping_offset(((atkSide ^ 1u32) as i32) as isize * 48))
                .cast::<i16>())
                .wrapping_offset(
                    ((((arg2) as i32).wrapping_mul(4i32)).wrapping_add(((arg3) as i32))) as isize,
                );
                (__p7).write(
                    (((((__p7).read()) as i32).wrapping_add(
                        ((((((((&raw const sPointsArray)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u16>())
                        .cast::<*mut u16>())
                        .wrapping_offset(((caseId) as i32) as isize))
                        .read())
                        .wrapping_offset(((arg1) as i32) as isize))
                        .read()) as i32),
                    )) as i16),
                );
                break 'l1;
            }
            if __sw1 == 10i32 {
                __fall = true;
                let __p8 = ((((movePoints).cast::<u8>())
                    .wrapping_offset(((arg1) as i32) as isize * 48))
                .cast::<i16>())
                .wrapping_offset(
                    ((((arg2) as i32).wrapping_mul(4i32)).wrapping_add(((arg3) as i32))) as isize,
                );
                (__p8).write(
                    (((((__p8).read()) as i32).wrapping_add(
                        (((((((&raw const sPointsArray)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u16>())
                        .cast::<*mut u16>())
                        .wrapping_offset(((caseId) as i32) as isize))
                        .read())
                        .read()) as i32),
                    )) as i16),
                );
                break 'l1;
            }
            if __sw1 == 11i32 {
                __fall = true;
                if ((crate::c::bf_read(
                    (((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_add(4),
                    0,
                    3,
                    false,
                ) as u32)
                    != (crate::c::bf_read(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((defSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(8))
                        .wrapping_add(4),
                        0,
                        3,
                        false,
                    ) as u32)
                        .wrapping_neg())
                    && (((arg1) as i32) == 10i32)
                {
                    if (crate::c::bf_read(
                        (((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((defSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_add(4),
                        0,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        let mut id: u32 = ((crate::c::bf_read(
                            (((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((defSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_add(4),
                            0,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32))
                        .wrapping_mul(4u32);
                        let __p9 = ((((movePoints).cast::<u8>())
                            .wrapping_offset(((defSide) as i32) as isize * 48))
                        .cast::<i16>())
                        .wrapping_offset(
                            (((id).wrapping_add(
                                (crate::c::bf_read(
                                    (((((tvPtr).wrapping_add(48)).cast::<u8>())
                                        .wrapping_offset(((defSide) as i32) as isize * 16))
                                    .cast::<u8>())
                                    .wrapping_add(3),
                                    6,
                                    2,
                                    false,
                                ) as u32),
                            )) as i32) as isize,
                        );
                        (__p9).write(
                            (((((__p9).read()) as i32).wrapping_add(
                                (((((((&raw const sPointsArray)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<*mut u16>())
                                .cast::<*mut u16>())
                                .wrapping_offset(((caseId) as i32) as isize))
                                .read())
                                .read()) as i32),
                            )) as i16),
                        );
                    }
                    if (crate::c::bf_read(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((defSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(8))
                        .wrapping_add(4),
                        0,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        let mut id: u32 = ((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((defSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(8))
                            .wrapping_add(4),
                            0,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32))
                        .wrapping_mul(4u32);
                        let __p10 = ((((movePoints).cast::<u8>())
                            .wrapping_offset(((defSide) as i32) as isize * 48))
                        .cast::<i16>())
                        .wrapping_offset(
                            (((id).wrapping_add(
                                (crate::c::bf_read(
                                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                        .wrapping_offset(((defSide) as i32) as isize * 16))
                                    .cast::<u8>())
                                    .wrapping_offset(8))
                                    .wrapping_add(3),
                                    6,
                                    2,
                                    false,
                                ) as u32),
                            )) as i32) as isize,
                        );
                        (__p10).write(
                            (((((__p10).read()) as i32).wrapping_add(
                                (((((((&raw const sPointsArray)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<*mut u16>())
                                .cast::<*mut u16>())
                                .wrapping_offset(((caseId) as i32) as isize))
                                .read())
                                .read()) as i32),
                            )) as i16),
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                __fall = true;
                if ((crate::c::bf_read(
                    (((((tvPtr).wrapping_add(48)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 16))
                    .cast::<u8>())
                    .wrapping_add(4),
                    3,
                    3,
                    false,
                ) as u32)
                    != (crate::c::bf_read(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((defSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(8))
                        .wrapping_add(4),
                        3,
                        3,
                        false,
                    ) as u32)
                        .wrapping_neg())
                    && (((arg1) as i32) == 13i32)
                {
                    if (crate::c::bf_read(
                        (((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((defSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_add(4),
                        3,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        let mut id: u32 = ((crate::c::bf_read(
                            (((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((defSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_add(4),
                            3,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32))
                        .wrapping_mul(4u32);
                        let __p11 = ((((movePoints).cast::<u8>())
                            .wrapping_offset(((defSide) as i32) as isize * 48))
                        .cast::<i16>())
                        .wrapping_offset(
                            (((id).wrapping_add(
                                (crate::c::bf_read(
                                    (((((tvPtr).wrapping_add(48)).cast::<u8>())
                                        .wrapping_offset(((defSide) as i32) as isize * 16))
                                    .cast::<u8>())
                                    .wrapping_add(4),
                                    6,
                                    2,
                                    false,
                                ) as u32),
                            )) as i32) as isize,
                        );
                        (__p11).write(
                            (((((__p11).read()) as i32).wrapping_add(
                                (((((((&raw const sPointsArray)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<*mut u16>())
                                .cast::<*mut u16>())
                                .wrapping_offset(((caseId) as i32) as isize))
                                .read())
                                .read()) as i32),
                            )) as i16),
                        );
                    }
                    if (crate::c::bf_read(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((defSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(8))
                        .wrapping_add(4),
                        3,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        let mut id: u32 = ((crate::c::bf_read(
                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                .wrapping_offset(((defSide) as i32) as isize * 16))
                            .cast::<u8>())
                            .wrapping_offset(8))
                            .wrapping_add(4),
                            3,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32))
                        .wrapping_mul(4u32);
                        let __p12 = ((((movePoints).cast::<u8>())
                            .wrapping_offset(((defSide) as i32) as isize * 48))
                        .cast::<i16>())
                        .wrapping_offset(
                            (((id).wrapping_add(
                                (crate::c::bf_read(
                                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                        .wrapping_offset(((defSide) as i32) as isize * 16))
                                    .cast::<u8>())
                                    .wrapping_offset(8))
                                    .wrapping_add(4),
                                    6,
                                    2,
                                    false,
                                ) as u32),
                            )) as i32) as isize,
                        );
                        (__p12).write(
                            (((((__p12).read()) as i32).wrapping_add(
                                (((((((&raw const sPointsArray)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<*mut u16>())
                                .cast::<*mut u16>())
                                .wrapping_offset(((caseId) as i32) as isize))
                                .read())
                                .read()) as i32),
                            )) as i16),
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                __fall = true;
                if ((((arg1) as i32) < 9i32) && (((arg2) as i32) != 0i32))
                    && ((crate::c::bf_read(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((defSide) as i32) as isize * 12))
                        .wrapping_add(0),
                        3,
                        3,
                        false,
                    ) as u32)
                        != 0u32)
                {
                    let mut id: u32 = ((crate::c::bf_read(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((defSide) as i32) as isize * 12))
                        .wrapping_add(0),
                        3,
                        3,
                        false,
                    ) as u32)
                        .wrapping_sub(1u32))
                    .wrapping_mul(4u32);
                    let __p13 = ((((movePoints).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 48))
                    .cast::<i16>())
                    .wrapping_offset(
                        (((id).wrapping_add(
                            (crate::c::bf_read(
                                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                    .wrapping_offset(((defSide) as i32) as isize * 12))
                                .wrapping_add(4),
                                2,
                                2,
                                false,
                            ) as u32),
                        )) as i32) as isize,
                    );
                    (__p13).write(
                        (((((__p13).read()) as i32).wrapping_add(
                            (((((((&raw const sPointsArray)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u16>())
                            .cast::<*mut u16>())
                            .wrapping_offset(((caseId) as i32) as isize))
                            .read())
                            .read()) as i32),
                        )) as i16),
                    );
                }
                break 'l1;
            }
            if __sw1 == 14i32 {
                __fall = true;
                if ((!(((arg1) as i32) < 9i32)) && (((arg2) as i32) != 0i32))
                    && ((crate::c::bf_read(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((defSide) as i32) as isize * 12))
                        .wrapping_add(0),
                        6,
                        3,
                        false,
                    ) as u32)
                        != 0u32)
                {
                    let mut id: u32 = ((crate::c::bf_read(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((defSide) as i32) as isize * 12))
                        .wrapping_add(0),
                        6,
                        3,
                        false,
                    ) as u32)
                        .wrapping_sub(1u32))
                    .wrapping_mul(4u32);
                    let __p14 = ((((movePoints).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 48))
                    .cast::<i16>())
                    .wrapping_offset(
                        (((id).wrapping_add(
                            (crate::c::bf_read(
                                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                    .wrapping_offset(((defSide) as i32) as isize * 12))
                                .wrapping_add(4),
                                4,
                                2,
                                false,
                            ) as u32),
                        )) as i32) as isize,
                    );
                    (__p14).write(
                        (((((__p14).read()) as i32).wrapping_add(
                            (((((((&raw const sPointsArray)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u16>())
                            .cast::<*mut u16>())
                            .wrapping_offset(((caseId) as i32) as isize))
                            .read())
                            .read()) as i32),
                        )) as i16),
                    );
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AddPointsOnFainting(targetFainted: u8) {
    unsafe {
        let mut targetFainted = targetFainted;
        let mut tvPtr: *mut u8 =
            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(516);
        let mut atkSide: u32 =
            ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read())) as u32);
        let mut defSide: u32 =
            ((GetBattlerSide(((&raw mut gBattlerTarget).cast::<u8>()).read())) as u32);
        let mut atkArrId: u32 = (crate::c::bf_read(
            ((((tvPtr).wrapping_add(80)).cast::<u8>())
                .wrapping_offset(((atkSide) as i32) as isize * 12))
            .wrapping_add(7),
            5,
            3,
            false,
        ) as u32);
        let mut i: i32 = 0i32;
        if (crate::c::bf_read(
            ((((tvPtr).wrapping_add(80)).cast::<u8>())
                .wrapping_offset(((atkSide) as i32) as isize * 12))
            .wrapping_add(7),
            1,
            4,
            false,
        ) as u32)
            != 0u32
        {
            'l1: {
                let __sw1 = (crate::c::bf_read(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                    .wrapping_add(7),
                    1,
                    4,
                    false,
                ) as u32);
                if __sw1 == 1u32 {
                    if (crate::c::bf_read(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(((atkArrId) as i32) as isize * 8))
                        .wrapping_add(0),
                        0,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        AddMovePoints(
                            19u8,
                            0u16,
                            ((atkSide ^ 1u32) as u8),
                            (((((crate::c::bf_read(
                                ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 16))
                                .cast::<u8>())
                                .wrapping_offset(((atkArrId) as i32) as isize * 8))
                                .wrapping_add(0),
                                0,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32))
                            .wrapping_mul(4u32))
                            .wrapping_add(
                                (crate::c::bf_read(
                                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                                    .cast::<u8>())
                                    .wrapping_offset(((atkArrId) as i32) as isize * 8))
                                    .wrapping_add(2),
                                    2,
                                    2,
                                    false,
                                ) as u32),
                            )) as u8),
                        );
                    }
                    break 'l1;
                }
                if __sw1 == 2u32 {
                    if (crate::c::bf_read(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(((atkArrId) as i32) as isize * 8))
                        .wrapping_add(0),
                        3,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        AddMovePoints(
                            19u8,
                            0u16,
                            ((atkSide ^ 1u32) as u8),
                            (((((crate::c::bf_read(
                                ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 16))
                                .cast::<u8>())
                                .wrapping_offset(((atkArrId) as i32) as isize * 8))
                                .wrapping_add(0),
                                3,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32))
                            .wrapping_mul(4u32))
                            .wrapping_add(
                                (crate::c::bf_read(
                                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                                    .cast::<u8>())
                                    .wrapping_offset(((atkArrId) as i32) as isize * 8))
                                    .wrapping_add(2),
                                    4,
                                    2,
                                    false,
                                ) as u32),
                            )) as u8),
                        );
                    }
                    break 'l1;
                }
                if __sw1 == 3u32 {
                    if (crate::c::bf_read(
                        (((((tvPtr).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 24))
                        .cast::<u8>())
                        .wrapping_offset(((atkArrId) as i32) as isize * 4))
                        .wrapping_add(0),
                        0,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        AddMovePoints(
                            19u8,
                            0u16,
                            ((atkSide ^ 1u32) as u8),
                            (((((crate::c::bf_read(
                                (((((tvPtr).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 24))
                                .cast::<u8>())
                                .wrapping_offset(((atkArrId) as i32) as isize * 4))
                                .wrapping_add(0),
                                0,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32))
                            .wrapping_mul(4u32))
                            .wrapping_add(
                                (crate::c::bf_read(
                                    (((((tvPtr).cast::<u8>())
                                        .wrapping_offset(((atkSide) as i32) as isize * 24))
                                    .cast::<u8>())
                                    .wrapping_offset(((atkArrId) as i32) as isize * 4))
                                    .wrapping_add(2),
                                    2,
                                    2,
                                    false,
                                ) as u32),
                            )) as u8),
                        );
                    }
                    if (crate::c::bf_read(
                        (((((tvPtr).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 24))
                        .cast::<u8>())
                        .wrapping_offset(((atkArrId) as i32) as isize * 4))
                        .wrapping_add(0),
                        3,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        AddMovePoints(
                            19u8,
                            0u16,
                            ((atkSide ^ 1u32) as u8),
                            (((((crate::c::bf_read(
                                (((((tvPtr).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 24))
                                .cast::<u8>())
                                .wrapping_offset(((atkArrId) as i32) as isize * 4))
                                .wrapping_add(0),
                                3,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32))
                            .wrapping_mul(4u32))
                            .wrapping_add(
                                (crate::c::bf_read(
                                    (((((tvPtr).cast::<u8>())
                                        .wrapping_offset(((atkSide) as i32) as isize * 24))
                                    .cast::<u8>())
                                    .wrapping_offset(((atkArrId) as i32) as isize * 4))
                                    .wrapping_add(2),
                                    4,
                                    2,
                                    false,
                                ) as u32),
                            )) as u8),
                        );
                    }
                    break 'l1;
                }
                if __sw1 == 4u32 {
                    if (crate::c::bf_read(
                        (((((tvPtr).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 24))
                        .cast::<u8>())
                        .wrapping_offset(((atkArrId) as i32) as isize * 4))
                        .wrapping_add(0),
                        6,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        AddMovePoints(
                            19u8,
                            0u16,
                            ((atkSide ^ 1u32) as u8),
                            (((((crate::c::bf_read(
                                (((((tvPtr).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 24))
                                .cast::<u8>())
                                .wrapping_offset(((atkArrId) as i32) as isize * 4))
                                .wrapping_add(0),
                                6,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32))
                            .wrapping_mul(4u32))
                            .wrapping_add(
                                (crate::c::bf_read(
                                    (((((tvPtr).cast::<u8>())
                                        .wrapping_offset(((atkSide) as i32) as isize * 24))
                                    .cast::<u8>())
                                    .wrapping_offset(((atkArrId) as i32) as isize * 4))
                                    .wrapping_add(2),
                                    6,
                                    2,
                                    false,
                                ) as u32),
                            )) as u8),
                        );
                    }
                    break 'l1;
                }
                if __sw1 == 5u32 {
                    if (crate::c::bf_read(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(((atkArrId) as i32) as isize * 8))
                        .wrapping_add(0),
                        6,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        AddMovePoints(
                            19u8,
                            0u16,
                            ((atkSide ^ 1u32) as u8),
                            (((((crate::c::bf_read(
                                ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 16))
                                .cast::<u8>())
                                .wrapping_offset(((atkArrId) as i32) as isize * 8))
                                .wrapping_add(0),
                                6,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32))
                            .wrapping_mul(4u32))
                            .wrapping_add(
                                (crate::c::bf_read(
                                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                                    .cast::<u8>())
                                    .wrapping_offset(((atkArrId) as i32) as isize * 8))
                                    .wrapping_add(2),
                                    6,
                                    2,
                                    false,
                                ) as u32),
                            )) as u8),
                        );
                    }
                    break 'l1;
                }
                if __sw1 == 6u32 {
                    if (crate::c::bf_read(
                        ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 16))
                        .cast::<u8>())
                        .wrapping_offset(((atkArrId) as i32) as isize * 8))
                        .wrapping_add(1),
                        1,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        AddMovePoints(
                            19u8,
                            0u16,
                            ((atkSide ^ 1u32) as u8),
                            (((((crate::c::bf_read(
                                ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 16))
                                .cast::<u8>())
                                .wrapping_offset(((atkArrId) as i32) as isize * 8))
                                .wrapping_add(1),
                                1,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32))
                            .wrapping_mul(4u32))
                            .wrapping_add(
                                (crate::c::bf_read(
                                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                                    .cast::<u8>())
                                    .wrapping_offset(((atkArrId) as i32) as isize * 8))
                                    .wrapping_add(3),
                                    0,
                                    2,
                                    false,
                                ) as u32),
                            )) as u8),
                        );
                    }
                    break 'l1;
                }
                if __sw1 == 7u32 {
                    if (crate::c::bf_read(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(0),
                        0,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        AddMovePoints(
                            19u8,
                            0u16,
                            ((atkSide ^ 1u32) as u8),
                            (((((crate::c::bf_read(
                                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 12))
                                .wrapping_add(0),
                                0,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32))
                            .wrapping_mul(4u32))
                            .wrapping_add(
                                (crate::c::bf_read(
                                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                                    .wrapping_add(4),
                                    0,
                                    2,
                                    false,
                                ) as u32),
                            )) as u8),
                        );
                    }
                    break 'l1;
                }
                if __sw1 == 8u32 {
                    if (crate::c::bf_read(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(1),
                        7,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        AddMovePoints(
                            20u8,
                            0u16,
                            ((atkSide) as u8),
                            (((((crate::c::bf_read(
                                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 12))
                                .wrapping_add(1),
                                7,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32))
                            .wrapping_mul(4u32))
                            .wrapping_add(
                                (crate::c::bf_read(
                                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                                    .wrapping_add(5),
                                    2,
                                    2,
                                    false,
                                ) as u32),
                            )) as u8),
                        );
                    }
                    break 'l1;
                }
                if __sw1 == 9u32 {
                    if (crate::c::bf_read(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(2),
                        2,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        AddMovePoints(
                            20u8,
                            0u16,
                            ((atkSide) as u8),
                            (((((crate::c::bf_read(
                                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 12))
                                .wrapping_add(2),
                                2,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32))
                            .wrapping_mul(4u32))
                            .wrapping_add(
                                (crate::c::bf_read(
                                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                                    .wrapping_add(5),
                                    4,
                                    2,
                                    false,
                                ) as u32),
                            )) as u8),
                        );
                    }
                    break 'l1;
                }
                if __sw1 == 10u32 {
                    if ((crate::c::bf_read(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(8),
                        6,
                        1,
                        false,
                    ) as u32)
                        != 0)
                        && ((crate::c::bf_read(
                            ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                .wrapping_offset(((atkSide) as i32) as isize * 12))
                            .wrapping_add(2),
                            5,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32)
                            != ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                            .read()) as u32))
                    {
                        AddMovePoints(
                            19u8,
                            0u16,
                            ((atkSide) as u8),
                            (((((crate::c::bf_read(
                                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 12))
                                .wrapping_add(2),
                                5,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32))
                            .wrapping_mul(4u32))
                            .wrapping_add(
                                (crate::c::bf_read(
                                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                                    .wrapping_add(5),
                                    6,
                                    2,
                                    false,
                                ) as u32),
                            )) as u8),
                        );
                    }
                    if (crate::c::bf_read(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide ^ 1u32) as i32) as isize * 12))
                        .wrapping_add(8),
                        6,
                        1,
                        false,
                    ) as u32)
                        != 0
                    {
                        AddMovePoints(
                            19u8,
                            0u16,
                            ((atkSide ^ 1u32) as u8),
                            (((((crate::c::bf_read(
                                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                    .wrapping_offset(((atkSide ^ 1u32) as i32) as isize * 12))
                                .wrapping_add(2),
                                5,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32))
                            .wrapping_mul(4u32))
                            .wrapping_add(
                                (crate::c::bf_read(
                                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                        .wrapping_offset(((atkSide ^ 1u32) as i32) as isize * 12))
                                    .wrapping_add(5),
                                    6,
                                    2,
                                    false,
                                ) as u32),
                            )) as u8),
                        );
                    }
                    break 'l1;
                }
                if __sw1 == 11u32 {
                    if (crate::c::bf_read(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide ^ 1u32) as i32) as isize * 12))
                        .wrapping_add(6),
                        4,
                        3,
                        false,
                    ) as u32)
                        != 0u32
                    {
                        AddMovePoints(
                            19u8,
                            0u16,
                            ((atkSide ^ 1u32) as u8),
                            (((((crate::c::bf_read(
                                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                    .wrapping_offset(((atkSide ^ 1u32) as i32) as isize * 12))
                                .wrapping_add(6),
                                4,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32))
                            .wrapping_mul(4u32))
                            .wrapping_add(
                                (crate::c::bf_read(
                                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                        .wrapping_offset(((atkSide ^ 1u32) as i32) as isize * 12))
                                    .wrapping_add(6),
                                    7,
                                    2,
                                    false,
                                ) as u32),
                            )) as u8),
                        );
                    }
                    break 'l1;
                }
                if __sw1 == 12u32 {
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < 2i32) {
                                break 'l2;
                            }
                            'l3: {
                                if (crate::c::bf_read(
                                    ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                        .wrapping_offset(((atkSide) as i32) as isize * 16))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 8))
                                    .wrapping_add(1),
                                    7,
                                    3,
                                    false,
                                ) as u32)
                                    != 0u32
                                {
                                    AddMovePoints(
                                        19u8,
                                        0u16,
                                        ((atkSide ^ 1u32) as u8),
                                        (((((crate::c::bf_read(
                                            ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                                .wrapping_offset(
                                                    ((atkSide) as i32) as isize * 16,
                                                ))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 8))
                                            .wrapping_add(1),
                                            7,
                                            3,
                                            false,
                                        ) as u32)
                                            .wrapping_sub(1u32))
                                        .wrapping_mul(4u32))
                                        .wrapping_add(
                                            (crate::c::bf_read(
                                                ((((((tvPtr).wrapping_add(48)).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((atkSide) as i32) as isize * 16,
                                                    ))
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize * 8))
                                                .wrapping_add(3),
                                                4,
                                                2,
                                                false,
                                            ) as u32),
                                        )) as u8),
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 13u32 {
                    if (crate::c::bf_read(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                        .wrapping_add(8),
                        0,
                        1,
                        false,
                    ) as u32)
                        != 0
                    {
                        AddMovePoints(
                            19u8,
                            0u16,
                            ((atkSide) as u8),
                            (((((crate::c::bf_read(
                                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 12))
                                .wrapping_add(8),
                                3,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32))
                            .wrapping_mul(4u32))
                            .wrapping_add(
                                (crate::c::bf_read(
                                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                        .wrapping_offset(((atkSide) as i32) as isize * 12))
                                    .wrapping_add(8),
                                    1,
                                    2,
                                    false,
                                ) as u32),
                            )) as u8),
                        );
                    }
                    if (crate::c::bf_read(
                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                            .wrapping_offset(((atkSide ^ 1u32) as i32) as isize * 12))
                        .wrapping_add(8),
                        0,
                        1,
                        false,
                    ) as u32)
                        != 0
                    {
                        AddMovePoints(
                            19u8,
                            0u16,
                            ((atkSide ^ 1u32) as u8),
                            (((((crate::c::bf_read(
                                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                    .wrapping_offset(((atkSide ^ 1u32) as i32) as isize * 12))
                                .wrapping_add(8),
                                3,
                                3,
                                false,
                            ) as u32)
                                .wrapping_sub(1u32))
                            .wrapping_mul(4u32))
                            .wrapping_add(
                                (crate::c::bf_read(
                                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                        .wrapping_offset(((atkSide ^ 1u32) as i32) as isize * 12))
                                    .wrapping_add(8),
                                    1,
                                    2,
                                    false,
                                ) as u32),
                            )) as u8),
                        );
                    }
                    break 'l1;
                }
                if __sw1 == 14u32 {
                    if ((targetFainted) as i32) == 1i32 {
                        AddMovePoints(
                            20u8,
                            0u16,
                            ((atkSide) as u8),
                            ((((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                            .read()) as i32)
                                .wrapping_mul(4i32)) as u32)
                                .wrapping_add(
                                    (crate::c::bf_read(
                                        ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                            .wrapping_offset(((atkSide) as i32) as isize * 12))
                                        .wrapping_add(3),
                                        6,
                                        2,
                                        false,
                                    ) as u32),
                                )) as u8),
                        );
                    }
                    break 'l1;
                }
                if __sw1 == 15u32 {
                    break 'l1;
                }
            }
        } else {
            if (crate::c::bf_read(
                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                    .wrapping_offset(((defSide) as i32) as isize * 12))
                .wrapping_add(7),
                1,
                4,
                false,
            ) as u32)
                == 7u32
            {
                if (crate::c::bf_read(
                    ((((tvPtr).wrapping_add(80)).cast::<u8>())
                        .wrapping_offset(((defSide) as i32) as isize * 12))
                    .wrapping_add(0),
                    0,
                    3,
                    false,
                ) as u32)
                    != 0u32
                {
                    AddMovePoints(
                        19u8,
                        0u16,
                        ((defSide ^ 1u32) as u8),
                        (((((crate::c::bf_read(
                            ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                .wrapping_offset(((defSide) as i32) as isize * 12))
                            .wrapping_add(0),
                            0,
                            3,
                            false,
                        ) as u32)
                            .wrapping_sub(1u32))
                        .wrapping_mul(4u32))
                        .wrapping_add(
                            (crate::c::bf_read(
                                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                    .wrapping_offset(((defSide) as i32) as isize * 12))
                                .wrapping_add(4),
                                0,
                                2,
                                false,
                            ) as u32),
                        )) as u8),
                    );
                }
            } else {
                AddMovePoints(
                    20u8,
                    0u16,
                    ((atkSide) as u8),
                    ((((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_mul(4i32)) as u32)
                        .wrapping_add(
                            (crate::c::bf_read(
                                ((((tvPtr).wrapping_add(80)).cast::<u8>())
                                    .wrapping_offset(((atkSide) as i32) as isize * 12))
                                .wrapping_add(3),
                                6,
                                2,
                                false,
                            ) as u32),
                        )) as u8),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TrySetBattleSeminarShow() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut dmgByMove = crate::ffi::Align4([0u8; 16]);
        let mut powerOverride: u16 = 0u16;
        let mut currMoveSaved: u16 = 0u16;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554435u32) != 0 {
            return;
        } else {
            if ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read())) as i32) == 1i32
            {
                return;
            } else {
                if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(24))
                .cast::<i8>())
                .wrapping_offset(6))
                .read()) as i32)
                    < 6i32
                {
                    return;
                } else {
                    if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize * 88,
                    ))
                    .wrapping_add(24))
                    .cast::<i8>())
                    .wrapping_offset(7))
                    .read()) as i32)
                        > 6i32
                    {
                        return;
                    } else {
                        if (((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) == 237i32)
                            || (((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) == 311i32)
                        {
                            return;
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 3276800u32)
                                != 0
                            {
                                return;
                            } else {
                                if ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                    .wrapping_add(12))
                                    .cast::<u16>())
                                    .wrapping_offset(
                                        (((((&raw mut gMoveSelectionCursor).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                    as i32)
                                                    as isize,
                                            ))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 12,
                                ))
                                .wrapping_add(1))
                                .read()) as i32)
                                    == 0i32
                                {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        i = 0i32;
        currMoveSaved = ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(12))
        .cast::<u16>())
        .wrapping_offset(
            (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize,
        ))
        .read();
        'l1: loop {
            'l2: {
                if ((currMoveSaved) as i32)
                    == ((((((&raw const sVariableDmgMoves)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                {
                    break 'l1;
                }
                i = (i).wrapping_add(1);
            }
            if !(((((((&raw const sVariableDmgMoves)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset((i) as isize))
            .read()) as i32)
                != 65535i32)
            {
                break 'l1;
            }
        }
        if ((((((&raw const sVariableDmgMoves)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset((i) as isize))
        .read()) as i32)
            != 65535i32
        {
            return;
        }
        (((&raw mut dmgByMove).cast::<i32>()).wrapping_offset(
            (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize,
        ))
        .write(((&raw mut gBattleMoveDamage).cast::<i32>()).read());
        currMoveSaved = ((&raw mut gCurrentMove).cast::<u16>()).read();
        {
            i = 0i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    ((&raw mut gCurrentMove).cast::<u16>()).write(
                        ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(12))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                    powerOverride = 0u16;
                    if (ShouldCalculateDamage(
                        ((&raw mut gCurrentMove).cast::<u16>()).read(),
                        ((&raw mut dmgByMove).cast::<i32>()).wrapping_offset((i) as isize),
                        &raw mut powerOverride,
                    )) != 0
                    {
                        let mut moveResultFlags: u8 = 0u8;
                        let mut sideStatus: u16 = ((((&raw mut gSideStatuses).cast::<u16>())
                            .cast::<u16>())
                        .wrapping_offset(
                            (((GetBattlerPosition(((&raw mut gBattlerTarget).cast::<u8>()).read()))
                                as i32)
                                & 1i32) as isize,
                        ))
                        .read();
                        ((&raw mut gBattleMoveDamage).cast::<i32>()).write(CalculateBaseDamage(
                            ((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ),
                            ((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ),
                            ((((&raw mut gCurrentMove).cast::<u16>()).read()) as u32),
                            sideStatus,
                            powerOverride,
                            0u8,
                            ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                            ((&raw mut gBattlerTarget).cast::<u8>()).read(),
                        ));
                        if ((((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                            .wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                        .read()
                            & 512u32)
                            != 0)
                            && (((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) as isize
                                    * 12,
                            ))
                            .wrapping_add(2))
                            .read()) as i32)
                                == 13i32)
                        {
                            let __p1 = (&raw mut gBattleMoveDamage).cast::<i32>();
                            (__p1).write(((__p1).read()).wrapping_mul(2i32));
                        }
                        if (crate::c::bf_read(
                            (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 16,
                            ))
                            .wrapping_add(0),
                            3,
                            1,
                            false,
                        ) as u32)
                            != 0
                        {
                            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(crate::c::div_i32(
                                (((&raw mut gBattleMoveDamage).cast::<i32>()).read())
                                    .wrapping_mul(15i32),
                                10i32,
                            ));
                        }
                        moveResultFlags = TypeCalc(
                            ((&raw mut gCurrentMove).cast::<u16>()).read(),
                            ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                            ((&raw mut gBattlerTarget).cast::<u8>()).read(),
                        );
                        (((&raw mut dmgByMove).cast::<i32>()).wrapping_offset((i) as isize))
                            .write(((&raw mut gBattleMoveDamage).cast::<i32>()).read());
                        if ((((&raw mut dmgByMove).cast::<i32>()).wrapping_offset((i) as isize))
                            .read()
                            == 0i32)
                            && (!((((moveResultFlags) as i32) & 41i32) != 0))
                        {
                            (((&raw mut dmgByMove).cast::<i32>()).wrapping_offset((i) as isize))
                                .write(1i32);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < 4i32) {
                    break 'l5;
                }
                'l6: {
                    if (i
                        != (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read()) as i32))
                        && ((((&raw mut dmgByMove).cast::<i32>()).wrapping_offset((i) as isize))
                            .read()
                            > (((&raw mut dmgByMove).cast::<i32>()).wrapping_offset(
                                (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                                .read()) as i32) as isize,
                            ))
                            .read())
                    {
                        let mut opponentSpecies: u16 = 0u16;
                        let mut playerSpecies: u16 = 0u16;
                        let mut bestMoveId: i32 = 0i32;
                        if (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read()) as i32)
                            != 0i32
                        {
                            bestMoveId = 0i32;
                        } else {
                            bestMoveId = 1i32;
                        }
                        {
                            i = 0i32;
                            'l7: loop {
                                if !(i < 4i32) {
                                    break 'l7;
                                }
                                'l8: {
                                    if (i
                                        != (((((&raw mut gMoveSelectionCursor).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                    as i32)
                                                    as isize,
                                            ))
                                        .read())
                                            as i32))
                                        && ((((&raw mut dmgByMove).cast::<i32>())
                                            .wrapping_offset((i) as isize))
                                        .read()
                                            > (((&raw mut dmgByMove).cast::<i32>())
                                                .wrapping_offset((bestMoveId) as isize))
                                            .read())
                                    {
                                        bestMoveId = i;
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        opponentSpecies = ((GetMonData3(
                            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            11i32,
                            core::ptr::null_mut(),
                        )) as u16);
                        playerSpecies = ((GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            11i32,
                            core::ptr::null_mut(),
                        )) as u16);
                        TryPutBattleSeminarOnAir(
                            opponentSpecies,
                            playerSpecies,
                            (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                            .read(),
                            ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(12))
                            .cast::<u16>(),
                            ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(12))
                            .cast::<u16>())
                            .wrapping_offset((bestMoveId) as isize))
                            .read(),
                        );
                        break 'l5;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
            (((&raw mut dmgByMove).cast::<i32>()).wrapping_offset(
                (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize,
            ))
            .read(),
        );
        ((&raw mut gCurrentMove).cast::<u16>()).write(currMoveSaved);
    }
}
pub(crate) unsafe extern "C" fn ShouldCalculateDamage(
    r#move: u16,
    dmg: *mut i32,
    powerOverride: *mut u16,
) -> u8 {
    unsafe {
        let mut r#move = r#move;
        let mut dmg = dmg;
        let mut powerOverride = powerOverride;
        if ((((((&raw mut gBattleMoves).cast::<u8>())
            .wrapping_offset(((r#move) as i32) as isize * 12))
        .wrapping_add(1))
        .read()) as i32)
            == 0i32
        {
            (dmg).write(0i32);
            return 0u8;
        } else {
            let mut i: i32 = 0i32;
            'l1: loop {
                'l2: {
                    if ((r#move) as i32)
                        == ((((((&raw const sVariableDmgMoves)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                    {
                        break 'l1;
                    }
                    i = (i).wrapping_add(1);
                }
                if !(((((((&raw const sVariableDmgMoves)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset((i) as isize))
                .read()) as i32)
                    != 65535i32)
                {
                    break 'l1;
                }
            }
            if ((((((&raw const sVariableDmgMoves)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset((i) as isize))
            .read()) as i32)
                != 65535i32
            {
                (dmg).write(0i32);
                return 0u8;
            } else {
                if ((r#move) as i32) == 149i32 {
                    (dmg).write(
                        ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(42))
                        .read()) as i32),
                    );
                    (dmg).write(crate::c::div_i32((dmg).read(), 2i32));
                    return 0u8;
                } else {
                    if ((r#move) as i32) == 222i32 {
                        (powerOverride).write(10u16);
                        return 1u8;
                    } else {
                        return 1u8;
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleTv_ClearExplosionFaintCause() {
    unsafe {
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0 {
            let mut tvPtr: *mut u8 =
                (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(516);
            crate::c::bf_write(
                (((tvPtr).wrapping_add(80)).cast::<u8>()).wrapping_add(7),
                1,
                4,
                (0u32) as i32,
            );
            crate::c::bf_write(
                ((((tvPtr).wrapping_add(80)).cast::<u8>()).wrapping_offset(12)).wrapping_add(7),
                1,
                4,
                (0u32) as i32,
            );
            crate::c::bf_write(
                (((tvPtr).wrapping_add(80)).cast::<u8>()).wrapping_add(7),
                5,
                3,
                (0u32) as i32,
            );
            crate::c::bf_write(
                ((((tvPtr).wrapping_add(80)).cast::<u8>()).wrapping_offset(12)).wrapping_add(7),
                5,
                3,
                (0u32) as i32,
            );
            crate::c::bf_write(
                (((tvPtr).wrapping_add(80)).cast::<u8>()).wrapping_add(8),
                3,
                3,
                (0u32) as i32,
            );
            crate::c::bf_write(
                ((((tvPtr).wrapping_add(80)).cast::<u8>()).wrapping_offset(12)).wrapping_add(8),
                3,
                3,
                (0u32) as i32,
            );
            crate::c::bf_write(
                (((tvPtr).wrapping_add(80)).cast::<u8>()).wrapping_add(8),
                1,
                2,
                (0u32) as i32,
            );
            crate::c::bf_write(
                ((((tvPtr).wrapping_add(80)).cast::<u8>()).wrapping_offset(12)).wrapping_add(8),
                1,
                2,
                (0u32) as i32,
            );
            crate::c::bf_write(
                (((tvPtr).wrapping_add(80)).cast::<u8>()).wrapping_add(8),
                0,
                1,
                (0u32) as i32,
            );
            crate::c::bf_write(
                ((((tvPtr).wrapping_add(80)).cast::<u8>()).wrapping_offset(12)).wrapping_add(8),
                0,
                1,
                (0u32) as i32,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerMoveSlotId(battler: u8, r#move: u16) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut r#move = r#move;
        let mut i: i32 = 0i32;
        let mut party: *mut u8 = core::ptr::null_mut();
        if ((GetBattlerSide(battler)) as i32) == 0i32 {
            party = (&raw mut gPlayerParty).cast::<u8>();
        } else {
            party = (&raw mut gEnemyParty).cast::<u8>();
        }
        i = 0i32;
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            if i >= 4i32 {
                break 'l1;
            }
            if GetMonData3(
                (party).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 100,
                ),
                (13i32).wrapping_add(i),
                core::ptr::null_mut(),
            ) == ((r#move) as u32)
            {
                break 'l1;
            }
            i = (i).wrapping_add(1);
        }
        return ((i) as u8);
    }
}
pub(crate) unsafe extern "C" fn AddPointsBasedOnWeather(
    weatherFlags: u16,
    r#move: u16,
    moveSlot: u8,
) {
    unsafe {
        let mut weatherFlags = weatherFlags;
        let mut r#move = r#move;
        let mut moveSlot = moveSlot;
        if (((weatherFlags) as i32) & 7i32) != 0 {
            AddMovePoints(3u8, r#move, moveSlot, 0u8);
        } else {
            if (((weatherFlags) as i32) & 96i32) != 0 {
                AddMovePoints(4u8, r#move, moveSlot, 0u8);
            } else {
                if (((weatherFlags) as i32) & 24i32) != 0 {
                    AddMovePoints(5u8, r#move, moveSlot, 0u8);
                } else {
                    if (((weatherFlags) as i32) & 128i32) != 0 {
                        AddMovePoints(6u8, r#move, moveSlot, 0u8);
                    }
                }
            }
        }
    }
}
