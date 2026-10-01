//! Translated from `src/apprentice.c` by tools/rustport/c2rs.py.
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
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gGameLanguage;
use crate::agb_main::gMain;
use crate::battle_tower::{CalcApprenticeChecksum, FrontierSpeechToString};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::VarSet;
use crate::event_object_movement::FreezeObjectEvents;
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_0x8005, gSpecialVar_0x8006, gSpecialVar_Result};
use crate::field_player_avatar::{PlayerFreeze, StopPlayerAvatar};
use crate::international_string_util::TVShowConvertInternationalString;
use crate::item_menu::ApprenticeOpenBagMenu;
use crate::load_save::gSaveBlock2Ptr;
use crate::menu::{
    AddTextPrinterForMessage, ClearStdWindowAndFrameToTransparent, CreateWindowTemplate,
    DrawDialogueFrame, InitMenuInUpperLeftCornerNormal, Menu_ProcessInput, Menu_ProcessInputNoWrap,
    RunTextPrintersAndIsPrinter0Active, SetStandardWindowBorderStyle,
};
use crate::new_game::GetTrainerId;
use crate::party_menu::ItemIdToBattleMoveId;
use crate::pokemon::CanSpeciesLearnTMHM;
use crate::random::Random;
use crate::script::{LockPlayerFieldControls, ScriptContext_Enable};
use crate::script_menu::{ConvertPixelWidthToTileWidth, ScriptMenu_AdjustLeftCoordFromWidth};
use crate::sound::PlaySE;
use crate::string_util::ConvertInternationalString;
use crate::string_util::{
    ConvertIntToDecimalStringN, StringCopy, StringCopy_PlayerName, StringExpandPlaceholders,
};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3, gStringVar4};
use crate::task::gTasks;
use crate::task::{DestroyTask, SwitchTaskToFollowupFunc};
use crate::task::{task_get, task_set};
use crate::text::GetStringWidth;
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{CopyWindowToVram, PutWindowTilemap, RemoveWindow};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AddWindow` with this module's view of its types.
#[inline]
unsafe fn AddWindow(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::AddWindow(a0 as _) }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `SetTaskFuncWithFollowupFunc` with this module's view of its types.
#[inline]
unsafe fn SetTaskFuncWithFollowupFunc(
    a0: u8,
    a1: Option<unsafe fn(u8)>,
    a2: Option<unsafe fn(u8)>,
) {
    unsafe {
        crate::task::SetTaskFuncWithFollowupFunc(
            a0,
            core::mem::transmute(a1),
            core::mem::transmute(a2),
        );
    }
}
// The C's names for task and sprite data slots.
const tNoBButton: usize = 4;
const tWrapAround: usize = 5;
const tWindowId: usize = 6;
// Data tables (translate with cdata.py): gApprentices sApprenticeFirstMeetingTexts sApprenticeWhichMonTexts sApprenticeHeldItemTexts sApprenticeWhichMoveTexts sApprenticeWhichMonFirstTexts sApprenticePickWinSpeechTexts sApprenticeChallengeTexts sValidApprenticeMoves sQuestionPossibilities sApprenticeFunctions sInitialApprenticeIds

/// `struct ApprenticePartyMovesData`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct ApprenticePartyMovesData {
    pub moveCounter: u8,
    pub moves: CArray<CArray<u16, 5>, 3>,
    pub moveSlots: CArray<CArray<u8, 5>, 3>,
}

unsafe impl Sync for ApprenticePartyMovesData {}

/// `struct ApprenticeQuestionData`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct ApprenticeQuestionData {
    pub speciesId: u16,
    pub altSpeciesId: u16,
    pub move1: u16,
    pub move2: u16,
}

unsafe impl Sync for ApprenticeQuestionData {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<ApprenticePartyMovesData>() == 48);
    assert!(offset_of!(ApprenticePartyMovesData, moveCounter) == 0);
    assert!(offset_of!(ApprenticePartyMovesData, moves) == 2);
    assert!(offset_of!(ApprenticePartyMovesData, moveSlots) == 32);
    assert!(size_of::<ApprenticeQuestionData>() == 8);
    assert!(offset_of!(ApprenticeQuestionData, speciesId) == 0);
    assert!(offset_of!(ApprenticeQuestionData, altSpeciesId) == 2);
    assert!(offset_of!(ApprenticeQuestionData, move1) == 4);
    assert!(offset_of!(ApprenticeQuestionData, move2) == 6);
};

static gApprentices: Table<CArray<ApprenticeTrainer, 16>> =
    Table((&raw const crate::data::apprentice::gApprentices).cast());
static sApprenticeChallengeTexts: Table<CArray<*mut u8, 16>> =
    Table((&raw const crate::data::apprentice::sApprenticeChallengeTexts).cast());
static sApprenticeFirstMeetingTexts: Table<CArray<CArray<*mut u8, 4>, 16>> =
    Table((&raw const crate::data::apprentice::sApprenticeFirstMeetingTexts).cast());
static sApprenticeFunctions: Table<CArray<Option<unsafe fn()>, 26>> =
    Table((&raw const crate::data::apprentice::sApprenticeFunctions).cast());
static sApprenticeHeldItemTexts: Table<CArray<CArray<*mut u8, 5>, 16>> =
    Table((&raw const crate::data::apprentice::sApprenticeHeldItemTexts).cast());
static sApprenticePickWinSpeechTexts: Table<CArray<CArray<*mut u8, 2>, 16>> =
    Table((&raw const crate::data::apprentice::sApprenticePickWinSpeechTexts).cast());
static sApprenticeWhichMonFirstTexts: Table<CArray<CArray<*mut u8, 2>, 16>> =
    Table((&raw const crate::data::apprentice::sApprenticeWhichMonFirstTexts).cast());
static sApprenticeWhichMonTexts: Table<CArray<CArray<*mut u8, 2>, 16>> =
    Table((&raw const crate::data::apprentice::sApprenticeWhichMonTexts).cast());
static sApprenticeWhichMoveTexts: Table<CArray<CArray<*mut u8, 2>, 16>> =
    Table((&raw const crate::data::apprentice::sApprenticeWhichMoveTexts).cast());
static sInitialApprenticeIds: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::apprentice::sInitialApprenticeIds).cast());
static sQuestionPossibilities: Table<CArray<u8, 10>> =
    Table((&raw const crate::data::apprentice::sQuestionPossibilities).cast());
static sValidApprenticeMoves: Table<CArray<u8, 355>> =
    Table((&raw const crate::data::apprentice::sValidApprenticeMoves).cast());

#[unsafe(link_section = "common_data")]
pub static mut gApprenticePartyMovesData: *mut ApprenticePartyMovesData = null_mut();
#[unsafe(link_section = "common_data")]
pub static mut gApprenticeQuestionData: *mut ApprenticeQuestionData = null_mut();
#[unsafe(link_section = "common_data")]
pub static mut gApprenticeFunc: Option<unsafe fn()> = None;

/// `AddTextPrinterParameterized` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinterParameterized(
    a0: u8,
    a1: u8,
    a2: *mut u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe {
        crate::text::AddTextPrinterParameterized(
            a0,
            a1,
            a2 as _,
            a3,
            a4,
            a5,
            core::mem::transmute(a6),
        )
    }
}
/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `GetItemName` with this module's view of its types.
#[inline]
unsafe fn GetItemName(a0: u16) -> *mut u8 {
    crate::item::GetItemName(a0) as *mut u8
}

pub unsafe fn BufferApprenticeChallengeText(saveApprenticeId: u8) {
    let mut num: u8 = (*gSaveBlock2Ptr).apprentices[saveApprenticeId].number;
    let mut i: u8 = 0;
    while num != 0 && i < APPRENTICE_COUNT as u8 {
        num = (num as i32 / 10) as u8;
        i += 1;
    }
    StringCopy_PlayerName(
        gStringVar1.as_mut_ptr(),
        (*gSaveBlock2Ptr).apprentices[saveApprenticeId]
            .playerName
            .as_mut_ptr(),
    );
    ConvertInternationalString(
        gStringVar1.as_mut_ptr(),
        (*gSaveBlock2Ptr).apprentices[saveApprenticeId].language,
    );
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        (*gSaveBlock2Ptr).apprentices[saveApprenticeId].number as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        i,
    );
    let challengeText: *mut u8 =
        sApprenticeChallengeTexts[(*gSaveBlock2Ptr).apprentices[saveApprenticeId].id()];
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), challengeText);
}
pub unsafe fn Apprentice_ScriptContext_Enable() {
    ScriptContext_Enable();
}
pub unsafe fn ResetApprenticeStruct(apprentice: *mut Apprentice) {
    for i in 0..6u8 {
        (*apprentice).speechWon[i] = EC_EMPTY_WORD;
    }
    (*apprentice).playerName[0] = EOS;
    (*apprentice).set_id(NUM_APPRENTICES);
}
#[unsafe(no_mangle)]
pub unsafe fn ResetAllApprenticeData() {
    let mut j: u8 = 0;
    (*gSaveBlock2Ptr).playerApprentice.set_saveId(0);
    for i in 0..(APPRENTICE_COUNT as u8) {
        j = 0;
        while j < 6 {
            (*gSaveBlock2Ptr).apprentices[i].speechWon[j] = EC_EMPTY_WORD;
            j += 1;
        }
        (*gSaveBlock2Ptr).apprentices[i].set_id(NUM_APPRENTICES);
        (*gSaveBlock2Ptr).apprentices[i].playerName[0] = EOS;
        (*gSaveBlock2Ptr).apprentices[i].set_lvlMode(0);
        (*gSaveBlock2Ptr).apprentices[i].number = 0;
        (*gSaveBlock2Ptr).apprentices[i].numQuestions = 0;
        for j in 0..TRAINER_ID_LENGTH {
            (*gSaveBlock2Ptr).apprentices[i].playerId[j] = 0;
        }
        (*gSaveBlock2Ptr).apprentices[i].language = gGameLanguage;
        (*gSaveBlock2Ptr).apprentices[i].checksum = 0;
    }
    Script_ResetPlayerApprentice();
}
unsafe fn GivenApprenticeLvlMode() -> u8 {
    ((*gSaveBlock2Ptr).playerApprentice.lvlMode() != 0) as u8
}
unsafe fn SetApprenticeId() {
    if (*gSaveBlock2Ptr).apprentices[0].number == 0 {
        loop {
            (*gSaveBlock2Ptr).playerApprentice.id = sInitialApprenticeIds[Random() % 8];
            if (*gSaveBlock2Ptr).playerApprentice.id != (*gSaveBlock2Ptr).apprentices[0].id() {
                break;
            }
        }
    } else {
        loop {
            (*gSaveBlock2Ptr).playerApprentice.id = (Random() as i32 % 16) as u8;
            if (*gSaveBlock2Ptr).playerApprentice.id != (*gSaveBlock2Ptr).apprentices[0].id() {
                break;
            }
        }
    }
}
unsafe fn SetPlayersApprenticeLvlMode(mode: u8) {
    (*gSaveBlock2Ptr).playerApprentice.set_lvlMode(mode);
}
pub(crate) unsafe fn ShuffleApprenticeSpecies() {
    let mut species: CArray<u8, 10> = zeroed();
    for i in 0..10u8 {
        species[i] = i;
    }
    let mut i: u8 = 0;
    while i < 50 {
        let rand1: u8 = (Random() % 10) as u8;
        let rand2: u8 = (Random() % 10) as u8;
        let temp: u8 = species[rand1];
        species[rand1] = species[rand2];
        species[rand2] = temp;
        i += 1;
    }
    for i in 0..(MULTI_PARTY_SIZE as u8) {
        (*gSaveBlock2Ptr).playerApprentice.speciesIds[i] =
            (species[i as i32 * 2] & 0xF) << 4 | species[i as i32 * 2 + 1] & 0xF;
    }
}
unsafe fn GetMonIdForQuestion(questionId: u8, party: *mut u8, partySlot: *mut u8) -> u8 {
    let mut count: u8 = 0;
    let mut monId: u8 = 0;
    if questionId == QUESTION_ID_WHICH_MOVE {
        loop {
            monId = (Random() as i32 % 3) as u8;
            count = 0;
            for i in 0..NUM_WHICH_MOVE_QUESTIONS {
                if (*gApprenticePartyMovesData).moves[monId][i] != MOVE_NONE {
                    count += 1;
                }
            }
            if count <= MULTI_PARTY_SIZE as u8 {
                break;
            }
        }
    } else if questionId == QUESTION_ID_WHAT_ITEM {
        monId = *party.at(*partySlot);
        *partySlot += 1;
    }
    monId
}
unsafe fn SetRandomQuestionData() {
    let mut questionOrder: CArray<u8, 10> = zeroed();
    let mut partyOrder: CArray<u8, 3> = zeroed();
    let mut j: u8 = 0;
    let mut rand1: u8 = 0;
    let mut rand2: u8 = 0;
    let mut id: u8 = 0;
    for i in 0..3u8 {
        partyOrder[i] = i;
    }
    let mut i: u8 = 0;
    while i < 10 {
        rand1 = (Random() % 3) as u8;
        rand2 = (Random() % 3) as u8;
        let temp: u8 = partyOrder[rand1];
        partyOrder[rand1] = partyOrder[rand2];
        partyOrder[rand2] = temp;
        i += 1;
    }
    for i in 0..10u8 {
        questionOrder[i] = sQuestionPossibilities[i];
    }
    for i in 0..50u8 {
        rand1 = (Random() % 10) as u8;
        rand2 = (Random() % 10) as u8;
        let temp: u8 = questionOrder[rand1];
        questionOrder[rand1] = questionOrder[rand2];
        questionOrder[rand2] = temp;
    }
    gApprenticePartyMovesData = AllocZeroed(48) as *mut ApprenticePartyMovesData;
    (*gApprenticePartyMovesData).moveCounter = 0;
    i = 0;
    while i < NUM_WHICH_MOVE_QUESTIONS {
        for j in 0..(MULTI_PARTY_SIZE as u8) {
            (*gApprenticePartyMovesData).moveSlots[j][i] = MAX_MON_MOVES as u8;
        }
        i += 1;
    }
    let mut partySlot: u8 = 0;
    for i in 0..APPRENTICE_MAX_QUESTIONS {
        (*gSaveBlock2Ptr).playerApprentice.questions[i].set_questionId(questionOrder[i]);
        if questionOrder[i] != QUESTION_ID_WHICH_FIRST {
            (*gSaveBlock2Ptr).playerApprentice.questions[i].set_monId(GetMonIdForQuestion(
                questionOrder[i],
                partyOrder.as_mut_ptr(),
                &raw mut partySlot,
            ));
            id = (*gSaveBlock2Ptr).playerApprentice.questions[i].monId();
            if questionOrder[i] == QUESTION_ID_WHICH_MOVE {
                loop {
                    rand1 = (Random() as i32 % 4) as u8;
                    j = 0;
                    while (j as i32) < (*gApprenticePartyMovesData).moveCounter as i32 + 1 {
                        if (*gApprenticePartyMovesData).moveSlots[id][j] == rand1 {
                            break;
                        }
                        j += 1;
                    }
                    if j as i32 == (*gApprenticePartyMovesData).moveCounter as i32 + 1 {
                        break;
                    }
                }
                (*gApprenticePartyMovesData).moveSlots[id]
                    [(*gApprenticePartyMovesData).moveCounter] = rand1;
                (*gSaveBlock2Ptr).playerApprentice.questions[i].set_moveSlot(rand1);
                (*gSaveBlock2Ptr).playerApprentice.questions[i].data =
                    GetRandomAlternateMove((*gSaveBlock2Ptr).playerApprentice.questions[i].monId());
            }
        }
    }
    Free(gApprenticePartyMovesData as *mut c_void);
    gApprenticePartyMovesData = null_mut();
}
unsafe fn GetRandomAlternateMove(monId: u8) -> u16 {
    let mut species: u16 = 0;
    let mut needTMs: u32 = FALSE as u32;
    let mut r#move: u16 = MOVE_NONE;
    let mut shouldUseMove: u32 = 0;
    let mut level: u8 = 0;
    let mut id: u8 = (if monId < 3 {
        shr_i32(
            (*gSaveBlock2Ptr).playerApprentice.speciesIds[monId] as i32,
            (shr_i32(
                (*gSaveBlock2Ptr).playerApprentice.party() as i32,
                monId as u32,
            ) as u32
                & 1)
                << 2,
        ) & 0xF
    } else {
        0
    }) as u8;
    species = gApprentices[(*gSaveBlock2Ptr).playerApprentice.id].species[id];
    let learnset: *mut u16 = (*(&raw const crate::data::pokemon::gLevelUpLearnsets)
        .cast::<CArray<*mut u16, 0>>())[species];
    if (*gSaveBlock2Ptr).playerApprentice.lvlMode() == APPRENTICE_LVL_MODE_50 {
        level = FRONTIER_MAX_LEVEL_50;
    } else {
        level = 60;
    }
    let mut j: u8 = 0;
    while *learnset.at(j) != LEVEL_UP_END {
        if *learnset.at(j) as i32 & LEVEL_UP_MOVE_LV > (level as i32) << 9 {
            break;
        }
        j += 1;
    }
    let numLearnsetMoves: u8 = j;
    let mut i: u8 = 0;
    while i < 5 {
        if Random() as i32 % 2 == 0 || needTMs == TRUE as u32 {
            loop {
                loop {
                    id = (Random() as i32 % 58) as u8;
                    shouldUseMove = CanSpeciesLearnTMHM(species, id);
                    if shouldUseMove != 0 {
                        break;
                    }
                }
                r#move = ItemIdToBattleMoveId(ITEM_TM01 + id as u16);
                shouldUseMove = TRUE as u32;
                if numLearnsetMoves <= MAX_MON_MOVES as u8 {
                    j = 0;
                } else {
                    j = numLearnsetMoves - MAX_MON_MOVES as u8;
                }
                while j < numLearnsetMoves {
                    if *learnset.at(j) as i32 & LEVEL_UP_MOVE_ID as i32 == r#move as i32 {
                        shouldUseMove = FALSE as u32;
                        break;
                    }
                    j += 1;
                }
                if shouldUseMove == TRUE as u32 {
                    break;
                }
            }
        } else {
            if numLearnsetMoves <= MAX_MON_MOVES as u8 {
                needTMs = TRUE as u32;
                continue;
            } else {
                loop {
                    let learnsetId: u8 =
                        rem_i32(Random() as i32, numLearnsetMoves as i32 - MAX_MON_MOVES) as u8;
                    r#move = *learnset.at(learnsetId) & LEVEL_UP_MOVE_ID;
                    shouldUseMove = TRUE as u32;
                    for j in (numLearnsetMoves - MAX_MON_MOVES as u8)..numLearnsetMoves {
                        if *learnset.at(j) as i32 & LEVEL_UP_MOVE_ID as i32 == r#move as i32 {
                            shouldUseMove = FALSE as u32;
                            break;
                        }
                    }
                    if shouldUseMove == TRUE as u32 {
                        break;
                    }
                }
            }
        }
        if TrySetMove(monId, r#move) != 0 {
            if sValidApprenticeMoves[r#move] != 0 {
                break;
            }
            i += 1;
        }
    }
    (*gApprenticePartyMovesData).moveCounter += 1;
    r#move
}
unsafe fn TrySetMove(monId: u8, r#move: u16) -> u8 {
    for i in 0..NUM_WHICH_MOVE_QUESTIONS {
        if (*gApprenticePartyMovesData).moves[monId][i] == r#move {
            return FALSE;
        }
    }
    (*gApprenticePartyMovesData).moves[monId][(*gApprenticePartyMovesData).moveCounter] = r#move;
    TRUE
}
unsafe fn GetLatestLearnedMoves(species: u16, moves: *mut u16) {
    let mut level: u8 = 0;
    if (*gSaveBlock2Ptr).playerApprentice.lvlMode() == APPRENTICE_LVL_MODE_50 {
        level = FRONTIER_MAX_LEVEL_50;
    } else {
        level = 60;
    }
    let learnset: *mut u16 = (*(&raw const crate::data::pokemon::gLevelUpLearnsets)
        .cast::<CArray<*mut u16, 0>>())[species];
    let mut i: u8 = 0;
    while *learnset.at(i) != LEVEL_UP_END {
        if *learnset.at(i) as i32 & LEVEL_UP_MOVE_LV > (level as i32) << 9 {
            break;
        }
        i += 1;
    }
    let mut numLearnsetMoves: u8 = i;
    if numLearnsetMoves > MAX_MON_MOVES as u8 {
        numLearnsetMoves = MAX_MON_MOVES as u8;
    }
    for j in 0..numLearnsetMoves {
        *moves.at(j) = *learnset.at(i as i32 - 1 - j as i32) & LEVEL_UP_MOVE_ID;
    }
}
unsafe fn GetDefaultMove(monId: u8, speciesArrayId: u8, moveSlot: u8) -> u16 {
    let mut moves: CArray<u16, 4> = zeroed();
    if (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() < NUM_WHICH_MON_QUESTIONS {
        return MOVE_NONE;
    }
    let mut numQuestions: u8 = 0;
    let mut i: u8 = 0;
    while i < APPRENTICE_MAX_QUESTIONS
        && (*gSaveBlock2Ptr).playerApprentice.questions[i].questionId() != 0
    {
        numQuestions += 1;
        i += 1;
    }
    GetLatestLearnedMoves(
        gApprentices[(*gSaveBlock2Ptr).playerApprentice.id].species[speciesArrayId],
        moves.as_mut_ptr(),
    );
    i = 0;
    while i < numQuestions
        && (i as i32) < (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3
    {
        if (*gSaveBlock2Ptr).playerApprentice.questions[i].questionId() == QUESTION_ID_WHICH_MOVE
            && (*gSaveBlock2Ptr).playerApprentice.questions[i].monId() == monId
            && (*gSaveBlock2Ptr).playerApprentice.questions[i].suggestedChange() != 0
        {
            moves[(*gSaveBlock2Ptr).playerApprentice.questions[i].moveSlot()] =
                (*gSaveBlock2Ptr).playerApprentice.questions[i].data;
        }
        i += 1;
    }
    moves[moveSlot]
}
unsafe fn SaveApprenticeParty(numQuestions: u8) {
    let mut apprenticeMons: CArray<*mut ApprenticeMon, 3> = zeroed();
    let mut j: u8 = 0;
    let mut speciesTableId: u32 = 0;
    for i in 0..(MULTI_PARTY_SIZE as u8) {
        (*gSaveBlock2Ptr).apprentices[0].party[i].species = 0;
        (*gSaveBlock2Ptr).apprentices[0].party[i].item = ITEM_NONE;
        for j in 0..(MAX_MON_MOVES as u8) {
            (*gSaveBlock2Ptr).apprentices[0].party[i].moves[j] = 0;
        }
    }
    j = (*gSaveBlock2Ptr).playerApprentice.leadMonId();
    for i in 0..(MULTI_PARTY_SIZE as u8) {
        apprenticeMons[j] = &raw mut (*gSaveBlock2Ptr).apprentices[0].party[i];
        j = ((j as i32 + 1) % 3) as u8;
    }
    let mut i: u8 = 0;
    while i < MULTI_PARTY_SIZE as u8 {
        speciesTableId = (if i < 3 {
            shr_i32(
                (*gSaveBlock2Ptr).playerApprentice.speciesIds[i] as i32,
                (shr_i32((*gSaveBlock2Ptr).playerApprentice.party() as i32, i as u32) as u32 & 1)
                    << 2,
            ) & 0xF
        } else {
            0
        }) as u32;
        (*apprenticeMons[i]).species =
            gApprentices[(*gSaveBlock2Ptr).playerApprentice.id].species[speciesTableId];
        GetLatestLearnedMoves(
            (*apprenticeMons[i]).species,
            (*apprenticeMons[i]).moves.as_mut_ptr(),
        );
        i += 1;
    }
    for i in 0..numQuestions {
        let questionId: u8 = (*gSaveBlock2Ptr).playerApprentice.questions[i].questionId();
        let monId: u8 = (*gSaveBlock2Ptr).playerApprentice.questions[i].monId();
        if questionId == QUESTION_ID_WHAT_ITEM {
            if (*gSaveBlock2Ptr).playerApprentice.questions[i].suggestedChange() != 0 {
                (*apprenticeMons[monId]).item =
                    (*gSaveBlock2Ptr).playerApprentice.questions[i].data;
            }
        } else if questionId == QUESTION_ID_WHICH_MOVE
            && (*gSaveBlock2Ptr).playerApprentice.questions[i].suggestedChange() != 0
        {
            let moveSlot: u32 = (*gSaveBlock2Ptr).playerApprentice.questions[i].moveSlot() as u32;
            (*apprenticeMons[monId]).moves[moveSlot] =
                (*gSaveBlock2Ptr).playerApprentice.questions[i].data;
        }
    }
}
unsafe fn CreateApprenticeMenu(menu: u8) {
    let mut i: u8 = 0;
    let mut strings: CArray<*mut u8, 3> = zeroed();
    let mut count: u8 = 2;
    let mut width: u8 = 0;
    let mut left: u8 = 0;
    let mut top: u8 = 0;
    match menu {
        APPRENTICE_ASK_WHICH_LEVEL => {
            left = 18;
            top = 8;
            strings[0] = (*(&raw const crate::data::strings::gText_Lv50).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
            strings[1] = (*(&raw const crate::data::strings::gText_OpenLevel)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        APPRENTICE_ASK_3SPECIES => {
            count = MULTI_PARTY_SIZE as u8;
            left = 18;
            top = 6;
            for i in 0..(MULTI_PARTY_SIZE as u8) {
                let mut species: u16 = 0;
                let speciesTableId: u32 = (if i < 3 {
                    shr_i32(
                        (*gSaveBlock2Ptr).playerApprentice.speciesIds[i] as i32,
                        (shr_i32((*gSaveBlock2Ptr).playerApprentice.party() as i32, i as u32)
                            as u32
                            & 1)
                            << 2,
                    ) & 0xF
                } else {
                    0
                }) as u32;
                species =
                    gApprentices[(*gSaveBlock2Ptr).playerApprentice.id].species[speciesTableId];
                strings[i] =
                    (*(&raw const crate::data::data_tables::gSpeciesNames)
                        .cast::<CArray<CArray<u8, 11>, 0>>())[species]
                        .as_ptr()
                        .cast_mut();
            }
        }
        APPRENTICE_ASK_2SPECIES => {
            left = 18;
            top = 8;
            if (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() >= NUM_WHICH_MON_QUESTIONS {
                return;
            }
            strings[1] = (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[(*gApprenticeQuestionData).altSpeciesId]
                .as_ptr()
                .cast_mut();
            strings[0] = (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[(*gApprenticeQuestionData).speciesId]
                .as_ptr()
                .cast_mut();
        }
        APPRENTICE_ASK_MOVES => {
            left = 17;
            top = 8;
            strings[0] = (*(&raw const crate::data::data_tables::gMoveNames)
                .cast::<CArray<CArray<u8, 13>, 355>>())[(*gApprenticeQuestionData).move1]
                .as_ptr()
                .cast_mut();
            strings[1] = (*(&raw const crate::data::data_tables::gMoveNames)
                .cast::<CArray<CArray<u8, 13>, 355>>())[(*gApprenticeQuestionData).move2]
                .as_ptr()
                .cast_mut();
        }
        APPRENTICE_ASK_GIVE => {
            left = 18;
            top = 8;
            strings[0] = (*(&raw const crate::data::strings::gText_Give).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
            strings[1] = (*(&raw const crate::data::strings::gText_NoNeed).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
        APPRENTICE_ASK_YES_NO => {
            left = 20;
            top = 8;
            strings[0] = (*(&raw const crate::data::strings::gText_Yes).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
            strings[1] = (*(&raw const crate::data::strings::gText_No).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
        _ => {
            return;
        }
    }
    let mut pixelWidth: i32 = 0;
    i = 0;
    while i < count {
        let width: i32 = GetStringWidth(FONT_NORMAL, strings[i], 0);
        if width > pixelWidth {
            pixelWidth = width;
        }
        i += 1;
    }
    width = ConvertPixelWidthToTileWidth(pixelWidth) as u8;
    left = ScriptMenu_AdjustLeftCoordFromWidth(left as i32, width as i32) as u8;
    let windowId: u8 = CreateAndShowWindow(left, top, width, count * 2);
    SetStandardWindowBorderStyle(windowId, FALSE);
    for i in 0..count {
        AddTextPrinterParameterized(
            windowId,
            FONT_NORMAL,
            strings[i],
            8,
            i * 16 + 1,
            TEXT_SKIP_DRAW,
            None,
        );
    }
    InitMenuInUpperLeftCornerNormal(windowId, count, 0);
    CreateChooseAnswerTask(TRUE, count, windowId);
}
pub(crate) unsafe fn Task_ChooseAnswer(taskId: u8) {
    let mut input: i8 = 0;
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if *data.at(5) == 0 {
        input = Menu_ProcessInputNoWrap();
    } else {
        input = Menu_ProcessInput();
    }
    match input {
        MENU_NOTHING_CHOSEN => {
            return;
        }
        MENU_B_PRESSED => {
            if *data.at(4) != 0 {
                return;
            }
            PlaySE(SE_SELECT);
            gSpecialVar_Result = MULTI_B_PRESSED;
        }
        _ => {
            gSpecialVar_Result = input as u16;
        }
    }
    RemoveAndHideWindow(*data.at(6) as u8);
    DestroyTask(taskId);
    ScriptContext_Enable();
}
unsafe fn CreateAndShowWindow(left: u8, top: u8, width: u8, height: u8) -> u8 {
    let mut winTemplate: WindowTemplate =
        CreateWindowTemplate(0, left + 1, top + 1, width, height, 15, 100);
    let windowId: u8 = AddWindow(&raw mut winTemplate) as u8;
    PutWindowTilemap(windowId);
    CopyWindowToVram(windowId, COPYWIN_FULL);
    windowId
}
unsafe fn RemoveAndHideWindow(windowId: u8) {
    ClearStdWindowAndFrameToTransparent(windowId, TRUE);
    RemoveWindow(windowId);
}
unsafe fn CreateChooseAnswerTask(noBButton: u8, answers: u8, windowId: u8) {
    let taskId: u8 = CreateTask(Some(Task_ChooseAnswer), 80);
    task_set(taskId, tNoBButton, noBButton as i16);
    if answers > 3 {
        task_set(taskId, tWrapAround, TRUE as i16);
    } else {
        task_set(taskId, tWrapAround, FALSE as i16);
    }
    task_set(taskId, tWindowId, windowId as i16);
}
#[unsafe(no_mangle)]
pub unsafe fn CallApprenticeFunction() {
    sApprenticeFunctions[*(&raw const crate::ffi::gSpecialVar_0x8004)
        .cast::<u16>()
        .cast_mut()]
    .unwrap_unchecked()();
}
pub(crate) unsafe fn Script_ResetPlayerApprentice() {
    SetApprenticeId();
    (*gSaveBlock2Ptr).playerApprentice.set_lvlMode(0);
    (*gSaveBlock2Ptr).playerApprentice.set_questionsAnswered(0);
    (*gSaveBlock2Ptr).playerApprentice.set_leadMonId(0);
    (*gSaveBlock2Ptr).playerApprentice.set_party(0);
    let mut i: u8 = 0;
    while i < MULTI_PARTY_SIZE as u8 {
        (*gSaveBlock2Ptr).playerApprentice.speciesIds[i] = 0;
        i += 1;
    }
    for i in 0..APPRENTICE_MAX_QUESTIONS {
        (*gSaveBlock2Ptr).playerApprentice.questions[i].set_questionId(0);
        (*gSaveBlock2Ptr).playerApprentice.questions[i].set_monId(0);
        (*gSaveBlock2Ptr).playerApprentice.questions[i].set_moveSlot(0);
        (*gSaveBlock2Ptr).playerApprentice.questions[i].set_suggestedChange(0);
        (*gSaveBlock2Ptr).playerApprentice.questions[i].data = 0;
    }
}
pub(crate) unsafe fn Script_GivenApprenticeLvlMode() {
    if GivenApprenticeLvlMode() == 0 {
        gSpecialVar_Result = FALSE as u16;
    } else {
        gSpecialVar_Result = TRUE as u16;
    }
}
pub(crate) unsafe fn Script_SetApprenticeLvlMode() {
    SetPlayersApprenticeLvlMode(gSpecialVar_0x8005 as u8);
}
pub(crate) unsafe fn Script_SetApprenticeId() {
    SetApprenticeId();
}
pub(crate) unsafe fn Script_SetRandomQuestionData() {
    SetRandomQuestionData();
}
pub(crate) unsafe fn IncrementQuestionsAnswered() {
    (*gSaveBlock2Ptr)
        .playerApprentice
        .set_questionsAnswered((*gSaveBlock2Ptr).playerApprentice.questionsAnswered() + 1);
}
pub(crate) unsafe fn GetNumApprenticePartyMonsAssigned() {
    gSpecialVar_Result = (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as u16;
}
pub(crate) unsafe fn IsFinalQuestion() {
    let questionNum: i32 = (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3;
    if questionNum < 0 {
        gSpecialVar_Result = FALSE as u16;
    } else {
        if questionNum > 8 {
            gSpecialVar_Result = TRUE as u16;
        }
        if (*gSaveBlock2Ptr).playerApprentice.questions[questionNum].questionId()
            == QUESTION_ID_WIN_SPEECH
        {
            gSpecialVar_Result = TRUE as u16;
        } else {
            gSpecialVar_Result = FALSE as u16;
        }
    }
}
pub(crate) unsafe fn Script_CreateApprenticeMenu() {
    CreateApprenticeMenu(gSpecialVar_0x8005 as u8);
}
pub(crate) unsafe fn Task_WaitForPrintingMessage(taskId: u8) {
    if RunTextPrintersAndIsPrinter0Active() == 0 {
        DestroyTask(taskId);
        if gSpecialVar_0x8005 != 0 {
            ExecuteFuncAfterButtonPress(Some(ScriptContext_Enable));
        } else {
            ScriptContext_Enable();
        }
    }
}
unsafe fn PrintApprenticeMessage() {
    let mut string: *mut u8 = null_mut();
    if gSpecialVar_0x8006 == APPRENTICE_MSG_WHICH_MON {
        string = sApprenticeWhichMonTexts[(*gSaveBlock2Ptr).playerApprentice.id][0];
    } else if gSpecialVar_0x8006 == APPRENTICE_MSG_THANKS_MON {
        string = sApprenticeWhichMonTexts[(*gSaveBlock2Ptr).playerApprentice.id][1];
    } else if gSpecialVar_0x8006 == APPRENTICE_MSG_WHICH_MOVE {
        string = sApprenticeWhichMoveTexts[(*gSaveBlock2Ptr).playerApprentice.id][0];
    } else if gSpecialVar_0x8006 == APPRENTICE_MSG_THANKS_MOVE {
        string = sApprenticeWhichMoveTexts[(*gSaveBlock2Ptr).playerApprentice.id][1];
    } else if gSpecialVar_0x8006 == APPRENTICE_MSG_WHICH_MON_FIRST {
        string = sApprenticeWhichMonFirstTexts[(*gSaveBlock2Ptr).playerApprentice.id][0];
    } else if gSpecialVar_0x8006 == APPRENTICE_MSG_THANKS_MON_FIRST {
        string = sApprenticeWhichMonFirstTexts[(*gSaveBlock2Ptr).playerApprentice.id][1];
    } else if gSpecialVar_0x8006 == APPRENTICE_MSG_WHAT_HELD_ITEM {
        string = sApprenticeHeldItemTexts[(*gSaveBlock2Ptr).playerApprentice.id][0];
    } else if gSpecialVar_0x8006 == APPRENTICE_MSG_PICK_WIN_SPEECH {
        string = sApprenticePickWinSpeechTexts[(*gSaveBlock2Ptr).playerApprentice.id][0];
    } else if gSpecialVar_0x8006 == APPRENTICE_MSG_THANKS_HELD_ITEM {
        string = sApprenticeHeldItemTexts[(*gSaveBlock2Ptr).playerApprentice.id][3];
    } else if gSpecialVar_0x8006 == APPRENTICE_MSG_HOLD_NOTHING {
        string = sApprenticeHeldItemTexts[(*gSaveBlock2Ptr).playerApprentice.id][1];
    } else if gSpecialVar_0x8006 == APPRENTICE_MSG_ITEM_ALREADY_SUGGESTED {
        string = sApprenticeHeldItemTexts[(*gSaveBlock2Ptr).playerApprentice.id][4];
    } else if gSpecialVar_0x8006 == APPRENTICE_MSG_THANKS_NO_HELD_ITEM {
        string = sApprenticeHeldItemTexts[(*gSaveBlock2Ptr).playerApprentice.id][2];
    } else if gSpecialVar_0x8006 == APPRENTICE_MSG_THANKS_WIN_SPEECH {
        string = sApprenticePickWinSpeechTexts[(*gSaveBlock2Ptr).playerApprentice.id][1];
    } else if gSpecialVar_0x8006 == APPRENTICE_MSG_PLEASE_TEACH {
        string = sApprenticeFirstMeetingTexts[(*gSaveBlock2Ptr).playerApprentice.id][0];
    } else if gSpecialVar_0x8006 == APPRENTICE_MSG_REJECT {
        string = sApprenticeFirstMeetingTexts[(*gSaveBlock2Ptr).playerApprentice.id][1];
    } else if gSpecialVar_0x8006 == APPRENTICE_MSG_WHICH_LVL_MODE {
        string = sApprenticeFirstMeetingTexts[(*gSaveBlock2Ptr).playerApprentice.id][2];
    } else if gSpecialVar_0x8006 == APPRENTICE_MSG_THANKS_LVL_MODE {
        string = sApprenticeFirstMeetingTexts[(*gSaveBlock2Ptr).playerApprentice.id][3];
    } else {
        ScriptContext_Enable();
        return;
    }
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), string);
    AddTextPrinterForMessage(TRUE);
    CreateTask(Some(Task_WaitForPrintingMessage), 1);
}
pub(crate) unsafe fn Script_PrintApprenticeMessage() {
    LockPlayerFieldControls();
    FreezeObjectEvents();
    PlayerFreeze();
    StopPlayerAvatar();
    DrawDialogueFrame(0, TRUE);
    PrintApprenticeMessage();
}
pub(crate) unsafe fn ApprenticeGetQuestion() {
    if (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() < NUM_WHICH_MON_QUESTIONS {
        gSpecialVar_Result = APPRENTICE_QUESTION_WHICH_MON;
    } else if (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() > 11 {
        gSpecialVar_Result = APPRENTICE_QUESTION_WIN_SPEECH;
    } else {
        let id: i32 = (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3;
        match (*gSaveBlock2Ptr).playerApprentice.questions[id].questionId() {
            QUESTION_ID_WHAT_ITEM => {
                gSpecialVar_Result = APPRENTICE_QUESTION_WHAT_ITEM;
            }
            QUESTION_ID_WHICH_MOVE => {
                gSpecialVar_Result = APPRENTICE_QUESTION_WHICH_MOVE;
            }
            QUESTION_ID_WHICH_FIRST => {
                gSpecialVar_Result = APPRENTICE_QUESTION_WHICH_FIRST;
            }
            _ => {
                gSpecialVar_Result = APPRENTICE_QUESTION_WIN_SPEECH;
            }
        }
    }
}
pub(crate) unsafe fn SetApprenticePartyMon() {
    if gSpecialVar_0x8005 != 0 {
        let partySlot: u8 = gSpecialVar_0x8006 as u8;
        (*gSaveBlock2Ptr).playerApprentice.set_party(
            (*gSaveBlock2Ptr).playerApprentice.party() | shl_i32(1, partySlot as u32) as u8,
        );
    }
}
pub(crate) unsafe fn SetApprenticeMonMove() {
    if (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() >= NUM_WHICH_MON_QUESTIONS {
        let id: u8 = (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() - 3;
        if gSpecialVar_0x8005 != 0 {
            (*gSaveBlock2Ptr).playerApprentice.questions[id].set_suggestedChange(TRUE);
        } else {
            (*gSaveBlock2Ptr).playerApprentice.questions[id].set_suggestedChange(FALSE);
        }
    }
}
pub(crate) unsafe fn InitQuestionData() {
    let mut count: u8 = 0;
    let mut id1: u8 = 0;
    let mut id2: u8 = 0;
    let mut i: u8 = 0;
    while i < APPRENTICE_MAX_QUESTIONS
        && (*gSaveBlock2Ptr).playerApprentice.questions[i].questionId() != 0
    {
        count += 1;
        i += 1;
    }
    gApprenticeQuestionData = AllocZeroed(8) as *mut ApprenticeQuestionData;
    if gSpecialVar_0x8005 == APPRENTICE_QUESTION_WHICH_MON {
        if (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() < NUM_WHICH_MON_QUESTIONS {
            id1 = (*gSaveBlock2Ptr).playerApprentice.speciesIds
                [(*gSaveBlock2Ptr).playerApprentice.questionsAnswered()]
                >> 4;
            (*gApprenticeQuestionData).altSpeciesId =
                gApprentices[(*gSaveBlock2Ptr).playerApprentice.id].species[id1];
            id2 = (*gSaveBlock2Ptr).playerApprentice.speciesIds
                [(*gSaveBlock2Ptr).playerApprentice.questionsAnswered()]
                & 0xF;
            (*gApprenticeQuestionData).speciesId =
                gApprentices[(*gSaveBlock2Ptr).playerApprentice.id].species[id2];
        }
    } else if gSpecialVar_0x8005 == APPRENTICE_QUESTION_WHICH_MOVE {
        if (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() >= NUM_WHICH_MON_QUESTIONS
            && ((*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32)
                < count as i32 + NUM_WHICH_MON_QUESTIONS as i32
            && (*gSaveBlock2Ptr).playerApprentice.questions
                [(*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3]
                .questionId()
                == QUESTION_ID_WHICH_MOVE
        {
            count = (*gSaveBlock2Ptr).playerApprentice.questions
                [(*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3]
                .monId();
            id1 = shr_i32(
                (*gSaveBlock2Ptr).playerApprentice.party() as i32,
                count as u32,
            ) as u8
                & 1;
            id1 = shr_i32(
                (*gSaveBlock2Ptr).playerApprentice.speciesIds[count] as i32,
                (id1 as u32) << 2,
            ) as u8
                & 0xF;
            (*gApprenticeQuestionData).speciesId =
                gApprentices[(*gSaveBlock2Ptr).playerApprentice.id].species[id1];
            (*gApprenticeQuestionData).move1 = GetDefaultMove(
                count,
                id1,
                (*gSaveBlock2Ptr).playerApprentice.questions
                    [(*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3]
                    .moveSlot(),
            );
            (*gApprenticeQuestionData).move2 = (*gSaveBlock2Ptr).playerApprentice.questions
                [(*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3]
                .data;
        }
    } else if gSpecialVar_0x8005 == APPRENTICE_QUESTION_WHAT_ITEM
        && (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() >= NUM_WHICH_MON_QUESTIONS
        && ((*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32)
            < count as i32 + NUM_WHICH_MON_QUESTIONS as i32
        && (*gSaveBlock2Ptr).playerApprentice.questions
            [(*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3]
            .questionId()
            == QUESTION_ID_WHAT_ITEM
    {
        count = (*gSaveBlock2Ptr).playerApprentice.questions
            [(*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3]
            .monId();
        id2 = shr_i32(
            (*gSaveBlock2Ptr).playerApprentice.party() as i32,
            count as u32,
        ) as u8
            & 1;
        id2 = shr_i32(
            (*gSaveBlock2Ptr).playerApprentice.speciesIds[count] as i32,
            (id2 as u32) << 2,
        ) as u8
            & 0xF;
        (*gApprenticeQuestionData).speciesId =
            gApprentices[(*gSaveBlock2Ptr).playerApprentice.id].species[id2];
    }
}
pub(crate) unsafe fn FreeQuestionData() {
    Free(gApprenticeQuestionData as *mut c_void);
    gApprenticeQuestionData = null_mut();
}
pub(crate) unsafe fn ApprenticeBufferString() {
    let mut stringDst: *mut u8 = null_mut();
    let mut text: CArray<u8, 16> = zeroed();
    let mut speciesArrayId: u32 = 0;
    match *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut()
    {
        0 => {
            stringDst = gStringVar1.as_mut_ptr();
        }
        1 => {
            stringDst = gStringVar2.as_mut_ptr();
        }
        2 => {
            stringDst = gStringVar3.as_mut_ptr();
        }
        _ => {
            return;
        }
    }
    match *(&raw const crate::ffi::gSpecialVar_0x8006)
        .cast::<u16>()
        .cast_mut()
    {
        APPRENTICE_BUFF_SPECIES1 => {
            StringCopy(
                stringDst,
                (*(&raw const crate::data::data_tables::gSpeciesNames)
                    .cast::<CArray<CArray<u8, 11>, 0>>())[(*gApprenticeQuestionData).speciesId]
                    .as_ptr()
                    .cast_mut(),
            );
        }
        APPRENTICE_BUFF_SPECIES2 => {
            StringCopy(
                stringDst,
                (*(&raw const crate::data::data_tables::gSpeciesNames)
                    .cast::<CArray<CArray<u8, 11>, 0>>())[(*gApprenticeQuestionData).altSpeciesId]
                    .as_ptr()
                    .cast_mut(),
            );
        }
        APPRENTICE_BUFF_SPECIES3 => {
            StringCopy(
                stringDst,
                (*(&raw const crate::data::data_tables::gSpeciesNames)
                    .cast::<CArray<CArray<u8, 11>, 0>>())[(*gApprenticeQuestionData).speciesId]
                    .as_ptr()
                    .cast_mut(),
            );
        }
        APPRENTICE_BUFF_MOVE1 => {
            StringCopy(
                stringDst,
                (*(&raw const crate::data::data_tables::gMoveNames)
                    .cast::<CArray<CArray<u8, 13>, 355>>())[(*gApprenticeQuestionData).move1]
                    .as_ptr()
                    .cast_mut(),
            );
        }
        APPRENTICE_BUFF_MOVE2 => {
            StringCopy(
                stringDst,
                (*(&raw const crate::data::data_tables::gMoveNames)
                    .cast::<CArray<CArray<u8, 13>, 355>>())[(*gApprenticeQuestionData).move2]
                    .as_ptr()
                    .cast_mut(),
            );
        }
        APPRENTICE_BUFF_ITEM => {
            StringCopy(
                stringDst,
                GetItemName(
                    (*gSaveBlock2Ptr).playerApprentice.questions
                        [(*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3]
                        .data,
                ),
            );
        }
        APPRENTICE_BUFF_NAME => {
            TVShowConvertInternationalString(
                text.as_mut_ptr(),
                GetApprenticeNameInLanguage(
                    (*gSaveBlock2Ptr).playerApprentice.id as u32,
                    GAME_LANGUAGE as i32,
                ),
                GAME_LANGUAGE as i32,
            );
            StringCopy(stringDst, text.as_mut_ptr());
        }
        APPRENTICE_BUFF_LEVEL => {
            if (*gSaveBlock2Ptr).playerApprentice.lvlMode() == APPRENTICE_LVL_MODE_50 {
                StringCopy(
                    stringDst,
                    (*(&raw const crate::data::strings::gText_Lv50).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
            } else {
                StringCopy(
                    stringDst,
                    (*(&raw const crate::data::strings::gText_OpenLevel).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
            }
        }
        APPRENTICE_BUFF_WIN_SPEECH => {
            FrontierSpeechToString((*gSaveBlock2Ptr).apprentices[0].speechWon.as_mut_ptr());
            StringCopy(stringDst, gStringVar4.as_mut_ptr());
        }
        APPRENTICE_BUFF_LEAD_MON_SPECIES => {
            speciesArrayId = (if (*gSaveBlock2Ptr).playerApprentice.leadMonId() < 3 {
                shr_i32(
                    (*gSaveBlock2Ptr).playerApprentice.speciesIds
                        [(*gSaveBlock2Ptr).playerApprentice.leadMonId()] as i32,
                    (shr_i32(
                        (*gSaveBlock2Ptr).playerApprentice.party() as i32,
                        (*gSaveBlock2Ptr).playerApprentice.leadMonId() as u32,
                    ) as u32
                        & 1)
                        << 2,
                ) & 0xF
            } else {
                0
            }) as u32;
            StringCopy(
                stringDst,
                (*(&raw const crate::data::data_tables::gSpeciesNames)
                    .cast::<CArray<CArray<u8, 11>, 0>>())
                    [gApprentices[(*gSaveBlock2Ptr).playerApprentice.id].species[speciesArrayId]]
                    .as_ptr()
                    .cast_mut(),
            );
        }
        _ => {}
    }
}
pub(crate) unsafe fn SetLeadApprenticeMon() {
    (*gSaveBlock2Ptr)
        .playerApprentice
        .set_leadMonId(gSpecialVar_0x8005 as u8);
}
pub(crate) unsafe fn Script_ApprenticeOpenBagMenu() {
    ApprenticeOpenBagMenu();
}
pub(crate) unsafe fn TrySetApprenticeHeldItem() {
    if (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() < NUM_WHICH_MON_QUESTIONS {
        return;
    }
    let mut count: u8 = 0;
    for j in 0..APPRENTICE_MAX_QUESTIONS {
        if (*gSaveBlock2Ptr).playerApprentice.questions[j].questionId() == QUESTION_ID_WIN_SPEECH {
            break;
        }
        count += 1;
    }
    'l3: for i in 0..count {
        'l2: {
            if i as i32 >= (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3 {
                break 'l3;
            }
            if (*gSaveBlock2Ptr).playerApprentice.questions[i].questionId() != QUESTION_ID_WHAT_ITEM
                || (*gSaveBlock2Ptr).playerApprentice.questions[i].suggestedChange() == 0
            {
                break 'l2;
            }
            if (*gSaveBlock2Ptr).playerApprentice.questions[i].data == gSpecialVar_0x8005 {
                (*gSaveBlock2Ptr).playerApprentice.questions
                    [(*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3]
                    .set_suggestedChange(FALSE);
                (*gSaveBlock2Ptr).playerApprentice.questions
                    [(*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3]
                    .data = *(&raw const crate::ffi::gSpecialVar_0x8005)
                    .cast::<u16>()
                    .cast_mut();
                gSpecialVar_Result = FALSE as u16;
                return;
            }
        }
    }
    (*gSaveBlock2Ptr).playerApprentice.questions
        [(*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3]
        .set_suggestedChange(TRUE);
    (*gSaveBlock2Ptr).playerApprentice.questions
        [(*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3]
        .data = *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut();
    gSpecialVar_Result = TRUE as u16;
}
pub(crate) unsafe fn ShiftSavedApprentices() {
    if (*gSaveBlock2Ptr).apprentices[0].playerName[0] == EOS {
        return;
    }
    let mut i: i32 = 0;
    while i < 3 {
        if (*gSaveBlock2Ptr).apprentices[i + 1].playerName[0] == EOS {
            (*gSaveBlock2Ptr).apprentices[i + 1] = (*gSaveBlock2Ptr).apprentices[0];
            return;
        }
        i += 1;
    }
    let mut apprenticeNum: i32 = 0xFFFF;
    let mut apprenticeIdx: i32 = -1;
    for i in 1..APPRENTICE_COUNT {
        if GetTrainerId((*gSaveBlock2Ptr).apprentices[i].playerId.as_mut_ptr())
            == GetTrainerId((*gSaveBlock2Ptr).playerTrainerId.as_mut_ptr())
            && ((*gSaveBlock2Ptr).apprentices[i].number as i32) < apprenticeNum
        {
            apprenticeNum = (*gSaveBlock2Ptr).apprentices[i].number as i32;
            apprenticeIdx = i;
        }
    }
    if apprenticeIdx > 0 {
        (*gSaveBlock2Ptr).apprentices[apprenticeIdx] = (*gSaveBlock2Ptr).apprentices[0];
    }
}
pub(crate) unsafe fn SaveApprentice() {
    (*gSaveBlock2Ptr).apprentices[0].set_id((*gSaveBlock2Ptr).playerApprentice.id);
    (*gSaveBlock2Ptr).apprentices[0].set_lvlMode((*gSaveBlock2Ptr).playerApprentice.lvlMode());
    let mut i: u8 = 0;
    while i < APPRENTICE_MAX_QUESTIONS
        && (*gSaveBlock2Ptr).playerApprentice.questions[i].questionId() != 0
    {
        i += 1;
    }
    (*gSaveBlock2Ptr).apprentices[0].numQuestions = i;
    (*gSaveBlock2Ptr).apprentices[0].number =
        (*gSaveBlock2Ptr).apprentices[0].number.saturating_add(1);
    SaveApprenticeParty((*gSaveBlock2Ptr).apprentices[0].numQuestions);
    for i in 0..TRAINER_ID_LENGTH {
        (*gSaveBlock2Ptr).apprentices[0].playerId[i] = (*gSaveBlock2Ptr).playerTrainerId[i];
    }
    StringCopy(
        (*gSaveBlock2Ptr).apprentices[0].playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
    );
    (*gSaveBlock2Ptr).apprentices[0].language = gGameLanguage;
    CalcApprenticeChecksum(&raw mut (*gSaveBlock2Ptr).apprentices[0]);
}
pub(crate) unsafe fn SetSavedApprenticeTrainerGfxId() {
    let mut objectEventGfxId: u8 = 0;
    let class: u8 = gApprentices[(*gSaveBlock2Ptr).apprentices[0].id()].facilityClass;
    let mut i: u8 = 0;
    while i < 30
        && (*(&raw const crate::data::battle_tower::gTowerMaleFacilityClasses)
            .cast::<CArray<u8, 30>>())[i]
            != class
    {
        i += 1;
    }
    if i != 30 {
        objectEventGfxId = (*(&raw const crate::data::battle_tower::gTowerMaleTrainerGfxIds)
            .cast::<CArray<u8, 30>>())[i];
        VarSet(VAR_OBJ_GFX_ID_0, objectEventGfxId as u16);
        return;
    }
    i = 0;
    while i < 20
        && (*(&raw const crate::data::battle_tower::gTowerFemaleFacilityClasses)
            .cast::<CArray<u8, 20>>())[i]
            != class
    {
        i += 1;
    }
    if i != 20 {
        objectEventGfxId = (*(&raw const crate::data::battle_tower::gTowerFemaleTrainerGfxIds)
            .cast::<CArray<u8, 20>>())[i];
        VarSet(VAR_OBJ_GFX_ID_0, objectEventGfxId as u16);
    }
}
pub(crate) unsafe fn SetPlayerApprenticeTrainerGfxId() {
    let mut objectEventGfxId: u8 = 0;
    let class: u8 = gApprentices[(*gSaveBlock2Ptr).playerApprentice.id].facilityClass;
    let mut i: u8 = 0;
    while i < 30
        && (*(&raw const crate::data::battle_tower::gTowerMaleFacilityClasses)
            .cast::<CArray<u8, 30>>())[i]
            != class
    {
        i += 1;
    }
    if i != 30 {
        objectEventGfxId = (*(&raw const crate::data::battle_tower::gTowerMaleTrainerGfxIds)
            .cast::<CArray<u8, 30>>())[i];
        VarSet(VAR_OBJ_GFX_ID_0, objectEventGfxId as u16);
        return;
    }
    i = 0;
    while i < 20
        && (*(&raw const crate::data::battle_tower::gTowerFemaleFacilityClasses)
            .cast::<CArray<u8, 20>>())[i]
            != class
    {
        i += 1;
    }
    if i != 20 {
        objectEventGfxId = (*(&raw const crate::data::battle_tower::gTowerFemaleTrainerGfxIds)
            .cast::<CArray<u8, 20>>())[i];
        VarSet(VAR_OBJ_GFX_ID_0, objectEventGfxId as u16);
    }
}
pub(crate) unsafe fn GetShouldCheckApprenticeGone() {
    gSpecialVar_0x8004 = TRUE as u16;
}
pub(crate) unsafe fn GetShouldApprenticeLeave() {
    gSpecialVar_0x8004 = TRUE as u16;
}
pub unsafe fn GetApprenticeNameInLanguage(apprenticeId: u32, language: i32) -> *mut u8 {
    let apprentice: *mut ApprenticeTrainer = (&raw const gApprentices[apprenticeId]).cast_mut();
    match language {
        1 => {
            return (*apprentice).name[0].as_mut_ptr();
        }
        LANGUAGE_ENGLISH => {
            return (*apprentice).name[1].as_mut_ptr();
        }
        LANGUAGE_FRENCH => {
            return (*apprentice).name[2].as_mut_ptr();
        }
        LANGUAGE_ITALIAN => {
            return (*apprentice).name[3].as_mut_ptr();
        }
        LANGUAGE_GERMAN => {
            return (*apprentice).name[4].as_mut_ptr();
        }
        _ => {
            return (*apprentice).name[5].as_mut_ptr();
        }
    }
    #[allow(unreachable_code)]
    {
        null_mut()
    }
}
pub(crate) unsafe fn Task_SwitchToFollowupFuncAfterButtonPress(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
        SwitchTaskToFollowupFunc(taskId);
    }
}
pub(crate) unsafe fn Task_ExecuteFuncAfterButtonPress(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
        gApprenticeFunc = core::mem::transmute::<_, Option<unsafe fn()>>(
            (task_get(taskId, 0) as u16 as u32 | (task_get(taskId, 1) as u32) << 16) as usize
                as *mut c_void,
        );
        gApprenticeFunc.unwrap_unchecked()();
        DestroyTask(taskId);
    }
}
unsafe fn ExecuteFuncAfterButtonPress(func: Option<unsafe fn()>) {
    let taskId: u8 = CreateTask(Some(Task_ExecuteFuncAfterButtonPress), 1);
    task_set(
        taskId,
        0,
        core::mem::transmute::<_, usize>(func) as u32 as i16,
    );
    task_set(
        taskId,
        1,
        (core::mem::transmute::<_, usize>(func) as u32 >> 16) as i16,
    );
}
unsafe fn ExecuteFollowupFuncAfterButtonPress(task: Option<unsafe fn(u8)>) {
    let taskId: u8 = CreateTask(Some(Task_SwitchToFollowupFuncAfterButtonPress), 1);
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_SwitchToFollowupFuncAfterButtonPress),
        task,
    );
}
