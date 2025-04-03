//! The global allocator

use linked_list_allocator::LockedHeap;

#[global_allocator]
/// heap allocator instance
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
/// panic when heap allocation error occurs
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

use config::memory::KERNEL_HEAP_SIZE;
use core::mem::MaybeUninit;
/// heap space ([u8; KERNEL_HEAP_SIZE])
static mut HEAP_SPACE: [MaybeUninit<u8>; KERNEL_HEAP_SIZE] =
    unsafe { MaybeUninit::uninit().assume_init() };

/// initiate heap allocator
pub fn init_heap() {
    #[allow(static_mut_refs)]
    HEAP_ALLOCATOR
        .lock()
        .init_from_slice(unsafe { HEAP_SPACE.as_mut_slice() });
}
