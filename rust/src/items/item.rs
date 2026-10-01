//! The bag, the PC's item storage and the item table (was src/item.c).
//!
//! The bag has five pockets ([`ITEMS_POCKET`] .. [`KEYITEMS_POCKET`]), each
//! a list of [`ItemSlot`]s in the save whose quantities are XORed with the
//! save's encryption key. `gBagPockets` points each pocket at its slots.
//! Inside the Battle Pyramid the player uses a separate bag, the pyramid
//! bag, and the bag functions switch to it there.
//!
//! The safe API is the [`Pocket`] type and the functions below it
//! ([`bag_has_item`], [`add_bag_item`], [`item_info`]...). They keep C's
//! exact results, including its quirks (16-bit counts that wrap, the
//! "should be return TRUE" breaks), so the game behaves as before. The C
//! names at the end are thin bridges for code that still calls them.

// The safe API is ahead of its callers: most still use the C names.
#![allow(dead_code)]

use crate::battle_pyramid_bag::gPyramidBagMenuState;
use crate::c::{CArray, Table};
use crate::consts::{
    ITEM_ACRO_BIKE, ITEM_BRIGHT_POWDER, ITEM_CHERI_BERRY, ITEM_ENIGMA_BERRY, ITEM_MACH_BIKE,
    ITEM_POKE_BALL,
};
use crate::event_data::{flag_get, var_get, var_set};
use crate::ffi::gSpecialVar_Result;
use crate::save_blocks::{save_block1, save_block2};
use crate::types::{BagPocket, Berry, Item, ItemSlot};

pub const ITEM_NONE: u16 = 0;
/// `ITEMS_COUNT`: ids from here on read as `ITEM_NONE`.
const ITEMS_COUNT: u16 = 377;

/// Pocket indices (a pocket *number*, as `Item::pocket` stores it, is the
/// index plus one; 0 is "no pocket").
pub const ITEMS_POCKET: usize = 0;
pub const BALLS_POCKET: usize = 1;
pub const TMHM_POCKET: usize = 2;
pub const BERRIES_POCKET: usize = 3;
pub const KEYITEMS_POCKET: usize = 4;
pub const POCKETS_COUNT: usize = 5;
const POCKET_NONE: u8 = 0;

pub const MAX_BAG_ITEM_CAPACITY: u16 = 99;
pub const MAX_BERRY_CAPACITY: u16 = 999;
pub const MAX_PC_ITEM_CAPACITY: u16 = 999;
const PC_ITEMS_COUNT: usize = 50;
const PYRAMID_BAG_ITEMS_COUNT: usize = 10;

const FIRST_BERRY_INDEX: u16 = ITEM_CHERI_BERRY;
const LAST_BERRY_INDEX: u16 = ITEM_ENIGMA_BERRY;

const PYRAMID_LOCATION_NONE: u8 = 0;
const FLAG_STORING_ITEMS_IN_PYRAMID_BAG: u16 = 0x4004;
const VAR_SECRET_BASE_LAST_ITEM_USED: u16 = 0x40ed;
const VAR_SECRET_BASE_LOW_TV_FLAGS: u16 = 0x40ee;
const SECRET_BASE_USED_BAG: u16 = 0x200;
const CHAR_SPACE: u8 = 0;

static ITEMS: Table<CArray<Item, { ITEMS_COUNT as usize }>> =
    Table((&raw const crate::data::item::gItems).cast());

/// The bag's pockets: pointers to their slots in the save, and how many
/// slots each has. Set by [`set_bag_items_pointers`].
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBagPockets: CArray<BagPocket, POCKETS_COUNT> = CArray(
    [BagPocket {
        itemSlots: core::ptr::null_mut(),
        capacity: 0,
    }; POCKETS_COUNT],
);

/// `ApplyNewEncryptionKeyToHword` with this module's view of its types.
#[inline]
unsafe fn ApplyNewEncryptionKeyToHword(a0: *mut u16, a1: u32) {
    unsafe {
        crate::load_save::ApplyNewEncryptionKeyToHword(a0 as _, a1);
    }
}
/// `CurMapIsSecretBase` with this module's view of its types.
#[inline]
unsafe fn CurMapIsSecretBase() -> u8 {
    unsafe { crate::secret_base::CurMapIsSecretBase() }
}
/// `CurrentBattlePyramidLocation` with this module's view of its types.
#[inline]
unsafe fn CurrentBattlePyramidLocation() -> u8 {
    unsafe { crate::battle_pyramid::CurrentBattlePyramidLocation() }
}
/// `GetItemListPosition` with this module's view of its types.
#[inline]
unsafe fn GetItemListPosition(a0: u8) -> u8 {
    unsafe { crate::item_menu::GetItemListPosition(a0) }
}
/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *const u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}

// --------------------------------------------------------------- item table

