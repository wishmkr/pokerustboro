//! Where the game continues from (was src/save_location.c): saving in a
//! Pokémon Center or a link lobby sets a flag so that continuing puts the
//! player back there properly.

use crate::save_blocks::{save_block1, save_block2};

const POKECENTER_SAVEWARP: u8 = 1 << 1;
const LOBBY_SAVEWARP: u8 = 1 << 2;
const UNK_SPECIAL_SAVE_WARP_FLAG_3: u8 = 1 << 3;
const CHAMPION_SAVEWARP: u8 = 1 << 7;
/// What SetUnlockedPokedexFlags sets in `gcnLinkFlags`.
const UNLOCKED_POKEDEX_FLAGS: u32 = 0x803f;

/// Maps as `group << 8 | num`: every Pokémon Center floor (and the Battle
/// Frontier's two).
const POKEMON_CENTER_MAPS: [u16; 38] = [
    0x0202, 0x0203, 0x0301, 0x0302, 0x0405, 0x0406, 0x0504, 0x0505, 0x0604, 0x0605, 0x0700, 0x0701,
    0x0804, 0x0805, 0x090b, 0x090c, 0x0a05, 0x0a06, 0x0b05, 0x0b06, 0x0c02, 0x0c03, 0x0d06, 0x0d07,
    0x0e03, 0x0e04, 0x0f02, 0x0f03, 0x100c, 0x100d, 0x100a, 0x100e, 0x1a35, 0x1a36, 0x1918, 0x1919,
    0x191a, 0x191b,
];
/// The Trade Center.
const RELOAD_MAPS: [u16; 1] = [0x1a05];

fn current_map() -> u16 {
    // SAFETY: the save blocks are set up at boot; the borrow ends here.
    let location = &unsafe { save_block1() }.location;
    u16::from(location.mapGroup as u8) << 8 | u16::from(location.mapNum as u8)
}

fn set_save_warp_flag(flag: u8, on: bool) {
    // SAFETY: as in current_map.
    let flags = &mut unsafe { save_block2() }.specialSaveWarpFlags;
    if on {
        *flags |= flag;
    } else {
        *flags &= !flag;
    }
}

pub fn try_set_map_save_warp_status() {
    let map = current_map();
    set_save_warp_flag(POKECENTER_SAVEWARP, POKEMON_CENTER_MAPS.contains(&map));
    set_save_warp_flag(LOBBY_SAVEWARP, RELOAD_MAPS.contains(&map));
    set_save_warp_flag(UNK_SPECIAL_SAVE_WARP_FLAG_3, false);
}

pub fn set_unlocked_pokedex_flags() {
    // SAFETY: as in current_map.
    unsafe { save_block2() }.gcnLinkFlags |= UNLOCKED_POKEDEX_FLAGS;
}

pub fn set_champion_save_warp() {
    set_save_warp_flag(CHAMPION_SAVEWARP, true);
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub fn TrySetMapSaveWarpStatus() {
    try_set_map_save_warp_status();
}

#[unsafe(no_mangle)]
pub fn SetUnlockedPokedexFlags() {
    set_unlocked_pokedex_flags();
}

#[unsafe(no_mangle)]
pub fn SetChampionSaveWarp() {
    set_champion_save_warp();
}
