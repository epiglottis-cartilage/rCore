//! Task management implementation
//!
//! Everything about task management, like starting and switching tasks is
//! implemented here.
//!
//! A single global instance of [`TaskManager`] called `TASK_MANAGER` controls
//! all the tasks in the operating system.
//!
//! Be careful when you see `__switch` ASM function in `switch.S`. Control flow around this function
//! might not be what you expect.

mod context;
mod manager;
mod pid;
mod processor;
mod switch;

#[allow(clippy::module_inception)]
mod task;

use crate::fs;
use crate::sbi::shutdown;
use alloc::sync::Arc;
use context::TaskContext;
pub use manager::{add_task, fetch_task, pid2task, remove_from_pid2task};
pub use pid::{KernelStack, PidHandle, pid_alloc};
pub use processor::{
    current_task, current_trap_cx, current_user_token, run_tasks, schedule, take_current_task,
};
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

mod cfg {
    pub use config::INIT_PROC_NAME;
    pub use config::memory::{KERNEL_STACK_SIZE, PAGE_SIZE, TRAMPOLINE, TRAP_CONTEXT};
    pub use config::signal::{SIG_NUM, SignalActions, SignalFlags, SignalID};
}

/// Suspend the current 'Running' task and run the next task in task list.
pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().unwrap();

    // ---- access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    // ---- release current PCB

    // push back to ready queue.
    add_task(task);
    // jump to scheduling cycle
    schedule(task_cx_ptr);
}

/// pid of usertests app in make run TEST=1
pub const IDLE_PID: usize = 0;

/// Exit the current 'Running' task and run the next task in task list.
pub fn exit_current_and_run_next(exit_code: i32) {
    // take from Processor
    let task = take_current_task().unwrap();

    let pid = task.getpid();
    if pid == IDLE_PID {
        log::debug!(
            "[kernel] Idle process exit with exit_code {} ...",
            exit_code
        );
        if exit_code != 0 {
            shutdown(true)
        } else {
            shutdown(false)
        }
    }

    // remove from pid2task
    remove_from_pid2task(task.getpid());
    // **** access current TCB exclusively
    let mut inner = task.inner_exclusive_access();
    // Change status to Zombie
    inner.task_status = TaskStatus::Zombie;
    // Record exit code
    inner.exit_code = exit_code;
    // do not move to its parent but under initproc

    // ++++++ access initproc TCB exclusively
    {
        let mut initproc_inner = unsafe { INITPROC.as_ref() }
            .unwrap()
            .inner_exclusive_access();
        for child in inner.children.iter() {
            child.inner_exclusive_access().parent =
                Some(Arc::downgrade(unsafe { INITPROC.as_ref() }.unwrap()));
            initproc_inner.children.push(child.clone());
        }
    }
    // ++++++ release parent PCB

    inner.children.clear();
    // deallocate user space
    inner.memory_set.recycle_data_pages();
    drop(inner);
    // **** release current PCB
    // drop task manually to maintain rc correctly
    drop(task);
    // we do not have to save task context
    let mut _unused = TaskContext::zero_init();
    schedule(&mut _unused as *mut _);
}

///Globle process that init user shell
#[unsafe(link_section = ".data")]
static mut INITPROC: Option<Arc<TaskControlBlock>> = None;
///Add init process to the manager

#[deny(dead_code)]
pub fn init() {
    pid::init();
    manager::init();
    processor::init();
    let init_proc = Arc::new({
        let inode = fs::open_file(cfg::INIT_PROC_NAME, fs::OpenFlag::RDONLY).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice())
    });
    unsafe {
        INITPROC = Some(init_proc.clone());
    }
    add_task(unsafe { INITPROC.as_ref() }.unwrap().clone());
}

pub fn check_signals_error_of_current() -> Option<(i32, &'static str)> {
    let task = current_task().unwrap();
    task.inner_exclusive_access().signals.check_error()
}

pub fn current_add_signal(signal: cfg::SignalFlags) {
    let task = current_task().unwrap();
    task.inner_exclusive_access().signals |= signal;
}

fn call_kernel_signal_handler(signal: cfg::SignalFlags) {
    let task = current_task().unwrap();
    let mut task_inner = task.inner_exclusive_access();
    match signal {
        cfg::SignalFlags::STOP => {
            task_inner.frozen = true;
            task_inner.signals ^= cfg::SignalFlags::STOP;
        }
        cfg::SignalFlags::CONT => {
            if task_inner.signals.contains(cfg::SignalFlags::CONT) {
                task_inner.signals ^= cfg::SignalFlags::CONT;
                task_inner.frozen = false;
            }
        }
        _ => {
            log::info!(
                "[K] call_kernel_signal_handler:: current task sigflag {:?}",
                task_inner.signals
            );
            task_inner.killed = true;
        }
    }
}

fn call_user_signal_handler(sig: cfg::SignalID, signal: cfg::SignalFlags) {
    let task = current_task().unwrap();
    let mut task_inner = task.inner_exclusive_access();

    let handler = task_inner.signal_actions.table[sig as usize].handler;
    if handler != 0 {
        // user handler

        // handle flag
        task_inner.handling_sig = Some(sig);
        task_inner.signals ^= signal;

        // backup trapframe
        let trap_ctx = task_inner.get_trap_cx();
        task_inner.trap_ctx_backup = Some(trap_ctx.clone());

        // modify trapframe
        trap_ctx.sepc = handler as usize;

        // put args (a0)
        trap_ctx.x[10] = sig as usize;
    } else {
        // default action
        println!("[K] task/call_user_signal_handler: default action: ignore it or kill process");
    }
}

fn check_pending_signals() {
    for sig in 0..cfg::SIG_NUM {
        let signal = cfg::SignalFlags::from_bits(1 << sig).unwrap();
        let sig: cfg::SignalID = sig.into();
        let task = current_task().unwrap();
        let task_inner = task.inner_exclusive_access();
        if task_inner.signals.contains(signal) && (!task_inner.signal_mask.contains(signal)) {
            let mut masked = true;
            match task_inner.handling_sig {
                None => masked = false,
                Some(handling_sig) => {
                    if !task_inner.signal_actions.table[handling_sig as usize]
                        .mask
                        .contains(signal)
                    {
                        masked = false;
                    }
                }
            }
            if !masked {
                drop(task_inner);
                drop(task);
                if sig.job_of_kernel() {
                    call_kernel_signal_handler(signal);
                } else {
                    call_user_signal_handler(sig, signal);
                    return;
                }
            }
        }
    }
}

pub fn handle_signals() {
    loop {
        check_pending_signals();
        let (frozen, killed) = {
            let task = current_task().unwrap();
            let task_inner = task.inner_exclusive_access();
            (task_inner.frozen, task_inner.killed)
        };
        if !frozen || killed {
            break;
        }
        suspend_current_and_run_next();
    }
}
