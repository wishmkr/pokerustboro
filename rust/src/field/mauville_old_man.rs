//! Translated from `src/mauville_old_man.c` by tools/rustport/c2rs.py.
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
    clippy::missing_transmute_annotations,
    clippy::while_immutable_condition,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gGameLanguage;
use crate::bard_music::CalcWordSounds;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::easy_chat::{
    CopyEasyChatWord, EasyChat_GetNumWordsInGroup, GetRandomEasyChatWordFromUnlockedGroup,
    UnlockRandomTrendySaying,
};
use crate::event_data::VarSet;
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_Result};
use crate::field_message_box::ShowFieldMessage;
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::m4a::{
    gMPlayInfo_SE2, m4aMPlayFadeOutTemporarily, m4aMPlayPitchControl, m4aMPlayStop,
    m4aMPlayVolumeControl, m4aSongNumStart,
};
use crate::menu::{
    DrawDialogueFrame, InitMenuInUpperLeftCornerNormal, Menu_ProcessInput,
    RunTextPrintersAndIsPrinter0Active, SetStandardWindowBorderStyle,
};
use crate::overworld::GetGameStat;
use crate::random::Random;
use crate::script::{ScriptContext_Enable, ScriptContext_Stop};
use crate::script_menu::{
    ClearToTransparentAndRemoveWindow, ConvertPixelWidthToTileWidth, CreateWindowFromRect,
};
use crate::sound::{FadeInBGM, FadeOutBGMTemporarily, IsBGMPausedOrStopped};
use crate::string_util::{ConvertIntToDecimalStringN, StringCopy};
use crate::string_util::{ConvertInternationalString, IsStringJapanese, StripExtCtrlCodes};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3, gStringVar4};
use crate::task::DestroyTask;
use crate::task::gTasks;
use crate::task::task_set;
use crate::text::GetStringWidth;
use crate::text::gDisableTextPrinters;
use crate::trader::{Trader_ResetFlag, TraderSetup};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::CopyWindowToVram;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
// The C's names for task and sprite data slots.
const tState: usize = 0;
const tWordState: usize = 1;
const tDelay: usize = 2;
const tCharIndex: usize = 3;
const tLyricsIndex: usize = 4;
const tUseNewSongLyrics: usize = 5;
// Data tables (translate with cdata.py): sDefaultBardSongLyrics sGiddyAdjectives sGiddyQuestions sStorytellerStories sNumStories sUnused

/// `struct Story`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Story {
    pub stat: u8,
    pub minVal: u8,
    pub title: *mut u8,
    pub action: *mut u8,
    pub fullText: *mut u8,
}

unsafe impl Sync for Story {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Story>() == 16);
    assert!(offset_of!(Story, stat) == 0);
    assert!(offset_of!(Story, minVal) == 1);
    assert!(offset_of!(Story, title) == 4);
    assert!(offset_of!(Story, action) == 8);
    assert!(offset_of!(Story, fullText) == 12);
};

const BARD_SONG_BASE_PITCH: i16 = 512;
const BARD_SONG_BASE_VOLUME: u16 = 256;
const BARD_STATE_GET_WORD: i16 = 2;
const BARD_STATE_HANDLE_WORD: i16 = 3;
const BARD_STATE_INIT: i16 = 0;
const BARD_STATE_PAUSE: i16 = 5;
const BARD_STATE_WAIT_BGM: i16 = 1;
const BARD_STATE_WAIT_WORD: i16 = 4;
const SOUND_STATE_END: u8 = 3;
const SOUND_STATE_PLAY: u8 = 1;
const SOUND_STATE_SET_BASE: u8 = 2;
const SOUND_STATE_START: u8 = 0;
const SOUND_STATE_WAIT: u8 = 4;

static sDefaultBardSongLyrics: Table<CArray<u16, 6>> =
    Table((&raw const crate::data::mauville_old_man::sDefaultBardSongLyrics).cast());
static sGiddyAdjectives: Table<CArray<*mut u8, 8>> =
    Table((&raw const crate::data::mauville_old_man::sGiddyAdjectives).cast());
static sGiddyQuestions: Table<CArray<*mut u8, 8>> =
    Table((&raw const crate::data::mauville_old_man::sGiddyQuestions).cast());
static sNumStories: Table<i32> =
    Table((&raw const crate::data::mauville_old_man::sNumStories).cast());
static sStorytellerStories: Table<CArray<Story, 36>> =
    Table((&raw const crate::data::mauville_old_man::sStorytellerStories).cast());

pub(crate) static sSelectedStory: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static mut gBardSong: BardSong = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static sUnusedPitchTableIndex: crate::global::Global<u16> =
    crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sStorytellerPtr: *mut MauvilleManStoryteller = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static sStorytellerWindowId: crate::global::Global<u8> = crate::global::Global::new(0);

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
/// `GetWordSoundTemplates` with this module's view of its types.
#[inline]
unsafe fn GetWordSoundTemplates(a0: u16) -> *mut BardSoundTemplate {
    crate::bard_music::GetWordSoundTemplates(a0) as *mut BardSoundTemplate
}

