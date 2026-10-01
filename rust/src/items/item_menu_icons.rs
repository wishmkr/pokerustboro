//! Translated from `src/item_menu_icons.c` by tools/rustport/c2rs.py.
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
    clippy::no_effect,
    unused_variables
)]

use crate::berry::IsEnigmaBerryValid;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::item_icon::AddItemIconSprite;
use crate::item_menu::gBagMenu;
use crate::menu_helpers::{
    CreateSwapLineSprites, SetSwapLineSpritesInvisibility, UpdateSwapLineSpritesPos,
};
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
/// `FreeSpriteOamMatrix` with this module's view of its types.
#[inline]
unsafe fn FreeSpriteOamMatrix(a0: *mut Sprite) {
    unsafe {
        crate::sprite::FreeSpriteOamMatrix(a0 as _);
    }
}
/// `InitSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn InitSpriteAffineAnim(a0: *mut Sprite) {
    unsafe {
        crate::sprite::InitSpriteAffineAnim(a0 as _);
    }
}
/// `LZDecompressWram` with this module's view of its types.
#[inline]
unsafe fn LZDecompressWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressWram(a0 as _, a1 as _);
    }
}
/// `LoadCompressedSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette) {
    unsafe {
        crate::decompress::LoadCompressedSpritePalette(a0 as _);
    }
}
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
}
/// `LoadSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16 {
    unsafe { crate::sprite::LoadSpriteSheet(a0 as _) }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
/// `StartSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAffineAnim(a0 as _, a1);
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
const sPocketId: usize = 0;
// Data tables (translate with cdata.py): sRotatingBall_Pal sRotatingBall_Gfx sCherryUnused sCherryUnused_Pal sBagOamData sSpriteAnim_Bag_Closed sSpriteAnim_Bag_Items sSpriteAnim_Bag_KeyItems sSpriteAnim_Bag_Pokeballs sSpriteAnim_Bag_TMsHMs sSpriteAnim_Bag_Berries sBagSpriteAnimTable sSpriteAffineAnim_BagNormal sSpriteAffineAnim_BagShake sBagAffineAnimCmds gBagMaleSpriteSheet gBagFemaleSpriteSheet gBagPaletteTable sBagSpriteTemplate sRotatingBallOamData sSpriteAffineAnim_RotatingBallStationary sRotatingBallSpriteAnimTable sSpriteAffineAnim_RotatingBallRotation1 sSpriteAffineAnim_RotatingBallRotation2 sRotatingBallAnimCmds sRotatingBallAnimCmds_FullRotation sRotatingBallTable sRotatingBallPaletteTable sRotatingBallSpriteTemplate sBerryPicOamData sBerryPicRotatingOamData sAnim_BerryPic sBerryPicSpriteAnimTable sBerryPicSpriteImageTable sBerryPicSpriteTemplate sSpriteAffineAnim_BerryPicRotation1 sSpriteAffineAnim_BerryPicRotation2 sBerryPicRotatingAnimCmds sBerryPicRotatingSpriteTemplate sBerryPicTable gBerryCheckCircleSpriteSheet gBerryCheckCirclePaletteTable sBerryCheckCircleOamData sSpriteAnim_BerryCheckCircle sBerryCheckCircleSpriteAnimTable sBerryCheckCircleSpriteTemplate

/// `struct CompressedTilesPal`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CompressedTilesPal {
    pub tiles: *mut u32,
    pub pal: *mut u32,
}

unsafe impl Sync for CompressedTilesPal {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<CompressedTilesPal>() == 8);
    assert!(offset_of!(CompressedTilesPal, tiles) == 0);
    assert!(offset_of!(CompressedTilesPal, pal) == 4);
};

const ANIM_BAG_NORMAL: u8 = 0;
const ANIM_BAG_SHAKE: u8 = 1;
const TAG_BAG_GFX: u16 = 100;
const TAG_BERRY_PIC_PAL: u16 = 30020;
const TAG_ITEM_ICON: u16 = 102;

static sBagSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::item_menu_icons::sBagSpriteTemplate).cast());
static sBerryCheckCircleSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::item_menu_icons::sBerryCheckCircleSpriteTemplate).cast());
static sBerryPicRotatingSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::item_menu_icons::sBerryPicRotatingSpriteTemplate).cast());
static sBerryPicSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::item_menu_icons::sBerryPicSpriteTemplate).cast());
static sBerryPicTable: Table<CArray<CompressedTilesPal, 43>> =
    Table((&raw const crate::data::item_menu_icons::sBerryPicTable).cast());
static sRotatingBallAnimCmds: Table<CArray<*mut AffineAnimCmd, 1>> =
    Table((&raw const crate::data::item_menu_icons::sRotatingBallAnimCmds).cast());
