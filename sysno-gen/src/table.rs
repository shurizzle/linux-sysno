use std::{
    borrow::Borrow,
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    rc::Rc,
};

fn parse_line(line: &str) -> Option<(usize, &str, &str)> {
    let line = if let Some(pos) = memchr::memchr(b'#', line.as_bytes()) {
        unsafe { line.get_unchecked(..pos) }
    } else {
        line
    }
    .trim();

    if line.is_empty() {
        return None;
    }

    let mut it = line.split_whitespace();

    let no = it.next()?.parse::<usize>().ok()?;
    let abi = it.next()?;
    if abi.is_empty() {
        return None;
    }
    let name = it.next()?;
    if name.is_empty() {
        return None;
    }

    Some((no, abi, name))
}

#[derive(Clone, PartialEq, PartialOrd, Hash, Eq)]
struct Key(Rc<Box<str>>);
impl Borrow<str> for Key {
    #[inline]
    fn borrow(&self) -> &str {
        <Rc<Box<str>> as AsRef<Box<str>>>::as_ref(&self.0).as_ref()
    }
}
impl Key {
    #[inline]
    pub fn into_inner(self) -> Rc<Box<str>> {
        self.0
    }
}

pub struct TableReader<T> {
    inner: T,
    buf: String,
    cache: HashMap<Key, ()>,
}

impl<T: BufRead> Iterator for TableReader<T> {
    type Item = std::io::Result<(usize, Rc<Box<str>>, Box<str>)>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.read_line(&mut self.buf) {
                Err(err) => return Some(Err(err)),
                Ok(0) => return None,
                Ok(_) => (),
            }

            let res = parse_line(self.buf.as_str()).map(|(no, abi, name)| {
                let name = name.to_string().into_boxed_str();

                let abi = if let Some((key, _)) = self.cache.get_key_value(abi) {
                    key.clone().into_inner()
                } else {
                    let abi = Key(Rc::new(abi.to_string().into_boxed_str()));
                    self.cache.insert(abi.clone(), ());
                    abi.into_inner()
                };

                Ok((no, abi, name))
            });
            self.buf.clear();

            if res.is_some() {
                return res;
            }
        }
    }
}

#[inline]
pub fn from_buf_reader<T: BufRead>(reader: T) -> TableReader<T> {
    TableReader {
        inner: reader,
        buf: String::new(),
        cache: HashMap::new(),
    }
}

#[inline]
pub fn from_reader<T: Read>(reader: T) -> TableReader<BufReader<T>> {
    from_buf_reader(BufReader::new(reader))
}
