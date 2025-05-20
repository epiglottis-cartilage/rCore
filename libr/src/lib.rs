#![no_std]
#![feature(linkage)]
#![feature(alloc_error_handler)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

pub use config::{
    fs::OpenFlag,
    signal::{SignalAction, SignalID},
    syscall::SyscallID,
};
use linked_list_allocator::LockedHeap;
use syscall::*;

const USER_HEAP_SIZE: usize = 16384;

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}
unsafe extern "Rust" {
    safe fn main(args: &[&[u8]]) -> i32;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start(args: *const &[&[u8]]) -> ! {
    unsafe {
        #[allow(static_mut_refs)]
        HEAP.lock().init(HEAP_SPACE.as_ptr() as _, USER_HEAP_SIZE);
    }
    exit(main(
        unsafe { args.as_ref() }.map(|arg| *arg).unwrap_or(&[]),
    ));
}
pub fn dup(fd: usize) -> isize {
    sys_dup(fd)
}
pub fn open(name: &str, flags: OpenFlag) -> isize {
    sys_open(&name, flags)
}
pub fn close(fd: usize) -> isize {
    sys_close(fd)
}
pub fn pipe() -> Option<(usize, usize)> {
    let mut pipefd = (0, 0);
    match sys_pipe(&mut pipefd.0, &mut pipefd.1) {
        0 => Some(pipefd),
        -1 => None,
        _ => panic!("unexpected return value: {}", -1),
    }
}
pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}
pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> ! {
    sys_exit(exit_code);
}
pub fn r#yield() {
    sys_yield();
}
pub fn get_time() -> usize {
    sys_get_time()
}
pub fn getpid() -> usize {
    sys_get_pid()
}
pub fn sbrk(delta: isize) -> isize {
    sys_sbrk(delta)
}
pub fn fork() -> isize {
    sys_fork()
}
pub fn exec(path: &str, argv: &[&str]) -> isize {
    sys_exec(&path, &argv)
}
pub fn wait(exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(-1, exit_code as *mut _) {
            -2 => {
                r#yield();
            }
            // -1 or a real pid
            exit_pid => return exit_pid,
        }
    }
}
pub fn waitpid(pid: usize, exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(pid as isize, exit_code as *mut _) {
            -2 => {
                r#yield();
            }
            // -1 or a real pid
            exit_pid => return exit_pid,
        }
    }
}
pub fn sleep(period_ms: usize) {
    let start = get_time();
    while get_time() < start + period_ms {
        r#yield();
    }
}
pub fn kill(pid: usize, signum: SignalID) -> isize {
    sys_kill(pid, signum)
}

pub fn sigaction(
    signum: SignalID,
    action: Option<&SignalAction>,
    old_action: Option<&mut SignalAction>,
) -> isize {
    sys_sigaction(
        signum,
        action.map_or(core::ptr::null(), |a| a),
        old_action.map_or(core::ptr::null_mut(), |a| a),
    )
}

pub fn sigprocmask(mask: u32) -> isize {
    sys_sigprocmask(mask)
}

pub fn sigreturn() -> isize {
    sys_sigreturn()
}
