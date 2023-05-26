mod format;
mod generate;
mod header;
mod kernel_org;
mod table;

use std::{fmt, fs::File, io::Write, path::Path, rc::Rc};

use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use format::Formatter;
use generate::{build_enum, SysnoEnum};
use kernel_org::fetch_file;

use crate::kernel_org::latest_version;

fn main() -> Result<()> {
    color_eyre::install()?;

    let base = {
        let mut args = std::env::args_os();

        if args.len() != 2 {
            bail!("Directory not present");
        }
        _ = args.next().unwrap();
        args.next().unwrap()
    };
    let base = &base;

    let version = latest_version()?;
    let formatter = Formatter::new()?;
    let formatter = &formatter;

    write_file(
        build_enum(
            header::from_reader(fetch_file(&version, "include/uapi/asm-generic/unistd.h")?).filter(
                |l| {
                    if let Ok(l) = l {
                        l.0.as_ref() != "sync_file_range"
                    } else {
                        true
                    }
                },
            ),
        )?,
        formatter,
        base,
        "generic",
    )?;

    write_file(
        table_enum(
            &version,
            "arch/x86/entry/syscalls/syscall_32.tbl",
            [ABI::I386],
        )?,
        formatter,
        base,
        "x86",
    )?;

    write_file(
        table_enum(
            &version,
            "arch/x86/entry/syscalls/syscall_64.tbl",
            [ABI::COMMON, ABI::B64],
        )?,
        formatter,
        base,
        "x86_64",
    )?;

    write_file(
        table_enum(&version, "arch/arm/tools/syscall.tbl", [ABI::COMMON])?,
        formatter,
        base,
        "arm",
    )?;

    write_file(
        table_enum(
            &version,
            "arch/sparc/kernel/syscalls/syscall.tbl",
            [ABI::COMMON, ABI::B32],
        )?,
        formatter,
        base,
        "sparc",
    )?;

    write_file(
        table_enum(
            &version,
            "arch/sparc/kernel/syscalls/syscall.tbl",
            [ABI::COMMON, ABI::B64],
        )?,
        formatter,
        base,
        "sparc64",
    )?;

    write_file(
        table_enum(
            &version,
            "arch/powerpc/kernel/syscalls/syscall.tbl",
            [ABI::COMMON, ABI::NOSPU, ABI::B32],
        )?,
        formatter,
        base,
        "powerpc",
    )?;

    write_file(
        table_enum(
            &version,
            "arch/powerpc/kernel/syscalls/syscall.tbl",
            [ABI::COMMON, ABI::NOSPU, ABI::B64],
        )?,
        formatter,
        base,
        "powerpc64",
    )?;

    write_file(
        table_enum(
            &version,
            "arch/mips/kernel/syscalls/syscall_o32.tbl",
            [ABI::O32],
        )?,
        formatter,
        base,
        "mips",
    )?;

    write_file(
        table_enum(
            &version,
            "arch/mips/kernel/syscalls/syscall_n64.tbl",
            [ABI::N64],
        )?,
        formatter,
        base,
        "mips64",
    )?;

    write_file(
        table_enum(
            &version,
            "arch/s390/kernel/syscalls/syscall.tbl",
            [ABI::COMMON, ABI::B64],
        )?,
        formatter,
        base,
        "s390x",
    )?;

    write_file(
        table_enum(
            &version,
            "arch/m68k/kernel/syscalls/syscall.tbl",
            [ABI::COMMON],
        )?,
        formatter,
        base,
        "m68k",
    )
}

#[derive(Debug, Copy, Clone)]
pub struct ABI<'a>(&'a str, usize);

impl<'a> ABI<'a> {
    pub const COMMON: Self = Self("common", 0);
    pub const I386: Self = Self("i386", 0);
    pub const NOSPU: Self = Self("nospu", 0);
    pub const B32: Self = Self("32", 0);
    pub const B64: Self = Self("64", 0);
    pub const O32: Self = Self("o32", 4000);
    pub const N64: Self = Self("n64", 5000);

    pub fn adjust(&self, n: usize) -> usize {
        self.1 + n
    }
}

fn table_enum<'a, T1, T2, T3>(version: T1, path: T2, abis: T3) -> Result<SysnoEnum>
where
    T1: fmt::Display,
    T2: fmt::Display,
    T3: AsRef<[ABI<'a>]>,
{
    let abis = abis.as_ref();

    build_enum(
        table::from_reader(fetch_file(version, path)?).filter_map(|l| match l {
            Ok((no, abi, name)) => {
                let abi = <Rc<Box<str>> as AsRef<Box<str>>>::as_ref(&abi).as_ref();
                abis.iter()
                    .find(|a| a.0 == abi)
                    .map(|abi| Ok((name, abi.adjust(no))))
            }
            Err(err) => Some(Err(err)),
        }),
    )
}

fn write_file<T: fmt::Display, P: AsRef<Path>, N: fmt::Display>(
    content: T,
    formatter: &Formatter,
    path: P,
    name: N,
) -> Result<()> {
    let content = formatter.format(content)?;

    let name = format!("{}.rs", name);
    let mut path = path.as_ref().to_path_buf();
    std::fs::create_dir_all(&path)
        .wrap_err_with(|| format!("Failed to create directory {}", path.display()))?;
    path.push(name);

    File::create(&path)
        .wrap_err_with(|| format!("Failed to write generated file {}", path.display()))?
        .write_all(content.as_bytes())
        .wrap_err_with(|| format!("Failed to write generated file {}", path.display()))
}
