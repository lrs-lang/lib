// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::str::{self, FromStr};
use std::borrow::{Cow};
use std::os::unix::ffi::{OsStrExt};
use std::ffi::{OsStr, OsString, AsOsStr, CString, NulError};
use std::path::{Path, PathBuf, AsPath};
use std::num::{Int};
use std::{ops};

pub use self::truncate::{SaturatingCast};
pub use self::string::*;

pub mod truncate;
pub mod string;

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

pub trait AsLinuxPath {
    fn as_linux_path(&self) -> &Path;

    fn to_cstring(&self) -> Result<CString, NulError> {
        self.as_linux_path().to_cstring()
    }
}

impl<T: AsLinuxStr+?Sized> AsLinuxPath for T {
    fn as_linux_path(&self) -> &Path {
        self.as_linux_str().as_path()
    }
}

pub trait UnsignedInt: Int { }

impl UnsignedInt for u8    { }
impl UnsignedInt for u16   { }
impl UnsignedInt for u32   { }
impl UnsignedInt for u64   { }
impl UnsignedInt for usize { }

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

pub fn is_signed<T: Int>() -> bool {
    T::min_value() < T::zero()
}
