//! Script flags and variables (was src/event_data.c).
//!
//! Flags are bits, variables are `u16`s, both addressed by id (`FLAG_*`,
//! `VAR_*`):
//!
//! | ids | what | where |
//! |---|---|---|
//! | flags `1..0x4000` | saved flags | `SaveBlock1::flags`, one bit each |
//! | flags `0x4000..0x4080` | special flags, reset at boot | this module |
//! | vars `0x4000..0x4100` | saved variables | `SaveBlock1::vars` |
//! | vars `0x8000..=0x8015` | special variables (`gSpecialVar_*`) | globals, reached through `gSpecialVars` |
//!
//! [`flag_get`], [`flag_set`], [`var_get`], [`var_set`] and the rest are the
//! safe API: they ignore ids outside the table above. The C names
//! (`FlagGet`, `VarSet`...) keep C's behaviour for any id, including
//! reading and writing past the end of the save's tables for invalid ones,
//! so that the game behaves exactly as before for code that still calls them.
//!
//! The `gSpecialVar_*` globals themselves are defined in [`crate::ffi`]: a
//! crate can define each `no_mangle` symbol only once and nearly every
//! module reads one of them.

use crate::c::CArray;
use crate::global::Global;
use crate::save_blocks::{save_block1, save_block2};

const SPECIAL_FLAGS_START: u16 = 0x4000;
const NUM_SPECIAL_FLAGS: u16 = 0x80;
const VARS_START: u16 = 0x4000;
const NUM_VARS: u16 = 0x100;
const SPECIAL_VARS_START: u16 = 0x8000;
const NUM_SPECIAL_VARS: u16 = 0x16;

/// The length of `SaveBlock1::flags`.
const SAVED_FLAG_BYTES: usize = 300;

const TEMP_FLAGS_START: u16 = 0x0000;
const NUM_TEMP_FLAGS: u16 = 32;
const DAILY_FLAGS_START: u16 = 0x0920;
const NUM_DAILY_FLAGS: u16 = 64;
const TEMP_VARS_START: u16 = 0x4000;
const NUM_TEMP_VARS: u16 = 16;

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

/// Written to the Pokédex and to `VAR_NATIONAL_DEX` when the National Dex
/// is enabled; all three must agree for it to count as enabled.
const NATIONAL_DEX_MAGIC: u8 = 0xda;
const NATIONAL_DEX_VAR_VALUE: u16 = 0x302;
const DEX_MODE_NATIONAL: u8 = 1;
const RESET_RTC_VAR_VALUE: u16 = 0x920;

/// Flags 0x4000..0x4080, not saved.
#[unsafe(link_section = "ewram_data")]
static SPECIAL_FLAGS: Global<CArray<u8, { NUM_SPECIAL_FLAGS as usize / 8 }>> =
    Global::new(CArray([0; NUM_SPECIAL_FLAGS as usize / 8]));

/// `ResetPokedexScrollPositions` with this module's view of its types.
#[inline]
unsafe fn ResetPokedexScrollPositions() {
    {
        crate::pokedex::ResetPokedexScrollPositions();
    }
}

// -------------------------------------------------------------------- flags

/// Where a flag is kept.
#[derive(Clone, Copy)]
enum FlagByte {
    /// byte of `SaveBlock1::flags`
    Saved(usize),
    /// byte of [`SPECIAL_FLAGS`]
    Special(usize),
}

/// The byte holding flag `id` (bit `id % 8`), if `id` is a valid flag.
fn flag_byte(id: u16) -> Option<FlagByte> {
    match id {
        0 => None,
        1..SPECIAL_FLAGS_START => {
            let byte = usize::from(id / 8);
            (byte < SAVED_FLAG_BYTES).then_some(FlagByte::Saved(byte))
        }
        _ if id < SPECIAL_FLAGS_START + NUM_SPECIAL_FLAGS => Some(FlagByte::Special(usize::from(
            (id - SPECIAL_FLAGS_START) / 8,
        ))),
        _ => None,
    }
}

fn update_flag_byte(byte: FlagByte, f: impl FnOnce(u8) -> u8) {
    match byte {
        // SAFETY: the save blocks are set up at boot; the borrow ends here.
        FlagByte::Saved(i) => {
            let flags = &mut unsafe { save_block1() }.flags;
            flags[i] = f(flags[i]);
        }
        FlagByte::Special(i) => {
            SPECIAL_FLAGS.update(|mut flags| {
                flags[i] = f(flags[i]);
                flags
            });
        }
    }
}

fn read_flag_byte(byte: FlagByte) -> u8 {
    match byte {
        // SAFETY: as in update_flag_byte.
        FlagByte::Saved(i) => unsafe { save_block1() }.flags[i],
        FlagByte::Special(i) => SPECIAL_FLAGS.get()[i],
    }
}

