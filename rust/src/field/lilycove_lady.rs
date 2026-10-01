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

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFavorLadyPtr: *mut LilycoveLadyFavor = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sQuizLadyPtr: *mut LilycoveLadyQuiz = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sContestLadyPtr: *mut LilycoveLadyContest = null_mut();

/// `GetItemName` with this module's view of its types.
#[inline]
unsafe fn GetItemName(a0: u16) -> *mut u8 {
    crate::item::GetItemName(a0) as *mut u8
}

pub unsafe fn GetLilycoveLadyId() -> u8 {
    (*gSaveBlock1Ptr).lilycoveLady.id
}
#[unsafe(no_mangle)]
pub unsafe fn SetLilycoveLadyGfx() {
    let mut lilycoveLady: *mut LilycoveLady = null_mut();
    VarSet(VAR_OBJ_GFX_ID_0, sLilycoveLadyGfxId[GetLilycoveLadyId()]);
    if GetLilycoveLadyId() == LILYCOVE_LADY_CONTEST {
        lilycoveLady = &raw mut (*gSaveBlock1Ptr).lilycoveLady;
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
    let mut id: u16 = ((*gSaveBlock2Ptr).playerTrainerId[1] as u16) << 8
        | (*gSaveBlock2Ptr).playerTrainerId[0] as u16;
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
    (*sFavorLadyPtr).favorId = (Random() % 6) as u8;
    let numItems: u8 = GetNumAcceptedItems(sFavorLadyAcceptedItemLists[(*sFavorLadyPtr).favorId]);
    let bestItem: u8 = rem_i32(Random() as i32, numItems as i32) as u8;
    (*sFavorLadyPtr).bestItem = *sFavorLadyAcceptedItemLists[(*sFavorLadyPtr).favorId].at(bestItem);
}
unsafe fn InitLilycoveFavorLady() {
    sFavorLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.favor;
    (*sFavorLadyPtr).id = LILYCOVE_LADY_FAVOR;
    (*sFavorLadyPtr).state = LILYCOVE_LADY_STATE_READY;
    (*sFavorLadyPtr).playerName[0] = EOS;
    (*sFavorLadyPtr).likedItem = FALSE;
    (*sFavorLadyPtr).numItemsGiven = 0;
    (*sFavorLadyPtr).itemId = ITEM_NONE;
    (*sFavorLadyPtr).language = gGameLanguage;
    FavorLadyPickFavorAndBestItem();
}
unsafe fn ResetFavorLadyForRecordMix() {
    sFavorLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.favor;
    (*sFavorLadyPtr).id = LILYCOVE_LADY_FAVOR;
    (*sFavorLadyPtr).state = LILYCOVE_LADY_STATE_READY;
}
#[unsafe(no_mangle)]
pub unsafe fn GetFavorLadyState() -> u8 {
    sFavorLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.favor;
    if (*sFavorLadyPtr).state == LILYCOVE_LADY_STATE_PRIZE {
        return LILYCOVE_LADY_STATE_PRIZE;
    } else if (*sFavorLadyPtr).state == LILYCOVE_LADY_STATE_COMPLETED {
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
    sFavorLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.favor;
    StringCopy(
        gStringVar1.as_mut_ptr(),
        GetFavorLadyRequest((*sFavorLadyPtr).favorId),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn HasAnotherPlayerGivenFavorLadyItem() -> u8 {
    sFavorLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.favor;
    if (*sFavorLadyPtr).playerName[0] != EOS {
        StringCopy_PlayerName(
            gStringVar3.as_mut_ptr(),
            (*sFavorLadyPtr).playerName.as_mut_ptr(),
        );
        ConvertInternationalString(gStringVar3.as_mut_ptr(), (*sFavorLadyPtr).language);
        return TRUE;
    }
    FALSE
}
unsafe fn BufferItemName(dest: *mut u8, itemId: u16) {
    StringCopy(dest, GetItemName(itemId));
}
#[unsafe(no_mangle)]
pub unsafe fn BufferFavorLadyItemName() {
    sFavorLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.favor;
    BufferItemName(gStringVar2.as_mut_ptr(), (*sFavorLadyPtr).itemId);
}
unsafe fn SetFavorLadyPlayerName(src: *mut u8, dest: *mut u8) {
    memset(dest, EOS as i32, 8);
    StringCopy_PlayerName(dest, src);
}
#[unsafe(no_mangle)]
pub unsafe fn BufferFavorLadyPlayerName() {
    sFavorLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.favor;
    SetFavorLadyPlayerName(
        (*sFavorLadyPtr).playerName.as_mut_ptr(),
        gStringVar3.as_mut_ptr(),
    );
    ConvertInternationalString(gStringVar3.as_mut_ptr(), (*sFavorLadyPtr).language);
}
#[unsafe(no_mangle)]
pub unsafe fn DidFavorLadyLikeItem() -> u8 {
    sFavorLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.favor;
    (if (*sFavorLadyPtr).likedItem != 0 {
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
    sFavorLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.favor;
    let numItems: u8 = GetNumAcceptedItems(sFavorLadyAcceptedItemLists[(*sFavorLadyPtr).favorId]);
    (*sFavorLadyPtr).state = LILYCOVE_LADY_STATE_COMPLETED;
    BufferItemName(gStringVar2.as_mut_ptr(), itemId);
    (*sFavorLadyPtr).itemId = itemId;
    SetFavorLadyPlayerName(
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        (*sFavorLadyPtr).playerName.as_mut_ptr(),
    );
    (*sFavorLadyPtr).language = gGameLanguage;
    let mut likedItem: u8 = FALSE;
    for i in 0..numItems {
        if *sFavorLadyAcceptedItemLists[(*sFavorLadyPtr).favorId].at(i) == itemId {
            likedItem = TRUE;
            (*sFavorLadyPtr).numItemsGiven += 1;
            (*sFavorLadyPtr).likedItem = TRUE;
            if (*sFavorLadyPtr).bestItem == itemId {
                (*sFavorLadyPtr).numItemsGiven = LILYCOVE_LADY_GIFT_THRESHOLD;
            }
            break;
        }
        (*sFavorLadyPtr).likedItem = FALSE;
    }
    likedItem
}
#[unsafe(no_mangle)]
pub unsafe fn Script_DoesFavorLadyLikeItem() -> u8 {
    DoesFavorLadyLikeItem(gSpecialVar_ItemId)
}
#[unsafe(no_mangle)]
pub unsafe fn IsFavorLadyThresholdMet() -> u8 {
    let mut numItemsGiven: u8 = 0;
    sFavorLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.favor;
    numItemsGiven = (*sFavorLadyPtr).numItemsGiven;
    (if numItemsGiven < LILYCOVE_LADY_GIFT_THRESHOLD {
        FALSE as i32
    } else {
        TRUE as i32
    }) as u8
}
unsafe fn FavorLadyBufferPrizeName(prize: u16) {
    BufferItemName(gStringVar2.as_mut_ptr(), prize);
}
#[unsafe(no_mangle)]
pub unsafe fn FavorLadyGetPrize() -> u16 {
    sFavorLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.favor;
    let prize: u16 = sFavorLadyPrizes[(*sFavorLadyPtr).favorId];
    FavorLadyBufferPrizeName(prize);
    (*sFavorLadyPtr).state = LILYCOVE_LADY_STATE_PRIZE;
    prize
}
#[unsafe(no_mangle)]
pub unsafe fn SetFavorLadyState_Complete() {
    InitLilycoveFavorLady();
    (*sFavorLadyPtr).state = LILYCOVE_LADY_STATE_COMPLETED;
}
pub unsafe fn FieldCallback_FavorLadyEnableScriptContexts() {
    ScriptContext_Enable();
}
unsafe fn QuizLadyPickQuestion() {
    let questionId: u8 = (Random() % 16) as u8;
    for i in 0..QUIZ_QUESTION_LEN {
        (*sQuizLadyPtr).question[i] = *sQuizLadyQuizQuestions[questionId].at(i);
    }
    (*sQuizLadyPtr).correctAnswer = sQuizLadyQuizAnswers[questionId];
    (*sQuizLadyPtr).prize = sQuizLadyPrizes[questionId];
    (*sQuizLadyPtr).questionId = questionId;
    (*sQuizLadyPtr).playerName[0] = EOS;
}
unsafe fn InitLilycoveQuizLady() {
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    (*sQuizLadyPtr).id = LILYCOVE_LADY_QUIZ;
    (*sQuizLadyPtr).state = LILYCOVE_LADY_STATE_READY;
    let mut i: u8 = 0;
    while i < QUIZ_QUESTION_LEN {
        (*sQuizLadyPtr).question[i] = EC_EMPTY_WORD;
        i += 1;
    }
    (*sQuizLadyPtr).correctAnswer = EC_EMPTY_WORD;
    (*sQuizLadyPtr).playerAnswer = EC_EMPTY_WORD;
    for i in 0..TRAINER_ID_LENGTH {
        (*sQuizLadyPtr).playerTrainerId[i] = 0;
    }
    (*sQuizLadyPtr).prize = ITEM_NONE;
    (*sQuizLadyPtr).waitingForChallenger = FALSE;
    (*sQuizLadyPtr).prevQuestionId = 16;
    (*sQuizLadyPtr).language = gGameLanguage;
    QuizLadyPickQuestion();
}
unsafe fn ResetQuizLadyForRecordMix() {
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    (*sQuizLadyPtr).id = LILYCOVE_LADY_QUIZ;
    (*sQuizLadyPtr).state = LILYCOVE_LADY_STATE_READY;
    (*sQuizLadyPtr).waitingForChallenger = FALSE;
    (*sQuizLadyPtr).playerAnswer = EC_EMPTY_WORD;
}
#[unsafe(no_mangle)]
pub unsafe fn GetQuizLadyState() -> u8 {
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    if (*sQuizLadyPtr).state == LILYCOVE_LADY_STATE_PRIZE {
        return LILYCOVE_LADY_STATE_PRIZE;
    } else if (*sQuizLadyPtr).state == LILYCOVE_LADY_STATE_COMPLETED {
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
    let quiz: *mut LilycoveLadyQuiz = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
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
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    if (*sQuizLadyPtr).playerName[0] == EOS {
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
            (*sQuizLadyPtr).playerName.as_mut_ptr(),
        );
        ConvertInternationalString(gStringVar1.as_mut_ptr(), (*sQuizLadyPtr).language);
        nameLen = GetPlayerNameLength((*sQuizLadyPtr).playerName.as_mut_ptr());
        if nameLen == GetPlayerNameLength((*gSaveBlock2Ptr).playerName.as_mut_ptr()) {
            let mut name: *mut u8 = (*sQuizLadyPtr).playerName.as_mut_ptr();
            for i in 0..nameLen {
                name = (*sQuizLadyPtr).playerName.as_mut_ptr();
                if *name.at(i) != (*gSaveBlock2Ptr).playerName[i] {
                    authorNameId = QUIZ_AUTHOR_NAME_OTHER_PLAYER;
                    break;
                }
            }
        }
    }
    authorNameId
}
unsafe fn IsQuizTrainerIdNotPlayer() -> u8 {
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    let mut notPlayer: u8 = FALSE;
    for i in 0..TRAINER_ID_LENGTH {
        if (*sQuizLadyPtr).playerTrainerId[i] != (*gSaveBlock2Ptr).playerTrainerId[i] as u16 {
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
    StringCopy(gStringVar1.as_mut_ptr(), GetItemName((*sQuizLadyPtr).prize));
}
#[unsafe(no_mangle)]
pub unsafe fn BufferQuizAuthorNameAndCheckIfLady() -> u8 {
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    if BufferQuizAuthorName() == QUIZ_AUTHOR_NAME_LADY {
        (*sQuizLadyPtr).language = gGameLanguage;
        return TRUE;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn IsQuizLadyWaitingForChallenger() -> u8 {
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    (*sQuizLadyPtr).waitingForChallenger
}
#[unsafe(no_mangle)]
pub unsafe fn QuizLadyGetPlayerAnswer() {
    ShowEasyChatScreen();
}
#[unsafe(no_mangle)]
pub unsafe fn IsQuizAnswerCorrect() -> u8 {
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    CopyEasyChatWord(gStringVar1.as_mut_ptr(), (*sQuizLadyPtr).correctAnswer);
    CopyEasyChatWord(gStringVar2.as_mut_ptr(), (*sQuizLadyPtr).playerAnswer);
    (if StringCompare(gStringVar1.as_mut_ptr(), gStringVar2.as_mut_ptr()) != 0 {
        FALSE as i32
    } else {
        TRUE as i32
    }) as u8
}
#[unsafe(no_mangle)]
pub unsafe fn BufferQuizPrizeItem() {
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    gSpecialVar_0x8005 = (*sQuizLadyPtr).prize;
}
#[unsafe(no_mangle)]
pub unsafe fn SetQuizLadyState_Complete() {
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    (*sQuizLadyPtr).state = LILYCOVE_LADY_STATE_COMPLETED;
}
#[unsafe(no_mangle)]
pub unsafe fn SetQuizLadyState_GivePrize() {
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    (*sQuizLadyPtr).state = LILYCOVE_LADY_STATE_PRIZE;
}
#[unsafe(no_mangle)]
pub unsafe fn ClearQuizLadyPlayerAnswer() {
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    (*sQuizLadyPtr).playerAnswer = EC_EMPTY_WORD;
}
#[unsafe(no_mangle)]
pub unsafe fn Script_QuizLadyOpenBagMenu() {
    QuizLadyOpenBagMenu();
}
#[unsafe(no_mangle)]
pub unsafe fn QuizLadyPickNewQuestion() {
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    if BufferQuizAuthorNameAndCheckIfLady() != 0 {
        (*sQuizLadyPtr).prevQuestionId = (*sQuizLadyPtr).questionId;
    } else {
        (*sQuizLadyPtr).prevQuestionId = 16;
    }
    QuizLadyPickQuestion();
}
#[unsafe(no_mangle)]
pub unsafe fn ClearQuizLadyQuestionAndAnswer() {
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    for i in 0..QUIZ_QUESTION_LEN {
        (*sQuizLadyPtr).question[i] = EC_EMPTY_WORD;
    }
    (*sQuizLadyPtr).correctAnswer = EC_EMPTY_WORD;
}
#[unsafe(no_mangle)]
pub unsafe fn QuizLadySetCustomQuestion() {
    gSpecialVar_0x8004 = EASY_CHAT_TYPE_QUIZ_SET_QUESTION as u16;
    ShowEasyChatScreen();
}
#[unsafe(no_mangle)]
pub unsafe fn QuizLadyTakePrizeForCustomQuiz() {
    RemoveBagItem(gSpecialVar_ItemId, 1);
}
#[unsafe(no_mangle)]
pub unsafe fn QuizLadyRecordCustomQuizData() {
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    (*sQuizLadyPtr).prize = gSpecialVar_ItemId;
    for i in 0..TRAINER_ID_LENGTH {
        (*sQuizLadyPtr).playerTrainerId[i] = (*gSaveBlock2Ptr).playerTrainerId[i] as u16;
    }
    StringCopy_PlayerName(
        (*sQuizLadyPtr).playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
    );
    (*sQuizLadyPtr).language = gGameLanguage;
}
#[unsafe(no_mangle)]
pub unsafe fn QuizLadySetWaitingForChallenger() {
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    (*sQuizLadyPtr).waitingForChallenger = TRUE;
}
#[unsafe(no_mangle)]
pub unsafe fn BufferQuizCorrectAnswer() {
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    CopyEasyChatWord(gStringVar3.as_mut_ptr(), (*sQuizLadyPtr).correctAnswer);
}
pub unsafe fn FieldCallback_QuizLadyEnableScriptContexts() {
    ScriptContext_Enable();
}
pub unsafe fn QuizLadyClearQuestionForRecordMix(lilycoveLady: *mut LilycoveLady) {
    sQuizLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    if (*lilycoveLady).quiz.prevQuestionId < 16 && (*sQuizLadyPtr).id == LILYCOVE_LADY_QUIZ {
        for i in 0..4u8 {
            if (*lilycoveLady).quiz.prevQuestionId != (*sQuizLadyPtr).questionId {
                break;
            }
            (*sQuizLadyPtr).questionId = (Random() % 16) as u8;
        }
        if (*lilycoveLady).quiz.prevQuestionId == (*sQuizLadyPtr).questionId {
            (*sQuizLadyPtr).questionId = (((*sQuizLadyPtr).questionId as i32 + 1) % 16) as u8;
        }
        (*sQuizLadyPtr).prevQuestionId = (*lilycoveLady).quiz.prevQuestionId;
    }
}
unsafe fn ResetContestLadyContestData() {
    (*sContestLadyPtr).playerName[0] = EOS;
    (*sContestLadyPtr).numGoodPokeblocksGiven = 0;
    (*sContestLadyPtr).numOtherPokeblocksGiven = 0;
    (*sContestLadyPtr).maxSheen = 0;
    (*sContestLadyPtr).category = (Random() as i32 % 5) as u8;
}
unsafe fn InitLilycoveContestLady() {
    sContestLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.contest;
    (*sContestLadyPtr).id = LILYCOVE_LADY_CONTEST;
    (*sContestLadyPtr).givenPokeblock = FALSE;
    ResetContestLadyContestData();
    (*sContestLadyPtr).language = gGameLanguage;
}
unsafe fn ResetContestLadyForRecordMix() {
    sContestLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.contest;
    (*sContestLadyPtr).id = LILYCOVE_LADY_CONTEST;
    (*sContestLadyPtr).givenPokeblock = FALSE;
    if (*sContestLadyPtr).numGoodPokeblocksGiven == LILYCOVE_LADY_GIFT_THRESHOLD
        || (*sContestLadyPtr).numOtherPokeblocksGiven == LILYCOVE_LADY_GIFT_THRESHOLD
    {
        ResetContestLadyContestData();
    }
}
unsafe fn ContestLadySavePlayerNameIfHighSheen(sheen: u8) {
    sContestLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.contest;
    if (*sContestLadyPtr).maxSheen <= sheen {
        (*sContestLadyPtr).maxSheen = sheen;
        memset((*sContestLadyPtr).playerName.as_mut_ptr(), EOS as i32, 8);
        memcpy(
            (*sContestLadyPtr).playerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
            8,
        );
        (*sContestLadyPtr).language = gGameLanguage;
    }
}
pub unsafe fn GivePokeblockToContestLady(pokeblock: *mut Pokeblock) -> u8 {
    let mut sheen: u8 = 0;
    let mut correctFlavor: u8 = FALSE;
    sContestLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.contest;
    match (*sContestLadyPtr).category {
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
        (*sContestLadyPtr).numGoodPokeblocksGiven += 1;
    } else {
        (*sContestLadyPtr).numOtherPokeblocksGiven += 1;
    }
    correctFlavor
}
unsafe fn BufferContestLadyCategoryAndMonName(category: *mut u8, nickname: *mut u8) {
    sContestLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.contest;
    StringCopy(
        category,
        sContestLadyCategoryNames[(*sContestLadyPtr).category],
    );
    StringCopy_Nickname(nickname, sContestLadyMonNames[(*sContestLadyPtr).category]);
}
pub unsafe fn BufferContestLadyMonName(category: *mut u8, nickname: *mut u8) {
    sContestLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.contest;
    *category = (*sContestLadyPtr).category;
    StringCopy(nickname, sContestLadyMonNames[(*sContestLadyPtr).category]);
}
pub unsafe fn BufferContestLadyPlayerName(dest: *mut u8) {
    sContestLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.contest;
    StringCopy(dest, (*sContestLadyPtr).playerName.as_mut_ptr());
}
pub unsafe fn BufferContestLadyLanguage(dest: *mut u8) {
    sContestLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.contest;
    *dest = (*sContestLadyPtr).language;
}
pub unsafe fn BufferContestName(dest: *mut u8, category: u8) {
    StringCopy(dest, sContestNames[category]);
}
pub unsafe fn GetContestLadyPokeblockState() -> u8 {
    sContestLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.contest;
    if (*sContestLadyPtr).numGoodPokeblocksGiven >= LILYCOVE_LADY_GIFT_THRESHOLD {
        return CONTEST_LADY_GOOD;
    } else if (*sContestLadyPtr).numGoodPokeblocksGiven == 0 {
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
    sContestLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.contest;
    if (*sContestLadyPtr).givenPokeblock == TRUE {
        return TRUE;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ShouldContestLadyShowGoOnAir() -> u8 {
    let mut putOnAir: u8 = FALSE;
    sContestLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.contest;
    if (*sContestLadyPtr).numGoodPokeblocksGiven >= LILYCOVE_LADY_GIFT_THRESHOLD
        || (*sContestLadyPtr).numOtherPokeblocksGiven >= LILYCOVE_LADY_GIFT_THRESHOLD
    {
        putOnAir = TRUE;
    }
    putOnAir
}
#[unsafe(no_mangle)]
pub unsafe fn Script_BufferContestLadyCategoryAndMonName() {
    BufferContestLadyCategoryAndMonName(gStringVar2.as_mut_ptr(), gStringVar1.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe fn OpenPokeblockCaseForContestLady() {
    OpenPokeblockCase(PBLOCK_CASE_GIVE, Some(CB2_ReturnToField));
}
#[unsafe(no_mangle)]
pub unsafe fn SetContestLadyGivenPokeblock() {
    sContestLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.contest;
    (*sContestLadyPtr).givenPokeblock = TRUE;
}
#[unsafe(no_mangle)]
pub unsafe fn GetContestLadyMonSpecies() {
    sContestLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.contest;
    gSpecialVar_0x8005 = sContestLadyMonSpecies[(*sContestLadyPtr).category];
}
#[unsafe(no_mangle)]
pub unsafe fn GetContestLadyCategory() -> u8 {
    sContestLadyPtr = &raw mut (*gSaveBlock1Ptr).lilycoveLady.contest;
    (*sContestLadyPtr).category
}
