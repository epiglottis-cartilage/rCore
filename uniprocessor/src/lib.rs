#![no_std]
#![feature(deref_pure_trait)]
///! This is a cell provide for multithreaded environment.
///! It is not safe to use in multi-core environment.
///! But our os run in single-core environment, then it is safe, up to now.
use core::{
    cell::{LazyCell, RefCell},
    ops::{Deref, DerefMut, DerefPure},
};

/// # Uniprocessor Safe Cell
pub struct UpSafeCell<T>(
    /// inner data
    RefCell<T>,
);

/// We should only use it in uniprocessor.
/// And we should make sure that the Ref is dropped before switch stack.
unsafe impl<T> Sync for UpSafeCell<T> {}

impl<T> UpSafeCell<T> {
    /// User is responsible to guarantee that inner struct is only used in uniprocessor.
    pub const unsafe fn new(value: T) -> Self {
        Self(RefCell::new(value))
    }
}
impl<T> Deref for UpSafeCell<T> {
    type Target = RefCell<T>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for UpSafeCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
/// Clearly, it is pure.
unsafe impl<T> DerefPure for UpSafeCell<T> {}

/// # Uniprocessor Safe LazyCell
pub struct UpSafeLazyCell<T, F = fn() -> T>(LazyCell<T, F>);

impl<T, F: FnOnce() -> T> UpSafeLazyCell<T, F> {
    pub const unsafe fn new(f: F) -> Self {
        Self(LazyCell::new(f))
    }
}
unsafe impl<T> Sync for UpSafeLazyCell<T> {}
impl<T> Deref for UpSafeLazyCell<T> {
    type Target = LazyCell<T>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for UpSafeLazyCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
/// Clearly, it is pure.
unsafe impl<T> DerefPure for UpSafeLazyCell<T> {}
