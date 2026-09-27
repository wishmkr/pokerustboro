//! The roaming legendary. It wanders a fixed graph of Hoenn routes and can be
//! bumped into on whichever route it currently occupies.

use crate::ffi::{GetMonData2, MON_DATA_PERSONALITY, SetMonData, gSpecialVar_0x8004};
use core::ffi::c_int;

/// Despite the stored map group, the roamer is hard-coded to group 0.
const ROAMER_MAP_GROUP: u8 = 0;

const MAP_GRP: usize = 0;
const MAP_NUM: usize = 1;

/// `MAP_NUM(MAP_UNDEFINED)`, the filler for short location sets.
const NONE: u8 = 0xff;

const SPECIES_LATIAS: u16 = 407;
const SPECIES_LATIOS: u16 = 408;
const USE_RANDOM_IVS: u32 = 32;
const OT_ID_PLAYER_ID: u8 = 0;

const MON_DATA_COOL: c_int = 22;
const MON_DATA_BEAUTY: c_int = 23;
const MON_DATA_CUTE: c_int = 24;
const MON_DATA_SMART: c_int = 33;
const MON_DATA_TOUGH: c_int = 47;
const MON_DATA_STATUS: c_int = 55;
const MON_DATA_HP: c_int = 57;
const MON_DATA_MAX_HP: c_int = 58;
const MON_DATA_IVS: c_int = 66;

/// `offsetof(struct SaveBlock1, roamer)` and `struct Roamer`, 28 bytes.
const SAVE1_ROAMER_OFFSET: usize = 0x31dc;
const ROAMER_SIZE: usize = 28;
const R_IVS: usize = 0x00;
const R_PERSONALITY: usize = 0x04;
const R_SPECIES: usize = 0x08;
const R_HP: usize = 0x0a;
const R_LEVEL: usize = 0x0c;
const R_STATUS: usize = 0x0d;
const R_COOL: usize = 0x0e;
const R_BEAUTY: usize = 0x0f;
const R_CUTE: usize = 0x10;
const R_SMART: usize = 0x11;
const R_TOUGH: usize = 0x12;
const R_ACTIVE: usize = 0x13;

/// `offsetof(struct SaveBlock1, location)`; `mapGroup` then `mapNum`.
const SAVE1_LOCATION_MAP_GROUP: usize = 4;
const SAVE1_LOCATION_MAP_NUM: usize = 5;

const ROAMER_LEVEL: u8 = 40;

/// `MAP_NUM(MAP_ROUTE1xx)` runs linearly from route 110.
const fn route(number: u8) -> u8 {
    25 + (number - 110)
}

/// Each row is a location set: the map it applies to, then the maps the
/// roamer may move to from there.
///
/// Three properties of this table stop the move loops from spinning forever,
/// so preserve them if it is ever edited: at least two sets must begin with
/// different maps, every set needs at least three distinct maps, and every
/// map used should also begin a set of its own.
static ROAMER_LOCATIONS: [[u8; NUM_LOCATIONS_PER_SET]; NUM_LOCATION_SETS + 1] = [
    [
        route(110),
        route(111),
        route(117),
        route(118),
        route(134),
        NONE,
    ],
    [route(111), route(110), route(117), route(118), NONE, NONE],
    [route(117), route(111), route(110), route(118), NONE, NONE],
    [
        route(118),
        route(117),
        route(110),
        route(111),
        route(119),
        route(123),
    ],
    [route(119), route(118), route(120), NONE, NONE, NONE],
    [route(120), route(119), route(121), NONE, NONE, NONE],
    [route(121), route(120), route(122), route(123), NONE, NONE],
    [route(122), route(121), route(123), NONE, NONE, NONE],
    [route(123), route(122), route(118), NONE, NONE, NONE],
    [route(124), route(121), route(125), route(126), NONE, NONE],
    [route(125), route(124), route(127), NONE, NONE, NONE],
    [route(126), route(124), route(127), NONE, NONE, NONE],
    [route(127), route(125), route(126), route(128), NONE, NONE],
    [route(128), route(127), route(129), NONE, NONE, NONE],
    [route(129), route(128), route(130), NONE, NONE, NONE],
    [route(130), route(129), route(131), NONE, NONE, NONE],
    [route(131), route(130), route(132), NONE, NONE, NONE],
    [route(132), route(131), route(133), NONE, NONE, NONE],
    [route(133), route(132), route(134), NONE, NONE, NONE],
    [route(134), route(133), route(110), NONE, NONE, NONE],
    // The trailing row is excluded from the random pick.
    [NONE, NONE, NONE, NONE, NONE, NONE],
];

