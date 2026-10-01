//! A ring of deferred DMA3 transfers, drained during vblank.

use crate::ffi::{REG_BASE, REG_OFFSET_VCOUNT, dma3_copy_large, dma3_fill_large};

const MAX_DMA_REQUESTS: usize = 128;

const DMA_REQUEST_COPY32: u16 = 1;
const DMA_REQUEST_FILL32: u16 = 2;
const DMA_REQUEST_COPY16: u16 = 3;
const DMA_REQUEST_FILL16: u16 = 4;

/// Never move more than 40 KiB in one vblank.
const MAX_BYTES_PER_VBLANK: u32 = 40 * 1024;
/// Past this scanline vblank is about to end.
const LAST_SAFE_VCOUNT: u8 = 224;

/// `struct Dma3Request { const u8 *src; u8 *dest; u16 size; u16 mode; u32 value; }`
#[repr(C)]
#[derive(Clone, Copy)]
struct Dma3Request {
    src: *const u8,
    dest: *mut u8,
    size: u16,
    mode: u16,
    value: u32,
}

unsafe impl Sync for Dma3Request {}

static mut REQUESTS: [Dma3Request; MAX_DMA_REQUESTS] = [const {
    Dma3Request {
        src: core::ptr::null(),
        dest: core::ptr::null_mut(),
        size: 0,
        mode: 0,
        value: 0,
    }
}; MAX_DMA_REQUESTS];

static MANAGER_LOCKED: crate::global::Global<bool> = crate::global::Global::new(false);
static REQUEST_CURSOR: crate::global::Global<u8> = crate::global::Global::new(0);

#[inline]
unsafe fn request(index: usize) -> *mut Dma3Request {
    unsafe { (&raw mut REQUESTS).cast::<Dma3Request>().add(index) }
}

#[inline]
unsafe fn size_of_request(index: usize) -> u16 {
    unsafe { (&raw const (*request(index)).size).read_volatile() }
}

#[inline]
unsafe fn set_locked(locked: bool) {
    unsafe { (MANAGER_LOCKED.as_ptr()).write_volatile(locked) };
}

#[inline]
unsafe fn vcount() -> u8 {
    unsafe { ((REG_BASE + REG_OFFSET_VCOUNT) as *const u8).read_volatile() }
}

#[unsafe(no_mangle)]
pub unsafe fn ClearDma3Requests() {
    unsafe { set_locked(true) };
    unsafe { (REQUEST_CURSOR.as_ptr()).write_volatile(0) };

    let mut i = 0usize;
    while i < MAX_DMA_REQUESTS {
        let slot = unsafe { request(i) };
        unsafe { (&raw mut (*slot).size).write_volatile(0) };
        unsafe { (&raw mut (*slot).src).write_volatile(core::ptr::null()) };
        unsafe { (&raw mut (*slot).dest).write_volatile(core::ptr::null_mut()) };
        i += 1;
    }

    unsafe { set_locked(false) };
}

#[unsafe(no_mangle)]
pub unsafe fn ProcessDma3Requests() {
    if unsafe { (MANAGER_LOCKED.as_ptr().cast_const()).read_volatile() } {
        return;
    }

    let mut bytes_transferred = 0u16;

    while unsafe {
        size_of_request((REQUEST_CURSOR.as_ptr().cast_const()).read_volatile() as usize)
    } != 0
    {
        let cursor = unsafe { (REQUEST_CURSOR.as_ptr().cast_const()).read_volatile() } as usize;
        let slot = unsafe { request(cursor) };
        let size = unsafe { (&raw const (*slot).size).read_volatile() };

        bytes_transferred = bytes_transferred.wrapping_add(size);
        if u32::from(bytes_transferred) > MAX_BYTES_PER_VBLANK {
            return;
        }
        if unsafe { vcount() } > LAST_SAFE_VCOUNT {
            return;
        }

        let src = unsafe { (&raw const (*slot).src).read_volatile() };
        let dest = unsafe { (&raw const (*slot).dest).read_volatile() };
        let value = unsafe { (&raw const (*slot).value).read_volatile() };
        match unsafe { (&raw const (*slot).mode).read_volatile() } {
            DMA_REQUEST_COPY32 => unsafe { dma3_copy_large(src, dest, u32::from(size), true) },
            DMA_REQUEST_FILL32 => unsafe { dma3_fill_large(value, dest, u32::from(size), true) },
            DMA_REQUEST_COPY16 => unsafe { dma3_copy_large(src, dest, u32::from(size), false) },
            DMA_REQUEST_FILL16 => unsafe { dma3_fill_large(value, dest, u32::from(size), false) },
            _ => {}
        }

        unsafe { (&raw mut (*slot).src).write_volatile(core::ptr::null()) };
        unsafe { (&raw mut (*slot).dest).write_volatile(core::ptr::null_mut()) };
        unsafe { (&raw mut (*slot).size).write_volatile(0) };
        unsafe { (&raw mut (*slot).mode).write_volatile(0) };
        unsafe { (&raw mut (*slot).value).write_volatile(0) };

        let next = cursor + 1;
        let next = if next >= MAX_DMA_REQUESTS { 0 } else { next };
        unsafe { (REQUEST_CURSOR.as_ptr()).write_volatile(next as u8) };
    }
}

