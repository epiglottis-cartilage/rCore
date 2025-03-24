#![no_std]
#![no_main]

#[macro_use]
extern crate libr;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("Hello, world! from user\n");
    0
}
