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

For the UT99 BunnyTrack mod, we wanted to have the ability to programmatically associate a user-submitted demo
file with one or more flag captures. This can be accomplished by replicating a unique string from the server to
the client whenever a player caps. That string will then be stored in the UT99 demo file. Once a demo gets submitted
by a player, the idea is then to extract that particular string from the demo to associate it with a flag capture.

Traditional tools like the `strings` binary that you find on Linux and Mac don't cut it here, since ASCII strings
in demos aren't always byte-aligned. UT99 for example doesn't store a boolean value as a byte, but as a single bit.
`ut99-strings` is able to extract all strings from a demo file albeit with some false-positives.

The algorithm works by identifying sequences of printable ASCII characters, and this for each possible byte-alignment.

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