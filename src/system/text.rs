//! Text printers: glyph decoding, drawing text into windows a character per
//! frame, control codes, the down arrow and scrolling, and string widths.
//! The glyph and icon tables are in `data/text.rs`.

use core::ffi::c_void;

use crate::blit::FillBitmapRect4Bit;
use crate::braille::{FontFunc_Braille, GetGlyphWidth_Braille};
use crate::data::text::{
    sDarkDownArrowTiles, sDownArrowTiles, sDownArrowYCoords, sFontBoldJapaneseGlyphs,
    sFontHalfRowOffsets, sKeypadIconTiles, sKeypadIcons, sMenuCursorDimensions,
    sWindowVerticalScrollSpeeds,
};
use crate::dynamic_placeholder_text_util::DynamicPlaceholderTextUtil_GetPlaceholderPtr;
use crate::ffi::{
    A_BUTTON, Align4, B_BUTTON, COPYWIN_GFX, CpuSet, PlaySE, SE_SELECT, gStringVar1, gStringVar2,
    gStringVar3, joy_held, joy_new,
};
use crate::window::{
    BlitBitmapRectToWindow, CopyWindowToVram, FillWindowPixelBuffer, FillWindowPixelRect,
    ScrollWindow, gWindows,
};

// ------------------------------------------------------------- constants

const WINDOWS_MAX: usize = 32;

pub(crate) const RENDER_PRINT: u16 = 0;
pub(crate) const RENDER_FINISH: u16 = 1;
pub(crate) const RENDER_REPEAT: u16 = 2;
pub(crate) const RENDER_UPDATE: u16 = 3;

pub(crate) const RENDER_STATE_HANDLE_CHAR: u8 = 0;
pub(crate) const RENDER_STATE_WAIT: u8 = 1;
pub(crate) const RENDER_STATE_CLEAR: u8 = 2;
pub(crate) const RENDER_STATE_SCROLL_START: u8 = 3;
pub(crate) const RENDER_STATE_SCROLL: u8 = 4;
pub(crate) const RENDER_STATE_WAIT_SE: u8 = 5;
pub(crate) const RENDER_STATE_PAUSE: u8 = 6;

const FONT_SMALL: u8 = 0;
const FONT_NORMAL: u8 = 1;
const FONT_SHORT: u8 = 2;
const FONT_SHORT_COPY_1: u8 = 3;
const FONT_SHORT_COPY_2: u8 = 4;
const FONT_SHORT_COPY_3: u8 = 5;
const FONT_BRAILLE: u8 = 6;
const FONT_NARROW: u8 = 7;
const FONT_SMALL_NARROW: u8 = 8;
const FONT_BOLD: u8 = 9;

const TEXT_SKIP_DRAW: u8 = 0xff;
const TEXT_COLOR_TRANSPARENT: u16 = 0;
const TEXT_COLOR_WHITE: u8 = 1;
const TEXT_COLOR_LIGHT_GRAY: u8 = 3;

const FONTATTR_MAX_LETTER_WIDTH: u8 = 0;
const FONTATTR_MAX_LETTER_HEIGHT: u8 = 1;
const FONTATTR_LETTER_SPACING: u8 = 2;
const FONTATTR_LINE_SPACING: u8 = 3;
const FONTATTR_UNKNOWN: u8 = 4;
const FONTATTR_COLOR_FOREGROUND: u8 = 5;
const FONTATTR_COLOR_BACKGROUND: u8 = 6;
const FONTATTR_COLOR_SHADOW: u8 = 7;

const BATTLE_TYPE_RECORDED: u32 = 1 << 24;

const CHAR_DYNAMIC: u8 = 0xf7;
const CHAR_KEYPAD_ICON: u8 = 0xf8;
const CHAR_EXTRA_SYMBOL: u8 = 0xf9;
const CHAR_PROMPT_SCROLL: u8 = 0xfa;
const CHAR_PROMPT_CLEAR: u8 = 0xfb;
const EXT_CTRL_CODE_BEGIN: u8 = 0xfc;
const PLACEHOLDER_BEGIN: u8 = 0xfd;
const CHAR_NEWLINE: u8 = 0xfe;
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
const EXT_CTRL_CODE_CLEAR: u8 = 0x11;
const EXT_CTRL_CODE_SKIP_TO: u8 = 0x12;
const EXT_CTRL_CODE_CLEAR_TO: u8 = 0x13;
const EXT_CTRL_CODE_MIN_LETTER_SPACING: u8 = 0x14;
const EXT_CTRL_CODE_JPN: u8 = 0x15;
const EXT_CTRL_CODE_ENG: u8 = 0x16;
const EXT_CTRL_CODE_PAUSE_MUSIC: u8 = 0x17;
const EXT_CTRL_CODE_RESUME_MUSIC: u8 = 0x18;

const PLACEHOLDER_ID_STRING_VAR_1: u8 = 2;
const PLACEHOLDER_ID_STRING_VAR_2: u8 = 3;
const PLACEHOLDER_ID_STRING_VAR_3: u8 = 4;

// ----------------------------------------------------------------- layout

/// `struct TextPrinterTemplate`, 16 bytes.
const T_CURRENT_CHAR: usize = 0x00;
pub(crate) const T_WINDOW_ID: usize = 0x04;
pub(crate) const T_FONT_ID: usize = 0x05;
pub(crate) const T_X: usize = 0x06;
pub(crate) const T_Y: usize = 0x07;
pub(crate) const T_CURRENT_X: usize = 0x08;
pub(crate) const T_CURRENT_Y: usize = 0x09;
pub(crate) const T_LETTER_SPACING: usize = 0x0a;
pub(crate) const T_LINE_SPACING: usize = 0x0b;
/// `unk:4` low, `fgColor:4` high.
const T_COLORS0: usize = 0x0c;
/// `bgColor:4` low, `shadowColor:4` high.
const T_COLORS1: usize = 0x0d;
const TEMPLATE_SIZE: usize = 0x10;

/// `struct TextPrinter`, 36 bytes.
const P_CALLBACK: usize = 0x10;
const P_SUB: usize = 0x14;
const P_SUB_FIELDS: usize = 7;
const P_ACTIVE: usize = 0x1b;
pub(crate) const P_STATE: usize = 0x1c;
pub(crate) const P_TEXT_SPEED: usize = 0x1d;
pub(crate) const P_DELAY_COUNTER: usize = 0x1e;
pub(crate) const P_SCROLL_DISTANCE: usize = 0x1f;
const P_MIN_LETTER_SPACING: usize = 0x20;
const P_JAPANESE: usize = 0x21;
const PRINTER_SIZE: usize = 0x24;

/// `struct TextPrinterSubStruct` bits, relative to `P_SUB`.
/// byte 0: `fontId:4`, `hasPrintBeenSpedUp:1`
/// byte 1: `downArrowDelay:5`, `downArrowYPosIdx:2`, `hasFontIdBeenSet:1`
/// byte 2: `autoScrollDelay`
pub(crate) const SUB_SPED_UP: u8 = 0x10;
const SUB_ARROW_DELAY_MASK: u8 = 0x1f;
const SUB_ARROW_Y_SHIFT: u8 = 5;
const SUB_ARROW_Y_MASK: u8 = 0x60;
const SUB_FONT_ID_SET: u8 = 0x80;

/// `struct TextGlyph`
const GLYPH_BOTTOM: usize = 0x40;
pub(crate) const GLYPH_WIDTH: usize = 0x80;
const GLYPH_HEIGHT: usize = 0x81;
const GLYPH_SIZE: usize = 0x84;

/// `TextFlags` bits.
pub(crate) const FLAG_CAN_AB_SPEED_UP: u8 = 0x01;
const FLAG_ALTERNATE_DOWN_ARROW: u8 = 0x02;
pub(crate) const FLAG_AUTO_SCROLL: u8 = 0x04;

/// `struct Window`: an 8-byte template, then `tileData`.
const WINDOW_SIZE: usize = 12;
const WINDOW_WIDTH: usize = 3;
const WINDOW_HEIGHT: usize = 4;
const WINDOW_TILE_DATA: usize = 8;

type TextPrinterCallback = unsafe extern "C" fn(*mut u8, u16);
type FontFunction = unsafe extern "C" fn(*mut u8) -> u16;
type GlyphWidthFunction = unsafe extern "C" fn(u16, u32) -> u32;

/// `struct FontInfo`, 12 bytes on the GBA.
#[repr(C, align(4))]
pub struct FontInfo {
    font_function: Option<FontFunction>,
    max_letter_width: u8,
    max_letter_height: u8,
    letter_spacing: u8,
    line_spacing: u8,
    /// `unk:4` low, `fgColor:4` high.
    colors0: u8,
    /// `bgColor:4` low, `shadowColor:4` high.
    colors1: u8,
}

