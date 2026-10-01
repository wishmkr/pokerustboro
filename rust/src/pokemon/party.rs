//! The player's party and PC boxes.
//!
//! Like the save blocks (see crate::save_blocks), these are the game's
//! globals: the party is `gPlayerParty` and the PC is the block
//! `gPokemonStoragePtr` points to (the save code sets it at boot).

use crate::c::CArray;
use crate::types::{Pokemon, PokemonStorage};

pub const PARTY_SIZE: usize = 6;
pub const TOTAL_BOXES_COUNT: usize = 14;
pub const IN_BOX_COUNT: usize = 30;

/// The player's party.
///
/// # Safety
/// No other reference to the party may be in use while the result lives.
pub unsafe fn player_party<'a>() -> &'a mut CArray<Pokemon, PARTY_SIZE> {
    // SAFETY: a static; exclusivity is the caller's promise.
    unsafe { &mut *(&raw mut crate::pokemon::gPlayerParty) }
}

/// The PC's boxes.
///
/// # Safety
/// As for [`player_party`]; the storage is set up at boot.
pub unsafe fn pc_storage<'a>() -> &'a mut PokemonStorage {
    // SAFETY: set up at boot; exclusivity is the caller's promise.
    unsafe { &mut *crate::load_save::gPokemonStoragePtr.cast::<PokemonStorage>() }
}
