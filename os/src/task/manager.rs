//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::collections::btree_map::BTreeMap;
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
static mut PID2TCB: Option<UPSafeCell<BTreeMap<usize, Arc<TaskControlBlock>>>> = None;
#[deny(dead_code)]
///Initialize the task manager
pub fn init() {
    let task_manager = unsafe { UPSafeCell::new(TaskManager::new()) };
    log::debug!("init TASK_MANAGER at {:#p}", addr_of!(TASK_MANAGER));
    unsafe {
        TASK_MANAGER = Some(task_manager);
    }
    let pid2task = unsafe { UPSafeCell::new(BTreeMap::new()) };
    log::debug!("init PID2TASK at {:#p}", addr_of!(PID2TCB));
    unsafe {
        PID2TCB = Some(pid2task);
    }
}
///Interface offered to add task
pub fn add_task(task: Arc<TaskControlBlock>) {
    unsafe { PID2TCB.as_ref() }
        .unwrap()
        .exclusive_access()
        .insert(task.getpid(), Arc::clone(&task));
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
pub fn pid2task(pid: usize) -> Option<Arc<TaskControlBlock>> {
    let map = unsafe { PID2TCB.as_ref() }.unwrap().exclusive_access();
    map.get(&pid).map(Arc::clone)
}

pub fn remove_from_pid2task(pid: usize) {
    let mut map = unsafe { PID2TCB.as_ref() }.unwrap().exclusive_access();
    if map.remove(&pid).is_none() {
        panic!("cannot find pid {} in pid2task!", pid);
    }
}
