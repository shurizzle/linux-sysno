mod format;
mod generate;
mod header;
mod kernel_org;
mod table;

use std::{cell::RefCell, fmt, fs::File, io::Write, path::Path, rc::Rc};

use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use format::Formatter;
use generate::{build_enum, SysnoEnum};
use kernel_org::fetch_file;

use crate::kernel_org::latest_version;

const __ARM_NR_BASE: usize = 0x0f0000;
const __X32_SYSCALL_BIT: usize = 0x40000000;

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

    let generic = header::from_reader(fetch_file(&version, "include/uapi/asm-generic/unistd.h")?)
        .filter(|l| {
            if let Ok(l) = l {
                l.0.as_ref() != "sync_file_range"
            } else {
                true
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut aarch64 = generic.clone();

    let generic = build_enum(generic.into_iter().map(Ok))?;
    write_file(generic, formatter, base, "generic")?;

    write_file(
        {
            aarch64.push(("cacheflush".to_string().into_boxed_str(), __ARM_NR_BASE + 2));
            aarch64.push(("set_tls".to_string().into_boxed_str(), __ARM_NR_BASE + 5));
            build_enum(aarch64.into_iter().map(Ok))?
        },
        formatter,
        base,
        "aarch64",
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

    {
        let x32 = Rc::new(RefCell::new(Vec::new()));
        write_file(
            build_enum({
                let x32 = x32.clone();
                table::from_reader(fetch_file(
                    &version,
                    "arch/x86/entry/syscalls/syscall_64.tbl",
                )?)
                .filter_map(move |l| match l {
                    Ok((no, abi, name)) => {
                        let abi = <Rc<Box<str>> as AsRef<Box<str>>>::as_ref(&abi).as_ref();

                        if abi == ABI::COMMON.0 {
                            let no = ABI::COMMON.adjust(no);
                            Rc::as_ref(&x32)
                                .borrow_mut()
                                .push((name.clone(), no | __X32_SYSCALL_BIT));
                            Some(Ok((name, no)))
                        } else if abi == ABI::B64.0 {
                            Some(Ok((name, ABI::B64.adjust(no))))
                        } else if abi == ABI::X32.0 {
                            Rc::as_ref(&x32)
                                .borrow_mut()
                                .push((name, no | __X32_SYSCALL_BIT));
                            None
                        } else {
                            None
                        }
                    }
                    Err(err) => Some(Err(err)),
                })
            })?,
            formatter,
            base,
            "x86_64",
        )?;

        let x32 = Rc::try_unwrap(x32)
            .expect("x32 should not be borrowed")
            .into_inner();
        write_file(build_enum(x32.into_iter().map(Ok))?, formatter, base, "x32")?;
    }

    write_file(
        build_enum(
            table_iter(&version, "arch/arm/tools/syscall.tbl", [ABI::COMMON])?.chain(
                [
                    ("breakpoint", __ARM_NR_BASE + 1),
                    ("cacheflush", __ARM_NR_BASE + 2),
                    ("usr26", __ARM_NR_BASE + 3),
                    ("usr32", __ARM_NR_BASE + 4),
                    ("set_tls", __ARM_NR_BASE + 5),
                    ("get_tls", __ARM_NR_BASE + 6),
                ]
                .into_iter()
                .map(|(name, no)| Ok((name.to_string().into_boxed_str(), no))),
            ),
        )?,
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
pub struct ABI<'a>(&'a str, fn(usize) -> usize);

impl<'a> ABI<'a> {
    pub const COMMON: Self = Self("common", |n| n);
    pub const I386: Self = Self("i386", |n| n);
    pub const NOSPU: Self = Self("nospu", |n| n);
    pub const B32: Self = Self("32", |n| n);
    pub const B64: Self = Self("64", |n| n);
    pub const X32: Self = Self("x32", |n| n);
    pub const O32: Self = Self("o32", |n| n + 4000);
    pub const N64: Self = Self("n64", |n| n + 5000);

    #[inline]
    pub fn adjust(&self, n: usize) -> usize {
        (self.1)(n)
    }
}

fn table_enum<'a, T1, T2, T3>(version: T1, path: T2, abis: T3) -> Result<SysnoEnum>
where
    T1: fmt::Display,
    T2: fmt::Display,
    T3: AsRef<[ABI<'a>]> + 'a,
{
    build_enum(table_iter(version, path, abis)?)
}

fn table_iter<'a, T1, T2, T3>(
    version: T1,
    path: T2,
    abis: T3,
) -> Result<impl Iterator<Item = std::io::Result<(Box<str>, usize)>> + 'a>
where
    T1: fmt::Display,
    T2: fmt::Display,
    T3: AsRef<[ABI<'a>]> + 'a,
{
    Ok(
        table::from_reader(fetch_file(version, path)?).filter_map(move |l| match l {
            Ok((no, abi, name)) => {
                let abi = <Rc<Box<str>> as AsRef<Box<str>>>::as_ref(&abi).as_ref();
                abis.as_ref()
                    .iter()
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
    path.push(&name);

    let mut f = File::create(&path)
        .wrap_err_with(|| format!("Failed to write generated file {}", path.display()))?;
    writeln!(f, "//! Syscalls for arch `{}`.\n", unsafe {
        name.get_unchecked(..(name.len() - 3))
    })
    .wrap_err_with(|| format!("Failed to write generated file {}", path.display()))?;
    f.write_all(content.as_bytes())
        .wrap_err_with(|| format!("Failed to write generated file {}", path.display()))
}
