//! Translated from `src/mystery_gift.c` by tools/rustport/c2rs.py.
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
    unused_assignments,
    unused_variables
)]

use crate::battle_tower::ClearEReaderTrainer;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::easy_chat::InitQuestionnaireWords;
use crate::event_data::{ClearMysteryGiftFlags, ClearMysteryGiftVars, FlagGet};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::new_game::CopyTrainerId;
use crate::script::{ClearRamScript, ValidateSavedRamScript};
use crate::string_util::StringCopy;
#[allow(unused_imports)]
use crate::types::*;
use crate::util::CalcCRC16WithTable;
use crate::wonder_news::WonderNews_Reset;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): sReceivedGiftFlags

const GAME_DATA_VALID_GIFT_TYPE_1: u16 = 4;
const GAME_DATA_VALID_GIFT_TYPE_2: u32 = 512;
const GAME_DATA_VALID_VAR: u32 = 257;

static sReceivedGiftFlags: Table<CArray<u16, 20>> =
    Table((&raw const crate::data::mystery_gift::sReceivedGiftFlags).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static sStatsEnabled: crate::global::Global<u32> = crate::global::Global::new(0);

unsafe extern "C" {
    static RomHeaderGameCode: CArray<u8, 4>;
    static RomHeaderSoftwareVersion: u8;
}
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

#[unsafe(no_mangle)]
pub unsafe fn ClearMysteryGift() {
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                &raw mut (*gSaveBlock1Ptr).mysteryGift as *mut c_void,
                0x50000db,
            );
        }
    }
    ClearSavedWonderNewsMetadata();
    InitQuestionnaireWords();
}
#[unsafe(no_mangle)]
pub unsafe fn GetSavedWonderNews() -> *mut WonderNews {
    &raw mut (*gSaveBlock1Ptr).mysteryGift.news
}
#[unsafe(no_mangle)]
pub unsafe fn GetSavedWonderCard() -> *mut WonderCard {
    &raw mut (*gSaveBlock1Ptr).mysteryGift.card
}
pub unsafe fn GetSavedWonderCardMetadata() -> *mut WonderCardMetadata {
    &raw mut (*gSaveBlock1Ptr).mysteryGift.cardMetadata
}
#[unsafe(no_mangle)]
pub unsafe fn GetSavedWonderNewsMetadata() -> *mut WonderNewsMetadata {
    &raw mut (*gSaveBlock1Ptr).mysteryGift.newsMetadata
}
pub unsafe fn GetQuestionnaireWordsPtr() -> *mut u16 {
    (*gSaveBlock1Ptr)
        .mysteryGift
        .questionnaireWords
        .as_mut_ptr()
}
pub unsafe fn ClearSavedWonderNewsAndRelated() {
    ClearSavedWonderNews();
}
#[unsafe(no_mangle)]
pub unsafe fn SaveWonderNews(news: *mut WonderNews) -> u32 {
    if ValidateWonderNews(news) == 0 {
        return FALSE as u32;
    }
    ClearSavedWonderNews();
    (*gSaveBlock1Ptr).mysteryGift.news = *news;
    (*gSaveBlock1Ptr).mysteryGift.newsCrc = CalcCRC16WithTable(
        &raw mut (*gSaveBlock1Ptr).mysteryGift.news as *mut c_void as *mut u8,
        444,
    ) as u32;
    TRUE as u32
}
#[unsafe(no_mangle)]
pub unsafe fn ValidateSavedWonderNews() -> u32 {
    if CalcCRC16WithTable(
        &raw mut (*gSaveBlock1Ptr).mysteryGift.news as *mut c_void as *mut u8,
        444,
    ) as u32
        != (*gSaveBlock1Ptr).mysteryGift.newsCrc
    {
        return FALSE as u32;
    }
    if ValidateWonderNews(&raw mut (*gSaveBlock1Ptr).mysteryGift.news) == 0 {
        return FALSE as u32;
    }
    TRUE as u32
}
unsafe fn ValidateWonderNews(news: *mut WonderNews) -> u32 {
    if (*news).id == 0 {
        return FALSE as u32;
    }
    TRUE as u32
}
pub unsafe fn IsSendingSavedWonderNewsAllowed() -> u32 {
    let news: *mut WonderNews = &raw mut (*gSaveBlock1Ptr).mysteryGift.news;
    if (*news).sendType == SEND_TYPE_DISALLOWED {
        return FALSE as u32;
    }
    TRUE as u32
}
unsafe fn ClearSavedWonderNews() {
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                GetSavedWonderNews() as *mut c_void,
                0x500006f,
            );
        }
    }
    (*gSaveBlock1Ptr).mysteryGift.newsCrc = 0;
}
unsafe fn ClearSavedWonderNewsMetadata() {
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                GetSavedWonderNewsMetadata() as *mut c_void,
                0x5000001,
            );
        }
    }
    WonderNews_Reset();
}
#[unsafe(no_mangle)]
pub unsafe fn IsWonderNewsSameAsSaved(news: *mut u8) -> u32 {
    let savedNews: *mut u8 = &raw mut (*gSaveBlock1Ptr).mysteryGift.news as *mut u8;
    if ValidateSavedWonderNews() == 0 {
        return FALSE as u32;
    }
    for i in 0..444u32 {
        if *savedNews.at(i) != *news.at(i) {
            return FALSE as u32;
        }
    }
    TRUE as u32
}
pub unsafe fn ClearSavedWonderCardAndRelated() {
    ClearSavedWonderCard();
    ClearSavedWonderCardMetadata();
    ClearSavedTrainerIds();
    ClearRamScript();
    ClearMysteryGiftFlags();
    ClearMysteryGiftVars();
    ClearEReaderTrainer(&raw mut (*gSaveBlock2Ptr).frontier.ereaderTrainer);
}
#[unsafe(no_mangle)]
pub unsafe fn SaveWonderCard(card: *mut WonderCard) -> u32 {
    if ValidateWonderCard(card) == 0 {
        return FALSE as u32;
    }
    ClearSavedWonderCardAndRelated();
    memcpy(
        &raw mut (*gSaveBlock1Ptr).mysteryGift.card as *mut u8,
        card as *mut u8,
        332,
    );
    (*gSaveBlock1Ptr).mysteryGift.cardCrc = CalcCRC16WithTable(
        &raw mut (*gSaveBlock1Ptr).mysteryGift.card as *mut c_void as *mut u8,
        332,
    ) as u32;
    let metadata: *mut WonderCardMetadata = &raw mut (*gSaveBlock1Ptr).mysteryGift.cardMetadata;
    (*metadata).iconSpecies = (*gSaveBlock1Ptr).mysteryGift.card.iconSpecies;
    TRUE as u32
}
#[unsafe(no_mangle)]
pub unsafe fn ValidateSavedWonderCard() -> u32 {
    if (*gSaveBlock1Ptr).mysteryGift.cardCrc
        != CalcCRC16WithTable(
            &raw mut (*gSaveBlock1Ptr).mysteryGift.card as *mut c_void as *mut u8,
            332,
        ) as u32
    {
        return FALSE as u32;
    }
    if ValidateWonderCard(&raw mut (*gSaveBlock1Ptr).mysteryGift.card) == 0 {
        return FALSE as u32;
    }
    if ValidateSavedRamScript() == 0 {
        return FALSE as u32;
    }
    TRUE as u32
}
unsafe fn ValidateWonderCard(card: *mut WonderCard) -> u32 {
    if (*card).flagId == 0 {
        return FALSE as u32;
    }
    if (*card).r#type() >= CARD_TYPE_COUNT {
        return FALSE as u32;
    }
    if !((*card).sendType() == SEND_TYPE_DISALLOWED
        || (*card).sendType() == SEND_TYPE_ALLOWED
        || (*card).sendType() == SEND_TYPE_ALLOWED_ALWAYS)
    {
        return FALSE as u32;
    }
    if (*card).bgType() >= NUM_WONDER_BGS {
        return FALSE as u32;
    }
    if (*card).maxStamps > MAX_STAMP_CARD_STAMPS {
        return FALSE as u32;
    }
    TRUE as u32
}
pub unsafe fn IsSendingSavedWonderCardAllowed() -> u32 {
    let card: *mut WonderCard = &raw mut (*gSaveBlock1Ptr).mysteryGift.card;
    if (*card).sendType() == SEND_TYPE_DISALLOWED {
        return FALSE as u32;
    }
    TRUE as u32
}
unsafe fn ClearSavedWonderCard() {
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                &raw mut (*gSaveBlock1Ptr).mysteryGift.card as *mut c_void,
                0x5000053,
            );
        }
    }
    (*gSaveBlock1Ptr).mysteryGift.cardCrc = 0;
}
unsafe fn ClearSavedWonderCardMetadata() {
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                GetSavedWonderCardMetadata() as *mut c_void,
                0x5000009,
            );
        }
    }
    (*gSaveBlock1Ptr).mysteryGift.cardMetadataCrc = 0;
}
pub unsafe fn GetWonderCardFlagID() -> u16 {
    if ValidateSavedWonderCard() != 0 {
        return (*gSaveBlock1Ptr).mysteryGift.card.flagId;
    }
    0
}
#[unsafe(no_mangle)]
pub unsafe fn DisableWonderCardSending(card: *mut WonderCard) {
    if (*card).sendType() == SEND_TYPE_ALLOWED {
        (*card).set_sendType(SEND_TYPE_DISALLOWED);
    }
}
fn IsWonderCardFlagIDInValidRange(flagId: u16) -> u32 {
    if (WONDER_CARD_FLAG_OFFSET..1020).contains(&flagId) {
        return TRUE as u32;
    }
    FALSE as u32
}
pub unsafe fn IsSavedWonderCardGiftNotReceived() -> u32 {
    let value: u16 = GetWonderCardFlagID();
    if IsWonderCardFlagIDInValidRange(value) == 0 {
        return FALSE as u32;
    }
    if FlagGet(sReceivedGiftFlags[value as i32 - WONDER_CARD_FLAG_OFFSET as i32]) == TRUE {
        return FALSE as u32;
    }
    TRUE as u32
}
unsafe fn GetNumStampsInMetadata(data: *mut WonderCardMetadata, size: i32) -> i32 {
    let mut numStamps: i32 = 0;
    for i in 0..size {
        if (*data).stampData[1][i] != 0 && (*data).stampData[0][i] != 0 {
            numStamps += 1;
        }
    }
    numStamps
}
unsafe fn IsStampInMetadata(
    metadata: *mut WonderCardMetadata,
    stamp: *mut u16,
    maxStamps: i32,
) -> u32 {
    for i in 0..maxStamps {
        if (*metadata).stampData[1][i] == *stamp.at(1) {
            return TRUE as u32;
        }
        if (*metadata).stampData[0][i] == *stamp {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
unsafe fn ValidateStamp(stamp: *mut u16) -> u32 {
    if *stamp.at(1) == 0 {
        return FALSE as u32;
    }
    if *stamp == 0 {
        return FALSE as u32;
    }
    if *stamp >= NUM_SPECIES {
        return FALSE as u32;
    }
    TRUE as u32
}
unsafe fn GetNumStampsInSavedCard() -> i32 {
    let mut card: *mut WonderCard = null_mut();
    if ValidateSavedWonderCard() == 0 {
        return 0;
    }
    card = &raw mut (*gSaveBlock1Ptr).mysteryGift.card;
    if (*card).r#type() != CARD_TYPE_STAMP {
        return 0;
    }
    GetNumStampsInMetadata(
        &raw mut (*gSaveBlock1Ptr).mysteryGift.cardMetadata,
        (*card).maxStamps as i32,
    )
}
#[unsafe(no_mangle)]
pub unsafe fn MysteryGift_TrySaveStamp(stamp: *mut u16) -> u32 {
    let card: *mut WonderCard = &raw mut (*gSaveBlock1Ptr).mysteryGift.card;
    let maxStamps: i32 = (*card).maxStamps as i32;
    if ValidateStamp(stamp) == 0 {
        return FALSE as u32;
    }
    if IsStampInMetadata(
        &raw mut (*gSaveBlock1Ptr).mysteryGift.cardMetadata,
        stamp,
        maxStamps,
    ) != 0
    {
        return FALSE as u32;
    }
    for i in 0..maxStamps {
        if (*gSaveBlock1Ptr).mysteryGift.cardMetadata.stampData[1][i] == 0
            && (*gSaveBlock1Ptr).mysteryGift.cardMetadata.stampData[0][i] == 0
        {
            (*gSaveBlock1Ptr).mysteryGift.cardMetadata.stampData[1][i] = *stamp.at(1);
            (*gSaveBlock1Ptr).mysteryGift.cardMetadata.stampData[0][i] = *stamp;
            return TRUE as u32;
        }
    }
    FALSE as u32
}
#[unsafe(no_mangle)]
pub unsafe fn MysteryGift_LoadLinkGameData(data: *mut MysteryGiftLinkGameData, isWonderNews: u32) {
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(&raw mut tmp as *mut c_void, data as *mut c_void, 0x5000019);
        }
    }
    (*data).validationVar = GAME_DATA_VALID_VAR;
    (*data).validationFlag1 = 1;
    (*data).validationFlag2 = 1;
    if isWonderNews != 0 {
        (*data).validationGiftType1 = 5;
        (*data).validationGiftType2 = 513;
    } else {
        (*data).validationGiftType1 = GAME_DATA_VALID_GIFT_TYPE_1;
        (*data).validationGiftType2 = GAME_DATA_VALID_GIFT_TYPE_2;
    }
    if ValidateSavedWonderCard() != 0 {
        (*data).flagId = (*GetSavedWonderCard()).flagId;
        (*data).cardMetadata = *GetSavedWonderCardMetadata();
        (*data).maxStamps = (*GetSavedWonderCard()).maxStamps;
    } else {
        (*data).flagId = 0;
    }
    let mut i: i32 = 0;
    while i < NUM_QUESTIONNAIRE_WORDS {
        (*data).questionnaireWords[i] = (*gSaveBlock1Ptr).mysteryGift.questionnaireWords[i];
        i += 1;
    }
    CopyTrainerId(
        (*data).playerTrainerId.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerTrainerId.as_mut_ptr(),
    );
    StringCopy(
        (*data).playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
    );
    for i in 0..EASY_CHAT_BATTLE_WORDS_COUNT {
        (*data).easyChatProfile[i] = (*gSaveBlock1Ptr).easyChatProfile[i];
    }
    memcpy(
        (*data).romHeaderGameCode.as_mut_ptr(),
        RomHeaderGameCode.as_ptr().cast_mut(),
        GAME_CODE_LENGTH,
    );
    (*data).romHeaderSoftwareVersion = RomHeaderSoftwareVersion;
}
#[unsafe(no_mangle)]
pub unsafe fn MysteryGift_ValidateLinkGameData(
    data: *mut MysteryGiftLinkGameData,
    isWonderNews: u32,
) -> u32 {
    if (*data).validationVar != GAME_DATA_VALID_VAR {
        return FALSE as u32;
    }
    if (*data).validationFlag1 as i32 & 1 == 0 {
        return FALSE as u32;
    }
    if (*data).validationFlag2 & 1 == 0 {
        return FALSE as u32;
    }
    if isWonderNews == 0 {
        if (*data).validationGiftType1 as i32 & GAME_DATA_VALID_GIFT_TYPE_1 as i32 == 0 {
            return FALSE as u32;
        }
        if (*data).validationGiftType2 & 896 == 0 {
            return FALSE as u32;
        }
    }
    TRUE as u32
}
#[unsafe(no_mangle)]
pub unsafe fn MysteryGift_CompareCardFlags(
    flagId: *mut u16,
    data: *mut MysteryGiftLinkGameData,
    unused: *mut c_void,
) -> u32 {
    if (*data).flagId == 0 {
        return HAS_NO_CARD;
    }
    if *flagId == (*data).flagId {
        return HAS_SAME_CARD;
    }
    HAS_DIFF_CARD
}
#[unsafe(no_mangle)]
pub unsafe fn MysteryGift_CheckStamps(
    stamp: *mut u16,
    data: *mut MysteryGiftLinkGameData,
    unused: *mut c_void,
) -> u32 {
    let stampsMissing: i32 = (*data).maxStamps as i32
        - GetNumStampsInMetadata(&raw mut (*data).cardMetadata, (*data).maxStamps as i32);
    if stampsMissing == 0 {
        return 1;
    }
    if IsStampInMetadata(
        &raw mut (*data).cardMetadata,
        stamp,
        (*data).maxStamps as i32,
    ) != 0
    {
        return 3;
    }
    if stampsMissing == 1 {
        return 4;
    }
    2
}
#[unsafe(no_mangle)]
pub unsafe fn MysteryGift_DoesQuestionnaireMatch(
    data: *mut MysteryGiftLinkGameData,
    words: *mut u16,
) -> u32 {
    for i in 0..NUM_QUESTIONNAIRE_WORDS {
        if (*data).questionnaireWords[i] != *words.at(i) {
            return FALSE as u32;
        }
    }
    TRUE as u32
}
unsafe fn GetNumStampsInLinkData(data: *mut MysteryGiftLinkGameData) -> i32 {
    GetNumStampsInMetadata(&raw mut (*data).cardMetadata, (*data).maxStamps as i32)
}
#[unsafe(no_mangle)]
pub unsafe fn MysteryGift_GetCardStatFromLinkData(
    data: *mut MysteryGiftLinkGameData,
    stat: u32,
) -> u16 {
    match stat {
        CARD_STAT_BATTLES_WON => {
            return (*data).cardMetadata.battlesWon;
        }
        CARD_STAT_BATTLES_LOST => {
            return (*data).cardMetadata.battlesLost;
        }
        CARD_STAT_NUM_TRADES => {
            return (*data).cardMetadata.numTrades;
        }
        CARD_STAT_NUM_STAMPS => {
            return GetNumStampsInLinkData(data) as u16;
        }
        CARD_STAT_MAX_STAMPS => {
            return (*data).maxStamps as u16;
        }
        _ => {
            return 0;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn IncrementCardStat(statType: u32) {
    let card: *mut WonderCard = &raw mut (*gSaveBlock1Ptr).mysteryGift.card;
    if (*card).r#type() == CARD_TYPE_LINK_STAT {
        let mut stat: *mut u16 = null_mut();
        match statType {
            CARD_STAT_BATTLES_WON => {
                stat = &raw mut (*gSaveBlock1Ptr).mysteryGift.cardMetadata.battlesWon;
            }
            CARD_STAT_BATTLES_LOST => {
                stat = &raw mut (*gSaveBlock1Ptr).mysteryGift.cardMetadata.battlesLost;
            }
            CARD_STAT_NUM_TRADES => {
                stat = &raw mut (*gSaveBlock1Ptr).mysteryGift.cardMetadata.numTrades;
            }
            CARD_STAT_NUM_STAMPS | CARD_STAT_MAX_STAMPS => {}
            _ => {}
        }
        if stat.is_null() {
        } else if ({
            *stat += 1;
            *stat
        }) > MAX_WONDER_CARD_STAT
        {
            *stat = MAX_WONDER_CARD_STAT;
        }
    }
}
pub unsafe fn MysteryGift_GetCardStat(stat: u32) -> u16 {
    'l1: {
        match stat {
            CARD_STAT_BATTLES_WON => {
                let card: *mut WonderCard = &raw mut (*gSaveBlock1Ptr).mysteryGift.card;
                if (*card).r#type() == CARD_TYPE_LINK_STAT {
                    let metadata: *mut WonderCardMetadata =
                        &raw mut (*gSaveBlock1Ptr).mysteryGift.cardMetadata;
                    return (*metadata).battlesWon;
                }
                break 'l1;
            }
            CARD_STAT_BATTLES_LOST => {
                let card: *mut WonderCard = &raw mut (*gSaveBlock1Ptr).mysteryGift.card;
                if (*card).r#type() == CARD_TYPE_LINK_STAT {
                    let metadata: *mut WonderCardMetadata =
                        &raw mut (*gSaveBlock1Ptr).mysteryGift.cardMetadata;
                    return (*metadata).battlesLost;
                }
                break 'l1;
            }
            CARD_STAT_NUM_TRADES => {
                let card: *mut WonderCard = &raw mut (*gSaveBlock1Ptr).mysteryGift.card;
                if (*card).r#type() == CARD_TYPE_LINK_STAT {
                    let metadata: *mut WonderCardMetadata =
                        &raw mut (*gSaveBlock1Ptr).mysteryGift.cardMetadata;
                    return (*metadata).numTrades;
                }
                break 'l1;
            }
            CARD_STAT_NUM_STAMPS => {
                let card: *mut WonderCard = &raw mut (*gSaveBlock1Ptr).mysteryGift.card;
                if (*card).r#type() == CARD_TYPE_STAMP {
                    return GetNumStampsInSavedCard() as u16;
                }
                break 'l1;
            }
            CARD_STAT_MAX_STAMPS => {
                let card: *mut WonderCard = &raw mut (*gSaveBlock1Ptr).mysteryGift.card;
                if (*card).r#type() == CARD_TYPE_STAMP {
                    return (*card).maxStamps as u16;
                }
                break 'l1;
            }
            _ => {}
        }
    }
    0
}
pub fn MysteryGift_DisableStats() {
    sStatsEnabled.set(FALSE as u32);
}
pub unsafe fn MysteryGift_TryEnableStatsByFlagId(flagId: u16) -> u32 {
    sStatsEnabled.set(FALSE as u32);
    if flagId == 0 {
        return FALSE as u32;
    }
    if ValidateSavedWonderCard() == 0 {
        return FALSE as u32;
    }
    if (*gSaveBlock1Ptr).mysteryGift.card.flagId != flagId {
        return FALSE as u32;
    }
    sStatsEnabled.set(TRUE as u32);
    TRUE as u32
}
pub unsafe fn MysteryGift_TryIncrementStat(stat: u32, trainerId: u32) {
    if sStatsEnabled.get() != 0 {
        match stat {
            CARD_STAT_NUM_TRADES => {
                IncrementCardStatForNewTrainer(
                    CARD_STAT_NUM_TRADES,
                    trainerId,
                    (*gSaveBlock1Ptr).mysteryGift.trainerIds[1].as_mut_ptr(),
                    5,
                );
            }
            CARD_STAT_BATTLES_WON => {
                IncrementCardStatForNewTrainer(
                    CARD_STAT_BATTLES_WON,
                    trainerId,
                    (*gSaveBlock1Ptr).mysteryGift.trainerIds[0].as_mut_ptr(),
                    5,
                );
            }
            CARD_STAT_BATTLES_LOST => {
                IncrementCardStatForNewTrainer(
                    CARD_STAT_BATTLES_LOST,
                    trainerId,
                    (*gSaveBlock1Ptr).mysteryGift.trainerIds[0].as_mut_ptr(),
                    5,
                );
            }
            _ => {}
        }
    }
}
unsafe fn ClearSavedTrainerIds() {
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (*gSaveBlock1Ptr).mysteryGift.trainerIds.as_mut_ptr() as *mut c_void,
                0x500000a,
            );
        }
    }
}
unsafe fn RecordTrainerId(trainerId: u32, trainerIds: *mut u32, size: i32) -> u32 {
    let mut j: i32 = 0;
    let mut i: i32 = 0;
    while i < size {
        if *trainerIds.at(i) == trainerId {
            break;
        }
        i += 1;
    }
    if i == size {
        j = size - 1;
        while j > 0 {
            *trainerIds.at(j) = *trainerIds.at(j - 1);
            j -= 1;
        }
        *trainerIds = trainerId;
        return TRUE as u32;
    } else {
        j = i;
        while j > 0 {
            *trainerIds.at(j) = *trainerIds.at(j - 1);
            j -= 1;
        }
        *trainerIds = trainerId;
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn IncrementCardStatForNewTrainer(
    stat: u32,
    trainerId: u32,
    trainerIds: *mut u32,
    size: i32,
) {
    if RecordTrainerId(trainerId, trainerIds, size) != 0 {
        IncrementCardStat(stat);
    }
}