/// Whether flag `id` is set (false for an invalid id).
pub fn flag_get(id: u16) -> bool {
    flag_byte(id).is_some_and(|byte| read_flag_byte(byte) & 1 << (id % 8) != 0)
}

/// Sets flag `id` (nothing for an invalid id).
pub fn flag_set(id: u16) {
    if let Some(byte) = flag_byte(id) {
        update_flag_byte(byte, |b| b | 1 << (id % 8));
    }
}

/// Clears flag `id` (nothing for an invalid id).
pub fn flag_clear(id: u16) {
    if let Some(byte) = flag_byte(id) {
        update_flag_byte(byte, |b| b & !(1 << (id % 8)));
    }
}

// --------------------------------------------------------------------- vars

/// Variable `id`: a saved one (index into `SaveBlock1::vars`) or a special
/// one (index into `gSpecialVars`).
#[derive(Clone, Copy)]
enum VarSlot {
    Saved(usize),
    Special(usize),
}

fn var_slot(id: u16) -> Option<VarSlot> {
    match id {
        VARS_START..SPECIAL_VARS_START if id < VARS_START + NUM_VARS => {
            Some(VarSlot::Saved(usize::from(id - VARS_START)))
        }
        SPECIAL_VARS_START.. if id < SPECIAL_VARS_START + NUM_SPECIAL_VARS => {
            Some(VarSlot::Special(usize::from(id - SPECIAL_VARS_START)))
        }
        _ => None,
    }
}

/// `gSpecialVars`: pointers to the `gSpecialVar_*` globals, in id order
/// (data/event_scripts.s, now crate::asmdata).
fn special_vars() -> *const *mut u16 {
    crate::asmdata::gSpecialVars.cast()
}

fn var_ptr(slot: VarSlot) -> *mut u16 {
    match slot {
        // SAFETY: the save blocks are set up at boot; no reference is kept.
        VarSlot::Saved(i) => &raw mut unsafe { save_block1() }.vars[i],
        // SAFETY: `i` is within the table, whose entries point to globals.
        VarSlot::Special(i) => unsafe { special_vars().add(usize::from(i)).read() },
    }
}

/// The value of variable `id`. An id that isn't a variable reads as itself:
/// that's how scripts pass a number where a variable is expected.
pub fn var_get(id: u16) -> u16 {
    match var_slot(id) {
        // SAFETY: var_ptr gives a valid pointer for a valid slot.
        Some(slot) => unsafe { var_ptr(slot).read() },
        None => id,
    }
}

/// Sets variable `id`. Returns false for an id that isn't a variable.
pub fn var_set(id: u16, value: u16) -> bool {
    match var_slot(id) {
        Some(slot) => {
            // SAFETY: as in var_get.
            unsafe { var_ptr(slot).write(value) };
            true
        }
        None => false,
    }
}

// ------------------------------------------------------------------ resets

/// Clears all flags and variables (new game).
pub fn init_event_data() {
    // SAFETY: the save blocks are set up at boot; the borrow ends here.
    let save = unsafe { save_block1() };
    save.flags.0.fill(0);
    save.vars.0.fill(0);
    SPECIAL_FLAGS.set(CArray([0; NUM_SPECIAL_FLAGS as usize / 8]));
}

/// Clears the temporary flags and variables and a few system flags (on
/// every map change).
pub fn clear_temp_field_event_data() {
    {
        // SAFETY: as in init_event_data.
        let save = unsafe { save_block1() };
        let temp_flags =
            usize::from(TEMP_FLAGS_START / 8)..usize::from((TEMP_FLAGS_START + NUM_TEMP_FLAGS) / 8);
        save.flags.0[temp_flags].fill(0);
        let temp_vars = usize::from(TEMP_VARS_START - VARS_START)
            ..usize::from(TEMP_VARS_START - VARS_START + NUM_TEMP_VARS);
        save.vars.0[temp_vars].fill(0);
    }
    for flag in [
        FLAG_SYS_ENC_UP_ITEM,
        FLAG_SYS_ENC_DOWN_ITEM,
        FLAG_SYS_USE_STRENGTH,
        FLAG_SYS_CTRL_OBJ_DELETE,
        FLAG_NURSE_UNION_ROOM_REMINDER,
    ] {
        flag_clear(flag);
    }
}

/// Clears the flags that reset every day.
pub fn clear_daily_flags() {
    // SAFETY: as in init_event_data.
    let save = unsafe { save_block1() };
    let daily =
        usize::from(DAILY_FLAGS_START / 8)..usize::from((DAILY_FLAGS_START + NUM_DAILY_FLAGS) / 8);
    save.flags.0[daily].fill(0);
}

