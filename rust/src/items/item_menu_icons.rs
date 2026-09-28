//! Translated from `src/item_menu_icons.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    unused_mut,
    unused_variables,
    unused_assignments,
    unused_parens,
    unused_braces,
    unused_labels,
    unused_comparisons,
    overflowing_literals,
    unused_unsafe,
    dead_code,
    unreachable_code,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
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

unsafe extern "C" {
    static mut gBagMenu: *mut BagMenu;
    static mut gDecompressionBuffer: CArray<u8, 16384>;
    static mut gSprites: CArray<Sprite, 65>;
    fn AddItemIconSprite(a0: u16, a1: u16, a2: u16) -> u8;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateSwapLineSprites(a0: *mut u8, a1: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn FreeSpriteOamMatrix(a0: *mut Sprite);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn InitSpriteAffineAnim(a0: *mut Sprite);
    fn IsEnigmaBerryValid() -> u32;
    fn LZDecompressWram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn SetSwapLineSpritesInvisibility(a0: *mut u8, a1: u8, a2: u8);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn UpdateSwapLineSpritesPos(a0: *mut u8, a1: u8, a2: i16, a3: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveBagSprite(id: u8) {
    let mut spriteId: *mut u8 = &raw mut (*gBagMenu).spriteIds[id];
    if *spriteId != SPRITE_NONE {
        FreeSpriteTilesByTag(id as u16 + TAG_BAG_GFX);
        FreeSpritePaletteByTag(id as u16 + TAG_BAG_GFX);
        FreeSpriteOamMatrix(&raw mut gSprites[*spriteId]);
        DestroySprite(&raw mut gSprites[*spriteId]);
        *spriteId = SPRITE_NONE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddBagVisualSprite(bagPocketId: u8) {
    let mut spriteId: *mut u8 = &raw mut (*gBagMenu).spriteIds[0];
    *spriteId = CreateSprite((&raw const *sBagSpriteTemplate).cast_mut(), 68, 66, 0);
    SetBagVisualPocketId(bagPocketId, FALSE);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBagVisualPocketId(bagPocketId: u8, isSwitchingPockets: u8) {
    let mut sprite: *mut Sprite = &raw mut gSprites[(*gBagMenu).spriteIds[0]];
    if isSwitchingPockets != 0 {
        (*sprite).y2 = -5;
        (*sprite).callback = Some(SpriteCB_BagVisualSwitchingPockets);
        (*sprite).data[0] = bagPocketId as i16 + 1;
        StartSpriteAnim(sprite, POCKET_NONE);
    } else {
        StartSpriteAnim(sprite, bagPocketId + 1);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BagVisualSwitchingPockets(sprite: *mut Sprite) {
    if (*sprite).y2 != 0 {
        (*sprite).y2 += 1;
    } else {
        StartSpriteAnim(sprite, (*sprite).data[0] as u8);
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShakeBagSprite() {
    let mut sprite: *mut Sprite = &raw mut gSprites[(*gBagMenu).spriteIds[0]];
    if (*sprite).affineAnimEnded() != 0 {
        StartSpriteAffineAnim(sprite, ANIM_BAG_SHAKE);
        (*sprite).callback = Some(SpriteCB_ShakeBagSprite);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ShakeBagSprite(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0 {
        StartSpriteAffineAnim(sprite, ANIM_BAG_NORMAL);
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddSwitchPocketRotatingBallSprite(rotationDirection: i16) {
    let mut spriteId: *mut u8 = &raw mut (*gBagMenu).spriteIds[1];
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
pub(crate) unsafe extern "C" fn UpdateSwitchPocketRotatingBallCoords(sprite: *mut Sprite) {
    (*sprite).centerToCornerVecX = (*sprite).data[1] as i8 - ((*sprite).data[3] as i8 + 1 & 1);
    (*sprite).centerToCornerVecY = (*sprite).data[1] as i8 - ((*sprite).data[3] as i8 + 1 & 1);
}
pub(crate) unsafe extern "C" fn SpriteCB_SwitchPocketRotatingBallInit(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn SpriteCB_SwitchPocketRotatingBallContinue(sprite: *mut Sprite) {
    (*sprite).data[3] += 1;
    UpdateSwitchPocketRotatingBallCoords(sprite);
    if (*sprite).data[3] == 16 {
        RemoveBagSprite(ITEMMENUSPRITE_BALL);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddBagItemIconSprite(itemId: u16, id: u8) {
    let mut spriteId: *mut u8 = &raw mut (*gBagMenu).spriteIds[id as i32 + ITEMMENUSPRITE_ITEM];
    if *spriteId == SPRITE_NONE {
        let mut iconSpriteId: u8 = 0;
        FreeSpriteTilesByTag(id as u16 + TAG_ITEM_ICON);
        FreeSpritePaletteByTag(id as u16 + TAG_ITEM_ICON);
        iconSpriteId =
            AddItemIconSprite(id as u16 + TAG_ITEM_ICON, id as u16 + TAG_ITEM_ICON, itemId);
        if iconSpriteId != MAX_SPRITES {
            *spriteId = iconSpriteId;
            gSprites[iconSpriteId].x2 = 24;
            gSprites[iconSpriteId].y2 = 88;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveBagItemIconSprite(id: u8) {
    RemoveBagSprite(id + ITEMMENUSPRITE_ITEM as u8);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateItemMenuSwapLine() {
    CreateSwapLineSprites(&raw mut (*gBagMenu).spriteIds[4], ITEMMENU_SWAP_LINE_LENGTH);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetItemMenuSwapLineInvisibility(invisible: u8) {
    SetSwapLineSpritesInvisibility(
        &raw mut (*gBagMenu).spriteIds[4],
        ITEMMENU_SWAP_LINE_LENGTH,
        invisible,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateItemMenuSwapLinePos(y: u8) {
    UpdateSwapLineSpritesPos(
        &raw mut (*gBagMenu).spriteIds[4],
        136,
        120,
        (y as u16 + 1) * 16,
    );
}
pub(crate) unsafe extern "C" fn ArrangeBerryGfx(mut src: *mut c_void, mut dest: *mut c_void) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    memset(dest as *mut u8, 0, 0x800);
    dest = (dest as *mut u8).at(256) as *mut c_void;
    i = 0;
    while i < 6 {
        dest = (dest as *mut u8).at(32) as *mut c_void;
        j = 0;
        while j < 6 {
            memcpy(dest as *mut u8, src as *mut u8, 0x20);
            dest = (dest as *mut u8).at(32) as *mut c_void;
            src = (src as *mut u8).at(32) as *mut c_void;
            j += 1;
        }
        if i != 5 {
            dest = (dest as *mut u8).at(32) as *mut c_void;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn LoadBerryGfx(berryId: u8) {
    let mut pal: CompressedSpritePalette = zeroed();
    if berryId == 42 && IsEnigmaBerryValid() != 0 {}
    pal.data = sBerryPicTable[berryId].pal;
    pal.tag = TAG_BERRY_PIC_PAL;
    LoadCompressedSpritePalette(&raw mut pal);
    LZDecompressWram(
        sBerryPicTable[berryId].tiles,
        &raw mut gDecompressionBuffer[4096] as *mut c_void,
    );
    ArrangeBerryGfx(
        &raw mut gDecompressionBuffer[4096] as *mut c_void,
        &raw mut gDecompressionBuffer[0] as *mut c_void,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateBerryTagSprite(id: u8, x: i16, y: i16) -> u8 {
    LoadBerryGfx(id);
    return CreateSprite((&raw const *sBerryPicSpriteTemplate).cast_mut(), x, y, 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeBerryTagSpritePalette() {
    FreeSpritePaletteByTag(TAG_BERRY_PIC_PAL);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateSpinningBerrySprite(
    berryId: u8,
    x: u8,
    y: u8,
    startAffine: u8,
) -> u8 {
    let mut spriteId: u8 = 0;
    FreeSpritePaletteByTag(TAG_BERRY_PIC_PAL);
    LoadBerryGfx(berryId);
    spriteId = CreateSprite(
        (&raw const *sBerryPicRotatingSpriteTemplate).cast_mut(),
        x as i16,
        y as i16,
        0,
    );
    if startAffine == TRUE {
        StartSpriteAffineAnim(&raw mut gSprites[spriteId], 1);
    }
    return spriteId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateBerryFlavorCircleSprite(x: i16) -> u8 {
    return CreateSprite(
        (&raw const *sBerryCheckCircleSpriteTemplate).cast_mut(),
        x,
        116,
        0,
    );
}
