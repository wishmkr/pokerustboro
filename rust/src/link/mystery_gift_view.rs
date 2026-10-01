//! Translated from `src/mystery_gift_view.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs,
    overflowing_literals,
    clippy::too_many_arguments,
    clippy::unnecessary_cast,
    dead_code,
    unused_assignments
)]

use crate::bg::{
    ChangeBgY, CopyBgTilemapBufferToVram, FillBgTilemapBufferRect_Palette0, HideBg, ShowBg,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::{ClearGpuRegBits, SetGpuReg, SetGpuRegBits};
use crate::list_menu::{AddScrollIndicatorArrowPair, RemoveScrollIndicatorArrowPair};
use crate::menu::{
    AddTextPrinterParameterized3, DecompressAndCopyTileDataToVram,
    FreeTempTileDataBuffersIfPossible,
};
use crate::mystery_gift_menu::{
    MG_DrawCheckerboardPattern, PrintMysteryGiftOrEReaderHeader, gGiftIsFromEReader,
};
use crate::palette::{BeginNormalPaletteFade, LoadPalette, UpdatePaletteFade, gPaletteFade};
use crate::pokemon_icon::{
    CreateMonIconNoPersonality, FreeAndDestroyMonIconSprite, FreeMonIconPalettes,
    GetIconSpeciesNoPersonality, LoadMonIconPalettes,
};
use crate::sprite::gSprites;
use crate::sprite::{FreeSpritePaletteByTag, FreeSpriteTilesByTag};
use crate::text::GetFontAttribute;
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{CopyWindowToVram, FillWindowPixelBuffer, PutWindowTilemap, RemoveWindow};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AddWindow` with this module's view of its types.
#[inline]
unsafe fn AddWindow(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::AddWindow(a0 as _) }
}
/// `ConvertIntToDecimalStringN` with this module's view of its types.
#[inline]
unsafe fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8 {
    unsafe { crate::string_util::ConvertIntToDecimalStringN(a0 as _, a1, a2, a3) as *mut u8 }
}
/// `CopyRectToBgTilemapBufferRect` with this module's view of its types.
#[inline]
unsafe fn CopyRectToBgTilemapBufferRect(
    a0: u8,
    a1: *mut c_void,
    a2: u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: u8,
    a7: u8,
    a8: u8,
    a9: u8,
    a10: u8,
    a11: i16,
    a12: i16,
) {
    unsafe {
        crate::bg::CopyRectToBgTilemapBufferRect(
            a0, a1 as _, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12,
        );
    }
}
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `GetStringWidth` with this module's view of its types.
#[inline]
unsafe fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32 {
    unsafe { crate::text::GetStringWidth(a0, a1 as _, a2) }
}
/// `LoadCompressedSpriteSheetUsingHeap` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheetUsingHeap(a0: *mut CompressedSpriteSheet) -> u8 {
    unsafe { crate::decompress::LoadCompressedSpriteSheetUsingHeap(a0 as _) }
}
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
// Data tables (translate with cdata.py): sCard_TextColorTable sCard_FooterTextOffsets sCard_WindowTemplates sWonderCardBgPal1 sWonderCardBgPal2 sWonderCardBgPal3 sWonderCardBgPal4 sWonderCardBgPal5 sWonderCardBgPal6 sWonderCardBgPal7 sWonderCardBgPal8 sWonderCardBgGfx1 sWonderCardBgTilemap1 sWonderCardBgGfx2 sWonderCardBgTilemap2 sWonderCardBgGfx3 sWonderCardBgTilemap3 sWonderCardBgGfx7 sWonderCardBgTilemap7 sWonderCardBgGfx8 sWonderCardBgTilemap8 sStampShadowPal1 sStampShadowPal2 sStampShadowPal3 sStampShadowPal4 sStampShadowPal5 sStampShadowPal6 sStampShadowPal7 sStampShadowPal8 sStampShadowGfx sSpriteSheet_StampShadow sSpritePalettes_StampShadow sSpriteTemplate_StampShadow sCardGraphics sNews_TextColorTable sNews_WindowTemplates sNews_ArrowsTemplate sWonderNewsPal1 sWonderNewsPal7 sWonderNewsPal8 sWonderNewsGfx1 sWonderNewsTilemap1 sWonderNewsGfx2 sWonderNewsTilemap2 sWonderNewsGfx3 sWonderNewsTilemap3 sWonderNewsGfx7 sWonderNewsTilemap7 sWonderNewsGfx8 sWonderNewsTilemap8 sNewsGraphics

/// `struct WonderCardData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WonderCardData {
    pub card: WonderCard,
    pub cardMetadata: WonderCardMetadata,
    pub gfx: *mut WonderGraphics,
    pub enterExitState: u8,
    pub statFooterWidth: u8,
    pub windowIds: CArray<u16, 3>,
    pub monIconSpriteId: u8,
    pub stampSpriteIds: CArray<CArray<u8, 2>, 7>,
    pub titleText: CArray<u8, 41>,
    pub subtitleText: CArray<u8, 41>,
    pub idNumberText: CArray<u8, 7>,
    pub bodyText: CArray<CArray<u8, 41>, 4>,
    pub footerLine1Text: CArray<u8, 41>,
    pub giftText: CArray<u8, 41>,
    pub statTextData: CArray<CardStatTextData, 8>,
    pub bgTilemapBuffer: CArray<u8, 4096>,
}

unsafe impl Sync for WonderCardData {}

/// `struct WonderNewsData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WonderNewsData {
    pub news: WonderNews,
    pub gfx: *mut WonderGraphics,
    bits_448: u8,
    pub arrowTaskId: u8,
    bits_450: u8,
    bits_451: u8,
    pub scrollEnd: u16,
    pub scrollOffset: u16,
    pub windowIds: CArray<u16, 2>,
    pub unused: CArray<u8, 2>,
    pub titleText: CArray<u8, 41>,
    pub bodyText: CArray<CArray<u8, 41>, 10>,
    pub arrowsTemplate: ScrollArrowsTemplate,
    pub bgTilemapBuffer: CArray<u8, 4096>,
}

