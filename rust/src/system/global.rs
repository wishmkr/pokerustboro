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

// SAFETY: single core; see the module documentation. No `T: Send` bound:
// there are no threads to send to, and the game's structs hold raw pointers.
unsafe impl<T> Sync for Global<T> {}

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

/// A field (or element) of a struct or array [`Global`], read or written by
/// value, like [`Global::get`] / [`Global::set`] for the whole value.
///
/// ```ignore
/// let keys = gl!(gMain.newKeys);          // read
/// gl!(gMain.state = 3);                   // write
/// gl!(gSprites[id].x2 += 1);              // read-modify-write
/// let p: *mut u8 = gl!(&raw gMain.state); // address, for pointer code
/// ```
///
/// The path after the static may only be `.field` and `[index]`, where an
/// index is a local variable or a literal (anything else is a compile
/// error): no method call, no dereference, nothing that runs code while the
/// place is formed. A value to store is evaluated before the place. So no
/// reference into the static outlives the one load or store, exactly as
/// with `get`/`set`. Indexing is bounds-checked.
#[macro_export]
macro_rules! gl {
    // the address of the place
    (&raw $g:ident $($p:tt)*) => { $crate::gl!(@raw ($g) $($p)*) };
    (&raw @at $g:path; $($p:tt)*) => { $crate::gl!(@raw ($g) $($p)*) };
    (@raw $g:tt $($p:tt)*) => {{
        $crate::gl!(@path $($p)*);
        // SAFETY: a field/element projection of a valid static; see `gl!`.
        #[allow(unused_unsafe)]
        let p = unsafe { &raw mut (*$g.as_ptr())$($p)* };
        p
    }};
    // `place = value`, `place op= value` or a read of `place`
    (@w $g:tt [$($acc:tt)*] = $v:expr) => {{
        $crate::gl!(@path $($acc)*);
        // SAFETY: as above; `Place::set` computes the value before it
        // forms the place. (The place's type is known when the value is
        // checked, so `Some(func)` coerces to the field's fn pointer type.)
        #[allow(unused_unsafe)]
        let place = unsafe { $crate::global::Place::new(|| &raw mut (*$g.as_ptr())$($acc)*) };
        place.set($v);
    }};
    (@w $g:tt [$($acc:tt)*] += $v:expr) => {{
        let value = $v;
        $crate::gl!(@path $($acc)*);
        // SAFETY: as above.
        #[allow(unused_unsafe)]
        let () = unsafe { (*$g.as_ptr())$($acc)* += value };
    }};
    (@w $g:tt [$($acc:tt)*] -= $v:expr) => {{
        let value = $v;
        $crate::gl!(@path $($acc)*);
        // SAFETY: as above.
        #[allow(unused_unsafe)]
        let () = unsafe { (*$g.as_ptr())$($acc)* -= value };
    }};
    (@w $g:tt [$($acc:tt)*] *= $v:expr) => {{
        let value = $v;
        $crate::gl!(@path $($acc)*);
        // SAFETY: as above.
        #[allow(unused_unsafe)]
        let () = unsafe { (*$g.as_ptr())$($acc)* *= value };
    }};
    (@w $g:tt [$($acc:tt)*] /= $v:expr) => {{
        let value = $v;
        $crate::gl!(@path $($acc)*);
        // SAFETY: as above.
        #[allow(unused_unsafe)]
        let () = unsafe { (*$g.as_ptr())$($acc)* /= value };
    }};
    (@w $g:tt [$($acc:tt)*] %= $v:expr) => {{
        let value = $v;
        $crate::gl!(@path $($acc)*);
        // SAFETY: as above.
        #[allow(unused_unsafe)]
        let () = unsafe { (*$g.as_ptr())$($acc)* %= value };
    }};
    (@w $g:tt [$($acc:tt)*] &= $v:expr) => {{
        let value = $v;
        $crate::gl!(@path $($acc)*);
        // SAFETY: as above.
        #[allow(unused_unsafe)]
        let () = unsafe { (*$g.as_ptr())$($acc)* &= value };
    }};
    (@w $g:tt [$($acc:tt)*] |= $v:expr) => {{
        let value = $v;
        $crate::gl!(@path $($acc)*);
        // SAFETY: as above.
        #[allow(unused_unsafe)]
        let () = unsafe { (*$g.as_ptr())$($acc)* |= value };
    }};
    (@w $g:tt [$($acc:tt)*] ^= $v:expr) => {{
        let value = $v;
        $crate::gl!(@path $($acc)*);
        // SAFETY: as above.
        #[allow(unused_unsafe)]
        let () = unsafe { (*$g.as_ptr())$($acc)* ^= value };
    }};
    (@w $g:tt [$($acc:tt)*] <<= $v:expr) => {{
        let value = $v;
        $crate::gl!(@path $($acc)*);
        // SAFETY: as above.
        #[allow(unused_unsafe)]
        let () = unsafe { (*$g.as_ptr())$($acc)* <<= value };
    }};
    (@w $g:tt [$($acc:tt)*] >>= $v:expr) => {{
        let value = $v;
        $crate::gl!(@path $($acc)*);
        // SAFETY: as above.
        #[allow(unused_unsafe)]
        let () = unsafe { (*$g.as_ptr())$($acc)* >>= value };
    }};
    (@w $g:tt [$($acc:tt)*]) => {{
        $crate::gl!(@path $($acc)*);
        // SAFETY: as above.
        #[allow(unused_unsafe)]
        let v = unsafe { (*$g.as_ptr())$($acc)* };
        v
    }};
    (@w $g:tt [$($acc:tt)*] $t:tt $($rest:tt)*) => {
        $crate::gl!(@w $g [$($acc)* $t] $($rest)*)
    };
    // the path may only be `.field` and `[local or literal]`
    (@path) => {};
    (@path . $f:ident $($rest:tt)*) => { $crate::gl!(@path $($rest)*) };
    (@path [$i:ident] $($rest:tt)*) => { $crate::gl!(@path $($rest)*) };
    (@path [$i:literal] $($rest:tt)*) => { $crate::gl!(@path $($rest)*) };
    (@at $g:path; $($rest:tt)*) => { $crate::gl!(@w ($g) [] $($rest)*) };
    ($g:ident $($rest:tt)*) => { $crate::gl!(@w ($g) [] $($rest)*) };
}

