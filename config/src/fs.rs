/// Use a block size of 512 bytes
pub const BLOCK_SZ: usize = 512;
pub const BLOCK_BITS: usize = BLOCK_SZ * 8;
/// Use a block cache of 16 blocks
pub const BLOCK_CACHE_SIZE: usize = 16;

/// Magic number for sanity check
pub const EFS_MAGIC: u32 = 0x94740454;
/// The max number of direct inodes
pub const INODE_DIRECT_COUNT: usize = 28;
/// The max length of inode name
pub const NAME_LENGTH_LIMIT: usize = 27;
/// The max number of indirect inodes
pub const INODE_INDIRECT_COUNT: usize = BLOCK_SZ / 4;
/// The max number of indirect1 inodes
pub const INODE_INDIRECT1_COUNT: usize = INODE_INDIRECT_COUNT;
/// The max number of indirect2 inodes
pub const INODE_INDIRECT2_COUNT: usize = INODE_INDIRECT1_COUNT * INODE_INDIRECT1_COUNT;
/// The upper bound of direct inode index
pub const DIRECT_BOUND: usize = INODE_DIRECT_COUNT;
/// The upper bound of indirect1 inode index
pub const INDIRECT1_BOUND: usize = DIRECT_BOUND + INODE_INDIRECT1_COUNT;
/// The upper bound of indirect2 inode indexs
#[allow(unused)]
pub const INDIRECT2_BOUND: usize = INDIRECT1_BOUND + INODE_INDIRECT2_COUNT;

pub mod file_descriptor {
    pub const STDIN: usize = 0;
    pub const STDOUT: usize = 1;
    pub const STDERR: usize = 2;
}
