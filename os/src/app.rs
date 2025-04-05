use include_bytes_aligned::*;
pub const APP: [&str; 7] = [
    "load_fault",
    "power_3",
    "power_5",
    "power_7",
    "sbrk_test",
    "sleep",
    "store_fault",
];
pub const APP_DATA: [&[u8]; 7] = [
    include_bytes_aligned!(4,"../../target/riscv64gc-unknown-none-elf/release/load_fault"),
    include_bytes_aligned!(4,"../../target/riscv64gc-unknown-none-elf/release/power_3"),
    include_bytes_aligned!(4,"../../target/riscv64gc-unknown-none-elf/release/power_5"),
    include_bytes_aligned!(4,"../../target/riscv64gc-unknown-none-elf/release/power_7"),
    include_bytes_aligned!(4,"../../target/riscv64gc-unknown-none-elf/release/sbrk_test"),
    include_bytes_aligned!(4,"../../target/riscv64gc-unknown-none-elf/release/sleep"),
    include_bytes_aligned!(4,"../../target/riscv64gc-unknown-none-elf/release/store_fault"),
];
