//! The game's heap: a doubly linked list of blocks carved out of `gHeap`.
//!
//! Block headers live inside the managed memory and store real GBA pointers,
//! so every field is reached through byte offsets rather than a Rust struct.
//! A `repr(C)` mirror would be 32 bytes on a 64-bit host instead of 16.

use crate::ffi::CpuSet;

pub const HEAP_SIZE: usize = 0x1c000;

/// `sizeof(struct MemBlock)`
const MEM_BLOCK_SIZE: usize = 16;
/// `bool16 flag` - whether the block is allocated.
const BLOCK_FLAG: usize = 0;
/// `u16 magic` - must equal `MALLOC_SYSTEM_ID`.
const BLOCK_MAGIC: usize = 2;
/// `u32 size` - payload bytes, not counting the header.
const BLOCK_SIZE: usize = 4;
const BLOCK_PREV: usize = 8;
const BLOCK_NEXT: usize = 12;
/// `u8 data[0]` - the payload starts right after the header.
const BLOCK_DATA: usize = 16;

const MALLOC_SYSTEM_ID: u16 = 0xa3a3;

const CPU_SET_SRC_FIXED: u32 = 0x0100_0000;
const CPU_SET_32BIT: u32 = 0x0400_0000;

#[repr(C, align(4))]
pub struct Heap(pub [u8; HEAP_SIZE]);

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gHeap: Heap = Heap([0; HEAP_SIZE]);

static mut HEAP_START: *mut u8 = core::ptr::null_mut();
static mut HEAP_BYTES: u32 = 0;

#[inline]
unsafe fn flag(block: *mut u8) -> u16 {
    unsafe { block.add(BLOCK_FLAG).cast::<u16>().read() }
}

#[inline]
unsafe fn set_flag(block: *mut u8, value: u16) {
    unsafe { block.add(BLOCK_FLAG).cast::<u16>().write(value) };
}

#[inline]
unsafe fn magic(block: *mut u8) -> u16 {
    unsafe { block.add(BLOCK_MAGIC).cast::<u16>().read() }
}

#[inline]
unsafe fn set_magic(block: *mut u8, value: u16) {
    unsafe { block.add(BLOCK_MAGIC).cast::<u16>().write(value) };
}

#[inline]
unsafe fn size(block: *mut u8) -> u32 {
    unsafe { block.add(BLOCK_SIZE).cast::<u32>().read() }
}

#[inline]
unsafe fn set_size(block: *mut u8, value: u32) {
    unsafe { block.add(BLOCK_SIZE).cast::<u32>().write(value) };
}

#[inline]
unsafe fn prev(block: *mut u8) -> *mut u8 {
    unsafe { block.add(BLOCK_PREV).cast::<*mut u8>().read() }
}

#[inline]
unsafe fn set_prev(block: *mut u8, value: *mut u8) {
    unsafe { block.add(BLOCK_PREV).cast::<*mut u8>().write(value) };
}

#[inline]
unsafe fn next(block: *mut u8) -> *mut u8 {
    unsafe { block.add(BLOCK_NEXT).cast::<*mut u8>().read() }
}

#[inline]
unsafe fn set_next(block: *mut u8, value: *mut u8) {
    unsafe { block.add(BLOCK_NEXT).cast::<*mut u8>().write(value) };
}

#[inline]
unsafe fn data(block: *mut u8) -> *mut u8 {
    unsafe { block.add(BLOCK_DATA) }
}

