//! Shared layout constants, C ABI declarations and raw accessors.
//!
//! Every offset here was read out of the real ARM build with
//! `tools/rustport/probe.sh` rather than inferred from host Rust layouts.
//!
//! One ABI rule catches out every hand-written layout: the project builds with
//! `-mabi=apcs-gnu`, whose structure-size boundary is 32 bits, so **every C
//! structure is padded up to a multiple of four bytes**. `struct { u8 a; }` is
//! four bytes wide and `struct { u16 a; u8 b; u16 c; }` is eight, not six.
//! Mirror that in Rust with `#[repr(C, align(4))]` whenever a type has to
//! match a C structure's size or an array's stride.
//! Accessors deliberately use raw pointer arithmetic: a C index is rarely
//! statically bounded, and Rust slice indexing would emit
//! `core::panicking::panic_bounds_check`, which cannot link into this ROM.

#![allow(dead_code)]

use core::ffi::{c_int, c_void};

// ---------------------------------------------------------------- layouts

/// `sizeof(struct Task)`
pub const TASK_SIZE: usize = 40;
/// `offsetof(struct Task, data)`
pub const TASK_DATA_OFFSET: usize = 8;
/// `offsetof(struct Task, func)`
pub const TASK_FUNC_OFFSET: usize = 0;
/// `offsetof(struct Task, isActive)`
pub const TASK_IS_ACTIVE_OFFSET: usize = 4;
/// `offsetof(struct Task, prev)`
pub const TASK_PREV_OFFSET: usize = 5;
/// `offsetof(struct Task, next)`
pub const TASK_NEXT_OFFSET: usize = 6;
/// `offsetof(struct Task, priority)`
pub const TASK_PRIORITY_OFFSET: usize = 7;
pub const NUM_TASKS: usize = 16;
pub const NUM_TASK_DATA: usize = 16;

pub const HEAD_SENTINEL: u8 = 0xfe;
pub const TAIL_SENTINEL: u8 = 0xff;
/// `TASK_NONE` is spelled the same as `TAIL_SENTINEL` in the original.
pub const TASK_NONE: u8 = TAIL_SENTINEL;

/// One `struct Task` as raw bytes. A Rust struct with a real function
/// pointer would be 48 bytes on a 64-bit host, so the pool is kept as bytes
/// and reached through the accessors below.
#[repr(C, align(4))]
pub struct TaskArm(pub [u8; TASK_SIZE]);

/// `sizeof(struct Pokemon)`
pub const POKEMON_SIZE: usize = 100;
/// `sizeof(struct BoxPokemon)`
pub const BOX_POKEMON_SIZE: usize = 80;
pub const PARTY_SIZE: usize = 6;
pub const IN_BOX_COUNT: usize = 30;
pub const TOTAL_BOXES_COUNT: usize = 14;
/// `PokemonStorage.boxes` starts here despite the stale `0x0001` source
/// comment; verified from the `GetBoxedMonPtr` disassembly.
pub const STORAGE_BOXES_OFFSET: usize = 4;

/// `sizeof(struct ObjectEvent)`
pub const OBJECT_EVENT_SIZE: usize = 36;
/// `ObjectEvent.active` lives in the first bitfield byte.
pub const OBJECT_EVENT_ACTIVE_BIT: u8 = 1 << 0;
/// `ObjectEvent.singleMovementActive`, same byte.
pub const OBJECT_EVENT_SINGLE_MOVEMENT_BIT: u8 = 1 << 1;

/// `offsetof(struct PlayerAvatar, tileTransitionState)`
pub const PLAYER_AVATAR_TILE_TRANSITION_OFFSET: usize = 3;
pub const T_TILE_TRANSITION: u8 = 1;

/// `sizeof(struct Sprite)`
pub const SPRITE_SIZE: usize = 68;
/// `Sprite.oam.paletteNum` is the high nibble of OAM attribute byte 5.
pub const SPRITE_OAM_PALETTE_BYTE: usize = 5;
/// `offsetof(struct Sprite, callback)`
pub const SPRITE_CALLBACK_OFFSET: usize = 28;
/// `Sprite.invisible` is bit 2 of this byte.
pub const SPRITE_FLAGS_BYTE: usize = 0x3e;
pub const SPRITE_INVISIBLE_BIT: u8 = 1 << 2;
pub const SPRITE_DATA_OFFSET: usize = 0x2e;
pub const SPRITE_FLAGS1_BYTE: usize = 0x3f;
pub const SPRITE_ANIM_ENDED_BIT: u8 = 1 << 4;

pub const DISPLAY_WIDTH: u16 = 240;
pub const DISPLAY_HEIGHT: u16 = 160;

/// `struct PlttData` is one RGB555 halfword: red in bits 0-4, green in 5-9,
/// blue in 10-14.
#[inline]
pub const fn rgb(r: i32, g: i32, b: i32) -> u16 {
    ((r as u16) & 0x1f) | (((g as u16) & 0x1f) << 5) | (((b as u16) & 0x1f) << 10)
}

