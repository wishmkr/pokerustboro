//! Translated from `src/contest_ai.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sContestAICmdTable
#[allow(unused_imports)]
use crate::data::contest_ai::*;

unsafe extern "C" {
    static mut gAIScriptPtr: u8;
    static mut gContestAI_ScriptsTable: u8;
    static mut gContestEffects: u8;
    static mut gContestMonRound1Points: u8;
    static mut gContestMons: u8;
    static mut gContestMoves: u8;
    static mut gContestResources: u8;
    static mut gSpecialVar_ContestCategory: u8;
    fn AreMovesContestCombo(a0: u16, a1: u16) -> u8;
    fn Contest_GetMoveExcitement(a0: u16) -> i8;
    fn Contest_IsMonsTurnDisabled(a0: u8) -> u8;
    fn IsContestantAllowedToCombo(a0: u8) -> u8;
    fn Random() -> u16;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestAI_ResetAI(contestantAI: u8) {
    unsafe {
        let mut contestantAI = contestantAI;
        let mut i: i32 = 0i32;
        crate::c::memset(
            ((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read(),
            0i32,
            68u32,
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(5))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(100u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(65))
        .write(contestantAI);
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(64))
        .write(0u8);
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(20)
        .cast::<u32>())
        .write(
            ((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(65))
                .read()) as i32) as isize
                    * 64,
            ))
            .wrapping_add(24)
            .cast::<u32>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestAI_GetActionToUse() -> u8 {
    unsafe {
        'l1: loop {
            if !(((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(20)
            .cast::<u32>())
            .read()
                != 0u32)
            {
                break 'l1;
            }
            if (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(20)
            .cast::<u32>())
            .read()
                & 1u32)
                != 0
            {
                (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .write(0u8);
                ContestAI_DoAIProcessing();
            }
            let __p1 = (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(20)
            .cast::<u32>();
            (__p1).write(((__p1).read() >> 1));
            let __p2 = (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(16);
            (__p2).write(((__p2).read()).wrapping_add(1));
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .write(0u8);
        }
        'l2: loop {
            if !((1i32) != 0) {
                break 'l2;
            }
            let mut moveIndex: u8 = ((if (0i32) != 0 {
                crate::c::rem_i32(((Random()) as i32), 4i32)
            } else {
                (((Random()) as i32) & 3i32)
            }) as u8);
            let mut score: u8 = ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(5))
            .cast::<u8>())
            .wrapping_offset(((moveIndex) as i32) as isize))
            .read();
            let mut i: i32 = 0i32;
            {
                i = 0i32;
                'l3: loop {
                    if !(i < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        if ((score) as i32)
                            < ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(12)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(5))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                        {
                            break 'l3;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if i == 4i32 {
                return moveIndex;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAI_DoAIProcessing() {
    unsafe {
        'l1: loop {
            if !((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .read()) as i32)
                != 2i32)
            {
                break 'l1;
            }
            'l2: {
                let __sw1 = (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .read()) as i32);
                if __sw1 == 3i32 {
                    break 'l2;
                }
                if __sw1 == 0i32 {
                    ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                        ((((&raw mut gContestAI_ScriptsTable).cast::<*mut u8>())
                            .cast::<*mut u8>())
                        .wrapping_offset(
                            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(12)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(16))
                            .read()) as i32) as isize,
                        ))
                        .read(),
                    );
                    if ((((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(12)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(65))
                        .read()) as i32) as isize
                            * 64,
                    ))
                    .wrapping_add(30))
                    .cast::<u16>())
                    .wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(12)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        == 0i32
                    {
                        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(12)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(2)
                        .cast::<u16>())
                        .write(0u16);
                    } else {
                        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(12)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(2)
                        .cast::<u16>())
                        .write(
                            ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(12)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(65))
                                .read()) as i32) as isize
                                    * 64,
                            ))
                            .wrapping_add(30))
                            .cast::<u16>())
                            .wrapping_offset(
                                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(12)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4))
                                .read()) as i32) as isize,
                            ))
                            .read(),
                        );
                    }
                    let __p2 = (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read());
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    break 'l2;
                }
                if __sw1 == 1i32 {
                    if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read()) as i32)
                        != 0i32
                    {
                        (((((&raw const sContestAICmdTable)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .wrapping_offset(
                            (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                                as isize,
                        ))
                        .read())
                        .unwrap_unchecked()();
                    } else {
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(12)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(5))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(12)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4))
                            .read()) as i32) as isize,
                        ))
                        .write(0u8);
                        let __p3 = (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(12)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(9);
                        (__p3).write((((((__p3).read()) as i32) | 1i32) as u8));
                    }
                    if (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(9))
                    .read()) as i32)
                        & 1i32)
                        != 0
                    {
                        let __p4 = (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(12)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4);
                        (__p4).write(((__p4).read()).wrapping_add(1));
                        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(12)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4))
                        .read()) as i32)
                            < 4i32
                        {
                            (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(12)
                                .cast::<*mut u8>())
                            .read())
                            .write(0u8);
                        } else {
                            let __p5 = (((((&raw mut gContestResources).cast::<*mut u8>())
                                .read())
                            .wrapping_add(12)
                            .cast::<*mut u8>())
                            .read());
                            (__p5).write(((__p5).read()).wrapping_add(1));
                        }
                        let __p6 = (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(12)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(9);
                        (__p6).write((((((__p6).read()) as i32) & (-2i32)) as u8));
                    }
                    break 'l2;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetContestantIdByTurn(turn: u8) -> u8 {
    unsafe {
        let mut turn = turn;
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
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == ((turn) as i32)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((i) as u8);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_score() {
    unsafe {
        let mut score: i16 = ((((((((((((&raw mut gContestResources).cast::<*mut u8>())
            .read())
        .wrapping_add(12)
        .cast::<*mut u8>())
        .read())
        .wrapping_add(5))
        .cast::<u8>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            .wrapping_add(
                (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                    as i8) as i32),
            )) as i16);
        if ((score) as i32) > 255i32 {
            score = 255i16;
        } else {
            if ((score) as i32) < 0i32 {
                score = 0i16;
            }
        }
        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(5))
        .cast::<u8>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .read()) as i32) as isize,
        ))
        .write(((score) as u8));
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_appeal_num() {
    unsafe {
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_add(1))
            .read()) as i16),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_appeal_num_less_than() {
    unsafe {
        ContestAICmd_get_appeal_num();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            < (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_appeal_num_more_than() {
    unsafe {
        ContestAICmd_get_appeal_num();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            > (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_appeal_num_eq() {
    unsafe {
        ContestAICmd_get_appeal_num();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_appeal_num_not_eq() {
    unsafe {
        ContestAICmd_get_appeal_num();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_excitement() {
    unsafe {
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_add(19)
            .cast::<i8>())
            .read()) as i16),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_excitement_less_than() {
    unsafe {
        ContestAICmd_get_excitement();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            < (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_excitement_more_than() {
    unsafe {
        ContestAICmd_get_excitement();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            > (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_excitement_eq() {
    unsafe {
        ContestAICmd_get_excitement();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_excitement_not_eq() {
    unsafe {
        ContestAICmd_get_excitement();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_user_order() {
    unsafe {
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(
            (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .cast::<u8>())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(65))
                .read()) as i32) as isize,
            ))
            .read()) as i16),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_order_less_than() {
    unsafe {
        ContestAICmd_get_user_order();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            < (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_order_more_than() {
    unsafe {
        ContestAICmd_get_user_order();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            > (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_order_eq() {
    unsafe {
        ContestAICmd_get_user_order();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_order_not_eq() {
    unsafe {
        ContestAICmd_get_user_order();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_user_condition() {
    unsafe {
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(
            ((crate::c::div_i32(
                (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(65))
                    .read()) as i32) as isize
                        * 28,
                ))
                .wrapping_add(13)
                .cast::<i8>())
                .read()) as i32),
                10i32,
            )) as i16),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_condition_less_than() {
    unsafe {
        ContestAICmd_get_user_condition();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            < (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_condition_more_than() {
    unsafe {
        ContestAICmd_get_user_condition();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            > (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_condition_eq() {
    unsafe {
        ContestAICmd_get_user_condition();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_condition_not_eq() {
    unsafe {
        ContestAICmd_get_user_condition();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_points() {
    unsafe {
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(
            (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(65))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(4)
            .cast::<i16>())
            .read(),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_points_less_than() {
    unsafe {
        ContestAICmd_get_points();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            < ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    << 8)) as i16) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_points_more_than() {
    unsafe {
        ContestAICmd_get_points();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            > ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    << 8)) as i16) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_points_eq() {
    unsafe {
        ContestAICmd_get_points();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    << 8)) as i16) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_points_not_eq() {
    unsafe {
        ContestAICmd_get_points();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    << 8)) as i16) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_preliminary_points() {
    unsafe {
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(
            ((((&raw mut gContestMonRound1Points).cast::<i16>()).cast::<i16>()).wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(65))
                .read()) as i32) as isize,
            ))
            .read(),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_preliminary_points_less_than() {
    unsafe {
        ContestAICmd_get_preliminary_points();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            < ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    << 8)) as i16) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_preliminary_points_more_than() {
    unsafe {
        ContestAICmd_get_preliminary_points();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            > ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    << 8)) as i16) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_preliminary_points_eq() {
    unsafe {
        ContestAICmd_get_preliminary_points();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    << 8)) as i16) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_preliminary_points_not_eq() {
    unsafe {
        ContestAICmd_get_preliminary_points();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    << 8)) as i16) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_contest_type() {
    unsafe {
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(((((&raw mut gSpecialVar_ContestCategory).cast::<u16>()).read()) as i16));
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_contest_type_eq() {
    unsafe {
        ContestAICmd_get_contest_type();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_contest_type_not_eq() {
    unsafe {
        ContestAICmd_get_contest_type();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_move_excitement() {
    unsafe {
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(
            ((Contest_GetMoveExcitement(
                ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(65))
                    .read()) as i32) as isize
                        * 64,
                ))
                .wrapping_add(30))
                .cast::<u16>())
                .wrapping_offset(
                    ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(4))
                    .read()) as i32) as isize,
                ))
                .read(),
            )) as i16),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_move_excitement_less_than() {
    unsafe {
        ContestAICmd_get_move_excitement();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            < ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i8) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_move_excitement_more_than() {
    unsafe {
        ContestAICmd_get_move_excitement();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            > ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i8) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_move_excitement_eq() {
    unsafe {
        ContestAICmd_get_move_excitement();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i8) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_move_excitement_not_eq() {
    unsafe {
        ContestAICmd_get_move_excitement();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i8) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_move_effect() {
    unsafe {
        let mut r#move: u16 = ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(65))
            .read()) as i32) as isize
                * 64,
        ))
        .wrapping_add(30))
        .cast::<u16>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .read()) as i32) as isize,
        ))
        .read();
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(
            (((((&raw mut gContestMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 8))
            .read()) as i16),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_move_effect_eq() {
    unsafe {
        ContestAICmd_get_move_effect();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_move_effect_not_eq() {
    unsafe {
        ContestAICmd_get_move_effect();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_move_effect_type() {
    unsafe {
        let mut r#move: u16 = ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(65))
            .read()) as i32) as isize
                * 64,
        ))
        .wrapping_add(30))
        .cast::<u16>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .read()) as i32) as isize,
        ))
        .read();
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(
            (((((&raw mut gContestEffects).cast::<u8>()).wrapping_offset(
                (((((&raw mut gContestMoves).cast::<u8>())
                    .wrapping_offset(((r#move) as i32) as isize * 8))
                .read()) as i32) as isize
                    * 4,
            ))
            .read()) as i16),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_move_effect_type_eq() {
    unsafe {
        ContestAICmd_get_move_effect_type();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_move_effect_type_not_eq() {
    unsafe {
        ContestAICmd_get_move_effect_type();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_check_most_appealing_move() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut r#move: u16 = ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(65))
            .read()) as i32) as isize
                * 64,
        ))
        .wrapping_add(30))
        .cast::<u16>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .read()) as i32) as isize,
        ))
        .read();
        let mut appeal: u8 = ((((&raw mut gContestEffects).cast::<u8>()).wrapping_offset(
            (((((&raw mut gContestMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 8))
            .read()) as i32) as isize
                * 4,
        ))
        .wrapping_add(1))
        .read();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut newMove: u16 = ((((((&raw mut gContestMons).cast::<u8>())
                        .wrapping_offset(
                            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(12)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(65))
                            .read()) as i32) as isize
                                * 64,
                        ))
                    .wrapping_add(30))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read();
                    if (((newMove) as i32) != 0i32)
                        && (((appeal) as i32)
                            < ((((((&raw mut gContestEffects).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gContestMoves).cast::<u8>())
                                    .wrapping_offset(((newMove) as i32) as isize * 8))
                                .read()) as i32) as isize
                                    * 4,
                            ))
                            .wrapping_add(1))
                            .read()) as i32))
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if i == 4i32 {
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(24)
            .cast::<i16>())
            .write(1i16);
        } else {
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(24)
            .cast::<i16>())
            .write(0i16);
        }
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_most_appealing_move() {
    unsafe {
        ContestAICmd_check_most_appealing_move();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != 0i32
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_check_most_jamming_move() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut r#move: u16 = ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(65))
            .read()) as i32) as isize
                * 64,
        ))
        .wrapping_add(30))
        .cast::<u16>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .read()) as i32) as isize,
        ))
        .read();
        let mut jam: u8 = ((((&raw mut gContestEffects).cast::<u8>()).wrapping_offset(
            (((((&raw mut gContestMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 8))
            .read()) as i32) as isize
                * 4,
        ))
        .wrapping_add(2))
        .read();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut newMove: u16 = ((((((&raw mut gContestMons).cast::<u8>())
                        .wrapping_offset(
                            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(12)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(65))
                            .read()) as i32) as isize
                                * 64,
                        ))
                    .wrapping_add(30))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read();
                    if (((newMove) as i32) != 0i32)
                        && (((jam) as i32)
                            < ((((((&raw mut gContestEffects).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gContestMoves).cast::<u8>())
                                    .wrapping_offset(((newMove) as i32) as isize * 8))
                                .read()) as i32) as isize
                                    * 4,
                            ))
                            .wrapping_add(2))
                            .read()) as i32))
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if i == 4i32 {
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(24)
            .cast::<i16>())
            .write(1i16);
        } else {
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(24)
            .cast::<i16>())
            .write(0i16);
        }
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_most_jamming_move() {
    unsafe {
        ContestAICmd_check_most_jamming_move();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != 0i32
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_num_move_hearts() {
    unsafe {
        let mut r#move: u16 = ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(65))
            .read()) as i32) as isize
                * 64,
        ))
        .wrapping_add(30))
        .cast::<u16>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .read()) as i32) as isize,
        ))
        .read();
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(
            ((crate::c::div_i32(
                ((((((&raw mut gContestEffects).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gContestMoves).cast::<u8>())
                        .wrapping_offset(((r#move) as i32) as isize * 8))
                    .read()) as i32) as isize
                        * 4,
                ))
                .wrapping_add(1))
                .read()) as i32),
                10i32,
            )) as i16),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_num_move_hearts_less_than() {
    unsafe {
        ContestAICmd_get_num_move_hearts();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            < (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_num_move_hearts_more_than() {
    unsafe {
        ContestAICmd_get_num_move_hearts();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            > (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_num_move_hearts_eq() {
    unsafe {
        ContestAICmd_get_num_move_hearts();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_num_move_hearts_not_eq() {
    unsafe {
        ContestAICmd_get_num_move_hearts();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_num_move_jam_hearts() {
    unsafe {
        let mut r#move: u16 = ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(65))
            .read()) as i32) as isize
                * 64,
        ))
        .wrapping_add(30))
        .cast::<u16>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .read()) as i32) as isize,
        ))
        .read();
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(
            ((crate::c::div_i32(
                ((((((&raw mut gContestEffects).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gContestMoves).cast::<u8>())
                        .wrapping_offset(((r#move) as i32) as isize * 8))
                    .read()) as i32) as isize
                        * 4,
                ))
                .wrapping_add(2))
                .read()) as i32),
                10i32,
            )) as i16),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_num_move_jam_hearts_less_than() {
    unsafe {
        ContestAICmd_get_num_move_jam_hearts();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            < (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_num_move_jam_hearts_more_than() {
    unsafe {
        ContestAICmd_get_num_move_jam_hearts();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            > (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_num_move_jam_hearts_eq() {
    unsafe {
        ContestAICmd_get_num_move_jam_hearts();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_num_move_jam_hearts_not_eq() {
    unsafe {
        ContestAICmd_get_num_move_jam_hearts();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_move_used_count() {
    unsafe {
        let mut result: i16 = 0i16;
        let mut r#move: u16 = ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(65))
            .read()) as i32) as isize
                * 64,
        ))
        .wrapping_add(30))
        .cast::<u16>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .read()) as i32) as isize,
        ))
        .read();
        if ((r#move) as i32)
            != (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(65))
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(8)
            .cast::<u16>())
            .read()) as i32)
        {
            result = 0i16;
        } else {
            result = ((((crate::c::bf_read(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(65))
                    .read()) as i32) as isize
                        * 28,
                ))
                .wrapping_add(11),
                4,
                3,
                false,
            ) as u8) as i32)
                .wrapping_add(1i32)) as i16);
        }
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(result);
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_most_used_count_less_than() {
    unsafe {
        ContestAICmd_get_move_used_count();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            < (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_most_used_count_more_than() {
    unsafe {
        ContestAICmd_get_move_used_count();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            > (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_most_used_count_eq() {
    unsafe {
        ContestAICmd_get_move_used_count();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_most_used_count_not_eq() {
    unsafe {
        ContestAICmd_get_move_used_count();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_check_combo_starter() {
    unsafe {
        let mut result: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut r#move: u16 = ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(65))
            .read()) as i32) as isize
                * 64,
        ))
        .wrapping_add(30))
        .cast::<u16>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .read()) as i32) as isize,
        ))
        .read();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(12)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(65))
                        .read()) as i32) as isize
                            * 64,
                    ))
                    .wrapping_add(30))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read())
                        != 0
                    {
                        result = AreMovesContestCombo(
                            r#move,
                            ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(12)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(65))
                                .read()) as i32) as isize
                                    * 64,
                            ))
                            .wrapping_add(30))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                        if (result) != 0 {
                            result = 1u8;
                            break 'l1;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (result) != 0 {
            result = 1u8;
        }
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(((result) as i16));
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_combo_starter() {
    unsafe {
        ContestAICmd_check_combo_starter();
        if (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read())
            != 0
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_not_combo_starter() {
    unsafe {
        ContestAICmd_check_combo_starter();
        if !((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read())
            != 0)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_check_combo_finisher() {
    unsafe {
        let mut result: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut r#move: u16 = ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(65))
            .read()) as i32) as isize
                * 64,
        ))
        .wrapping_add(30))
        .cast::<u16>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .read()) as i32) as isize,
        ))
        .read();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(12)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(65))
                        .read()) as i32) as isize
                            * 64,
                    ))
                    .wrapping_add(30))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read())
                        != 0
                    {
                        result = AreMovesContestCombo(
                            ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(12)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(65))
                                .read()) as i32) as isize
                                    * 64,
                            ))
                            .wrapping_add(30))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read(),
                            r#move,
                        );
                        if (result) != 0 {
                            result = 1u8;
                            break 'l1;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (result) != 0 {
            result = 1u8;
        }
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(((result) as i16));
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_combo_finisher() {
    unsafe {
        ContestAICmd_check_combo_finisher();
        if (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read())
            != 0
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_not_combo_finisher() {
    unsafe {
        ContestAICmd_check_combo_finisher();
        if !((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read())
            != 0)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_check_would_finish_combo() {
    unsafe {
        let mut result: u8 = 0u8;
        let mut r#move: u16 = ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(65))
            .read()) as i32) as isize
                * 64,
        ))
        .wrapping_add(30))
        .cast::<u16>())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .read()) as i32) as isize,
        ))
        .read();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(65))
            .read()) as i32) as isize
                * 28,
        ))
        .wrapping_add(8)
        .cast::<u16>())
        .read())
            != 0
        {
            result = AreMovesContestCombo(
                (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(65))
                    .read()) as i32) as isize
                        * 28,
                ))
                .wrapping_add(8)
                .cast::<u16>())
                .read(),
                r#move,
            );
        }
        if (result) != 0 {
            result = 1u8;
        }
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(((result) as i16));
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_would_finish_combo() {
    unsafe {
        ContestAICmd_check_would_finish_combo();
        if (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read())
            != 0
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_would_not_finish_combo() {
    unsafe {
        ContestAICmd_check_would_finish_combo();
        if !((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read())
            != 0)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_condition() {
    unsafe {
        let mut contestant: u8 = GetContestantIdByTurn(
            ((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read(),
        );
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(
            ((crate::c::div_i32(
                (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(13)
                .cast::<i8>())
                .read()) as i32),
                10i32,
            )) as i16),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_condition_less_than() {
    unsafe {
        ContestAICmd_get_condition();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            < (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_condition_more_than() {
    unsafe {
        ContestAICmd_get_condition();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            > (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_condition_eq() {
    unsafe {
        ContestAICmd_get_condition();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_condition_not_eq() {
    unsafe {
        ContestAICmd_get_condition();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_used_combo_starter() {
    unsafe {
        let mut result: u16 = 0u16;
        let mut contestant: u8 = GetContestantIdByTurn(
            ((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read(),
        );
        if (IsContestantAllowedToCombo(contestant)) != 0 {
            result = ((if (((((&raw mut gContestMoves).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(8)
                .cast::<u16>())
                .read()) as i32) as isize
                    * 8,
            ))
            .wrapping_add(2))
            .read())
                != 0
            {
                1i32
            } else {
                0i32
            }) as u16);
        }
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(((result) as i16));
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_combo_starter_less_than() {
    unsafe {
        ContestAICmd_get_used_combo_starter();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            < (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_combo_starter_more_than() {
    unsafe {
        ContestAICmd_get_used_combo_starter();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            > (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_combo_starter_eq() {
    unsafe {
        ContestAICmd_get_used_combo_starter();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_combo_starter_not_eq() {
    unsafe {
        ContestAICmd_get_used_combo_starter();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_check_can_participate() {
    unsafe {
        if (Contest_IsMonsTurnDisabled(GetContestantIdByTurn(
            ((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read(),
        ))) != 0
        {
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(24)
            .cast::<i16>())
            .write(0i16);
        } else {
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(24)
            .cast::<i16>())
            .write(1i16);
        }
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_can_participate() {
    unsafe {
        ContestAICmd_check_can_participate();
        if (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read())
            != 0
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_cannot_participate() {
    unsafe {
        ContestAICmd_check_can_participate();
        if !((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read())
            != 0)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_completed_combo() {
    unsafe {
        let mut contestant: u8 = GetContestantIdByTurn(
            ((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read(),
        );
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(
            ((crate::c::bf_read(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(21),
                3,
                1,
                false,
            ) as u8) as i16),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_completed_combo() {
    unsafe {
        ContestAICmd_get_completed_combo();
        if (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read())
            != 0
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_not_completed_combo() {
    unsafe {
        ContestAICmd_get_completed_combo();
        if !((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read())
            != 0)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_points_diff() {
    unsafe {
        let mut contestant: u8 = GetContestantIdByTurn(
            ((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read(),
        );
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(
            (((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(4)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_sub(
                    (((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(12)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(65))
                        .read()) as i32) as isize
                            * 28,
                    ))
                    .wrapping_add(4)
                    .cast::<i16>())
                    .read()) as i32),
                )) as i16),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_points_more_than_mon() {
    unsafe {
        ContestAICmd_get_points_diff();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            < 0i32
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_points_less_than_mon() {
    unsafe {
        ContestAICmd_get_points_diff();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            > 0i32
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_points_eq_mon() {
    unsafe {
        ContestAICmd_get_points_diff();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_points_not_eq_mon() {
    unsafe {
        ContestAICmd_get_points_diff();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != 0i32
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_preliminary_points_diff() {
    unsafe {
        let mut contestant: u8 = GetContestantIdByTurn(
            ((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read(),
        );
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(
            ((((((((&raw mut gContestMonRound1Points).cast::<i16>()).cast::<i16>())
                .wrapping_offset(((contestant) as i32) as isize))
            .read()) as i32)
                .wrapping_sub(
                    ((((((&raw mut gContestMonRound1Points).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(
                            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(12)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(65))
                            .read()) as i32) as isize,
                        ))
                    .read()) as i32),
                )) as i16),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_preliminary_points_more_than_mon() {
    unsafe {
        ContestAICmd_get_preliminary_points_diff();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            < 0i32
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_preliminary_points_less_than_mon() {
    unsafe {
        ContestAICmd_get_preliminary_points_diff();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            > 0i32
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_preliminary_points_eq_mon() {
    unsafe {
        ContestAICmd_get_preliminary_points_diff();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_preliminary_points_not_eq_mon() {
    unsafe {
        ContestAICmd_get_preliminary_points_diff();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != 0i32
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_used_moves_effect() {
    unsafe {
        let mut contestant: u8 = GetContestantIdByTurn(
            ((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read(),
        );
        let mut round: u8 =
            ((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2)).read();
        let mut r#move: u16 = ((((((((((&raw mut gContestResources).cast::<*mut u8>())
            .read())
        .cast::<*mut u8>())
        .read())
        .wrapping_add(28))
        .cast::<u8>())
        .wrapping_offset(((round) as i32) as isize * 8))
        .cast::<u16>())
        .wrapping_offset(((contestant) as i32) as isize))
        .read();
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(
            (((((&raw mut gContestMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 8))
            .read()) as i16),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(3));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_effect_less_than() {
    unsafe {
        ContestAICmd_get_used_moves_effect();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            < (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_effect_more_than() {
    unsafe {
        ContestAICmd_get_used_moves_effect();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            > (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_effect_eq() {
    unsafe {
        ContestAICmd_get_used_moves_effect();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_effect_not_eq() {
    unsafe {
        ContestAICmd_get_used_moves_effect();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_used_moves_excitement() {
    unsafe {
        let mut contestant: u8 = GetContestantIdByTurn(
            ((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read(),
        );
        let mut round: u8 =
            ((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2)).read();
        let mut result: i8 = ((((((((((((&raw mut gContestResources).cast::<*mut u8>())
            .read())
        .cast::<*mut u8>())
        .read())
        .wrapping_add(68))
        .cast::<u8>())
        .wrapping_offset(((round) as i32) as isize * 4))
        .cast::<u8>())
        .wrapping_offset(((contestant) as i32) as isize))
        .read()) as i8);
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(((result) as i16));
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(3));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_excitement_less_than() {
    unsafe {
        ContestAICmd_get_used_moves_excitement();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            < (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_excitement_more_than() {
    unsafe {
        ContestAICmd_get_used_moves_excitement();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            > (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_excitement_eq() {
    unsafe {
        ContestAICmd_get_used_moves_excitement();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_excitement_not_eq() {
    unsafe {
        ContestAICmd_get_used_moves_excitement();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_used_moves_effect_type() {
    unsafe {
        let mut contestant: u8 = GetContestantIdByTurn(
            ((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read(),
        );
        let mut round: u8 =
            ((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2)).read();
        let mut r#move: u16 = ((((((((((&raw mut gContestResources).cast::<*mut u8>())
            .read())
        .cast::<*mut u8>())
        .read())
        .wrapping_add(28))
        .cast::<u8>())
        .wrapping_offset(((round) as i32) as isize * 8))
        .cast::<u16>())
        .wrapping_offset(((contestant) as i32) as isize))
        .read();
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(
            (((((&raw mut gContestEffects).cast::<u8>()).wrapping_offset(
                (((((&raw mut gContestMoves).cast::<u8>())
                    .wrapping_offset(((r#move) as i32) as isize * 8))
                .read()) as i32) as isize
                    * 4,
            ))
            .read()) as i16),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(3));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_effect_type_eq() {
    unsafe {
        ContestAICmd_get_used_moves_effect_type();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            == (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_effect_type_not_eq() {
    unsafe {
        ContestAICmd_get_used_moves_effect_type();
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read()) as i32)
            != (((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_save_result() {
    unsafe {
        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(26))
        .cast::<i16>())
        .wrapping_offset(
            ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                as i32) as isize,
        ))
        .write(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(24)
            .cast::<i16>())
            .read(),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_setvar() {
    unsafe {
        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(26))
        .cast::<i16>())
        .wrapping_offset(
            ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                as i32) as isize,
        ))
        .write(
            ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2)).read())
                as i32)
                | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .wrapping_offset(1))
                .read()) as i32)
                    << 8)) as i16),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(4));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_add() {
    unsafe {
        let __p1 = (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(26))
        .cast::<i16>())
        .wrapping_offset(
            ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                as i32) as isize,
        );
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .read()) as i8) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 8)),
            )) as i16),
        );
        let __p2 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(4));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_addvar() {
    unsafe {
        let __p1 = (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(26))
        .cast::<i16>())
        .wrapping_offset(
            ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                as i32) as isize,
        );
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(26))
                .cast::<i16>())
                .wrapping_offset(
                    ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32) as isize,
                ))
                .read()) as i32),
            )) as i16),
        );
        let __p2 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(3));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_addvar_duplicate() {
    unsafe {
        let __p1 = (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(26))
        .cast::<i16>())
        .wrapping_offset(
            ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                as i32) as isize,
        );
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(26))
                .cast::<i16>())
                .wrapping_offset(
                    ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32) as isize,
                ))
                .read()) as i32),
            )) as i16),
        );
        let __p2 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(3));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_less_than() {
    unsafe {
        if ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(26))
        .cast::<i16>())
        .wrapping_offset(
            ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                as i32) as isize,
        ))
        .read()) as i32)
            < (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2)).read())
                as i32)
                | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .wrapping_offset(1))
                .read()) as i32)
                    << 8))
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(4))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(8));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_greater_than() {
    unsafe {
        if ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(26))
        .cast::<i16>())
        .wrapping_offset(
            ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                as i32) as isize,
        ))
        .read()) as i32)
            > (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2)).read())
                as i32)
                | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .wrapping_offset(1))
                .read()) as i32)
                    << 8))
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(4))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(8));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_eq() {
    unsafe {
        if ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(26))
        .cast::<i16>())
        .wrapping_offset(
            ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                as i32) as isize,
        ))
        .read()) as i32)
            == (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2)).read())
                as i32)
                | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .wrapping_offset(1))
                .read()) as i32)
                    << 8))
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(4))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(8));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_not_eq() {
    unsafe {
        if ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(26))
        .cast::<i16>())
        .wrapping_offset(
            ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                as i32) as isize,
        ))
        .read()) as i32)
            != (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2)).read())
                as i32)
                | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .wrapping_offset(1))
                .read()) as i32)
                    << 8))
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(4))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(8));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_less_than_var() {
    unsafe {
        if ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(26))
        .cast::<i16>())
        .wrapping_offset(
            ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                as i32) as isize,
        ))
        .read()) as i32)
            < ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(26))
            .cast::<i16>())
            .wrapping_offset(
                ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2)).read())
                    as i32) as isize,
            ))
            .read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(7));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_greater_than_var() {
    unsafe {
        if ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(26))
        .cast::<i16>())
        .wrapping_offset(
            ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                as i32) as isize,
        ))
        .read()) as i32)
            > ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(26))
            .cast::<i16>())
            .wrapping_offset(
                ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2)).read())
                    as i32) as isize,
            ))
            .read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(7));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_eq_var() {
    unsafe {
        if ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(26))
        .cast::<i16>())
        .wrapping_offset(
            ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                as i32) as isize,
        ))
        .read()) as i32)
            == ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(26))
            .cast::<i16>())
            .wrapping_offset(
                ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2)).read())
                    as i32) as isize,
            ))
            .read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(7));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_not_eq_var() {
    unsafe {
        if ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(26))
        .cast::<i16>())
        .wrapping_offset(
            ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                as i32) as isize,
        ))
        .read()) as i32)
            != ((((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(26))
            .cast::<i16>())
            .wrapping_offset(
                ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2)).read())
                    as i32) as isize,
            ))
            .read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(7));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_random_less_than() {
    unsafe {
        if (((Random()) as i32) & 255i32)
            < ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_random_greater_than() {
    unsafe {
        if (((Random()) as i32) & 255i32)
            > ((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_goto() {
    unsafe {
        ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
            ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                as i32)
                | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .wrapping_offset(1))
                .read()) as i32)
                    << 8))
                | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .wrapping_offset(2))
                .read()) as i32)
                    << 16))
                | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .wrapping_offset(3))
                .read()) as i32)
                    << 24)) as usize as *mut u8),
        );
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_call() {
    unsafe {
        AIStackPushVar((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(5));
        ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
            ((((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1)).read())
                as i32)
                | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .wrapping_offset(1))
                .read()) as i32)
                    << 8))
                | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .wrapping_offset(2))
                .read()) as i32)
                    << 16))
                | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .wrapping_offset(3))
                .read()) as i32)
                    << 24)) as usize as *mut u8),
        );
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_end() {
    unsafe {
        if !((AIStackPop()) != 0) {
            let __p1 = (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(9);
            (__p1).write((((((__p1).read()) as i32) | 1i32) as u8));
        }
    }
}
pub(crate) unsafe extern "C" fn AIStackPushVar(ptr: *mut u8) {
    unsafe {
        let mut ptr = ptr;
        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(32))
        .cast::<*mut u8>())
        .wrapping_offset(
            (({
                let __p1 = (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(64);
                let __t2 = (__p1).read();
                (__p1).write(((__p1).read()).wrapping_add(1));
                __t2
            }) as i32) as isize,
        ))
        .write(ptr);
    }
}
pub(crate) unsafe extern "C" fn AIStackPop() -> u8 {
    unsafe {
        if ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(64))
        .read()) as i32)
            != 0i32
        {
            let __p1 = (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(64);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32))
                .cast::<*mut u8>())
                .wrapping_offset(
                    ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(64))
                    .read()) as i32) as isize,
                ))
                .read(),
            );
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
pub(crate) unsafe extern "C" fn ContestAICmd_check_user_has_exciting_move() {
    unsafe {
        let mut result: i32 = 0i32;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(12)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(65))
                        .read()) as i32) as isize
                            * 64,
                    ))
                    .wrapping_add(30))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read())
                        != 0
                    {
                        if ((Contest_GetMoveExcitement(
                            ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(12)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(65))
                                .read()) as i32) as isize
                                    * 64,
                            ))
                            .wrapping_add(30))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        )) as i32)
                            == 1i32
                        {
                            result = 1i32;
                            break 'l1;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(((result) as i16));
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_has_exciting_move() {
    unsafe {
        ContestAICmd_check_user_has_exciting_move();
        if (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read())
            != 0
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_doesnt_have_exciting_move() {
    unsafe {
        ContestAICmd_check_user_has_exciting_move();
        if !((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read())
            != 0)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_check_user_has_move() {
    unsafe {
        let mut hasMove: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut targetMove: u16 = ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read())
            .wrapping_offset(1))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                .wrapping_offset(1))
            .read()) as i32)
                << 8)) as u16);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut r#move: u16 = ((((((&raw mut gContestMons).cast::<u8>())
                        .wrapping_offset(
                            ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(12)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(65))
                            .read()) as i32) as isize
                                * 64,
                        ))
                    .wrapping_add(30))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read();
                    if ((r#move) as i32) == ((targetMove) as i32) {
                        hasMove = 1i32;
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .write(((hasMove) as i16));
        let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(3));
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_has_move() {
    unsafe {
        ContestAICmd_check_user_has_move();
        if (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read())
            != 0
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_doesnt_have_move() {
    unsafe {
        ContestAICmd_check_user_has_move();
        if !((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(24)
        .cast::<i16>())
        .read())
            != 0)
        {
            ((&raw mut gAIScriptPtr).cast::<*mut u8>()).write(
                (((((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).read()) as i32)
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(1))
                        .read()) as i32)
                        << 8))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(2))
                        .read()) as i32)
                        << 16))
                    | (((((((&raw mut gAIScriptPtr).cast::<*mut u8>()).read()).wrapping_offset(3))
                        .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(4));
        }
    }
}
