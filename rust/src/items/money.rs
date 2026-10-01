use crate::ffi::{
    ANIMCMD_END, ANIMCMD_FRAME_0_0, AddTextPrinterParameterized, AddWindow, AnimCmd, COPYWIN_GFX,
    COPYWIN_MAP, ClearStdWindowAndFrameToTransparent, CompressedSpritePalette,
    CompressedSpriteSheet, ConvertIntToDecimalStringN, CopyWindowToVram, CreateSprite,
    DestroySpriteAndFreeResources, DrawStdFrameWithCustomTileAndPalette, FONT_NORMAL,
    FillWindowPixelBuffer, LoadCompressedSpritePalette, LoadCompressedSpriteSheet, OamData,
    PutWindowTilemap, RemoveWindow, RomPtr, SetWindowTemplateFields, SpriteCallbackDummy,
    SpriteTemplate, StringExpandPlaceholders, StringLength, WindowTemplate,
    gDummySpriteAffineAnimTable, gSpecialVar_0x8005, gStringVar1, gStringVar4, sprite,
};
use crate::save_blocks::{save_block1, save_block2};
use crate::types::{SaveBlock1, SaveBlock2};
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
    data: (&raw const (*(&raw const crate::data::graphics::gShopMenuMoney_Gfx).cast::<u32>()))
        .cast(),
    size: 256,
    tag: MONEY_LABEL_TAG,
};

static SPRITE_PALETTE_MONEY_LABEL: CompressedSpritePalette = CompressedSpritePalette {
    data: (&raw const (*(&raw const crate::data::graphics::gShopMenu_Pal).cast::<u32>())).cast(),
    tag: MONEY_LABEL_TAG,
};

/// The player's money as the save stores it: XORed with the save's
/// encryption key.
pub struct Money<'a> {
    stored: &'a mut u32,
    key: u32,
}

impl<'a> Money<'a> {
    /// The player's money.
    pub fn player(save1: &'a mut SaveBlock1, save2: &SaveBlock2) -> Self {
        Self::from_parts(&mut save1.money, save2.encryptionKey)
    }

    /// Money stored in `stored`, encrypted with `key`.
    pub fn from_parts(stored: &'a mut u32, key: u32) -> Self {
        Self { stored, key }
    }

    /// How much money there is.
    pub fn get(&self) -> u32 {
        *self.stored ^ self.key
    }

    /// Sets the amount (not capped).
    pub fn set(&mut self, amount: u32) {
        *self.stored = amount ^ self.key;
    }

    /// Whether there is at least `cost`.
    pub fn is_enough(&self, cost: u32) -> bool {
        self.get() >= cost
    }

    /// Adds money, up to [`MAX_MONEY`] (receiving money never leaves less).
    pub fn add(&mut self, amount: u32) {
        let total = self.get().saturating_add(amount).min(MAX_MONEY);
        self.set(total);
    }

    /// Takes money away, down to zero.
    pub fn remove(&mut self, amount: u32) {
        let left = self.get().saturating_sub(amount);
        self.set(left);
    }
}

// ------------------------------------------------------------------ C names

/// The money at `money_ptr`, with the save's key.
///
/// # Safety
/// `money_ptr` must point to encrypted money (in practice a save field) and
/// the save blocks must be set up; nothing else may use them meanwhile.
unsafe fn money_at(money_ptr: *mut u32) -> Money<'static> {
    unsafe { Money::from_parts(&mut *money_ptr, save_block2().encryptionKey) }
}

/// The player's money (see [`money_at`]).
unsafe fn player_money() -> Money<'static> {
    unsafe { Money::player(save_block1(), save_block2()) }
}

#[unsafe(no_mangle)]
pub unsafe fn GetMoney(money_ptr: *mut u32) -> u32 {
    unsafe { money_at(money_ptr) }.get()
}

#[unsafe(no_mangle)]
pub unsafe fn SetMoney(money_ptr: *mut u32, amount: u32) {
    unsafe { money_at(money_ptr) }.set(amount);
}

#[unsafe(no_mangle)]
pub unsafe fn IsEnoughMoney(money_ptr: *mut u32, cost: u32) -> u8 {
    unsafe { money_at(money_ptr) }.is_enough(cost).into()
}

#[unsafe(no_mangle)]
pub unsafe fn AddMoney(money_ptr: *mut u32, amount: u32) {
    unsafe { money_at(money_ptr) }.add(amount);
}

