# Chapter 2

## create

Here I rename it to `libr` just like `libc`
```sh
cargo new libr --lib
```

Under the `libr/src` there is `linker.ld` which is going to be used by program in user mode.

For example, this is how hello_world is created:
```sh
cargo new user/hello_world --bin
mkdir user/.cargo
touch user/.cargo/config.toml
```
this will automatically modify the workspace.
but to make the program linked correctly, don't forget to setup `config.toml`

## changes

1. using different `riscv` crate.
2. `lib_usr` get moved and renamed.
3. user binary is managed individually 