const NUM_LOCATION_SETS: usize = 20;
const NUM_LOCATIONS_PER_SET: usize = 6;

#[unsafe(link_section = "ewram_data")]
static mut LOCATION_HISTORY: crate::ffi::Align4<[[u8; 2]; 3]> = crate::ffi::Align4([[0; 2]; 3]);

#[unsafe(link_section = "ewram_data")]
static mut ROAMER_LOCATION: [u8; 2] = [0; 2];

unsafe extern "C" {
    static mut gSaveBlock1Ptr: *mut u8;
    static mut gEnemyParty: u8;

    fn Random() -> u16;
    fn ZeroEnemyPartyMons();
    #[allow(clippy::too_many_arguments)]
    fn CreateMon(
        mon: *mut u8,
        species: u16,
        level: u8,
        fixed_iv: u8,
        has_fixed_personality: u8,
        fixed_personality: u32,
        ot_id_type: u8,
        ot_id: u32,
    );
    fn CreateMonWithIVsPersonality(
        mon: *mut u8,
        species: u16,
        level: u32,
        ivs: u32,
        personality: u32,
    );
}

/// `ROAMER` - `&gSaveBlock1Ptr->roamer`
#[inline]
unsafe fn roamer() -> *mut u8 {
    unsafe { gSaveBlock1Ptr.add(SAVE1_ROAMER_OFFSET) }
}

#[inline]
unsafe fn roamer_active() -> bool {
    let active = unsafe { roamer().add(R_ACTIVE).read_volatile() };
    active != 0
}

#[inline]
unsafe fn location(index: usize) -> u8 {
    unsafe {
        (&raw const ROAMER_LOCATION)
            .cast::<u8>()
            .add(index)
            .read_volatile()
    }
}

#[inline]
unsafe fn set_location(index: usize, value: u8) {
    unsafe {
        (&raw mut ROAMER_LOCATION)
            .cast::<u8>()
            .add(index)
            .write_volatile(value)
    };
}

#[inline]
unsafe fn history(slot: usize, index: usize) -> u8 {
    unsafe {
        (&raw const LOCATION_HISTORY)
            .cast::<u8>()
            .add(slot * 2 + index)
            .read_volatile()
    }
}

#[inline]
unsafe fn set_history(slot: usize, index: usize, value: u8) {
    unsafe {
        (&raw mut LOCATION_HISTORY)
            .cast::<u8>()
            .add(slot * 2 + index)
            .write_volatile(value)
    };
}

