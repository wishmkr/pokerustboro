//! Professor Birch's Pokédex rating, from the PC or in person (was
//! src/birch_pc.c).

use crate::event_data::is_national_pokedex_enabled;
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_0x8005, gSpecialVar_0x8006};
use crate::pokedex::{GetHoennPokedexCount, GetNationalPokedexCount, GetSetPokedexFlag};
use crate::pokemon::SpeciesToNationalPokedexNum;

const FLAG_GET_SEEN: u8 = 0;
const FLAG_GET_CAUGHT: u8 = 1;
const SPECIES_JIRACHI: u16 = 409;
const SPECIES_DEOXYS: u16 = 410;
const HOENN_DEX_COUNT: u16 = 202;
/// Jirachi and Deoxys are in the Hoenn dex but don't count for completing it.
const HOENN_DEX_COUNT_WITHOUT_MYTHICALS: u16 = HOENN_DEX_COUNT - 2;

/// `ShowFieldMessage` with this module's view of its types.
#[inline]
unsafe fn ShowFieldMessage(a0: *const u8) -> u8 {
    unsafe { crate::field_message_box::ShowFieldMessage(a0 as _) }
}

/// The rating for fewer than `10 * (i + 1)` Pokémon.
fn rating_texts() -> [*const u8; 20] {
    [
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan10.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan20.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan30.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan40.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan50.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan60.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan70.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan80.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan90.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan100.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan110.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan120.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan130.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan140.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan150.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan160.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan170.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan180.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan190.cast::<u8>()),
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan200.cast::<u8>()),
    ]
}

fn is_caught(species: u16) -> bool {
    // SAFETY: plain Pokédex queries.
    unsafe { GetSetPokedexFlag(SpeciesToNationalPokedexNum(species), FLAG_GET_CAUGHT) != 0 }
}

/// Puts the seen and caught counts in `VAR_0x8005`/`VAR_0x8006`: Hoenn's if
/// `VAR_0x8004` is 0, else the national ones. Returns whether the national
/// dex is on.
pub fn script_get_pokedex_info() -> bool {
    // SAFETY: the special vars are plain u16s only the script engine uses.
    unsafe {
        let (seen, caught) = if *(&raw const gSpecialVar_0x8004) == 0 {
            (
                GetHoennPokedexCount(FLAG_GET_SEEN),
                GetHoennPokedexCount(FLAG_GET_CAUGHT),
            )
        } else {
            (
                GetNationalPokedexCount(FLAG_GET_SEEN),
                GetNationalPokedexCount(FLAG_GET_CAUGHT),
            )
        };
        *(&raw mut gSpecialVar_0x8005) = seen;
        *(&raw mut gSpecialVar_0x8006) = caught;
    }
    is_national_pokedex_enabled()
}

/// Birch's comment on `count` Pokémon caught in the Hoenn dex.
pub fn pokedex_rating_text(count: u16) -> *const u8 {
    let jirachi_or_deoxys_counted = match count {
        // one of them caught: one short of the 200 that count
        HOENN_DEX_COUNT_WITHOUT_MYTHICALS => {
            is_caught(SPECIES_JIRACHI) || is_caught(SPECIES_DEOXYS)
        }
        // both caught: still one short
        201 => is_caught(SPECIES_JIRACHI) && is_caught(SPECIES_DEOXYS),
        HOENN_DEX_COUNT => false,
        200.. => return &raw const (*crate::asmdata::gBirchDexRatingText_LessThan10.cast::<u8>()),
        _ => {
            let texts = rating_texts();
            return texts
                .get(usize::from(count / 10))
                .copied()
                .unwrap_or(texts[0]);
        }
    };
    if jirachi_or_deoxys_counted {
        &raw const (*crate::asmdata::gBirchDexRatingText_LessThan200.cast::<u8>())
    } else {
        &raw const (*crate::asmdata::gBirchDexRatingText_DexCompleted.cast::<u8>())
    }
}

pub fn show_pokedex_rating_message() {
    // SAFETY: as in script_get_pokedex_info; ShowFieldMessage takes any text.
    unsafe { ShowFieldMessage(pokedex_rating_text(*(&raw const gSpecialVar_0x8004))) };
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub fn ScriptGetPokedexInfo() -> u16 {
    script_get_pokedex_info().into()
}

#[unsafe(no_mangle)]
pub fn GetPokedexRatingText(count: u16) -> *const u8 {
    pokedex_rating_text(count)
}

#[unsafe(no_mangle)]
pub fn ShowPokedexRatingMessage() {
    show_pokedex_rating_message();
}
