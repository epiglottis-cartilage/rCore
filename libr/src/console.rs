use core::fmt::{self, Write};

use config::fs::fd::*;

use super::{read, write};

struct Stdout;
struct Stderr;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}
impl Write for Stderr {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDERR, s.as_bytes());
        Ok(())
    }
}
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}
pub fn eprint(args: fmt::Arguments) {
    Stderr.write_fmt(args).unwrap();
}

pub fn getchar() -> u8 {
    let mut c = [0u8; 1];
    read(STDIN, &mut c);
    c[0]
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! eprint {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::eprint(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! eprintln {
    ($fmt: literal $(, $($arg: tt)+)?) => {};
}
