
    .align 4
    .section .data
    .global _num_app
_num_app:
    .quad 3
    
    .quad app_0_start
        
    .quad app_1_start
        
    .quad app_2_start
        
    .quad app_2_end
    
    .section .data
    .global app_0_start
    .global app_0_end
app_0_start:
    .incbin "target/riscv64gc-unknown-none-elf/release/hello_world.bin"
app_0_end:
        
    .section .data
    .global app_1_start
    .global app_1_end
app_1_start:
    .incbin "target/riscv64gc-unknown-none-elf/release/store_fault.bin"
app_1_end:
        
    .section .data
    .global app_2_start
    .global app_2_end
app_2_start:
    .incbin "target/riscv64gc-unknown-none-elf/release/power.bin"
app_2_end:
        
