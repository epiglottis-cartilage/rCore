use riscv::register::satp::Mode;
pub const VA_MODE: Mode = Mode::Sv39;
/// physical address
pub const PA_WIDTH: usize = 56;
pub const VA_WIDTH: usize = match VA_MODE {
    Mode::Bare => panic!("Bare mode is not supported in VirtAddr"),
    Mode::Sv39 => 39,
    Mode::Sv48 => 48,
    Mode::Sv57 => 57,
    Mode::Sv64 => 64,
};

pub const PPN_WIDTH: usize = PA_WIDTH - PAGE_SIZE_BITS;
pub const VPN_WIDTH: usize = VA_WIDTH - PAGE_SIZE_BITS;

pub const MEMORY_END: usize = super::qemu::MEMORY_END;

pub const KERNEL_HEAP_SIZE: usize = 0xF0_000;

pub const PAGE_SIZE: usize = 1 << PAGE_SIZE_BITS;
pub const PAGE_SIZE_BITS: usize = 12;

pub const MMIO: &[(usize, usize)] = super::qemu::MMIO;

pub const USER_STACK_SIZE: usize = PAGE_SIZE * 2;
pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE * 2;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;
/// Return (bottom, top) of a kernel stack in kernel space.
pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}
