//! Trap handling functionality
//!
//! For rCore, we have a single trap entry point, namely `__alltraps`. At
//! initialization in [`init()`], we set the `stvec` CSR to point to it.
//!
//! All traps go through `__alltraps`, which is defined in `trap.S`. The
//! assembly language code does just enough work restore the kernel space
//! context, ensuring that Rust code safely runs, and transfers control to
//! [`trap_handler()`].
//!
//! It then calls different functionality based on what exactly the exception
//! was. For example, timer interrupts trigger task preemption, and syscalls go
//! to [`syscall()`].

mod context;

use crate::syscall::syscall;
use crate::task;
use config::memory::{TRAMPOLINE, TRAP_CONTEXT};
use core::arch::{asm, global_asm};
use riscv::{
    interrupt::{Exception, Interrupt},
    register::{
        scause::{self, Trap},
        sie, stval, stvec,
    },
};

global_asm!(include_str!("trap.S"));

/// initialize CSR `stvec` as the entry of `__alltraps`
pub fn init() {
    set_kernel_trap_entry();
}

/// timer interrupt enabled
pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

fn set_kernel_trap_entry() {
    unsafe {
        let mut trap = stvec::Stvec::from_bits(0);
        trap.set_address(trap_from_kernel as usize);
        trap.set_trap_mode(stvec::TrapMode::Direct);
        stvec::write(trap);
    }
}

fn set_user_trap_entry() {
    unsafe {
        let mut trap = stvec::Stvec::from_bits(0);
        trap.set_address(TRAMPOLINE);
        trap.set_trap_mode(stvec::TrapMode::Direct);
        stvec::write(trap);
    }
}

/// Unimplement: traps/interrupts/exceptions from kernel mode
/// Todo: Chapter 9: I/O device
#[repr(align(16))]
fn trap_from_kernel() -> ! {
    let scause = scause::read(); // get trap cause
    let stval = stval::read(); // get extra value
    if let Ok(reason) = scause.cause().try_into::<Interrupt, Exception>() {
        match reason {
            Trap::Interrupt(i) => match i {
                Interrupt::SupervisorSoft => todo!(),
                Interrupt::MachineSoft => todo!(),
                Interrupt::SupervisorTimer => {
                    crate::timer::set_next_trigger();
                    task::suspend_current_and_run_next();
                }
                Interrupt::MachineTimer => todo!(),
                Interrupt::SupervisorExternal => todo!(),
                Interrupt::MachineExternal => todo!(),
            },
            Trap::Exception(e) => match e {
                Exception::InstructionMisaligned => todo!(),
                Exception::InstructionFault => todo!(),
                Exception::IllegalInstruction => {
                    println!("[kernel] IllegalInstruction in application, kernel killed it.");
                    task::exit_current_and_run_next();
                }
                Exception::Breakpoint => todo!(),
                Exception::LoadMisaligned => todo!(),
                Exception::LoadFault => todo!(),
                Exception::StoreMisaligned => todo!(),
                Exception::StoreFault => {
                    println!("[kernel] PageFault in application, kernel killed it.");
                    task::exit_current_and_run_next();
                }
                Exception::UserEnvCall => todo!(),
                Exception::SupervisorEnvCall => todo!(),
                Exception::MachineEnvCall => todo!(),
                Exception::InstructionPageFault => todo!(),
                Exception::LoadPageFault => todo!(),
                Exception::StorePageFault => {
                    println!("[kernel] PageFault in application, kernel killed it.");
                    task::exit_current_and_run_next();
                }
            },
        }
    } else {
        panic!(
            "Unsupported trap {:?}, stval = {:#x}!",
            scause.cause(),
            stval
        );
    }
    panic!("a trap from kernel!");
}

/// handle an interrupt, exception, or system call from user space
#[repr(align(16))]
pub(crate) fn trap_handler() -> ! {
    set_kernel_trap_entry();
    let cx = task::current_trap_cx();
    let scause = scause::read(); // get trap cause
    let stval = stval::read(); // get extra value
    if let Ok(reason) = scause.cause().try_into::<Interrupt, Exception>() {
        match reason {
            Trap::Interrupt(i) => match i {
                Interrupt::SupervisorSoft => todo!(),
                Interrupt::MachineSoft => todo!(),
                Interrupt::SupervisorTimer => {
                    crate::timer::set_next_trigger();
                    task::suspend_current_and_run_next();
                }
                Interrupt::MachineTimer => todo!(),
                Interrupt::SupervisorExternal => todo!(),
                Interrupt::MachineExternal => todo!(),
            },
            Trap::Exception(e) => match e {
                Exception::InstructionMisaligned => todo!(),
                Exception::InstructionFault => todo!(),
                Exception::IllegalInstruction => {
                    println!("[kernel] IllegalInstruction in application, kernel killed it.");
                    task::exit_current_and_run_next();
                }
                Exception::Breakpoint => todo!(),
                Exception::LoadMisaligned => todo!(),
                Exception::LoadFault => todo!(),
                Exception::StoreMisaligned => todo!(),
                Exception::StoreFault => {
                    println!("[kernel] PageFault in application, kernel killed it.");
                    task::exit_current_and_run_next();
                }
                Exception::UserEnvCall => {
                    cx.sepc += 4;
                    cx.x[10] = syscall(cx.x[17].into(), [cx.x[10], cx.x[11], cx.x[12]]) as usize;
                }
                Exception::SupervisorEnvCall => todo!(),
                Exception::MachineEnvCall => todo!(),
                Exception::InstructionPageFault => todo!(),
                Exception::LoadPageFault => todo!(),
                Exception::StorePageFault => {
                    println!("[kernel] PageFault in application, kernel killed it.");
                    task::exit_current_and_run_next();
                }
            },
        }
    } else {
        panic!(
            "Unsupported trap {:?}, stval = {:#x}!",
            scause.cause(),
            stval
        );
    }
    trap_return();
}

unsafe extern "C" {
    pub(crate) unsafe fn __alltraps();
    pub(crate) unsafe fn __restore();
}

/// set the new addr of __restore asm function in TRAMPOLINE page,
/// set the reg a0 = trap_cx_ptr, reg a1 = phy addr of usr page table,
/// finally, jump to new addr of __restore asm function
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = task::current_user_token().bits();
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",             // jump to new addr of __restore asm function
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,      // a0 = virt addr of Trap Context
            in("a1") user_satp,        // a1 = phy addr of usr page table
            options(noreturn)
        );
    }
}
pub use context::TrapContext;
