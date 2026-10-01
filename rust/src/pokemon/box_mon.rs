//! A Pokémon's stored data: encryption, substructs and the
//! `GetBoxMonData` / `SetBoxMonData` fields (was part of src/pokemon.c).
//!
//! A `BoxPokemon` keeps its personality, trainer id, names and a few flags in
//! the clear, and the rest in four 12-byte substructs:
//!
//! | substruct | holds |
//! |---|---|
//! | growth (0) | species, held item, experience, PP bonuses, friendship |
//! | attacks (1) | moves and their PP |
//! | condition (2) | EVs and contest stats |
//! | misc (3) | Pokérus, where it was met, ball, IVs, egg flag, ability, ribbons |
//!
//! The substructs are stored in one of 24 orders, chosen by `personality %
//! 24`, and XORed word by word with `personality ^ otId`. A checksum (the
//! sum of their decrypted halfwords) guards them: a Pokémon whose checksum
//! doesn't match becomes a Bad Egg the first time its data is read.

use crate::c::CArray;
use crate::consts::*;
use crate::types::{
    BoxPokemon, PokemonSubstruct0, PokemonSubstruct1, PokemonSubstruct2, PokemonSubstruct3,
};

/// For each `personality % 24`: the position of substructs 0, 1, 2 and 3.
const SUBSTRUCT_POSITIONS: [[u8; 4]; 24] = [
    [0, 1, 2, 3],
    [0, 1, 3, 2],
    [0, 2, 1, 3],
    [0, 3, 1, 2],
    [0, 2, 3, 1],
    [0, 3, 2, 1],
    [1, 0, 2, 3],
    [1, 0, 3, 2],
    [2, 0, 1, 3],
    [3, 0, 1, 2],
    [2, 0, 3, 1],
    [3, 0, 2, 1],
    [1, 2, 0, 3],
    [1, 3, 0, 2],
    [2, 1, 0, 3],
    [3, 1, 0, 2],
    [2, 3, 0, 1],
    [3, 2, 0, 1],
    [1, 2, 3, 0],
    [1, 3, 2, 0],
    [2, 1, 3, 0],
    [3, 1, 2, 0],
    [2, 3, 1, 0],
    [3, 2, 1, 0],
];

/// A Pokémon's four substructs, decrypted.
#[derive(Clone, Copy)]
pub struct Substructs {
    pub growth: PokemonSubstruct0,
    pub attacks: PokemonSubstruct1,
    pub condition: PokemonSubstruct2,
    pub misc: PokemonSubstruct3,
}

type Words = [u32; 12];

impl Substructs {
    /// Splits decrypted words (in stored order) into the substructs.
    fn from_words(words: &Words, positions: [u8; 4]) -> Self {
        let part = |kind: usize| -> [u32; 3] {
            let at = 3 * usize::from(positions[kind & 3] & 3);
            [words[at], words[at + 1], words[at + 2]]
        };
        // SAFETY: each substruct is 12 bytes of plain integers (size checked
        // in types/pokemon.rs), so any 12 bytes are a valid value.
        unsafe {
            Self {
                growth: core::mem::transmute::<[u32; 3], PokemonSubstruct0>(part(0)),
                attacks: core::mem::transmute::<[u32; 3], PokemonSubstruct1>(part(1)),
                condition: core::mem::transmute::<[u32; 3], PokemonSubstruct2>(part(2)),
                misc: core::mem::transmute::<[u32; 3], PokemonSubstruct3>(part(3)),
            }
        }
    }

    /// The substructs as words, in stored order.
    fn to_words(&self, positions: [u8; 4]) -> Words {
        // SAFETY: as in from_words, the other way round.
        let parts: [[u32; 3]; 4] = unsafe {
            [
                core::mem::transmute::<PokemonSubstruct0, [u32; 3]>(self.growth),
                core::mem::transmute::<PokemonSubstruct1, [u32; 3]>(self.attacks),
                core::mem::transmute::<PokemonSubstruct2, [u32; 3]>(self.condition),
                core::mem::transmute::<PokemonSubstruct3, [u32; 3]>(self.misc),
            ]
        };
        let mut words = [0; 12];
        for (kind, part) in parts.iter().enumerate() {
            let at = 3 * usize::from(positions[kind] & 3);
            words[at..at + 3].copy_from_slice(part);
        }
        words
    }
}

