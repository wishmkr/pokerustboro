//! Translated from `src/contest_ai.c` by tools/rustport/c2rs.py.
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
    clippy::eq_op,
    clippy::manual_clamp,
    clippy::type_complexity,
    unused_assignments
)]

use crate::battle_ai_script_commands::gAIScriptPtr;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::contest::gContestMonRound1Points;
use crate::contest::{
    Contest_GetMoveExcitement, Contest_IsMonsTurnDisabled, IsContestantAllowedToCombo,
    gContestMons, gContestResources, gSpecialVar_ContestCategory,
};
use crate::contest_effect::AreMovesContestCombo;
use crate::random::Random;
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

static sContestAICmdTable: Table<CArray<Option<unsafe fn()>, 136>> =
    Table((&raw const crate::data::contest_ai::sContestAICmdTable).cast());

pub unsafe fn ContestAI_ResetAI(contestantAI: u8) {
    memset((*gContestResources).aiData as *mut u8, 0, 68);
    for i in 0..MAX_MON_MOVES {
        (*(*gContestResources).aiData).moveScores[i] = 100;
    }
    (*(*gContestResources).aiData).contestantId = contestantAI;
    (*(*gContestResources).aiData).stackSize = 0;
    (*(*gContestResources).aiData).aiFlags =
        gContestMons[(*(*gContestResources).aiData).contestantId].aiFlags;
}
pub unsafe fn ContestAI_GetActionToUse() -> u8 {
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
        let moveIndex: u8 = (if 0 != 0 {
            Random() as i32 % 4
        } else {
            Random() as i32 & 3
        }) as u8;
        let score: u8 = (*(*gContestResources).aiData).moveScores[moveIndex];
        let mut i: i32 = 0;
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
        0
    }
}
unsafe fn ContestAI_DoAIProcessing() {
    while (*(*gContestResources).aiData).aiState != CONTESTAI_FINISHED {
        match (*(*gContestResources).aiData).aiState {
            CONTESTAI_DO_NOT_PROCESS => {}
            CONTESTAI_SETTING_UP => {
                gAIScriptPtr = (*crate::asmdata::gContestAI_ScriptsTable
                    .cast::<CArray<*mut u8, 0>>()
                    .cast_mut())[(*(*gContestResources).aiData).currentAIFlag];
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
unsafe fn GetContestantIdByTurn(turn: u8) -> u8 {
    let mut i: i32 = 0;
    while i < CONTESTANT_COUNT {
        if (*(*gContestResources).appealResults).turnOrder[i] == turn {
            break;
        }
        i += 1;
    }
    i as u8
}
pub(crate) unsafe fn ContestAICmd_score() {
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
pub(crate) unsafe fn ContestAICmd_get_appeal_num() {
    (*(*gContestResources).aiData).scriptResult =
        (*(*gContestResources).contest).appealNumber as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe fn ContestAICmd_if_appeal_num_less_than() {
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
pub(crate) unsafe fn ContestAICmd_if_appeal_num_more_than() {
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
pub(crate) unsafe fn ContestAICmd_if_appeal_num_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_appeal_num_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_get_excitement() {
    (*(*gContestResources).aiData).scriptResult =
        (*(*gContestResources).contest).applauseLevel as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe fn ContestAICmd_if_excitement_less_than() {
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
pub(crate) unsafe fn ContestAICmd_if_excitement_more_than() {
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
pub(crate) unsafe fn ContestAICmd_if_excitement_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_excitement_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_get_user_order() {
    (*(*gContestResources).aiData).scriptResult = (*(*gContestResources).appealResults).turnOrder
        [(*(*gContestResources).aiData).contestantId]
        as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe fn ContestAICmd_if_user_order_less_than() {
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
pub(crate) unsafe fn ContestAICmd_if_user_order_more_than() {
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
pub(crate) unsafe fn ContestAICmd_if_user_order_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_user_order_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_get_user_condition() {
    (*(*gContestResources).aiData).scriptResult = ((*(*gContestResources)
        .status
        .at((*(*gContestResources).aiData).contestantId))
    .condition
        / 10) as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe fn ContestAICmd_if_user_condition_less_than() {
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
pub(crate) unsafe fn ContestAICmd_if_user_condition_more_than() {
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
pub(crate) unsafe fn ContestAICmd_if_user_condition_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_user_condition_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_get_points() {
    (*(*gContestResources).aiData).scriptResult = (*(*gContestResources)
        .status
        .at((*(*gContestResources).aiData).contestantId))
    .pointTotal;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe fn ContestAICmd_if_points_less_than() {
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
pub(crate) unsafe fn ContestAICmd_if_points_more_than() {
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
pub(crate) unsafe fn ContestAICmd_if_points_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_points_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_get_preliminary_points() {
    (*(*gContestResources).aiData).scriptResult =
        gContestMonRound1Points[(*(*gContestResources).aiData).contestantId];
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe fn ContestAICmd_if_preliminary_points_less_than() {
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
pub(crate) unsafe fn ContestAICmd_if_preliminary_points_more_than() {
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
pub(crate) unsafe fn ContestAICmd_if_preliminary_points_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_preliminary_points_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_get_contest_type() {
    (*(*gContestResources).aiData).scriptResult = gSpecialVar_ContestCategory as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe fn ContestAICmd_if_contest_type_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_contest_type_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_get_move_excitement() {
    (*(*gContestResources).aiData).scriptResult = Contest_GetMoveExcitement(
        gContestMons[(*(*gContestResources).aiData).contestantId].moves
            [(*(*gContestResources).aiData).nextMoveIndex],
    ) as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe fn ContestAICmd_if_move_excitement_less_than() {
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
pub(crate) unsafe fn ContestAICmd_if_move_excitement_more_than() {
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
pub(crate) unsafe fn ContestAICmd_if_move_excitement_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_move_excitement_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_get_move_effect() {
    let r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    (*(*gContestResources).aiData).scriptResult =
        (*(&raw const crate::data::contest_effect::gContestMoves).cast::<CArray<ContestMove, 0>>())
            [r#move]
            .effect as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe fn ContestAICmd_if_move_effect_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_move_effect_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_get_move_effect_type() {
    let r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    (*(*gContestResources).aiData).scriptResult =
        (*(&raw const crate::data::contest_effect::gContestEffects)
            .cast::<CArray<ContestEffect, 0>>())
            [(*(&raw const crate::data::contest_effect::gContestMoves)
                .cast::<CArray<ContestMove, 0>>())[r#move]
                .effect]
            .effectType as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe fn ContestAICmd_if_move_effect_type_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_move_effect_type_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_check_most_appealing_move() {
    let r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    let appeal: u8 = (*(&raw const crate::data::contest_effect::gContestEffects)
        .cast::<CArray<ContestEffect, 0>>())
        [(*(&raw const crate::data::contest_effect::gContestMoves)
            .cast::<CArray<ContestMove, 0>>())[r#move]
            .effect]
        .appeal;
    let mut i: i32 = 0;
    while i < MAX_MON_MOVES {
        let newMove: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves[i];
        if newMove != 0
            && appeal
                < (*(&raw const crate::data::contest_effect::gContestEffects)
                    .cast::<CArray<ContestEffect, 0>>())
                    [(*(&raw const crate::data::contest_effect::gContestMoves).cast::<CArray<
                        ContestMove,
                        0,
                    >>(
                    ))[newMove]
                        .effect]
                    .appeal
        {
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
pub(crate) unsafe fn ContestAICmd_if_most_appealing_move() {
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
pub(crate) unsafe fn ContestAICmd_check_most_jamming_move() {
    let r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    let jam: u8 = (*(&raw const crate::data::contest_effect::gContestEffects)
        .cast::<CArray<ContestEffect, 0>>())
        [(*(&raw const crate::data::contest_effect::gContestMoves)
            .cast::<CArray<ContestMove, 0>>())[r#move]
            .effect]
        .jam;
    let mut i: i32 = 0;
    while i < MAX_MON_MOVES {
        let newMove: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves[i];
        if newMove != MOVE_NONE
            && jam
                < (*(&raw const crate::data::contest_effect::gContestEffects)
                    .cast::<CArray<ContestEffect, 0>>())
                    [(*(&raw const crate::data::contest_effect::gContestMoves).cast::<CArray<
                        ContestMove,
                        0,
                    >>(
                    ))[newMove]
                        .effect]
                    .jam
        {
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
pub(crate) unsafe fn ContestAICmd_if_most_jamming_move() {
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
pub(crate) unsafe fn ContestAICmd_get_num_move_hearts() {
    let r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    (*(*gContestResources).aiData).scriptResult =
        ((*(&raw const crate::data::contest_effect::gContestEffects)
            .cast::<CArray<ContestEffect, 0>>())
            [(*(&raw const crate::data::contest_effect::gContestMoves)
                .cast::<CArray<ContestMove, 0>>())[r#move]
                .effect]
            .appeal as i32
            / 10) as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe fn ContestAICmd_if_num_move_hearts_less_than() {
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
pub(crate) unsafe fn ContestAICmd_if_num_move_hearts_more_than() {
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
pub(crate) unsafe fn ContestAICmd_if_num_move_hearts_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_num_move_hearts_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_get_num_move_jam_hearts() {
    let r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    (*(*gContestResources).aiData).scriptResult =
        ((*(&raw const crate::data::contest_effect::gContestEffects)
            .cast::<CArray<ContestEffect, 0>>())
            [(*(&raw const crate::data::contest_effect::gContestMoves)
                .cast::<CArray<ContestMove, 0>>())[r#move]
                .effect]
            .jam as i32
            / 10) as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe fn ContestAICmd_if_num_move_jam_hearts_less_than() {
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
pub(crate) unsafe fn ContestAICmd_if_num_move_jam_hearts_more_than() {
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
pub(crate) unsafe fn ContestAICmd_if_num_move_jam_hearts_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_num_move_jam_hearts_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_get_move_used_count() {
    let mut result: i16 = 0;
    let r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
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
pub(crate) unsafe fn ContestAICmd_if_most_used_count_less_than() {
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
pub(crate) unsafe fn ContestAICmd_if_most_used_count_more_than() {
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
pub(crate) unsafe fn ContestAICmd_if_most_used_count_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_most_used_count_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_check_combo_starter() {
    let mut result: u8 = 0;
    let r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    for i in 0..MAX_MON_MOVES {
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
    }
    if result != 0 {
        result = 1;
    }
    (*(*gContestResources).aiData).scriptResult = result as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe fn ContestAICmd_if_combo_starter() {
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
pub(crate) unsafe fn ContestAICmd_if_not_combo_starter() {
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
pub(crate) unsafe fn ContestAICmd_check_combo_finisher() {
    let mut result: u8 = 0;
    let r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
        [(*(*gContestResources).aiData).nextMoveIndex];
    for i in 0..MAX_MON_MOVES {
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
    }
    if result != 0 {
        result = 1;
    }
    (*(*gContestResources).aiData).scriptResult = result as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe fn ContestAICmd_if_combo_finisher() {
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
pub(crate) unsafe fn ContestAICmd_if_not_combo_finisher() {
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
pub(crate) unsafe fn ContestAICmd_check_would_finish_combo() {
    let mut result: u8 = 0;
    let r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves
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
pub(crate) unsafe fn ContestAICmd_if_would_finish_combo() {
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
pub(crate) unsafe fn ContestAICmd_if_would_not_finish_combo() {
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
pub(crate) unsafe fn ContestAICmd_get_condition() {
    let contestant: u8 = GetContestantIdByTurn(*gAIScriptPtr.at(1));
    (*(*gContestResources).aiData).scriptResult =
        ((*(*gContestResources).status.at(contestant)).condition / 10) as i16;
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe fn ContestAICmd_if_condition_less_than() {
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
pub(crate) unsafe fn ContestAICmd_if_condition_more_than() {
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
pub(crate) unsafe fn ContestAICmd_if_condition_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_condition_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_get_used_combo_starter() {
    let mut result: u16 = FALSE as u16;
    let contestant: u8 = GetContestantIdByTurn(*gAIScriptPtr.at(1));
    if IsContestantAllowedToCombo(contestant) != 0 {
        result = (if (*(&raw const crate::data::contest_effect::gContestMoves)
            .cast::<CArray<ContestMove, 0>>())
            [(*(*gContestResources).status.at(contestant)).prevMove]
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
pub(crate) unsafe fn ContestAICmd_if_used_combo_starter_less_than() {
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
pub(crate) unsafe fn ContestAICmd_if_used_combo_starter_more_than() {
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
pub(crate) unsafe fn ContestAICmd_if_used_combo_starter_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_used_combo_starter_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_check_can_participate() {
    if Contest_IsMonsTurnDisabled(GetContestantIdByTurn(*gAIScriptPtr.at(1))) != 0 {
        (*(*gContestResources).aiData).scriptResult = FALSE as i16;
    } else {
        (*(*gContestResources).aiData).scriptResult = TRUE as i16;
    }
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe fn ContestAICmd_if_can_participate() {
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
pub(crate) unsafe fn ContestAICmd_if_cannot_participate() {
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
pub(crate) unsafe fn ContestAICmd_get_completed_combo() {
    let contestant: u8 = GetContestantIdByTurn(*gAIScriptPtr.at(1));
    (*(*gContestResources).aiData).scriptResult =
        (*(*gContestResources).status.at(contestant)).completedComboFlag() as i16;
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe fn ContestAICmd_if_completed_combo() {
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
pub(crate) unsafe fn ContestAICmd_if_not_completed_combo() {
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
pub(crate) unsafe fn ContestAICmd_get_points_diff() {
    let contestant: u8 = GetContestantIdByTurn(*gAIScriptPtr.at(1));
    (*(*gContestResources).aiData).scriptResult = (*(*gContestResources).status.at(contestant))
        .pointTotal
        - (*(*gContestResources)
            .status
            .at((*(*gContestResources).aiData).contestantId))
        .pointTotal;
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe fn ContestAICmd_if_points_more_than_mon() {
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
pub(crate) unsafe fn ContestAICmd_if_points_less_than_mon() {
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
pub(crate) unsafe fn ContestAICmd_if_points_eq_mon() {
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
pub(crate) unsafe fn ContestAICmd_if_points_not_eq_mon() {
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
pub(crate) unsafe fn ContestAICmd_get_preliminary_points_diff() {
    let contestant: u8 = GetContestantIdByTurn(*gAIScriptPtr.at(1));
    (*(*gContestResources).aiData).scriptResult = gContestMonRound1Points[contestant]
        - gContestMonRound1Points[(*(*gContestResources).aiData).contestantId];
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe fn ContestAICmd_if_preliminary_points_more_than_mon() {
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
pub(crate) unsafe fn ContestAICmd_if_preliminary_points_less_than_mon() {
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
pub(crate) unsafe fn ContestAICmd_if_preliminary_points_eq_mon() {
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
pub(crate) unsafe fn ContestAICmd_if_preliminary_points_not_eq_mon() {
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
pub(crate) unsafe fn ContestAICmd_get_used_moves_effect() {
    let contestant: u8 = GetContestantIdByTurn(*gAIScriptPtr.at(1));
    let round: u8 = *gAIScriptPtr.at(2);
    let r#move: u16 = (*(*gContestResources).contest).moveHistory[round][contestant];
    (*(*gContestResources).aiData).scriptResult =
        (*(&raw const crate::data::contest_effect::gContestMoves).cast::<CArray<ContestMove, 0>>())
            [r#move]
            .effect as i16;
    gAIScriptPtr = gAIScriptPtr.at(3);
}
pub(crate) unsafe fn ContestAICmd_if_used_moves_effect_less_than() {
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
pub(crate) unsafe fn ContestAICmd_if_used_moves_effect_more_than() {
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
pub(crate) unsafe fn ContestAICmd_if_used_moves_effect_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_used_moves_effect_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_get_used_moves_excitement() {
    let contestant: u8 = GetContestantIdByTurn(*gAIScriptPtr.at(1));
    let round: u8 = *gAIScriptPtr.at(2);
    let result: i8 = (*(*gContestResources).contest).excitementHistory[round][contestant] as i8;
    (*(*gContestResources).aiData).scriptResult = result as i16;
    gAIScriptPtr = gAIScriptPtr.at(3);
}
pub(crate) unsafe fn ContestAICmd_if_used_moves_excitement_less_than() {
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
pub(crate) unsafe fn ContestAICmd_if_used_moves_excitement_more_than() {
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
pub(crate) unsafe fn ContestAICmd_if_used_moves_excitement_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_used_moves_excitement_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_get_used_moves_effect_type() {
    let contestant: u8 = GetContestantIdByTurn(*gAIScriptPtr.at(1));
    let round: u8 = *gAIScriptPtr.at(2);
    let r#move: u16 = (*(*gContestResources).contest).moveHistory[round][contestant];
    (*(*gContestResources).aiData).scriptResult =
        (*(&raw const crate::data::contest_effect::gContestEffects)
            .cast::<CArray<ContestEffect, 0>>())
            [(*(&raw const crate::data::contest_effect::gContestMoves)
                .cast::<CArray<ContestMove, 0>>())[r#move]
                .effect]
            .effectType as i16;
    gAIScriptPtr = gAIScriptPtr.at(3);
}
pub(crate) unsafe fn ContestAICmd_if_used_moves_effect_type_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_used_moves_effect_type_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_save_result() {
    (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)] =
        (*(*gContestResources).aiData).scriptResult;
    gAIScriptPtr = gAIScriptPtr.at(2);
}
pub(crate) unsafe fn ContestAICmd_setvar() {
    (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)] =
        *gAIScriptPtr.at(2) as i16 | (*gAIScriptPtr.at(2).at(1) as i16) << 8;
    gAIScriptPtr = gAIScriptPtr.at(4);
}
pub(crate) unsafe fn ContestAICmd_add() {
    (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)] +=
        *gAIScriptPtr.at(2) as i8 as i16 | (*gAIScriptPtr.at(3) as i16) << 8;
    gAIScriptPtr = gAIScriptPtr.at(4);
}
pub(crate) unsafe fn ContestAICmd_addvar() {
    (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)] +=
        (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(2)];
    gAIScriptPtr = gAIScriptPtr.at(3);
}
pub(crate) unsafe fn ContestAICmd_addvar_duplicate() {
    (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(1)] +=
        (*(*gContestResources).aiData).vars[*gAIScriptPtr.at(2)];
    gAIScriptPtr = gAIScriptPtr.at(3);
}
pub(crate) unsafe fn ContestAICmd_if_less_than() {
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
pub(crate) unsafe fn ContestAICmd_if_greater_than() {
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
pub(crate) unsafe fn ContestAICmd_if_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_not_eq() {
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
pub(crate) unsafe fn ContestAICmd_if_less_than_var() {
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
pub(crate) unsafe fn ContestAICmd_if_greater_than_var() {
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
pub(crate) unsafe fn ContestAICmd_if_eq_var() {
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
pub(crate) unsafe fn ContestAICmd_if_not_eq_var() {
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
pub(crate) unsafe fn ContestAICmd_if_random_less_than() {
    if Random() as i32 & 0xFF < *gAIScriptPtr.at(1) as i32 {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe fn ContestAICmd_if_random_greater_than() {
    if Random() as i32 & 0xFF > *gAIScriptPtr.at(1) as i32 {
        gAIScriptPtr = (*gAIScriptPtr.at(2) as i32
            | (*gAIScriptPtr.at(2).at(1) as i32) << 8
            | (*gAIScriptPtr.at(2).at(2) as i32) << 16
            | (*gAIScriptPtr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    } else {
        gAIScriptPtr = gAIScriptPtr.at(6);
    }
}
pub(crate) unsafe fn ContestAICmd_goto() {
    gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
        | (*gAIScriptPtr.at(1).at(1) as i32) << 8
        | (*gAIScriptPtr.at(1).at(2) as i32) << 16
        | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
}
pub(crate) unsafe fn ContestAICmd_call() {
    AIStackPushVar(gAIScriptPtr.at(5));
    gAIScriptPtr = (*gAIScriptPtr.at(1) as i32
        | (*gAIScriptPtr.at(1).at(1) as i32) << 8
        | (*gAIScriptPtr.at(1).at(2) as i32) << 16
        | (*gAIScriptPtr.at(1).at(3) as i32) << 24) as usize as *mut u8;
}
pub(crate) unsafe fn ContestAICmd_end() {
    if AIStackPop() == 0 {
        (*(*gContestResources).aiData).aiAction |= AI_ACTION_DONE;
    }
}
pub(crate) unsafe fn AIStackPushVar(ptr: *mut u8) {
    (*(*gContestResources).aiData).stack[{
        let t1 = (*(*gContestResources).aiData).stackSize;
        (*(*gContestResources).aiData).stackSize += 1;
        t1
    }] = ptr;
}
pub(crate) unsafe fn AIStackPop() -> u8 {
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
        0
    }
}
pub(crate) unsafe fn ContestAICmd_check_user_has_exciting_move() {
    let mut result: i32 = 0;
    for i in 0..MAX_MON_MOVES {
        if gContestMons[(*(*gContestResources).aiData).contestantId].moves[i] != 0
            && Contest_GetMoveExcitement(
                gContestMons[(*(*gContestResources).aiData).contestantId].moves[i],
            ) == 1
        {
            result = 1;
            break;
        }
    }
    (*(*gContestResources).aiData).scriptResult = result as i16;
    gAIScriptPtr = gAIScriptPtr.at(1);
}
pub(crate) unsafe fn ContestAICmd_if_user_has_exciting_move() {
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
pub(crate) unsafe fn ContestAICmd_if_user_doesnt_have_exciting_move() {
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
pub(crate) unsafe fn ContestAICmd_check_user_has_move() {
    let mut hasMove: i32 = FALSE as i32;
    let targetMove: u16 = *gAIScriptPtr.at(1) as u16 | (*gAIScriptPtr.at(1).at(1) as u16) << 8;
    for i in 0..MAX_MON_MOVES {
        let r#move: u16 = gContestMons[(*(*gContestResources).aiData).contestantId].moves[i];
        if r#move == targetMove {
            hasMove = TRUE as i32;
            break;
        }
    }
    (*(*gContestResources).aiData).scriptResult = hasMove as i16;
    gAIScriptPtr = gAIScriptPtr.at(3);
}
pub(crate) unsafe fn ContestAICmd_if_user_has_move() {
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
pub(crate) unsafe fn ContestAICmd_if_user_doesnt_have_move() {
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
