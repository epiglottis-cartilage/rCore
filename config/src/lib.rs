#![no_std]
pub mod memory;
pub mod syscall;
pub mod timer;

pub const MAX_APP_NUM: usize = 4;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;
