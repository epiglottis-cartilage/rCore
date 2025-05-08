#![no_std]
#![no_main]

#[macro_use]
extern crate libr;
use libr::{OpenFlag, close, open, read};

#[unsafe(no_mangle)]
fn main() -> i32 {
    let fd = open("filea\0", OpenFlag::RDONLY);
    if fd == -1 {
        panic!("Error occured when opening file");
    }
    let fd = fd as usize;
    let mut buf = [0u8; 256];
    loop {
        let size = read(fd, &mut buf) as usize;
        if size == 0 {
            break;
        }
        println!(
            "I read this:{}",
            core::str::from_utf8(&buf[..size]).unwrap()
        );
    }
    close(fd);
    0
}
