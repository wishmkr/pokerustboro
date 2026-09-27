//! The braille font. It has its own font function because braille messages
//! handle fewer control codes and scroll at the player's text speed.
//! For printing braille messages see `ScrCmd_braillemessage`.

use crate::data::braille::{sFont_Braille, sScrollDistances};
use crate::ffi::{A_BUTTON, B_BUTTON, COPYWIN_GFX, joy_held, joy_new};
use crate::load_save::gSaveBlock2Ptr;
use crate::text::{
    CopyGlyphToWindow, DecompressGlyphTile, FLAG_AUTO_SCROLL, FLAG_CAN_AB_SPEED_UP,
    GenerateFontHalfRowLookupTable, P_DELAY_COUNTER, P_SCROLL_DISTANCE, P_STATE, P_TEXT_SPEED,
    RENDER_FINISH, RENDER_PRINT, RENDER_REPEAT, RENDER_STATE_CLEAR, RENDER_STATE_HANDLE_CHAR,
    RENDER_STATE_PAUSE, RENDER_STATE_SCROLL, RENDER_STATE_SCROLL_START, RENDER_STATE_WAIT,
    RENDER_STATE_WAIT_SE, RENDER_UPDATE, SUB_SPED_UP, T_CURRENT_X, T_CURRENT_Y, T_FONT_ID,
    T_LETTER_SPACING, T_LINE_SPACING, T_WINDOW_ID, T_X, T_Y, TextPrinterClearDownArrow,
    TextPrinterInitDownArrowCounters, TextPrinterWait, TextPrinterWaitWithDownArrow, bg_color,
    bg_fill, fg_color, font_info, get, glyph_bottom, glyph_top, glyph_width, next_char, set,
    set_auto_scroll_delay, set_bg_color, set_fg_color, set_glyph_size, set_shadow_color,
    set_sub_flag, set_sub_font_id, shadow_color, skip_chars, sub_flag, text_flag,
};
use crate::window::{CopyWindowToVram, FillWindowPixelBuffer, ScrollWindow};

const CHAR_NEWLINE: u8 = 0xfe;
const PLACEHOLDER_BEGIN: u8 = 0xfd;
const EXT_CTRL_CODE_BEGIN: u8 = 0xfc;
const CHAR_PROMPT_CLEAR: u8 = 0xfb;
const CHAR_PROMPT_SCROLL: u8 = 0xfa;
const CHAR_EXTRA_SYMBOL: u8 = 0xf9;
const CHAR_KEYPAD_ICON: u8 = 0xf8;
const EOS: u8 = 0xff;

const EXT_CTRL_CODE_COLOR: u8 = 0x01;
const EXT_CTRL_CODE_HIGHLIGHT: u8 = 0x02;
const EXT_CTRL_CODE_SHADOW: u8 = 0x03;
const EXT_CTRL_CODE_COLOR_HIGHLIGHT_SHADOW: u8 = 0x04;
const EXT_CTRL_CODE_PALETTE: u8 = 0x05;
const EXT_CTRL_CODE_FONT: u8 = 0x06;
const EXT_CTRL_CODE_RESET_FONT: u8 = 0x07;
const EXT_CTRL_CODE_PAUSE: u8 = 0x08;
const EXT_CTRL_CODE_PAUSE_UNTIL_PRESS: u8 = 0x09;
const EXT_CTRL_CODE_WAIT_SE: u8 = 0x0a;
const EXT_CTRL_CODE_PLAY_BGM: u8 = 0x0b;
const EXT_CTRL_CODE_ESCAPE: u8 = 0x0c;
const EXT_CTRL_CODE_SHIFT_RIGHT: u8 = 0x0d;
const EXT_CTRL_CODE_SHIFT_DOWN: u8 = 0x0e;
const EXT_CTRL_CODE_FILL_WINDOW: u8 = 0x0f;
const EXT_CTRL_CODE_PLAY_SE: u8 = 0x10;

/// `gSaveBlock2Ptr->optionsTextSpeed`: bits 0-2 of the halfword at 0x14.
const SB2_OPTIONS: usize = 0x14;

