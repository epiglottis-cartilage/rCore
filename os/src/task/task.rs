//! Types related to task management

use super::TaskContext;
use crate::memory::{KERNEL_SPACE, MapPermission, MemorySet, PhysPageNum, VirtAddr};
use crate::trap::{TrapContext, trap_handler};
use config::memory::{TRAP_CONTEXT, kernel_stack_position};

pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub context: TaskContext,
    pub memory_set: MemorySet,
    pub trap_cx_ppn: PhysPageNum,
    #[allow(unused)]
    pub base_size: usize,
    pub heap_bottom: usize,
    pub program_brk: usize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

impl TaskControlBlock {
    pub fn new(elf_data: &[u8], app_id: usize) -> Self {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        let trap_location = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        let status = TaskStatus::Ready;
        // map a kernel-stack in kernel space
        let (kernel_stack_bottom, kernel_stack_top) = kernel_stack_position(app_id);
        KERNEL_SPACE.exclusive_access().insert_framed_area(
            kernel_stack_bottom.into(),
            kernel_stack_top.into(),
            MapPermission::R | MapPermission::W,
        );
        let task_control_block = Self {
            status,
            context: TaskContext::goto_restore(kernel_stack_top),
            memory_set,
            trap_cx_ppn: trap_location,
            base_size: user_sp,
            heap_bottom: user_sp,
            program_brk: user_sp,
        };
        // prepare TrapContext in user space
        let trap_cx = task_control_block.get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.exclusive_access().token().bits(),
            kernel_stack_top,
            trap_handler as usize,
        );
        println!("{:#X?}", trap_cx);
        task_control_block
    }

    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.as_mut()
    }

    pub fn get_user_token(&self) -> riscv::register::satp::Satp {
        self.memory_set.token()
    }
}
