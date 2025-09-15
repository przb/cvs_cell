#![no_std]

//! # `CvsCell`
//!
//! Similar to a [`Cell`](core::cell::Cell), but with a few major differences.
//! - [`CvsCell`] implements both [`Send`] **and** [`Sync`].
//! - The constructor, [`new`](CvsCell::new), is unsafe.
//! - All reads and writes are volatile.
//!
//! This is designed to be used in a single-threaded environment, though it can be used with
//! multiple threads if the programmer verifies that a reference is not shared between multiple
//! threads.
//!
//! ## Safety
//!
//! This [`CvsCell`] cannot be shared between multiple threads. This is must be validated
//! by the user of the [`CvsCell`], hence why [`new`](CvsCell::new) is marked as unsafe.
//!
//! ## Examples
//!
//! ### Intended Usage
//!
//! ```rust
//! use cvs_cell::CvsCell;
//!
//! static C: CvsCell<u32> = unsafe { CvsCell::new(0) };
//!
//! fn main() {
//!   assert_eq!(C.get(), 0);
//! }
//! ```
//!
//! ### Unsafe Usage
//! This is a bit of a contrived example, but this would cause race conditions, as the
//! the cell is being shared between threads
//! ```rust,no_run
//! use cvs_cell::CvsCell;
//!
//! static C: CvsCell<u32> = unsafe { CvsCell::new(0) };
//!
//! fn increment_1000() {
//!   for i in 0..1000 {
//!     C.update(|old_val| old_val + 1);
//!   }
//! }
//!
//! fn main() {
//!   // Using a thread scope so we don't need to manually join them all at the end
//!   let jh = std::thread::scope(|s|{
//!     // Spawn 10 threads
//!     for i in 0..10 {
//!       s.spawn(increment_1000);
//!     }
//!   });
//!
//!   // This may not equal 10,000!
//!   assert_eq!(C.get(), 10 * 1000)
//! }
//! ```

use core::cell::UnsafeCell;

/// A `const volatile static` (CVS) Cell. A mutable memory location.
///
/// # Safety
/// A [`CvsCell`], and a reference to a [`CvsCell`], cannot be shared between
/// threads safely. Doing so will cause data races. This should only be used
/// in single-threaded code and outside of interrupt driven pre-emption. See
/// the module-level docs for more info.
///
/// # Memory Layout
/// As with the [`core::cell::Cell`], a [`CvsCell<T>`] has the same [memory
/// layout and caveats as `UnsafeCell<T>`](core::cell::UnsafeCell#memory-layout).
/// In particular, this means that [`CvsCell<T>`] has the same in-memory
/// representation as its inner type `T`.
///
#[repr(transparent)]
pub struct CvsCell<T> {
    value: UnsafeCell<T>,
}

// implicit impl for send

/// Implementing Sync, since [`new`](CvsCell::new) is unsafe. The caller has to
/// verify this cell is not shared between threads.
unsafe impl<T> Sync for CvsCell<T> {}

impl<T> CvsCell<T> {
    /// Make a new [`CvsCell`] with the given value `val`.
    ///
    /// # Safety
    /// A [`CvsCell`] is only safe to use when not shared between threads. When
    /// creating a [`CvsCell`] with the [`CvsCell::new`] function, the caller
    /// must manually verify that the [`CvsCell`] is not shared between threads.
    ///
    /// See the module-level docs for more info.
    ///
    /// # Example
    /// ```
    /// use cvs_cell::CvsCell;
    ///
    /// // SAFETY:
    /// // The calller must verify this cell, and a reference to this cell is not shared between threads
    /// static FOO: CvsCell<u32> = unsafe { CvsCell::new(0) };
    ///
    /// assert_eq!(FOO.get(), 0);
    /// ```
    #[inline]
    pub const unsafe fn new(val: T) -> Self {
        Self {
            value: UnsafeCell::new(val),
        }
    }

    /// Sets the new value inside the cell to `val`. Replaces the old value
    ///
    /// # Example
    /// ```
    /// use cvs_cell::CvsCell;
    ///
    /// static FOO: CvsCell<u32> = unsafe { CvsCell::new(0) };
    ///
    /// FOO.set(100);
    /// ```
    #[inline]
    pub fn set(&self, val: T) {
        unsafe { self.value.get().write_volatile(val) };
    }

    /// Consumes the cell, returning the inner value
    #[inline]
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }

    /// Get a copy of the current value of the cell
    ///
    /// # Example
    /// ```
    /// use cvs_cell::CvsCell;
    ///
    /// static FOO: CvsCell<u32> = unsafe { CvsCell::new(0) };
    ///
    /// assert_eq!(FOO.get(), 0);
    /// ```
    #[inline]
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { self.value.get().read_volatile() }
    }

    /// Update the value in the cell with the given function. Returns the old data
    ///
    /// # Example
    /// ```
    /// use cvs_cell::CvsCell;
    ///
    /// static FOO: CvsCell<u32> = unsafe { CvsCell::new(4) };
    ///
    /// let old_val = FOO.update(|old_val| old_val * 2);
    ///
    /// assert_eq!(old_val, 4);
    /// assert_eq!(FOO.get(), 8);
    /// ```
    #[inline]
    pub fn update(&self, f: impl FnOnce(T) -> T) -> T
    where
        T: Copy,
    {
        let old = self.get();
        self.set(f(old));
        old
    }

    /// Gets a mutable pointer to the wrapped value
    ///
    /// # Example
    /// ```
    /// use cvs_cell::CvsCell;
    ///
    /// static FOO: CvsCell<u32> = unsafe { CvsCell::new(0) };
    ///
    /// let ptr = FOO.as_ptr();
    /// ```
    #[inline]
    pub const fn as_ptr(&self) -> *mut T {
        self.value.get()
    }
}

#[cfg(test)]
mod tests {}
