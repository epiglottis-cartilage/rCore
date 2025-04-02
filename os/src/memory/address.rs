//! Implementation of physical and virtual address and page number.

use super::PageTableEntry;
use config::memory::{PAGE_SIZE, PAGE_SIZE_BITS};
use core::fmt::{self, Debug, Formatter};
use riscv::register::satp::Mode;

/// physical address
const VA_MODE: Mode = Mode::Sv39;
const PA_WIDTH: usize = 56;
const VA_WIDTH: usize = match VA_MODE {
    Mode::Bare => panic!("Bare mode is not supported in VirtAddr"),
    Mode::Sv39 => 39,
    Mode::Sv48 => 48,
    Mode::Sv57 => 57,
    Mode::Sv64 => 64,
};

const PPN_WIDTH: usize = PA_WIDTH - PAGE_SIZE_BITS;
const VPN_WIDTH: usize = VA_WIDTH - PAGE_SIZE_BITS;

/// Definitions
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(pub usize);

/// virtual address
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(pub usize);

/// physical page number
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysPageNum(pub usize);

/// virtual page number
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtPageNum(pub usize);

/// Debugging

impl Debug for VirtAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("VA:{:#x}", self.0))
    }
}
impl Debug for VirtPageNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("VPN:{:#x}", self.0))
    }
}
impl Debug for PhysAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PA:{:#x}", self.0))
    }
}
impl Debug for PhysPageNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PPN:{:#x}", self.0))
    }
}

/// T: {PhysAddr, VirtAddr, PhysPageNum, VirtPageNum}
/// T -> usize: T.0
/// usize -> T: usize.into()

impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PA_WIDTH) - 1))
    }
}
impl From<usize> for PhysPageNum {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PPN_WIDTH) - 1))
    }
}
impl From<usize> for VirtAddr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << VA_WIDTH) - 1))
    }
}
impl From<usize> for VirtPageNum {
    fn from(v: usize) -> Self {
        Self(v & ((1 << VPN_WIDTH) - 1))
    }
}
impl From<PhysAddr> for usize {
    fn from(v: PhysAddr) -> Self {
        v.0
    }
}
impl From<PhysPageNum> for usize {
    fn from(v: PhysPageNum) -> Self {
        v.0
    }
}
impl From<VirtAddr> for usize {
    fn from(v: VirtAddr) -> Self {
        if v.0 >= (1 << (VA_WIDTH - 1)) {
            v.0 | (!((1 << VA_WIDTH) - 1))
        } else {
            v.0
        }
    }
}
impl From<VirtPageNum> for usize {
    fn from(v: VirtPageNum) -> Self {
        v.0
    }
}

impl VirtAddr {
    pub fn floor(&self) -> VirtPageNum {
        VirtPageNum(self.0 / PAGE_SIZE)
    }
    pub fn ceil(&self) -> VirtPageNum {
        if self.0 == 0 {
            VirtPageNum(0)
        } else {
            VirtPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
        }
    }
    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }
}
impl From<VirtAddr> for VirtPageNum {
    fn from(v: VirtAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}
impl From<VirtPageNum> for VirtAddr {
    fn from(v: VirtPageNum) -> Self {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}
impl PhysAddr {
    pub fn floor(&self) -> PhysPageNum {
        PhysPageNum(self.0 / PAGE_SIZE)
    }
    pub fn ceil(&self) -> PhysPageNum {
        if self.0 == 0 {
            PhysPageNum(0)
        } else {
            PhysPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
        }
    }
    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }
}
impl From<PhysAddr> for PhysPageNum {
    fn from(v: PhysAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}
impl From<PhysPageNum> for PhysAddr {
    fn from(v: PhysPageNum) -> Self {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}

impl VirtPageNum {
    pub fn indexes(&self) -> [usize; 3] {
        let vpn = self.0;
        let mask = (1 << 9) - 1;
        [(vpn >> 18) & mask, (vpn >> 9) & mask, vpn & mask]
    }
}

impl PhysPageNum {
    #[inline]
    pub fn as_page_table(
        &self,
    ) -> &'static mut [PageTableEntry; PAGE_SIZE / size_of::<PageTableEntry>()] {
        self.as_mut()
    }
    #[inline]
    pub fn as_bytes(&self) -> &'static mut [u8; PAGE_SIZE] {
        self.as_mut()
    }
    pub fn as_mut<T>(&self) -> &'static mut T {
        assert!(core::mem::size_of::<T>() <= PAGE_SIZE);
        let pa: PhysAddr = (*self).into();
        unsafe { (pa.0 as *mut T).as_mut().unwrap() }
    }
}

impl core::iter::Step for VirtPageNum {
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        (end.0 - start.0, Some(end.0 - start.0))
    }

    fn forward_checked(mut start: Self, count: usize) -> Option<Self> {
        start.0 = start.0.checked_add(count)?;
        Some(start)
    }

    fn backward_checked(mut start: Self, count: usize) -> Option<Self> {
        start.0 = start.0.checked_sub(count)?;
        Some(start)
    }

    fn forward(mut start: Self, count: usize) -> Self {
        start.0 += count;
        start
    }

    fn backward(mut start: Self, count: usize) -> Self {
        start.0 -= count;
        start
    }
}

impl core::iter::Step for PhysPageNum {
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        (end.0 - start.0, Some(end.0 - start.0))
    }

    fn forward_checked(mut start: Self, count: usize) -> Option<Self> {
        start.0 = start.0.checked_add(count)?;
        Some(start)
    }

    fn backward_checked(mut start: Self, count: usize) -> Option<Self> {
        start.0 = start.0.checked_sub(count)?;
        Some(start)
    }

    fn forward(mut start: Self, count: usize) -> Self {
        start.0 += count;
        start
    }

    fn backward(mut start: Self, count: usize) -> Self {
        start.0 -= count;
        start
    }
}