/// The checksum of decrypted substruct words: the sum of their halfwords.
fn checksum(words: &Words) -> u16 {
    words.iter().fold(0u16, |sum, w| {
        sum.wrapping_add(*w as u16).wrapping_add((*w >> 16) as u16)
    })
}

impl BoxPokemon {
    fn positions(&self) -> [u8; 4] {
        SUBSTRUCT_POSITIONS[(self.personality % 24) as usize]
    }

    fn key(&self) -> u32 {
        self.personality ^ self.otId
    }

    fn stored_words(&self) -> Words {
        // SAFETY: the secure data is plain integers under any view.
        unsafe { self.secure.raw.0 }
    }

    fn store_words(&mut self, words: Words) {
        self.secure.raw = CArray(words);
    }

    /// The decrypted substructs, and whether the checksum matched. If it
    /// didn't, the Pokémon has just become a Bad Egg (as in C, this is
    /// written back).
    pub fn read_substructs(&mut self) -> (Substructs, bool) {
        let key = self.key();
        let positions = self.positions();
        let mut words = self.stored_words().map(|w| w ^ key);
        let ok = checksum(&words) == self.checksum;
        let mut substructs = Substructs::from_words(&words, positions);
        if !ok {
            self.set_isBadEgg(1);
            self.set_isEgg(1);
            substructs.misc.set_isEgg(1);
            words = substructs.to_words(positions);
            self.store_words(words.map(|w| w ^ key));
        }
        (substructs, ok)
    }

    /// Stores new substructs: a fresh checksum, then encrypted.
    pub fn write_substructs(&mut self, substructs: &Substructs) {
        let words = substructs.to_words(self.positions());
        self.checksum = checksum(&words);
        let key = self.key();
        self.store_words(words.map(|w| w ^ key));
    }
}

impl BoxPokemon {
    /// The species (`SPECIES_EGG` for a Bad Egg), as `MON_DATA_SPECIES`.
    pub fn species(&mut self) -> u16 {
        let (s, _) = self.read_substructs();
        if self.isBadEgg() != 0 {
            SPECIES_EGG
        } else {
            s.growth.species
        }
    }

    /// The species, or `SPECIES_EGG` for an egg or a Bad Egg
    /// (`MON_DATA_SPECIES_OR_EGG`).
    pub fn species_or_egg(&mut self) -> u16 {
        let (s, _) = self.read_substructs();
        let species = s.growth.species;
        if species != 0 && (s.misc.isEgg() != 0 || self.isBadEgg() != 0) {
            SPECIES_EGG
        } else {
            species
        }
    }

    /// Whether it is an egg (`MON_DATA_IS_EGG`).
    pub fn is_egg(&mut self) -> bool {
        self.read_substructs().0.misc.isEgg() != 0
    }

    /// The name to show for it (`MON_DATA_NICKNAME`: "EGG" for an egg,
    /// marked up for a Japanese one), written to `out` with an EOS.
    pub fn nickname(&mut self, out: &mut [u8; POKEMON_NAME_LENGTH + 1]) {
        // SAFETY: `out` has room for the longest name (a Japanese one:
        // 2 + 5 + 2 characters and EOS).
        unsafe { get_box_mon_data(self, MON_DATA_NICKNAME, out.as_mut_ptr()) };
    }
}

// ----------------------------------------------------------- the C fields

pub const POKEMON_NAME_LENGTH: usize = 10;
const PLAYER_NAME_LENGTH: usize = 7;
const EOS: u8 = 0xff;
const EXT_CTRL_CODE_BEGIN: u8 = 0xfc;
const EXT_CTRL_CODE_JPN: u8 = 0x15;
const EXT_CTRL_CODE_ENG: u8 = 0x16;
const LANGUAGE_JAPANESE: u8 = 1;
const MOVES_COUNT: u16 = 355;
const SPECIES_EGG: u16 = 412;
const MAX_IV_MASK: u32 = 31;

/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *const u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}
/// `StringLength` with this module's view of its types.
#[inline]
unsafe fn StringLength(a0: *const u8) -> u16 {
    unsafe { crate::string_util::StringLength(a0 as _) }
}

