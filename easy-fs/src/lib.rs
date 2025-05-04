#![no_std]

extern crate alloc;
mod bitmap;
mod block_cache;
mod block_dev;
mod efs;
mod layout;
mod vfs;
use config::fs as config;

use bitmap::Bitmap;
use block_cache::{block_cache_sync_all, get_block_cache};
pub use block_dev::BlockDevice;
pub use efs::EasyFileSystem;
use layout::*;
pub use vfs::Inode;
const DIRENT_SZ: usize = core::mem::size_of::<DirEntry>();

#[deny(dead_code)]
pub fn init() {
    block_cache::init();
}
