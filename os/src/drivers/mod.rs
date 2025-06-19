pub mod block;

pub use block::BLOCK_DEVICE;

type BlockDeviceImpl = block::VirtIOBlock;