// --------------------------------------------------------- National Pokédex

pub fn disable_national_pokedex() {
    // SAFETY: the save blocks are set up at boot; the borrow ends here.
    unsafe { save_block2() }.pokedex.nationalMagic = 0;
    var_set(VAR_NATIONAL_DEX, 0);
    flag_clear(FLAG_SYS_NATIONAL_DEX);
}

pub fn enable_national_pokedex() {
    {
        // SAFETY: as in disable_national_pokedex.
        let dex = &mut unsafe { save_block2() }.pokedex;
        dex.nationalMagic = NATIONAL_DEX_MAGIC;
    }
    var_set(VAR_NATIONAL_DEX, NATIONAL_DEX_VAR_VALUE);
    flag_set(FLAG_SYS_NATIONAL_DEX);
    {
        // SAFETY: as above.
        let dex = &mut unsafe { save_block2() }.pokedex;
        dex.mode = DEX_MODE_NATIONAL;
        dex.order = 0;
    }
    // SAFETY: a plain C function with no preconditions.
    unsafe { ResetPokedexScrollPositions() };
}

pub fn is_national_pokedex_enabled() -> bool {
    // SAFETY: as in disable_national_pokedex.
    let magic = unsafe { save_block2() }.pokedex.nationalMagic;
    magic == NATIONAL_DEX_MAGIC
        && var_get(VAR_NATIONAL_DEX) == NATIONAL_DEX_VAR_VALUE
        && flag_get(FLAG_SYS_NATIONAL_DEX)
}

// -------------------------------------------- Mystery Event / Gift, RTC reset

pub fn clear_mystery_gift_flags() {
    flag_clear(FLAG_MYSTERY_GIFT_DONE);
    for flag in FLAG_MYSTERY_GIFT_1..FLAG_MYSTERY_GIFT_1 + NUM_MYSTERY_GIFT_FLAGS {
        flag_clear(flag);
    }
}

pub fn clear_mystery_gift_vars() {
    var_set(VAR_GIFT_PICHU_SLOT, 0);
    for var in VAR_GIFT_UNUSED_1..VAR_GIFT_UNUSED_1 + NUM_GIFT_UNUSED_VARS {
        var_set(var, 0);
    }
}

pub fn enable_reset_rtc(enable: bool) {
    var_set(
        VAR_RESET_RTC_ENABLE,
        if enable { RESET_RTC_VAR_VALUE } else { 0 },
    );
    if enable {
        flag_set(FLAG_SYS_RESET_RTC_ENABLE);
    } else {
        flag_clear(FLAG_SYS_RESET_RTC_ENABLE);
    }
}

pub fn can_reset_rtc() -> bool {
    flag_get(FLAG_SYS_RESET_RTC_ENABLE) && var_get(VAR_RESET_RTC_ENABLE) == RESET_RTC_VAR_VALUE
}

// ------------------------------------------------------------------ C names
//
// The flag and var functions below keep C's exact behaviour for every id
// (a pointer computed without bounds checks), for callers that may rely on
// it; the rest only forward to the safe functions.

/// C's `GetFlagPointer`.
#[unsafe(no_mangle)]
pub unsafe fn GetFlagPointer(id: u16) -> *mut u8 {
    if id == 0 {
        core::ptr::null_mut()
    } else if id < SPECIAL_FLAGS_START {
        unsafe { save_block1() }
            .flags
            .as_mut_ptr()
            .wrapping_add(usize::from(id / 8))
    } else {
        SPECIAL_FLAGS
            .as_ptr()
            .cast::<u8>()
            .wrapping_add(usize::from((id - SPECIAL_FLAGS_START) / 8))
    }
}

