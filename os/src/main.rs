#![no_std]
#![no_main]
pub mod lang_items;

mod sbi;
#[macro_use]
mod console;
mod loader;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.asm"));

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    trap::init();
    println!("[kernel] Hello, world!");
    loader::load_apps();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    sbi::shutdown(false);
}

fn clear_bss() {
    unsafe extern "C" {
        static start_of_bss: usize;
        static end_of_bss: usize;
    }
    unsafe { (start_of_bss..end_of_bss).for_each(|a| (a as *mut u8).write_volatile(0)) };
}
