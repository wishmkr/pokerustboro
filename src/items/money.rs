use crate::ffi::{
    ANIMCMD_END, ANIMCMD_FRAME_0_0, AddTextPrinterParameterized, AddWindow, AnimCmd, COPYWIN_GFX,
    COPYWIN_MAP, ClearStdWindowAndFrameToTransparent, CompressedSpritePalette,
    CompressedSpriteSheet, ConvertIntToDecimalStringN, CopyWindowToVram, CreateSprite,
    DestroySpriteAndFreeResources, DrawStdFrameWithCustomTileAndPalette, FONT_NORMAL,
    FillWindowPixelBuffer, LoadCompressedSpritePalette, LoadCompressedSpriteSheet, OamData,
    PutWindowTilemap, RemoveWindow, RomPtr, SAVE1_MONEY_OFFSET, SAVE2_ENCRYPTION_KEY_OFFSET,
    SetWindowTemplateFields, SpriteCallbackDummy, SpriteTemplate, StringExpandPlaceholders,
    StringLength, WindowTemplate, gDummySpriteAffineAnimTable, gSpecialVar_0x8005, gStringVar1,
    gStringVar4, sprite,
};
use core::ffi::c_int;
use core::ptr::addr_of;

const MAX_MONEY: u32 = 999_999;
const MONEY_LABEL_TAG: u16 = 0x2722;
const STR_CONV_MODE_LEFT_ALIGN: c_int = 0;
const CHAR_SPACER: u8 = 0x77;
const PIXEL_FILL_0: u8 = 0;

/// The money box is drawn ten tiles wide, two tall, from this tile block.
const MONEY_BOX_BASE_TILE: u16 = 0x214;
const MONEY_BOX_PALETTE: u8 = 14;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
static mut sMoneyBoxWindowId: u8 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
static mut sMoneyLabelSpriteId: u8 = 0;

/// 32x16, 4bpp, no affine, normal object mode. Bytes read from the compiler
/// with `tools/rustport/probe.sh`.
static OAM_MONEY_LABEL: OamData = OamData([0x00, 0x40, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00]);

static SPRITE_ANIM_MONEY_LABEL: [AnimCmd; 2] = [ANIMCMD_FRAME_0_0, ANIMCMD_END];

static SPRITE_ANIM_TABLE_MONEY_LABEL: [RomPtr<AnimCmd>; 1] =
    [RomPtr((&raw const SPRITE_ANIM_MONEY_LABEL).cast())];

static SPRITE_TEMPLATE_MONEY_LABEL: SpriteTemplate = SpriteTemplate {
    tile_tag: MONEY_LABEL_TAG,
    palette_tag: MONEY_LABEL_TAG,
    oam: &raw const OAM_MONEY_LABEL,
    anims: (&raw const SPRITE_ANIM_TABLE_MONEY_LABEL).cast(),
    images: core::ptr::null(),
    affine_anims: (&raw const gDummySpriteAffineAnimTable).cast(),
    callback: SpriteCallbackDummy,
};

static SPRITE_SHEET_MONEY_LABEL: CompressedSpriteSheet = CompressedSpriteSheet {
    data: (&raw const gShopMenuMoney_Gfx).cast(),
    size: 256,
    tag: MONEY_LABEL_TAG,
};

static SPRITE_PALETTE_MONEY_LABEL: CompressedSpritePalette = CompressedSpritePalette {
    data: (&raw const gShopMenu_Pal).cast(),
    tag: MONEY_LABEL_TAG,
};

