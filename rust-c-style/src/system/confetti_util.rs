//! The confetti effect writes straight into the second half of the OAM
//! buffer rather than going through the sprite system.

use crate::ffi::{
    AllocZeroed, Free, oam_shape, oam_size, set_oam_palette_num, set_oam_priority,
    set_oam_tile_num, set_oam_x, set_oam_y,
};

const MAX_CONFETTI: u8 = 64;
/// Confetti occupy OAM slots 64 and up.
const OAM_SLOT_BASE: usize = 64;
const OAM_ENTRY_SIZE: usize = 8;
/// `offsetof(struct Main, oamBuffer)`
const MAIN_OAM_BUFFER_OFFSET: usize = 56;

const DISPLAY_WIDTH: u16 = 240;
const DISPLAY_HEIGHT: u16 = 160;

/// `sizeof(struct ConfettiUtil)` and its field offsets, read with
/// `tools/rustport/probe.sh`.
const CONFETTI_SIZE: usize = 48;
const CONFETTI_OAM: usize = 0;
const CONFETTI_X: usize = 8;
const CONFETTI_Y: usize = 10;
const CONFETTI_X_DELTA: usize = 12;
const CONFETTI_Y_DELTA: usize = 14;
const CONFETTI_TILE_TAG: usize = 16;
const CONFETTI_PAL_TAG: usize = 18;
const CONFETTI_TILE_NUM: usize = 20;
const CONFETTI_ID: usize = 22;
const CONFETTI_ANIM_NUM: usize = 24;
/// `active:1`, `allowUpdates:1`, `dummied:1`, `priority:2` share this byte.
const CONFETTI_FLAGS: usize = 25;
const CONFETTI_DATA: usize = 26;
const CONFETTI_DATA_COUNT: usize = 8;
const CONFETTI_CALLBACK: usize = 44;

const FLAG_ACTIVE: u8 = 1 << 0;
const FLAG_ALLOW_UPDATES: u8 = 1 << 1;
const FLAG_DUMMIED: u8 = 1 << 2;
const PRIORITY_SHIFT: u32 = 3;
const PRIORITY_MASK: u8 = 0b11 << PRIORITY_SHIFT;

type ConfettiCallback = unsafe extern "C" fn(*mut u8);

/// The anonymous `{ u8 count; struct ConfettiUtil *array; }` work block.
/// The pointer is four-byte aligned, so `array` starts at byte 4.
const WORK_COUNT: usize = 0;
const WORK_ARRAY: usize = 4;
const WORK_SIZE: usize = 8;

#[unsafe(link_section = "ewram_data")]
static mut WORK: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gMain: u8;
    static gDummyOamData: u8;

    fn GetSpriteTileStartByTag(tag: u16) -> u16;
    fn GetTilesPerImage(shape: u8, size: u8) -> u8;
    fn IndexOfSpritePaletteTag(tag: u16) -> u8;
}

#[inline]
unsafe fn work_count() -> u8 {
    unsafe { WORK.add(WORK_COUNT).read_volatile() }
}

#[inline]
unsafe fn work_array() -> *mut u8 {
    unsafe { WORK.add(WORK_ARRAY).cast::<*mut u8>().read_volatile() }
}

#[inline]
unsafe fn entry(index: usize) -> *mut u8 {
    unsafe { work_array().add(index * CONFETTI_SIZE) }
}

/// `&gMain.oamBuffer[slot + 64]`
#[inline]
unsafe fn oam_slot(slot: usize) -> *mut u8 {
    unsafe {
        (&raw mut gMain).add(MAIN_OAM_BUFFER_OFFSET + (slot + OAM_SLOT_BASE) * OAM_ENTRY_SIZE)
    }
}

#[inline]
unsafe fn copy_dummy_oam_to(destination: *mut u8) {
    unsafe {
        core::ptr::copy_nonoverlapping(&raw const gDummyOamData, destination, OAM_ENTRY_SIZE)
    };
}

#[inline]
unsafe fn flags(index: usize) -> u8 {
    unsafe { entry(index).add(CONFETTI_FLAGS).read_volatile() }
}

#[inline]
unsafe fn set_flag(confetti: *mut u8, flag: u8) {
    let slot = unsafe { confetti.add(CONFETTI_FLAGS) };
    unsafe { slot.write_volatile(slot.read_volatile() | flag) };
}

