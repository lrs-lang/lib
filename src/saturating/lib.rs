// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_saturating"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;

/// Objects that can be cast to another object in a saturating way.
///
/// = Remarks
///
/// This is used to cast between integer type so that `256.saturating_cast():u8 == 255`.
pub trait SaturatingCast<T> {
    fn saturating_cast(self) -> T;
}

// truncations that always preserve the value
macro_rules! trnc_up {
    ($src:ty as $($dst:ty),+) => {
        $(impl SaturatingCast<$dst> for $src {
            fn saturating_cast(self) -> $dst {
                self as $dst
            }
        })+
    }
}

trnc_up!(u8 as u8, u16, i16, u32, i32, u64, i64, usize, isize);
trnc_up!(i8 as i8, i16, i32, i64, isize);
trnc_up!(u16 as u16, u32, i32, u64, i64, usize, isize);
trnc_up!(i16 as i16, i32, i64, isize);
trnc_up!(u32 as u32, u64, i64, usize);
trnc_up!(i32 as i32, i64, isize);
trnc_up!(u64 as u64);
trnc_up!(i64 as i64);
trnc_up!(usize as u64, usize);
trnc_up!(isize as i64, isize);
#[cfg(target_pointer_width = "64")]
trnc_up!(u64 as usize);
#[cfg(target_pointer_width = "64")]
trnc_up!(i64 as isize);
#[cfg(target_pointer_width = "32")]
trnc_up!(usize as u32);
#[cfg(target_pointer_width = "32")]
trnc_up!(isize as i32);

// truncations of signed tyes to unsigned types of equal or larger width
macro_rules! trnc_s2u_up {
    ($src:ty as $($dst:ty),+) => {
        $(impl SaturatingCast<$dst> for $src {
            fn saturating_cast(self) -> $dst {
                if self >= 0 {
                    self as $dst
                } else {
                    0
                }
            }
        })+
    }
}

trnc_s2u_up!(i8 as u8, u16, u32, u64, usize);
trnc_s2u_up!(i16 as u16, u32, u64, usize);
trnc_s2u_up!(i32 as u32, u64, usize);
trnc_s2u_up!(i64 as u64);
trnc_s2u_up!(isize as usize);
#[cfg(target_pointer_width = "64")]
trnc_s2u_up!(i64 as usize);
#[cfg(target_pointer_width = "32")]
trnc_s2u_up!(isize as u32);

// truncation of signed types to types of smaller width
macro_rules! trnc_s_down {
    ($src:ty as $($dst:ident),+) => {
        $(impl SaturatingCast<$dst> for $src {
            fn saturating_cast(self) -> $dst {
                if self > $dst::max() as $src  {
                    $dst::max()
                } else if self < $dst::min() as $src {
                    $dst::min()
                } else {
                    self as $dst
                }
            }
        })+
    }
}

trnc_s_down!(i16 as u8, i8);
trnc_s_down!(i32 as u8, i8, u16, i16);
trnc_s_down!(i64 as u8, i8, u16, i16, u32, i32);
trnc_s_down!(isize as u8, i8, u16, i16);
#[cfg(target_pointer_width = "64")]
trnc_s_down!(isize as u32, i32);
#[cfg(target_pointer_width = "32")]
trnc_s_down!(i64 as usize, isize);

// truncation of unsigned types to types of equal or smaller width
macro_rules! trnc_u_down {
    ($src:ty as $($dst:ident),+) => {
        $(impl SaturatingCast<$dst> for $src {
            fn saturating_cast(self) -> $dst {
                if self > $dst::max() as $src  {
                    $dst::max()
                } else {
                    self as $dst
                }
            }
        })+
    }
}

trnc_u_down!(u16 as i8, u8, i16);
trnc_u_down!(u32 as i8, u8, i16, u16, i32);
trnc_u_down!(u64 as i8, u8, i16, u16, i32, u32, i64);
trnc_u_down!(usize as i8, u8, i16, u16, i32);
#[cfg(target_pointer_width = "64")]
trnc_u_down!(usize as u32, i64);
#[cfg(target_pointer_width = "32")]
trnc_u_down!(u64 as isize, usize);
