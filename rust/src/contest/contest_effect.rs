//! Translated from `src/contest_effect.c` by tools/rustport/c2rs.py.
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
    unused_assignments
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::contest::gContestantTurnOrder;
use crate::contest::{
    Contest_IsMonsTurnDisabled, IsContestantAllowedToCombo, MakeContestantNervous,
    SetContestantEffectStringID, SetContestantEffectStringID2, SetStartledString,
    gContestResources, gSpecialVar_ContestCategory,
};
use crate::random::Random;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): gContestMoves gContestEffects gComboStarterLookupTable gContestEffectFuncs

static gComboStarterLookupTable: Table<CArray<u8, 63>> =
    Table((&raw const crate::data::contest_effect::gComboStarterLookupTable).cast());
static gContestEffects: Table<CArray<ContestEffect, 48>> =
    Table((&raw const crate::data::contest_effect::gContestEffects).cast());
static gContestMoves: Table<CArray<ContestMove, 355>> =
    Table((&raw const crate::data::contest_effect::gContestMoves).cast());

pub unsafe fn AreMovesContestCombo(lastMove: u16, nextMove: u16) -> u8 {
    let mut nextMoveComboMoves: CArray<u8, 4> = zeroed();
    let lastMoveComboStarterId: u8 = gContestMoves[lastMove].comboStarterId;
    nextMoveComboMoves[0] = gContestMoves[nextMove].comboMoves[0];
    nextMoveComboMoves[1] = gContestMoves[nextMove].comboMoves[1];
    nextMoveComboMoves[2] = gContestMoves[nextMove].comboMoves[2];
    nextMoveComboMoves[3] = gContestMoves[nextMove].comboMoves[3];
    if lastMoveComboStarterId == 0 {
        return FALSE;
    } else if lastMoveComboStarterId == nextMoveComboMoves[0]
        || lastMoveComboStarterId == nextMoveComboMoves[1]
        || lastMoveComboStarterId == nextMoveComboMoves[2]
        || lastMoveComboStarterId == nextMoveComboMoves[3]
    {
        return gComboStarterLookupTable[lastMoveComboStarterId];
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) fn ContestEffect_HighlyAppealing() {}
pub(crate) unsafe fn ContestEffect_UserMoreEasilyStartled() {
    (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
    .set_moreEasilyStartled(TRUE);
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_MORE_CONSCIOUS,
    );
}
pub(crate) unsafe fn ContestEffect_GreatAppealButNoMoreMoves() {
    (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
    .set_exploded(TRUE);
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_NO_APPEAL,
    );
}
pub(crate) unsafe fn ContestEffect_RepetitionNotBoring() {
    (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
    .set_usedRepeatableMove(TRUE);
    (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
    .set_repeatedMove(FALSE);
    (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
    .set_moveRepeatCount(0);
}
pub(crate) unsafe fn ContestEffect_AvoidStartleOnce() {
    (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
    .jamSafetyCount = 1;
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_SETTLE_DOWN,
    );
}
pub(crate) unsafe fn ContestEffect_AvoidStartle() {
    (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
    .set_immune(TRUE);
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_OBLIVIOUS_TO_OTHERS,
    );
}
pub(crate) unsafe fn ContestEffect_AvoidStartleSlightly() {
    (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
    .jamReduction = 20;
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_LESS_AWARE,
    );
}
pub(crate) unsafe fn ContestEffect_UserLessEasilyStartled() {
    (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
    .set_resistant(TRUE);
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_STOPPED_CARING,
    );
}
pub(crate) unsafe fn ContestEffect_StartleFrontMon() {
    let mut idx: u8 = 0;
    let a: u8 = (*(*crate::gl!(gContestResources)).appealResults).contestant;
    if (*(*crate::gl!(gContestResources)).appealResults).turnOrder[a] != 0 {
        let mut i: i32 = 0;
        while i < CONTESTANT_COUNT {
            if (*(*crate::gl!(gContestResources)).appealResults).turnOrder[a] as i32 - 1
                == (*(*crate::gl!(gContestResources)).appealResults).turnOrder[i] as i32
            {
                break;
            }
            i += 1;
        }
        (*(*crate::gl!(gContestResources)).appealResults).jamQueue[0] = i as u8;
        (*(*crate::gl!(gContestResources)).appealResults).jamQueue[1] = CONTESTANT_NONE;
        idx = WasAtLeastOneOpponentJammed();
    }
    if idx == 0 {
        SetContestantEffectStringID2(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_MESSED_UP2,
        );
    }
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_ATTEMPT_STARTLE,
    );
}
pub(crate) unsafe fn ContestEffect_StartlePrevMons() {
    let mut idx: u8 = 0;
    let contestant: u8 = (*(*crate::gl!(gContestResources)).appealResults).contestant;
    if (*(*crate::gl!(gContestResources)).appealResults).turnOrder[contestant] != 0 {
        let mut j: i32 = 0;
        for i in 0..CONTESTANT_COUNT {
            if (*(*crate::gl!(gContestResources)).appealResults).turnOrder[contestant]
                > (*(*crate::gl!(gContestResources)).appealResults).turnOrder[i]
            {
                (*(*crate::gl!(gContestResources)).appealResults).jamQueue[{
                    let t1 = j;
                    j += 1;
                    t1
                }] = i as u8;
            }
        }
        (*(*crate::gl!(gContestResources)).appealResults).jamQueue[j] = CONTESTANT_NONE;
        idx = WasAtLeastOneOpponentJammed();
    }
    if idx == 0 {
        SetContestantEffectStringID2(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_MESSED_UP2,
        );
    }
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_ATTEMPT_STARTLE,
    );
}
pub(crate) unsafe fn ContestEffect_StartlePrevMon2() {
    let rval: u8 = (Random() as i32 % 10) as u8;
    let mut jam: i32 = 0;
    if rval < 2 {
        jam = 20;
    } else if rval < 8 {
        jam = 40;
    } else {
        jam = 60;
    }
    (*(*crate::gl!(gContestResources)).appealResults).jam = jam as i16;
    ContestEffect_StartleFrontMon();
}
pub(crate) unsafe fn ContestEffect_StartlePrevMons2() {
    let mut numStartled: u8 = 0;
    let contestant: u8 = (*(*crate::gl!(gContestResources)).appealResults).contestant;
    let turnOrder: u8 = (*(*crate::gl!(gContestResources)).appealResults).turnOrder[contestant];
    if turnOrder != 0 {
        for i in 0..4i32 {
            if (*(*crate::gl!(gContestResources)).appealResults).turnOrder[contestant]
                > (*(*crate::gl!(gContestResources)).appealResults).turnOrder[i]
            {
                let mut jam: u8 = 0;
                (*(*crate::gl!(gContestResources)).appealResults).jamQueue[0] = i as u8;
                (*(*crate::gl!(gContestResources)).appealResults).jamQueue[1] = CONTESTANT_NONE;
                let rval: u8 = (Random() as i32 % 10) as u8;
                if rval == 0 {
                    jam = 0;
                } else if rval <= 2 {
                    jam = 10;
                } else if rval <= 4 {
                    jam = 20;
                } else if rval <= 6 {
                    jam = 30;
                } else if rval <= 8 {
                    jam = 40;
                } else {
                    jam = 60;
                }
                (*(*crate::gl!(gContestResources)).appealResults).jam = jam as i16;
                if WasAtLeastOneOpponentJammed() != 0 {
                    numStartled += 1;
                }
            }
        }
    }
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_ATTEMPT_STARTLE,
    );
    if numStartled == 0 {
        SetContestantEffectStringID2(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_MESSED_UP2,
        );
    }
}
pub(crate) unsafe fn ContestEffect_ShiftJudgeAttention() {
    let mut hitAny: u32 = FALSE as u32;
    let contestant: u8 = (*(*crate::gl!(gContestResources)).appealResults).contestant;
    if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
        [(*(*crate::gl!(gContestResources)).appealResults).contestant]
        != 0
    {
        for i in 0..4i32 {
            if (*(*crate::gl!(gContestResources)).appealResults).turnOrder[contestant]
                > (*(*crate::gl!(gContestResources)).appealResults).turnOrder[i]
                && (*(*crate::gl!(gContestResources)).status.at(i)).hasJudgesAttention() != 0
                && CanUnnerveContestant(i as u8) != 0
            {
                (*(*crate::gl!(gContestResources)).status.at(i)).set_hasJudgesAttention(FALSE);
                (*(*crate::gl!(gContestResources)).status.at(i))
                    .set_judgesAttentionWasRemoved(TRUE);
                SetContestantEffectStringID(i as u8, CONTEST_STRING_JUDGE_LOOK_AWAY2);
                hitAny = TRUE as u32;
            }
        }
    }
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_DAZZLE_ATTEMPT,
    );
    if hitAny == 0 {
        SetContestantEffectStringID2(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_MESSED_UP2,
        );
    }
}
pub(crate) unsafe fn ContestEffect_StartleMonWithJudgesAttention() {
    let mut numStartled: u8 = 0;
    let contestant: u8 = (*(*crate::gl!(gContestResources)).appealResults).contestant;
    if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
        [(*(*crate::gl!(gContestResources)).appealResults).contestant]
        != 0
    {
        for i in 0..4i32 {
            if (*(*crate::gl!(gContestResources)).appealResults).turnOrder[contestant]
                > (*(*crate::gl!(gContestResources)).appealResults).turnOrder[i]
            {
                if (*(*crate::gl!(gContestResources)).status.at(i)).hasJudgesAttention() != 0 {
                    (*(*crate::gl!(gContestResources)).appealResults).jam = 50;
                } else {
                    (*(*crate::gl!(gContestResources)).appealResults).jam = 10;
                }
                (*(*crate::gl!(gContestResources)).appealResults).jamQueue[0] = i as u8;
                (*(*crate::gl!(gContestResources)).appealResults).jamQueue[1] = CONTESTANT_NONE;
                if WasAtLeastOneOpponentJammed() != 0 {
                    numStartled += 1;
                }
            }
        }
    }
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_ATTEMPT_STARTLE,
    );
    if numStartled == 0 {
        SetContestantEffectStringID2(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_MESSED_UP2,
        );
    }
}
pub(crate) unsafe fn ContestEffect_JamsOthersButMissOneTurn() {
    (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
    .set_turnSkipped(TRUE);
    ContestEffect_StartlePrevMons();
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_ATTEMPT_STARTLE,
    );
}
pub(crate) unsafe fn ContestEffect_StartleMonsSameTypeAppeal() {
    let r#move: u16 =
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .currMove;
    JamByMoveCategory(gContestMoves[r#move].contestCategory());
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_ATTEMPT_STARTLE,
    );
}
pub(crate) unsafe fn ContestEffect_StartleMonsCoolAppeal() {
    JamByMoveCategory(CONTEST_CATEGORY_COOL);
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_ATTEMPT_STARTLE,
    );
}
pub(crate) unsafe fn ContestEffect_StartleMonsBeautyAppeal() {
    JamByMoveCategory(CONTEST_CATEGORY_BEAUTY);
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_ATTEMPT_STARTLE,
    );
}
pub(crate) unsafe fn ContestEffect_StartleMonsCuteAppeal() {
    JamByMoveCategory(CONTEST_CATEGORY_CUTE);
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_ATTEMPT_STARTLE,
    );
}
pub(crate) unsafe fn ContestEffect_StartleMonsSmartAppeal() {
    JamByMoveCategory(CONTEST_CATEGORY_SMART);
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_ATTEMPT_STARTLE,
    );
}
pub(crate) unsafe fn ContestEffect_StartleMonsToughAppeal() {
    JamByMoveCategory(CONTEST_CATEGORY_TOUGH);
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_ATTEMPT_STARTLE,
    );
}
pub(crate) unsafe fn ContestEffect_MakeFollowingMonNervous() {
    let mut hitAny: u32 = FALSE as u32;
    if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
        [(*(*crate::gl!(gContestResources)).appealResults).contestant]
        != 3
    {
        for i in 0..4i32 {
            if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
                [(*(*crate::gl!(gContestResources)).appealResults).contestant] as i32
                + 1
                == (*(*crate::gl!(gContestResources)).appealResults).turnOrder[i] as i32
            {
                if CanUnnerveContestant(i as u8) != 0 {
                    MakeContestantNervous(i as u8);
                    SetContestantEffectStringID(i as u8, CONTEST_STRING_NERVOUS);
                    hitAny = TRUE as u32;
                } else {
                    SetContestantEffectStringID(i as u8, CONTEST_STRING_UNAFFECTED);
                    hitAny = TRUE as u32;
                }
            }
        }
    }
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_UNNERVE_ATTEMPT,
    );
    if hitAny == 0 {
        SetContestantEffectStringID2(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_MESSED_UP2,
        );
    }
}
pub(crate) unsafe fn ContestEffect_MakeFollowingMonsNervous() {
    let mut numUnnerved: u8 = 0;
    let mut contestantUnnerved: u32 = FALSE as u32;
    let mut contestantIds: CArray<u8, 5> = zeroed();
    let mut oddsMod: CArray<i16, 4> = zeroed();
    let mut odds: CArray<i16, 4> = zeroed();
    memset(contestantIds.as_mut_ptr(), CONTESTANT_NONE as i32, 5);
    let mut i: i32 = 0;
    let mut numAfter: i32 = 0;
    while i < CONTESTANT_COUNT {
        if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
            [(*(*crate::gl!(gContestResources)).appealResults).contestant]
            < (*(*crate::gl!(gContestResources)).appealResults).turnOrder[i]
            && (*(*crate::gl!(gContestResources)).status.at(i)).nervous() == 0
            && Contest_IsMonsTurnDisabled(i as u8) == 0
        {
            contestantIds[{
                let t1 = numAfter;
                numAfter += 1;
                t1
            }] = i as u8;
        }
        i += 1;
    }
    if numAfter == 1 {
        odds[0] = 60;
    } else if numAfter == 2 {
        odds[0] = 30;
        odds[1] = 30;
    } else if numAfter == 3 {
        odds[0] = 20;
        odds[1] = 20;
        odds[2] = 20;
    } else {
        for i in 0..CONTESTANT_COUNT {
            odds[i] = 0;
        }
    }
    i = 0;
    while i < CONTESTANT_COUNT {
        if (*(*crate::gl!(gContestResources)).status.at(i)).hasJudgesAttention() != 0
            && IsContestantAllowedToCombo(i as u8) != 0
        {
            oddsMod[i] = gComboStarterLookupTable[gContestMoves
                [(*(*crate::gl!(gContestResources)).status.at(i)).prevMove]
                .comboStarterId] as i16
                * 10;
        } else {
            oddsMod[i] = 0;
        }
        oddsMod[i] -= ((*(*crate::gl!(gContestResources)).status.at(i)).condition / 10) as i16 * 10;
        i += 1;
    }
    if odds[0] != 0 {
        i = 0;
        while contestantIds[i] != CONTESTANT_NONE {
            if Random() as i32 % 100 < odds[i] as i32 + oddsMod[contestantIds[i]] as i32 {
                if CanUnnerveContestant(contestantIds[i]) != 0 {
                    MakeContestantNervous(contestantIds[i]);
                    SetContestantEffectStringID(contestantIds[i], CONTEST_STRING_NERVOUS);
                    numUnnerved += 1;
                } else {
                    contestantUnnerved = TRUE as u32;
                }
            } else {
                contestantUnnerved = TRUE as u32;
            }
            if contestantUnnerved != 0 {
                contestantUnnerved = FALSE as u32;
                SetContestantEffectStringID(contestantIds[i], CONTEST_STRING_UNAFFECTED);
                numUnnerved += 1;
            }
            (*(*crate::gl!(gContestResources)).appealResults).unnervedPokes[contestantIds[i]] = 1;
            i += 1;
        }
    }
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_UNNERVE_WAITING,
    );
    if numUnnerved == 0 {
        SetContestantEffectStringID2(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_MESSED_UP2,
        );
    }
}
pub(crate) unsafe fn ContestEffect_WorsenConditionOfPrevMons() {
    let mut numHit: u8 = 0;
    for i in 0..CONTESTANT_COUNT {
        if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
            [(*(*crate::gl!(gContestResources)).appealResults).contestant]
            > (*(*crate::gl!(gContestResources)).appealResults).turnOrder[i]
            && (*(*crate::gl!(gContestResources)).status.at(i)).condition > 0
            && CanUnnerveContestant(i as u8) != 0
        {
            (*(*crate::gl!(gContestResources)).status.at(i)).condition = 0;
            (*(*crate::gl!(gContestResources)).status.at(i)).set_conditionMod(CONDITION_LOSE);
            SetContestantEffectStringID(i as u8, CONTEST_STRING_REGAINED_FORM);
            numHit += 1;
        }
    }
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_TAUNT_WELL,
    );
    if numHit == 0 {
        SetContestantEffectStringID2(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_IGNORED,
        );
    }
}
pub(crate) unsafe fn ContestEffect_BadlyStartlesMonsInGoodCondition() {
    let mut numHit: u8 = 0;
    for i in 0..CONTESTANT_COUNT {
        if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
            [(*(*crate::gl!(gContestResources)).appealResults).contestant]
            > (*(*crate::gl!(gContestResources)).appealResults).turnOrder[i]
        {
            if (*(*crate::gl!(gContestResources)).status.at(i)).condition > 0 {
                (*(*crate::gl!(gContestResources)).appealResults).jam = 40;
            } else {
                (*(*crate::gl!(gContestResources)).appealResults).jam = 10;
            }
            (*(*crate::gl!(gContestResources)).appealResults).jamQueue[0] = i as u8;
            (*(*crate::gl!(gContestResources)).appealResults).jamQueue[1] = CONTESTANT_NONE;
            if WasAtLeastOneOpponentJammed() != 0 {
                numHit += 1;
            }
        }
    }
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_JAM_WELL,
    );
    if numHit == 0 {
        SetContestantEffectStringID2(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_IGNORED,
        );
    }
}
pub(crate) unsafe fn ContestEffect_BetterIfFirst() {
    if (*gContestantTurnOrder.as_ptr())
        [(*(*crate::gl!(gContestResources)).appealResults).contestant]
        == 0
    {
        let r#move: u16 =
            (*(*crate::gl!(gContestResources))
                .status
                .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
            .currMove;
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .appeal += 2 * gContestEffects[gContestMoves[r#move].effect].appeal as i16;
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_HUSTLE_STANDOUT,
        );
    }
}
pub(crate) unsafe fn ContestEffect_BetterIfLast() {
    if (*gContestantTurnOrder.as_ptr())
        [(*(*crate::gl!(gContestResources)).appealResults).contestant]
        == 3
    {
        let r#move: u16 =
            (*(*crate::gl!(gContestResources))
                .status
                .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
            .currMove;
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .appeal += 2 * gContestEffects[gContestMoves[r#move].effect].appeal as i16;
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_WORK_HARD_UNNOTICED,
        );
    }
}
pub(crate) unsafe fn ContestEffect_AppealAsGoodAsPrevOnes() {
    let mut appealSum: i32 = 0;
    for i in 0..CONTESTANT_COUNT {
        if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
            [(*(*crate::gl!(gContestResources)).appealResults).contestant]
            > (*(*crate::gl!(gContestResources)).appealResults).turnOrder[i]
        {
            appealSum += (*(*crate::gl!(gContestResources)).status.at(i)).appeal as i32;
        }
    }
    if appealSum < 0 {
        appealSum = 0;
    }
    if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
        [(*(*crate::gl!(gContestResources)).appealResults).contestant]
        == 0
        || appealSum == 0
    {
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_APPEAL_NOT_WELL,
        );
    } else {
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .appeal += (appealSum / 2) as i16;
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_WORK_BEFORE,
        );
    }
    (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
    .appeal = RoundTowardsZero(
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .appeal,
    );
}
pub(crate) unsafe fn ContestEffect_AppealAsGoodAsPrevOne() {
    let mut appeal: i16 = 0;
    if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
        [(*(*crate::gl!(gContestResources)).appealResults).contestant]
        != 0
    {
        for i in 0..CONTESTANT_COUNT {
            if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
                [(*(*crate::gl!(gContestResources)).appealResults).contestant] as i32
                - 1
                == (*(*crate::gl!(gContestResources)).appealResults).turnOrder[i] as i32
            {
                appeal = (*(*crate::gl!(gContestResources)).status.at(i)).appeal;
            }
        }
    }
    if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
        [(*(*crate::gl!(gContestResources)).appealResults).contestant]
        == 0
        || appeal <= 0
    {
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_APPEAL_NOT_WELL2,
        );
    } else {
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .appeal += appeal;
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_WORK_PRECEDING,
        );
    }
}
pub(crate) unsafe fn ContestEffect_BetterWhenLater() {
    let whichTurn: u8 = (*(*crate::gl!(gContestResources)).appealResults).turnOrder
        [(*(*crate::gl!(gContestResources)).appealResults).contestant];
    if whichTurn == 0 {
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .appeal = 10;
    } else {
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .appeal = 20 * whichTurn as i16;
    }
    if whichTurn == 0 {
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_APPEAL_NOT_SHOWN_WELL,
        );
    } else if whichTurn == 1 {
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_APPEAL_SLIGHTLY_WELL,
        );
    } else if whichTurn == 2 {
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_APPEAL_PRETTY_WELL,
        );
    } else {
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_APPEAL_EXCELLENTLY,
        );
    }
}
pub(crate) unsafe fn ContestEffect_QualityDependsOnTiming() {
    let rval: u8 = (Random() as i32 % 10) as u8;
    let mut appeal: i16 = 0;
    if rval < 3 {
        appeal = 10;
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_APPEAL_NOT_VERY_WELL,
        );
    } else if rval < 6 {
        appeal = 20;
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_APPEAL_SLIGHTLY_WELL2,
        );
    } else if rval < 8 {
        appeal = 40;
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_APPEAL_PRETTY_WELL2,
        );
    } else if rval < 9 {
        appeal = 60;
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_APPEAL_VERY_WELL,
        );
    } else {
        appeal = 80;
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_APPEAL_EXCELLENTLY2,
        );
    }
    (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
    .appeal = appeal;
}
pub(crate) unsafe fn ContestEffect_BetterIfSameType() {
    let turnOrder: i8 = (*(*crate::gl!(gContestResources)).appealResults).turnOrder
        [(*(*crate::gl!(gContestResources)).appealResults).contestant]
        as i8;
    let mut i: i8 = turnOrder - 1;
    let mut j: i8 = 0;
    if turnOrder == 0 {
        return;
    }
    loop {
        j = 0;
        while j < CONTESTANT_COUNT as i8 {
            if (*(*crate::gl!(gContestResources)).appealResults).turnOrder[j] as i32 == i as i32 {
                break;
            }
            j += 1;
        }
        if (*(*crate::gl!(gContestResources)).status.at(j)).noMoreTurns() != 0
            || (*(*crate::gl!(gContestResources)).status.at(j)).nervous() != 0
            || (*(*crate::gl!(gContestResources)).status.at(j)).numTurnsSkipped() != 0
        {
            if ({
                i -= 1;
                i
            }) < 0
            {
                return;
            }
        } else {
            break;
        }
    }
    let r#move: u16 =
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .currMove;
    if gContestMoves[r#move].contestCategory()
        == gContestMoves[(*(*crate::gl!(gContestResources)).status.at(j)).currMove]
            .contestCategory()
    {
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .appeal += gContestEffects[gContestMoves[r#move].effect].appeal as i16 * 2;
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_SAME_TYPE_GOOD,
        );
    }
}
pub(crate) unsafe fn ContestEffect_BetterIfDiffType() {
    if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
        [(*(*crate::gl!(gContestResources)).appealResults).contestant]
        != 0
    {
        let r#move: u16 =
            (*(*crate::gl!(gContestResources))
                .status
                .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
            .currMove;
        for i in 0..CONTESTANT_COUNT {
            if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
                [(*(*crate::gl!(gContestResources)).appealResults).contestant] as i32
                - 1
                == (*(*crate::gl!(gContestResources)).appealResults).turnOrder[i] as i32
                && gContestMoves[r#move].contestCategory()
                    != gContestMoves[(*(*crate::gl!(gContestResources)).status.at(i)).currMove]
                        .contestCategory()
            {
                (*(*crate::gl!(gContestResources)).status.at((*(*crate::gl!(
                    gContestResources
                ))
                .appealResults)
                    .contestant))
                .appeal += gContestEffects[gContestMoves[r#move].effect].appeal as i16 * 2;
                SetContestantEffectStringID(
                    (*(*crate::gl!(gContestResources)).appealResults).contestant,
                    CONTEST_STRING_DIFF_TYPE_GOOD,
                );
                break;
            }
        }
    }
}
pub(crate) unsafe fn ContestEffect_AffectedByPrevAppeal() {
    if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
        [(*(*crate::gl!(gContestResources)).appealResults).contestant]
        != 0
    {
        for i in 0..CONTESTANT_COUNT {
            if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
                [(*(*crate::gl!(gContestResources)).appealResults).contestant] as i32
                - 1
                == (*(*crate::gl!(gContestResources)).appealResults).turnOrder[i] as i32
            {
                if (*(*crate::gl!(gContestResources)).status.at((*(*crate::gl!(
                    gContestResources
                ))
                .appealResults)
                    .contestant))
                .appeal
                    > (*(*crate::gl!(gContestResources)).status.at(i)).appeal
                {
                    (*(*crate::gl!(gContestResources)).status.at((*(*crate::gl!(
                        gContestResources
                    ))
                    .appealResults)
                        .contestant))
                    .appeal *= 2;
                    SetContestantEffectStringID(
                        (*(*crate::gl!(gContestResources)).appealResults).contestant,
                        CONTEST_STRING_STOOD_OUT_AS_MUCH,
                    );
                } else if (*(*crate::gl!(gContestResources)).status.at((*(*crate::gl!(
                    gContestResources
                ))
                .appealResults)
                    .contestant))
                .appeal
                    < (*(*crate::gl!(gContestResources)).status.at(i)).appeal
                {
                    (*(*crate::gl!(gContestResources)).status.at((*(*crate::gl!(
                        gContestResources
                    ))
                    .appealResults)
                        .contestant))
                    .appeal = 0;
                    SetContestantEffectStringID(
                        (*(*crate::gl!(gContestResources)).appealResults).contestant,
                        CONTEST_STRING_NOT_AS_WELL,
                    );
                }
            }
        }
    }
}
pub(crate) unsafe fn ContestEffect_ImproveConditionPreventNervousness() {
    if (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
    .condition
        < 30
    {
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .condition += 10;
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .set_conditionMod(CONDITION_GAIN);
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_CONDITION_ROSE,
        );
    } else {
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_NO_CONDITION_IMPROVE,
        );
    }
}
pub(crate) unsafe fn ContestEffect_BetterWithGoodCondition() {
    (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
    .set_appealTripleCondition(TRUE);
    if (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
    .condition
        != 0
    {
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_HOT_STATUS,
        );
    } else {
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_BAD_CONDITION_WEAK_APPEAL,
        );
    }
}
pub(crate) unsafe fn ContestEffect_NextAppealEarlier() {
    let mut i: i8 = 0;
    let mut j: i8 = 0;
    let mut turnOrder: CArray<u8, 4> = zeroed();
    if (*(*crate::gl!(gContestResources)).contest).appealNumber != CONTEST_LAST_APPEAL {
        for i in 0..(CONTESTANT_COUNT as i8) {
            turnOrder[i] = (*(*crate::gl!(gContestResources)).status.at(i)).nextTurnOrder;
        }
        turnOrder[(*(*crate::gl!(gContestResources)).appealResults).contestant] = CONTESTANT_NONE;
        i = 0;
        while i < CONTESTANT_COUNT as i8 {
            j = 0;
            while j < CONTESTANT_COUNT as i8 {
                if j as i32 != (*(*crate::gl!(gContestResources)).appealResults).contestant as i32
                    && i as i32 == turnOrder[j] as i32
                    && turnOrder[j]
                        == (*(*crate::gl!(gContestResources)).status.at(j)).nextTurnOrder
                {
                    turnOrder[j] += 1;
                    break;
                }
                j += 1;
            }
            if j == CONTESTANT_COUNT as i8 {
                break;
            }
            i += 1;
        }
        turnOrder[(*(*crate::gl!(gContestResources)).appealResults).contestant] = 0;
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .set_turnOrderMod(1);
        for i in 0..(CONTESTANT_COUNT as i8) {
            (*(*crate::gl!(gContestResources)).status.at(i)).nextTurnOrder = turnOrder[i];
        }
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .set_turnOrderModAction(1);
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_MOVE_UP_LINE,
        );
    }
}
pub(crate) unsafe fn ContestEffect_NextAppealLater() {
    let mut i: i8 = 0;
    let mut j: i8 = 0;
    let mut turnOrder: CArray<u8, 4> = zeroed();
    if (*(*crate::gl!(gContestResources)).contest).appealNumber != CONTEST_LAST_APPEAL {
        for i in 0..(CONTESTANT_COUNT as i8) {
            turnOrder[i] = (*(*crate::gl!(gContestResources)).status.at(i)).nextTurnOrder;
        }
        turnOrder[(*(*crate::gl!(gContestResources)).appealResults).contestant] = CONTESTANT_NONE;
        i = 3;
        while i > -1 {
            j = 0;
            while j < CONTESTANT_COUNT as i8 {
                if j as i32 != (*(*crate::gl!(gContestResources)).appealResults).contestant as i32
                    && i as i32 == turnOrder[j] as i32
                    && turnOrder[j]
                        == (*(*crate::gl!(gContestResources)).status.at(j)).nextTurnOrder
                {
                    turnOrder[j] -= 1;
                    break;
                }
                j += 1;
            }
            if j == CONTESTANT_COUNT as i8 {
                break;
            }
            i -= 1;
        }
        turnOrder[(*(*crate::gl!(gContestResources)).appealResults).contestant] = 3;
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .set_turnOrderMod(1);
        for i in 0..(CONTESTANT_COUNT as i8) {
            (*(*crate::gl!(gContestResources)).status.at(i)).nextTurnOrder = turnOrder[i];
        }
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .set_turnOrderModAction(2);
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_MOVE_BACK_LINE,
        );
    }
}
pub(crate) fn ContestEffect_MakeScramblingTurnOrderEasier() {}
pub(crate) unsafe fn ContestEffect_ScrambleNextTurnOrder() {
    let mut i: i8 = 0;
    let mut turnOrder: CArray<u8, 4> = zeroed();
    let mut unselectedContestants: CArray<u8, 4> = zeroed();
    if (*(*crate::gl!(gContestResources)).contest).appealNumber != CONTEST_LAST_APPEAL {
        for i in 0..(CONTESTANT_COUNT as i8) {
            turnOrder[i] = (*(*crate::gl!(gContestResources)).status.at(i)).nextTurnOrder;
            unselectedContestants[i] = i as u8;
        }
        i = 0;
        while i < CONTESTANT_COUNT as i8 {
            let mut rval: u8 = rem_i32(Random() as i32, CONTESTANT_COUNT - i as i32) as u8;
            for j in 0..(CONTESTANT_COUNT as i8) {
                if unselectedContestants[j] != CONTESTANT_NONE {
                    if rval == 0 {
                        turnOrder[j] = i as u8;
                        unselectedContestants[j] = CONTESTANT_NONE;
                        break;
                    } else {
                        rval -= 1;
                    }
                }
            }
            i += 1;
        }
        for i in 0..(CONTESTANT_COUNT as i8) {
            (*(*crate::gl!(gContestResources)).status.at(i)).nextTurnOrder = turnOrder[i];
            (*(*crate::gl!(gContestResources)).status.at(i)).set_turnOrderMod(2);
        }
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .set_turnOrderModAction(3);
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_SCRAMBLE_ORDER,
        );
    }
}
pub(crate) unsafe fn ContestEffect_ExciteAudienceInAnyContest() {
    if gContestMoves[(*(*crate::gl!(gContestResources)).status.at((*(*crate::gl!(
        gContestResources
    ))
    .appealResults)
        .contestant))
    .currMove]
        .contestCategory() as u16
        != gSpecialVar_ContestCategory.get()
    {
        (*(*crate::gl!(gContestResources))
            .status
            .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
        .set_overrideCategoryExcitementMod(TRUE);
    }
}
pub(crate) unsafe fn ContestEffect_BadlyStartleMonsWithGoodAppeals() {
    let mut numJammed: u8 = 0;
    for i in 0..CONTESTANT_COUNT {
        if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
            [(*(*crate::gl!(gContestResources)).appealResults).contestant]
            > (*(*crate::gl!(gContestResources)).appealResults).turnOrder[i]
        {
            if (*(*crate::gl!(gContestResources)).status.at(i)).appeal > 0 {
                (*(*crate::gl!(gContestResources)).appealResults).jam =
                    (*(*crate::gl!(gContestResources)).status.at(i)).appeal / 2;
                (*(*crate::gl!(gContestResources)).appealResults).jam =
                    RoundUp((*(*crate::gl!(gContestResources)).appealResults).jam);
            } else {
                (*(*crate::gl!(gContestResources)).appealResults).jam = 10;
            }
            (*(*crate::gl!(gContestResources)).appealResults).jamQueue[0] = i as u8;
            (*(*crate::gl!(gContestResources)).appealResults).jamQueue[1] = CONTESTANT_NONE;
            if WasAtLeastOneOpponentJammed() != 0 {
                numJammed += 1;
            }
        }
    }
    if numJammed == 0 {
        SetContestantEffectStringID2(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_MESSED_UP2,
        );
    }
    SetContestantEffectStringID(
        (*(*crate::gl!(gContestResources)).appealResults).contestant,
        CONTEST_STRING_ATTEMPT_STARTLE,
    );
}
pub(crate) unsafe fn ContestEffect_BetterWhenAudienceExcited() {
    let mut appeal: i16 = 0;
    if (*(*crate::gl!(gContestResources)).contest).applauseLevel == 0 {
        appeal = 10;
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_APPEAL_NOT_VERY_WELL,
        );
    } else if (*(*crate::gl!(gContestResources)).contest).applauseLevel == 1 {
        appeal = 20;
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_APPEAL_SLIGHTLY_WELL2,
        );
    } else if (*(*crate::gl!(gContestResources)).contest).applauseLevel == 2 {
        appeal = 30;
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_APPEAL_PRETTY_WELL2,
        );
    } else if (*(*crate::gl!(gContestResources)).contest).applauseLevel == 3 {
        appeal = 50;
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_APPEAL_VERY_WELL,
        );
    } else {
        appeal = 60;
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_APPEAL_EXCELLENTLY2,
        );
    }
    (*(*crate::gl!(gContestResources))
        .status
        .at((*(*crate::gl!(gContestResources)).appealResults).contestant))
    .appeal = appeal;
}
pub(crate) unsafe fn ContestEffect_DontExciteAudience() {
    if (*(*crate::gl!(gContestResources)).excitement).frozen() == 0 {
        (*(*crate::gl!(gContestResources)).excitement).set_frozen(TRUE);
        (*(*crate::gl!(gContestResources)).excitement)
            .set_freezer((*(*crate::gl!(gContestResources)).appealResults).contestant);
        SetContestantEffectStringID(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_ATTRACTED_ATTENTION,
        );
    }
}
unsafe fn JamByMoveCategory(category: u8) {
    let mut numJammed: i32 = 0;
    for i in 0..CONTESTANT_COUNT {
        if (*(*crate::gl!(gContestResources)).appealResults).turnOrder
            [(*(*crate::gl!(gContestResources)).appealResults).contestant]
            > (*(*crate::gl!(gContestResources)).appealResults).turnOrder[i]
        {
            if category
                == gContestMoves[(*(*crate::gl!(gContestResources)).status.at(i)).currMove]
                    .contestCategory()
            {
                (*(*crate::gl!(gContestResources)).appealResults).jam = 40;
            } else {
                (*(*crate::gl!(gContestResources)).appealResults).jam = 10;
            }
            (*(*crate::gl!(gContestResources)).appealResults).jamQueue[0] = i as u8;
            (*(*crate::gl!(gContestResources)).appealResults).jamQueue[1] = CONTESTANT_NONE;
            if WasAtLeastOneOpponentJammed() != 0 {
                numJammed += 1;
            }
        }
    }
    if numJammed == 0 {
        SetContestantEffectStringID2(
            (*(*crate::gl!(gContestResources)).appealResults).contestant,
            CONTEST_STRING_MESSED_UP2,
        );
    }
}
unsafe fn CanUnnerveContestant(i: u8) -> u8 {
    (*(*crate::gl!(gContestResources)).appealResults).unnervedPokes[i] = 1;
    if (*(*crate::gl!(gContestResources)).status.at(i)).immune() != 0 {
        SetContestantEffectStringID(i, CONTEST_STRING_AVOID_SEEING);
        return FALSE;
    } else if (*(*crate::gl!(gContestResources)).status.at(i)).jamSafetyCount != 0 {
        (*(*crate::gl!(gContestResources)).status.at(i)).jamSafetyCount -= 1;
        SetContestantEffectStringID(i, CONTEST_STRING_AVERT_GAZE);
        return FALSE;
    } else if (*(*crate::gl!(gContestResources)).status.at(i)).noMoreTurns() == 0
        && (*(*crate::gl!(gContestResources)).status.at(i)).numTurnsSkipped() == 0
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn WasAtLeastOneOpponentJammed() -> u8 {
    let mut jamBuffer: CArray<i16, 4> = CArray([0, 0, 0, 0]);
    let mut i: i32 = 0;
    while (*(*crate::gl!(gContestResources)).appealResults).jamQueue[i] != CONTESTANT_NONE {
        let contestant: u8 = (*(*crate::gl!(gContestResources)).appealResults).jamQueue[i];
        if CanUnnerveContestant(contestant) != 0 {
            (*(*crate::gl!(gContestResources)).appealResults).jam2 =
                (*(*crate::gl!(gContestResources)).appealResults).jam;
            if (*(*crate::gl!(gContestResources)).status.at(contestant)).moreEasilyStartled() != 0 {
                (*(*crate::gl!(gContestResources)).appealResults).jam2 *= 2;
            }
            if (*(*crate::gl!(gContestResources)).status.at(contestant)).resistant() != 0 {
                (*(*crate::gl!(gContestResources)).appealResults).jam2 = 10;
                SetContestantEffectStringID(contestant, CONTEST_STRING_LITTLE_DISTRACTED);
            } else {
                (*(*crate::gl!(gContestResources)).appealResults).jam2 -=
                    (*(*crate::gl!(gContestResources)).status.at(contestant)).jamReduction as i16;
                if (*(*crate::gl!(gContestResources)).appealResults).jam2 <= 0 {
                    (*(*crate::gl!(gContestResources)).appealResults).jam2 = 0;
                    SetContestantEffectStringID(contestant, CONTEST_STRING_NOT_FAZED);
                } else {
                    JamContestant(
                        contestant,
                        (*(*crate::gl!(gContestResources)).appealResults).jam2 as u8,
                    );
                    SetStartledString(
                        contestant,
                        (*(*crate::gl!(gContestResources)).appealResults).jam2 as u8,
                    );
                    jamBuffer[contestant] = (*(*crate::gl!(gContestResources)).appealResults).jam2;
                }
            }
        }
        i += 1;
    }
    for i in 0..CONTESTANT_COUNT {
        if jamBuffer[i] != 0 {
            return TRUE;
        }
    }
    FALSE
}
unsafe fn JamContestant(i: u8, jam: u8) {
    (*(*crate::gl!(gContestResources)).status.at(i)).appeal -= jam as i16;
    (*(*crate::gl!(gContestResources)).status.at(i)).jam += jam;
}
fn RoundTowardsZero(mut score: i16) -> i16 {
    let absScore: i16 = ((if score < 0 {
        -(score as i32)
    } else {
        score as i32
    }) % 10) as i16;
    if score < 0 {
        if absScore != 0 {
            score -= 10 - absScore;
        }
    } else {
        score -= absScore;
    }
    score
}
unsafe fn RoundUp(mut score: i16) -> i16 {
    let absScore: i16 = ((if score < 0 {
        -(score as i32)
    } else {
        score as i32
    }) % 10) as i16;
    if absScore != 0 {
        score += 10 - absScore;
    }
    score
}
