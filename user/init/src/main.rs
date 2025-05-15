#![no_std]
#![no_main]

#[macro_use]
extern crate libr;

use libr::{exec, fork, wait, r#yield};
#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("[initproc] Init process started");
    if fork() == 0 {
        exec("shell", &[]);
    } else {
        loop {
            let mut exit_code: i32 = 0;
            let pid = wait(&mut exit_code);
            if pid == -1 {
                r#yield();
                continue;
            }
            println!(
                "[initproc] Released a zombie process, pid={}, exit_code={}",
                pid, exit_code,
            );
        }
    }
    0
}
