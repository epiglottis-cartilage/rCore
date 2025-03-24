//! App management syscalls
use crate::batch;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    batch::run_next_app()
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    batch::suspend_current_and_run_next();
    0
}