/// The item table entry of `item` (`ITEM_NONE`'s for an id past the table).
pub fn item_info(item: u16) -> &'static Item {
    let item = if item >= ITEMS_COUNT { ITEM_NONE } else { item };
    &ITEMS[item]
}

/// The pocket number of `item` (0 for none).
pub fn item_pocket(item: u16) -> u8 {
    item_info(item).pocket
}

// -------------------------------------------------------------- bag pocket

/// One pocket of the bag.
pub struct Pocket<'a> {
    index: usize,
    slots: &'a mut [ItemSlot],
    key: u16,
}

impl<'a> Pocket<'a> {
    /// Pocket `index` over `slots`, whose quantities are encrypted with `key`
    /// (only its low 16 bits matter).
    pub fn new(index: usize, slots: &'a mut [ItemSlot], key: u32) -> Self {
        Self {
            index,
            slots,
            key: key as u16,
        }
    }

    pub fn slots(&self) -> &[ItemSlot] {
        self.slots
    }

    /// The quantity in `slot` (0 past the end).
    pub fn quantity(&self, slot: usize) -> u16 {
        self.slots.get(slot).map_or(0, |s| s.quantity ^ self.key)
    }

    /// How many of one item a slot can hold.
    fn slot_capacity(&self) -> u16 {
        if self.index == BERRIES_POCKET {
            MAX_BERRY_CAPACITY
        } else {
            MAX_BAG_ITEM_CAPACITY
        }
    }

    /// TMs, HMs and berries never take a second slot.
    fn one_slot_per_item(&self) -> bool {
        self.index == TMHM_POCKET || self.index == BERRIES_POCKET
    }

    pub fn is_empty(&self) -> bool {
        self.slots.iter().all(|s| s.itemId == ITEM_NONE)
    }

    /// Whether the pocket holds `count` of `item`, over all its slots.
    pub fn has_item(&self, item: u16, mut count: u16) -> bool {
        for slot in self.slots.iter().filter(|s| s.itemId == item) {
            let quantity = slot.quantity ^ self.key;
            if quantity >= count {
                return true;
            }
            count = count.wrapping_sub(quantity);
            if count == 0 {
                return true;
            }
        }
        false
    }

    /// Whether `count` more of `item` fit.
    pub fn has_space(&self, item: u16, mut count: u16) -> bool {
        let cap = self.slot_capacity();
        for slot in self.slots.iter().filter(|s| s.itemId == item) {
            let owned = slot.quantity ^ self.key;
            if u32::from(owned) + u32::from(count) <= u32::from(cap) {
                return true;
            }
            if self.one_slot_per_item() {
                return false;
            }
            count = count.wrapping_sub(cap.wrapping_sub(owned));
            if count == 0 {
                break; // C breaks here instead of returning
            }
        }
        if count > 0 {
            for _ in self.slots.iter().filter(|s| s.itemId == ITEM_NONE) {
                if count > cap {
                    if self.one_slot_per_item() {
                        return false;
                    }
                    count -= cap;
                } else {
                    count = 0;
                    break;
                }
            }
            if count > 0 {
                return false; // the pocket is full
            }
        }
        true
    }

    /// Adds `count` of `item`: first to its slots, then to empty ones. If
    /// they don't all fit, nothing changes and it returns false.
    pub fn add(&mut self, item: u16, mut count: u16) -> bool {
        // work on a copy and keep it only if everything fits
        const EMPTY: ItemSlot = ItemSlot {
            itemId: ITEM_NONE,
            quantity: 0,
        };
        let mut buffer = [EMPTY; 64];
        let Some(copy) = buffer.get_mut(..self.slots.len()) else {
            return false;
        };
        for (dst, src) in copy.iter_mut().zip(self.slots.iter()) {
            *dst = *src;
        }
        let (cap, one_slot, key) = (self.slot_capacity(), self.one_slot_per_item(), self.key);

        let mut done = false;
        for slot in copy.iter_mut().filter(|s| s.itemId == item) {
            let owned = slot.quantity ^ key;
            if u32::from(owned) + u32::from(count) <= u32::from(cap) {
                slot.quantity = (owned + count) ^ key;
                done = true;
                break;
            }
            if one_slot {
                return false;
            }
            count = count.wrapping_sub(cap.wrapping_sub(owned));
            slot.quantity = cap ^ key;
            if count == 0 {
                break;
            }
        }
        if !done && count > 0 {
            for slot in copy.iter_mut().filter(|s| s.itemId == ITEM_NONE) {
                slot.itemId = item;
                if count > cap {
                    if one_slot {
                        return false;
                    }
                    count -= cap;
                    slot.quantity = cap ^ key;
                } else {
                    slot.quantity = count ^ key;
                    count = 0;
                    break;
                }
            }
            if count > 0 {
                return false;
            }
        }
        for (dst, src) in self.slots.iter_mut().zip(copy.iter()) {
            *dst = *src;
        }
        true
    }

