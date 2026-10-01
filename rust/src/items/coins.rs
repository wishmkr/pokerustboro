//! Game Corner coins (was src/coins.c): the player's coin count and the
//! window that shows it.
//!
//! The count lives in the save (`SaveBlock1::coins`), XORed with the save's
//! encryption key like the player's money. [`Coins`] is the safe API for it.
//! The `extern "C"` functions below keep the C names that the rest of the
//! game still calls; they only hand [`Coins`] the save blocks.

use crate::ffi::{
    AddTextPrinterParameterized, AddWindow, ConvertIntToDecimalStringN,
    DrawStdFrameWithCustomTileAndPalette, FONT_NORMAL, FillWindowPixelBuffer, PutWindowTilemap,
    RemoveWindow, SetWindowTemplateFields, StringExpandPlaceholders, WindowTemplate, gStringVar1,
    gStringVar4,
};
use crate::save_blocks::{save_block1, save_block2};
use crate::types::{SaveBlock1, SaveBlock2};
use core::ffi::c_int;

/// The most coins the player can hold.
pub const MAX_COINS: u16 = 9_999;

/// A save's coin count, decoded with its encryption key.
pub struct Coins<'a> {
    stored: &'a mut u16,
    key: u16,
}

impl<'a> Coins<'a> {
    /// The coins of this save.
    pub fn new(save1: &'a mut SaveBlock1, save2: &SaveBlock2) -> Self {
        Self::from_parts(&mut save1.coins, save2.encryptionKey)
    }

    /// Coins stored in `stored`, encrypted with `key` (only its low 16 bits
    /// matter, as in C).
    pub fn from_parts(stored: &'a mut u16, key: u32) -> Self {
        Self {
            stored,
            key: key as u16,
        }
    }

    /// How many coins the player has.
    pub fn get(&self) -> u16 {
        *self.stored ^ self.key
    }

    /// Sets the player's coins (not capped; the callers stay in range).
    pub fn set(&mut self, amount: u16) {
        *self.stored = amount ^ self.key;
    }

    /// Adds coins, up to [`MAX_COINS`]. Returns false, changing nothing, if
    /// the player already has the maximum.
    pub fn add(&mut self, amount: u16) -> bool {
        let owned = self.get();
        if owned >= MAX_COINS {
            return false;
        }
        self.set(owned.saturating_add(amount).min(MAX_COINS));
        true
    }

    /// Takes coins away. Returns false, changing nothing, if the player has
    /// fewer than `amount`.
    pub fn remove(&mut self, amount: u16) -> bool {
        match self.get().checked_sub(amount) {
            Some(left) => {
                self.set(left);
                true
            }
            None => false,
        }
    }
}

// ------------------------------------------------------------------ C names

/// `GetStringRightAlignXOffset` with this module's view of its types.
#[inline]
unsafe fn GetStringRightAlignXOffset(a0: c_int, a1: *const u8, a2: c_int) -> c_int {
    unsafe { crate::international_string_util::GetStringRightAlignXOffset(a0, a1 as _, a2) }
}
/// `ClearStdWindowAndFrame` with this module's view of its types.
#[inline]
unsafe fn ClearStdWindowAndFrame(a0: u8, a1: u8) {
    unsafe {
        crate::menu::ClearStdWindowAndFrame(a0, a1);
    }
}

/// The save's coins.
///
/// # Safety
/// The save blocks must be set up (they are, from boot on), and nothing else
/// may be using them while the result lives.
unsafe fn save_coins() -> Coins<'static> {
    unsafe { Coins::new(save_block1(), save_block2()) }
}

#[unsafe(no_mangle)]
pub unsafe fn GetCoins() -> u16 {
    unsafe { save_coins() }.get()
}

#[unsafe(no_mangle)]
pub unsafe fn SetCoins(amount: u16) {
    unsafe { save_coins() }.set(amount);
}

#[unsafe(no_mangle)]
pub unsafe fn AddCoins(amount: u16) -> u8 {
    unsafe { save_coins() }.add(amount).into()
}

#[unsafe(no_mangle)]
pub unsafe fn RemoveCoins(amount: u16) -> u8 {
    unsafe { save_coins() }.remove(amount).into()
}

// ------------------------------------------------------------ coins window

/// The window showing the player's coins, while it is open.
#[unsafe(link_section = "ewram_data")]
static COINS_WINDOW_ID: crate::global::Global<u8> = crate::global::Global::new(0);

const STR_CONV_MODE_RIGHT_ALIGN: c_int = 1;

/// Prints "`amount` COINS" right-aligned in the coins window.
#[unsafe(no_mangle)]
pub unsafe fn PrintCoinsString(amount: u32) {
    unsafe {
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast(),
            amount as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            4,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast(),
            &raw const (*(&raw const crate::data::strings::gText_Coins).cast::<u8>()),
        );
        let text = (&raw const gStringVar4).cast::<u8>();
        let x = GetStringRightAlignXOffset(c_int::from(FONT_NORMAL), text, 0x40);
        AddTextPrinterParameterized(
            COINS_WINDOW_ID.get(),
            FONT_NORMAL,
            text,
            x as u8,
            1,
            0,
            None,
        );
    }
}

/// Opens the coins window at tile (x, y), showing `amount`.
#[unsafe(no_mangle)]
pub unsafe fn ShowCoinsWindow(amount: u32, x: u8, y: u8) {
    let mut template = WindowTemplate::default();
    unsafe {
        SetWindowTemplateFields(&raw mut template, 0, x, y, 8, 2, 0xf, 0x141);
        COINS_WINDOW_ID.set(AddWindow(&raw const template) as u8);
        FillWindowPixelBuffer(COINS_WINDOW_ID.get(), 0);
        PutWindowTilemap(COINS_WINDOW_ID.get());
        DrawStdFrameWithCustomTileAndPalette(COINS_WINDOW_ID.get(), 0, 0x214, 0xe);
        PrintCoinsString(amount);
    }
}

/// Closes the coins window.
#[unsafe(no_mangle)]
pub unsafe fn HideCoinsWindow() {
    unsafe {
        ClearStdWindowAndFrame(COINS_WINDOW_ID.get(), 1);
        RemoveWindow(COINS_WINDOW_ID.get());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coins_are_stored_encrypted() {
        let mut stored = 0;
        let mut coins = Coins::from_parts(&mut stored, 0xdead_beef);
        coins.set(9_999);
        assert_eq!(coins.get(), 9_999);
        assert_eq!(stored, 9_999 ^ 0xbeef);
    }

    #[test]
    fn adding_caps_at_the_maximum() {
        let mut stored = 0;
        let mut coins = Coins::from_parts(&mut stored, 0x1234);
        coins.set(9_990);
        assert!(coins.add(50));
        assert_eq!(coins.get(), MAX_COINS);
        // already full: nothing changes
        assert!(!coins.add(1));
        assert_eq!(coins.get(), MAX_COINS);
        // a huge amount doesn't wrap around
        coins.set(5);
        assert!(coins.add(u16::MAX));
        assert_eq!(coins.get(), MAX_COINS);
    }

    #[test]
    fn removing_needs_enough_coins() {
        let mut stored = 0;
        let mut coins = Coins::from_parts(&mut stored, 0);
        coins.set(100);
        assert!(!coins.remove(101));
        assert_eq!(coins.get(), 100);
        assert!(coins.remove(100));
        assert_eq!(coins.get(), 0);
    }
}
