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

use crate::task;
use crate::{syscall::syscall, timer};

mod cfg {
    pub use config::memory::*;
    pub use config::signal::*;
}
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
        trap.set_address(cfg::TRAMPOLINE);
        trap.set_trap_mode(stvec::TrapMode::Direct);
        stvec::write(trap);
    }
}

/// Unimplement: traps/interrupts/exceptions from kernel mode
/// Todo: Chapter 9: I/O device
#[repr(align(4))]
fn trap_from_kernel() -> ! {
    let scause = scause::read(); // get trap cause
    let stval = stval::read(); // get extra value
    if let Ok(reason) = scause.cause().try_into::<Interrupt, Exception>() {
        panic!("trap from kerne {:?}", reason);
    } else {
        panic!(
            "Unsupported trap {:?}, stval = {:#x}!",
            scause.cause(),
            stval
        );
    }
}

/// handle an interrupt, exception, or system call from user space
#[repr(align(4))]
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
                    timer::set_next_trigger();
                    task::suspend_current_and_run_next();
                }
                Interrupt::MachineTimer => todo!(),
                Interrupt::SupervisorExternal => todo!(),
                Interrupt::MachineExternal => todo!(),
            },
            Trap::Exception(e) => match e {
                Exception::IllegalInstruction => {
                    log::error!("[kernel] IllegalInstruction in application.");
                    task::current_add_signal(cfg::SignalID::ILL);
                }
                Exception::Breakpoint => todo!(),
                exception @ (Exception::LoadFault
                | Exception::StoreFault
                | Exception::LoadPageFault
                | Exception::StorePageFault
                | Exception::LoadMisaligned
                | Exception::StoreMisaligned
                | Exception::InstructionMisaligned
                | Exception::InstructionFault) => {
                    log::error!(
                        "[kernel] {:?} in application, bad addr = {:#x}, bad instruction = {:#x}.",
                        exception,
                        stval,
                        task::current_trap_cx().sepc,
                    );
                    task::current_add_signal(cfg::SignalID::SEGV);
                }
                Exception::UserEnvCall => {
                    cx.sepc += 4;
                    cx.x[10] = syscall(
                        unsafe { core::mem::transmute(cx.x[17]) },
                        [cx.x[10], cx.x[11], cx.x[12]],
                    ) as usize;
                }
                Exception::SupervisorEnvCall => todo!(),
                Exception::MachineEnvCall => todo!(),
                Exception::InstructionPageFault => todo!(),
            },
        }
    } else {
        panic!(
            "Unsupported trap {:?}, stval = {:#x}!",
            scause.cause(),
            stval
        );
    }
    // handle signals (handle the sent signal)
    //println!("[K] trap_handler:: handle_signals");
    task::handle_signals();

    // check error signals (if error then exit)
    if let Some((errno, msg)) = task::check_signals_error_of_current() {
        log::info!("[kernel] {}", msg);
        task::exit_current_and_run_next(errno);
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
    let trap_cx_ptr = cfg::TRAP_CONTEXT;
    let user_satp: riscv::register::satp::Satp = task::current_user_token().into();
    let restore_va = __restore as usize - __alltraps as usize + cfg::TRAMPOLINE;
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",          // jump to new addr of __restore asm function
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,      // a0 = virt addr of Trap Context
            in("a1") user_satp.bits(),        // a1 = phy addr of usr page table
            options(noreturn)
        );
    }
}
pub use context::TrapContext;
