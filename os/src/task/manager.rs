//! Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::{UpSafeCell, UpSafeLazyCell};
use alloc::collections::VecDeque;
use alloc::collections::btree_map::BTreeMap;
use alloc::sync::Arc;

/// A array of `TaskControlBlock` that is thread-safe
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

static TASK_MANAGER: UpSafeLazyCell<UpSafeCell<TaskManager>> =
    unsafe { UpSafeLazyCell::new(|| UpSafeCell::new(TaskManager::new())) };
static PID2TCB: UpSafeLazyCell<UpSafeCell<BTreeMap<usize, Arc<TaskControlBlock>>>> =
    unsafe { UpSafeLazyCell::new(|| UpSafeCell::new(BTreeMap::new())) };

///Interface offered to add task
pub fn add_task(task: Arc<TaskControlBlock>) {
    PID2TCB
        .borrow_mut()
        .insert(task.getpid(), Arc::clone(&task));
    TASK_MANAGER.borrow_mut().add(task);
}
///Interface offered to pop the first task
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.borrow_mut().fetch()
}
pub fn pid2task(pid: usize) -> Option<Arc<TaskControlBlock>> {
    let map = PID2TCB.borrow_mut();
    map.get(&pid).map(Arc::clone)
}

pub fn remove_from_pid2task(pid: usize) {
    let mut map = PID2TCB.borrow_mut();
    if map.remove(&pid).is_none() {
        panic!("cannot find pid {} in pid2task!", pid);
    }
}
