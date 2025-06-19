#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(step_trait)]
#![feature(new_range_api)]
#![feature(fn_align)]
#![allow(static_mut_refs)]
#![feature(iterator_try_collect)]
#![feature(slice_from_ptr_range)]
pub mod lang_items;

mod sbi;
#[macro_use]
mod console;
mod drivers;
mod fs;
mod label;
mod logging;
mod memory;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;
extern crate alloc;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

use log::*;

#[unsafe(export_name = "rust_main")]
pub fn main() -> ! {
    clear_bss();
    logging::init();
    info!("Hello, world!");
    memory::init();
    info!("back to world!");
    memory::remap_test();
    trap::init();

    fs::list_apps();
    task::add_init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_tasks();
    sbi::shutdown(false);
}

/// Clear the .bss section
fn clear_bss() {
    unsafe {
        let sbss = label::sbss as usize as *mut u8;
        let ebss = label::ebss as usize as *mut u8;
        core::slice::from_mut_ptr_range(sbss..ebss).fill(0);
    };
}
