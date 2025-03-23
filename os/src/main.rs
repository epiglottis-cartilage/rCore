#![no_std]
#![no_main]
pub mod lang_items;

mod sbi;
#[macro_use]
mod console;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    panic!("end");
}

fn clear_bss() {
    unsafe extern "C" {
        static start_of_bss: usize;
        static end_of_bss: usize;
    }

    unsafe { (start_of_bss..end_of_bss).for_each(|a| (a as *mut u8).write_volatile(0)) };
}
