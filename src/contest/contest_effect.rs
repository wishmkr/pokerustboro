//! Translated from `src/contest_effect.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gContestMoves gContestEffects gComboStarterLookupTable gContestEffectFuncs
#[allow(unused_imports)]
use crate::data::contest_effect::*;

unsafe extern "C" {
    static mut gContestResources: u8;
    static mut gContestantTurnOrder: u8;
    static mut gSpecialVar_ContestCategory: u8;
    fn Contest_IsMonsTurnDisabled(a0: u8) -> u8;
    fn IsContestantAllowedToCombo(a0: u8) -> u8;
    fn MakeContestantNervous(a0: u8);
    fn Random() -> u16;
    fn SetContestantEffectStringID(a0: u8, a1: u8);
    fn SetContestantEffectStringID2(a0: u8, a1: u8);
    fn SetStartledString(a0: u8, a1: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AreMovesContestCombo(lastMove: u16, nextMove: u16) -> u8 {
    unsafe {
        let mut lastMove = lastMove;
        let mut nextMove = nextMove;
        let mut nextMoveComboMoves = crate::ffi::Align4([0u8; 4]);
        let mut lastMoveComboStarterId: u8 =
            (((((&raw const gContestMoves).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((lastMove) as i32) as isize * 8))
            .wrapping_add(2))
            .read();
        ((&raw mut nextMoveComboMoves).cast::<u8>()).write(
            ((((((&raw const gContestMoves).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((nextMove) as i32) as isize * 8))
            .wrapping_add(3))
            .cast::<u8>())
            .read(),
        );
        (((&raw mut nextMoveComboMoves).cast::<u8>()).wrapping_offset(1)).write(
            (((((((&raw const gContestMoves).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((nextMove) as i32) as isize * 8))
            .wrapping_add(3))
            .cast::<u8>())
            .wrapping_offset(1))
            .read(),
        );
        (((&raw mut nextMoveComboMoves).cast::<u8>()).wrapping_offset(2)).write(
            (((((((&raw const gContestMoves).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((nextMove) as i32) as isize * 8))
            .wrapping_add(3))
            .cast::<u8>())
            .wrapping_offset(2))
            .read(),
        );
        (((&raw mut nextMoveComboMoves).cast::<u8>()).wrapping_offset(3)).write(
            (((((((&raw const gContestMoves).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((nextMove) as i32) as isize * 8))
            .wrapping_add(3))
            .cast::<u8>())
            .wrapping_offset(3))
            .read(),
        );
        if ((lastMoveComboStarterId) as i32) == 0i32 {
            return 0u8;
        } else {
            if (((((lastMoveComboStarterId) as i32)
                == ((((&raw mut nextMoveComboMoves).cast::<u8>()).read()) as i32))
                || (((lastMoveComboStarterId) as i32)
                    == (((((&raw mut nextMoveComboMoves).cast::<u8>()).wrapping_offset(1)).read())
                        as i32)))
                || (((lastMoveComboStarterId) as i32)
                    == (((((&raw mut nextMoveComboMoves).cast::<u8>()).wrapping_offset(2)).read())
                        as i32)))
                || (((lastMoveComboStarterId) as i32)
                    == (((((&raw mut nextMoveComboMoves).cast::<u8>()).wrapping_offset(3)).read())
                        as i32))
            {
                return ((((&raw const gComboStarterLookupTable)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((lastMoveComboStarterId) as i32) as isize))
                .read();
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
pub(crate) unsafe extern "C" fn ContestEffect_HighlyAppealing() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn ContestEffect_UserMoreEasilyStartled() {
    unsafe {
        crate::c::bf_write(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(16),
            2,
            1,
            (1u8) as i32,
        );
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_GreatAppealButNoMoreMoves() {
    unsafe {
        crate::c::bf_write(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(17),
            3,
            1,
            (1u8) as i32,
        );
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            1u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_RepetitionNotBoring() {
    unsafe {
        crate::c::bf_write(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(16),
            3,
            1,
            (1u8) as i32,
        );
        crate::c::bf_write(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(21),
            0,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(11),
            4,
            3,
            (0u8) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_AvoidStartleOnce() {
    unsafe {
        (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize
                * 28,
        ))
        .wrapping_add(18))
        .write(1u8);
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            2u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_AvoidStartle() {
    unsafe {
        crate::c::bf_write(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(16),
            1,
            1,
            (1u8) as i32,
        );
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            3u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_AvoidStartleSlightly() {
    unsafe {
        (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize
                * 28,
        ))
        .wrapping_add(15))
        .write(20u8);
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            4u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_UserLessEasilyStartled() {
    unsafe {
        crate::c::bf_write(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(16),
            0,
            1,
            (1u8) as i32,
        );
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            5u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_StartleFrontMon() {
    unsafe {
        let mut idx: u8 = 0u8;
        let mut a: u8 = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(17))
        .read();
        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(((a) as i32) as isize))
        .read()) as i32)
            != 0i32
        {
            let mut i: i32 = 0i32;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset(((a) as i32) as isize))
                        .read()) as i32)
                            .wrapping_sub(1i32)
                            == (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                        {
                            break 'l1;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8))
            .cast::<u8>())
            .write(((i) as u8));
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8))
            .cast::<u8>())
            .wrapping_offset(1))
            .write(255u8);
            idx = WasAtLeastOneOpponentJammed();
        }
        if ((idx) as i32) == 0i32 {
            SetContestantEffectStringID2(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                54u8,
            );
        }
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            48u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_StartlePrevMons() {
    unsafe {
        let mut idx: u8 = 0u8;
        let mut contestant: u8 = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(17))
        .read();
        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(((contestant) as i32) as isize))
        .read()) as i32)
            != 0i32
        {
            let mut i: i32 = 0i32;
            let mut j: i32 = 0i32;
            {
                i = 0i32;
                j = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset(((contestant) as i32) as isize))
                        .read()) as i32)
                            > (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                        {
                            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(
                                ({
                                    let __t1 = j;
                                    j = (j).wrapping_add(1);
                                    __t1
                                }) as isize,
                            ))
                            .write(((i) as u8));
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8))
            .cast::<u8>())
            .wrapping_offset((j) as isize))
            .write(255u8);
            idx = WasAtLeastOneOpponentJammed();
        }
        if ((idx) as i32) == 0i32 {
            SetContestantEffectStringID2(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                54u8,
            );
        }
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            48u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_StartlePrevMon2() {
    unsafe {
        let mut rval: u8 = ((crate::c::rem_i32(((Random()) as i32), 10i32)) as u8);
        let mut jam: i32 = 0i32;
        if ((rval) as i32) < 2i32 {
            jam = 20i32;
        } else {
            if ((rval) as i32) < 8i32 {
                jam = 40i32;
            } else {
                jam = 60i32;
            }
        }
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<i16>())
        .write(((jam) as i16));
        ContestEffect_StartleFrontMon();
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_StartlePrevMons2() {
    unsafe {
        let mut numStartled: u8 = 0u8;
        let mut contestant: u8 = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(17))
        .read();
        let mut turnOrder: u8 = (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(((contestant) as i32) as isize))
        .read();
        if ((turnOrder) as i32) != 0i32 {
            let mut i: i32 = 0i32;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset(((contestant) as i32) as isize))
                        .read()) as i32)
                            > (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                        {
                            let mut rval: u8 = 0u8;
                            let mut jam: u8 = 0u8;
                            (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(8))
                            .cast::<u8>())
                            .write(((i) as u8));
                            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .write(255u8);
                            rval = ((crate::c::rem_i32(((Random()) as i32), 10i32)) as u8);
                            if ((rval) as i32) == 0i32 {
                                jam = 0u8;
                            } else {
                                if ((rval) as i32) <= 2i32 {
                                    jam = 10u8;
                                } else {
                                    if ((rval) as i32) <= 4i32 {
                                        jam = 20u8;
                                    } else {
                                        if ((rval) as i32) <= 6i32 {
                                            jam = 30u8;
                                        } else {
                                            if ((rval) as i32) <= 8i32 {
                                                jam = 40u8;
                                            } else {
                                                jam = 60u8;
                                            }
                                        }
                                    }
                                }
                            }
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<i16>())
                            .write(((jam) as i16));
                            if (WasAtLeastOneOpponentJammed()) != 0 {
                                numStartled = (numStartled).wrapping_add(1);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            48u8,
        );
        if ((numStartled) as i32) == 0i32 {
            SetContestantEffectStringID2(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                54u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_ShiftJudgeAttention() {
    unsafe {
        let mut hitAny: u32 = 0u32;
        let mut contestant: u8 = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(17))
        .read();
        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            != 0i32
        {
            let mut i: i32 = 0i32;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset(((contestant) as i32) as isize))
                        .read()) as i32)
                            > (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32))
                            && ((crate::c::bf_read(
                                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 28))
                                .wrapping_add(21),
                                4,
                                1,
                                false,
                            ) as u8)
                                != 0))
                            && ((CanUnnerveContestant(((i) as u8))) != 0)
                        {
                            crate::c::bf_write(
                                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 28))
                                .wrapping_add(21),
                                4,
                                1,
                                (0u8) as i32,
                            );
                            crate::c::bf_write(
                                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 28))
                                .wrapping_add(21),
                                5,
                                1,
                                (1u8) as i32,
                            );
                            SetContestantEffectStringID(((i) as u8), 8u8);
                            hitAny = 1u32;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            7u8,
        );
        if !((hitAny) != 0) {
            SetContestantEffectStringID2(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                54u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_StartleMonWithJudgesAttention() {
    unsafe {
        let mut numStartled: u8 = 0u8;
        let mut contestant: u8 = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(17))
        .read();
        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            != 0i32
        {
            let mut i: i32 = 0i32;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset(((contestant) as i32) as isize))
                        .read()) as i32)
                            > (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                        {
                            if (crate::c::bf_read(
                                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 28))
                                .wrapping_add(21),
                                4,
                                1,
                                false,
                            ) as u8)
                                != 0
                            {
                                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(8)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<i16>())
                                .write(50i16);
                            } else {
                                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(8)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<i16>())
                                .write(10i16);
                            }
                            (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(8))
                            .cast::<u8>())
                            .write(((i) as u8));
                            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .write(255u8);
                            if (WasAtLeastOneOpponentJammed()) != 0 {
                                numStartled = (numStartled).wrapping_add(1);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            48u8,
        );
        if ((numStartled) as i32) == 0i32 {
            SetContestantEffectStringID2(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                54u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_JamsOthersButMissOneTurn() {
    unsafe {
        crate::c::bf_write(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(17),
            2,
            1,
            (1u8) as i32,
        );
        ContestEffect_StartlePrevMons();
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            48u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_StartleMonsSameTypeAppeal() {
    unsafe {
        let mut r#move: u16 = (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize
                * 28,
        ))
        .wrapping_add(6)
        .cast::<u16>())
        .read();
        JamByMoveCategory(
            (crate::c::bf_read(
                ((((&raw const gContestMoves).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((r#move) as i32) as isize * 8))
                .wrapping_add(1),
                0,
                3,
                false,
            ) as u8),
        );
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            48u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_StartleMonsCoolAppeal() {
    unsafe {
        JamByMoveCategory(0u8);
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            48u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_StartleMonsBeautyAppeal() {
    unsafe {
        JamByMoveCategory(1u8);
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            48u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_StartleMonsCuteAppeal() {
    unsafe {
        JamByMoveCategory(2u8);
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            48u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_StartleMonsSmartAppeal() {
    unsafe {
        JamByMoveCategory(3u8);
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            48u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_StartleMonsToughAppeal() {
    unsafe {
        JamByMoveCategory(4u8);
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            48u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_MakeFollowingMonNervous() {
    unsafe {
        let mut hitAny: u32 = 0u32;
        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            != 3i32
        {
            let mut i: i32 = 0i32;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(17))
                            .read()) as i32) as isize,
                        ))
                        .read()) as i32)
                            .wrapping_add(1i32)
                            == (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                        {
                            if (CanUnnerveContestant(((i) as u8))) != 0 {
                                MakeContestantNervous(((i) as u8));
                                SetContestantEffectStringID(((i) as u8), 10u8);
                                hitAny = 1u32;
                            } else {
                                SetContestantEffectStringID(((i) as u8), 60u8);
                                hitAny = 1u32;
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            9u8,
        );
        if !((hitAny) != 0) {
            SetContestantEffectStringID2(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                54u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_MakeFollowingMonsNervous() {
    unsafe {
        let mut numUnnerved: u8 = 0u8;
        let mut contestantUnnerved: u32 = 0u32;
        let mut contestantIds = crate::ffi::Align4([0u8; 5]);
        let mut i: i32 = 0i32;
        let mut numAfter: i32 = 0i32;
        let mut oddsMod = crate::ffi::Align4([0u8; 8]);
        let mut odds = crate::ffi::Align4([0u8; 8]);
        crate::c::memset(
            (&raw mut contestantIds).cast::<u8>(),
            255i32,
            crate::c::div_u32(5u32, 1u32),
        );
        {
            i = 0i32;
            numAfter = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        < (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32))
                        && (!((crate::c::bf_read(
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 28))
                            .wrapping_add(12),
                            0,
                            1,
                            false,
                        ) as u8)
                            != 0)))
                        && (!((Contest_IsMonsTurnDisabled(((i) as u8))) != 0))
                    {
                        (((&raw mut contestantIds).cast::<u8>()).wrapping_offset(
                            ({
                                let __t1 = numAfter;
                                numAfter = (numAfter).wrapping_add(1);
                                __t1
                            }) as isize,
                        ))
                        .write(((i) as u8));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if numAfter == 1i32 {
            ((&raw mut odds).cast::<i16>()).write(60i16);
        } else {
            if numAfter == 2i32 {
                ((&raw mut odds).cast::<i16>()).write(30i16);
                (((&raw mut odds).cast::<i16>()).wrapping_offset(1)).write(30i16);
            } else {
                if numAfter == 3i32 {
                    ((&raw mut odds).cast::<i16>()).write(20i16);
                    (((&raw mut odds).cast::<i16>()).wrapping_offset(1)).write(20i16);
                    (((&raw mut odds).cast::<i16>()).wrapping_offset(2)).write(20i16);
                } else {
                    {
                        i = 0i32;
                        'l3: loop {
                            if !(i < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                (((&raw mut odds).cast::<i16>()).wrapping_offset((i) as isize))
                                    .write(0i16);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
            }
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < 4i32) {
                    break 'l5;
                }
                'l6: {
                    if ((crate::c::bf_read(
                        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(21),
                        4,
                        1,
                        false,
                    ) as u8)
                        != 0)
                        && ((IsContestantAllowedToCombo(((i) as u8))) != 0)
                    {
                        (((&raw mut oddsMod).cast::<i16>()).wrapping_offset((i) as isize)).write(
                            ((((((((&raw const gComboStarterLookupTable)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((((&raw const gContestMoves).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    (((((((((&raw mut gContestResources).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset((i) as isize * 28))
                                    .wrapping_add(8)
                                    .cast::<u16>())
                                    .read()) as i32) as isize
                                        * 8,
                                ))
                                .wrapping_add(2))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i32)
                                .wrapping_mul(10i32)) as i16),
                        );
                    } else {
                        (((&raw mut oddsMod).cast::<i16>()).wrapping_offset((i) as isize))
                            .write(0i16);
                    }
                    let __p2 = ((&raw mut oddsMod).cast::<i16>()).wrapping_offset((i) as isize);
                    (__p2).write(
                        (((((__p2).read()) as i32).wrapping_sub(
                            (crate::c::div_i32(
                                (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 28))
                                .wrapping_add(13)
                                .cast::<i8>())
                                .read()) as i32),
                                10i32,
                            ))
                            .wrapping_mul(10i32),
                        )) as i16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((&raw mut odds).cast::<i16>()).read()) as i32) != 0i32 {
            {
                i = 0i32;
                'l7: loop {
                    if !((((((&raw mut contestantIds).cast::<u8>()).wrapping_offset((i) as isize))
                        .read()) as i32)
                        != 255i32)
                    {
                        break 'l7;
                    }
                    'l8: {
                        if crate::c::rem_i32(((Random()) as i32), 100i32)
                            < (((((&raw mut odds).cast::<i16>()).wrapping_offset((i) as isize))
                                .read()) as i32)
                                .wrapping_add(
                                    (((((&raw mut oddsMod).cast::<i16>()).wrapping_offset(
                                        (((((&raw mut contestantIds).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32),
                                )
                        {
                            if (CanUnnerveContestant(
                                (((&raw mut contestantIds).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read(),
                            )) != 0
                            {
                                MakeContestantNervous(
                                    (((&raw mut contestantIds).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read(),
                                );
                                SetContestantEffectStringID(
                                    (((&raw mut contestantIds).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read(),
                                    10u8,
                                );
                                numUnnerved = (numUnnerved).wrapping_add(1);
                            } else {
                                contestantUnnerved = 1u32;
                            }
                        } else {
                            contestantUnnerved = 1u32;
                        }
                        if (contestantUnnerved) != 0 {
                            contestantUnnerved = 0u32;
                            SetContestantEffectStringID(
                                (((&raw mut contestantIds).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read(),
                                60u8,
                            );
                            numUnnerved = (numUnnerved).wrapping_add(1);
                        }
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(13))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut contestantIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize,
                        ))
                        .write(1u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            11u8,
        );
        if ((numUnnerved) as i32) == 0i32 {
            SetContestantEffectStringID2(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                54u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_WorsenConditionOfPrevMons() {
    unsafe {
        let mut numHit: u8 = 0u8;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        > (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32))
                        && ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(13)
                        .cast::<i8>())
                        .read()) as i32)
                            > 0i32))
                        && ((CanUnnerveContestant(((i) as u8))) != 0)
                    {
                        (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(13)
                        .cast::<i8>())
                        .write(0i8);
                        crate::c::bf_write(
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 28))
                            .wrapping_add(16),
                            4,
                            2,
                            (2u8) as i32,
                        );
                        SetContestantEffectStringID(((i) as u8), 13u8);
                        numHit = (numHit).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            12u8,
        );
        if ((numHit) as i32) == 0i32 {
            SetContestantEffectStringID2(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                57u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_BadlyStartlesMonsInGoodCondition() {
    unsafe {
        let mut numHit: u8 = 0u8;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        > (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                    {
                        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(13)
                        .cast::<i8>())
                        .read()) as i32)
                            > 0i32
                        {
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<i16>())
                            .write(40i16);
                        } else {
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<i16>())
                            .write(10i16);
                        }
                        (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8))
                        .cast::<u8>())
                        .write(((i) as u8));
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .write(255u8);
                        if (WasAtLeastOneOpponentJammed()) != 0 {
                            numHit = (numHit).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            14u8,
        );
        if ((numHit) as i32) == 0i32 {
            SetContestantEffectStringID2(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                57u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_BetterIfFirst() {
    unsafe {
        if (((((&raw mut gContestantTurnOrder).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            == 0i32
        {
            let mut r#move: u16 = (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(6)
            .cast::<u16>())
            .read();
            let __p1 = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(2)
            .cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    (2i32).wrapping_mul(
                        (((((((&raw const gContestEffects).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw const gContestMoves).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((r#move) as i32) as isize * 8))
                                .read()) as i32) as isize
                                    * 4,
                            ))
                        .wrapping_add(1))
                        .read()) as i32),
                    ),
                )) as i16),
            );
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                15u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_BetterIfLast() {
    unsafe {
        if (((((&raw mut gContestantTurnOrder).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            == 3i32
        {
            let mut r#move: u16 = (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(6)
            .cast::<u16>())
            .read();
            let __p1 = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(2)
            .cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    (2i32).wrapping_mul(
                        (((((((&raw const gContestEffects).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw const gContestMoves).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((r#move) as i32) as isize * 8))
                                .read()) as i32) as isize
                                    * 4,
                            ))
                        .wrapping_add(1))
                        .read()) as i32),
                    ),
                )) as i16),
            );
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                16u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_AppealAsGoodAsPrevOnes() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut appealSum: i32 = 0i32;
        {
            i = 0i32;
            appealSum = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        > (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                    {
                        appealSum = (appealSum).wrapping_add(
                            (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 28))
                            .wrapping_add(2)
                            .cast::<i16>())
                            .read()) as i32),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if appealSum < 0i32 {
            appealSum = 0i32;
        }
        if ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            == 0i32)
            || (appealSum == 0i32)
        {
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                18u8,
            );
        } else {
            let __p1 = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(2)
            .cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(crate::c::div_i32(appealSum, 2i32)))
                    as i16),
            );
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                17u8,
            );
        }
        (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize
                * 28,
        ))
        .wrapping_add(2)
        .cast::<i16>())
        .write(RoundTowardsZero(
            (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(2)
            .cast::<i16>())
            .read(),
        ));
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_AppealAsGoodAsPrevOne() {
    unsafe {
        let mut appeal: i16 = 0i16;
        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            != 0i32
        {
            let mut i: i32 = 0i32;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(17))
                            .read()) as i32) as isize,
                        ))
                        .read()) as i32)
                            .wrapping_sub(1i32)
                            == (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                        {
                            appeal = (((((((&raw mut gContestResources).cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 28))
                            .wrapping_add(2)
                            .cast::<i16>())
                            .read();
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        if ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            == 0i32)
            || (((appeal) as i32) <= 0i32)
        {
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                20u8,
            );
        } else {
            let __p1 = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(2)
            .cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(((appeal) as i32))) as i16));
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                19u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_BetterWhenLater() {
    unsafe {
        let mut whichTurn: u8 = (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize,
        ))
        .read();
        if ((whichTurn) as i32) == 0i32 {
            (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(2)
            .cast::<i16>())
            .write(10i16);
        } else {
            (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(2)
            .cast::<i16>())
            .write((((20i32).wrapping_mul(((whichTurn) as i32))) as i16));
        }
        if ((whichTurn) as i32) == 0i32 {
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                21u8,
            );
        } else {
            if ((whichTurn) as i32) == 1i32 {
                SetContestantEffectStringID(
                    ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(17))
                    .read(),
                    22u8,
                );
            } else {
                if ((whichTurn) as i32) == 2i32 {
                    SetContestantEffectStringID(
                        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read(),
                        23u8,
                    );
                } else {
                    SetContestantEffectStringID(
                        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read(),
                        24u8,
                    );
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_QualityDependsOnTiming() {
    unsafe {
        let mut rval: u8 = ((crate::c::rem_i32(((Random()) as i32), 10i32)) as u8);
        let mut appeal: i16 = 0i16;
        if ((rval) as i32) < 3i32 {
            appeal = 10i16;
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                26u8,
            );
        } else {
            if ((rval) as i32) < 6i32 {
                appeal = 20i16;
                SetContestantEffectStringID(
                    ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(17))
                    .read(),
                    27u8,
                );
            } else {
                if ((rval) as i32) < 8i32 {
                    appeal = 40i16;
                    SetContestantEffectStringID(
                        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read(),
                        28u8,
                    );
                } else {
                    if ((rval) as i32) < 9i32 {
                        appeal = 60i16;
                        SetContestantEffectStringID(
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(17))
                            .read(),
                            29u8,
                        );
                    } else {
                        appeal = 80i16;
                        SetContestantEffectStringID(
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(17))
                            .read(),
                            30u8,
                        );
                    }
                }
            }
        }
        (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize
                * 28,
        ))
        .wrapping_add(2)
        .cast::<i16>())
        .write(appeal);
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_BetterIfSameType() {
    unsafe {
        let mut turnOrder: i8 = (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize,
        ))
        .read()) as i8);
        let mut i: i8 = ((((turnOrder) as i32).wrapping_sub(1i32)) as i8);
        let mut j: i8 = 0i8;
        let mut r#move: u16 = 0u16;
        if ((turnOrder) as i32) == 0i32 {
            return;
        }
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            {
                j = 0i8;
                'l2: loop {
                    if !(((j) as i32) < 4i32) {
                        break 'l2;
                    }
                    'l3: {
                        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset(((j) as i32) as isize))
                        .read()) as i32)
                            == ((i) as i32)
                        {
                            break 'l2;
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            if (((crate::c::bf_read(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((j) as i32) as isize * 28))
                .wrapping_add(11),
                7,
                1,
                false,
            ) as u8)
                != 0)
                || ((crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((j) as i32) as isize * 28))
                    .wrapping_add(12),
                    0,
                    1,
                    false,
                ) as u8)
                    != 0))
                || ((crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((j) as i32) as isize * 28))
                    .wrapping_add(12),
                    1,
                    2,
                    false,
                ) as u8)
                    != 0)
            {
                if (({
                    let __t1 = (i).wrapping_sub(1);
                    i = __t1;
                    __t1
                }) as i32)
                    < 0i32
                {
                    return;
                }
            } else {
                break 'l1;
            }
        }
        r#move = (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize
                * 28,
        ))
        .wrapping_add(6)
        .cast::<u16>())
        .read();
        if ((crate::c::bf_read(
            ((((&raw const gContestMoves).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 8))
            .wrapping_add(1),
            0,
            3,
            false,
        ) as u8) as i32)
            == ((crate::c::bf_read(
                ((((&raw const gContestMoves).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((j) as i32) as isize * 28))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 8,
                    ))
                .wrapping_add(1),
                0,
                3,
                false,
            ) as u8) as i32)
        {
            let __p2 = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(2)
            .cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    (((((((&raw const gContestEffects).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw const gContestMoves).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset(((r#move) as i32) as isize * 8))
                            .read()) as i32) as isize
                                * 4,
                        ))
                    .wrapping_add(1))
                    .read()) as i32)
                        .wrapping_mul(2i32),
                )) as i16),
            );
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                31u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_BetterIfDiffType() {
    unsafe {
        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            != 0i32
        {
            let mut r#move: u16 = (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(6)
            .cast::<u16>())
            .read();
            let mut i: i32 = 0i32;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(17))
                            .read()) as i32) as isize,
                        ))
                        .read()) as i32)
                            .wrapping_sub(1i32)
                            == (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32))
                            && (((crate::c::bf_read(
                                ((((&raw const gContestMoves).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((r#move) as i32) as isize * 8))
                                .wrapping_add(1),
                                0,
                                3,
                                false,
                            ) as u8) as i32)
                                != ((crate::c::bf_read(
                                    ((((&raw const gContestMoves).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset(
                                        (((((((((&raw mut gContestResources)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset((i) as isize * 28))
                                        .wrapping_add(6)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize
                                            * 8,
                                    ))
                                    .wrapping_add(1),
                                    0,
                                    3,
                                    false,
                                ) as u8) as i32))
                        {
                            let __p1 = ((((((&raw mut gContestResources).cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(
                                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(8)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(17))
                                .read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(2)
                            .cast::<i16>();
                            (__p1).write(
                                (((((__p1).read()) as i32).wrapping_add(
                                    (((((((&raw const gContestEffects).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset(
                                        ((((((&raw const gContestMoves).cast::<u8>().cast_mut())
                                            .cast::<u8>())
                                        .wrapping_offset(((r#move) as i32) as isize * 8))
                                        .read()) as i32)
                                            as isize
                                            * 4,
                                    ))
                                    .wrapping_add(1))
                                    .read()) as i32)
                                        .wrapping_mul(2i32),
                                )) as i16),
                            );
                            SetContestantEffectStringID(
                                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(8)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(17))
                                .read(),
                                32u8,
                            );
                            break 'l1;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_AffectedByPrevAppeal() {
    unsafe {
        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            != 0i32
        {
            let mut i: i32 = 0i32;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(17))
                            .read()) as i32) as isize,
                        ))
                        .read()) as i32)
                            .wrapping_sub(1i32)
                            == (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                        {
                            if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(
                                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(8)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(17))
                                .read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(2)
                            .cast::<i16>())
                            .read()) as i32)
                                > (((((((((&raw mut gContestResources).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 28))
                                .wrapping_add(2)
                                .cast::<i16>())
                                .read()) as i32)
                            {
                                let __p1 = ((((((&raw mut gContestResources).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(
                                    ((((((((&raw mut gContestResources).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(8)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(17))
                                    .read()) as i32) as isize
                                        * 28,
                                ))
                                .wrapping_add(2)
                                .cast::<i16>();
                                (__p1)
                                    .write((((((__p1).read()) as i32).wrapping_mul(2i32)) as i16));
                                SetContestantEffectStringID(
                                    ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                        .wrapping_add(8)
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(17))
                                    .read(),
                                    33u8,
                                );
                            } else {
                                if (((((((((&raw mut gContestResources).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(
                                    ((((((((&raw mut gContestResources).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(8)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(17))
                                    .read()) as i32) as isize
                                        * 28,
                                ))
                                .wrapping_add(2)
                                .cast::<i16>())
                                .read()) as i32)
                                    < (((((((((&raw mut gContestResources).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset((i) as isize * 28))
                                    .wrapping_add(2)
                                    .cast::<i16>())
                                    .read()) as i32)
                                {
                                    (((((((&raw mut gContestResources).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(
                                        ((((((((&raw mut gContestResources).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(8)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(17))
                                        .read()) as i32)
                                            as isize
                                            * 28,
                                    ))
                                    .wrapping_add(2)
                                    .cast::<i16>())
                                    .write(0i16);
                                    SetContestantEffectStringID(
                                        ((((((&raw mut gContestResources).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(8)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(17))
                                        .read(),
                                        34u8,
                                    );
                                }
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_ImproveConditionPreventNervousness() {
    unsafe {
        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize
                * 28,
        ))
        .wrapping_add(13)
        .cast::<i8>())
        .read()) as i32)
            < 30i32
        {
            let __p1 = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(13)
            .cast::<i8>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(10i32)) as i8));
            crate::c::bf_write(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(17))
                    .read()) as i32) as isize
                        * 28,
                ))
                .wrapping_add(16),
                4,
                2,
                (1u8) as i32,
            );
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                35u8,
            );
        } else {
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                58u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_BetterWithGoodCondition() {
    unsafe {
        crate::c::bf_write(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(17),
            5,
            1,
            (1u8) as i32,
        );
        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize
                * 28,
        ))
        .wrapping_add(13)
        .cast::<i8>())
        .read()) as i32)
            != 0i32
        {
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                36u8,
            );
        } else {
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                59u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_NextAppealEarlier() {
    unsafe {
        let mut i: i8 = 0i8;
        let mut j: i8 = 0i8;
        let mut turnOrder = crate::ffi::Align4([0u8; 4]);
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read()).cast::<*mut u8>())
            .read())
        .wrapping_add(1))
        .read()) as i32)
            != 4i32
        {
            {
                i = 0i8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((&raw mut turnOrder).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(
                            (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 28))
                            .wrapping_add(25))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            (((&raw mut turnOrder).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize,
            ))
            .write(255u8);
            {
                i = 0i8;
                'l3: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        {
                            j = 0i8;
                            'l5: loop {
                                if !(((j) as i32) < 4i32) {
                                    break 'l5;
                                }
                                'l6: {
                                    if ((((j) as i32)
                                        != ((((((((&raw mut gContestResources)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(8)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(17))
                                        .read())
                                            as i32))
                                        && (((i) as i32)
                                            == (((((&raw mut turnOrder).cast::<u8>())
                                                .wrapping_offset(((j) as i32) as isize))
                                            .read())
                                                as i32)))
                                        && ((((((&raw mut turnOrder).cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize))
                                        .read())
                                            as i32)
                                            == (((((((((&raw mut gContestResources)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(4)
                                            .cast::<*mut u8>())
                                            .read())
                                            .wrapping_offset(((j) as i32) as isize * 28))
                                            .wrapping_add(25))
                                            .read())
                                                as i32))
                                    {
                                        let __p1 = ((&raw mut turnOrder).cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize);
                                        (__p1).write(((__p1).read()).wrapping_add(1));
                                        break 'l5;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        if ((j) as i32) == 4i32 {
                            break 'l3;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            (((&raw mut turnOrder).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize,
            ))
            .write(0u8);
            crate::c::bf_write(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(17))
                    .read()) as i32) as isize
                        * 28,
                ))
                .wrapping_add(16),
                6,
                2,
                (1u8) as i32,
            );
            {
                i = 0i8;
                'l7: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l7;
                    }
                    'l8: {
                        (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((i) as i32) as isize * 28))
                        .wrapping_add(25))
                        .write(
                            (((&raw mut turnOrder).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            crate::c::bf_write(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(17))
                    .read()) as i32) as isize
                        * 28,
                ))
                .wrapping_add(17),
                0,
                2,
                (1u8) as i32,
            );
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                37u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_NextAppealLater() {
    unsafe {
        let mut i: i8 = 0i8;
        let mut j: i8 = 0i8;
        let mut turnOrder = crate::ffi::Align4([0u8; 4]);
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read()).cast::<*mut u8>())
            .read())
        .wrapping_add(1))
        .read()) as i32)
            != 4i32
        {
            {
                i = 0i8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((&raw mut turnOrder).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(
                            (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 28))
                            .wrapping_add(25))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            (((&raw mut turnOrder).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize,
            ))
            .write(255u8);
            {
                i = 3i8;
                'l3: loop {
                    if !(((i) as i32) > (-1i32)) {
                        break 'l3;
                    }
                    'l4: {
                        {
                            j = 0i8;
                            'l5: loop {
                                if !(((j) as i32) < 4i32) {
                                    break 'l5;
                                }
                                'l6: {
                                    if ((((j) as i32)
                                        != ((((((((&raw mut gContestResources)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(8)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(17))
                                        .read())
                                            as i32))
                                        && (((i) as i32)
                                            == (((((&raw mut turnOrder).cast::<u8>())
                                                .wrapping_offset(((j) as i32) as isize))
                                            .read())
                                                as i32)))
                                        && ((((((&raw mut turnOrder).cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize))
                                        .read())
                                            as i32)
                                            == (((((((((&raw mut gContestResources)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(4)
                                            .cast::<*mut u8>())
                                            .read())
                                            .wrapping_offset(((j) as i32) as isize * 28))
                                            .wrapping_add(25))
                                            .read())
                                                as i32))
                                    {
                                        let __p1 = ((&raw mut turnOrder).cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize);
                                        (__p1).write(((__p1).read()).wrapping_sub(1));
                                        break 'l5;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        if ((j) as i32) == 4i32 {
                            break 'l3;
                        }
                    }
                    i = (i).wrapping_sub(1);
                }
            }
            (((&raw mut turnOrder).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32) as isize,
            ))
            .write(3u8);
            crate::c::bf_write(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(17))
                    .read()) as i32) as isize
                        * 28,
                ))
                .wrapping_add(16),
                6,
                2,
                (1u8) as i32,
            );
            {
                i = 0i8;
                'l7: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l7;
                    }
                    'l8: {
                        (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((i) as i32) as isize * 28))
                        .wrapping_add(25))
                        .write(
                            (((&raw mut turnOrder).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            crate::c::bf_write(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(17))
                    .read()) as i32) as isize
                        * 28,
                ))
                .wrapping_add(17),
                0,
                2,
                (2u8) as i32,
            );
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                38u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_MakeScramblingTurnOrderEasier() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn ContestEffect_ScrambleNextTurnOrder() {
    unsafe {
        let mut i: i8 = 0i8;
        let mut j: i8 = 0i8;
        let mut turnOrder = crate::ffi::Align4([0u8; 4]);
        let mut unselectedContestants = crate::ffi::Align4([0u8; 4]);
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read()).cast::<*mut u8>())
            .read())
        .wrapping_add(1))
        .read()) as i32)
            != 4i32
        {
            {
                i = 0i8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((&raw mut turnOrder).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(
                            (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 28))
                            .wrapping_add(25))
                            .read(),
                        );
                        (((&raw mut unselectedContestants).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(((i) as u8));
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i8;
                'l3: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        let mut rval: u8 = ((crate::c::rem_i32(
                            ((Random()) as i32),
                            (4i32).wrapping_sub(((i) as i32)),
                        )) as u8);
                        {
                            j = 0i8;
                            'l5: loop {
                                if !(((j) as i32) < 4i32) {
                                    break 'l5;
                                }
                                'l6: {
                                    if (((((&raw mut unselectedContestants).cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize))
                                    .read()) as i32)
                                        != 255i32
                                    {
                                        if ((rval) as i32) == 0i32 {
                                            (((&raw mut turnOrder).cast::<u8>())
                                                .wrapping_offset(((j) as i32) as isize))
                                            .write(((i) as u8));
                                            (((&raw mut unselectedContestants).cast::<u8>())
                                                .wrapping_offset(((j) as i32) as isize))
                                            .write(255u8);
                                            break 'l5;
                                        } else {
                                            rval = (rval).wrapping_sub(1);
                                        }
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
                i = 0i8;
                'l7: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l7;
                    }
                    'l8: {
                        (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((i) as i32) as isize * 28))
                        .wrapping_add(25))
                        .write(
                            (((&raw mut turnOrder).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                        crate::c::bf_write(
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 28))
                            .wrapping_add(16),
                            6,
                            2,
                            (2u8) as i32,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            crate::c::bf_write(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(17))
                    .read()) as i32) as isize
                        * 28,
                ))
                .wrapping_add(17),
                0,
                2,
                (3u8) as i32,
            );
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                39u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_ExciteAudienceInAnyContest() {
    unsafe {
        if ((crate::c::bf_read(
            ((((&raw const gContestMoves).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(17))
                    .read()) as i32) as isize
                        * 28,
                ))
                .wrapping_add(6)
                .cast::<u16>())
                .read()) as i32) as isize
                    * 8,
            ))
            .wrapping_add(1),
            0,
            3,
            false,
        ) as u8) as i32)
            != ((((&raw mut gSpecialVar_ContestCategory).cast::<u16>()).read()) as i32)
        {
            crate::c::bf_write(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(17))
                    .read()) as i32) as isize
                        * 28,
                ))
                .wrapping_add(17),
                4,
                1,
                (1u8) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_BadlyStartleMonsWithGoodAppeals() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut numJammed: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        > (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                    {
                        if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(2)
                        .cast::<i16>())
                        .read()) as i32)
                            > 0i32
                        {
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<i16>())
                            .write(
                                ((crate::c::div_i32(
                                    (((((((((&raw mut gContestResources).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset((i) as isize * 28))
                                    .wrapping_add(2)
                                    .cast::<i16>())
                                    .read()) as i32),
                                    2i32,
                                )) as i16),
                            );
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<i16>())
                            .write(RoundUp(
                                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(8)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<i16>())
                                .read(),
                            ));
                        } else {
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<i16>())
                            .write(10i16);
                        }
                        (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8))
                        .cast::<u8>())
                        .write(((i) as u8));
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .write(255u8);
                        if (WasAtLeastOneOpponentJammed()) != 0 {
                            numJammed = (numJammed).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((numJammed) as i32) == 0i32 {
            SetContestantEffectStringID2(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                54u8,
            );
        }
        SetContestantEffectStringID(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read(),
            48u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_BetterWhenAudienceExcited() {
    unsafe {
        let mut appeal: i16 = 0i16;
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read()).cast::<*mut u8>())
            .read())
        .wrapping_add(19)
        .cast::<i8>())
        .read()) as i32)
            == 0i32
        {
            appeal = 10i16;
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                26u8,
            );
        } else {
            if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_add(19)
            .cast::<i8>())
            .read()) as i32)
                == 1i32
            {
                appeal = 20i16;
                SetContestantEffectStringID(
                    ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(17))
                    .read(),
                    27u8,
                );
            } else {
                if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(19)
                .cast::<i8>())
                .read()) as i32)
                    == 2i32
                {
                    appeal = 30i16;
                    SetContestantEffectStringID(
                        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read(),
                        28u8,
                    );
                } else {
                    if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(19)
                    .cast::<i8>())
                    .read()) as i32)
                        == 3i32
                    {
                        appeal = 50i16;
                        SetContestantEffectStringID(
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(17))
                            .read(),
                            29u8,
                        );
                    } else {
                        appeal = 60i16;
                        SetContestantEffectStringID(
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(17))
                            .read(),
                            30u8,
                        );
                    }
                }
            }
        }
        (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read()) as i32) as isize
                * 28,
        ))
        .wrapping_add(2)
        .cast::<i16>())
        .write(appeal);
    }
}
pub(crate) unsafe extern "C" fn ContestEffect_DontExciteAudience() {
    unsafe {
        if !((crate::c::bf_read(
            (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1),
            0,
            1,
            false,
        ) as u8)
            != 0)
        {
            crate::c::bf_write(
                (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1),
                0,
                1,
                (1u8) as i32,
            );
            crate::c::bf_write(
                (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1),
                1,
                3,
                (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read()) as i32,
            );
            SetContestantEffectStringID(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                61u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn JamByMoveCategory(category: u8) {
    unsafe {
        let mut category = category;
        let mut i: i32 = 0i32;
        let mut numJammed: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        > (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                    {
                        if ((category) as i32)
                            == ((crate::c::bf_read(
                                ((((&raw const gContestMoves).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    (((((((((&raw mut gContestResources).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset((i) as isize * 28))
                                    .wrapping_add(6)
                                    .cast::<u16>())
                                    .read()) as i32) as isize
                                        * 8,
                                ))
                                .wrapping_add(1),
                                0,
                                3,
                                false,
                            ) as u8) as i32)
                        {
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<i16>())
                            .write(40i16);
                        } else {
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<i16>())
                            .write(10i16);
                        }
                        (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8))
                        .cast::<u8>())
                        .write(((i) as u8));
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .write(255u8);
                        if (WasAtLeastOneOpponentJammed()) != 0 {
                            numJammed = (numJammed).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if numJammed == 0i32 {
            SetContestantEffectStringID2(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read(),
                54u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CanUnnerveContestant(i: u8) -> u8 {
    unsafe {
        let mut i = i;
        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(13))
        .cast::<u8>())
        .wrapping_offset(((i) as i32) as isize))
        .write(1u8);
        if (crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((i) as i32) as isize * 28))
            .wrapping_add(16),
            1,
            1,
            false,
        ) as u8)
            != 0
        {
            SetContestantEffectStringID(i, 45u8);
            return 0u8;
        } else {
            if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((i) as i32) as isize * 28))
            .wrapping_add(18))
            .read()) as i32)
                != 0i32
            {
                let __p1 = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((i) as i32) as isize * 28))
                .wrapping_add(18);
                (__p1).write(((__p1).read()).wrapping_sub(1));
                SetContestantEffectStringID(i, 44u8);
                return 0u8;
            } else {
                if (!((crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize * 28))
                    .wrapping_add(11),
                    7,
                    1,
                    false,
                ) as u8)
                    != 0))
                    && (((crate::c::bf_read(
                        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((i) as i32) as isize * 28))
                        .wrapping_add(12),
                        1,
                        2,
                        false,
                    ) as u8) as i32)
                        == 0i32)
                {
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
pub(crate) unsafe extern "C" fn WasAtLeastOneOpponentJammed() -> u8 {
    unsafe {
        let mut jamBuffer = crate::ffi::Align4([0u8; 8]);
        (&raw mut jamBuffer)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i16>()
            .write(0i16);
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8))
                .cast::<u8>())
                .wrapping_offset((i) as isize))
                .read()) as i32)
                    != 255i32)
                {
                    break 'l1;
                }
                'l2: {
                    let mut contestant: u8 =
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read();
                    if (CanUnnerveContestant(contestant)) != 0 {
                        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(6)
                        .cast::<i16>())
                        .write(
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<i16>())
                            .read(),
                        );
                        if (crate::c::bf_read(
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((contestant) as i32) as isize * 28))
                            .wrapping_add(16),
                            2,
                            1,
                            false,
                        ) as u8)
                            != 0
                        {
                            let __p1 = (((((&raw mut gContestResources).cast::<*mut u8>())
                                .read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(6)
                            .cast::<i16>();
                            (__p1).write((((((__p1).read()) as i32).wrapping_mul(2i32)) as i16));
                        }
                        if (crate::c::bf_read(
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((contestant) as i32) as isize * 28))
                            .wrapping_add(16),
                            0,
                            1,
                            false,
                        ) as u8)
                            != 0
                        {
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(6)
                            .cast::<i16>())
                            .write(10i16);
                            SetContestantEffectStringID(contestant, 47u8);
                        } else {
                            let __p2 = (((((&raw mut gContestResources).cast::<*mut u8>())
                                .read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(6)
                            .cast::<i16>();
                            (__p2).write(
                                (((((__p2).read()) as i32).wrapping_sub(
                                    (((((((((&raw mut gContestResources).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((contestant) as i32) as isize * 28))
                                    .wrapping_add(15))
                                    .read()) as i32),
                                )) as i16),
                            );
                            if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(6)
                            .cast::<i16>())
                            .read()) as i32)
                                <= 0i32
                            {
                                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(8)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(6)
                                .cast::<i16>())
                                .write(0i16);
                                SetContestantEffectStringID(contestant, 46u8);
                            } else {
                                JamContestant(
                                    contestant,
                                    ((((((((&raw mut gContestResources).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(8)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(6)
                                    .cast::<i16>())
                                    .read()) as u8),
                                );
                                SetStartledString(
                                    contestant,
                                    ((((((((&raw mut gContestResources).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(8)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(6)
                                    .cast::<i16>())
                                    .read()) as u8),
                                );
                                (((&raw mut jamBuffer).cast::<i16>())
                                    .wrapping_offset(((contestant) as i32) as isize))
                                .write(
                                    ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                        .wrapping_add(8)
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(6)
                                    .cast::<i16>())
                                    .read(),
                                );
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    if (((((&raw mut jamBuffer).cast::<i16>()).wrapping_offset((i) as isize))
                        .read()) as i32)
                        != 0i32
                    {
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn JamContestant(i: u8, jam: u8) {
    unsafe {
        let mut i = i;
        let mut jam = jam;
        let __p1 = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((i) as i32) as isize * 28))
        .wrapping_add(2)
        .cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(((jam) as i32))) as i16));
        let __p2 = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((i) as i32) as isize * 28))
        .wrapping_add(14);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(((jam) as i32))) as u8));
    }
}
pub(crate) unsafe extern "C" fn RoundTowardsZero(score: i16) -> i16 {
    unsafe {
        let mut score = score;
        let mut absScore: i16 = ((crate::c::rem_i32(
            (if ((score) as i32) < 0i32 {
                ((score) as i32).wrapping_neg()
            } else {
                ((score) as i32)
            }),
            10i32,
        )) as i16);
        if ((score) as i32) < 0i32 {
            if ((absScore) as i32) != 0i32 {
                score = ((((score) as i32).wrapping_sub((10i32).wrapping_sub(((absScore) as i32))))
                    as i16);
            }
        } else {
            score = ((((score) as i32).wrapping_sub(((absScore) as i32))) as i16);
        }
        return score;
    }
}
pub(crate) unsafe extern "C" fn RoundUp(score: i16) -> i16 {
    unsafe {
        let mut score = score;
        let mut absScore: i16 = ((crate::c::rem_i32(
            (if ((score) as i32) < 0i32 {
                ((score) as i32).wrapping_neg()
            } else {
                ((score) as i32)
            }),
            10i32,
        )) as i16);
        if ((absScore) as i32) != 0i32 {
            score =
                ((((score) as i32).wrapping_add((10i32).wrapping_sub(((absScore) as i32)))) as i16);
        }
        return score;
    }
}
