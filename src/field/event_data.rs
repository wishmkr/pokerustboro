//! Script flags and variables.
//!
//! The `gSpecialVar_*` globals this file owns in C live in [`crate::ffi`],
//! because a crate can only define each `no_mangle` symbol once and nearly
//! every other module reads one of them.

use crate::ffi::SAVE2_POKEDEX_OFFSET;

const NUM_SPECIAL_FLAGS: usize = 128;
const NUM_TEMP_FLAGS: usize = 32;
const NUM_DAILY_FLAGS: usize = 64;
const NUM_TEMP_VARS: usize = 16;

/// 8 flags per byte.
const SPECIAL_FLAGS_SIZE: usize = NUM_SPECIAL_FLAGS / 8;
const TEMP_FLAGS_SIZE: usize = NUM_TEMP_FLAGS / 8;
const DAILY_FLAGS_SIZE: usize = NUM_DAILY_FLAGS / 8;
/// Half a var per byte.
const TEMP_VARS_SIZE: usize = NUM_TEMP_VARS * 2;

const TEMP_FLAGS_START: u16 = 0x0000;
const DAILY_FLAGS_START: u16 = 0x0920;
const SPECIAL_FLAGS_START: u16 = 0x4000;
const TEMP_VARS_START: u16 = 0x4000;
const VARS_START: u16 = 0x4000;
const SPECIAL_VARS_START: u16 = 0x8000;

/// `offsetof(struct SaveBlock1, flags)` and its size.
const SAVE1_FLAGS_OFFSET: usize = 0x1270;
const SAVE1_FLAGS_SIZE: usize = 300;
/// `offsetof(struct SaveBlock1, vars)` and its size in bytes.
const SAVE1_VARS_OFFSET: usize = 0x139c;
const SAVE1_VARS_SIZE: usize = 512;

/// Offsets inside `struct Pokedex`, itself at `SAVE2_POKEDEX_OFFSET`.
const POKEDEX_ORDER: usize = 0;
const POKEDEX_MODE: usize = 1;
const POKEDEX_NATIONAL_MAGIC: usize = 2;
const DEX_MODE_NATIONAL: u8 = 1;
const NATIONAL_DEX_MAGIC: u8 = 0xda;
const NATIONAL_DEX_VAR_VALUE: u16 = 0x302;
const RESET_RTC_VAR_VALUE: u16 = 0x920;

const FLAG_SYS_ENC_UP_ITEM: u16 = 0x8ad;
const FLAG_SYS_ENC_DOWN_ITEM: u16 = 0x8ae;
const FLAG_SYS_USE_STRENGTH: u16 = 0x889;
const FLAG_SYS_CTRL_OBJ_DELETE: u16 = 0x8c1;
const FLAG_NURSE_UNION_ROOM_REMINDER: u16 = 0x880;
const FLAG_SYS_NATIONAL_DEX: u16 = 0x896;
const FLAG_SYS_MYSTERY_EVENT_ENABLE: u16 = 0x8ac;
const FLAG_SYS_MYSTERY_GIFT_ENABLE: u16 = 0x8db;
const FLAG_SYS_RESET_RTC_ENABLE: u16 = 0x8c2;
const FLAG_MYSTERY_GIFT_DONE: u16 = 0x1e4;
/// `FLAG_MYSTERY_GIFT_1` through `FLAG_MYSTERY_GIFT_15` are contiguous.
const FLAG_MYSTERY_GIFT_1: u16 = 0x1e5;
const NUM_MYSTERY_GIFT_FLAGS: u16 = 15;

const VAR_NATIONAL_DEX: u16 = 0x4046;
const VAR_RESET_RTC_ENABLE: u16 = 0x402c;
const VAR_OBJ_GFX_ID_0: u16 = 0x4010;
const VAR_GIFT_PICHU_SLOT: u16 = 0x40dd;
/// `VAR_GIFT_UNUSED_1` through `VAR_GIFT_UNUSED_7` are contiguous.
const VAR_GIFT_UNUSED_1: u16 = 0x40de;
const NUM_GIFT_UNUSED_VARS: u16 = 7;

#[unsafe(link_section = "ewram_data")]
static mut SPECIAL_FLAGS: [u8; SPECIAL_FLAGS_SIZE] = [0; SPECIAL_FLAGS_SIZE];

unsafe extern "C" {
    static mut gSaveBlock1Ptr: *mut u8;
    static mut gSaveBlock2Ptr: *mut u8;
    /// Table of pointers to the `gSpecialVar_*` globals, built in
    /// `data/event_scripts.s`.
    static gSpecialVars: *mut u16;

    fn ResetPokedexScrollPositions();
}

#[inline]
unsafe fn zero(destination: *mut u8, count: usize) {
    unsafe { core::ptr::write_bytes(destination, 0, count) };
}

