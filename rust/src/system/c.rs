//! Small helpers for code translated from C by `tools/rustport/c2rs.py`:
//! C integer semantics that Rust's operators don't give without panics.
#![allow(dead_code)]

/// C `/` on `int`. Division by zero is undefined in C; GCC's runtime
/// (`__aeabi_idiv0`) returns 0, which is what the game gets on hardware.
#[inline]
pub fn div_i32(a: i32, b: i32) -> i32 {
    if b == 0 { 0 } else { a.wrapping_div(b) }
}

#[inline]
pub fn rem_i32(a: i32, b: i32) -> i32 {
    if b == 0 { 0 } else { a.wrapping_rem(b) }
}

#[inline]
pub fn div_u32(a: u32, b: u32) -> u32 {
    a.checked_div(b).unwrap_or(0)
}

#[inline]
pub fn rem_u32(a: u32, b: u32) -> u32 {
    a.checked_rem(b).unwrap_or(0)
}

#[inline]
pub fn div_i64(a: i64, b: i64) -> i64 {
    if b == 0 { 0 } else { a.wrapping_div(b) }
}

#[inline]
pub fn rem_i64(a: i64, b: i64) -> i64 {
    if b == 0 { 0 } else { a.wrapping_rem(b) }
}

#[inline]
pub fn div_u64(a: u64, b: u64) -> u64 {
    a.checked_div(b).unwrap_or(0)
}

#[inline]
pub fn rem_u64(a: u64, b: u64) -> u64 {
    a.checked_rem(b).unwrap_or(0)
}

// Shifts by a register amount, as the ARM7 does them: the bottom byte of the
// amount counts, and 32 or more shifts everything out (C calls it undefined;
// the game gets what the CPU does).

#[inline]
pub fn shl_u32(x: u32, n: u32) -> u32 {
    let n = n & 0xff;
    if n >= 32 { 0 } else { x << n }
}

#[inline]
pub fn shl_i32(x: i32, n: u32) -> i32 {
    shl_u32(x as u32, n) as i32
}

#[inline]
pub fn shr_u32(x: u32, n: u32) -> u32 {
    let n = n & 0xff;
    if n >= 32 { 0 } else { x >> n }
}

#[inline]
pub fn shr_i32(x: i32, n: u32) -> i32 {
    let n = n & 0xff;
    if n >= 32 { x >> 31 } else { x >> n }
}

/// A volatile store preceded by a compiler fence, so every earlier store
/// (say, to a stack buffer a DMA is about to read) has really happened.
///
/// # Safety
/// As `write_volatile`.
#[inline(always)]
pub unsafe fn volatile_write<T>(p: *mut T, v: T) {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    unsafe { p.write_volatile(v) }
}

/// Reads a bitfield `width` bits wide starting `shift` bits into the byte at
/// `p`, byte by byte (the field may straddle bytes).
///
/// # Safety
/// `p` must be valid for the bytes the field covers.
#[inline(always)]
pub unsafe fn bf_read(p: *const u8, shift: u32, width: u32, signed: bool) -> i32 {
    // Called with constant shift/width, so this folds to a byte or halfword
    // load, a shift and a mask.
    let bits = shift + width;
    let value = if bits <= 8 {
        (u32::from(unsafe { p.read() }) >> shift) & (u32::MAX >> (32 - width))
    } else if bits <= 32 {
        let mut raw = 0u32;
        for i in 0..bits.div_ceil(8) as usize {
            raw |= u32::from(unsafe { p.add(i).read() }) << (8 * i);
        }
        (raw >> shift) & (u32::MAX >> (32 - width))
    } else {
        let mut raw = 0u64;
        for i in 0..bits.div_ceil(8) as usize {
            raw |= u64::from(unsafe { p.add(i).read() }) << (8 * i);
        }
        ((raw >> shift) & (u64::MAX >> (64 - width))) as u32
    };
    if signed && width < 32 && value >> (width - 1) & 1 == 1 {
        (value | (u32::MAX << width)) as i32
    } else {
        value as i32
    }
}

/// Writes the low `width` bits of `value` into a bitfield (see [`bf_read`]).
///
/// # Safety
/// `p` must be valid for the bytes the field covers.
#[inline(always)]
pub unsafe fn bf_write(p: *mut u8, shift: u32, width: u32, value: i32) {
    let bits = shift + width;
    if bits <= 8 {
        let mask = ((u32::MAX >> (32 - width)) << shift) as u8;
        let old = unsafe { p.read() };
        unsafe { p.write((old & !mask) | (((value as u32) << shift) as u8 & mask)) };
        return;
    }
    let bytes = bits.div_ceil(8) as usize;
    let mask = (u64::MAX >> (64 - width)) << shift;
    let mut raw = 0u64;
    for i in 0..bytes {
        raw |= u64::from(unsafe { p.add(i).read() }) << (8 * i);
    }
    raw = (raw & !mask) | ((u64::from(value as u32) << shift) & mask);
    for i in 0..bytes {
        unsafe { p.add(i).write((raw >> (8 * i)) as u8) };
    }
}

// libc functions used by translated code, with C's signatures as c2rs sees
// them (declaring `memcpy` & co. with other types trips a rustc lint).

/// # Safety
/// As C `memcpy`.
pub unsafe fn memcpy(dst: *mut u8, src: *mut u8, n: u32) -> *mut u8 {
    unsafe { core::ptr::copy_nonoverlapping(src, dst, n as usize) };
    dst
}

/// # Safety
/// As C `memset`.
pub unsafe fn memset(dst: *mut u8, c: i32, n: u32) -> *mut u8 {
    unsafe { core::ptr::write_bytes(dst, c as u8, n as usize) };
    dst
}

