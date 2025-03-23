#![no_std]
#![no_main]
pub mod lang_items;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
