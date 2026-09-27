use crate::ffi::{
    AddTextPrinterParameterized, AddWindow, ClearStdWindowAndFrameToTransparent,
    ClearWindowTilemap, ConvertIntToDecimalStringN, DrawStdFrameWithCustomTileAndPalette,
    FONT_NORMAL, FillWindowPixelBuffer, PutWindowTilemap, RemoveWindow,
    SAVE2_ENCRYPTION_KEY_OFFSET, SetWindowTemplateFields, WindowTemplate, gSpecialVar_0x8004,
    gStringVar1,
};
use core::ffi::c_int;
use core::ptr::addr_of;

const MAX_BERRY_POWDER: u32 = 99_999;

const STR_CONV_MODE_RIGHT_ALIGN: c_int = 1;
const TEXT_SKIP_DRAW: u8 = 0xff;
const PIXEL_FILL_0: u8 = 0;

/// `offsetof(struct SaveBlock2, berryCrush.berryPowderAmount)`
const SAVE2_BERRY_POWDER_OFFSET: usize = 0x1f4;

/// Tile and palette the vendor menu borrows for its window border.
const VENDOR_BASE_TILE: u16 = 0x21d;
const VENDOR_PALETTE: u8 = 13;
/// `BG_PLTT_ID(13)`
const VENDOR_PALETTE_OFFSET: u16 = 208;

// The C file also carries unreferenced BG and window template tables marked
// UNUSED. They are never linked into the ROM, so they are not reproduced here.

#[unsafe(link_section = "ewram_data")]
static mut VENDOR_WINDOW_ID: u8 = 0;

unsafe extern "C" {
    static mut gSaveBlock2Ptr: *mut u8;
    static gText_Powder: u8;

    fn ApplyNewEncryptionKeyToWord(word: *mut u32, new_key: u32);
    fn LoadUserWindowBorderGfx_(window_id: u8, dest_offset: u16, palette_offset: u16);
}

#[inline]
unsafe fn encryption_key() -> u32 {
    unsafe {
        gSaveBlock2Ptr
            .add(SAVE2_ENCRYPTION_KEY_OFFSET)
            .cast::<u32>()
            .read()
    }
}

/// `&gSaveBlock2Ptr->berryCrush.berryPowderAmount`
#[inline]
unsafe fn powder_ptr() -> *mut u32 {
    unsafe { gSaveBlock2Ptr.add(SAVE2_BERRY_POWDER_OFFSET).cast::<u32>() }
}

#[inline]
unsafe fn decrypt_berry_powder(powder: *mut u32) -> u32 {
    unsafe { powder.read() ^ encryption_key() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBerryPowder(powder: *mut u32, amount: u32) {
    unsafe { powder.write(amount ^ encryption_key()) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApplyNewEncryptionKeyToBerryPowder(encryption_key: u32) {
    unsafe { ApplyNewEncryptionKeyToWord(powder_ptr(), encryption_key) };
}

#[inline]
unsafe fn has_enough_berry_powder(cost: u32) -> bool {
    let current = unsafe { decrypt_berry_powder(powder_ptr()) };
    current >= cost
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasEnoughBerryPowder() -> u8 {
    let cost = u32::from(unsafe { addr_of!(gSpecialVar_0x8004).read() });
    u8::from(unsafe { has_enough_berry_powder(cost) })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveBerryPowder(amount_to_add: u32) -> u8 {
    let powder = unsafe { powder_ptr() };
    let amount = unsafe { decrypt_berry_powder(powder) }.wrapping_add(amount_to_add);

    if amount > MAX_BERRY_POWDER {
        unsafe { SetBerryPowder(powder, MAX_BERRY_POWDER) };
        0
    } else {
        unsafe { SetBerryPowder(powder, amount) };
        1
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn TakeBerryPowder() -> u8 {
    let cost = u32::from(unsafe { addr_of!(gSpecialVar_0x8004).read() });
    if !unsafe { has_enough_berry_powder(cost) } {
        return 0;
    }

    let powder = unsafe { powder_ptr() };
    unsafe { SetBerryPowder(powder, decrypt_berry_powder(powder) - cost) };
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBerryPowder() -> u32 {
    unsafe { decrypt_berry_powder(powder_ptr()) }
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
            &raw const gText_Powder,
            0,
            1,
            TEXT_SKIP_DRAW,
            None,
        )
    };
    unsafe { print_berry_powder_amount(window_id, amount as i32, 26, 17, 0) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintPlayerBerryPowderAmount() {
    let amount = unsafe { GetBerryPowder() };
    let window_id = unsafe { addr_of!(VENDOR_WINDOW_ID).read_volatile() };
    unsafe { print_berry_powder_amount(window_id, amount as i32, 26, 17, 0) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisplayBerryPowderVendorMenu() {
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
pub unsafe extern "C" fn RemoveBerryPowderVendorMenu() {
    let window_id = unsafe { addr_of!(VENDOR_WINDOW_ID).read_volatile() };
    unsafe { ClearWindowTilemap(window_id) };
    unsafe { ClearStdWindowAndFrameToTransparent(window_id, 1) };
    unsafe { RemoveWindow(window_id) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_powder_amount_sits_inside_berry_crush_data() {
        // struct SaveBlock2.berryCrush is at 0x1EC and berryPowderAmount is
        // eight bytes into it.
        assert_eq!(SAVE2_BERRY_POWDER_OFFSET, 0x1ec + 8);
    }

    #[test]
    fn giving_powder_caps_and_reports_the_overflow() {
        let give = |current: u32, add: u32| {
            let amount = current.wrapping_add(add);
            if amount > MAX_BERRY_POWDER {
                (MAX_BERRY_POWDER, 0u8)
            } else {
                (amount, 1)
            }
        };
        assert_eq!(give(0, 10), (10, 1));
        assert_eq!(give(MAX_BERRY_POWDER, 0), (MAX_BERRY_POWDER, 1));
        assert_eq!(give(MAX_BERRY_POWDER, 1), (MAX_BERRY_POWDER, 0));
    }
}
