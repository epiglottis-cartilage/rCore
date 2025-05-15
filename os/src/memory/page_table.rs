//! Implementation of [`PageTableEntry`] and [`PageTable`].

use super::{FrameTracker, PhysAddr, PhysPageNum, VirtAddr, VirtPageNum, frame_alloc};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

bitflags::bitflags! {
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
        self.find(va.floor()).map(|page_table_entry| {
            let aligned_pa: PhysAddr = page_table_entry.ppn().into();
            (aligned_pa.0 | va.page_offset()).into()
        })
    }
    pub fn token(&self) -> usize {
        let stap: riscv::register::satp::Satp = self.root_ppn.into();
        stap.bits()
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

pub fn translate_slice<T>(token: usize, ptr: *const *const [T]) -> (Vec<&'static mut [T]>, usize) {
    // assert
    let mut raw_ptr = *translate_ref(token, ptr as *const *const T);
    let mut len = *translate_ref(token, unsafe { (ptr as *const usize).add(1) });

    let page_table = PageTable::from_ppn(token.into());
    let mut result = Vec::new();
    loop {
        let part: &'static mut [u8] = page_table
            .translate_va((raw_ptr as usize).into())
            .unwrap()
            .to_end();
        assert_eq!(part.len() % size_of::<T>(), 0);

        let part: &'static mut [T] = unsafe {
            core::slice::from_raw_parts_mut(part.as_ptr() as _, part.len() / size_of::<T>())
        };
        let part_len = part.len();
        if len <= part_len {
            result.push(&mut part[..len]);
            break;
        } else {
            result.push(part);
            len -= part_len;
            raw_ptr = (raw_ptr as usize + part_len) as _;
        }
    }
    (result, len)
}

pub fn translate_necked_slice<T: 'static>(
    token: usize,
    ptr: *const *const [*const [T]],
) -> impl DoubleEndedIterator<Item = *const *const [T]> {
    let raw_ptr = translate_ref(token, ptr as *const *const [T]);
    let len = *translate_ref(token, unsafe { (ptr as *const usize).add(1) });

    unsafe {
        core::slice::from_raw_parts(raw_ptr as *const _, len * 2)
            .iter()
            .map(|ptr: &*const [T]| ptr as *const _)
            .step_by(2)
    }
}

pub fn translate_str_slice(token: usize, ptr: *const *const [*const str]) -> Option<Vec<String>> {
    translate_necked_slice(token, ptr as *const *const [*const [u8]])
        .into_iter()
        .map(|ptr| translate_str(token, ptr as _))
        .try_collect()
}

/// translate a pointer to a mutable u8 Vec end with `\0` through page table to a `String`
pub fn translate_str(token: usize, ptr: *const *const str) -> Option<String> {
    let (strs, len) = translate_slice(token, ptr as *const *const [u8]);
    let bytes = strs.iter().fold(Vec::with_capacity(len), |mut acc, str| {
        acc.extend_from_slice(*str);
        acc
    });
    String::from_utf8(bytes).ok()
}

/// translate a generic through page table and return a mutable reference
pub fn translate_ref_mut<T>(token: usize, ptr: *mut T) -> &'static mut T {
    PageTable::from_ppn(token.into())
        .translate_va((ptr as usize).into())
        .unwrap()
        .as_mut()
}

/// translate a generic through page table and return a reference
pub fn translate_ref<T>(token: usize, ptr: *const T) -> &'static T {
    PageTable::from_ppn(token.into())
        .translate_va((ptr as usize).into())
        .unwrap()
        .as_mut()
}

/// translate a generic through page table and return a reference
pub fn translate_to<T>(token: usize, src: *const T, dst: &mut T) {
    let page_table = PageTable::from_ppn(PhysPageNum(token));
    let mut len = size_of::<T>();
    let mut src = src as *const u8;
    let mut dst = dst as *const _ as *mut u8;
    loop {
        let part = page_table
            .translate_va((src as usize).into())
            .unwrap()
            .to_end();
        if len <= part.len() {
            unsafe {
                core::ptr::copy(src, dst, len);
            }
            break;
        } else {
            unsafe {
                core::ptr::copy(src, dst, part.len());
                len -= part.len();
                src = src.add(part.len());
                dst = dst.add(part.len());
            }
        }
    }
}
/// Array of u8 slice that user communicate with os
pub struct UserBuffer(pub Vec<&'static mut [u8]>);

impl UserBuffer {
    /// Create a `UserBuffer` by parameter
    pub fn new(buffers: Vec<&'static mut [u8]>) -> Self {
        Self(buffers)
    }
    /// Length of `UserBuffer`
    pub fn len(&self) -> usize {
        self.0.iter().map(|buf| buf.len()).sum()
    }
    /// Iterator over the buffer
    pub fn as_bytes(&mut self) -> impl Iterator<Item = &mut u8> {
        self.0.iter_mut().flat_map(|buf| buf.iter_mut())
    }
    /// Iterator over the buffer
    pub fn into_bytes(self) -> impl Iterator<Item = &'static mut u8> {
        self.0.into_iter().flat_map(|buf| buf.into_iter())
    }
}
