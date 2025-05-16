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
pub use frame_allocator::{FrameTracker, frame_alloc, frame_dealloc};
pub use memory_set::remap_test;
pub use memory_set::{KERNEL_SPACE, MapPermission, MemorySet, kernel_token};
pub use page_table::{PageTable, PageTableDirect, PageTableEntryFlags};
pub use page_table::{
    PageTableEntry, UserBuffer, translate_necked_slice, translate_ref, translate_ref_mut,
    translate_sized, translate_slice, translate_str, translate_str_slice,
};

use config::memory as cfg;

#[deny(dead_code)]
/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    heap_allocator::init();
    frame_allocator::init();
    memory_set::init();
    unsafe { KERNEL_SPACE.as_ref() }
        .unwrap()
        .exclusive_access()
        .activate();
}
