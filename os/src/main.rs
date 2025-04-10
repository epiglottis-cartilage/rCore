#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(step_trait)]
#![feature(new_range_api)]
#![feature(fn_align)]

pub mod lang_items;

mod sbi;
#[macro_use]
mod console;
mod label;
mod loader;
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

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    trap::init();
    info!("Hello, world!");
    memory::init();
    info!("back to world!");
    memory::remap_test();
    info!("back to world!");
    trap::init();
    loader::list_apps();
    task::add_initproc();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_tasks();
    sbi::shutdown(false);
}

/// Clear the .bss section
fn clear_bss() {
    unsafe {
        core::slice::from_raw_parts_mut(
            label::sbss as usize as *mut u8,
            label::ebss as usize - label::sbss as usize,
        )
        .fill(0);
    };
}
