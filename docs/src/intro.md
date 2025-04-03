This is my own implication of rCore. 

## NOTE

Using EndeavourOS x86_64.
So there will be some change from the official tutorial.

1. `riscv64-elf-gdb` is provided by EndeavourOS, instead of `riscv64-unknown-elf-gdb` so it is renamed.
2. No makefile. Instead, most used commands are written in script (including python)

## NOTE

python is also needed, 3.12+

## NOTE

location of `.cargo/` is important, make sure they are put at correct location 

## helpful

[turn off stupid error message from rust-analyzer](https://github.com/rust-lang/vscode-rust/issues/729)

## BUGS

- [ ] `sbss`, `ebss` are not used correctly in chapter 1,2,3 