    /// Takes up to `count` of `item` from one slot (emptying it if it runs
    /// out); returns how many are still to take.
    fn take_from(slot: &mut ItemSlot, key: u16, item: u16, count: u16) -> u16 {
        if slot.itemId != item {
            return count;
        }
        let owned = slot.quantity ^ key;
        let (left, remaining) = if owned >= count {
            (owned - count, 0)
        } else {
            (0, count - owned)
        };
        slot.quantity = left ^ key;
        if left == 0 {
            slot.itemId = ITEM_NONE;
        }
        remaining
    }

    /// Removes `count` of `item`, starting with slot `first` (where the bag
    /// menu's cursor is) and then in order. Returns false, changing
    /// nothing, if the pocket holds fewer.
    pub fn remove(&mut self, item: u16, mut count: u16, first: usize) -> bool {
        if self.count(item) < count {
            return false;
        }
        let key = self.key;
        if let Some(slot) = self.slots.get_mut(first) {
            if slot.itemId == item {
                count = Self::take_from(slot, key, item, count);
                if count == 0 {
                    return true;
                }
            }
        }
        for slot in self.slots.iter_mut() {
            count = Self::take_from(slot, key, item, count);
            if count == 0 {
                return true;
            }
        }
        true
    }

    /// Total quantity of `item` (wraps past 65535, as in C).
    pub fn count(&self, item: u16) -> u16 {
        self.slots
            .iter()
            .filter(|s| s.itemId == item)
            .fold(0u16, |sum, s| sum.wrapping_add(s.quantity ^ self.key))
    }

    /// Empties every slot.
    pub fn clear(&mut self) {
        for slot in self.slots.iter_mut() {
            *slot = ItemSlot {
                itemId: ITEM_NONE,
                quantity: self.key,
            };
        }
    }

    /// Moves the slots holding nothing to the end (C's exact swap order).
    pub fn compact(&mut self) {
        let key = self.key;
        swap_pairs(self.slots, |a, _| a.quantity ^ key == 0);
    }

    /// Sorts by item id, empty slots last (for berries and TMs/HMs).
    pub fn sort_by_item_id(&mut self) {
        let key = self.key;
        swap_pairs(self.slots, |a, b| {
            a.quantity ^ key == 0 || (b.quantity ^ key != 0 && a.itemId > b.itemId)
        });
    }
}

/// C's sorting loop: for every pair i < j in order, swap them when
/// `should_swap(slots[i], slots[j])`.
fn swap_pairs(slots: &mut [ItemSlot], should_swap: impl Fn(&ItemSlot, &ItemSlot) -> bool) {
    let n = slots.len();
    for i in 0..n.saturating_sub(1) {
        for j in i + 1..n {
            if let Ok([a, b]) = slots.get_disjoint_mut([i, j]) {
                if should_swap(a, b) {
                    core::mem::swap(a, b);
                }
            }
        }
    }
}

/// Moves the slot at `from` to `to` (as the bag menu's "move item" does:
/// moving down lands just above `to`).
pub fn move_item_slot(slots: &mut [ItemSlot], from: usize, to: usize) {
    let swap_next = |slots: &mut [ItemSlot], i: usize| {
        if let Ok([a, b]) = slots.get_disjoint_mut([i, i + 1]) {
            core::mem::swap(a, b);
        }
    };
    if to > from {
        for i in from..to - 1 {
            swap_next(slots, i);
        }
    } else {
        for i in (to..from).rev() {
            swap_next(slots, i);
        }
    }
}

// --------------------------------------------------------------------- bag

/// Pocket `index` of the bag.
///
/// # Safety
/// `gBagPockets` must be set up ([`set_bag_items_pointers`]) and the pocket
/// not in use elsewhere while the result lives.
unsafe fn bag_pocket<'a>(index: usize) -> Pocket<'a> {
    unsafe {
        let pocket = gBagPockets[index];
        let slots = core::slice::from_raw_parts_mut(pocket.itemSlots, usize::from(pocket.capacity));
        Pocket::new(index, slots, save_block2().encryptionKey)
    }
}

/// Points `gBagPockets` at the pockets in the save.
pub fn set_bag_items_pointers() {
    // SAFETY: the save blocks are set up at boot; gBagPockets is only
    // pointed at them here.
    unsafe {
        let save = save_block1();
        let pockets: [(*mut ItemSlot, usize); POCKETS_COUNT] = [
            (
                save.bagPocket_Items.as_mut_ptr(),
                save.bagPocket_Items.len(),
            ),
            (
                save.bagPocket_PokeBalls.as_mut_ptr(),
                save.bagPocket_PokeBalls.len(),
            ),
            (save.bagPocket_TMHM.as_mut_ptr(), save.bagPocket_TMHM.len()),
            (
                save.bagPocket_Berries.as_mut_ptr(),
                save.bagPocket_Berries.len(),
            ),
            (
                save.bagPocket_KeyItems.as_mut_ptr(),
                save.bagPocket_KeyItems.len(),
            ),
        ];
        for (i, (slots, capacity)) in pockets.into_iter().enumerate() {
            gBagPockets[i] = BagPocket {
                itemSlots: slots,
                capacity: capacity as u8,
            };
        }
    }
}