impl WonderNewsData {
    #[inline(always)]
    pub fn arrowsRemoved(&self) -> u8 {
        ((self.bits_448 as u32) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_arrowsRemoved(&mut self, v: u8) {
        self.bits_448 = (self.bits_448 & !(0x1 << 0)) | (v & 0x1);
    }
    #[inline(always)]
    pub fn enterExitState(&self) -> u8 {
        ((self.bits_448 as u32 >> 1) & 0x7f) as u8
    }
    #[inline(always)]
    pub fn set_enterExitState(&mut self, v: u8) {
        self.bits_448 = (self.bits_448 & !(0x7f << 1)) | ((v & 0x7f) << 1);
    }
    #[inline(always)]
    pub fn scrolling(&self) -> u8 {
        ((self.bits_450 as u32) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_scrolling(&mut self, v: u8) {
        self.bits_450 = (self.bits_450 & !(0x1 << 0)) | (v & 0x1);
    }
    #[inline(always)]
    pub fn scrollIncrement(&self) -> u8 {
        ((self.bits_450 as u32 >> 1) & 0x7f) as u8
    }
    #[inline(always)]
    pub fn set_scrollIncrement(&mut self, v: u8) {
        self.bits_450 = (self.bits_450 & !(0x7f << 1)) | ((v & 0x7f) << 1);
    }
    #[inline(always)]
    pub fn scrollingDown(&self) -> u8 {
        ((self.bits_451 as u32) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_scrollingDown(&mut self, v: u8) {
        self.bits_451 = (self.bits_451 & !(0x1 << 0)) | (v & 0x1);
    }
    #[inline(always)]
    pub fn scrollTotal(&self) -> u8 {
        ((self.bits_451 as u32 >> 1) & 0x7f) as u8
    }
    #[inline(always)]
    pub fn set_scrollTotal(&mut self, v: u8) {
        self.bits_451 = (self.bits_451 & !(0x7f << 1)) | ((v & 0x7f) << 1);
    }
}

unsafe impl Sync for WonderNewsData {}

/// `struct WonderGraphics`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WonderGraphics {
    bits_0: u8,
    bits_1: u8,
    pub tiles: *mut u32,
    pub map: *mut u32,
    pub pal: *mut u16,
}

impl WonderGraphics {
    #[inline(always)]
    pub fn titleTextPal(&self) -> u8 {
        ((self.bits_0 as u32) & 0xf) as u8
    }
    #[inline(always)]
    pub fn set_titleTextPal(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !0xf) | (v & 0xf);
    }
    #[inline(always)]
    pub fn bodyTextPal(&self) -> u8 {
        ((self.bits_0 as u32 >> 4) & 0xf) as u8
    }
    #[inline(always)]
    pub fn set_bodyTextPal(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !(0xf << 4)) | ((v & 0xf) << 4);
    }
    #[inline(always)]
    pub fn footerTextPal(&self) -> u8 {
        ((self.bits_1 as u32) & 0xf) as u8
    }
    #[inline(always)]
    pub fn set_footerTextPal(&mut self, v: u8) {
        self.bits_1 = (self.bits_1 & !0xf) | (v & 0xf);
    }
    #[inline(always)]
    pub fn stampShadowPal(&self) -> u8 {
        ((self.bits_1 as u32 >> 4) & 0xf) as u8
    }
    #[inline(always)]
    pub fn set_stampShadowPal(&mut self, v: u8) {
        self.bits_1 = (self.bits_1 & !(0xf << 4)) | ((v & 0xf) << 4);
    }
}

unsafe impl Sync for WonderGraphics {}

/// `struct CardStatTextData`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct CardStatTextData {
    pub width: u8,
    pub statText: CArray<u8, 41>,
    pub statNumberText: CArray<u8, 4>,
}

unsafe impl Sync for CardStatTextData {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<WonderCardData>() == 5212);
    assert!(offset_of!(WonderCardData, card) == 0);
    assert!(offset_of!(WonderCardData, cardMetadata) == 332);
    assert!(offset_of!(WonderCardData, gfx) == 368);
    assert!(offset_of!(WonderCardData, enterExitState) == 372);
    assert!(offset_of!(WonderCardData, statFooterWidth) == 373);
    assert!(offset_of!(WonderCardData, windowIds) == 374);
    assert!(offset_of!(WonderCardData, monIconSpriteId) == 380);
    assert!(offset_of!(WonderCardData, stampSpriteIds) == 381);
    assert!(offset_of!(WonderCardData, titleText) == 395);
    assert!(offset_of!(WonderCardData, subtitleText) == 436);
    assert!(offset_of!(WonderCardData, idNumberText) == 477);
    assert!(offset_of!(WonderCardData, bodyText) == 484);
    assert!(offset_of!(WonderCardData, footerLine1Text) == 648);
    assert!(offset_of!(WonderCardData, giftText) == 689);
    assert!(offset_of!(WonderCardData, statTextData) == 732);
    assert!(offset_of!(WonderCardData, bgTilemapBuffer) == 1116);
    assert!(size_of::<WonderNewsData>() == 5028);
    assert!(offset_of!(WonderNewsData, news) == 0);
    assert!(offset_of!(WonderNewsData, gfx) == 444);
    assert!(offset_of!(WonderNewsData, bits_448) == 448);
    assert!(offset_of!(WonderNewsData, arrowTaskId) == 449);
    assert!(offset_of!(WonderNewsData, bits_450) == 450);
    assert!(offset_of!(WonderNewsData, bits_451) == 451);
    assert!(offset_of!(WonderNewsData, scrollEnd) == 452);
    assert!(offset_of!(WonderNewsData, scrollOffset) == 454);
    assert!(offset_of!(WonderNewsData, windowIds) == 456);
    assert!(offset_of!(WonderNewsData, unused) == 460);
    assert!(offset_of!(WonderNewsData, titleText) == 462);
    assert!(offset_of!(WonderNewsData, bodyText) == 503);
    assert!(offset_of!(WonderNewsData, arrowsTemplate) == 916);
    assert!(offset_of!(WonderNewsData, bgTilemapBuffer) == 932);
    assert!(size_of::<WonderGraphics>() == 16);
    assert!(offset_of!(WonderGraphics, bits_0) == 0);
    assert!(offset_of!(WonderGraphics, bits_1) == 1);
    assert!(offset_of!(WonderGraphics, tiles) == 4);
    assert!(offset_of!(WonderGraphics, map) == 8);
    assert!(offset_of!(WonderGraphics, pal) == 12);
    assert!(size_of::<CardStatTextData>() == 48);
    assert!(offset_of!(CardStatTextData, width) == 0);
    assert!(offset_of!(CardStatTextData, statText) == 1);
    assert!(offset_of!(CardStatTextData, statNumberText) == 42);
};

const CARD_WIN_BODY: u8 = 1;
const CARD_WIN_FOOTER: u8 = 2;
const CARD_WIN_HEADER: u8 = 0;
const NEWS_WIN_BODY: i32 = 1;
const NEWS_WIN_TITLE: i32 = 0;
const TAG_STAMP_SHADOW: u16 = 32768;

static sCardGraphics: Table<CArray<WonderGraphics, 8>> =
    Table((&raw const crate::data::mystery_gift_view::sCardGraphics).cast());
static sCard_FooterTextOffsets: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::mystery_gift_view::sCard_FooterTextOffsets).cast());
static sCard_TextColorTable: Table<CArray<CArray<u8, 3>, 2>> =
    Table((&raw const crate::data::mystery_gift_view::sCard_TextColorTable).cast());
