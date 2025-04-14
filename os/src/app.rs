use include_bytes_aligned::*;
pub const APP: [&str; 12] = [
    "exit",
    "forktree",
    "init",
    "load_fault",
    "power_3",
    "power_5",
    "power_7",
    "sbrk_test",
    "shell",
    "sleep",
    "stack_overflow",
    "store_fault",
];
pub const APP_DATA: [&[u8]; 12] = [
    include_bytes_aligned!(4, "../../target/riscv64gc-unknown-none-elf/release/exit"),
    include_bytes_aligned!(4, "../../target/riscv64gc-unknown-none-elf/release/forktree"),
    include_bytes_aligned!(4, "../../target/riscv64gc-unknown-none-elf/release/init"),
    include_bytes_aligned!(4, "../../target/riscv64gc-unknown-none-elf/release/load_fault"),
    include_bytes_aligned!(4, "../../target/riscv64gc-unknown-none-elf/release/power_3"),
    include_bytes_aligned!(4, "../../target/riscv64gc-unknown-none-elf/release/power_5"),
    include_bytes_aligned!(4, "../../target/riscv64gc-unknown-none-elf/release/power_7"),
    include_bytes_aligned!(4, "../../target/riscv64gc-unknown-none-elf/release/sbrk_test"),
    include_bytes_aligned!(4, "../../target/riscv64gc-unknown-none-elf/release/shell"),
    include_bytes_aligned!(4, "../../target/riscv64gc-unknown-none-elf/release/sleep"),
    include_bytes_aligned!(4, "../../target/riscv64gc-unknown-none-elf/release/stack_overflow"),
    include_bytes_aligned!(4, "../../target/riscv64gc-unknown-none-elf/release/store_fault"),
];
