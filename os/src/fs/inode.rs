//! `Arc<Inode>` -> `OSInodeInner`: In order to open files concurrently
//! we need to wrap `Inode` into `Arc`,but `Mutex` in `Inode` prevents
//! file systems from being accessed simultaneously
//!
//! `UPSafeCell<OSInodeInner>` -> `OSInode`: for static `ROOT_INODE`,we
//! need to wrap `OSInodeInner` into `UPSafeCell`

use super::File;
use super::cfg::OpenFlag;
use crate::memory::UserBuffer;
use crate::sync::UpSafeCell;
use crate::{drivers::BLOCK_DEVICE, sync::UpSafeLazyCell};
use alloc::sync::Arc;
use alloc::vec::Vec;
use easy_fs::{EasyFileSystem, Inode};
/// A wrapper around a filesystem inode
/// to implement File trait atop
pub struct OSInode {
    readable: bool,
    writable: bool,
    inner: UpSafeCell<OSInodeInner>,
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
            inner: unsafe { UpSafeCell::new(OSInodeInner { offset: 0, inode }) },
        }
    }
    /// Read all data inside a inode into vector
    pub fn read_all(&self) -> Vec<u8> {
        let mut inner = self.inner.borrow_mut();
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

pub static ROOT_INODE: UpSafeLazyCell<Arc<Inode>> = unsafe {
    UpSafeLazyCell::new(|| {
        let efs = EasyFileSystem::open(BLOCK_DEVICE.clone());
        Arc::new(EasyFileSystem::root_inode(&efs))
    })
};

/// List all files in the filesystems
pub fn list_apps() {
    println!("/**** APPS ****");
    for app in ROOT_INODE.ls() {
        println!("{}", app);
    }
    println!("**************/");
}

///Open file with flags
pub fn open_file(name: &str, flags: OpenFlag) -> Option<Arc<OSInode>> {
    let (readable, writable) = flags.read_write();
    if flags.contains(OpenFlag::CREATE) {
        if let Some(inode) = ROOT_INODE.find(name) {
            // clear size
            inode.clear();
            Some(Arc::new(OSInode::new(readable, writable, inode)))
        } else {
            // create file
            ROOT_INODE
                .create(name)
                .map(|inode| Arc::new(OSInode::new(readable, writable, inode)))
        }
    } else {
        ROOT_INODE.find(name).map(|inode| {
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
        let mut inner = self.inner.borrow_mut();
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
        let mut inner = self.inner.borrow_mut();
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
