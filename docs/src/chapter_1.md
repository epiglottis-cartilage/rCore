# [Chapter 1](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter1/index.html)

**This may be out of date, goto the brach chapter1 instead!!**

## intro

The first kernel instruction.

## Run

~~At the root directory~~ for unknown reason the linker doesn't work at the root directory. So, unlucky you have to do this:
```sh
cd os
cargo build --release
cd ..
```

I don't know how to use `tmux` so do following individually
```sh
./scripts/build_os
./scripts/qemu-debug
./scripts/gdb
```

## Changes

Compared to the original rCore, flowing thins are changed:

1. location of the stack, form `bss` to `data`.
2. update `rustsbi`.