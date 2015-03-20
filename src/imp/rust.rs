use std::str::{self, FromStr};
use std::borrow::{Cow};
use std::os::unix::{OsStrExt};
use std::ffi::{OsStr, OsString, AsOsStr};
use std::path::{Path, PathBuf, AsPath};
use std::num::{UnsignedInt};
use std::{ops};

pub trait AsStr {
    fn as_str(&self) -> Option<&str>;
    fn as_str_lossy<'a>(&'a self) -> Cow<'a, str>;
}

impl AsStr for [u8] {
    fn as_str(&self) -> Option<&str> {
        str::from_utf8(self).ok()
    }

    fn as_str_lossy<'a>(&'a self) -> Cow<'a, str> {
        String::from_utf8_lossy(self)
    }
}

pub trait AsLinuxStr {
    fn as_linux_str(&self) -> &OsStr;
    
    fn as_bytes(&self) -> &[u8] {
        <OsStr as OsStrExt>::as_bytes(self.as_linux_str())
    }
}

impl<'a, T: AsLinuxStr+?Sized> AsLinuxStr for &'a T {
    fn as_linux_str(&self) -> &OsStr {
        (*self).as_linux_str()
    }
}

impl AsLinuxStr for OsStr    { fn as_linux_str(&self) -> &OsStr { self.as_os_str() } }
impl AsLinuxStr for OsString { fn as_linux_str(&self) -> &OsStr { self.as_os_str() } }
impl AsLinuxStr for str      { fn as_linux_str(&self) -> &OsStr { self.as_os_str() } }
impl AsLinuxStr for String   { fn as_linux_str(&self) -> &OsStr { self.as_os_str() } }
impl AsLinuxStr for Path     { fn as_linux_str(&self) -> &OsStr { self.as_os_str() } }
impl AsLinuxStr for PathBuf  { fn as_linux_str(&self) -> &OsStr { self.as_os_str() } }

impl AsLinuxStr for [u8] {
    fn as_linux_str(&self) -> &OsStr {
        OsStr::from_bytes(self)
    }
}

pub trait AsLinuxPath {
    fn as_linux_path(&self) -> &Path;
}

impl<T: AsLinuxStr+?Sized> AsLinuxPath for T {
    fn as_linux_path(&self) -> &Path {
        self.as_linux_str().as_path()
    }
}

pub enum ParseErr<E> {
    NotUtf8,
    Err(E),
}

pub trait ByteSliceExt {
    fn parse<F: FromStr>(&self) -> ::std::result::Result<F, ParseErr<<F as FromStr>::Err>>;
}

impl<'a> ByteSliceExt for &'a [u8] {
    fn parse<F: FromStr>(&self) -> ::std::result::Result<F, ParseErr<<F as FromStr>::Err>> {
        let st = match ::std::str::from_utf8(self) {
            Ok(st) => st,
            Err(_) => return Err(ParseErr::NotUtf8),
        };
        match st.parse() {
            Ok(r) => Ok(r),
            Err(e) => Err(ParseErr::Err(e)),
        }
    }
}

pub trait UIntRange<T: UnsignedInt> {
    fn to_range(self) -> ops::Range<T>;
}

impl<T: UnsignedInt> UIntRange<T> for ops::Range<T> {
    fn to_range(self) -> ops::Range<T> { self }
}

impl<T: UnsignedInt> UIntRange<T> for ops::RangeTo<T> {
    fn to_range(self) -> ops::Range<T> { ops::Range { start: T::zero(), end: self.end } }
}

impl<T: UnsignedInt> UIntRange<T> for ops::RangeFrom<T> {
    fn to_range(self) -> ops::Range<T> { ops::Range { start: self.start, end: T::max_value() } }
}

impl<T: UnsignedInt> UIntRange<T> for ops::RangeFull {
    fn to_range(self) -> ops::Range<T> { ops::Range { start: T::zero(), end: T::max_value() } }
}

pub trait IteratorExt2: Iterator {
    fn collect_into_opt(&mut self, mut buf: &mut [Option<<Self as Iterator>::Item>]) -> usize {
        let mut count = 0;
        while buf.len() > 0 {
            let tmp = buf;
            tmp[0] = self.next();
            if tmp[0].is_none() {
                break;
            }
            count += 1;
            buf = &mut tmp[1..];
        }
        count
    }

    fn collect_into(&mut self, mut buf: &mut [<Self as Iterator>::Item]) -> usize {
        let mut count = 0;
        while buf.len() > 0 {
            let tmp = buf;
            tmp[0] = match self.next() {
                Some(v) => v,
                _ => break,
            };
            count += 1;
            buf = &mut tmp[1..];
        }
        count
    }
}

impl<T: Iterator> IteratorExt2 for T { }
