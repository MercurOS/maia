# Maia - Bootloader for MercurOS

## Build Requirements

Install the `riscv64gc-unknown-none-elf` target by running:
```
rustup target add riscv64gc-unknown-none-elf
```

## Building

The OS kernel to be loaded by Maia needs to be built, and the environment variable
`KERNEL` must be set to point to the kernel ELF binary. Maia is intended to be used
with the [Mercurius kernel](https://github.com/MercurOS/mercurius).

To build Maia, set KERNEL and run cargo build.

Example:
```
$ export KERNEL="../mercurius/target/riscv64gc-unknown-none-elf/release/mercuros-mercurius"
$ cargo build --release
```

For additional debug output, optional cargo features are available:

 - `debug_kernel` prints debug information during the kernel ELF loading process
 - `debug_mmap` prints out the contents of the UEFI provided memory map
 - `debug_all` is shorthand for enabling all the debug features

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-ASL](LICENSE-ASL) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed as above,
without any additional terms or conditions.