static sRotatingBallAnimCmds_FullRotation: Table<CArray<*mut AffineAnimCmd, 1>> =
    Table((&raw const crate::data::item_menu_icons::sRotatingBallAnimCmds_FullRotation).cast());
static sRotatingBallPaletteTable: Table<SpritePalette> =
    Table((&raw const crate::data::item_menu_icons::sRotatingBallPaletteTable).cast());
static sRotatingBallSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::item_menu_icons::sRotatingBallSpriteTemplate).cast());
static sRotatingBallTable: Table<SpriteSheet> =
    Table((&raw const crate::data::item_menu_icons::sRotatingBallTable).cast());

pub unsafe fn RemoveBagSprite(id: u8) {
    let spriteId: *mut u8 = &raw mut (*gBagMenu).spriteIds[id];
    if *spriteId != SPRITE_NONE {
        FreeSpriteTilesByTag(id as u16 + TAG_BAG_GFX);
        FreeSpritePaletteByTag(id as u16 + TAG_BAG_GFX);
        FreeSpriteOamMatrix(&raw mut gSprites[*spriteId]);
        DestroySprite(&raw mut gSprites[*spriteId]);
        *spriteId = SPRITE_NONE;
    }
}
pub unsafe fn AddBagVisualSprite(bagPocketId: u8) {
    let spriteId: *mut u8 = &raw mut (*gBagMenu).spriteIds[0];
    *spriteId = CreateSprite((&raw const *sBagSpriteTemplate).cast_mut(), 68, 66, 0);
    SetBagVisualPocketId(bagPocketId, FALSE);
}
pub unsafe fn SetBagVisualPocketId(bagPocketId: u8, isSwitchingPockets: u8) {
    let sprite: *mut Sprite = &raw mut gSprites[(*gBagMenu).spriteIds[0]];
    if isSwitchingPockets != 0 {
        (*sprite).y2 = -5;
        (*sprite).callback = Some(SpriteCB_BagVisualSwitchingPockets);
        (*sprite).data[sPocketId] = bagPocketId as i16 + 1;
        StartSpriteAnim(sprite, POCKET_NONE);
    } else {
        StartSpriteAnim(sprite, bagPocketId + 1);
    }
}
pub(crate) unsafe fn SpriteCB_BagVisualSwitchingPockets(sprite: *mut Sprite) {
    if (*sprite).y2 != 0 {
        (*sprite).y2 += 1;
    } else {
        StartSpriteAnim(sprite, (*sprite).data[sPocketId] as u8);
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub unsafe fn ShakeBagSprite() {
    let sprite: *mut Sprite = &raw mut gSprites[(*gBagMenu).spriteIds[0]];
    if (*sprite).affineAnimEnded() != 0 {
        StartSpriteAffineAnim(sprite, ANIM_BAG_SHAKE);
        (*sprite).callback = Some(SpriteCB_ShakeBagSprite);
    }
}
pub(crate) unsafe fn SpriteCB_ShakeBagSprite(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0 {
        StartSpriteAffineAnim(sprite, ANIM_BAG_NORMAL);
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub unsafe fn AddSwitchPocketRotatingBallSprite(rotationDirection: i16) {
    let spriteId: *mut u8 = &raw mut (*gBagMenu).spriteIds[1];
    LoadSpriteSheet((&raw const *sRotatingBallTable).cast_mut());
    LoadSpritePalette((&raw const *sRotatingBallPaletteTable).cast_mut());
    *spriteId = CreateSprite(
        (&raw const *sRotatingBallSpriteTemplate).cast_mut(),
        16,
        16,
        0,
    );
    gSprites[*spriteId].data[0] = rotationDirection;
}
unsafe fn UpdateSwitchPocketRotatingBallCoords(sprite: *mut Sprite) {
    (*sprite).centerToCornerVecX = (*sprite).data[1] as i8 - (((*sprite).data[3] as i8 + 1) & 1);
    (*sprite).centerToCornerVecY = (*sprite).data[1] as i8 - (((*sprite).data[3] as i8 + 1) & 1);
}
pub(crate) unsafe fn SpriteCB_SwitchPocketRotatingBallInit(sprite: *mut Sprite) {
    (*sprite).oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
    if (*sprite).data[0] == -1 {
        (*sprite).affineAnims = sRotatingBallAnimCmds.as_ptr().cast_mut();
    } else {
        (*sprite).affineAnims = sRotatingBallAnimCmds_FullRotation.as_ptr().cast_mut();
    }
    InitSpriteAffineAnim(sprite);
    (*sprite).data[1] = (*sprite).centerToCornerVecX as i16;
    (*sprite).data[1] = (*sprite).centerToCornerVecY as i16;
    UpdateSwitchPocketRotatingBallCoords(sprite);
    (*sprite).callback = Some(SpriteCB_SwitchPocketRotatingBallContinue);
}
pub(crate) unsafe fn SpriteCB_SwitchPocketRotatingBallContinue(sprite: *mut Sprite) {
    (*sprite).data[3] += 1;
    UpdateSwitchPocketRotatingBallCoords(sprite);
    if (*sprite).data[3] == 16 {
        RemoveBagSprite(ITEMMENUSPRITE_BALL);
    }
}
pub unsafe fn AddBagItemIconSprite(itemId: u16, id: u8) {
    let spriteId: *mut u8 = &raw mut (*gBagMenu).spriteIds[id as i32 + ITEMMENUSPRITE_ITEM];
    if *spriteId == SPRITE_NONE {
        FreeSpriteTilesByTag(id as u16 + TAG_ITEM_ICON);
        FreeSpritePaletteByTag(id as u16 + TAG_ITEM_ICON);
        let iconSpriteId: u8 =
            AddItemIconSprite(id as u16 + TAG_ITEM_ICON, id as u16 + TAG_ITEM_ICON, itemId);
        if iconSpriteId != MAX_SPRITES {
            *spriteId = iconSpriteId;
            gSprites[iconSpriteId].x2 = 24;
            gSprites[iconSpriteId].y2 = 88;
        }
    }
}
pub unsafe fn RemoveBagItemIconSprite(id: u8) {
    RemoveBagSprite(id + ITEMMENUSPRITE_ITEM as u8);
}
pub unsafe fn CreateItemMenuSwapLine() {
    CreateSwapLineSprites(&raw mut (*gBagMenu).spriteIds[4], ITEMMENU_SWAP_LINE_LENGTH);
}
pub unsafe fn SetItemMenuSwapLineInvisibility(invisible: u8) {
    SetSwapLineSpritesInvisibility(
        &raw mut (*gBagMenu).spriteIds[4],
        ITEMMENU_SWAP_LINE_LENGTH,
        invisible,
    );
}
pub unsafe fn UpdateItemMenuSwapLinePos(y: u8) {
    UpdateSwapLineSpritesPos(
        &raw mut (*gBagMenu).spriteIds[4],
        136,
        120,
        (y as u16 + 1) * 16,
    );
}
unsafe fn ArrangeBerryGfx(mut src: *mut c_void, mut dest: *mut c_void) {
    memset(dest as *mut u8, 0, 0x800);
    dest = (dest as *mut u8).at(256) as *mut c_void;
    for i in 0..6u8 {
        dest = (dest as *mut u8).at(32) as *mut c_void;
        for j in 0..6u8 {
            memcpy(dest as *mut u8, src as *mut u8, 0x20);
            dest = (dest as *mut u8).at(32) as *mut c_void;
            src = (src as *mut u8).at(32) as *mut c_void;
        }
        if i != 5 {
            dest = (dest as *mut u8).at(32) as *mut c_void;
        }
    }
}
pub(crate) unsafe fn LoadBerryGfx(berryId: u8) {
    let mut pal: CompressedSpritePalette = zeroed();
    if berryId == 42 {
        IsEnigmaBerryValid();
        0;
    }
    pal.data = sBerryPicTable[berryId].pal;
    pal.tag = TAG_BERRY_PIC_PAL;
    LoadCompressedSpritePalette(&raw mut pal);
    LZDecompressWram(
        sBerryPicTable[berryId].tiles,
        &raw mut (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())[4096] as *mut c_void,
    );
    ArrangeBerryGfx(
        &raw mut (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())[4096] as *mut c_void,
        &raw mut (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())[0] as *mut c_void,
    );
}
pub unsafe fn CreateBerryTagSprite(id: u8, x: i16, y: i16) -> u8 {
    LoadBerryGfx(id);
    CreateSprite((&raw const *sBerryPicSpriteTemplate).cast_mut(), x, y, 0)
}
pub unsafe fn FreeBerryTagSpritePalette() {
    FreeSpritePaletteByTag(TAG_BERRY_PIC_PAL);
}
pub unsafe fn CreateSpinningBerrySprite(berryId: u8, x: u8, y: u8, startAffine: u8) -> u8 {
    FreeSpritePaletteByTag(TAG_BERRY_PIC_PAL);
    LoadBerryGfx(berryId);
    let spriteId: u8 = CreateSprite(
        (&raw const *sBerryPicRotatingSpriteTemplate).cast_mut(),
        x as i16,
        y as i16,
        0,
    );
    if startAffine == TRUE {
        StartSpriteAffineAnim(&raw mut gSprites[spriteId], 1);
    }
    spriteId
}
pub unsafe fn CreateBerryFlavorCircleSprite(x: i16) -> u8 {
    CreateSprite(
        (&raw const *sBerryCheckCircleSpriteTemplate).cast_mut(),
        x,
        116,
        0,
    )
}
