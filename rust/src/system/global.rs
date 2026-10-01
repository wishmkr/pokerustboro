//! Safe global variables.
//!
//! The game keeps its state in globals, as C does. `static mut` makes every
//! access unsafe; [`Global`] is the safe replacement for globals that are
//! read and written whole (counters, flags, ids, small structs): a `Cell`
//! that can live in a static.
//!
//! It is sound because the GBA has a single CPU core. The only concurrency
//! is interrupts, and `get`/`set` never hand out a reference, so an
//! interrupt can't observe a half-finished borrow; like in C, a
//! read-modify-write interrupted by a handler that writes the same global
//! loses one of the two writes, which the game already lives with.
//!
//! `Global<T>` has the layout of `T`, so the symbol stays compatible with
//! code that still declares it as a plain `T` (`extern { static gRngValue: u32; }`).

use core::cell::UnsafeCell;

#[repr(transparent)]
pub struct Global<T>(UnsafeCell<T>);

// SAFETY: single core; see the module documentation.
unsafe impl<T: Send> Sync for Global<T> {}

impl<T> Global<T> {
    pub const fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }

    /// A raw pointer to the value, for code that still works through pointers.
    pub const fn as_ptr(&self) -> *mut T {
        self.0.get()
    }
}

impl<T: Copy> Global<T> {
    /// The current value.
    #[inline(always)]
    pub fn get(&self) -> T {
        // SAFETY: no reference to the value exists (none is ever handed out).
        unsafe { *self.0.get() }
    }

    /// Replaces the value.
    #[inline(always)]
    pub fn set(&self, value: T) {
        // SAFETY: as in `get`.
        unsafe { *self.0.get() = value }
    }

    /// Sets the value to `f(value)` and returns the new value.
    #[inline(always)]
    pub fn update(&self, f: impl FnOnce(T) -> T) -> T {
        let new = f(self.get());
        self.set(new);
        new
    }
}

#[cfg(test)]
mod tests {
    use super::Global;

    static COUNTER: Global<u32> = Global::new(0);

    #[test]
    fn get_set_update() {
        COUNTER.set(41);
        assert_eq!(COUNTER.update(|n| n + 1), 42);
        assert_eq!(COUNTER.get(), 42);
    }
}
