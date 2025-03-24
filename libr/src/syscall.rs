use core::arch::asm;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallID {
    Read = 63,
    Write = 64,
    Exit = 93,
    Yield = 124,
}
impl From<usize> for SyscallID {
    fn from(value: usize) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

fn syscall(id: SyscallID, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id as usize,
        );
    }
    ret
}
pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(
        SyscallID::Read,
        [fd, buffer.as_mut_ptr() as usize, buffer.len()],
    )
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(
        SyscallID::Write,
        [fd, buffer.as_ptr() as usize, buffer.len()],
    )
}

pub fn sys_exit(exit_code: i32) -> ! {
    syscall(SyscallID::Exit, [exit_code as usize, 0, 0]);
    panic!("sys_exit never returns!");
}
pub fn sys_yield() -> isize {
    syscall(SyscallID::Yield, [0, 0, 0])
}
