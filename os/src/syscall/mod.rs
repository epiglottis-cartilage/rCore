//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

use config::syscall::SyscallID;

mod fs;
mod process;

use fs::*;
use process::*;

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: SyscallID, args: [usize; 3]) -> isize {
    match syscall_id {
        SyscallID::Write => sys_write(args[0], args[1] as *const u8, args[2]),
        SyscallID::Exit => sys_exit(args[0] as i32),
        SyscallID::Yield => sys_yield(),
        _ => panic!("Unsupported syscall_id: {:?}", syscall_id),
    }
}
