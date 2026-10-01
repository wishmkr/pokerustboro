//! Translated from `src/mon_markings.c` by tools/rustport/c2rs.py.
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
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::load_save::gSaveBlock2Ptr;
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{FreeSpritePaletteByTag, FreeSpriteTilesByTag};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CalcCenterToCornerVec` with this module's view of its types.
#[inline]
unsafe fn CalcCenterToCornerVec(a0: *mut Sprite, a1: u8, a2: u8, a3: u8) {
    unsafe {
        crate::sprite::CalcCenterToCornerVec(a0 as _, a1, a2, a3);
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
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
}
/// `LoadSpritePalettes` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalettes(a0: *mut SpritePalette) {
    unsafe {
        crate::sprite::LoadSpritePalettes(a0 as _);
    }
}
/// `LoadSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16 {
    unsafe { crate::sprite::LoadSpriteSheet(a0 as _) }
}
/// `LoadSpriteSheets` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheets(a0: *mut SpriteSheet) {
    unsafe {
        crate::sprite::LoadSpriteSheets(a0 as _);
    }
}
/// `RequestDma3Copy` with this module's view of its types.
#[inline]
unsafe fn RequestDma3Copy(a0: *mut c_void, a1: *mut c_void, a2: u16, a3: u8) -> i16 {
    unsafe { crate::dma3_manager::RequestDma3Copy(a0 as _, a1 as _, a2, a3) }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
// The C's names for task and sprite data slots.
const sCursorYOffset: usize = 0;
const sMarkingId: usize = 0;
// Data tables (translate with cdata.py): sMonMarkings_Pal sMonMarkings_Gfx sOamData_MenuWindow sOamData_8x8 sAnim_Marking_CircleOff sAnim_Marking_CircleOn sAnim_Marking_SquareOff sAnim_Marking_SquareOn sAnim_Marking_TriangleOff sAnim_Marking_TriangleOn sAnim_Marking_HeartOff sAnim_Marking_HeartOn sAnim_Cursor sAnim_OKCancelText sAnims_MenuSprite sAnim_MenuWindow_UpperHalf sAnim_MenuWindow_LowerHalf sAnims_MenuWindow sOamData_MarkingCombo sAnim_MarkingCombo_AllOff sAnim_MarkingCombo_Circle sAnim_MarkingCombo_Square sAnim_MarkingCombo_CircleSquare sAnim_MarkingCombo_Triangle sAnim_MarkingCombo_CircleTriangle sAnim_MarkingCombo_SquareTriangle sAnim_MarkingCombo_CircleSquareTriangle sAnim_MarkingCombo_Heart sAnim_MarkingCombo_CircleHeart sAnim_MarkingCombo_SquareHeart sAnim_MarkingCombo_CircleSquareHeart sAnim_MarkingCombo_TriangleHeart sAnim_MarkingCombo_CircleTriangleHeart sAnim_MarkingCombo_SquareTriangleHeart sAnim_MarkingCombo_AllOn sAnims_MarkingCombo

const ANIM_CURSOR: u8 = 8;
const ANIM_TEXT: u8 = 9;
const SELECTION_CANCEL: i8 = 5;
const SELECTION_OK: i8 = 4;

static sAnims_MarkingCombo: Table<CArray<*mut AnimCmd, 16>> =
    Table((&raw const crate::data::mon_markings::sAnims_MarkingCombo).cast());
static sAnims_MenuSprite: Table<CArray<*mut AnimCmd, 10>> =
    Table((&raw const crate::data::mon_markings::sAnims_MenuSprite).cast());
static sAnims_MenuWindow: Table<CArray<*mut AnimCmd, 2>> =
    Table((&raw const crate::data::mon_markings::sAnims_MenuWindow).cast());
static sMonMarkings_Gfx: Table<CArray<u8, 2048>> =
    Table((&raw const crate::data::mon_markings::sMonMarkings_Gfx).cast());
static sMonMarkings_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::mon_markings::sMonMarkings_Pal).cast());
static sOamData_8x8: Table<OamData> =
    Table((&raw const crate::data::mon_markings::sOamData_8x8).cast());
static sOamData_MarkingCombo: Table<OamData> =
    Table((&raw const crate::data::mon_markings::sOamData_MarkingCombo).cast());
static sOamData_MenuWindow: Table<OamData> =
    Table((&raw const crate::data::mon_markings::sOamData_MenuWindow).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMenu: *mut MonMarkingsMenu = null_mut();

/// `CpuFastSet` with this module's view of its types.
#[inline]
unsafe fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuFastSet(a0 as _, a1 as _, a2);
    }
}
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `GetWindowFrameTilesPal` with this module's view of its types.
#[inline]
unsafe fn GetWindowFrameTilesPal(a0: u8) -> *mut TilesPal {
    unsafe { crate::text_window::GetWindowFrameTilesPal(a0) as *mut TilesPal }
}