/// Copies up to `max` characters of `name` to `data` at `at`, stopping at
/// EOS if `stop_at_eos`; returns how many it copied.
///
/// # Safety
/// `name` must be readable up to its EOS or `max`, and `data` must have
/// room (or be null: the GBA ignores writes to address 0, which is where C
/// writes for a call without a buffer).
unsafe fn write_name(
    data: *mut u8,
    at: usize,
    name: *const u8,
    max: usize,
    stop_at_eos: bool,
) -> usize {
    let mut n = 0;
    while n < max {
        let c = unsafe { *name.add(n) };
        if stop_at_eos && c == EOS {
            break;
        }
        if !data.is_null() {
            unsafe { *data.add(at + n) = c };
        }
        n += 1;
    }
    n
}

/// C's `GetBoxMonData`: field `field` of `mon`. Name fields are copied to
/// `data`, `MON_DATA_KNOWN_MOVES` reads a move list from it.
///
/// # Safety
/// `data` must be what the field expects (see above), or null for the
/// others.
pub unsafe fn get_box_mon_data(mon: &mut BoxPokemon, field: i32, data: *mut u8) -> u32 {
    let put = |i: usize, c: u8| {
        if !data.is_null() {
            unsafe { *data.add(i) = c };
        }
    };
    if field <= MON_DATA_ENCRYPT_SEPARATOR {
        return match field {
            MON_DATA_PERSONALITY => mon.personality,
            MON_DATA_OT_ID => mon.otId,
            MON_DATA_NICKNAME => {
                if mon.isBadEgg() != 0 {
                    // SAFETY: the text ends in EOS within 10 characters.
                    let n = unsafe {
                        write_name(
                            data,
                            0,
                            (*(&raw const crate::data::battle_message::gText_BadEgg)
                                .cast::<CArray<u8, 0>>())
                            .as_ptr(),
                            POKEMON_NAME_LENGTH,
                            true,
                        )
                    };
                    put(n, EOS);
                    n as u32
                } else if mon.isEgg() != 0 {
                    // SAFETY: data has room for a name (the caller's promise).
                    unsafe {
                        StringCopy(
                            data,
                            &raw const (*(&raw const crate::data::strings::gText_EggNickname)
                                .cast::<u8>()),
                        );
                        u32::from(StringLength(data))
                    }
                } else if mon.language == LANGUAGE_JAPANESE {
                    put(0, EXT_CTRL_CODE_BEGIN);
                    put(1, EXT_CTRL_CODE_JPN);
                    let n = unsafe { write_name(data, 2, mon.nickname.as_ptr(), 5, true) };
                    put(2 + n, EXT_CTRL_CODE_BEGIN);
                    put(3 + n, EXT_CTRL_CODE_ENG);
                    put(4 + n, EOS);
                    (4 + n) as u32
                } else {
                    let n = unsafe {
                        write_name(data, 0, mon.nickname.as_ptr(), POKEMON_NAME_LENGTH, false)
                    };
                    put(n, EOS);
                    n as u32
                }
            }
            MON_DATA_LANGUAGE => mon.language.into(),
            MON_DATA_SANITY_IS_BAD_EGG => mon.isBadEgg().into(),
            MON_DATA_SANITY_HAS_SPECIES => mon.hasSpecies().into(),
            MON_DATA_SANITY_IS_EGG => mon.isEgg().into(),
            MON_DATA_OT_NAME => {
                let n =
                    unsafe { write_name(data, 0, mon.otName.as_ptr(), PLAYER_NAME_LENGTH, false) };
                put(n, EOS);
                n as u32
            }
            MON_DATA_MARKINGS => mon.markings.into(),
            MON_DATA_CHECKSUM => mon.checksum.into(),
            MON_DATA_ENCRYPT_SEPARATOR => mon.unknown.into(),
            _ => 0,
        };
    }

    let (s, _) = mon.read_substructs();
    let (growth, attacks, condition, misc) = (&s.growth, &s.attacks, &s.condition, &s.misc);
    let has_species = growth.species != 0;
    let bad_egg = mon.isBadEgg() != 0;
    match field {
        MON_DATA_SPECIES => (if bad_egg { SPECIES_EGG } else { growth.species }).into(),
        MON_DATA_HELD_ITEM => growth.heldItem.into(),
        MON_DATA_EXP => growth.experience,
        MON_DATA_PP_BONUSES => growth.ppBonuses.into(),
        MON_DATA_FRIENDSHIP => growth.friendship.into(),
        MON_DATA_MOVE1..=MON_DATA_MOVE4 => attacks.moves[field - MON_DATA_MOVE1].into(),
        MON_DATA_PP1..=MON_DATA_PP4 => attacks.pp[field - MON_DATA_PP1].into(),
        MON_DATA_HP_EV => condition.hpEV.into(),
        MON_DATA_ATK_EV => condition.attackEV.into(),
        MON_DATA_DEF_EV => condition.defenseEV.into(),
        MON_DATA_SPEED_EV => condition.speedEV.into(),
        MON_DATA_SPATK_EV => condition.spAttackEV.into(),
        MON_DATA_SPDEF_EV => condition.spDefenseEV.into(),
        MON_DATA_COOL => condition.cool.into(),
        MON_DATA_BEAUTY => condition.beauty.into(),
        MON_DATA_CUTE => condition.cute.into(),
        MON_DATA_SMART => condition.smart.into(),
        MON_DATA_TOUGH => condition.tough.into(),
        MON_DATA_SHEEN => condition.sheen.into(),
        MON_DATA_POKERUS => misc.pokerus.into(),
        MON_DATA_MET_LOCATION => misc.metLocation.into(),
        MON_DATA_MET_LEVEL => misc.metLevel().into(),
        MON_DATA_MET_GAME => misc.metGame().into(),
        MON_DATA_POKEBALL => misc.pokeball().into(),
        MON_DATA_OT_GENDER => misc.otGender().into(),
        MON_DATA_HP_IV => misc.hpIV(),
        MON_DATA_ATK_IV => misc.attackIV(),
        MON_DATA_DEF_IV => misc.defenseIV(),
        MON_DATA_SPEED_IV => misc.speedIV(),
        MON_DATA_SPATK_IV => misc.spAttackIV(),
        MON_DATA_SPDEF_IV => misc.spDefenseIV(),
        MON_DATA_IS_EGG => misc.isEgg(),
        MON_DATA_ABILITY_NUM => misc.abilityNum(),
        MON_DATA_COOL_RIBBON => misc.coolRibbon(),
        MON_DATA_BEAUTY_RIBBON => misc.beautyRibbon(),
        MON_DATA_CUTE_RIBBON => misc.cuteRibbon(),
        MON_DATA_SMART_RIBBON => misc.smartRibbon(),
        MON_DATA_TOUGH_RIBBON => misc.toughRibbon(),
        MON_DATA_CHAMPION_RIBBON => misc.championRibbon(),
        MON_DATA_WINNING_RIBBON => misc.winningRibbon(),
        MON_DATA_VICTORY_RIBBON => misc.victoryRibbon(),
        MON_DATA_ARTIST_RIBBON => misc.artistRibbon(),
        MON_DATA_EFFORT_RIBBON => misc.effortRibbon(),
        MON_DATA_MARINE_RIBBON => misc.marineRibbon(),
        MON_DATA_LAND_RIBBON => misc.landRibbon(),
        MON_DATA_SKY_RIBBON => misc.skyRibbon(),
        MON_DATA_COUNTRY_RIBBON => misc.countryRibbon(),
        MON_DATA_NATIONAL_RIBBON => misc.nationalRibbon(),
        MON_DATA_EARTH_RIBBON => misc.earthRibbon(),
        MON_DATA_WORLD_RIBBON => misc.worldRibbon(),
        MON_DATA_UNUSED_RIBBONS => misc.unusedRibbons(),
        MON_DATA_MODERN_FATEFUL_ENCOUNTER => misc.modernFatefulEncounter(),
        MON_DATA_SPECIES_OR_EGG => {
            if has_species && (misc.isEgg() != 0 || bad_egg) {
                SPECIES_EGG.into()
            } else {
                growth.species.into()
            }
        }
        MON_DATA_IVS => {
            misc.hpIV()
                | misc.attackIV() << 5
                | misc.defenseIV() << 10
                | misc.speedIV() << 15
                | misc.spAttackIV() << 20
                | misc.spDefenseIV() << 25
        }
        MON_DATA_KNOWN_MOVES if has_species && misc.isEgg() == 0 => {
            // `data` is a list of moves ending in MOVES_COUNT: a bit for each
            // one the Pokémon knows
            let moves = data.cast::<u16>();
            let mut known = 0;
            let mut i = 0;
            loop {
                // SAFETY: the caller passes a MOVES_COUNT-terminated list.
                let m = unsafe { *moves.add(i) };
                if m == MOVES_COUNT {
                    break known;
                }
                if attacks.moves.0.contains(&m) {
                    known |= unsafe {
                        (&*(&raw const crate::util::gBitTable).cast::<CArray<u32, 32>>())[i]
                    };
                }
                i += 1;
            }
        }
        MON_DATA_RIBBON_COUNT if has_species && misc.isEgg() == 0 => {
            misc.coolRibbon()
                + misc.beautyRibbon()
                + misc.cuteRibbon()
                + misc.smartRibbon()
                + misc.toughRibbon()
                + misc.championRibbon()
                + misc.winningRibbon()
                + misc.victoryRibbon()
                + misc.artistRibbon()
                + misc.effortRibbon()
                + misc.marineRibbon()
                + misc.landRibbon()
                + misc.skyRibbon()
                + misc.countryRibbon()
                + misc.nationalRibbon()
                + misc.earthRibbon()
                + misc.worldRibbon()
        }
        MON_DATA_RIBBONS if has_species && misc.isEgg() == 0 => {
            misc.championRibbon()
                | misc.coolRibbon() << 1
                | misc.beautyRibbon() << 4
                | misc.cuteRibbon() << 7
                | misc.smartRibbon() << 10
                | misc.toughRibbon() << 13
                | misc.winningRibbon() << 16
                | misc.victoryRibbon() << 17
                | misc.artistRibbon() << 18
                | misc.effortRibbon() << 19
                | misc.marineRibbon() << 20
                | misc.landRibbon() << 21
                | misc.skyRibbon() << 22
                | misc.countryRibbon() << 23
                | misc.nationalRibbon() << 24
                | misc.earthRibbon() << 25
                | misc.worldRibbon() << 26
        }
        _ => 0,
    }
}

