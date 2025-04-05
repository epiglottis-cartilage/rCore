
    .align 3
    .section .data
    .global app_address
app_address:
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_3_start
    .quad app_4_start
    .quad app_5_start
    .quad app_6_start
    .quad app_6_end

    .section .data
    .global app_0_start
    .global app_0_end
app_0_start:
    .incbin "target/riscv64gc-unknown-none-elf/release/load_fault"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
app_1_start:
    .incbin "target/riscv64gc-unknown-none-elf/release/power_3"
app_1_end:

    .section .data
    .global app_2_start
    .global app_2_end
app_2_start:
    .incbin "target/riscv64gc-unknown-none-elf/release/power_5"
app_2_end:

    .section .data
    .global app_3_start
    .global app_3_end
app_3_start:
    .incbin "target/riscv64gc-unknown-none-elf/release/power_7"
app_3_end:

    .section .data
    .global app_4_start
    .global app_4_end
app_4_start:
    .incbin "target/riscv64gc-unknown-none-elf/release/sbrk_test"
app_4_end:

    .section .data
    .global app_5_start
    .global app_5_end
app_5_start:
    .incbin "target/riscv64gc-unknown-none-elf/release/sleep"
app_5_end:

    .section .data
    .global app_6_start
    .global app_6_end
app_6_start:
    .incbin "target/riscv64gc-unknown-none-elf/release/store_fault"
app_6_end:
