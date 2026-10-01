use crate::ffi::{
    AddTextPrinterParameterized, AddWindow, ClearStdWindowAndFrameToTransparent,
    ClearWindowTilemap, ConvertIntToDecimalStringN, DrawStdFrameWithCustomTileAndPalette,
    FONT_NORMAL, FillWindowPixelBuffer, PutWindowTilemap, RemoveWindow, SetWindowTemplateFields,
    WindowTemplate, gSpecialVar_0x8004, gStringVar1,
};
use crate::types::SaveBlock2;
use core::ffi::c_int;
use core::ptr::addr_of;

const MAX_BERRY_POWDER: u32 = 99_999;

const STR_CONV_MODE_RIGHT_ALIGN: c_int = 1;
const TEXT_SKIP_DRAW: u8 = 0xff;
const PIXEL_FILL_0: u8 = 0;

/// Tile and palette the vendor menu borrows for its window border.
const VENDOR_BASE_TILE: u16 = 0x21d;
const VENDOR_PALETTE: u8 = 13;
/// `BG_PLTT_ID(13)`
const VENDOR_PALETTE_OFFSET: u16 = 208;

// The C file also carries unreferenced BG and window template tables marked
// UNUSED. They are never linked into the ROM, so they are not reproduced here.

#[unsafe(link_section = "ewram_data")]
static mut VENDOR_WINDOW_ID: u8 = 0;

/// `ApplyNewEncryptionKeyToWord` with this module's view of its types.
#[inline]
unsafe fn ApplyNewEncryptionKeyToWord(a0: *mut u32, a1: u32) {
    unsafe {
        crate::load_save::ApplyNewEncryptionKeyToWord(a0 as _, a1);
    }
}
/// `LoadUserWindowBorderGfx_` with this module's view of its types.
#[inline]
unsafe fn LoadUserWindowBorderGfx_(a0: u8, a1: u16, a2: u16) {
    unsafe {
        crate::text_window::LoadUserWindowBorderGfx_(a0, a1, a2 as _);
    }
}

/// The player's berry powder (from Berry Crush), stored in the save XORed
/// with its encryption key.
pub struct BerryPowder<'a> {
    stored: &'a mut u32,
    key: u32,
}

impl<'a> BerryPowder<'a> {
    pub fn new(save: &'a mut SaveBlock2) -> Self {
        let key = save.encryptionKey;
        Self::from_parts(&mut save.berryCrush.berryPowderAmount, key)
    }

    pub fn from_parts(stored: &'a mut u32, key: u32) -> Self {
        Self { stored, key }
    }

    pub fn get(&self) -> u32 {
        *self.stored ^ self.key
    }

    pub fn set(&mut self, amount: u32) {
        *self.stored = amount ^ self.key;
    }

    pub fn has_enough(&self, cost: u32) -> bool {
        self.get() >= cost
    }

    /// Adds powder. Past [`MAX_BERRY_POWDER`] it stops there and returns
    /// false. (As in C, a sum past 2^32 wraps around first.)
    pub fn give(&mut self, amount: u32) -> bool {
        let total = self.get().wrapping_add(amount);
        if total > MAX_BERRY_POWDER {
            self.set(MAX_BERRY_POWDER);
            false
        } else {
            self.set(total);
            true
        }
    }

    /// Spends powder. Returns false, changing nothing, if there isn't enough.
    pub fn take(&mut self, cost: u32) -> bool {
        match self.get().checked_sub(cost) {
            Some(left) => {
                self.set(left);
                true
            }
            None => false,
        }
    }
}

// ------------------------------------------------------------------ C names

/// # Safety
/// The save blocks must be set up and not in use elsewhere meanwhile.
unsafe fn save() -> &'static mut SaveBlock2 {
    unsafe { crate::save_blocks::save_block2() }
}

/// The powder at `powder` (a save field), with the save's key.
#[unsafe(no_mangle)]
pub unsafe fn SetBerryPowder(powder: *mut u32, amount: u32) {
    unsafe { BerryPowder::from_parts(&mut *powder, save().encryptionKey) }.set(amount);
}

#[unsafe(no_mangle)]
pub unsafe fn ApplyNewEncryptionKeyToBerryPowder(encryption_key: u32) {
    unsafe {
        ApplyNewEncryptionKeyToWord(&raw mut save().berryCrush.berryPowderAmount, encryption_key)
    };
}

