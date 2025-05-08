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
        SyscallID::GetTime => sys_get_time(),
        SyscallID::GetPid => sys_get_pid(),
        SyscallID::Sbrk => sys_sbrk(args[0] as isize),
        SyscallID::Fork => sys_fork(),
        SyscallID::Exec => sys_exec(args[0] as *const u8),
        SyscallID::WaitPid => sys_waitpid(args[0] as isize, args[1] as *mut i32),
        SyscallID::Read => sys_read(args[0], args[1] as *mut u8, args[2]),
        SyscallID::Open => sys_open(args[0] as *const u8, args[1]),
        SyscallID::Close => sys_close(args[0]),
        // _ => unreachable!("Unsupported syscall_id: {:?}", syscall_id),
    }
}
