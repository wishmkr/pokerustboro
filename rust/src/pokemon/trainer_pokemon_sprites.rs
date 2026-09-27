//! Standalone Pokémon and trainer picture sprites (outside battle): decode
//! the picture into a heap buffer, point a sprite at it, and free both later.
//! Up to eight such pictures can exist at once.

use crate::decompress::{
    DecompressPicFromTable, LoadCompressedSpritePalette, LoadSpecialPokePic,
    LoadSpecialPokePic_DontHandleDeoxys,
};
use crate::ffi::{
    CompressedSpritePalette, CompressedSpriteSheet, OamData, RomPtr, ST_OAM_4BPP,
    ST_OAM_AFFINE_NORMAL, ST_OAM_OBJ_NORMAL, ST_OAM_SQUARE, SpriteTemplate, set_oam_palette_num,
    sprite,
};
use crate::malloc::{Alloc, Free};
use crate::sprite::{
    CreateSprite, DestroySprite, FreeSpritePaletteByTag, GetSpritePaletteTagByPaletteNum,
    gDummySpriteAffineAnimTable,
};
use crate::window::{BlitBitmapRectToWindow, GetWindowAttribute};

const PICS_COUNT: usize = 8;
const PIC_SPRITE_SIZE: u32 = 0x800;
const MAX_PIC_FRAMES: u32 = 4;
const MON_PIC_SIZE: u32 = 0x800;
const MAX_MON_PIC_FRAMES: u32 = 4;
const TRAINER_PIC_SIZE: u32 = 0x800;
const MAX_TRAINER_PIC_FRAMES: u32 = 4;
const TRAINER_PIC_WIDTH: u16 = 64;
const TRAINER_PIC_HEIGHT: u16 = 64;
const TAG_NONE: u16 = 0xffff;
const F_MON_PIC_NO_AFFINE: u8 = 0x80;
const MON_PIC_AFFINE_BACK: u8 = 0;
const MON_PIC_AFFINE_FRONT: u8 = 1;
const MON_PIC_AFFINE_NONE: u8 = 3;
const FACILITY_CLASS_BRENDAN: usize = 0x3c;
const FACILITY_CLASS_MAY: usize = 0x3f;
const MALE: u8 = 0;
const WINDOW_TILE_DATA: u8 = 7;
const PLTT_SIZE_4BPP: u16 = 0x20;
const OBJ_PLTT_OFFSET: u16 = 0x100;
const SHEET_SIZE: usize = 8;
const FAILED: u16 = 0xffff;

/// `struct SpriteFrameImage { const void *data; u16 size; }`
#[repr(C, align(4))]
struct SpriteFrameImage {
    data: *const u8,
    size: u16,
}

/// `struct PicData`, 12 bytes.
#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct PicData {
    frames: *mut u8,
    images: *mut SpriteFrameImage,
    palette_tag: u16,
    sprite_id: u8,
    active: u8,
}

const DUMMY_PIC: PicData = PicData {
    frames: core::ptr::null_mut(),
    images: core::ptr::null_mut(),
    palette_tag: 0,
    sprite_id: 0,
    active: 0,
};

static OAM_NORMAL: OamData = OamData::simple(ST_OAM_SQUARE, 3, 0);
static OAM_AFFINE: OamData = OamData::new(
    ST_OAM_AFFINE_NORMAL,
    ST_OAM_OBJ_NORMAL,
    ST_OAM_4BPP,
    ST_OAM_SQUARE,
    3,
    0,
);

/// `sCreatingSpriteTemplate`: zero-initialised storage like the original
/// (EWRAM is not loaded from ROM), filled in before each use.
#[unsafe(link_section = "ewram_data")]
static mut CREATING_TEMPLATE: crate::ffi::Align4<[u8; 24]> = crate::ffi::Align4([0; 24]);

#[unsafe(link_section = "ewram_data")]
static mut SPRITE_PICS: [PicData; PICS_COUNT] = [DUMMY_PIC; PICS_COUNT];

