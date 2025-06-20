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
mod cfg {
    pub use config::signal::*;
    pub use config::syscall::*;
}

mod fs;
mod process;

use fs::*;
use process::*;

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: cfg::SyscallID, args: [usize; 3]) -> isize {
    use cfg::SyscallID;
    match syscall_id {
        SyscallID::Dup => sys_dup(args[0]),
        SyscallID::Write => sys_write(args[0], args[1] as _, args[2]),
        SyscallID::Exit => sys_exit(args[0] as _),
        SyscallID::Yield => sys_yield(),
        SyscallID::Kill => sys_kill(args[0], args[1] as _),
        SyscallID::SigAction => sys_sigaction(args[0] as _, args[1] as _, args[2] as _),
        SyscallID::SigProcMask => sys_sigprocmask(args[0] as _),
        SyscallID::SigReturn => sys_sigreturn(),
        SyscallID::GetTime => sys_get_time(),
        SyscallID::GetPid => sys_get_pid(),
        SyscallID::Sbrk => sys_sbrk(args[0] as _),
        SyscallID::Fork => sys_fork(),
        SyscallID::Exec => sys_exec(args[0] as _, args[1] as _),
        SyscallID::WaitPid => sys_waitpid(args[0] as _, args[1] as _),
        SyscallID::Read => sys_read(args[0], args[1] as _, args[2]),
        SyscallID::Open => sys_open(args[0] as _, args[1]),
        SyscallID::Close => sys_close(args[0]),
        SyscallID::Pipe => sys_pipe(args[0] as _, args[1] as _),
        SyscallID::PowerOff => crate::sbi::shutdown(false), // _ => unreachable!("Unsupported syscall_id: {:?}", syscall_id),
    }
}
