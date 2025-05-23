//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use core::ptr::addr_of;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    ///Add a task to `TaskManager`
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    ///Remove the first task and return it,or `None` if `TaskManager` is empty
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
}

static mut TASK_MANAGER: Option<UPSafeCell<TaskManager>> = None;
#[deny(dead_code)]
///Initialize the task manager
pub fn init() {
    let task_manager = unsafe { UPSafeCell::new(TaskManager::new()) };
    log::debug!("init TASK_MANAGER at {:#p}", addr_of!(TASK_MANAGER));
    unsafe {
        TASK_MANAGER = Some(task_manager);
    }
}
///Interface offered to add task
pub fn add_task(task: Arc<TaskControlBlock>) {
    unsafe { TASK_MANAGER.as_ref() }
        .unwrap()
        .exclusive_access()
        .add(task);
}
///Interface offered to pop the first task
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    unsafe { TASK_MANAGER.as_ref() }
        .unwrap()
        .exclusive_access()
        .fetch()
}
