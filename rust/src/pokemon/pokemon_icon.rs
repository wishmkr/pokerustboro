//! Translated from `src/pokemon_icon.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::mail_data::MailSpeciesToSpecies;
use crate::palette::LoadPalette;
use crate::sprite::RequestSpriteCopy;
use crate::sprite::gSprites;
use crate::sprite::{FreeSpritePaletteByTag, IndexOfSpritePaletteTag};
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
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
}
// Data tables (translate with cdata.py): gMonIconTable gMonIconPaletteIndices gMonIconPaletteTable sMonIconOamData sAnim_0 sAnim_1 sAnim_2 sAnim_3 sAnim_4 sMonIconAnims sAffineAnim_0 sAffineAnim_1 sMonIconAffineAnims sSpriteImageSizes

/// `struct MonIconSpriteTemplate`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MonIconSpriteTemplate {
    pub oam: *mut OamData,
    pub image: *mut u8,
    pub anims: *mut *mut AnimCmd,
    pub affineAnims: *mut *mut AffineAnimCmd,
    pub callback: Option<unsafe fn(*mut Sprite)>,
    pub paletteTag: u16,
}

unsafe impl Sync for MonIconSpriteTemplate {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<MonIconSpriteTemplate>() == 24);
    assert!(offset_of!(MonIconSpriteTemplate, oam) == 0);
    assert!(offset_of!(MonIconSpriteTemplate, image) == 4);
    assert!(offset_of!(MonIconSpriteTemplate, anims) == 8);
    assert!(offset_of!(MonIconSpriteTemplate, affineAnims) == 12);
    assert!(offset_of!(MonIconSpriteTemplate, callback) == 16);
    assert!(offset_of!(MonIconSpriteTemplate, paletteTag) == 20);
};

const INVALID_ICON_SPECIES: u16 = 260;

static gMonIconPaletteIndices: Table<CArray<u8, 440>> =
    Table((&raw const crate::data::pokemon_icon::gMonIconPaletteIndices).cast());
static gMonIconPaletteTable: Table<CArray<SpritePalette, 6>> =
    Table((&raw const crate::data::pokemon_icon::gMonIconPaletteTable).cast());
static gMonIconTable: Table<CArray<*mut u8, 440>> =
    Table((&raw const crate::data::pokemon_icon::gMonIconTable).cast());
static sMonIconAffineAnims: Table<CArray<*mut AffineAnimCmd, 2>> =
    Table((&raw const crate::data::pokemon_icon::sMonIconAffineAnims).cast());
static sMonIconAnims: Table<CArray<*mut AnimCmd, 5>> =
    Table((&raw const crate::data::pokemon_icon::sMonIconAnims).cast());
static sMonIconOamData: Table<OamData> =
    Table((&raw const crate::data::pokemon_icon::sMonIconOamData).cast());
static sSpriteImageSizes: Table<CArray<CArray<u16, 4>, 3>> =
    Table((&raw const crate::data::pokemon_icon::sSpriteImageSizes).cast());

