//! Translated from `src/lilycove_lady.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gGameLanguage;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::easy_chat::{CopyEasyChatWord, IsEasyChatAnswerUnlocked, ShowEasyChatScreen};
use crate::event_data::VarSet;
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_0x8005, gSpecialVar_Result};
use crate::item::RemoveBagItem;
use crate::item_menu::{FavorLadyOpenBagMenu, QuizLadyOpenBagMenu, gSpecialVar_ItemId};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::overworld::CB2_ReturnToField;
use crate::pokeblock::OpenPokeblockCase;
use crate::random::Random;
use crate::script::ScriptContext_Enable;
use crate::string_util::ConvertInternationalString;
use crate::string_util::{StringCompare, StringCopy, StringCopy_Nickname, StringCopy_PlayerName};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): sContestLadyMonGfxId sLilycoveLadyGfxId sQuizLadyQuestion1 sQuizLadyQuestion2 sQuizLadyQuestion3 sQuizLadyQuestion4 sQuizLadyQuestion5 sQuizLadyQuestion6 sQuizLadyQuestion7 sQuizLadyQuestion8 sQuizLadyQuestion9 sQuizLadyQuestion10 sQuizLadyQuestion11 sQuizLadyQuestion12 sQuizLadyQuestion13 sQuizLadyQuestion14 sQuizLadyQuestion15 sQuizLadyQuestion16 sQuizLadyQuizQuestions sQuizLadyQuizAnswers sQuizLadyPrizes sFavorLadyRequests sFavorLadyAcceptedItems_Slippery sFavorLadyAcceptedItems_Roundish sFavorLadyAcceptedItems_Whamish sFavorLadyAcceptedItems_Shiny sFavorLadyAcceptedItems_Sticky sFavorLadyAcceptedItems_Pointy sFavorLadyAcceptedItemLists sFavorLadyPrizes sContestLadyMonNames sContestLadyCategoryNames sContestNames sContestLadyMonSpecies

static sContestLadyCategoryNames: Table<CArray<*mut u8, 5>> =
    Table((&raw const crate::data::lilycove_lady::sContestLadyCategoryNames).cast());
static sContestLadyMonGfxId: Table<CArray<u16, 5>> =
    Table((&raw const crate::data::lilycove_lady::sContestLadyMonGfxId).cast());
static sContestLadyMonNames: Table<CArray<*mut u8, 5>> =
    Table((&raw const crate::data::lilycove_lady::sContestLadyMonNames).cast());
static sContestLadyMonSpecies: Table<CArray<u16, 5>> =
    Table((&raw const crate::data::lilycove_lady::sContestLadyMonSpecies).cast());
static sContestNames: Table<CArray<*mut u8, 5>> =
    Table((&raw const crate::data::lilycove_lady::sContestNames).cast());
static sFavorLadyAcceptedItemLists: Table<CArray<*mut u16, 6>> =
    Table((&raw const crate::data::lilycove_lady::sFavorLadyAcceptedItemLists).cast());
static sFavorLadyPrizes: Table<CArray<u16, 6>> =
    Table((&raw const crate::data::lilycove_lady::sFavorLadyPrizes).cast());
static sFavorLadyRequests: Table<CArray<*mut u8, 6>> =
    Table((&raw const crate::data::lilycove_lady::sFavorLadyRequests).cast());
static sLilycoveLadyGfxId: Table<CArray<u16, 3>> =
    Table((&raw const crate::data::lilycove_lady::sLilycoveLadyGfxId).cast());
static sQuizLadyPrizes: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::lilycove_lady::sQuizLadyPrizes).cast());
static sQuizLadyQuizAnswers: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::lilycove_lady::sQuizLadyQuizAnswers).cast());
static sQuizLadyQuizQuestions: Table<CArray<*mut u16, 16>> =
    Table((&raw const crate::data::lilycove_lady::sQuizLadyQuizQuestions).cast());

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sFavorLadyPtr: crate::global::Global<*mut LilycoveLadyFavor> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sQuizLadyPtr: crate::global::Global<*mut LilycoveLadyQuiz> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sContestLadyPtr: crate::global::Global<*mut LilycoveLadyContest> =
    crate::global::Global::new(null_mut());

/// `GetItemName` with this module's view of its types.
#[inline]
unsafe fn GetItemName(a0: u16) -> *mut u8 {
    crate::item::GetItemName(a0) as *mut u8
}

