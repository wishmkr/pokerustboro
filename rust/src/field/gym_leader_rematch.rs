//! Gym leader rematches after the game is beaten (was
//! src/gym_leader_rematch.c).
//!
//! Now and then a leader who isn't already waiting for a rematch asks for
//! one: among those, one of the leaders with the fewest rematches fought.

use crate::battle_setup::gRematchTable;
use crate::event_data::flag_get;
use crate::random::random;
use crate::save_blocks::save_block1;

const FLAG_WATTSON_REMATCH_AVAILABLE: u16 = 0x5b;
const FLAG_SYS_GAME_CLEAR: u16 = 0x864;
const REMATCHES_COUNT: i32 = 5;

const REMATCH_ROXANNE: u16 = 65;
const REMATCH_BRAWLY: u16 = 66;
const REMATCH_WATTSON: u16 = 67;
const REMATCH_FLANNERY: u16 = 68;
const REMATCH_NORMAN: u16 = 69;
const REMATCH_WINONA: u16 = 70;
const REMATCH_TATE_AND_LIZA: u16 = 71;
const REMATCH_JUAN: u16 = 72;

const LEADERS_AFTER_NEW_MAUVILLE: [u16; 8] = [
    REMATCH_ROXANNE,
    REMATCH_BRAWLY,
    REMATCH_WATTSON,
    REMATCH_FLANNERY,
    REMATCH_NORMAN,
    REMATCH_WINONA,
    REMATCH_TATE_AND_LIZA,
    REMATCH_JUAN,
];
/// Wattson isn't available until New Mauville is done.
const LEADERS_BEFORE_NEW_MAUVILLE: [u16; 7] = [
    REMATCH_ROXANNE,
    REMATCH_BRAWLY,
    REMATCH_FLANNERY,
    REMATCH_NORMAN,
    REMATCH_WINONA,
    REMATCH_TATE_AND_LIZA,
    REMATCH_JUAN,
];

/// `HasTrainerBeenFought` with this module's view of its types.
#[inline]
unsafe fn HasTrainerBeenFought(a0: u16) -> u8 {
    unsafe { crate::battle_setup::HasTrainerBeenFought(a0) }
}

/// How many of a leader's rematches have been fought.
fn rematch_index(leader: u16) -> i32 {
    let trainers = &gRematchTable[leader].trainerIds;
    // SAFETY: HasTrainerBeenFought only reads a flag.
    trainers
        .0
        .iter()
        .position(|&id| unsafe { HasTrainerBeenFought(id) } == 0)
        .map_or(REMATCHES_COUNT, |i| i as i32)
}

/// Whether a leader is already waiting for a rematch.
fn wants_rematch(leader: u16) -> bool {
    // SAFETY: the save blocks are set up at boot; the borrow ends here.
    unsafe { save_block1() }.trainerRematches[leader] != 0
}

fn update_gym_leader_rematch_from(leaders: &[u16], max_rematch: i32) {
    let idle = || leaders.iter().copied().filter(|&l| !wants_rematch(l));
    let Some(lowest) = idle().map(rematch_index).min() else {
        return;
    };
    if lowest > max_rematch {
        return;
    }
    let behind = || idle().filter(|&l| rematch_index(l) == lowest);
    let count = behind().count() as i32;
    if count == 0 {
        return;
    }
    let chosen = i32::from(random()) % count;
    if let Some(leader) = behind().nth(chosen as usize) {
        // SAFETY: as in wants_rematch.
        unsafe { save_block1() }.trainerRematches[leader] = lowest as u8;
    }
}

pub fn update_gym_leader_rematch() {
    if !flag_get(FLAG_SYS_GAME_CLEAR) || random() % 100 > 30 {
        return;
    }
    if flag_get(FLAG_WATTSON_REMATCH_AVAILABLE) {
        update_gym_leader_rematch_from(&LEADERS_AFTER_NEW_MAUVILLE, 5);
    } else {
        update_gym_leader_rematch_from(&LEADERS_BEFORE_NEW_MAUVILLE, 1);
    }
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub fn UpdateGymLeaderRematch() {
    update_gym_leader_rematch();
}