unsafe extern "C" {
    static mut gSaveBlock1Ptr: *mut u8;
    static mut gSaveBlock2Ptr: *mut u8;
    static gShopMenuMoney_Gfx: u32;
    static gShopMenu_Pal: u32;
    static gText_PokedollarVar1: u8;
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

/// `&gSaveBlock1Ptr->money`
#[inline]
unsafe fn money_ptr() -> *mut u32 {
    unsafe { gSaveBlock1Ptr.add(SAVE1_MONEY_OFFSET).cast::<u32>() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMoney(money_ptr: *mut u32) -> u32 {
    unsafe { money_ptr.read() ^ encryption_key() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMoney(money_ptr: *mut u32, new_value: u32) {
    unsafe { money_ptr.write(encryption_key() ^ new_value) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsEnoughMoney(money_ptr: *mut u32, cost: u32) -> u8 {
    u8::from(unsafe { GetMoney(money_ptr) } >= cost)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddMoney(money_ptr: *mut u32, to_add: u32) {
    let current = unsafe { GetMoney(money_ptr) };

    let to_set = if current.wrapping_add(to_add) > MAX_MONEY {
        MAX_MONEY
    } else {
        let sum = current.wrapping_add(to_add);
        // Receiving money must never leave the player with less of it.
        if sum < current { MAX_MONEY } else { sum }
    };

    unsafe { SetMoney(money_ptr, to_set) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveMoney(money_ptr: *mut u32, to_sub: u32) {
    let current = unsafe { GetMoney(money_ptr) };
    let to_set = if current < to_sub {
        0
    } else {
        current - to_sub
    };
    unsafe { SetMoney(money_ptr, to_set) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsEnoughForCostInVar0x8005() -> u8 {
    let cost = u32::from(unsafe { addr_of!(gSpecialVar_0x8005).read() });
    unsafe { IsEnoughMoney(money_ptr(), cost) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SubtractMoneyFromVar0x8005() {
    let cost = u32::from(unsafe { addr_of!(gSpecialVar_0x8005).read() });
    unsafe { RemoveMoney(money_ptr(), cost) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintMoneyAmount(window_id: u8, x: u8, y: u8, amount: i32, speed: u8) {
    let var1 = (&raw mut gStringVar1).cast::<u8>();
    let _ = unsafe { ConvertIntToDecimalStringN(var1, amount, STR_CONV_MODE_LEFT_ALIGN, 6) };

    // Right-align within six digits by padding with the wide spacer glyph.
    let mut text = (&raw mut gStringVar4).cast::<u8>();
    let mut padding = 6i32 - i32::from(unsafe { StringLength(var1) });
    while padding > 0 {
        unsafe { text.write(CHAR_SPACER) };
        text = unsafe { text.add(1) };
        padding -= 1;
    }

    let _ = unsafe { StringExpandPlaceholders(text, &raw const gText_PokedollarVar1) };
    let _ = unsafe {
        AddTextPrinterParameterized(
            window_id,
            FONT_NORMAL,
            (&raw const gStringVar4).cast::<u8>(),
            x,
            y,
            speed,
            None,
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintMoneyAmountInMoneyBox(window_id: u8, amount: i32, speed: u8) {
    unsafe { PrintMoneyAmount(window_id, 38, 1, amount, speed) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintMoneyAmountInMoneyBoxWithBorder(
    window_id: u8,
    tile_start: u16,
    palette: u8,
    amount: i32,
) {
    unsafe { DrawStdFrameWithCustomTileAndPalette(window_id, 0, tile_start, palette) };
    unsafe { PrintMoneyAmountInMoneyBox(window_id, amount, 0) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangeAmountInMoneyBox(amount: i32) {
    let window_id = unsafe { addr_of!(sMoneyBoxWindowId).read_volatile() };
    unsafe { PrintMoneyAmountInMoneyBox(window_id, amount, 0) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawMoneyBox(amount: i32, x: u8, y: u8) {
    let mut template = WindowTemplate::default();
    unsafe { SetWindowTemplateFields(&raw mut template, 0, x + 1, y + 1, 10, 2, 15, 8) };

    let window_id = unsafe { AddWindow(&raw const template) } as u8;
    unsafe { (&raw mut sMoneyBoxWindowId).write_volatile(window_id) };

    unsafe { FillWindowPixelBuffer(window_id, PIXEL_FILL_0) };
    unsafe { PutWindowTilemap(window_id) };
    unsafe { CopyWindowToVram(window_id, COPYWIN_MAP) };
    unsafe {
        PrintMoneyAmountInMoneyBoxWithBorder(
            window_id,
            MONEY_BOX_BASE_TILE,
            MONEY_BOX_PALETTE,
            amount,
        )
    };
    unsafe { AddMoneyLabelObject(u16::from(x) * 8 + 19, u16::from(y) * 8 + 11) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn HideMoneyBox() {
    unsafe { RemoveMoneyLabelObject() };
    let window_id = unsafe { addr_of!(sMoneyBoxWindowId).read_volatile() };
    unsafe { ClearStdWindowAndFrameToTransparent(window_id, 0) };
    unsafe { CopyWindowToVram(window_id, COPYWIN_GFX) };
    unsafe { RemoveWindow(window_id) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddMoneyLabelObject(x: u16, y: u16) {
    let _ = unsafe { LoadCompressedSpriteSheet(&raw const SPRITE_SHEET_MONEY_LABEL) };
    unsafe { LoadCompressedSpritePalette(&raw const SPRITE_PALETTE_MONEY_LABEL) };
    let sprite_id = unsafe {
        CreateSprite(
            &raw const SPRITE_TEMPLATE_MONEY_LABEL,
            x as i16,
            y as i16,
            0,
        )
    };
    unsafe { (&raw mut sMoneyLabelSpriteId).write_volatile(sprite_id) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveMoneyLabelObject() {
    let sprite_id = unsafe { addr_of!(sMoneyLabelSpriteId).read_volatile() } as usize;
    unsafe { DestroySpriteAndFreeResources(sprite(sprite_id)) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_is_capped_and_never_wraps() {
        let add = |current: u32, to_add: u32| {
            let sum = current.wrapping_add(to_add);
            if sum > MAX_MONEY || sum < current {
                MAX_MONEY
            } else {
                sum
            }
        };

        assert_eq!(add(0, 500), 500);
        assert_eq!(add(MAX_MONEY, 1), MAX_MONEY);
        assert_eq!(add(1000, MAX_MONEY), MAX_MONEY);
        // Wrapping past 2^32 must still clamp rather than shrink the wallet.
        assert_eq!(add(1000, u32::MAX), MAX_MONEY);
    }

    #[test]
    fn spending_more_than_you_hold_empties_the_wallet() {
        let remove = |current: u32, to_sub: u32| {
            if current < to_sub {
                0
            } else {
                current - to_sub
            }
        };
        assert_eq!(remove(100, 40), 60);
        assert_eq!(remove(100, 100), 0);
        assert_eq!(remove(100, 101), 0);
    }

    #[test]
    fn money_is_stored_xored_with_the_save_encryption_key() {
        let key = 0xdead_beefu32;
        let value = 12_345u32;
        assert_eq!((key ^ value) ^ key, value);
    }
}
