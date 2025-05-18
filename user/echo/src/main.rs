#![no_std]
#![no_main]

#[macro_use]
extern crate libr;

#[unsafe(no_mangle)]
fn main(args: &[&str]) -> i32 {
    for arg in &args[1..] {
        println!("{}", arg);
    }
    0
}
