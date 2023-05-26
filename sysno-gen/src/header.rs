use std::io::{BufRead, BufReader, Read};

use atoi::FromRadix10;

fn parse_line(line: &str) -> Option<(Box<str>, usize)> {
    fn space0(b: &str) -> &str {
        b.trim_start()
    }

    fn space1(b: &str) -> Option<&str> {
        if b.chars().next()?.is_whitespace() {
            Some(b.trim_start())
        } else {
            None
        }
    }

    fn int<T: FromRadix10>(b: &str) -> Option<(T, &str)> {
        let (res, size) = T::from_radix_10(b.as_bytes());
        if size == 0 {
            None
        } else {
            Some((res, unsafe { b.get_unchecked(size..) }))
        }
    }

    fn id(b: &str) -> Option<(Box<str>, &str)> {
        b.char_indices()
            .find(|&(i, c)| {
                if i == 0 {
                    !c.is_ascii_alphabetic() && c != '_'
                } else {
                    !c.is_ascii_alphanumeric() && c != '_'
                }
            })
            .and_then(|(i, _)| unsafe {
                let name = b.get_unchecked(..i);

                let name = if name.starts_with("__NR_") {
                    Some(name.get_unchecked(5..))
                } else if name.starts_with("__NR3264_") {
                    Some(name.get_unchecked(9..))
                } else {
                    None
                };

                if let Some(name) = name {
                    if name == "syscalls" || name == "arch_specific_syscall" {
                        None
                    } else {
                        Some((name.to_string().into_boxed_str(), b.get_unchecked(i..)))
                    }
                } else {
                    None
                }
            })
            .or_else(|| {
                if b.is_empty() {
                    None
                } else {
                    Some((b.to_string().into_boxed_str(), unsafe {
                        b.get_unchecked(b.len()..)
                    }))
                }
            })
    }

    fn end(b: &str) -> Option<()> {
        let b = b.trim();
        if b.is_empty() {
            Some(())
        } else if let Some(b) = b.strip_prefix("/*") {
            let b = b.strip_suffix("*/")?.trim();
            if b.is_empty() {
                Some(())
            } else {
                None
            }
        } else if b.starts_with("//") {
            Some(())
        } else {
            None
        }
    }

    let line = space0(line);
    let line = line.strip_prefix('#')?;
    let line = space0(line);
    if let Some(line) = line.strip_prefix("define") {
        let line = space1(line)?;
        let (name, line) = id(line)?;
        let line = space1(line)?;

        if let Some((value, line)) = int::<usize>(line) {
            end(line)?;
            Some((name, value))
        } else {
            None
        }
    } else {
        None
    }
}

pub struct SysnoReader<T> {
    inner: T,
    buf: String,
}

impl<T: BufRead> Iterator for SysnoReader<T> {
    type Item = std::io::Result<(Box<str>, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.read_line(&mut self.buf) {
                Err(err) => return Some(Err(err)),
                Ok(len) => {
                    if len == 0 {
                        return None;
                    }
                }
            }

            let res = parse_line(self.buf.as_str());
            self.buf.clear();

            if let Some(res) = res {
                return Some(Ok(res));
            }
        }
    }
}

#[inline]
pub fn from_buf_reader<T: BufRead>(reader: T) -> SysnoReader<T> {
    SysnoReader {
        inner: reader,
        buf: String::new(),
    }
}

#[inline]
pub fn from_reader<T: Read>(reader: T) -> SysnoReader<BufReader<T>> {
    from_buf_reader(BufReader::new(reader))
}
