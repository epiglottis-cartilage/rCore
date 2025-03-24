#!/bin/sh

cd os
cargo build --release
cd ..

rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os \
 -O binary target/riscv64gc-unknown-none-elf/release/os.bin