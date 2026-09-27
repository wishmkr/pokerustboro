use crate::ffi::{
    ConvertInternationalString, FONT_NORMAL, StringCopy, StringLength, StripExtCtrlCodes, gWindows,
};
use core::ffi::{c_int, c_void};

const EOS: u8 = 0xff;
const CHAR_SPACE: u8 = 0x00;
const EXT_CTRL_CODE_BEGIN: u8 = 0xfc;
const EXT_CTRL_CODE_RESET_FONT: u8 = 7;
const EXT_CTRL_CODE_CLEAR: u8 = 17;
const EXT_CTRL_CODE_JPN: u8 = 21;
const EXT_CTRL_CODE_ENG: u8 = 22;

const LANGUAGE_JAPANESE: c_int = 1;
const LANGUAGE_ENGLISH: c_int = 2;

const PLAYER_NAME_LENGTH: u16 = 7;
const TILE_SIZE_4BPP: usize = 32;

/// A list menu is never allowed to grow past the screen.
const MAX_LIST_MENU_TILE_WIDTH: c_int = 28;

/// `sizeof(struct PokedexEntry)`; `categoryName` is its first field.
const POKEDEX_ENTRY_SIZE: usize = 32;

/// `sizeof(struct MenuAction)`; `text` is its first field.
const MENU_ACTION_SIZE: usize = 8;
/// `sizeof(struct ListMenuItem)`; `name` is its first field.
const LIST_MENU_ITEM_SIZE: usize = 8;

/// Offsets inside `struct ListMenuTemplate`.
const LIST_MENU_ITEMS: usize = 0;
const LIST_MENU_TOTAL_ITEMS: usize = 12;
const LIST_MENU_ITEM_X: usize = 18;
/// `fontId:6` occupies the low six bits of this byte.
const LIST_MENU_FONT_ID: usize = 23;
const LIST_MENU_FONT_ID_MASK: u8 = 0x3f;

/// `CPU_FAST_SET_SRC_FIXED`
const CPU_FAST_SET_SRC_FIXED: u32 = 0x0100_0000;

unsafe extern "C" {
    static gPokedexEntries: u8;
    static gText_Pokemon: u8;

    fn GetStringWidth(font_id: u8, string: *const u8, letter_spacing: i16) -> i32;
    fn ConvertPixelWidthToTileWidth(width: c_int) -> c_int;
    fn CpuFastSet(src: *const c_void, dest: *mut c_void, control: u32);
}

