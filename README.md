# UT99-STRINGS

## Usage
```
Tool to extract ASCII strings from UT99 DEMOs

Usage: ut99-strings [OPTIONS] <FILE>

Arguments:
  <FILE>  File path to a UT99 DEMO

Options:
  -l, --length <LENGTH>  Minimum string length [default: 6]
  -h, --help             Print help
```

## Description

TODO

## Build
1. Install [Rust](https://www.rust-lang.org/learn/get-started)
2. Go to the project root folder
3. Execute `cargo build --release`
4. The binary can now be found in the `/target/release` folder

Note that this process will only produce a binary which can run on the OS / architecture
where the compilation executed. If you wish to produce binaries for another OS / architecture,
then you could look into [cross-compilation](https://github.com/cross-rs/cross), or compile
the project on another OS / architecture.

Furthermore, the resulting binary doesn't require any runtime to be installed in order to run the binary,
since all libraries are statically linked in.