/// C's `SetBoxMonData`: sets field `field` of `mon` from the bytes at `data`
/// (little-endian for 16- and 32-bit fields, as C reads them).
///
/// # Safety
/// `data` must hold as many bytes as the field needs.
pub unsafe fn set_box_mon_data(mon: &mut BoxPokemon, field: i32, data: *const u8) {
    // SAFETY: the caller's promise.
    let byte = |i: usize| unsafe { *data.add(i) };
    let u16_ = || u16::from(byte(0)) | u16::from(byte(1)) << 8;
    let u32_ = || {
        u32::from(byte(0))
            | u32::from(byte(1)) << 8
            | u32::from(byte(2)) << 16
            | u32::from(byte(3)) << 24
    };

    if field <= MON_DATA_ENCRYPT_SEPARATOR {
        match field {
            MON_DATA_PERSONALITY => mon.personality = u32_(),
            MON_DATA_OT_ID => mon.otId = u32_(),
            MON_DATA_NICKNAME => {
                for (i, c) in mon.nickname.0.iter_mut().enumerate() {
                    *c = byte(i);
                }
            }
            MON_DATA_LANGUAGE => mon.language = byte(0),
            MON_DATA_SANITY_IS_BAD_EGG => mon.set_isBadEgg(byte(0)),
            MON_DATA_SANITY_HAS_SPECIES => mon.set_hasSpecies(byte(0)),
            MON_DATA_SANITY_IS_EGG => mon.set_isEgg(byte(0)),
            MON_DATA_OT_NAME => {
                for (i, c) in mon.otName.0.iter_mut().enumerate() {
                    *c = byte(i);
                }
            }
            MON_DATA_MARKINGS => mon.markings = byte(0),
            MON_DATA_CHECKSUM => mon.checksum = u16_(),
            MON_DATA_ENCRYPT_SEPARATOR => mon.unknown = u16_(),
            _ => {}
        }
        return;
    }

    let (mut s, ok) = mon.read_substructs();
    if !ok {
        return; // it has just become a Bad Egg; C stops there
    }
    let b = || u32::from(byte(0));
    let misc = &mut s.misc;
    match field {
        MON_DATA_SPECIES => {
            s.growth.species = u16_();
            mon.set_hasSpecies((s.growth.species != 0).into());
        }
        MON_DATA_HELD_ITEM => s.growth.heldItem = u16_(),
        MON_DATA_EXP => s.growth.experience = u32_(),
        MON_DATA_PP_BONUSES => s.growth.ppBonuses = byte(0),
        MON_DATA_FRIENDSHIP => s.growth.friendship = byte(0),
        MON_DATA_MOVE1..=MON_DATA_MOVE4 => s.attacks.moves[field - MON_DATA_MOVE1] = u16_(),
        MON_DATA_PP1..=MON_DATA_PP4 => s.attacks.pp[field - MON_DATA_PP1] = byte(0),
        MON_DATA_HP_EV => s.condition.hpEV = byte(0),
        MON_DATA_ATK_EV => s.condition.attackEV = byte(0),
        MON_DATA_DEF_EV => s.condition.defenseEV = byte(0),
        MON_DATA_SPEED_EV => s.condition.speedEV = byte(0),
        MON_DATA_SPATK_EV => s.condition.spAttackEV = byte(0),
        MON_DATA_SPDEF_EV => s.condition.spDefenseEV = byte(0),
        MON_DATA_COOL => s.condition.cool = byte(0),
        MON_DATA_BEAUTY => s.condition.beauty = byte(0),
        MON_DATA_CUTE => s.condition.cute = byte(0),
        MON_DATA_SMART => s.condition.smart = byte(0),
        MON_DATA_TOUGH => s.condition.tough = byte(0),
        MON_DATA_SHEEN => s.condition.sheen = byte(0),
        MON_DATA_POKERUS => misc.pokerus = byte(0),
        MON_DATA_MET_LOCATION => misc.metLocation = byte(0),
        MON_DATA_MET_LEVEL => misc.set_metLevel(byte(0).into()),
        MON_DATA_MET_GAME => misc.set_metGame(byte(0).into()),
        MON_DATA_POKEBALL => misc.set_pokeball(byte(0).into()),
        MON_DATA_OT_GENDER => misc.set_otGender(byte(0).into()),
        MON_DATA_HP_IV => misc.set_hpIV(b()),
        MON_DATA_ATK_IV => misc.set_attackIV(b()),
        MON_DATA_DEF_IV => misc.set_defenseIV(b()),
        MON_DATA_SPEED_IV => misc.set_speedIV(b()),
        MON_DATA_SPATK_IV => misc.set_spAttackIV(b()),
        MON_DATA_SPDEF_IV => misc.set_spDefenseIV(b()),
        MON_DATA_IS_EGG => {
            misc.set_isEgg(b());
            mon.set_isEgg((misc.isEgg() != 0).into());
        }
        MON_DATA_ABILITY_NUM => misc.set_abilityNum(b()),
        MON_DATA_COOL_RIBBON => misc.set_coolRibbon(b()),
        MON_DATA_BEAUTY_RIBBON => misc.set_beautyRibbon(b()),
        MON_DATA_CUTE_RIBBON => misc.set_cuteRibbon(b()),
        MON_DATA_SMART_RIBBON => misc.set_smartRibbon(b()),
        MON_DATA_TOUGH_RIBBON => misc.set_toughRibbon(b()),
        MON_DATA_CHAMPION_RIBBON => misc.set_championRibbon(b()),
        MON_DATA_WINNING_RIBBON => misc.set_winningRibbon(b()),
        MON_DATA_VICTORY_RIBBON => misc.set_victoryRibbon(b()),
        MON_DATA_ARTIST_RIBBON => misc.set_artistRibbon(b()),
        MON_DATA_EFFORT_RIBBON => misc.set_effortRibbon(b()),
        MON_DATA_MARINE_RIBBON => misc.set_marineRibbon(b()),
        MON_DATA_LAND_RIBBON => misc.set_landRibbon(b()),
        MON_DATA_SKY_RIBBON => misc.set_skyRibbon(b()),
        MON_DATA_COUNTRY_RIBBON => misc.set_countryRibbon(b()),
        MON_DATA_NATIONAL_RIBBON => misc.set_nationalRibbon(b()),
        MON_DATA_EARTH_RIBBON => misc.set_earthRibbon(b()),
        MON_DATA_WORLD_RIBBON => misc.set_worldRibbon(b()),
        MON_DATA_UNUSED_RIBBONS => misc.set_unusedRibbons(b()),
        MON_DATA_MODERN_FATEFUL_ENCOUNTER => misc.set_modernFatefulEncounter(b()),
        MON_DATA_IVS => {
            let ivs = u32_();
            misc.set_hpIV(ivs & MAX_IV_MASK);
            misc.set_attackIV(ivs >> 5 & MAX_IV_MASK);
            misc.set_defenseIV(ivs >> 10 & MAX_IV_MASK);
            misc.set_speedIV(ivs >> 15 & MAX_IV_MASK);
            misc.set_spAttackIV(ivs >> 20 & MAX_IV_MASK);
            misc.set_spDefenseIV(ivs >> 25 & MAX_IV_MASK);
        }
        _ => {}
    }
    mon.write_substructs(&s);
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub unsafe fn GetBoxMonData3(mon: *mut BoxPokemon, field: i32, data: *mut u8) -> u32 {
    unsafe { get_box_mon_data(&mut *mon, field, data) }
}

#[unsafe(no_mangle)]
pub unsafe fn GetBoxMonData2(mon: *mut BoxPokemon, field: i32) -> u32 {
    unsafe { get_box_mon_data(&mut *mon, field, core::ptr::null_mut()) }
}

#[unsafe(no_mangle)]
pub unsafe fn SetBoxMonData(mon: *mut BoxPokemon, field: i32, data: *const u8) {
    unsafe { set_box_mon_data(&mut *mon, field, data) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blank_mon(personality: u32, ot_id: u32) -> BoxPokemon {
        // SAFETY: plain data; all zeroes is valid.
        let mut mon: BoxPokemon = unsafe { core::mem::zeroed() };
        mon.personality = personality;
        mon.otId = ot_id;
        // an all-zero Pokémon: encrypt zeroes with a matching checksum
        let s = Substructs::from_words(&[0; 12], mon.positions());
        mon.write_substructs(&s);
        mon
    }

    #[test]
    fn fields_round_trip_through_the_encryption() {
        for personality in [0, 1, 7, 23, 24, 0xdead_beef] {
            let mut mon = blank_mon(personality, 0x1234_5678);
            unsafe {
                set_box_mon_data(&mut mon, MON_DATA_SPECIES, 25u16.to_le_bytes().as_ptr());
                set_box_mon_data(&mut mon, MON_DATA_EXP, 1_000_000u32.to_le_bytes().as_ptr());
                set_box_mon_data(&mut mon, MON_DATA_MOVE3, 85u16.to_le_bytes().as_ptr());
                set_box_mon_data(&mut mon, MON_DATA_ATK_IV, [31u8].as_ptr());
                assert_eq!(
                    get_box_mon_data(&mut mon, MON_DATA_SPECIES, core::ptr::null_mut()),
                    25
                );
                assert_eq!(
                    get_box_mon_data(&mut mon, MON_DATA_EXP, core::ptr::null_mut()),
                    1_000_000
                );
                assert_eq!(
                    get_box_mon_data(&mut mon, MON_DATA_MOVE3, core::ptr::null_mut()),
                    85
                );
                assert_eq!(
                    get_box_mon_data(&mut mon, MON_DATA_ATK_IV, core::ptr::null_mut()),
                    31
                );
            }
            assert_eq!(mon.hasSpecies(), 1);
            assert_eq!(mon.isBadEgg(), 0);
        }
    }

    #[test]
    fn a_wrong_checksum_makes_a_bad_egg() {
        let mut mon = blank_mon(5, 6);
        mon.checksum ^= 1;
        let species =
            unsafe { get_box_mon_data(&mut mon, MON_DATA_SPECIES, core::ptr::null_mut()) };
        assert_eq!(species, u32::from(SPECIES_EGG));
        assert_eq!(mon.isBadEgg(), 1);
        assert_eq!(mon.isEgg(), 1);
        assert_eq!(
            unsafe { get_box_mon_data(&mut mon, MON_DATA_IS_EGG, core::ptr::null_mut()) },
            1
        );
    }

    #[test]
    fn each_order_places_every_substruct_once() {
        for positions in SUBSTRUCT_POSITIONS {
            let mut seen = [false; 4];
            for p in positions {
                seen[usize::from(p)] = true;
            }
            assert_eq!(seen, [true; 4]);
        }
    }
}
