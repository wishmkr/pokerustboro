use crate::ffi::{
    GetVarPointer, StringCopy, VarSet, gSpecialVar_Result, gStringVar1, gStringVar2, gStringVar3,
};
use core::ffi::c_int;
use core::ptr::{addr_of, addr_of_mut};

const DEFAULT_MAX_SIZE: u16 = 0x8000;

const PARTY_NOTHING_CHOSEN: u16 = 0xff;

const COMPARE_SIZE_NONE: u8 = 0;
const COMPARE_SIZE_INCORRECT_SPECIES: u8 = 1;
const COMPARE_SIZE_SMALLER: u8 = 2;
const COMPARE_SIZE_LARGER: u8 = 3;

const MON_DATA_PERSONALITY: c_int = 0;
const MON_DATA_SPECIES: c_int = 11;
const MON_DATA_HP_IV: c_int = 39;
const MON_DATA_ATK_IV: c_int = 40;
const MON_DATA_DEF_IV: c_int = 41;
const MON_DATA_SPEED_IV: c_int = 42;
const MON_DATA_SPATK_IV: c_int = 43;
const MON_DATA_SPDEF_IV: c_int = 44;
const MON_DATA_IS_EGG: c_int = 45;

const STR_CONV_MODE_LEFT_ALIGN: c_int = 0;

const SPECIES_LOTAD: u16 = 295;
const SPECIES_SEEDOT: u16 = 298;

const VAR_SEEDOT_SIZE_RECORD: u16 = 0x4047;
const VAR_LOTAD_SIZE_RECORD: u16 = 0x404f;

const POKEMON_SIZE: usize = 100;
const POKEMON_NAME_BUFFER: usize = 11;

/// `CM_PER_INCH * 10` exactly as the C compiler folds it.
const CM_PER_INCH_X10: f64 = 2.54_f64 * 10.0;

/// `struct UnknownStruct { u16 unk0; u8 unk2; u16 unk4; }`
///
/// Fields sit at 0, 2 and 4. The stride is eight rather than six because
/// `-mabi=apcs-gnu` rounds every structure up to a multiple of four bytes.
#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct SizeEntry {
    base: u16,
    step: u8,
    threshold: u16,
}

const fn entry(base: u16, step: u8, threshold: i32) -> SizeEntry {
    SizeEntry {
        base,
        step,
        // The original table writes the last thresholds as negative constants,
        // which the C compiler wraps into u16. The wrapped sequence is
        // monotonically increasing, which is what the lookup relies on.
        threshold: threshold as u16,
    }
}

const BIG_MON_SIZE_TABLE: [SizeEntry; 16] = [
    entry(290, 1, 0),
    entry(300, 1, 10),
    entry(400, 2, 110),
    entry(500, 4, 310),
    entry(600, 20, 710),
    entry(700, 50, 2710),
    entry(800, 100, 7710),
    entry(900, 150, 17710),
    entry(1000, 150, 32710),
    entry(1100, 100, -17826),
    entry(1200, 50, -7826),
    entry(1300, 20, -2826),
    entry(1400, 5, -826),
    entry(1500, 2, -326),
    entry(1600, 1, -126),
    entry(1700, 1, -26),
];

unsafe extern "C" {
    static mut gPlayerParty: u8;
    static mut gSaveBlock2Ptr: *mut u8;
    static gSpeciesNames: u8;
    static gText_DecimalPoint: u8;
    static gText_Marco: u8;

    fn GetMonData2(mon: *mut u8, field: c_int) -> u32;
    fn GetPokedexHeightWeight(dex_num: u16, data: u8) -> u16;
    fn SpeciesToNationalPokedexNum(species: u16) -> u16;
    fn ConvertIntToDecimalStringN(dest: *mut u8, value: i32, mode: c_int, n: u8) -> *mut u8;
    fn StringAppend(dest: *mut u8, src: *const u8) -> *mut u8;
}

unsafe fn party_mon(index: usize) -> *mut u8 {
    unsafe { (&raw mut gPlayerParty).add(index * POKEMON_SIZE) }
}

unsafe fn species_name(species: u16) -> *const u8 {
    unsafe { (&raw const gSpeciesNames).add(species as usize * POKEMON_NAME_BUFFER) }
}

