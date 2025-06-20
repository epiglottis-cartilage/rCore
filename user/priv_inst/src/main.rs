#![no_std]
#![no_main]

#[macro_use]
extern crate libr;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("Try to execute privileged instruction in U Mode");
    println!("Kernel should kill this application!");
    use core::arch::asm;
    unsafe { asm!("sret") };
    0
}
