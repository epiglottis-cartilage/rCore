//! `Arc<Inode>` -> `OSInodeInner`: In order to open files concurrently
//! we need to wrap `Inode` into `Arc`,but `Mutex` in `Inode` prevents
//! file systems from being accessed simultaneously
//!
//! `UPSafeCell<OSInodeInner>` -> `OSInode`: for static `ROOT_INODE`,we
//! need to wrap `OSInodeInner` into `UPSafeCell`
use core::ptr::addr_of_mut;

use super::File;
use crate::drivers::BLOCK_DEVICE;
use crate::memory::UserBuffer;
use crate::sync::UPSafeCell;
use alloc::sync::Arc;
use alloc::vec::Vec;
use config::fs::OpenFlag;
use easy_fs::{EasyFileSystem, Inode};
/// A wrapper around a filesystem inode
/// to implement File trait atop
pub struct OSInode {
    readable: bool,
    writable: bool,
    inner: UPSafeCell<OSInodeInner>,
}
/// The OS inode inner in 'UPSafeCell'
pub struct OSInodeInner {
    offset: usize,
    inode: Arc<Inode>,
}

impl OSInode {
    /// Construct an OS inode from a inode
    pub fn new(readable: bool, writable: bool, inode: Arc<Inode>) -> Self {
        Self {
            readable,
            writable,
            inner: unsafe { UPSafeCell::new(OSInodeInner { offset: 0, inode }) },
        }
    }
    /// Read all data inside a inode into vector
    pub fn read_all(&self) -> Vec<u8> {
        let mut inner = self.inner.exclusive_access();
        let mut buffer = [0u8; 512];
        let mut v: Vec<u8> = Vec::new();
        loop {
            let len = inner.inode.read_at(inner.offset, &mut buffer);
            if len == 0 {
                break;
            }
            inner.offset += len;
            v.extend_from_slice(&buffer[..len]);
        }
        v
    }
}

pub static mut ROOT_INODE: Option<Arc<Inode>> = None;

#[deny(dead_code)]
pub fn init() {
    let efs = EasyFileSystem::open(unsafe { BLOCK_DEVICE.as_ref() }.unwrap().clone());
    let root_inode = Arc::new(EasyFileSystem::root_inode(&efs));
    unsafe {
        core::ptr::write_volatile(addr_of_mut!(ROOT_INODE), Some(root_inode.clone()));
    };
}
/// List all files in the filesystems
pub fn list_apps() {
    println!("/**** APPS ****");
    for app in unsafe { ROOT_INODE.as_ref() }.unwrap().ls() {
        println!("{}", app);
    }
    println!("**************/");
}

///Open file with flags
pub fn open_file(name: &str, flags: OpenFlag) -> Option<Arc<OSInode>> {
    let (readable, writable) = flags.read_write();
    if flags.contains(OpenFlag::CREATE) {
        if let Some(inode) = unsafe { ROOT_INODE.as_ref() }.unwrap().find(name) {
            // clear size
            inode.clear();
            Some(Arc::new(OSInode::new(readable, writable, inode)))
        } else {
            // create file
            unsafe { ROOT_INODE.as_ref() }
                .unwrap()
                .create(name)
                .map(|inode| Arc::new(OSInode::new(readable, writable, inode)))
        }
    } else {
        unsafe { ROOT_INODE.as_ref() }
            .unwrap()
            .find(name)
            .map(|inode| {
                if flags.contains(OpenFlag::TRUNC) {
                    inode.clear();
                }
                Arc::new(OSInode::new(readable, writable, inode))
            })
    }
}

impl File for OSInode {
    fn readable(&self) -> bool {
        self.readable
    }
    fn writable(&self) -> bool {
        self.writable
    }
    fn read(&self, mut buf: UserBuffer) -> usize {
        let mut inner = self.inner.exclusive_access();
        let mut total_read_size = 0usize;
        for slice in buf.0.iter_mut() {
            let read_size = inner.inode.read_at(inner.offset, *slice);
            if read_size == 0 {
                break;
            }
            inner.offset += read_size;
            total_read_size += read_size;
        }
        total_read_size
    }
    fn write(&self, buf: UserBuffer) -> usize {
        let mut inner = self.inner.exclusive_access();
        let mut total_write_size = 0usize;
        for slice in buf.0.iter() {
            let write_size = inner.inode.write_at(inner.offset, *slice);
            assert_eq!(write_size, slice.len());
            inner.offset += write_size;
            total_write_size += write_size;
        }
        total_write_size
    }
}
