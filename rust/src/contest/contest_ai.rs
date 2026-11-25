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
    memset((*crate::gl!(gContestResources)).aiData as *mut u8, 0, 68);
    for i in 0..MAX_MON_MOVES {
        (*(*crate::gl!(gContestResources)).aiData).moveScores[i] = 100;
    }
    (*(*crate::gl!(gContestResources)).aiData).contestantId = contestantAI;
    (*(*crate::gl!(gContestResources)).aiData).stackSize = 0;
    (*(*crate::gl!(gContestResources)).aiData).aiFlags =
        (*gContestMons.as_ptr())[(*(*crate::gl!(gContestResources)).aiData).contestantId].aiFlags;
}
pub unsafe fn ContestAI_GetActionToUse() -> u8 {
    while (*(*crate::gl!(gContestResources)).aiData).aiFlags != 0 {
        if (*(*crate::gl!(gContestResources)).aiData).aiFlags & 1 != 0 {
            (*(*crate::gl!(gContestResources)).aiData).aiState = CONTESTAI_SETTING_UP;
            ContestAI_DoAIProcessing();
        }
        (*(*crate::gl!(gContestResources)).aiData).aiFlags >>= 1;
        (*(*crate::gl!(gContestResources)).aiData).currentAIFlag += 1;
        (*(*crate::gl!(gContestResources)).aiData).nextMoveIndex = 0;
    }
    loop {
        let moveIndex: u8 = (if 0 != 0 {
            Random() as i32 % 4
        } else {
            Random() as i32 & 3
        }) as u8;
        let score: u8 = (*(*crate::gl!(gContestResources)).aiData).moveScores[moveIndex];
        let mut i: i32 = 0;
        while i < MAX_MON_MOVES {
            if score < (*(*crate::gl!(gContestResources)).aiData).moveScores[i] {
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
    while (*(*crate::gl!(gContestResources)).aiData).aiState != CONTESTAI_FINISHED {
        match (*(*crate::gl!(gContestResources)).aiData).aiState {
            CONTESTAI_DO_NOT_PROCESS => {}
            CONTESTAI_SETTING_UP => {
                crate::gl!(
                    gAIScriptPtr = (*crate::asmdata::gContestAI_ScriptsTable
                        .cast::<CArray<*mut u8, 0>>()
                        .cast_mut())[(*(*crate::gl!(gContestResources)).aiData).currentAIFlag]
                );
                if (*gContestMons.as_ptr())[(*(*crate::gl!(gContestResources)).aiData).contestantId]
                    .moves[(*(*crate::gl!(gContestResources)).aiData).nextMoveIndex]
                    == MOVE_NONE
                {
                    (*(*crate::gl!(gContestResources)).aiData).nextMove = MOVE_NONE;
                } else {
                    (*(*crate::gl!(gContestResources)).aiData).nextMove = (*gContestMons.as_ptr())
                        [(*(*crate::gl!(gContestResources)).aiData).contestantId]
                        .moves[(*(*crate::gl!(gContestResources)).aiData).nextMoveIndex];
                }
                (*(*crate::gl!(gContestResources)).aiData).aiState += 1;
            }
            CONTESTAI_PROCESSING => {
                if (*(*crate::gl!(gContestResources)).aiData).nextMove != MOVE_NONE {
                    sContestAICmdTable[*crate::gl!(gAIScriptPtr)].unwrap_unchecked()();
                } else {
                    (*(*crate::gl!(gContestResources)).aiData).moveScores
                        [(*(*crate::gl!(gContestResources)).aiData).nextMoveIndex] = 0;
                    (*(*crate::gl!(gContestResources)).aiData).aiAction |= AI_ACTION_DONE;
                }
                if (*(*crate::gl!(gContestResources)).aiData).aiAction as i32
                    & AI_ACTION_DONE as i32
                    != 0
                {
                    (*(*crate::gl!(gContestResources)).aiData).nextMoveIndex += 1;
                    if (*(*crate::gl!(gContestResources)).aiData).nextMoveIndex
                        < MAX_MON_MOVES as u8
                    {
                        (*(*crate::gl!(gContestResources)).aiData).aiState = 0;
                    } else {
                        (*(*crate::gl!(gContestResources)).aiData).aiState += 1;
                    }
                    (*(*crate::gl!(gContestResources)).aiData).aiAction &= 254;
                }
            }
            _ => {}
        }
    }
}
unsafe fn GetContestantIdByTurn(turn: u8) -> u8 {
    let mut i: i32 = 0;
    while i < CONTESTANT_COUNT {
        if (*(*crate::gl!(gContestResources)).appealResults).turnOrder[i] == turn {
            break;
        }
        i += 1;
    }
    i as u8
}
pub(crate) unsafe fn ContestAICmd_score() {
    let mut score: i16 = (*(*crate::gl!(gContestResources)).aiData).moveScores
        [(*(*crate::gl!(gContestResources)).aiData).nextMoveIndex] as i16
        + *(*gAIScriptPtr.as_ptr()).at(1) as i8 as i16;
    if score > 255 {
        score = 255;
    } else if score < 0 {
        score = 0;
    }
    (*(*crate::gl!(gContestResources)).aiData).moveScores
        [(*(*crate::gl!(gContestResources)).aiData).nextMoveIndex] = score as u8;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) unsafe fn ContestAICmd_get_appeal_num() {
    (*(*crate::gl!(gContestResources)).aiData).scriptResult =
        (*(*crate::gl!(gContestResources)).contest).appealNumber as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_appeal_num_less_than() {
    ContestAICmd_get_appeal_num();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult < *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_appeal_num_more_than() {
    ContestAICmd_get_appeal_num();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult > *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_appeal_num_eq() {
    ContestAICmd_get_appeal_num();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_appeal_num_not_eq() {
    ContestAICmd_get_appeal_num();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_get_excitement() {
    (*(*crate::gl!(gContestResources)).aiData).scriptResult =
        (*(*crate::gl!(gContestResources)).contest).applauseLevel as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_excitement_less_than() {
    ContestAICmd_get_excitement();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult < *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_excitement_more_than() {
    ContestAICmd_get_excitement();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult > *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_excitement_eq() {
    ContestAICmd_get_excitement();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_excitement_not_eq() {
    ContestAICmd_get_excitement();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_get_user_order() {
    (*(*crate::gl!(gContestResources)).aiData).scriptResult =
        (*(*crate::gl!(gContestResources)).appealResults).turnOrder
            [(*(*crate::gl!(gContestResources)).aiData).contestantId] as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_user_order_less_than() {
    ContestAICmd_get_user_order();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult < *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_user_order_more_than() {
    ContestAICmd_get_user_order();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult > *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_user_order_eq() {
    ContestAICmd_get_user_order();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_user_order_not_eq() {
    ContestAICmd_get_user_order();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_get_user_condition() {
    (*(*crate::gl!(gContestResources)).aiData).scriptResult = ((*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).aiData).contestantId))
    .condition
        / 10) as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_user_condition_less_than() {
    ContestAICmd_get_user_condition();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult < *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_user_condition_more_than() {
    ContestAICmd_get_user_condition();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult > *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_user_condition_eq() {
    ContestAICmd_get_user_condition();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_user_condition_not_eq() {
    ContestAICmd_get_user_condition();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_get_points() {
    (*(*crate::gl!(gContestResources)).aiData).scriptResult = (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).aiData).contestantId))
    .pointTotal;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_points_less_than() {
    ContestAICmd_get_points();
    if ((*(*crate::gl!(gContestResources)).aiData).scriptResult as i32)
        < *crate::gl!(gAIScriptPtr) as i16 as i32
            | (*(*gAIScriptPtr.as_ptr()).at(1) as i16 as i32) << 8
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn ContestAICmd_if_points_more_than() {
    ContestAICmd_get_points();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult as i32
        > *crate::gl!(gAIScriptPtr) as i16 as i32
            | (*(*gAIScriptPtr.as_ptr()).at(1) as i16 as i32) << 8
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn ContestAICmd_if_points_eq() {
    ContestAICmd_get_points();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult as i32
        == *crate::gl!(gAIScriptPtr) as i16 as i32
            | (*(*gAIScriptPtr.as_ptr()).at(1) as i16 as i32) << 8
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn ContestAICmd_if_points_not_eq() {
    ContestAICmd_get_points();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult as i32
        != *crate::gl!(gAIScriptPtr) as i16 as i32
            | (*(*gAIScriptPtr.as_ptr()).at(1) as i16 as i32) << 8
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn ContestAICmd_get_preliminary_points() {
    (*(*crate::gl!(gContestResources)).aiData).scriptResult = (*gContestMonRound1Points.as_ptr())
        [(*(*crate::gl!(gContestResources)).aiData).contestantId];
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_preliminary_points_less_than() {
    ContestAICmd_get_preliminary_points();
    if ((*(*crate::gl!(gContestResources)).aiData).scriptResult as i32)
        < *crate::gl!(gAIScriptPtr) as i16 as i32
            | (*(*gAIScriptPtr.as_ptr()).at(1) as i16 as i32) << 8
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn ContestAICmd_if_preliminary_points_more_than() {
    ContestAICmd_get_preliminary_points();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult as i32
        > *crate::gl!(gAIScriptPtr) as i16 as i32
            | (*(*gAIScriptPtr.as_ptr()).at(1) as i16 as i32) << 8
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn ContestAICmd_if_preliminary_points_eq() {
    ContestAICmd_get_preliminary_points();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult as i32
        == *crate::gl!(gAIScriptPtr) as i16 as i32
            | (*(*gAIScriptPtr.as_ptr()).at(1) as i16 as i32) << 8
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn ContestAICmd_if_preliminary_points_not_eq() {
    ContestAICmd_get_preliminary_points();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult as i32
        != *crate::gl!(gAIScriptPtr) as i16 as i32
            | (*(*gAIScriptPtr.as_ptr()).at(1) as i16 as i32) << 8
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn ContestAICmd_get_contest_type() {
    (*(*crate::gl!(gContestResources)).aiData).scriptResult =
        gSpecialVar_ContestCategory.get() as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_contest_type_eq() {
    ContestAICmd_get_contest_type();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_contest_type_not_eq() {
    ContestAICmd_get_contest_type();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_get_move_excitement() {
    (*(*crate::gl!(gContestResources)).aiData).scriptResult = Contest_GetMoveExcitement(
        (*gContestMons.as_ptr())[(*(*crate::gl!(gContestResources)).aiData).contestantId].moves
            [(*(*crate::gl!(gContestResources)).aiData).nextMoveIndex],
    ) as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_move_excitement_less_than() {
    ContestAICmd_get_move_excitement();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult
        < *crate::gl!(gAIScriptPtr) as i8 as i16
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_move_excitement_more_than() {
    ContestAICmd_get_move_excitement();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult
        > *crate::gl!(gAIScriptPtr) as i8 as i16
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_move_excitement_eq() {
    ContestAICmd_get_move_excitement();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult
        == *crate::gl!(gAIScriptPtr) as i8 as i16
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_move_excitement_not_eq() {
    ContestAICmd_get_move_excitement();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult
        != *crate::gl!(gAIScriptPtr) as i8 as i16
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_get_move_effect() {
    let r#move: u16 = (*gContestMons.as_ptr())
        [(*(*crate::gl!(gContestResources)).aiData).contestantId]
        .moves[(*(*crate::gl!(gContestResources)).aiData).nextMoveIndex];
    (*(*crate::gl!(gContestResources)).aiData).scriptResult =
        (*(&raw const crate::data::contest_effect::gContestMoves).cast::<CArray<ContestMove, 0>>())
            [r#move]
            .effect as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_move_effect_eq() {
    ContestAICmd_get_move_effect();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_move_effect_not_eq() {
    ContestAICmd_get_move_effect();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_get_move_effect_type() {
    let r#move: u16 = (*gContestMons.as_ptr())
        [(*(*crate::gl!(gContestResources)).aiData).contestantId]
        .moves[(*(*crate::gl!(gContestResources)).aiData).nextMoveIndex];
    (*(*crate::gl!(gContestResources)).aiData).scriptResult =
        (*(&raw const crate::data::contest_effect::gContestEffects)
            .cast::<CArray<ContestEffect, 0>>())
            [(*(&raw const crate::data::contest_effect::gContestMoves)
                .cast::<CArray<ContestMove, 0>>())[r#move]
                .effect]
            .effectType as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_move_effect_type_eq() {
    ContestAICmd_get_move_effect_type();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_move_effect_type_not_eq() {
    ContestAICmd_get_move_effect_type();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_check_most_appealing_move() {
    let r#move: u16 = (*gContestMons.as_ptr())
        [(*(*crate::gl!(gContestResources)).aiData).contestantId]
        .moves[(*(*crate::gl!(gContestResources)).aiData).nextMoveIndex];
    let appeal: u8 = (*(&raw const crate::data::contest_effect::gContestEffects)
        .cast::<CArray<ContestEffect, 0>>())
        [(*(&raw const crate::data::contest_effect::gContestMoves)
            .cast::<CArray<ContestMove, 0>>())[r#move]
            .effect]
        .appeal;
    let mut i: i32 = 0;
    while i < MAX_MON_MOVES {
        let newMove: u16 = (*gContestMons.as_ptr())
            [(*(*crate::gl!(gContestResources)).aiData).contestantId]
            .moves[i];
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
        (*(*crate::gl!(gContestResources)).aiData).scriptResult = TRUE as i16;
    } else {
        (*(*crate::gl!(gContestResources)).aiData).scriptResult = FALSE as i16;
    }
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_most_appealing_move() {
    ContestAICmd_check_most_appealing_move();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != FALSE as i16 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_check_most_jamming_move() {
    let r#move: u16 = (*gContestMons.as_ptr())
        [(*(*crate::gl!(gContestResources)).aiData).contestantId]
        .moves[(*(*crate::gl!(gContestResources)).aiData).nextMoveIndex];
    let jam: u8 = (*(&raw const crate::data::contest_effect::gContestEffects)
        .cast::<CArray<ContestEffect, 0>>())
        [(*(&raw const crate::data::contest_effect::gContestMoves)
            .cast::<CArray<ContestMove, 0>>())[r#move]
            .effect]
        .jam;
    let mut i: i32 = 0;
    while i < MAX_MON_MOVES {
        let newMove: u16 = (*gContestMons.as_ptr())
            [(*(*crate::gl!(gContestResources)).aiData).contestantId]
            .moves[i];
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
        (*(*crate::gl!(gContestResources)).aiData).scriptResult = TRUE as i16;
    } else {
        (*(*crate::gl!(gContestResources)).aiData).scriptResult = FALSE as i16;
    }
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_most_jamming_move() {
    ContestAICmd_check_most_jamming_move();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != FALSE as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_get_num_move_hearts() {
    let r#move: u16 = (*gContestMons.as_ptr())
        [(*(*crate::gl!(gContestResources)).aiData).contestantId]
        .moves[(*(*crate::gl!(gContestResources)).aiData).nextMoveIndex];
    (*(*crate::gl!(gContestResources)).aiData).scriptResult =
        ((*(&raw const crate::data::contest_effect::gContestEffects)
            .cast::<CArray<ContestEffect, 0>>())
            [(*(&raw const crate::data::contest_effect::gContestMoves)
                .cast::<CArray<ContestMove, 0>>())[r#move]
                .effect]
            .appeal as i32
            / 10) as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_num_move_hearts_less_than() {
    ContestAICmd_get_num_move_hearts();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult < *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_num_move_hearts_more_than() {
    ContestAICmd_get_num_move_hearts();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult > *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_num_move_hearts_eq() {
    ContestAICmd_get_num_move_hearts();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_num_move_hearts_not_eq() {
    ContestAICmd_get_num_move_hearts();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_get_num_move_jam_hearts() {
    let r#move: u16 = (*gContestMons.as_ptr())
        [(*(*crate::gl!(gContestResources)).aiData).contestantId]
        .moves[(*(*crate::gl!(gContestResources)).aiData).nextMoveIndex];
    (*(*crate::gl!(gContestResources)).aiData).scriptResult =
        ((*(&raw const crate::data::contest_effect::gContestEffects)
            .cast::<CArray<ContestEffect, 0>>())
            [(*(&raw const crate::data::contest_effect::gContestMoves)
                .cast::<CArray<ContestMove, 0>>())[r#move]
                .effect]
            .jam as i32
            / 10) as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_num_move_jam_hearts_less_than() {
    ContestAICmd_get_num_move_jam_hearts();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult < *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_num_move_jam_hearts_more_than() {
    ContestAICmd_get_num_move_jam_hearts();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult > *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_num_move_jam_hearts_eq() {
    ContestAICmd_get_num_move_jam_hearts();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_num_move_jam_hearts_not_eq() {
    ContestAICmd_get_num_move_jam_hearts();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_get_move_used_count() {
    let mut result: i16 = 0;
    let r#move: u16 = (*gContestMons.as_ptr())
        [(*(*crate::gl!(gContestResources)).aiData).contestantId]
        .moves[(*(*crate::gl!(gContestResources)).aiData).nextMoveIndex];
    if r#move
        != (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).aiData).contestantId))
        .prevMove
    {
        result = 0;
    } else {
        result = (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).aiData).contestantId))
        .moveRepeatCount() as i16
            + 1;
    }
    (*(*crate::gl!(gContestResources)).aiData).scriptResult = result;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_most_used_count_less_than() {
    ContestAICmd_get_move_used_count();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult < *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_most_used_count_more_than() {
    ContestAICmd_get_move_used_count();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult > *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_most_used_count_eq() {
    ContestAICmd_get_move_used_count();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_most_used_count_not_eq() {
    ContestAICmd_get_move_used_count();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_check_combo_starter() {
    let mut result: u8 = 0;
    let r#move: u16 = (*gContestMons.as_ptr())
        [(*(*crate::gl!(gContestResources)).aiData).contestantId]
        .moves[(*(*crate::gl!(gContestResources)).aiData).nextMoveIndex];
    for i in 0..MAX_MON_MOVES {
        if (*gContestMons.as_ptr())[(*(*crate::gl!(gContestResources)).aiData).contestantId].moves
            [i]
            != 0
        {
            result = AreMovesContestCombo(
                r#move,
                (*gContestMons.as_ptr())[(*(*crate::gl!(gContestResources)).aiData).contestantId]
                    .moves[i],
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
    (*(*crate::gl!(gContestResources)).aiData).scriptResult = result as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_combo_starter() {
    ContestAICmd_check_combo_starter();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_if_not_combo_starter() {
    ContestAICmd_check_combo_starter();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_check_combo_finisher() {
    let mut result: u8 = 0;
    let r#move: u16 = (*gContestMons.as_ptr())
        [(*(*crate::gl!(gContestResources)).aiData).contestantId]
        .moves[(*(*crate::gl!(gContestResources)).aiData).nextMoveIndex];
    for i in 0..MAX_MON_MOVES {
        if (*gContestMons.as_ptr())[(*(*crate::gl!(gContestResources)).aiData).contestantId].moves
            [i]
            != 0
        {
            result = AreMovesContestCombo(
                (*gContestMons.as_ptr())[(*(*crate::gl!(gContestResources)).aiData).contestantId]
                    .moves[i],
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
    (*(*crate::gl!(gContestResources)).aiData).scriptResult = result as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_combo_finisher() {
    ContestAICmd_check_combo_finisher();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_if_not_combo_finisher() {
    ContestAICmd_check_combo_finisher();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_check_would_finish_combo() {
    let mut result: u8 = 0;
    let r#move: u16 = (*gContestMons.as_ptr())
        [(*(*crate::gl!(gContestResources)).aiData).contestantId]
        .moves[(*(*crate::gl!(gContestResources)).aiData).nextMoveIndex];
    if (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).aiData).contestantId))
    .prevMove
        != 0
    {
        result = AreMovesContestCombo(
            (*(*crate::gl!(gContestResources))
                .status
                .at((*(*crate::gl!(gContestResources)).aiData).contestantId))
            .prevMove,
            r#move,
        );
    }
    if result != 0 {
        result = 1;
    }
    (*(*crate::gl!(gContestResources)).aiData).scriptResult = result as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_would_finish_combo() {
    ContestAICmd_check_would_finish_combo();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_if_would_not_finish_combo() {
    ContestAICmd_check_would_finish_combo();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_get_condition() {
    let contestant: u8 = GetContestantIdByTurn(*(*gAIScriptPtr.as_ptr()).at(1));
    (*(*crate::gl!(gContestResources)).aiData).scriptResult =
        ((*(*crate::gl!(gContestResources)).status.at(contestant)).condition / 10) as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) unsafe fn ContestAICmd_if_condition_less_than() {
    ContestAICmd_get_condition();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult < *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_condition_more_than() {
    ContestAICmd_get_condition();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult > *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_condition_eq() {
    ContestAICmd_get_condition();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_condition_not_eq() {
    ContestAICmd_get_condition();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_get_used_combo_starter() {
    let mut result: u16 = FALSE as u16;
    let contestant: u8 = GetContestantIdByTurn(*(*gAIScriptPtr.as_ptr()).at(1));
    if IsContestantAllowedToCombo(contestant) != 0 {
        result = (if (*(&raw const crate::data::contest_effect::gContestMoves)
            .cast::<CArray<ContestMove, 0>>())
            [(*(*crate::gl!(gContestResources)).status.at(contestant)).prevMove]
            .comboStarterId
            != 0
        {
            TRUE as i32
        } else {
            FALSE as i32
        }) as u16;
    }
    (*(*crate::gl!(gContestResources)).aiData).scriptResult = result as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) unsafe fn ContestAICmd_if_used_combo_starter_less_than() {
    ContestAICmd_get_used_combo_starter();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult < *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_used_combo_starter_more_than() {
    ContestAICmd_get_used_combo_starter();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult > *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_used_combo_starter_eq() {
    ContestAICmd_get_used_combo_starter();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_used_combo_starter_not_eq() {
    ContestAICmd_get_used_combo_starter();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_check_can_participate() {
    if Contest_IsMonsTurnDisabled(GetContestantIdByTurn(*(*gAIScriptPtr.as_ptr()).at(1))) != 0 {
        (*(*crate::gl!(gContestResources)).aiData).scriptResult = FALSE as i16;
    } else {
        (*(*crate::gl!(gContestResources)).aiData).scriptResult = TRUE as i16;
    }
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) unsafe fn ContestAICmd_if_can_participate() {
    ContestAICmd_check_can_participate();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_if_cannot_participate() {
    ContestAICmd_check_can_participate();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_get_completed_combo() {
    let contestant: u8 = GetContestantIdByTurn(*(*gAIScriptPtr.as_ptr()).at(1));
    (*(*crate::gl!(gContestResources)).aiData).scriptResult =
        (*(*crate::gl!(gContestResources)).status.at(contestant)).completedComboFlag() as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) unsafe fn ContestAICmd_if_completed_combo() {
    ContestAICmd_get_completed_combo();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_if_not_completed_combo() {
    ContestAICmd_get_completed_combo();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_get_points_diff() {
    let contestant: u8 = GetContestantIdByTurn(*(*gAIScriptPtr.as_ptr()).at(1));
    (*(*crate::gl!(gContestResources)).aiData).scriptResult =
        (*(*crate::gl!(gContestResources)).status.at(contestant)).pointTotal
            - (*(*crate::gl!(gContestResources))
                .status
                .at((*(*crate::gl!(gContestResources)).aiData).contestantId))
            .pointTotal;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) unsafe fn ContestAICmd_if_points_more_than_mon() {
    ContestAICmd_get_points_diff();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult < 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_if_points_less_than_mon() {
    ContestAICmd_get_points_diff();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult > 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_if_points_eq_mon() {
    ContestAICmd_get_points_diff();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_if_points_not_eq_mon() {
    ContestAICmd_get_points_diff();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_get_preliminary_points_diff() {
    let contestant: u8 = GetContestantIdByTurn(*(*gAIScriptPtr.as_ptr()).at(1));
    (*(*crate::gl!(gContestResources)).aiData).scriptResult =
        crate::gl!(gContestMonRound1Points[contestant])
            - (*gContestMonRound1Points.as_ptr())
                [(*(*crate::gl!(gContestResources)).aiData).contestantId];
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) unsafe fn ContestAICmd_if_preliminary_points_more_than_mon() {
    ContestAICmd_get_preliminary_points_diff();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult < 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_if_preliminary_points_less_than_mon() {
    ContestAICmd_get_preliminary_points_diff();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult > 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_if_preliminary_points_eq_mon() {
    ContestAICmd_get_preliminary_points_diff();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_if_preliminary_points_not_eq_mon() {
    ContestAICmd_get_preliminary_points_diff();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_get_used_moves_effect() {
    let contestant: u8 = GetContestantIdByTurn(*(*gAIScriptPtr.as_ptr()).at(1));
    let round: u8 = *(*gAIScriptPtr.as_ptr()).at(2);
    let r#move: u16 = (*(*crate::gl!(gContestResources)).contest).moveHistory[round][contestant];
    (*(*crate::gl!(gContestResources)).aiData).scriptResult =
        (*(&raw const crate::data::contest_effect::gContestMoves).cast::<CArray<ContestMove, 0>>())
            [r#move]
            .effect as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(3));
}
pub(crate) unsafe fn ContestAICmd_if_used_moves_effect_less_than() {
    ContestAICmd_get_used_moves_effect();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult < *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_used_moves_effect_more_than() {
    ContestAICmd_get_used_moves_effect();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult > *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_used_moves_effect_eq() {
    ContestAICmd_get_used_moves_effect();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_used_moves_effect_not_eq() {
    ContestAICmd_get_used_moves_effect();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_get_used_moves_excitement() {
    let contestant: u8 = GetContestantIdByTurn(*(*gAIScriptPtr.as_ptr()).at(1));
    let round: u8 = *(*gAIScriptPtr.as_ptr()).at(2);
    let result: i8 =
        (*(*crate::gl!(gContestResources)).contest).excitementHistory[round][contestant] as i8;
    (*(*crate::gl!(gContestResources)).aiData).scriptResult = result as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(3));
}
pub(crate) unsafe fn ContestAICmd_if_used_moves_excitement_less_than() {
    ContestAICmd_get_used_moves_excitement();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult < *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_used_moves_excitement_more_than() {
    ContestAICmd_get_used_moves_excitement();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult > *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_used_moves_excitement_eq() {
    ContestAICmd_get_used_moves_excitement();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_used_moves_excitement_not_eq() {
    ContestAICmd_get_used_moves_excitement();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_get_used_moves_effect_type() {
    let contestant: u8 = GetContestantIdByTurn(*(*gAIScriptPtr.as_ptr()).at(1));
    let round: u8 = *(*gAIScriptPtr.as_ptr()).at(2);
    let r#move: u16 = (*(*crate::gl!(gContestResources)).contest).moveHistory[round][contestant];
    (*(*crate::gl!(gContestResources)).aiData).scriptResult =
        (*(&raw const crate::data::contest_effect::gContestEffects)
            .cast::<CArray<ContestEffect, 0>>())
            [(*(&raw const crate::data::contest_effect::gContestMoves)
                .cast::<CArray<ContestMove, 0>>())[r#move]
                .effect]
            .effectType as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(3));
}
pub(crate) unsafe fn ContestAICmd_if_used_moves_effect_type_eq() {
    ContestAICmd_get_used_moves_effect_type();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_if_used_moves_effect_type_not_eq() {
    ContestAICmd_get_used_moves_effect_type();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != *crate::gl!(gAIScriptPtr) as i16 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn ContestAICmd_save_result() {
    (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(1)] =
        (*(*crate::gl!(gContestResources)).aiData).scriptResult;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(2));
}
pub(crate) unsafe fn ContestAICmd_setvar() {
    (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(1)] =
        *(*gAIScriptPtr.as_ptr()).at(2) as i16
            | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i16) << 8;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
}
pub(crate) unsafe fn ContestAICmd_add() {
    (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(1)] +=
        *(*gAIScriptPtr.as_ptr()).at(2) as i8 as i16
            | (*(*gAIScriptPtr.as_ptr()).at(3) as i16) << 8;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
}
pub(crate) unsafe fn ContestAICmd_addvar() {
    (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(1)] +=
        (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(2)];
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(3));
}
pub(crate) unsafe fn ContestAICmd_addvar_duplicate() {
    (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(1)] +=
        (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(2)];
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(3));
}
pub(crate) unsafe fn ContestAICmd_if_less_than() {
    if ((*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(1)] as i32)
        < *(*gAIScriptPtr.as_ptr()).at(2) as i32
            | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(4) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(8));
    }
}
pub(crate) unsafe fn ContestAICmd_if_greater_than() {
    if (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(1)] as i32
        > *(*gAIScriptPtr.as_ptr()).at(2) as i32
            | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(4) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(8));
    }
}
pub(crate) unsafe fn ContestAICmd_if_eq() {
    if (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(1)] as i32
        == *(*gAIScriptPtr.as_ptr()).at(2) as i32
            | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(4) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(8));
    }
}
pub(crate) unsafe fn ContestAICmd_if_not_eq() {
    if (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(1)] as i32
        != *(*gAIScriptPtr.as_ptr()).at(2) as i32
            | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(4) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(4).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(8));
    }
}
pub(crate) unsafe fn ContestAICmd_if_less_than_var() {
    if (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(1)]
        < (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(2)]
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(3) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(7));
    }
}
pub(crate) unsafe fn ContestAICmd_if_greater_than_var() {
    if (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(1)]
        > (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(2)]
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(3) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(7));
    }
}
pub(crate) unsafe fn ContestAICmd_if_eq_var() {
    if (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(1)]
        == (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(2)]
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(3) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(7));
    }
}
pub(crate) unsafe fn ContestAICmd_if_not_eq_var() {
    if (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(1)]
        != (*(*crate::gl!(gContestResources)).aiData).vars[*(*gAIScriptPtr.as_ptr()).at(2)]
    {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(3) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(7));
    }
}
pub(crate) unsafe fn ContestAICmd_if_random_less_than() {
    if Random() as i32 & 0xFF < *(*gAIScriptPtr.as_ptr()).at(1) as i32 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn ContestAICmd_if_random_greater_than() {
    if Random() as i32 & 0xFF > *(*gAIScriptPtr.as_ptr()).at(1) as i32 {
        crate::gl!(
            gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(2) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn ContestAICmd_goto() {
    crate::gl!(
        gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
            | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
            | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
            | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24) as usize
            as *mut u8
    );
}
pub(crate) unsafe fn ContestAICmd_call() {
    AIStackPushVar((*gAIScriptPtr.as_ptr()).at(5));
    crate::gl!(
        gAIScriptPtr = (*(*gAIScriptPtr.as_ptr()).at(1) as i32
            | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as i32) << 8
            | (*(*gAIScriptPtr.as_ptr()).at(1).at(2) as i32) << 16
            | (*(*gAIScriptPtr.as_ptr()).at(1).at(3) as i32) << 24) as usize
            as *mut u8
    );
}
pub(crate) unsafe fn ContestAICmd_end() {
    if AIStackPop() == 0 {
        (*(*crate::gl!(gContestResources)).aiData).aiAction |= AI_ACTION_DONE;
    }
}
pub(crate) unsafe fn AIStackPushVar(ptr: *mut u8) {
    (*(*crate::gl!(gContestResources)).aiData).stack[{
        let t1 = (*(*crate::gl!(gContestResources)).aiData).stackSize;
        (*(*crate::gl!(gContestResources)).aiData).stackSize += 1;
        t1
    }] = ptr;
}
pub(crate) unsafe fn AIStackPop() -> u8 {
    if (*(*crate::gl!(gContestResources)).aiData).stackSize != 0 {
        (*(*crate::gl!(gContestResources)).aiData).stackSize -= 1;
        crate::gl!(
            gAIScriptPtr = (*(*crate::gl!(gContestResources)).aiData).stack
                [(*(*crate::gl!(gContestResources)).aiData).stackSize]
        );
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
        if (*gContestMons.as_ptr())[(*(*crate::gl!(gContestResources)).aiData).contestantId].moves
            [i]
            != 0
            && Contest_GetMoveExcitement(
                (*gContestMons.as_ptr())[(*(*crate::gl!(gContestResources)).aiData).contestantId]
                    .moves[i],
            ) == 1
        {
            result = 1;
            break;
        }
    }
    (*(*crate::gl!(gContestResources)).aiData).scriptResult = result as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(1));
}
pub(crate) unsafe fn ContestAICmd_if_user_has_exciting_move() {
    ContestAICmd_check_user_has_exciting_move();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_if_user_doesnt_have_exciting_move() {
    ContestAICmd_check_user_has_exciting_move();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_check_user_has_move() {
    let mut hasMove: i32 = FALSE as i32;
    let targetMove: u16 = *(*gAIScriptPtr.as_ptr()).at(1) as u16
        | (*(*gAIScriptPtr.as_ptr()).at(1).at(1) as u16) << 8;
    for i in 0..MAX_MON_MOVES {
        let r#move: u16 = (*gContestMons.as_ptr())
            [(*(*crate::gl!(gContestResources)).aiData).contestantId]
            .moves[i];
        if r#move == targetMove {
            hasMove = TRUE as i32;
            break;
        }
    }
    (*(*crate::gl!(gContestResources)).aiData).scriptResult = hasMove as i16;
    crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(3));
}
pub(crate) unsafe fn ContestAICmd_if_user_has_move() {
    ContestAICmd_check_user_has_move();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult != 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn ContestAICmd_if_user_doesnt_have_move() {
    ContestAICmd_check_user_has_move();
    if (*(*crate::gl!(gContestResources)).aiData).scriptResult == 0 {
        crate::gl!(
            gAIScriptPtr = (*crate::gl!(gAIScriptPtr) as i32
                | (*(*gAIScriptPtr.as_ptr()).at(1) as i32) << 8
                | (*(*gAIScriptPtr.as_ptr()).at(2) as i32) << 16
                | (*(*gAIScriptPtr.as_ptr()).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gAIScriptPtr = (*gAIScriptPtr.as_ptr()).at(4));
    }
}
