//! Save block storage and the save-block shuffle.
//!
//! The three save blocks live in oversized EWRAM buffers and the pointers to
//! them are moved to a random offset inside those buffers on every map load.
//! This is Game Freak's anti-cheat "ASLR": memory editors cannot rely on a
//! fixed address.

use crate::berry_powder::ApplyNewEncryptionKeyToBerryPowder;
use crate::decoration_inventory::SetDecorationInventoriesPointers;
use crate::ffi::{CpuSet, OBJECT_EVENT_SIZE, PARTY_SIZE, POKEMON_SIZE};
use crate::malloc::{HEAP_SIZE, InitHeap, gHeap};
use crate::pokemon::gPlayerPartyCount;
use crate::trainer_hill::gTrainerHillVBlankCounter;

/// The pointers are shifted by up to this many bytes, in word steps.
const SAVEBLOCK_MOVE_RANGE: u16 = 128;

const SAVEBLOCK2_SIZE: usize = 0xf2c;
const SAVEBLOCK1_SIZE: usize = 0x3d88;
const POKEMON_STORAGE_SIZE: usize = 0x83d0;
const SAVEBLOCK2_ASLR_SIZE: usize = 0xfac;
const SAVEBLOCK1_ASLR_SIZE: usize = 0x3e08;
const POKEMON_STORAGE_ASLR_SIZE: usize = 0x8450;

const OBJECT_EVENTS_COUNT: usize = 16;
const ITEM_SLOT_SIZE: usize = 4;
const MAIL_SIZE: usize = 36;
const MAIL_COUNT: usize = 16;

/// `offsetof(struct SaveBlock2, ...)`
const SAVE2_SPECIAL_SAVE_WARP_FLAGS: usize = 0x09;
const SAVE2_TRAINER_ID: usize = 0x0a;
const SAVE2_ENCRYPTION_KEY: usize = 0xac;

/// `offsetof(struct SaveBlock1, ...)`
const SAVE1_MONEY: usize = 0x490;
const SAVE1_COINS: usize = 0x494;
const SAVE1_PLAYER_PARTY_COUNT: usize = 0x234;
const SAVE1_PLAYER_PARTY: usize = 0x238;
const SAVE1_OBJECT_EVENTS: usize = 0xa30;
const SAVE1_MAIL: usize = 0x2be0;

/// Each bag pocket: where it lives in SaveBlock1, where it lives in
/// `gLoadedSaveData`, and how many slots it has.
const POCKETS: [(usize, usize, usize); 5] = [
    (0x560, 0x000, 30), // items
    (0x5d8, 0x078, 30), // key items
    (0x650, 0x0f0, 16), // Poke Balls
    (0x690, 0x130, 64), // TMs and HMs
    (0x790, 0x230, 46), // berries
];
const LOADED_MAIL: usize = 0x2e8;
const LOADED_SAVE_DATA_SIZE: usize = 0x528;

/// `offsetof(struct Main, vblankCallback)` and `hblankCallback`.
const MAIN_VBLANK_CALLBACK: usize = 0x0c;
const MAIN_HBLANK_CALLBACK: usize = 0x10;

const CONTINUE_GAME_WARP: u8 = 1 << 0;

const CPU_SET_SRC_FIXED: u32 = 0x0100_0000;

#[repr(C, align(4))]
pub struct Blob<const N: usize>(pub [u8; N]);

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSaveblock2: Blob<SAVEBLOCK2_ASLR_SIZE> = Blob([0; SAVEBLOCK2_ASLR_SIZE]);

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSaveblock1: Blob<SAVEBLOCK1_ASLR_SIZE> = Blob([0; SAVEBLOCK1_ASLR_SIZE]);

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPokemonStorage: Blob<POKEMON_STORAGE_ASLR_SIZE> =
    Blob([0; POKEMON_STORAGE_ASLR_SIZE]);

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLoadedSaveData: Blob<LOADED_SAVE_DATA_SIZE> = Blob([0; LOADED_SAVE_DATA_SIZE]);

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static gLastEncryptionKey: crate::global::Global<u32> = crate::global::Global::new(0);

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFlashMemoryPresent: u32 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSaveBlock1Ptr: *mut crate::types::SaveBlock1 = core::ptr::null_mut();

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSaveBlock2Ptr: *mut crate::types::SaveBlock2 = core::ptr::null_mut();

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gPokemonStoragePtr: *mut u8 = core::ptr::null_mut();

