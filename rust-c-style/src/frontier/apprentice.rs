//! Translated from `src/apprentice.c` by tools/rustport/c2rs.py.
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
static sApprenticeFunctions: Table<CArray<Option<unsafe extern "C" fn()>, 26>> =
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

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gApprenticePartyMovesData: *mut ApprenticePartyMovesData = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gApprenticeQuestionData: *mut ApprenticeQuestionData = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gApprenticeFunc: Option<unsafe extern "C" fn()> = None;

unsafe extern "C" {
    static gGameLanguage: u8;
    static gLevelUpLearnsets: CArray<*mut u16, 0>;
    static mut gMain: Main;
    static gMoveNames: CArray<CArray<u8, 13>, 355>;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSpecialVar_0x8006: u16;
    static mut gSpecialVar_Result: u16;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_Give: CArray<u8, 0>;
    static gText_Lv50: CArray<u8, 0>;
    static gText_No: CArray<u8, 0>;
    static gText_NoNeed: CArray<u8, 0>;
    static gText_OpenLevel: CArray<u8, 0>;
    static gText_Yes: CArray<u8, 0>;
    static gTowerFemaleFacilityClasses: CArray<u8, 20>;
    static gTowerFemaleTrainerGfxIds: CArray<u8, 20>;
    static gTowerMaleFacilityClasses: CArray<u8, 30>;
    static gTowerMaleTrainerGfxIds: CArray<u8, 30>;
    fn AddTextPrinterForMessage(a0: u8);
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn ApprenticeOpenBagMenu();
    fn CalcApprenticeChecksum(a0: *mut Apprentice);
    fn CanSpeciesLearnTMHM(a0: u16, a1: u8) -> u32;
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn ConvertPixelWidthToTileWidth(a0: i32) -> i32;
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWindowTemplate(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u16,
    ) -> WindowTemplate;
    fn DestroyTask(a0: u8);
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn Free(a0: *mut c_void);
    fn FreezeObjectEvents();
    fn FrontierSpeechToString(a0: *mut u16);
    fn GetItemName(a0: u16) -> *mut u8;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetTrainerId(a0: *mut u8) -> u32;
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn ItemIdToBattleMoveId(a0: u16) -> u16;
    fn LockPlayerFieldControls();
    fn Menu_ProcessInput() -> i8;
    fn Menu_ProcessInputNoWrap() -> i8;
    fn PlaySE(a0: u16);
    fn PlayerFreeze();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveWindow(a0: u8);
    fn RunTextPrintersAndIsPrinter0Active() -> u16;
    fn ScriptContext_Enable();
    fn ScriptMenu_AdjustLeftCoordFromWidth(a0: i32, a1: i32) -> i32;
    fn SetStandardWindowBorderStyle(a0: u8, a1: u8);
    fn SetTaskFuncWithFollowupFunc(
        a0: u8,
        a1: Option<unsafe extern "C" fn(u8)>,
        a2: Option<unsafe extern "C" fn(u8)>,
    );
    fn StopPlayerAvatar();
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy_PlayerName(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn SwitchTaskToFollowupFunc(a0: u8);
    fn TVShowConvertInternationalString(a0: *mut u8, a1: *mut u8, a2: i32);
    fn VarSet(a0: u16, a1: u16) -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferApprenticeChallengeText(saveApprenticeId: u8) {
    let mut i: u8 = 0;
    let mut num: u8 = 0;
    let mut challengeText: *mut u8 = null_mut();
    num = (*gSaveBlock2Ptr).apprentices[saveApprenticeId].number;
    i = 0;
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
    challengeText = sApprenticeChallengeTexts[(*gSaveBlock2Ptr).apprentices[saveApprenticeId].id()];
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), challengeText);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Apprentice_ScriptContext_Enable() {
    ScriptContext_Enable();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetApprenticeStruct(apprentice: *mut Apprentice) {
    let mut i: u8 = 0;
    i = 0;
    while i < 6 {
        (*apprentice).speechWon[i] = EC_EMPTY_WORD;
        i += 1;
    }
    (*apprentice).playerName[0] = EOS;
    (*apprentice).set_id(NUM_APPRENTICES);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetAllApprenticeData() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    (*gSaveBlock2Ptr).playerApprentice.set_saveId(0);
    i = 0;
    while i < APPRENTICE_COUNT as u8 {
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
        j = 0;
        while j < TRAINER_ID_LENGTH {
            (*gSaveBlock2Ptr).apprentices[i].playerId[j] = 0;
            j += 1;
        }
        (*gSaveBlock2Ptr).apprentices[i].language = gGameLanguage;
        (*gSaveBlock2Ptr).apprentices[i].checksum = 0;
        i += 1;
    }
    Script_ResetPlayerApprentice();
}
pub(crate) unsafe extern "C" fn GivenApprenticeLvlMode() -> u8 {
    return ((*gSaveBlock2Ptr).playerApprentice.lvlMode() != 0) as u8;
}
pub(crate) unsafe extern "C" fn SetApprenticeId() {
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
pub(crate) unsafe extern "C" fn SetPlayersApprenticeLvlMode(mode: u8) {
    (*gSaveBlock2Ptr).playerApprentice.set_lvlMode(mode);
}
pub(crate) unsafe extern "C" fn ShuffleApprenticeSpecies() {
    let mut species: CArray<u8, 10> = zeroed();
    let mut i: u8 = 0;
    i = 0;
    while i < 10 {
        species[i] = i;
        i += 1;
    }
    i = 0;
    while i < 50 {
        let mut temp: u8 = 0;
        let mut rand1: u8 = (Random() % 10) as u8;
        let mut rand2: u8 = (Random() % 10) as u8;
        temp = species[rand1];
        species[rand1] = species[rand2];
        species[rand2] = temp;
        i += 1;
    }
    i = 0;
    while i < MULTI_PARTY_SIZE as u8 {
        (*gSaveBlock2Ptr).playerApprentice.speciesIds[i] =
            (species[i as i32 * 2] & 0xF) << 4 | species[i as i32 * 2 + 1] & 0xF;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetMonIdForQuestion(
    questionId: u8,
    party: *mut u8,
    partySlot: *mut u8,
) -> u8 {
    let mut i: u8 = 0;
    let mut count: u8 = 0;
    let mut monId: u8 = 0;
    if questionId == QUESTION_ID_WHICH_MOVE {
        loop {
            monId = (Random() as i32 % 3) as u8;
            count = 0;
            i = 0;
            while i < NUM_WHICH_MOVE_QUESTIONS {
                if (*gApprenticePartyMovesData).moves[monId][i] != MOVE_NONE {
                    count += 1;
                }
                i += 1;
            }
            if count <= MULTI_PARTY_SIZE as u8 {
                break;
            }
        }
    } else if questionId == QUESTION_ID_WHAT_ITEM {
        monId = *party.at(*partySlot);
        *partySlot += 1;
    }
    return monId;
}
pub(crate) unsafe extern "C" fn SetRandomQuestionData() {
    let mut questionOrder: CArray<u8, 10> = zeroed();
    let mut partyOrder: CArray<u8, 3> = zeroed();
    let mut partySlot: u8 = 0;
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut rand1: u8 = 0;
    let mut rand2: u8 = 0;
    let mut id: u8 = 0;
    i = 0;
    while i < 3 {
        partyOrder[i] = i;
        i += 1;
    }
    i = 0;
    while i < 10 {
        let mut temp: u8 = 0;
        rand1 = (Random() % 3) as u8;
        rand2 = (Random() % 3) as u8;
        temp = partyOrder[rand1];
        partyOrder[rand1] = partyOrder[rand2];
        partyOrder[rand2] = temp;
        i += 1;
    }
    i = 0;
    while i < 10 {
        questionOrder[i] = sQuestionPossibilities[i];
        i += 1;
    }
    i = 0;
    while i < 50 {
        let mut temp: u8 = 0;
        rand1 = (Random() % 10) as u8;
        rand2 = (Random() % 10) as u8;
        temp = questionOrder[rand1];
        questionOrder[rand1] = questionOrder[rand2];
        questionOrder[rand2] = temp;
        i += 1;
    }
    gApprenticePartyMovesData = AllocZeroed(48) as *mut ApprenticePartyMovesData;
    (*gApprenticePartyMovesData).moveCounter = 0;
    i = 0;
    while i < NUM_WHICH_MOVE_QUESTIONS {
        j = 0;
        while j < MULTI_PARTY_SIZE as u8 {
            (*gApprenticePartyMovesData).moveSlots[j][i] = MAX_MON_MOVES as u8;
            j += 1;
        }
        i += 1;
    }
    partySlot = 0;
    i = 0;
    while i < APPRENTICE_MAX_QUESTIONS {
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
        i += 1;
    }
    Free(gApprenticePartyMovesData as *mut c_void);
    gApprenticePartyMovesData = null_mut();
}
pub(crate) unsafe extern "C" fn GetRandomAlternateMove(monId: u8) -> u16 {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut id: u8 = 0;
    let mut numLearnsetMoves: u8 = 0;
    let mut species: u16 = 0;
    let mut learnset: *mut u16 = null_mut();
    let mut needTMs: u32 = FALSE as u32;
    let mut r#move: u16 = MOVE_NONE;
    let mut shouldUseMove: u32 = 0;
    let mut level: u8 = 0;
    id = (if monId < 3 {
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
    learnset = gLevelUpLearnsets[species];
    j = 0;
    if (*gSaveBlock2Ptr).playerApprentice.lvlMode() == APPRENTICE_LVL_MODE_50 {
        level = FRONTIER_MAX_LEVEL_50;
    } else {
        level = 60;
    }
    j = 0;
    while *learnset.at(j) != LEVEL_UP_END {
        if *learnset.at(j) as i32 & LEVEL_UP_MOVE_LV > (level as i32) << 9 {
            break;
        }
        j += 1;
    }
    numLearnsetMoves = j;
    i = 0;
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
                    let mut learnsetId: u8 =
                        rem_i32(Random() as i32, numLearnsetMoves as i32 - MAX_MON_MOVES) as u8;
                    r#move = *learnset.at(learnsetId) & LEVEL_UP_MOVE_ID;
                    shouldUseMove = TRUE as u32;
                    j = numLearnsetMoves - MAX_MON_MOVES as u8;
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
    return r#move;
}
pub(crate) unsafe extern "C" fn TrySetMove(monId: u8, r#move: u16) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < NUM_WHICH_MOVE_QUESTIONS {
        if (*gApprenticePartyMovesData).moves[monId][i] == r#move {
            return FALSE;
        }
        i += 1;
    }
    (*gApprenticePartyMovesData).moves[monId][(*gApprenticePartyMovesData).moveCounter] = r#move;
    return TRUE;
}
pub(crate) unsafe extern "C" fn GetLatestLearnedMoves(species: u16, mut moves: *mut u16) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut level: u8 = 0;
    let mut numLearnsetMoves: u8 = 0;
    let mut learnset: *mut u16 = null_mut();
    if (*gSaveBlock2Ptr).playerApprentice.lvlMode() == APPRENTICE_LVL_MODE_50 {
        level = FRONTIER_MAX_LEVEL_50;
    } else {
        level = 60;
    }
    learnset = gLevelUpLearnsets[species];
    i = 0;
    while *learnset.at(i) != LEVEL_UP_END {
        if *learnset.at(i) as i32 & LEVEL_UP_MOVE_LV > (level as i32) << 9 {
            break;
        }
        i += 1;
    }
    numLearnsetMoves = i;
    if numLearnsetMoves > MAX_MON_MOVES as u8 {
        numLearnsetMoves = MAX_MON_MOVES as u8;
    }
    j = 0;
    while j < numLearnsetMoves {
        *moves.at(j) = *learnset.at(i as i32 - 1 - j as i32) & LEVEL_UP_MOVE_ID;
        j += 1;
    }
}
pub(crate) unsafe extern "C" fn GetDefaultMove(monId: u8, speciesArrayId: u8, moveSlot: u8) -> u16 {
    let mut moves: CArray<u16, 4> = zeroed();
    let mut i: u8 = 0;
    let mut numQuestions: u8 = 0;
    if (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() < NUM_WHICH_MON_QUESTIONS {
        return MOVE_NONE;
    }
    numQuestions = 0;
    i = 0;
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
    return moves[moveSlot];
}
pub(crate) unsafe extern "C" fn SaveApprenticeParty(numQuestions: u8) {
    let mut apprenticeMons: CArray<*mut ApprenticeMon, 3> = zeroed();
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut speciesTableId: u32 = 0;
    i = 0;
    while i < MULTI_PARTY_SIZE as u8 {
        (*gSaveBlock2Ptr).apprentices[0].party[i].species = 0;
        (*gSaveBlock2Ptr).apprentices[0].party[i].item = ITEM_NONE;
        j = 0;
        while j < MAX_MON_MOVES as u8 {
            (*gSaveBlock2Ptr).apprentices[0].party[i].moves[j] = 0;
            j += 1;
        }
        i += 1;
    }
    j = (*gSaveBlock2Ptr).playerApprentice.leadMonId();
    i = 0;
    while i < MULTI_PARTY_SIZE as u8 {
        apprenticeMons[j] = &raw mut (*gSaveBlock2Ptr).apprentices[0].party[i];
        j = ((j as i32 + 1) % 3) as u8;
        i += 1;
    }
    i = 0;
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
    i = 0;
    while i < numQuestions {
        let mut questionId: u8 = (*gSaveBlock2Ptr).playerApprentice.questions[i].questionId();
        let mut monId: u8 = (*gSaveBlock2Ptr).playerApprentice.questions[i].monId();
        if questionId == QUESTION_ID_WHAT_ITEM {
            if (*gSaveBlock2Ptr).playerApprentice.questions[i].suggestedChange() != 0 {
                (*apprenticeMons[monId]).item =
                    (*gSaveBlock2Ptr).playerApprentice.questions[i].data;
            }
        } else if questionId == QUESTION_ID_WHICH_MOVE {
            if (*gSaveBlock2Ptr).playerApprentice.questions[i].suggestedChange() != 0 {
                let mut moveSlot: u32 =
                    (*gSaveBlock2Ptr).playerApprentice.questions[i].moveSlot() as u32;
                (*apprenticeMons[monId]).moves[moveSlot] =
                    (*gSaveBlock2Ptr).playerApprentice.questions[i].data;
            }
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateApprenticeMenu(menu: u8) {
    let mut i: u8 = 0;
    let mut windowId: u8 = 0;
    let mut strings: CArray<*mut u8, 3> = zeroed();
    let mut count: u8 = 2;
    let mut width: u8 = 0;
    let mut left: u8 = 0;
    let mut top: u8 = 0;
    let mut pixelWidth: i32 = 0;
    match menu {
        APPRENTICE_ASK_WHICH_LEVEL => {
            left = 18;
            top = 8;
            strings[0] = gText_Lv50.as_ptr().cast_mut();
            strings[1] = gText_OpenLevel.as_ptr().cast_mut();
        }
        APPRENTICE_ASK_3SPECIES => {
            count = MULTI_PARTY_SIZE as u8;
            left = 18;
            top = 6;
            i = 0;
            while i < MULTI_PARTY_SIZE as u8 {
                let mut species: u16 = 0;
                let mut speciesTableId: u32 = 0;
                speciesTableId = (if i < 3 {
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
                strings[i] = gSpeciesNames[species].as_ptr().cast_mut();
                i += 1;
            }
        }
        APPRENTICE_ASK_2SPECIES => {
            left = 18;
            top = 8;
            if (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() >= NUM_WHICH_MON_QUESTIONS {
                return;
            }
            strings[1] = gSpeciesNames[(*gApprenticeQuestionData).altSpeciesId]
                .as_ptr()
                .cast_mut();
            strings[0] = gSpeciesNames[(*gApprenticeQuestionData).speciesId]
                .as_ptr()
                .cast_mut();
        }
        APPRENTICE_ASK_MOVES => {
            left = 17;
            top = 8;
            strings[0] = gMoveNames[(*gApprenticeQuestionData).move1]
                .as_ptr()
                .cast_mut();
            strings[1] = gMoveNames[(*gApprenticeQuestionData).move2]
                .as_ptr()
                .cast_mut();
        }
        APPRENTICE_ASK_GIVE => {
            left = 18;
            top = 8;
            strings[0] = gText_Give.as_ptr().cast_mut();
            strings[1] = gText_NoNeed.as_ptr().cast_mut();
        }
        APPRENTICE_ASK_YES_NO => {
            left = 20;
            top = 8;
            strings[0] = gText_Yes.as_ptr().cast_mut();
            strings[1] = gText_No.as_ptr().cast_mut();
        }
        _ => {
            left = 0;
            top = 0;
            return;
        }
    }
    pixelWidth = 0;
    i = 0;
    while i < count {
        let mut width: i32 = GetStringWidth(FONT_NORMAL, strings[i], 0);
        if width > pixelWidth {
            pixelWidth = width;
        }
        i += 1;
    }
    width = ConvertPixelWidthToTileWidth(pixelWidth) as u8;
    left = ScriptMenu_AdjustLeftCoordFromWidth(left as i32, width as i32) as u8;
    windowId = CreateAndShowWindow(left, top, width, count * 2);
    SetStandardWindowBorderStyle(windowId, FALSE);
    i = 0;
    while i < count {
        AddTextPrinterParameterized(
            windowId,
            FONT_NORMAL,
            strings[i],
            8,
            i * 16 + 1,
            TEXT_SKIP_DRAW,
            None,
        );
        i += 1;
    }
    InitMenuInUpperLeftCornerNormal(windowId, count, 0);
    CreateChooseAnswerTask(TRUE, count, windowId);
}
pub(crate) unsafe extern "C" fn Task_ChooseAnswer(taskId: u8) {
    let mut input: i8 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
pub(crate) unsafe extern "C" fn CreateAndShowWindow(
    left: u8,
    top: u8,
    width: u8,
    height: u8,
) -> u8 {
    let mut windowId: u8 = 0;
    let mut winTemplate: WindowTemplate = zeroed();
    winTemplate = CreateWindowTemplate(0, left + 1, top + 1, width, height, 15, 100);
    windowId = AddWindow(&raw mut winTemplate) as u8;
    PutWindowTilemap(windowId);
    CopyWindowToVram(windowId, COPYWIN_FULL);
    return windowId;
}
pub(crate) unsafe extern "C" fn RemoveAndHideWindow(windowId: u8) {
    ClearStdWindowAndFrameToTransparent(windowId, TRUE);
    RemoveWindow(windowId);
}
pub(crate) unsafe extern "C" fn CreateChooseAnswerTask(noBButton: u8, answers: u8, windowId: u8) {
    let mut taskId: u8 = CreateTask(Some(Task_ChooseAnswer), 80);
    gTasks[taskId].data[4] = noBButton as i16;
    if answers > 3 {
        gTasks[taskId].data[5] = TRUE as i16;
    } else {
        gTasks[taskId].data[5] = FALSE as i16;
    }
    gTasks[taskId].data[6] = windowId as i16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallApprenticeFunction() {
    sApprenticeFunctions[gSpecialVar_0x8004].unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn Script_ResetPlayerApprentice() {
    let mut i: u8 = 0;
    SetApprenticeId();
    (*gSaveBlock2Ptr).playerApprentice.set_lvlMode(0);
    (*gSaveBlock2Ptr).playerApprentice.set_questionsAnswered(0);
    (*gSaveBlock2Ptr).playerApprentice.set_leadMonId(0);
    (*gSaveBlock2Ptr).playerApprentice.set_party(0);
    i = 0;
    while i < MULTI_PARTY_SIZE as u8 {
        (*gSaveBlock2Ptr).playerApprentice.speciesIds[i] = 0;
        i += 1;
    }
    i = 0;
    while i < APPRENTICE_MAX_QUESTIONS {
        (*gSaveBlock2Ptr).playerApprentice.questions[i].set_questionId(0);
        (*gSaveBlock2Ptr).playerApprentice.questions[i].set_monId(0);
        (*gSaveBlock2Ptr).playerApprentice.questions[i].set_moveSlot(0);
        (*gSaveBlock2Ptr).playerApprentice.questions[i].set_suggestedChange(0);
        (*gSaveBlock2Ptr).playerApprentice.questions[i].data = 0;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn Script_GivenApprenticeLvlMode() {
    if GivenApprenticeLvlMode() == 0 {
        gSpecialVar_Result = FALSE as u16;
    } else {
        gSpecialVar_Result = TRUE as u16;
    }
}
pub(crate) unsafe extern "C" fn Script_SetApprenticeLvlMode() {
    SetPlayersApprenticeLvlMode(gSpecialVar_0x8005 as u8);
}
pub(crate) unsafe extern "C" fn Script_SetApprenticeId() {
    SetApprenticeId();
}
pub(crate) unsafe extern "C" fn Script_SetRandomQuestionData() {
    SetRandomQuestionData();
}
pub(crate) unsafe extern "C" fn IncrementQuestionsAnswered() {
    (*gSaveBlock2Ptr)
        .playerApprentice
        .set_questionsAnswered((*gSaveBlock2Ptr).playerApprentice.questionsAnswered() + 1);
}
pub(crate) unsafe extern "C" fn GetNumApprenticePartyMonsAssigned() {
    gSpecialVar_Result = (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as u16;
}
pub(crate) unsafe extern "C" fn IsFinalQuestion() {
    let mut questionNum: i32 = (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3;
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
pub(crate) unsafe extern "C" fn Script_CreateApprenticeMenu() {
    CreateApprenticeMenu(gSpecialVar_0x8005 as u8);
}
pub(crate) unsafe extern "C" fn Task_WaitForPrintingMessage(taskId: u8) {
    if RunTextPrintersAndIsPrinter0Active() == 0 {
        DestroyTask(taskId);
        if gSpecialVar_0x8005 != 0 {
            ExecuteFuncAfterButtonPress(Some(ScriptContext_Enable));
        } else {
            ScriptContext_Enable();
        }
    }
}
pub(crate) unsafe extern "C" fn PrintApprenticeMessage() {
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
pub(crate) unsafe extern "C" fn Script_PrintApprenticeMessage() {
    LockPlayerFieldControls();
    FreezeObjectEvents();
    PlayerFreeze();
    StopPlayerAvatar();
    DrawDialogueFrame(0, TRUE);
    PrintApprenticeMessage();
}
pub(crate) unsafe extern "C" fn ApprenticeGetQuestion() {
    if (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() < NUM_WHICH_MON_QUESTIONS {
        gSpecialVar_Result = APPRENTICE_QUESTION_WHICH_MON;
    } else if (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() > 11 {
        gSpecialVar_Result = APPRENTICE_QUESTION_WIN_SPEECH;
    } else {
        let mut id: i32 = (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3;
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
pub(crate) unsafe extern "C" fn SetApprenticePartyMon() {
    if gSpecialVar_0x8005 != 0 {
        let mut partySlot: u8 = gSpecialVar_0x8006 as u8;
        (*gSaveBlock2Ptr).playerApprentice.set_party(
            (*gSaveBlock2Ptr).playerApprentice.party() | shl_i32(1, partySlot as u32) as u8,
        );
    }
}
pub(crate) unsafe extern "C" fn SetApprenticeMonMove() {
    if (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() >= NUM_WHICH_MON_QUESTIONS {
        let mut id: u8 = (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() - 3;
        if gSpecialVar_0x8005 != 0 {
            (*gSaveBlock2Ptr).playerApprentice.questions[id].set_suggestedChange(TRUE);
        } else {
            (*gSaveBlock2Ptr).playerApprentice.questions[id].set_suggestedChange(FALSE);
        }
    }
}
pub(crate) unsafe extern "C" fn InitQuestionData() {
    let mut i: u8 = 0;
    let mut count: u8 = 0;
    let mut id1: u8 = 0;
    let mut id2: u8 = 0;
    i = 0;
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
    } else if gSpecialVar_0x8005 == APPRENTICE_QUESTION_WHAT_ITEM {
        if (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() >= NUM_WHICH_MON_QUESTIONS
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
}
pub(crate) unsafe extern "C" fn FreeQuestionData() {
    Free(gApprenticeQuestionData as *mut c_void);
    gApprenticeQuestionData = null_mut();
}
pub(crate) unsafe extern "C" fn ApprenticeBufferString() {
    let mut stringDst: *mut u8 = null_mut();
    let mut text: CArray<u8, 16> = zeroed();
    let mut speciesArrayId: u32 = 0;
    match gSpecialVar_0x8005 {
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
    match gSpecialVar_0x8006 {
        APPRENTICE_BUFF_SPECIES1 => {
            StringCopy(
                stringDst,
                gSpeciesNames[(*gApprenticeQuestionData).speciesId]
                    .as_ptr()
                    .cast_mut(),
            );
        }
        APPRENTICE_BUFF_SPECIES2 => {
            StringCopy(
                stringDst,
                gSpeciesNames[(*gApprenticeQuestionData).altSpeciesId]
                    .as_ptr()
                    .cast_mut(),
            );
        }
        APPRENTICE_BUFF_SPECIES3 => {
            StringCopy(
                stringDst,
                gSpeciesNames[(*gApprenticeQuestionData).speciesId]
                    .as_ptr()
                    .cast_mut(),
            );
        }
        APPRENTICE_BUFF_MOVE1 => {
            StringCopy(
                stringDst,
                gMoveNames[(*gApprenticeQuestionData).move1]
                    .as_ptr()
                    .cast_mut(),
            );
        }
        APPRENTICE_BUFF_MOVE2 => {
            StringCopy(
                stringDst,
                gMoveNames[(*gApprenticeQuestionData).move2]
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
                StringCopy(stringDst, gText_Lv50.as_ptr().cast_mut());
            } else {
                StringCopy(stringDst, gText_OpenLevel.as_ptr().cast_mut());
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
                gSpeciesNames
                    [gApprentices[(*gSaveBlock2Ptr).playerApprentice.id].species[speciesArrayId]]
                    .as_ptr()
                    .cast_mut(),
            );
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SetLeadApprenticeMon() {
    (*gSaveBlock2Ptr)
        .playerApprentice
        .set_leadMonId(gSpecialVar_0x8005 as u8);
}
pub(crate) unsafe extern "C" fn Script_ApprenticeOpenBagMenu() {
    ApprenticeOpenBagMenu();
}
pub(crate) unsafe extern "C" fn TrySetApprenticeHeldItem() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut count: u8 = 0;
    if (*gSaveBlock2Ptr).playerApprentice.questionsAnswered() < NUM_WHICH_MON_QUESTIONS {
        return;
    }
    count = 0;
    j = 0;
    while j < APPRENTICE_MAX_QUESTIONS {
        if (*gSaveBlock2Ptr).playerApprentice.questions[j].questionId() == QUESTION_ID_WIN_SPEECH {
            break;
        }
        count += 1;
        j += 1;
    }
    i = 0;
    'l3: while i < count {
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
                    .data = gSpecialVar_0x8005;
                gSpecialVar_Result = FALSE as u16;
                return;
            }
        }
        i += 1;
    }
    (*gSaveBlock2Ptr).playerApprentice.questions
        [(*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3]
        .set_suggestedChange(TRUE);
    (*gSaveBlock2Ptr).playerApprentice.questions
        [(*gSaveBlock2Ptr).playerApprentice.questionsAnswered() as i32 - 3]
        .data = gSpecialVar_0x8005;
    gSpecialVar_Result = TRUE as u16;
}
pub(crate) unsafe extern "C" fn ShiftSavedApprentices() {
    let mut i: i32 = 0;
    let mut apprenticeNum: i32 = 0;
    let mut apprenticeIdx: i32 = 0;
    if (*gSaveBlock2Ptr).apprentices[0].playerName[0] == EOS {
        return;
    }
    i = 0;
    while i < 3 {
        if (*gSaveBlock2Ptr).apprentices[i + 1].playerName[0] == EOS {
            (*gSaveBlock2Ptr).apprentices[i + 1] = (*gSaveBlock2Ptr).apprentices[0];
            return;
        }
        i += 1;
    }
    apprenticeNum = 0xFFFF;
    apprenticeIdx = -1;
    i = 1;
    while i < APPRENTICE_COUNT {
        if GetTrainerId((*gSaveBlock2Ptr).apprentices[i].playerId.as_mut_ptr())
            == GetTrainerId((*gSaveBlock2Ptr).playerTrainerId.as_mut_ptr())
            && ((*gSaveBlock2Ptr).apprentices[i].number as i32) < apprenticeNum
        {
            apprenticeNum = (*gSaveBlock2Ptr).apprentices[i].number as i32;
            apprenticeIdx = i;
        }
        i += 1;
    }
    if apprenticeIdx > 0 {
        (*gSaveBlock2Ptr).apprentices[apprenticeIdx] = (*gSaveBlock2Ptr).apprentices[0];
    }
}
pub(crate) unsafe extern "C" fn SaveApprentice() {
    let mut i: u8 = 0;
    (*gSaveBlock2Ptr).apprentices[0].set_id((*gSaveBlock2Ptr).playerApprentice.id);
    (*gSaveBlock2Ptr).apprentices[0].set_lvlMode((*gSaveBlock2Ptr).playerApprentice.lvlMode());
    i = 0;
    while i < APPRENTICE_MAX_QUESTIONS
        && (*gSaveBlock2Ptr).playerApprentice.questions[i].questionId() != 0
    {
        i += 1;
    }
    (*gSaveBlock2Ptr).apprentices[0].numQuestions = i;
    if (*gSaveBlock2Ptr).apprentices[0].number < 255 {
        (*gSaveBlock2Ptr).apprentices[0].number += 1;
    }
    SaveApprenticeParty((*gSaveBlock2Ptr).apprentices[0].numQuestions);
    i = 0;
    while i < TRAINER_ID_LENGTH {
        (*gSaveBlock2Ptr).apprentices[0].playerId[i] = (*gSaveBlock2Ptr).playerTrainerId[i];
        i += 1;
    }
    StringCopy(
        (*gSaveBlock2Ptr).apprentices[0].playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
    );
    (*gSaveBlock2Ptr).apprentices[0].language = gGameLanguage;
    CalcApprenticeChecksum(&raw mut (*gSaveBlock2Ptr).apprentices[0]);
}
pub(crate) unsafe extern "C" fn SetSavedApprenticeTrainerGfxId() {
    let mut i: u8 = 0;
    let mut objectEventGfxId: u8 = 0;
    let mut class: u8 = gApprentices[(*gSaveBlock2Ptr).apprentices[0].id()].facilityClass;
    i = 0;
    while i < 30 && gTowerMaleFacilityClasses[i] != class {
        i += 1;
    }
    if i != 30 {
        objectEventGfxId = gTowerMaleTrainerGfxIds[i];
        VarSet(VAR_OBJ_GFX_ID_0, objectEventGfxId as u16);
        return;
    }
    i = 0;
    while i < 20 && gTowerFemaleFacilityClasses[i] != class {
        i += 1;
    }
    if i != 20 {
        objectEventGfxId = gTowerFemaleTrainerGfxIds[i];
        VarSet(VAR_OBJ_GFX_ID_0, objectEventGfxId as u16);
    }
}
pub(crate) unsafe extern "C" fn SetPlayerApprenticeTrainerGfxId() {
    let mut i: u8 = 0;
    let mut objectEventGfxId: u8 = 0;
    let mut class: u8 = gApprentices[(*gSaveBlock2Ptr).playerApprentice.id].facilityClass;
    i = 0;
    while i < 30 && gTowerMaleFacilityClasses[i] != class {
        i += 1;
    }
    if i != 30 {
        objectEventGfxId = gTowerMaleTrainerGfxIds[i];
        VarSet(VAR_OBJ_GFX_ID_0, objectEventGfxId as u16);
        return;
    }
    i = 0;
    while i < 20 && gTowerFemaleFacilityClasses[i] != class {
        i += 1;
    }
    if i != 20 {
        objectEventGfxId = gTowerFemaleTrainerGfxIds[i];
        VarSet(VAR_OBJ_GFX_ID_0, objectEventGfxId as u16);
    }
}
pub(crate) unsafe extern "C" fn GetShouldCheckApprenticeGone() {
    gSpecialVar_0x8004 = TRUE as u16;
}
pub(crate) unsafe extern "C" fn GetShouldApprenticeLeave() {
    gSpecialVar_0x8004 = TRUE as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetApprenticeNameInLanguage(apprenticeId: u32, language: i32) -> *mut u8 {
    let mut apprentice: *mut ApprenticeTrainer = (&raw const gApprentices[apprenticeId]).cast_mut();
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
        return null_mut();
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchToFollowupFuncAfterButtonPress(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
        SwitchTaskToFollowupFunc(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_ExecuteFuncAfterButtonPress(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
        gApprenticeFunc = core::mem::transmute::<_, Option<unsafe extern "C" fn()>>(
            (gTasks[taskId].data[0] as u16 as u32 | (gTasks[taskId].data[1] as u32) << 16) as usize
                as *mut c_void,
        );
        gApprenticeFunc.unwrap_unchecked()();
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn ExecuteFuncAfterButtonPress(func: Option<unsafe extern "C" fn()>) {
    let mut taskId: u8 = CreateTask(Some(Task_ExecuteFuncAfterButtonPress), 1);
    gTasks[taskId].data[0] = core::mem::transmute::<_, usize>(func) as u32 as i16;
    gTasks[taskId].data[1] = (core::mem::transmute::<_, usize>(func) as u32 >> 16) as i16;
}
pub(crate) unsafe extern "C" fn ExecuteFollowupFuncAfterButtonPress(
    task: Option<unsafe extern "C" fn(u8)>,
) {
    let mut taskId: u8 = CreateTask(Some(Task_SwitchToFollowupFuncAfterButtonPress), 1);
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_SwitchToFollowupFuncAfterButtonPress),
        task,
    );
}
