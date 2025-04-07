//! App management syscalls
use crate::{task, timer};

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    task::exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    task::suspend_current_and_run_next();
    0
}

/// get time in milliseconds
pub fn sys_get_time() -> isize {
    timer::get_time_ms() as isize
}

/// change data segment size
pub fn sys_sbrk(size: isize) -> isize {
    if let Some(old_brk) = task::change_program_brk(size) {
        old_brk as _
    } else {
        -1
    }
}

pub fn sys_fork() -> isize {
    todo!()
}

pub fn sys_exec(path: *const u8) -> isize {
    todo!()
}

/// If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, return -2.
pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    todo!()
}