unsafe extern "C" {
    static gMonFrontPicTable: CompressedSpriteSheet;
    static gMonBackPicTable: CompressedSpriteSheet;
    static gTrainerFrontPicTable: CompressedSpriteSheet;
    static gTrainerBackPicTable: CompressedSpriteSheet;
    static gTrainerFrontPicPaletteTable: CompressedSpritePalette;
    static gAnims_MonPic: u8;
    static gTrainerFrontAnimsPtrTable: [*const RomPtr<crate::ffi::AnimCmd>; 1];
    static gMonFrontAnimsPtrTable: [*const RomPtr<crate::ffi::AnimCmd>; 1];
    static gAffineAnims_BattleSpriteOpponentSide: u8;
    static gAffineAnims_BattleSpritePlayerSide: u8;
    static gFacilityClassToPicIndex: u8;

    fn GetMonSpritePalFromSpeciesAndPersonality(
        species: u16,
        ot_id: u32,
        personality: u32,
    ) -> *const u32;
    fn GetMonSpritePalStructFromOtIdPersonality(
        species: u16,
        ot_id: u32,
        personality: u32,
    ) -> *const CompressedSpritePalette;
    fn LoadCompressedPalette(src: *const u32, offset: u16, size: u16);
}

unsafe extern "C" fn dummy_pic_sprite_callback(_sprite: *mut u8) {}

#[inline]
fn template() -> *mut SpriteTemplate {
    (&raw mut CREATING_TEMPLATE).cast()
}

#[inline]
fn pics() -> *mut PicData {
    (&raw mut SPRITE_PICS).cast()
}

#[inline]
unsafe fn sheet(table: *const CompressedSpriteSheet, index: u16) -> *const CompressedSpriteSheet {
    table
        .cast::<u8>()
        .wrapping_add(usize::from(index) * SHEET_SIZE)
        .cast()
}

