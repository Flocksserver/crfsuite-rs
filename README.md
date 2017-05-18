# crfsuite-rs

Rust bindings for CRFSuite

## Requirements

This lib uses [Bindgen](https://github.com/servo/rust-bindgen) to generate FFI bindings, hence you need to have clang installed

```bash
$ sudo apt-get install llvm-3.9-dev libclang-3.9-dev clang-3.9 # ubuntu, see http://apt.llvm.org/ before 16.10
$ sudo pacman -S clang # ArchLinux
$ brew install llvm@3.9 # macOS

```
