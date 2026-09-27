//! Game string primitives.
//!
//! Strings are not ASCII: they use the game's own character set (see
//! `charmap.txt`) and are terminated by `EOS` (0xFF) rather than NUL.

use core::ffi::c_int;

pub const EOS: u8 = 0xff;
const CHAR_SPACE: u8 = 0x00;
const CHAR_SPACER: u8 = 0x77;
const CHAR_QUESTION_MARK: u8 = 0xac;
const CHAR_EXTRA_SYMBOL: u8 = 0xf9;
const PLACEHOLDER_BEGIN: u8 = 0xfd;
const CHAR_NEWLINE: u8 = 0xfe;
/// Every Japanese character sorts at or below this code point.
const JAPANESE_CHAR_END: u8 = 0xa0;

const POKEMON_NAME_LENGTH: usize = 10;
const PLAYER_NAME_LENGTH: usize = 7;

const MALE: u8 = 0;
const LANGUAGE_JAPANESE: u8 = 1;
const FONT_BRAILLE: u8 = 6;
const NUM_BRAILLE_CHARS: u8 = 64;

const STR_CONV_MODE_RIGHT_ALIGN: c_int = 1;
const STR_CONV_MODE_LEADING_ZEROS: c_int = 2;

const EXT_CTRL_CODE_BEGIN: u8 = 0xfc;
const EXT_CTRL_CODE_COLOR: u8 = 1;
const EXT_CTRL_CODE_HIGHLIGHT: u8 = 2;
const EXT_CTRL_CODE_SHADOW: u8 = 3;
const EXT_CTRL_CODE_COLOR_HIGHLIGHT_SHADOW: u8 = 4;
const EXT_CTRL_CODE_FONT: u8 = 6;
const EXT_CTRL_CODE_RESET_FONT: u8 = 7;
const EXT_CTRL_CODE_PAUSE_UNTIL_PRESS: u8 = 9;
const EXT_CTRL_CODE_PLAY_BGM: u8 = 11;
const EXT_CTRL_CODE_SHIFT_DOWN: u8 = 14;
const EXT_CTRL_CODE_FILL_WINDOW: u8 = 15;
const EXT_CTRL_CODE_JPN: u8 = 21;
const EXT_CTRL_CODE_ENG: u8 = 22;
const EXT_CTRL_CODE_PAUSE_MUSIC: u8 = 23;
const EXT_CTRL_CODE_RESUME_MUSIC: u8 = 24;

/// `offsetof(struct SaveBlock2, playerName)` and `playerGender`.
const SAVE2_PLAYER_NAME: usize = 0x00;
const SAVE2_PLAYER_GENDER: usize = 0x08;

/// `__("0123456789ABCDEF")` in the game's character set.
static DIGITS: [u8; 16] = [
    0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0,
];

static POWERS_OF_TEN: [i32; 10] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
];

/// Total bytes an extended control code occupies, including the 0xFC lead
/// byte's payload. Index 0 and any unlisted code give 1 and 0 respectively.
static EXT_CTRL_CODE_LENGTHS: [u8; 25] = [
    1, // 0
    2, // COLOR
    2, // HIGHLIGHT
    2, // SHADOW
    4, // COLOR_HIGHLIGHT_SHADOW
    2, // PALETTE
    2, // FONT
    1, // RESET_FONT
    2, // PAUSE
    1, // PAUSE_UNTIL_PRESS
    1, // WAIT_SE
    3, // PLAY_BGM
    2, // ESCAPE
    2, // SHIFT_RIGHT
    2, // SHIFT_DOWN
    1, // FILL_WINDOW
    3, // PLAY_SE
    2, // CLEAR
    2, // SKIP_TO
    2, // CLEAR_TO
    2, // MIN_LETTER_SPACING
    1, // JPN
    1, // ENG
    1, // PAUSE_MUSIC
    1, // RESUME_MUSIC
];

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gStringVar1: crate::ffi::Align4<[u8; 0x100]> = crate::ffi::Align4([0; 0x100]);

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gStringVar2: crate::ffi::Align4<[u8; 0x100]> = crate::ffi::Align4([0; 0x100]);

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gStringVar3: crate::ffi::Align4<[u8; 0x100]> = crate::ffi::Align4([0; 0x100]);

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gStringVar4: crate::ffi::Align4<[u8; 0x3e8]> = crate::ffi::Align4([0; 0x3e8]);