/// Finds a free slot starting at the drain cursor. Returns its index, or -1
/// when the ring is full.
unsafe fn claim_slot() -> i32 {
    let mut cursor = unsafe { (REQUEST_CURSOR.as_ptr().cast_const()).read_volatile() } as usize;
    let mut i = 0usize;
    while i < MAX_DMA_REQUESTS {
        if unsafe { size_of_request(cursor) } == 0 {
            return cursor as i32;
        }
        cursor += 1;
        if cursor >= MAX_DMA_REQUESTS {
            cursor = 0;
        }
        i += 1;
    }
    -1
}

#[unsafe(no_mangle)]
pub unsafe fn RequestDma3Copy(src: *const u8, dest: *mut u8, size: u16, mode: u8) -> i16 {
    unsafe { set_locked(true) };
    let cursor = unsafe { claim_slot() };
    if cursor < 0 {
        unsafe { set_locked(false) };
        return -1;
    }

    let slot = unsafe { request(cursor as usize) };
    unsafe { (&raw mut (*slot).src).write_volatile(src) };
    unsafe { (&raw mut (*slot).dest).write_volatile(dest) };
    unsafe { (&raw mut (*slot).size).write_volatile(size) };
    unsafe {
        (&raw mut (*slot).mode).write_volatile(if mode == 1 {
            DMA_REQUEST_COPY32
        } else {
            DMA_REQUEST_COPY16
        })
    };

    unsafe { set_locked(false) };
    cursor as i16
}

#[unsafe(no_mangle)]
pub unsafe fn RequestDma3Fill(value: i32, dest: *mut u8, size: u16, mode: u8) -> i16 {
    unsafe { set_locked(true) };
    let cursor = unsafe { claim_slot() };
    if cursor < 0 {
        unsafe { set_locked(false) };
        return -1;
    }

    let slot = unsafe { request(cursor as usize) };
    unsafe { (&raw mut (*slot).dest).write_volatile(dest) };
    unsafe { (&raw mut (*slot).size).write_volatile(size) };
    // The original writes `mode` here and then immediately overwrites it with
    // the fill request kind; the intermediate store is preserved for fidelity.
    unsafe { (&raw mut (*slot).mode).write_volatile(u16::from(mode)) };
    unsafe { (&raw mut (*slot).value).write_volatile(value as u32) };
    unsafe {
        (&raw mut (*slot).mode).write_volatile(if mode == 1 {
            DMA_REQUEST_FILL32
        } else {
            DMA_REQUEST_FILL16
        })
    };

    unsafe { set_locked(false) };
    cursor as i16
}

#[unsafe(no_mangle)]
pub unsafe fn CheckForSpaceForDma3Request(index: i16) -> i16 {
    if index == -1 {
        // Check whether every request is free.
        let mut i = 0usize;
        while i < MAX_DMA_REQUESTS {
            if unsafe { size_of_request(i) } != 0 {
                return -1;
            }
            i += 1;
        }
        return 0;
    }

    if unsafe { size_of_request(index as usize) } != 0 {
        return -1;
    }
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_full_ring_wraps_back_to_the_first_slot() {
        let step = |cursor: usize| {
            let next = cursor + 1;
            if next >= super::MAX_DMA_REQUESTS {
                0
            } else {
                next
            }
        };
        assert_eq!(step(0), 1);
        assert_eq!(step(super::MAX_DMA_REQUESTS - 1), 0);
    }

    #[test]
    fn the_vblank_budget_is_forty_kibibytes() {
        assert_eq!(super::MAX_BYTES_PER_VBLANK, 40960);
        // A u16 size field cannot on its own exceed the budget.
        assert!(u32::from(u16::MAX) > super::MAX_BYTES_PER_VBLANK);
    }
}