/// `IdentifyFlash` with this module's view of its types.
#[inline]
unsafe fn IdentifyFlash() -> u16 {
    unsafe { crate::agb_flash_1m::IdentifyFlash() }
}
/// `InitFlashTimer` with this module's view of its types.
#[inline]
unsafe fn InitFlashTimer() {
    unsafe {
        crate::agb_main::InitFlashTimer();
    }
}
/// `Random` with this module's view of its types.
#[inline]
unsafe fn Random() -> u16 {
    crate::random::Random()
}
/// `SetBagItemsPointers` with this module's view of its types.
#[inline]
unsafe fn SetBagItemsPointers() {
    {
        crate::item::SetBagItemsPointers();
    }
}
/// `SetContinueGameWarpToDynamicWarp` with this module's view of its types.
#[inline]
unsafe fn SetContinueGameWarpToDynamicWarp(a0: i32) {
    unsafe {
        crate::overworld::SetContinueGameWarpToDynamicWarp(a0);
    }
}
/// `ApplyNewEncryptionKeyToGameStats` with this module's view of its types.
#[inline]
unsafe fn ApplyNewEncryptionKeyToGameStats(a0: u32) {
    unsafe {
        crate::overworld::ApplyNewEncryptionKeyToGameStats(a0);
    }
}
/// `ApplyNewEncryptionKeyToBagItems` with this module's view of its types.
#[inline]
unsafe fn ApplyNewEncryptionKeyToBagItems(a0: u32) {
    unsafe {
        crate::item::ApplyNewEncryptionKeyToBagItems(a0);
    }
}
/// `ApplyNewEncryptionKeyToBagItems_` with this module's view of its types.
#[inline]
unsafe fn ApplyNewEncryptionKeyToBagItems_(a0: u32) {
    unsafe {
        crate::item::ApplyNewEncryptionKeyToBagItems_(a0);
    }
}

#[inline]
unsafe fn save1() -> *mut u8 {
    unsafe { (&raw const gSaveBlock1Ptr).read_volatile().cast() }
}

#[inline]
unsafe fn save2() -> *mut u8 {
    unsafe { (&raw const gSaveBlock2Ptr).read_volatile().cast() }
}

#[inline]
unsafe fn encryption_key_ptr() -> *mut u32 {
    unsafe { save2().add(SAVE2_ENCRYPTION_KEY).cast::<u32>() }
}

/// `CpuFill16(0, dest, size)`
#[inline]
unsafe fn cpu_fill16_zero(dest: *mut u8, size: usize) {
    let zero = 0u16;
    unsafe {
        CpuSet(
            (&raw const zero).cast(),
            dest.cast(),
            CPU_SET_SRC_FIXED | (size as u32 / 2 & 0x1f_ffff),
        )
    };
}

#[inline]
unsafe fn copy(src: *const u8, dest: *mut u8, size: usize) {
    unsafe { core::ptr::copy_nonoverlapping(src, dest, size) };
}

