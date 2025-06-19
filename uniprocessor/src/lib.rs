//! Wrap a static data structure inside it so that we are
//! able to access it without any `unsafe`.
//!
//! We should only use it in uniprocessor.
#![no_std]
use core::cell::{LazyCell, RefCell};

/// # Uniprocessor Safe Cell
pub struct UpSafeCell<T>(
    /// inner data
    RefCell<T>,
);

unsafe impl<T> Sync for UpSafeCell<T> {}

impl<T> UpSafeCell<T> {
    /// User is responsible to guarantee that inner struct is only used in
    /// uniprocessor.
    pub unsafe fn new(value: T) -> Self {
        Self(RefCell::new(value))
    }
}
impl<T> core::ops::Deref for UpSafeCell<T> {
    type Target = RefCell<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// # Uniprocessor Safe LazyCell
pub struct UpSafeLazyCell<T, F = fn() -> T>(LazyCell<T, F>);

impl<T, F: FnOnce() -> T> UpSafeLazyCell<T, F> {
    pub const unsafe fn new(f: F) -> Self {
        Self(LazyCell::new(f))
    }
}
unsafe impl<T> Sync for UpSafeLazyCell<T> {}
impl<T, F: FnOnce() -> T> core::ops::Deref for UpSafeLazyCell<T, F> {
    type Target = LazyCell<T, F>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
