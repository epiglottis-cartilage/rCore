use crate::memory::{KERNEL_SPACE, MapPermission, VirtAddr};
use crate::sync::UPSafeCell;
use alloc::vec::Vec;
// use lazy_static::lazy_static;

struct PidAllocator {
    current: usize,
    recycled: Vec<usize>,
}

impl PidAllocator {
    pub fn new() -> Self {
        Self {
            current: 0,
            recycled: Vec::new(),
        }
    }
    pub fn alloc(&mut self) -> PidHandle {
        PidHandle(match self.recycled.pop() {
            Some(pid) => pid,
            None => {
                self.current += 1;
                self.current
            }
        })
    }
    pub fn dealloc(&mut self, pid: usize) {
        if pid < self.current {
            self.recycled.push(pid);
        } else if pid == self.current {
            self.current -= 1;
        } else {
            panic!("PID {} has not been allocated!", pid);
        }
    }
}

pub struct PidHandle(pub usize);
impl Drop for PidHandle {
    fn drop(&mut self) {
        PID_ALLOCATOR.exclusive_access().dealloc(self.0);
    }
}

#[unsafe(link_section = ".data")]
static PID_ALLOCATOR: UPSafeCell<PidAllocator> =
    unsafe { core::mem::transmute([0x01u8; size_of::<UPSafeCell<PidAllocator>>()]) };

#[deny(unused)]
/// Initialize the PID allocator
pub fn init() {
    let pid_allocator = unsafe { UPSafeCell::new(PidAllocator::new()) };
    log::debug!(
        "init PID_ALLOCATOR at {:#p}",
        core::ptr::addr_of!(PID_ALLOCATOR)
    );
    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of!(PID_ALLOCATOR) as _, pid_allocator);
    };
}

pub fn pid_alloc() -> PidHandle {
    PID_ALLOCATOR.exclusive_access().alloc()
}

/// Return (bottom, top) of a kernel stack in kernel space.
pub fn kernel_stack_position(pid: usize) -> (usize, usize) {
    use config::memory::{KERNEL_STACK_SIZE, PAGE_SIZE, TRAMPOLINE};
    let top = TRAMPOLINE - pid * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}

///Kernelstack for each app
pub struct KernelStack {
    pid: usize,
}

impl KernelStack {
    ///Create a kernelstack from pid
    pub fn new(pid_handle: &PidHandle) -> Self {
        let pid = pid_handle.0;
        let (kernel_stack_bottom, kernel_stack_top) = kernel_stack_position(pid);
        KERNEL_SPACE.exclusive_access().insert_framed_area(
            kernel_stack_bottom.into(),
            kernel_stack_top.into(),
            MapPermission::R | MapPermission::W,
        );
        KernelStack { pid: pid_handle.0 }
    }
    #[allow(unused)]
    ///Push a value on top of kernelstack
    pub fn push_on_top<T>(&self, value: T) -> *mut T
    where
        T: Sized,
    {
        let kernel_stack_top = self.get_top();
        let ptr_mut = (kernel_stack_top - core::mem::size_of::<T>()) as *mut T;
        unsafe {
            *ptr_mut = value;
        }
        ptr_mut
    }
    ///Get the value on the top of kernelstack
    pub fn get_top(&self) -> usize {
        let (_, kernel_stack_top) = kernel_stack_position(self.pid);
        kernel_stack_top
    }
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        let (kernel_stack_bottom, _) = kernel_stack_position(self.pid);
        let kernel_stack_bottom_va: VirtAddr = kernel_stack_bottom.into();
        KERNEL_SPACE
            .exclusive_access()
            .pop_area_with_start_vpn(kernel_stack_bottom_va.into());
    }
}