#[inline]
pub const fn rgb_red(color: u16) -> i32 {
    (color & 0x1f) as i32
}

#[inline]
pub const fn rgb_green(color: u16) -> i32 {
    ((color >> 5) & 0x1f) as i32
}

#[inline]
pub const fn rgb_blue(color: u16) -> i32 {
    ((color >> 10) & 0x1f) as i32
}

/// `PaletteFadeControl.active` is byte 7, bit 7.
pub const PALETTE_FADE_ACTIVE_OFFSET: usize = 7;
pub const PALETTE_FADE_ACTIVE_BIT: u8 = 1 << 7;
pub const PLTT_SIZE: usize = 1024;
pub const PLTT_BUFFER_SIZE: usize = PLTT_SIZE / 2;

pub const POKEMON_NAME_BUFFER: usize = 11;
pub const LOCALID_PLAYER: u8 = 0xff;

// ------------------------------------------------------- Get/SetMonData ids

pub const MON_DATA_PERSONALITY: c_int = 0;
pub const MON_DATA_OT_ID: c_int = 1;
pub const MON_DATA_NICKNAME: c_int = 2;
pub const MON_DATA_LANGUAGE: c_int = 3;
pub const MON_DATA_SANITY_IS_BAD_EGG: c_int = 4;
pub const MON_DATA_SANITY_HAS_SPECIES: c_int = 5;
pub const MON_DATA_SANITY_IS_EGG: c_int = 6;
pub const MON_DATA_OT_NAME: c_int = 7;
pub const MON_DATA_SPECIES: c_int = 11;
pub const MON_DATA_HELD_ITEM: c_int = 12;
pub const MON_DATA_HP_IV: c_int = 39;
pub const MON_DATA_ATK_IV: c_int = 40;
pub const MON_DATA_DEF_IV: c_int = 41;
pub const MON_DATA_SPEED_IV: c_int = 42;
pub const MON_DATA_SPATK_IV: c_int = 43;
pub const MON_DATA_SPDEF_IV: c_int = 44;
pub const MON_DATA_IS_EGG: c_int = 45;
pub const MON_DATA_LEVEL: c_int = 56;
pub const MON_DATA_HP: c_int = 57;
pub const MON_DATA_MAX_HP: c_int = 58;
pub const MON_DATA_MAIL: c_int = 64;

// ------------------------------------------------------- GBA I/O registers

pub const REG_BASE: usize = 0x0400_0000;

pub const REG_OFFSET_DISPCNT: usize = 0x00;
pub const REG_OFFSET_DISPSTAT: usize = 0x04;
pub const REG_OFFSET_VCOUNT: usize = 0x06;
pub const REG_OFFSET_DMA3: usize = 0xd4;
pub const REG_OFFSET_IE: usize = 0x200;
pub const REG_OFFSET_IME: usize = 0x208;

pub const DISPCNT_FORCED_BLANK: u16 = 0x0080;
pub const DISPSTAT_VBLANK_INTR: u16 = 0x0008;
pub const DISPSTAT_HBLANK_INTR: u16 = 0x0010;
pub const INTR_FLAG_VBLANK: u16 = 0x0001;
pub const INTR_FLAG_HBLANK: u16 = 0x0002;

/// The largest run DMA3 is given in one go before the code re-checks VCOUNT.
pub const MAX_DMA_BLOCK_SIZE: usize = 0x1000;

const DMA_DEST_INC: u16 = 0x0000;
const DMA_SRC_INC: u16 = 0x0000;
const DMA_SRC_FIXED: u16 = 0x0100;
const DMA_16BIT: u16 = 0x0000;
const DMA_32BIT: u16 = 0x0400;
const DMA_START_NOW: u16 = 0x0000;
const DMA_ENABLE: u16 = 0x8000;

/// Reads a 16-bit I/O register.
#[inline]
pub unsafe fn read_reg16(offset: usize) -> u16 {
    unsafe { ((REG_BASE + offset) as *const u16).read_volatile() }
}

/// Writes a 16-bit I/O register.
#[inline]
pub unsafe fn write_reg16(offset: usize, value: u16) {
    unsafe { ((REG_BASE + offset) as *mut u16).write_volatile(value) };
}

/// `DmaSet(3, src, dest, control)` - source, destination and control, then a
/// dummy read of the control register, exactly as the macro does.
#[inline]
unsafe fn dma3_set(src: u32, dest: u32, control: u32) {
    let regs = (REG_BASE + REG_OFFSET_DMA3) as *mut u32;
    // Every store the transfer may read (stack temporaries included) must
    // have happened before the DMA starts.
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    unsafe {
        regs.write_volatile(src);
        regs.add(1).write_volatile(dest);
        regs.add(2).write_volatile(control);
        let _ = regs.add(2).read_volatile();
    }
}

