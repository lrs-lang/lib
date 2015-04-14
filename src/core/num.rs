// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ops::{
    Eq, Ord, Add, Sub, Mul, Div, Rem, BitOr, BitAnd, BitXor, Shl, Shr, Range, RangeFrom,
    RangeTo, Ordering
};
use iter::{Iterator, IntoIterator};
use option::{Option};
use option::Option::{Some, None};

macro_rules! int_impls {
    ($($t:ident)+) => {
        $(
            impl Eq for $t {
                fn eq(&self, other: &$t) -> bool { *self == *other }
            }

            impl Ord for $t {
                fn cmp(&self, other: &$t) -> Ordering {
                    if *self < *other {
                        Ordering::Less
                    } else if *self == *other {
                        Ordering::Equal
                    } else {
                        Ordering::Greater
                    }
                }
            }

            impl Add for $t {
                type Output = $t;
                fn add(self, other: $t) -> $t { self + other }
            }

            impl Sub for $t {
                type Output = $t;
                fn sub(self, other: $t) -> $t { self - other }
            }

            impl Mul for $t {
                type Output = $t;
                fn mul(self, other: $t) -> $t { self * other }
            }

            impl Div for $t {
                type Output = $t;
                fn div(self, other: $t) -> $t { self / other }
            }

            impl Rem for $t {
                type Output = $t;
                fn rem(self, other: $t) -> $t { self % other }
            }

            impl BitOr for $t {
                type Output = $t;
                fn bitor(self, other: $t) -> $t { self | other }
            }

            impl BitAnd for $t {
                type Output = $t;
                fn bitand(self, other: $t) -> $t { self & other }
            }

            impl BitXor for $t {
                type Output = $t;
                fn bitxor(self, other: $t) -> $t { self ^ other }
            }

            impl Shl<i8> for $t {
                type Output = $t;
                fn shl(self, other: i8) -> $t { self << other }
            }

            impl Shr<i8> for $t {
                type Output = $t;
                fn shr(self, other: i8) -> $t { self >> other }
            }

            impl Shl<u8> for $t {
                type Output = $t;
                fn shl(self, other: u8) -> $t { self << other }
            }

            impl Shr<u8> for $t {
                type Output = $t;
                fn shr(self, other: u8) -> $t { self >> other }
            }

            impl Shl<i16> for $t {
                type Output = $t;
                fn shl(self, other: i16) -> $t { self << other }
            }

            impl Shr<i16> for $t {
                type Output = $t;
                fn shr(self, other: i16) -> $t { self >> other }
            }

            impl Shl<u16> for $t {
                type Output = $t;
                fn shl(self, other: u16) -> $t { self << other }
            }

            impl Shr<u16> for $t {
                type Output = $t;
                fn shr(self, other: u16) -> $t { self >> other }
            }

            impl Shl<i32> for $t {
                type Output = $t;
                fn shl(self, other: i32) -> $t { self << other }
            }

            impl Shr<i32> for $t {
                type Output = $t;
                fn shr(self, other: i32) -> $t { self >> other }
            }

            impl Shl<u32> for $t {
                type Output = $t;
                fn shl(self, other: u32) -> $t { self << other }
            }

            impl Shr<u32> for $t {
                type Output = $t;
                fn shr(self, other: u32) -> $t { self >> other }
            }

            impl Shl<i64> for $t {
                type Output = $t;
                fn shl(self, other: i64) -> $t { self << other }
            }

            impl Shr<i64> for $t {
                type Output = $t;
                fn shr(self, other: i64) -> $t { self >> other }
            }

            impl Shl<u64> for $t {
                type Output = $t;
                fn shl(self, other: u64) -> $t { self << other }
            }

            impl Shr<u64> for $t {
                type Output = $t;
                fn shr(self, other: u64) -> $t { self >> other }
            }

            impl Shl<usize> for $t {
                type Output = $t;
                fn shl(self, other: usize) -> $t { self << other }
            }

            impl Shr<usize> for $t {
                type Output = $t;
                fn shr(self, other: usize) -> $t { self >> other }
            }

            impl Iterator for Range<$t> {
                type Item = $t;
                fn next(&mut self) -> Option<$t> {
                    if self.start < self.end {
                        self.start += 1;
                        Some(self.start - 1)
                    } else {
                        None
                    }
                }
            }

            impl Iterator for RangeFrom<$t> {
                type Item = $t;
                fn next(&mut self) -> Option<$t> {
                    if self.start < $t::MAX {
                        self.start += 1;
                        Some(self.start - 1)
                    } else {
                        None
                    }
                }
            }

            impl IntoIterator for RangeTo<$t> {
                type Item = $t;
                type IntoIter = Range<$t>;
                fn into_iter(self) -> Range<$t> { Range { start: $t::MIN, end: self.end } }
            }
        )+
    }
}

int_impls!(i8 u8 i16 u16 i32 u32 i64 u64 isize usize);

macro_rules! signed_int_modules {
    ($($t:ident $width:expr)+) => {
        $(
            pub mod $t {
                pub const MIN: $t = 1 << ($width - 1);
                pub const MAX: $t = !0 ^ MIN;
            }
        )+
    }
}

signed_int_modules!(i8 8 i16 16 i32 32 i64 64);

macro_rules! unsigned_int_modules {
    ($($t:ident)+) => {
        $(
            pub mod $t {
                pub const MIN: $t = 0;
                pub const MAX: $t = !0;
            }
        )+
    }
}

unsigned_int_modules!(u8 u16 u32 u64);

pub mod isize {
    #[cfg(target_pointer_width = "32")]
    pub const BITS: usize = 32;
    #[cfg(target_pointer_width = "64")]
    pub const BITS: usize = 64;
    pub const BYTES: usize = BITS / 8;
    pub const MIN: isize = 1 << (BITS - 1);
    pub const MAX: isize = !0 ^ MIN;
}

pub mod usize {
    #[cfg(target_pointer_width = "32")]
    pub const BITS: usize = 32;
    #[cfg(target_pointer_width = "64")]
    pub const BITS: usize = 64;
    pub const BYTES: usize = BITS / 8;
    pub const MIN: usize = 0;
    pub const MAX: usize = !0;
}
