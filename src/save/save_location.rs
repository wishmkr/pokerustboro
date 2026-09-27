use core::ptr::{addr_of, addr_of_mut};

const POKECENTER_SAVEWARP: u8 = 1 << 1;
const LOBBY_SAVEWARP: u8 = 1 << 2;
const UNKNOWN_SAVEWARP: u8 = 1 << 3;
const CHAMPION_SAVEWARP: u8 = 1 << 7;

const POKEMON_CENTER_MAPS: [u16; 38] = [
    0x0202, 0x0203, 0x0301, 0x0302, 0x0405, 0x0406, 0x0504, 0x0505, 0x0604, 0x0605, 0x0700, 0x0701,
    0x0804, 0x0805, 0x090b, 0x090c, 0x0a05, 0x0a06, 0x0b05, 0x0b06, 0x0c02, 0x0c03, 0x0d06, 0x0d07,
    0x0e03, 0x0e04, 0x0f02, 0x0f03, 0x100c, 0x100d, 0x100a, 0x100e, 0x1a35, 0x1a36, 0x1918, 0x1919,
    0x191a, 0x191b,
];
const RELOAD_MAPS: [u16; 1] = [0x1a05];

#[repr(C)]
struct WarpData {
    map_group: i8,
    map_num: i8,
    warp_id: i8,
    padding: u8,
    x: i16,
    y: i16,
}

#[repr(C)]
struct SaveBlock1LocationView {
    position: [u8; 4],
    location: WarpData,
}

#[repr(C)]
struct SaveBlock2WarpView {
    player_name: [u8; 8],
    player_gender: u8,
    special_save_warp_flags: u8,
    middle: [u8; 0x9e],
    gcn_link_flags: u32,
}

unsafe extern "C" {
    static mut gSaveBlock1Ptr: *mut SaveBlock1LocationView;
    static mut gSaveBlock2Ptr: *mut SaveBlock2WarpView;
}

fn contains_map(list: &[u16], map: u16) -> bool {
    list.contains(&map)
}

unsafe fn current_map() -> u16 {
    let save = unsafe { gSaveBlock1Ptr };
    let group = unsafe { addr_of!((*save).location.map_group).read() } as u8;
    let number = unsafe { addr_of!((*save).location.map_num).read() } as u8;
    (u16::from(group) << 8) | u16::from(number)
}

unsafe fn update_warp_flag(flag: u8, enabled: bool) {
    let save = unsafe { gSaveBlock2Ptr };
    let field = unsafe { addr_of_mut!((*save).special_save_warp_flags) };
    let value = unsafe { field.read() };
    unsafe { field.write(if enabled { value | flag } else { value & !flag }) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySetMapSaveWarpStatus() {
    let map = unsafe { current_map() };
    unsafe { update_warp_flag(POKECENTER_SAVEWARP, contains_map(&POKEMON_CENTER_MAPS, map)) };
    unsafe { update_warp_flag(LOBBY_SAVEWARP, contains_map(&RELOAD_MAPS, map)) };
    unsafe { update_warp_flag(UNKNOWN_SAVEWARP, false) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUnlockedPokedexFlags() {
    let save = unsafe { gSaveBlock2Ptr };
    let flags = unsafe { addr_of_mut!((*save).gcn_link_flags) };
    unsafe { flags.write(flags.read() | 0x803f) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetChampionSaveWarp() {
    unsafe { update_warp_flag(CHAMPION_SAVEWARP, true) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_views_match_documented_c_offsets() {
        assert_eq!(core::mem::size_of::<WarpData>(), 8);
        assert_eq!(core::mem::offset_of!(SaveBlock1LocationView, location), 4);
        assert_eq!(
            core::mem::offset_of!(SaveBlock2WarpView, special_save_warp_flags),
            9
        );
        assert_eq!(
            core::mem::offset_of!(SaveBlock2WarpView, gcn_link_flags),
            0xa8
        );
    }

    #[test]
    fn map_lists_match_the_original_sentinelled_tables() {
        assert_eq!(POKEMON_CENTER_MAPS.len(), 38);
        assert!(contains_map(&POKEMON_CENTER_MAPS, 0x0202));
        assert!(contains_map(&POKEMON_CENTER_MAPS, 0x191b));
        assert!(!contains_map(&POKEMON_CENTER_MAPS, 0x1a05));
        assert!(contains_map(&RELOAD_MAPS, 0x1a05));
    }
}
