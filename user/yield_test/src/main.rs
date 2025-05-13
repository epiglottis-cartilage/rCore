#![no_std]
#![no_main]

#[macro_use]
extern crate libr;
use libr::{getpid, r#yield};

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    println!("Hello, I am process {}.", getpid());
    for i in 0..5 {
        r#yield();
        println!("Back in process {}, iteration {}.", getpid(), i);
    }
    println!("yield pass.");
    0
}
