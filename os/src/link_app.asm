
    .align 3
    .section .data
    .global num_app
num_app:
    .quad 4
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_3_start
    .quad app_3_end

    .section .data
    .global app_0_start
    .global app_0_end
app_0_start:
    .incbin "target/riscv64gc-unknown-none-elf/release/power_3"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
app_1_start:
    .incbin "target/riscv64gc-unknown-none-elf/release/power_5"
app_1_end:

    .section .data
    .global app_2_start
    .global app_2_end
app_2_start:
    .incbin "target/riscv64gc-unknown-none-elf/release/power_7"
app_2_end:

    .section .data
    .global app_3_start
    .global app_3_end
app_3_start:
    .incbin "target/riscv64gc-unknown-none-elf/release/sleep"
app_3_end:
