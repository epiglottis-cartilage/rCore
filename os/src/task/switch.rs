//! Switching to a different task's context happens here.

use super::TaskContext;

#[unsafe(naked)]
pub(crate) unsafe extern "C" fn switch(
    current_task_cx_ptr: *mut TaskContext,
    next_task_cx_ptr: *const TaskContext,
) {
    use core::arch::naked_asm;
    naked_asm!(
        // save kernel stack of current task
        "sd sp, 8(a0)",
        // save ra & s0~s11 of current execution
        "sd ra, 0(a0)",
        "sd s0, 16(a0)",
        "sd s1, 24(a0)",
        "sd s2, 32(a0)",
        "sd s3, 40(a0)",
        "sd s4, 48(a0)",
        "sd s5, 56(a0)",
        "sd s6, 64(a0)",
        "sd s7, 72(a0)",
        "sd s8, 80(a0)",
        "sd s9, 88(a0)",
        "sd s10, 96(a0)",
        "sd s11, 104(a0)",
        // restore ra & s0~s11 of next execution
        "ld ra, 0(a1)",
        "ld s0, 16(a1)",
        "ld s1, 24(a1)",
        "ld s2, 32(a1)",
        "ld s3, 40(a1)",
        "ld s4, 48(a1)",
        "ld s5, 56(a1)",
        "ld s6, 64(a1)",
        "ld s7, 72(a1)",
        "ld s8, 80(a1)",
        "ld s9, 88(a1)",
        "ld s10, 96(a1)",
        "ld s11, 104(a1)",
        // restore kernel stack of next task
        "ld sp, 8(a1)",
        "ret",
        // in("a0") current_task_cx_ptr,
        // in("a1") next_task_cx_ptr,
    );
}