static sCard_WindowTemplates: Table<CArray<WindowTemplate, 3>> =
    Table((&raw const crate::data::mystery_gift_view::sCard_WindowTemplates).cast());
static sNewsGraphics: Table<CArray<WonderGraphics, 8>> =
    Table((&raw const crate::data::mystery_gift_view::sNewsGraphics).cast());
static sNews_ArrowsTemplate: Table<ScrollArrowsTemplate> =
    Table((&raw const crate::data::mystery_gift_view::sNews_ArrowsTemplate).cast());
static sNews_TextColorTable: Table<CArray<CArray<u8, 3>, 2>> =
    Table((&raw const crate::data::mystery_gift_view::sNews_TextColorTable).cast());
static sNews_WindowTemplates: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::mystery_gift_view::sNews_WindowTemplates).cast());
static sSpritePalettes_StampShadow: Table<CArray<SpritePalette, 8>> =
    Table((&raw const crate::data::mystery_gift_view::sSpritePalettes_StampShadow).cast());
static sSpriteSheet_StampShadow: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::mystery_gift_view::sSpriteSheet_StampShadow).cast());
static sSpriteTemplate_StampShadow: Table<SpriteTemplate> =
    Table((&raw const crate::data::mystery_gift_view::sSpriteTemplate_StampShadow).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sWonderCardData: *mut WonderCardData = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sWonderNewsData: *mut WonderNewsData = null_mut();

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `GetTextWindowPalette` with this module's view of its types.
#[inline]
unsafe fn GetTextWindowPalette(a0: u8) -> *mut u16 {
    unsafe { crate::text_window::GetTextWindowPalette(a0) as *mut u16 }
}
/// `LZ77UnCompWram` with this module's view of its types.
#[inline]
unsafe fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompWram(a0 as _, a1 as _);
    }
}

