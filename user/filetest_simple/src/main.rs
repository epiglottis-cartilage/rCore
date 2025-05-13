#![no_std]
#![no_main]

#[macro_use]
extern crate libr;
use libr::{OpenFlag, close, open, read, write};

#[unsafe(no_mangle)]
fn main() -> i32 {
    let test_str = "I'm going write some 💩 in your disk🥵";
    let filea = "filea\0";
    let fd = open(filea, OpenFlag::CREATE | OpenFlag::WRONLY);
    assert!(fd > 0);
    let fd = fd as usize;
    write(fd, test_str.as_bytes());
    println!("{}", test_str);
    close(fd);

    let fd = open(filea, OpenFlag::RDONLY);
    assert!(fd > 0);
    let fd = fd as usize;
    let mut buffer = [0u8; 100];
    let read_len = read(fd, &mut buffer) as usize;
    close(fd);

    assert_eq!(test_str, core::str::from_utf8(&buffer[..read_len]).unwrap(),);
    println!("file_test passed!");
    0
}
