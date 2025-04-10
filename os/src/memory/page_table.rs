//! Implementation of [`PageTableEntry`] and [`PageTable`].

use super::{FrameTracker, PhysAddr, PhysPageNum, VirtAddr, VirtPageNum, frame_alloc};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use bitflags::bitflags;

bitflags! {
    /// page table entry flags
    pub struct PageTableEntryFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
/// page table entry structure
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PageTableEntryFlags) -> Self {
        PageTableEntry {
            bits: ppn.0 << 10 | flags.bits() as usize,
        }
    }
    pub fn empty() -> Self {
        PageTableEntry { bits: 0 }
    }
    pub fn ppn(&self) -> PhysPageNum {
        (self.bits >> 10 & ((1usize << 44) - 1)).into()
    }
    pub fn flags(&self) -> PageTableEntryFlags {
        unsafe { PageTableEntryFlags::from_bits(self.bits as u8).unwrap_unchecked() }
    }
    pub fn valid(&self) -> bool {
        self.flags().contains(PageTableEntryFlags::V)
    }
    pub fn readable(&self) -> bool {
        self.flags().contains(PageTableEntryFlags::R)
    }
    pub fn writable(&self) -> bool {
        self.flags().contains(PageTableEntryFlags::W)
    }
    pub fn executable(&self) -> bool {
        self.flags().contains(PageTableEntryFlags::X)
    }
}

/// page table structure
pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<FrameTracker>,
}

/// Assume that it won't oom when creating/mapping.
impl PageTable {
    pub fn new() -> Self {
        let frame = frame_alloc().unwrap();
        PageTable {
            root_ppn: frame.ppn,
            frames: vec![frame],
        }
    }
    /// Temporarily used to get arguments from user space.
    pub fn from_ppn(ppn: PhysPageNum) -> Self {
        Self {
            root_ppn: ppn,
            frames: Vec::new(),
        }
    }
    fn find_or_insert(&mut self, vpn: VirtPageNum) -> &mut PageTableEntry {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        for (i, idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.as_page_table()[*idx];
            if i == 2 {
                return pte;
            }
            if !pte.valid() {
                let frame = frame_alloc().unwrap();
                *pte = PageTableEntry::new(frame.ppn, PageTableEntryFlags::V);
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        unreachable!()
    }
    fn find(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for (i, idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.as_page_table()[*idx];
            if i == 2 {
                result = Some(pte);
                break;
            }
            if !pte.valid() {
                break;
            }
            ppn = pte.ppn();
        }
        result
    }
    #[allow(unused)]
    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PageTableEntryFlags) {
        let pte = self.find_or_insert(vpn);
        assert!(!pte.valid(), "vpn {:?} is mapped before mapping", vpn);
        *pte = PageTableEntry::new(ppn, flags | PageTableEntryFlags::V);
    }
    #[allow(unused)]
    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find(vpn).unwrap();
        assert!(pte.valid(), "vpn {:?} is invalid before unmapping", vpn);
        *pte = PageTableEntry::empty();
    }
    pub fn translate_vp(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find(vpn).map(|pte| *pte)
    }
    pub fn translate_va(&self, va: VirtAddr) -> Option<PhysAddr> {
        self.find(va.into()).map(|page_table_entry| {
            let aligned_pa: usize = page_table_entry.ppn().into();
            (aligned_pa | va.page_offset()).into()
        })
    }
    pub fn token(&self) -> PhysPageNum {
        self.root_ppn
    }
}

/// translate a pointer to a mutable u8 Vec through page table
pub fn translate_sized(
    token: PhysPageNum,
    mut ptr: *const u8,
    mut len: usize,
) -> Vec<&'static mut [u8]> {
    let page_table = PageTable::from_ppn(token);
    let mut resutl = Vec::new();
    loop {
        let part = page_table
            .translate_va((ptr as usize).into())
            .unwrap()
            .to_end();
        let part_len = part.len();
        if len <= part_len {
            resutl.push(&mut part[..len]);
            break;
        } else {
            resutl.push(part);
            len -= part_len;
            ptr = (ptr as usize + part_len) as _;
        }
    }
    resutl
}

/// translate a pointer to a mutable u8 Vec end with `\0` through page table to a `String`
pub fn translate_str(token: PhysPageNum, ptr: *const u8) -> Option<String> {
    let page_table = PageTable::from_ppn(token);

    let res = (ptr as usize..)
        .map(|ptr| ptr.into())
        .map(|va| *page_table.translate_va(va).unwrap().as_mut())
        .take_while(|x: &u8| *x != b'\0')
        .collect::<Vec<_>>();
    String::from_utf8(res).ok()
}

///translate a generic through page table and return a mutable reference
pub fn translated_refmut<T>(token: PhysPageNum, ptr: *mut T) -> &'static mut T {
    PageTable::from_ppn(token)
        .translate_va((ptr as usize).into())
        .unwrap()
        .as_mut()
}
