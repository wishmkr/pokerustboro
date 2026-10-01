//! The Lilycove Department Store's lottery corner (was src/lottery_corner.c).
//!
//! Each day draws a five-digit number. The player wins with a Pokémon (in the
//! party or the PC, not an egg) whose trainer id ends in the same digits:
//! the more digits match, the better the prize.

use crate::box_mon::POKEMON_NAME_LENGTH;
use crate::event_data::{var_get, var_set};
use crate::party::{IN_BOX_COUNT, PARTY_SIZE, TOTAL_BOXES_COUNT, pc_storage, player_party};
use crate::random::random;
use crate::types::BoxPokemon;

const VAR_POKELOT_PRIZE_ITEM: u16 = 0x4045;
const VAR_POKELOT_RND1: u16 = 0x404b;
const VAR_POKELOT_RND2: u16 = 0x404c;
const VAR_0X8004: u16 = 0x8004;
const VAR_0X8005: u16 = 0x8005;
const VAR_0X8006: u16 = 0x8006;
const VAR_RESULT: u16 = 0x800d;

const ITEM_NONE: u16 = 0;
/// Prizes for 2, 3, 4 and 5 matching digits: PP Up, Exp. Share, Max Revive,
/// Master Ball.
const LOTTERY_PRIZES: [u16; 4] = [70, 183, 26, 1];

/// How many trailing decimal digits (up to 5) two numbers share.
pub fn matching_digits(mut winning: u16, mut ot_id: u16) -> u8 {
    let mut count = 0;
    while count < 5 && winning % 10 == ot_id % 10 {
        winning /= 10;
        ot_id /= 10;
        count += 1;
    }
    count
}

pub fn set_lottery_number(number: u32) {
    var_set(VAR_POKELOT_RND1, number as u16);
    var_set(VAR_POKELOT_RND2, (number >> 16) as u16);
}

pub fn lottery_number() -> u32 {
    u32::from(var_get(VAR_POKELOT_RND2)) << 16 | u32::from(var_get(VAR_POKELOT_RND1))
}

/// A new random number and no prize waiting (a new game).
pub fn reset_lottery_corner() {
    let low = random();
    let high = random();
    set_lottery_number(u32::from(high) << 16 | u32::from(low));
    var_set(VAR_POKELOT_PRIZE_ITEM, ITEM_NONE);
}

/// The number after `days` days: from a random start, advanced by the second
/// ISO C generator once a day.
pub fn set_random_lottery_number(days: u16) {
    let mut number = u32::from(random());
    for _ in 0..days {
        number = number.wrapping_mul(1_103_515_245).wrapping_add(12_345);
    }
    set_lottery_number(number);
}

/// Where the best ticket was found.
enum Winner {
    Party(usize),
    Box(usize, usize),
}

/// Script special: checks every Pokémon's trainer id against the number in
/// `VAR_RESULT`. Sets `VAR_0X8004` to the prize rank (0: none), `VAR_0X8005`
/// to the prize item, `VAR_0X8006` to 0 (party) or 1 (PC), and puts the
/// winning Pokémon's name in `gStringVar1`.
pub fn pick_lottery_corner_ticket() {
    let winning = var_get(VAR_RESULT);
    let mut rank = 0u16;
    let mut winner = Winner::Box(0, 0);
    let mut check = |mon: &mut BoxPokemon, place: Winner| {
        if mon.species() != 0 && !mon.is_egg() {
            let digits = matching_digits(winning, mon.otId as u16);
            if u16::from(digits) > rank && digits > 1 {
                rank = u16::from(digits) - 1;
                winner = place;
            }
        }
    };

    // SAFETY: nothing else uses the party or the PC during this special.
    let party = unsafe { player_party() };
    for i in 0..PARTY_SIZE {
        let mon = &mut party[i].r#box;
        if mon.species() == 0 {
            break; // the party is filled from the front
        }
        check(mon, Winner::Party(i));
    }
    // SAFETY: as above.
    let storage = unsafe { pc_storage() };
    for b in 0..TOTAL_BOXES_COUNT {
        for slot in 0..IN_BOX_COUNT {
            check(&mut storage.boxes[b][slot], Winner::Box(b, slot));
        }
    }

    var_set(VAR_0X8004, rank);
    if rank == 0 {
        return;
    }
    var_set(
        VAR_0X8005,
        LOTTERY_PRIZES
            .get(usize::from(rank) - 1)
            .copied()
            .unwrap_or(ITEM_NONE),
    );
    let mut name = [0u8; POKEMON_NAME_LENGTH + 1];
    match winner {
        Winner::Party(i) => {
            var_set(VAR_0X8006, 0);
            party[i].r#box.nickname(&mut name);
        }
        Winner::Box(b, slot) => {
            var_set(VAR_0X8006, 1);
            storage.boxes[b][slot].nickname(&mut name);
        }
    }
    // SAFETY: gStringVar1 is a 256-byte string buffer, used only here now.
    unsafe {
        let buffer = &mut (*(&raw mut crate::string_util::gStringVar1)).0.0;
        for (dst, src) in buffer.iter_mut().zip(name) {
            *dst = src;
        }
        StringGet_Nickname(buffer.as_mut_ptr());
    }
}

/// `StringGet_Nickname` with this module's view of its types.
#[inline]
unsafe fn StringGet_Nickname(a0: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringGet_Nickname(a0 as _) as *mut u8 }
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub fn ResetLotteryCorner() {
    reset_lottery_corner();
}

#[unsafe(no_mangle)]
pub fn SetRandomLotteryNumber(days: u16) {
    set_random_lottery_number(days);
}

#[unsafe(no_mangle)]
pub fn RetrieveLotteryNumber() {
    var_set(VAR_RESULT, lottery_number() as u16);
}

#[unsafe(no_mangle)]
pub fn PickLotteryCornerTicket() {
    pick_lottery_corner_ticket();
}

#[unsafe(no_mangle)]
pub fn SetLotteryNumber(number: u32) {
    set_lottery_number(number);
}

#[unsafe(no_mangle)]
pub fn GetLotteryNumber() -> u32 {
    lottery_number()
}

#[unsafe(no_mangle)]
pub fn SetLotteryNumber16_Unused(number: u16) {
    set_lottery_number(number.into());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching_compares_decimal_digits_from_the_right() {
        assert_eq!(matching_digits(12_345, 52_345), 4);
        assert_eq!(matching_digits(12_345, 12_345), 5);
        assert_eq!(matching_digits(12_345, 12_346), 0);
    }
}