/// `CpuFastFill8(value, dest, size)`
unsafe fn cpu_fast_fill8(value: u8, dest: *mut u8, size: usize) {
    let temp = u32::from_ne_bytes([value; 4]);
    unsafe {
        CpuFastSet(
            (&raw const temp).cast(),
            dest.cast(),
            CPU_FAST_SET_SRC_FIXED | (size as u32 / 4 & 0x1f_ffff),
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetStringWidthDifference(
    font_id: c_int,
    string: *const u8,
    total_width: c_int,
    letter_spacing: c_int,
) -> c_int {
    let string_width = unsafe { GetStringWidth(font_id as u8, string, letter_spacing as i16) };
    if total_width > string_width {
        total_width - string_width
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetStringCenterAlignXOffsetWithLetterSpacing(
    font_id: c_int,
    string: *const u8,
    total_width: c_int,
    letter_spacing: c_int,
) -> c_int {
    let difference =
        unsafe { GetStringWidthDifference(font_id, string, total_width, letter_spacing) };
    difference / 2
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetStringCenterAlignXOffset(
    font_id: c_int,
    string: *const u8,
    total_width: c_int,
) -> c_int {
    unsafe { GetStringCenterAlignXOffsetWithLetterSpacing(font_id, string, total_width, 0) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetStringRightAlignXOffset(
    font_id: c_int,
    string: *const u8,
    total_width: c_int,
) -> c_int {
    unsafe { GetStringWidthDifference(font_id, string, total_width, 0) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMaxWidthInMenuTable(actions: *const u8, num_actions: c_int) -> c_int {
    let mut max_width = 0i32;
    let mut index = 0isize;
    while index < num_actions as isize {
        let text = unsafe {
            actions
                .offset(index * MENU_ACTION_SIZE as isize)
                .cast::<*const u8>()
                .read()
        };
        let width = unsafe { GetStringWidth(FONT_NORMAL, text, 0) };
        if width > max_width {
            max_width = width;
        }
        index += 1;
    }

    unsafe { ConvertPixelWidthToTileWidth(max_width) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMaxWidthInSubsetOfMenuTable(
    actions: *const u8,
    action_ids: *const u8,
    num_actions: c_int,
) -> c_int {
    let mut max_width = 0i32;
    let mut index = 0isize;
    while index < num_actions as isize {
        let action_id = unsafe { action_ids.offset(index).read() } as isize;
        let text = unsafe {
            actions
                .offset(action_id * MENU_ACTION_SIZE as isize)
                .cast::<*const u8>()
                .read()
        };
        let width = unsafe { GetStringWidth(FONT_NORMAL, text, 0) };
        if width > max_width {
            max_width = width;
        }
        index += 1;
    }

    unsafe { ConvertPixelWidthToTileWidth(max_width) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Intl_GetListMenuWidth(list_menu: *const u8) -> c_int {
    let items = unsafe { list_menu.add(LIST_MENU_ITEMS).cast::<*const u8>().read() };
    let total_items = unsafe { list_menu.add(LIST_MENU_TOTAL_ITEMS).cast::<u16>().read() };
    let font_id = unsafe { list_menu.add(LIST_MENU_FONT_ID).read() } & LIST_MENU_FONT_ID_MASK;
    let item_x = unsafe { list_menu.add(LIST_MENU_ITEM_X).read() };

    let mut max_width = 0i32;
    let mut index = 0usize;
    while index < total_items as usize {
        let name = unsafe {
            items
                .add(index * LIST_MENU_ITEM_SIZE)
                .cast::<*const u8>()
                .read()
        };
        let width = unsafe { GetStringWidth(font_id, name, 0) };
        if width > max_width {
            max_width = width;
        }
        index += 1;
    }

    let final_width = (max_width + i32::from(item_x) + 9) / 8;
    final_width.min(MAX_LIST_MENU_TILE_WIDTH)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyMonCategoryText(dex_num: c_int, dest: *mut u8) {
    let category_name =
        unsafe { (&raw const gPokedexEntries).add(dex_num as usize * POKEDEX_ENTRY_SIZE) };
    let string = unsafe { StringCopy(dest, category_name) };
    unsafe { string.write(CHAR_SPACE) };
    let _ = unsafe { StringCopy(string.add(1), &raw const gText_Pokemon) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetStringClearToWidth(
    dest: *mut u8,
    font_id: c_int,
    string: *const u8,
    total_string_width: c_int,
) -> *mut u8 {
    let (mut buffer, width) = if string.is_null() {
        (dest, 0)
    } else {
        (unsafe { StringCopy(dest, string) }, unsafe {
            GetStringWidth(font_id as u8, string, 0)
        })
    };

    let clear_width = total_string_width - width;
    if clear_width > 0 {
        unsafe { buffer.write(EXT_CTRL_CODE_BEGIN) };
        buffer = unsafe { buffer.add(1) };
        unsafe { buffer.write(EXT_CTRL_CODE_CLEAR) };
        buffer = unsafe { buffer.add(1) };
        unsafe { buffer.write(clear_width as u8) };
        buffer = unsafe { buffer.add(1) };
        unsafe { buffer.write(EOS) };
    }

    buffer
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PadNameString(dest: *mut u8, pad_char: u8) {
    unsafe { StripExtCtrlCodes(dest) };
    let mut length = unsafe { StringLength(dest) };

    if pad_char == EXT_CTRL_CODE_BEGIN {
        // Padding with control codes takes two bytes at a time.
        while length < PLAYER_NAME_LENGTH - 1 {
            unsafe { dest.add(length as usize).write(EXT_CTRL_CODE_BEGIN) };
            unsafe {
                dest.add(length as usize + 1)
                    .write(EXT_CTRL_CODE_RESET_FONT)
            };
            length += 2;
        }
    } else {
        while length < PLAYER_NAME_LENGTH - 1 {
            unsafe { dest.add(length as usize).write(pad_char) };
            length += 1;
        }
    }

    unsafe { dest.add(length as usize).write(EOS) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertInternationalPlayerName(string: *mut u8) {
    if unsafe { StringLength(string) } < PLAYER_NAME_LENGTH - 1 {
        unsafe { ConvertInternationalString(string, LANGUAGE_JAPANESE as u8) };
    } else {
        unsafe { StripExtCtrlCodes(string) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertInternationalPlayerNameStripChar(string: *mut u8, remove_char: u8) {
    if unsafe { StringLength(string) } < PLAYER_NAME_LENGTH - 1 {
        unsafe { ConvertInternationalString(string, LANGUAGE_JAPANESE as u8) };
    } else if remove_char == EXT_CTRL_CODE_BEGIN {
        unsafe { StripExtCtrlCodes(string) };
    } else {
        let mut buffer = string;
        while unsafe { buffer.add(1).read() } != EOS {
            buffer = unsafe { buffer.add(1) };
        }

        while buffer >= string && unsafe { buffer.read() } == remove_char {
            unsafe { buffer.write(EOS) };
            buffer = unsafe { buffer.sub(1) };
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertInternationalContestantName(string: *mut u8) {
    let mut cursor = string;
    let first = unsafe { cursor.read() };
    cursor = unsafe { cursor.add(1) };
    let second = unsafe { cursor.read() };
    cursor = unsafe { cursor.add(1) };

    if first != EXT_CTRL_CODE_BEGIN || second != EXT_CTRL_CODE_JPN {
        return;
    }

    while unsafe { cursor.read() } != EOS {
        if unsafe { cursor.read() } == EXT_CTRL_CODE_BEGIN
            && unsafe { cursor.add(1).read() } == EXT_CTRL_CODE_ENG
        {
            return;
        }
        cursor = unsafe { cursor.add(1) };
    }

    unsafe { cursor.write(EXT_CTRL_CODE_BEGIN) };
    cursor = unsafe { cursor.add(1) };
    unsafe { cursor.write(EXT_CTRL_CODE_ENG) };
    cursor = unsafe { cursor.add(1) };
    unsafe { cursor.write(EOS) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn TVShowConvertInternationalString(
    dest: *mut u8,
    src: *const u8,
    language: c_int,
) {
    let _ = unsafe { StringCopy(dest, src) };
    unsafe { ConvertInternationalString(dest, language as u8) };
}

/// Latin languages cannot be told apart from a string alone, so this defaults
/// to English exactly as every release of the game does.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNicknameLanguage(string: *mut u8) -> c_int {
    if unsafe { string.read() } == EXT_CTRL_CODE_BEGIN
        && unsafe { string.add(1).read() } == EXT_CTRL_CODE_JPN
    {
        LANGUAGE_JAPANESE
    } else {
        LANGUAGE_ENGLISH
    }
}

/// Used by Pokenav's Match Call to erase the previous trainer's flavor text
/// when switching between their info pages.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FillWindowTilesByRow(
    window_id: c_int,
    column_start: c_int,
    row_start: c_int,
    num_fill_tiles: c_int,
    num_rows: c_int,
) {
    let window = unsafe {
        (&raw mut gWindows)
            .cast::<u8>()
            .add(window_id as usize * WINDOW_STRIDE)
    };
    let width = unsafe { window.add(WINDOW_WIDTH_OFFSET).read_volatile() };
    let tile_data = unsafe { window.add(WINDOW_TILE_DATA_OFFSET).cast::<*mut u8>().read() };

    let fill_size = num_fill_tiles as usize * TILE_SIZE_4BPP;
    let window_row_size = width as usize * TILE_SIZE_4BPP;
    let mut cursor = unsafe {
        tile_data.add(row_start as usize * window_row_size + column_start as usize * TILE_SIZE_4BPP)
    };

    let mut remaining = num_rows;
    while remaining > 0 {
        unsafe { cpu_fast_fill8(0x11, cursor, fill_size) };
        cursor = unsafe { cursor.add(window_row_size) };
        remaining -= 1;
    }
}

/// `sizeof(struct Window)` on ARM: an eight-byte `WindowTemplate` then a
/// four-byte pointer.
const WINDOW_STRIDE: usize = 12;

/// `offsetof(struct Window, window.width)`
const WINDOW_WIDTH_OFFSET: usize = 3;
/// `offsetof(struct Window, tileData)`
const WINDOW_TILE_DATA_OFFSET: usize = 8;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ffi::WindowTemplate;

    #[test]
    fn window_layout_matches_the_arm_structure() {
        assert_eq!(WINDOW_STRIDE, 12);
        assert_eq!(core::mem::offset_of!(WindowTemplate, width), 3);
        assert_eq!(WINDOW_TILE_DATA_OFFSET, 8);
    }

    #[test]
    fn list_menu_width_is_clamped_to_the_screen() {
        let width = |max_width: i32, item_x: i32| ((max_width + item_x + 9) / 8).min(28);
        assert_eq!(width(0, 0), 1);
        assert_eq!(width(64, 8), 10);
        assert_eq!(width(1000, 0), 28);
    }

    #[test]
    fn a_cleared_string_ends_with_a_terminated_clear_code() {
        let mut buffer = [0u8; 8];
        buffer[0] = EXT_CTRL_CODE_BEGIN;
        buffer[1] = EXT_CTRL_CODE_CLEAR;
        buffer[2] = 40;
        buffer[3] = EOS;
        assert_eq!(&buffer[..4], &[0xfc, 17, 40, 0xff]);
    }
}
