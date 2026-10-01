//! Mystery Event ribbons for the whole party (was
//! src/give_gift_ribbon_to_party.c).

use crate::box_mon::set_box_mon_data;
use crate::consts::*;
use crate::event_data::flag_set;
use crate::party::player_party;
use crate::save_blocks::save_block1;

const GIFT_RIBBONS_COUNT: usize = 11;
const MAX_GIFT_RIBBON: u8 = 64;
const FLAG_SYS_RIBBON_GET: u16 = 0x89b;

/// The Pokémon field of each gift ribbon. The last four gift ribbons are
/// unused bits with no field of their own.
const GIFT_RIBBON_FIELDS: [i32; GIFT_RIBBONS_COUNT - 4] = [
    MON_DATA_MARINE_RIBBON,
    MON_DATA_LAND_RIBBON,
    MON_DATA_SKY_RIBBON,
    MON_DATA_COUNTRY_RIBBON,
    MON_DATA_NATIONAL_RIBBON,
    MON_DATA_EARTH_RIBBON,
    MON_DATA_WORLD_RIBBON,
];

/// Records gift ribbon `index` as `ribbon_id` and gives it to every party
/// Pokémon that isn't an egg.
///
/// C reads past its field table for the four unused ribbons and sets
/// whatever field the stack holds; here those only get recorded.
pub fn give_gift_ribbon_to_party(index: u8, ribbon_id: u8) {
    let index = usize::from(index);
    if ribbon_id > MAX_GIFT_RIBBON {
        return;
    }
    // SAFETY: the save blocks are set up at boot; the borrow ends here.
    let Some(slot) = unsafe { save_block1() }.giftRibbons.0.get_mut(index) else {
        return;
    };
    *slot = ribbon_id;
    let Some(&field) = GIFT_RIBBON_FIELDS.get(index) else {
        return;
    };

    let mut got_ribbon = false;
    // SAFETY: the party isn't borrowed elsewhere; the borrow ends here.
    for mon in unsafe { player_party() }.0.iter_mut() {
        let mon = &mut mon.r#box;
        if mon.species() != 0 && mon.isEgg() == 0 {
            // SAFETY: a ribbon field takes one byte.
            unsafe { set_box_mon_data(mon, field, &1u8) };
            got_ribbon = true;
        }
    }
    if got_ribbon {
        flag_set(FLAG_SYS_RIBBON_GET);
    }
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub fn GiveGiftRibbonToParty(index: u8, ribbon_id: u8) {
    give_gift_ribbon_to_party(index, ribbon_id);
}
