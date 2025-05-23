#![no_std]
#![no_main]

#[macro_use]
extern crate libr;

use core::ptr::{null_mut, read_volatile};

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("\nload_fault APP running...\n");
    println!("Into Test load_fault, we will insert an invalid load operation...");
    println!("Kernel should kill this application!");
    unsafe {
        #[allow(invalid_null_arguments)]
        let _i = read_volatile(null_mut::<u8>());
    }
    0
}
