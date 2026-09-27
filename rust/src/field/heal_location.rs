use core::ptr;

#[repr(C, align(4))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HealLocation {
    map_group: i8,
    map_num: i8,
    x: u16,
    y: u16,
}

const fn location(map_group: i8, map_num: i8, x: u16, y: u16) -> HealLocation {
    HealLocation {
        map_group,
        map_num,
        x,
        y,
    }
}

#[unsafe(link_section = ".rodata")]
static HEAL_LOCATIONS: [HealLocation; 22] = [
    location(1, 1, 4, 2),
    location(1, 3, 4, 2),
    location(0, 0, 20, 17),
    location(0, 1, 19, 20),
    location(0, 2, 22, 6),
    location(0, 3, 16, 39),
    location(0, 4, 5, 7),
    location(0, 5, 24, 15),
    location(0, 6, 28, 17),
    location(0, 7, 43, 32),
    location(0, 8, 27, 49),
    location(0, 9, 5, 9),
    location(0, 9, 14, 9),
    location(0, 10, 6, 17),
    location(0, 11, 2, 11),
    location(0, 12, 9, 7),
    location(0, 13, 14, 8),
    location(0, 14, 16, 4),
    location(0, 15, 8, 16),
    location(0, 8, 18, 6),
    location(26, 9, 15, 20),
    location(26, 14, 3, 52),
];

#[unsafe(no_mangle)]
pub extern "C" fn GetHealLocationIndexByMap(map_group: u16, map_num: u16) -> u32 {
    let mut index = 0;
    while index < HEAL_LOCATIONS.len() {
        let entry = unsafe {
            (&raw const HEAL_LOCATIONS)
                .cast::<HealLocation>()
                .add(index)
                .read()
        };
        if i32::from(entry.map_group) == i32::from(map_group)
            && i32::from(entry.map_num) == i32::from(map_num)
        {
            return index as u32 + 1;
        }
        index += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn GetHealLocationByMap(map_group: u16, map_num: u16) -> *const HealLocation {
    let index = GetHealLocationIndexByMap(map_group, map_num);
    GetHealLocation(index)
}

#[unsafe(no_mangle)]
pub extern "C" fn GetHealLocation(index: u32) -> *const HealLocation {
    if index == 0 || index > HEAL_LOCATIONS.len() as u32 {
        ptr::null()
    } else {
        unsafe {
            (&raw const HEAL_LOCATIONS)
                .cast::<HealLocation>()
                .add(index as usize - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arm_c_layout_is_preserved() {
        assert_eq!(core::mem::size_of::<HealLocation>(), 8);
        assert_eq!(core::mem::align_of::<HealLocation>(), 4);
        assert_eq!(core::mem::size_of_val(&HEAL_LOCATIONS), 176);
    }

    #[test]
    fn lookups_match_generated_location_order() {
        assert_eq!(GetHealLocationIndexByMap(1, 1), 1);
        assert_eq!(GetHealLocationIndexByMap(0, 9), 12);
        assert_eq!(GetHealLocationIndexByMap(26, 14), 22);
        assert_eq!(GetHealLocationIndexByMap(99, 99), 0);

        assert!(GetHealLocation(0).is_null());
        assert!(GetHealLocation(23).is_null());
        let last = unsafe { *GetHealLocation(22) };
        assert_eq!(last, location(26, 14, 3, 52));
    }
}