/// A place in a [`Global`] formed only when written; see `gl!`.
#[doc(hidden)]
pub struct Place<T, F: FnOnce() -> *mut T>(F, core::marker::PhantomData<*mut T>);

impl<T, F: FnOnce() -> *mut T> Place<T, F> {
    /// # Safety
    ///
    /// `f` must return a valid, aligned pointer to a `T` that no reference
    /// points into when it is called.
    #[inline(always)]
    pub unsafe fn new(f: F) -> Self {
        Self(f, core::marker::PhantomData)
    }

    /// Stores `value` (already computed by the caller) in the place.
    #[inline(always)]
    pub fn set(self, value: T) {
        // SAFETY: the contract of `new`.
        unsafe { *(self.0)() = value }
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

    #[derive(Clone, Copy)]
    struct Inner {
        a: [u16; 4],
        b: u8,
    }
    struct Outer {
        x: u32,
        inner: [Inner; 3],
    }
    static OUTER: Global<Outer> = Global::new(Outer {
        x: 0,
        inner: [Inner { a: [0; 4], b: 0 }; 3],
    });

    #[test]
    fn field_places() {
        let i = 2;
        gl!(OUTER.x = 7);
        gl!(OUTER.x += 3);
        gl!(OUTER.x <<= 1);
        gl!(OUTER.inner[i].a[1] = 5);
        gl!(OUTER.inner[i].a[1] |= 0x30);
        gl!(OUTER.inner[0].b = gl!(OUTER.inner[i].a[1]) as u8);
        gl!(@at crate::global::tests::OUTER; .inner[1].b -= 1);
        assert_eq!(gl!(OUTER.x), 20);
        assert_eq!(gl!(OUTER.inner[i].a[1]), 0x35);
        assert_eq!(gl!(OUTER.inner[0].b), 0x35);
        assert_eq!(gl!(OUTER.inner[1].b), 0xff_u8.wrapping_add(0));
        let p = gl!(&raw OUTER.inner[i].a[1]);
        fn f() {}
        static FNS: Global<[Option<fn()>; 2]> = Global::new([None; 2]);
        gl!(FNS[1] = Some(f));
        assert!(gl!(FNS[1]).is_some());
        assert_eq!(
            p as usize,
            OUTER.as_ptr() as usize
                + core::mem::offset_of!(Outer, inner)
                + 2 * core::mem::size_of::<Inner>()
                + 2
        );
    }
}
