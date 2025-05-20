#![no_std]
#![feature(default_field_values)]
pub mod fs;
pub mod memory;
mod qemu;
pub mod signal;
pub mod syscall;
pub mod timer;

pub const INIT_PROC_NAME: &str = "init";
