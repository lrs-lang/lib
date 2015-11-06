// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_wrapping"]
#![crate_type = "lib"]
#![feature(plugin, no_std, const_fn)]
#![plugin(lrs_core_plugin)]
#![no_std]

use core::ops::{
    Eq, Add, Sub, Mul, Div, Rem, BitOr, BitAnd, BitXor, Shl, Shr,
    Ordering, PartialOrd, Deref, DerefMut,
};
use core::marker::{Pod, Copy};
use core::cmp::{Ord};
use core::option::{Option};
use core::option::Option::{Some};

macro_rules! wint_impls {
    (
        $t:ident;
        $raw_t:ident
     ) => {
        pub struct $t(pub $raw_t);

        impl $t {
            /// Adds another integer to this one and returns the value if no overflow
            /// occurred.
            ///
            /// [argument, other]
            /// The integer that will be added to this one.
            ///
            /// [return_value]
            /// Returns the sum if no overflow occurred, `None` otherwise.
            pub fn checked_add(self, other: $raw_t) -> Option<$t> {
                self.0.checked_add(other).map(|r| $t(r))
            }

            /// Subtracts another integer from this one and returns the value if no
            /// overflow occurred.
            ///
            /// [argument, other]
            /// The integer that will be subtracted from this one.
            ///
            /// [return_value]
            /// Returns the difference if no overflow occurred, `None` otherwise.
            pub fn checked_sub(self, other: $raw_t) -> Option<$t> {
                self.0.checked_sub(other).map(|r| $t(r))
            }

            /// Multiplies this integer by another one and returns the value if no
            /// overflow occurred.
            ///
            /// [argument, other]
            /// The integer that will be multiplied.
            ///
            /// [return_value]
            /// Returns the product if no overflow occurred, `None` otherwise.
            pub fn checked_mul(self, other: $raw_t) -> Option<$t> {
                self.0.checked_mul(other).map(|r| $t(r))
            }

            /// Adds another integer to this one without overflow, trimming the value to
            /// the range that can be represented in this type.
            ///
            /// = Examples
            ///
            /// ----
            /// let x: u8 = 200;
            /// let y: u8 = 100;
            /// assert!(x.saturating_add(y) == 255);
            /// ----
            pub fn saturating_add(self, other: $raw_t) -> $t {
                $t(self.0.saturating_add(other))
            }

            /// Subtracts another integer to this one without overflow, trimming the value
            /// to the range that can be represented in this type.
            pub fn saturating_sub(self, other: $raw_t) -> $t {
                $t(self.0.saturating_sub(other))
            }

            /// Calculates the next power of two greater or equal the current value.
            ///
            /// [return_value]
            /// Returns the next power of two or `1` on overflow.
            pub fn next_power_of_two(self) -> $t {
                $t(self.0.next_power_of_two())
            }

            /// Calculates the next power of two greater or equal the current value.
            ///
            /// [return_value]
            /// Returns the next power of two or `None` on overflow.
            pub fn checked_next_power_of_two(self) -> Option<$t> {
                self.0.checked_next_power_of_two().map(|r| $t(r))
            }

            /// Counts the set bits in this integer.
            pub fn count_ones(self) -> usize {
                self.0.count_ones()
            }

            /// Counts the unset bits in this integer.
            pub fn count_zeros(self) -> usize {
                self.0.count_zeros()
            }

            /// Returns the length of longest sequence of set bits starting at the most
            /// significant bit.
            pub fn leading_ones(self) -> usize {
                self.0.leading_ones()
            }

            /// Returns the length of longest sequence of unset bits starting at the most
            /// significant bit.
            pub fn leading_zeros(self) -> usize {
                self.0.leading_zeros()
            }

            /// Returns the length of longest sequence of set bits starting at the least
            /// significant bit.
            pub fn trailing_ones(self) -> usize {
                self.0.trailing_ones()
            }

            /// Returns the length of longest sequence of unset bits starting at the least
            /// significant bit.
            pub fn trailing_zeros(self) -> usize {
                self.0.trailing_zeros()
            }

            /// Swaps the bytes in this integer.
            pub fn swap(self) -> $t {
                $t(self.0.swap())
            }

            /// Interprets this integer as a value in big-endian representation and
            /// returns the value in host-endian representation.
            ///
            /// = Examples
            ///
            /// This example shows the behavior on a little-endian machine.
            ///
            /// ----
            /// let x: u16 = 0x0100;
            /// assert!(x.from_be() == 1);
            /// ----
            pub fn from_be(self) -> $t {
                $t(self.0.from_be())
            }

            /// Interprets this integer as a value in little-endian representation and
            /// returns the value in host-endian representation.
            ///
            /// = Examples
            ///
            /// This example shows the behavior on a little-endian machine.
            ///
            /// ----
            /// let x: u16 = 0x0001;
            /// assert!(x.from_be() == 1);
            /// ----
            pub fn from_le(self) -> $t {
                $t(self.0.from_le())
            }

            /// Interprets this integer as a value in host-endian representation and
            /// returns the value in big-endian representation.
            pub fn to_be(self) -> $t {
                $t(self.0.to_be())
            }

            /// Interprets this integer as a value in host-endian representation and
            /// returns the value in little-endian representation.
            pub fn to_le(self) -> $t {
                $t(self.0.to_le())
            }

            /// Divides this integer by another one and returns both the quotient and the
            /// remainder.
            ///
            /// [argument, other]
            /// The divisor.
            ///
            /// [return_value]
            /// Returns the quotient and the remainder.
            pub fn div_rem(self, other: $raw_t) -> ($t, $t) {
                ($t(self.0 / other), $t(self.0 % other))
            }

            /// Rotates the value to the right.
            ///
            /// [argument, bits]
            /// The number of bits to rotate.
            ///
            /// = Remarks
            ///
            /// The bits argument must be in the range `(0, width)`, otherwise the
            /// behavior is undefined. Here, width is the width of the type in bits.
            ///
            /// This method is implemented for signed types due to a compiler limitation.
            /// It should only be used on unsigned values.
            ///
            /// = Examples
            ///
            /// ----
            /// let x = 1u32;
            /// assert!(x.rotate_right(1) == 0x8000_0000);
            /// ----
            pub fn rotate_right(self, bits: usize) -> $t {
                $t(self.0.rotate_right(bits))
            }

            /// Rotates the value to the right.
            ///
            /// [argument, bits]
            /// The number of bits to rotate.
            ///
            /// = Remarks
            ///
            /// The bits argument must be in the range `(0, width)`, otherwise the
            /// behavior is undefined. Here, width is the width of the type in bits.
            ///
            /// This method is implemented for signed types due to a compiler limitation.
            /// It should only be used on unsigned values.
            ///
            /// = Examples
            ///
            /// ----
            /// let x = 1u32;
            /// assert!(x.rotate_left(1) == 3);
            /// ----
            pub fn rotate_left(self, bits: usize) -> $t {
                $t(self.0.rotate_left(bits))
            }

            /// Returns the minimum value of this type.
            pub const fn min() -> $t {
                $t($raw_t::min())
            }

            /// Returns the maximum value of this type.
            pub const fn max() -> $t {
                $t($raw_t::max())
            }

            /// Returns the bit-width of this type.
            pub const fn bits() -> usize {
                $raw_t::bits()
            }

            /// Returns the byte-width of this type.
            pub const fn bytes() -> usize {
                $raw_t::bytes()
            }
        }

        impl Eq for $t {
            fn eq(&self, other: &$t) -> bool { self.0 == other.0 }
        }

        impl Eq<$raw_t> for $t {
            fn eq(&self, other: &$raw_t) -> bool { self.0 == *other }
        }

        impl Ord for $t {
            fn cmp(&self, other: &$t) -> Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl PartialOrd for $t {
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Add for $t {
            type Output = $t;
            fn add(self, other: $t) -> $t { $t(self.0.wrapping_add(other.0)) }
        }

        impl Add<$raw_t> for $t {
            type Output = $t;
            fn add(self, other: $raw_t) -> $t { $t(self.0.wrapping_add(other)) }
        }

        impl Sub for $t {
            type Output = $t;
            fn sub(self, other: $t) -> $t { $t(self.0.wrapping_sub(other.0)) }
        }

        impl Sub<$raw_t> for $t {
            type Output = $t;
            fn sub(self, other: $raw_t) -> $t { $t(self.0.wrapping_sub(other)) }
        }

        impl Mul for $t {
            type Output = $t;
            fn mul(self, other: $t) -> $t { $t(self.0.wrapping_mul(other.0)) }
        }

        impl Mul<$raw_t> for $t {
            type Output = $t;
            fn mul(self, other: $raw_t) -> $t { $t(self.0.wrapping_mul(other)) }
        }

        impl Div for $t {
            type Output = $t;
            fn div(self, other: $t) -> $t { $t(self.0 / other.0) }
        }

        impl Div<$raw_t> for $t {
            type Output = $t;
            fn div(self, other: $raw_t) -> $t { $t(self.0 / other) }
        }

        impl Rem for $t {
            type Output = $t;
            fn rem(self, other: $t) -> $t { $t(self.0 % other.0) }
        }

        impl Rem<$raw_t> for $t {
            type Output = $t;
            fn rem(self, other: $raw_t) -> $t { $t(self.0 % other) }
        }

        impl BitOr for $t {
            type Output = $t;
            fn bitor(self, other: $t) -> $t { $t(self.0 | other.0) }
        }

        impl BitOr<$raw_t> for $t {
            type Output = $t;
            fn bitor(self, other: $raw_t) -> $t { $t(self.0 | other) }
        }

        impl BitAnd for $t {
            type Output = $t;
            fn bitand(self, other: $t) -> $t { $t(self.0 & other.0) }
        }

        impl BitAnd<$raw_t> for $t {
            type Output = $t;
            fn bitand(self, other: $raw_t) -> $t { $t(self.0 & other) }
        }

        impl BitXor for $t {
            type Output = $t;
            fn bitxor(self, other: $t) -> $t { $t(self.0 ^ other.0) }
        }

        impl BitXor<$raw_t> for $t {
            type Output = $t;
            fn bitxor(self, other: $raw_t) -> $t { $t(self.0 ^ other) }
        }

        impl Shl<i8> for $t {
            type Output = $t;
            fn shl(self, other: i8) -> $t { $t(self.0 << other) }
        }

        impl Shr<i8> for $t {
            type Output = $t;
            fn shr(self, other: i8) -> $t { $t(self.0 >> other) }
        }

        impl Shl<u8> for $t {
            type Output = $t;
            fn shl(self, other: u8) -> $t { $t(self.0 << other) }
        }

        impl Shr<u8> for $t {
            type Output = $t;
            fn shr(self, other: u8) -> $t { $t(self.0 >> other) }
        }

        impl Shl<i16> for $t {
            type Output = $t;
            fn shl(self, other: i16) -> $t { $t(self.0 << other) }
        }

        impl Shr<i16> for $t {
            type Output = $t;
            fn shr(self, other: i16) -> $t { $t(self.0 >> other) }
        }

        impl Shl<u16> for $t {
            type Output = $t;
            fn shl(self, other: u16) -> $t { $t(self.0 << other) }
        }

        impl Shr<u16> for $t {
            type Output = $t;
            fn shr(self, other: u16) -> $t { $t(self.0 >> other) }
        }

        impl Shl<i32> for $t {
            type Output = $t;
            fn shl(self, other: i32) -> $t { $t(self.0 << other) }
        }

        impl Shr<i32> for $t {
            type Output = $t;
            fn shr(self, other: i32) -> $t { $t(self.0 >> other) }
        }

        impl Shl<u32> for $t {
            type Output = $t;
            fn shl(self, other: u32) -> $t { $t(self.0 << other) }
        }

        impl Shr<u32> for $t {
            type Output = $t;
            fn shr(self, other: u32) -> $t { $t(self.0 >> other) }
        }

        impl Shl<i64> for $t {
            type Output = $t;
            fn shl(self, other: i64) -> $t { $t(self.0 << other) }
        }

        impl Shr<i64> for $t {
            type Output = $t;
            fn shr(self, other: i64) -> $t { $t(self.0 >> other) }
        }

        impl Shl<u64> for $t {
            type Output = $t;
            fn shl(self, other: u64) -> $t { $t(self.0 << other) }
        }

        impl Shr<u64> for $t {
            type Output = $t;
            fn shr(self, other: u64) -> $t { $t(self.0 >> other) }
        }

        impl Shl<usize> for $t {
            type Output = $t;
            fn shl(self, other: usize) -> $t { $t(self.0 << other) }
        }

        impl Shr<usize> for $t {
            type Output = $t;
            fn shr(self, other: usize) -> $t { $t(self.0 >> other) }
        }

        impl Pod for $t { }

        impl Copy for $t { }

        impl Deref for $t {
            type Target = $raw_t;
            fn deref(&self) -> &$raw_t {
                &self.0
            }
        }

        impl DerefMut for $t {
            fn deref_mut(&mut self) -> &mut $raw_t {
                &mut self.0
            }
        }
    }
}

wint_impls!(W8; u8);
wint_impls!(W16; u16);
wint_impls!(W32; u32);
wint_impls!(W64; u64);
wint_impls!(Wsize; usize);