unsafe fn SetupBard() {
    let bard: *mut MauvilleManBard = &raw mut (*gSaveBlock1Ptr).oldMan.bard;
    (*bard).id = MAUVILLE_MAN_BARD;
    (*bard).hasChangedSong = FALSE;
    (*bard).language = gGameLanguage;
    for i in 0..(NUM_BARD_SONG_WORDS as u16) {
        (*bard).songLyrics[i] = sDefaultBardSongLyrics[i];
    }
}
unsafe fn SetupHipster() {
    let hipster: *mut MauvilleManHipster = &raw mut (*gSaveBlock1Ptr).oldMan.hipster;
    (*hipster).id = MAUVILLE_MAN_HIPSTER;
    (*hipster).taughtWord = FALSE;
    (*hipster).language = gGameLanguage;
}
unsafe fn SetupStoryteller() {
    StorytellerSetup();
}
unsafe fn SetupGiddy() {
    let giddy: *mut MauvilleManGiddy = &raw mut (*gSaveBlock1Ptr).oldMan.giddy;
    (*giddy).id = MAUVILLE_MAN_GIDDY;
    (*giddy).taleCounter = 0;
    (*giddy).language = gGameLanguage;
}
unsafe fn SetupTrader() {
    TraderSetup();
}
#[unsafe(no_mangle)]
pub unsafe fn SetMauvilleOldMan() {
    let trainerId: u16 = ((*gSaveBlock2Ptr).playerTrainerId[1] as u16) << 8
        | (*gSaveBlock2Ptr).playerTrainerId[0] as u16;
    match trainerId as i32 % 10 / 2 {
        0 => {
            SetupBard();
        }
        1 => {
            SetupHipster();
        }
        2 => {
            SetupTrader();
        }
        3 => {
            SetupStoryteller();
        }
        4 => {
            SetupGiddy();
        }
        _ => {}
    }
    SetMauvilleOldManObjEventGfx();
}
pub unsafe fn GetCurrentMauvilleOldMan() -> u8 {
    (*gSaveBlock1Ptr).oldMan.common.id
}
#[unsafe(no_mangle)]
pub unsafe fn Script_GetCurrentMauvilleMan() {
    gSpecialVar_Result = GetCurrentMauvilleOldMan() as u16;
}
#[unsafe(no_mangle)]
pub unsafe fn HasBardSongBeenChanged() {
    gSpecialVar_Result = (*gSaveBlock1Ptr).oldMan.bard.hasChangedSong as u16;
}
#[unsafe(no_mangle)]
pub unsafe fn SaveBardSongLyrics() {
    let bard: *mut MauvilleManBard = &raw mut (*gSaveBlock1Ptr).oldMan.bard;
    StringCopy(
        (*bard).playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
    );
    let mut i: u16 = 0;
    while i < TRAINER_ID_LENGTH as u16 {
        (*bard).playerTrainerId[i] = (*gSaveBlock2Ptr).playerTrainerId[i];
        i += 1;
    }
    for i in 0..(NUM_BARD_SONG_WORDS as u16) {
        (*bard).songLyrics[i] = (*bard).newSongLyrics[i];
    }
    (*bard).hasChangedSong = TRUE;
}
unsafe fn PrepareSongText() {
    let bard: *mut MauvilleManBard = &raw mut (*gSaveBlock1Ptr).oldMan.bard;
    let mut lyrics: *mut u16 = if gSpecialVar_0x8004 == 0 {
        (*bard).songLyrics.as_mut_ptr()
    } else {
        (*bard).newSongLyrics.as_mut_ptr()
    };
    let mut wordEnd: *mut u8 = gStringVar4.as_mut_ptr();
    let mut str: *mut u8 = wordEnd;
    for paragraphNum in 0..2u16 {
        wordEnd = CopyEasyChatWord(
            wordEnd,
            *({
                let t2 = lyrics;
                lyrics = lyrics.at(1);
                t2
            }),
        );
        while wordEnd != str {
            if *str == CHAR_SPACE {
                *str = CHAR_BARD_WORD_DELIMIT;
            }
            str = str.at(1);
        }
        str = str.at(1);
        *({
            let t3 = wordEnd;
            wordEnd = wordEnd.at(1);
            t3
        }) = CHAR_SPACE;
        wordEnd = CopyEasyChatWord(
            wordEnd,
            *({
                let t5 = lyrics;
                lyrics = lyrics.at(1);
                t5
            }),
        );
        while wordEnd != str {
            if *str == CHAR_SPACE {
                *str = CHAR_BARD_WORD_DELIMIT;
            }
            str = str.at(1);
        }
        str = str.at(1);
        *({
            let t6 = wordEnd;
            wordEnd = wordEnd.at(1);
            t6
        }) = CHAR_NEWLINE;
        wordEnd = CopyEasyChatWord(
            wordEnd,
            *({
                let t8 = lyrics;
                lyrics = lyrics.at(1);
                t8
            }),
        );
        while wordEnd != str {
            if *str == CHAR_SPACE {
                *str = CHAR_BARD_WORD_DELIMIT;
            }
            str = str.at(1);
        }
        if paragraphNum == 0 {
            *({
                let t9 = wordEnd;
                wordEnd = wordEnd.at(1);
                t9
            }) = EXT_CTRL_CODE_BEGIN;
            *({
                let t10 = wordEnd;
                wordEnd = wordEnd.at(1);
                t10
            }) = EXT_CTRL_CODE_FILL_WINDOW;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn PlayBardSong() {
    StartBardSong(gSpecialVar_0x8004 as u8);
    ScriptContext_Stop();
}
#[unsafe(no_mangle)]
pub unsafe fn HasHipsterTaughtWord() {
    gSpecialVar_Result = (*gSaveBlock1Ptr).oldMan.hipster.taughtWord as u16;
}
#[unsafe(no_mangle)]
pub unsafe fn SetHipsterTaughtWord() {
    (*gSaveBlock1Ptr).oldMan.hipster.taughtWord = TRUE;
}
#[unsafe(no_mangle)]
pub unsafe fn HipsterTryTeachWord() {
    let word: u16 = UnlockRandomTrendySaying();
    if word == EC_EMPTY_WORD {
        gSpecialVar_Result = FALSE as u16;
    } else {
        CopyEasyChatWord(gStringVar1.as_mut_ptr(), word);
        gSpecialVar_Result = TRUE as u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GiddyShouldTellAnotherTale() {
    let giddy: *mut MauvilleManGiddy = &raw mut (*gSaveBlock1Ptr).oldMan.giddy;
    if (*giddy).taleCounter == GIDDY_MAX_TALES {
        gSpecialVar_Result = FALSE as u16;
        (*giddy).taleCounter = 0;
    } else {
        gSpecialVar_Result = TRUE as u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GenerateGiddyLine() {
    let giddy: *mut MauvilleManGiddy = &raw mut (*gSaveBlock1Ptr).oldMan.giddy;
    if (*giddy).taleCounter == 0 {
        InitGiddyTaleList();
    }
    if (*giddy).randomWords[(*giddy).taleCounter] != EC_EMPTY_WORD {
        let mut adjective: u32 = Random() as u32;
        adjective %= 8;
        let mut stringPtr: *mut u8 = CopyEasyChatWord(
            gStringVar4.as_mut_ptr(),
            (*giddy).randomWords[(*giddy).taleCounter],
        );
        stringPtr = StringCopy(
            stringPtr,
            (*crate::asmdata::GiddyText_Is.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        stringPtr = StringCopy(stringPtr, sGiddyAdjectives[adjective]);
        StringCopy(
            stringPtr,
            (*crate::asmdata::GiddyText_DontYouAgree.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    } else {
        StringCopy(
            gStringVar4.as_mut_ptr(),
            sGiddyQuestions[(*giddy).questionList[{
                let t1 = (*giddy).questionNum;
                (*giddy).questionNum += 1;
                t1
            }]],
        );
    }
    if Random() as i32 % 10 == 0 {
        (*giddy).taleCounter = GIDDY_MAX_TALES;
    } else {
        (*giddy).taleCounter += 1;
    }
    gSpecialVar_Result = TRUE as u16;
}
unsafe fn InitGiddyTaleList() {
    let giddy: *mut MauvilleManGiddy = &raw mut (*gSaveBlock1Ptr).oldMan.giddy;
    let mut wordGroupsAndCount: CArray<CArray<u16, 2>, 6> = zeroed();
    wordGroupsAndCount[0][0] = 0;
    wordGroupsAndCount[0][1] = 0;
    wordGroupsAndCount[1][0] = EC_GROUP_LIFESTYLE as u16;
    wordGroupsAndCount[1][1] = 0;
    wordGroupsAndCount[2][0] = EC_GROUP_HOBBIES as u16;
    wordGroupsAndCount[2][1] = 0;
    wordGroupsAndCount[3][0] = EC_GROUP_MOVE_1 as u16;
    wordGroupsAndCount[3][1] = 0;
    wordGroupsAndCount[4][0] = EC_GROUP_MOVE_2 as u16;
    wordGroupsAndCount[4][1] = 0;
    wordGroupsAndCount[5][0] = EC_GROUP_POKEMON_NATIONAL as u16;
    wordGroupsAndCount[5][1] = 0;
    let mut temp: u16 = 0;
    let mut var: u16 = 0;
    for i in 0..GIDDY_MAX_QUESTIONS {
        (*giddy).questionList[i] = i as u8;
    }
    for i in 0..GIDDY_MAX_QUESTIONS {
        var = rem_i32(Random() as i32, i as i32 + 1) as u16;
        temp = (*giddy).questionList[i] as u16;
        (*giddy).questionList[i] = (*giddy).questionList[var];
        (*giddy).questionList[var] = temp as u8;
    }
    let mut totalWords: u16 = 0;
    let mut i: u16 = 0;
    while i < 6 {
        wordGroupsAndCount[i][1] = EasyChat_GetNumWordsInGroup(wordGroupsAndCount[i][0] as u8);
        totalWords += wordGroupsAndCount[i][1];
        i += 1;
    }
    (*giddy).questionNum = 0;
    temp = 0;
    for i in 0..(GIDDY_MAX_TALES as u16) {
        var = (Random() as i32 % 10) as u16;
        if var < 3 && temp < GIDDY_MAX_QUESTIONS {
            (*giddy).randomWords[i] = EC_EMPTY_WORD;
            temp += 1;
        } else {
            let mut randWord: i16 = rem_i32(Random() as i32, totalWords as i32) as i16;
            var = 0;
            while i < 6 {
                if ({
                    randWord -= wordGroupsAndCount[var][1] as i16;
                    randWord
                }) <= 0
                {
                    break;
                }
                var += 1;
            }
            if var == 6 {
                var = 0;
            }
            (*giddy).randomWords[i] =
                GetRandomEasyChatWordFromUnlockedGroup(wordGroupsAndCount[var][0]);
        }
    }
}
unsafe fn ResetBardFlag() {
    (*gSaveBlock1Ptr).oldMan.bard.hasChangedSong = FALSE;
}
unsafe fn ResetHipsterFlag() {
    (*gSaveBlock1Ptr).oldMan.hipster.taughtWord = FALSE;
}
unsafe fn ResetTraderFlag() {
    Trader_ResetFlag();
}
unsafe fn ResetStorytellerFlag() {
    Storyteller_ResetFlag();
}
pub unsafe fn ResetMauvilleOldManFlag() {
    match GetCurrentMauvilleOldMan() {
        MAUVILLE_MAN_BARD => {
            ResetBardFlag();
        }
        MAUVILLE_MAN_HIPSTER => {
            ResetHipsterFlag();
        }
        MAUVILLE_MAN_STORYTELLER => {
            ResetStorytellerFlag();
        }
        MAUVILLE_MAN_TRADER => {
            ResetTraderFlag();
        }
        MAUVILLE_MAN_GIDDY => {}
        _ => {}
    }
    SetMauvilleOldManObjEventGfx();
}
unsafe fn StartBardSong(useNewSongLyrics: u8) {
    let taskId: u8 = CreateTask(Some(Task_BardSong), 80);
    task_set(taskId, tUseNewSongLyrics, useNewSongLyrics as i16);
}
fn EnableTextPrinters() {
    gDisableTextPrinters.set(FALSE);
}
pub(crate) unsafe fn DisableTextPrinters(printer: *mut TextPrinterTemplate, renderCmd: u16) {
    gDisableTextPrinters.set(TRUE);
}
unsafe fn DrawSongTextWindow(str: *mut u8) {
    DrawDialogueFrame(0, 0);
    AddTextPrinterParameterized(0, FONT_NORMAL, str, 0, 1, 1, Some(DisableTextPrinters));
    gDisableTextPrinters.set(TRUE);
    CopyWindowToVram(0, COPYWIN_FULL);
}
unsafe fn BardSing(task: *mut Task, song: *mut BardSong) {
    'l1: {
        match (*task).data[tState] {
            BARD_STATE_INIT => {
                let bard: *mut MauvilleManBard = &raw mut (*gSaveBlock1Ptr).oldMan.bard;
                let mut lyrics: *mut u16 = null_mut();
                if gSpecialVar_0x8004 == 0 {
                    lyrics = (*bard).songLyrics.as_mut_ptr();
                } else {
                    lyrics = (*bard).newSongLyrics.as_mut_ptr();
                }
                for i in 0..NUM_BARD_SONG_WORDS {
                    (*song).lyrics[i] = *lyrics.at(i);
                }
                (*song).lyricsIndex = 0;
                break 'l1;
            }
            BARD_STATE_GET_WORD => {
                let easyChatWord: u16 = (*song).lyrics[(*song).lyricsIndex];
                (*song).soundTemplates = GetWordSoundTemplates(easyChatWord);
                CalcWordSounds(
                    song,
                    (if 0 != 0 {
                        easyChatWord as i32 % 4
                    } else {
                        easyChatWord as i32 & 3
                    }) as u16
                        + (easyChatWord >> 3 & 1),
                );
                (*song).lyricsIndex += 1;
                if (*(*song).soundTemplates).songId != PHONEME_ID_NONE {
                    (*song).state = SOUND_STATE_START;
                } else {
                    (*song).state = SOUND_STATE_END;
                    (*song).timer = 2;
                }
                break 'l1;
            }
            BARD_STATE_HANDLE_WORD | BARD_STATE_WAIT_WORD => {
                let template: *mut BardSoundTemplate =
                    (*song).soundTemplates.at((*song).soundIndex);
                match (*song).state {
                    SOUND_STATE_START => {
                        (*song).timer = (*song).sounds[(*song).soundIndex].length as u8;
                        if (*template).songId < NUM_PHONEME_SONGS {
                            let phonemeTripletId: u8 = ((*template).songId as i32 / 3) as u8;
                            m4aSongNumStart(560 + phonemeTripletId as u16 * 3);
                        }
                        (*song).state = SOUND_STATE_SET_BASE;
                        (*song).timer -= 1;
                    }
                    SOUND_STATE_SET_BASE => {
                        (*song).state = SOUND_STATE_PLAY;
                        if (*template).songId < NUM_PHONEME_SONGS {
                            (*song).volume = BARD_SONG_BASE_VOLUME + (*template).volume as u16 * 16;
                            m4aMPlayVolumeControl(
                                &raw mut gMPlayInfo_SE2,
                                TRACKS_ALL,
                                (*song).volume,
                            );
                            (*song).pitch = BARD_SONG_BASE_PITCH
                                + (*song).sounds[(*song).soundIndex].pitch as i16;
                            m4aMPlayPitchControl(
                                &raw mut gMPlayInfo_SE2,
                                TRACKS_ALL,
                                (*song).pitch,
                            );
                        }
                    }
                    SOUND_STATE_PLAY => {
                        if (*song).voiceInflection > 10 {
                            (*song).volume -= 2;
                        }
                        if (*song).voiceInflection as i32 & 1 != 0 {
                            (*song).pitch += 64;
                        } else {
                            (*song).pitch -= 64;
                        }
                        m4aMPlayVolumeControl(&raw mut gMPlayInfo_SE2, TRACKS_ALL, (*song).volume);
                        m4aMPlayPitchControl(&raw mut gMPlayInfo_SE2, TRACKS_ALL, (*song).pitch);
                        (*song).voiceInflection += 1;
                        (*song).timer -= 1;
                        if (*song).timer == 0 {
                            if ({
                                (*song).soundIndex += 1;
                                (*song).soundIndex
                            }) != MAX_BARD_SOUNDS_PER_WORD
                                && (*(*song).soundTemplates.at((*song).soundIndex)).songId
                                    != PHONEME_ID_NONE
                            {
                                (*song).state = SOUND_STATE_START;
                            } else {
                                (*song).state = SOUND_STATE_END;
                                (*song).timer = 2;
                            }
                        }
                    }
                    SOUND_STATE_END
                        if ({
                            (*song).timer -= 1;
                            (*song).timer
                        }) == 0 =>
                    {
                        m4aMPlayStop(&raw mut gMPlayInfo_SE2);
                        (*song).state = SOUND_STATE_WAIT;
                    }
                    _ => {}
                }
                break 'l1;
            }
            BARD_STATE_PAUSE | BARD_STATE_WAIT_BGM => {}
            _ => {}
        }
    }
}
pub(crate) unsafe fn Task_BardSong(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    BardSing(task, &raw mut gBardSong);
    match (*task).data[tState] {
        BARD_STATE_INIT => {
            PrepareSongText();
            DrawSongTextWindow(gStringVar4.as_mut_ptr());
            (*task).data[tWordState] = 0;
            (*task).data[tDelay] = 0;
            (*task).data[tCharIndex] = 0;
            (*task).data[tLyricsIndex] = 0;
            FadeOutBGMTemporarily(4);
            (*task).data[tState] = BARD_STATE_WAIT_BGM;
        }
        BARD_STATE_WAIT_BGM => {
            if IsBGMPausedOrStopped() != 0 {
                (*task).data[tState] = BARD_STATE_GET_WORD;
            }
        }
        BARD_STATE_GET_WORD => {
            let bard: *mut MauvilleManBard = &raw mut (*gSaveBlock1Ptr).oldMan.bard;
            let mut str: *mut u8 = &raw mut (*(&raw const crate::string_util::gStringVar4)
                .cast::<CArray<u8, 1000>>()
                .cast_mut())[(*task).data[tCharIndex]];
            let mut wordLen: u16 = 0;
            while *str != CHAR_SPACE
                && *str != CHAR_NEWLINE
                && *str != EXT_CTRL_CODE_BEGIN
                && *str != EOS
            {
                str = str.at(1);
                wordLen += 1;
            }
            if (*task).data[tUseNewSongLyrics] == 0 {
                sUnusedPitchTableIndex.set(
                    (if 0 != 0 {
                        (*bard).songLyrics[(*task).data[tLyricsIndex]] as i32 % 4
                    } else {
                        (*bard).songLyrics[(*task).data[tLyricsIndex]] as i32 & 3
                    }) as u16
                        + ((*bard).songLyrics[(*task).data[tLyricsIndex]] >> 3 & 1),
                );
            } else {
                sUnusedPitchTableIndex.set(
                    (if 0 != 0 {
                        (*bard).newSongLyrics[(*task).data[tLyricsIndex]] as i32 % 4
                    } else {
                        (*bard).newSongLyrics[(*task).data[tLyricsIndex]] as i32 & 3
                    }) as u16
                        + ((*bard).newSongLyrics[(*task).data[tLyricsIndex]] >> 3 & 1),
                );
            }
            gBardSong.length = div_i32(gBardSong.length as i32, wordLen as i32) as i16;
            if gBardSong.length <= 0 {
                gBardSong.length = 1;
            }
            (*task).data[tLyricsIndex] += 1;
            if (*task).data[tDelay] == 0 {
                (*task).data[tState] = BARD_STATE_HANDLE_WORD;
                (*task).data[tWordState] = 0;
            } else {
                (*task).data[tState] = BARD_STATE_PAUSE;
                (*task).data[tWordState] = 0;
            }
        }
        BARD_STATE_PAUSE => {
            if (*task).data[tDelay] == 0 {
                (*task).data[tState] = BARD_STATE_HANDLE_WORD;
            } else {
                (*task).data[tDelay] -= 1;
            }
        }
        BARD_STATE_HANDLE_WORD => {
            if gStringVar4[(*task).data[tCharIndex]] == EOS {
                FadeInBGM(6);
                m4aMPlayFadeOutTemporarily(&raw mut gMPlayInfo_SE2, 2);
                ScriptContext_Enable();
                DestroyTask(taskId);
            } else if gStringVar4[(*task).data[tCharIndex]] == CHAR_SPACE {
                EnableTextPrinters();
                (*task).data[tCharIndex] += 1;
                (*task).data[tState] = BARD_STATE_GET_WORD;
                (*task).data[tDelay] = 0;
            } else if gStringVar4[(*task).data[tCharIndex]] == CHAR_NEWLINE {
                (*task).data[tCharIndex] += 1;
                (*task).data[tState] = BARD_STATE_GET_WORD;
                (*task).data[tDelay] = 0;
            } else if gStringVar4[(*task).data[tCharIndex]] == EXT_CTRL_CODE_BEGIN {
                (*task).data[tCharIndex] += 2;
                (*task).data[tState] = BARD_STATE_GET_WORD;
                (*task).data[tDelay] = 8;
            } else if gStringVar4[(*task).data[tCharIndex]] == CHAR_BARD_WORD_DELIMIT {
                gStringVar4[(*task).data[tCharIndex]] = CHAR_SPACE;
                EnableTextPrinters();
                (*task).data[tCharIndex] += 1;
                (*task).data[tDelay] = 0;
            } else {
                match (*task).data[tWordState] {
                    0 => {
                        EnableTextPrinters();
                        (*task).data[tWordState] += 1;
                    }
                    1 => {
                        (*task).data[tWordState] += 1;
                    }
                    2 => {
                        (*task).data[tCharIndex] += 1;
                        (*task).data[tWordState] = 0;
                        (*task).data[tDelay] = gBardSong.length;
                        (*task).data[tState] = BARD_STATE_WAIT_WORD;
                    }
                    _ => {}
                }
            }
        }
        BARD_STATE_WAIT_WORD => {
            (*task).data[tDelay] -= 1;
            if (*task).data[tDelay] == 0 {
                (*task).data[tState] = BARD_STATE_HANDLE_WORD;
            }
        }
        _ => {}
    }
    RunTextPrintersAndIsPrinter0Active();
}
#[unsafe(no_mangle)]
pub unsafe fn SetMauvilleOldManObjEventGfx() {
    VarSet(VAR_OBJ_GFX_ID_0, OBJ_EVENT_GFX_BARD as u16);
}
pub unsafe fn SanitizeMauvilleOldManForRuby(oldMan: *mut OldMan) {
    let mut playerName: CArray<u8, 8> = zeroed();
    'l1: {
        match (*oldMan).common.id {
            MAUVILLE_MAN_TRADER => {
                let trader: *mut MauvilleOldManTrader = &raw mut (*oldMan).trader;
                for i in 0..NUM_TRADER_ITEMS {
                    if (*trader).language[i] == LANGUAGE_JAPANESE {
                        ConvertInternationalString(
                            (*trader).playerNames[i].as_mut_ptr(),
                            LANGUAGE_JAPANESE,
                        );
                    }
                }
                break 'l1;
            }
            MAUVILLE_MAN_STORYTELLER => {
                let storyteller: *mut MauvilleManStoryteller = &raw mut (*oldMan).storyteller;
                for i in 0..NUM_STORYTELLER_TALES {
                    if (*storyteller).gameStatIDs[i] != 0 {
                        memcpy(
                            playerName.as_mut_ptr(),
                            (*storyteller).trainerNames[i].as_mut_ptr(),
                            PLAYER_NAME_LENGTH as u32,
                        );
                        playerName[7] = EOS;
                        if IsStringJapanese(playerName.as_mut_ptr()) != 0 {
                            memset(playerName.as_mut_ptr(), CHAR_SPACE as i32, 8);
                            StringCopy(
                                playerName.as_mut_ptr(),
                                (*(&raw const crate::data::strings::gText_Friend)
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                            memcpy(
                                (*storyteller).trainerNames[i].as_mut_ptr(),
                                playerName.as_mut_ptr(),
                                PLAYER_NAME_LENGTH as u32,
                            );
                            (*storyteller).language[i] = GAME_LANGUAGE;
                        }
                    }
                }
                break 'l1;
            }
            _ => {}
        }
    }
}
unsafe fn SetMauvilleOldManLanguage(
    oldMan: *mut OldMan,
    language1: u32,
    language2: u32,
    language3: u32,
) {
    match (*oldMan).common.id {
        MAUVILLE_MAN_TRADER => {
            let trader: *mut MauvilleOldManTrader = &raw mut (*oldMan).trader;
            for i in 0..NUM_TRADER_ITEMS {
                if IsStringJapanese((*trader).playerNames[i].as_mut_ptr()) != 0 {
                    (*trader).language[i] = language1 as u8;
                } else {
                    (*trader).language[i] = language2 as u8;
                }
            }
        }
        MAUVILLE_MAN_STORYTELLER => {
            let storyteller: *mut MauvilleManStoryteller = &raw mut (*oldMan).storyteller;
            for i in 0..NUM_STORYTELLER_TALES {
                if IsStringJapanese((*storyteller).trainerNames[i].as_mut_ptr()) != 0 {
                    (*storyteller).language[i] = language1 as u8;
                } else {
                    (*storyteller).language[i] = language2 as u8;
                }
            }
        }
        MAUVILLE_MAN_BARD => {
            let bard: *mut MauvilleManBard = &raw mut (*oldMan).bard;
            if language3 == LANGUAGE_JAPANESE as u32 {
                (*bard).language = language1 as u8;
            } else {
                (*bard).language = language2 as u8;
            }
        }
        MAUVILLE_MAN_HIPSTER => {
            let hipster: *mut MauvilleManHipster = &raw mut (*oldMan).hipster;
            if language3 == LANGUAGE_JAPANESE as u32 {
                (*hipster).language = language1 as u8;
            } else {
                (*hipster).language = language2 as u8;
            }
        }
        MAUVILLE_MAN_GIDDY => {
            let giddy: *mut MauvilleManGiddy = &raw mut (*oldMan).giddy;
            if language3 == LANGUAGE_JAPANESE as u32 {
                (*giddy).language = language1 as u8;
            } else {
                (*giddy).language = language2 as u8;
            }
        }
        _ => {}
    }
}
pub unsafe fn SanitizeReceivedEmeraldOldMan(oldMan: *mut OldMan, version: u32, language: u32) {
    let mut playerName: CArray<u8, 8> = zeroed();
    if (*oldMan).common.id == MAUVILLE_MAN_STORYTELLER && language == LANGUAGE_JAPANESE as u32 {
        let storyteller: *mut MauvilleManStoryteller = &raw mut (*oldMan).storyteller;
        for i in 0..NUM_STORYTELLER_TALES {
            if (*storyteller).gameStatIDs[i] != 0 {
                memcpy(
                    playerName.as_mut_ptr(),
                    (*storyteller).trainerNames[i].as_mut_ptr(),
                    PLAYER_NAME_LENGTH as u32,
                );
                playerName[7] = EOS;
                if IsStringJapanese(playerName.as_mut_ptr()) != 0 {
                    (*storyteller).language[i] = LANGUAGE_JAPANESE;
                } else {
                    (*storyteller).language[i] = GAME_LANGUAGE;
                }
            }
        }
    }
}
pub unsafe fn SanitizeReceivedRubyOldMan(oldMan: *mut OldMan, version: u32, language: u32) {
    let isRuby: u32 = (version == VERSION_SAPPHIRE as u32 || version == VERSION_RUBY as u32) as u32;
    match (*oldMan).common.id {
        MAUVILLE_MAN_TRADER => {
            let trader: *mut MauvilleOldManTrader = &raw mut (*oldMan).trader;
            if isRuby != 0 {
                for i in 0..NUM_TRADER_ITEMS {
                    let str: *mut u8 = (*trader).playerNames[i].as_mut_ptr();
                    if *str == EXT_CTRL_CODE_BEGIN && *str.at(1) == EXT_CTRL_CODE_JPN {
                        StripExtCtrlCodes(str);
                        (*trader).language[i] = LANGUAGE_JAPANESE;
                    } else {
                        (*trader).language[i] = language as u8;
                    }
                }
            } else {
                for i in 0..NUM_TRADER_ITEMS {
                    if (*trader).language[i] == LANGUAGE_JAPANESE {
                        StripExtCtrlCodes((*trader).playerNames[i].as_mut_ptr());
                    }
                }
            }
        }
        MAUVILLE_MAN_STORYTELLER => {
            let storyteller: *mut MauvilleManStoryteller = &raw mut (*oldMan).storyteller;
            if isRuby != 0 {
                for i in 0..NUM_STORYTELLER_TALES {
                    if (*storyteller).gameStatIDs[i] != 0 {
                        (*storyteller).language[i] = language as u8;
                    }
                }
            }
        }
        MAUVILLE_MAN_BARD => {
            let bard: *mut MauvilleManBard = &raw mut (*oldMan).bard;
            if isRuby != 0 {
                (*bard).language = language as u8;
            }
        }
        MAUVILLE_MAN_HIPSTER => {
            let hipster: *mut MauvilleManHipster = &raw mut (*oldMan).hipster;
            if isRuby != 0 {
                (*hipster).language = language as u8;
            }
        }
        MAUVILLE_MAN_GIDDY => {
            let giddy: *mut MauvilleManGiddy = &raw mut (*oldMan).giddy;
            if isRuby != 0 {
                (*giddy).language = language as u8;
            }
        }
        _ => {}
    }
}
unsafe fn StorytellerSetup() {
    sStorytellerPtr = &raw mut (*gSaveBlock1Ptr).oldMan.storyteller;
    (*sStorytellerPtr).id = MAUVILLE_MAN_STORYTELLER;
    (*sStorytellerPtr).alreadyRecorded = FALSE;
    for i in 0..NUM_STORYTELLER_TALES {
        (*sStorytellerPtr).gameStatIDs[i] = 0;
        (*sStorytellerPtr).trainerNames[0][i] = EOS;
    }
}
unsafe fn Storyteller_ResetFlag() {
    sStorytellerPtr = &raw mut (*gSaveBlock1Ptr).oldMan.storyteller;
    (*sStorytellerPtr).id = MAUVILLE_MAN_STORYTELLER;
    (*sStorytellerPtr).alreadyRecorded = FALSE;
}
unsafe fn StorytellerGetGameStat(mut stat: u8) -> u32 {
    if stat == 50 {
        stat = GAME_STAT_SAVED_GAME;
    }
    GetGameStat(stat)
}
unsafe fn GetStoryByStat(stat: u32) -> *mut Story {
    let mut i: i32 = 0;
    while i < *sNumStories {
        if sStorytellerStories[i].stat as u32 == stat {
            return (&raw const sStorytellerStories[i]).cast_mut();
        }
        i += 1;
    }
    (&raw const sStorytellerStories[*sNumStories - 1]).cast_mut()
}
unsafe fn GetStoryTitleByStat(stat: u32) -> *mut u8 {
    (*GetStoryByStat(stat)).title
}
unsafe fn GetStoryTextByStat(stat: u32) -> *mut u8 {
    (*GetStoryByStat(stat)).fullText
}
unsafe fn GetStoryActionByStat(stat: u32) -> *mut u8 {
    (*GetStoryByStat(stat)).action
}
unsafe fn GetFreeStorySlot() -> u8 {
    let mut i: u8 = 0;
    while i < NUM_STORYTELLER_TALES as u8 {
        if (*sStorytellerPtr).gameStatIDs[i] == 0 {
            break;
        }
        i += 1;
    }
    i
}
unsafe fn StorytellerGetRecordedTrainerStat(trainer: u32) -> u32 {
    let ptr: *mut u8 = (*sStorytellerPtr).statValues[trainer].as_mut_ptr();
    *ptr as u32 | (*ptr.at(1) as u32) << 8 | (*ptr.at(2) as u32) << 16 | (*ptr.at(3) as u32) << 24
}
unsafe fn StorytellerSetRecordedTrainerStat(trainer: u32, val: u32) {
    let ptr: *mut u8 = (*sStorytellerPtr).statValues[trainer].as_mut_ptr();
    *ptr = val as u8;
    *ptr.at(1) = (val >> 8) as u8;
    *ptr.at(2) = (val >> 16) as u8;
    *ptr.at(3) = (val >> 24) as u8;
}
unsafe fn HasTrainerStatIncreased(trainer: u32) -> u32 {
    if StorytellerGetGameStat((*sStorytellerPtr).gameStatIDs[trainer])
        > StorytellerGetRecordedTrainerStat(trainer)
    {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetStoryByStattellerPlayerName(player: u32, dst: *mut c_void) {
    let name: *mut u8 = (*sStorytellerPtr).trainerNames[player].as_mut_ptr();
    memset(dst as *mut u8, EOS as i32, 8);
    memcpy(dst as *mut u8, name, PLAYER_NAME_LENGTH as u32);
}
unsafe fn StorytellerSetPlayerName(player: u32, src: *mut u8) {
    let name: *mut u8 = (*sStorytellerPtr).trainerNames[player].as_mut_ptr();
    memset(name, EOS as i32, PLAYER_NAME_LENGTH as u32);
    memcpy(name, src, PLAYER_NAME_LENGTH as u32);
}
unsafe fn StorytellerRecordNewStat(player: u32, stat: u32) {
    (*sStorytellerPtr).gameStatIDs[player] = stat as u8;
    StorytellerSetPlayerName(player, (*gSaveBlock2Ptr).playerName.as_mut_ptr());
    StorytellerSetRecordedTrainerStat(player, StorytellerGetGameStat(stat as u8));
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        StorytellerGetGameStat(stat as u8) as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        10,
    );
    StringCopy(gStringVar2.as_mut_ptr(), GetStoryActionByStat(stat));
    (*sStorytellerPtr).language[player] = gGameLanguage;
}
unsafe fn ScrambleStatList(arr: *mut u8, count: i32) {
    let mut i: i32 = 0;
    while i < count {
        *arr.at(i) = i as u8;
        i += 1;
    }
    for i in 0..count {
        let a: u32 = rem_i32(Random() as i32, count) as u32;
        let b: u32 = rem_i32(Random() as i32, count) as u32;
        let temp: u8 = *arr.at(a);
        *arr.at(a) = *arr.at(b);
        *arr.at(b) = temp;
    }
}
unsafe fn StorytellerInitializeRandomStat() -> u8 {
    let mut storyIds: CArray<u8, 36> = zeroed();
    let mut j: i32 = 0;
    ScrambleStatList(storyIds.as_mut_ptr(), *sNumStories);
    let mut i: i32 = 0;
    while i < *sNumStories {
        let stat: u8 = sStorytellerStories[storyIds[i]].stat;
        let minVal: u8 = sStorytellerStories[storyIds[i]].minVal;
        j = 0;
        while j < NUM_STORYTELLER_TALES {
            if (*sStorytellerPtr).gameStatIDs[j] == stat {
                break;
            }
            j += 1;
        }
        if j == NUM_STORYTELLER_TALES && StorytellerGetGameStat(stat) >= minVal as u32 {
            (*sStorytellerPtr).alreadyRecorded = TRUE;
            if GetFreeStorySlot() == NUM_STORYTELLER_TALES as u8 {
                StorytellerRecordNewStat(sSelectedStory.get() as u32, stat as u32);
            } else {
                StorytellerRecordNewStat(GetFreeStorySlot() as u32, stat as u32);
            }
            return TRUE;
        }
        i += 1;
    }
    FALSE
}
unsafe fn StorytellerDisplayStory(player: u32) {
    let stat: u8 = (*sStorytellerPtr).gameStatIDs[player];
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        StorytellerGetRecordedTrainerStat(player) as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        10,
    );
    StringCopy(gStringVar2.as_mut_ptr(), GetStoryActionByStat(stat as u32));
    GetStoryByStattellerPlayerName(player, gStringVar3.as_mut_ptr() as *mut c_void);
    ConvertInternationalString(
        gStringVar3.as_mut_ptr(),
        (*sStorytellerPtr).language[player],
    );
    ShowFieldMessage(GetStoryTextByStat(stat as u32));
}
unsafe fn PrintStoryList() {
    let mut width: i32 = GetStringWidth(
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Exit).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
    );
    for i in 0..NUM_STORYTELLER_TALES {
        let gameStatID: u16 = (*sStorytellerPtr).gameStatIDs[i] as u16;
        if gameStatID == 0 {
            break;
        }
        let curWidth: i32 = GetStringWidth(FONT_NORMAL, GetStoryTitleByStat(gameStatID as u32), 0);
        if curWidth > width {
            width = curWidth;
        }
    }
    sStorytellerWindowId.set(CreateWindowFromRect(
        0,
        0,
        ConvertPixelWidthToTileWidth(width) as u8,
        GetFreeStorySlot() * 2 + 2,
    ));
    SetStandardWindowBorderStyle(sStorytellerWindowId.get(), FALSE);
    let mut i: i32 = 0;
    while i < NUM_STORYTELLER_TALES {
        let gameStatID: u16 = (*sStorytellerPtr).gameStatIDs[i] as u16;
        if gameStatID == 0 {
            break;
        }
        AddTextPrinterParameterized(
            sStorytellerWindowId.get(),
            FONT_NORMAL,
            GetStoryTitleByStat(gameStatID as u32),
            8,
            16 * i as u8 + 1,
            TEXT_SKIP_DRAW,
            None,
        );
        i += 1;
    }
    AddTextPrinterParameterized(
        sStorytellerWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Exit).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        8,
        16 * i as u8 + 1,
        TEXT_SKIP_DRAW,
        None,
    );
    InitMenuInUpperLeftCornerNormal(sStorytellerWindowId.get(), GetFreeStorySlot() + 1, 0);
    CopyWindowToVram(sStorytellerWindowId.get(), COPYWIN_FULL);
}
pub(crate) unsafe fn Task_StoryListMenu(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let mut selection: i32 = 0;
    'l1: {
        match (*task).data[0] {
            0 => {
                PrintStoryList();
                (*task).data[0] += 1;
            }
            1 => {
                selection = Menu_ProcessInput() as i32;
                if selection == MENU_NOTHING_CHOSEN as i32 {
                    break 'l1;
                }
                if selection == MENU_B_PRESSED as i32 || selection == GetFreeStorySlot() as i32 {
                    gSpecialVar_Result = 0;
                } else {
                    gSpecialVar_Result = 1;
                    sSelectedStory.set(selection as u8);
                }
                ClearToTransparentAndRemoveWindow(sStorytellerWindowId.get());
                DestroyTask(taskId);
                ScriptContext_Enable();
            }
            _ => {}
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn StorytellerStoryListMenu() {
    CreateTask(Some(Task_StoryListMenu), 80);
}
#[unsafe(no_mangle)]
pub unsafe fn Script_StorytellerDisplayStory() {
    StorytellerDisplayStory(sSelectedStory.get() as u32);
}
#[unsafe(no_mangle)]
pub unsafe fn StorytellerGetFreeStorySlot() -> u8 {
    sStorytellerPtr = &raw mut (*gSaveBlock1Ptr).oldMan.storyteller;
    GetFreeStorySlot()
}
#[unsafe(no_mangle)]
pub unsafe fn StorytellerUpdateStat() -> u8 {
    sStorytellerPtr = &raw mut (*gSaveBlock1Ptr).oldMan.storyteller;
    let stat: u8 = (*sStorytellerPtr).gameStatIDs[sSelectedStory.get()];
    if HasTrainerStatIncreased(sSelectedStory.get() as u32) == TRUE as u32 {
        StorytellerRecordNewStat(sSelectedStory.get() as u32, stat as u32);
        return TRUE;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn HasStorytellerAlreadyRecorded() -> u8 {
    sStorytellerPtr = &raw mut (*gSaveBlock1Ptr).oldMan.storyteller;
    if (*sStorytellerPtr).alreadyRecorded == FALSE {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn Script_StorytellerInitializeRandomStat() -> u8 {
    sStorytellerPtr = &raw mut (*gSaveBlock1Ptr).oldMan.storyteller;
    StorytellerInitializeRandomStat()
}