/// # Safety
/// As C `memcmp`.
pub unsafe fn memcmp(a: *mut u8, b: *mut u8, n: u32) -> i32 {
    for i in 0..n as usize {
        let (x, y) = unsafe { (a.add(i).read(), b.add(i).read()) };
        if x != y {
            return i32::from(x) - i32::from(y);
        }
    }
    0
}

/// # Safety
/// As C `strcmp`.
pub unsafe fn strcmp(a: *mut u8, b: *mut u8) -> i32 {
    let mut i = 0;
    loop {
        let (x, y) = unsafe { (a.add(i).read(), b.add(i).read()) };
        if x != y || x == 0 {
            return i32::from(x) - i32::from(y);
        }
        i += 1;
    }
}

/// # Safety
/// As C `strcpy`.
pub unsafe fn strcpy(dst: *mut u8, src: *mut u8) -> *mut u8 {
    let mut i = 0;
    loop {
        let c = unsafe { src.add(i).read() };
        unsafe { dst.add(i).write(c) };
        if c == 0 {
            return dst;
        }
        i += 1;
    }
}

/// The memory barrier LLVM calls for fences on ARMv4T, which has no barrier
/// instruction. The ARM7TDMI is a single in-order core, so there is nothing
/// to order (newlib's version for this CPU is empty too).
#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
pub fn __sync_synchronize() {}

/// A C struct or union passed by value, as its bytes. The alignment matches
/// the C type so the size (and so the calling convention) does too.
#[repr(C, align(1))]
#[derive(Clone, Copy)]
pub struct Rec1<const N: usize>(pub [u8; N]);

#[repr(C, align(2))]
#[derive(Clone, Copy)]
pub struct Rec2<const N: usize>(pub [u8; N]);

#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct Rec4<const N: usize>(pub [u8; N]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitfields_round_trip_across_bytes() {
        let mut bytes = [0u8; 4];
        unsafe { bf_write(bytes.as_mut_ptr(), 5, 7, 0x7f) };
        assert_eq!(bytes, [0xe0, 0x0f, 0, 0]);
        assert_eq!(unsafe { bf_read(bytes.as_ptr(), 5, 7, false) }, 0x7f);
        assert_eq!(unsafe { bf_read(bytes.as_ptr(), 5, 7, true) }, -1);
    }

    #[test]
    fn division_by_zero_is_zero() {
        assert_eq!(div_i32(7, 0), 0);
        assert_eq!(rem_u32(7, 0), 0);
        assert_eq!(div_i32(i32::MIN, -1), i32::MIN);
    }
}

/// A C array: `[T; N]` whose indexing, like C's, is not bounds-checked (the
/// game reads past the end of some arrays, and a check would add panics).
/// Any integer type indexes it; negative indices address before the start.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CArray<T, const N: usize>(pub [T; N]);

impl<T, const N: usize> CArray<T, N> {
    /// The number of elements (C's `ARRAY_COUNT`).
    pub const LEN: usize = N;

    #[inline(always)]
    pub const fn len(&self) -> usize {
        N
    }

    /// The array decayed to a pointer to its first element.
    #[inline(always)]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.0.as_mut_ptr()
    }

    #[inline(always)]
    pub const fn as_ptr(&self) -> *const T {
        self.0.as_ptr()
    }
}

/// An integer usable as a C array index or pointer offset.
pub trait CIndex: Copy {
    fn offset(self) -> isize;
}

macro_rules! c_index {
    ($($t:ty),*) => {$(
        impl CIndex for $t {
            #[inline(always)]
            fn offset(self) -> isize {
                self as isize
            }
        }
    )*};
}
c_index!(i8, u8, i16, u16, i32, u32, isize, usize);

impl<T, I: CIndex, const N: usize> core::ops::Index<I> for CArray<T, N> {
    type Output = T;
    #[inline(always)]
    fn index(&self, i: I) -> &T {
        unsafe { &*self.0.as_ptr().wrapping_offset(i.offset()) }
    }
}

impl<T, I: CIndex, const N: usize> core::ops::IndexMut<I> for CArray<T, N> {
    #[inline(always)]
    fn index_mut(&mut self, i: I) -> &mut T {
        unsafe { &mut *self.0.as_mut_ptr().wrapping_offset(i.offset()) }
    }
}

// The game's data is only touched from its one thread (and interrupts,
// as in C): let its tables live in statics.
unsafe impl<T, const N: usize> Sync for CArray<T, N> {}

/// C pointer arithmetic: `p.at(i)` is `p + i` (wrapping, like the hardware).
pub trait CPtr: Sized {
    fn at<I: CIndex>(self, i: I) -> Self;
}

impl<T> CPtr for *mut T {
    #[inline(always)]
    fn at<I: CIndex>(self, i: I) -> Self {
        self.wrapping_offset(i.offset())
    }
}

impl<T> CPtr for *const T {
    #[inline(always)]
    fn at<I: CIndex>(self, i: I) -> Self {
        self.wrapping_offset(i.offset())
    }
}

/// A static that C would align to 4 bytes (GCC word-aligns global arrays
/// and structs; DMA and 32-bit copies rely on it). Derefs to the value, so
/// `sBuffer[i]` and `sState.field` read as usual.
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct Aligned<T>(pub T);

impl<T> core::ops::Deref for Aligned<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> core::ops::DerefMut for Aligned<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

/// A typed view of a const table that cdata.py stored as bytes: derefs to
/// the table, so `sTable[i].field` reads as in C.
pub struct Table<T>(pub *const T);

unsafe impl<T> Sync for Table<T> {}

impl<T> core::ops::Deref for Table<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &T {
        unsafe { &*self.0 }
    }
}