pub unsafe fn InitMonMarkingsMenu(ptr: *mut MonMarkingsMenu) {
    sMenu = ptr;
}
unsafe fn BufferMenuWindowTiles() {
    let frame: *mut TilesPal =
        GetWindowFrameTilesPal((*gSaveBlock2Ptr).optionsWindowFrameType() as u8);
    (*sMenu).frameTiles = (*frame).tiles;
    (*sMenu).framePalette = (*frame).pal;
    (*sMenu).tileLoadState = 0;
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (*sMenu).windowSpriteTiles.as_mut_ptr() as *mut c_void,
                0x1000800,
            );
        }
    }
}
unsafe fn BufferMenuFrameTiles() -> u8 {
    let dest: *mut u8 = (*sMenu)
        .windowSpriteTiles
        .as_mut_ptr()
        .at((*sMenu).tileLoadState as i32 * 0x100);
    match (*sMenu).tileLoadState {
        0 => {
            CpuFastSet((*sMenu).frameTiles as *mut c_void, dest as *mut c_void, 8);
            for i in 0..6u16 {
                CpuFastSet(
                    (*sMenu).frameTiles.at(32) as *mut c_void,
                    dest.at(32 * (i as i32 + 1)) as *mut c_void,
                    8,
                );
            }
            CpuFastSet(
                (*sMenu).frameTiles.at(64) as *mut c_void,
                dest.at(224) as *mut c_void,
                8,
            );
            (*sMenu).tileLoadState += 1;
        }
        13 => {
            CpuFastSet(
                (*sMenu).frameTiles.at(192) as *mut c_void,
                dest as *mut c_void,
                8,
            );
            for i in 0..6u16 {
                CpuFastSet(
                    (*sMenu).frameTiles.at(224) as *mut c_void,
                    dest.at(32 * (i as i32 + 1)) as *mut c_void,
                    8,
                );
            }
            CpuFastSet(
                (*sMenu).frameTiles.at(256) as *mut c_void,
                dest.at(224) as *mut c_void,
                8,
            );
            (*sMenu).tileLoadState += 1;
            return FALSE;
        }
        14 => {
            return FALSE;
        }
        _ => {
            CpuFastSet(
                (*sMenu).frameTiles.at(96) as *mut c_void,
                dest as *mut c_void,
                8,
            );
            for i in 0..6u16 {
                CpuFastSet(
                    (*sMenu).frameTiles.at(128) as *mut c_void,
                    dest.at(32 * (i as i32 + 1)) as *mut c_void,
                    8,
                );
            }
            CpuFastSet(
                (*sMenu).frameTiles.at(160) as *mut c_void,
                dest.at(224) as *mut c_void,
                8,
            );
            (*sMenu).tileLoadState += 1;
        }
    }
    TRUE
}
pub unsafe fn BufferMonMarkingsMenuTiles() {
    BufferMenuWindowTiles();
    while BufferMenuFrameTiles() != 0 {}
}
pub unsafe fn OpenMonMarkingsMenu(markings: u8, x: i16, y: i16) {
    (*sMenu).cursorPos = 0;
    (*sMenu).markings = markings;
    for i in 0..NUM_MON_MARKINGS {
        (*sMenu).markingsArray[i] = shr_i32((*sMenu).markings as i32, i as u32) as u8 & 1;
    }
    CreateMonMarkingsMenuSprites(x, y, (*sMenu).baseTileTag, (*sMenu).basePaletteTag);
}
pub unsafe fn FreeMonMarkingsMenu() {
    for i in 0..2u16 {
        FreeSpriteTilesByTag((*sMenu).baseTileTag + i);
        FreeSpritePaletteByTag((*sMenu).basePaletteTag + i);
    }
    let mut i: u16 = 0;
    while i < 2 {
        if (*sMenu).windowSprites[i].is_null() {
            return;
        }
        DestroySprite((*sMenu).windowSprites[i]);
        (*sMenu).windowSprites[i] = null_mut();
        i += 1;
    }
    for i in 0..NUM_MON_MARKINGS {
        if (*sMenu).markingSprites[i].is_null() {
            return;
        }
        DestroySprite((*sMenu).markingSprites[i]);
        (*sMenu).markingSprites[i] = null_mut();
    }
    if !(*sMenu).cursorSprite.is_null() {
        DestroySprite((*sMenu).cursorSprite);
        (*sMenu).cursorSprite = null_mut();
    }
    if !(*sMenu).textSprite.is_null() {
        DestroySprite((*sMenu).textSprite);
        (*sMenu).textSprite = null_mut();
    }
}
pub unsafe fn HandleMonMarkingsMenuInput() -> u8 {
    if gMain.newKeys as i32 & DPAD_UP != 0 {
        PlaySE(SE_SELECT);
        if ({
            (*sMenu).cursorPos -= 1;
            (*sMenu).cursorPos
        }) < 0
        {
            (*sMenu).cursorPos = SELECTION_CANCEL;
        }
        return TRUE;
    }
    if gMain.newKeys as i32 & DPAD_DOWN != 0 {
        PlaySE(SE_SELECT);
        if ({
            (*sMenu).cursorPos += 1;
            (*sMenu).cursorPos
        }) > SELECTION_CANCEL
        {
            (*sMenu).cursorPos = 0;
        }
        return TRUE;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        match (*sMenu).cursorPos {
            SELECTION_OK => {
                (*sMenu).markings = 0;
                for i in 0..NUM_MON_MARKINGS {
                    (*sMenu).markings |= shl_i32((*sMenu).markingsArray[i] as i32, i as u32) as u8;
                }
                return FALSE;
            }
            SELECTION_CANCEL => {
                return FALSE;
            }
            _ => {}
        }
        (*sMenu).markingsArray[(*sMenu).cursorPos] =
            ((*sMenu).markingsArray[(*sMenu).cursorPos] == 0) as u8;
        return TRUE;
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_SELECT);
        return FALSE;
    }
    TRUE
}
unsafe fn CreateMonMarkingsMenuSprites(x: i16, y: i16, baseTileTag: u16, basePaletteTag: u16) {
    let mut spriteId: u8 = 0;
    let mut sheets: CArray<SpriteSheet, 3> = zeroed();
    sheets[0].data = (*sMenu).windowSpriteTiles.as_mut_ptr() as *mut c_void;
    sheets[0].size = 0x1000;
    sheets[0].tag = baseTileTag;
    sheets[1].data = (*(&raw const crate::data::graphics::gMonMarkingsMenu_Gfx)
        .cast::<CArray<u8, 0>>())
    .as_ptr()
    .cast_mut() as *mut c_void;
    sheets[1].size = 0x320;
    sheets[1].tag = baseTileTag + 1;
    let mut palettes: CArray<SpritePalette, 3> = zeroed();
    palettes[0].data = (*sMenu).framePalette;
    palettes[0].tag = basePaletteTag;
    palettes[1].data = (*(&raw const crate::data::graphics::gMonMarkingsMenu_Pal)
        .cast::<CArray<u16, 0>>())
    .as_ptr()
    .cast_mut();
    palettes[1].tag = basePaletteTag + 1;
    let mut template: SpriteTemplate = zeroed();
    template.tileTag = baseTileTag;
    template.paletteTag = basePaletteTag;
    template.oam = (&raw const *sOamData_MenuWindow).cast_mut();
    template.anims = sAnims_MenuWindow.as_ptr().cast_mut();
    template.images = null_mut();
    template.affineAnims = (*(&raw const crate::sprite::gDummySpriteAffineAnimTable)
        .cast::<CArray<*mut AffineAnimCmd, 0>>())
    .as_ptr()
    .cast_mut();
    template.callback = Some(SpriteCB_Dummy);
    LoadSpriteSheets(sheets.as_mut_ptr());
    LoadSpritePalettes(palettes.as_mut_ptr());
    let mut i: u16 = 0;
    while i < 2 {
        spriteId = CreateSprite(&raw mut template, x + 32, y + 32, 1);
        if spriteId != MAX_SPRITES {
            (*sMenu).windowSprites[i] = &raw mut gSprites[spriteId];
            StartSpriteAnim(&raw mut gSprites[spriteId], i as u8);
        } else {
            (*sMenu).windowSprites[i] = null_mut();
            return;
        }
        i += 1;
    }
    (*(*sMenu).windowSprites[1]).y = y + 96;
    template.tileTag += 1;
    template.paletteTag += 1;
    template.anims = sAnims_MenuSprite.as_ptr().cast_mut();
    template.callback = Some(SpriteCB_Marking);
    template.oam = (&raw const *sOamData_8x8).cast_mut();
    for i in 0..NUM_MON_MARKINGS {
        spriteId = CreateSprite(&raw mut template, x + 32, y + 16 + 16 * i as i16, 0);
        if spriteId != MAX_SPRITES {
            (*sMenu).markingSprites[i] = &raw mut gSprites[spriteId];
            gSprites[spriteId].data[0] = i as i16;
        } else {
            (*sMenu).markingSprites[i] = null_mut();
            return;
        }
    }
    template.callback = Some(SpriteCallbackDummy);
    spriteId = CreateSprite(&raw mut template, 0, 0, 0);
    if spriteId != MAX_SPRITES {
        (*sMenu).textSprite = &raw mut gSprites[spriteId];
        (*(*sMenu).textSprite).oam.set_shape(0);
        (*(*sMenu).textSprite).oam.set_size(2);
        StartSpriteAnim((*sMenu).textSprite, ANIM_TEXT);
        (*(*sMenu).textSprite).x = x + 32;
        (*(*sMenu).textSprite).y = y + 80;
        CalcCenterToCornerVec((*sMenu).textSprite, 1, 2, ST_OAM_AFFINE_OFF as u8);
    } else {
        (*sMenu).textSprite = null_mut();
    }
    template.callback = Some(SpriteCB_Cursor);
    spriteId = CreateSprite(&raw mut template, x + 12, 0, 0);
    if spriteId != MAX_SPRITES {
        (*sMenu).cursorSprite = &raw mut gSprites[spriteId];
        (*(*sMenu).cursorSprite).data[0] = y + 16;
        StartSpriteAnim((*sMenu).cursorSprite, ANIM_CURSOR);
    } else {
        (*sMenu).cursorSprite = null_mut();
    }
}
pub(crate) fn SpriteCB_Dummy(sprite: *mut Sprite) {}
pub(crate) unsafe fn SpriteCB_Marking(sprite: *mut Sprite) {
    if (*sMenu).markingsArray[(*sprite).data[sMarkingId]] != 0 {
        StartSpriteAnim(sprite, 2 * (*sprite).data[sMarkingId] as u8 + 1);
    } else {
        StartSpriteAnim(sprite, 2 * (*sprite).data[sMarkingId] as u8);
    }
}
pub(crate) unsafe fn SpriteCB_Cursor(sprite: *mut Sprite) {
    (*sprite).y = 16 * (*sMenu).cursorPos as i16 + (*sprite).data[sCursorYOffset];
}
pub unsafe fn CreateMonMarkingAllCombosSprite(
    tileTag: u16,
    paletteTag: u16,
    mut palette: *mut u16,
) -> *mut Sprite {
    if palette.is_null() {
        palette = sMonMarkings_Pal.as_ptr().cast_mut();
    }
    CreateMarkingComboSprite(tileTag, paletteTag, palette, 16)
}
pub unsafe fn CreateMonMarkingComboSprite(
    tileTag: u16,
    paletteTag: u16,
    mut palette: *mut u16,
) -> *mut Sprite {
    if palette.is_null() {
        palette = sMonMarkings_Pal.as_ptr().cast_mut();
    }
    CreateMarkingComboSprite(tileTag, paletteTag, palette, 1)
}
pub(crate) unsafe fn CreateMarkingComboSprite(
    tileTag: u16,
    paletteTag: u16,
    palette: *mut u16,
    size: u16,
) -> *mut Sprite {
    let mut template: SpriteTemplate = zeroed();
    let mut sheet: SpriteSheet = zeroed();
    sheet.data = sMonMarkings_Gfx.as_ptr().cast_mut() as *mut c_void;
    sheet.size = 0x80;
    sheet.tag = tileTag;
    let mut sprPalette: SpritePalette = zeroed();
    sprPalette.data = palette;
    sprPalette.tag = paletteTag;
    template.tileTag = tileTag;
    template.paletteTag = paletteTag;
    template.oam = (&raw const *sOamData_MarkingCombo).cast_mut();
    template.anims = sAnims_MarkingCombo.as_ptr().cast_mut();
    template.images = null_mut();
    template.affineAnims = (*(&raw const crate::sprite::gDummySpriteAffineAnimTable)
        .cast::<CArray<*mut AffineAnimCmd, 0>>())
    .as_ptr()
    .cast_mut();
    template.callback = Some(SpriteCB_Dummy);
    sheet.size = size * 0x80;
    LoadSpriteSheet(&raw mut sheet);
    LoadSpritePalette(&raw mut sprPalette);
    let spriteId: u8 = CreateSprite(&raw mut template, 0, 0, 0);
    if spriteId != MAX_SPRITES {
        return &raw mut gSprites[spriteId];
    } else {
        return null_mut();
    }
    #[allow(unreachable_code)]
    {
        null_mut()
    }
}
pub unsafe fn UpdateMonMarkingTiles(markings: u8, dest: *mut c_void) {
    RequestDma3Copy(
        (&raw const sMonMarkings_Gfx[markings as i32 * 0x80]).cast_mut() as *mut c_void,
        dest,
        0x80,
        0x10,
    );
}
