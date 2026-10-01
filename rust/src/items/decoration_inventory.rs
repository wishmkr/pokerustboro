//! The player's decorations for their secret base (was
//! src/decoration_inventory.c).
//!
//! Decorations come in eight categories (desks, chairs, plants, ...), each
//! a short list of decoration ids in the save, kept sorted with the empty
//! slots (0) last. `gDecorationInventories` points each category at its
//! list.

use crate::c::CArray;
use crate::save_blocks::save_block1;
use crate::types::{Decoration, DecorationInventory};

pub const DECORCAT_COUNT: usize = 8;
pub const DECOR_NONE: u8 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gDecorationInventories: CArray<DecorationInventory, DECORCAT_COUNT> =
    unsafe { core::mem::zeroed() };

/// `InitDecorationContextItems` with this module's view of its types.
#[inline]
unsafe fn InitDecorationContextItems() {
    unsafe {
        crate::decoration::InitDecorationContextItems();
    }
}

/// The category of decoration `decor`.
fn category_of(decor: u8) -> u8 {
    // SAFETY: the decoration table covers every decoration id.
    unsafe {
        (&*(&raw const crate::data::decoration::gDecorations).cast::<CArray<Decoration, 0>>())
            [decor]
            .category
    }
}

/// Category `category`'s list.
///
/// # Safety
/// The inventories must be set up ([`set_decoration_inventories_pointers`])
/// and the list not in use elsewhere while the result lives.
unsafe fn inventory<'a>(category: u8) -> &'a mut [u8] {
    unsafe {
        let inventories = &*(&raw const gDecorationInventories);
        let inv = inventories[category];
        core::slice::from_raw_parts_mut(inv.items, usize::from(inv.size))
    }
}

/// Points each category at its list in the save.
pub fn set_decoration_inventories_pointers() {
    // SAFETY: the save blocks are set up at boot; the inventories are only
    // pointed at them here.
    unsafe {
        let save = save_block1();
        let lists: [(*mut u8, usize); DECORCAT_COUNT] = [
            (
                save.decorationDesks.as_mut_ptr(),
                save.decorationDesks.len(),
            ),
            (
                save.decorationChairs.as_mut_ptr(),
                save.decorationChairs.len(),
            ),
            (
                save.decorationPlants.as_mut_ptr(),
                save.decorationPlants.len(),
            ),
            (
                save.decorationOrnaments.as_mut_ptr(),
                save.decorationOrnaments.len(),
            ),
            (save.decorationMats.as_mut_ptr(), save.decorationMats.len()),
            (
                save.decorationPosters.as_mut_ptr(),
                save.decorationPosters.len(),
            ),
            (
                save.decorationDolls.as_mut_ptr(),
                save.decorationDolls.len(),
            ),
            (
                save.decorationCushions.as_mut_ptr(),
                save.decorationCushions.len(),
            ),
        ];
        let inventories = &mut *(&raw mut gDecorationInventories);
        for (inv, (items, size)) in inventories.0.iter_mut().zip(lists) {
            *inv = DecorationInventory {
                items,
                size: size as u8,
            };
        }
        InitDecorationContextItems();
    }
}

pub fn clear_decoration_inventories() {
    for category in 0..DECORCAT_COUNT as u8 {
        // SAFETY: set up with the save; the borrow ends here.
        unsafe { inventory(category) }.fill(DECOR_NONE);
    }
}

/// The first empty slot of a category.
pub fn first_empty_decor_slot(category: u8) -> Option<usize> {
    // SAFETY: as in clear_decoration_inventories.
    unsafe { inventory(category) }
        .iter()
        .position(|&d| d == DECOR_NONE)
}

pub fn has_decoration(decor: u8) -> bool {
    // SAFETY: as in clear_decoration_inventories.
    unsafe { inventory(category_of(decor)) }
        .iter()
        .any(|&d| d == decor)
}

/// Adds a decoration; false if it's none or its category is full.
pub fn add_decoration(decor: u8) -> bool {
    if decor == DECOR_NONE {
        return false;
    }
    let category = category_of(decor);
    let Some(slot) = first_empty_decor_slot(category) else {
        return false;
    };
    // SAFETY: as in clear_decoration_inventories.
    if let Some(item) = unsafe { inventory(category) }.get_mut(slot) {
        *item = decor;
    }
    true
}

pub fn has_space_for(decor: u8) -> bool {
    decor != DECOR_NONE && first_empty_decor_slot(category_of(decor)).is_some()
}

/// Removes one of a decoration (keeping its category sorted); false if the
/// player has none.
pub fn remove_decoration(decor: u8) -> bool {
    if decor == DECOR_NONE {
        return false;
    }
    let category = category_of(decor);
    // SAFETY: as in clear_decoration_inventories.
    let list = unsafe { inventory(category) };
    let Some(item) = list.iter_mut().find(|d| **d == decor) else {
        return false;
    };
    *item = DECOR_NONE;
    condense_decorations_in_category(category);
    true
}

/// Sorts a category by id with the empty slots last (C's exact swap order).
pub fn condense_decorations_in_category(category: u8) {
    // SAFETY: as in clear_decoration_inventories.
    let list = unsafe { inventory(category) };
    let n = list.len();
    for i in 0..n {
        for j in i + 1..n {
            if let Ok([a, b]) = list.get_disjoint_mut([i, j]) {
                if *b != DECOR_NONE && (*a == DECOR_NONE || *a > *b) {
                    core::mem::swap(a, b);
                }
            }
        }
    }
}

pub fn owned_in_category(category: u8) -> u8 {
    // SAFETY: as in clear_decoration_inventories.
    unsafe { inventory(category) }
        .iter()
        .filter(|&&d| d != DECOR_NONE)
        .count() as u8
}

pub fn owned_decorations() -> u8 {
    (0..DECORCAT_COUNT as u8).fold(0u8, |sum, c| sum.wrapping_add(owned_in_category(c)))
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub fn SetDecorationInventoriesPointers() {
    set_decoration_inventories_pointers();
}

#[unsafe(no_mangle)]
pub fn ClearDecorationInventories() {
    clear_decoration_inventories();
}

#[unsafe(no_mangle)]
pub fn GetFirstEmptyDecorSlot(category: u8) -> i8 {
    first_empty_decor_slot(category).map_or(-1, |slot| slot as i8)
}

#[unsafe(no_mangle)]
pub fn CheckHasDecoration(decor: u8) -> u8 {
    has_decoration(decor).into()
}

#[unsafe(no_mangle)]
pub fn DecorationAdd(decor: u8) -> u8 {
    add_decoration(decor).into()
}

#[unsafe(no_mangle)]
pub fn DecorationCheckSpace(decor: u8) -> u8 {
    has_space_for(decor).into()
}

#[unsafe(no_mangle)]
pub fn DecorationRemove(decor: u8) -> i8 {
    remove_decoration(decor).into()
}

#[unsafe(no_mangle)]
pub fn CondenseDecorationsInCategory(category: u8) {
    condense_decorations_in_category(category);
}

#[unsafe(no_mangle)]
pub fn GetNumOwnedDecorationsInCategory(category: u8) -> u8 {
    owned_in_category(category)
}

#[unsafe(no_mangle)]
pub fn GetNumOwnedDecorations() -> u8 {
    owned_decorations()
}