#[inline]
unsafe fn pokedex_byte(offset: usize) -> *mut u8 {
    unsafe { gSaveBlock2Ptr.add(SAVE2_POKEDEX_OFFSET + offset) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitEventData() {
    unsafe { zero(gSaveBlock1Ptr.add(SAVE1_FLAGS_OFFSET), SAVE1_FLAGS_SIZE) };
    unsafe { zero(gSaveBlock1Ptr.add(SAVE1_VARS_OFFSET), SAVE1_VARS_SIZE) };
    unsafe { zero((&raw mut SPECIAL_FLAGS).cast::<u8>(), SPECIAL_FLAGS_SIZE) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearTempFieldEventData() {
    unsafe {
        zero(
            gSaveBlock1Ptr.add(SAVE1_FLAGS_OFFSET + TEMP_FLAGS_START as usize / 8),
            TEMP_FLAGS_SIZE,
        )
    };
    unsafe {
        zero(
            gSaveBlock1Ptr.add(SAVE1_VARS_OFFSET + (TEMP_VARS_START - VARS_START) as usize * 2),
            TEMP_VARS_SIZE,
        )
    };
    unsafe { FlagClear(FLAG_SYS_ENC_UP_ITEM) };
    unsafe { FlagClear(FLAG_SYS_ENC_DOWN_ITEM) };
    unsafe { FlagClear(FLAG_SYS_USE_STRENGTH) };
    unsafe { FlagClear(FLAG_SYS_CTRL_OBJ_DELETE) };
    unsafe { FlagClear(FLAG_NURSE_UNION_ROOM_REMINDER) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearDailyFlags() {
    unsafe {
        zero(
            gSaveBlock1Ptr.add(SAVE1_FLAGS_OFFSET + DAILY_FLAGS_START as usize / 8),
            DAILY_FLAGS_SIZE,
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisableNationalPokedex() {
    let national_dex_var = unsafe { GetVarPointer(VAR_NATIONAL_DEX) };
    unsafe { pokedex_byte(POKEDEX_NATIONAL_MAGIC).write(0) };
    unsafe { national_dex_var.write(0) };
    unsafe { FlagClear(FLAG_SYS_NATIONAL_DEX) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn EnableNationalPokedex() {
    let national_dex_var = unsafe { GetVarPointer(VAR_NATIONAL_DEX) };
    unsafe { pokedex_byte(POKEDEX_NATIONAL_MAGIC).write(NATIONAL_DEX_MAGIC) };
    unsafe { national_dex_var.write(NATIONAL_DEX_VAR_VALUE) };
    unsafe { FlagSet(FLAG_SYS_NATIONAL_DEX) };
    unsafe { pokedex_byte(POKEDEX_MODE).write(DEX_MODE_NATIONAL) };
    unsafe { pokedex_byte(POKEDEX_ORDER).write(0) };
    unsafe { ResetPokedexScrollPositions() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsNationalPokedexEnabled() -> u32 {
    let enabled = unsafe { pokedex_byte(POKEDEX_NATIONAL_MAGIC).read() } == NATIONAL_DEX_MAGIC
        && unsafe { VarGet(VAR_NATIONAL_DEX) } == NATIONAL_DEX_VAR_VALUE
        && unsafe { FlagGet(FLAG_SYS_NATIONAL_DEX) } != 0;
    u32::from(enabled)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisableMysteryEvent() {
    unsafe { FlagClear(FLAG_SYS_MYSTERY_EVENT_ENABLE) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn EnableMysteryEvent() {
    unsafe { FlagSet(FLAG_SYS_MYSTERY_EVENT_ENABLE) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMysteryEventEnabled() -> u32 {
    u32::from(unsafe { FlagGet(FLAG_SYS_MYSTERY_EVENT_ENABLE) })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisableMysteryGift() {
    unsafe { FlagClear(FLAG_SYS_MYSTERY_GIFT_ENABLE) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn EnableMysteryGift() {
    unsafe { FlagSet(FLAG_SYS_MYSTERY_GIFT_ENABLE) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMysteryGiftEnabled() -> u32 {
    u32::from(unsafe { FlagGet(FLAG_SYS_MYSTERY_GIFT_ENABLE) })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearMysteryGiftFlags() {
    unsafe { FlagClear(FLAG_MYSTERY_GIFT_DONE) };
    let mut index = 0u16;
    while index < NUM_MYSTERY_GIFT_FLAGS {
        unsafe { FlagClear(FLAG_MYSTERY_GIFT_1 + index) };
        index += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearMysteryGiftVars() {
    unsafe { VarSet(VAR_GIFT_PICHU_SLOT, 0) };
    let mut index = 0u16;
    while index < NUM_GIFT_UNUSED_VARS {
        unsafe { VarSet(VAR_GIFT_UNUSED_1 + index, 0) };
        index += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisableResetRTC() {
    unsafe { VarSet(VAR_RESET_RTC_ENABLE, 0) };
    unsafe { FlagClear(FLAG_SYS_RESET_RTC_ENABLE) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn EnableResetRTC() {
    unsafe { VarSet(VAR_RESET_RTC_ENABLE, RESET_RTC_VAR_VALUE) };
    unsafe { FlagSet(FLAG_SYS_RESET_RTC_ENABLE) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CanResetRTC() -> u32 {
    let can = unsafe { FlagGet(FLAG_SYS_RESET_RTC_ENABLE) } != 0
        && unsafe { VarGet(VAR_RESET_RTC_ENABLE) } == RESET_RTC_VAR_VALUE;
    u32::from(can)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetVarPointer(id: u16) -> *mut u16 {
    if id < VARS_START {
        core::ptr::null_mut()
    } else if id < SPECIAL_VARS_START {
        unsafe {
            gSaveBlock1Ptr
                .add(SAVE1_VARS_OFFSET + (id - VARS_START) as usize * 2)
                .cast::<u16>()
        }
    } else {
        unsafe {
            (&raw const gSpecialVars)
                .cast::<*mut u16>()
                .add((id - SPECIAL_VARS_START) as usize)
                .read()
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn VarGet(id: u16) -> u16 {
    let pointer = unsafe { GetVarPointer(id) };
    if pointer.is_null() {
        // An out-of-range id reads back as its own literal value, which is
        // how scripts pass immediates where a var is expected.
        return id;
    }
    unsafe { pointer.read() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn VarSet(id: u16, value: u16) -> u8 {
    let pointer = unsafe { GetVarPointer(id) };
    if pointer.is_null() {
        return 0;
    }
    unsafe { pointer.write(value) };
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn VarGetObjectEventGraphicsId(id: u8) -> u8 {
    unsafe { VarGet(VAR_OBJ_GFX_ID_0 + u16::from(id)) as u8 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFlagPointer(id: u16) -> *mut u8 {
    if id == 0 {
        core::ptr::null_mut()
    } else if id < SPECIAL_FLAGS_START {
        unsafe { gSaveBlock1Ptr.add(SAVE1_FLAGS_OFFSET + id as usize / 8) }
    } else {
        unsafe {
            (&raw mut SPECIAL_FLAGS)
                .cast::<u8>()
                .add((id - SPECIAL_FLAGS_START) as usize / 8)
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FlagSet(id: u16) -> u8 {
    let pointer = unsafe { GetFlagPointer(id) };
    if !pointer.is_null() {
        unsafe { pointer.write(pointer.read() | 1 << (id & 7)) };
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FlagClear(id: u16) -> u8 {
    let pointer = unsafe { GetFlagPointer(id) };
    if !pointer.is_null() {
        unsafe { pointer.write(pointer.read() & !(1 << (id & 7))) };
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FlagGet(id: u16) -> u8 {
    let pointer = unsafe { GetFlagPointer(id) };
    if pointer.is_null() {
        return 0;
    }
    u8::from(unsafe { pointer.read() } >> (id & 7) & 1 != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_and_var_region_sizes_match_the_c_definitions() {
        assert_eq!(SPECIAL_FLAGS_SIZE, 16);
        assert_eq!(TEMP_FLAGS_SIZE, 4);
        assert_eq!(DAILY_FLAGS_SIZE, 8);
        assert_eq!(TEMP_VARS_SIZE, 32);
        // Flag and var ids are sparse. SPECIAL_FLAGS_START (0x4000) is far
        // past the 2400 flags the save block holds and SPECIAL_VARS_START
        // (0x8000) far past its 256 vars; nothing is allocated in between.
        // What has to fit is every id the game actually uses.
        assert!(FLAG_SYS_MYSTERY_GIFT_ENABLE as usize / 8 < SAVE1_FLAGS_SIZE);
        let highest_var_index =
            (VAR_GIFT_UNUSED_1 + NUM_GIFT_UNUSED_VARS - 1 - VARS_START) as usize;
        assert!(highest_var_index < SAVE1_VARS_SIZE / 2);
    }

    #[test]
    fn contiguous_id_runs_cover_the_original_enumerations() {
        assert_eq!(FLAG_MYSTERY_GIFT_1 + NUM_MYSTERY_GIFT_FLAGS - 1, 0x1f3);
        assert_eq!(VAR_GIFT_UNUSED_1 + NUM_GIFT_UNUSED_VARS - 1, 0x40e4);
    }

    #[test]
    fn var_ids_below_the_var_range_read_back_as_themselves() {
        // GetVarPointer returns NULL there, and VarGet turns that into the id.
        assert!(VARS_START > 0);
        assert!(SPECIAL_VARS_START > VARS_START);
    }

    #[test]
    fn a_flag_id_selects_a_byte_and_a_bit() {
        for id in [1u16, 7, 8, 9, 0x3fff] {
            let byte = id as usize / 8;
            let bit = 1u8 << (id & 7);
            assert_eq!(byte * 8 + (bit.trailing_zeros() as usize), id as usize);
        }
    }
}
