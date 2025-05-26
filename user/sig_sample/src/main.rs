#![no_std]
#![no_main]

extern crate libr;

use libr::*;

fn func() {
    println!("user_sig_test passed");
    sigreturn();
}

#[unsafe(no_mangle)]
fn main() -> i32 {
    let mut new = SignalAction::default();
    let mut old = SignalAction::default();
    new.handler = func as usize;

    println!("signal_simple: sigaction");
    if sigaction(SignalID::USR1, Some(&new), Some(&mut old)) < 0 {
        panic!("Sigaction failed!");
    }
    println!("signal_simple: kill");
    if kill(getpid() as usize, SignalID::USR1) < 0 {
        println!("Kill failed!");
        exit(1);
    }
    println!("signal_simple: Done");
    0
}
