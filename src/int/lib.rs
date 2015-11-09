// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_int"]
#![crate_type = "lib"]
#![feature(plugin, prelude_import, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

use core::ops::{Range, RangeTo};

mod std { pub use ::core::std::*; }

/// Objects that can be interpreted as a bounded range.
pub trait BoundedRange<T> {
    /// Returns the range.
    fn to_range(self) -> Range<T>;
}

impl<T> BoundedRange<T> for Range<T> {
    fn to_range(self) -> Range<T> { self }
}

/// Integers.
pub trait Int {
    /// Returns whether the value is negative.
    fn negative(&self) -> bool;
    /// Casts the value to an `i64` and possibly discards significant bits.
    ///
    /// = Remarks
    ///
    /// For example, `u64::MAX.cast_i64() == -1`.
    fn cast_i64(&self) -> i64;
}

macro_rules! int_impl {
    ($name:ident) => {
        impl Int for $name {
            fn negative(&self) -> bool { $name::negative(*self) }
            fn cast_i64(&self) -> i64 { *self as i64 }
        }
    }
}

int_impl!(u8);
int_impl!(u16);
int_impl!(u32);
int_impl!(u64);
int_impl!(usize);
int_impl!(i8);
int_impl!(i16);
int_impl!(i32);
int_impl!(i64);
int_impl!(isize);

/// Unsigned integers.
pub trait UnsignedInt : Int+Sized {
    /// Calculates the next power of two greater or equal the current value.
    ///
    /// [return_value]
    /// The next power of two or `1` on overflow.
    fn next_power_of_two(&self) -> Self;
    /// Calculates the next power of two greater or equal the current value.
    ///
    /// [return_value]
    /// The next power of two or `None` on overflow.
    fn checked_next_power_of_two(&self) -> Option<Self>;
}

macro_rules! uint_impl {
    ($name:ident) => {
        impl UnsignedInt for $name {
            fn next_power_of_two(&self) -> $name {
                $name::next_power_of_two(*self)
            }

            fn checked_next_power_of_two(&self) -> Option<$name> {
                $name::checked_next_power_of_two(*self)
            }
        }

        impl BoundedRange<$name> for RangeTo<$name> {
            fn to_range(self) -> Range<$name> {
                Range { start: 0, end: self.end }
            }
        }
    }
}

uint_impl!(u8);
uint_impl!(u16);
uint_impl!(u32);
uint_impl!(u64);
uint_impl!(usize);

/// Signed integers.
pub trait SignedInt : Int {
}

macro_rules! sint_impl {
    ($name:ident) => {
        impl SignedInt for $name {
        }
    }
}

sint_impl!(i8);
sint_impl!(i16);
sint_impl!(i32);
sint_impl!(i64);
sint_impl!(isize);
