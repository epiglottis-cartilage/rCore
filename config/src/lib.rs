#![no_std]
#![feature(default_field_values)]
pub mod memory;
pub mod syscall;
pub mod timer;

pub const INIT_PROC_NAME: &str = "init";
