//! Types related to task management

use super::TaskContext;
use super::cfg::{SignalActions, SignalFlags, SignalID, TRAP_CONTEXT};
use super::{KernelStack, PidHandle, pid_alloc};
use crate::fs::{Stderr, Stdin, Stdout};
use crate::memory::{self, KERNEL_SPACE, MemorySet, PageTableDirect, PhysPageNum, VirtAddr};
use crate::sync::UpSafeCell;
use crate::trap::{TrapContext, trap_handler};
use alloc::sync::{Arc, Weak};
use alloc::{vec, vec::Vec};
use core::cell::RefMut;

pub struct TaskControlBlock {
    // immutable
    pub pid: PidHandle,
    pub kernel_stack: KernelStack,
    // mutable
    inner: UpSafeCell<TaskControlBlockInner>,
}

pub struct TaskControlBlockInner {
    pub trap_cx_ppn: PhysPageNum,
    pub base_size: usize,
    // pub heap_bottom: usize,
    // pub program_brk: usize,
    pub task_cx: TaskContext,
    pub task_status: TaskStatus,
    pub memory_set: MemorySet,
    pub parent: Option<Weak<TaskControlBlock>>,
    pub children: Vec<Arc<TaskControlBlock>>,
    pub exit_code: i32,
    pub fd_table: Vec<Option<Arc<dyn crate::fs::File + Send + Sync>>>,
    pub signals: SignalFlags,
    pub signal_mask: SignalFlags,
    // the signal which is being handling
    pub handling_sig: Option<SignalID>,
    // Signal actions
    pub signal_actions: SignalActions,
    // if the task is killed
    pub killed: bool,
    // if the task is frozen by a signal
    pub frozen: bool,
    pub trap_ctx_backup: Option<TrapContext>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Ready,
    Running,
    Zombie,
}