impl FontInfo {
    pub(crate) fn max_letter_height(&self) -> u8 {
        self.max_letter_height
    }
}

const fn font(
    function: Option<FontFunction>,
    max_letter_width: u8,
    max_letter_height: u8,
    line_spacing: u8,
    fg: u8,
    bg: u8,
    shadow: u8,
) -> FontInfo {
    FontInfo {
        font_function: function,
        max_letter_width,
        max_letter_height,
        letter_spacing: 0,
        line_spacing,
        colors0: fg << 4,
        colors1: bg | shadow << 4,
    }
}

static FONT_INFOS: [FontInfo; 10] = [
    font(Some(font_func_small), 5, 12, 0, 2, 1, 3),
    font(Some(font_func_normal), 6, 16, 0, 2, 1, 3),
    font(Some(font_func_short), 6, 14, 0, 2, 1, 3),
    font(Some(font_func_short_copy1), 6, 14, 0, 2, 1, 3),
    font(Some(font_func_short_copy2), 6, 14, 0, 2, 1, 3),
    font(Some(font_func_short_copy3), 6, 14, 0, 2, 1, 3),
    font(Some(FontFunc_Braille), 8, 16, 8, 2, 1, 3),
    font(Some(font_func_narrow), 5, 16, 0, 2, 1, 3),
    font(Some(font_func_small_narrow), 5, 8, 0, 2, 1, 3),
    font(None, 8, 8, 0, 1, 2, 15),
];

static GLYPH_WIDTH_FUNCS: [(u8, GlyphWidthFunction); 9] = [
    (FONT_SMALL, glyph_width_small),
    (FONT_NORMAL, glyph_width_normal),
    (FONT_SHORT, glyph_width_short),
    (FONT_SHORT_COPY_1, glyph_width_short),
    (FONT_SHORT_COPY_2, glyph_width_short),
    (FONT_SHORT_COPY_3, glyph_width_short),
    (FONT_BRAILLE, GetGlyphWidth_Braille),
    (FONT_NARROW, glyph_width_narrow),
    (FONT_SMALL_NARROW, glyph_width_small_narrow),
];

// ------------------------------------------------------------------ state

#[unsafe(link_section = "ewram_data")]
static mut TEMP_TEXT_PRINTER: Align4<[u8; PRINTER_SIZE]> = Align4([0; PRINTER_SIZE]);

#[unsafe(link_section = "ewram_data")]
static mut TEXT_PRINTERS: Align4<[[u8; PRINTER_SIZE]; WINDOWS_MAX]> =
    Align4([[0; PRINTER_SIZE]; WINDOWS_MAX]);

static mut FONT_HALF_ROW_LOOKUP_TABLE: Align4<[u16; 0x51]> = Align4([0; 0x51]);
static mut LAST_TEXT_BG_COLOR: u16 = 0;
static mut LAST_TEXT_FG_COLOR: u16 = 0;
static mut LAST_TEXT_SHADOW_COLOR: u16 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFonts: *const FontInfo = core::ptr::null();

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gDisableTextPrinters: u8 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gCurGlyph: Align4<[u8; GLYPH_SIZE]> = Align4([0; GLYPH_SIZE]);

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gTextFlags: u8 = 0;

unsafe extern "C" {
    static gBattleTypeFlags: u32;
    static mut gMPlayInfo_BGM: u8;

    static gFontNormalLatinGlyphs: u16;
    static gFontNormalLatinGlyphWidths: u8;
    static gFontNormalJapaneseGlyphs: u16;
    static gFontSmallLatinGlyphs: u16;
    static gFontSmallLatinGlyphWidths: u8;
    static gFontSmallJapaneseGlyphs: u16;
    static gFontShortLatinGlyphs: u16;
    static gFontShortLatinGlyphWidths: u8;
    static gFontShortJapaneseGlyphs: u16;
    static gFontShortJapaneseGlyphWidths: u8;
    static gFontNarrowLatinGlyphs: u16;
    static gFontNarrowLatinGlyphWidths: u8;
    static gFontSmallNarrowLatinGlyphs: u16;
    static gFontSmallNarrowLatinGlyphWidths: u8;

    fn GetPlayerTextSpeed() -> u32;
    fn IsSEPlaying() -> u8;
    fn PlayBGM(song: u16);
    fn m4aMPlayStop(info: *mut u8);
    fn m4aMPlayContinue(info: *mut u8);
}

// ---------------------------------------------------------------- helpers

#[inline]
pub(crate) fn text_flag(flag: u8) -> bool {
    let flags = unsafe { (&raw const gTextFlags).read_volatile() };
    flags & flag != 0
}

#[inline]
unsafe fn printer_slot(index: usize) -> *mut u8 {
    unsafe {
        (&raw mut TEXT_PRINTERS)
            .cast::<u8>()
            .add(index * PRINTER_SIZE)
    }
}

#[inline]
unsafe fn temp_printer() -> *mut u8 {
    (&raw mut TEMP_TEXT_PRINTER).cast()
}

#[inline]
pub(crate) unsafe fn get(p: *mut u8, offset: usize) -> u8 {
    unsafe { p.add(offset).read() }
}

#[inline]
pub(crate) unsafe fn set(p: *mut u8, offset: usize, value: u8) {
    unsafe { p.add(offset).write(value) };
}

#[inline]
pub(crate) unsafe fn current_char(p: *mut u8) -> *const u8 {
    unsafe { p.add(T_CURRENT_CHAR).cast::<*const u8>().read() }
}

#[inline]
pub(crate) unsafe fn set_current_char(p: *mut u8, value: *const u8) {
    unsafe { p.add(T_CURRENT_CHAR).cast::<*const u8>().write(value) };
}

/// `*currentChar++`
#[inline]
pub(crate) unsafe fn next_char(p: *mut u8) -> u8 {
    let c = unsafe { current_char(p) };
    unsafe { set_current_char(p, c.wrapping_add(1)) };
    unsafe { c.read() }
}

#[inline]
pub(crate) unsafe fn skip_chars(p: *mut u8, n: usize) {
    let c = unsafe { current_char(p) };
    unsafe { set_current_char(p, c.wrapping_add(n)) };
}

#[inline]
pub(crate) unsafe fn fg_color(p: *mut u8) -> u8 {
    unsafe { get(p, T_COLORS0) >> 4 }
}

#[inline]
pub(crate) unsafe fn bg_color(p: *mut u8) -> u8 {
    unsafe { get(p, T_COLORS1) & 0xf }
}

#[inline]
pub(crate) unsafe fn shadow_color(p: *mut u8) -> u8 {
    unsafe { get(p, T_COLORS1) >> 4 }
}

#[inline]
pub(crate) unsafe fn set_fg_color(p: *mut u8, value: u8) {
    let byte = unsafe { get(p, T_COLORS0) };
    unsafe { set(p, T_COLORS0, (byte & 0x0f) | (value << 4)) };
}

#[inline]
pub(crate) unsafe fn set_bg_color(p: *mut u8, value: u8) {
    let byte = unsafe { get(p, T_COLORS1) };
    unsafe { set(p, T_COLORS1, (byte & 0xf0) | (value & 0x0f)) };
}

#[inline]
pub(crate) unsafe fn set_shadow_color(p: *mut u8, value: u8) {
    let byte = unsafe { get(p, T_COLORS1) };
    unsafe { set(p, T_COLORS1, (byte & 0x0f) | (value << 4)) };
}

pub(crate) unsafe fn regenerate_lookup(p: *mut u8) {
    unsafe { GenerateFontHalfRowLookupTable(fg_color(p), bg_color(p), shadow_color(p)) };
}

/// `PIXEL_FILL(bgColor)`
#[inline]
pub(crate) unsafe fn bg_fill(p: *mut u8) -> u8 {
    let bg = unsafe { bg_color(p) };
    (bg << 4) | bg
}

#[inline]
pub(crate) unsafe fn sub_font_id(p: *mut u8) -> u8 {
    unsafe { get(p, P_SUB) & 0x0f }
}

#[inline]
pub(crate) unsafe fn set_sub_font_id(p: *mut u8, font_id: u8) {
    let byte = unsafe { get(p, P_SUB) };
    unsafe { set(p, P_SUB, (byte & 0xf0) | (font_id & 0x0f)) };
}

#[inline]
pub(crate) unsafe fn sub_flag(p: *mut u8, byte: usize, flag: u8) -> bool {
    let value = unsafe { get(p, P_SUB + byte) };
    value & flag != 0
}

#[inline]
pub(crate) unsafe fn set_sub_flag(p: *mut u8, byte: usize, flag: u8, value: bool) {
    let current = unsafe { get(p, P_SUB + byte) };
    let updated = if value {
        current | flag
    } else {
        current & !flag
    };
    unsafe { set(p, P_SUB + byte, updated) };
}