/// The first mon of the enemy party, which the roamer is built into.
#[inline]
unsafe fn enemy_mon() -> *mut u8 {
    (&raw mut gEnemyParty).cast::<u8>()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearRoamerData() {
    unsafe { core::ptr::write_bytes(roamer(), 0, ROAMER_SIZE) };
    unsafe {
        roamer()
            .add(R_SPECIES)
            .cast::<u16>()
            .write_volatile(SPECIES_LATIAS)
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearRoamerLocationData() {
    for slot in 0..3 {
        unsafe { set_history(slot, MAP_GRP, 0) };
        unsafe { set_history(slot, MAP_NUM, 0) };
    }
    unsafe { set_location(MAP_GRP, 0) };
    unsafe { set_location(MAP_NUM, 0) };
}

unsafe fn create_initial_roamer_mon(create_latios: u16) {
    let species = if create_latios == 0 {
        SPECIES_LATIAS
    } else {
        SPECIES_LATIOS
    };
    let roamer = unsafe { roamer() };
    unsafe { roamer.add(R_SPECIES).cast::<u16>().write_volatile(species) };

    let mon = unsafe { enemy_mon() };
    unsafe {
        CreateMon(
            mon,
            species,
            ROAMER_LEVEL,
            USE_RANDOM_IVS as u8,
            0,
            0,
            OT_ID_PLAYER_ID,
            0,
        )
    };

    unsafe { roamer.add(R_LEVEL).write_volatile(ROAMER_LEVEL) };
    unsafe { roamer.add(R_STATUS).write_volatile(0) };
    unsafe { roamer.add(R_ACTIVE).write_volatile(1) };
    unsafe {
        roamer
            .add(R_IVS)
            .cast::<u32>()
            .write_volatile(GetMonData2(mon, MON_DATA_IVS))
    };
    unsafe {
        roamer
            .add(R_PERSONALITY)
            .cast::<u32>()
            .write_volatile(GetMonData2(mon, MON_DATA_PERSONALITY))
    };
    unsafe {
        roamer
            .add(R_HP)
            .cast::<u16>()
            .write_volatile(GetMonData2(mon, MON_DATA_MAX_HP) as u16)
    };
    unsafe {
        roamer
            .add(R_COOL)
            .write_volatile(GetMonData2(mon, MON_DATA_COOL) as u8)
    };
    unsafe {
        roamer
            .add(R_BEAUTY)
            .write_volatile(GetMonData2(mon, MON_DATA_BEAUTY) as u8)
    };
    unsafe {
        roamer
            .add(R_CUTE)
            .write_volatile(GetMonData2(mon, MON_DATA_CUTE) as u8)
    };
    unsafe {
        roamer
            .add(R_SMART)
            .write_volatile(GetMonData2(mon, MON_DATA_SMART) as u8)
    };
    unsafe {
        roamer
            .add(R_TOUGH)
            .write_volatile(GetMonData2(mon, MON_DATA_TOUGH) as u8)
    };

    unsafe { set_location(MAP_GRP, ROAMER_MAP_GROUP) };
    let set = unsafe { Random() } as usize % NUM_LOCATION_SETS;
    unsafe { set_location(MAP_NUM, ROAMER_LOCATIONS[set][0]) };
}

/// `gSpecialVar_0x8004` carries the MULTI_TV_LATI choice: 0 for Red (Latias),
/// 1 for Blue (Latios).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitRoamer() {
    unsafe { ClearRoamerData() };
    unsafe { ClearRoamerLocationData() };
    let choice = unsafe { (&raw const gSpecialVar_0x8004).read_volatile() };
    unsafe { create_initial_roamer_mon(choice) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateLocationHistoryForRoamer() {
    // Shift the three-entry history along and record where the player is now.
    for index in [MAP_GRP, MAP_NUM] {
        unsafe { set_history(2, index, history(1, index)) };
        unsafe { set_history(1, index, history(0, index)) };
    }

    unsafe {
        set_history(
            0,
            MAP_GRP,
            gSaveBlock1Ptr.add(SAVE1_LOCATION_MAP_GROUP).read_volatile(),
        )
    };
    unsafe {
        set_history(
            0,
            MAP_NUM,
            gSaveBlock1Ptr.add(SAVE1_LOCATION_MAP_NUM).read_volatile(),
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RoamerMoveToOtherLocationSet() {
    if !unsafe { roamer_active() } {
        return;
    }

    unsafe { set_location(MAP_GRP, ROAMER_MAP_GROUP) };

    // Pick a set whose first map differs from where the roamer is now.
    loop {
        let set = unsafe { Random() } as usize % NUM_LOCATION_SETS;
        let map_num = ROAMER_LOCATIONS[set][0];
        if unsafe { location(MAP_NUM) } != map_num {
            unsafe { set_location(MAP_NUM, map_num) };
            return;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RoamerMove() {
    // One move in sixteen jumps to an unrelated part of the map.
    if unsafe { Random() } % 16 == 0 {
        unsafe { RoamerMoveToOtherLocationSet() };
        return;
    }

    if !unsafe { roamer_active() } {
        return;
    }

    let mut loc_set = 0usize;
    while loc_set < NUM_LOCATION_SETS {
        if unsafe { location(MAP_NUM) } == ROAMER_LOCATIONS[loc_set][0] {
            // Pick a neighbour, skipping the set's own map, the filler
            // entries, and wherever the roamer was two moves ago.
            let map_num = loop {
                let pick = unsafe { Random() } as usize % (NUM_LOCATIONS_PER_SET - 1) + 1;
                let candidate = ROAMER_LOCATIONS[loc_set][pick];
                let was_there_recently = unsafe { history(2, MAP_GRP) } == ROAMER_MAP_GROUP
                    && unsafe { history(2, MAP_NUM) } == candidate;
                if !was_there_recently && candidate != NONE {
                    break candidate;
                }
            };
            unsafe { set_location(MAP_NUM, map_num) };
            return;
        }
        loc_set += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRoamerAt(map_group: u8, map_num: u8) -> u8 {
    let here = unsafe { roamer_active() }
        && map_group == unsafe { location(MAP_GRP) }
        && map_num == unsafe { location(MAP_NUM) };
    u8::from(here)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateRoamerMonInstance() {
    let mon = unsafe { enemy_mon() };
    let roamer = unsafe { roamer() };
    unsafe { ZeroEnemyPartyMons() };
    unsafe {
        CreateMonWithIVsPersonality(
            mon,
            roamer.add(R_SPECIES).cast::<u16>().read_volatile(),
            u32::from(roamer.add(R_LEVEL).read_volatile()),
            roamer.add(R_IVS).cast::<u32>().read_volatile(),
            roamer.add(R_PERSONALITY).cast::<u32>().read_volatile(),
        )
    };

    // The roamer's status is a u8 but SetMonData reads four bytes, so this
    // also picks up cool, beauty and cute. That is the retail behaviour and
    // is preserved deliberately; the BUGFIX build copies status to a u32
    // first instead.
    unsafe { SetMonData(mon, MON_DATA_STATUS, roamer.add(R_STATUS).cast()) };
    unsafe { SetMonData(mon, MON_DATA_HP, roamer.add(R_HP).cast()) };
    unsafe { SetMonData(mon, MON_DATA_COOL, roamer.add(R_COOL).cast()) };
    unsafe { SetMonData(mon, MON_DATA_BEAUTY, roamer.add(R_BEAUTY).cast()) };
    unsafe { SetMonData(mon, MON_DATA_CUTE, roamer.add(R_CUTE).cast()) };
    unsafe { SetMonData(mon, MON_DATA_SMART, roamer.add(R_SMART).cast()) };
    unsafe { SetMonData(mon, MON_DATA_TOUGH, roamer.add(R_TOUGH).cast()) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryStartRoamerEncounter() -> u8 {
    let map_group = unsafe { gSaveBlock1Ptr.add(SAVE1_LOCATION_MAP_GROUP).read_volatile() };
    let map_num = unsafe { gSaveBlock1Ptr.add(SAVE1_LOCATION_MAP_NUM).read_volatile() };

    if unsafe { IsRoamerAt(map_group, map_num) } == 1 && unsafe { Random() } % 4 == 0 {
        unsafe { CreateRoamerMonInstance() };
        1
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateRoamerHPStatus(mon: *mut u8) {
    let roamer = unsafe { roamer() };
    unsafe {
        roamer
            .add(R_HP)
            .cast::<u16>()
            .write_volatile(GetMonData2(mon, MON_DATA_HP) as u16)
    };
    unsafe {
        roamer
            .add(R_STATUS)
            .write_volatile(GetMonData2(mon, MON_DATA_STATUS) as u8)
    };

    unsafe { RoamerMoveToOtherLocationSet() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetRoamerInactive() {
    unsafe { roamer().add(R_ACTIVE).write_volatile(0) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRoamerLocation(map_group: *mut u8, map_num: *mut u8) {
    unsafe { map_group.write(location(MAP_GRP)) };
    unsafe { map_num.write(location(MAP_NUM)) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roamer_field_offsets_match_the_arm_structure() {
        assert_eq!(R_IVS, 0x00);
        assert_eq!(R_PERSONALITY, 0x04);
        assert_eq!(R_SPECIES, 0x08);
        assert_eq!(R_HP, 0x0a);
        assert_eq!(R_ACTIVE, 0x13);
        // Twenty used bytes plus an eight-byte filler.
        assert_eq!(ROAMER_SIZE, 0x14 + 8);
    }

    #[test]
    fn setting_status_also_overwrites_the_three_contest_stats() {
        // SetMonData reads four bytes from a one-byte field, so it spans
        // status, cool, beauty and cute. This is retail behaviour.
        assert_eq!(R_STATUS + 1, R_COOL);
        assert_eq!(R_COOL + 1, R_BEAUTY);
        assert_eq!(R_BEAUTY + 1, R_CUTE);
    }

    #[test]
    fn route_numbers_are_linear_from_route_110() {
        assert_eq!(route(110), 25);
        assert_eq!(route(117), 32);
        assert_eq!(route(134), 49);
    }

    #[test]
    fn the_location_table_cannot_spin_the_move_loops() {
        // Every set must start with a distinct map, or
        // RoamerMoveToOtherLocationSet never finds a different one.
        let mut firsts: [u8; NUM_LOCATION_SETS] = [0; NUM_LOCATION_SETS];
        for i in 0..NUM_LOCATION_SETS {
            firsts[i] = ROAMER_LOCATIONS[i][0];
        }
        for i in 0..NUM_LOCATION_SETS {
            for j in (i + 1)..NUM_LOCATION_SETS {
                assert_ne!(firsts[i], firsts[j], "sets {i} and {j} start alike");
            }
        }

        // Every set needs at least three real maps, or RoamerMove's inner
        // loop can reject every candidate forever.
        for (i, set) in ROAMER_LOCATIONS.iter().take(NUM_LOCATION_SETS).enumerate() {
            let real = set.iter().filter(|&&m| m != NONE).count();
            assert!(real >= 3, "set {i} has only {real} maps");
        }
    }

    #[test]
    fn the_trailing_row_is_excluded_from_the_random_pick() {
        assert_eq!(ROAMER_LOCATIONS.len(), NUM_LOCATION_SETS + 1);
        assert_eq!(ROAMER_LOCATIONS[NUM_LOCATION_SETS], [NONE; 6]);
    }
}
