//! The four-puff smokescreen impact used by Smokescreen, Octazooka and the
//! Smoke Ball, plus a few tables that share this file with it in the original.

use crate::decompress::{LoadCompressedSpritePaletteUsingHeap, LoadCompressedSpriteSheetUsingHeap};
use crate::ffi::{
    ANIMCMD_END, AnimCmd, CompressedSpritePalette, CompressedSpriteSheet, OamData, RomPtr,
    ST_OAM_H_RECTANGLE, ST_OAM_SQUARE, SpriteCallbackDummy, SpriteTemplate, anim_frame,
    gDummySpriteAffineAnimTable, gDummySpriteAnimTable, set_sprite_callback, sprite,
    sprite_anim_ended, sprite_data,
};
use crate::sprite::{
    AnimateSprite, CreateSprite, DestroySprite, FreeSpritePaletteByTag, FreeSpriteTilesByTag,
    GetSpriteTileStartByTag, StartSpriteAnim,
};
use crate::util::CreateInvisibleSpriteWithCallback;

const TAG_SMOKESCREEN: u16 = 55019;
const PALTAG_SHADOW: u16 = 55039;
const GFXTAG_SHADOW: u16 = 55129;

const STRONGER: u8 = 0;
const WEAKER: u8 = 1;
const RANDOM: u8 = 2;

/// Which target each nature prefers in the Battle Palace. Belongs to
/// battle_gfx_sfx_util.c but lives here in the original ROM layout.
#[unsafe(no_mangle)]
pub static gBattlePalaceNatureToMoveTarget: crate::ffi::RomBytes<25> = crate::ffi::RomBytes([
    STRONGER, STRONGER, WEAKER, STRONGER, WEAKER, // Hardy..Naughty
    WEAKER, RANDOM, STRONGER, STRONGER, STRONGER, // Bold..Lax
    WEAKER, WEAKER, WEAKER, STRONGER, RANDOM, // Timid..Naive
    WEAKER, STRONGER, WEAKER, WEAKER, STRONGER, // Modest..Rash
    STRONGER, STRONGER, WEAKER, WEAKER, STRONGER, // Calm..Quirky
]);

/// `SpriteCB_SetInvisible` with this module's view of its types.
#[inline]
unsafe fn SpriteCB_SetInvisible(a0: *mut u8) {
    unsafe {
        crate::battle_gfx_sfx_util::SpriteCB_SetInvisible(a0 as _);
    }
}

static SMOKESCREEN_SHEET: CompressedSpriteSheet = CompressedSpriteSheet {
    data: (&raw const (*(&raw const crate::data::graphics::gSmokescreenImpactTiles).cast::<u32>()))
        .cast(),
    size: 0x180,
    tag: TAG_SMOKESCREEN,
};

static SMOKESCREEN_PALETTE: CompressedSpritePalette = CompressedSpritePalette {
    data: (&raw const (*(&raw const crate::data::graphics::gSmokescreenImpactPalette)
        .cast::<u32>()))
        .cast(),
    tag: TAG_SMOKESCREEN,
};

static OAM_SMOKESCREEN: OamData = OamData::simple(ST_OAM_SQUARE, 1, 1);

/// One puff per quadrant, flipped so the four form a symmetric cloud.
static ANIMS: [[AnimCmd; 4]; 4] = {
    let mut anims = [[ANIMCMD_END; 4]; 4];
    let mut i = 0;
    while i < 4 {
        let h = i & 1 != 0;
        let v = i & 2 != 0;
        anims[i] = [
            anim_frame(0, 4, h, v),
            anim_frame(4, 4, h, v),
            anim_frame(8, 4, h, v),
            ANIMCMD_END,
        ];
        i += 1;
    }
    anims
};

static ANIM_TABLE: [RomPtr<AnimCmd>; 4] = [
    RomPtr((&raw const ANIMS[0]).cast()),
    RomPtr((&raw const ANIMS[1]).cast()),
    RomPtr((&raw const ANIMS[2]).cast()),
    RomPtr((&raw const ANIMS[3]).cast()),
];

static SMOKESCREEN_TEMPLATE: SpriteTemplate = SpriteTemplate {
    tile_tag: TAG_SMOKESCREEN,
    palette_tag: TAG_SMOKESCREEN,
    oam: &raw const OAM_SMOKESCREEN,
    anims: ANIM_TABLE.as_ptr(),
    images: core::ptr::null(),
    affine_anims: (&raw const gDummySpriteAffineAnimTable).cast(),
    callback: sprite_cb_smokescreen_impact,
};