pub unsafe fn CreateMonIcon(
    species: u16,
    callback: Option<unsafe fn(*mut Sprite)>,
    x: i16,
    y: i16,
    subpriority: u8,
    personality: u32,
    handleDeoxys: u32,
) -> u8 {
    let mut iconTemplate: MonIconSpriteTemplate = zeroed();
    iconTemplate.oam = (&raw const *sMonIconOamData).cast_mut();
    iconTemplate.image = GetMonIconPtr(species, personality, handleDeoxys);
    iconTemplate.anims = sMonIconAnims.as_ptr().cast_mut();
    iconTemplate.affineAnims = sMonIconAffineAnims.as_ptr().cast_mut();
    iconTemplate.callback = callback;
    iconTemplate.paletteTag = POKE_ICON_BASE_PAL_TAG + gMonIconPaletteIndices[species] as u16;
    if species > NUM_SPECIES {
        iconTemplate.paletteTag = POKE_ICON_BASE_PAL_TAG;
    }
    let spriteId: u8 = CreateMonIconSprite(&raw mut iconTemplate, x, y, subpriority);
    UpdateMonIconFrame(&raw mut gSprites[spriteId]);
    spriteId
}
pub unsafe fn CreateMonIconNoPersonality(
    species: u16,
    callback: Option<unsafe fn(*mut Sprite)>,
    x: i16,
    y: i16,
    subpriority: u8,
    handleDeoxys: u32,
) -> u8 {
    let mut iconTemplate: MonIconSpriteTemplate = zeroed();
    iconTemplate.oam = (&raw const *sMonIconOamData).cast_mut();
    iconTemplate.image = null_mut();
    iconTemplate.anims = sMonIconAnims.as_ptr().cast_mut();
    iconTemplate.affineAnims = sMonIconAffineAnims.as_ptr().cast_mut();
    iconTemplate.callback = callback;
    iconTemplate.paletteTag = POKE_ICON_BASE_PAL_TAG + gMonIconPaletteIndices[species] as u16;
    iconTemplate.image = GetMonIconTiles(species, handleDeoxys);
    let spriteId: u8 = CreateMonIconSprite(&raw mut iconTemplate, x, y, subpriority);
    UpdateMonIconFrame(&raw mut gSprites[spriteId]);
    spriteId
}
pub unsafe fn GetIconSpecies(species: u16, personality: u32) -> u16 {
    let mut result: u16 = 0;
    if species == SPECIES_UNOWN {
        let mut letter: u16 = GetUnownLetterByPersonality(personality);
        if letter == 0 {
            letter = SPECIES_UNOWN;
        } else {
            letter += 412;
        }
        result = letter;
    } else {
        if species > NUM_SPECIES {
            result = INVALID_ICON_SPECIES;
        } else {
            result = species;
        }
    }
    result
}
#[unsafe(no_mangle)]
pub fn GetUnownLetterByPersonality(personality: u32) -> u16 {
    if personality == 0 {
        return 0;
    } else {
        return (((personality & 0x03000000) >> 18
            | (personality & 0x00030000) >> 12
            | (personality & 0x00000300) >> 6
            | (personality & 0x00000003))
            % 28) as u16;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetIconSpeciesNoPersonality(mut species: u16) -> u16 {
    let mut value: u16 = 0;
    if MailSpeciesToSpecies(species, &raw mut value) == SPECIES_UNOWN {
        if value == 0 {
            value += SPECIES_UNOWN;
        } else {
            value += 412;
        }
        return value;
    } else {
        if species > NUM_SPECIES {
            species = INVALID_ICON_SPECIES;
        }
        return GetIconSpecies(species, 0);
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetMonIconPtr(species: u16, personality: u32, handleDeoxys: u32) -> *mut u8 {
    GetMonIconTiles(GetIconSpecies(species, personality), handleDeoxys)
}
pub unsafe fn FreeAndDestroyMonIconSprite(sprite: *mut Sprite) {
    FreeAndDestroyMonIconSprite_(sprite);
}
pub unsafe fn LoadMonIconPalettes() {
    for i in 0..6u8 {
        LoadSpritePalette((&raw const gMonIconPaletteTable[i]).cast_mut());
    }
}
pub unsafe fn SafeLoadMonIconPalette(mut species: u16) {
    if species > NUM_SPECIES {
        species = INVALID_ICON_SPECIES;
    }
    let palIndex: u8 = gMonIconPaletteIndices[species];
    if IndexOfSpritePaletteTag(gMonIconPaletteTable[palIndex].tag) == 0xFF {
        LoadSpritePalette((&raw const gMonIconPaletteTable[palIndex]).cast_mut());
    }
}
pub unsafe fn LoadMonIconPalette(species: u16) {
    let palIndex: u8 = gMonIconPaletteIndices[species];
    if IndexOfSpritePaletteTag(gMonIconPaletteTable[palIndex].tag) == 0xFF {
        LoadSpritePalette((&raw const gMonIconPaletteTable[palIndex]).cast_mut());
    }
}
pub unsafe fn FreeMonIconPalettes() {
    for i in 0..6u8 {
        FreeSpritePaletteByTag(gMonIconPaletteTable[i].tag);
    }
}
pub unsafe fn SafeFreeMonIconPalette(mut species: u16) {
    if species > NUM_SPECIES {
        species = INVALID_ICON_SPECIES;
    }
    let palIndex: u8 = gMonIconPaletteIndices[species];
    FreeSpritePaletteByTag(gMonIconPaletteTable[palIndex].tag);
}
pub unsafe fn FreeMonIconPalette(species: u16) {
    let palIndex: u8 = gMonIconPaletteIndices[species];
    FreeSpritePaletteByTag(gMonIconPaletteTable[palIndex].tag);
}
pub unsafe fn SpriteCB_MonIcon(sprite: *mut Sprite) {
    UpdateMonIconFrame(sprite);
}
pub unsafe fn GetMonIconTiles(species: u16, handleDeoxys: u32) -> *mut u8 {
    let mut iconSprite: *mut u8 = gMonIconTable[species];
    if species == SPECIES_DEOXYS as u16 && handleDeoxys == TRUE as u32 {
        iconSprite = (0x400 + iconSprite as usize as u32) as usize as *mut u8;
    }
    iconSprite
}
pub unsafe fn TryLoadAllMonIconPalettesAtOffset(mut offset: u16) {
    if offset <= 160 {
        for i in 0..6i32 {
            LoadPalette(gMonIconPaletteTable[i].data as *mut c_void, offset, 32);
            offset += 16;
        }
    }
}
pub fn GetValidMonIconPalIndex(mut species: u16) -> u8 {
    if species > NUM_SPECIES {
        species = INVALID_ICON_SPECIES;
    }
    gMonIconPaletteIndices[species]
}
pub unsafe fn GetMonIconPaletteIndexFromSpecies(species: u16) -> u8 {
    gMonIconPaletteIndices[species]
}
pub unsafe fn GetValidMonIconPalettePtr(mut species: u16) -> *mut u16 {
    if species > NUM_SPECIES {
        species = INVALID_ICON_SPECIES;
    }
    gMonIconPaletteTable[gMonIconPaletteIndices[species]].data
}
pub unsafe fn UpdateMonIconFrame(sprite: *mut Sprite) -> u8 {
    let mut result: u8 = 0;
    if (*sprite).animDelayCounter() == 0 {
        let frame: i16 = (*(*(*sprite).anims.at((*sprite).animNum)).at((*sprite).animCmdIndex))
            .frame
            .imageValue() as i16;
        match frame {
            -1 => {}
            -2 => {
                (*sprite).animCmdIndex = 0;
            }
            _ => {
                RequestSpriteCopy(
                    ((*sprite).images as *mut u8).at(sSpriteImageSizes[(*sprite).oam.shape()]
                        [(*sprite).oam.size()]
                        as i32
                        * frame as i32),
                    (OBJ_VRAM0 + (*sprite).oam.tileNum() as i32 * 32) as usize as *mut u8,
                    sSpriteImageSizes[(*sprite).oam.shape()][(*sprite).oam.size()],
                );
                (*sprite).set_animDelayCounter(
                    (*(*(*sprite).anims.at((*sprite).animNum)).at((*sprite).animCmdIndex))
                        .frame
                        .duration() as u8,
                );
                (*sprite).animCmdIndex += 1;
                result = (*sprite).animCmdIndex;
            }
        }
    } else {
        (*sprite).set_animDelayCounter((*sprite).animDelayCounter() - 1);
    }
    result
}
pub(crate) unsafe fn CreateMonIconSprite(
    iconTemplate: *mut MonIconSpriteTemplate,
    x: i16,
    y: i16,
    subpriority: u8,
) -> u8 {
    let mut image: SpriteFrameImage = zeroed();
    image.data = null_mut();
    image.size = sSpriteImageSizes[(*(*iconTemplate).oam).shape()][(*(*iconTemplate).oam).size()];
    let mut spriteTemplate: SpriteTemplate = zeroed();
    spriteTemplate.tileTag = TAG_NONE;
    spriteTemplate.paletteTag = (*iconTemplate).paletteTag;
    spriteTemplate.oam = (*iconTemplate).oam;
    spriteTemplate.anims = (*iconTemplate).anims;
    spriteTemplate.images = &raw mut image;
    spriteTemplate.affineAnims = (*iconTemplate).affineAnims;
    spriteTemplate.callback = (*iconTemplate).callback;
    let spriteId: u8 = CreateSprite(&raw mut spriteTemplate, x, y, subpriority);
    gSprites[spriteId].set_animPaused(TRUE);
    gSprites[spriteId].set_animBeginning(FALSE as u16);
    gSprites[spriteId].images = (*iconTemplate).image as *mut SpriteFrameImage;
    spriteId
}
unsafe fn FreeAndDestroyMonIconSprite_(sprite: *mut Sprite) {
    let mut image: SpriteFrameImage = zeroed();
    image.data = null_mut();
    image.size = sSpriteImageSizes[(*sprite).oam.shape()][(*sprite).oam.size()];
    (*sprite).images = &raw mut image;
    DestroySprite(sprite);
}
pub unsafe fn SetPartyHPBarSprite(sprite: *mut Sprite, animNum: u8) {
    (*sprite).animNum = animNum;
    (*sprite).set_animDelayCounter(0);
    (*sprite).animCmdIndex = 0;
}