/// Rounds an allocation up to the next multiple of four.
#[inline]
const fn aligned(size: u32) -> u32 {
    if size & 3 != 0 {
        4 * ((size / 4) + 1)
    } else {
        size
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PutMemBlockHeader(
    block: *mut u8,
    previous: *mut u8,
    following: *mut u8,
    block_size: u32,
) {
    unsafe { set_flag(block, 0) };
    unsafe { set_magic(block, MALLOC_SYSTEM_ID) };
    unsafe { set_size(block, block_size) };
    unsafe { set_prev(block, previous) };
    unsafe { set_next(block, following) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PutFirstMemBlockHeader(block: *mut u8, block_size: u32) {
    unsafe { PutMemBlockHeader(block, block, block, block_size - MEM_BLOCK_SIZE as u32) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AllocInternal(heap_start: *mut u8, requested: u32) -> *mut u8 {
    let head = heap_start;
    let mut pos = heap_start;
    let requested = aligned(requested);

    loop {
        if unsafe { flag(pos) } == 0 {
            let found = unsafe { size(pos) };

            if found >= requested {
                if found - requested < 2 * MEM_BLOCK_SIZE as u32 {
                    // Close enough to the request: hand over the whole block.
                    unsafe { set_flag(pos, 1) };
                } else {
                    // Much bigger than the request: split off the remainder.
                    let remainder = found - MEM_BLOCK_SIZE as u32 - requested;
                    let split_block = unsafe { data(pos).add(requested as usize) };

                    unsafe { set_flag(pos, 1) };
                    unsafe { set_size(pos, requested) };
                    unsafe { PutMemBlockHeader(split_block, pos, next(pos), remainder) };
                    unsafe { set_next(pos, split_block) };

                    if unsafe { next(split_block) } != head {
                        unsafe { set_prev(next(split_block), split_block) };
                    }
                }

                return unsafe { data(pos) };
            }
        }

        if unsafe { next(pos) } == head {
            return core::ptr::null_mut();
        }
        pos = unsafe { next(pos) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeInternal(heap_start: *mut u8, pointer: *mut u8) {
    if pointer.is_null() {
        return;
    }

    let head = heap_start;
    let block = unsafe { pointer.sub(MEM_BLOCK_SIZE) };
    unsafe { set_flag(block, 0) };

    // Merge forwards when the following block is free.
    if unsafe { next(block) } != head && unsafe { flag(next(block)) } == 0 {
        let following = unsafe { next(block) };
        unsafe { set_size(block, size(block) + MEM_BLOCK_SIZE as u32 + size(following)) };
        unsafe { set_magic(following, 0) };
        unsafe { set_next(block, next(following)) };
        if unsafe { next(block) } != head {
            unsafe { set_prev(next(block), block) };
        }
    }

    // Merge backwards when the preceding block is free.
    if block != head && unsafe { flag(prev(block)) } == 0 {
        let previous = unsafe { prev(block) };
        unsafe { set_next(previous, next(block)) };
        if unsafe { next(block) } != head {
            unsafe { set_prev(next(block), previous) };
        }
        unsafe { set_magic(block, 0) };
        unsafe {
            set_size(
                previous,
                size(previous) + MEM_BLOCK_SIZE as u32 + size(block),
            )
        };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AllocZeroedInternal(heap_start: *mut u8, requested: u32) -> *mut u8 {
    let memory = unsafe { AllocInternal(heap_start, requested) };
    if memory.is_null() {
        return memory;
    }

    // `CpuFill32(0, mem, size)` fills from a fixed zero word.
    let zero = 0u32;
    let words = aligned(requested) / 4 & 0x1f_ffff;
    unsafe {
        CpuSet(
            (&raw const zero).cast(),
            memory.cast(),
            CPU_SET_32BIT | CPU_SET_SRC_FIXED | words,
        )
    };

    memory
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckMemBlockInternal(heap_start: *mut u8, pointer: *mut u8) -> u32 {
    let head = heap_start;
    let block = unsafe { pointer.sub(MEM_BLOCK_SIZE) };

    if unsafe { magic(block) } != MALLOC_SYSTEM_ID {
        return 0;
    }
    if unsafe { magic(next(block)) } != MALLOC_SYSTEM_ID {
        return 0;
    }
    if unsafe { next(block) } != head && unsafe { prev(next(block)) } != block {
        return 0;
    }
    if unsafe { magic(prev(block)) } != MALLOC_SYSTEM_ID {
        return 0;
    }
    if unsafe { prev(block) } != head && unsafe { next(prev(block)) } != block {
        return 0;
    }
    if unsafe { next(block) } != head
        && unsafe { next(block) } != unsafe { data(block).add(size(block) as usize) }
    {
        return 0;
    }

    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitHeap(heap_start: *mut u8, heap_size: u32) {
    unsafe { (&raw mut HEAP_START).write(heap_start) };
    unsafe { (&raw mut HEAP_BYTES).write(heap_size) };
    unsafe { PutFirstMemBlockHeader(heap_start, heap_size) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Alloc(requested: u32) -> *mut u8 {
    unsafe { AllocInternal((&raw const HEAP_START).read(), requested) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AllocZeroed(requested: u32) -> *mut u8 {
    unsafe { AllocZeroedInternal((&raw const HEAP_START).read(), requested) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Free(pointer: *mut u8) {
    unsafe { FreeInternal((&raw const HEAP_START).read(), pointer) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckMemBlock(pointer: *mut u8) -> u32 {
    unsafe { CheckMemBlockInternal((&raw const HEAP_START).read(), pointer) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckHeap() -> u32 {
    let head = unsafe { (&raw const HEAP_START).read() };
    let mut pos = head;

    loop {
        if unsafe { CheckMemBlockInternal(head, data(pos)) } == 0 {
            return 0;
        }
        pos = unsafe { next(pos) };
        if pos == head {
            return 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_offsets_match_the_arm_structure() {
        assert_eq!(BLOCK_FLAG, 0);
        assert_eq!(BLOCK_MAGIC, 2);
        assert_eq!(BLOCK_SIZE, 4);
        assert_eq!(BLOCK_PREV, 8);
        assert_eq!(BLOCK_NEXT, 12);
        assert_eq!(BLOCK_DATA, MEM_BLOCK_SIZE);
        assert_eq!(MEM_BLOCK_SIZE, 16);
    }

    #[test]
    fn allocations_round_up_to_a_word() {
        assert_eq!(aligned(0), 0);
        assert_eq!(aligned(1), 4);
        assert_eq!(aligned(3), 4);
        assert_eq!(aligned(4), 4);
        assert_eq!(aligned(5), 8);
        assert_eq!(aligned(100), 100);
    }

    #[test]
    fn a_split_only_happens_when_two_headers_still_fit() {
        // The remainder must leave room for its own header plus something
        // worth allocating, hence the 2 * sizeof(MemBlock) threshold.
        let splits = |found: u32, want: u32| found - want >= 2 * MEM_BLOCK_SIZE as u32;
        assert!(!splits(40, 32));
        assert!(splits(64, 32));
    }

    #[test]
    fn the_heap_is_a_whole_number_of_blocks() {
        assert_eq!(HEAP_SIZE, 0x1c000);
        assert_eq!(HEAP_SIZE % MEM_BLOCK_SIZE, 0);
    }
}
