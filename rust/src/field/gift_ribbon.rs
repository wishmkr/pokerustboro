use crate::ffi::FlagSet;
use core::ffi::{c_int, c_void};

const POKEMON_SIZE: usize = 100;
const GIFT_RIBBONS_COUNT: usize = 11;
const MAX_GIFT_RIBBON: u8 = 64;
const FLAG_SYS_RIBBON_GET: u16 = 0x89b;
const MON_DATA_SPECIES: c_int = 11;
const MON_DATA_SANITY_IS_EGG: c_int = 6;
const RIBBON_FIELDS: [c_int; 7] = [72, 73, 74, 75, 76, 77, 78];

#[repr(C)]
struct SaveBlock1GiftRibbonView {
    prefix: [u8; 0x31a8],
    gift_ribbons: [u8; GIFT_RIBBONS_COUNT],
}

unsafe extern "C" {
    static mut gSaveBlock1Ptr: *mut SaveBlock1GiftRibbonView;
    static mut gPlayerParty: u8;

    fn GetMonData2(mon: *mut u8, field: c_int) -> u32;
    fn SetMonData(mon: *mut u8, field: c_int, value: *const c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveGiftRibbonToParty(index: u8, ribbon_id: u8) {
    let index = usize::from(index);
    if index >= GIFT_RIBBONS_COUNT || ribbon_id > MAX_GIFT_RIBBON {
        return;
    }

    let save = unsafe { gSaveBlock1Ptr };
    unsafe {
        (&raw mut (*save).gift_ribbons)
            .cast::<u8>()
            .add(index)
            .write(ribbon_id)
    };

    // Indices 7..10 are reserved bits and have no corresponding mon-data field.
    if index >= RIBBON_FIELDS.len() {
        return;
    }

    let data = 1_u8;
    let mut got_ribbon = false;
    let party = &raw mut gPlayerParty;
    let mut party_index = 0;
    while party_index < 6 {
        let mon = unsafe { party.add(party_index * POKEMON_SIZE) };
        if unsafe { GetMonData2(mon, MON_DATA_SPECIES) } != 0
            && unsafe { GetMonData2(mon, MON_DATA_SANITY_IS_EGG) } == 0
        {
            unsafe {
                SetMonData(
                    mon,
                    RIBBON_FIELDS[index],
                    (&raw const data).cast::<c_void>(),
                )
            };
            got_ribbon = true;
        }
        party_index += 1;
    }

    if got_ribbon {
        let _ = unsafe { FlagSet(FLAG_SYS_RIBBON_GET) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_offset_and_ribbon_ids_match_c_data() {
        assert_eq!(
            core::mem::offset_of!(SaveBlock1GiftRibbonView, gift_ribbons),
            0x31a8
        );
        assert_eq!(RIBBON_FIELDS, [72, 73, 74, 75, 76, 77, 78]);
    }
}