pub unsafe fn WonderCard_Init(card: *mut WonderCard, metadata: *mut WonderCardMetadata) -> u32 {
    if card.is_null() || metadata.is_null() {
        return FALSE as u32;
    }
    sWonderCardData = AllocZeroed(5212) as *mut WonderCardData;
    if sWonderCardData.is_null() {
        return FALSE as u32;
    }
    (*sWonderCardData).card = *card;
    (*sWonderCardData).cardMetadata = *metadata;
    if (*sWonderCardData).card.bgType() >= NUM_WONDER_BGS {
        (*sWonderCardData).card.set_bgType(0);
    }
    if (*sWonderCardData).card.r#type() >= CARD_TYPE_COUNT {
        (*sWonderCardData).card.set_type(0);
    }
    if (*sWonderCardData).card.maxStamps > MAX_STAMP_CARD_STAMPS {
        (*sWonderCardData).card.maxStamps = 0;
    }
    (*sWonderCardData).gfx =
        (&raw const sCardGraphics[(*sWonderCardData).card.bgType()]).cast_mut();
    TRUE as u32
}
pub unsafe fn WonderCard_Destroy() {
    if !sWonderCardData.is_null() {
        *sWonderCardData = {
            let lit1: WonderCardData = zeroed();
            lit1
        };
        Free(sWonderCardData as *mut c_void);
        sWonderCardData = null_mut();
    }
}
pub unsafe fn WonderCard_Enter() -> i32 {
    if sWonderCardData.is_null() {
        return -1;
    }
    match (*sWonderCardData).enterExitState {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        }
        1 => {
            if UpdatePaletteFade() != 0 {
                return 0;
            }
        }
        2 => {
            FillBgTilemapBufferRect_Palette0(
                0,
                0x000,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
            );
            FillBgTilemapBufferRect_Palette0(
                1,
                0x000,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
            );
            FillBgTilemapBufferRect_Palette0(
                2,
                0x000,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
            );
            CopyBgTilemapBufferToVram(0);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
            DecompressAndCopyTileDataToVram(
                2,
                (*(*sWonderCardData).gfx).tiles as *mut c_void,
                0,
                0x008,
                0,
            );
            (*sWonderCardData).windowIds[0] =
                AddWindow((&raw const sCard_WindowTemplates[0]).cast_mut());
            (*sWonderCardData).windowIds[1] =
                AddWindow((&raw const sCard_WindowTemplates[1]).cast_mut());
            (*sWonderCardData).windowIds[2] =
                AddWindow((&raw const sCard_WindowTemplates[2]).cast_mut());
        }
        3 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return 0;
            }
            LoadPalette(GetTextWindowPalette(1) as *mut c_void, 32, 32);
            gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
            LoadPalette((*(*sWonderCardData).gfx).pal as *mut c_void, 16, 32);
            LZ77UnCompWram(
                (*(*sWonderCardData).gfx).map,
                (*sWonderCardData).bgTilemapBuffer.as_mut_ptr() as *mut c_void,
            );
            CopyRectToBgTilemapBufferRect(
                2,
                (*sWonderCardData).bgTilemapBuffer.as_mut_ptr() as *mut c_void,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
                1,
                0x008,
                0,
            );
            CopyBgTilemapBufferToVram(2);
        }
        4 => {
            BufferCardText();
        }
        5 => {
            DrawCardWindow(CARD_WIN_HEADER);
            DrawCardWindow(CARD_WIN_BODY);
            DrawCardWindow(CARD_WIN_FOOTER);
            CopyBgTilemapBufferToVram(1);
        }
        6 => {
            LoadMonIconPalettes();
        }
        7 => {
            ShowBg(1);
            ShowBg(2);
            gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
            CreateCardSprites();
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            UpdatePaletteFade();
        }
        _ => {
            if UpdatePaletteFade() != 0 {
                return 0;
            }
            (*sWonderCardData).enterExitState = 0;
            return 1;
        }
    }
    (*sWonderCardData).enterExitState += 1;
    0
}
pub unsafe fn WonderCard_Exit(useCancel: u32) -> i32 {
    if sWonderCardData.is_null() {
        return -1;
    }
    match (*sWonderCardData).enterExitState {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        }
        1 => {
            if UpdatePaletteFade() != 0 {
                return 0;
            }
        }
        2 => {
            FillBgTilemapBufferRect_Palette0(
                0,
                0x000,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
            );
            FillBgTilemapBufferRect_Palette0(
                1,
                0x000,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
            );
            FillBgTilemapBufferRect_Palette0(
                2,
                0x000,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
            );
            CopyBgTilemapBufferToVram(0);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
        }
        3 => {
            HideBg(1);
            HideBg(2);
            RemoveWindow((*sWonderCardData).windowIds[2] as u8);
            RemoveWindow((*sWonderCardData).windowIds[1] as u8);
            RemoveWindow((*sWonderCardData).windowIds[0] as u8);
        }
        4 => {
            DestroyCardSprites();
            FreeMonIconPalettes();
        }
        5 => {
            PrintMysteryGiftOrEReaderHeader(gGiftIsFromEReader, useCancel);
            CopyBgTilemapBufferToVram(0);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
        }
        _ => {
            if UpdatePaletteFade() != 0 {
                return 0;
            }
            (*sWonderCardData).enterExitState = 0;
            return 1;
        }
    }
    (*sWonderCardData).enterExitState += 1;
    0
}
unsafe fn BufferCardText() {
    let mut charsUntilStat: u16 = 0;
    let mut stats: CArray<u16, 3> = CArray([0, 0, 0]);
    memcpy(
        (*sWonderCardData).titleText.as_mut_ptr(),
        (*sWonderCardData).card.titleText.as_mut_ptr(),
        WONDER_CARD_TEXT_LENGTH,
    );
    (*sWonderCardData).titleText[40] = EOS;
    memcpy(
        (*sWonderCardData).subtitleText.as_mut_ptr(),
        (*sWonderCardData).card.subtitleText.as_mut_ptr(),
        WONDER_CARD_TEXT_LENGTH,
    );
    (*sWonderCardData).subtitleText[40] = EOS;
    if (*sWonderCardData).card.idNumber > 0xf423f {
        (*sWonderCardData).card.idNumber = 0xf423f;
    }
    ConvertIntToDecimalStringN(
        (*sWonderCardData).idNumberText.as_mut_ptr(),
        (*sWonderCardData).card.idNumber as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        6,
    );
    let mut i: u16 = 0;
    while i < WONDER_CARD_BODY_TEXT_LINES {
        memcpy(
            (*sWonderCardData).bodyText[i].as_mut_ptr(),
            (*sWonderCardData).card.bodyText[i].as_mut_ptr(),
            WONDER_CARD_TEXT_LENGTH,
        );
        (*sWonderCardData).bodyText[i][40] = EOS;
        i += 1;
    }
    memcpy(
        (*sWonderCardData).footerLine1Text.as_mut_ptr(),
        (*sWonderCardData).card.footerLine1Text.as_mut_ptr(),
        WONDER_CARD_TEXT_LENGTH,
    );
    (*sWonderCardData).footerLine1Text[40] = EOS;
    match (*sWonderCardData).card.r#type() {
        CARD_TYPE_GIFT => {
            memcpy(
                (*sWonderCardData).giftText.as_mut_ptr(),
                (*sWonderCardData).card.footerLine2Text.as_mut_ptr(),
                WONDER_CARD_TEXT_LENGTH,
            );
            (*sWonderCardData).giftText[40] = EOS;
        }
        CARD_TYPE_STAMP => {
            (*sWonderCardData).giftText[0] = EOS;
        }
        CARD_TYPE_LINK_STAT => {
            (*sWonderCardData).giftText[0] = EOS;
            stats[0] = (if (*sWonderCardData).cardMetadata.battlesWon < MAX_WONDER_CARD_STAT {
                (*sWonderCardData).cardMetadata.battlesWon as i32
            } else {
                MAX_WONDER_CARD_STAT as i32
            }) as u16;
            stats[1] = (if (*sWonderCardData).cardMetadata.battlesLost < MAX_WONDER_CARD_STAT {
                (*sWonderCardData).cardMetadata.battlesLost as i32
            } else {
                MAX_WONDER_CARD_STAT as i32
            }) as u16;
            stats[2] = (if (*sWonderCardData).cardMetadata.numTrades < MAX_WONDER_CARD_STAT {
                (*sWonderCardData).cardMetadata.numTrades as i32
            } else {
                MAX_WONDER_CARD_STAT as i32
            }) as u16;
            for i in 0..8u16 {
                memset(
                    (*sWonderCardData).statTextData[i]
                        .statNumberText
                        .as_mut_ptr(),
                    EOS as i32,
                    4,
                );
                memset(
                    (*sWonderCardData).statTextData[i].statText.as_mut_ptr(),
                    EOS as i32,
                    41,
                );
            }
            i = 0;
            charsUntilStat = 0;
            while i < WONDER_CARD_TEXT_LENGTH as u16 {
                if (*sWonderCardData).card.footerLine2Text[i] != CHAR_DYNAMIC {
                    (*sWonderCardData).statTextData[(*sWonderCardData).statFooterWidth].statText
                        [charsUntilStat] = (*sWonderCardData).card.footerLine2Text[i];
                    charsUntilStat += 1;
                } else {
                    let id: u8 = (*sWonderCardData).card.footerLine2Text[i as i32 + 1];
                    if id >= 3 {
                        i += 2;
                    } else {
                        ConvertIntToDecimalStringN(
                            (*sWonderCardData).statTextData[(*sWonderCardData).statFooterWidth]
                                .statNumberText
                                .as_mut_ptr(),
                            stats[id] as i32,
                            STR_CONV_MODE_LEADING_ZEROS,
                            3,
                        );
                        (*sWonderCardData).statTextData[(*sWonderCardData).statFooterWidth].width =
                            (*sWonderCardData).card.footerLine2Text[i as i32 + 2];
                        (*sWonderCardData).statFooterWidth += 1;
                        if (*sWonderCardData).statFooterWidth >= 8 {
                            break;
                        }
                        charsUntilStat = 0;
                        i += 2;
                    }
                }
                i += 1;
            }
        }
        _ => {}
    }
}
unsafe fn DrawCardWindow(whichWindow: u8) {
    let mut i: i8 = 0;
    let windowId: i32 = (*sWonderCardData).windowIds[whichWindow] as i32;
    PutWindowTilemap(windowId as u8);
    FillWindowPixelBuffer(windowId as u8, 0);
    'l1: {
        match whichWindow {
            CARD_WIN_HEADER => {
                AddTextPrinterParameterized3(
                    windowId as u8,
                    FONT_SHORT_COPY_1,
                    0,
                    1,
                    sCard_TextColorTable[(*(*sWonderCardData).gfx).titleTextPal()]
                        .as_ptr()
                        .cast_mut(),
                    0,
                    (*sWonderCardData).titleText.as_mut_ptr(),
                );
                let mut x: i32 = 160
                    - GetStringWidth(
                        FONT_SHORT_COPY_1,
                        (*sWonderCardData).subtitleText.as_mut_ptr(),
                        GetFontAttribute(FONT_SHORT_COPY_1, FONTATTR_LETTER_SPACING) as i16,
                    );
                if x < 0 {
                    x = 0;
                }
                AddTextPrinterParameterized3(
                    windowId as u8,
                    FONT_SHORT_COPY_1,
                    x as u8,
                    17,
                    sCard_TextColorTable[(*(*sWonderCardData).gfx).titleTextPal()]
                        .as_ptr()
                        .cast_mut(),
                    0,
                    (*sWonderCardData).subtitleText.as_mut_ptr(),
                );
                if (*sWonderCardData).card.idNumber != 0 {
                    AddTextPrinterParameterized3(
                        windowId as u8,
                        FONT_NORMAL,
                        166,
                        17,
                        sCard_TextColorTable[(*(*sWonderCardData).gfx).titleTextPal()]
                            .as_ptr()
                            .cast_mut(),
                        0,
                        (*sWonderCardData).idNumberText.as_mut_ptr(),
                    );
                }
                break 'l1;
            }
            CARD_WIN_BODY => {
                while i < WONDER_CARD_BODY_TEXT_LINES as i8 {
                    AddTextPrinterParameterized3(
                        windowId as u8,
                        FONT_SHORT_COPY_1,
                        0,
                        16 * i as u8 + 2,
                        sCard_TextColorTable[(*(*sWonderCardData).gfx).bodyTextPal()]
                            .as_ptr()
                            .cast_mut(),
                        0,
                        (*sWonderCardData).bodyText[i].as_mut_ptr(),
                    );
                    i += 1;
                }
            }
            CARD_WIN_FOOTER => {
                AddTextPrinterParameterized3(
                    windowId as u8,
                    FONT_SHORT_COPY_1,
                    0,
                    sCard_FooterTextOffsets[(*sWonderCardData).card.r#type()],
                    sCard_TextColorTable[(*(*sWonderCardData).gfx).footerTextPal()]
                        .as_ptr()
                        .cast_mut(),
                    0,
                    (*sWonderCardData).footerLine1Text.as_mut_ptr(),
                );
                if (*sWonderCardData).card.r#type() != CARD_TYPE_LINK_STAT {
                    AddTextPrinterParameterized3(
                        windowId as u8,
                        FONT_SHORT_COPY_1,
                        0,
                        16 + sCard_FooterTextOffsets[(*sWonderCardData).card.r#type()],
                        sCard_TextColorTable[(*(*sWonderCardData).gfx).footerTextPal()]
                            .as_ptr()
                            .cast_mut(),
                        0,
                        (*sWonderCardData).giftText.as_mut_ptr(),
                    );
                } else {
                    let mut x: i32 = 0;
                    let y: i32 =
                        sCard_FooterTextOffsets[(*sWonderCardData).card.r#type()] as i32 + 16;
                    let spacing: i32 =
                        GetFontAttribute(FONT_SHORT_COPY_1, FONTATTR_LETTER_SPACING) as i32;
                    while (i as i32) < (*sWonderCardData).statFooterWidth as i32 {
                        AddTextPrinterParameterized3(
                            windowId as u8,
                            FONT_SHORT_COPY_1,
                            x as u8,
                            y as u8,
                            sCard_TextColorTable[(*(*sWonderCardData).gfx).footerTextPal()]
                                .as_ptr()
                                .cast_mut(),
                            0,
                            (*sWonderCardData).statTextData[i].statText.as_mut_ptr(),
                        );
                        if (*sWonderCardData).statTextData[i].statNumberText[0] != EOS {
                            x += GetStringWidth(
                                FONT_SHORT_COPY_1,
                                (*sWonderCardData).statTextData[i].statText.as_mut_ptr(),
                                spacing as i16,
                            );
                            AddTextPrinterParameterized3(
                                windowId as u8,
                                FONT_SHORT_COPY_1,
                                x as u8,
                                y as u8,
                                sCard_TextColorTable[(*(*sWonderCardData).gfx).footerTextPal()]
                                    .as_ptr()
                                    .cast_mut(),
                                0,
                                (*sWonderCardData).statTextData[i]
                                    .statNumberText
                                    .as_mut_ptr(),
                            );
                            x += GetStringWidth(
                                FONT_SHORT_COPY_1,
                                (*sWonderCardData).statTextData[i]
                                    .statNumberText
                                    .as_mut_ptr(),
                                spacing as i16,
                            ) + (*sWonderCardData).statTextData[i].width as i32;
                        }
                        i += 1;
                    }
                }
            }
            _ => {}
        }
    }
    CopyWindowToVram(windowId as u8, COPYWIN_FULL);
}
unsafe fn CreateCardSprites() {
    let mut i: u8 = 0;
    (*sWonderCardData).monIconSpriteId = SPRITE_NONE;
    if (*sWonderCardData).cardMetadata.iconSpecies != SPECIES_NONE {
        (*sWonderCardData).monIconSpriteId = CreateMonIconNoPersonality(
            GetIconSpeciesNoPersonality((*sWonderCardData).cardMetadata.iconSpecies),
            Some(SpriteCallbackDummy),
            220,
            20,
            0,
            0,
        );
        gSprites[(*sWonderCardData).monIconSpriteId]
            .oam
            .set_priority(2);
    }
    if (*sWonderCardData).card.maxStamps != 0 && (*sWonderCardData).card.r#type() == CARD_TYPE_STAMP
    {
        LoadCompressedSpriteSheetUsingHeap((&raw const *sSpriteSheet_StampShadow).cast_mut());
        LoadSpritePalette(
            (&raw const sSpritePalettes_StampShadow[(*(*sWonderCardData).gfx).stampShadowPal()])
                .cast_mut(),
        );
        while i < (*sWonderCardData).card.maxStamps {
            (*sWonderCardData).stampSpriteIds[i][0] = SPRITE_NONE;
            (*sWonderCardData).stampSpriteIds[i][1] = SPRITE_NONE;
            (*sWonderCardData).stampSpriteIds[i][0] = CreateSprite(
                (&raw const *sSpriteTemplate_StampShadow).cast_mut(),
                216 - 32 * i as i16,
                144,
                8,
            );
            if (*sWonderCardData).cardMetadata.stampData[0][i] != 0 {
                (*sWonderCardData).stampSpriteIds[i][1] = CreateMonIconNoPersonality(
                    GetIconSpeciesNoPersonality((*sWonderCardData).cardMetadata.stampData[0][i]),
                    Some(SpriteCallbackDummy),
                    216 - 32 * i as i16,
                    136,
                    0,
                    0,
                );
            }
            i += 1;
        }
    }
}
unsafe fn DestroyCardSprites() {
    let mut i: u8 = 0;
    if (*sWonderCardData).monIconSpriteId != SPRITE_NONE {
        FreeAndDestroyMonIconSprite(&raw mut gSprites[(*sWonderCardData).monIconSpriteId]);
    }
    if (*sWonderCardData).card.maxStamps != 0 && (*sWonderCardData).card.r#type() == CARD_TYPE_STAMP
    {
        while i < (*sWonderCardData).card.maxStamps {
            if (*sWonderCardData).stampSpriteIds[i][0] != SPRITE_NONE {
                DestroySprite(&raw mut gSprites[(*sWonderCardData).stampSpriteIds[i][0]]);
            }
            if (*sWonderCardData).stampSpriteIds[i][1] != SPRITE_NONE {
                FreeAndDestroyMonIconSprite(
                    &raw mut gSprites[(*sWonderCardData).stampSpriteIds[i][1]],
                );
            }
            i += 1;
        }
        FreeSpriteTilesByTag(TAG_STAMP_SHADOW);
        FreeSpritePaletteByTag(TAG_STAMP_SHADOW);
    }
}
pub unsafe fn WonderNews_Init(news: *mut WonderNews) -> u32 {
    if news.is_null() {
        return FALSE as u32;
    }
    sWonderNewsData = AllocZeroed(5028) as *mut WonderNewsData;
    if sWonderNewsData.is_null() {
        return FALSE as u32;
    }
    (*sWonderNewsData).news = *news;
    if (*sWonderNewsData).news.bgType >= NUM_WONDER_BGS {
        (*sWonderNewsData).news.bgType = 0;
    }
    (*sWonderNewsData).gfx = (&raw const sNewsGraphics[(*sWonderNewsData).news.bgType]).cast_mut();
    (*sWonderNewsData).arrowTaskId = TASK_NONE;
    TRUE as u32
}
pub unsafe fn WonderNews_Destroy() {
    if !sWonderNewsData.is_null() {
        *sWonderNewsData = {
            let lit1: WonderNewsData = zeroed();
            lit1
        };
        Free(sWonderNewsData as *mut c_void);
        sWonderNewsData = null_mut();
    }
}
pub unsafe fn WonderNews_Enter() -> i32 {
    if sWonderNewsData.is_null() {
        return -1;
    }
    match (*sWonderNewsData).enterExitState() {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        }
        1 => {
            if UpdatePaletteFade() != 0 {
                return 0;
            }
            ChangeBgY(0, 0, BG_COORD_SET);
            ChangeBgY(1, 0, BG_COORD_SET);
            ChangeBgY(2, 0, BG_COORD_SET);
            ChangeBgY(3, 0, BG_COORD_SET);
            SetGpuReg(REG_OFFSET_WIN0H, DISPLAY_WIDTH);
            SetGpuReg(REG_OFFSET_WIN0V, 6808);
            SetGpuReg(REG_OFFSET_WININ, 31);
            SetGpuReg(REG_OFFSET_WINOUT, 27);
            SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
        }
        2 => {
            FillBgTilemapBufferRect_Palette0(
                0,
                0x000,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
            );
            FillBgTilemapBufferRect_Palette0(
                1,
                0x000,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
            );
            FillBgTilemapBufferRect_Palette0(
                2,
                0x000,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
            );
            FillBgTilemapBufferRect_Palette0(
                3,
                0x000,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
            );
            CopyBgTilemapBufferToVram(0);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
            CopyBgTilemapBufferToVram(3);
            DecompressAndCopyTileDataToVram(
                3,
                (*(*sWonderNewsData).gfx).tiles as *mut c_void,
                0,
                8,
                0,
            );
            (*sWonderNewsData).windowIds[0] =
                AddWindow((&raw const sNews_WindowTemplates[0]).cast_mut());
            (*sWonderNewsData).windowIds[1] =
                AddWindow((&raw const sNews_WindowTemplates[1]).cast_mut());
        }
        3 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return 0;
            }
            LoadPalette(GetTextWindowPalette(1) as *mut c_void, 32, 32);
            gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
            LoadPalette((*(*sWonderNewsData).gfx).pal as *mut c_void, 16, 32);
            LZ77UnCompWram(
                (*(*sWonderNewsData).gfx).map,
                (*sWonderNewsData).bgTilemapBuffer.as_mut_ptr() as *mut c_void,
            );
            CopyRectToBgTilemapBufferRect(
                1,
                (*sWonderNewsData).bgTilemapBuffer.as_mut_ptr() as *mut c_void,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                3,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                3,
                1,
                8,
                0,
            );
            CopyRectToBgTilemapBufferRect(
                3,
                (*sWonderNewsData).bgTilemapBuffer.as_mut_ptr() as *mut c_void,
                0,
                3,
                DISPLAY_TILE_WIDTH,
                23,
                0,
                3,
                DISPLAY_TILE_WIDTH,
                23,
                1,
                8,
                0,
            );
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(3);
        }
        4 => {
            BufferNewsText();
        }
        5 => {
            DrawNewsWindows();
            CopyBgTilemapBufferToVram(0);
            CopyBgTilemapBufferToVram(2);
        }
        6 => {
            ShowBg(1);
            ShowBg(2);
            ShowBg(3);
            gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
            (*sWonderNewsData).arrowTaskId = AddScrollIndicatorArrowPair(
                &raw mut (*sWonderNewsData).arrowsTemplate,
                &raw mut (*sWonderNewsData).scrollOffset,
            );
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            UpdatePaletteFade();
        }
        _ => {
            if UpdatePaletteFade() != 0 {
                return 0;
            }
            (*sWonderNewsData).set_enterExitState(0);
            return 1;
        }
    }
    (*sWonderNewsData).set_enterExitState((*sWonderNewsData).enterExitState() + 1);
    0
}
pub unsafe fn WonderNews_Exit(useCancel: u32) -> i32 {
    if sWonderNewsData.is_null() {
        return -1;
    }
    match (*sWonderNewsData).enterExitState() {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        }
        1 => {
            if UpdatePaletteFade() != 0 {
                return 0;
            }
            ChangeBgY(2, 0, BG_COORD_SET);
            SetGpuReg(REG_OFFSET_WIN0H, 0);
            SetGpuReg(REG_OFFSET_WIN0V, 0);
            SetGpuReg(REG_OFFSET_WININ, 0);
            SetGpuReg(REG_OFFSET_WINOUT, 0);
            ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
        }
        2 => {
            FillBgTilemapBufferRect_Palette0(
                0,
                0x000,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
            );
            FillBgTilemapBufferRect_Palette0(
                1,
                0x000,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
            );
            FillBgTilemapBufferRect_Palette0(2, 0x000, 0, 0, DISPLAY_TILE_WIDTH, 24);
            FillBgTilemapBufferRect_Palette0(3, 0x000, 0, 0, DISPLAY_TILE_WIDTH, 24);
            CopyBgTilemapBufferToVram(0);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
            CopyBgTilemapBufferToVram(3);
        }
        3 => {
            HideBg(1);
            HideBg(2);
            RemoveWindow((*sWonderNewsData).windowIds[1] as u8);
            RemoveWindow((*sWonderNewsData).windowIds[0] as u8);
        }
        4 => {
            ChangeBgY(2, 0, BG_COORD_SET);
            ChangeBgY(3, 0, BG_COORD_SET);
            if (*sWonderNewsData).arrowTaskId != TASK_NONE {
                RemoveScrollIndicatorArrowPair((*sWonderNewsData).arrowTaskId);
                (*sWonderNewsData).arrowTaskId = TASK_NONE;
            }
        }
        5 => {
            PrintMysteryGiftOrEReaderHeader(gGiftIsFromEReader, useCancel);
            MG_DrawCheckerboardPattern(3);
            CopyBgTilemapBufferToVram(0);
            CopyBgTilemapBufferToVram(3);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
        }
        _ => {
            if UpdatePaletteFade() != 0 {
                return 0;
            }
            (*sWonderNewsData).set_enterExitState(0);
            return 1;
        }
    }
    (*sWonderNewsData).set_enterExitState((*sWonderNewsData).enterExitState() + 1);
    0
}
pub unsafe fn WonderNews_RemoveScrollIndicatorArrowPair() {
    if (*sWonderNewsData).arrowsRemoved() == 0 && (*sWonderNewsData).arrowTaskId != TASK_NONE {
        RemoveScrollIndicatorArrowPair((*sWonderNewsData).arrowTaskId);
        (*sWonderNewsData).arrowTaskId = TASK_NONE;
        (*sWonderNewsData).set_arrowsRemoved(TRUE);
    }
}
pub unsafe fn WonderNews_AddScrollIndicatorArrowPair() {
    if (*sWonderNewsData).arrowsRemoved() != 0 {
        (*sWonderNewsData).arrowTaskId = AddScrollIndicatorArrowPair(
            &raw mut (*sWonderNewsData).arrowsTemplate,
            &raw mut (*sWonderNewsData).scrollOffset,
        );
        (*sWonderNewsData).set_arrowsRemoved(FALSE);
    }
}
pub unsafe fn WonderNews_GetInput(input: u16) -> u32 {
    if (*sWonderNewsData).scrolling() != 0 {
        UpdateNewsScroll();
        return NEWS_INPUT_NONE;
    }
    match input {
        1 => {
            return NEWS_INPUT_A;
        }
        2 => {
            return NEWS_INPUT_B;
        }
        64 => {
            if (*sWonderNewsData).scrollOffset == 0 {
                return NEWS_INPUT_NONE;
            }
            if (*sWonderNewsData).arrowsRemoved() != 0 {
                return NEWS_INPUT_NONE;
            }
            (*sWonderNewsData).set_scrollingDown(FALSE);
        }
        128 => {
            if (*sWonderNewsData).scrollOffset == (*sWonderNewsData).scrollEnd {
                return NEWS_INPUT_NONE;
            }
            if (*sWonderNewsData).arrowsRemoved() != 0 {
                return NEWS_INPUT_NONE;
            }
            (*sWonderNewsData).set_scrollingDown(TRUE);
        }
        _ => {
            return NEWS_INPUT_NONE;
        }
    }
    (*sWonderNewsData).set_scrolling(TRUE);
    (*sWonderNewsData).set_scrollIncrement(2);
    (*sWonderNewsData).set_scrollTotal(0);
    if (*sWonderNewsData).scrollingDown() == 0 {
        return NEWS_INPUT_SCROLL_UP;
    } else {
        return NEWS_INPUT_SCROLL_DOWN;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn BufferNewsText() {
    memcpy(
        (*sWonderNewsData).titleText.as_mut_ptr(),
        (*sWonderNewsData).news.titleText.as_mut_ptr(),
        WONDER_NEWS_TEXT_LENGTH,
    );
    (*sWonderNewsData).titleText[40] = EOS;
    for i in 0..WONDER_NEWS_BODY_TEXT_LINES {
        memcpy(
            (*sWonderNewsData).bodyText[i].as_mut_ptr(),
            (*sWonderNewsData).news.bodyText[i].as_mut_ptr(),
            WONDER_NEWS_TEXT_LENGTH,
        );
        (*sWonderNewsData).bodyText[i][40] = EOS;
        if i > 7 && (*sWonderNewsData).bodyText[i][0] != EOS {
            (*sWonderNewsData).scrollEnd += 1;
        }
    }
    (*sWonderNewsData).arrowsTemplate = *sNews_ArrowsTemplate;
    (*sWonderNewsData).arrowsTemplate.fullyDownThreshold = (*sWonderNewsData).scrollEnd;
}
unsafe fn DrawNewsWindows() {
    PutWindowTilemap((*sWonderNewsData).windowIds[0] as u8);
    PutWindowTilemap((*sWonderNewsData).windowIds[1] as u8);
    FillWindowPixelBuffer((*sWonderNewsData).windowIds[0] as u8, 0);
    FillWindowPixelBuffer((*sWonderNewsData).windowIds[1] as u8, 0);
    let mut x: i32 =
        (224 - GetStringWidth(
            FONT_SHORT_COPY_1,
            (*sWonderNewsData).titleText.as_mut_ptr(),
            GetFontAttribute(FONT_SHORT_COPY_1, FONTATTR_LETTER_SPACING) as i16,
        )) / 2;
    if x < 0 {
        x = 0;
    }
    AddTextPrinterParameterized3(
        (*sWonderNewsData).windowIds[0] as u8,
        FONT_SHORT_COPY_1,
        x as u8,
        6,
        sNews_TextColorTable[(*(*sWonderNewsData).gfx).titleTextPal()]
            .as_ptr()
            .cast_mut(),
        0,
        (*sWonderNewsData).titleText.as_mut_ptr(),
    );
    for i in 0..WONDER_NEWS_BODY_TEXT_LINES {
        AddTextPrinterParameterized3(
            (*sWonderNewsData).windowIds[1] as u8,
            FONT_SHORT_COPY_1,
            0,
            16 * i + 2,
            sNews_TextColorTable[(*(*sWonderNewsData).gfx).bodyTextPal()]
                .as_ptr()
                .cast_mut(),
            0,
            (*sWonderNewsData).bodyText[i].as_mut_ptr(),
        );
    }
    CopyWindowToVram((*sWonderNewsData).windowIds[0] as u8, COPYWIN_FULL);
    CopyWindowToVram((*sWonderNewsData).windowIds[1] as u8, COPYWIN_FULL);
}
unsafe fn UpdateNewsScroll() {
    let mut bgMove: u16 = (*sWonderNewsData).scrollIncrement() as u16;
    bgMove *= 256;
    if (*sWonderNewsData).scrollingDown() != 0 {
        ChangeBgY(2, bgMove as i32, BG_COORD_ADD);
        ChangeBgY(3, bgMove as i32, BG_COORD_ADD);
    } else {
        ChangeBgY(2, bgMove as i32, BG_COORD_SUB);
        ChangeBgY(3, bgMove as i32, BG_COORD_SUB);
    }
    (*sWonderNewsData)
        .set_scrollTotal((*sWonderNewsData).scrollTotal() + (*sWonderNewsData).scrollIncrement());
    if (*sWonderNewsData).scrollTotal() > 15 {
        if (*sWonderNewsData).scrollingDown() != 0 {
            (*sWonderNewsData).scrollOffset += 1;
        } else {
            (*sWonderNewsData).scrollOffset -= 1;
        }
        (*sWonderNewsData).set_scrolling(FALSE);
        (*sWonderNewsData).set_scrollTotal(0);
    }
}
