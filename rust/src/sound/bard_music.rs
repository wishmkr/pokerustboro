//! The Mauville bard's song: turning easy chat words into phoneme sounds.
//! The phoneme tables themselves are in `data/bard_music.rs`.

use crate::data::bard_music::{
    sBardSoundTemplates_Moves, sBardSoundTemplates_Pokemon, sBardSoundTemplatesTable,
    sEmptyPhonemeTemplate, sPhonemeLengths, sPitchTables,
};

const MAX_BARD_SOUNDS_PER_WORD: usize = 6;
const NUM_BARD_PITCH_TABLES_PER_SIZE: usize = 5;
/// Tables are chosen one size larger than the most sounds a word can have.
const BASE_PITCH_TABLE_INDEX: usize = NUM_BARD_PITCH_TABLES_PER_SIZE * MAX_BARD_SOUNDS_PER_WORD;
const PHONEME_ID_NONE: u8 = 0xff;

const EC_MASK_BITS: u32 = 9;
const EC_MASK_INDEX: u16 = (1 << EC_MASK_BITS) - 1;
const EC_GROUP_POKEMON: u16 = 0x00;
const EC_GROUP_MOVE_1: u16 = 0x12;
const EC_GROUP_MOVE_2: u16 = 0x13;
const EC_GROUP_POKEMON_NATIONAL: u16 = 0x15;

/// `struct BardSoundTemplate { u8 songId; s8 lengthAdjustment; u16; s16 volume; }`
const TEMPLATE_SIZE: usize = 8;
const WORD_TEMPLATES_SIZE: usize = TEMPLATE_SIZE * MAX_BARD_SOUNDS_PER_WORD;

/// `struct BardSong`
const SONG_LENGTH: usize = 0x04;
const SONG_VOICE_INFLECTION: usize = 0x0a;
const SONG_SOUND_INDEX: usize = 0x01;
const SONG_SOUNDS: usize = 0x18;
const SONG_SOUND_TEMPLATES: usize = 0x30;
/// `struct BardSound { u16 length; u16 pitch; }`
const SOUND_SIZE: usize = 4;

unsafe extern "C" {
    fn IsBardWordInvalid(easy_chat_word: u16) -> u8;
}

unsafe fn word_pitch(table_index: usize, pitch_index: usize) -> i16 {
    let table = sPitchTables[table_index % sPitchTables.len()].0;
    unsafe { table.cast::<i16>().add(pitch_index).read() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWordSoundTemplates(easy_chat_word: u16) -> *const u8 {
    if unsafe { IsBardWordInvalid(easy_chat_word) } != 0 {
        return sEmptyPhonemeTemplate.as_ptr();
    }

    let category = easy_chat_word >> EC_MASK_BITS;
    let subword = usize::from(easy_chat_word & EC_MASK_INDEX);
    let base = match category {
        EC_GROUP_POKEMON | EC_GROUP_POKEMON_NATIONAL => sBardSoundTemplates_Pokemon.as_ptr(),
        EC_GROUP_MOVE_1 | EC_GROUP_MOVE_2 => sBardSoundTemplates_Moves.as_ptr(),
        _ => sBardSoundTemplatesTable[usize::from(category) % sBardSoundTemplatesTable.len()].0,
    };
    base.wrapping_add(subword * WORD_TEMPLATES_SIZE)
}

/// Fills in each sound's length and pitch for the word whose templates are
/// already in `song->soundTemplates`. `pitch_table_index` is 0-4.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalcWordSounds(song: *mut u8, pitch_table_index: u16) {
    let length = unsafe { song.add(SONG_LENGTH).cast::<i16>() };
    unsafe { length.write(0) };
    let templates = unsafe { song.add(SONG_SOUND_TEMPLATES).cast::<*const u8>().read() };
    let phoneme_lengths = sPhonemeLengths.as_ptr().cast::<i32>();

    for i in 0..MAX_BARD_SOUNDS_PER_WORD {
        let template = unsafe { templates.add(i * TEMPLATE_SIZE) };
        let song_id = unsafe { template.read() };
        if song_id == PHONEME_ID_NONE {
            continue;
        }
        let adjustment = i32::from(unsafe { template.add(1).read() } as i8);
        let base_length = unsafe { phoneme_lengths.add(usize::from(song_id)).read() };
        let sound = unsafe { song.add(SONG_SOUNDS + i * SOUND_SIZE).cast::<u16>() };
        let sound_length = adjustment.wrapping_add(base_length) as u16;
        unsafe { sound.write(sound_length) };
        let pitch =
            unsafe { word_pitch(usize::from(pitch_table_index) + BASE_PITCH_TABLE_INDEX, i) };
        unsafe { sound.add(1).write(pitch as u16) };
        unsafe { length.write(length.read().wrapping_add(sound_length as i16)) };
    }
    unsafe { song.add(SONG_SOUND_INDEX).write(0) };
    unsafe { song.add(SONG_VOICE_INFLECTION).cast::<i16>().write(0) };
}