/// Whether the bag in use is the Battle Pyramid's.
fn in_pyramid_bag() -> bool {
    // SAFETY: plain C function with no preconditions.
    let location = unsafe { CurrentBattlePyramidLocation() };
    location != PYRAMID_LOCATION_NONE || flag_get(FLAG_STORING_ITEMS_IN_PYRAMID_BAG)
}

/// The pocket index of `item`, if it goes in the bag at all.
fn pocket_index(item: u16) -> Option<usize> {
    match item_pocket(item) {
        POCKET_NONE => None,
        number => Some(usize::from(number) - 1),
    }
}

pub fn bag_has_item(item: u16, count: u16) -> bool {
    let Some(index) = pocket_index(item) else {
        return false;
    };
    if in_pyramid_bag() {
        return pyramid_bag_has_item(item, count);
    }
    // SAFETY: gBagPockets is set up with the save; the borrow ends here.
    unsafe { bag_pocket(index) }.has_item(item, count)
}

pub fn bag_has_space(item: u16, count: u16) -> bool {
    let Some(index) = pocket_index(item) else {
        return false;
    };
    if in_pyramid_bag() {
        return pyramid_bag_has_space(item, count);
    }
    // SAFETY: as in bag_has_item.
    unsafe { bag_pocket(index) }.has_space(item, count)
}

pub fn add_bag_item(item: u16, count: u16) -> bool {
    let Some(index) = pocket_index(item) else {
        return false;
    };
    if in_pyramid_bag() {
        return add_pyramid_bag_item(item, count);
    }
    // SAFETY: as in bag_has_item.
    unsafe { bag_pocket(index) }.add(item, count)
}

pub fn remove_bag_item(item: u16, count: u16) -> bool {
    let Some(index) = pocket_index(item) else {
        return false;
    };
    if item == ITEM_NONE {
        return false;
    }
    if in_pyramid_bag() {
        return remove_pyramid_bag_item(item, count);
    }
    // SAFETY: as in bag_has_item.
    let mut pocket = unsafe { bag_pocket(index) };
    if pocket.count(item) < count {
        return false;
    }
    // SAFETY: plain C functions.
    if unsafe { CurMapIsSecretBase() } != 0 {
        var_set(
            VAR_SECRET_BASE_LOW_TV_FLAGS,
            var_get(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_BAG,
        );
        var_set(VAR_SECRET_BASE_LAST_ITEM_USED, item);
    }
    let first = usize::from(unsafe { GetItemListPosition(index as u8) });
    pocket.remove(item, count, first)
}

/// Whether bag pocket `index` is empty (true for an invalid index).
pub fn bag_pocket_is_empty(index: usize) -> bool {
    // SAFETY: as in bag_has_item.
    index >= POCKETS_COUNT || unsafe { bag_pocket(index) }.is_empty()
}

/// Whether the player has any berry. Also sets `VAR_RESULT`.
pub fn has_at_least_one_berry() -> bool {
    let found = (FIRST_BERRY_INDEX..ITEM_BRIGHT_POWDER).any(|item| bag_has_item(item, 1));
    // SAFETY: a plain global.
    unsafe { gSpecialVar_Result = found.into() };
    found
}

/// Total quantity of `item` in its bag pocket (0 if it has none).
pub fn count_in_bag(item: u16) -> u16 {
    // SAFETY: as in bag_has_item.
    pocket_index(item).map_or(0, |index| unsafe { bag_pocket(index) }.count(item))
}

pub fn clear_bag() {
    for index in 0..POCKETS_COUNT {
        // SAFETY: as in bag_has_item.
        unsafe { bag_pocket(index) }.clear();
    }
}

/// Swaps the registered bike between the Mach Bike and the Acro Bike.
pub fn swap_registered_bike() {
    // SAFETY: the save blocks are set up at boot; the borrow ends here.
    let save = unsafe { save_block1() };
    save.registeredItem = match save.registeredItem {
        ITEM_MACH_BIKE => ITEM_ACRO_BIKE,
        ITEM_ACRO_BIKE => ITEM_MACH_BIKE,
        other => other,
    };
}

// ------------------------------------------------------------------ PC items

/// The PC's items (quantities are not encrypted).
fn pc_items() -> &'static mut CArray<ItemSlot, PC_ITEMS_COUNT> {
    // SAFETY: the save blocks are set up at boot; callers keep the borrow short.
    &mut unsafe { save_block1() }.pcItems
}

pub fn count_used_pc_item_slots() -> u8 {
    pc_items()
        .0
        .iter()
        .filter(|s| s.itemId != ITEM_NONE)
        .count() as u8
}