/// `Dma3CopyLarge_(src, dest, size, bit)`
pub unsafe fn dma3_copy_large(src: *const u8, dest: *mut u8, size: u32, bit32: bool) {
    let unit = if bit32 { 4u32 } else { 2 };
    let flags = u32::from(
        DMA_ENABLE
            | DMA_START_NOW
            | DMA_SRC_INC
            | DMA_DEST_INC
            | if bit32 { DMA_32BIT } else { DMA_16BIT },
    ) << 16;

    let mut src = src as usize as u32;
    let mut dest = dest as usize as u32;
    let mut size = size;
    loop {
        if size as usize <= MAX_DMA_BLOCK_SIZE {
            unsafe { dma3_set(src, dest, flags | (size / unit)) };
            return;
        }
        unsafe { dma3_set(src, dest, flags | (MAX_DMA_BLOCK_SIZE as u32 / unit)) };
        src += MAX_DMA_BLOCK_SIZE as u32;
        dest += MAX_DMA_BLOCK_SIZE as u32;
        size -= MAX_DMA_BLOCK_SIZE as u32;
    }
}

/// `Dma3FillLarge_(value, dest, size, bit)`
pub unsafe fn dma3_fill_large(value: u32, dest: *mut u8, size: u32, bit32: bool) {
    let unit = if bit32 { 4u32 } else { 2 };
    let flags = u32::from(
        DMA_ENABLE
            | DMA_START_NOW
            | DMA_SRC_FIXED
            | DMA_DEST_INC
            | if bit32 { DMA_32BIT } else { DMA_16BIT },
    ) << 16;

    // The macro fills from a stack temporary of the transfer width. The DMA
    // unit reads it behind the compiler's back, so the store must be
    // volatile: a plain `let` was optimised away and battle animations
    // (`monbg`) filled VRAM with stale stack bytes instead of zeros.
    let mut word = 0u32;
    let mut half = 0u16;
    unsafe {
        (&raw mut word).write_volatile(value);
        (&raw mut half).write_volatile(value as u16);
    }
    let src = if bit32 {
        (&raw const word) as usize as u32
    } else {
        (&raw const half) as usize as u32
    };

    let mut dest = dest as usize as u32;
    let mut size = size;
    loop {
        if size as usize <= MAX_DMA_BLOCK_SIZE {
            unsafe { dma3_set(src, dest, flags | (size / unit)) };
            return;
        }
        unsafe { dma3_set(src, dest, flags | (MAX_DMA_BLOCK_SIZE as u32 / unit)) };
        dest += MAX_DMA_BLOCK_SIZE as u32;
        size -= MAX_DMA_BLOCK_SIZE as u32;
    }
}

// ------------------------------------------------------- embedded assets

