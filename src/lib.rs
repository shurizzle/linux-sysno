#![no_std]

#[cfg(feature = "generic")]
pub mod generic;

#[cfg(any(feature = "arm", all(target_os = "linux", target_arch = "arm")))]
pub mod arm;

#[cfg(any(feature = "mips", all(target_os = "linux", target_arch = "mips")))]
pub mod mips;

#[cfg(any(feature = "mips64", all(target_os = "linux", target_arch = "mips64")))]
pub mod mips64;

#[cfg(any(feature = "powerpc", all(target_os = "linux", target_arch = "powerpc")))]
pub mod powerpc;

#[cfg(any(
    feature = "powerpc64",
    all(target_os = "linux", target_arch = "powerpc64")
))]
pub mod powerpc64;

#[cfg(any(feature = "s390x", all(target_os = "linux", target_arch = "s390x")))]
pub mod s390x;

#[cfg(any(feature = "sparc", all(target_os = "linux", target_arch = "sparc")))]
pub mod sparc;

#[cfg(any(feature = "sparc64", all(target_os = "linux", target_arch = "sparc64")))]
pub mod sparc64;

#[cfg(any(feature = "x86", all(target_os = "linux", target_arch = "x86")))]
pub mod x86;

#[cfg(any(feature = "x86_64", all(target_os = "linux", target_arch = "x86_64")))]
pub mod x86_64;

#[cfg(any(feature = "aarch64", all(target_os = "linux", target_arch = "aarch64")))]
pub mod aarch64;

#[cfg(any(feature = "riscv32", all(target_os = "linux", target_arch = "riscv32")))]
#[path = "generic.rs"]
pub mod riscv32;

#[cfg(any(feature = "riscv64", all(target_os = "linux", target_arch = "riscv64")))]
#[path = "generic.rs"]
pub mod riscv64;

#[cfg(any(feature = "m68k", all(target_os = "linux", target_arch = "m68k")))]
#[path = "generic.rs"]
pub mod m68k;

#[cfg(any(
    feature = "loongarch64",
    all(target_os = "linux", target_arch = "loongarch64")
))]
#[path = "generic.rs"]
pub mod loongarch64;

#[cfg(all(target_os = "linux", target_arch = "arm"))]
pub use arm::*;

#[cfg(all(target_os = "linux", target_arch = "mips"))]
pub use mips::*;

#[cfg(all(target_os = "linux", target_arch = "mips64"))]
pub use mips64::*;

#[cfg(all(target_os = "linux", target_arch = "powerpc"))]
pub use powerpc::*;

#[cfg(all(target_os = "linux", target_arch = "powerpc64"))]
pub use powerpc64::*;

#[cfg(all(target_os = "linux", target_arch = "s390x"))]
pub use s390x::*;

#[cfg(all(target_os = "linux", target_arch = "sparc"))]
pub use sparc::*;

#[cfg(all(target_os = "linux", target_arch = "sparc64"))]
pub use sparc64::*;

#[cfg(all(target_os = "linux", target_arch = "x86"))]
pub use x86::*;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub use x86_64::*;

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub use aarch64::*;

#[cfg(all(target_os = "linux", target_arch = "riscv32"))]
pub use riscv32::*;

#[cfg(all(target_os = "linux", target_arch = "riscv64"))]
pub use riscv64::*;

#[cfg(all(target_os = "linux", target_arch = "m68k"))]
pub use m68k::*;

#[cfg(all(target_os = "linux", target_arch = "loongarch64"))]
pub use loongarch64::*;

#[cfg(test)]
mod tests {
    #[test]
    fn is_present() {
        let _ = super::Sysno::write;
    }
}