#[unsafe(no_mangle)]
pub unsafe fn RemoveMoney(money_ptr: *mut u32, amount: u32) {
    unsafe { money_at(money_ptr) }.remove(amount);
}

/// Script special: whether the player can pay `VAR_0x8005`.
#[unsafe(no_mangle)]
pub unsafe fn IsEnoughForCostInVar0x8005() -> u8 {
    let cost = u32::from(unsafe { gSpecialVar_0x8005 });
    unsafe { player_money() }.is_enough(cost).into()
}

/// Script special: the player pays `VAR_0x8005`.
#[unsafe(no_mangle)]
pub unsafe fn SubtractMoneyFromVar0x8005() {
    let cost = u32::from(unsafe { gSpecialVar_0x8005 });
    unsafe { player_money() }.remove(cost);
}

#[unsafe(no_mangle)]
pub unsafe fn PrintMoneyAmount(window_id: u8, x: u8, y: u8, amount: i32, speed: u8) {
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

    let _ = unsafe {
        StringExpandPlaceholders(
            text,
            &raw const (*(&raw const crate::data::strings::gText_PokedollarVar1).cast::<u8>()),
        )
    };
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
pub unsafe fn PrintMoneyAmountInMoneyBox(window_id: u8, amount: i32, speed: u8) {
    unsafe { PrintMoneyAmount(window_id, 38, 1, amount, speed) };
}

#[unsafe(no_mangle)]
pub unsafe fn PrintMoneyAmountInMoneyBoxWithBorder(
    window_id: u8,
    tile_start: u16,
    palette: u8,
    amount: i32,
) {
    unsafe { DrawStdFrameWithCustomTileAndPalette(window_id, 0, tile_start, palette) };
    unsafe { PrintMoneyAmountInMoneyBox(window_id, amount, 0) };
}

#[unsafe(no_mangle)]
pub unsafe fn ChangeAmountInMoneyBox(amount: i32) {
    let window_id = unsafe { addr_of!(sMoneyBoxWindowId).read_volatile() };
    unsafe { PrintMoneyAmountInMoneyBox(window_id, amount, 0) };
}

#[unsafe(no_mangle)]
pub unsafe fn DrawMoneyBox(amount: i32, x: u8, y: u8) {
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
pub unsafe fn HideMoneyBox() {
    unsafe { RemoveMoneyLabelObject() };
    let window_id = unsafe { addr_of!(sMoneyBoxWindowId).read_volatile() };
    unsafe { ClearStdWindowAndFrameToTransparent(window_id, 0) };
    unsafe { CopyWindowToVram(window_id, COPYWIN_GFX) };
    unsafe { RemoveWindow(window_id) };
}

#[unsafe(no_mangle)]
pub unsafe fn AddMoneyLabelObject(x: u16, y: u16) {
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
pub unsafe fn RemoveMoneyLabelObject() {
    let sprite_id = unsafe { addr_of!(sMoneyLabelSpriteId).read_volatile() } as usize;
    unsafe { DestroySpriteAndFreeResources(sprite(sprite_id)) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_is_stored_encrypted() {
        let mut stored = 0;
        let mut money = Money::from_parts(&mut stored, 0xdead_beef);
        money.set(12_345);
        assert_eq!(money.get(), 12_345);
        assert_eq!(stored, 12_345 ^ 0xdead_beef);
    }

    #[test]
    fn money_is_capped_and_never_wraps() {
        let mut stored = 0;
        let mut money = Money::from_parts(&mut stored, 0x5555_aaaa);
        money.set(0);
        money.add(500);
        assert_eq!(money.get(), 500);
        money.add(MAX_MONEY);
        assert_eq!(money.get(), MAX_MONEY);
        money.set(1000);
        // past 2^32: still the maximum, not less money
        money.add(u32::MAX);
        assert_eq!(money.get(), MAX_MONEY);
    }

    #[test]
    fn spending_more_than_you_hold_empties_the_wallet() {
        let mut stored = 0;
        let mut money = Money::from_parts(&mut stored, 0);
        money.set(100);
        assert!(money.is_enough(100));
        assert!(!money.is_enough(101));
        money.remove(40);
        assert_eq!(money.get(), 60);
        money.remove(61);
        assert_eq!(money.get(), 0);
    }
}