/// Defines a ROM data symbol from a generated asset file, the Rust
/// counterpart of the `INCGFX_*` macros.
///
/// The asset must also be listed in `rust_assets.mk`, which is what actually
/// builds it and makes `rust.o` depend on it.
#[macro_export]
macro_rules! incbin {
    ($(#[$meta:meta])* $name:ident, $path:expr) => {
        $(#[$meta])*
        #[unsafe(no_mangle)]
        pub static $name: $crate::ffi::RomBytes<{ include_bytes!($path).len() }> =
            $crate::ffi::RomBytes(*include_bytes!($path));
    };
}

/// Word alignment for a RAM static. GCC word-aligns global arrays and any
/// struct holding a pointer, and C code (and the BIOS/DMA) relies on it:
/// an ARM word store to a misaligned address silently writes the aligned
/// word below, clobbering the neighbouring variable. Every Rust static that
/// C can see, or that holds pointers or is copied in words, must use this.
#[repr(C, align(4))]
pub struct Align4<T>(pub T);

/// A word-aligned blob of ROM data. GBA copy routines move it in 32-bit
/// units, so the alignment is load-bearing, not cosmetic.
#[repr(C, align(4))]
pub struct RomBytes<const N: usize>(pub [u8; N]);

impl<const N: usize> RomBytes<N> {
    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }
}

// ------------------------------------------------- ROM tables and sprites

/// Lets a `static` hold raw pointers. ROM tables are immutable and the GBA is
/// single-threaded, so the `Sync` promise costs nothing; Rust just needs it
/// spelled out before a static may contain a pointer.
#[repr(transparent)]
pub struct RomPtr<T: ?Sized>(pub *const T);

unsafe impl<T: ?Sized> Sync for RomPtr<T> {}

impl<T: ?Sized> RomPtr<T> {
    #[inline]
    pub const fn get(&self) -> *const T {
        self.0
    }
}

/// `struct OamData` is eight bytes of packed bitfields laid out exactly like
/// GBA hardware OAM:
///
/// | attr | bits  | field                                   |
/// |------|-------|-----------------------------------------|
/// | 0    | 0-7   | `y`                                     |
/// | 0    | 8-9   | `affineMode`                            |
/// | 0    | 10-11 | `objMode`                               |
/// | 0    | 12    | `mosaic`                                |
/// | 0    | 13    | `bpp`                                   |
/// | 0    | 14-15 | `shape`                                 |
/// | 1    | 0-8   | `x`                                     |
/// | 1    | 9-13  | `matrixNum` (flip bits when not affine) |
/// | 1    | 14-15 | `size`                                  |
/// | 2    | 0-9   | `tileNum`                               |
/// | 2    | 10-11 | `priority`                              |
/// | 2    | 12-15 | `paletteNum`                            |
/// | 3    | 0-15  | `affineParam`                           |
/// `struct OamData` is eight bytes of packed bitfields. Entries are kept as
/// raw bytes copied from the compiler so no bitfield packing has to be
/// re-derived; use `tools/rustport/probe.sh` to read new ones.
#[repr(C, align(4))]
pub struct OamData(pub [u8; 8]);

unsafe impl Sync for OamData {}

pub const ST_OAM_AFFINE_OFF: u8 = 0;
pub const ST_OAM_AFFINE_NORMAL: u8 = 1;
pub const ST_OAM_AFFINE_DOUBLE: u8 = 3;
pub const ST_OAM_OBJ_NORMAL: u8 = 0;
pub const ST_OAM_OBJ_BLEND: u8 = 1;
pub const ST_OAM_4BPP: u8 = 0;
pub const ST_OAM_8BPP: u8 = 1;
pub const ST_OAM_SQUARE: u8 = 0;
pub const ST_OAM_H_RECTANGLE: u8 = 1;
pub const ST_OAM_V_RECTANGLE: u8 = 2;

impl OamData {
    /// An OAM template with every position/tile/palette field zero, which is
    /// what nearly all sprite templates use.
    pub const fn new(
        affine_mode: u8,
        obj_mode: u8,
        bpp: u8,
        shape: u8,
        size: u8,
        priority: u8,
    ) -> Self {
        Self([
            0,
            (affine_mode & 3) | (obj_mode & 3) << 2 | (bpp & 1) << 5 | (shape & 3) << 6,
            0,
            (size & 3) << 6,
            0,
            (priority & 3) << 2,
            0,
            0,
        ])
    }

    /// Non-affine, normal, 4bpp: `.shape`, `.size` and `.priority` only.
    pub const fn simple(shape: u8, size: u8, priority: u8) -> Self {
        Self::new(
            ST_OAM_AFFINE_OFF,
            ST_OAM_OBJ_NORMAL,
            ST_OAM_4BPP,
            shape,
            size,
            priority,
        )
    }
}

/// `union AnimCmd` is one 32-bit word.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct AnimCmd(pub u32);

/// `ANIMCMD_FRAME(imageValue, duration)` with both fields zero.
pub const ANIMCMD_FRAME_0_0: AnimCmd = AnimCmd(0x0000_0000);
/// `ANIMCMD_END`
pub const ANIMCMD_END: AnimCmd = AnimCmd(0x0000_ffff);

/// `ANIMCMD_FRAME(imageValue, duration, .hFlip, .vFlip)`:
/// `imageValue:16, duration:6, hFlip:1, vFlip:1`.
pub const fn anim_frame(image: u16, duration: u32, h_flip: bool, v_flip: bool) -> AnimCmd {
    AnimCmd(image as u32 | (duration & 0x3f) << 16 | (h_flip as u32) << 22 | (v_flip as u32) << 23)
}

/// `ANIMCMD_JUMP(target)`
pub const fn anim_jump(target: u32) -> AnimCmd {
    AnimCmd(0xfffd | target << 16)
}

/// `ANIMCMD_LOOP(count)`
pub const fn anim_loop(count: u32) -> AnimCmd {
    AnimCmd(0xfffe | count << 16)
}

pub type SpriteCallback = unsafe extern "C" fn(*mut u8);

/// `struct SpriteTemplate`, 24 bytes.
#[repr(C)]
pub struct SpriteTemplate {
    pub tile_tag: u16,
    pub palette_tag: u16,
    pub oam: *const OamData,
    pub anims: *const RomPtr<AnimCmd>,
    pub images: *const u8,
    pub affine_anims: *const RomPtr<u8>,
    pub callback: SpriteCallback,
}

unsafe impl Sync for SpriteTemplate {}

/// `struct CompressedSpriteSheet`, 8 bytes.
#[repr(C)]
pub struct CompressedSpriteSheet {
    pub data: *const u32,
    pub size: u16,
    pub tag: u16,
}

unsafe impl Sync for CompressedSpriteSheet {}

/// `struct CompressedSpritePalette`, 8 bytes.
#[repr(C)]
pub struct CompressedSpritePalette {
    pub data: *const u32,
    pub tag: u16,
}

unsafe impl Sync for CompressedSpritePalette {}

/// `struct WindowTemplate`, 8 bytes.
#[repr(C, align(4))]
#[derive(Clone, Copy, Default)]
pub struct WindowTemplate {
    pub bg: u8,
    pub tilemap_left: u8,
    pub tilemap_top: u8,
    pub width: u8,
    pub height: u8,
    pub palette_num: u8,
    pub base_block: u16,
}

/// `struct BgTemplate`: one word of u16 bitfields. `baseTile` doesn't fit in
/// the first halfword, so GCC starts it at bit 16.
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct BgTemplate(pub u32);

impl BgTemplate {
    pub const fn new(
        bg: u32,
        char_base_index: u32,
        map_base_index: u32,
        screen_size: u32,
        palette_mode: u32,
        priority: u32,
        base_tile: u32,
    ) -> Self {
        Self(
            (bg & 3)
                | (char_base_index & 3) << 2
                | (map_base_index & 0x1f) << 4
                | (screen_size & 3) << 9
                | (palette_mode & 1) << 11
                | (priority & 3) << 12
                | (base_tile & 0x3ff) << 16,
        )
    }
}

/// `DUMMY_WIN_TEMPLATE`
pub const DUMMY_WIN_TEMPLATE: WindowTemplate = WindowTemplate {
    bg: 0xff,
    tilemap_left: 0,
    tilemap_top: 0,
    width: 0,
    height: 0,
    palette_num: 0,
    base_block: 0,
};

/// `gMain.state`
pub const MAIN_STATE_OFFSET: usize = 0x438;
/// `gMain.heldKeys` / `gMain.newKeys`
pub const MAIN_HELD_KEYS_OFFSET: usize = 0x2c;
pub const MAIN_NEW_KEYS_OFFSET: usize = 0x2e;
pub const A_BUTTON: u16 = 0x0001;
pub const B_BUTTON: u16 = 0x0002;

unsafe extern "C" {
    pub static mut gMain: u8;
}

/// `&gMain.state`
#[inline]
pub unsafe fn main_state() -> *mut u8 {
    unsafe { (&raw mut gMain).add(MAIN_STATE_OFFSET) }
}

/// `JOY_NEW(mask)`
#[inline]
pub unsafe fn joy_new(mask: u16) -> bool {
    let keys = unsafe {
        (&raw const gMain)
            .add(MAIN_NEW_KEYS_OFFSET)
            .cast::<u16>()
            .read_volatile()
    };
    keys & mask != 0
}

/// `JOY_HELD(mask)`
#[inline]
pub unsafe fn joy_held(mask: u16) -> bool {
    let keys = unsafe {
        (&raw const gMain)
            .add(MAIN_HELD_KEYS_OFFSET)
            .cast::<u16>()
            .read_volatile()
    };
    keys & mask != 0
}
pub const PALETTES_BG: u32 = 0x0000_ffff;
pub const RGB_WHITE: u16 = 0x7fff;
pub const RGB_WHITEALPHA: u16 = 0xffff;
pub const MENU_B_PRESSED: i8 = -1;
pub const SE_SELECT: u16 = 5;
pub const PLTT_SIZE_4BPP: u16 = 0x20;

pub const FONT_NORMAL: u8 = 1;
pub const COPYWIN_MAP: u8 = 1;
pub const COPYWIN_GFX: u8 = 2;
pub const COPYWIN_FULL_MODE: u8 = 3;

/// `offsetof(struct SaveBlock1, money)`
pub const SAVE1_MONEY_OFFSET: usize = 0x490;
/// `offsetof(struct SaveBlock2, encryptionKey)`
pub const SAVE2_ENCRYPTION_KEY_OFFSET: usize = 0xac;
/// `offsetof(struct SaveBlock2, pokedex)`
pub const SAVE2_POKEDEX_OFFSET: usize = 0x18;

// --------------------------------------------------------------- callbacks

pub type TaskFunc = unsafe extern "C" fn(u8);
pub type MainCallback = unsafe extern "C" fn();
/// `bool8 (*)(void)` field-move setup callbacks.
pub type FieldCallback = unsafe extern "C" fn() -> u8;
pub type MenuFieldCallback = unsafe extern "C" fn();

unsafe extern "C" {
    pub static mut gPlayerParty: u8;
    pub static mut gEnemyParty: u8;
    pub static mut gObjectEvents: u8;
    pub static mut gPlayerAvatar: u8;
    pub static mut gPaletteFade: u8;
    pub static mut gPlttBufferUnfaded: [u16; PLTT_BUFFER_SIZE];
    pub static mut gPlttBufferFaded: [u16; PLTT_BUFFER_SIZE];
    pub static mut gSelectedObjectEvent: u8;

    pub fn CreateTask(func: TaskFunc, priority: u8) -> u8;
    pub fn DestroyTask(task_id: u8);
    pub fn FuncIsActiveTask(func: TaskFunc) -> u8;
    pub fn FindTaskIdByFunc(func: TaskFunc) -> u8;

    pub fn GetMonData2(mon: *mut u8, field: c_int) -> u32;
    pub fn GetMonData3(mon: *mut u8, field: c_int, destination: *mut u8) -> u32;
    pub fn SetMonData(mon: *mut u8, field: c_int, value: *const c_void);
    pub fn GetBoxMonData2(mon: *mut u8, field: c_int) -> u32;
    pub fn GetBoxMonData3(mon: *mut u8, field: c_int, destination: *mut u8) -> u32;
    pub fn GetMonNickname(mon: *mut u8, destination: *mut u8) -> *mut u8;

    pub fn PlaySE(song: u16);

    pub fn CpuSet(src: *const core::ffi::c_void, dest: *mut core::ffi::c_void, control: u32);
    pub fn CpuFastSet(src: *const core::ffi::c_void, dest: *mut core::ffi::c_void, control: u32);

    pub fn AddTextPrinterParameterized(
        window_id: u8,
        font_id: u8,
        string: *const u8,
        x: u8,
        y: u8,
        speed: u8,
        callback: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;

    pub fn SetWindowTemplateFields(
        template: *mut WindowTemplate,
        bg: u8,
        left: u8,
        top: u8,
        width: u8,
        height: u8,
        palette_num: u8,
        base_block: u16,
    );
    pub fn DrawStdFrameWithCustomTileAndPalette(
        window_id: u8,
        copy_to_vram: u8,
        base_tile_num: u16,
        palette_num: u8,
    );
    pub fn ClearStdWindowAndFrameToTransparent(window_id: u8, copy_to_vram: u8);

}

#[allow(unused_imports)]
pub use crate::bg::{GetBgAttribute, ShowBg};
pub use crate::decompress::{LoadCompressedSpritePalette, LoadCompressedSpriteSheet};
pub use crate::event_data::{
    ClearDailyFlags, FlagClear, FlagGet, FlagSet, GetVarPointer, VarGet, VarSet,
};
#[allow(unused_imports)]
pub use crate::malloc::{Alloc, AllocZeroed, Free};
#[allow(unused_imports)]
pub use crate::sprite::{
    CreateSprite, DestroySpriteAndFreeResources, MAX_SPRITES, SpriteCallbackDummy, gDummyOamData,
    gDummySpriteAffineAnimTable, gDummySpriteAnimTable, gSprites,
};
/// Flags and variables are implemented in Rust now; re-exported here so
/// callers keep a single import site.
#[allow(unused_imports)]
pub use crate::string_util::{
    ConvertIntToDecimalStringN, ConvertIntToHexStringN, ConvertInternationalString,
    ConvertUIntToDecimalStringN, EOS, GetExtCtrlCodeLength, StringAppend, StringAppendN,
    StringCompare, StringCompareN, StringCopy, StringCopy_Nickname, StringCopy_PlayerName,
    StringCopyN, StringCopyPadded, StringExpandPlaceholders, StringFill, StringFillWithTerminator,
    StringGet_Nickname, StringLength, StringLength_Multibyte, StripExtCtrlCodes, gStringVar1,
    gStringVar2, gStringVar3, gStringVar4,
};
#[allow(unused_imports)]
pub use crate::task::gTasks;
pub use crate::window::{
    AddWindow, ClearWindowTilemap, CopyWindowToVram, FillWindowPixelBuffer, PutWindowTilemap,
    RemoveWindow, gWindows,
};

// ------------------------------------------------------ script variables

/// The script-visible special variables, owned by `event_data.rs`. They live
/// here because most modules touch one or two of them and a crate can only
/// define each `no_mangle` symbol once.
macro_rules! special_var {
    ($($name:ident),* $(,)?) => {
        $(
            #[unsafe(no_mangle)]
            #[unsafe(link_section = "ewram_data")]
            pub static mut $name: u16 = 0;
        )*
    };
}

special_var!(
    gSpecialVar_0x8000,
    gSpecialVar_0x8001,
    gSpecialVar_0x8002,
    gSpecialVar_0x8003,
    gSpecialVar_0x8004,
    gSpecialVar_0x8005,
    gSpecialVar_0x8006,
    gSpecialVar_0x8007,
    gSpecialVar_0x8008,
    gSpecialVar_0x8009,
    gSpecialVar_0x800A,
    gSpecialVar_0x800B,
    gSpecialVar_Result,
    gSpecialVar_LastTalked,
    gSpecialVar_Facing,
    gSpecialVar_MonBoxId,
    gSpecialVar_MonBoxPos,
    gSpecialVar_Unused_0x8014,
);

// --------------------------------------------------------------- accessors

/// Reads OAM attribute word `index` (0-3) of an eight-byte OAM entry.
#[inline]
pub unsafe fn oam_attr(oam: *const u8, index: usize) -> u16 {
    unsafe { oam.add(index * 2).cast::<u16>().read_volatile() }
}

#[inline]
unsafe fn set_oam_field(oam: *mut u8, attr: usize, shift: u32, width: u32, value: u16) {
    let mask = ((1u32 << width) - 1) as u16;
    let slot = unsafe { oam.add(attr * 2).cast::<u16>() };
    let current = unsafe { slot.read_volatile() };
    unsafe { slot.write_volatile((current & !(mask << shift)) | ((value & mask) << shift)) };
}

/// `oam->y`
#[inline]
pub unsafe fn set_oam_y(oam: *mut u8, y: u16) {
    unsafe { set_oam_field(oam, 0, 0, 8, y) };
}

/// `oam->shape`
#[inline]
pub unsafe fn oam_shape(oam: *const u8) -> u8 {
    (unsafe { oam_attr(oam, 0) } >> 14) as u8
}

/// `oam->x`
#[inline]
pub unsafe fn set_oam_x(oam: *mut u8, x: u16) {
    unsafe { set_oam_field(oam, 1, 0, 9, x) };
}

/// `oam->size`
#[inline]
pub unsafe fn oam_size(oam: *const u8) -> u8 {
    (unsafe { oam_attr(oam, 1) } >> 14) as u8
}

/// `oam->tileNum`
#[inline]
pub unsafe fn set_oam_tile_num(oam: *mut u8, tile_num: u16) {
    unsafe { set_oam_field(oam, 2, 0, 10, tile_num) };
}

/// `oam->priority`
#[inline]
pub unsafe fn set_oam_priority(oam: *mut u8, priority: u8) {
    unsafe { set_oam_field(oam, 2, 10, 2, u16::from(priority)) };
}

/// `oam->paletteNum`
#[inline]
pub unsafe fn set_oam_palette_num(oam: *mut u8, palette_num: u8) {
    unsafe { set_oam_field(oam, 2, 12, 4, u16::from(palette_num)) };
}

/// `&gTasks[taskId]`
#[inline]
pub unsafe fn task(task_id: u8) -> *mut u8 {
    unsafe {
        (&raw mut gTasks)
            .cast::<u8>()
            .add(task_id as usize * TASK_SIZE)
    }
}

/// `&gTasks[taskId].data[index]`
#[inline]
pub unsafe fn task_data_ptr(task_id: u8, index: usize) -> *mut i16 {
    unsafe {
        task(task_id)
            .add(TASK_DATA_OFFSET + index * 2)
            .cast::<i16>()
    }
}

#[inline]
pub unsafe fn task_data(task_id: u8, index: usize) -> i16 {
    unsafe { task_data_ptr(task_id, index).read() }
}

#[inline]
pub unsafe fn set_task_data(task_id: u8, index: usize, value: i16) {
    unsafe { task_data_ptr(task_id, index).write(value) };
}

/// `gTasks[taskId].func = func`
#[inline]
pub unsafe fn set_task_func(task_id: u8, func: TaskFunc) {
    unsafe { task(task_id).cast::<TaskFunc>().write(func) };
}

/// Field moves stash a callback in `data[8]`/`data[9]`, high half first.
/// The order is load-bearing; the original code is not symmetric with any
/// generic 32-bit task argument helper.
#[inline]
pub unsafe fn set_field_move_callback(task_id: u8, callback: MenuFieldCallback) {
    let address = callback as usize as u32;
    unsafe { set_task_data(task_id, 8, (address >> 16) as i16) };
    unsafe { set_task_data(task_id, 9, address as i16) };
}

/// `&gPlayerParty[index]`
#[inline]
pub unsafe fn party_mon(index: usize) -> *mut u8 {
    unsafe { (&raw mut gPlayerParty).add(index * POKEMON_SIZE) }
}

/// `&gObjectEvents[index]`
#[inline]
pub unsafe fn object_event(index: usize) -> *mut u8 {
    unsafe { (&raw mut gObjectEvents).add(index * OBJECT_EVENT_SIZE) }
}

/// `gObjectEvents[index].singleMovementActive`
#[inline]
pub unsafe fn object_event_single_movement_active(index: usize) -> bool {
    unsafe { object_event(index).read_volatile() & OBJECT_EVENT_SINGLE_MOVEMENT_BIT != 0 }
}

/// `gObjectEvents[index].active`
#[inline]
pub unsafe fn object_event_active(index: usize) -> bool {
    unsafe { object_event(index).read_volatile() & OBJECT_EVENT_ACTIVE_BIT != 0 }
}

/// `&gSprites[index]`
#[inline]
pub unsafe fn sprite(index: usize) -> *mut u8 {
    unsafe { (&raw mut gSprites).cast::<u8>().add(index * SPRITE_SIZE) }
}

/// `sprite->data[index]`
#[inline]
pub unsafe fn sprite_data(sprite: *mut u8, index: usize) -> *mut i16 {
    unsafe { sprite.add(SPRITE_DATA_OFFSET + index * 2).cast() }
}

/// `sprite->animEnded`
#[inline]
pub unsafe fn sprite_anim_ended(sprite: *mut u8) -> bool {
    let flags = unsafe { sprite.add(SPRITE_FLAGS1_BYTE).read() };
    flags & SPRITE_ANIM_ENDED_BIT != 0
}

/// `sprite->callback = callback`
#[inline]
pub unsafe fn set_sprite_callback(sprite: *mut u8, callback: SpriteCallback) {
    unsafe {
        sprite
            .add(SPRITE_CALLBACK_OFFSET)
            .cast::<SpriteCallback>()
            .write(callback)
    };
}

/// `gSprites[index].oam.paletteNum`
#[inline]
pub unsafe fn sprite_palette_num(index: usize) -> u8 {
    unsafe { sprite(index).add(SPRITE_OAM_PALETTE_BYTE).read_volatile() >> 4 }
}

/// `gPaletteFade.active`
#[inline]
pub unsafe fn palette_fade_active() -> bool {
    unsafe {
        (&raw const gPaletteFade)
            .add(PALETTE_FADE_ACTIVE_OFFSET)
            .read_volatile()
            & PALETTE_FADE_ACTIVE_BIT
            != 0
    }
}

/// `gPlayerAvatar.tileTransitionState`
#[inline]
pub unsafe fn player_avatar_tile_transition_state() -> u8 {
    unsafe {
        (&raw const gPlayerAvatar)
            .add(PLAYER_AVATAR_TILE_TRANSITION_OFFSET)
            .read_volatile()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_addresses_match_the_arm_structure() {
        // Offsets are byte counts into gTasks, independent of host pointer size.
        let base = |id: u8, index: usize| id as usize * TASK_SIZE + TASK_DATA_OFFSET + index * 2;
        assert_eq!(base(0, 0), 8);
        assert_eq!(base(0, 8), 24);
        assert_eq!(base(1, 0), 48);
        assert_eq!(base(15, 15), 638);
        assert_eq!(TASK_SIZE * NUM_TASKS, 640);
    }

    #[test]
    fn storage_and_party_strides_match_the_arm_structures() {
        assert_eq!(POKEMON_SIZE * PARTY_SIZE, 600);
        assert_eq!(
            STORAGE_BOXES_OFFSET + TOTAL_BOXES_COUNT * IN_BOX_COUNT * BOX_POKEMON_SIZE,
            0x8344
        );
    }

    #[test]
    fn rom_table_types_match_the_arm_structures() {
        // Host pointers are eight bytes, so only the fields that do not
        // contain pointers can be size-checked here.
        assert_eq!(core::mem::size_of::<OamData>(), 8);
        assert_eq!(core::mem::size_of::<AnimCmd>(), 4);
        assert_eq!(core::mem::size_of::<WindowTemplate>(), 8);
        assert_eq!(core::mem::offset_of!(WindowTemplate, base_block), 6);
        assert_eq!(ANIMCMD_END.0, 0x0000_ffff);
    }

    #[test]
    fn oam_field_writes_land_on_the_documented_bits() {
        let mut oam = [0u8; 8];
        let p = oam.as_mut_ptr();
        unsafe {
            set_oam_y(p, 0xff);
            set_oam_x(p, 0x1ff);
            set_oam_tile_num(p, 0x3ff);
            set_oam_priority(p, 3);
            set_oam_palette_num(p, 0xf);
        }
        assert_eq!(u16::from_le_bytes([oam[0], oam[1]]), 0x00ff);
        assert_eq!(u16::from_le_bytes([oam[2], oam[3]]), 0x01ff);
        assert_eq!(u16::from_le_bytes([oam[4], oam[5]]), 0xffff);
        assert_eq!(u16::from_le_bytes([oam[6], oam[7]]), 0x0000);

        // Writing a field again must not disturb its neighbours.
        unsafe { set_oam_priority(p, 0) };
        assert_eq!(u16::from_le_bytes([oam[4], oam[5]]), 0xf3ff);
    }

    #[test]
    fn oam_shape_and_size_read_the_money_label_entry() {
        // 32x16: shape 1 (horizontal), size 2.
        let oam = OamData([0x00, 0x40, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00]);
        let p = oam.0.as_ptr();
        assert_eq!(unsafe { oam_shape(p) }, 1);
        assert_eq!(unsafe { oam_size(p) }, 2);
    }

    #[test]
    fn dma_control_words_match_the_macro_expansions() {
        let copy32 =
            u32::from(DMA_ENABLE | DMA_START_NOW | DMA_32BIT | DMA_SRC_INC | DMA_DEST_INC) << 16;
        let fill16 =
            u32::from(DMA_ENABLE | DMA_START_NOW | DMA_16BIT | DMA_SRC_FIXED | DMA_DEST_INC) << 16;
        assert_eq!(copy32, 0x8400_0000);
        assert_eq!(fill16, 0x8100_0000);
    }

    #[test]
    fn palette_buffers_cover_one_full_palette_block() {
        assert_eq!(PLTT_BUFFER_SIZE, 512);
    }
}