unsafe extern "C" {
    fn IsSEPlaying() -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FontFunc_Braille(printer: *mut u8) -> u16 {
    match unsafe { get(printer, P_STATE) } {
        RENDER_STATE_HANDLE_CHAR => unsafe { handle_char(printer) },
        RENDER_STATE_WAIT => {
            if unsafe { TextPrinterWait(printer) } != 0 {
                unsafe { set(printer, P_STATE, RENDER_STATE_HANDLE_CHAR) };
            }
            RENDER_UPDATE
        }
        RENDER_STATE_CLEAR => {
            if unsafe { TextPrinterWaitWithDownArrow(printer) } != 0 {
                unsafe { FillWindowPixelBuffer(get(printer, T_WINDOW_ID), bg_fill(printer)) };
                unsafe { set(printer, T_CURRENT_X, get(printer, T_X)) };
                unsafe { set(printer, T_CURRENT_Y, get(printer, T_Y)) };
                unsafe { set(printer, P_STATE, RENDER_STATE_HANDLE_CHAR) };
            }
            RENDER_UPDATE
        }
        RENDER_STATE_SCROLL_START => {
            if unsafe { TextPrinterWaitWithDownArrow(printer) } != 0 {
                unsafe { TextPrinterClearDownArrow(printer) };
                let info = unsafe { font_info(get(printer, T_FONT_ID)) };
                let distance = unsafe { (*info).max_letter_height() }
                    .wrapping_add(unsafe { get(printer, T_LINE_SPACING) });
                unsafe { set(printer, P_SCROLL_DISTANCE, distance) };
                unsafe { set(printer, T_CURRENT_X, get(printer, T_X)) };
                unsafe { set(printer, P_STATE, RENDER_STATE_SCROLL) };
            }
            RENDER_UPDATE
        }
        RENDER_STATE_SCROLL => {
            let distance = unsafe { get(printer, P_SCROLL_DISTANCE) };
            if distance != 0 {
                let window_id = unsafe { get(printer, T_WINDOW_ID) };
                let fill = unsafe { bg_fill(printer) };
                let speed = unsafe { scroll_distance() };
                if distance < speed {
                    unsafe { ScrollWindow(window_id, 0, distance, fill) };
                    unsafe { set(printer, P_SCROLL_DISTANCE, 0) };
                } else {
                    unsafe { ScrollWindow(window_id, 0, speed, fill) };
                    unsafe { set(printer, P_SCROLL_DISTANCE, distance - speed) };
                }
                unsafe { CopyWindowToVram(window_id, COPYWIN_GFX) };
            } else {
                unsafe { set(printer, P_STATE, RENDER_STATE_HANDLE_CHAR) };
            }
            RENDER_UPDATE
        }
        RENDER_STATE_WAIT_SE => {
            if unsafe { IsSEPlaying() } == 0 {
                unsafe { set(printer, P_STATE, RENDER_STATE_HANDLE_CHAR) };
            }
            RENDER_UPDATE
        }
        RENDER_STATE_PAUSE => {
            let delay = unsafe { get(printer, P_DELAY_COUNTER) };
            if delay != 0 {
                unsafe { set(printer, P_DELAY_COUNTER, delay - 1) };
            } else {
                unsafe { set(printer, P_STATE, RENDER_STATE_HANDLE_CHAR) };
            }
            RENDER_UPDATE
        }
        _ => RENDER_FINISH,
    }
}

/// `sScrollDistances[gSaveBlock2Ptr->optionsTextSpeed]`
unsafe fn scroll_distance() -> u8 {
    let sb2 = unsafe { (&raw const gSaveBlock2Ptr).read() };
    let options = unsafe { sb2.add(SB2_OPTIONS).cast::<u16>().read() };
    let speed = usize::from(options & 7);
    unsafe { sScrollDistances.as_ptr().add(speed).read() }
}

unsafe fn handle_char(printer: *mut u8) -> u16 {
    if unsafe { joy_held(A_BUTTON | B_BUTTON) } && unsafe { sub_flag(printer, 0, SUB_SPED_UP) } {
        unsafe { set(printer, P_DELAY_COUNTER, 0) };
    }
    let delay = unsafe { get(printer, P_DELAY_COUNTER) };
    if delay != 0 && unsafe { get(printer, P_TEXT_SPEED) } != 0 {
        unsafe { set(printer, P_DELAY_COUNTER, delay - 1) };
        if text_flag(FLAG_CAN_AB_SPEED_UP) && unsafe { joy_new(A_BUTTON | B_BUTTON) } {
            unsafe { set_sub_flag(printer, 0, SUB_SPED_UP, true) };
            unsafe { set(printer, P_DELAY_COUNTER, 0) };
        }
        return RENDER_UPDATE;
    }
    if text_flag(FLAG_AUTO_SCROLL) {
        unsafe { set(printer, P_DELAY_COUNTER, 3) };
    } else {
        unsafe { set(printer, P_DELAY_COUNTER, get(printer, P_TEXT_SPEED)) };
    }

    let raw = unsafe { next_char(printer) };
    let mut char_ = u16::from(raw);
    match raw {
        EOS => return RENDER_FINISH,
        CHAR_NEWLINE => {
            unsafe { set(printer, T_CURRENT_X, get(printer, T_X)) };
            let info = unsafe { font_info(get(printer, T_FONT_ID)) };
            let step = unsafe { (*info).max_letter_height() }
                .wrapping_add(unsafe { get(printer, T_LINE_SPACING) });
            let y = unsafe { get(printer, T_CURRENT_Y) }.wrapping_add(step);
            unsafe { set(printer, T_CURRENT_Y, y) };
            return RENDER_REPEAT;
        }
        PLACEHOLDER_BEGIN => {
            unsafe { skip_chars(printer, 1) };
            return RENDER_REPEAT;
        }
        EXT_CTRL_CODE_BEGIN => {
            let code = unsafe { next_char(printer) };
            char_ = u16::from(code);
            let regenerate = |printer: *mut u8| unsafe {
                GenerateFontHalfRowLookupTable(
                    fg_color(printer),
                    bg_color(printer),
                    shadow_color(printer),
                )
            };
            match code {
                EXT_CTRL_CODE_COLOR => {
                    let v = unsafe { next_char(printer) };
                    unsafe { set_fg_color(printer, v) };
                    regenerate(printer);
                    return RENDER_REPEAT;
                }
                EXT_CTRL_CODE_HIGHLIGHT => {
                    let v = unsafe { next_char(printer) };
                    unsafe { set_bg_color(printer, v) };
                    regenerate(printer);
                    return RENDER_REPEAT;
                }
                EXT_CTRL_CODE_SHADOW => {
                    let v = unsafe { next_char(printer) };
                    unsafe { set_shadow_color(printer, v) };
                    regenerate(printer);
                    return RENDER_REPEAT;
                }
                EXT_CTRL_CODE_COLOR_HIGHLIGHT_SHADOW => {
                    let fg = unsafe { next_char(printer) };
                    let bg = unsafe { next_char(printer) };
                    let shadow = unsafe { next_char(printer) };
                    unsafe { set_fg_color(printer, fg) };
                    unsafe { set_bg_color(printer, bg) };
                    unsafe { set_shadow_color(printer, shadow) };
                    regenerate(printer);
                    return RENDER_REPEAT;
                }
                EXT_CTRL_CODE_PALETTE => {
                    unsafe { skip_chars(printer, 1) };
                    return RENDER_REPEAT;
                }
                EXT_CTRL_CODE_FONT => {
                    let font_id = unsafe { next_char(printer) };
                    unsafe { set_sub_font_id(printer, font_id) };
                    return RENDER_REPEAT;
                }
                EXT_CTRL_CODE_RESET_FONT => return RENDER_REPEAT,
                EXT_CTRL_CODE_PAUSE => {
                    let d = unsafe { next_char(printer) };
                    unsafe { set(printer, P_DELAY_COUNTER, d) };
                    unsafe { set(printer, P_STATE, RENDER_STATE_PAUSE) };
                    return RENDER_REPEAT;
                }
                EXT_CTRL_CODE_PAUSE_UNTIL_PRESS => {
                    unsafe { set(printer, P_STATE, RENDER_STATE_WAIT) };
                    if text_flag(FLAG_AUTO_SCROLL) {
                        unsafe { set_auto_scroll_delay(printer, 0) };
                    }
                    return RENDER_UPDATE;
                }
                EXT_CTRL_CODE_WAIT_SE => {
                    unsafe { set(printer, P_STATE, RENDER_STATE_WAIT_SE) };
                    return RENDER_UPDATE;
                }
                // Braille skips the song ids rather than playing them.
                EXT_CTRL_CODE_PLAY_BGM | EXT_CTRL_CODE_PLAY_SE => {
                    unsafe { skip_chars(printer, 2) };
                    return RENDER_REPEAT;
                }
                EXT_CTRL_CODE_ESCAPE => {
                    // Reads the byte *after* the escaped one, as the original does.
                    unsafe { skip_chars(printer, 1) };
                    let c = unsafe { crate::text::current_char(printer).read() };
                    char_ = u16::from(c);
                }
                EXT_CTRL_CODE_SHIFT_RIGHT => {
                    let x =
                        unsafe { get(printer, T_X) }.wrapping_add(unsafe { next_char(printer) });
                    unsafe { set(printer, T_CURRENT_X, x) };
                    return RENDER_REPEAT;
                }
                EXT_CTRL_CODE_SHIFT_DOWN => {
                    let y =
                        unsafe { get(printer, T_Y) }.wrapping_add(unsafe { next_char(printer) });
                    unsafe { set(printer, T_CURRENT_Y, y) };
                    return RENDER_REPEAT;
                }
                EXT_CTRL_CODE_FILL_WINDOW => {
                    unsafe { FillWindowPixelBuffer(get(printer, T_WINDOW_ID), bg_fill(printer)) };
                    return RENDER_REPEAT;
                }
                _ => {}
            }
        }
        CHAR_PROMPT_CLEAR => {
            unsafe { set(printer, P_STATE, RENDER_STATE_CLEAR) };
            unsafe { TextPrinterInitDownArrowCounters(printer) };
            return RENDER_UPDATE;
        }
        CHAR_PROMPT_SCROLL => {
            unsafe { set(printer, P_STATE, RENDER_STATE_SCROLL_START) };
            unsafe { TextPrinterInitDownArrowCounters(printer) };
            return RENDER_UPDATE;
        }
        CHAR_EXTRA_SYMBOL => {
            char_ = u16::from(unsafe { next_char(printer) }) | 0x100;
        }
        CHAR_KEYPAD_ICON => {
            unsafe { skip_chars(printer, 1) };
            return RENDER_PRINT;
        }
        _ => {}
    }

    unsafe { decompress_glyph_braille(char_) };
    unsafe { CopyGlyphToWindow(printer) };
    let x = unsafe { get(printer, T_CURRENT_X) }
        .wrapping_add(unsafe { glyph_width() })
        .wrapping_add(unsafe { get(printer, T_LETTER_SPACING) });
    unsafe { set(printer, T_CURRENT_X, x) };
    RENDER_PRINT
}

/// Braille cells are 16x16, eight per row of 0x100 halfwords.
unsafe fn decompress_glyph_braille(glyph: u16) {
    let glyph = usize::from(glyph);
    let glyphs = sFont_Braille
        .as_ptr()
        .cast::<u16>()
        .wrapping_add(0x100 * (glyph / 8) + 0x10 * (glyph % 8));
    unsafe { DecompressGlyphTile(glyphs.cast(), glyph_top().cast()) };
    unsafe { DecompressGlyphTile(glyphs.wrapping_add(0x8).cast(), glyph_top().add(8).cast()) };
    unsafe { DecompressGlyphTile(glyphs.wrapping_add(0x80).cast(), glyph_bottom().cast()) };
    unsafe {
        DecompressGlyphTile(
            glyphs.wrapping_add(0x88).cast(),
            glyph_bottom().add(8).cast(),
        )
    };
    unsafe { set_glyph_size(16, 16) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetGlyphWidth_Braille(_glyph_id: u16, _is_japanese: u32) -> u32 {
    16
}
