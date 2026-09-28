//! Translated from `src/contest_ai.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sContestAICmdTable

const AI_ACTION_DONE: u8 = 1;

static sContestAICmdTable: Table<CArray<Option<unsafe extern "C" fn()>, 136>> =
    Table((&raw const crate::data::contest_ai::sContestAICmdTable).cast());

unsafe extern "C" {
    static mut gAIScriptPtr: *mut u8;
    static mut gContestAI_ScriptsTable: CArray<*mut u8, 0>;
    static gContestEffects: CArray<ContestEffect, 0>;
    static mut gContestMonRound1Points: CArray<i16, 4>;
    static mut gContestMons: CArray<ContestPokemon, 4>;
    static gContestMoves: CArray<ContestMove, 0>;
    static mut gContestResources: *mut ContestResources;
    static mut gSpecialVar_ContestCategory: u16;
    fn AreMovesContestCombo(a0: u16, a1: u16) -> u8;
    fn Contest_GetMoveExcitement(a0: u16) -> i8;
    fn Contest_IsMonsTurnDisabled(a0: u8) -> u8;
    fn IsContestantAllowedToCombo(a0: u8) -> u8;
    fn Random() -> u16;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestAI_ResetAI(contestantAI: u8) {
    let mut i: i32 = 0;
    memset((*gContestResources).aiData as *mut u8, 0, 68);
    i = 0;
    while i < MAX_MON_MOVES {
        (*(*gContestResources).aiData).moveScores[i] = 100;
        i += 1;
    }
    (*(*gContestResources).aiData).contestantId = contestantAI;
    (*(*gContestResources).aiData).stackSize = 0;
    (*(*gContestResources).aiData).aiFlags =
        gContestMons[(*(*gContestResources).aiData).contestantId].aiFlags;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestAI_GetActionToUse() -> u8 {
    while (*(*gContestResources).aiData).aiFlags != 0 {
        if (*(*gContestResources).aiData).aiFlags & 1 != 0 {
            (*(*gContestResources).aiData).aiState = CONTESTAI_SETTING_UP;
            ContestAI_DoAIProcessing();
        }
        (*(*gContestResources).aiData).aiFlags >>= 1;
        (*(*gContestResources).aiData).currentAIFlag += 1;
        (*(*gContestResources).aiData).nextMoveIndex = 0;
    }
    loop {
        let mut moveIndex: u8 = (if 0 != 0 {
            Random() as i32 % 4
        } else {
            Random() as i32 & 3
        }) as u8;
        let mut score: u8 = (*(*gContestResources).aiData).moveScores[moveIndex];
        let mut i: i32 = 0;
        i = 0;
        while i < MAX_MON_MOVES {
            if score < (*(*gContestResources).aiData).moveScores[i] {
                break;
            }
            i += 1;
        }
        if i == MAX_MON_MOVES {
            return moveIndex;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ContestAI_DoAIProcessing() {
    while (*(*gContestResources).aiData).aiState != CONTESTAI_FINISHED {
        match (*(*gContestResources).aiData).aiState {
            CONTESTAI_DO_NOT_PROCESS => {}
            CONTESTAI_SETTING_UP => {
                gAIScriptPtr =
                    gContestAI_ScriptsTable[(*(*gContestResources).aiData).currentAIFlag];
                if gContestMons[(*(*gContestResources).aiData).contestantId].moves
                    [(*(*gContestResources).aiData).nextMoveIndex]
                    == MOVE_NONE
                {
                    (*(*gContestResources).aiData).nextMove = MOVE_NONE;
                } else {
                    (*(*gContestResources).aiData).nextMove = gContestMons
                        [(*(*gContestResources).aiData).contestantId]
                        .moves[(*(*gContestResources).aiData).nextMoveIndex];
                }
                (*(*gContestResources).aiData).aiState += 1;
            }
            CONTESTAI_PROCESSING => {
                if (*(*gContestResources).aiData).nextMove != MOVE_NONE {
                    sContestAICmdTable[*gAIScriptPtr].unwrap_unchecked()();
                } else {
                    (*(*gContestResources).aiData).moveScores
                        [(*(*gContestResources).aiData).nextMoveIndex] = 0;
                    (*(*gContestResources).aiData).aiAction |= AI_ACTION_DONE;
                }
                if (*(*gContestResources).aiData).aiAction as i32 & AI_ACTION_DONE as i32 != 0 {
                    (*(*gContestResources).aiData).nextMoveIndex += 1;
                    if (*(*gContestResources).aiData).nextMoveIndex < MAX_MON_MOVES as u8 {
                        (*(*gContestResources).aiData).aiState = 0;
                    } else {
                        (*(*gContestResources).aiData).aiState += 1;
                    }
                    (*(*gContestResources).aiData).aiAction &= 254;
                }
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn GetContestantIdByTurn(turn: u8) -> u8 {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        if (*(*gContestResources).appealResults).turnOrder[i] == turn {
            break;
        }
        i += 1;
    }
    return i as u8;
}
pub(crate) unsafe extern "C" fn ContestAICmd_score() {
    let mut score: i16 = (*(*gContestResources).aiData).moveScores
        [(*(*gContestResources).aiData).nextMoveIndex] as i16
        + *gAIScriptPtr.at(1) as i8 as i16;
    if score > 255 {
        score = 255;
    } else if score < 0 {
        score = 0;
    }
    (*(*gContestResources).aiData).moveScores[(*(*gContestResources).aiData).nextMoveIndex] =
        score as u8;
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_appeal_num() {
    (*(*gContestResources).aiData).scriptResult =
        (*(*gContestResources).contest).appealNumber as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_appeal_num_less_than() {
    ContestAICmd_get_appeal_num();
    if (*(*gContestResources).aiData).scriptResult < *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_appeal_num_more_than() {
    ContestAICmd_get_appeal_num();
    if (*(*gContestResources).aiData).scriptResult > *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_appeal_num_eq() {
    ContestAICmd_get_appeal_num();
    if (*(*gContestResources).aiData).scriptResult == *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_appeal_num_not_eq() {
    ContestAICmd_get_appeal_num();
    if (*(*gContestResources).aiData).scriptResult != *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_excitement() {
    (*(*gContestResources).aiData).scriptResult =
        (*(*gContestResources).contest).applauseLevel as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_excitement_less_than() {
    ContestAICmd_get_excitement();
    if (*(*gContestResources).aiData).scriptResult < *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_excitement_more_than() {
    ContestAICmd_get_excitement();
    if (*(*gContestResources).aiData).scriptResult > *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_excitement_eq() {
    ContestAICmd_get_excitement();
    if (*(*gContestResources).aiData).scriptResult == *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_excitement_not_eq() {
    ContestAICmd_get_excitement();
    if (*(*gContestResources).aiData).scriptResult != *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_user_order() {
    (*(*gContestResources).aiData).scriptResult = (*(*gContestResources).appealResults).turnOrder
        [(*(*gContestResources).aiData).contestantId]
        as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_order_less_than() {
    ContestAICmd_get_user_order();
    if (*(*gContestResources).aiData).scriptResult < *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_order_more_than() {
    ContestAICmd_get_user_order();
    if (*(*gContestResources).aiData).scriptResult > *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_order_eq() {
    ContestAICmd_get_user_order();
    if (*(*gContestResources).aiData).scriptResult == *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_order_not_eq() {
    ContestAICmd_get_user_order();
    if (*(*gContestResources).aiData).scriptResult != *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_user_condition() {
    (*(*gContestResources).aiData).scriptResult = ((*(*gContestResources)
        .status
        .at((*(*gContestResources).aiData).contestantId))
    .condition
        / 10) as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_condition_less_than() {
    ContestAICmd_get_user_condition();
    if (*(*gContestResources).aiData).scriptResult < *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_condition_more_than() {
    ContestAICmd_get_user_condition();
    if (*(*gContestResources).aiData).scriptResult > *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_condition_eq() {
    ContestAICmd_get_user_condition();
    if (*(*gContestResources).aiData).scriptResult == *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_condition_not_eq() {
    ContestAICmd_get_user_condition();
    if (*(*gContestResources).aiData).scriptResult != *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_points() {
    (*(*gContestResources).aiData).scriptResult = (*(*gContestResources)
        .status
        .at((*(*gContestResources).aiData).contestantId))
    .pointTotal;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_points_less_than() {
    ContestAICmd_get_points();
    if ((*(*gContestResources).aiData).scriptResult as i32)
        < *gAIScriptPtr as i16 as i32 | (*gAIScriptPtr.at(1) as i16 as i32) << 8
    {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_points_more_than() {
    ContestAICmd_get_points();
    if (*(*gContestResources).aiData).scriptResult as i32
        > *gAIScriptPtr as i16 as i32 | (*gAIScriptPtr.at(1) as i16 as i32) << 8
    {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_points_eq() {
    ContestAICmd_get_points();
    if (*(*gContestResources).aiData).scriptResult as i32
        == *gAIScriptPtr as i16 as i32 | (*gAIScriptPtr.at(1) as i16 as i32) << 8
    {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_points_not_eq() {
    ContestAICmd_get_points();
    if (*(*gContestResources).aiData).scriptResult as i32
        != *gAIScriptPtr as i16 as i32 | (*gAIScriptPtr.at(1) as i16 as i32) << 8
    {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_preliminary_points() {
    (*(*gContestResources).aiData).scriptResult =
        gContestMonRound1Points[(*(*gContestResources).aiData).contestantId];
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_preliminary_points_less_than() {
    ContestAICmd_get_preliminary_points();
    if ((*(*gContestResources).aiData).scriptResult as i32)
        < *gAIScriptPtr as i16 as i32 | (*gAIScriptPtr.at(1) as i16 as i32) << 8
    {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_preliminary_points_more_than() {
    ContestAICmd_get_preliminary_points();
    if (*(*gContestResources).aiData).scriptResult as i32
        > *gAIScriptPtr as i16 as i32 | (*gAIScriptPtr.at(1) as i16 as i32) << 8
    {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_preliminary_points_eq() {
    ContestAICmd_get_preliminary_points();
    if (*(*gContestResources).aiData).scriptResult as i32
        == *gAIScriptPtr as i16 as i32 | (*gAIScriptPtr.at(1) as i16 as i32) << 8
    {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_preliminary_points_not_eq() {
    ContestAICmd_get_preliminary_points();
    if (*(*gContestResources).aiData).scriptResult as i32
        != *gAIScriptPtr as i16 as i32 | (*gAIScriptPtr.at(1) as i16 as i32) << 8
    {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_contest_type() {
    (*(*gContestResources).aiData).scriptResult = gSpecialVar_ContestCategory as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_contest_type_eq() {
    ContestAICmd_get_contest_type();
    if (*(*gContestResources).aiData).scriptResult == *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_contest_type_not_eq() {
    ContestAICmd_get_contest_type();
    if (*(*gContestResources).aiData).scriptResult != *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_move_excitement() {
    (*(*gContestResources).aiData).scriptResult = Contest_GetMoveExcitement(
        gContestMons[(*(*gContestResources).aiData).contestantId].moves
            [(*(*gContestResources).aiData).nextMoveIndex],
    ) as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_move_excitement_less_than() {
    ContestAICmd_get_move_excitement();
    if (*(*gContestResources).aiData).scriptResult < *gAIScriptPtr as i8 as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_move_excitement_more_than() {
    ContestAICmd_get_move_excitement();
    if (*(*gContestResources).aiData).scriptResult > *gAIScriptPtr as i8 as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_move_excitement_eq() {
    ContestAICmd_get_move_excitement();
    if (*(*gContestResources).aiData).scriptResult == *gAIScriptPtr as i8 as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_move_excitement_not_eq() {
    ContestAICmd_get_move_excitement();
    if (*(*gContestResources).aiData).scriptResult != *gAIScriptPtr as i8 as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_move_effect() {
    let mut r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    (*(*gContestResources).aiData).scriptResult = gContestMoves[r#move].effect as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_move_effect_eq() {
    ContestAICmd_get_move_effect();
    if (*(*gContestResources).aiData).scriptResult == *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_move_effect_not_eq() {
    ContestAICmd_get_move_effect();
    if (*(*gContestResources).aiData).scriptResult != *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_move_effect_type() {
    let mut r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    (*(*gContestResources).aiData).scriptResult =
        gContestEffects[gContestMoves[r#move].effect].effectType as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_move_effect_type_eq() {
    ContestAICmd_get_move_effect_type();
    if (*(*gContestResources).aiData).scriptResult == *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_move_effect_type_not_eq() {
    ContestAICmd_get_move_effect_type();
    if (*(*gContestResources).aiData).scriptResult != *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_check_most_appealing_move() {
    let mut i: i32 = 0;
    let mut r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    let mut appeal: u8 = gContestEffects[gContestMoves[r#move].effect].appeal;
    i = 0;
    while i < MAX_MON_MOVES {
        let mut newMove: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves[i];
        if newMove != 0 && appeal < gContestEffects[gContestMoves[newMove].effect].appeal {
            break;
        }
        i += 1;
    }
    if i == MAX_MON_MOVES {
        (*(*gContestResources).aiData).scriptResult = TRUE as i16;
    } else {
        (*(*gContestResources).aiData).scriptResult = FALSE as i16;
    }
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_most_appealing_move() {
    ContestAICmd_check_most_appealing_move();
    if (*(*gContestResources).aiData).scriptResult != FALSE as i16 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_check_most_jamming_move() {
    let mut i: i32 = 0;
    let mut r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    let mut jam: u8 = gContestEffects[gContestMoves[r#move].effect].jam;
    i = 0;
    while i < MAX_MON_MOVES {
        let mut newMove: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves[i];
        if newMove != MOVE_NONE && jam < gContestEffects[gContestMoves[newMove].effect].jam {
            break;
        }
        i += 1;
    }
    if i == MAX_MON_MOVES {
        (*(*gContestResources).aiData).scriptResult = TRUE as i16;
    } else {
        (*(*gContestResources).aiData).scriptResult = FALSE as i16;
    }
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_most_jamming_move() {
    ContestAICmd_check_most_jamming_move();
    if (*(*gContestResources).aiData).scriptResult != FALSE as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_num_move_hearts() {
    let mut r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    (*(*gContestResources).aiData).scriptResult =
        (gContestEffects[gContestMoves[r#move].effect].appeal as i32 / 10) as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_num_move_hearts_less_than() {
    ContestAICmd_get_num_move_hearts();
    if (*(*gContestResources).aiData).scriptResult < *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_num_move_hearts_more_than() {
    ContestAICmd_get_num_move_hearts();
    if (*(*gContestResources).aiData).scriptResult > *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_num_move_hearts_eq() {
    ContestAICmd_get_num_move_hearts();
    if (*(*gContestResources).aiData).scriptResult == *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_num_move_hearts_not_eq() {
    ContestAICmd_get_num_move_hearts();
    if (*(*gContestResources).aiData).scriptResult != *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_num_move_jam_hearts() {
    let mut r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    (*(*gContestResources).aiData).scriptResult =
        (gContestEffects[gContestMoves[r#move].effect].jam as i32 / 10) as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_num_move_jam_hearts_less_than() {
    ContestAICmd_get_num_move_jam_hearts();
    if (*(*gContestResources).aiData).scriptResult < *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_num_move_jam_hearts_more_than() {
    ContestAICmd_get_num_move_jam_hearts();
    if (*(*gContestResources).aiData).scriptResult > *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_num_move_jam_hearts_eq() {
    ContestAICmd_get_num_move_jam_hearts();
    if (*(*gContestResources).aiData).scriptResult == *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_num_move_jam_hearts_not_eq() {
    ContestAICmd_get_num_move_jam_hearts();
    if (*(*gContestResources).aiData).scriptResult != *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_move_used_count() {
    let mut result: i16 = 0;
    let mut r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    if r#move
        != (*(*gContestResources)
            .status
            .at((*(*gContestResources).aiData).contestantId))
        .prevMove
    {
        result = 0;
    } else {
        result = (*(*gContestResources)
            .status
            .at((*(*gContestResources).aiData).contestantId))
        .moveRepeatCount() as i16
            + 1;
    }
    (*(*gContestResources).aiData).scriptResult = result;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_most_used_count_less_than() {
    ContestAICmd_get_move_used_count();
    if (*(*gContestResources).aiData).scriptResult < *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_most_used_count_more_than() {
    ContestAICmd_get_move_used_count();
    if (*(*gContestResources).aiData).scriptResult > *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_most_used_count_eq() {
    ContestAICmd_get_move_used_count();
    if (*(*gContestResources).aiData).scriptResult == *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_most_used_count_not_eq() {
    ContestAICmd_get_move_used_count();
    if (*(*gContestResources).aiData).scriptResult != *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_check_combo_starter() {
    let mut result: u8 = 0;
    let mut i: i32 = 0;
    let mut r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    i = 0;
    while i < MAX_MON_MOVES {
        if gContestMons[(*(*gContestResources).aiData).contestantId].moves[i] != 0 {
            result = AreMovesContestCombo(
                r#move,
                gContestMons[(*(*gContestResources).aiData).contestantId].moves[i],
            );
            if result != 0 {
                result = 1;
                break;
            }
        }
        i += 1;
    }
    if result != 0 {
        result = 1;
    }
    (*(*gContestResources).aiData).scriptResult = result as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_combo_starter() {
    ContestAICmd_check_combo_starter();
    if (*(*gContestResources).aiData).scriptResult != 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_not_combo_starter() {
    ContestAICmd_check_combo_starter();
    if (*(*gContestResources).aiData).scriptResult == 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_check_combo_finisher() {
    let mut result: u8 = 0;
    let mut i: i32 = 0;
    let mut r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    i = 0;
    while i < MAX_MON_MOVES {
        if gContestMons[(*(*gContestResources).aiData).contestantId].moves[i] != 0 {
            result = AreMovesContestCombo(
                gContestMons[(*(*gContestResources).aiData).contestantId].moves[i],
                r#move,
            );
            if result != 0 {
                result = 1;
                break;
            }
        }
        i += 1;
    }
    if result != 0 {
        result = 1;
    }
    (*(*gContestResources).aiData).scriptResult = result as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_combo_finisher() {
    ContestAICmd_check_combo_finisher();
    if (*(*gContestResources).aiData).scriptResult != 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_not_combo_finisher() {
    ContestAICmd_check_combo_finisher();
    if (*(*gContestResources).aiData).scriptResult == 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_check_would_finish_combo() {
    let mut result: u8 = 0;
    let mut r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    if (*(*gContestResources)
        .status
        .at((*(*gContestResources).aiData).contestantId))
    .prevMove
        != 0
    {
        result = AreMovesContestCombo(
            (*(*gContestResources)
                .status
                .at((*(*gContestResources).aiData).contestantId))
            .prevMove,
            r#move,
        );
    }
    if result != 0 {
        result = 1;
    }
    (*(*gContestResources).aiData).scriptResult = result as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_would_finish_combo() {
    ContestAICmd_check_would_finish_combo();
    if (*(*gContestResources).aiData).scriptResult != 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_would_not_finish_combo() {
    ContestAICmd_check_would_finish_combo();
    if (*(*gContestResources).aiData).scriptResult == 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_condition() {
    let mut contestant: u8 = GetContestantIdByTurn(*gAIScriptPtr.at(1));
    (*(*gContestResources).aiData).scriptResult =
        ((*(*gContestResources).status.at(contestant)).condition / 10) as i16;
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_condition_less_than() {
    ContestAICmd_get_condition();
    if (*(*gContestResources).aiData).scriptResult < *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_condition_more_than() {
    ContestAICmd_get_condition();
    if (*(*gContestResources).aiData).scriptResult > *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_condition_eq() {
    ContestAICmd_get_condition();
    if (*(*gContestResources).aiData).scriptResult == *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_condition_not_eq() {
    ContestAICmd_get_condition();
    if (*(*gContestResources).aiData).scriptResult != *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_used_combo_starter() {
    let mut result: u16 = FALSE as u16;
    let mut contestant: u8 = GetContestantIdByTurn(*gAIScriptPtr.at(1));
    if IsContestantAllowedToCombo(contestant) != 0 {
        result = (if gContestMoves[(*(*gContestResources).status.at(contestant)).prevMove]
            .comboStarterId
            != 0
        {
            TRUE as i32
        } else {
            FALSE as i32
        }) as u16;
    }
    (*(*gContestResources).aiData).scriptResult = result as i16;
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_combo_starter_less_than() {
    ContestAICmd_get_used_combo_starter();
    if (*(*gContestResources).aiData).scriptResult < *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_combo_starter_more_than() {
    ContestAICmd_get_used_combo_starter();
    if (*(*gContestResources).aiData).scriptResult > *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_combo_starter_eq() {
    ContestAICmd_get_used_combo_starter();
    if (*(*gContestResources).aiData).scriptResult == *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_combo_starter_not_eq() {
    ContestAICmd_get_used_combo_starter();
    if (*(*gContestResources).aiData).scriptResult != *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_check_can_participate() {
    if Contest_IsMonsTurnDisabled(GetContestantIdByTurn(*gAIScriptPtr.at(1))) != 0 {
        (*(*gContestResources).aiData).scriptResult = FALSE as i16;
    } else {
        (*(*gContestResources).aiData).scriptResult = TRUE as i16;
    }
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_can_participate() {
    ContestAICmd_check_can_participate();
    if (*(*gContestResources).aiData).scriptResult != 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_cannot_participate() {
    ContestAICmd_check_can_participate();
    if (*(*gContestResources).aiData).scriptResult == 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_completed_combo() {
    let mut contestant: u8 = GetContestantIdByTurn(*gAIScriptPtr.at(1));
    (*(*gContestResources).aiData).scriptResult =
        (*(*gContestResources).status.at(contestant)).completedComboFlag() as i16;
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_completed_combo() {
    ContestAICmd_get_completed_combo();
    if (*(*gContestResources).aiData).scriptResult != 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_not_completed_combo() {
    ContestAICmd_get_completed_combo();
    if (*(*gContestResources).aiData).scriptResult == 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_points_diff() {
    let mut contestant: u8 = GetContestantIdByTurn(*gAIScriptPtr.at(1));
    (*(*gContestResources).aiData).scriptResult = (*(*gContestResources).status.at(contestant))
        .pointTotal
        - (*(*gContestResources)
            .status
            .at((*(*gContestResources).aiData).contestantId))
        .pointTotal;
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_points_more_than_mon() {
    ContestAICmd_get_points_diff();
    if (*(*gContestResources).aiData).scriptResult < 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_points_less_than_mon() {
    ContestAICmd_get_points_diff();
    if (*(*gContestResources).aiData).scriptResult > 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_points_eq_mon() {
    ContestAICmd_get_points_diff();
    if (*(*gContestResources).aiData).scriptResult == 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_points_not_eq_mon() {
    ContestAICmd_get_points_diff();
    if (*(*gContestResources).aiData).scriptResult != 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_preliminary_points_diff() {
    let mut contestant: u8 = GetContestantIdByTurn(*gAIScriptPtr.at(1));
    (*(*gContestResources).aiData).scriptResult = gContestMonRound1Points[contestant]
        - gContestMonRound1Points[(*(*gContestResources).aiData).contestantId];
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_preliminary_points_more_than_mon() {
    ContestAICmd_get_preliminary_points_diff();
    if (*(*gContestResources).aiData).scriptResult < 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_preliminary_points_less_than_mon() {
    ContestAICmd_get_preliminary_points_diff();
    if (*(*gContestResources).aiData).scriptResult > 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_preliminary_points_eq_mon() {
    ContestAICmd_get_preliminary_points_diff();
    if (*(*gContestResources).aiData).scriptResult == 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_preliminary_points_not_eq_mon() {
    ContestAICmd_get_preliminary_points_diff();
    if (*(*gContestResources).aiData).scriptResult != 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_used_moves_effect() {
    let mut contestant: u8 = GetContestantIdByTurn(*gAIScriptPtr.at(1));
    let mut round: u8 = *gAIScriptPtr.at(2);
    let mut r#move: u16 = (*(*gContestResources).contest).moveHistory[round][contestant];
    (*(*gContestResources).aiData).scriptResult = gContestMoves[r#move].effect as i16;
    gAIScriptPtr = gAIScriptPtr.at(3);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_effect_less_than() {
    ContestAICmd_get_used_moves_effect();
    if (*(*gContestResources).aiData).scriptResult < *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_effect_more_than() {
    ContestAICmd_get_used_moves_effect();
    if (*(*gContestResources).aiData).scriptResult > *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_effect_eq() {
    ContestAICmd_get_used_moves_effect();
    if (*(*gContestResources).aiData).scriptResult == *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_effect_not_eq() {
    ContestAICmd_get_used_moves_effect();
    if (*(*gContestResources).aiData).scriptResult != *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_used_moves_excitement() {
    let mut contestant: u8 = GetContestantIdByTurn(*gAIScriptPtr.at(1));
    let mut round: u8 = *gAIScriptPtr.at(2);
    let mut result: i8 = (*(*gContestResources).contest).excitementHistory[round][contestant] as i8;
    (*(*gContestResources).aiData).scriptResult = result as i16;
    gAIScriptPtr = gAIScriptPtr.at(3);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_excitement_less_than() {
    ContestAICmd_get_used_moves_excitement();
    if (*(*gContestResources).aiData).scriptResult < *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_excitement_more_than() {
    ContestAICmd_get_used_moves_excitement();
    if (*(*gContestResources).aiData).scriptResult > *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_excitement_eq() {
    ContestAICmd_get_used_moves_excitement();
    if (*(*gContestResources).aiData).scriptResult == *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_excitement_not_eq() {
    ContestAICmd_get_used_moves_excitement();
    if (*(*gContestResources).aiData).scriptResult != *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_get_used_moves_effect_type() {
    let mut contestant: u8 = GetContestantIdByTurn(*gAIScriptPtr.at(1));
    let mut round: u8 = *gAIScriptPtr.at(2);
    let mut r#move: u16 = (*(*gContestResources).contest).moveHistory[round][contestant];
    (*(*gContestResources).aiData).scriptResult =
        gContestEffects[gContestMoves[r#move].effect].effectType as i16;
    gAIScriptPtr = gAIScriptPtr.at(3);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_effect_type_eq() {
    ContestAICmd_get_used_moves_effect_type();
    if (*(*gContestResources).aiData).scriptResult == *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_used_moves_effect_type_not_eq() {
    ContestAICmd_get_used_moves_effect_type();
    if (*(*gContestResources).aiData).scriptResult != *gAIScriptPtr as i16 {
        gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
            | (*gAIScriptPtr.at(1).at(1) as i32) << 8
            | (*gAIScriptPtr.at(1).at(2) as i32) << 16
            | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(5);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_save_result() {
    (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)] =
        (*(*gContestResources).aiData).scriptResult;
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn ContestAICmd_setvar() {
    (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)] =
        *gAIScriptPtr.at(2) as i16 | (*gAIScriptPtr.at(2).at(1) as i16) << 8;
    gAIScriptPtr = gAIScriptPtr.at(4);
}
pub(crate) unsafe extern "C" fn ContestAICmd_add() {
    (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)] +=
        *gAIScriptPtr.at(2) as i8 as i16 | (*gAIScriptPtr.at(3) as i16) << 8;
    gAIScriptPtr = gAIScriptPtr.at(4);
}
pub(crate) unsafe extern "C" fn ContestAICmd_addvar() {
    (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)] +=
        (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(2)];
    gAIScriptPtr = gAIScriptPtr.at(3);
}
pub(crate) unsafe extern "C" fn ContestAICmd_addvar_duplicate() {
    (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)] +=
        (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(2)];
    gAIScriptPtr = gAIScriptPtr.at(3);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_less_than() {
    if ((*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)] as i32)
        < *gAIScriptPtr.at(2) as i32 | (*gAIScriptPtr.at(2).at(1) as i32) << 8
    {
        gAIScriptPtr = (*gAIScriptPtr.at(4) as i32
            | (*gAIScriptPtr.at(4).at(1) as i32) << 8
            | (*gAIScriptPtr.at(4).at(2) as i32) << 16
            | (*gAIScriptPtr.at(4).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(8);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_greater_than() {
    if (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)] as i32
        > *gAIScriptPtr.at(2) as i32 | (*gAIScriptPtr.at(2).at(1) as i32) << 8
    {
        gAIScriptPtr = (*gAIScriptPtr.at(4) as i32
            | (*gAIScriptPtr.at(4).at(1) as i32) << 8
            | (*gAIScriptPtr.at(4).at(2) as i32) << 16
            | (*gAIScriptPtr.at(4).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(8);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_eq() {
    if (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)] as i32
        == *gAIScriptPtr.at(2) as i32 | (*gAIScriptPtr.at(2).at(1) as i32) << 8
    {
        gAIScriptPtr = (*gAIScriptPtr.at(4) as i32
            | (*gAIScriptPtr.at(4).at(1) as i32) << 8
            | (*gAIScriptPtr.at(4).at(2) as i32) << 16
            | (*gAIScriptPtr.at(4).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(8);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_not_eq() {
    if (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)] as i32
        != *gAIScriptPtr.at(2) as i32 | (*gAIScriptPtr.at(2).at(1) as i32) << 8
    {
        gAIScriptPtr = (*gAIScriptPtr.at(4) as i32
            | (*gAIScriptPtr.at(4).at(1) as i32) << 8
            | (*gAIScriptPtr.at(4).at(2) as i32) << 16
            | (*gAIScriptPtr.at(4).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(8);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_less_than_var() {
    if (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)]
        < (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(2)]
    {
        gAIScriptPtr = (*gAIScriptPtr.at(3) as i32
            | (*gAIScriptPtr.at(3).at(1) as i32) << 8
            | (*gAIScriptPtr.at(3).at(2) as i32) << 16
            | (*gAIScriptPtr.at(3).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(7);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_greater_than_var() {
    if (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)]
        > (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(2)]
    {
        gAIScriptPtr = (*gAIScriptPtr.at(3) as i32
            | (*gAIScriptPtr.at(3).at(1) as i32) << 8
            | (*gAIScriptPtr.at(3).at(2) as i32) << 16
            | (*gAIScriptPtr.at(3).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(7);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_eq_var() {
    if (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)]
        == (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(2)]
    {
        gAIScriptPtr = (*gAIScriptPtr.at(3) as i32
            | (*gAIScriptPtr.at(3).at(1) as i32) << 8
            | (*gAIScriptPtr.at(3).at(2) as i32) << 16
            | (*gAIScriptPtr.at(3).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(7);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_not_eq_var() {
    if (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)]
        != (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(2)]
    {
        gAIScriptPtr = (*gAIScriptPtr.at(3) as i32
            | (*gAIScriptPtr.at(3).at(1) as i32) << 8
            | (*gAIScriptPtr.at(3).at(2) as i32) << 16
            | (*gAIScriptPtr.at(3).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(7);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_random_less_than() {
    if Random() as i32 & 0xFF < *gAIScriptPtr.at(1) as i32 {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_random_greater_than() {
    if Random() as i32 & 0xFF > *gAIScriptPtr.at(1) as i32 {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_goto() {
    gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
        | (*gAIScriptPtr.at(1).at(1) as i32) << 8
        | (*gAIScriptPtr.at(1).at(2) as i32) << 16
        | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
}
pub(crate) unsafe extern "C" fn ContestAICmd_call() {
    AIStackPushVar(gAIScriptPtr.at(5));
    gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
        | (*gAIScriptPtr.at(1).at(1) as i32) << 8
        | (*gAIScriptPtr.at(1).at(2) as i32) << 16
        | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
}
pub(crate) unsafe extern "C" fn ContestAICmd_end() {
    if AIStackPop() == 0 {
        (*(*gContestResources).aiData).aiAction |= AI_ACTION_DONE;
    }
}
pub(crate) unsafe extern "C" fn AIStackPushVar(ptr: *mut u8) {
    (*(*gContestResources).aiData).stack[{
        let t1 = (*(*gContestResources).aiData).stackSize;
        (*(*gContestResources).aiData).stackSize += 1;
        t1
    }] = ptr;
}
pub(crate) unsafe extern "C" fn AIStackPop() -> u8 {
    if (*(*gContestResources).aiData).stackSize != 0 {
        (*(*gContestResources).aiData).stackSize -= 1;
        gAIScriptPtr =
            (*(*gContestResources).aiData).stack[(*(*gContestResources).aiData).stackSize];
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_check_user_has_exciting_move() {
    let mut result: i32 = 0;
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_MON_MOVES {
        if gContestMons[(*(*gContestResources).aiData).contestantId].moves[i] != 0 {
            if Contest_GetMoveExcitement(
                gContestMons[(*(*gContestResources).aiData).contestantId].moves[i],
            ) == 1
            {
                result = 1;
                break;
            }
        }
        i += 1;
    }
    (*(*gContestResources).aiData).scriptResult = result as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_has_exciting_move() {
    ContestAICmd_check_user_has_exciting_move();
    if (*(*gContestResources).aiData).scriptResult != 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_doesnt_have_exciting_move() {
    ContestAICmd_check_user_has_exciting_move();
    if (*(*gContestResources).aiData).scriptResult == 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_check_user_has_move() {
    let mut hasMove: i32 = FALSE as i32;
    let mut i: i32 = 0;
    let mut targetMove: u16 = *gAIScriptPtr.at(1) as u16 | (*gAIScriptPtr.at(1).at(1) as u16) << 8;
    i = 0;
    while i < MAX_MON_MOVES {
        let mut r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves[i];
        if r#move == targetMove {
            hasMove = TRUE as i32;
            break;
        }
        i += 1;
    }
    (*(*gContestResources).aiData).scriptResult = hasMove as i16;
    gAIScriptPtr = gAIScriptPtr.at(3);
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_has_move() {
    ContestAICmd_check_user_has_move();
    if (*(*gContestResources).aiData).scriptResult != 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn ContestAICmd_if_user_doesnt_have_move() {
    ContestAICmd_check_user_has_move();
    if (*(*gContestResources).aiData).scriptResult == 0 {
        gAIScriptPtr = (*gAIScriptPtr as i32
            | (*gAIScriptPtr.at(1) as i32) << 8
            | (*gAIScriptPtr.at(2) as i32) << 16
            | (*gAIScriptPtr.at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(4);
    }
}
