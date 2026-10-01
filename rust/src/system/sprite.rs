//! Sprites: the object layer, its animation state machines and the OAM
//! buffer that is shipped to hardware every frame.
//!
//! `struct Sprite` is 68 bytes of mixed pointers and bitfields shared with C,
//! so it is reached through byte offsets rather than a Rust struct: a Rust
//! mirror would be wider on a 64-bit host and the offsets would drift.

use crate::ffi::{
    AnimCmd, CpuSet, OamData, RomBytes, RomPtr, SpriteCallback, SpriteTemplate, oam_attr,
    set_oam_palette_num, set_oam_priority, set_oam_tile_num, set_oam_x, set_oam_y,
};
use core::ffi::c_void;

pub const MAX_SPRITES: usize = 64;
pub const OAM_MATRIX_COUNT: usize = 32;
const TOTAL_OBJ_TILE_COUNT: u16 = 1024;
const MAX_SPRITE_COPY_REQUESTS: usize = 64;
const TAG_NONE: u16 = 0xffff;
const TILE_SIZE_4BPP: u16 = 32;

const OBJ_VRAM0: usize = 0x0601_0000;
const OAM: usize = 0x0700_0000;
const OAM_BUFFER_BYTES: usize = 1024;
const OBJ_PLTT_OFFSET: u16 = 256;
const PLTT_SIZE_4BPP: u16 = 32;
const NO_ANCHOR: i32 = 2048;

const DISPLAY_WIDTH: i16 = 240;
const DISPLAY_HEIGHT: i16 = 160;

const ST_OAM_SQUARE: u8 = 0;
const ST_OAM_V_RECTANGLE: u8 = 2;
const ST_OAM_AFFINE_OFF: u16 = 0;
const ST_OAM_AFFINE_ON_MASK: u8 = 1;
const ST_OAM_AFFINE_DOUBLE: u8 = 3;
const ST_OAM_AFFINE_DOUBLE_MASK: u8 = 2;
const ST_OAM_SIZE_3: u8 = 3;

const SUBSPRITES_OFF: u8 = 0;
const SUBSPRITES_ON: u8 = 1;
const SUBSPRITES_IGNORE_PRIORITY: u8 = 2;

const ANIM_END: i16 = -1;
const AFFINE_ANIM_END: i16 = 0x7fff;

/// `offsetof(struct Main, oamBuffer)` and the `oamLoadDisabled` bit.
const MAIN_OAM_BUFFER: usize = 56;
const MAIN_OAM_LOAD_DISABLED_BYTE: usize = 0x439;
const MAIN_OAM_LOAD_DISABLED_BIT: u8 = 1 << 0;

// ------------------------------------------------ struct Sprite (68 bytes)

pub const SPRITE_SIZE: usize = 68;
const S_OAM: usize = 0x00;
const S_ANIMS: usize = 0x08;
const S_IMAGES: usize = 0x0c;
const S_AFFINE_ANIMS: usize = 0x10;
const S_TEMPLATE: usize = 0x14;
const S_SUBSPRITE_TABLES: usize = 0x18;
const S_CALLBACK: usize = 0x1c;
const S_X: usize = 0x20;
const S_Y: usize = 0x22;
const S_X2: usize = 0x24;
const S_Y2: usize = 0x26;
const S_CENTER_TO_CORNER_X: usize = 0x28;
const S_CENTER_TO_CORNER_Y: usize = 0x29;
const S_ANIM_NUM: usize = 0x2a;
const S_ANIM_CMD_INDEX: usize = 0x2b;
/// `animDelayCounter:6`, `animPaused:1`, `affineAnimPaused:1`.
const S_ANIM_STATE: usize = 0x2c;
const ANIM_DELAY_MASK: u8 = 0x3f;
const ANIM_PAUSED_BIT: u8 = 0x40;
const AFFINE_ANIM_PAUSED_BIT: u8 = 0x80;
const S_ANIM_LOOP_COUNTER: usize = 0x2d;
const S_DATA: usize = 0x2e;
/// First flag byte.
const S_FLAGS0: usize = 0x3e;
const F_IN_USE: u8 = 0x01;
const F_COORD_OFFSET_ENABLED: u8 = 0x02;
const F_INVISIBLE: u8 = 0x04;
/// Second flag byte.
const S_FLAGS1: usize = 0x3f;
const F_H_FLIP: u8 = 0x01;
const F_V_FLIP: u8 = 0x02;
const F_ANIM_BEGINNING: u8 = 0x04;
const F_AFFINE_ANIM_BEGINNING: u8 = 0x08;
const F_ANIM_ENDED: u8 = 0x10;
const F_AFFINE_ANIM_ENDED: u8 = 0x20;
const F_USING_SHEET: u8 = 0x40;
const F_ANCHORED: u8 = 0x80;
const S_SHEET_TILE_START: usize = 0x40;
/// `subspriteTableNum:6`, `subspriteMode:2`.
const S_SUBSPRITE_STATE: usize = 0x42;
const SUBSPRITE_TABLE_NUM_MASK: u8 = 0x3f;
const SUBSPRITE_MODE_SHIFT: u32 = 6;
const S_SUBPRIORITY: usize = 0x43;

/// The anchor coordinates live in general-purpose data slots 6 and 7.
const ANCHOR_X_SLOT: usize = 6;
const ANCHOR_Y_SLOT: usize = 7;

/// `union AnimCmd` is one word: the image index in the low half, then a
/// six-bit duration and the two flip bits.
const ANIM_IMAGE_VALUE_MASK: u32 = 0x0000_ffff;
const ANIM_DURATION_SHIFT: u32 = 16;
const ANIM_DURATION_MASK: u32 = 0x003f_0000;
const ANIM_H_FLIP: u32 = 0x0040_0000;
const ANIM_V_FLIP: u32 = 0x0080_0000;

/// `union AffineAnimCmd` is eight bytes: two scales, a rotation and a
/// duration, or a type plus a count/target.
const AFFINE_CMD_SIZE: usize = 8;
const AFFINE_X_SCALE: usize = 0;
const AFFINE_Y_SCALE: usize = 2;
const AFFINE_ROTATION: usize = 4;
const AFFINE_DURATION: usize = 5;
const AFFINE_COUNT: usize = 2;
const AFFINE_TARGET: usize = 2;

/// `struct AffineAnimState`, 12 bytes.
const AAS_SIZE: usize = 12;
const AAS_ANIM_NUM: usize = 0;
const AAS_ANIM_CMD_INDEX: usize = 1;
const AAS_DELAY_COUNTER: usize = 2;
const AAS_LOOP_COUNTER: usize = 3;
const AAS_X_SCALE: usize = 4;
const AAS_Y_SCALE: usize = 6;
const AAS_ROTATION: usize = 8;

/// `struct SpriteFrameImage`, `SpriteSheet` and `SpritePalette` all start
/// with a pointer.
const IMAGE_DATA: usize = 0;
const IMAGE_SIZE: usize = 4;
const SHEET_SIZE: usize = 4;
const SHEET_TAG: usize = 6;
const PALETTE_TAG: usize = 4;
const SPRITE_FRAME_IMAGE_STRIDE: usize = 8;
const SPRITE_SHEET_STRIDE: usize = 8;
const SPRITE_PALETTE_STRIDE: usize = 8;

/// `struct Subsprite`: two signed offsets then a packed halfword.
const SUBSPRITE_STRIDE: usize = 4;
const SUBSPRITE_X: usize = 0;
const SUBSPRITE_Y: usize = 1;
const SUBSPRITE_PACKED: usize = 2;
const SUBSPRITE_SHAPE_MASK: u16 = 0x0003;
const SUBSPRITE_SIZE_SHIFT: u32 = 2;
const SUBSPRITE_SIZE_MASK: u16 = 0x000c;
const SUBSPRITE_TILE_OFFSET_SHIFT: u32 = 4;
const SUBSPRITE_TILE_OFFSET_MASK: u16 = 0x3ff0;
const SUBSPRITE_PRIORITY_SHIFT: u32 = 14;

/// `struct SubspriteTable`: a count then a pointer.
const SUBSPRITE_TABLE_STRIDE: usize = 8;
const SUBSPRITE_TABLE_COUNT: usize = 0;
const SUBSPRITE_TABLE_SUBSPRITES: usize = 4;

/// `struct SpriteCopyRequest`: source, destination, size.
const COPY_REQUEST_STRIDE: usize = 12;
const COPY_SRC: usize = 0;
const COPY_DEST: usize = 4;
const COPY_SIZE: usize = 8;

/// The C copy-request array has pointer fields and therefore four-byte
/// alignment under the GBA ABI.  Keeping it as a bare byte array gives it
/// alignment one; an unaligned ARM word store then rounds the address down
/// and leaves only the upper half of each pointer in the request.
#[repr(C, align(4))]
struct SpriteCopyRequestStorage([[u8; COPY_REQUEST_STRIDE]; MAX_SPRITE_COPY_REQUESTS]);

/// BIOS `ObjAffineSet` uses the APCS layout. These structures are word
/// aligned, and the three-halfword source is padded to eight bytes.
#[repr(C, align(4))]
struct ObjAffineSource([i16; 3]);

#[repr(C, align(4))]
struct OamMatrixStorage([i16; 4]);

#[repr(C, align(4))]
struct AffineAnimStateStorage([[u8; AAS_SIZE]; OAM_MATRIX_COUNT]);

// ------------------------------------------------------------ static data

/// Vectors from a sprite's centre to its top-left corner, by shape and size.
/// Stored unsigned because the original reads them into a `u8` and doubles
/// them there, which relies on the wraparound.
static CENTER_TO_CORNER_VEC_TABLE: [[[u8; 2]; 4]; 3] = [
    // square
    [[0xfc, 0xfc], [0xf8, 0xf8], [0xf0, 0xf0], [0xe0, 0xe0]],
    // horizontal rectangle
    [[0xf8, 0xfc], [0xf0, 0xfc], [0xf0, 0xf8], [0xe0, 0xf0]],
    // vertical rectangle
    [[0xfc, 0xf8], [0xfc, 0xf0], [0xf8, 0xf0], [0xf0, 0xe0]],
];

/// Sprite dimensions in pixels, indexed by shape then size.
static OAM_DIMENSIONS: [[[u8; 2]; 4]; 3] = [
    [[8, 8], [16, 16], [32, 32], [64, 64]],
    [[16, 8], [32, 8], [32, 16], [64, 32]],
    [[8, 16], [8, 32], [16, 32], [32, 64]],
];

/// `DUMMY_OAM_DATA`: parked off-screen at the lowest priority.
/// y = 160, x = 304, priority = 3.
#[unsafe(no_mangle)]
pub static gDummyOamData: OamData = OamData([0xa0, 0x00, 0x30, 0x01, 0x00, 0x0c, 0x00, 0x00]);

static DUMMY_ANIM: AnimCmd = AnimCmd(ANIM_END as u16 as u32);

#[unsafe(no_mangle)]
pub static gDummySpriteAnimTable: [RomPtr<AnimCmd>; 1] = [RomPtr(&raw const DUMMY_ANIM)];

/// An eight-byte affine command whose type halfword is AFFINE_ANIM_END.
/// Word aligned like the C union: the type is read with a halfword load, and
/// at an odd address the ARM returns a rotated value, which is not END, so
/// every sprite using the dummy table would run garbage affine frames.
static DUMMY_AFFINE_ANIM: RomBytes<AFFINE_CMD_SIZE> = RomBytes([0xff, 0x7f, 0, 0, 0, 0, 0, 0]);

#[unsafe(no_mangle)]
pub static gDummySpriteAffineAnimTable: [RomPtr<u8>; 1] = [RomPtr(DUMMY_AFFINE_ANIM.as_ptr())];

#[unsafe(no_mangle)]
pub static gDummySpriteTemplate: SpriteTemplate = SpriteTemplate {
    tile_tag: 0,
    palette_tag: TAG_NONE,
    oam: &raw const gDummyOamData,
    anims: (&raw const gDummySpriteAnimTable).cast(),
    images: core::ptr::null(),
    affine_anims: (&raw const gDummySpriteAffineAnimTable).cast(),
    callback: SpriteCallbackDummy,
};

// ------------------------------------------------------------------- state

// iwram bss
static mut SPRITE_TILE_RANGE_TAGS: [u16; MAX_SPRITES] = [0; MAX_SPRITES];
static mut SPRITE_TILE_RANGES: [[u16; 2]; MAX_SPRITES] = [[0; 2]; MAX_SPRITES];
static mut AFFINE_ANIM_STATES: AffineAnimStateStorage =
    AffineAnimStateStorage([[0; AAS_SIZE]; OAM_MATRIX_COUNT]);
