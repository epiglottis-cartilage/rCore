#![no_std]
#![no_main]

#[macro_use]
extern crate libr;

use libr::{get_time, r#yield};

#[unsafe(no_mangle)]
fn main() -> i32 {
    let current_timer = get_time();
    let wait_for = current_timer + 3000;
    while get_time() < wait_for {
        r#yield();
    }
    println!("Test sleep OK!");
    0
}
