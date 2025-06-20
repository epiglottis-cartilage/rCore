#![no_std]
#![no_main]

#[macro_use]
extern crate libr;

use libr::{get_time, sleep, r#yield};

#[unsafe(no_mangle)]
fn main(args: &[&str]) -> i32 {
    let Some(duration) = args.get(1).map(|str| str.parse().unwrap()) else {
        println!("Usage: sleep <duration ms>");
        return 1;
    };
    let current_timer = get_time();
    sleep(duration);
    let elapsed = get_time() - current_timer;
    if elapsed >= duration {
        println!("Sleep OK!");
    } else {
        println!("Sleep failed!");
    }
    println!("Test sleep OK!");
    0
}