static mut SPRITE_PALETTE_TAGS: [u16; 16] = [0; 16];

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static gOamMatrixAllocBitmap: crate::global::Global<u32> = crate::global::Global::new(0);

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gReservedSpritePaletteCount: u8 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
/// The sprites, and one more (MAX_SPRITES + 1) that stays unused, as in C.
pub static mut gSprites: crate::c::CArray<crate::types::Sprite, 65> =
    unsafe { core::mem::zeroed() };
const _: () = assert!(MAX_SPRITES + 1 == 65);

#[unsafe(link_section = "ewram_data")]
static mut SPRITE_PRIORITIES: [u16; MAX_SPRITES] = [0; MAX_SPRITES];

#[unsafe(link_section = "ewram_data")]
static mut SPRITE_ORDER: crate::ffi::Align4<[u8; MAX_SPRITES]> =
    crate::ffi::Align4([0; MAX_SPRITES]);

#[unsafe(link_section = "ewram_data")]
static SHOULD_PROCESS_SPRITE_COPY_REQUESTS: crate::global::Global<u8> =
    crate::global::Global::new(0);

#[unsafe(link_section = "ewram_data")]
static SPRITE_COPY_REQUEST_COUNT: crate::global::Global<u8> = crate::global::Global::new(0);

#[unsafe(link_section = "ewram_data")]
static mut SPRITE_COPY_REQUESTS: SpriteCopyRequestStorage =
    SpriteCopyRequestStorage([[0; COPY_REQUEST_STRIDE]; MAX_SPRITE_COPY_REQUESTS]);

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gOamLimit: u8 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gReservedSpriteTileCount: u16 = 0;

#[unsafe(link_section = "ewram_data")]
static mut SPRITE_TILE_ALLOC_BITMAP: crate::ffi::Align4<[u8; 128]> = crate::ffi::Align4([0; 128]);

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSpriteCoordOffsetX: i16 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSpriteCoordOffsetY: i16 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gOamMatrices: crate::ffi::Align4<[[i16; 4]; OAM_MATRIX_COUNT]> =
    crate::ffi::Align4([[0; 4]; OAM_MATRIX_COUNT]);

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAffineAnimsDisabled: u8 = 0;

/// `ObjAffineSet` with this module's view of its types.
#[inline]
unsafe fn ObjAffineSet(a0: *const u8, a1: *mut u8, a2: i32, a3: i32) {
    unsafe {
        crate::syscall::ObjAffineSet(a0 as _, a1 as _, a2, a3);
    }
}
/// `LoadPalette` with this module's view of its types.
#[inline]
unsafe fn LoadPalette(a0: *const c_void, a1: u16, a2: u16) {
    unsafe {
        crate::palette::LoadPalette(a0 as _, a1, a2);
    }
}

// -------------------------------------------------------------- accessors

#[inline]
unsafe fn sprite_at(index: usize) -> *mut u8 {
    unsafe { (&raw mut gSprites).cast::<u8>().add(index * SPRITE_SIZE) }
}

#[inline]
unsafe fn u8_at(sprite: *mut u8, offset: usize) -> u8 {
    unsafe { sprite.add(offset).read_volatile() }
}

#[inline]
unsafe fn set_u8_at(sprite: *mut u8, offset: usize, value: u8) {
    unsafe { sprite.add(offset).write_volatile(value) };
}

#[inline]
unsafe fn i16_at(sprite: *mut u8, offset: usize) -> i16 {
    unsafe { sprite.add(offset).cast::<i16>().read_volatile() }
}

#[inline]
unsafe fn set_i16_at(sprite: *mut u8, offset: usize, value: i16) {
    unsafe { sprite.add(offset).cast::<i16>().write_volatile(value) };
}

#[inline]
unsafe fn u16_at(sprite: *mut u8, offset: usize) -> u16 {
    unsafe { sprite.add(offset).cast::<u16>().read_volatile() }
}

#[inline]
unsafe fn set_u16_at(sprite: *mut u8, offset: usize, value: u16) {
    unsafe { sprite.add(offset).cast::<u16>().write_volatile(value) };
}

#[inline]
unsafe fn ptr_at(sprite: *mut u8, offset: usize) -> *mut u8 {
    unsafe { sprite.add(offset).cast::<*mut u8>().read() }
}

#[inline]
unsafe fn set_ptr_at(sprite: *mut u8, offset: usize, value: *const u8) {
    unsafe { sprite.add(offset).cast::<*const u8>().write(value) };
}

#[inline]
unsafe fn flag(sprite: *mut u8, byte: usize, bit: u8) -> bool {
    let value = unsafe { u8_at(sprite, byte) };
    value & bit != 0
}

#[inline]
unsafe fn set_flag(sprite: *mut u8, byte: usize, bit: u8, value: bool) {
    let current = unsafe { u8_at(sprite, byte) };
    let updated = if value { current | bit } else { current & !bit };
    unsafe { set_u8_at(sprite, byte, updated) };
}

#[inline]
unsafe fn oam_of(sprite: *mut u8) -> *mut u8 {
    unsafe { sprite.add(S_OAM) }
}

#[inline]
unsafe fn oam_affine_mode(sprite: *mut u8) -> u8 {
    ((unsafe { oam_attr(oam_of(sprite), 0) } >> 8) & 0x3) as u8
}

#[inline]
unsafe fn oam_shape(sprite: *mut u8) -> u8 {
    (unsafe { oam_attr(oam_of(sprite), 0) } >> 14) as u8
}

#[inline]
unsafe fn oam_size(sprite: *mut u8) -> u8 {
    (unsafe { oam_attr(oam_of(sprite), 1) } >> 14) as u8
}

#[inline]
unsafe fn oam_matrix_num(sprite: *mut u8) -> u8 {
    ((unsafe { oam_attr(oam_of(sprite), 1) } >> 9) & 0x1f) as u8
}

#[inline]
unsafe fn set_oam_matrix_num(sprite: *mut u8, value: u8) {
    let oam = unsafe { oam_of(sprite) };
    let attr = unsafe { oam_attr(oam, 1) };
    let updated = (attr & !(0x1f << 9)) | ((u16::from(value) & 0x1f) << 9);
    unsafe { oam.add(2).cast::<u16>().write_volatile(updated) };
}

#[inline]
unsafe fn oam_tile_num(sprite: *mut u8) -> u16 {
    let attr = unsafe { oam_attr(oam_of(sprite), 2) };
    attr & 0x3ff
}

#[inline]
unsafe fn oam_priority(sprite: *mut u8) -> u8 {
    ((unsafe { oam_attr(oam_of(sprite), 2) } >> 10) & 0x3) as u8
}

#[inline]
unsafe fn oam_y(sprite: *mut u8) -> i16 {
    (unsafe { oam_attr(oam_of(sprite), 0) } & 0xff) as i16
}

#[inline]
unsafe fn matrix(index: usize) -> *mut i16 {
    unsafe { (&raw mut gOamMatrices).cast::<i16>().add(index * 4) }
}

#[inline]
unsafe fn affine_state(matrix_num: u8) -> *mut u8 {
    unsafe {
        (&raw mut AFFINE_ANIM_STATES)
            .cast::<AffineAnimStateStorage>()
            .cast::<u8>()
            .add(matrix_num as usize * AAS_SIZE)
    }
}

#[inline]
unsafe fn oam_buffer_entry(index: usize) -> *mut u8 {
    unsafe {
        (&raw mut (*(&raw const crate::agb_main::gMain).cast::<u8>().cast_mut()))
            .add(MAIN_OAM_BUFFER + index * 8)
    }
}

/// `sSpriteTileAllocBitmap` helpers.
#[inline]
unsafe fn tile_is_allocated(n: u16) -> bool {
    let byte = unsafe {
        (&raw const SPRITE_TILE_ALLOC_BITMAP)
            .cast::<u8>()
            .add(n as usize / 8)
            .read_volatile()
    };
    (byte >> (n % 8)) & 1 != 0
}

#[inline]
unsafe fn alloc_tile(n: u16) {
    let slot = unsafe {
        (&raw mut SPRITE_TILE_ALLOC_BITMAP)
            .cast::<u8>()
            .add(n as usize / 8)
    };
    unsafe { slot.write_volatile(slot.read_volatile() | (1 << (n % 8))) };
}

#[inline]
unsafe fn free_tile(n: u16) {
    let slot = unsafe {
        (&raw mut SPRITE_TILE_ALLOC_BITMAP)
            .cast::<u8>()
            .add(n as usize / 8)
    };
    unsafe { slot.write_volatile(slot.read_volatile() & !(1 << (n % 8))) };
}

/// `CpuCopy16(src, dest, size)`
#[inline]
unsafe fn cpu_copy16(src: *const u8, dest: *mut u8, size: u16) {
    unsafe { CpuSet(src.cast(), dest.cast(), u32::from(size) / 2 & 0x1f_ffff) };
}

/// `CpuCopy32(src, dest, size)`
#[inline]
unsafe fn cpu_copy32(src: *const u8, dest: *mut u8, size: u32) {
    unsafe {
        CpuSet(
            src.cast(),
            dest.cast(),
            0x0400_0000 | (size / 4 & 0x1f_ffff),
        )
    };
}

// ------------------------------------------------------------------- API

#[unsafe(no_mangle)]
pub unsafe fn SpriteCallbackDummy(_sprite: *mut u8) {}

#[unsafe(no_mangle)]
pub unsafe fn ResetSpriteData() {
    unsafe { ResetOamRange(0, 128) };
    unsafe { reset_all_sprites() };
    unsafe { ClearSpriteCopyRequests() };
    unsafe { ResetAffineAnimData() };
    unsafe { FreeSpriteTileRanges() };
    unsafe { (&raw mut gOamLimit).write_volatile(64) };
    unsafe { (&raw mut gReservedSpriteTileCount).write_volatile(0) };
    unsafe { AllocSpriteTiles(0) };
    unsafe { (&raw mut gSpriteCoordOffsetX).write_volatile(0) };
    unsafe { (&raw mut gSpriteCoordOffsetY).write_volatile(0) };
}

