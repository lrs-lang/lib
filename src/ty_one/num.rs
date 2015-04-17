// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
use core::{mem};

pub trait Int {
    fn negative(&self) -> bool;
    fn cast_i64(&self) -> i64;
}

macro_rules! int_impl {
    ($name:ident) => {
        impl Int for $name {
            fn negative(&self) -> bool { (*self).negative() }
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

pub trait UnsignedInt : Int {
    fn next_power_of_two(&self) -> Self;
    fn checked_next_power_of_two(&self) -> Option<Self>;
}

macro_rules! uint_impl {
    ($name:ident) => {
        impl UnsignedInt for $name {
            fn next_power_of_two(&self) -> $name {
                let bits = mem::size_of::<$name>() * 8;
                1 << ((bits - self.wrapping_sub(1).leading_zeros()) % bits)
            }

            fn checked_next_power_of_two(&self) -> Option<$name> {
                let npot = self.next_power_of_two();
                if npot < *self {
                    None
                } else {
                    Some(npot)
                }
            }
        }
    }
}

uint_impl!(u8);
uint_impl!(u16);
uint_impl!(u32);
uint_impl!(u64);
uint_impl!(usize);

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
