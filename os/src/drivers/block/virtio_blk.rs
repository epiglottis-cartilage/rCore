use core::ptr::addr_of_mut;

use super::BlockDevice;
use crate::memory::{
    FrameTracker, PageTable, PhysAddr, PhysPageNum, VirtAddr, frame_alloc, frame_dealloc,
    kernel_token,
};
use crate::sync::UPSafeCell;
use alloc::vec::Vec;
use virtio_drivers::{Hal, VirtIOBlk, VirtIOHeader};

#[allow(unused)]
const VIRTIO0: usize = 0x10001000;

pub struct VirtIOBlock(UPSafeCell<VirtIOBlk<'static, VirtioHal>>);

static mut QUEUE_FRAMES: Option<UPSafeCell<Vec<FrameTracker>>> = None;

#[deny(dead_code)]
pub fn init() {
    unsafe {
        core::ptr::write_volatile(
            addr_of_mut!(QUEUE_FRAMES),
            Some(UPSafeCell::new(Vec::new())),
        );
    }
}
impl BlockDevice for VirtIOBlock {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        self.0
            .exclusive_access()
            .read_block(block_id, buf)
            .expect("Error when reading VirtIOBlk");
    }
    fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.0
            .exclusive_access()
            .write_block(block_id, buf)
            .expect("Error when writing VirtIOBlk");
    }
}

impl VirtIOBlock {
    #[allow(unused)]
    pub fn new() -> Self {
        unsafe {
            Self(UPSafeCell::new(
                VirtIOBlk::<VirtioHal>::new(&mut *(VIRTIO0 as *mut VirtIOHeader)).unwrap(),
            ))
        }
    }
}

pub struct VirtioHal;

impl Hal for VirtioHal {
    fn dma_alloc(pages: usize) -> usize {
        let mut ppn_base = PhysPageNum(0);
        for i in 0..pages {
            let frame = frame_alloc().unwrap();
            if i == 0 {
                ppn_base = frame.ppn;
            }
            assert_eq!(frame.ppn.0, ppn_base.0 + i);
            unsafe { QUEUE_FRAMES.as_ref() }
                .unwrap()
                .exclusive_access()
                .push(frame);
        }
        let pa: PhysAddr = ppn_base.into();
        pa.0
    }

    fn dma_dealloc(pa: usize, pages: usize) -> i32 {
        let pa = PhysAddr::from(pa);
        let mut ppn_base: PhysPageNum = pa.into();
        for _ in 0..pages {
            frame_dealloc(ppn_base);
            ppn_base = ppn_base + 1;
        }
        0
    }

    fn phys_to_virt(addr: usize) -> usize {
        addr
    }

    fn virt_to_phys(vaddr: usize) -> usize {
        PageTable::from(kernel_token())
            .translate_va(VirtAddr::from(vaddr))
            .unwrap()
            .0
    }
}