#[inline]
unsafe fn read_i16(confetti: *mut u8, offset: usize) -> i16 {
    unsafe { confetti.add(offset).cast::<i16>().read_volatile() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConfettiUtil_Init(count: u8) -> u32 {
    if count == 0 {
        return 0;
    }
    let count = count.min(MAX_CONFETTI);

    unsafe { (&raw mut WORK).write(AllocZeroed(WORK_SIZE as u32)) };
    if unsafe { (&raw const WORK).read() }.is_null() {
        return 0;
    }

    let array = unsafe { AllocZeroed(u32::from(count) * CONFETTI_SIZE as u32) };
    unsafe { WORK.add(WORK_ARRAY).cast::<*mut u8>().write(array) };
    if array.is_null() {
        unsafe { Free(WORK) };
        unsafe { (&raw mut WORK).write(core::ptr::null_mut()) };
        return 0;
    }

    unsafe { WORK.add(WORK_COUNT).write(count) };
    let mut index = 0usize;
    while index < count as usize {
        let confetti = unsafe { entry(index) };
        unsafe { copy_dummy_oam_to(confetti.add(CONFETTI_OAM)) };
        unsafe { set_flag(confetti, FLAG_DUMMIED) };
        index += 1;
    }

    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConfettiUtil_Free() -> u32 {
    if unsafe { (&raw const WORK).read() }.is_null() {
        return 0;
    }

    let count = unsafe { work_count() } as usize;
    let mut index = 0usize;
    while index < count {
        unsafe { copy_dummy_oam_to(oam_slot(index)) };
        index += 1;
    }

    let array = unsafe { work_array() };
    unsafe { core::ptr::write_bytes(array, 0, count * CONFETTI_SIZE) };
    unsafe { Free(array) };
    unsafe {
        WORK.add(WORK_ARRAY)
            .cast::<*mut u8>()
            .write(core::ptr::null_mut())
    };

    unsafe { core::ptr::write_bytes(WORK, 0, WORK_SIZE) };
    unsafe { Free(WORK) };
    unsafe { (&raw mut WORK).write(core::ptr::null_mut()) };

    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConfettiUtil_Update() -> u32 {
    if unsafe { (&raw const WORK).read() }.is_null() || unsafe { work_array() }.is_null() {
        return 0;
    }

    let count = unsafe { work_count() } as usize;
    let mut index = 0usize;
    while index < count {
        let confetti = unsafe { entry(index) };
        let state = unsafe { flags(index) };
        if state & FLAG_ACTIVE == 0 || state & FLAG_ALLOW_UPDATES == 0 {
            index += 1;
            continue;
        }

        let callback = unsafe {
            confetti
                .add(CONFETTI_CALLBACK)
                .cast::<Option<ConfettiCallback>>()
                .read()
        };
        if let Some(callback) = callback {
            unsafe { callback(confetti) };
        }

        // The callback may have set `dummied`, so re-read the flags.
        if unsafe { flags(index) } & FLAG_DUMMIED != 0 {
            unsafe { copy_dummy_oam_to(oam_slot(index)) };
        } else {
            let oam = unsafe { confetti.add(CONFETTI_OAM) };
            let y = unsafe { read_i16(confetti, CONFETTI_Y) }
                .wrapping_add(unsafe { read_i16(confetti, CONFETTI_Y_DELTA) });
            let x = unsafe { read_i16(confetti, CONFETTI_X) }
                .wrapping_add(unsafe { read_i16(confetti, CONFETTI_X_DELTA) });
            unsafe { set_oam_y(oam, y as u16) };
            unsafe { set_oam_x(oam, x as u16) };
            let priority = (unsafe { flags(index) } & PRIORITY_MASK) >> PRIORITY_SHIFT;
            unsafe { set_oam_priority(oam, priority) };
            let tile_num = unsafe {
                confetti
                    .add(CONFETTI_TILE_NUM)
                    .cast::<u16>()
                    .read_volatile()
            };
            unsafe { set_oam_tile_num(oam, tile_num) };
            unsafe {
                core::ptr::copy_nonoverlapping(oam.cast_const(), oam_slot(index), OAM_ENTRY_SIZE)
            };
        }

        index += 1;
    }

    1
}

unsafe fn set_anim_and_tile_num(confetti: *mut u8, anim_num: u8) -> u32 {
    if confetti.is_null() {
        return 0;
    }

    let tile_tag = unsafe {
        confetti
            .add(CONFETTI_TILE_TAG)
            .cast::<u16>()
            .read_volatile()
    };
    let tile_start = unsafe { GetSpriteTileStartByTag(tile_tag) };
    if tile_start == 0xffff {
        return 0;
    }

    unsafe { confetti.add(CONFETTI_ANIM_NUM).write_volatile(anim_num) };
    let oam = unsafe { confetti.add(CONFETTI_OAM) };
    let per_image = unsafe { GetTilesPerImage(oam_shape(oam), oam_size(oam)) };
    let tile_num = u16::from(per_image)
        .wrapping_mul(u16::from(anim_num))
        .wrapping_add(tile_start);
    unsafe {
        confetti
            .add(CONFETTI_TILE_NUM)
            .cast::<u16>()
            .write_volatile(tile_num)
    };
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConfettiUtil_SetCallback(id: u8, func: Option<ConfettiCallback>) -> u8 {
    if unsafe { (&raw const WORK).read() }.is_null() || id >= unsafe { work_count() } {
        return 0xff;
    }
    if unsafe { flags(id as usize) } & FLAG_ACTIVE == 0 {
        return 0xff;
    }

    unsafe {
        entry(id as usize)
            .add(CONFETTI_CALLBACK)
            .cast::<Option<ConfettiCallback>>()
            .write(func)
    };
    id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConfettiUtil_SetData(id: u8, data_array_id: u8, data_value: i16) -> u8 {
    if unsafe { (&raw const WORK).read() }.is_null() || id >= unsafe { work_count() } {
        return 0xff;
    }
    // The last data slot is reserved for a task id, hence the - 1.
    if unsafe { flags(id as usize) } & FLAG_ACTIVE == 0
        || data_array_id as usize > CONFETTI_DATA_COUNT - 1
    {
        return 0xff;
    }

    unsafe {
        entry(id as usize)
            .add(CONFETTI_DATA + data_array_id as usize * 2)
            .cast::<i16>()
            .write_volatile(data_value)
    };
    id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConfettiUtil_AddNew(
    oam: *const u8,
    tile_tag: u16,
    pal_tag: u16,
    x: i16,
    y: i16,
    anim_num: u8,
    priority: u8,
) -> u8 {
    if unsafe { (&raw const WORK).read() }.is_null() || oam.is_null() {
        return 0xff;
    }

    let count = unsafe { work_count() } as usize;
    let mut confetti = core::ptr::null_mut::<u8>();
    let mut index = 0usize;
    while index < count {
        if unsafe { flags(index) } & FLAG_ACTIVE == 0 {
            confetti = unsafe { entry(index) };
            unsafe { core::ptr::write_bytes(confetti, 0, CONFETTI_SIZE) };
            unsafe { confetti.add(CONFETTI_ID).write_volatile(index as u8) };
            unsafe { set_flag(confetti, FLAG_ACTIVE | FLAG_ALLOW_UPDATES) };
            break;
        }
        index += 1;
    }

    if confetti.is_null() {
        return 0xff;
    }

    unsafe { core::ptr::copy_nonoverlapping(oam, confetti.add(CONFETTI_OAM), OAM_ENTRY_SIZE) };
    unsafe {
        confetti
            .add(CONFETTI_TILE_TAG)
            .cast::<u16>()
            .write_volatile(tile_tag)
    };
    unsafe {
        confetti
            .add(CONFETTI_PAL_TAG)
            .cast::<u16>()
            .write_volatile(pal_tag)
    };
    unsafe { confetti.add(CONFETTI_X).cast::<i16>().write_volatile(x) };
    unsafe { confetti.add(CONFETTI_Y).cast::<i16>().write_volatile(y) };
    unsafe { set_oam_palette_num(confetti.add(CONFETTI_OAM), IndexOfSpritePaletteTag(pal_tag)) };

    if priority < 4 {
        let slot = unsafe { confetti.add(CONFETTI_FLAGS) };
        let current = unsafe { slot.read_volatile() } & !PRIORITY_MASK;
        unsafe { slot.write_volatile(current | (priority << PRIORITY_SHIFT)) };
        unsafe { set_oam_priority(confetti.add(CONFETTI_OAM), priority) };
    }

    let _ = unsafe { set_anim_and_tile_num(confetti, anim_num) };

    unsafe { confetti.add(CONFETTI_ID).read_volatile() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConfettiUtil_Remove(id: u8) -> u8 {
    // The original does not range-check `id` here, only the work block.
    if unsafe { (&raw const WORK).read() }.is_null()
        || unsafe { flags(id as usize) } & FLAG_ACTIVE == 0
    {
        return 0xff;
    }

    let confetti = unsafe { entry(id as usize) };
    unsafe { core::ptr::write_bytes(confetti, 0, CONFETTI_SIZE) };
    unsafe { set_oam_y(confetti.add(CONFETTI_OAM), DISPLAY_HEIGHT) };
    unsafe { set_oam_x(confetti.add(CONFETTI_OAM), DISPLAY_WIDTH) };
    unsafe { set_flag(confetti, FLAG_DUMMIED) };
    unsafe { copy_dummy_oam_to(oam_slot(id as usize)) };
    id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confetti_field_offsets_match_the_arm_structure() {
        assert_eq!(CONFETTI_OAM, 0);
        assert_eq!(CONFETTI_X, 8);
        assert_eq!(CONFETTI_DATA + CONFETTI_DATA_COUNT * 2, 42);
        assert_eq!(CONFETTI_CALLBACK, 44);
        // Four bytes of callback, and APCS rounds the whole thing to 48.
        assert_eq!(CONFETTI_SIZE, 48);
    }

    #[test]
    fn the_flag_byte_packs_three_bits_and_a_two_bit_priority() {
        assert_eq!(FLAG_ACTIVE | FLAG_ALLOW_UPDATES | FLAG_DUMMIED, 0b0000_0111);
        assert_eq!(PRIORITY_MASK, 0b0001_1000);
        assert_eq!((3u8 << PRIORITY_SHIFT) & PRIORITY_MASK, 0x18);
    }

    #[test]
    fn confetti_use_the_second_half_of_the_oam_buffer() {
        assert_eq!(OAM_SLOT_BASE + MAX_CONFETTI as usize, 128);
    }
}
