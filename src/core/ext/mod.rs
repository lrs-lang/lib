// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ffi::{CString};
use std::path::{Path, AsPath};
use std::{self, ops};

use string::{AsLinuxStr};

pub use self::truncate::{SaturatingCast};

pub mod truncate;

pub trait AsLinuxPath {
    fn as_linux_path(&self) -> &Path;

    fn to_cstring(&self) -> Option<CString> {
        self.as_linux_path().as_os_str().to_cstring()
    }
}

impl<T: AsLinuxStr+?Sized> AsLinuxPath for T {
    fn as_linux_path(&self) -> &Path {
        self.as_linux_str().as_path()
    }
}

pub trait Int: PartialOrd + ops::Div<Output=Self> + ops::Rem<Output=Self> + Copy {
    fn zero() -> Self;
    fn max_value() -> Self;
    fn min_value() -> Self;
    fn is_negative(&self) -> bool;
    fn to_i64(&self) -> i64;
}

macro_rules! impl_int {
    ($t:ty, $max:expr, $min:expr) => {
        impl Int for $t {
            fn zero() -> $t { 0 }
            fn max_value() -> $t { $max }
            fn min_value() -> $t { $min }
            #[allow(unused_comparisons)]
            fn is_negative(&self) -> bool { *self < 0 }
            fn to_i64(&self) -> i64 { *self as i64 }
        }
    }
}

impl_int!(i8,    std::i8::MAX,    std::i8::MIN);
impl_int!(u8,    std::u8::MAX,    std::u8::MIN);
impl_int!(i16,   std::i16::MAX,   std::i16::MIN);
impl_int!(u16,   std::u16::MAX,   std::u16::MIN);
impl_int!(i32,   std::i32::MAX,   std::i32::MIN);
impl_int!(u32,   std::u32::MAX,   std::u32::MIN);
impl_int!(i64,   std::i64::MAX,   std::i64::MIN);
impl_int!(u64,   std::u64::MAX,   std::u64::MIN);
impl_int!(isize, std::isize::MAX, std::isize::MIN);
impl_int!(usize, std::usize::MAX, std::usize::MIN);

pub trait UnsignedInt: Int { }

impl UnsignedInt for u8    { }
impl UnsignedInt for u16   { }
impl UnsignedInt for u32   { }
impl UnsignedInt for u64   { }
impl UnsignedInt for usize { }

pub trait SignedInt: Int { }

impl SignedInt for i8    { }
impl SignedInt for i16   { }
impl SignedInt for i32   { }
impl SignedInt for i64   { }
impl SignedInt for isize { }

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

pub trait BoundedUIntRange<T: UnsignedInt>: UIntRange<T> { }

impl<T: UnsignedInt> BoundedUIntRange<T> for ops::Range<T> { }
impl<T: UnsignedInt> BoundedUIntRange<T> for ops::RangeTo<T> { }

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