/// C's `GetVarPointer`.
#[unsafe(no_mangle)]
pub unsafe fn GetVarPointer(id: u16) -> *mut u16 {
    if id < VARS_START {
        core::ptr::null_mut()
    } else if id < SPECIAL_VARS_START {
        unsafe { save_block1() }
            .vars
            .as_mut_ptr()
            .wrapping_add(usize::from(id - VARS_START))
    } else {
        unsafe {
            special_vars()
                .add(usize::from(id - SPECIAL_VARS_START))
                .read()
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe fn FlagSet(id: u16) -> u8 {
    let p = unsafe { GetFlagPointer(id) };
    if !p.is_null() {
        unsafe { *p |= 1 << (id & 7) };
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe fn FlagClear(id: u16) -> u8 {
    let p = unsafe { GetFlagPointer(id) };
    if !p.is_null() {
        unsafe { *p &= !(1 << (id & 7)) };
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe fn FlagGet(id: u16) -> u8 {
    let p = unsafe { GetFlagPointer(id) };
    if p.is_null() {
        return 0;
    }
    (unsafe { *p } >> (id & 7) & 1).into()
}

#[unsafe(no_mangle)]
pub unsafe fn VarGet(id: u16) -> u16 {
    let p = unsafe { GetVarPointer(id) };
    if p.is_null() { id } else { unsafe { *p } }
}

#[unsafe(no_mangle)]
pub unsafe fn VarSet(id: u16, value: u16) -> u8 {
    let p = unsafe { GetVarPointer(id) };
    if p.is_null() {
        return 0;
    }
    unsafe { *p = value };
    1
}

#[unsafe(no_mangle)]
pub unsafe fn VarGetObjectEventGraphicsId(id: u8) -> u8 {
    unsafe { VarGet(VAR_OBJ_GFX_ID_0 + u16::from(id)) as u8 }
}

#[unsafe(no_mangle)]
pub fn InitEventData() {
    init_event_data();
}

#[unsafe(no_mangle)]
pub fn ClearTempFieldEventData() {
    clear_temp_field_event_data();
}

#[unsafe(no_mangle)]
pub fn ClearDailyFlags() {
    clear_daily_flags();
}

#[unsafe(no_mangle)]
pub fn DisableNationalPokedex() {
    disable_national_pokedex();
}

#[unsafe(no_mangle)]
pub fn EnableNationalPokedex() {
    enable_national_pokedex();
}

#[unsafe(no_mangle)]
pub fn IsNationalPokedexEnabled() -> u32 {
    is_national_pokedex_enabled().into()
}

#[unsafe(no_mangle)]
pub fn DisableMysteryEvent() {
    flag_clear(FLAG_SYS_MYSTERY_EVENT_ENABLE);
}

#[unsafe(no_mangle)]
pub fn EnableMysteryEvent() {
    flag_set(FLAG_SYS_MYSTERY_EVENT_ENABLE);
}

#[unsafe(no_mangle)]
pub fn IsMysteryEventEnabled() -> u32 {
    flag_get(FLAG_SYS_MYSTERY_EVENT_ENABLE).into()
}

#[unsafe(no_mangle)]
pub fn DisableMysteryGift() {
    flag_clear(FLAG_SYS_MYSTERY_GIFT_ENABLE);
}

#[unsafe(no_mangle)]
pub fn EnableMysteryGift() {
    flag_set(FLAG_SYS_MYSTERY_GIFT_ENABLE);
}

#[unsafe(no_mangle)]
pub fn IsMysteryGiftEnabled() -> u32 {
    flag_get(FLAG_SYS_MYSTERY_GIFT_ENABLE).into()
}

#[unsafe(no_mangle)]
pub fn ClearMysteryGiftFlags() {
    clear_mystery_gift_flags();
}

#[unsafe(no_mangle)]
pub fn ClearMysteryGiftVars() {
    clear_mystery_gift_vars();
}

#[unsafe(no_mangle)]
pub fn DisableResetRTC() {
    enable_reset_rtc(false);
}

#[unsafe(no_mangle)]
pub fn EnableResetRTC() {
    enable_reset_rtc(true);
}

#[unsafe(no_mangle)]
pub fn CanResetRTC() -> u32 {
    can_reset_rtc().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_ids_map_to_their_byte() {
        assert!(flag_byte(0).is_none());
        assert!(matches!(
            flag_byte(SPECIAL_FLAGS_START),
            Some(FlagByte::Special(0))
        ));
        assert!(matches!(
            flag_byte(SPECIAL_FLAGS_START + 0x7f),
            Some(FlagByte::Special(15))
        ));
        assert!(flag_byte(SPECIAL_FLAGS_START + 0x80).is_none());
    }

    #[test]
    fn var_ids_map_to_their_slot() {
        assert!(var_slot(0x3fff).is_none());
        assert!(matches!(var_slot(0x4000), Some(VarSlot::Saved(0))));
        assert!(matches!(var_slot(0x40ff), Some(VarSlot::Saved(255))));
        assert!(var_slot(0x4100).is_none());
        assert!(matches!(var_slot(0x8015), Some(VarSlot::Special(21))));
        assert!(var_slot(0x8016).is_none());
    }

    #[test]
    fn a_number_passed_as_a_var_reads_as_itself() {
        assert_eq!(var_get(123), 123);
        assert_eq!(var_get(0x5000), 0x5000);
    }

    #[test]
    fn special_flags_can_be_set_and_cleared() {
        let id = SPECIAL_FLAGS_START + 9;
        flag_set(id);
        assert!(flag_get(id));
        assert!(!flag_get(id + 1));
        flag_clear(id);
        assert!(!flag_get(id));
    }
}