pub fn pc_has_item(item: u16, count: u16) -> bool {
    pc_items()
        .0
        .iter()
        .any(|s| s.itemId == item && s.quantity >= count)
}

/// Adds `count` of `item` to the PC. Returns false, changing nothing, if
/// there's no room.
pub fn add_pc_item(item: u16, mut count: u16) -> bool {
    let mut items = *pc_items();
    for slot in items.0.iter_mut() {
        if slot.itemId == item {
            if u32::from(slot.quantity) + u32::from(count) <= u32::from(MAX_PC_ITEM_CAPACITY) {
                slot.quantity += count;
                *pc_items() = items;
                return true;
            }
            count = count
                .wrapping_add(slot.quantity)
                .wrapping_sub(MAX_PC_ITEM_CAPACITY);
            slot.quantity = MAX_PC_ITEM_CAPACITY;
            if count == 0 {
                *pc_items() = items;
                return true;
            }
        }
    }
    if count > 0 {
        // C looks for the free slot in the save, not in its copy
        let Some(free) = pc_items().0.iter().position(|s| s.itemId == ITEM_NONE) else {
            return false;
        };
        items[free] = ItemSlot {
            itemId: item,
            quantity: count,
        };
    }
    *pc_items() = items;
    true
}

/// Takes `count` from PC slot `index`; an emptied slot is removed.
pub fn remove_pc_item(index: u8, count: u16) {
    let slot = &mut pc_items()[index];
    slot.quantity = slot.quantity.wrapping_sub(count);
    if slot.quantity == 0 {
        slot.itemId = ITEM_NONE;
        compact_pc_items();
    }
}

/// Moves the empty PC slots to the end (C's exact swap order).
pub fn compact_pc_items() {
    swap_pairs(&mut pc_items().0, |a, _| a.itemId == ITEM_NONE);
}

// ------------------------------------------------------ Battle Pyramid bag

/// The pyramid bag for the current level mode: item ids and quantities.
fn pyramid_bag() -> (
    &'static mut CArray<u16, PYRAMID_BAG_ITEMS_COUNT>,
    &'static mut CArray<u8, PYRAMID_BAG_ITEMS_COUNT>,
) {
    // SAFETY: the save blocks are set up at boot; callers keep the borrows short.
    let frontier = &mut unsafe { save_block2() }.frontier;
    let mode = usize::from(frontier.lvlMode());
    let bag = &mut frontier.pyramidBag;
    (&mut bag.itemId[mode], &mut bag.quantity[mode])
}

fn pyramid_bag_has_item(item: u16, mut count: u16) -> bool {
    let (items, quantities) = pyramid_bag();
    for i in 0..PYRAMID_BAG_ITEMS_COUNT {
        if items[i] == item {
            let quantity = u16::from(quantities[i]);
            if quantity >= count {
                return true;
            }
            count = count.wrapping_sub(quantity);
            if count == 0 {
                return true;
            }
        }
    }
    false
}

fn pyramid_bag_has_space(item: u16, mut count: u16) -> bool {
    let (items, quantities) = pyramid_bag();
    for i in 0..PYRAMID_BAG_ITEMS_COUNT {
        if items[i] == item || items[i] == ITEM_NONE {
            let sum = u32::from(quantities[i]) + u32::from(count);
            if sum <= u32::from(MAX_BAG_ITEM_CAPACITY) {
                return true;
            }
            count = (sum - u32::from(MAX_BAG_ITEM_CAPACITY)) as u16;
            if count == 0 {
                return true;
            }
        }
    }
    false
}

/// Fills `quantity` (a u8, as the pyramid bag stores it) with up to 99 of
/// `count`; returns what's left over.
fn fill_pyramid_slot(quantity: &mut u8, count: u16) -> u16 {
    // as in C, the u8 sum wraps before it is compared
    *quantity = quantity.wrapping_add(count as u8);
    if u16::from(*quantity) > MAX_BAG_ITEM_CAPACITY {
        let left = u16::from(*quantity) - MAX_BAG_ITEM_CAPACITY;
        *quantity = MAX_BAG_ITEM_CAPACITY as u8;
        left
    } else {
        0
    }
}

pub fn add_pyramid_bag_item(item: u16, mut count: u16) -> bool {
    let (items, quantities) = pyramid_bag();
    let (mut new_items, mut new_quantities) = (*items, *quantities);
    for i in 0..PYRAMID_BAG_ITEMS_COUNT {
        if new_items[i] == item && u16::from(new_quantities[i]) < MAX_BAG_ITEM_CAPACITY {
            count = fill_pyramid_slot(&mut new_quantities[i], count);
            if count == 0 {
                break;
            }
        }
    }
    if count > 0 {
        for i in 0..PYRAMID_BAG_ITEMS_COUNT {
            if new_items[i] == ITEM_NONE {
                new_items[i] = item;
                new_quantities[i] = 0;
                count = fill_pyramid_slot(&mut new_quantities[i], count);
                if count == 0 {
                    break;
                }
            }
        }
    }
    if count != 0 {
        return false;
    }
    let (items, quantities) = pyramid_bag();
    (*items, *quantities) = (new_items, new_quantities);
    true
}