unsafe fn mon_size_hash(mon: *mut u8) -> u32 {
    let personality = unsafe { GetMonData2(mon, MON_DATA_PERSONALITY) } as u16;
    let hp_iv = unsafe { GetMonData2(mon, MON_DATA_HP_IV) } as u16 & 0xf;
    let attack_iv = unsafe { GetMonData2(mon, MON_DATA_ATK_IV) } as u16 & 0xf;
    let defense_iv = unsafe { GetMonData2(mon, MON_DATA_DEF_IV) } as u16 & 0xf;
    let speed_iv = unsafe { GetMonData2(mon, MON_DATA_SPEED_IV) } as u16 & 0xf;
    let sp_atk_iv = unsafe { GetMonData2(mon, MON_DATA_SPATK_IV) } as u16 & 0xf;
    let sp_def_iv = unsafe { GetMonData2(mon, MON_DATA_SPDEF_IV) } as u16 & 0xf;

    let hibyte = u32::from((attack_iv ^ defense_iv) * hp_iv) ^ u32::from(personality & 0xff);
    let lobyte = u32::from((sp_atk_iv ^ sp_def_iv) * speed_iv) ^ u32::from(personality >> 8);

    (hibyte << 8).wrapping_add(lobyte)
}

fn translate_table_index(value: u16) -> usize {
    let mut index = 1usize;
    while index < 15 {
        if value < BIG_MON_SIZE_TABLE[index & 15].threshold {
            return index - 1;
        }
        index += 1;
    }
    index
}

unsafe fn mon_size(species: u16, params: u16) -> u32 {
    let height =
        u32::from(unsafe { GetPokedexHeightWeight(SpeciesToNationalPokedexNum(species), 0) });
    let selected = BIG_MON_SIZE_TABLE[translate_table_index(params) & 15];

    let base = u64::from(selected.base);
    let step = u64::from(selected.step);
    let threshold = u64::from(selected.threshold);

    // The C table never stores a zero step; the guard only keeps a division
    // trap out of the ROM.
    let offset = if step == 0 {
        0
    } else {
        u64::from(params).wrapping_sub(threshold) / step
    };

    (u64::from(height).wrapping_mul(base.wrapping_add(offset)) / 10) as u32
}

unsafe fn format_mon_size_record(string: *mut u8, size: u32) {
    // UNITS_IMPERIAL: centimeters are converted to inches in double precision.
    let size = (f64::from(size.wrapping_mul(10)) / CM_PER_INCH_X10) as u32;

    let string = unsafe {
        ConvertIntToDecimalStringN(string, (size / 10) as i32, STR_CONV_MODE_LEFT_ALIGN, 8)
    };
    let string = unsafe { StringAppend(string, &raw const gText_DecimalPoint) };
    let _ = unsafe {
        ConvertIntToDecimalStringN(string, (size % 10) as i32, STR_CONV_MODE_LEFT_ALIGN, 1)
    };
}

unsafe fn compare_mon_size(species: u16, size_record: *mut u16) -> u8 {
    let chosen = unsafe { addr_of!(gSpecialVar_Result).read() };
    if chosen == PARTY_NOTHING_CHOSEN {
        return COMPARE_SIZE_NONE;
    }

    let mon = unsafe { party_mon(chosen as usize) };
    if unsafe { GetMonData2(mon, MON_DATA_IS_EGG) } == 1
        || unsafe { GetMonData2(mon, MON_DATA_SPECIES) } as u16 != species
    {
        return COMPARE_SIZE_INCORRECT_SPECIES;
    }

    let size_params = unsafe { mon_size_hash(mon) } as u16;
    let new_size = unsafe { mon_size(species, size_params) };
    let old_size = unsafe { mon_size(species, size_record.read()) };

    unsafe { format_mon_size_record((&raw mut gStringVar2).cast::<u8>(), new_size) };

    if new_size <= old_size {
        COMPARE_SIZE_SMALLER
    } else {
        unsafe { size_record.write(size_params) };
        COMPARE_SIZE_LARGER
    }
}

