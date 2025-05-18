#![no_std]
#![no_main]

#[macro_use]
extern crate libr;
use libr::{OpenFlag, close, open, read};

#[unsafe(no_mangle)]
fn main(args: &[&str]) -> i32 {
    if args.len() != 2 {
        println!("Usage: cat <filename>");
        return 1;
    }
    let file = args[1];
    let fd = match open(file, OpenFlag::RDONLY) {
        fd if fd > 0 => fd as usize,
        _ => {
            println!("Failed to open file: {}", file);
            return 1;
        }
    };

    let mut buf = [0u8; 256];
    loop {
        let size = read(fd, &mut buf) as usize;
        if size == 0 {
            break;
        }
        println!(
            "I read this: {}",
            core::str::from_utf8(&buf[..size]).unwrap()
        );
    }
    close(fd);
    0
}
