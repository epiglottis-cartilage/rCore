mod virtio_blk;

use core::ptr::addr_of_mut;

pub use virtio_blk::VirtIOBlock;

use super::BlockDeviceImpl;
use alloc::sync::Arc;
use easy_fs::BlockDevice;

pub static mut BLOCK_DEVICE: Option<Arc<dyn BlockDevice>> = None;

#[deny(dead_code)]
pub fn init() {
    virtio_blk::init();
    let block_device: Arc<dyn BlockDevice> = Arc::new(BlockDeviceImpl::new());
    unsafe {
        core::ptr::write_volatile(addr_of_mut!(BLOCK_DEVICE), Some(block_device));
    };
}

#[allow(unused)]
pub fn block_device_test() {
    let block_device = unsafe { BLOCK_DEVICE.as_ref() }.unwrap().clone();
    let mut write_buffer = [0u8; 512];
    let mut read_buffer = [0u8; 512];
    for i in 0..512 {
        for byte in write_buffer.iter_mut() {
            *byte = i as u8;
        }
        block_device.write_block(i as usize, &write_buffer);
        block_device.read_block(i as usize, &mut read_buffer);
        assert_eq!(write_buffer, read_buffer);
    }
    println!("block device test passed!");
}
