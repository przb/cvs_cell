//! LIke a volatile cell, but with a differnet invariant.
//!
//!
//!
//!

#![no_std]

use core::{cell::UnsafeCell, mem};

/// A `const volatile static` (CVS) Cell.
#[repr(transparent)]
pub struct CvsCell<T> {
    value: UnsafeCell<T>,
}

// implicit impl for send

/// Implementing Sync, since [`new`](CvsCell::new) is unsafe. The caller has to verify this cell is not shared between threads.
unsafe impl<T> Sync for CvsCell<T> {}

impl<T> CvsCell<T> {
    /// make a new cell
    #[inline]
    pub const unsafe fn new(val: T) -> Self {
        Self {
            value: UnsafeCell::new(val),
        }
    }

    /// Sets the new value inside the cell
    #[inline]
    pub fn set(&self, val: T) {
        self.replace(val);
    }

    /// replace the value in the cell, and return the old value
    #[inline]
    pub fn replace(&self, val: T) -> T {
        let mut old = unsafe { self.value.get().read_volatile() };
        mem::replace(&mut old, val)
    }

    /// Consumes the cell, returning the inner value
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}

impl<T: Copy> CvsCell<T> {
    /// Get a copy of the current value of the cell
    #[inline]
    pub fn get(&self) -> T {
        unsafe { self.value.get().read_volatile() }
    }

    /// Update the value in the cell with the given function. Returns the old data
    #[inline]
    pub fn update(&self, f: impl FnOnce(T) -> T) -> T {
        let old = self.get();
        self.set(f(old));
        old
    }
}

#[cfg(test)]
mod tests {}