#[inline]
pub(crate) unsafe fn set_auto_scroll_delay(p: *mut u8, value: u8) {
    unsafe { set(p, P_SUB + 2, value) };
}

#[inline]
pub(crate) unsafe fn font_info(font_id: u8) -> *const FontInfo {
    unsafe { (&raw const gFonts).read().add(usize::from(font_id)) }
}

#[inline]
pub(crate) unsafe fn glyph() -> *mut u8 {
    (&raw mut gCurGlyph).cast()
}

#[inline]
pub(crate) unsafe fn glyph_top() -> *mut u32 {
    unsafe { glyph().cast() }
}

#[inline]
pub(crate) unsafe fn glyph_bottom() -> *mut u32 {
    unsafe { glyph().add(GLYPH_BOTTOM).cast() }
}

#[inline]
pub(crate) unsafe fn set_glyph_size(width: u8, height: u8) {
    unsafe { glyph().add(GLYPH_WIDTH).write(width) };
    unsafe { glyph().add(GLYPH_HEIGHT).write(height) };
}

#[inline]
pub(crate) unsafe fn glyph_width() -> u8 {
    unsafe { glyph().add(GLYPH_WIDTH).read() }
}

#[inline]
unsafe fn window_base(window_id: u8) -> *mut u8 {
    unsafe {
        (&raw mut gWindows)
            .cast::<u8>()
            .add(usize::from(window_id) * WINDOW_SIZE)
    }
}

