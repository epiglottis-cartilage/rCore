//! App management syscalls
use super::cfg::{SignalAction, SignalFlags};
use crate::{
    fs, memory,
    task::{self, current_task},
    timer,
};
use alloc::{borrow::ToOwned, string::String, sync::Arc};

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    task::exit_current_and_run_next(exit_code);
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

/// get time in milliseconds
pub fn sys_get_pid() -> isize {
    current_task().unwrap().pid.0 as _
}

/// change data segment size
pub fn sys_sbrk(_size: isize) -> isize {
    unimplemented!()
}

pub fn sys_fork() -> isize {
    let current_task = task::current_task().unwrap();
    let new_task = current_task.fork();
    let new_pid = new_task.pid.0;
    // modify trap context of new_task, because it returns immediately after switching
    let trap_cx = new_task.inner_exclusive_access().get_trap_cx();
    // we do not have to move to next instruction since we have done it before
    // for child process, fork returns 0
    trap_cx.x[10] = 0;
    // add new task to scheduler
    task::add_task(new_task);
    new_pid as isize
}

pub fn sys_exec(path: *const *const str, args: *const *const [*const str]) -> isize {
    let token = task::current_user_token();
    let path = if let Ok(path) = String::from_utf8(memory::translate_bytes(token, path)) {
        path
    } else {
        return -1;
    };
    let args = memory::translate_bytes_slice(token, args);
    if let Some(app_inode) = fs::open_file(path.as_str(), crate::fs::OpenFlag::RDONLY) {
        let all_data = app_inode.read_all();
        let task = current_task().unwrap();
        task.exec(all_data.as_slice(), args);
        0
    } else {
        -1
    }
}

/// If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, return -2.
pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let task = task::current_task().unwrap();
    // find a child process

    // ---- access current TCB exclusively
    let mut inner = task.inner_exclusive_access();
    if !inner
        .children
        .iter()
        .any(|p| pid == -1 || pid as usize == p.getpid())
    {
        return -1;
        // ---- release current PCB
    }
    let pair = inner.children.iter().enumerate().find(|(_, p)| {
        // ++++ temporarily access child PCB lock exclusively
        p.inner_exclusive_access().is_zombie() && (pid == -1 || pid as usize == p.getpid())
        // ++++ release child PCB
    });
    if let Some((idx, _)) = pair {
        let child = inner.children.remove(idx);
        // confirm that child will be deallocated after removing from children list
        assert_eq!(Arc::strong_count(&child), 1);
        let found_pid = child.getpid();
        // ++++ temporarily access child TCB exclusively
        let exit_code = child.inner_exclusive_access().exit_code;
        // ++++ release child PCB
        *memory::translate_ref_mut(inner.memory_set.token(), exit_code_ptr) = exit_code;
        found_pid as isize
    } else {
        -2
    }
}
pub fn sys_kill(pid: usize, signum: i32) -> isize {
    if let Some(task) = task::pid2task(pid) {
        if let Some(flag) = SignalFlags::from_bits(1 << signum) {
            // insert the signal if legal
            let mut task_ref = task.inner_exclusive_access();
            if task_ref.signals.contains(flag) {
                return -1;
            }
            task_ref.signals.insert(flag);
            0
        } else {
            -1
        }
    } else {
        -1
    }
}
fn check_sigaction_error(signal: SignalFlags, action: usize, old_action: usize) -> bool {
    action == 0 || old_action == 0 || signal == SignalFlags::KILL || signal == SignalFlags::STOP
}
pub fn sys_sigaction(
    signum: u64,
    action: *const SignalAction,
    old_action: *mut SignalAction,
) -> isize {
    let token = task::current_user_token();
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    // if signum as usize > MAX_SIG {
    //     return -1;
    // }
    if let Some(flag) = SignalFlags::from_bits(1 << signum) {
        if check_sigaction_error(flag, action as usize, old_action as usize) {
            return -1;
        }
        let prev_action = inner.signal_actions.table[signum as usize].clone();
        *memory::translate_ref_mut(token, old_action) = prev_action;
        inner.signal_actions.table[signum as usize] = memory::translate_ref(token, action).clone();
        0
    } else {
        -1
    }
}
pub fn sys_sigprocmask(mask: u32) -> isize {
    if let Some(task) = current_task() {
        let mut inner = task.inner_exclusive_access();
        let old_mask = inner.signal_mask;
        if let Some(flag) = SignalFlags::from_bits(mask) {
            inner.signal_mask = flag;
            return old_mask.bits() as isize;
        }
    }
    -1
}

pub fn sys_sigreturn() -> isize {
    if let Some(task) = current_task() {
        let mut inner = task.inner_exclusive_access();
        inner.handling_sig = None;
        // restore the trap context
        let trap_ctx = inner.get_trap_cx();
        *trap_ctx = inner.trap_ctx_backup.to_owned().unwrap();
        // Here we return the value of a0 in the trap_ctx,
        // otherwise it will be overwritten after we trap
        // back to the original execution of the application.
        trap_ctx.x[10] as isize
    } else {
        -1
    }
}
