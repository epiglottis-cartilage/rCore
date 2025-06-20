//! File and filesystem-related syscalls

use alloc::string::String;

use crate::fs;
use crate::memory;
use crate::task;

pub fn sys_dup(fd: usize) -> isize {
    let task = task::current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    let src;
    if let Some(fd) = inner.fd_table.get(fd).map_or(None, |fd| fd.as_ref()) {
        src = fd.clone();
    } else {
        return -1;
    }
    let new_fd = inner.alloc_fd();
    inner.fd_table[new_fd] = Some(src);
    new_fd as isize
}

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = task::current_user_token();
    let task = task::current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        if !file.writable() {
            return -1;
        }
        let file = file.clone();
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        file.write(memory::UserBuffer::new(memory::translate_sized(
            token.into(),
            buf,
            len,
        ))) as isize
    } else {
        -1
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = task::current_user_token();
    let task = task::current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        if !file.readable() {
            return -1;
        }
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        file.read(memory::UserBuffer::new(memory::translate_sized(
            token.into(),
            buf,
            len,
        ))) as isize
    } else {
        -1
    }
}

pub fn sys_open(path: *const *const str, flags: usize) -> isize {
    let task = task::current_task().unwrap();
    let token = task::current_user_token();
    let path = if let Ok(path) = String::from_utf8(memory::translate_bytes(token, path)) {
        path
    } else {
        return -1;
    };
    if let Some(inode) = fs::open_file(path.as_str(), fs::OpenFlag::from_bits(flags).unwrap()) {
        let mut inner = task.inner_exclusive_access();
        let fd = inner.alloc_fd();
        inner.fd_table[fd] = Some(inode);
        fd as isize
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    let task = task::current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    inner.fd_table[fd].take();
    0
}

pub fn sys_pipe(pipe_read: *mut usize, pipe_write: *mut usize) -> isize {
    let task = task::current_task().unwrap();
    let token = task::current_user_token();
    let mut inner = task.inner_exclusive_access();
    let pipes = fs::make_pipe();
    let read_fd = inner.alloc_fd();
    inner.fd_table[read_fd] = Some(pipes.0);
    let write_fd = inner.alloc_fd();
    inner.fd_table[write_fd] = Some(pipes.1);
    *memory::translate_ref_mut(token, pipe_read) = read_fd;
    *memory::translate_ref_mut(token, pipe_write) = write_fd;
    0
}