#[unsafe(no_mangle)]
pub unsafe fn AnimateSprites() {
    for i in 0..MAX_SPRITES {
        let sprite = unsafe { sprite_at(i) };
        if !unsafe { flag(sprite, S_FLAGS0, F_IN_USE) } {
            continue;
        }

        let callback = unsafe { sprite.add(S_CALLBACK).cast::<SpriteCallback>().read() };
        unsafe { callback(sprite) };

        if unsafe { flag(sprite, S_FLAGS0, F_IN_USE) } {
            unsafe { AnimateSprite(sprite) };
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe fn BuildOamBuffer() {
    unsafe { UpdateOamCoords() };
    unsafe { BuildSpritePriorities() };
    unsafe { SortSprites() };

    // OAM loading is suppressed while the buffer is rebuilt.
    let disabled_slot = unsafe {
        (&raw mut (*(&raw const crate::agb_main::gMain).cast::<u8>().cast_mut()))
            .add(MAIN_OAM_LOAD_DISABLED_BYTE)
    };
    let saved = unsafe { disabled_slot.read_volatile() };
    unsafe { disabled_slot.write_volatile(saved | MAIN_OAM_LOAD_DISABLED_BIT) };

    unsafe { AddSpritesToOamBuffer() };
    unsafe { CopyMatricesToOamBuffer() };

    unsafe { disabled_slot.write_volatile(saved) };
    unsafe { (SHOULD_PROCESS_SPRITE_COPY_REQUESTS.as_ptr()).write_volatile(1) };
}

#[unsafe(no_mangle)]
pub unsafe fn UpdateOamCoords() {
    let offset_x = unsafe { (&raw const gSpriteCoordOffsetX).read_volatile() };
    let offset_y = unsafe { (&raw const gSpriteCoordOffsetY).read_volatile() };

    for i in 0..MAX_SPRITES {
        let sprite = unsafe { sprite_at(i) };
        if !unsafe { flag(sprite, S_FLAGS0, F_IN_USE) }
            || unsafe { flag(sprite, S_FLAGS0, F_INVISIBLE) }
        {
            continue;
        }

        let (extra_x, extra_y) = if unsafe { flag(sprite, S_FLAGS0, F_COORD_OFFSET_ENABLED) } {
            (offset_x, offset_y)
        } else {
            (0, 0)
        };

        let x = unsafe { i16_at(sprite, S_X) }
            .wrapping_add(unsafe { i16_at(sprite, S_X2) })
            .wrapping_add(i16::from(
                unsafe { u8_at(sprite, S_CENTER_TO_CORNER_X) } as i8
            ))
            .wrapping_add(extra_x);
        let y = unsafe { i16_at(sprite, S_Y) }
            .wrapping_add(unsafe { i16_at(sprite, S_Y2) })
            .wrapping_add(i16::from(
                unsafe { u8_at(sprite, S_CENTER_TO_CORNER_Y) } as i8
            ))
            .wrapping_add(extra_y);

        unsafe { set_oam_x(oam_of(sprite), x as u16) };
        unsafe { set_oam_y(oam_of(sprite), y as u16) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn BuildSpritePriorities() {
    for i in 0..MAX_SPRITES {
        let sprite = unsafe { sprite_at(i) };
        let priority = u16::from(unsafe { u8_at(sprite, S_SUBPRIORITY) })
            | (u16::from(unsafe { oam_priority(sprite) }) << 8);
        unsafe {
            (&raw mut SPRITE_PRIORITIES)
                .cast::<u16>()
                .add(i)
                .write_volatile(priority)
        };
    }
}

#[inline]
unsafe fn sprite_order(index: usize) -> u8 {
    unsafe {
        (&raw const SPRITE_ORDER)
            .cast::<u8>()
            .add(index)
            .read_volatile()
    }
}

#[inline]
unsafe fn set_sprite_order(index: usize, value: u8) {
    unsafe {
        (&raw mut SPRITE_ORDER)
            .cast::<u8>()
            .add(index)
            .write_volatile(value)
    };
}

#[inline]
unsafe fn sprite_priority(index: usize) -> u16 {
    unsafe {
        (&raw const SPRITE_PRIORITIES)
            .cast::<u16>()
            .add(index)
            .read_volatile()
    }
}

/// The Y used for sorting: OAM Y wraps at 256, and a double-size affine
/// sprite that is tall enough can be pushed further up still.
unsafe fn sort_y(sprite: *mut u8) -> i16 {
    let mut y = unsafe { oam_y(sprite) };
    if y >= DISPLAY_HEIGHT {
        y -= 256;
    }

    if unsafe { oam_affine_mode(sprite) } == ST_OAM_AFFINE_DOUBLE
        && unsafe { oam_size(sprite) } == ST_OAM_SIZE_3
    {
        let shape = unsafe { oam_shape(sprite) };
        if (shape == ST_OAM_SQUARE || shape == ST_OAM_V_RECTANGLE) && y > 128 {
            y -= 256;
        }
    }

    y
}

#[unsafe(no_mangle)]
pub unsafe fn SortSprites() {
    // Insertion sort by priority, then by screen position.
    for i in 1..MAX_SPRITES {
        let mut j = i;
        loop {
            if j == 0 {
                break;
            }
            let first = unsafe { sprite_at(sprite_order(j - 1) as usize) };
            let second = unsafe { sprite_at(sprite_order(j) as usize) };
            let first_priority = unsafe { sprite_priority(sprite_order(j - 1) as usize) };
            let second_priority = unsafe { sprite_priority(sprite_order(j) as usize) };
            let first_y = unsafe { sort_y(first) };
            let second_y = unsafe { sort_y(second) };

            if !(first_priority > second_priority
                || (first_priority == second_priority && first_y < second_y))
            {
                break;
            }

            let temp = unsafe { sprite_order(j) };
            unsafe { set_sprite_order(j, sprite_order(j - 1)) };
            unsafe { set_sprite_order(j - 1, temp) };
            j -= 1;
            // The original reads sSpriteOrder[-1] once here when j reaches 0,
            // then immediately leaves the loop because of the j > 0 test, so
            // the values are never used. Breaking early is equivalent and
            // avoids the out-of-bounds read.
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe fn CopyMatricesToOamBuffer() {
    // Each matrix component rides in the affineParam of four OAM entries.
    for i in 0..OAM_MATRIX_COUNT {
        let source = unsafe { matrix(i) };
        for component in 0..4 {
            let entry = unsafe { oam_buffer_entry(4 * i + component) };
            let value = unsafe { source.add(component).read_volatile() } as u16;
            unsafe { entry.add(6).cast::<u16>().write_volatile(value) };
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe fn AddSpritesToOamBuffer() {
    let mut oam_index = 0u8;

    let mut i = 0usize;
    while i < MAX_SPRITES {
        let sprite = unsafe { sprite_at(sprite_order(i) as usize) };
        if unsafe { flag(sprite, S_FLAGS0, F_IN_USE) }
            && !unsafe { flag(sprite, S_FLAGS0, F_INVISIBLE) }
            && unsafe { AddSpriteToOamBuffer(sprite, &raw mut oam_index) } != 0
        {
            return;
        }
        i += 1;
    }

    // Park the unused entries off-screen.
    let limit = unsafe { (&raw const gOamLimit).read_volatile() };
    while oam_index < limit {
        unsafe {
            core::ptr::copy_nonoverlapping(
                gDummyOamData.0.as_ptr(),
                oam_buffer_entry(oam_index as usize),
                8,
            )
        };
        oam_index += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe fn CreateSprite(template: *const SpriteTemplate, x: i16, y: i16, subpriority: u8) -> u8 {
    for i in 0..MAX_SPRITES {
        if !unsafe { flag(sprite_at(i), S_FLAGS0, F_IN_USE) } {
            return unsafe { create_sprite_at(i as u8, template, x, y, subpriority) };
        }
    }
    MAX_SPRITES as u8
}

#[unsafe(no_mangle)]
pub unsafe fn CreateSpriteAtEnd(
    template: *const SpriteTemplate,
    x: i16,
    y: i16,
    subpriority: u8,
) -> u8 {
    let mut i = MAX_SPRITES as i32 - 1;
    while i > -1 {
        if !unsafe { flag(sprite_at(i as usize), S_FLAGS0, F_IN_USE) } {
            return unsafe { create_sprite_at(i as u8, template, x, y, subpriority) };
        }
        i -= 1;
    }
    MAX_SPRITES as u8
}

#[unsafe(no_mangle)]
pub unsafe fn CreateInvisibleSprite(callback: SpriteCallback) -> u8 {
    let index = unsafe { CreateSprite(&raw const gDummySpriteTemplate, 0, 0, 31) };
    if index == MAX_SPRITES as u8 {
        return MAX_SPRITES as u8;
    }

    let sprite = unsafe { sprite_at(index as usize) };
    unsafe { set_flag(sprite, S_FLAGS0, F_INVISIBLE, true) };
    unsafe {
        sprite
            .add(S_CALLBACK)
            .cast::<SpriteCallback>()
            .write(callback)
    };
    index
}

unsafe fn create_sprite_at(
    index: u8,
    template: *const SpriteTemplate,
    x: i16,
    y: i16,
    subpriority: u8,
) -> u8 {
    let sprite = unsafe { sprite_at(index as usize) };
    unsafe { reset_sprite(sprite) };

    unsafe { set_flag(sprite, S_FLAGS0, F_IN_USE, true) };
    unsafe { set_flag(sprite, S_FLAGS1, F_ANIM_BEGINNING, true) };
    unsafe { set_flag(sprite, S_FLAGS1, F_AFFINE_ANIM_BEGINNING, true) };
    unsafe { set_flag(sprite, S_FLAGS1, F_USING_SHEET, true) };

    unsafe { set_u8_at(sprite, S_SUBPRIORITY, subpriority) };
    unsafe {
        core::ptr::copy_nonoverlapping(
            (&raw const (*template).oam)
                .cast::<*const OamData>()
                .read()
                .cast::<u8>(),
            oam_of(sprite),
            8,
        )
    };
    unsafe {
        set_ptr_at(
            sprite,
            S_ANIMS,
            (&raw const (*template).anims).cast::<*const u8>().read(),
        )
    };
    unsafe {
        set_ptr_at(
            sprite,
            S_AFFINE_ANIMS,
            (&raw const (*template).affine_anims)
                .cast::<*const u8>()
                .read(),
        )
    };
    unsafe { set_ptr_at(sprite, S_TEMPLATE, template.cast()) };
    unsafe {
        sprite
            .add(S_CALLBACK)
            .cast::<SpriteCallback>()
            .write((&raw const (*template).callback).read())
    };
    unsafe { set_i16_at(sprite, S_X, x) };
    unsafe { set_i16_at(sprite, S_Y, y) };

    unsafe {
        CalcCenterToCornerVec(
            sprite,
            oam_shape(sprite),
            oam_size(sprite),
            oam_affine_mode(sprite),
        )
    };

    let tile_tag = unsafe { (&raw const (*template).tile_tag).read() };
    if tile_tag == TAG_NONE {
        let images = unsafe { (&raw const (*template).images).read() };
        unsafe { set_ptr_at(sprite, S_IMAGES, images) };
        let image_size = unsafe { images.add(IMAGE_SIZE).cast::<u16>().read() };
        let tile_num = unsafe { AllocSpriteTiles((image_size / TILE_SIZE_4BPP) as u8 as u16) };
        if tile_num == -1 {
            unsafe { reset_sprite(sprite) };
            return MAX_SPRITES as u8;
        }
        unsafe { set_oam_tile_num(oam_of(sprite), tile_num as u16) };
        unsafe { set_flag(sprite, S_FLAGS1, F_USING_SHEET, false) };
        unsafe { set_u16_at(sprite, S_SHEET_TILE_START, 0) };
    } else {
        unsafe {
            set_u16_at(
                sprite,
                S_SHEET_TILE_START,
                GetSpriteTileStartByTag(tile_tag),
            )
        };
        unsafe { SetSpriteSheetFrameTileNum(sprite) };
    }

    if unsafe { oam_affine_mode(sprite) } & ST_OAM_AFFINE_ON_MASK != 0 {
        unsafe { InitSpriteAffineAnim(sprite) };
    }

    let palette_tag = unsafe { (&raw const (*template).palette_tag).read() };
    if palette_tag != TAG_NONE {
        unsafe { set_oam_palette_num(oam_of(sprite), IndexOfSpritePaletteTag(palette_tag)) };
    }

    index
}

#[unsafe(no_mangle)]
pub unsafe fn CreateSpriteAndAnimate(
    template: *const SpriteTemplate,
    x: i16,
    y: i16,
    subpriority: u8,
) -> u8 {
    for i in 0..MAX_SPRITES {
        let sprite = unsafe { sprite_at(i) };
        if unsafe { flag(sprite, S_FLAGS0, F_IN_USE) } {
            continue;
        }

        let index = unsafe { create_sprite_at(i as u8, template, x, y, subpriority) };
        if index == MAX_SPRITES as u8 {
            return MAX_SPRITES as u8;
        }

        let callback = unsafe { sprite.add(S_CALLBACK).cast::<SpriteCallback>().read() };
        unsafe { callback(sprite) };
        if unsafe { flag(sprite, S_FLAGS0, F_IN_USE) } {
            unsafe { AnimateSprite(sprite) };
        }
        return index;
    }

    MAX_SPRITES as u8
}

#[unsafe(no_mangle)]
pub unsafe fn DestroySprite(sprite: *mut u8) {
    if !unsafe { flag(sprite, S_FLAGS0, F_IN_USE) } {
        return;
    }

    if !unsafe { flag(sprite, S_FLAGS1, F_USING_SHEET) } {
        let images = unsafe { ptr_at(sprite, S_IMAGES) };
        let image_size = unsafe { images.add(IMAGE_SIZE).cast::<u16>().read() };
        let start = unsafe { oam_tile_num(sprite) };
        let end = (image_size / TILE_SIZE_4BPP).wrapping_add(start);
        let mut i = start;
        while i < end {
            unsafe { free_tile(i) };
            i += 1;
        }
    }

    unsafe { reset_sprite(sprite) };
}

#[unsafe(no_mangle)]
pub unsafe fn ResetOamRange(start: u8, end: u8) {
    let mut i = start;
    while i < end {
        unsafe {
            core::ptr::copy_nonoverlapping(
                gDummyOamData.0.as_ptr(),
                oam_buffer_entry(i as usize),
                8,
            )
        };
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe fn LoadOam() {
    let disabled = unsafe {
        (&raw const (*(&raw const crate::agb_main::gMain).cast::<u8>().cast_mut()))
            .add(MAIN_OAM_LOAD_DISABLED_BYTE)
            .read_volatile()
    };
    if disabled & MAIN_OAM_LOAD_DISABLED_BIT == 0 {
        unsafe { cpu_copy32(oam_buffer_entry(0), OAM as *mut u8, OAM_BUFFER_BYTES as u32) };
    }
}

#[inline]
unsafe fn copy_request(index: usize) -> *mut u8 {
    unsafe {
        (&raw mut SPRITE_COPY_REQUESTS)
            .cast::<SpriteCopyRequestStorage>()
            .cast::<u8>()
            .add(index * COPY_REQUEST_STRIDE)
    }
}

#[unsafe(no_mangle)]
pub unsafe fn ClearSpriteCopyRequests() {
    unsafe { (SHOULD_PROCESS_SPRITE_COPY_REQUESTS.as_ptr()).write_volatile(0) };
    unsafe { (SPRITE_COPY_REQUEST_COUNT.as_ptr()).write_volatile(0) };

    for i in 0..MAX_SPRITE_COPY_REQUESTS {
        unsafe { core::ptr::write_bytes(copy_request(i), 0, COPY_REQUEST_STRIDE) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn ResetOamMatrices() {
    for i in 0..OAM_MATRIX_COUNT {
        let slot = unsafe { matrix(i) };
        // The identity matrix in 8.8 fixed point.
        unsafe { slot.write_volatile(0x0100) };
        unsafe { slot.add(1).write_volatile(0) };
        unsafe { slot.add(2).write_volatile(0) };
        unsafe { slot.add(3).write_volatile(0x0100) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn SetOamMatrix(matrix_num: u8, a: u16, b: u16, c: u16, d: u16) {
    let slot = unsafe { matrix(matrix_num as usize) };
    unsafe { slot.write_volatile(a as i16) };
    unsafe { slot.add(1).write_volatile(b as i16) };
    unsafe { slot.add(2).write_volatile(c as i16) };
    unsafe { slot.add(3).write_volatile(d as i16) };
}

/// `*sprite = sDummySprite`. The dummy is written out field by field rather
/// than copied from a static, because a static `struct Sprite` would need
/// relocations inside a fixed 68-byte ARM layout.
unsafe fn reset_sprite(sprite: *mut u8) {
    unsafe { core::ptr::write_bytes(sprite, 0, SPRITE_SIZE) };
    unsafe { core::ptr::copy_nonoverlapping(gDummyOamData.0.as_ptr(), oam_of(sprite), 8) };
    unsafe { set_ptr_at(sprite, S_ANIMS, (&raw const gDummySpriteAnimTable).cast()) };
    unsafe {
        set_ptr_at(
            sprite,
            S_AFFINE_ANIMS,
            (&raw const gDummySpriteAffineAnimTable).cast(),
        )
    };
    unsafe { set_ptr_at(sprite, S_TEMPLATE, (&raw const gDummySpriteTemplate).cast()) };
    unsafe {
        sprite
            .add(S_CALLBACK)
            .cast::<SpriteCallback>()
            .write(SpriteCallbackDummy)
    };
    unsafe { set_i16_at(sprite, S_X, DISPLAY_WIDTH + 64) };
    unsafe { set_i16_at(sprite, S_Y, DISPLAY_HEIGHT) };
    unsafe { set_u8_at(sprite, S_SUBPRIORITY, 0xff) };
}

#[unsafe(no_mangle)]
pub unsafe fn ResetSprite(sprite: *mut u8) {
    unsafe { reset_sprite(sprite) };
}

unsafe fn reset_all_sprites() {
    for i in 0..MAX_SPRITES {
        unsafe { reset_sprite(sprite_at(i)) };
        unsafe { set_sprite_order(i, i as u8) };
    }
    // The spare slot past the end is reset too.
    unsafe { reset_sprite(sprite_at(MAX_SPRITES)) };
}

#[unsafe(no_mangle)]
pub unsafe fn CalcCenterToCornerVec(sprite: *mut u8, shape: u8, size: u8, affine_mode: u8) {
    let entry = CENTER_TO_CORNER_VEC_TABLE[(shape & 3).min(2) as usize][(size & 3) as usize];
    // The originals are negative values held in a u8 and doubled there, so
    // the doubling wraps rather than sign-extending.
    let (mut x, mut y) = (entry[0], entry[1]);
    if affine_mode & ST_OAM_AFFINE_DOUBLE_MASK != 0 {
        x = x.wrapping_mul(2);
        y = y.wrapping_mul(2);
    }

    unsafe { set_u8_at(sprite, S_CENTER_TO_CORNER_X, x) };
    unsafe { set_u8_at(sprite, S_CENTER_TO_CORNER_Y, y) };
}

#[unsafe(no_mangle)]
pub unsafe fn AllocSpriteTiles(tile_count: u16) -> i16 {
    let reserved = unsafe { (&raw const gReservedSpriteTileCount).read_volatile() };

    if tile_count == 0 {
        // Free every unreserved tile.
        let mut i = reserved;
        while i < TOTAL_OBJ_TILE_COUNT {
            unsafe { free_tile(i) };
            i += 1;
        }
        return 0;
    }

    let mut i = reserved;
    let start;
    loop {
        while unsafe { tile_is_allocated(i) } {
            i += 1;
            if i == TOTAL_OBJ_TILE_COUNT {
                return -1;
            }
        }

        let candidate = i;
        let mut found = 1u16;
        while found != tile_count {
            i += 1;
            if i == TOTAL_OBJ_TILE_COUNT {
                return -1;
            }
            if !unsafe { tile_is_allocated(i) } {
                found += 1;
            } else {
                break;
            }
        }

        if found == tile_count {
            start = candidate;
            break;
        }
    }

    let mut i = start;
    while i < tile_count + start {
        unsafe { alloc_tile(i) };
        i += 1;
    }

    start as i16
}

#[unsafe(no_mangle)]
pub unsafe fn SpriteTileAllocBitmapOp(bit: u16, op: u8) -> u8 {
    let index = (bit / 8) as usize;
    let shift = (bit % 8) as u8;
    let slot = unsafe { (&raw mut SPRITE_TILE_ALLOC_BITMAP).cast::<u8>().add(index) };

    match op {
        0 => {
            unsafe { slot.write_volatile(slot.read_volatile() & !(1u8 << shift)) };
            0
        }
        1 => {
            unsafe { slot.write_volatile(slot.read_volatile() | (1u8 << shift)) };
            0
        }
        _ => (1u8 << shift) & unsafe { slot.read_volatile() },
    }
}

#[unsafe(no_mangle)]
pub unsafe fn ProcessSpriteCopyRequests() {
    if unsafe { (SHOULD_PROCESS_SPRITE_COPY_REQUESTS.as_ptr().cast_const()).read_volatile() } == 0 {
        return;
    }

    let mut i = 0usize;
    while unsafe { (SPRITE_COPY_REQUEST_COUNT.as_ptr().cast_const()).read_volatile() } > 0 {
        let request = unsafe { copy_request(i) };
        unsafe {
            cpu_copy16(
                request.add(COPY_SRC).cast::<*const u8>().read(),
                request.add(COPY_DEST).cast::<*mut u8>().read(),
                request.add(COPY_SIZE).cast::<u16>().read(),
            )
        };
        let count = unsafe { (SPRITE_COPY_REQUEST_COUNT.as_ptr().cast_const()).read_volatile() };
        unsafe { (SPRITE_COPY_REQUEST_COUNT.as_ptr()).write_volatile(count - 1) };
        i += 1;
    }

    unsafe { (SHOULD_PROCESS_SPRITE_COPY_REQUESTS.as_ptr()).write_volatile(0) };
}

unsafe fn request_sprite_frame_image_copy(index: u16, tile_num: u16, images: *const u8) {
    let count = unsafe { (SPRITE_COPY_REQUEST_COUNT.as_ptr().cast_const()).read_volatile() };
    if count as usize >= MAX_SPRITE_COPY_REQUESTS {
        return;
    }

    let image = unsafe { images.add(index as usize * SPRITE_FRAME_IMAGE_STRIDE) };
    let request = unsafe { copy_request(count as usize) };
    unsafe {
        request
            .add(COPY_SRC)
            .cast::<*const u8>()
            .write(image.add(IMAGE_DATA).cast::<*const u8>().read())
    };
    unsafe {
        request
            .add(COPY_DEST)
            .cast::<*mut u8>()
            .write((OBJ_VRAM0 + TILE_SIZE_4BPP as usize * tile_num as usize) as *mut u8)
    };
    unsafe {
        request
            .add(COPY_SIZE)
            .cast::<u16>()
            .write(image.add(IMAGE_SIZE).cast::<u16>().read())
    };
    unsafe { (SPRITE_COPY_REQUEST_COUNT.as_ptr()).write_volatile(count + 1) };
}

#[unsafe(no_mangle)]
pub unsafe fn RequestSpriteCopy(src: *const u8, dest: *mut u8, size: u16) {
    let count = unsafe { (SPRITE_COPY_REQUEST_COUNT.as_ptr().cast_const()).read_volatile() };
    if count as usize >= MAX_SPRITE_COPY_REQUESTS {
        return;
    }

    let request = unsafe { copy_request(count as usize) };
    unsafe { request.add(COPY_SRC).cast::<*const u8>().write(src) };
    unsafe { request.add(COPY_DEST).cast::<*mut u8>().write(dest) };
    unsafe { request.add(COPY_SIZE).cast::<u16>().write(size) };
    unsafe { (SPRITE_COPY_REQUEST_COUNT.as_ptr()).write_volatile(count + 1) };
}

#[unsafe(no_mangle)]
pub unsafe fn CopyFromSprites(dest: *mut u8) {
    unsafe { core::ptr::copy_nonoverlapping(sprite_at(0), dest, SPRITE_SIZE * MAX_SPRITES) };
}

#[unsafe(no_mangle)]
pub unsafe fn CopyToSprites(src: *const u8) {
    unsafe { core::ptr::copy_nonoverlapping(src, sprite_at(0), SPRITE_SIZE * MAX_SPRITES) };
}

#[unsafe(no_mangle)]
pub unsafe fn ResetAllSprites() {
    unsafe { reset_all_sprites() };
}

#[unsafe(no_mangle)]
pub unsafe fn FreeSpriteTiles(sprite: *mut u8) {
    let template = unsafe { ptr_at(sprite, S_TEMPLATE) };
    let tile_tag = unsafe { template.cast::<u16>().read() };
    if tile_tag != TAG_NONE {
        unsafe { FreeSpriteTilesByTag(tile_tag) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn FreeSpritePalette(sprite: *mut u8) {
    let template = unsafe { ptr_at(sprite, S_TEMPLATE) };
    let palette_tag = unsafe { template.add(2).cast::<u16>().read() };
    unsafe { FreeSpritePaletteByTag(palette_tag) };
}

#[unsafe(no_mangle)]
pub unsafe fn FreeSpriteOamMatrix(sprite: *mut u8) {
    if unsafe { oam_affine_mode(sprite) } & ST_OAM_AFFINE_ON_MASK != 0 {
        unsafe { FreeOamMatrix(oam_matrix_num(sprite)) };
        let oam = unsafe { oam_of(sprite) };
        let attr = unsafe { oam_attr(oam, 0) };
        unsafe {
            oam.cast::<u16>()
                .write_volatile((attr & !(0x3 << 8)) | (ST_OAM_AFFINE_OFF << 8))
        };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn DestroySpriteAndFreeResources(sprite: *mut u8) {
    unsafe { FreeSpriteTiles(sprite) };
    unsafe { FreeSpritePalette(sprite) };
    unsafe { FreeSpriteOamMatrix(sprite) };
    unsafe { DestroySprite(sprite) };
}

#[unsafe(no_mangle)]
pub unsafe fn AnimateSprite(sprite: *mut u8) {
    if unsafe { flag(sprite, S_FLAGS1, F_ANIM_BEGINNING) } {
        unsafe { begin_anim(sprite) };
    } else {
        unsafe { continue_anim(sprite) };
    }

    if unsafe { (&raw const gAffineAnimsDisabled).read_volatile() } != 0 {
        return;
    }

    if unsafe { flag(sprite, S_FLAGS1, F_AFFINE_ANIM_BEGINNING) } {
        unsafe { begin_affine_anim(sprite) };
    } else {
        unsafe { continue_affine_anim(sprite) };
    }
}

/// `sprite->anims[animNum][animCmdIndex]` as its raw 32-bit word.
#[inline]
unsafe fn anim_cmd(sprite: *mut u8) -> u32 {
    let table = unsafe { ptr_at(sprite, S_ANIMS) };
    let anim_num = unsafe { u8_at(sprite, S_ANIM_NUM) } as usize;
    let anim = unsafe { table.cast::<*const u32>().add(anim_num).read() };
    let index = unsafe { u8_at(sprite, S_ANIM_CMD_INDEX) } as usize;
    unsafe { anim.add(index).read() }
}

/// The same, at an arbitrary index offset from the current one.
#[inline]
unsafe fn anim_cmd_at(sprite: *mut u8, index: i32) -> u32 {
    let table = unsafe { ptr_at(sprite, S_ANIMS) };
    let anim_num = unsafe { u8_at(sprite, S_ANIM_NUM) } as usize;
    let anim = unsafe { table.cast::<*const u32>().add(anim_num).read() };
    unsafe { anim.offset(index as isize).read() }
}

#[inline]
fn anim_image_value(cmd: u32) -> i16 {
    (cmd & ANIM_IMAGE_VALUE_MASK) as u16 as i16
}

#[inline]
fn anim_duration(cmd: u32) -> u8 {
    ((cmd & ANIM_DURATION_MASK) >> ANIM_DURATION_SHIFT) as u8
}

#[inline]
fn anim_h_flip(cmd: u32) -> u8 {
    u8::from(cmd & ANIM_H_FLIP != 0)
}

#[inline]
fn anim_v_flip(cmd: u32) -> u8 {
    u8::from(cmd & ANIM_V_FLIP != 0)
}

#[inline]
unsafe fn set_anim_delay_counter(sprite: *mut u8, value: u8) {
    let state = unsafe { u8_at(sprite, S_ANIM_STATE) };
    unsafe {
        set_u8_at(
            sprite,
            S_ANIM_STATE,
            (state & !ANIM_DELAY_MASK) | (value & ANIM_DELAY_MASK),
        )
    };
}

#[inline]
unsafe fn anim_delay_counter(sprite: *mut u8) -> u8 {
    let state = unsafe { u8_at(sprite, S_ANIM_STATE) };
    state & ANIM_DELAY_MASK
}

/// Applies one animation frame: sets the delay, the flip bits and either the
/// sheet tile or a pending image copy.
unsafe fn apply_anim_frame(sprite: *mut u8, cmd: u32) {
    let image_value = anim_image_value(cmd);
    let mut duration = anim_duration(cmd);
    if duration != 0 {
        duration -= 1;
    }
    unsafe { set_anim_delay_counter(sprite, duration) };

    if unsafe { oam_affine_mode(sprite) } & ST_OAM_AFFINE_ON_MASK == 0 {
        unsafe { SetSpriteOamFlipBits(sprite, anim_h_flip(cmd), anim_v_flip(cmd)) };
    }

    if unsafe { flag(sprite, S_FLAGS1, F_USING_SHEET) } {
        let start = unsafe { u16_at(sprite, S_SHEET_TILE_START) };
        unsafe { set_oam_tile_num(oam_of(sprite), start.wrapping_add(image_value as u16)) };
    } else {
        unsafe {
            request_sprite_frame_image_copy(
                image_value as u16,
                oam_tile_num(sprite),
                ptr_at(sprite, S_IMAGES),
            )
        };
    }
}

unsafe fn begin_anim(sprite: *mut u8) {
    unsafe { set_u8_at(sprite, S_ANIM_CMD_INDEX, 0) };
    unsafe { set_flag(sprite, S_FLAGS1, F_ANIM_ENDED, false) };
    unsafe { set_u8_at(sprite, S_ANIM_LOOP_COUNTER, 0) };

    let cmd = unsafe { anim_cmd(sprite) };
    if anim_image_value(cmd) == -1 {
        return;
    }

    unsafe { set_flag(sprite, S_FLAGS1, F_ANIM_BEGINNING, false) };
    unsafe { apply_anim_frame(sprite, cmd) };
}

unsafe fn continue_anim(sprite: *mut u8) {
    if unsafe { anim_delay_counter(sprite) } != 0 {
        if !unsafe { flag(sprite, S_ANIM_STATE, ANIM_PAUSED_BIT) } {
            unsafe { set_anim_delay_counter(sprite, anim_delay_counter(sprite).wrapping_sub(1)) };
        }
        let cmd = unsafe { anim_cmd(sprite) };
        if unsafe { oam_affine_mode(sprite) } & ST_OAM_AFFINE_ON_MASK == 0 {
            unsafe { SetSpriteOamFlipBits(sprite, anim_h_flip(cmd), anim_v_flip(cmd)) };
        }
        return;
    }

    if unsafe { flag(sprite, S_ANIM_STATE, ANIM_PAUSED_BIT) } {
        return;
    }

    let index = unsafe { u8_at(sprite, S_ANIM_CMD_INDEX) };
    unsafe { set_u8_at(sprite, S_ANIM_CMD_INDEX, index.wrapping_add(1)) };

    // Negative types select loop/jump/end; anything else is a frame.
    let cmd_type = unsafe { anim_cmd_at(sprite, i32::from(u8_at(sprite, S_ANIM_CMD_INDEX))) };
    let cmd_type = (cmd_type & 0xffff) as u16 as i16;
    match cmd_type {
        -3 => unsafe { anim_cmd_loop(sprite) },
        -2 => unsafe { anim_cmd_jump(sprite) },
        -1 => unsafe { anim_cmd_end(sprite) },
        _ => unsafe { anim_cmd_frame(sprite) },
    }
}

unsafe fn anim_cmd_frame(sprite: *mut u8) {
    let cmd = unsafe { anim_cmd(sprite) };
    unsafe { apply_anim_frame(sprite, cmd) };
}

unsafe fn anim_cmd_end(sprite: *mut u8) {
    let index = unsafe { u8_at(sprite, S_ANIM_CMD_INDEX) };
    unsafe { set_u8_at(sprite, S_ANIM_CMD_INDEX, index.wrapping_sub(1)) };
    unsafe { set_flag(sprite, S_FLAGS1, F_ANIM_ENDED, true) };
}

unsafe fn anim_cmd_jump(sprite: *mut u8) {
    let cmd = unsafe { anim_cmd(sprite) };
    let target = ((cmd & ANIM_DURATION_MASK) >> ANIM_DURATION_SHIFT) as u8;
    unsafe { set_u8_at(sprite, S_ANIM_CMD_INDEX, target) };
    let cmd = unsafe { anim_cmd(sprite) };
    unsafe { apply_anim_frame(sprite, cmd) };
}

unsafe fn anim_cmd_loop(sprite: *mut u8) {
    if unsafe { u8_at(sprite, S_ANIM_LOOP_COUNTER) } != 0 {
        let counter = unsafe { u8_at(sprite, S_ANIM_LOOP_COUNTER) };
        unsafe { set_u8_at(sprite, S_ANIM_LOOP_COUNTER, counter.wrapping_sub(1)) };
    } else {
        let cmd = unsafe { anim_cmd(sprite) };
        let count = ((cmd & ANIM_DURATION_MASK) >> ANIM_DURATION_SHIFT) as u8;
        unsafe { set_u8_at(sprite, S_ANIM_LOOP_COUNTER, count) };
    }
    unsafe { jump_to_top_of_anim_loop(sprite) };
    unsafe { continue_anim(sprite) };
}

/// Walks back to the command after the previous loop marker.
unsafe fn jump_to_top_of_anim_loop(sprite: *mut u8) {
    if unsafe { u8_at(sprite, S_ANIM_LOOP_COUNTER) } == 0 {
        return;
    }

    let mut index = unsafe { u8_at(sprite, S_ANIM_CMD_INDEX) }.wrapping_sub(1);
    unsafe { set_u8_at(sprite, S_ANIM_CMD_INDEX, index) };

    loop {
        let previous = unsafe { anim_cmd_at(sprite, i32::from(index) - 1) };
        if ((previous & 0xffff) as u16 as i16) == -3 {
            break;
        }
        if index == 0 {
            break;
        }
        index = index.wrapping_sub(1);
        unsafe { set_u8_at(sprite, S_ANIM_CMD_INDEX, index) };
    }

    unsafe { set_u8_at(sprite, S_ANIM_CMD_INDEX, index.wrapping_sub(1)) };
}

// --------------------------------------------------------- affine anims

#[inline]
unsafe fn affine_cmd(sprite: *mut u8, matrix_num: u8) -> *const u8 {
    let table = unsafe { ptr_at(sprite, S_AFFINE_ANIMS) };
    let state = unsafe { affine_state(matrix_num) };
    let anim_num = unsafe { state.add(AAS_ANIM_NUM).read_volatile() } as usize;
    let anim = unsafe { table.cast::<*const u8>().add(anim_num).read() };
    let index = unsafe { state.add(AAS_ANIM_CMD_INDEX).read_volatile() } as usize;
    unsafe { anim.add(index * AFFINE_CMD_SIZE) }
}

/// Four s16 fields of an affine frame command.
struct AffineFrame {
    x_scale: i16,
    y_scale: i16,
    rotation: u8,
    duration: u8,
}

unsafe fn begin_affine_anim(sprite: *mut u8) {
    if unsafe { oam_affine_mode(sprite) } & ST_OAM_AFFINE_ON_MASK == 0 {
        return;
    }

    let table = unsafe { ptr_at(sprite, S_AFFINE_ANIMS) };
    let first = unsafe { table.cast::<*const u8>().read() };
    if unsafe { first.cast::<i16>().read() } == AFFINE_ANIM_END {
        return;
    }

    let matrix_num = unsafe { GetSpriteMatrixNum(sprite) };
    unsafe { affine_anim_state_restart(matrix_num) };
    let frame = unsafe { get_affine_anim_frame(matrix_num, sprite) };
    unsafe { set_flag(sprite, S_FLAGS1, F_AFFINE_ANIM_BEGINNING, false) };
    unsafe { set_flag(sprite, S_FLAGS1, F_AFFINE_ANIM_ENDED, false) };
    // The delay is the duration *after* ApplyAffineAnimFrame decremented it:
    // the original passes the command by pointer.
    let duration = unsafe { apply_affine_anim_frame(matrix_num, frame) };
    unsafe {
        affine_state(matrix_num)
            .add(AAS_DELAY_COUNTER)
            .write_volatile(duration)
    };

    if unsafe { flag(sprite, S_FLAGS1, F_ANCHORED) } {
        unsafe { update_sprite_matrix_anchor_pos(sprite) };
    }
}

unsafe fn continue_affine_anim(sprite: *mut u8) {
    if unsafe { oam_affine_mode(sprite) } & ST_OAM_AFFINE_ON_MASK == 0 {
        return;
    }

    let matrix_num = unsafe { GetSpriteMatrixNum(sprite) };
    let state = unsafe { affine_state(matrix_num) };

    if unsafe { state.add(AAS_DELAY_COUNTER).read_volatile() } != 0 {
        unsafe { affine_anim_delay(matrix_num, sprite) };
    } else if unsafe { flag(sprite, S_ANIM_STATE, AFFINE_ANIM_PAUSED_BIT) } {
        return;
    } else {
        let index = unsafe { state.add(AAS_ANIM_CMD_INDEX).read_volatile() };
        unsafe {
            state
                .add(AAS_ANIM_CMD_INDEX)
                .write_volatile(index.wrapping_add(1))
        };
        let cmd_type = unsafe { affine_cmd(sprite, matrix_num).cast::<u16>().read() };
        // 32765..32767 select loop/jump/end; anything else is a frame.
        match cmd_type {
            32765 => unsafe { affine_anim_cmd_loop(matrix_num, sprite) },
            32766 => unsafe { affine_anim_cmd_jump(matrix_num, sprite) },
            32767 => unsafe { affine_anim_cmd_end(matrix_num, sprite) },
            _ => unsafe { affine_anim_cmd_frame(matrix_num, sprite) },
        }
    }

    if unsafe { flag(sprite, S_FLAGS1, F_ANCHORED) } {
        unsafe { update_sprite_matrix_anchor_pos(sprite) };
    }
}

unsafe fn affine_anim_delay(matrix_num: u8, sprite: *mut u8) {
    let paused = unsafe { flag(sprite, S_ANIM_STATE, AFFINE_ANIM_PAUSED_BIT) };
    if !paused {
        let state = unsafe { affine_state(matrix_num) };
        let counter = unsafe { state.add(AAS_DELAY_COUNTER).read_volatile() };
        unsafe {
            state
                .add(AAS_DELAY_COUNTER)
                .write_volatile(counter.wrapping_sub(1))
        };
    }

    if !paused {
        let frame = unsafe { get_affine_anim_frame(matrix_num, sprite) };
        unsafe { apply_affine_anim_frame_relative(matrix_num, &frame) };
    }
}

unsafe fn affine_anim_cmd_loop(matrix_num: u8, sprite: *mut u8) {
    let state = unsafe { affine_state(matrix_num) };
    if unsafe { state.add(AAS_LOOP_COUNTER).read_volatile() } != 0 {
        let counter = unsafe { state.add(AAS_LOOP_COUNTER).read_volatile() };
        unsafe {
            state
                .add(AAS_LOOP_COUNTER)
                .write_volatile(counter.wrapping_sub(1))
        };
    } else {
        let count = unsafe {
            affine_cmd(sprite, matrix_num)
                .add(AFFINE_COUNT)
                .cast::<i16>()
                .read()
        };
        unsafe { state.add(AAS_LOOP_COUNTER).write_volatile(count as u8) };
    }
    unsafe { jump_to_top_of_affine_anim_loop(matrix_num, sprite) };
    unsafe { continue_affine_anim(sprite) };
}

unsafe fn jump_to_top_of_affine_anim_loop(matrix_num: u8, sprite: *mut u8) {
    let state = unsafe { affine_state(matrix_num) };
    if unsafe { state.add(AAS_LOOP_COUNTER).read_volatile() } == 0 {
        return;
    }

    let mut index = unsafe { state.add(AAS_ANIM_CMD_INDEX).read_volatile() }.wrapping_sub(1);
    unsafe { state.add(AAS_ANIM_CMD_INDEX).write_volatile(index) };

    loop {
        let table = unsafe { ptr_at(sprite, S_AFFINE_ANIMS) };
        let anim_num = unsafe { state.add(AAS_ANIM_NUM).read_volatile() } as usize;
        let anim = unsafe { table.cast::<*const u8>().add(anim_num).read() };
        let previous = unsafe {
            anim.add((index as usize).wrapping_sub(1) * AFFINE_CMD_SIZE)
                .cast::<u16>()
                .read()
        };
        if previous == 32765 {
            break;
        }
        if index == 0 {
            break;
        }
        index = index.wrapping_sub(1);
        unsafe { state.add(AAS_ANIM_CMD_INDEX).write_volatile(index) };
    }

    unsafe {
        state
            .add(AAS_ANIM_CMD_INDEX)
            .write_volatile(index.wrapping_sub(1))
    };
}

unsafe fn affine_anim_cmd_jump(matrix_num: u8, sprite: *mut u8) {
    let target = unsafe {
        affine_cmd(sprite, matrix_num)
            .add(AFFINE_TARGET)
            .cast::<u16>()
            .read()
    };
    let state = unsafe { affine_state(matrix_num) };
    unsafe { state.add(AAS_ANIM_CMD_INDEX).write_volatile(target as u8) };

    let frame = unsafe { get_affine_anim_frame(matrix_num, sprite) };
    // The delay is the duration *after* ApplyAffineAnimFrame decremented it:
    // the original passes the command by pointer.
    let duration = unsafe { apply_affine_anim_frame(matrix_num, frame) };
    unsafe { state.add(AAS_DELAY_COUNTER).write_volatile(duration) };
}

unsafe fn affine_anim_cmd_end(matrix_num: u8, sprite: *mut u8) {
    unsafe { set_flag(sprite, S_FLAGS1, F_AFFINE_ANIM_ENDED, true) };
    let state = unsafe { affine_state(matrix_num) };
    let index = unsafe { state.add(AAS_ANIM_CMD_INDEX).read_volatile() };
    unsafe {
        state
            .add(AAS_ANIM_CMD_INDEX)
            .write_volatile(index.wrapping_sub(1))
    };

    let dummy = AffineFrame {
        x_scale: 0,
        y_scale: 0,
        rotation: 0,
        duration: 0,
    };
    unsafe { apply_affine_anim_frame_relative(matrix_num, &dummy) };
}

unsafe fn affine_anim_cmd_frame(matrix_num: u8, sprite: *mut u8) {
    let frame = unsafe { get_affine_anim_frame(matrix_num, sprite) };
    // The delay is the duration *after* ApplyAffineAnimFrame decremented it:
    // the original passes the command by pointer.
    let duration = unsafe { apply_affine_anim_frame(matrix_num, frame) };
    unsafe {
        affine_state(matrix_num)
            .add(AAS_DELAY_COUNTER)
            .write_volatile(duration)
    };
}

unsafe fn get_affine_anim_frame(matrix_num: u8, sprite: *mut u8) -> AffineFrame {
    let cmd = unsafe { affine_cmd(sprite, matrix_num) };
    AffineFrame {
        x_scale: unsafe { cmd.add(AFFINE_X_SCALE).cast::<i16>().read() },
        y_scale: unsafe { cmd.add(AFFINE_Y_SCALE).cast::<i16>().read() },
        rotation: unsafe { cmd.add(AFFINE_ROTATION).read() },
        duration: unsafe { cmd.add(AFFINE_DURATION).read() },
    }
}

/// Returns the command's duration as the original leaves it in the caller's
/// struct: one lower when it was nonzero.
unsafe fn apply_affine_anim_frame(matrix_num: u8, mut frame: AffineFrame) -> u8 {
    if frame.duration != 0 {
        frame.duration -= 1;
        unsafe { apply_affine_anim_frame_relative(matrix_num, &frame) };
        frame.duration
    } else {
        // A zero duration sets the scale and rotation outright.
        let state = unsafe { affine_state(matrix_num) };
        unsafe {
            state
                .add(AAS_X_SCALE)
                .cast::<i16>()
                .write_volatile(frame.x_scale)
        };
        unsafe {
            state
                .add(AAS_Y_SCALE)
                .cast::<i16>()
                .write_volatile(frame.y_scale)
        };
        unsafe {
            state
                .add(AAS_ROTATION)
                .cast::<u16>()
                .write_volatile(u16::from(frame.rotation) << 8)
        };

        let dummy = AffineFrame {
            x_scale: 0,
            y_scale: 0,
            rotation: 0,
            duration: 0,
        };
        unsafe { apply_affine_anim_frame_relative(matrix_num, &dummy) };
        0
    }
}

unsafe fn apply_affine_anim_frame_relative(matrix_num: u8, frame: &AffineFrame) {
    let state = unsafe { affine_state(matrix_num) };

    let x_scale =
        unsafe { state.add(AAS_X_SCALE).cast::<i16>().read_volatile() }.wrapping_add(frame.x_scale);
    let y_scale =
        unsafe { state.add(AAS_Y_SCALE).cast::<i16>().read_volatile() }.wrapping_add(frame.y_scale);
    // The low byte of the rotation is always cleared.
    let rotation = (unsafe { state.add(AAS_ROTATION).cast::<u16>().read_volatile() }
        .wrapping_add(u16::from(frame.rotation) << 8))
        & !0xff;

    unsafe { state.add(AAS_X_SCALE).cast::<i16>().write_volatile(x_scale) };
    unsafe { state.add(AAS_Y_SCALE).cast::<i16>().write_volatile(y_scale) };
    unsafe {
        state
            .add(AAS_ROTATION)
            .cast::<u16>()
            .write_volatile(rotation)
    };

    let src = ObjAffineSource([
        unsafe { ConvertScaleParam(x_scale) },
        unsafe { ConvertScaleParam(y_scale) },
        rotation as i16,
    ]);
    let mut result = OamMatrixStorage([0; 4]);
    unsafe { ObjAffineSet(src.0.as_ptr().cast(), result.0.as_mut_ptr().cast(), 1, 2) };
    unsafe { copy_oam_matrix(matrix_num, &result) };
}

unsafe fn copy_oam_matrix(dest_matrix_index: u8, src: &OamMatrixStorage) {
    let slot = unsafe { matrix(dest_matrix_index as usize) };
    for i in 0..4 {
        unsafe { slot.add(i).write_volatile(src.0[i]) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn ConvertScaleParam(scale: i16) -> i16 {
    // The original divides without a guard, which traps on some emulators.
    if scale == 0 {
        return 0;
    }
    (0x10000i32 / i32::from(scale)) as i16
}

#[unsafe(no_mangle)]
pub unsafe fn GetSpriteMatrixNum(sprite: *mut u8) -> u8 {
    if unsafe { oam_affine_mode(sprite) } & ST_OAM_AFFINE_ON_MASK != 0 {
        unsafe { oam_matrix_num(sprite) }
    } else {
        0
    }
}

/// Shifts a sprite as it scales so a chosen edge stays put. Only the minigame
/// countdown uses it, so the digits do not slide while they squash.
#[unsafe(no_mangle)]
pub unsafe fn SetSpriteMatrixAnchor(sprite: *mut u8, x: i16, y: i16) {
    unsafe { set_i16_at(sprite, S_DATA + ANCHOR_X_SLOT * 2, x) };
    unsafe { set_i16_at(sprite, S_DATA + ANCHOR_Y_SLOT * 2, y) };
    unsafe { set_flag(sprite, S_FLAGS1, F_ANCHORED, true) };
}

fn anchor_coord(a0: i32, a1: i32, coord: i32) -> i32 {
    let sub_result = a1 - a0;
    let var1 = if sub_result < 0 {
        -(sub_result) >> 9
    } else {
        -(sub_result >> 9)
    };
    if a0 == 0 {
        return coord - var1;
    }
    coord - (((coord.wrapping_mul(a1)) as u32 / a0 as u32) as i32 + var1)
}

unsafe fn update_sprite_matrix_anchor_pos(sprite: *mut u8) {
    let x = i32::from(unsafe { i16_at(sprite, S_DATA + ANCHOR_X_SLOT * 2) });
    let y = i32::from(unsafe { i16_at(sprite, S_DATA + ANCHOR_Y_SLOT * 2) });
    let matrix_num = unsafe { oam_matrix_num(sprite) } as usize;
    let shape = (unsafe { oam_shape(sprite) } & 3).min(2) as usize;
    let size = (unsafe { oam_size(sprite) } & 3) as usize;

    if x != NO_ANCHOR {
        let dimension = i32::from(OAM_DIMENSIONS[shape][size][0]);
        let a = i32::from(unsafe { matrix(matrix_num).read_volatile() });
        let var1 = dimension << 8;
        let var2 = if a == 0 { 0 } else { (dimension << 16) / a };
        unsafe { set_i16_at(sprite, S_X2, anchor_coord(var1, var2, x) as i16) };
    }

    if y != NO_ANCHOR {
        let dimension = i32::from(OAM_DIMENSIONS[shape][size][1]);
        let d = i32::from(unsafe { matrix(matrix_num).add(3).read_volatile() });
        let var1 = dimension << 8;
        let var2 = if d == 0 { 0 } else { (dimension << 16) / d };
        unsafe { set_i16_at(sprite, S_Y2, anchor_coord(var1, var2, y) as i16) };
    }
}

/// Bits 3 and 4 of `matrixNum` double as the flip flags when the sprite is
/// not affine.
#[unsafe(no_mangle)]
pub unsafe fn SetSpriteOamFlipBits(sprite: *mut u8, h_flip: u8, v_flip: u8) {
    let sprite_h = u8::from(unsafe { flag(sprite, S_FLAGS1, F_H_FLIP) });
    let sprite_v = u8::from(unsafe { flag(sprite, S_FLAGS1, F_V_FLIP) });
    let value = (unsafe { oam_matrix_num(sprite) } & 0x7)
        | (((h_flip ^ sprite_h) & 1) << 3)
        | (((v_flip ^ sprite_v) & 1) << 4);
    unsafe { set_oam_matrix_num(sprite, value) };
}

unsafe fn affine_anim_state_restart(matrix_num: u8) {
    let state = unsafe { affine_state(matrix_num) };
    unsafe { state.add(AAS_ANIM_CMD_INDEX).write_volatile(0) };
    unsafe { state.add(AAS_DELAY_COUNTER).write_volatile(0) };
    unsafe { state.add(AAS_LOOP_COUNTER).write_volatile(0) };
}

unsafe fn affine_anim_state_start(matrix_num: u8, anim_num: u8) {
    let state = unsafe { affine_state(matrix_num) };
    unsafe { state.add(AAS_ANIM_NUM).write_volatile(anim_num) };
    unsafe { affine_anim_state_restart(matrix_num) };
    unsafe { state.add(AAS_X_SCALE).cast::<i16>().write_volatile(0x0100) };
    unsafe { state.add(AAS_Y_SCALE).cast::<i16>().write_volatile(0x0100) };
    unsafe { state.add(AAS_ROTATION).cast::<u16>().write_volatile(0) };
}

#[unsafe(no_mangle)]
pub unsafe fn AffineAnimStateReset(matrix_num: u8) {
    unsafe { affine_anim_state_start(matrix_num, 0) };
}

#[unsafe(no_mangle)]
pub unsafe fn StartSpriteAnim(sprite: *mut u8, anim_num: u8) {
    unsafe { set_u8_at(sprite, S_ANIM_NUM, anim_num) };
    unsafe { set_flag(sprite, S_FLAGS1, F_ANIM_BEGINNING, true) };
    unsafe { set_flag(sprite, S_FLAGS1, F_ANIM_ENDED, false) };
}

#[unsafe(no_mangle)]
pub unsafe fn StartSpriteAnimIfDifferent(sprite: *mut u8, anim_num: u8) {
    if unsafe { u8_at(sprite, S_ANIM_NUM) } != anim_num {
        unsafe { StartSpriteAnim(sprite, anim_num) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn SeekSpriteAnim(sprite: *mut u8, anim_cmd_index: u8) {
    let paused = unsafe { flag(sprite, S_ANIM_STATE, ANIM_PAUSED_BIT) };
    unsafe { set_u8_at(sprite, S_ANIM_CMD_INDEX, anim_cmd_index.wrapping_sub(1)) };
    unsafe { set_anim_delay_counter(sprite, 0) };
    unsafe { set_flag(sprite, S_FLAGS1, F_ANIM_BEGINNING, false) };
    unsafe { set_flag(sprite, S_FLAGS1, F_ANIM_ENDED, false) };
    unsafe { set_flag(sprite, S_ANIM_STATE, ANIM_PAUSED_BIT, false) };
    unsafe { continue_anim(sprite) };
    if unsafe { anim_delay_counter(sprite) } != 0 {
        unsafe { set_anim_delay_counter(sprite, anim_delay_counter(sprite).wrapping_add(1)) };
    }
    unsafe { set_flag(sprite, S_ANIM_STATE, ANIM_PAUSED_BIT, paused) };
}

#[unsafe(no_mangle)]
pub unsafe fn StartSpriteAffineAnim(sprite: *mut u8, anim_num: u8) {
    let matrix_num = unsafe { GetSpriteMatrixNum(sprite) };
    unsafe { affine_anim_state_start(matrix_num, anim_num) };
    unsafe { set_flag(sprite, S_FLAGS1, F_AFFINE_ANIM_BEGINNING, true) };
    unsafe { set_flag(sprite, S_FLAGS1, F_AFFINE_ANIM_ENDED, false) };
}

#[unsafe(no_mangle)]
pub unsafe fn StartSpriteAffineAnimIfDifferent(sprite: *mut u8, anim_num: u8) {
    let matrix_num = unsafe { GetSpriteMatrixNum(sprite) };
    let current = unsafe { affine_state(matrix_num).add(AAS_ANIM_NUM).read_volatile() };
    if current != anim_num {
        unsafe { StartSpriteAffineAnim(sprite, anim_num) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn ChangeSpriteAffineAnim(sprite: *mut u8, anim_num: u8) {
    let matrix_num = unsafe { GetSpriteMatrixNum(sprite) };
    unsafe {
        affine_state(matrix_num)
            .add(AAS_ANIM_NUM)
            .write_volatile(anim_num)
    };
    unsafe { set_flag(sprite, S_FLAGS1, F_AFFINE_ANIM_BEGINNING, true) };
    unsafe { set_flag(sprite, S_FLAGS1, F_AFFINE_ANIM_ENDED, false) };
}

#[unsafe(no_mangle)]
pub unsafe fn ChangeSpriteAffineAnimIfDifferent(sprite: *mut u8, anim_num: u8) {
    let matrix_num = unsafe { GetSpriteMatrixNum(sprite) };
    let current = unsafe { affine_state(matrix_num).add(AAS_ANIM_NUM).read_volatile() };
    if current != anim_num {
        unsafe { ChangeSpriteAffineAnim(sprite, anim_num) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn SetSpriteSheetFrameTileNum(sprite: *mut u8) {
    if !unsafe { flag(sprite, S_FLAGS1, F_USING_SHEET) } {
        return;
    }

    let mut tile_offset = anim_image_value(unsafe { anim_cmd(sprite) });
    if tile_offset < 0 {
        tile_offset = 0;
    }
    let start = unsafe { u16_at(sprite, S_SHEET_TILE_START) };
    unsafe { set_oam_tile_num(oam_of(sprite), start.wrapping_add(tile_offset as u16)) };
}

#[unsafe(no_mangle)]
pub unsafe fn ResetAffineAnimData() {
    unsafe { (&raw mut gAffineAnimsDisabled).write_volatile(0) };
    unsafe { (gOamMatrixAllocBitmap.as_ptr()).write_volatile(0) };
    unsafe { ResetOamMatrices() };
    for i in 0..OAM_MATRIX_COUNT {
        unsafe { AffineAnimStateReset(i as u8) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn AllocOamMatrix() -> u8 {
    let bitmap = unsafe { (gOamMatrixAllocBitmap.as_ptr().cast_const()).read_volatile() };
    let mut i = 0u8;
    let mut bit = 1u32;

    while (i as usize) < OAM_MATRIX_COUNT {
        if bitmap & bit == 0 {
            unsafe { (gOamMatrixAllocBitmap.as_ptr()).write_volatile(bitmap | bit) };
            return i;
        }
        i += 1;
        bit <<= 1;
    }

    0xff
}

#[unsafe(no_mangle)]
pub unsafe fn FreeOamMatrix(matrix_num: u8) {
    let bit = 1u32 << matrix_num;
    let bitmap = unsafe { (gOamMatrixAllocBitmap.as_ptr().cast_const()).read_volatile() };
    unsafe { (gOamMatrixAllocBitmap.as_ptr()).write_volatile(bitmap & !bit) };
    unsafe { SetOamMatrix(matrix_num, 0x100, 0, 0, 0x100) };
}

#[unsafe(no_mangle)]
pub unsafe fn InitSpriteAffineAnim(sprite: *mut u8) {
    let matrix_num = unsafe { AllocOamMatrix() };
    if matrix_num == 0xff {
        return;
    }

    unsafe {
        CalcCenterToCornerVec(
            sprite,
            oam_shape(sprite),
            oam_size(sprite),
            oam_affine_mode(sprite),
        )
    };
    unsafe { set_oam_matrix_num(sprite, matrix_num) };
    unsafe { set_flag(sprite, S_FLAGS1, F_AFFINE_ANIM_BEGINNING, true) };
    unsafe { AffineAnimStateReset(matrix_num) };
}

#[unsafe(no_mangle)]
pub unsafe fn SetOamMatrixRotationScaling(
    matrix_num: u8,
    x_scale: i16,
    y_scale: i16,
    rotation: u16,
) {
    let src = ObjAffineSource([
        unsafe { ConvertScaleParam(x_scale) },
        unsafe { ConvertScaleParam(y_scale) },
        rotation as i16,
    ]);
    let mut result = OamMatrixStorage([0; 4]);
    unsafe { ObjAffineSet(src.0.as_ptr().cast(), result.0.as_mut_ptr().cast(), 1, 2) };
    unsafe { copy_oam_matrix(matrix_num, &result) };
}

#[unsafe(no_mangle)]
pub unsafe fn LoadSpriteSheet(sheet: *const u8) -> u16 {
    let size = unsafe { sheet.add(SHEET_SIZE).cast::<u16>().read() };
    let tile_start = unsafe { AllocSpriteTiles(size / TILE_SIZE_4BPP) };
    if tile_start < 0 {
        return 0;
    }

    let tag = unsafe { sheet.add(SHEET_TAG).cast::<u16>().read() };
    unsafe { AllocSpriteTileRange(tag, tile_start as u16, size / TILE_SIZE_4BPP) };
    unsafe {
        cpu_copy16(
            sheet.cast::<*const u8>().read(),
            (OBJ_VRAM0 + TILE_SIZE_4BPP as usize * tile_start as usize) as *mut u8,
            size,
        )
    };
    tile_start as u16
}

#[unsafe(no_mangle)]
pub unsafe fn LoadSpriteSheets(sheets: *const u8) {
    let mut i = 0usize;
    loop {
        let sheet = unsafe { sheets.add(i * SPRITE_SHEET_STRIDE) };
        if unsafe { sheet.cast::<*const u8>().read() }.is_null() {
            return;
        }
        unsafe { LoadSpriteSheet(sheet) };
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe fn FreeSpriteTilesByTag(tag: u16) {
    let index = unsafe { IndexOfSpriteTileTag(tag) };
    if index == 0xff {
        return;
    }

    let range = unsafe {
        (&raw const SPRITE_TILE_RANGES)
            .cast::<u16>()
            .add(index as usize * 2)
    };
    let start = unsafe { range.read_volatile() };
    let count = unsafe { range.add(1).read_volatile() };

    let mut i = start;
    while i < start + count {
        unsafe { free_tile(i) };
        i += 1;
    }

    unsafe {
        (&raw mut SPRITE_TILE_RANGE_TAGS)
            .cast::<u16>()
            .add(index as usize)
            .write_volatile(TAG_NONE)
    };
}

#[unsafe(no_mangle)]
pub unsafe fn FreeSpriteTileRanges() {
    for i in 0..MAX_SPRITES {
        unsafe {
            (&raw mut SPRITE_TILE_RANGE_TAGS)
                .cast::<u16>()
                .add(i)
                .write_volatile(TAG_NONE)
        };
        let range = unsafe { (&raw mut SPRITE_TILE_RANGES).cast::<u16>().add(i * 2) };
        unsafe { range.write_volatile(0) };
        unsafe { range.add(1).write_volatile(0) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn GetSpriteTileStartByTag(tag: u16) -> u16 {
    let index = unsafe { IndexOfSpriteTileTag(tag) };
    if index == 0xff {
        return 0xffff;
    }
    unsafe {
        (&raw const SPRITE_TILE_RANGES)
            .cast::<u16>()
            .add(index as usize * 2)
            .read_volatile()
    }
}

#[unsafe(no_mangle)]
pub unsafe fn IndexOfSpriteTileTag(tag: u16) -> u8 {
    for i in 0..MAX_SPRITES {
        let stored = unsafe {
            (&raw const SPRITE_TILE_RANGE_TAGS)
                .cast::<u16>()
                .add(i)
                .read_volatile()
        };
        if stored == tag {
            return i as u8;
        }
    }
    0xff
}

#[unsafe(no_mangle)]
pub unsafe fn GetSpriteTileTagByTileStart(start: u16) -> u16 {
    for i in 0..MAX_SPRITES {
        let tag = unsafe {
            (&raw const SPRITE_TILE_RANGE_TAGS)
                .cast::<u16>()
                .add(i)
                .read_volatile()
        };
        let range_start = unsafe {
            (&raw const SPRITE_TILE_RANGES)
                .cast::<u16>()
                .add(i * 2)
                .read_volatile()
        };
        if tag != TAG_NONE && range_start == start {
            return tag;
        }
    }
    TAG_NONE
}

#[unsafe(no_mangle)]
pub unsafe fn AllocSpriteTileRange(tag: u16, start: u16, count: u16) {
    let free_index = unsafe { IndexOfSpriteTileTag(TAG_NONE) } as usize;
    unsafe {
        (&raw mut SPRITE_TILE_RANGE_TAGS)
            .cast::<u16>()
            .add(free_index)
            .write_volatile(tag)
    };
    let range = unsafe {
        (&raw mut SPRITE_TILE_RANGES)
            .cast::<u16>()
            .add(free_index * 2)
    };
    unsafe { range.write_volatile(start) };
    unsafe { range.add(1).write_volatile(count) };
}

#[unsafe(no_mangle)]
pub unsafe fn FreeAllSpritePalettes() {
    unsafe { (&raw mut gReservedSpritePaletteCount).write_volatile(0) };
    for i in 0..16 {
        unsafe {
            (&raw mut SPRITE_PALETTE_TAGS)
                .cast::<u16>()
                .add(i)
                .write_volatile(TAG_NONE)
        };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn LoadSpritePalette(palette: *const u8) -> u8 {
    let tag = unsafe { palette.add(PALETTE_TAG).cast::<u16>().read() };
    let index = unsafe { IndexOfSpritePaletteTag(tag) };
    if index != 0xff {
        return index;
    }

    let index = unsafe { IndexOfSpritePaletteTag(TAG_NONE) };
    if index == 0xff {
        return 0xff;
    }

    unsafe {
        (&raw mut SPRITE_PALETTE_TAGS)
            .cast::<u16>()
            .add(index as usize)
            .write_volatile(tag)
    };
    // PLTT_ID: sixteen colours per slot.
    unsafe {
        LoadPalette(
            palette.cast::<*const c_void>().read(),
            OBJ_PLTT_OFFSET + u16::from(index) * 16,
            PLTT_SIZE_4BPP,
        )
    };
    index
}

#[unsafe(no_mangle)]
pub unsafe fn LoadSpritePalettes(palettes: *const u8) {
    let mut i = 0usize;
    loop {
        let palette = unsafe { palettes.add(i * SPRITE_PALETTE_STRIDE) };
        if unsafe { palette.cast::<*const u8>().read() }.is_null() {
            return;
        }
        if unsafe { LoadSpritePalette(palette) } == 0xff {
            return;
        }
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe fn AllocSpritePalette(tag: u16) -> u8 {
    let index = unsafe { IndexOfSpritePaletteTag(TAG_NONE) };
    if index == 0xff {
        return 0xff;
    }
    unsafe {
        (&raw mut SPRITE_PALETTE_TAGS)
            .cast::<u16>()
            .add(index as usize)
            .write_volatile(tag)
    };
    index
}

#[unsafe(no_mangle)]
pub unsafe fn IndexOfSpritePaletteTag(tag: u16) -> u8 {
    let reserved = unsafe { (&raw const gReservedSpritePaletteCount).read_volatile() };
    let mut i = reserved as usize;
    while i < 16 {
        let stored = unsafe {
            (&raw const SPRITE_PALETTE_TAGS)
                .cast::<u16>()
                .add(i)
                .read_volatile()
        };
        if stored == tag {
            return i as u8;
        }
        i += 1;
    }
    0xff
}

#[unsafe(no_mangle)]
pub unsafe fn GetSpritePaletteTagByPaletteNum(palette_num: u8) -> u16 {
    unsafe {
        (&raw const SPRITE_PALETTE_TAGS)
            .cast::<u16>()
            .add(palette_num as usize)
            .read_volatile()
    }
}

#[unsafe(no_mangle)]
pub unsafe fn FreeSpritePaletteByTag(tag: u16) {
    let index = unsafe { IndexOfSpritePaletteTag(tag) };
    if index != 0xff {
        unsafe {
            (&raw mut SPRITE_PALETTE_TAGS)
                .cast::<u16>()
                .add(index as usize)
                .write_volatile(TAG_NONE)
        };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn SetSubspriteTables(sprite: *mut u8, tables: *const u8) {
    unsafe { set_ptr_at(sprite, S_SUBSPRITE_TABLES, tables) };
    let state = unsafe { u8_at(sprite, S_SUBSPRITE_STATE) };
    unsafe {
        set_u8_at(
            sprite,
            S_SUBSPRITE_STATE,
            (state & !SUBSPRITE_TABLE_NUM_MASK) & !(0x3 << SUBSPRITE_MODE_SHIFT)
                | (SUBSPRITES_ON << SUBSPRITE_MODE_SHIFT),
        )
    };
}

#[inline]
unsafe fn subsprite_table_num(sprite: *mut u8) -> u8 {
    let state = unsafe { u8_at(sprite, S_SUBSPRITE_STATE) };
    state & SUBSPRITE_TABLE_NUM_MASK
}

#[inline]
unsafe fn subsprite_mode(sprite: *mut u8) -> u8 {
    let state = unsafe { u8_at(sprite, S_SUBSPRITE_STATE) };
    state >> SUBSPRITE_MODE_SHIFT
}

#[unsafe(no_mangle)]
pub unsafe fn AddSpriteToOamBuffer(sprite: *mut u8, oam_index: *mut u8) -> u8 {
    let index = unsafe { oam_index.read() };
    if index >= unsafe { (&raw const gOamLimit).read_volatile() } {
        return 1;
    }

    let tables = unsafe { ptr_at(sprite, S_SUBSPRITE_TABLES) };
    if tables.is_null() || unsafe { subsprite_mode(sprite) } == SUBSPRITES_OFF {
        unsafe {
            core::ptr::copy_nonoverlapping(oam_of(sprite), oam_buffer_entry(index as usize), 8)
        };
        unsafe { oam_index.write(index + 1) };
        return 0;
    }

    unsafe { AddSubspritesToOamBuffer(sprite, oam_buffer_entry(index as usize), oam_index) }
}

#[unsafe(no_mangle)]
pub unsafe fn AddSubspritesToOamBuffer(
    sprite: *mut u8,
    dest_oam: *mut u8,
    oam_index: *mut u8,
) -> u8 {
    let limit = unsafe { (&raw const gOamLimit).read_volatile() };
    if unsafe { oam_index.read() } >= limit {
        return 1;
    }

    let tables = unsafe { ptr_at(sprite, S_SUBSPRITE_TABLES) };
    let table =
        unsafe { tables.add(subsprite_table_num(sprite) as usize * SUBSPRITE_TABLE_STRIDE) };
    let subsprites = unsafe {
        table
            .add(SUBSPRITE_TABLE_SUBSPRITES)
            .cast::<*const u8>()
            .read()
    };
    let oam = unsafe { oam_of(sprite) };

    if subsprites.is_null() {
        unsafe { core::ptr::copy_nonoverlapping(oam, dest_oam, 8) };
        unsafe { oam_index.write(oam_index.read() + 1) };
        return 0;
    }

    let tile_num = unsafe { oam_tile_num(sprite) };
    let count = unsafe { table.add(SUBSPRITE_TABLE_COUNT).read() };
    let matrix_num = unsafe { oam_matrix_num(sprite) };
    let h_flip = (matrix_num >> 3) & 1;
    let v_flip = (matrix_num >> 4) & 1;
    let base_x = (unsafe { oam_attr(oam, 1) } & 0x1ff)
        .wrapping_sub(unsafe { u8_at(sprite, S_CENTER_TO_CORNER_X) } as i8 as u16);
    let base_y = (unsafe { oam_attr(oam, 0) } & 0xff)
        .wrapping_sub(unsafe { u8_at(sprite, S_CENTER_TO_CORNER_Y) } as i8 as u16);

    let mut i = 0u8;
    while i < count {
        if unsafe { oam_index.read() } >= limit {
            return 1;
        }

        let subsprite = unsafe { subsprites.add(i as usize * SUBSPRITE_STRIDE) };
        let packed = unsafe { subsprite.add(SUBSPRITE_PACKED).cast::<u16>().read() };
        let shape = (packed & SUBSPRITE_SHAPE_MASK) as u8;
        let size = ((packed & SUBSPRITE_SIZE_MASK) >> SUBSPRITE_SIZE_SHIFT) as u8;
        let tile_offset = (packed & SUBSPRITE_TILE_OFFSET_MASK) >> SUBSPRITE_TILE_OFFSET_SHIFT;
        let priority = (packed >> SUBSPRITE_PRIORITY_SHIFT) as u8;

        let mut x = unsafe { subsprite.add(SUBSPRITE_X).cast::<i8>().read() } as i16;
        let mut y = unsafe { subsprite.add(SUBSPRITE_Y).cast::<i8>().read() } as i16;

        // A flipped subsprite is mirrored about its own far edge.
        if h_flip != 0 {
            let width = i16::from(OAM_DIMENSIONS[(shape & 3).min(2) as usize][size as usize][0]);
            x = (!(x.wrapping_add(width))).wrapping_add(1);
        }
        if v_flip != 0 {
            let height = i16::from(OAM_DIMENSIONS[(shape & 3).min(2) as usize][size as usize][1]);
            y = (!(y.wrapping_add(height))).wrapping_add(1);
        }

        let entry = unsafe { dest_oam.add(i as usize * 8) };
        unsafe { core::ptr::copy_nonoverlapping(oam, entry, 8) };

        let attr0 = unsafe { oam_attr(entry, 0) };
        unsafe {
            entry
                .cast::<u16>()
                .write_volatile((attr0 & !(0x3 << 14)) | (u16::from(shape) << 14))
        };
        let attr1 = unsafe { oam_attr(entry, 1) };
        unsafe {
            entry
                .add(2)
                .cast::<u16>()
                .write_volatile((attr1 & !(0x3 << 14)) | (u16::from(size) << 14))
        };
        unsafe { set_oam_x(entry, base_x.wrapping_add(x as u16)) };
        unsafe { set_oam_y(entry, base_y.wrapping_add(y as u16)) };
        unsafe { set_oam_tile_num(entry, tile_num.wrapping_add(tile_offset)) };

        if unsafe { subsprite_mode(sprite) } != SUBSPRITES_IGNORE_PRIORITY {
            unsafe { set_oam_priority(entry, priority) };
        }

        unsafe { oam_index.write(oam_index.read() + 1) };
        i += 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprite_field_offsets_match_the_documented_structure() {
        assert_eq!(SPRITE_SIZE, 68);
        assert_eq!(S_CALLBACK, 0x1c);
        assert_eq!(S_DATA + 8 * 2, S_FLAGS0);
        assert_eq!(S_FLAGS1, 0x3f);
        assert_eq!(S_SUBPRIORITY, 0x43);
    }

    #[test]
    fn sprite_copy_requests_are_word_aligned_for_arm_pointer_accesses() {
        assert_eq!(core::mem::align_of::<SpriteCopyRequestStorage>(), 4);
        assert_eq!(core::mem::size_of::<SpriteCopyRequestStorage>(), 12 * 64);
        assert_eq!(core::mem::align_of::<ObjAffineSource>(), 4);
        assert_eq!(core::mem::size_of::<ObjAffineSource>(), 8);
        assert_eq!(core::mem::align_of::<OamMatrixStorage>(), 4);
        assert_eq!(core::mem::size_of::<OamMatrixStorage>(), 8);
        assert_eq!(core::mem::align_of::<AffineAnimStateStorage>(), 4);
        assert_eq!(core::mem::size_of::<AffineAnimStateStorage>(), 12 * 32);
    }

    #[test]
    fn the_copy_request_array_is_reachable_and_sized() {
        // Disabling ProcessSpriteCopyRequests once let LLVM prove this array
        // write-only and delete it, taking every image-array sprite's
        // graphics with it. Reading it here keeps that from passing silently.
        assert_eq!(COPY_REQUEST_STRIDE, 12);
        assert_eq!(MAX_SPRITE_COPY_REQUESTS, 64);
        assert_eq!(
            core::mem::size_of::<SpriteCopyRequestStorage>(),
            COPY_REQUEST_STRIDE * MAX_SPRITE_COPY_REQUESTS
        );
        assert_eq!(COPY_SRC, 0);
        assert_eq!(COPY_DEST, 4);
        assert_eq!(COPY_SIZE, 8);
    }

    #[test]
    fn the_dummy_oam_entry_parks_the_sprite_off_screen() {
        let attr0 = u16::from_le_bytes([gDummyOamData.0[0], gDummyOamData.0[1]]);
        let attr1 = u16::from_le_bytes([gDummyOamData.0[2], gDummyOamData.0[3]]);
        let attr2 = u16::from_le_bytes([gDummyOamData.0[4], gDummyOamData.0[5]]);
        assert_eq!(attr0 & 0xff, DISPLAY_HEIGHT as u16);
        assert_eq!(attr1 & 0x1ff, (DISPLAY_WIDTH + 64) as u16);
        // Lowest priority.
        assert_eq!((attr2 >> 10) & 0x3, 3);
    }

    #[test]
    fn center_to_corner_vectors_are_the_negated_half_dimensions() {
        for shape in 0..3 {
            for size in 0..4 {
                let vec = CENTER_TO_CORNER_VEC_TABLE[shape][size];
                let dim = OAM_DIMENSIONS[shape][size];
                assert_eq!(vec[0] as i8, -(dim[0] as i8) / 2);
                assert_eq!(vec[1] as i8, -(dim[1] as i8) / 2);
            }
        }
    }

    #[test]
    fn doubling_a_center_to_corner_vector_wraps_in_a_u8() {
        // -32 doubled is -64, which is what the u8 wraparound produces.
        assert_eq!(0xe0u8.wrapping_mul(2) as i8, -64);
        assert_eq!(0xfcu8.wrapping_mul(2) as i8, -8);
    }

    #[test]
    fn anim_command_fields_sit_where_the_bitfields_put_them() {
        // imageValue 5, duration 3, hFlip set.
        let cmd = 5u32 | (3 << ANIM_DURATION_SHIFT) | ANIM_H_FLIP;
        assert_eq!(anim_image_value(cmd), 5);
        assert_eq!(anim_duration(cmd), 3);
        assert_eq!(anim_h_flip(cmd), 1);
        assert_eq!(anim_v_flip(cmd), 0);
        // ANIM_END is all ones in the low half.
        assert_eq!(anim_image_value(0x0000_ffff), -1);
    }

    #[test]
    fn the_animation_state_byte_packs_a_counter_and_two_pause_bits() {
        assert_eq!(ANIM_DELAY_MASK, 0x3f);
        assert_eq!(ANIM_PAUSED_BIT, 0x40);
        assert_eq!(AFFINE_ANIM_PAUSED_BIT, 0x80);
        assert_eq!(
            ANIM_DELAY_MASK | ANIM_PAUSED_BIT | AFFINE_ANIM_PAUSED_BIT,
            0xff
        );
    }

    #[test]
    fn subsprite_packing_matches_the_bitfield_widths() {
        assert_eq!(SUBSPRITE_SHAPE_MASK, 0x0003);
        assert_eq!(SUBSPRITE_SIZE_MASK, 0x000c);
        assert_eq!(SUBSPRITE_TILE_OFFSET_MASK, 0x3ff0);
        assert_eq!(
            SUBSPRITE_SHAPE_MASK | SUBSPRITE_SIZE_MASK | SUBSPRITE_TILE_OFFSET_MASK | 0xc000,
            0xffff
        );
    }

    #[test]
    fn scale_conversion_inverts_the_fixed_point_scale() {
        // 0x100 is 1.0 in 8.8, so its reciprocal is itself.
        assert_eq!(unsafe { ConvertScaleParam(0x100) }, 0x100);
        assert_eq!(unsafe { ConvertScaleParam(0x200) }, 0x80);
        // The original divides unguarded; zero is trapped here instead.
        assert_eq!(unsafe { ConvertScaleParam(0) }, 0);
    }
}
