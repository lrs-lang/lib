// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ops::{
    Eq, Add, Sub, Mul, Div, Rem, BitOr, BitAnd, BitXor, Shl, Shr, Range, RangeFrom,
    RangeTo, Ordering, PartialOrd,
};
use marker::{Pod};
use cmp::{Ord};
use iter::{Iterator, IntoIterator};
use option::{Option};
use option::Option::{Some, None};
use intrinsics::{self};

macro_rules! int_impls {
    (
        $t:ident;
        as_i=$as_i:ident;
        as_u=$as_u:ident;
        $width:expr;
        $as_str:expr;
        signed=$signed:expr;
        $ctpop:ident;
        pop_ty=$pop_ty:ident;
        $ctlz:ident;
        $cttz:ident;
        $bswap:ident;
        size_t=$size_t:ident;
        $checked_add:ident;
        $checked_sub:ident;
        $checked_mul:ident;
     ) => {
        #[lang = $as_str]
        impl $t {
            /// Adds `other` and `self` without triggering overflow checking.
            pub fn wrapping_add(self, other: $t) -> $t {
                unsafe { intrinsics::overflowing_add(self, other) }
            }

            /// Like `wrapping_add`.
            pub fn wrapping_sub(self, other: $t) -> $t {
                unsafe { intrinsics::overflowing_sub(self, other) }
            }

            /// Like `wrapping_add`.
            pub fn wrapping_mul(self, other: $t) -> $t {
                unsafe { intrinsics::overflowing_mul(self, other) }
            }

            /// Adds `other` and `self` and returns the value if no overflow occurred.
            pub fn checked_add(self, other: $t) -> Option<$t> {
                unsafe {
                    match intrinsics::$checked_add(self as $size_t, other as $size_t) {
                        (val, false) => Some(val as $t),
                        _ => None,
                    }
                }
            }

            /// Like `checked_add`.
            pub fn checked_sub(self, other: $t) -> Option<$t> {
                unsafe {
                    match intrinsics::$checked_sub(self as $size_t, other as $size_t) {
                        (val, false) => Some(val as $t),
                        _ => None,
                    }
                }
            }

            /// Like `checked_add`.
            pub fn checked_mul(self, other: $t) -> Option<$t> {
                unsafe {
                    match intrinsics::$checked_mul(self as $size_t, other as $size_t) {
                        (val, false) => Some(val as $t),
                        _ => None,
                    }
                }
            }

            /// Adds `other` and `self` and returns the result if no overflow occurred or
            /// the maximum value in the direction in which the overflow occurred.
            ///
            /// # Example
            ///
            /// ```
            /// let x: u8 = 200;
            /// let y: u8 = 100;
            /// assert!(x.saturating_add(y) == 255);
            /// ```
            pub fn saturating_add(self, other: $t) -> $t {
                match self.checked_add(other) {
                    Some(val) => val,
                    _ => if other > 0 {
                        self::$t::MAX
                    } else {
                        self::$t::MIN
                    },
                }
            }

            /// Like `saturating_add`.
            pub fn saturating_sub(self, other: $t) -> $t {
                match self.checked_sub(other) {
                    Some(val) => val,
                    _ => if other > 0 {
                        self::$t::MIN
                    } else {
                        self::$t::MAX
                    },
                }
            }

            /// Calculates the next power of two greater or equal the current value.
            ///
            /// On overflow `1` is returned.
            pub fn next_power_of_two(self) -> $t {
                1 << (($width - self.wrapping_sub(1).leading_zeros()) % $width)
            }

            /// Like `next_power_of_two` but returns `None` on overflow.
            pub fn checked_next_power_of_two(self) -> Option<$t> {
                let npot = self.next_power_of_two();
                if npot < self {
                    None
                } else {
                    Some(npot)
                }
            }

            /// Returns whether this type is signed.
            pub fn signed(self) -> bool {
                $signed
            }

            /// Casts this type to the signed type of same width.
            pub fn as_signed(self) -> $as_i {
                self as $as_i
            }

            /// Like `as_signed`.
            pub fn as_unsigned(self) -> $as_u {
                self as $as_u
            }

            /// Returns the width in bits of this type.
            pub fn width(self) -> usize {
                $width
            }

            /// Returns whether `self` is negative.
            #[allow(unused_comparisons)]
            pub fn negative(self) -> bool {
                if $signed {
                    self < 0
                } else {
                    false
                }
            }

            /// Returns the absolute value of `self`.
            ///
            /// # Example
            ///
            /// ```
            /// let x: i8 = -128;
            /// assert!(x.abs() == -128);
            /// ```
            #[allow(unused_comparisons)]
            pub fn abs(self) -> $t {
                if $signed {
                    if self < 0 {
                        0 - self
                    } else {
                        self
                    }
                } else {
                    self
                }
            }

            /// Counts the bits set in `self`.
            pub fn count_ones(self) -> usize {
                unsafe { intrinsics::$ctpop(self as $pop_ty) as usize }
            }

            /// Like `count_ones`.
            pub fn count_zeroes(self) -> usize {
                (!self).count_ones()
            }

            /// Returns the length of longest sequence of set bits starting at the most
            /// significant bit.
            pub fn leading_ones(self) -> usize {
                (!self).leading_zeros()
            }

            /// Like `leading_ones`.
            pub fn leading_zeros(self) -> usize {
                unsafe { intrinsics::$ctlz(self as $pop_ty) as usize }
            }

            /// Like `leading_ones`.
            pub fn trailing_ones(self) -> usize {
                (!self).trailing_zeros()
            }

            /// Like `leading_ones`.
            pub fn trailing_zeros(self) -> usize {
                unsafe { intrinsics::$cttz(self as $pop_ty) as usize }
            }

            /// Swaps the bytes in `self`.
            pub fn swap(self) -> $t {
                unsafe { intrinsics::$bswap(self as $pop_ty) as $t }
            }

            /// Interprets `self` as a value in big-endian representation and returns
            /// the value in host-endian representation.
            ///
            /// # Example on a little-endian machine.
            ///
            /// ```
            /// let x: u16 = 0x0100;
            /// assert!(x.from_be() == 1);
            /// ```
            #[cfg(target_endian = "little")]
            pub fn from_be(self) -> $t {
                unsafe { intrinsics::$bswap(self as $pop_ty) as $t }
            }

            /// Interprets `self` as a value in big-endian representation and returns
            /// the value in host-endian representation.
            ///
            /// # Example on a little-endian machine.
            ///
            /// ```
            /// let x: u16 = 0x0100;
            /// assert!(x.from_be() == 1);
            /// ```
            #[cfg(target_endian = "big")]
            pub fn from_be(self) -> $t { self }

            /// Like `from_be`.
            #[cfg(target_endian = "little")]
            pub fn from_le(self) -> $t { self }

            /// Like `from_be`.
            #[cfg(target_endian = "big")]
            pub fn from_le(self) -> $t {
                unsafe { intrinsics::$bswap(self as $pop_ty) as $t }
            }

            /// Interprets `self` as a value in host-endian representation and returns the
            /// value in big-endian representation.
            #[cfg(target_endian = "little")]
            pub fn to_be(self) -> $t {
                unsafe { intrinsics::$bswap(self as $pop_ty) as $t }
            }

            /// Interprets `self` as a value in host-endian representation and returns the
            /// value in big-endian representation.
            #[cfg(target_endian = "big")]
            pub fn to_be(self) -> $t { self }

            /// Like `to_be`.
            #[cfg(target_endian = "little")]
            pub fn to_le(self) -> $t { self }

            /// Like `to_be`.
            #[cfg(target_endian = "big")]
            pub fn to_le(self) -> $t {
                unsafe { intrinsics::$bswap(self as $pop_ty) as $t }
            }

            /// Returns `(self / other, self % other)`.
            pub fn div_rem(self, other: $t) -> ($t, $t) {
                (self / other, self % other)
            }
        }

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

        impl PartialOrd for $t {
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                Some(self.cmp(other))
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

        impl Pod for $t { }
    }
}

int_impls!(i8    ; as_i=i8    ; as_u=u8    ; 8  ; "i8"    ; signed=true  ; ctpop8  ; pop_ty=u8  ; ctlz8  ; cttz8  ; bswap8  ; size_t=i8  ; i8_add_with_overflow  ; i8_sub_with_overflow   ; i8_mul_with_overflow ; );
int_impls!(u8    ; as_i=i8    ; as_u=u8    ; 8  ; "u8"    ; signed=false ; ctpop8  ; pop_ty=u8  ; ctlz8  ; cttz8  ; bswap8  ; size_t=u8  ; u8_add_with_overflow  ; u8_sub_with_overflow   ; u8_mul_with_overflow ; );
int_impls!(i16   ; as_i=i16   ; as_u=u16   ; 16 ; "i16"   ; signed=true  ; ctpop16 ; pop_ty=u16 ; ctlz16 ; cttz16 ; bswap16 ; size_t=i16 ; i16_add_with_overflow ; i16_sub_with_overflow  ; i16_mul_with_overflow; );
int_impls!(u16   ; as_i=i16   ; as_u=u16   ; 16 ; "u16"   ; signed=false ; ctpop16 ; pop_ty=u16 ; ctlz16 ; cttz16 ; bswap16 ; size_t=u16 ; u16_add_with_overflow ; u16_sub_with_overflow  ; u16_mul_with_overflow; );
int_impls!(i32   ; as_i=i32   ; as_u=u32   ; 32 ; "i32"   ; signed=true  ; ctpop32 ; pop_ty=u32 ; ctlz32 ; cttz32 ; bswap32 ; size_t=i32 ; i32_add_with_overflow ; i32_sub_with_overflow  ; i32_mul_with_overflow; );
int_impls!(u32   ; as_i=i32   ; as_u=u32   ; 32 ; "u32"   ; signed=false ; ctpop32 ; pop_ty=u32 ; ctlz32 ; cttz32 ; bswap32 ; size_t=u32 ; u32_add_with_overflow ; u32_sub_with_overflow  ; u32_mul_with_overflow; );
int_impls!(i64   ; as_i=i64   ; as_u=u64   ; 64 ; "i64"   ; signed=true  ; ctpop64 ; pop_ty=u64 ; ctlz64 ; cttz64 ; bswap64 ; size_t=i64 ; i64_add_with_overflow ; i64_sub_with_overflow  ; i64_mul_with_overflow; );
int_impls!(u64   ; as_i=i64   ; as_u=u64   ; 64 ; "u64"   ; signed=false ; ctpop64 ; pop_ty=u64 ; ctlz64 ; cttz64 ; bswap64 ; size_t=u64 ; u64_add_with_overflow ; u64_sub_with_overflow  ; u64_mul_with_overflow; );
#[cfg(target_pointer_width = "64")]
int_impls!(isize ; as_i=isize ; as_u=usize ; 64 ; "isize" ; signed=true  ; ctpop64 ; pop_ty=u64 ; ctlz64 ; cttz64 ; bswap64 ; size_t=i64 ; i64_add_with_overflow ; i64_sub_with_overflow  ; i64_mul_with_overflow; );
#[cfg(target_pointer_width = "64")]
int_impls!(usize ; as_i=isize ; as_u=usize ; 64 ; "usize" ; signed=false ; ctpop64 ; pop_ty=u64 ; ctlz64 ; cttz64 ; bswap64 ; size_t=u64 ; u64_add_with_overflow ; u64_sub_with_overflow  ; u64_mul_with_overflow; );
#[cfg(target_pointer_width = "32")]
int_impls!(isize ; as_i=isize ; as_u=usize ; 32 ; "isize" ; signed=true  ; ctpop32 ; pop_ty=u32 ; ctlz32 ; cttz32 ; bswap32 ; size_t=i32 ; i32_add_with_overflow ; i32_sub_with_overflow  ; i32_mul_with_overflow; );
#[cfg(target_pointer_width = "32")]
int_impls!(usize ; as_i=isize ; as_u=usize ; 32 ; "usize" ; signed=false ; ctpop32 ; pop_ty=u32 ; ctlz32 ; cttz32 ; bswap32 ; size_t=u32 ; u32_add_with_overflow ; u32_sub_with_overflow  ; u32_mul_with_overflow; );

macro_rules! signed_int_modules {
    ($($t:ident $width:expr)+) => {
        $(
            pub mod $t {
                pub const MIN: $t = 1 << ($width - 1);
                pub const MAX: $t = !MIN;
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
    pub const MAX: isize = !MIN;
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
