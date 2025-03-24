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

use crate::batch::run_next_app;
use crate::syscall::syscall;
use core::arch::global_asm;
use riscv::{
    interrupt::{Exception, Interrupt},
    register::{
        scause::{self, Trap},
        stval, stvec,
    },
};

global_asm!(include_str!("trap.S"));

/// initialize CSR `stvec` as the entry of `__alltraps`
pub fn init() {
    unsafe extern "C" {
        static __alltraps: usize;
    }
    unsafe {
        let mut trap = stvec::Stvec::from_bits(__alltraps);
        trap.set_trap_mode(stvec::TrapMode::Direct);
        stvec::write(trap);
    }
}

#[unsafe(no_mangle)]
/// handle an interrupt, exception, or system call from user space
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read(); // get trap cause
    let stval = stval::read(); // get extra value
    if let Ok(reason) = scause.cause().try_into::<Interrupt, Exception>() {
        match reason {
            Trap::Interrupt(i) => match i {
                Interrupt::SupervisorSoft => todo!(),
                Interrupt::MachineSoft => todo!(),
                Interrupt::SupervisorTimer => todo!(),
                Interrupt::MachineTimer => todo!(),
                Interrupt::SupervisorExternal => todo!(),
                Interrupt::MachineExternal => todo!(),
            },
            Trap::Exception(e) => match e {
                Exception::InstructionMisaligned => todo!(),
                Exception::InstructionFault => todo!(),
                Exception::IllegalInstruction => {
                    println!("[kernel] IllegalInstruction in application, kernel killed it.");
                    run_next_app();
                }
                Exception::Breakpoint => todo!(),
                Exception::LoadMisaligned => todo!(),
                Exception::LoadFault => todo!(),
                Exception::StoreMisaligned => todo!(),
                Exception::StoreFault => {
                    println!("[kernel] PageFault in application, kernel killed it.");
                    run_next_app();
                }
                Exception::UserEnvCall => {
                    cx.sepc += 4;
                    cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
                }
                Exception::SupervisorEnvCall => todo!(),
                Exception::MachineEnvCall => todo!(),
                Exception::InstructionPageFault => todo!(),
                Exception::LoadPageFault => todo!(),
                Exception::StorePageFault => {
                    println!("[kernel] PageFault in application, kernel killed it.");
                    run_next_app();
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
    cx
}

pub use context::TrapContext;
