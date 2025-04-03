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