impl TaskControlBlockInner {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.as_mut()
    }
    pub fn get_user_token(&self) -> PageTableDirect {
        self.memory_set.token()
    }
    fn get_status(&self) -> TaskStatus {
        self.task_status
    }
    pub fn is_zombie(&self) -> bool {
        self.get_status() == TaskStatus::Zombie
    }
    pub fn alloc_fd(&mut self) -> usize {
        if let Some(fd) = self.fd_table.iter().position(|fd| fd.is_none()) {
            fd
        } else {
            self.fd_table.push(None);
            self.fd_table.len() - 1
        }
    }
}
impl TaskControlBlock {
    pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskControlBlockInner> {
        self.inner.borrow_mut()
    }
    pub fn new(elf_data: &[u8]) -> Self {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        // alloc a pid and a kernel stack in kernel space
        let pid_handle = pid_alloc();
        let kernel_stack = KernelStack::new(&pid_handle);
        let kernel_stack_top = kernel_stack.get_top();
        // push a task context which goes to trap_return to the top of kernel stack
        let task_control_block = Self {
            pid: pid_handle,
            kernel_stack,
            inner: unsafe {
                UpSafeCell::new(TaskControlBlockInner {
                    trap_cx_ppn,
                    base_size: user_sp,
                    task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                    task_status: TaskStatus::Ready,
                    memory_set,
                    parent: None,
                    children: Vec::new(),
                    exit_code: 0,
                    fd_table: vec![
                        // 0 -> stdin
                        Some(Arc::new(Stdin)),
                        // 1 -> stdout
                        Some(Arc::new(Stdout)),
                        // 2 -> stderr
                        Some(Arc::new(Stderr)),
                    ],
                    signals: SignalFlags::empty(),
                    signal_mask: SignalFlags::empty(),
                    handling_sig: None,
                    signal_actions: SignalActions::default(),
                    killed: false,
                    frozen: false,
                    trap_ctx_backup: None,
                })
            },
        };
        // prepare TrapContext in user space
        let trap_cx = task_control_block.inner_exclusive_access().get_trap_cx();
        let kernel_ppn: PhysPageNum = unsafe { KERNEL_SPACE.as_ref() }
            .unwrap()
            .borrow_mut()
            .token()
            .into();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            kernel_ppn.into(),
            kernel_stack_top,
            trap_handler as usize,
        );
        // trap_cx.x[10] = argv_ptr;
        task_control_block
    }
    pub fn exec(&self, elf_data: &[u8], args: Vec<Vec<u8>>) {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, mut user_sp, entry_point) = MemorySet::from_elf(elf_data);

        // argv[0] : str  <-----|
        // ...     : str  <---| |
        // argv[n] : str  <-| | |
        // argv[n] : &str --| | |
        // ...     : &str ----| |
        // argv[0] : &str ------|
        // // argv    : &[&str;n]

        // push arguments on user stack
        for arg in args.iter() {
            user_sp -= arg.len() as usize;
            // copying bytes from kernel space to user space
            let mut arg_slice = arg.as_slice();

            memory::translate_sized(memory_set.token(), user_sp as *mut u8, arg.len())
                .into_iter()
                .for_each(|dst| {
                    let (src, remain) = arg_slice.split_at(dst.len());
                    dst.copy_from_slice(src);
                    arg_slice = remain;
                });
        }
        let mut arg_ptr = user_sp;

        // align to &str
        user_sp -= user_sp % align_of::<&str>();
        for arg in args.iter().rev() {
            user_sp -= core::mem::size_of::<usize>();
            let len = memory::translate_ref_mut(memory_set.token(), user_sp as *mut usize);
            *len = arg.len();
            user_sp -= core::mem::size_of::<&u8>();
            let ptr = memory::translate_ref_mut(memory_set.token(), user_sp as *mut *const u8);
            *ptr = arg_ptr as _;
            arg_ptr += arg.len() as usize;
        }
        let args_ptr = user_sp;

        // align to &[&str]
        user_sp -= user_sp % align_of::<&[&str]>();
        user_sp -= core::mem::size_of::<usize>();
        let len = memory::translate_ref_mut(memory_set.token(), user_sp as *mut usize);
        *len = args.len();
        user_sp -= core::mem::size_of::<&u8>();
        let ptr = memory::translate_ref_mut(memory_set.token(), user_sp as *mut *const u8);
        *ptr = args_ptr as _;
        let argv_ptr = user_sp;

        // make the user_sp aligned to 8
        user_sp -= user_sp % core::mem::size_of::<usize>();

        // **** access current TCB exclusively
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();

        // **** access current TCB exclusively
        let mut inner = self.inner_exclusive_access();
        // substitute memory_set
        inner.memory_set = memory_set;
        // update trap_cx ppn
        inner.trap_cx_ppn = trap_cx_ppn;
        // initialize trap_cx
        let mut trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            unsafe { KERNEL_SPACE.as_ref() }
                .unwrap()
                .borrow_mut()
                .token(),
            self.kernel_stack.get_top(),
            trap_handler as usize,
        );
        trap_cx.x[10] = argv_ptr;
        *inner.get_trap_cx() = trap_cx;
        // **** release current PCB
    }
    pub fn fork(self: &Arc<Self>) -> Arc<Self> {
        // ---- access parent PCB exclusively
        let mut parent_inner = self.inner_exclusive_access();
        // copy user space(include trap context)
        let memory_set = MemorySet::from_existed_user(&parent_inner.memory_set);
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        // alloc a pid and a kernel stack in kernel space
        let pid_handle = pid_alloc();
        let kernel_stack = KernelStack::new(&pid_handle);
        let kernel_stack_top = kernel_stack.get_top();
        let new_fd_table = parent_inner
            .fd_table
            .iter()
            .map(|fd| fd.as_ref().map(|fd| fd.clone()))
            .collect();
        let task_control_block = Arc::new(TaskControlBlock {
            pid: pid_handle,
            kernel_stack,
            inner: unsafe {
                UpSafeCell::new(TaskControlBlockInner {
                    trap_cx_ppn,
                    base_size: parent_inner.base_size,
                    task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                    task_status: TaskStatus::Ready,
                    memory_set,
                    parent: Some(Arc::downgrade(self)),
                    children: Vec::new(),
                    exit_code: 0,
                    fd_table: new_fd_table,
                    signals: SignalFlags::empty(),
                    signal_mask: SignalFlags::empty(),
                    handling_sig: None,
                    signal_actions: SignalActions::default(),
                    killed: false,
                    frozen: false,
                    trap_ctx_backup: None,
                })
            },
        });
        // add child
        parent_inner.children.push(task_control_block.clone());
        // modify kernel_sp in trap_cx
        // **** access children PCB exclusively
        let trap_cx = task_control_block.inner_exclusive_access().get_trap_cx();
        trap_cx.kernel_sp = kernel_stack_top;
        // return
        task_control_block
        // ---- release parent PCB automatically
        // **** release children PCB automatically
    }
    pub fn getpid(&self) -> usize {
        self.pid.0
    }
}