#[unsafe(no_mangle)]
pub static gSpriteSheet_EnemyShadow: CompressedSpriteSheet = CompressedSpriteSheet {
    data: (&raw const (*(&raw const crate::data::graphics::gEnemyMonShadow_Gfx).cast::<u32>()))
        .cast(),
    size: 0x80,
    tag: GFXTAG_SHADOW,
};

static OAM_ENEMY_SHADOW: OamData = OamData::simple(ST_OAM_H_RECTANGLE, 1, 3);

#[unsafe(no_mangle)]
pub static gSpriteTemplate_EnemyShadow: SpriteTemplate = SpriteTemplate {
    tile_tag: GFXTAG_SHADOW,
    palette_tag: PALTAG_SHADOW,
    oam: &raw const OAM_ENEMY_SHADOW,
    anims: (&raw const gDummySpriteAnimTable).cast(),
    images: core::ptr::null(),
    affine_anims: (&raw const gDummySpriteAffineAnimTable).cast(),
    callback: SpriteCB_SetInvisible,
};

// Main sprite: data[0] counts live puffs, data[1] keeps it after they end.
const S_ACTIVE_SPRITES: usize = 0;
const S_PERSIST: usize = 1;
// Puffs: data[0] is the main sprite's id.
const S_MAIN_SPRITE_ID: usize = 0;

#[unsafe(no_mangle)]
pub unsafe fn SmokescreenImpact(x: i16, y: i16, persist: u8) -> u8 {
    if unsafe { GetSpriteTileStartByTag(SMOKESCREEN_SHEET.tag) } == 0xffff {
        unsafe { LoadCompressedSpriteSheetUsingHeap(&raw const SMOKESCREEN_SHEET) };
        unsafe { LoadCompressedSpritePaletteUsingHeap(&raw const SMOKESCREEN_PALETTE) };
    }

    let main_id = unsafe { CreateInvisibleSpriteWithCallback(sprite_cb_smokescreen_impact_main) };
    let main = unsafe { sprite(usize::from(main_id)) };
    unsafe { sprite_data(main, S_PERSIST).write(i16::from(persist)) };

    // Top left, top right, bottom left, bottom right.
    let corners = [
        (x.wrapping_sub(16), y.wrapping_sub(16)),
        (x, y.wrapping_sub(16)),
        (x.wrapping_sub(16), y),
        (x, y),
    ];
    for (anim, (px, py)) in corners.into_iter().enumerate() {
        let id = unsafe { CreateSprite(&raw const SMOKESCREEN_TEMPLATE, px, py, 2) };
        let puff = unsafe { sprite(usize::from(id)) };
        unsafe { sprite_data(puff, S_MAIN_SPRITE_ID).write(i16::from(main_id)) };
        let active = unsafe { sprite_data(main, S_ACTIVE_SPRITES) };
        unsafe { active.write(active.read().wrapping_add(1)) };
        if anim != 0 {
            unsafe { StartSpriteAnim(puff, anim as u8) };
        }
        unsafe { AnimateSprite(puff) };
    }

    main_id
}

unsafe fn sprite_cb_smokescreen_impact_main(main: *mut u8) {
    if unsafe { sprite_data(main, S_ACTIVE_SPRITES).read() } == 0 {
        unsafe { FreeSpriteTilesByTag(SMOKESCREEN_SHEET.tag) };
        unsafe { FreeSpritePaletteByTag(SMOKESCREEN_PALETTE.tag) };
        if unsafe { sprite_data(main, S_PERSIST).read() } == 0 {
            unsafe { DestroySprite(main) };
        } else {
            unsafe { set_sprite_callback(main, SpriteCallbackDummy) };
        }
    }
}

unsafe fn sprite_cb_smokescreen_impact(puff: *mut u8) {
    if unsafe { sprite_anim_ended(puff) } {
        let main_id = unsafe { sprite_data(puff, S_MAIN_SPRITE_ID).read() } as u16;
        let active = unsafe { sprite_data(sprite(usize::from(main_id)), S_ACTIVE_SPRITES) };
        unsafe { active.write(active.read().wrapping_sub(1)) };
        unsafe { DestroySprite(puff) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oam_and_anim_words_match_gcc() {
        assert_eq!(
            OAM_ENEMY_SHADOW.0,
            [0x00, 0x40, 0x00, 0x40, 0x00, 0x0c, 0x00, 0x00]
        );
        assert_eq!(ANIMS[3][2].0, 0x00c4_0008);
        assert_eq!(ANIMS[0][0].0, 0x0004_0000);
    }
}
