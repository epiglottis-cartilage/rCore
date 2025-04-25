#![no_std]

extern crate alloc;
mod block_cache;
mod block_dev;
use config::fs::{BLOCK_CACHE_SIZE, BLOCK_SZ};

use block_cache::{block_cache_sync_all, get_block_cache};
use block_dev::BlockDevice;

#[deny(dead_code)]
pub fn init() {
    block_cache::init();
}
