#![no_std]
#![no_main]

#[macro_use]
extern crate libr;

use core::ptr::null_mut;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("\nstore_fault APP running...\n");
    println!("Into Test store_fault, we will insert an invalid store operation...");
    println!("Kernel should kill this application!");
    unsafe {
        null_mut::<u8>().write_volatile(1);
    }
    0
}
