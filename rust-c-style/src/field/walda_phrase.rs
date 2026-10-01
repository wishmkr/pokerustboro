//! Walda's phrase. A fifteen-letter phrase, restricted to a 32-letter
//! alphabet, encodes a storage-box wallpaper; the phrase only "works" if the
//! encoded data happens to check out against the player's trainer id.

use crate::ffi::{
    EOS, StringCompare, StringCopy, StringLength, gSpecialVar_0x8004, gSpecialVar_Result,
    gStringVar1, gStringVar2,
};

/// Thirty-two allowed letters, so each contributes five bits.
const BITS_PER_LETTER: usize = 5;
const WALDA_PHRASE_LENGTH: usize = 15;
const NAMING_SCREEN_WALDA: u8 = 4;

/// `offsetof(struct SaveBlock2, playerTrainerId)`
const SAVE2_TRAINER_ID: usize = 0x0a;

/// The nine-byte scratch array the wallpaper is calculated in.
const NUM_WALLPAPER_DATA_BYTES: usize = 9;
const BG_COLOR_LO: usize = 0;
const BG_COLOR_HI: usize = 1;
const FG_COLOR_LO: usize = 2;
const FG_COLOR_HI: usize = 3;
const ICON_ID: usize = 4;
const PATTERN_ID: usize = 5;
const TID_CHECK_HI: usize = 6;
const TID_CHECK_LO: usize = 7;
const KEY: usize = 8;

const PHRASE_CHANGED: u16 = 0;
const PHRASE_NO_CHANGE: u16 = 1;
const PHRASE_EMPTY: u16 = 2;

/// The letters a working phrase may use, in the game's character set.
/// Every vowel is excluded, as are X/x, Y/y, l, r, t, v, w and z.
static WALDA_LETTERS: [u8; 1 << BITS_PER_LETTER] = [
    0xbc, 0xbd, 0xbe, 0xc0, 0xc1, 0xc2, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xca, 0xcb, 0xcc, 0xcd, 0xce,
    0xd0, 0xd1, 0xd4, 0xd6, 0xd7, 0xd8, 0xda, 0xdb, 0xdc, 0xde, 0xdf, 0xe1, 0xe2, 0xe4, 0xe5, 0xe7,
];

type MainCallback = unsafe extern "C" fn();

unsafe extern "C" {
    static mut gSaveBlock2Ptr: *mut u8;
    static mut gFieldCallback: Option<MainCallback>;
    static gText_Peekaboo: u8;

    fn GetTrainerId(trainer_id: *const u8) -> u16;
    fn DoNamingScreen(
        template_num: u8,
        destination: *mut u8,
        mon_species: u16,
        mon_gender: u16,
        personality: u32,
        callback: MainCallback,
    );
    fn SetMainCallback2(callback: MainCallback);
    fn CB2_ReturnToField();
    fn FieldCB_ContinueScriptHandleMusic();

    fn GetWaldaPhrasePtr() -> *mut u8;
    fn IsWaldaPhraseEmpty() -> u32;
    fn SetWaldaPhrase(phrase: *const u8);
    fn SetWaldaWallpaperPatternId(pattern_id: u8);
    fn SetWaldaWallpaperIconId(icon_id: u8);
    fn SetWaldaWallpaperColors(background: u16, foreground: u16);
    fn SetWaldaWallpaperLockedOrUnlocked(unlocked: u32);
}

/// Converts a position in the phrase to a bit number into the letter array.
#[inline]
const fn to_bit_offset(i: usize) -> usize {
    3 + 8 * i
}

#[inline]
fn data_bit(data: &[u8], bit_num: usize) -> bool {
    // Bits are numbered from the most significant end of each byte.
    let flag = 0x80u8 >> (bit_num % 8);
    data[bit_num / 8] & flag != 0
}

#[inline]
fn set_data_bit(data: &mut [u8], bit_num: usize, value: bool) {
    let flag = 0x80u8 >> (bit_num % 8);
    let index = bit_num / 8;
    if value {
        data[index] |= flag;
    } else {
        data[index] &= !flag;
    }
}

fn data_bits(data: &[u8], offset: usize, num_bits: usize) -> u32 {
    let mut bits = 0u32;
    for i in 0..num_bits {
        bits <<= 1;
        bits |= u32::from(data_bit(data, offset + i));
    }
    bits
}

fn copy_bits(
    data: &mut [u8],
    letters: &[u8],
    set_offset: usize,
    get_offset: usize,
    num_bits: usize,
) {
    for i in 0..num_bits {
        set_data_bit(data, set_offset + i, data_bit(letters, get_offset + i));
    }
}