pub fn remove_pyramid_bag_item(item: u16, mut count: u16) -> bool {
    // SAFETY: a plain global of the pyramid bag menu.
    let cursor = unsafe {
        gPyramidBagMenuState
            .cursorPosition
            .wrapping_add(gPyramidBagMenuState.scrollPosition)
    };
    let (items, quantities) = pyramid_bag();
    let i = usize::from(cursor);
    if i < PYRAMID_BAG_ITEMS_COUNT && items[i] == item && u16::from(quantities[i]) >= count {
        quantities[i] = quantities[i].wrapping_sub(count as u8);
        if quantities[i] == 0 {
            items[i] = ITEM_NONE;
        }
        return true;
    }
    let (mut new_items, mut new_quantities) = (*items, *quantities);
    for i in 0..PYRAMID_BAG_ITEMS_COUNT {
        if new_items[i] == item {
            let quantity = u16::from(new_quantities[i]);
            if quantity >= count {
                new_quantities[i] = (quantity - count) as u8;
                count = 0;
                if new_quantities[i] == 0 {
                    new_items[i] = ITEM_NONE;
                }
            } else {
                count -= quantity;
                new_quantities[i] = 0;
                new_items[i] = ITEM_NONE;
            }
            if count == 0 {
                break;
            }
        }
    }
    if count != 0 {
        return false;
    }
    let (items, quantities) = pyramid_bag();
    (*items, *quantities) = (new_items, new_quantities);
    true
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub unsafe fn ApplyNewEncryptionKeyToBagItems(new_key: u32) {
    for index in 0..POCKETS_COUNT {
        // SAFETY: gBagPockets points into the save.
        unsafe {
            let pocket = gBagPockets[index];
            for slot in 0..usize::from(pocket.capacity) {
                ApplyNewEncryptionKeyToHword(
                    &raw mut (*pocket.itemSlots.add(slot)).quantity,
                    new_key,
                );
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe fn ApplyNewEncryptionKeyToBagItems_(new_key: u32) {
    unsafe { ApplyNewEncryptionKeyToBagItems(new_key) };
}

#[unsafe(no_mangle)]
pub fn SetBagItemsPointers() {
    set_bag_items_pointers();
}

#[unsafe(no_mangle)]
pub unsafe fn CopyItemName(item: u16, dst: *mut u8) {
    unsafe { StringCopy(dst, item_info(item).name.as_ptr()) };
}

#[unsafe(no_mangle)]
pub unsafe fn CopyItemNameHandlePlural(item: u16, dst: *mut u8, quantity: u32) {
    unsafe {
        if item == ITEM_POKE_BALL {
            if quantity < 2 {
                StringCopy(dst, item_info(ITEM_POKE_BALL).name.as_ptr());
            } else {
                StringCopy(
                    dst,
                    &raw const (*(&raw const crate::data::strings::gText_PokeBalls).cast::<u8>()),
                );
            }
        } else if (FIRST_BERRY_INDEX..=LAST_BERRY_INDEX).contains(&item) {
            GetBerryCountString(
                dst,
                (&*(&raw const crate::data::berry::gBerries).cast::<CArray<Berry, 0>>())
                    [item - FIRST_BERRY_INDEX]
                    .name
                    .as_ptr(),
                quantity,
            );
        } else {
            StringCopy(dst, item_info(item).name.as_ptr());
        }
    }
}

/// "`berry_name` BERRY" or "... BERRIES".
#[unsafe(no_mangle)]
pub unsafe fn GetBerryCountString(dst: *mut u8, berry_name: *const u8, quantity: u32) {
    let berries: *const u8 = if quantity < 2 {
        &raw const (*(&raw const crate::data::strings::gText_Berry).cast::<u8>())
    } else {
        &raw const (*(&raw const crate::data::strings::gText_Berries).cast::<u8>())
    };
    unsafe {
        let end = StringCopy(dst, berry_name);
        *end = CHAR_SPACE;
        StringCopy(end.add(1), berries);
    }
}

/// (C doesn't check the pocket number: 0 reads before `gBagPockets`.)
#[unsafe(no_mangle)]
pub unsafe fn IsBagPocketNonEmpty(pocket_number: u8) -> u8 {
    let index = usize::from(pocket_number).wrapping_sub(1);
    (!unsafe { bag_pocket(index) }.is_empty()).into()
}

#[unsafe(no_mangle)]
pub fn CheckBagHasItem(item: u16, count: u16) -> u8 {
    bag_has_item(item, count).into()
}

#[unsafe(no_mangle)]
pub fn HasAtLeastOneBerry() -> u8 {
    has_at_least_one_berry().into()
}

#[unsafe(no_mangle)]
pub fn CheckBagHasSpace(item: u16, count: u16) -> u8 {
    bag_has_space(item, count).into()
}

#[unsafe(no_mangle)]
pub fn AddBagItem(item: u16, count: u16) -> u8 {
    add_bag_item(item, count).into()
}

#[unsafe(no_mangle)]
pub fn RemoveBagItem(item: u16, count: u16) -> u8 {
    remove_bag_item(item, count).into()
}

#[unsafe(no_mangle)]
pub fn GetPocketByItemId(item: u16) -> u8 {
    item_pocket(item)
}

/// Empties `count` slots at `slots`.
#[unsafe(no_mangle)]
pub unsafe fn ClearItemSlots(slots: *mut ItemSlot, count: u8) {
    // SAFETY: the caller passes `count` slots; the key is the save's.
    unsafe {
        let slots = core::slice::from_raw_parts_mut(slots, usize::from(count));
        Pocket::new(ITEMS_POCKET, slots, save_block2().encryptionKey).clear();
    }
}

#[unsafe(no_mangle)]
pub fn CountUsedPCItemSlots() -> u8 {
    count_used_pc_item_slots()
}

#[unsafe(no_mangle)]
pub fn CheckPCHasItem(item: u16, count: u16) -> u8 {
    pc_has_item(item, count).into()
}

#[unsafe(no_mangle)]
pub fn AddPCItem(item: u16, count: u16) -> u8 {
    add_pc_item(item, count).into()
}

#[unsafe(no_mangle)]
pub fn RemovePCItem(index: u8, count: u16) {
    remove_pc_item(index, count);
}

#[unsafe(no_mangle)]
pub fn CompactPCItems() {
    compact_pc_items();
}

#[unsafe(no_mangle)]
pub fn SwapRegisteredBike() {
    swap_registered_bike();
}

#[unsafe(no_mangle)]
pub unsafe fn BagGetItemIdByPocketPosition(pocket_number: u8, position: u16) -> u16 {
    unsafe {
        (*gBagPockets[usize::from(pocket_number) - 1]
            .itemSlots
            .add(usize::from(position)))
        .itemId
    }
}

#[unsafe(no_mangle)]
pub unsafe fn BagGetQuantityByPocketPosition(pocket_number: u8, position: u16) -> u16 {
    unsafe { bag_pocket(usize::from(pocket_number) - 1) }.quantity(usize::from(position))
}

/// Compacts the pocket `bag_pocket` points to.
#[unsafe(no_mangle)]
pub unsafe fn CompactItemsInBagPocket(bag_pocket: *mut BagPocket) {
    unsafe { pocket_at(bag_pocket) }.compact();
}

#[unsafe(no_mangle)]
pub unsafe fn SortBerriesOrTMHMs(bag_pocket: *mut BagPocket) {
    unsafe { pocket_at(bag_pocket) }.sort_by_item_id();
}

/// A pocket from a `BagPocket` pointer (always one of `gBagPockets`).
unsafe fn pocket_at<'a>(bag_pocket: *mut BagPocket) -> Pocket<'a> {
    unsafe {
        let pocket = *bag_pocket;
        let slots = core::slice::from_raw_parts_mut(pocket.itemSlots, usize::from(pocket.capacity));
        Pocket::new(ITEMS_POCKET, slots, save_block2().encryptionKey)
    }
}

#[unsafe(no_mangle)]
pub unsafe fn MoveItemSlotInList(slots: *mut ItemSlot, from: u32, to: u32) {
    let len = from.max(to) as usize + 1;
    // SAFETY: both positions are within the list the caller passes.
    move_item_slot(
        unsafe { core::slice::from_raw_parts_mut(slots, len) },
        from as usize,
        to as usize,
    );
}

#[unsafe(no_mangle)]
pub fn ClearBag() {
    clear_bag();
}

/// (C doesn't check the item has a pocket: then it reads before `gBagPockets`.)
#[unsafe(no_mangle)]
pub unsafe fn CountTotalItemQuantityInBag(item: u16) -> u16 {
    let index = usize::from(item_pocket(item)).wrapping_sub(1);
    unsafe { bag_pocket(index) }.count(item)
}

#[unsafe(no_mangle)]
pub fn AddPyramidBagItem(item: u16, count: u16) -> u8 {
    add_pyramid_bag_item(item, count).into()
}

#[unsafe(no_mangle)]
pub fn RemovePyramidBagItem(item: u16, count: u16) -> u8 {
    remove_pyramid_bag_item(item, count).into()
}

#[unsafe(no_mangle)]
pub fn GetItemName(item: u16) -> *const u8 {
    item_info(item).name.as_ptr()
}

#[unsafe(no_mangle)]
pub fn GetItemId(item: u16) -> u16 {
    item_info(item).itemId
}

#[unsafe(no_mangle)]
pub fn GetItemPrice(item: u16) -> u16 {
    item_info(item).price
}

#[unsafe(no_mangle)]
pub fn GetItemHoldEffect(item: u16) -> u8 {
    item_info(item).holdEffect
}

#[unsafe(no_mangle)]
pub fn GetItemHoldEffectParam(item: u16) -> u8 {
    item_info(item).holdEffectParam
}

#[unsafe(no_mangle)]
pub fn GetItemDescription(item: u16) -> *const u8 {
    item_info(item).description
}

#[unsafe(no_mangle)]
pub fn GetItemImportance(item: u16) -> u8 {
    item_info(item).importance
}

#[unsafe(no_mangle)]
pub fn GetItemRegistrability(item: u16) -> u8 {
    item_info(item).registrability
}

#[unsafe(no_mangle)]
pub fn GetItemPocket(item: u16) -> u8 {
    item_pocket(item)
}

#[unsafe(no_mangle)]
pub fn GetItemType(item: u16) -> u8 {
    item_info(item).r#type
}

#[unsafe(no_mangle)]
pub fn GetItemFieldFunc(item: u16) -> Option<unsafe fn(u8)> {
    item_info(item).fieldUseFunc
}

#[unsafe(no_mangle)]
pub fn GetItemBattleUsage(item: u16) -> u8 {
    item_info(item).battleUsage
}

#[unsafe(no_mangle)]
pub fn GetItemBattleFunc(item: u16) -> Option<unsafe fn(u8)> {
    item_info(item).battleUseFunc
}

#[unsafe(no_mangle)]
pub fn GetItemSecondaryId(item: u16) -> u8 {
    item_info(item).secondaryId
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slots<const N: usize>(items: [(u16, u16); N], key: u16) -> [ItemSlot; N] {
        items.map(|(itemId, quantity)| ItemSlot {
            itemId,
            quantity: quantity ^ key,
        })
    }

    #[test]
    fn adding_fills_existing_slots_then_empty_ones() {
        let mut s = slots([(5, 98), (0, 0), (0, 0)], 0x1234);
        let mut pocket = Pocket::new(ITEMS_POCKET, &mut s, 0x1234);
        assert!(pocket.add(5, 3));
        assert_eq!((pocket.slots()[0].itemId, pocket.quantity(0)), (5, 99));
        assert_eq!((pocket.slots()[1].itemId, pocket.quantity(1)), (5, 2));
    }

    #[test]
    fn a_failed_add_changes_nothing() {
        let mut s = slots([(5, 99), (7, 1)], 0);
        let mut pocket = Pocket::new(ITEMS_POCKET, &mut s, 0);
        assert!(!pocket.has_space(5, 1));
        assert!(!pocket.add(5, 1));
        assert_eq!(pocket.quantity(0), 99);
        assert_eq!(pocket.slots()[1].itemId, 7);
    }

    #[test]
    fn tms_never_take_a_second_slot() {
        let mut s = slots([(300, 99), (0, 0)], 0);
        let mut pocket = Pocket::new(TMHM_POCKET, &mut s, 0);
        assert!(!pocket.add(300, 1));
        assert_eq!(pocket.slots()[1].itemId, ITEM_NONE);
    }

    #[test]
    fn removing_starts_at_the_cursor_and_frees_empty_slots() {
        let mut s = slots([(5, 2), (5, 3)], 0x55);
        let mut pocket = Pocket::new(ITEMS_POCKET, &mut s, 0x55);
        assert!(!pocket.remove(5, 6, 1));
        assert!(pocket.remove(5, 4, 1));
        assert_eq!(pocket.slots()[1].itemId, ITEM_NONE);
        assert_eq!(pocket.quantity(0), 1);
        assert_eq!(pocket.count(5), 1);
    }

    #[test]
    fn sorting_puts_empty_slots_last() {
        let mut s = slots([(0, 0), (140, 1), (133, 5)], 0);
        let mut pocket = Pocket::new(BERRIES_POCKET, &mut s, 0);
        pocket.sort_by_item_id();
        let ids: [u16; 3] = core::array::from_fn(|i| pocket.slots()[i].itemId);
        assert_eq!(ids, [133, 140, 0]);
    }

    #[test]
    fn moving_a_slot_matches_the_bag_menu() {
        let mut s = slots([(1, 1), (2, 1), (3, 1), (4, 1)], 0);
        move_item_slot(&mut s, 0, 3);
        assert_eq!(s.map(|x| x.itemId), [2, 3, 1, 4]);
        move_item_slot(&mut s, 2, 0);
        assert_eq!(s.map(|x| x.itemId), [1, 2, 3, 4]);
    }

    #[test]
    fn pyramid_slots_hold_up_to_99() {
        let mut q = 90u8;
        assert_eq!(fill_pyramid_slot(&mut q, 5), 0);
        assert_eq!(q, 95);
        assert_eq!(fill_pyramid_slot(&mut q, 10), 6);
        assert_eq!(q, 99);
    }
}
