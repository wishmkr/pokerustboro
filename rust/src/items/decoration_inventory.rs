const CATEGORY_COUNT: usize = 8;
const INVENTORY_SIZE: usize = 8;
const DECORATION_SIZE: usize = 32;
const DECORATION_CATEGORY_OFFSET: usize = 19;

const INVENTORY_OFFSETS_AND_SIZES: [(usize, u8); CATEGORY_COUNT] = [
    (0x2734, 10),
    (0x273e, 10),
    (0x2748, 10),
    (0x2752, 30),
    (0x2770, 30),
    (0x278e, 10),
    (0x2798, 40),
    (0x27c0, 10),
];

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gDecorationInventories: crate::ffi::Align4<[[u8; INVENTORY_SIZE]; CATEGORY_COUNT]> =
    crate::ffi::Align4([[0; INVENTORY_SIZE]; CATEGORY_COUNT]);

unsafe extern "C" {
    static mut gSaveBlock1Ptr: *mut u8;
    static gDecorations: u8;

    fn InitDecorationContextItems();
}

unsafe fn inventory(category: u8) -> *mut u8 {
    unsafe {
        (&raw mut gDecorationInventories)
            .cast::<u8>()
            .add(category as usize * INVENTORY_SIZE)
    }
}

unsafe fn inventory_items(category: u8) -> *mut u8 {
    unsafe { inventory(category).cast::<*mut u8>().read() }
}

unsafe fn inventory_size(category: u8) -> u8 {
    unsafe { inventory(category).add(4).read() }
}

unsafe fn decoration_category(decoration: u8) -> u8 {
    unsafe {
        (&raw const gDecorations)
            .add(decoration as usize * DECORATION_SIZE + DECORATION_CATEGORY_OFFSET)
            .read()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDecorationInventoriesPointers() {
    let save = unsafe { gSaveBlock1Ptr };
    let mut category = 0usize;
    while category < CATEGORY_COUNT {
        let entry = unsafe { inventory(category as u8) };
        let (offset, size) = INVENTORY_OFFSETS_AND_SIZES[category];
        unsafe { entry.cast::<*mut u8>().write(save.add(offset)) };
        unsafe { entry.add(4).write(size) };
        category += 1;
    }
    unsafe { InitDecorationContextItems() };
}

unsafe fn clear_inventory(category: u8) {
    let items = unsafe { inventory_items(category) };
    let size = unsafe { inventory_size(category) } as usize;
    let mut index = 0usize;
    while index < size {
        unsafe { items.add(index).write(0) };
        index += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearDecorationInventories() {
    let mut category = 0u8;
    while category < CATEGORY_COUNT as u8 {
        unsafe { clear_inventory(category) };
        category += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFirstEmptyDecorSlot(category: u8) -> i8 {
    let items = unsafe { inventory_items(category) };
    let size = unsafe { inventory_size(category) } as usize;
    let mut index = 0usize;
    while index < size {
        if unsafe { items.add(index).read() } == 0 {
            return index as i8;
        }
        index += 1;
    }
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckHasDecoration(decoration: u8) -> u8 {
    let category = unsafe { decoration_category(decoration) };
    let items = unsafe { inventory_items(category) };
    let size = unsafe { inventory_size(category) } as usize;
    let mut index = 0usize;
    while index < size {
        if unsafe { items.add(index).read() } == decoration {
            return 1;
        }
        index += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DecorationAdd(decoration: u8) -> u8 {
    if decoration == 0 {
        return 0;
    }
    let category = unsafe { decoration_category(decoration) };
    let index = unsafe { GetFirstEmptyDecorSlot(category) };
    if index < 0 {
        return 0;
    }
    unsafe {
        inventory_items(category)
            .add(index as usize)
            .write(decoration)
    };
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DecorationCheckSpace(decoration: u8) -> u8 {
    if decoration == 0 {
        return 0;
    }
    let category = unsafe { decoration_category(decoration) };
    (unsafe { GetFirstEmptyDecorSlot(category) } != -1) as u8
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DecorationRemove(decoration: u8) -> i8 {
    if decoration == 0 {
        return 0;
    }
    let category = unsafe { decoration_category(decoration) };
    let items = unsafe { inventory_items(category) };
    let size = unsafe { inventory_size(category) } as usize;
    let mut index = 0usize;
    while index < size {
        if unsafe { items.add(index).read() } == decoration {
            unsafe { items.add(index).write(0) };
            unsafe { CondenseDecorationsInCategory(category) };
            return 1;
        }
        index += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CondenseDecorationsInCategory(category: u8) {
    let items = unsafe { inventory_items(category) };
    let size = unsafe { inventory_size(category) } as usize;
    let mut left = 0usize;
    while left < size {
        let mut right = left + 1;
        while right < size {
            let left_value = unsafe { items.add(left).read() };
            let right_value = unsafe { items.add(right).read() };
            if right_value != 0 && (left_value == 0 || left_value > right_value) {
                unsafe { items.add(left).write(right_value) };
                unsafe { items.add(right).write(left_value) };
            }
            right += 1;
        }
        left += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNumOwnedDecorationsInCategory(category: u8) -> u8 {
    let items = unsafe { inventory_items(category) };
    let size = unsafe { inventory_size(category) } as usize;
    let mut count = 0u8;
    let mut index = 0usize;
    while index < size {
        if unsafe { items.add(index).read() } != 0 {
            count += 1;
        }
        index += 1;
    }
    count
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNumOwnedDecorations() -> u8 {
    let mut count = 0u8;
    let mut category = 0u8;
    while category < CATEGORY_COUNT as u8 {
        count = count.wrapping_add(unsafe { GetNumOwnedDecorationsInCategory(category) });
        category += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_offsets_and_arm_inventory_shape_match_c() {
        assert_eq!(INVENTORY_OFFSETS_AND_SIZES[0], (0x2734, 10));
        assert_eq!(INVENTORY_OFFSETS_AND_SIZES[7], (0x27c0, 10));
        assert_eq!(INVENTORY_SIZE, 8);
        assert_eq!(DECORATION_CATEGORY_OFFSET, 19);
    }
}