/// Script special: whether the player has `VAR_0x8004` powder.
#[unsafe(no_mangle)]
pub unsafe fn HasEnoughBerryPowder() -> u8 {
    let cost = u32::from(unsafe { gSpecialVar_0x8004 });
    BerryPowder::new(unsafe { save() }).has_enough(cost).into()
}

#[unsafe(no_mangle)]
pub unsafe fn GiveBerryPowder(amount: u32) -> u8 {
    BerryPowder::new(unsafe { save() }).give(amount).into()
}

/// Script special: the player spends `VAR_0x8004` powder.
#[unsafe(no_mangle)]
pub unsafe fn TakeBerryPowder() -> u8 {
    let cost = u32::from(unsafe { gSpecialVar_0x8004 });
    BerryPowder::new(unsafe { save() }).take(cost).into()
}

#[unsafe(no_mangle)]
pub unsafe fn GetBerryPowder() -> u32 {
    BerryPowder::new(unsafe { save() }).get()
}

unsafe fn print_berry_powder_amount(window_id: u8, amount: i32, x: u8, y: u8, speed: u8) {
    let var1 = (&raw mut gStringVar1).cast::<u8>();
    let _ = unsafe { ConvertIntToDecimalStringN(var1, amount, STR_CONV_MODE_RIGHT_ALIGN, 5) };
    let _ = unsafe { AddTextPrinterParameterized(window_id, FONT_NORMAL, var1, x, y, speed, None) };
}

unsafe fn draw_player_powder_amount(
    window_id: u8,
    base_tile_offset: u16,
    palette_num: u8,
    amount: u32,
) {
    unsafe { DrawStdFrameWithCustomTileAndPalette(window_id, 0, base_tile_offset, palette_num) };
    let _ = unsafe {
        AddTextPrinterParameterized(
            window_id,
            FONT_NORMAL,
            &raw const (*(&raw const crate::data::strings::gText_Powder).cast::<u8>()),
            0,
            1,
            TEXT_SKIP_DRAW,
            None,
        )
    };
    unsafe { print_berry_powder_amount(window_id, amount as i32, 26, 17, 0) };
}

#[unsafe(no_mangle)]
pub unsafe fn PrintPlayerBerryPowderAmount() {
    let amount = unsafe { GetBerryPowder() };
    let window_id = unsafe { addr_of!(VENDOR_WINDOW_ID).read_volatile() };
    unsafe { print_berry_powder_amount(window_id, amount as i32, 26, 17, 0) };
}

#[unsafe(no_mangle)]
pub unsafe fn DisplayBerryPowderVendorMenu() {
    let mut template = WindowTemplate::default();
    unsafe { SetWindowTemplateFields(&raw mut template, 0, 1, 1, 7, 4, 15, 0x1c) };

    let window_id = unsafe { AddWindow(&raw const template) } as u8;
    unsafe { (&raw mut VENDOR_WINDOW_ID).write_volatile(window_id) };

    unsafe { FillWindowPixelBuffer(window_id, PIXEL_FILL_0) };
    unsafe { PutWindowTilemap(window_id) };
    unsafe { LoadUserWindowBorderGfx_(window_id, VENDOR_BASE_TILE, VENDOR_PALETTE_OFFSET) };
    unsafe {
        draw_player_powder_amount(
            window_id,
            VENDOR_BASE_TILE,
            VENDOR_PALETTE,
            GetBerryPowder(),
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe fn RemoveBerryPowderVendorMenu() {
    let window_id = unsafe { addr_of!(VENDOR_WINDOW_ID).read_volatile() };
    unsafe { ClearWindowTilemap(window_id) };
    unsafe { ClearStdWindowAndFrameToTransparent(window_id, 1) };
    unsafe { RemoveWindow(window_id) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn giving_powder_caps_and_reports_the_overflow() {
        let mut stored = 0;
        let mut powder = BerryPowder::from_parts(&mut stored, 0xabcd_ef01);
        powder.set(0);
        assert!(powder.give(10));
        assert_eq!(powder.get(), 10);
        powder.set(MAX_BERRY_POWDER);
        assert!(powder.give(0));
        assert!(!powder.give(1));
        assert_eq!(powder.get(), MAX_BERRY_POWDER);
    }

    #[test]
    fn taking_powder_needs_enough() {
        let mut stored = 0;
        let mut powder = BerryPowder::from_parts(&mut stored, 7);
        powder.set(50);
        assert!(!powder.take(51));
        assert_eq!(powder.get(), 50);
        assert!(powder.take(50));
        assert_eq!(powder.get(), 0);
    }
}
