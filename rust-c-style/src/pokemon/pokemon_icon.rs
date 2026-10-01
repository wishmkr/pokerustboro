//! Translated from `src/pokemon_icon.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): gMonIconTable gMonIconPaletteIndices gMonIconPaletteTable sMonIconOamData sAnim_0 sAnim_1 sAnim_2 sAnim_3 sAnim_4 sMonIconAnims sAffineAnim_0 sAffineAnim_1 sMonIconAffineAnims sSpriteImageSizes

/// `struct MonIconSpriteTemplate`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MonIconSpriteTemplate {
    pub oam: *mut OamData,
    pub image: *mut u8,
    pub anims: *mut *mut AnimCmd,
    pub affineAnims: *mut *mut AffineAnimCmd,
    pub callback: Option<unsafe extern "C" fn(*mut Sprite)>,
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

unsafe extern "C" {
    static mut gSprites: CArray<Sprite, 65>;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn FreeSpritePaletteByTag(a0: u16);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn MailSpeciesToSpecies(a0: u16, a1: *mut u16) -> u16;
    fn RequestSpriteCopy(a0: *mut u8, a1: *mut u8, a2: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonIcon(
    species: u16,
    callback: Option<unsafe extern "C" fn(*mut Sprite)>,
    x: i16,
    y: i16,
    subpriority: u8,
    personality: u32,
    handleDeoxys: u32,
) -> u8 {
    let mut spriteId: u8 = 0;
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
    spriteId = CreateMonIconSprite(&raw mut iconTemplate, x, y, subpriority);
    UpdateMonIconFrame(&raw mut gSprites[spriteId]);
    return spriteId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonIconNoPersonality(
    species: u16,
    callback: Option<unsafe extern "C" fn(*mut Sprite)>,
    x: i16,
    y: i16,
    subpriority: u8,
    handleDeoxys: u32,
) -> u8 {
    let mut spriteId: u8 = 0;
    let mut iconTemplate: MonIconSpriteTemplate = zeroed();
    iconTemplate.oam = (&raw const *sMonIconOamData).cast_mut();
    iconTemplate.image = null_mut();
    iconTemplate.anims = sMonIconAnims.as_ptr().cast_mut();
    iconTemplate.affineAnims = sMonIconAffineAnims.as_ptr().cast_mut();
    iconTemplate.callback = callback;
    iconTemplate.paletteTag = POKE_ICON_BASE_PAL_TAG + gMonIconPaletteIndices[species] as u16;
    iconTemplate.image = GetMonIconTiles(species, handleDeoxys);
    spriteId = CreateMonIconSprite(&raw mut iconTemplate, x, y, subpriority);
    UpdateMonIconFrame(&raw mut gSprites[spriteId]);
    return spriteId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetIconSpecies(species: u16, personality: u32) -> u16 {
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
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUnownLetterByPersonality(personality: u32) -> u16 {
    if personality == 0 {
        return 0;
    } else {
        return (((personality & 0x03000000) >> 18
            | (personality & 0x00030000) >> 12
            | (personality & 0x00000300) >> 6
            | (personality & 0x00000003) >> 0)
            % 28) as u16;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetIconSpeciesNoPersonality(mut species: u16) -> u16 {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonIconPtr(
    species: u16,
    personality: u32,
    handleDeoxys: u32,
) -> *mut u8 {
    return GetMonIconTiles(GetIconSpecies(species, personality), handleDeoxys);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeAndDestroyMonIconSprite(sprite: *mut Sprite) {
    FreeAndDestroyMonIconSprite_(sprite);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadMonIconPalettes() {
    let mut i: u8 = 0;
    i = 0;
    while i < 6 {
        LoadSpritePalette((&raw const gMonIconPaletteTable[i]).cast_mut());
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SafeLoadMonIconPalette(mut species: u16) {
    let mut palIndex: u8 = 0;
    if species > NUM_SPECIES {
        species = INVALID_ICON_SPECIES;
    }
    palIndex = gMonIconPaletteIndices[species];
    if IndexOfSpritePaletteTag(gMonIconPaletteTable[palIndex].tag) == 0xFF {
        LoadSpritePalette((&raw const gMonIconPaletteTable[palIndex]).cast_mut());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadMonIconPalette(species: u16) {
    let mut palIndex: u8 = gMonIconPaletteIndices[species];
    if IndexOfSpritePaletteTag(gMonIconPaletteTable[palIndex].tag) == 0xFF {
        LoadSpritePalette((&raw const gMonIconPaletteTable[palIndex]).cast_mut());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeMonIconPalettes() {
    let mut i: u8 = 0;
    i = 0;
    while i < 6 {
        FreeSpritePaletteByTag(gMonIconPaletteTable[i].tag);
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SafeFreeMonIconPalette(mut species: u16) {
    let mut palIndex: u8 = 0;
    if species > NUM_SPECIES {
        species = INVALID_ICON_SPECIES;
    }
    palIndex = gMonIconPaletteIndices[species];
    FreeSpritePaletteByTag(gMonIconPaletteTable[palIndex].tag);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeMonIconPalette(species: u16) {
    let mut palIndex: u8 = 0;
    palIndex = gMonIconPaletteIndices[species];
    FreeSpritePaletteByTag(gMonIconPaletteTable[palIndex].tag);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_MonIcon(sprite: *mut Sprite) {
    UpdateMonIconFrame(sprite);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonIconTiles(species: u16, handleDeoxys: u32) -> *mut u8 {
    let mut iconSprite: *mut u8 = gMonIconTable[species];
    if species == SPECIES_DEOXYS as u16 && handleDeoxys == TRUE as u32 {
        iconSprite = (0x400 + iconSprite as usize as u32) as usize as *mut u8;
    }
    return iconSprite;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryLoadAllMonIconPalettesAtOffset(mut offset: u16) {
    let mut i: i32 = 0;
    if offset <= 160 {
        i = 0;
        while i < 6 {
            LoadPalette(gMonIconPaletteTable[i].data as *mut c_void, offset, 32);
            offset += 16;
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetValidMonIconPalIndex(mut species: u16) -> u8 {
    if species > NUM_SPECIES {
        species = INVALID_ICON_SPECIES;
    }
    return gMonIconPaletteIndices[species];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonIconPaletteIndexFromSpecies(species: u16) -> u8 {
    return gMonIconPaletteIndices[species];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetValidMonIconPalettePtr(mut species: u16) -> *mut u16 {
    if species > NUM_SPECIES {
        species = INVALID_ICON_SPECIES;
    }
    return gMonIconPaletteTable[gMonIconPaletteIndices[species]].data;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateMonIconFrame(sprite: *mut Sprite) -> u8 {
    let mut result: u8 = 0;
    if (*sprite).animDelayCounter() == 0 {
        let mut frame: i16 = (*(*(*sprite).anims.at((*sprite).animNum)).at((*sprite).animCmdIndex))
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
                        .duration() as u8
                        & 0xFF,
                );
                (*sprite).animCmdIndex += 1;
                result = (*sprite).animCmdIndex;
            }
        }
    } else {
        (*sprite).set_animDelayCounter((*sprite).animDelayCounter() - 1);
    }
    return result;
}
pub(crate) unsafe extern "C" fn CreateMonIconSprite(
    iconTemplate: *mut MonIconSpriteTemplate,
    x: i16,
    y: i16,
    subpriority: u8,
) -> u8 {
    let mut spriteId: u8 = 0;
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
    spriteId = CreateSprite(&raw mut spriteTemplate, x, y, subpriority);
    gSprites[spriteId].set_animPaused(TRUE);
    gSprites[spriteId].set_animBeginning(FALSE as u16);
    gSprites[spriteId].images = (*iconTemplate).image as *mut SpriteFrameImage;
    return spriteId;
}
pub(crate) unsafe extern "C" fn FreeAndDestroyMonIconSprite_(sprite: *mut Sprite) {
    let mut image: SpriteFrameImage = zeroed();
    image.data = null_mut();
    image.size = sSpriteImageSizes[(*sprite).oam.shape()][(*sprite).oam.size()];
    (*sprite).images = &raw mut image;
    DestroySprite(sprite);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPartyHPBarSprite(sprite: *mut Sprite, animNum: u8) {
    (*sprite).animNum = animNum;
    (*sprite).set_animDelayCounter(0);
    (*sprite).animCmdIndex = 0;
}
