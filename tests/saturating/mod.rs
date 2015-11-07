// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// TODO: Many cases still missing.

use std::num::{SaturatingCast};

// truncations that always preserve the value
macro_rules! trnc_up {
    ($src:ident as $($dst:ty),+; $name:ident) => {
        #[test] fn $name() {
            $(
            for i in $src::min()..$src::min()+127 {
                test!(SaturatingCast::<$dst>::saturating_cast(i) == i as $dst);
            }
            for i in $src::max()-127..$src::max() {
                test!(SaturatingCast::<$dst>::saturating_cast(i) == i as $dst);
            }
            )+
        }
    }
}

trnc_up!(u8 as u8, u16, i16, u32, i32, u64, i64, usize, isize; u8_trnc_up);
trnc_up!(i8 as i8, i16, i32, i64, isize; i8_trnc_up);
trnc_up!(u16 as u16, u32, i32, u64, i64, usize, isize; u16_trnc_up);
trnc_up!(i16 as i16, i32, i64, isize; i16_trnc_up);
trnc_up!(u32 as u32, u64, i64, usize; u32_trnc_up);
trnc_up!(i32 as i32, i64, isize; i32_trnc_up);
trnc_up!(u64 as u64; u64_trnc_up);
trnc_up!(i64 as i64; i64_trnc_up);
trnc_up!(usize as u64, usize; usize_trnc_up);
trnc_up!(isize as i64, isize; isize_trnc_up);
#[cfg(target_pointer_width = "64")]
trnc_up!(u64 as usize; u64_trnc_up_64);
#[cfg(target_pointer_width = "64")]
trnc_up!(i64 as isize; i64_trnc_up_64);
#[cfg(target_pointer_width = "32")]
trnc_up!(usize as u32; usize_trnc_up_32);
#[cfg(target_pointer_width = "32")]
trnc_up!(isize as i32; isize_trnc_up_32);

// truncations of signed tyes to unsigned types of equal or larger width
macro_rules! trnc_s2u_up {
    ($src:ident as $($dst:ty),+; $name:ident) => {
        #[test] fn $name() {
            $(
            for i in $src::min()..$src::min()+127 {
                test!(SaturatingCast::<$dst>::saturating_cast(i) == 0);
            }
            for i in $src::max()-127..$src::max() {
                test!(SaturatingCast::<$dst>::saturating_cast(i) == i as $dst);
            }
            )+
        }
    }
}

trnc_s2u_up!(i8 as u8, u16, u32, u64, usize; i8_trnc_s2u_up);
trnc_s2u_up!(i16 as u16, u32, u64, usize; i16_trnc_s2u_up);
trnc_s2u_up!(i32 as u32, u64, usize; i32_trnc_s2u_up);
trnc_s2u_up!(i64 as u64; i64_trnc_s2u_up);
trnc_s2u_up!(isize as usize; isize_trnc_s2u_up);
#[cfg(target_pointer_width = "64")]
trnc_s2u_up!(i64 as usize; i64_trnc_s2u_up_64);
#[cfg(target_pointer_width = "32")]
trnc_s2u_up!(isize as u32; isize_trnc_s2u_up_32);

// truncation of signed types to types of smaller width
macro_rules! trnc_s_down {
    ($src:ident as $($dst:ident),+; $name:ident) => {
        #[test] fn $name() {
            $(
            for i in $src::min()..$src::min()+127 {
                test!(SaturatingCast::<$dst>::saturating_cast(i) == $dst::min());
            }
            for i in $src::max()-127..$src::max() {
                test!(SaturatingCast::<$dst>::saturating_cast(i) == $dst::max());
            }
            )+
        }
    }
}

trnc_s_down!(i16 as u8, i8; i16_trnc_s_down);
trnc_s_down!(i32 as u8, i8, u16, i16; i32_trnc_s_down);
trnc_s_down!(i64 as u8, i8, u16, i16, u32, i32; i64_trnc_s_down);
trnc_s_down!(isize as u8, i8, u16, i16; isize_trnc_s_down);
#[cfg(target_pointer_width = "64")]
trnc_s_down!(isize as u32, i32; isize_trnc_s_down_64);
#[cfg(target_pointer_width = "32")]
trnc_s_down!(i64 as usize, isize; isize_trnc_s_down_32);

// truncation of unsigned types to types of equal or smaller width
macro_rules! trnc_u_down {
    ($src:ident as $($dst:ident),+; $name:ident) => {
        #[test] fn $name() {
            $(
            for i in 0..127 {
                test!(SaturatingCast::<$dst>::saturating_cast(i) == i as $dst);
            }
            for i in $src::max()-127..$src::max() {
                test!(SaturatingCast::<$dst>::saturating_cast(i) == $dst::max());
            }
            )+
        }
    }
}

trnc_u_down!(u16 as i8, u8, i16; u16_trnc_u_down);
trnc_u_down!(u32 as i8, u8, i16, u16, i32; u32_trnc_u_down);
trnc_u_down!(u64 as i8, u8, i16, u16, i32, u32, i64; u64_trnc_u_down);
trnc_u_down!(usize as i8, u8, i16, u16, i32; usize_trnc_u_down);
#[cfg(target_pointer_width = "64")]
trnc_u_down!(usize as u32, i64; usize_trnc_u_down_64);
#[cfg(target_pointer_width = "32")]
trnc_u_down!(u64 as isize, usize; u64_trnc_u_down_32);