#[inline]
unsafe fn trainer_palette(index: u16) -> *const CompressedSpritePalette {
    (&raw const gTrainerFrontPicPaletteTable)
        .cast::<u8>()
        .wrapping_add(usize::from(index) * SHEET_SIZE)
        .cast()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetAllPicSprites() -> u16 {
    for i in 0..PICS_COUNT {
        unsafe { pics().add(i).write(DUMMY_PIC) };
    }
    0
}

unsafe fn decompress_pic(
    pic_id: u16,
    personality: u32,
    is_front_pic: u8,
    dest: *mut u8,
    is_trainer: bool,
    ignore_deoxys: bool,
) -> bool {
    if !is_trainer {
        let table = if is_front_pic != 0 {
            &raw const gMonFrontPicTable
        } else {
            &raw const gMonBackPicTable
        };
        let src = unsafe { sheet(table, pic_id) };
        let species = i32::from(pic_id);
        if ignore_deoxys {
            unsafe {
                LoadSpecialPokePic_DontHandleDeoxys(src, dest, species, personality, is_front_pic)
            };
        } else {
            unsafe { LoadSpecialPokePic(src, dest, species, personality, is_front_pic) };
        }
    } else {
        // Trainer back pics aren't compressed; decompressing them works only
        // because the bytes where a header would be are all zero. Kept as is.
        let table = if is_front_pic != 0 {
            &raw const gTrainerFrontPicTable
        } else {
            &raw const gTrainerBackPicTable
        };
        unsafe { DecompressPicFromTable(sheet(table, pic_id), dest, i32::from(pic_id)) };
    }
    false
}

unsafe fn load_palette_by_tag_or_slot(
    species: u16,
    ot_id: u32,
    personality: u32,
    palette_slot: u8,
    palette_tag: u16,
    is_trainer: bool,
) {
    unsafe { (*template()).palette_tag = palette_tag };
    let offset = OBJ_PLTT_OFFSET + u16::from(palette_slot) * 16;
    match (is_trainer, palette_tag == TAG_NONE) {
        (false, true) => unsafe {
            LoadCompressedPalette(
                GetMonSpritePalFromSpeciesAndPersonality(species, ot_id, personality),
                offset,
                PLTT_SIZE_4BPP,
            )
        },
        (false, false) => unsafe {
            LoadCompressedSpritePalette(GetMonSpritePalStructFromOtIdPersonality(
                species,
                ot_id,
                personality,
            ))
        },
        (true, true) => unsafe {
            LoadCompressedPalette((*trainer_palette(species)).data, offset, PLTT_SIZE_4BPP)
        },
        (true, false) => unsafe { LoadCompressedSpritePalette(trainer_palette(species)) },
    }
}

unsafe fn load_palette_by_slot(
    species: u16,
    ot_id: u32,
    personality: u32,
    palette_slot: u8,
    is_trainer: bool,
) {
    let offset = u16::from(palette_slot) * 16;
    let src = if is_trainer {
        unsafe { (*trainer_palette(species)).data }
    } else {
        unsafe { GetMonSpritePalFromSpeciesAndPersonality(species, ot_id, personality) }
    };
    unsafe { LoadCompressedPalette(src, offset, PLTT_SIZE_4BPP) };
}

fn free_slot() -> Option<usize> {
    (0..PICS_COUNT).find(|&i| unsafe { (*pics().add(i)).active } == 0)
}

/// Allocates the frame buffer and the image table pointing into it.
unsafe fn alloc_frames(frame_size: u32, frames: u32) -> Option<(*mut u8, *mut SpriteFrameImage)> {
    let frame_pics = unsafe { Alloc(frame_size * frames) };
    if frame_pics.is_null() {
        return None;
    }
    let images = unsafe { Alloc(8 * frames) }.cast::<SpriteFrameImage>();
    if images.is_null() {
        unsafe { Free(frame_pics) };
        return None;
    }
    Some((frame_pics, images))
}

unsafe fn fill_images(
    frame_pics: *mut u8,
    images: *mut SpriteFrameImage,
    frame_size: u32,
    frames: u32,
) {
    for j in 0..frames {
        let image = SpriteFrameImage {
            data: frame_pics.wrapping_add((frame_size * j) as usize),
            size: frame_size as u16,
        };
        unsafe { images.add(j as usize).write(image) };
    }
}

unsafe fn finish_pic(
    slot: usize,
    frame_pics: *mut u8,
    images: *mut SpriteFrameImage,
    x: i16,
    y: i16,
    palette_slot: u8,
    palette_tag: u16,
) -> u16 {
    let sprite_id = unsafe { CreateSprite(template(), x, y, 0) };
    if palette_tag == TAG_NONE {
        unsafe { set_oam_palette_num(sprite(usize::from(sprite_id)), palette_slot) };
    }
    let pic = PicData {
        frames: frame_pics,
        images,
        palette_tag,
        sprite_id,
        active: 1,
    };
    unsafe { pics().add(slot).write(pic) };
    u16::from(sprite_id)
}

#[allow(clippy::too_many_arguments)]
unsafe fn create_pic_sprite(
    species: u16,
    ot_id: u32,
    personality: u32,
    is_front_pic: u8,
    x: i16,
    y: i16,
    palette_slot: u8,
    palette_tag: u16,
    is_trainer: bool,
    ignore_deoxys: bool,
) -> u16 {
    let Some(slot) = free_slot() else {
        return FAILED;
    };
    let Some((frame_pics, images)) = (unsafe { alloc_frames(PIC_SPRITE_SIZE, MAX_PIC_FRAMES) })
    else {
        return FAILED;
    };
    if unsafe {
        decompress_pic(
            species,
            personality,
            is_front_pic,
            frame_pics,
            is_trainer,
            ignore_deoxys,
        )
    } {
        return FAILED;
    }
    unsafe { fill_images(frame_pics, images, PIC_SPRITE_SIZE, MAX_PIC_FRAMES) };

    let t = template();
    unsafe {
        (*t).tile_tag = TAG_NONE;
        (*t).oam = &raw const OAM_NORMAL;
        (*t).anims = if is_trainer {
            (&raw const gTrainerFrontAnimsPtrTable)
                .cast::<*const RomPtr<crate::ffi::AnimCmd>>()
                .read()
        } else {
            (&raw const gAnims_MonPic).cast()
        };
        (*t).images = images.cast();
        (*t).affine_anims = (&raw const gDummySpriteAffineAnimTable).cast();
        (*t).callback = dummy_pic_sprite_callback;
    }
    unsafe {
        load_palette_by_tag_or_slot(
            species,
            ot_id,
            personality,
            palette_slot,
            palette_tag,
            is_trainer,
        )
    };
    unsafe { finish_pic(slot, frame_pics, images, x, y, palette_slot, palette_tag) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonPicSprite_Affine(
    species: u16,
    ot_id: u32,
    personality: u32,
    mut flags: u8,
    x: i16,
    y: i16,
    palette_slot: u8,
    palette_tag: u16,
) -> u16 {
    let Some(slot) = free_slot() else {
        return FAILED;
    };
    let frame_pics = unsafe { Alloc(MON_PIC_SIZE * MAX_MON_PIC_FRAMES) };
    if frame_pics.is_null() {
        return FAILED;
    }
    let kind = if flags & F_MON_PIC_NO_AFFINE != 0 {
        flags &= !F_MON_PIC_NO_AFFINE;
        MON_PIC_AFFINE_NONE
    } else {
        flags
    };
    let images = unsafe { Alloc(8 * MAX_MON_PIC_FRAMES) }.cast::<SpriteFrameImage>();
    if images.is_null() {
        unsafe { Free(frame_pics) };
        return FAILED;
    }
    // The remaining flags double as isFrontPic, as in the original.
    if unsafe { decompress_pic(species, personality, flags, frame_pics, false, false) } {
        return FAILED;
    }
    unsafe { fill_images(frame_pics, images, MON_PIC_SIZE, MAX_MON_PIC_FRAMES) };

    let t = template();
    unsafe {
        (*t).tile_tag = TAG_NONE;
        (*t).anims = (&raw const gMonFrontAnimsPtrTable)
            .cast::<*const RomPtr<crate::ffi::AnimCmd>>()
            .add(usize::from(species))
            .read();
        (*t).images = images.cast();
        let (affine, oam): (*const u8, *const OamData) = match kind {
            MON_PIC_AFFINE_FRONT => (
                &raw const gAffineAnims_BattleSpriteOpponentSide,
                &raw const OAM_AFFINE,
            ),
            MON_PIC_AFFINE_BACK => (
                &raw const gAffineAnims_BattleSpritePlayerSide,
                &raw const OAM_AFFINE,
            ),
            _ => (
                (&raw const gDummySpriteAffineAnimTable).cast(),
                &raw const OAM_NORMAL,
            ),
        };
        (*t).affine_anims = affine.cast();
        (*t).oam = oam;
        (*t).callback = dummy_pic_sprite_callback;
    }
    unsafe {
        load_palette_by_tag_or_slot(
            species,
            ot_id,
            personality,
            palette_slot,
            palette_tag,
            false,
        )
    };
    unsafe { finish_pic(slot, frame_pics, images, x, y, palette_slot, palette_tag) }
}

unsafe fn free_and_destroy_pic_sprite(sprite_id: u16) -> u16 {
    let Some(slot) =
        (0..PICS_COUNT).find(|&i| u16::from(unsafe { (*pics().add(i)).sprite_id }) == sprite_id)
    else {
        return FAILED;
    };
    let pic = unsafe { pics().add(slot).read() };
    let s = unsafe { sprite(usize::from(sprite_id)) };
    if pic.palette_tag != TAG_NONE {
        let palette_num = unsafe { s.add(5).read() } >> 4;
        unsafe { FreeSpritePaletteByTag(GetSpritePaletteTagByPaletteNum(palette_num)) };
    }
    unsafe { DestroySprite(s) };
    unsafe { Free(pic.frames) };
    unsafe { Free(pic.images.cast()) };
    unsafe { pics().add(slot).write(DUMMY_PIC) };
    0
}

#[allow(clippy::too_many_arguments)]
unsafe fn create_trainer_card_sprite(
    species: u16,
    ot_id: u32,
    personality: u32,
    is_front_pic: u8,
    dest_x: u16,
    dest_y: u16,
    palette_slot: u8,
    window_id: u8,
    is_trainer: bool,
) -> u16 {
    let frame_pics = unsafe { Alloc(TRAINER_PIC_SIZE * MAX_TRAINER_PIC_FRAMES) };
    if frame_pics.is_null()
        || unsafe {
            decompress_pic(
                species,
                personality,
                is_front_pic,
                frame_pics,
                is_trainer,
                false,
            )
        }
    {
        return FAILED;
    }
    unsafe {
        BlitBitmapRectToWindow(
            window_id,
            frame_pics,
            0,
            0,
            TRAINER_PIC_WIDTH,
            i32::from(TRAINER_PIC_HEIGHT),
            dest_x,
            dest_y,
            TRAINER_PIC_WIDTH,
            TRAINER_PIC_HEIGHT,
        )
    };
    unsafe { load_palette_by_slot(species, ot_id, personality, palette_slot, is_trainer) };
    unsafe { Free(frame_pics) };
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonPicSprite_HandleDeoxys(
    species: u16,
    ot_id: u32,
    personality: u32,
    is_front_pic: u8,
    x: i16,
    y: i16,
    palette_slot: u8,
    palette_tag: u16,
) -> u16 {
    unsafe {
        create_pic_sprite(
            species,
            ot_id,
            personality,
            is_front_pic,
            x,
            y,
            palette_slot,
            palette_tag,
            false,
            false,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeAndDestroyMonPicSprite(sprite_id: u16) -> u16 {
    unsafe { free_and_destroy_pic_sprite(sprite_id) }
}

/// Unused (FRLG only).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTrainerCardMonIconSprite(
    species: u16,
    ot_id: u32,
    personality: u32,
    is_front_pic: u8,
    dest_x: u16,
    dest_y: u16,
    palette_slot: u8,
    window_id: u8,
) -> u16 {
    unsafe {
        create_trainer_card_sprite(
            species,
            ot_id,
            personality,
            is_front_pic,
            dest_x,
            dest_y,
            palette_slot,
            window_id,
            false,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTrainerPicSprite(
    species: u16,
    is_front_pic: u8,
    x: i16,
    y: i16,
    palette_slot: u8,
    palette_tag: u16,
) -> u16 {
    unsafe {
        create_pic_sprite(
            species,
            0,
            0,
            is_front_pic,
            x,
            y,
            palette_slot,
            palette_tag,
            true,
            false,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeAndDestroyTrainerPicSprite(sprite_id: u16) -> u16 {
    unsafe { free_and_destroy_pic_sprite(sprite_id) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTrainerCardTrainerPicSprite(
    species: u16,
    is_front_pic: u8,
    dest_x: u16,
    dest_y: u16,
    palette_slot: u8,
    window_id: u8,
) -> u16 {
    unsafe {
        create_trainer_card_sprite(
            species,
            0,
            0,
            is_front_pic,
            dest_x,
            dest_y,
            palette_slot,
            window_id,
            true,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerGenderToFrontTrainerPicId_Debug(gender: u8, get_class: u8) -> u16 {
    if get_class == 1 {
        let class = if gender != MALE {
            FACILITY_CLASS_MAY
        } else {
            FACILITY_CLASS_BRENDAN
        };
        return u16::from(unsafe { (&raw const gFacilityClassToPicIndex).add(class).read() });
    }
    u16::from(gender)
}

/// `LoadPicSpriteInWindow`: unused in Emerald, kept for completeness.
#[allow(dead_code)]
unsafe fn load_pic_sprite_in_window(
    species: u16,
    ot_id: u32,
    personality: u32,
    is_front_pic: u8,
    palette_slot: u8,
    window_id: u8,
    is_trainer: bool,
) -> u16 {
    let tiles = unsafe { GetWindowAttribute(window_id, WINDOW_TILE_DATA) } as usize as *mut u8;
    if unsafe { decompress_pic(species, personality, is_front_pic, tiles, false, false) } {
        return FAILED;
    }
    unsafe { load_palette_by_slot(species, ot_id, personality, palette_slot, is_trainer) };
    0
}
