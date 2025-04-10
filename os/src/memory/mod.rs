//! Memory management implementation
//!
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//!
//! Every task or process has a memory_set to control its virtual memory.

mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
pub use frame_allocator::{FrameTracker, frame_alloc};
pub use memory_set::remap_test;
pub use memory_set::{KERNEL_SPACE, MapPermission, MemorySet};
use page_table::{PageTable, PageTableEntryFlags};
pub use page_table::{PageTableEntry, translate_sized, translate_str, translated_refmut};

#[deny(dead_code)]
/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    heap_allocator::init();
    frame_allocator::init();
    KERNEL_SPACE.exclusive_access().activate();
}
