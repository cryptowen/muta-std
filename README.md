# muta-std
[![Crates.io](https://img.shields.io/crates/v/muta-std.svg)](https://crates.io/crates/muta-std)

This library contains serveral modules that help you write huobi-chain riscv contract with Rust.

This project is highly inspired by [ckb-std](https://github.com/nervosnetwork/ckb-std).

### Usage

Check [tests](https://github.com/huwenchao/muta-std/blob/master/test/contract/src/main.rs) to learn how to use.

### Modules

* `syscalls` module: defines [huobi-chain riscv syscalls](https://github.com/HuobiGroup/huobi-chain/blob/master/services/riscv/src/vm/syscall/mod.rs)
* `high_level` module: defines high level APIs
* `debug!` macro: a `println!` like macro helps debugging
* `entry!` macro: defines contract entry point
* `default_alloc!` and `libc_alloc!` macro: defines global allocator for no-std rust
