use crate::ffi::{
    AddTextPrinterParameterized, AddWindow, ConvertIntToDecimalStringN,
    DrawStdFrameWithCustomTileAndPalette, FONT_NORMAL, FillWindowPixelBuffer, PutWindowTilemap,
    RemoveWindow, SAVE2_ENCRYPTION_KEY_OFFSET, SetWindowTemplateFields, StringExpandPlaceholders,
    WindowTemplate, gStringVar1, gStringVar4,
};
use core::ffi::c_int;
use core::ptr::addr_of;

const MAX_COINS: u16 = 9_999;
const STR_CONV_MODE_RIGHT_ALIGN: c_int = 1;

/// `offsetof(struct SaveBlock1, coins)`
const SAVE1_COINS_OFFSET: usize = 0x494;

unsafe extern "C" {
    static mut gSaveBlock1Ptr: *mut u8;
    static mut gSaveBlock2Ptr: *mut u8;
    static gText_Coins: u8;

    fn GetStringRightAlignXOffset(font_id: c_int, text: *const u8, total_width: c_int) -> c_int;
    fn ClearStdWindowAndFrame(window_id: u8, copy_to_vram: u8);
}

/// `&gSaveBlock1Ptr->coins`
#[inline]
unsafe fn coins_ptr() -> *mut u16 {
    unsafe { gSaveBlock1Ptr.add(SAVE1_COINS_OFFSET).cast::<u16>() }
}

/// `gSaveBlock2Ptr->encryptionKey`
#[inline]
unsafe fn encryption_key() -> u32 {
    unsafe {
        gSaveBlock2Ptr
            .add(SAVE2_ENCRYPTION_KEY_OFFSET)
            .cast::<u32>()
            .read()
    }
}

#[unsafe(link_section = "ewram_data")]
static mut COINS_WINDOW_ID: u8 = 0;

const fn decode_coins(stored: u16, key: u32) -> u16 {
    (stored as u32 ^ key) as u16
}

const fn encode_coins(amount: u16, key: u32) -> u16 {
    (amount as u32 ^ key) as u16
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintCoinsString(coin_amount: u32) {
    unsafe {
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            coin_amount as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            4,
        )
    };
    unsafe { StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), addr_of!(gText_Coins)) };
    let x_align = unsafe {
        GetStringRightAlignXOffset(
            c_int::from(FONT_NORMAL),
            (&raw const gStringVar4).cast::<u8>(),
            0x40,
        )
    };
    let window_id = unsafe { (&raw const COINS_WINDOW_ID).read() };
    let _ = unsafe {
        AddTextPrinterParameterized(
            window_id,
            FONT_NORMAL,
            (&raw const gStringVar4).cast::<u8>(),
            x_align as u8,
            1,
            0,
            None,
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowCoinsWindow(coin_amount: u32, x: u8, y: u8) {
    let mut template = WindowTemplate::default();
    unsafe { SetWindowTemplateFields(&raw mut template, 0, x, y, 8, 2, 0xf, 0x141) };
    let window_id = unsafe { AddWindow(&raw const template) } as u8;
    unsafe { (&raw mut COINS_WINDOW_ID).write(window_id) };
    unsafe { FillWindowPixelBuffer(window_id, 0) };
    unsafe { PutWindowTilemap(window_id) };
    unsafe { DrawStdFrameWithCustomTileAndPalette(window_id, 0, 0x214, 0xe) };
    unsafe { PrintCoinsString(coin_amount) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn HideCoinsWindow() {
    let window_id = unsafe { (&raw const COINS_WINDOW_ID).read() };
    unsafe { ClearStdWindowAndFrame(window_id, 1) };
    unsafe { RemoveWindow(window_id) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCoins() -> u16 {
    decode_coins(unsafe { coins_ptr().read() }, unsafe { encryption_key() })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCoins(coin_amount: u16) {
    let stored = encode_coins(coin_amount, unsafe { encryption_key() });
    unsafe { coins_ptr().write(stored) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddCoins(to_add: u16) -> u8 {
    let owned = unsafe { GetCoins() };
    if owned >= MAX_COINS {
        return 0;
    }
    let amount = (u32::from(owned) + u32::from(to_add)).min(u32::from(MAX_COINS)) as u16;
    unsafe { SetCoins(amount) };
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveCoins(to_subtract: u16) -> u8 {
    let owned = unsafe { GetCoins() };
    if owned >= to_subtract {
        unsafe { SetCoins(owned - to_subtract) };
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_offsets_match_the_c_structures() {
        assert_eq!(SAVE1_COINS_OFFSET, 0x494);
        assert_eq!(SAVE2_ENCRYPTION_KEY_OFFSET, 0xac);
    }

    #[test]
    fn coin_encryption_matches_the_c_xor_conversion() {
        let key = 0xdead_beef;
        let stored = encode_coins(9_999, key);
        assert_eq!(stored, 9_999 ^ 0xbeef);
        assert_eq!(decode_coins(stored, key), 9_999);
    }
}
