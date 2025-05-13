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
pub use manager::{add_task, fetch_task};
pub use pid::{KernelStack, PidHandle, pid_alloc};
pub use processor::{
    current_task, current_trap_cx, current_user_token, run_tasks, schedule, take_current_task,
};
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

mod cfg {
    pub use config::INIT_PROC_NAME;
    pub use config::memory::{KERNEL_STACK_SIZE, PAGE_SIZE, TRAMPOLINE, TRAP_CONTEXT};
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
        let inode = fs::open_file(cfg::INIT_PROC_NAME, crate::fs::OpenFlag::RDONLY).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice())
    });
    unsafe {
        INITPROC = Some(init_proc.clone());
    }
    add_task(unsafe { INITPROC.as_ref() }.unwrap().clone());
}
