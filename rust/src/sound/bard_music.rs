//! The Mauville bard's song: turning easy chat words into phoneme sounds.
//! The phoneme tables themselves are in `data/bard_music.rs`.

use crate::data::bard_music::{
    sBardSoundTemplates_Moves, sBardSoundTemplates_Pokemon, sBardSoundTemplatesTable,
    sEmptyPhonemeTemplate, sPhonemeLengths, sPitchTables,
};
use crate::types::{BardSong, BardSoundTemplate};

const MAX_BARD_SOUNDS_PER_WORD: usize = 6;
const NUM_BARD_PITCH_TABLES_PER_SIZE: usize = 5;
/// The pitch tables are chosen one size larger than the most sounds a word
/// can have (as in C; the last pitch goes unused).
const BASE_PITCH_TABLE_INDEX: usize = NUM_BARD_PITCH_TABLES_PER_SIZE * MAX_BARD_SOUNDS_PER_WORD;
const PHONEME_ID_NONE: u8 = 0xff;

const EC_MASK_BITS: u32 = 9;
const EC_MASK_INDEX: u16 = (1 << EC_MASK_BITS) - 1;
const EC_GROUP_POKEMON: u16 = 0x00;
const EC_GROUP_MOVE_1: u16 = 0x12;
const EC_GROUP_MOVE_2: u16 = 0x13;
const EC_GROUP_POKEMON_NATIONAL: u16 = 0x15;

/// A word's sounds: one template per possible sound.
type WordTemplates = [BardSoundTemplate; MAX_BARD_SOUNDS_PER_WORD];
/// A pitch table of the size [`BASE_PITCH_TABLE_INDEX`] picks: a pitch per
/// sound, one spare, and `PITCH_END`.
type PitchTable = [i16; MAX_BARD_SOUNDS_PER_WORD + 2];

/// `IsBardWordInvalid` with this module's view of its types.
#[inline]
unsafe fn IsBardWordInvalid(a0: u16) -> u8 {
    unsafe { crate::easy_chat::IsBardWordInvalid(a0) }
}

/// The sound templates for an easy chat word. Like C, this doesn't check
/// the word's index against its group's table.
pub fn word_sound_templates(easy_chat_word: u16) -> *const BardSoundTemplate {
    // SAFETY: IsBardWordInvalid only looks the word up.
    if unsafe { IsBardWordInvalid(easy_chat_word) } != 0 {
        return sEmptyPhonemeTemplate.as_ptr().cast();
    }
    let group = easy_chat_word >> EC_MASK_BITS;
    let index = usize::from(easy_chat_word & EC_MASK_INDEX);
    let words: *const WordTemplates = match group {
        EC_GROUP_POKEMON | EC_GROUP_POKEMON_NATIONAL => sBardSoundTemplates_Pokemon.as_ptr().cast(),
        EC_GROUP_MOVE_1 | EC_GROUP_MOVE_2 => sBardSoundTemplates_Moves.as_ptr().cast(),
        _ => sBardSoundTemplatesTable
            .get(usize::from(group))
            .map_or(core::ptr::null(), |t| t.0.cast()),
    };
    words.wrapping_add(index).cast()
}

/// Fills in each sound's length and pitch for the word whose templates are
/// already in `song.soundTemplates`, with pitch table `pitch_table_index`
/// (0-4).
pub fn calc_word_sounds(song: &mut BardSong, pitch_table_index: u16) {
    let pitches = sPitchTables
        .get(usize::from(pitch_table_index) + BASE_PITCH_TABLE_INDEX)
        // SAFETY: these tables all hold a PitchTable.
        .map(|t| unsafe { &*t.0.cast::<PitchTable>() });
    // SAFETY: the templates come from word_sound_templates: a whole word's.
    let templates = unsafe { &*song.soundTemplates.cast::<WordTemplates>() };
    // SAFETY: sPhonemeLengths is an s32 per phoneme.
    let phoneme_lengths = unsafe { &*sPhonemeLengths.as_ptr().cast::<[i32; 52]>() };

    song.length = 0;
    for (i, template) in templates.iter().enumerate() {
        if template.songId == PHONEME_ID_NONE {
            continue;
        }
        let base = phoneme_lengths
            .get(usize::from(template.songId))
            .copied()
            .unwrap_or(0);
        let length = i32::from(template.lengthAdjustment).wrapping_add(base) as u16;
        let pitch = pitches.and_then(|p| p.get(i)).copied().unwrap_or(0);
        song.sounds[i].length = length;
        song.sounds[i].pitch = pitch as u16;
        song.length = song.length.wrapping_add(length as i16);
    }
    song.soundIndex = 0;
    song.voiceInflection = 0;
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub fn GetWordSoundTemplates(easy_chat_word: u16) -> *const BardSoundTemplate {
    word_sound_templates(easy_chat_word)
}

/// # Safety
/// `song` must be a valid song whose templates are a word's.
#[unsafe(no_mangle)]
pub unsafe fn CalcWordSounds(song: *mut BardSong, pitch_table_index: u16) {
    // SAFETY: the caller's promise.
    calc_word_sounds(unsafe { &mut *song }, pitch_table_index);
}
