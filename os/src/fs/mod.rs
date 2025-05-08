//! File system in os
mod inode;
mod stdio;

use crate::memory::UserBuffer;
/// File trait
pub trait File: Send + Sync {
    /// If readable
    fn readable(&self) -> bool;
    /// If writable
    fn writable(&self) -> bool;
    /// Read file to `UserBuffer`
    fn read(&self, buf: UserBuffer) -> usize;
    /// Write `UserBuffer` to file
    fn write(&self, buf: UserBuffer) -> usize;
}

pub use inode::{OSInode, list_apps, open_file};
pub use stdio::{Stderr, Stdin, Stdout};

#[deny(dead_code)]
pub fn init() {
    inode::init();
}
