use crate::ffi::{
    StringGet_Nickname, VarGet, VarSet, gSpecialVar_0x8004, gSpecialVar_0x8005, gSpecialVar_0x8006,
    gSpecialVar_Result, gStringVar1,
};
use core::ffi::c_int;
use core::ptr::{addr_of, addr_of_mut};

const PARTY_SIZE: usize = 6;
const POKEMON_SIZE: usize = 100;
const TOTAL_BOXES_COUNT: usize = 14;
const IN_BOX_COUNT: usize = 30;
const BOX_POKEMON_SIZE: usize = 80;
const STORAGE_BOXES_OFFSET: usize = 4;

const VAR_POKELOT_PRIZE_ITEM: u16 = 0x4045;
const VAR_POKELOT_RND1: u16 = 0x404b;
const VAR_POKELOT_RND2: u16 = 0x404c;
const MON_DATA_OT_ID: c_int = 1;
const MON_DATA_NICKNAME: c_int = 2;
const MON_DATA_SPECIES: c_int = 11;
const MON_DATA_IS_EGG: c_int = 45;
static LOTTERY_PRIZES: [u16; 4] = [70, 183, 26, 1];

unsafe extern "C" {
    static mut gPlayerParty: u8;
    static mut gPokemonStoragePtr: *mut u8;

    fn Random() -> u16;
    fn GetMonData2(mon: *mut u8, field: c_int) -> u32;
    fn GetMonData3(mon: *mut u8, field: c_int, destination: *mut u8) -> u32;
    fn GetBoxMonData2(mon: *mut u8, field: c_int) -> u32;
    fn GetBoxMonData3(mon: *mut u8, field: c_int, destination: *mut u8) -> u32;
}

const fn randomize2(value: u32) -> u32 {
    value.wrapping_mul(1_103_515_245).wrapping_add(12_345)
}

const fn matching_digits(mut winning: u16, mut trainer_id: u16) -> u8 {
    let mut count = 0;
    while count < 5 {
        if winning % 10 != trainer_id % 10 {
            break;
        }
        winning /= 10;
        trainer_id /= 10;
        count += 1;
    }
    count
}

unsafe fn party_mon(index: usize) -> *mut u8 {
    unsafe { (&raw mut gPlayerParty).add(index * POKEMON_SIZE) }
}

unsafe fn box_mon(box_index: usize, slot: usize) -> *mut u8 {
    unsafe {
        gPokemonStoragePtr
            .add(STORAGE_BOXES_OFFSET + (box_index * IN_BOX_COUNT + slot) * BOX_POKEMON_SIZE)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetLotteryCorner() {
    let low = unsafe { Random() };
    let high = unsafe { Random() };
    unsafe { SetLotteryNumber((u32::from(high) << 16) | u32::from(low)) };
    let _ = unsafe { VarSet(VAR_POKELOT_PRIZE_ITEM, 0) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetRandomLotteryNumber(mut days: u16) {
    let mut value = u32::from(unsafe { Random() });
    while days != 0 {
        days -= 1;
        value = randomize2(value);
    }
    unsafe { SetLotteryNumber(value) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RetrieveLotteryNumber() {
    unsafe { addr_of_mut!(gSpecialVar_Result).write(GetLotteryNumber() as u16) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PickLotteryCornerTicket() {
    unsafe { addr_of_mut!(gSpecialVar_0x8004).write(0) };
    let winning = unsafe { addr_of!(gSpecialVar_Result).read() };
    let mut best_box = 0usize;
    let mut best_slot = 0usize;

    let mut index = 0usize;
    while index < PARTY_SIZE {
        let mon = unsafe { party_mon(index) };
        if unsafe { GetMonData2(mon, MON_DATA_SPECIES) } == 0 {
            break;
        }
        if unsafe { GetMonData2(mon, MON_DATA_IS_EGG) } == 0 {
            let trainer_id = unsafe { GetMonData2(mon, MON_DATA_OT_ID) } as u16;
            let count = matching_digits(winning, trainer_id);
            if count > unsafe { addr_of!(gSpecialVar_0x8004).read() } as u8 && count > 1 {
                unsafe { addr_of_mut!(gSpecialVar_0x8004).write(u16::from(count - 1)) };
                best_box = TOTAL_BOXES_COUNT;
                best_slot = index;
            }
        }
        index += 1;
    }

    let mut box_index = 0usize;
    while box_index < TOTAL_BOXES_COUNT {
        let mut slot = 0usize;
        while slot < IN_BOX_COUNT {
            let mon = unsafe { box_mon(box_index, slot) };
            if unsafe { GetBoxMonData2(mon, MON_DATA_SPECIES) } != 0
                && unsafe { GetBoxMonData2(mon, MON_DATA_IS_EGG) } == 0
            {
                let trainer_id = unsafe { GetBoxMonData2(mon, MON_DATA_OT_ID) } as u16;
                let count = matching_digits(winning, trainer_id);
                if count > unsafe { addr_of!(gSpecialVar_0x8004).read() } as u8 && count > 1 {
                    unsafe { addr_of_mut!(gSpecialVar_0x8004).write(u16::from(count - 1)) };
                    best_box = box_index;
                    best_slot = slot;
                }
            }
            slot += 1;
        }
        box_index += 1;
    }

    let prize_index = unsafe { addr_of!(gSpecialVar_0x8004).read() };
    if prize_index == 0 {
        return;
    }
    let prize = unsafe {
        (&raw const LOTTERY_PRIZES)
            .cast::<u16>()
            .add(prize_index as usize - 1)
            .read()
    };
    unsafe { addr_of_mut!(gSpecialVar_0x8005).write(prize) };
    let string = (&raw mut gStringVar1).cast::<u8>();
    if best_box == TOTAL_BOXES_COUNT {
        unsafe { addr_of_mut!(gSpecialVar_0x8006).write(0) };
        let _ = unsafe { GetMonData3(party_mon(best_slot), MON_DATA_NICKNAME, string) };
    } else {
        unsafe { addr_of_mut!(gSpecialVar_0x8006).write(1) };
        let _ = unsafe { GetBoxMonData3(box_mon(best_box, best_slot), MON_DATA_NICKNAME, string) };
    }
    let _ = unsafe { StringGet_Nickname(string) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLotteryNumber(value: u32) {
    let _ = unsafe { VarSet(VAR_POKELOT_RND1, value as u16) };
    let _ = unsafe { VarSet(VAR_POKELOT_RND2, (value >> 16) as u16) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLotteryNumber() -> u32 {
    let high = unsafe { VarGet(VAR_POKELOT_RND1) };
    let low = unsafe { VarGet(VAR_POKELOT_RND2) };
    (u32::from(low) << 16) | u32::from(high)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLotteryNumber16_Unused(value: u16) {
    unsafe { SetLotteryNumber(u32::from(value)) };
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

    #[test]
    fn storage_offsets_match_the_packed_c_structure() {
        assert_eq!(STORAGE_BOXES_OFFSET + 14 * 30 * 80, 0x8344);
    }
}