// ------------------------------------------------------------- public API

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DeactivateAllTextPrinters() {
    for i in 0..WINDOWS_MAX {
        unsafe { set(printer_slot(i), P_ACTIVE, 0) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddTextPrinterParameterized(
    window_id: u8,
    font_id: u8,
    string: *const u8,
    x: u8,
    y: u8,
    speed: u8,
    callback: Option<TextPrinterCallback>,
) -> u16 {
    let mut template = Align4([0u8; TEMPLATE_SIZE]);
    let t = template.0.as_mut_ptr();
    let info = unsafe { font_info(font_id) };
    unsafe {
        set_current_char(t, string);
        set(t, T_WINDOW_ID, window_id);
        set(t, T_FONT_ID, font_id);
        set(t, T_X, x);
        set(t, T_Y, y);
        set(t, T_CURRENT_X, x);
        set(t, T_CURRENT_Y, y);
        set(t, T_LETTER_SPACING, (*info).letter_spacing);
        set(t, T_LINE_SPACING, (*info).line_spacing);
        // unk and fgColor share a byte, as do bgColor and shadowColor.
        set(t, T_COLORS0, (*info).colors0);
        set(t, T_COLORS1, (*info).colors1);
    }
    unsafe { AddTextPrinter(t, speed, callback) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddTextPrinter(
    template: *mut u8,
    speed: u8,
    callback: Option<TextPrinterCallback>,
) -> u16 {
    if unsafe { (&raw const gFonts).read() }.is_null() {
        return 0;
    }

    let temp = unsafe { temp_printer() };
    unsafe {
        set(temp, P_ACTIVE, 1);
        set(temp, P_STATE, RENDER_STATE_HANDLE_CHAR);
        set(temp, P_TEXT_SPEED, speed);
        set(temp, P_DELAY_COUNTER, 0);
        set(temp, P_SCROLL_DISTANCE, 0);
        temp.add(P_SUB).write_bytes(0, P_SUB_FIELDS);
        core::ptr::copy_nonoverlapping(template, temp, TEMPLATE_SIZE);
        temp.add(P_CALLBACK)
            .cast::<Option<TextPrinterCallback>>()
            .write(callback);
        set(temp, P_MIN_LETTER_SPACING, 0);
        set(temp, P_JAPANESE, 0);
    }

    unsafe { regenerate_lookup(template) };
    let window_id = unsafe { get(template, T_WINDOW_ID) };
    let slot = unsafe { printer_slot(usize::from(window_id)) };
    if speed != TEXT_SKIP_DRAW && speed != 0 {
        unsafe { set(temp, P_TEXT_SPEED, speed - 1) };
        unsafe { core::ptr::copy_nonoverlapping(temp, slot, PRINTER_SIZE) };
    } else {
        unsafe { set(temp, P_TEXT_SPEED, 0) };
        // Render all the text (up to a limit) at once.
        for _ in 0..0x400 {
            if unsafe { render_font(temp) } == RENDER_FINISH {
                break;
            }
        }
        // It is in the window now, but only drawn if asked.
        if speed != TEXT_SKIP_DRAW {
            unsafe { CopyWindowToVram(get(temp, T_WINDOW_ID), COPYWIN_GFX) };
        }
        unsafe { set(slot, P_ACTIVE, 0) };
    }
    unsafe { (&raw mut gDisableTextPrinters).write(0) };
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunTextPrinters() {
    if unsafe { (&raw const gDisableTextPrinters).read() } != 0 {
        return;
    }
    for i in 0..WINDOWS_MAX {
        let printer = unsafe { printer_slot(i) };
        if unsafe { get(printer, P_ACTIVE) } == 0 {
            continue;
        }
        let command = unsafe { render_font(printer) };
        match command {
            RENDER_PRINT | RENDER_UPDATE => {
                if command == RENDER_PRINT {
                    unsafe { CopyWindowToVram(get(printer, T_WINDOW_ID), COPYWIN_GFX) };
                }
                let callback = unsafe {
                    printer
                        .add(P_CALLBACK)
                        .cast::<Option<TextPrinterCallback>>()
                        .read()
                };
                if let Some(callback) = callback {
                    unsafe { callback(printer, command) };
                }
            }
            RENDER_FINISH => unsafe { set(printer, P_ACTIVE, 0) },
            _ => {}
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsTextPrinterActive(id: u8) -> u16 {
    u16::from(unsafe { get(printer_slot(usize::from(id)), P_ACTIVE) })
}

unsafe fn render_font(printer: *mut u8) -> u16 {
    loop {
        let info = unsafe { font_info(get(printer, T_FONT_ID)) };
        let result = match unsafe { (*info).font_function } {
            Some(function) => unsafe { function(printer) },
            // The original calls through the null pointer of FONT_BOLD.
            None => RENDER_FINISH,
        };
        if result != RENDER_REPEAT {
            return result;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GenerateFontHalfRowLookupTable(
    fg_color: u8,
    bg_color: u8,
    shadow_color: u8,
) {
    unsafe { (&raw mut LAST_TEXT_BG_COLOR).write(u16::from(bg_color)) };
    unsafe { (&raw mut LAST_TEXT_FG_COLOR).write(u16::from(fg_color)) };
    unsafe { (&raw mut LAST_TEXT_SHADOW_COLOR).write(u16::from(shadow_color)) };

    let (fg, bg, shadow) = (
        u32::from(fg_color),
        u32::from(bg_color),
        u32::from(shadow_color),
    );
    let colors = [bg, fg, shadow];
    let table = (&raw mut FONT_HALF_ROW_LOOKUP_TABLE).cast::<u16>();
    // Every combination of four pixels, each bg/fg/shadow. The lowest
    // nibble varies slowest and the highest fastest, as in the unrolled C.
    let mut index = 0;
    for &p0 in &colors {
        for &p1 in &colors {
            for &p2 in &colors {
                for &p3 in &colors {
                    let value = (p3 << 12) | (p2 << 8) | (p1 << 4) | p0;
                    unsafe { table.add(index).write(value as u16) };
                    index += 1;
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveTextColors(
    fg_color: *mut u8,
    bg_color: *mut u8,
    shadow_color: *mut u8,
) {
    unsafe { bg_color.write((&raw const LAST_TEXT_BG_COLOR).read() as u8) };
    unsafe { fg_color.write((&raw const LAST_TEXT_FG_COLOR).read() as u8) };
    unsafe { shadow_color.write((&raw const LAST_TEXT_SHADOW_COLOR).read() as u8) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RestoreTextColors(
    fg_color: *mut u8,
    bg_color: *mut u8,
    shadow_color: *mut u8,
) {
    unsafe {
        GenerateFontHalfRowLookupTable(fg_color.read(), bg_color.read(), shadow_color.read())
    };
}

#[inline]
unsafe fn half_row(byte: u32) -> u32 {
    let offset = unsafe {
        sFontHalfRowOffsets
            .as_ptr()
            .add((byte & 0xff) as usize)
            .read()
    };
    let table = (&raw const FONT_HALF_ROW_LOOKUP_TABLE).cast::<u16>();
    u32::from(unsafe { table.add(usize::from(offset)).read() })
}

/// Expands one 8x8 1bpp-ish font tile (two bits a pixel) into 4bpp pixels
/// using the current colour lookup table.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DecompressGlyphTile(src: *const c_void, dest: *mut c_void) {
    let src = src.cast::<u16>();
    let dest = dest.cast::<u32>();
    for row in 0..8 {
        let temp = u32::from(unsafe { src.add(row).read() });
        let value = (unsafe { half_row(temp & 0xff) } << 16) | unsafe { half_row(temp >> 8) };
        unsafe { dest.add(row).write(value) };
    }
}

/// `GLYPH_COPY`: ORs `width`x`height` pixels of glyph data into 4bpp tiles.
#[allow(clippy::too_many_arguments)]
unsafe fn glyph_copy(
    window_tiles: *mut u8,
    width_offset: u32,
    x: u32,
    y: u32,
    mut glyph_pixels: *const u32,
    width: i32,
    height: i32,
) {
    let x_end = x.wrapping_add(width as u32);
    let y_end = y.wrapping_add(height as u32);
    let mut i = y;
    while i < y_end {
        let mut pixel_data = unsafe { glyph_pixels.read() };
        glyph_pixels = glyph_pixels.wrapping_add(1);
        let mut j = x;
        while j < x_end {
            let to_orr = pixel_data & 0xf;
            if to_orr != 0 {
                let offset = (j / 8) * 32 + (j % 8) / 2 + (i / 8) * width_offset + (i % 8) * 4;
                let dst = window_tiles.wrapping_add(offset as usize);
                let bits = (j & 1) * 4;
                let old = u32::from(unsafe { dst.read() });
                unsafe { dst.write(((to_orr << bits) | (old & (0xf0 >> bits))) as u8) };
            }
            pixel_data >>= 4;
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyGlyphToWindow(printer: *mut u8) {
    let window = unsafe { window_base(get(printer, T_WINDOW_ID)) };
    let win_width = i32::from(unsafe { get(window, WINDOW_WIDTH) });
    let win_height = i32::from(unsafe { get(window, WINDOW_HEIGHT) });
    let current_x = unsafe { get(printer, T_CURRENT_X) };
    let current_y = unsafe { get(printer, T_CURRENT_Y) };

    let mut gw = win_width * 8 - i32::from(current_x);
    let cur_width = i32::from(unsafe { glyph_width() });
    if gw > cur_width {
        gw = cur_width;
    }
    let mut glyph_height = win_height * 8 - i32::from(current_y);
    let cur_height = i32::from(unsafe { glyph().add(GLYPH_HEIGHT).read() });
    if glyph_height > cur_height {
        glyph_height = cur_height;
    }

    let x = u32::from(current_x);
    let y = u32::from(current_y);
    let pixels = unsafe { glyph_top() }.cast_const();
    let tiles = unsafe { window.add(WINDOW_TILE_DATA).cast::<*mut u8>().read() };
    let width_offset = (win_width * 32) as u32;

    let copy = |dx: u32, dy: u32, block: usize, w: i32, h: i32| unsafe {
        glyph_copy(
            tiles,
            width_offset,
            x + dx,
            y + dy,
            pixels.wrapping_add(block),
            w,
            h,
        )
    };
    if gw < 9 {
        if glyph_height < 9 {
            copy(0, 0, 0, gw, glyph_height);
        } else {
            copy(0, 0, 0, gw, 8);
            copy(0, 8, 16, gw, glyph_height - 8);
        }
    } else if glyph_height < 9 {
        copy(0, 0, 0, 8, glyph_height);
        copy(8, 0, 8, gw - 8, glyph_height);
    } else {
        copy(0, 0, 0, 8, 8);
        copy(8, 0, 8, gw - 8, 8);
        copy(0, 8, 16, 8, glyph_height - 8);
        copy(8, 8, 24, gw - 8, glyph_height - 8);
    }
}

/// `struct Bitmap`: pixels, then width and height as two 16-bit fields.
#[repr(C, align(4))]
struct Bitmap {
    pixels: *mut u8,
    dimensions: u32,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearTextSpan(printer: *mut u8, width: u32) {
    let bg = unsafe { (&raw const LAST_TEXT_BG_COLOR).read() };
    if bg == TEXT_COLOR_TRANSPARENT {
        return;
    }
    let window = unsafe { window_base(get(printer, T_WINDOW_ID)) };
    let w = u32::from(unsafe { get(window, WINDOW_WIDTH) }) << 3;
    let h = u32::from(unsafe { get(window, WINDOW_HEIGHT) }) << 3;
    let mut bitmap = Bitmap {
        pixels: unsafe { window.add(WINDOW_TILE_DATA).cast::<*mut u8>().read() },
        dimensions: (w & 0xffff) | ((h & 0xffff) << 16),
    };
    unsafe {
        FillBitmapRect4Bit(
            (&raw mut bitmap).cast(),
            u16::from(get(printer, T_CURRENT_X)),
            u16::from(get(printer, T_CURRENT_Y)),
            width as u16,
            u16::from(glyph().add(GLYPH_HEIGHT).read()),
            bg as u8,
        )
    };
}

// ------------------------------------------------------------ font funcs

unsafe fn font_func(printer: *mut u8, font_id: u8) -> u16 {
    if !unsafe { sub_flag(printer, 1, SUB_FONT_ID_SET) } {
        unsafe { set_sub_font_id(printer, font_id) };
        unsafe { set_sub_flag(printer, 1, SUB_FONT_ID_SET, true) };
    }
    unsafe { render_text(printer) }
}

unsafe extern "C" fn font_func_small(printer: *mut u8) -> u16 {
    unsafe { font_func(printer, FONT_SMALL) }
}

unsafe extern "C" fn font_func_normal(printer: *mut u8) -> u16 {
    unsafe { font_func(printer, FONT_NORMAL) }
}

unsafe extern "C" fn font_func_short(printer: *mut u8) -> u16 {
    unsafe { font_func(printer, FONT_SHORT) }
}

unsafe extern "C" fn font_func_short_copy1(printer: *mut u8) -> u16 {
    unsafe { font_func(printer, FONT_SHORT_COPY_1) }
}

unsafe extern "C" fn font_func_short_copy2(printer: *mut u8) -> u16 {
    unsafe { font_func(printer, FONT_SHORT_COPY_2) }
}

unsafe extern "C" fn font_func_short_copy3(printer: *mut u8) -> u16 {
    unsafe { font_func(printer, FONT_SHORT_COPY_3) }
}

unsafe extern "C" fn font_func_narrow(printer: *mut u8) -> u16 {
    unsafe { font_func(printer, FONT_NARROW) }
}

unsafe extern "C" fn font_func_small_narrow(printer: *mut u8) -> u16 {
    unsafe { font_func(printer, FONT_SMALL_NARROW) }
}

// ------------------------------------------------------------- down arrow

#[unsafe(no_mangle)]
pub unsafe extern "C" fn TextPrinterInitDownArrowCounters(printer: *mut u8) {
    if text_flag(FLAG_AUTO_SCROLL) {
        unsafe { set_auto_scroll_delay(printer, 0) };
    } else {
        let byte = unsafe { get(printer, P_SUB + 1) };
        unsafe {
            set(
                printer,
                P_SUB + 1,
                byte & !(SUB_ARROW_Y_MASK | SUB_ARROW_DELAY_MASK),
            )
        };
    }
}

fn arrow_tiles() -> *const u8 {
    if text_flag(FLAG_ALTERNATE_DOWN_ARROW) {
        sDarkDownArrowTiles.as_ptr()
    } else {
        sDownArrowTiles.as_ptr()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn TextPrinterDrawDownArrow(printer: *mut u8) {
    if text_flag(FLAG_AUTO_SCROLL) {
        return;
    }
    let byte = unsafe { get(printer, P_SUB + 1) };
    let delay = byte & SUB_ARROW_DELAY_MASK;
    if delay != 0 {
        unsafe {
            set(
                printer,
                P_SUB + 1,
                (byte & !SUB_ARROW_DELAY_MASK) | (delay - 1),
            )
        };
        return;
    }

    let window_id = unsafe { get(printer, T_WINDOW_ID) };
    let x = u16::from(unsafe { get(printer, T_CURRENT_X) });
    let y = u16::from(unsafe { get(printer, T_CURRENT_Y) });
    unsafe { FillWindowPixelRect(window_id, bg_fill(printer), x, y, 8, 16) };
    let y_index = (byte & SUB_ARROW_Y_MASK) >> SUB_ARROW_Y_SHIFT;
    let src_y = unsafe { sDownArrowYCoords.as_ptr().add(usize::from(y_index)).read() };
    unsafe {
        BlitBitmapRectToWindow(
            window_id,
            arrow_tiles(),
            0,
            u16::from(src_y),
            8,
            16,
            x,
            y,
            8,
            16,
        )
    };
    unsafe { CopyWindowToVram(window_id, COPYWIN_GFX) };

    // downArrowDelay = 8, downArrowYPosIdx++ (wrapping within its two bits).
    let next_y = (y_index + 1) & 3;
    let keep = byte & SUB_FONT_ID_SET;
    unsafe { set(printer, P_SUB + 1, keep | (next_y << SUB_ARROW_Y_SHIFT) | 8) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn TextPrinterClearDownArrow(printer: *mut u8) {
    let window_id = unsafe { get(printer, T_WINDOW_ID) };
    unsafe {
        FillWindowPixelRect(
            window_id,
            bg_fill(printer),
            u16::from(get(printer, T_CURRENT_X)),
            u16::from(get(printer, T_CURRENT_Y)),
            8,
            16,
        )
    };
    unsafe { CopyWindowToVram(window_id, COPYWIN_GFX) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn TextPrinterWaitAutoMode(printer: *mut u8) -> u8 {
    let delay = unsafe { get(printer, P_SUB + 2) };
    if delay == 49 {
        1
    } else {
        unsafe { set_auto_scroll_delay(printer, delay.wrapping_add(1)) };
        0
    }
}

unsafe fn a_or_b_pressed() -> bool {
    if unsafe { joy_new(A_BUTTON | B_BUTTON) } {
        unsafe { PlaySE(SE_SELECT) };
        true
    } else {
        false
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn TextPrinterWaitWithDownArrow(printer: *mut u8) -> u16 {
    if text_flag(FLAG_AUTO_SCROLL) {
        u16::from(unsafe { TextPrinterWaitAutoMode(printer) })
    } else {
        unsafe { TextPrinterDrawDownArrow(printer) };
        u16::from(unsafe { a_or_b_pressed() })
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn TextPrinterWait(printer: *mut u8) -> u16 {
    if text_flag(FLAG_AUTO_SCROLL) {
        u16::from(unsafe { TextPrinterWaitAutoMode(printer) })
    } else {
        u16::from(unsafe { a_or_b_pressed() })
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawDownArrow(
    window_id: u8,
    x: u16,
    y: u16,
    bg_color: u8,
    draw_arrow: u8,
    counter: *mut u8,
    y_coord_index: *mut u8,
) {
    let count = unsafe { counter.read() };
    if count != 0 {
        unsafe { counter.write(count - 1) };
        return;
    }
    unsafe { FillWindowPixelRect(window_id, (bg_color << 4) | bg_color, x, y, 8, 16) };
    if draw_arrow == 0 {
        let index = unsafe { y_coord_index.read() } & 3;
        let src_y = unsafe { sDownArrowYCoords.as_ptr().add(usize::from(index)).read() };
        unsafe {
            BlitBitmapRectToWindow(
                window_id,
                arrow_tiles(),
                0,
                u16::from(src_y),
                8,
                16,
                x,
                y.wrapping_sub(2),
                8,
                16,
            )
        };
        unsafe { CopyWindowToVram(window_id, COPYWIN_GFX) };
        unsafe { counter.write(8) };
        unsafe { y_coord_index.write(y_coord_index.read().wrapping_add(1)) };
    }
}

// ------------------------------------------------------------- rendering

/// Handles a control code after `EXT_CTRL_CODE_BEGIN`. `Some(ret)` returns
/// from `RenderText`; `None` means "draw `char` as a glyph".
unsafe fn render_ext_ctrl_code(printer: *mut u8, char_: &mut u16) -> Option<u16> {
    let code = unsafe { next_char(printer) };
    *char_ = u16::from(code);
    let ret = match code {
        EXT_CTRL_CODE_COLOR => {
            let value = unsafe { next_char(printer) };
            unsafe { set_fg_color(printer, value) };
            unsafe { regenerate_lookup(printer) };
            RENDER_REPEAT
        }
        EXT_CTRL_CODE_HIGHLIGHT => {
            let value = unsafe { next_char(printer) };
            unsafe { set_bg_color(printer, value) };
            unsafe { regenerate_lookup(printer) };
            RENDER_REPEAT
        }
        EXT_CTRL_CODE_SHADOW => {
            let value = unsafe { next_char(printer) };
            unsafe { set_shadow_color(printer, value) };
            unsafe { regenerate_lookup(printer) };
            RENDER_REPEAT
        }
        EXT_CTRL_CODE_COLOR_HIGHLIGHT_SHADOW => {
            let fg = unsafe { next_char(printer) };
            let bg = unsafe { next_char(printer) };
            let shadow = unsafe { next_char(printer) };
            unsafe { set_fg_color(printer, fg) };
            unsafe { set_bg_color(printer, bg) };
            unsafe { set_shadow_color(printer, shadow) };
            unsafe { regenerate_lookup(printer) };
            RENDER_REPEAT
        }
        EXT_CTRL_CODE_PALETTE => {
            unsafe { skip_chars(printer, 1) };
            RENDER_REPEAT
        }
        EXT_CTRL_CODE_FONT => {
            let font_id = unsafe { next_char(printer) };
            unsafe { set_sub_font_id(printer, font_id) };
            RENDER_REPEAT
        }
        EXT_CTRL_CODE_RESET_FONT => RENDER_REPEAT,
        EXT_CTRL_CODE_PAUSE => {
            let delay = unsafe { next_char(printer) };
            unsafe { set(printer, P_DELAY_COUNTER, delay) };
            unsafe { set(printer, P_STATE, RENDER_STATE_PAUSE) };
            RENDER_REPEAT
        }
        EXT_CTRL_CODE_PAUSE_UNTIL_PRESS => {
            unsafe { set(printer, P_STATE, RENDER_STATE_WAIT) };
            if text_flag(FLAG_AUTO_SCROLL) {
                unsafe { set_auto_scroll_delay(printer, 0) };
            }
            RENDER_UPDATE
        }
        EXT_CTRL_CODE_WAIT_SE => {
            unsafe { set(printer, P_STATE, RENDER_STATE_WAIT_SE) };
            RENDER_UPDATE
        }
        EXT_CTRL_CODE_PLAY_BGM | EXT_CTRL_CODE_PLAY_SE => {
            let low = u16::from(unsafe { next_char(printer) });
            let high = u16::from(unsafe { next_char(printer) });
            let song = low | (high << 8);
            if code == EXT_CTRL_CODE_PLAY_BGM {
                unsafe { PlayBGM(song) };
            } else {
                unsafe { PlaySE(song) };
            }
            RENDER_REPEAT
        }
        EXT_CTRL_CODE_ESCAPE => {
            *char_ = u16::from(unsafe { next_char(printer) }) | 0x100;
            return None;
        }
        EXT_CTRL_CODE_SHIFT_RIGHT => {
            let x = unsafe { get(printer, T_X) }.wrapping_add(unsafe { next_char(printer) });
            unsafe { set(printer, T_CURRENT_X, x) };
            RENDER_REPEAT
        }
        EXT_CTRL_CODE_SHIFT_DOWN => {
            let y = unsafe { get(printer, T_Y) }.wrapping_add(unsafe { next_char(printer) });
            unsafe { set(printer, T_CURRENT_Y, y) };
            RENDER_REPEAT
        }
        EXT_CTRL_CODE_FILL_WINDOW => {
            unsafe { FillWindowPixelBuffer(get(printer, T_WINDOW_ID), bg_fill(printer)) };
            unsafe { set(printer, T_CURRENT_X, get(printer, T_X)) };
            unsafe { set(printer, T_CURRENT_Y, get(printer, T_Y)) };
            RENDER_REPEAT
        }
        EXT_CTRL_CODE_PAUSE_MUSIC => {
            unsafe { m4aMPlayStop(&raw mut gMPlayInfo_BGM) };
            RENDER_REPEAT
        }
        EXT_CTRL_CODE_RESUME_MUSIC => {
            unsafe { m4aMPlayContinue(&raw mut gMPlayInfo_BGM) };
            RENDER_REPEAT
        }
        EXT_CTRL_CODE_CLEAR => {
            let width = i32::from(unsafe { next_char(printer) });
            if width > 0 {
                unsafe { clear_and_advance(printer, width) };
                RENDER_PRINT
            } else {
                RENDER_REPEAT
            }
        }
        EXT_CTRL_CODE_SKIP_TO => {
            let x = unsafe { next_char(printer) }.wrapping_add(unsafe { get(printer, T_X) });
            unsafe { set(printer, T_CURRENT_X, x) };
            RENDER_REPEAT
        }
        EXT_CTRL_CODE_CLEAR_TO => {
            let target =
                i32::from(unsafe { next_char(printer) }) + i32::from(unsafe { get(printer, T_X) });
            let width = target - i32::from(unsafe { get(printer, T_CURRENT_X) });
            if width > 0 {
                unsafe { clear_and_advance(printer, width) };
                RENDER_PRINT
            } else {
                RENDER_REPEAT
            }
        }
        EXT_CTRL_CODE_MIN_LETTER_SPACING => {
            let spacing = unsafe { next_char(printer) };
            unsafe { set(printer, P_MIN_LETTER_SPACING, spacing) };
            RENDER_REPEAT
        }
        EXT_CTRL_CODE_JPN => {
            unsafe { set(printer, P_JAPANESE, 1) };
            RENDER_REPEAT
        }
        EXT_CTRL_CODE_ENG => {
            unsafe { set(printer, P_JAPANESE, 0) };
            RENDER_REPEAT
        }
        // An unknown code is drawn as a glyph, as in the original.
        _ => return None,
    };
    Some(ret)
}

unsafe fn clear_and_advance(printer: *mut u8, width: i32) {
    unsafe { ClearTextSpan(printer, width as u32) };
    let x = unsafe { get(printer, T_CURRENT_X) }.wrapping_add(width as u8);
    unsafe { set(printer, T_CURRENT_X, x) };
}

unsafe fn render_text(printer: *mut u8) -> u16 {
    match unsafe { get(printer, P_STATE) } {
        RENDER_STATE_HANDLE_CHAR => unsafe { render_handle_char(printer) },
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
                let distance = unsafe { (*info).max_letter_height }
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
                let speed_index = unsafe { GetPlayerTextSpeed() } as usize;
                let speed = unsafe { sWindowVerticalScrollSpeeds.as_ptr().add(speed_index).read() };
                let window_id = unsafe { get(printer, T_WINDOW_ID) };
                let fill = unsafe { bg_fill(printer) };
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

unsafe fn render_handle_char(printer: *mut u8) -> u16 {
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

    let recorded = unsafe { (&raw const gBattleTypeFlags).read() } & BATTLE_TYPE_RECORDED != 0;
    if !recorded && text_flag(FLAG_AUTO_SCROLL) {
        unsafe { set(printer, P_DELAY_COUNTER, 3) };
    } else {
        unsafe { set(printer, P_DELAY_COUNTER, get(printer, P_TEXT_SPEED)) };
    }

    let raw = unsafe { next_char(printer) };
    let mut char_ = u16::from(raw);
    match raw {
        CHAR_NEWLINE => {
            unsafe { set(printer, T_CURRENT_X, get(printer, T_X)) };
            let info = unsafe { font_info(get(printer, T_FONT_ID)) };
            let step = unsafe { (*info).max_letter_height }
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
            if let Some(ret) = unsafe { render_ext_ctrl_code(printer, &mut char_) } {
                return ret;
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
            let icon = unsafe { next_char(printer) };
            let width = unsafe {
                DrawKeypadIcon(
                    get(printer, T_WINDOW_ID),
                    icon,
                    u16::from(get(printer, T_CURRENT_X)),
                    u16::from(get(printer, T_CURRENT_Y)),
                )
            };
            unsafe { glyph().add(GLYPH_WIDTH).write(width) };
            let x = unsafe { get(printer, T_CURRENT_X) }
                .wrapping_add(width)
                .wrapping_add(unsafe { get(printer, T_LETTER_SPACING) });
            unsafe { set(printer, T_CURRENT_X, x) };
            return RENDER_PRINT;
        }
        EOS => return RENDER_FINISH,
        _ => {}
    }

    let japanese = u32::from(unsafe { get(printer, P_JAPANESE) });
    match unsafe { sub_font_id(printer) } {
        FONT_SMALL => unsafe { decompress_glyph_small(char_, japanese) },
        FONT_NORMAL => unsafe { decompress_glyph_normal(char_, japanese) },
        FONT_SHORT | FONT_SHORT_COPY_1 | FONT_SHORT_COPY_2 | FONT_SHORT_COPY_3 => unsafe {
            decompress_glyph_short(char_, japanese)
        },
        FONT_NARROW => unsafe { decompress_glyph_narrow(char_, japanese) },
        FONT_SMALL_NARROW => unsafe { decompress_glyph_small_narrow(char_, japanese) },
        _ => {}
    }

    unsafe { CopyGlyphToWindow(printer) };

    let width = unsafe { glyph_width() };
    let min_spacing = unsafe { get(printer, P_MIN_LETTER_SPACING) };
    let x = unsafe { get(printer, T_CURRENT_X) };
    if min_spacing != 0 {
        unsafe { set(printer, T_CURRENT_X, x.wrapping_add(width)) };
        let extra = i32::from(min_spacing) - i32::from(width);
        if extra > 0 {
            unsafe { clear_and_advance(printer, extra) };
        }
    } else if japanese != 0 {
        let spacing = unsafe { get(printer, T_LETTER_SPACING) };
        unsafe {
            set(
                printer,
                T_CURRENT_X,
                x.wrapping_add(width).wrapping_add(spacing),
            )
        };
    } else {
        unsafe { set(printer, T_CURRENT_X, x.wrapping_add(width)) };
    }
    RENDER_PRINT
}

// ----------------------------------------------------------- string width

/// Adds one glyph to a line, honouring the minimum glyph width and the
/// letter spacing Japanese text puts between (not after) glyphs.
fn add_glyph(
    line_width: &mut u32,
    min_glyph_width: i32,
    letter_spacing: i32,
    mut glyph_width: i32,
    next: *const u8,
    is_japanese: u32,
) {
    if min_glyph_width > 0 {
        if glyph_width < min_glyph_width {
            glyph_width = min_glyph_width;
        }
        *line_width = line_width.wrapping_add(glyph_width as u32);
    } else {
        *line_width = line_width.wrapping_add(glyph_width as u32);
        if is_japanese != 0 && unsafe { next.read() } != EOS {
            *line_width = line_width.wrapping_add(letter_spacing as u32);
        }
    }
}

fn glyph_width_func(font_id: u8) -> Option<GlyphWidthFunction> {
    GLYPH_WIDTH_FUNCS
        .iter()
        .find(|(id, _)| *id == font_id)
        .map(|(_, func)| *func)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetStringWidth(
    font_id: u8,
    mut string: *const u8,
    letter_spacing: i16,
) -> i32 {
    let mut is_japanese = 0u32;
    let mut min_glyph_width = 0i32;

    let Some(mut func) = glyph_width_func(font_id) else {
        return 0;
    };
    let mut local_letter_spacing = if letter_spacing == -1 {
        i32::from(unsafe { GetFontAttribute(font_id, FONTATTR_LETTER_SPACING) })
    } else {
        i32::from(letter_spacing)
    };

    let mut width: i32 = 0;
    let mut line_width: u32 = 0;

    loop {
        let c = unsafe { string.read() };
        if c == EOS {
            break;
        }
        match c {
            CHAR_NEWLINE => {
                if line_width > width as u32 {
                    width = line_width as i32;
                }
                line_width = 0;
            }
            PLACEHOLDER_BEGIN | CHAR_DYNAMIC => {
                let buffer: *const u8 = if c == PLACEHOLDER_BEGIN {
                    string = string.wrapping_add(1);
                    match unsafe { string.read() } {
                        PLACEHOLDER_ID_STRING_VAR_1 => (&raw const gStringVar1).cast(),
                        PLACEHOLDER_ID_STRING_VAR_2 => (&raw const gStringVar2).cast(),
                        PLACEHOLDER_ID_STRING_VAR_3 => (&raw const gStringVar3).cast(),
                        _ => return 0,
                    }
                } else {
                    string = string.wrapping_add(1);
                    unsafe { DynamicPlaceholderTextUtil_GetPlaceholderPtr(string.read()) }
                };
                let mut b = buffer;
                loop {
                    let g = unsafe { b.read() };
                    if g == EOS {
                        break;
                    }
                    b = b.wrapping_add(1);
                    let glyph_width = unsafe { func(u16::from(g), is_japanese) } as i32;
                    // The original checks the *source* string's next byte here.
                    add_glyph(
                        &mut line_width,
                        min_glyph_width,
                        local_letter_spacing,
                        glyph_width,
                        string.wrapping_add(1),
                        is_japanese,
                    );
                }
            }
            EXT_CTRL_CODE_BEGIN => {
                string = string.wrapping_add(1);
                match unsafe { string.read() } {
                    EXT_CTRL_CODE_COLOR_HIGHLIGHT_SHADOW => string = string.wrapping_add(3),
                    EXT_CTRL_CODE_PLAY_BGM | EXT_CTRL_CODE_PLAY_SE => {
                        string = string.wrapping_add(2)
                    }
                    EXT_CTRL_CODE_COLOR
                    | EXT_CTRL_CODE_HIGHLIGHT
                    | EXT_CTRL_CODE_SHADOW
                    | EXT_CTRL_CODE_PALETTE
                    | EXT_CTRL_CODE_PAUSE
                    | EXT_CTRL_CODE_ESCAPE
                    | EXT_CTRL_CODE_SHIFT_RIGHT
                    | EXT_CTRL_CODE_SHIFT_DOWN => string = string.wrapping_add(1),
                    EXT_CTRL_CODE_FONT => {
                        string = string.wrapping_add(1);
                        let new_font = unsafe { string.read() };
                        match glyph_width_func(new_font) {
                            Some(f) => func = f,
                            None => return 0,
                        }
                        if letter_spacing == -1 {
                            local_letter_spacing = i32::from(unsafe {
                                GetFontAttribute(new_font, FONTATTR_LETTER_SPACING)
                            });
                        }
                    }
                    EXT_CTRL_CODE_CLEAR => {
                        string = string.wrapping_add(1);
                        line_width = line_width.wrapping_add(u32::from(unsafe { string.read() }));
                    }
                    EXT_CTRL_CODE_SKIP_TO => {
                        string = string.wrapping_add(1);
                        line_width = u32::from(unsafe { string.read() });
                    }
                    EXT_CTRL_CODE_CLEAR_TO => {
                        string = string.wrapping_add(1);
                        let target = u32::from(unsafe { string.read() });
                        if target > line_width {
                            line_width = target;
                        }
                    }
                    EXT_CTRL_CODE_MIN_LETTER_SPACING => {
                        string = string.wrapping_add(1);
                        min_glyph_width = i32::from(unsafe { string.read() });
                    }
                    EXT_CTRL_CODE_JPN => is_japanese = 1,
                    EXT_CTRL_CODE_ENG => is_japanese = 0,
                    _ => {}
                }
            }
            CHAR_KEYPAD_ICON | CHAR_EXTRA_SYMBOL => {
                string = string.wrapping_add(1);
                let next = unsafe { string.read() };
                let glyph_width = if c == CHAR_EXTRA_SYMBOL {
                    (unsafe { func(u16::from(next) | 0x100, is_japanese) }) as i32
                } else {
                    i32::from(unsafe { GetKeypadIconWidth(next) })
                };
                add_glyph(
                    &mut line_width,
                    min_glyph_width,
                    local_letter_spacing,
                    glyph_width,
                    string.wrapping_add(1),
                    is_japanese,
                );
            }
            CHAR_PROMPT_SCROLL | CHAR_PROMPT_CLEAR => {}
            _ => {
                let glyph_width = unsafe { func(u16::from(c), is_japanese) } as i32;
                add_glyph(
                    &mut line_width,
                    min_glyph_width,
                    local_letter_spacing,
                    glyph_width,
                    string.wrapping_add(1),
                    is_japanese,
                );
            }
        }
        string = string.wrapping_add(1);
    }

    if line_width > width as u32 {
        line_width as i32
    } else {
        width
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RenderTextHandleBold(
    mut pixels: *mut u8,
    mut font_id: u8,
    string: *const u8,
) -> u8 {
    let mut backup = [0u8; 3];
    unsafe { SaveTextColors(&raw mut backup[0], &raw mut backup[1], &raw mut backup[2]) };

    let mut fg = TEXT_COLOR_WHITE;
    let mut bg = TEXT_COLOR_TRANSPARENT as u8;
    let mut shadow = TEXT_COLOR_LIGHT_GRAY;
    unsafe { GenerateFontHalfRowLookupTable(fg, bg, shadow) };

    let mut pos = 0usize;
    let read = |pos: &mut usize| -> u8 {
        let c = unsafe { string.add(*pos).read() };
        *pos += 1;
        c
    };

    loop {
        let temp = read(&mut pos);
        match temp {
            EXT_CTRL_CODE_BEGIN => {
                let code = read(&mut pos);
                match code {
                    EXT_CTRL_CODE_COLOR_HIGHLIGHT_SHADOW => {
                        fg = read(&mut pos);
                        bg = read(&mut pos);
                        shadow = read(&mut pos);
                        unsafe { GenerateFontHalfRowLookupTable(fg, bg, shadow) };
                    }
                    EXT_CTRL_CODE_COLOR => {
                        fg = read(&mut pos);
                        unsafe { GenerateFontHalfRowLookupTable(fg, bg, shadow) };
                    }
                    EXT_CTRL_CODE_HIGHLIGHT => {
                        bg = read(&mut pos);
                        unsafe { GenerateFontHalfRowLookupTable(fg, bg, shadow) };
                    }
                    EXT_CTRL_CODE_SHADOW => {
                        shadow = read(&mut pos);
                        unsafe { GenerateFontHalfRowLookupTable(fg, bg, shadow) };
                    }
                    EXT_CTRL_CODE_FONT => font_id = read(&mut pos),
                    EXT_CTRL_CODE_PLAY_BGM | EXT_CTRL_CODE_PLAY_SE => pos += 2,
                    EXT_CTRL_CODE_PALETTE
                    | EXT_CTRL_CODE_PAUSE
                    | EXT_CTRL_CODE_ESCAPE
                    | EXT_CTRL_CODE_SHIFT_RIGHT
                    | EXT_CTRL_CODE_SHIFT_DOWN
                    | EXT_CTRL_CODE_CLEAR
                    | EXT_CTRL_CODE_SKIP_TO
                    | EXT_CTRL_CODE_CLEAR_TO
                    | EXT_CTRL_CODE_MIN_LETTER_SPACING => pos += 1,
                    _ => {}
                }
            }
            CHAR_DYNAMIC | CHAR_KEYPAD_ICON | CHAR_EXTRA_SYMBOL | PLACEHOLDER_BEGIN => pos += 1,
            CHAR_PROMPT_SCROLL | CHAR_PROMPT_CLEAR | CHAR_NEWLINE | EOS => {}
            _ => {
                if font_id == FONT_BOLD {
                    unsafe { decompress_glyph_bold(u16::from(temp)) };
                } else {
                    unsafe { decompress_glyph_normal(u16::from(temp), 1) };
                }
                unsafe { cpu_copy32(glyph_top().cast(), pixels, 0x20) };
                unsafe { cpu_copy32(glyph_bottom().cast(), pixels.add(0x20), 0x20) };
                pixels = pixels.wrapping_add(0x40);
            }
        }
        if temp == EOS {
            break;
        }
    }

    unsafe { RestoreTextColors(&raw mut backup[0], &raw mut backup[1], &raw mut backup[2]) };
    1
}

#[inline]
unsafe fn cpu_copy32(src: *const u8, dest: *mut u8, size: u32) {
    const CPU_SET_32BIT: u32 = 0x0400_0000;
    unsafe { CpuSet(src.cast(), dest.cast(), CPU_SET_32BIT | (size / 4)) };
}

// --------------------------------------------------------- keypad icons

/// `sKeypadIcons[id]`: `{ u16 tileOffset; u8 width; u8 height; }`
#[inline]
unsafe fn keypad_icon(id: u8) -> *const u8 {
    unsafe { sKeypadIcons.as_ptr().add(usize::from(id) * 4) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawKeypadIcon(window_id: u8, keypad_icon_id: u8, x: u16, y: u16) -> u8 {
    let icon = unsafe { keypad_icon(keypad_icon_id) };
    let tile_offset = usize::from(unsafe { icon.cast::<u16>().read() });
    let width = unsafe { icon.add(2).read() };
    let height = unsafe { icon.add(3).read() };
    unsafe {
        BlitBitmapRectToWindow(
            window_id,
            sKeypadIconTiles.as_ptr().wrapping_add(tile_offset * 0x20),
            0,
            0,
            0x80,
            0x80,
            x,
            y,
            u16::from(width),
            u16::from(height),
        )
    };
    width
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetKeypadIconTileOffset(keypad_icon_id: u8) -> u8 {
    unsafe { keypad_icon(keypad_icon_id).cast::<u16>().read() as u8 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetKeypadIconWidth(keypad_icon_id: u8) -> u8 {
    unsafe { keypad_icon(keypad_icon_id).add(2).read() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetKeypadIconHeight(keypad_icon_id: u8) -> u8 {
    unsafe { keypad_icon(keypad_icon_id).add(3).read() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDefaultFontsPointer() {
    unsafe { (&raw mut gFonts).write(FONT_INFOS.as_ptr()) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFontAttribute(font_id: u8, attribute_id: u8) -> u8 {
    let info = FONT_INFOS.as_ptr().wrapping_add(usize::from(font_id));
    let info = unsafe { &*info };
    match attribute_id {
        FONTATTR_MAX_LETTER_WIDTH => info.max_letter_width,
        FONTATTR_MAX_LETTER_HEIGHT => info.max_letter_height,
        FONTATTR_LETTER_SPACING => info.letter_spacing,
        FONTATTR_LINE_SPACING => info.line_spacing,
        FONTATTR_UNKNOWN => info.colors0 & 0x0f,
        FONTATTR_COLOR_FOREGROUND => info.colors0 >> 4,
        FONTATTR_COLOR_BACKGROUND => info.colors1 & 0x0f,
        FONTATTR_COLOR_SHADOW => info.colors1 >> 4,
        _ => 0,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMenuCursorDimensionByFont(font_id: u8, which_dimension: u8) -> u8 {
    let index = usize::from(font_id) * 2 + usize::from(which_dimension);
    unsafe { sMenuCursorDimensions.as_ptr().add(index).read() }
}

// ---------------------------------------------------------- glyph decoding

/// Latin glyphs: 0x20 halfwords each, one or two tiles wide.
unsafe fn decompress_latin(glyphs: *const u16, widths: *const u8, glyph_id: u16, height: u8) {
    let glyphs = glyphs.wrapping_add(0x20 * usize::from(glyph_id));
    let width = unsafe { widths.add(usize::from(glyph_id)).read() };
    unsafe { glyph().add(GLYPH_WIDTH).write(width) };
    if width <= 8 {
        unsafe { DecompressGlyphTile(glyphs.cast(), glyph_top().cast()) };
        unsafe { DecompressGlyphTile(glyphs.wrapping_add(0x10).cast(), glyph_bottom().cast()) };
    } else {
        unsafe { decompress_wide(glyphs, 0x8, 0x10, 0x18) };
    }
    unsafe { glyph().add(GLYPH_HEIGHT).write(height) };
}

/// Four tiles: top-left, top-right, bottom-left, bottom-right.
unsafe fn decompress_wide(
    glyphs: *const u16,
    top_right: usize,
    bottom_left: usize,
    bottom_right: usize,
) {
    unsafe { DecompressGlyphTile(glyphs.cast(), glyph_top().cast()) };
    unsafe {
        DecompressGlyphTile(
            glyphs.wrapping_add(top_right).cast(),
            glyph_top().add(8).cast(),
        )
    };
    unsafe {
        DecompressGlyphTile(
            glyphs.wrapping_add(bottom_left).cast(),
            glyph_bottom().cast(),
        )
    };
    unsafe {
        DecompressGlyphTile(
            glyphs.wrapping_add(bottom_right).cast(),
            glyph_bottom().add(8).cast(),
        )
    };
}

/// Japanese half-width glyphs: 16 per row of 0x100 halfwords, 8x16.
unsafe fn decompress_japanese(glyphs: *const u16, glyph_id: u16, width: u8, height: u8) {
    let id = usize::from(glyph_id);
    let glyphs = glyphs.wrapping_add(0x100 * (id >> 4) + 0x8 * (id & 0xf));
    unsafe { DecompressGlyphTile(glyphs.cast(), glyph_top().cast()) };
    unsafe { DecompressGlyphTile(glyphs.wrapping_add(0x80).cast(), glyph_bottom().cast()) };
    unsafe { set_glyph_size(width, height) };
}

unsafe fn decompress_glyph_small(glyph_id: u16, is_japanese: u32) {
    if is_japanese == 1 {
        unsafe { decompress_japanese(&raw const gFontSmallJapaneseGlyphs, glyph_id, 8, 12) };
    } else {
        unsafe {
            decompress_latin(
                &raw const gFontSmallLatinGlyphs,
                &raw const gFontSmallLatinGlyphWidths,
                glyph_id,
                13,
            )
        };
    }
}

unsafe fn decompress_glyph_narrow(glyph_id: u16, is_japanese: u32) {
    if is_japanese == 1 {
        unsafe { decompress_japanese(&raw const gFontNormalJapaneseGlyphs, glyph_id, 8, 15) };
    } else {
        unsafe {
            decompress_latin(
                &raw const gFontNarrowLatinGlyphs,
                &raw const gFontNarrowLatinGlyphWidths,
                glyph_id,
                15,
            )
        };
    }
}

unsafe fn decompress_glyph_small_narrow(glyph_id: u16, is_japanese: u32) {
    if is_japanese == 1 {
        unsafe { decompress_japanese(&raw const gFontSmallJapaneseGlyphs, glyph_id, 8, 12) };
    } else {
        unsafe {
            decompress_latin(
                &raw const gFontSmallNarrowLatinGlyphs,
                &raw const gFontSmallNarrowLatinGlyphWidths,
                glyph_id,
                12,
            )
        };
    }
}

unsafe fn decompress_glyph_short(glyph_id: u16, is_japanese: u32) {
    if is_japanese == 1 {
        let id = usize::from(glyph_id);
        let base =
            (&raw const gFontShortJapaneseGlyphs).wrapping_add(0x100 * (id >> 3) + 0x10 * (id & 7));
        unsafe { decompress_wide(base, 0x8, 0x80, 0x88) };
        let width = unsafe { (&raw const gFontShortJapaneseGlyphWidths).add(id).read() };
        unsafe { set_glyph_size(width, 14) };
    } else {
        unsafe {
            decompress_latin(
                &raw const gFontShortLatinGlyphs,
                &raw const gFontShortLatinGlyphWidths,
                glyph_id,
                14,
            )
        };
    }
}

unsafe fn decompress_glyph_normal(glyph_id: u16, is_japanese: u32) {
    if is_japanese == 1 {
        unsafe { decompress_japanese(&raw const gFontNormalJapaneseGlyphs, glyph_id, 8, 15) };
    } else {
        unsafe {
            decompress_latin(
                &raw const gFontNormalLatinGlyphs,
                &raw const gFontNormalLatinGlyphWidths,
                glyph_id,
                15,
            )
        };
    }
}

unsafe fn decompress_glyph_bold(glyph_id: u16) {
    unsafe { decompress_japanese(sFontBoldJapaneseGlyphs.as_ptr().cast(), glyph_id, 8, 12) };
}

unsafe extern "C" fn glyph_width_small(glyph_id: u16, is_japanese: u32) -> u32 {
    if is_japanese == 1 {
        8
    } else {
        u32::from(unsafe {
            (&raw const gFontSmallLatinGlyphWidths)
                .add(usize::from(glyph_id))
                .read()
        })
    }
}

unsafe extern "C" fn glyph_width_narrow(glyph_id: u16, is_japanese: u32) -> u32 {
    if is_japanese == 1 {
        8
    } else {
        u32::from(unsafe {
            (&raw const gFontNarrowLatinGlyphWidths)
                .add(usize::from(glyph_id))
                .read()
        })
    }
}

unsafe extern "C" fn glyph_width_small_narrow(glyph_id: u16, is_japanese: u32) -> u32 {
    if is_japanese == 1 {
        8
    } else {
        u32::from(unsafe {
            (&raw const gFontSmallNarrowLatinGlyphWidths)
                .add(usize::from(glyph_id))
                .read()
        })
    }
}

unsafe extern "C" fn glyph_width_short(glyph_id: u16, is_japanese: u32) -> u32 {
    let widths = if is_japanese == 1 {
        &raw const gFontShortJapaneseGlyphWidths
    } else {
        &raw const gFontShortLatinGlyphWidths
    };
    u32::from(unsafe { widths.add(usize::from(glyph_id)).read() })
}

unsafe extern "C" fn glyph_width_normal(glyph_id: u16, is_japanese: u32) -> u32 {
    if is_japanese == 1 {
        8
    } else {
        u32::from(unsafe {
            (&raw const gFontNormalLatinGlyphWidths)
                .add(usize::from(glyph_id))
                .read()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_table_matches_the_unrolled_original() {
        unsafe { GenerateFontHalfRowLookupTable(2, 1, 3) };
        let table = unsafe { (&raw const FONT_HALF_ROW_LOOKUP_TABLE).read() }.0;
        // First block: temp = bg<<8 | bg<<4 | bg, then bg12/fg12/shadow12.
        assert_eq!(table[0], 0x1111);
        assert_eq!(table[1], 0x2111);
        assert_eq!(table[2], 0x3111);
        // Fourth block (index 9..): temp = bg<<8 | fg<<4 | bg.
        assert_eq!(table[9], 0x1121);
        // Last entry: shadow everywhere except the low nibble, which is shadow too.
        assert_eq!(table[0x50], 0x3333);
        // Tenth block (index 27): temp = bg<<8 | bg<<4 | fg.
        assert_eq!(table[27], 0x1112);
    }

    #[test]
    fn font_info_colours_pack_like_gcc() {
        let normal = font(None, 6, 16, 0, 2, 1, 3);
        assert_eq!(normal.colors0, 0x20);
        assert_eq!(normal.colors1, 0x31);
        let bold = font(None, 8, 8, 0, 1, 2, 15);
        assert_eq!(bold.colors1, 0xf2);
    }
}
