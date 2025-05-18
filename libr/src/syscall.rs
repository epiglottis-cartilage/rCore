use config::{fs::OpenFlag, syscall::SyscallID};
use core::arch::asm;

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
pub(crate) fn sys_dup(fd: usize) -> isize {
    syscall(SyscallID::Dup, [fd, 0, 0])
}
pub(crate) fn sys_open(path: &&str, flag: OpenFlag) -> isize {
    syscall(SyscallID::Open, [path as *const _ as _, flag.bits(), 0])
}
pub(crate) fn sys_close(fd: usize) -> isize {
    syscall(SyscallID::Close, [fd, 0, 0])
}
pub(crate) fn sys_pipe(pipe_read: &mut usize, pipe_write: &mut usize) -> isize {
    syscall(
        SyscallID::Pipe,
        [pipe_read as *const _ as _, pipe_write as *const _ as _, 0],
    )
}
pub(crate) fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(
        SyscallID::Read,
        [fd, buffer.as_mut_ptr() as _, buffer.len()],
    )
}
pub(crate) fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SyscallID::Write, [fd, buffer.as_ptr() as _, buffer.len()])
}
pub(crate) fn sys_exit(exit_code: i32) -> ! {
    syscall(SyscallID::Exit, [exit_code as _, 0, 0]);
    panic!("sys_exit never returns!");
}
pub(crate) fn sys_yield() -> isize {
    syscall(SyscallID::Yield, [0, 0, 0])
}
pub(crate) fn sys_get_time() -> usize {
    syscall(SyscallID::GetTime, [0, 0, 0]).cast_unsigned()
}
pub fn sys_get_pid() -> usize {
    syscall(SyscallID::GetPid, [0, 0, 0]).cast_unsigned()
}

pub(crate) fn sys_sbrk(delta: isize) -> isize {
    syscall(SyscallID::Sbrk, [delta.cast_unsigned(), 0, 0])
}
pub(crate) fn sys_fork() -> isize {
    syscall(SyscallID::Fork, [0, 0, 0])
}
pub(crate) fn sys_exec(path: &&str, argv: &&[&str]) -> isize {
    syscall(
        SyscallID::Exec,
        [path as *const _ as _, argv as *const _ as _, 0],
    )
}
pub(crate) fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    syscall(SyscallID::WaitPid, [pid as _, exit_code as _, 0])
}