pub unsafe fn GetLilycoveLadyId() -> u8 {
    (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.id
}
#[unsafe(no_mangle)]
pub unsafe fn SetLilycoveLadyGfx() {
    let mut lilycoveLady: *mut LilycoveLady = null_mut();
    VarSet(VAR_OBJ_GFX_ID_0, sLilycoveLadyGfxId[GetLilycoveLadyId()]);
    if GetLilycoveLadyId() == LILYCOVE_LADY_CONTEST {
        lilycoveLady = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady;
        VarSet(
            VAR_OBJ_GFX_ID_1,
            sContestLadyMonGfxId[(*lilycoveLady).contest.category],
        );
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn InitLilycoveLady() {
    let mut id: u16 = ((*crate::gl!(gSaveBlock2Ptr)).playerTrainerId[1] as u16) << 8
        | (*crate::gl!(gSaveBlock2Ptr)).playerTrainerId[0] as u16;
    id = (id as i32 % 6) as u16;
    id >>= 1;
    match id {
        0 => {
            InitLilycoveQuizLady();
        }
        1 => {
            InitLilycoveFavorLady();
        }
        2 => {
            InitLilycoveContestLady();
        }
        _ => {}
    }
}
pub unsafe fn ResetLilycoveLadyForRecordMix() {
    match GetLilycoveLadyId() {
        LILYCOVE_LADY_QUIZ => {
            ResetQuizLadyForRecordMix();
        }
        LILYCOVE_LADY_FAVOR => {
            ResetFavorLadyForRecordMix();
        }
        LILYCOVE_LADY_CONTEST => {
            ResetContestLadyForRecordMix();
        }
        _ => {}
    }
}
pub unsafe fn InitLilycoveLadyRandomly() {
    let lady: u8 = (Random() as i32 % 3) as u8;
    match lady {
        LILYCOVE_LADY_QUIZ => {
            InitLilycoveQuizLady();
        }
        LILYCOVE_LADY_FAVOR => {
            InitLilycoveFavorLady();
        }
        LILYCOVE_LADY_CONTEST => {
            InitLilycoveContestLady();
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn Script_GetLilycoveLadyId() {
    gSpecialVar_Result = GetLilycoveLadyId() as u16;
}
unsafe fn GetNumAcceptedItems(mut itemsArray: *mut u16) -> u8 {
    let mut numItems: u8 = 0;
    while *itemsArray != ITEM_NONE {
        numItems += 1;
        itemsArray = itemsArray.at(1);
    }
    numItems
}
unsafe fn FavorLadyPickFavorAndBestItem() {
    (*crate::gl!(sFavorLadyPtr)).favorId = (Random() % 6) as u8;
    let numItems: u8 =
        GetNumAcceptedItems(sFavorLadyAcceptedItemLists[(*crate::gl!(sFavorLadyPtr)).favorId]);
    let bestItem: u8 = rem_i32(Random() as i32, numItems as i32) as u8;
    (*crate::gl!(sFavorLadyPtr)).bestItem =
        *sFavorLadyAcceptedItemLists[(*crate::gl!(sFavorLadyPtr)).favorId].at(bestItem);
}
unsafe fn InitLilycoveFavorLady() {
    crate::gl!(sFavorLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.favor);
    (*crate::gl!(sFavorLadyPtr)).id = LILYCOVE_LADY_FAVOR;
    (*crate::gl!(sFavorLadyPtr)).state = LILYCOVE_LADY_STATE_READY;
    (*crate::gl!(sFavorLadyPtr)).playerName[0] = EOS;
    (*crate::gl!(sFavorLadyPtr)).likedItem = FALSE;
    (*crate::gl!(sFavorLadyPtr)).numItemsGiven = 0;
    (*crate::gl!(sFavorLadyPtr)).itemId = ITEM_NONE;
    (*crate::gl!(sFavorLadyPtr)).language = gGameLanguage;
    FavorLadyPickFavorAndBestItem();
}
unsafe fn ResetFavorLadyForRecordMix() {
    crate::gl!(sFavorLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.favor);
    (*crate::gl!(sFavorLadyPtr)).id = LILYCOVE_LADY_FAVOR;
    (*crate::gl!(sFavorLadyPtr)).state = LILYCOVE_LADY_STATE_READY;
}
#[unsafe(no_mangle)]
pub unsafe fn GetFavorLadyState() -> u8 {
    crate::gl!(sFavorLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.favor);
    if (*crate::gl!(sFavorLadyPtr)).state == LILYCOVE_LADY_STATE_PRIZE {
        return LILYCOVE_LADY_STATE_PRIZE;
    } else if (*crate::gl!(sFavorLadyPtr)).state == LILYCOVE_LADY_STATE_COMPLETED {
        return LILYCOVE_LADY_STATE_COMPLETED;
    } else {
        return LILYCOVE_LADY_STATE_READY;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetFavorLadyRequest(idx: u8) -> *mut u8 {
    sFavorLadyRequests[idx]
}
#[unsafe(no_mangle)]
pub unsafe fn BufferFavorLadyRequest() {
    crate::gl!(sFavorLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.favor);
    StringCopy(
        gStringVar1.as_mut_ptr(),
        GetFavorLadyRequest((*crate::gl!(sFavorLadyPtr)).favorId),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn HasAnotherPlayerGivenFavorLadyItem() -> u8 {
    crate::gl!(sFavorLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.favor);
    if (*crate::gl!(sFavorLadyPtr)).playerName[0] != EOS {
        StringCopy_PlayerName(
            (*gStringVar3.as_ptr()).as_mut_ptr(),
            (*crate::gl!(sFavorLadyPtr)).playerName.as_mut_ptr(),
        );
        ConvertInternationalString(
            (*gStringVar3.as_ptr()).as_mut_ptr(),
            (*crate::gl!(sFavorLadyPtr)).language,
        );
        return TRUE;
    }
    FALSE
}
unsafe fn BufferItemName(dest: *mut u8, itemId: u16) {
    StringCopy(dest, GetItemName(itemId));
}
#[unsafe(no_mangle)]
pub unsafe fn BufferFavorLadyItemName() {
    crate::gl!(sFavorLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.favor);
    BufferItemName(
        (*gStringVar2.as_ptr()).as_mut_ptr(),
        (*crate::gl!(sFavorLadyPtr)).itemId,
    );
}
unsafe fn SetFavorLadyPlayerName(src: *mut u8, dest: *mut u8) {
    memset(dest, EOS as i32, 8);
    StringCopy_PlayerName(dest, src);
}
#[unsafe(no_mangle)]
pub unsafe fn BufferFavorLadyPlayerName() {
    crate::gl!(sFavorLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.favor);
    SetFavorLadyPlayerName(
        (*crate::gl!(sFavorLadyPtr)).playerName.as_mut_ptr(),
        (*gStringVar3.as_ptr()).as_mut_ptr(),
    );
    ConvertInternationalString(
        (*gStringVar3.as_ptr()).as_mut_ptr(),
        (*crate::gl!(sFavorLadyPtr)).language,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn DidFavorLadyLikeItem() -> u8 {
    crate::gl!(sFavorLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.favor);
    (if (*crate::gl!(sFavorLadyPtr)).likedItem != 0 {
        TRUE as i32
    } else {
        FALSE as i32
    }) as u8
}
#[unsafe(no_mangle)]
pub unsafe fn Script_FavorLadyOpenBagMenu() {
    FavorLadyOpenBagMenu();
}
unsafe fn DoesFavorLadyLikeItem(itemId: u16) -> u8 {
    crate::gl!(sFavorLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.favor);
    let numItems: u8 =
        GetNumAcceptedItems(sFavorLadyAcceptedItemLists[(*crate::gl!(sFavorLadyPtr)).favorId]);
    (*crate::gl!(sFavorLadyPtr)).state = LILYCOVE_LADY_STATE_COMPLETED;
    BufferItemName((*gStringVar2.as_ptr()).as_mut_ptr(), itemId);
    (*crate::gl!(sFavorLadyPtr)).itemId = itemId;
    SetFavorLadyPlayerName(
        (*crate::gl!(gSaveBlock2Ptr)).playerName.as_mut_ptr(),
        (*crate::gl!(sFavorLadyPtr)).playerName.as_mut_ptr(),
    );
    (*crate::gl!(sFavorLadyPtr)).language = gGameLanguage;
    let mut likedItem: u8 = FALSE;
    for i in 0..numItems {
        if *sFavorLadyAcceptedItemLists[(*crate::gl!(sFavorLadyPtr)).favorId].at(i) == itemId {
            likedItem = TRUE;
            (*crate::gl!(sFavorLadyPtr)).numItemsGiven += 1;
            (*crate::gl!(sFavorLadyPtr)).likedItem = TRUE;
            if (*crate::gl!(sFavorLadyPtr)).bestItem == itemId {
                (*crate::gl!(sFavorLadyPtr)).numItemsGiven = LILYCOVE_LADY_GIFT_THRESHOLD;
            }
            break;
        }
        (*crate::gl!(sFavorLadyPtr)).likedItem = FALSE;
    }
    likedItem
}
#[unsafe(no_mangle)]
pub unsafe fn Script_DoesFavorLadyLikeItem() -> u8 {
    DoesFavorLadyLikeItem(gSpecialVar_ItemId.get())
}
#[unsafe(no_mangle)]
pub unsafe fn IsFavorLadyThresholdMet() -> u8 {
    let mut numItemsGiven: u8 = 0;
    crate::gl!(sFavorLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.favor);
    numItemsGiven = (*crate::gl!(sFavorLadyPtr)).numItemsGiven;
    (if numItemsGiven < LILYCOVE_LADY_GIFT_THRESHOLD {
        FALSE as i32
    } else {
        TRUE as i32
    }) as u8
}
unsafe fn FavorLadyBufferPrizeName(prize: u16) {
    BufferItemName((*gStringVar2.as_ptr()).as_mut_ptr(), prize);
}
#[unsafe(no_mangle)]
pub unsafe fn FavorLadyGetPrize() -> u16 {
    crate::gl!(sFavorLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.favor);
    let prize: u16 = sFavorLadyPrizes[(*crate::gl!(sFavorLadyPtr)).favorId];
    FavorLadyBufferPrizeName(prize);
    (*crate::gl!(sFavorLadyPtr)).state = LILYCOVE_LADY_STATE_PRIZE;
    prize
}
#[unsafe(no_mangle)]
pub unsafe fn SetFavorLadyState_Complete() {
    InitLilycoveFavorLady();
    (*crate::gl!(sFavorLadyPtr)).state = LILYCOVE_LADY_STATE_COMPLETED;
}
pub unsafe fn FieldCallback_FavorLadyEnableScriptContexts() {
    ScriptContext_Enable();
}
unsafe fn QuizLadyPickQuestion() {
    let questionId: u8 = (Random() % 16) as u8;
    for i in 0..QUIZ_QUESTION_LEN {
        (*crate::gl!(sQuizLadyPtr)).question[i] = *sQuizLadyQuizQuestions[questionId].at(i);
    }
    (*crate::gl!(sQuizLadyPtr)).correctAnswer = sQuizLadyQuizAnswers[questionId];
    (*crate::gl!(sQuizLadyPtr)).prize = sQuizLadyPrizes[questionId];
    (*crate::gl!(sQuizLadyPtr)).questionId = questionId;
    (*crate::gl!(sQuizLadyPtr)).playerName[0] = EOS;
}
unsafe fn InitLilycoveQuizLady() {
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    (*crate::gl!(sQuizLadyPtr)).id = LILYCOVE_LADY_QUIZ;
    (*crate::gl!(sQuizLadyPtr)).state = LILYCOVE_LADY_STATE_READY;
    let mut i: u8 = 0;
    while i < QUIZ_QUESTION_LEN {
        (*crate::gl!(sQuizLadyPtr)).question[i] = EC_EMPTY_WORD;
        i += 1;
    }
    (*crate::gl!(sQuizLadyPtr)).correctAnswer = EC_EMPTY_WORD;
    (*crate::gl!(sQuizLadyPtr)).playerAnswer = EC_EMPTY_WORD;
    for i in 0..TRAINER_ID_LENGTH {
        (*crate::gl!(sQuizLadyPtr)).playerTrainerId[i] = 0;
    }
    (*crate::gl!(sQuizLadyPtr)).prize = ITEM_NONE;
    (*crate::gl!(sQuizLadyPtr)).waitingForChallenger = FALSE;
    (*crate::gl!(sQuizLadyPtr)).prevQuestionId = 16;
    (*crate::gl!(sQuizLadyPtr)).language = gGameLanguage;
    QuizLadyPickQuestion();
}
unsafe fn ResetQuizLadyForRecordMix() {
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    (*crate::gl!(sQuizLadyPtr)).id = LILYCOVE_LADY_QUIZ;
    (*crate::gl!(sQuizLadyPtr)).state = LILYCOVE_LADY_STATE_READY;
    (*crate::gl!(sQuizLadyPtr)).waitingForChallenger = FALSE;
    (*crate::gl!(sQuizLadyPtr)).playerAnswer = EC_EMPTY_WORD;
}
#[unsafe(no_mangle)]
pub unsafe fn GetQuizLadyState() -> u8 {
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    if (*crate::gl!(sQuizLadyPtr)).state == LILYCOVE_LADY_STATE_PRIZE {
        return LILYCOVE_LADY_STATE_PRIZE;
    } else if (*crate::gl!(sQuizLadyPtr)).state == LILYCOVE_LADY_STATE_COMPLETED {
        return LILYCOVE_LADY_STATE_COMPLETED;
    } else {
        return LILYCOVE_LADY_STATE_READY;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetQuizAuthor() -> u8 {
    let mut i: i32 = 0;
    let quiz: *mut LilycoveLadyQuiz = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz;
    if IsEasyChatAnswerUnlocked((*quiz).correctAnswer as i32) == FALSE as u32 {
        i = (*quiz).questionId as i32;
        loop {
            if ({
                i += 1;
                i
            }) >= 16
            {
                i = 0;
            }
            if IsEasyChatAnswerUnlocked(sQuizLadyQuizAnswers[i] as i32) != FALSE as u32 {
                break;
            }
        }
        for j in 0..(QUIZ_QUESTION_LEN as i32) {
            (*quiz).question[j] = *sQuizLadyQuizQuestions[i].at(j);
        }
        (*quiz).correctAnswer = sQuizLadyQuizAnswers[i];
        (*quiz).prize = sQuizLadyPrizes[i];
        (*quiz).questionId = i as u8;
        (*quiz).playerName[0] = EOS;
    }
    let authorNameId: u8 = BufferQuizAuthorName();
    if authorNameId == QUIZ_AUTHOR_NAME_LADY {
        return QUIZ_AUTHOR_LADY;
    } else if authorNameId == QUIZ_AUTHOR_NAME_OTHER_PLAYER || IsQuizTrainerIdNotPlayer() != 0 {
        return QUIZ_AUTHOR_OTHER_PLAYER;
    } else {
        return QUIZ_AUTHOR_PLAYER;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn BufferQuizAuthorName() -> u8 {
    let mut nameLen: u8 = 0;
    let mut authorNameId: u8 = QUIZ_AUTHOR_NAME_PLAYER;
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    if (*crate::gl!(sQuizLadyPtr)).playerName[0] == EOS {
        StringCopy_PlayerName(
            gStringVar1.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_QuizLady_Lady).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        authorNameId = QUIZ_AUTHOR_NAME_LADY;
    } else {
        StringCopy_PlayerName(
            gStringVar1.as_mut_ptr(),
            (*crate::gl!(sQuizLadyPtr)).playerName.as_mut_ptr(),
        );
        ConvertInternationalString(
            gStringVar1.as_mut_ptr(),
            (*crate::gl!(sQuizLadyPtr)).language,
        );
        nameLen = GetPlayerNameLength((*crate::gl!(sQuizLadyPtr)).playerName.as_mut_ptr());
        if nameLen == GetPlayerNameLength((*crate::gl!(gSaveBlock2Ptr)).playerName.as_mut_ptr()) {
            let mut name: *mut u8 = (*crate::gl!(sQuizLadyPtr)).playerName.as_mut_ptr();
            for i in 0..nameLen {
                name = (*crate::gl!(sQuizLadyPtr)).playerName.as_mut_ptr();
                if *name.at(i) != (*crate::gl!(gSaveBlock2Ptr)).playerName[i] {
                    authorNameId = QUIZ_AUTHOR_NAME_OTHER_PLAYER;
                    break;
                }
            }
        }
    }
    authorNameId
}
unsafe fn IsQuizTrainerIdNotPlayer() -> u8 {
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    let mut notPlayer: u8 = FALSE;
    for i in 0..TRAINER_ID_LENGTH {
        if (*crate::gl!(sQuizLadyPtr)).playerTrainerId[i]
            != (*crate::gl!(gSaveBlock2Ptr)).playerTrainerId[i] as u16
        {
            notPlayer = TRUE;
            break;
        }
    }
    notPlayer
}
unsafe fn GetPlayerNameLength(playerName: *mut u8) -> u8 {
    let mut len: u8 = 0;
    let mut ptr: *mut u8 = playerName;
    while *ptr != EOS {
        len += 1;
        ptr = ptr.at(1);
    }
    len
}
#[unsafe(no_mangle)]
pub unsafe fn BufferQuizPrizeName() {
    StringCopy(
        gStringVar1.as_mut_ptr(),
        GetItemName((*crate::gl!(sQuizLadyPtr)).prize),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn BufferQuizAuthorNameAndCheckIfLady() -> u8 {
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    if BufferQuizAuthorName() == QUIZ_AUTHOR_NAME_LADY {
        (*crate::gl!(sQuizLadyPtr)).language = gGameLanguage;
        return TRUE;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn IsQuizLadyWaitingForChallenger() -> u8 {
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    (*crate::gl!(sQuizLadyPtr)).waitingForChallenger
}
#[unsafe(no_mangle)]
pub unsafe fn QuizLadyGetPlayerAnswer() {
    ShowEasyChatScreen();
}
#[unsafe(no_mangle)]
pub unsafe fn IsQuizAnswerCorrect() -> u8 {
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    CopyEasyChatWord(
        gStringVar1.as_mut_ptr(),
        (*crate::gl!(sQuizLadyPtr)).correctAnswer,
    );
    CopyEasyChatWord(
        (*gStringVar2.as_ptr()).as_mut_ptr(),
        (*crate::gl!(sQuizLadyPtr)).playerAnswer,
    );
    (if StringCompare(
        gStringVar1.as_mut_ptr(),
        (*gStringVar2.as_ptr()).as_mut_ptr(),
    ) != 0
    {
        FALSE as i32
    } else {
        TRUE as i32
    }) as u8
}
#[unsafe(no_mangle)]
pub unsafe fn BufferQuizPrizeItem() {
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    gSpecialVar_0x8005 = (*crate::gl!(sQuizLadyPtr)).prize;
}
#[unsafe(no_mangle)]
pub unsafe fn SetQuizLadyState_Complete() {
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    (*crate::gl!(sQuizLadyPtr)).state = LILYCOVE_LADY_STATE_COMPLETED;
}
#[unsafe(no_mangle)]
pub unsafe fn SetQuizLadyState_GivePrize() {
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    (*crate::gl!(sQuizLadyPtr)).state = LILYCOVE_LADY_STATE_PRIZE;
}
#[unsafe(no_mangle)]
pub unsafe fn ClearQuizLadyPlayerAnswer() {
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    (*crate::gl!(sQuizLadyPtr)).playerAnswer = EC_EMPTY_WORD;
}
#[unsafe(no_mangle)]
pub unsafe fn Script_QuizLadyOpenBagMenu() {
    QuizLadyOpenBagMenu();
}
#[unsafe(no_mangle)]
pub unsafe fn QuizLadyPickNewQuestion() {
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    if BufferQuizAuthorNameAndCheckIfLady() != 0 {
        (*crate::gl!(sQuizLadyPtr)).prevQuestionId = (*crate::gl!(sQuizLadyPtr)).questionId;
    } else {
        (*crate::gl!(sQuizLadyPtr)).prevQuestionId = 16;
    }
    QuizLadyPickQuestion();
}
#[unsafe(no_mangle)]
pub unsafe fn ClearQuizLadyQuestionAndAnswer() {
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    for i in 0..QUIZ_QUESTION_LEN {
        (*crate::gl!(sQuizLadyPtr)).question[i] = EC_EMPTY_WORD;
    }
    (*crate::gl!(sQuizLadyPtr)).correctAnswer = EC_EMPTY_WORD;
}
#[unsafe(no_mangle)]
pub unsafe fn QuizLadySetCustomQuestion() {
    gSpecialVar_0x8004 = EASY_CHAT_TYPE_QUIZ_SET_QUESTION as u16;
    ShowEasyChatScreen();
}
#[unsafe(no_mangle)]
pub fn QuizLadyTakePrizeForCustomQuiz() {
    RemoveBagItem(gSpecialVar_ItemId.get(), 1);
}
#[unsafe(no_mangle)]
pub unsafe fn QuizLadyRecordCustomQuizData() {
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    (*crate::gl!(sQuizLadyPtr)).prize = gSpecialVar_ItemId.get();
    for i in 0..TRAINER_ID_LENGTH {
        (*crate::gl!(sQuizLadyPtr)).playerTrainerId[i] =
            (*crate::gl!(gSaveBlock2Ptr)).playerTrainerId[i] as u16;
    }
    StringCopy_PlayerName(
        (*crate::gl!(sQuizLadyPtr)).playerName.as_mut_ptr(),
        (*crate::gl!(gSaveBlock2Ptr)).playerName.as_mut_ptr(),
    );
    (*crate::gl!(sQuizLadyPtr)).language = gGameLanguage;
}
#[unsafe(no_mangle)]
pub unsafe fn QuizLadySetWaitingForChallenger() {
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    (*crate::gl!(sQuizLadyPtr)).waitingForChallenger = TRUE;
}
#[unsafe(no_mangle)]
pub unsafe fn BufferQuizCorrectAnswer() {
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    CopyEasyChatWord(
        (*gStringVar3.as_ptr()).as_mut_ptr(),
        (*crate::gl!(sQuizLadyPtr)).correctAnswer,
    );
}
pub unsafe fn FieldCallback_QuizLadyEnableScriptContexts() {
    ScriptContext_Enable();
}
pub unsafe fn QuizLadyClearQuestionForRecordMix(lilycoveLady: *mut LilycoveLady) {
    crate::gl!(sQuizLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.quiz);
    if (*lilycoveLady).quiz.prevQuestionId < 16
        && (*crate::gl!(sQuizLadyPtr)).id == LILYCOVE_LADY_QUIZ
    {
        for i in 0..4u8 {
            if (*lilycoveLady).quiz.prevQuestionId != (*crate::gl!(sQuizLadyPtr)).questionId {
                break;
            }
            (*crate::gl!(sQuizLadyPtr)).questionId = (Random() % 16) as u8;
        }
        if (*lilycoveLady).quiz.prevQuestionId == (*crate::gl!(sQuizLadyPtr)).questionId {
            (*crate::gl!(sQuizLadyPtr)).questionId =
                (((*crate::gl!(sQuizLadyPtr)).questionId as i32 + 1) % 16) as u8;
        }
        (*crate::gl!(sQuizLadyPtr)).prevQuestionId = (*lilycoveLady).quiz.prevQuestionId;
    }
}
unsafe fn ResetContestLadyContestData() {
    (*crate::gl!(sContestLadyPtr)).playerName[0] = EOS;
    (*crate::gl!(sContestLadyPtr)).numGoodPokeblocksGiven = 0;
    (*crate::gl!(sContestLadyPtr)).numOtherPokeblocksGiven = 0;
    (*crate::gl!(sContestLadyPtr)).maxSheen = 0;
    (*crate::gl!(sContestLadyPtr)).category = (Random() as i32 % 5) as u8;
}
unsafe fn InitLilycoveContestLady() {
    crate::gl!(sContestLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.contest);
    (*crate::gl!(sContestLadyPtr)).id = LILYCOVE_LADY_CONTEST;
    (*crate::gl!(sContestLadyPtr)).givenPokeblock = FALSE;
    ResetContestLadyContestData();
    (*crate::gl!(sContestLadyPtr)).language = gGameLanguage;
}
unsafe fn ResetContestLadyForRecordMix() {
    crate::gl!(sContestLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.contest);
    (*crate::gl!(sContestLadyPtr)).id = LILYCOVE_LADY_CONTEST;
    (*crate::gl!(sContestLadyPtr)).givenPokeblock = FALSE;
    if (*crate::gl!(sContestLadyPtr)).numGoodPokeblocksGiven == LILYCOVE_LADY_GIFT_THRESHOLD
        || (*crate::gl!(sContestLadyPtr)).numOtherPokeblocksGiven == LILYCOVE_LADY_GIFT_THRESHOLD
    {
        ResetContestLadyContestData();
    }
}
unsafe fn ContestLadySavePlayerNameIfHighSheen(sheen: u8) {
    crate::gl!(sContestLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.contest);
    if (*crate::gl!(sContestLadyPtr)).maxSheen <= sheen {
        (*crate::gl!(sContestLadyPtr)).maxSheen = sheen;
        memset(
            (*crate::gl!(sContestLadyPtr)).playerName.as_mut_ptr(),
            EOS as i32,
            8,
        );
        memcpy(
            (*crate::gl!(sContestLadyPtr)).playerName.as_mut_ptr(),
            (*crate::gl!(gSaveBlock2Ptr)).playerName.as_mut_ptr(),
            8,
        );
        (*crate::gl!(sContestLadyPtr)).language = gGameLanguage;
    }
}
pub unsafe fn GivePokeblockToContestLady(pokeblock: *mut Pokeblock) -> u8 {
    let mut sheen: u8 = 0;
    let mut correctFlavor: u8 = FALSE;
    crate::gl!(sContestLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.contest);
    match (*crate::gl!(sContestLadyPtr)).category {
        CONTEST_CATEGORY_COOL => {
            if (*pokeblock).spicy != 0 {
                sheen = (*pokeblock).spicy;
                correctFlavor = TRUE;
            }
        }
        CONTEST_CATEGORY_BEAUTY => {
            if (*pokeblock).dry != 0 {
                sheen = (*pokeblock).dry;
                correctFlavor = TRUE;
            }
        }
        CONTEST_CATEGORY_CUTE => {
            if (*pokeblock).sweet != 0 {
                sheen = (*pokeblock).sweet;
                correctFlavor = TRUE;
            }
        }
        CONTEST_CATEGORY_SMART => {
            if (*pokeblock).bitter != 0 {
                sheen = (*pokeblock).bitter;
                correctFlavor = TRUE;
            }
        }
        CONTEST_CATEGORY_TOUGH if (*pokeblock).sour != 0 => {
            sheen = (*pokeblock).sour;
            correctFlavor = TRUE;
        }
        _ => {}
    }
    if correctFlavor == TRUE {
        ContestLadySavePlayerNameIfHighSheen(sheen);
        (*crate::gl!(sContestLadyPtr)).numGoodPokeblocksGiven += 1;
    } else {
        (*crate::gl!(sContestLadyPtr)).numOtherPokeblocksGiven += 1;
    }
    correctFlavor
}
unsafe fn BufferContestLadyCategoryAndMonName(category: *mut u8, nickname: *mut u8) {
    crate::gl!(sContestLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.contest);
    StringCopy(
        category,
        sContestLadyCategoryNames[(*crate::gl!(sContestLadyPtr)).category],
    );
    StringCopy_Nickname(
        nickname,
        sContestLadyMonNames[(*crate::gl!(sContestLadyPtr)).category],
    );
}
pub unsafe fn BufferContestLadyMonName(category: *mut u8, nickname: *mut u8) {
    crate::gl!(sContestLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.contest);
    *category = (*crate::gl!(sContestLadyPtr)).category;
    StringCopy(
        nickname,
        sContestLadyMonNames[(*crate::gl!(sContestLadyPtr)).category],
    );
}
pub unsafe fn BufferContestLadyPlayerName(dest: *mut u8) {
    crate::gl!(sContestLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.contest);
    StringCopy(dest, (*crate::gl!(sContestLadyPtr)).playerName.as_mut_ptr());
}
pub unsafe fn BufferContestLadyLanguage(dest: *mut u8) {
    crate::gl!(sContestLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.contest);
    *dest = (*crate::gl!(sContestLadyPtr)).language;
}
pub unsafe fn BufferContestName(dest: *mut u8, category: u8) {
    StringCopy(dest, sContestNames[category]);
}
pub unsafe fn GetContestLadyPokeblockState() -> u8 {
    crate::gl!(sContestLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.contest);
    if (*crate::gl!(sContestLadyPtr)).numGoodPokeblocksGiven >= LILYCOVE_LADY_GIFT_THRESHOLD {
        return CONTEST_LADY_GOOD;
    } else if (*crate::gl!(sContestLadyPtr)).numGoodPokeblocksGiven == 0 {
        return CONTEST_LADY_BAD;
    } else {
        return CONTEST_LADY_NORMAL;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn HasPlayerGivenContestLadyPokeblock() -> u8 {
    crate::gl!(sContestLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.contest);
    if (*crate::gl!(sContestLadyPtr)).givenPokeblock == TRUE {
        return TRUE;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ShouldContestLadyShowGoOnAir() -> u8 {
    let mut putOnAir: u8 = FALSE;
    crate::gl!(sContestLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.contest);
    if (*crate::gl!(sContestLadyPtr)).numGoodPokeblocksGiven >= LILYCOVE_LADY_GIFT_THRESHOLD
        || (*crate::gl!(sContestLadyPtr)).numOtherPokeblocksGiven >= LILYCOVE_LADY_GIFT_THRESHOLD
    {
        putOnAir = TRUE;
    }
    putOnAir
}
#[unsafe(no_mangle)]
pub unsafe fn Script_BufferContestLadyCategoryAndMonName() {
    BufferContestLadyCategoryAndMonName(
        (*gStringVar2.as_ptr()).as_mut_ptr(),
        gStringVar1.as_mut_ptr(),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn OpenPokeblockCaseForContestLady() {
    OpenPokeblockCase(PBLOCK_CASE_GIVE, Some(CB2_ReturnToField));
}
#[unsafe(no_mangle)]
pub unsafe fn SetContestLadyGivenPokeblock() {
    crate::gl!(sContestLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.contest);
    (*crate::gl!(sContestLadyPtr)).givenPokeblock = TRUE;
}
#[unsafe(no_mangle)]
pub unsafe fn GetContestLadyMonSpecies() {
    crate::gl!(sContestLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.contest);
    gSpecialVar_0x8005 = sContestLadyMonSpecies[(*crate::gl!(sContestLadyPtr)).category];
}
#[unsafe(no_mangle)]
pub unsafe fn GetContestLadyCategory() -> u8 {
    crate::gl!(sContestLadyPtr = &raw mut (*crate::gl!(gSaveBlock1Ptr)).lilycoveLady.contest);
    (*crate::gl!(sContestLadyPtr)).category
}
