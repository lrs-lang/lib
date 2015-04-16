// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
use core::{mem};

pub trait UnsignedInt {
    fn next_power_of_two(self) -> Self;
    fn checked_next_power_of_two(self) -> Option<Self>;
}

macro_rules! uint_impl {
    ($name:ident) => {
        impl UnsignedInt for $name {
            fn next_power_of_two(self) -> $name {
                let bits = mem::size_of::<$name>() * 8;
                1 << ((bits - self.wrapping_sub(1).leading_zeros()) % bits)
            }

            fn checked_next_power_of_two(self) -> Option<$name> {
                let npot = self.next_power_of_two();
                if npot < self {
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
