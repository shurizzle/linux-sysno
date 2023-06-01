# linux-sysno

[![Crates.io](https://img.shields.io/crates/v/linux-sysno?style=for-the-badge)](https://crates.io/crates/linux-sysno)
[![docs.rs](https://img.shields.io/docsrs/linux-sysno?style=for-the-badge)](https://docs.rs/linux-sysno)
![Crates.io](https://img.shields.io/crates/l/linux-sysno?style=for-the-badge)

This is just a list of syscall numbers for (almost) all the cpu architectures supported by the Linux kernel.
The current architecture's sysnos (if target is linux) are exported in module's root even without the corresponding feature flag.

### `#![no_std]`

This library is `no_std`, I mean, it's just an enum.

### Feature flags

- `arm`: Enable arm architecture in its own module.
- `mips`: Enable mips architecture in its own module.
- `mips64`: Enable mips64 architecture in its own module.
- `powerpc`: Enable powerpc architecture in its own module.
- `powerpc64`: Enable powerpc64 architecture in its own module.
- `s390x`: Enable s390x architecture in its own module.
- `sparc`: Enable sparc architecture in its own module.
- `sparc64`: Enable sparc64 architecture in its own module.
- `x86`: Enable x86 architecture in its own module.
- `x86_64`: Enable x86_64 architecture in its own module.
- `x32`: Enable x32 architecture in its own module.
- `aarch64`: Enable aarch64 architecture in its own module.
- `riscv32`: Enable riscv32 architecture in its own module.
- `riscv64`: Enable riscv64 architecture in its own module.
- `m68k`: Enable m68k architecture in its own module.
- `loongarch64`: Enable loongarch64 architecture in its own module.
- `all`: Enable all the architectures.

### MSRV

1.16.0