#[unsafe(link_section = "ewram_data")]
static mut UNKNOWN_STRING_VAR: [u8; 16] = [0; 16];

unsafe extern "C" {
    static mut gSaveBlock2Ptr: *mut u8;

    static gText_ExpandedPlaceholder_Empty: u8;
    static gText_ExpandedPlaceholder_Kun: u8;
    static gText_ExpandedPlaceholder_Chan: u8;
    static gText_ExpandedPlaceholder_Brendan: u8;
    static gText_ExpandedPlaceholder_May: u8;
    static gText_ExpandedPlaceholder_Emerald: u8;
    static gText_ExpandedPlaceholder_Aqua: u8;
    static gText_ExpandedPlaceholder_Magma: u8;
    static gText_ExpandedPlaceholder_Archie: u8;
    static gText_ExpandedPlaceholder_Maxie: u8;
    static gText_ExpandedPlaceholder_Kyogre: u8;
    static gText_ExpandedPlaceholder_Groudon: u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringCopy_Nickname(dest: *mut u8, src: *const u8) -> *mut u8 {
    let mut i = 0usize;
    while i < POKEMON_NAME_LENGTH {
        let c = unsafe { src.add(i).read() };
        unsafe { dest.add(i).write(c) };
        if c == EOS {
            return unsafe { dest.add(i) };
        }
        i += 1;
    }

    unsafe { dest.add(i).write(EOS) };
    unsafe { dest.add(i) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringGet_Nickname(string: *mut u8) -> *mut u8 {
    let mut i = 0usize;
    while i < POKEMON_NAME_LENGTH {
        if unsafe { string.add(i).read() } == EOS {
            return unsafe { string.add(i) };
        }
        i += 1;
    }

    unsafe { string.add(i).write(EOS) };
    unsafe { string.add(i) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringCopy_PlayerName(dest: *mut u8, src: *const u8) -> *mut u8 {
    let mut i = 0usize;
    while i < PLAYER_NAME_LENGTH {
        let c = unsafe { src.add(i).read() };
        unsafe { dest.add(i).write(c) };
        if c == EOS {
            return unsafe { dest.add(i) };
        }
        i += 1;
    }

    unsafe { dest.add(i).write(EOS) };
    unsafe { dest.add(i) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringCopy(dest: *mut u8, src: *const u8) -> *mut u8 {
    let mut dest = dest;
    let mut src = src;
    loop {
        let c = unsafe { src.read() };
        if c == EOS {
            break;
        }
        unsafe { dest.write(c) };
        dest = unsafe { dest.add(1) };
        src = unsafe { src.add(1) };
    }
    unsafe { dest.write(EOS) };
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringAppend(dest: *mut u8, src: *const u8) -> *mut u8 {
    let mut dest = dest;
    while unsafe { dest.read() } != EOS {
        dest = unsafe { dest.add(1) };
    }
    unsafe { StringCopy(dest, src) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringCopyN(dest: *mut u8, src: *const u8, n: u8) -> *mut u8 {
    let mut i = 0usize;
    while i < n as usize {
        unsafe { dest.add(i).write(src.add(i).read()) };
        i += 1;
    }
    unsafe { dest.add(n as usize) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringAppendN(dest: *mut u8, src: *const u8, n: u8) -> *mut u8 {
    let mut dest = dest;
    while unsafe { dest.read() } != EOS {
        dest = unsafe { dest.add(1) };
    }
    unsafe { StringCopyN(dest, src, n) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringLength(string: *const u8) -> u16 {
    let mut length = 0u16;
    while unsafe { string.add(length as usize).read() } != EOS {
        length += 1;
    }
    length
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringCompare(str1: *const u8, str2: *const u8) -> i32 {
    let mut str1 = str1;
    let mut str2 = str2;
    while unsafe { str1.read() } == unsafe { str2.read() } {
        if unsafe { str1.read() } == EOS {
            return 0;
        }
        str1 = unsafe { str1.add(1) };
        str2 = unsafe { str2.add(1) };
    }
    i32::from(unsafe { str1.read() }) - i32::from(unsafe { str2.read() })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringCompareN(str1: *const u8, str2: *const u8, n: u32) -> i32 {
    let mut str1 = str1;
    let mut str2 = str2;
    let mut n = n;
    while unsafe { str1.read() } == unsafe { str2.read() } {
        if unsafe { str1.read() } == EOS {
            return 0;
        }
        str1 = unsafe { str1.add(1) };
        str2 = unsafe { str2.add(1) };
        n = n.wrapping_sub(1);
        if n == 0 {
            return 0;
        }
    }
    i32::from(unsafe { str1.read() }) - i32::from(unsafe { str2.read() })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsStringLengthAtLeast(string: *const u8, n: i32) -> u8 {
    let mut i = 0i32;
    while i < n {
        let c = unsafe { string.offset(i as isize).read() };
        if c != 0 && c != EOS {
            return 1;
        }
        i += 1;
    }
    0
}

/// Shared body of the decimal and hexadecimal converters.
///
/// `radix_powers` walks from the largest place value down to one, and the
/// three-state machine decides whether a leading zero is skipped, padded with
/// a spacer, or written out.
unsafe fn convert_to_string(
    dest: *mut u8,
    mut value: u32,
    mode: c_int,
    largest_power: u32,
    radix: u32,
    max_digit: u32,
) -> *mut u8 {
    const WAITING_FOR_NONZERO_DIGIT: u8 = 0;
    const WRITING_DIGITS: u8 = 1;
    const WRITING_SPACES: u8 = 2;

    let mut state = match mode {
        STR_CONV_MODE_RIGHT_ALIGN => WRITING_SPACES,
        STR_CONV_MODE_LEADING_ZEROS => WRITING_DIGITS,
        _ => WAITING_FOR_NONZERO_DIGIT,
    };

    let mut dest = dest;
    let mut power = largest_power;
    while power > 0 {
        let digit = value / power;
        let remainder = value - power * digit;

        let write = state == WRITING_DIGITS || digit != 0 || power == 1;
        if write {
            state = WRITING_DIGITS;
            let c = if digit <= max_digit {
                DIGITS[digit as usize]
            } else {
                CHAR_QUESTION_MARK
            };
            unsafe { dest.write(c) };
            dest = unsafe { dest.add(1) };
        } else if state == WRITING_SPACES {
            unsafe { dest.write(CHAR_SPACER) };
            dest = unsafe { dest.add(1) };
        }

        value = remainder;
        power /= radix;
    }

    unsafe { dest.write(EOS) };
    dest
}

/// `n` is a digit count from 1 to 10; the original indexes
/// `sPowersOfTen[n - 1]` with no check, so it is clamped here instead.
#[inline]
fn largest_power_of_ten(n: u8) -> u32 {
    POWERS_OF_TEN[(n.clamp(1, 10) - 1) as usize] as u32
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertIntToDecimalStringN(
    dest: *mut u8,
    value: i32,
    mode: c_int,
    n: u8,
) -> *mut u8 {
    unsafe { convert_to_string(dest, value as u32, mode, largest_power_of_ten(n), 10, 9) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertUIntToDecimalStringN(
    dest: *mut u8,
    value: u32,
    mode: c_int,
    n: u8,
) -> *mut u8 {
    unsafe { convert_to_string(dest, value, mode, largest_power_of_ten(n), 10, 9) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertIntToHexStringN(
    dest: *mut u8,
    value: i32,
    mode: c_int,
    n: u8,
) -> *mut u8 {
    let mut largest = 1u32;
    let mut i = 1u8;
    while i < n {
        largest = largest.wrapping_mul(16);
        i += 1;
    }
    unsafe { convert_to_string(dest, value as u32, mode, largest, 16, 0xf) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringExpandPlaceholders(dest: *mut u8, src: *const u8) -> *mut u8 {
    let mut dest = dest;
    let mut src = src;

    loop {
        let c = unsafe { src.read() };
        src = unsafe { src.add(1) };

        match c {
            PLACEHOLDER_BEGIN => {
                let placeholder_id = unsafe { src.read() };
                src = unsafe { src.add(1) };
                let expanded = unsafe { GetExpandedPlaceholder(u32::from(placeholder_id)) };
                dest = unsafe { StringExpandPlaceholders(dest, expanded) };
            }
            EXT_CTRL_CODE_BEGIN => {
                unsafe { dest.write(c) };
                dest = unsafe { dest.add(1) };
                let code = unsafe { src.read() };
                src = unsafe { src.add(1) };
                unsafe { dest.write(code) };
                dest = unsafe { dest.add(1) };

                // The original relies on fallthrough: a colour triple copies
                // three more bytes, a BGM id two, and everything unlisted one.
                let extra = match code {
                    EXT_CTRL_CODE_RESET_FONT
                    | EXT_CTRL_CODE_PAUSE_UNTIL_PRESS
                    | EXT_CTRL_CODE_FILL_WINDOW
                    | EXT_CTRL_CODE_JPN
                    | EXT_CTRL_CODE_ENG
                    | EXT_CTRL_CODE_PAUSE_MUSIC
                    | EXT_CTRL_CODE_RESUME_MUSIC => 0,
                    EXT_CTRL_CODE_COLOR_HIGHLIGHT_SHADOW => 3,
                    EXT_CTRL_CODE_PLAY_BGM => 2,
                    _ => 1,
                };
                let mut i = 0;
                while i < extra {
                    unsafe { dest.write(src.read()) };
                    dest = unsafe { dest.add(1) };
                    src = unsafe { src.add(1) };
                    i += 1;
                }
            }
            EOS => {
                unsafe { dest.write(EOS) };
                return dest;
            }
            // CHAR_PROMPT_SCROLL, CHAR_PROMPT_CLEAR and CHAR_NEWLINE are
            // listed as their own cases in the original switch but do exactly
            // what the default does: copy the byte through.
            _ => {
                unsafe { dest.write(c) };
                dest = unsafe { dest.add(1) };
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringBraille(dest: *mut u8, src: *const u8) -> *mut u8 {
    let set_braille_font = [EXT_CTRL_CODE_BEGIN, EXT_CTRL_CODE_FONT, FONT_BRAILLE, EOS];
    let goto_line2 = [
        CHAR_NEWLINE,
        EXT_CTRL_CODE_BEGIN,
        EXT_CTRL_CODE_SHIFT_DOWN,
        2,
        EOS,
    ];

    let mut dest = unsafe { StringCopy(dest, set_braille_font.as_ptr()) };
    let mut src = src;

    loop {
        let c = unsafe { src.read() };
        src = unsafe { src.add(1) };

        match c {
            EOS => {
                unsafe { dest.write(c) };
                return dest;
            }
            CHAR_NEWLINE => {
                dest = unsafe { StringCopy(dest, goto_line2.as_ptr()) };
            }
            _ => {
                // Each braille glyph is drawn as a dot pattern followed by
                // its outline, which sits NUM_BRAILLE_CHARS later.
                unsafe { dest.write(c) };
                dest = unsafe { dest.add(1) };
                unsafe { dest.write(c.wrapping_add(NUM_BRAILLE_CHARS)) };
                dest = unsafe { dest.add(1) };
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetExpandedPlaceholder(id: u32) -> *const u8 {
    let save = unsafe { gSaveBlock2Ptr };
    let gender = unsafe { save.add(SAVE2_PLAYER_GENDER).read() };

    match id {
        0 => (&raw const UNKNOWN_STRING_VAR).cast::<u8>(),
        1 => unsafe { save.add(SAVE2_PLAYER_NAME) },
        2 => (&raw const gStringVar1).cast::<u8>(),
        3 => (&raw const gStringVar2).cast::<u8>(),
        4 => (&raw const gStringVar3).cast::<u8>(),
        5 => {
            if gender == MALE {
                &raw const gText_ExpandedPlaceholder_Kun
            } else {
                &raw const gText_ExpandedPlaceholder_Chan
            }
        }
        // The rival is the opposite gender to the player.
        6 => {
            if gender == MALE {
                &raw const gText_ExpandedPlaceholder_May
            } else {
                &raw const gText_ExpandedPlaceholder_Brendan
            }
        }
        7 => &raw const gText_ExpandedPlaceholder_Emerald,
        8 => &raw const gText_ExpandedPlaceholder_Aqua,
        9 => &raw const gText_ExpandedPlaceholder_Magma,
        10 => &raw const gText_ExpandedPlaceholder_Archie,
        11 => &raw const gText_ExpandedPlaceholder_Maxie,
        12 => &raw const gText_ExpandedPlaceholder_Kyogre,
        13 => &raw const gText_ExpandedPlaceholder_Groudon,
        _ => &raw const gText_ExpandedPlaceholder_Empty,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringFill(dest: *mut u8, c: u8, n: u16) -> *mut u8 {
    let mut dest = dest;
    let mut i = 0u16;
    while i < n {
        unsafe { dest.write(c) };
        dest = unsafe { dest.add(1) };
        i += 1;
    }
    unsafe { dest.write(EOS) };
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringCopyPadded(dest: *mut u8, src: *const u8, c: u8, n: u16) -> *mut u8 {
    let mut dest = dest;
    let mut src = src;
    let mut n = n;

    while unsafe { src.read() } != EOS {
        unsafe { dest.write(src.read()) };
        dest = unsafe { dest.add(1) };
        src = unsafe { src.add(1) };
        if n != 0 {
            n -= 1;
        }
    }

    // The counter is decremented once more and compared against the u16
    // wraparound, so a zero remainder writes no padding at all.
    n = n.wrapping_sub(1);
    while n != u16::MAX {
        unsafe { dest.write(c) };
        dest = unsafe { dest.add(1) };
        n = n.wrapping_sub(1);
    }

    unsafe { dest.write(EOS) };
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringFillWithTerminator(dest: *mut u8, n: u16) -> *mut u8 {
    unsafe { StringFill(dest, EOS, n) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringCopyN_Multibyte(dest: *mut u8, src: *mut u8, n: u32) -> *mut u8 {
    let mut dest = dest;
    let mut src = src;

    let mut i = n.wrapping_sub(1);
    while i != u32::MAX {
        if unsafe { src.read() } == EOS {
            break;
        }
        let c = unsafe { src.read() };
        unsafe { dest.write(c) };
        dest = unsafe { dest.add(1) };
        src = unsafe { src.add(1) };
        if c == CHAR_EXTRA_SYMBOL {
            unsafe { dest.write(src.read()) };
            dest = unsafe { dest.add(1) };
            src = unsafe { src.add(1) };
        }
        i = i.wrapping_sub(1);
    }

    unsafe { dest.write(EOS) };
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringLength_Multibyte(string: *const u8) -> u32 {
    let mut string = string;
    let mut length = 0u32;

    while unsafe { string.read() } != EOS {
        if unsafe { string.read() } == CHAR_EXTRA_SYMBOL {
            string = unsafe { string.add(1) };
        }
        string = unsafe { string.add(1) };
        length += 1;
    }

    length
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WriteColorChangeControlCode(
    dest: *mut u8,
    color_type: u32,
    color: u8,
) -> *mut u8 {
    let mut dest = dest;
    unsafe { dest.write(EXT_CTRL_CODE_BEGIN) };
    dest = unsafe { dest.add(1) };

    let code = match color_type {
        0 => Some(EXT_CTRL_CODE_COLOR),
        1 => Some(EXT_CTRL_CODE_SHADOW),
        2 => Some(EXT_CTRL_CODE_HIGHLIGHT),
        _ => None,
    };
    if let Some(code) = code {
        unsafe { dest.write(code) };
        dest = unsafe { dest.add(1) };
    }

    unsafe { dest.write(color) };
    dest = unsafe { dest.add(1) };
    unsafe { dest.write(EOS) };
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsStringJapanese(string: *mut u8) -> u32 {
    let mut string = string;
    while unsafe { string.read() } != EOS {
        let c = unsafe { string.read() };
        if c <= JAPANESE_CHAR_END && c != CHAR_SPACE {
            return 1;
        }
        string = unsafe { string.add(1) };
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsStringNJapanese(string: *mut u8, n: i32) -> u32 {
    let mut string = string;
    let mut i = 0i32;
    while unsafe { string.read() } != EOS && i < n {
        let c = unsafe { string.read() };
        if c <= JAPANESE_CHAR_END && c != CHAR_SPACE {
            return 1;
        }
        string = unsafe { string.add(1) };
        i += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetExtCtrlCodeLength(code: u8) -> u8 {
    if (code as usize) < EXT_CTRL_CODE_LENGTHS.len() {
        EXT_CTRL_CODE_LENGTHS[code as usize]
    } else {
        0
    }
}

unsafe fn skip_ext_ctrl_code(string: *const u8) -> *const u8 {
    let mut string = string;
    while unsafe { string.read() } == EXT_CTRL_CODE_BEGIN {
        string = unsafe { string.add(1) };
        string = unsafe { string.add(GetExtCtrlCodeLength(string.read()) as usize) };
    }
    string
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringCompareWithoutExtCtrlCodes(str1: *const u8, str2: *const u8) -> i32 {
    let mut str1 = str1;
    let mut str2 = str2;
    let mut result = 0i32;

    loop {
        str1 = unsafe { skip_ext_ctrl_code(str1) };
        str2 = unsafe { skip_ext_ctrl_code(str2) };

        let c1 = unsafe { str1.read() };
        let c2 = unsafe { str2.read() };

        if c1 > c2 {
            break;
        }
        if c1 < c2 {
            result = if c2 == EOS { 1 } else { -1 };
        }
        if c1 == EOS {
            return result;
        }

        str1 = unsafe { str1.add(1) };
        str2 = unsafe { str2.add(1) };
    }

    if unsafe { str1.read() } == EOS { -1 } else { 1 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertInternationalString(string: *mut u8, language: u8) {
    if language != LANGUAGE_JAPANESE {
        return;
    }

    unsafe { StripExtCtrlCodes(string) };
    let mut i = unsafe { StringLength(string) } as u8;
    unsafe { string.add(i as usize).write(EXT_CTRL_CODE_BEGIN) };
    i = i.wrapping_add(1);
    unsafe { string.add(i as usize).write(EXT_CTRL_CODE_ENG) };
    i = i.wrapping_add(1);
    unsafe { string.add(i as usize).write(EOS) };

    // Shift everything two bytes along to make room for the language marker.
    i = i.wrapping_sub(1);
    while i != u8::MAX {
        unsafe {
            string
                .add(i as usize + 2)
                .write(string.add(i as usize).read())
        };
        i = i.wrapping_sub(1);
    }

    unsafe { string.write(EXT_CTRL_CODE_BEGIN) };
    unsafe { string.add(1).write(EXT_CTRL_CODE_JPN) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StripExtCtrlCodes(string: *mut u8) {
    let mut src_index = 0usize;
    let mut dest_index = 0usize;

    while unsafe { string.add(src_index).read() } != EOS {
        if unsafe { string.add(src_index).read() } == EXT_CTRL_CODE_BEGIN {
            src_index += 1;
            src_index += unsafe { GetExtCtrlCodeLength(string.add(src_index).read()) } as usize;
        } else {
            let c = unsafe { string.add(src_index).read() };
            unsafe { string.add(dest_index).write(c) };
            dest_index += 1;
            src_index += 1;
        }
    }

    unsafe { string.add(dest_index).write(EOS) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_use_the_games_character_set_not_ascii() {
        // '0' is 0xA1 and 'A' is 0xBB in charmap.txt.
        assert_eq!(DIGITS[0], 0xa1);
        assert_eq!(DIGITS[9], 0xaa);
        assert_eq!(DIGITS[10], 0xbb);
        assert_eq!(DIGITS[15], 0xc0);
        for i in 0..10 {
            assert_eq!(DIGITS[i], 0xa1 + i as u8);
        }
    }

    #[test]
    fn powers_of_ten_cover_every_digit_count() {
        assert_eq!(POWERS_OF_TEN.len(), 10);
        assert_eq!(largest_power_of_ten(1), 1);
        assert_eq!(largest_power_of_ten(6), 100_000);
        assert_eq!(largest_power_of_ten(10), 1_000_000_000);
        // Out-of-range digit counts are clamped rather than read out of bounds.
        assert_eq!(largest_power_of_ten(0), 1);
        assert_eq!(largest_power_of_ten(255), 1_000_000_000);
    }

    #[test]
    fn control_code_lengths_match_the_designated_initialisers() {
        assert_eq!(EXT_CTRL_CODE_LENGTHS.len(), 25);
        assert_eq!(unsafe { GetExtCtrlCodeLength(0) }, 1);
        assert_eq!(unsafe { GetExtCtrlCodeLength(EXT_CTRL_CODE_COLOR) }, 2);
        assert_eq!(
            unsafe { GetExtCtrlCodeLength(EXT_CTRL_CODE_COLOR_HIGHLIGHT_SHADOW) },
            4
        );
        assert_eq!(unsafe { GetExtCtrlCodeLength(EXT_CTRL_CODE_PLAY_BGM) }, 3);
        assert_eq!(
            unsafe { GetExtCtrlCodeLength(EXT_CTRL_CODE_RESUME_MUSIC) },
            1
        );
        // Anything past the table is zero length.
        assert_eq!(unsafe { GetExtCtrlCodeLength(25) }, 0);
        assert_eq!(unsafe { GetExtCtrlCodeLength(0xff) }, 0);
    }

    #[test]
    fn decimal_conversion_matches_each_alignment_mode() {
        let mut buffer = [0u8; 16];

        let end = unsafe { ConvertIntToDecimalStringN(buffer.as_mut_ptr(), 42, 0, 4) };
        assert_eq!(&buffer[..2], &[DIGITS[4], DIGITS[2]]);
        assert_eq!(buffer[2], EOS);
        assert_eq!(unsafe { end.offset_from(buffer.as_mut_ptr()) }, 2);

        let _ = unsafe {
            ConvertIntToDecimalStringN(buffer.as_mut_ptr(), 42, STR_CONV_MODE_RIGHT_ALIGN, 4)
        };
        assert_eq!(
            &buffer[..5],
            &[CHAR_SPACER, CHAR_SPACER, DIGITS[4], DIGITS[2], EOS]
        );

        let _ = unsafe {
            ConvertIntToDecimalStringN(buffer.as_mut_ptr(), 42, STR_CONV_MODE_LEADING_ZEROS, 4)
        };
        assert_eq!(
            &buffer[..5],
            &[DIGITS[0], DIGITS[0], DIGITS[4], DIGITS[2], EOS]
        );
    }

    #[test]
    fn zero_always_prints_a_single_digit() {
        let mut buffer = [0u8; 16];
        let _ = unsafe { ConvertIntToDecimalStringN(buffer.as_mut_ptr(), 0, 0, 6) };
        assert_eq!(&buffer[..2], &[DIGITS[0], EOS]);
    }

    #[test]
    fn hex_conversion_uses_the_letter_digits() {
        let mut buffer = [0u8; 16];
        let _ = unsafe { ConvertIntToHexStringN(buffer.as_mut_ptr(), 0xbeef, 0, 4) };
        assert_eq!(
            &buffer[..5],
            &[DIGITS[0xb], DIGITS[0xe], DIGITS[0xe], DIGITS[0xf], EOS]
        );
    }

    #[test]
    fn copy_and_length_agree_on_eos_termination() {
        let src = [1u8, 2, 3, EOS];
        let mut dest = [0u8; 8];
        let end = unsafe { StringCopy(dest.as_mut_ptr(), src.as_ptr()) };
        assert_eq!(&dest[..4], &[1, 2, 3, EOS]);
        assert_eq!(unsafe { end.offset_from(dest.as_mut_ptr()) }, 3);
        assert_eq!(unsafe { StringLength(dest.as_ptr()) }, 3);
    }

    #[test]
    fn stripping_control_codes_removes_the_whole_sequence() {
        // 0xFC 0x01 0x02 is a two-byte colour code, so three bytes go away.
        let mut string = [
            EXT_CTRL_CODE_BEGIN,
            EXT_CTRL_CODE_COLOR,
            0x02,
            0xbb,
            0xbc,
            EOS,
        ];
        unsafe { StripExtCtrlCodes(string.as_mut_ptr()) };
        assert_eq!(&string[..3], &[0xbb, 0xbc, EOS]);
    }
}