#[unsafe(no_mangle)]
pub unsafe fn CheckForFlashMemory() {
    // IdentifyFlash returns 0 on success.
    if unsafe { IdentifyFlash() } == 0 {
        unsafe { (&raw mut gFlashMemoryPresent).write_volatile(1) };
        unsafe { InitFlashTimer() };
    } else {
        unsafe { (&raw mut gFlashMemoryPresent).write_volatile(0) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn ClearSav2() {
    unsafe { cpu_fill16_zero((&raw mut gSaveblock2).cast(), SAVEBLOCK2_ASLR_SIZE) };
}

#[unsafe(no_mangle)]
pub unsafe fn ClearSav1() {
    unsafe { cpu_fill16_zero((&raw mut gSaveblock1).cast(), SAVEBLOCK1_ASLR_SIZE) };
}

/// `offset` is the sum of the trainer id bytes.
#[unsafe(no_mangle)]
pub unsafe fn SetSaveBlocksPointers(offset: u16) {
    let offset = offset.wrapping_add(unsafe { Random() }) & (SAVEBLOCK_MOVE_RANGE - 4);
    let offset = offset as usize;

    unsafe {
        (&raw mut gSaveBlock2Ptr)
            .write_volatile((&raw mut gSaveblock2).cast::<u8>().add(offset).cast())
    };
    unsafe {
        (&raw mut gSaveBlock1Ptr)
            .write_volatile((&raw mut gSaveblock1).cast::<u8>().add(offset).cast())
    };
    unsafe {
        (&raw mut gPokemonStoragePtr)
            .write_volatile((&raw mut gPokemonStorage).cast::<u8>().add(offset))
    };

    unsafe { SetBagItemsPointers() };
    SetDecorationInventoriesPointers();
}

#[unsafe(no_mangle)]
pub unsafe fn MoveSaveBlocks_ResetHeap() {
    let main =
        (&raw mut (*(&raw const crate::agb_main::gMain).cast::<u8>().cast_mut())).cast::<u8>();
    let vblank_slot = unsafe { main.add(MAIN_VBLANK_CALLBACK).cast::<*mut u8>() };
    let hblank_slot = unsafe { main.add(MAIN_HBLANK_CALLBACK).cast::<*mut u8>() };

    // Interrupt callbacks are parked while the blocks move under them.
    let vblank = unsafe { vblank_slot.read_volatile() };
    let hblank = unsafe { hblank_slot.read_volatile() };
    unsafe { vblank_slot.write_volatile(core::ptr::null_mut()) };
    unsafe { hblank_slot.write_volatile(core::ptr::null_mut()) };
    unsafe { (&raw mut gTrainerHillVBlankCounter).write_volatile(core::ptr::null_mut()) };

    // The heap doubles as scratch space for the copies.
    let heap = (&raw mut gHeap).cast::<u8>();
    let save2_copy = heap;
    let save1_copy = unsafe { heap.add(SAVEBLOCK2_SIZE) };
    let storage_copy = unsafe { heap.add(SAVEBLOCK2_SIZE + SAVEBLOCK1_SIZE) };

    unsafe { copy(save2(), save2_copy, SAVEBLOCK2_SIZE) };
    unsafe { copy(save1(), save1_copy, SAVEBLOCK1_SIZE) };
    unsafe {
        copy(
            (&raw const gPokemonStoragePtr).read_volatile(),
            storage_copy,
            POKEMON_STORAGE_SIZE,
        )
    };

    let mut trainer_id_sum = 0u16;
    for i in 0..4 {
        trainer_id_sum += u16::from(unsafe { save2_copy.add(SAVE2_TRAINER_ID + i).read() });
    }
    unsafe { SetSaveBlocksPointers(trainer_id_sum) };

    unsafe { copy(save2_copy, save2(), SAVEBLOCK2_SIZE) };
    unsafe { copy(save1_copy, save1(), SAVEBLOCK1_SIZE) };
    unsafe {
        copy(
            storage_copy,
            (&raw const gPokemonStoragePtr).read_volatile(),
            POKEMON_STORAGE_SIZE,
        )
    };

    // The copies trampled the heap, so start it over.
    unsafe { InitHeap(heap, HEAP_SIZE as u32) };

    unsafe { hblank_slot.write_volatile(hblank) };
    unsafe { vblank_slot.write_volatile(vblank) };

    let high = u32::from(unsafe { Random() }) << 16;
    let key = high.wrapping_add(u32::from(unsafe { Random() }));
    unsafe { apply_new_encryption_key_to_all_encrypted_data(key) };
    unsafe { encryption_key_ptr().write_volatile(key) };
}

#[inline]
unsafe fn warp_flags() -> *mut u8 {
    unsafe { save2().add(SAVE2_SPECIAL_SAVE_WARP_FLAGS) }
}

#[unsafe(no_mangle)]
pub unsafe fn UseContinueGameWarp() -> u32 {
    u32::from(unsafe { warp_flags().read_volatile() } & CONTINUE_GAME_WARP)
}

#[unsafe(no_mangle)]
pub unsafe fn ClearContinueGameWarpStatus() {
    let flags = unsafe { warp_flags() };
    unsafe { flags.write_volatile(flags.read_volatile() & !CONTINUE_GAME_WARP) };
}

#[unsafe(no_mangle)]
pub unsafe fn SetContinueGameWarpStatus() {
    let flags = unsafe { warp_flags() };
    unsafe { flags.write_volatile(flags.read_volatile() | CONTINUE_GAME_WARP) };
}

#[unsafe(no_mangle)]
pub unsafe fn SetContinueGameWarpStatusToDynamicWarp() {
    unsafe { SetContinueGameWarpToDynamicWarp(0) };
    unsafe { SetContinueGameWarpStatus() };
}

#[unsafe(no_mangle)]
pub unsafe fn ClearContinueGameWarpStatus2() {
    unsafe { ClearContinueGameWarpStatus() };
}

#[unsafe(no_mangle)]
pub unsafe fn SavePlayerParty() {
    let count = unsafe { (&raw const gPlayerPartyCount).read_volatile() };
    unsafe { save1().add(SAVE1_PLAYER_PARTY_COUNT).write_volatile(count) };
    unsafe {
        copy(
            (&raw const (*(&raw const crate::pokemon::gPlayerParty)
                .cast::<u8>()
                .cast_mut()))
                .cast(),
            save1().add(SAVE1_PLAYER_PARTY),
            POKEMON_SIZE * PARTY_SIZE,
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe fn LoadPlayerParty() {
    let count = unsafe { save1().add(SAVE1_PLAYER_PARTY_COUNT).read_volatile() };
    unsafe { (&raw mut gPlayerPartyCount).write_volatile(count) };
    unsafe {
        copy(
            save1().add(SAVE1_PLAYER_PARTY),
            (&raw mut (*(&raw const crate::pokemon::gPlayerParty)
                .cast::<u8>()
                .cast_mut()))
                .cast(),
            POKEMON_SIZE * PARTY_SIZE,
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe fn SaveObjectEvents() {
    unsafe {
        copy(
            (&raw const (*(&raw const crate::field_player_avatar::gObjectEvents)
                .cast::<u8>()
                .cast_mut()))
                .cast(),
            save1().add(SAVE1_OBJECT_EVENTS),
            OBJECT_EVENT_SIZE * OBJECT_EVENTS_COUNT,
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe fn LoadObjectEvents() {
    unsafe {
        copy(
            save1().add(SAVE1_OBJECT_EVENTS),
            (&raw mut (*(&raw const crate::field_player_avatar::gObjectEvents)
                .cast::<u8>()
                .cast_mut()))
                .cast(),
            OBJECT_EVENT_SIZE * OBJECT_EVENTS_COUNT,
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe fn CopyPartyAndObjectsToSave() {
    unsafe { SavePlayerParty() };
    unsafe { SaveObjectEvents() };
}

#[unsafe(no_mangle)]
pub unsafe fn CopyPartyAndObjectsFromSave() {
    unsafe { LoadPlayerParty() };
    unsafe { LoadObjectEvents() };
}

#[unsafe(no_mangle)]
pub unsafe fn LoadPlayerBag() {
    let loaded = (&raw mut gLoadedSaveData).cast::<u8>();
    for (save_offset, loaded_offset, count) in POCKETS {
        unsafe {
            copy(
                save1().add(save_offset),
                loaded.add(loaded_offset),
                count * ITEM_SLOT_SIZE,
            )
        };
    }
    unsafe {
        copy(
            save1().add(SAVE1_MAIL),
            loaded.add(LOADED_MAIL),
            MAIL_SIZE * MAIL_COUNT,
        )
    };

    let key = unsafe { encryption_key_ptr().read_volatile() };
    unsafe { (gLastEncryptionKey.as_ptr()).write_volatile(key) };
}

#[unsafe(no_mangle)]
pub unsafe fn SavePlayerBag() {
    let loaded = (&raw const gLoadedSaveData).cast::<u8>();
    for (save_offset, loaded_offset, count) in POCKETS {
        unsafe {
            copy(
                loaded.add(loaded_offset),
                save1().add(save_offset),
                count * ITEM_SLOT_SIZE,
            )
        };
    }
    unsafe {
        copy(
            loaded.add(LOADED_MAIL),
            save1().add(SAVE1_MAIL),
            MAIL_SIZE * MAIL_COUNT,
        )
    };

    // The loaded bag is still encrypted with the key from when it was
    // loaded, so re-key it from that old key to the current one.
    let current = unsafe { encryption_key_ptr().read_volatile() };
    unsafe {
        encryption_key_ptr()
            .write_volatile((gLastEncryptionKey.as_ptr().cast_const()).read_volatile())
    };
    unsafe { ApplyNewEncryptionKeyToBagItems(current) };
    unsafe { encryption_key_ptr().write_volatile(current) };
}

#[unsafe(no_mangle)]
pub unsafe fn ApplyNewEncryptionKeyToHword(hword: *mut u16, new_key: u32) {
    let key = unsafe { encryption_key_ptr().read_volatile() };
    let value = unsafe { hword.read() } ^ key as u16 ^ new_key as u16;
    unsafe { hword.write(value) };
}

#[unsafe(no_mangle)]
pub unsafe fn ApplyNewEncryptionKeyToWord(word: *mut u32, new_key: u32) {
    let key = unsafe { encryption_key_ptr().read_volatile() };
    let value = unsafe { word.read() } ^ key ^ new_key;
    unsafe { word.write(value) };
}

unsafe fn apply_new_encryption_key_to_all_encrypted_data(key: u32) {
    unsafe { ApplyNewEncryptionKeyToGameStats(key) };
    unsafe { ApplyNewEncryptionKeyToBagItems_(key) };
    unsafe { ApplyNewEncryptionKeyToBerryPowder(key) };
    unsafe { ApplyNewEncryptionKeyToWord(save1().add(SAVE1_MONEY).cast(), key) };
    unsafe { ApplyNewEncryptionKeyToHword(save1().add(SAVE1_COINS).cast(), key) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_aslr_buffer_has_room_for_the_largest_shift() {
        let max_shift = (SAVEBLOCK_MOVE_RANGE - 4) as usize;
        assert!(SAVEBLOCK2_SIZE + max_shift <= SAVEBLOCK2_ASLR_SIZE);
        assert!(SAVEBLOCK1_SIZE + max_shift <= SAVEBLOCK1_ASLR_SIZE);
        assert!(POKEMON_STORAGE_SIZE + max_shift <= POKEMON_STORAGE_ASLR_SIZE);
    }

    #[test]
    fn shifts_are_word_aligned() {
        for raw in [0u16, 1, 2, 3, 127, 255, 1000] {
            let offset = raw & (SAVEBLOCK_MOVE_RANGE - 4);
            assert_eq!(offset % 4, 0);
            assert!(offset < SAVEBLOCK_MOVE_RANGE);
        }
    }

    #[test]
    fn the_heap_can_hold_all_three_blocks_during_the_move() {
        assert!(SAVEBLOCK2_SIZE + SAVEBLOCK1_SIZE + POKEMON_STORAGE_SIZE <= HEAP_SIZE);
    }

    #[test]
    fn loaded_save_data_pockets_are_packed_back_to_back() {
        let mut expected = 0;
        for (_, loaded, count) in POCKETS {
            assert_eq!(loaded, expected);
            expected += count * ITEM_SLOT_SIZE;
        }
        assert_eq!(expected, LOADED_MAIL);
        assert_eq!(LOADED_MAIL + MAIL_SIZE * MAIL_COUNT, LOADED_SAVE_DATA_SIZE);
    }
}
