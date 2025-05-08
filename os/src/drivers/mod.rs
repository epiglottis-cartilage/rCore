pub mod block;

pub use block::BLOCK_DEVICE;

type BlockDeviceImpl = block::VirtIOBlock;

#[deny(dead_code)]
pub fn init() {
    block::init();
}