/// Stores the species name in `gStringVar1`, the trainer name in `gStringVar2`
/// and the formatted size in `gStringVar3`.
unsafe fn mon_size_record_info(species: u16, size_record: *mut u16) {
    let record = unsafe { size_record.read() };
    let size = unsafe { mon_size(species, record) };

    unsafe { format_mon_size_record((&raw mut gStringVar3).cast::<u8>(), size) };
    let _ = unsafe { StringCopy((&raw mut gStringVar1).cast::<u8>(), species_name(species)) };

    let name: *const u8 = if record == DEFAULT_MAX_SIZE {
        &raw const gText_Marco
    } else {
        unsafe { gSaveBlock2Ptr }
    };
    let _ = unsafe { StringCopy((&raw mut gStringVar2).cast::<u8>(), name) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitSeedotSizeRecord() {
    let _ = unsafe { VarSet(VAR_SEEDOT_SIZE_RECORD, DEFAULT_MAX_SIZE) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSeedotSizeRecordInfo() {
    let size_record = unsafe { GetVarPointer(VAR_SEEDOT_SIZE_RECORD) };
    unsafe { mon_size_record_info(SPECIES_SEEDOT, size_record) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CompareSeedotSize() {
    let size_record = unsafe { GetVarPointer(VAR_SEEDOT_SIZE_RECORD) };
    let result = unsafe { compare_mon_size(SPECIES_SEEDOT, size_record) };
    unsafe { addr_of_mut!(gSpecialVar_Result).write(u16::from(result)) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitLotadSizeRecord() {
    let _ = unsafe { VarSet(VAR_LOTAD_SIZE_RECORD, DEFAULT_MAX_SIZE) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLotadSizeRecordInfo() {
    let size_record = unsafe { GetVarPointer(VAR_LOTAD_SIZE_RECORD) };
    unsafe { mon_size_record_info(SPECIES_LOTAD, size_record) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CompareLotadSize() {
    let size_record = unsafe { GetVarPointer(VAR_LOTAD_SIZE_RECORD) };
    let result = unsafe { compare_mon_size(SPECIES_LOTAD, size_record) };
    unsafe { addr_of_mut!(gSpecialVar_Result).write(u16::from(result)) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_layout_matches_the_arm_structure() {
        // Verified with tools/rustport/probe.sh: APCS pads the six used
        // bytes out to an eight-byte stride.
        assert_eq!(core::mem::size_of::<SizeEntry>(), 8);
        assert_eq!(core::mem::align_of::<SizeEntry>(), 4);
        assert_eq!(core::mem::offset_of!(SizeEntry, base), 0);
        assert_eq!(core::mem::offset_of!(SizeEntry, step), 2);
        assert_eq!(core::mem::offset_of!(SizeEntry, threshold), 4);
    }

    #[test]
    fn negative_initializers_wrap_into_an_increasing_threshold_ladder() {
        assert_eq!(BIG_MON_SIZE_TABLE[9].threshold, 47_710);
        assert_eq!(BIG_MON_SIZE_TABLE[15].threshold, 65_510);

        let mut index = 1;
        while index < BIG_MON_SIZE_TABLE.len() {
            assert!(BIG_MON_SIZE_TABLE[index].threshold > BIG_MON_SIZE_TABLE[index - 1].threshold);
            index += 1;
        }
    }

    #[test]
    fn table_lookup_matches_the_original_scan() {
        assert_eq!(translate_table_index(0), 0);
        assert_eq!(translate_table_index(9), 0);
        assert_eq!(translate_table_index(10), 1);
        assert_eq!(translate_table_index(109), 1);
        assert_eq!(translate_table_index(110), 2);
        assert_eq!(translate_table_index(32_710), 8);
        assert_eq!(translate_table_index(47_709), 8);
        assert_eq!(translate_table_index(47_710), 9);
        // The scan stops at index 15, so anything past the last checked
        // threshold selects the final entry.
        assert_eq!(translate_table_index(65_535), 15);
    }

    #[test]
    fn the_lookup_never_underflows_the_threshold_subtraction() {
        for params in [0u16, 1, 10, 700, 32_710, 47_710, 65_535] {
            let selected = BIG_MON_SIZE_TABLE[translate_table_index(params)];
            assert!(params >= selected.threshold);
        }
    }

    #[test]
    fn imperial_conversion_uses_the_folded_double_constant() {
        assert_eq!(CM_PER_INCH_X10, 2.54_f64 * 10.0);
        // Sizes are carried in tenths, so 100 means 10.0 cm, which is
        // 3.93 in and prints as 39 tenths of an inch.
        assert_eq!((f64::from(100u32 * 10) / CM_PER_INCH_X10) as u32, 39);
        // 254 tenths of a cm is exactly 10.0 in.
        assert_eq!((f64::from(254u32 * 10) / CM_PER_INCH_X10) as u32, 100);
    }
}
