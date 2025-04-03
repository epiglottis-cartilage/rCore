#![no_std]
#![no_main]

#[macro_use]
extern crate libr;

use core::ptr::slice_from_raw_parts_mut;
use libr::sbrk;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("Test sbrk start.");
    const PAGE_SIZE: isize = 0x1000;
    let origin_brk = sbrk(0);
    println!("origin break point = {:x}", origin_brk);
    let brk = sbrk(PAGE_SIZE);
    if brk != origin_brk {
        return -1;
    }
    let brk = sbrk(0);
    println!("one page allocated,  break point = {:x}", brk);
    println!("try write to allocated page");
    let new_page = unsafe {
        &mut *slice_from_raw_parts_mut(
            origin_brk as usize as *const u8 as *mut u8,
            PAGE_SIZE as usize,
        )
    };
    for pos in 0..PAGE_SIZE as usize {
        new_page[pos] = 1;
    }
    println!("write ok");
    sbrk(PAGE_SIZE * 10);
    let brk = sbrk(0);
    println!("10 page allocated,  break point = {:x}", brk);
    sbrk(PAGE_SIZE * -11);
    let brk = sbrk(0);
    println!("11 page DEALLOCATED,  break point = {:x}", brk);
    println!("try DEALLOCATED more one page, should be failed.");
    let ret = sbrk(PAGE_SIZE * -1);
    if ret != -1 {
        println!("Test sbrk failed!");
        return -1;
    }
    println!("Test sbrk almost OK!");
    println!("now write to deallocated page, should cause page fault.");
    for pos in 0..PAGE_SIZE as usize {
        new_page[pos] = 2;
    }
    0
}