/// Rotates the first `size` bytes left by `num_shifts` bits, carrying the top
/// bit of byte 0 around into the bottom.
fn rotate_left(data: &mut [u8], size: usize, num_shifts: u32) {
    for _ in 0..num_shifts {
        let mut carry = (data[0] & 0x80) >> 7;
        let mut j = size;
        while j > 0 {
            j -= 1;
            let next = (data[j] & 0x80) >> 7;
            data[j] = (data[j] << 1) | carry;
            carry = next;
        }
    }
}

fn mask_data(data: &mut [u8], size: usize, mask: u8) {
    let mask = mask | (mask << 4);
    for byte in data.iter_mut().take(size) {
        *byte ^= mask;
    }
}

fn letter_table_id(letter: u8) -> u8 {
    let mut i = 0usize;
    while i < WALDA_LETTERS.len() {
        if WALDA_LETTERS[i] == letter {
            return i as u8;
        }
        i += 1;
    }
    WALDA_LETTERS.len() as u8
}

/// Builds a wallpaper from the phrase and trainer id, or reports that Walda
/// "didn't like" the phrase.
unsafe fn try_calculate_wallpaper(
    trainer_id: u16,
    phrase: *const u8,
) -> Option<(u16, u16, u8, u8)> {
    if unsafe { StringLength(phrase) } as usize != WALDA_PHRASE_LENGTH {
        return None;
    }

    let mut letters = [0u8; WALDA_PHRASE_LENGTH];
    for i in 0..WALDA_PHRASE_LENGTH {
        letters[i] = letter_table_id(unsafe { phrase.add(i).read() });
        if letters[i] as usize == WALDA_LETTERS.len() {
            return None;
        }
    }

    // Nine bytes is 72 bits, but fifteen letters carry 75. The first fourteen
    // fill bits 0..69, the last contributes only its first two, and its
    // remaining three have to already match the array's first three.
    let mut data = [0u8; NUM_WALLPAPER_DATA_BYTES];
    for i in 0..WALDA_PHRASE_LENGTH - 1 {
        copy_bits(
            &mut data,
            &letters,
            BITS_PER_LETTER * i,
            to_bit_offset(i),
            BITS_PER_LETTER,
        );
    }
    let last = WALDA_PHRASE_LENGTH - 1;
    copy_bits(
        &mut data,
        &letters,
        BITS_PER_LETTER * last,
        to_bit_offset(last),
        2,
    );

    if data_bits(&data, 0, 3) != data_bits(&letters, to_bit_offset(last) + 2, 3) {
        return None;
    }

    // Arbitrary scrambling driven by the last byte.
    rotate_left(&mut data, NUM_WALLPAPER_DATA_BYTES, 21);
    let key = data[KEY];
    rotate_left(
        &mut data,
        NUM_WALLPAPER_DATA_BYTES - 1,
        u32::from(key & 0xf),
    );
    mask_data(&mut data, NUM_WALLPAPER_DATA_BYTES - 1, key >> 4);

    // The result only counts if it checks out against the trainer id.
    if data[TID_CHECK_HI]
        != (data[BG_COLOR_LO] ^ data[FG_COLOR_LO] ^ data[ICON_ID] ^ (trainer_id >> 8) as u8)
    {
        return None;
    }
    if data[TID_CHECK_LO]
        != (data[BG_COLOR_HI] ^ data[FG_COLOR_HI] ^ data[PATTERN_ID] ^ (trainer_id & 0xff) as u8)
    {
        return None;
    }

    let background = u16::from_le_bytes([data[BG_COLOR_LO], data[BG_COLOR_HI]]);
    let foreground = u16::from_le_bytes([data[FG_COLOR_LO], data[FG_COLOR_HI]]);
    Some((background, foreground, data[ICON_ID], data[PATTERN_ID]))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryBufferWaldaPhrase() -> u16 {
    if unsafe { IsWaldaPhraseEmpty() } != 0 {
        return 0;
    }
    let _ = unsafe { StringCopy((&raw mut gStringVar1).cast::<u8>(), GetWaldaPhrasePtr()) };
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoWaldaNamingScreen() {
    let buffer = (&raw mut gStringVar2).cast::<u8>();
    let _ = unsafe { StringCopy(buffer, GetWaldaPhrasePtr()) };
    unsafe {
        DoNamingScreen(
            NAMING_SCREEN_WALDA,
            buffer,
            0,
            0,
            0,
            cb2_handle_given_walda_phrase,
        )
    };
}

unsafe fn walda_phrase_input_case(input: *mut u8) -> u16 {
    if unsafe { input.read() } == EOS {
        return PHRASE_EMPTY;
    }
    if unsafe { StringCompare(input, GetWaldaPhrasePtr()) } == 0 {
        return PHRASE_NO_CHANGE;
    }
    PHRASE_CHANGED
}

unsafe extern "C" fn cb2_handle_given_walda_phrase() {
    let buffer = (&raw mut gStringVar2).cast::<u8>();
    let mut case = unsafe { walda_phrase_input_case(buffer) };

    match case {
        PHRASE_EMPTY => {
            // Nothing typed: fall back to the default phrase only if there
            // is no saved one to keep.
            if unsafe { IsWaldaPhraseEmpty() } != 0 {
                unsafe { SetWaldaPhrase(&raw const gText_Peekaboo) };
            } else {
                case = PHRASE_NO_CHANGE;
            }
        }
        PHRASE_CHANGED => unsafe { SetWaldaPhrase(buffer) },
        _ => {}
    }
    unsafe { (&raw mut gSpecialVar_0x8004).write_volatile(case) };

    let _ = unsafe { StringCopy((&raw mut gStringVar1).cast::<u8>(), GetWaldaPhrasePtr()) };
    unsafe { (&raw mut gFieldCallback).write(Some(FieldCB_ContinueScriptHandleMusic)) };
    unsafe { SetMainCallback2(CB2_ReturnToField) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryGetWallpaperWithWaldaPhrase() -> u16 {
    let trainer_id = unsafe { GetTrainerId(gSaveBlock2Ptr.add(SAVE2_TRAINER_ID)) };
    let result = unsafe { try_calculate_wallpaper(trainer_id, GetWaldaPhrasePtr()) };

    let success = u16::from(result.is_some());
    unsafe { (&raw mut gSpecialVar_Result).write_volatile(success) };

    if let Some((background, foreground, icon_id, pattern_id)) = result {
        unsafe { SetWaldaWallpaperPatternId(pattern_id) };
        unsafe { SetWaldaWallpaperIconId(icon_id) };
        unsafe { SetWaldaWallpaperColors(background, foreground) };
    }

    unsafe { SetWaldaWallpaperLockedOrUnlocked(u32::from(success)) };
    success
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_alphabet_has_exactly_thirty_two_distinct_letters() {
        assert_eq!(WALDA_LETTERS.len(), 1 << BITS_PER_LETTER);
        for i in 0..WALDA_LETTERS.len() {
            for j in (i + 1)..WALDA_LETTERS.len() {
                assert_ne!(WALDA_LETTERS[i], WALDA_LETTERS[j]);
            }
        }
        // An unknown letter reports the sentinel index.
        assert_eq!(letter_table_id(0x00), 32);
        assert_eq!(letter_table_id(WALDA_LETTERS[0]), 0);
        assert_eq!(letter_table_id(WALDA_LETTERS[31]), 31);
    }

    #[test]
    fn fifteen_letters_just_overflow_the_nine_byte_array() {
        // 15 * 5 = 75 bits into 72, so the last letter only fits two.
        assert_eq!(WALDA_PHRASE_LENGTH * BITS_PER_LETTER, 75);
        assert_eq!(NUM_WALLPAPER_DATA_BYTES * 8, 72);
        // The last letter's bits must stay inside the letter array.
        assert!(to_bit_offset(WALDA_PHRASE_LENGTH - 1) + 5 <= WALDA_PHRASE_LENGTH * 8);
    }

    #[test]
    fn bits_are_numbered_from_the_top_of_each_byte() {
        let data = [0b1000_0000u8, 0b0000_0001];
        assert!(data_bit(&data, 0));
        assert!(!data_bit(&data, 1));
        assert!(data_bit(&data, 15));
        assert_eq!(data_bits(&data, 0, 2), 0b10);
        assert_eq!(data_bits(&data, 14, 2), 0b01);
    }

    #[test]
    fn rotating_by_a_whole_byte_shifts_the_bytes_along() {
        let mut data = [0x12u8, 0x34, 0x56];
        rotate_left(&mut data, 3, 8);
        assert_eq!(data, [0x34, 0x56, 0x12]);
        // A full turn is the identity.
        let mut data = [0x12u8, 0x34, 0x56];
        rotate_left(&mut data, 3, 24);
        assert_eq!(data, [0x12, 0x34, 0x56]);
    }

    #[test]
    fn masking_is_its_own_inverse() {
        let original = [0x12u8, 0x34, 0x56, 0x78];
        let mut data = original;
        mask_data(&mut data, 4, 0x0b);
        assert_ne!(data, original);
        mask_data(&mut data, 4, 0x0b);
        assert_eq!(data, original);
    }
}
