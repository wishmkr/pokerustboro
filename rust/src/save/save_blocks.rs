//! Access to the save blocks.
//!
//! The game state that gets saved lives in two structs in EWRAM,
//! `SaveBlock1` (the world: flags, vars, items, map state...) and
//! `SaveBlock2` (the player: name, options, Pokédex, play time...). The save
//! code (load_save.rs) keeps them behind `gSaveBlock1Ptr` and
//! `gSaveBlock2Ptr`, which it sets at boot before any game code runs and
//! moves only while nothing else runs (when it shuffles the blocks around to
//! defeat memory-editing cheats).

use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::types::{SaveBlock1, SaveBlock2};

/// The first save block.
///
/// # Safety
/// No other reference to the block may be in use while the result lives
/// (keep it short: within one function, not across calls into other game
/// code that might take its own).
pub unsafe fn save_block1<'a>() -> &'a mut SaveBlock1 {
    // SAFETY: set up at boot (see the module documentation); exclusivity is
    // the caller's promise.
    unsafe { &mut *gSaveBlock1Ptr }
}

/// The second save block.
///
/// # Safety
/// As for [`save_block1`].
pub unsafe fn save_block2<'a>() -> &'a mut SaveBlock2 {
    // SAFETY: as in save_block1.
    unsafe { &mut *gSaveBlock2Ptr }
}
