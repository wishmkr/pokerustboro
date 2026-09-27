use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_0x8005, gSpecialVar_0x8006};
use core::ptr::{addr_of, addr_of_mut};

const FLAG_GET_SEEN: u8 = 0;
const FLAG_GET_CAUGHT: u8 = 1;
const SPECIES_JIRACHI: u16 = 409;
const SPECIES_DEOXYS: u16 = 410;
const HOENN_DEX_COUNT: u16 = 202;

unsafe extern "C" {

    static gBirchDexRatingText_LessThan10: u8;
    static gBirchDexRatingText_LessThan20: u8;
    static gBirchDexRatingText_LessThan30: u8;
    static gBirchDexRatingText_LessThan40: u8;
    static gBirchDexRatingText_LessThan50: u8;
    static gBirchDexRatingText_LessThan60: u8;
    static gBirchDexRatingText_LessThan70: u8;
    static gBirchDexRatingText_LessThan80: u8;
    static gBirchDexRatingText_LessThan90: u8;
    static gBirchDexRatingText_LessThan100: u8;
    static gBirchDexRatingText_LessThan110: u8;
    static gBirchDexRatingText_LessThan120: u8;
    static gBirchDexRatingText_LessThan130: u8;
    static gBirchDexRatingText_LessThan140: u8;
    static gBirchDexRatingText_LessThan150: u8;
    static gBirchDexRatingText_LessThan160: u8;
    static gBirchDexRatingText_LessThan170: u8;
    static gBirchDexRatingText_LessThan180: u8;
    static gBirchDexRatingText_LessThan190: u8;
    static gBirchDexRatingText_LessThan200: u8;
    static gBirchDexRatingText_DexCompleted: u8;

    fn GetHoennPokedexCount(case_id: u8) -> u16;
    fn GetNationalPokedexCount(case_id: u8) -> u16;
    fn IsNationalPokedexEnabled() -> u32;
    fn SpeciesToNationalPokedexNum(species: u16) -> u16;
    fn GetSetPokedexFlag(national_dex_number: u16, case_id: u8) -> i8;
    fn ShowFieldMessage(message: *const u8) -> u8;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RatingKind {
    Band(u8),
    NeedsMythicalCheck,
    NeedsBothMythicalsCheck,
    Complete,
    Fallback,
}

const fn rating_kind(count: u16) -> RatingKind {
    if count < 200 {
        RatingKind::Band((count / 10) as u8)
    } else if count == 200 {
        RatingKind::NeedsMythicalCheck
    } else if count == HOENN_DEX_COUNT - 1 {
        RatingKind::NeedsBothMythicalsCheck
    } else if count == HOENN_DEX_COUNT {
        RatingKind::Complete
    } else {
        RatingKind::Fallback
    }
}

unsafe fn band_text(band: u8) -> *const u8 {
    match band {
        0 => addr_of!(gBirchDexRatingText_LessThan10),
        1 => addr_of!(gBirchDexRatingText_LessThan20),
        2 => addr_of!(gBirchDexRatingText_LessThan30),
        3 => addr_of!(gBirchDexRatingText_LessThan40),
        4 => addr_of!(gBirchDexRatingText_LessThan50),
        5 => addr_of!(gBirchDexRatingText_LessThan60),
        6 => addr_of!(gBirchDexRatingText_LessThan70),
        7 => addr_of!(gBirchDexRatingText_LessThan80),
        8 => addr_of!(gBirchDexRatingText_LessThan90),
        9 => addr_of!(gBirchDexRatingText_LessThan100),
        10 => addr_of!(gBirchDexRatingText_LessThan110),
        11 => addr_of!(gBirchDexRatingText_LessThan120),
        12 => addr_of!(gBirchDexRatingText_LessThan130),
        13 => addr_of!(gBirchDexRatingText_LessThan140),
        14 => addr_of!(gBirchDexRatingText_LessThan150),
        15 => addr_of!(gBirchDexRatingText_LessThan160),
        16 => addr_of!(gBirchDexRatingText_LessThan170),
        17 => addr_of!(gBirchDexRatingText_LessThan180),
        18 => addr_of!(gBirchDexRatingText_LessThan190),
        _ => addr_of!(gBirchDexRatingText_LessThan200),
    }
}

unsafe fn mythical_is_caught(species: u16) -> bool {
    let dex_number = unsafe { SpeciesToNationalPokedexNum(species) };
    unsafe { GetSetPokedexFlag(dex_number, FLAG_GET_CAUGHT) != 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptGetPokedexInfo() -> u16 {
    if unsafe { addr_of!(gSpecialVar_0x8004).read() } == 0 {
        unsafe { addr_of_mut!(gSpecialVar_0x8005).write(GetHoennPokedexCount(FLAG_GET_SEEN)) };
        unsafe { addr_of_mut!(gSpecialVar_0x8006).write(GetHoennPokedexCount(FLAG_GET_CAUGHT)) };
    } else {
        unsafe { addr_of_mut!(gSpecialVar_0x8005).write(GetNationalPokedexCount(FLAG_GET_SEEN)) };
        unsafe { addr_of_mut!(gSpecialVar_0x8006).write(GetNationalPokedexCount(FLAG_GET_CAUGHT)) };
    }
    unsafe { IsNationalPokedexEnabled() as u16 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokedexRatingText(count: u16) -> *const u8 {
    match rating_kind(count) {
        RatingKind::Band(band) => unsafe { band_text(band) },
        RatingKind::NeedsMythicalCheck => {
            if unsafe { mythical_is_caught(SPECIES_JIRACHI) }
                || unsafe { mythical_is_caught(SPECIES_DEOXYS) }
            {
                addr_of!(gBirchDexRatingText_LessThan200)
            } else {
                addr_of!(gBirchDexRatingText_DexCompleted)
            }
        }
        RatingKind::NeedsBothMythicalsCheck => {
            if unsafe { mythical_is_caught(SPECIES_JIRACHI) }
                && unsafe { mythical_is_caught(SPECIES_DEOXYS) }
            {
                addr_of!(gBirchDexRatingText_LessThan200)
            } else {
                addr_of!(gBirchDexRatingText_DexCompleted)
            }
        }
        RatingKind::Complete => addr_of!(gBirchDexRatingText_DexCompleted),
        RatingKind::Fallback => addr_of!(gBirchDexRatingText_LessThan10),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowPokedexRatingMessage() {
    let count = unsafe { addr_of!(gSpecialVar_0x8004).read() };
    let text = unsafe { GetPokedexRatingText(count) };
    let _ = unsafe { ShowFieldMessage(text) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating_bands_and_completion_edges_match_the_original() {
        assert_eq!(rating_kind(0), RatingKind::Band(0));
        assert_eq!(rating_kind(9), RatingKind::Band(0));
        assert_eq!(rating_kind(10), RatingKind::Band(1));
        assert_eq!(rating_kind(199), RatingKind::Band(19));
        assert_eq!(rating_kind(200), RatingKind::NeedsMythicalCheck);
        assert_eq!(rating_kind(201), RatingKind::NeedsBothMythicalsCheck);
        assert_eq!(rating_kind(202), RatingKind::Complete);
        assert_eq!(rating_kind(203), RatingKind::Fallback);
    }
}
