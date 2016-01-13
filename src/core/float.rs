// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ops::{
    Eq, Add, Sub, Mul, Div, Rem, Range, AddAssign, SubAssign, MulAssign, DivAssign,
    RemAssign, Ordering, PartialOrd,
};
use marker::{Pod};
use cmp::{Ord};
use iter::{Iterator};
use option::{Option};
use option::Option::{Some, None};
use intrinsics::{self};

macro_rules! float_impls {
    (
        $t:ident;
        $abs:ident;
        $copysign:ident;
        $sqrt:ident;
        $pow:ident;
        $powi:ident;
        $sin:ident;
        $cos:ident;
        $exp:ident;
        $exp2:ident;
        $ln:ident;
        $log10:ident;
        $log2:ident;
        $floor:ident;
        $ceil:ident;
        $as_str:expr
     ) => {
        #[lang = $as_str]
        impl $t {
            pub const fn nan() -> Self {
                0.0 / 0.0
            }

            pub const fn inf() -> Self {
                1.0 / 0.0
            }

            pub fn is_nan(self) -> bool {
                self != self
            }

            pub fn is_inf(self) -> bool {
                self == Self::inf()
            }

            pub fn finite(self) -> bool {
                !self.is_nan() && !self.is_inf()
            }

            pub fn negative(self) -> bool {
                self < 0.0
            }

            pub fn abs(self) -> Self {
                unsafe { intrinsics::$abs(self) }
            }

            pub fn signum(self) -> Self {
                if self.is_nan() {
                    Self::nan()
                } else {
                    unsafe { intrinsics::$copysign(1.0, self) }
                }
            }

            pub fn sqrt(self) -> Self {
                unsafe { intrinsics::$sqrt(self) }
            }

            pub fn powi(self, other: i32) -> Self {
                unsafe { intrinsics::$powi(self, other) }
            }

            pub fn sin(self) -> Self {
                unsafe { intrinsics::$sin(self) }
            }

            pub fn cos(self) -> Self {
                unsafe { intrinsics::$cos(self) }
            }

            pub fn pow(self, other: Self) -> Self {
                unsafe { intrinsics::$pow(self, other) }
            }

            pub fn exp(self) -> Self {
                unsafe { intrinsics::$exp(self) }
            }

            pub fn exp2(self) -> Self {
                unsafe { intrinsics::$exp2(self) }
            }

            pub fn ln(self) -> Self {
                unsafe { intrinsics::$ln(self) }
            }

            pub fn log10(self) -> Self {
                unsafe { intrinsics::$log10(self) }
            }

            pub fn log2(self) -> Self {
                unsafe { intrinsics::$log2(self) }
            }

            pub fn floor(self) -> Self {
                unsafe { intrinsics::$floor(self) }
            }

            pub fn ceil(self) -> Self {
                unsafe { intrinsics::$ceil(self) }
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

        impl AddAssign for $t {
            fn add_assign(&mut self, other: $t) { *self += other }
        }

        impl Sub for $t {
            type Output = $t;
            fn sub(self, other: $t) -> $t { self - other }
        }

        impl SubAssign for $t {
            fn sub_assign(&mut self, other: $t) { *self -= other }
        }

        impl Mul for $t {
            type Output = $t;
            fn mul(self, other: $t) -> $t { self * other }
        }

        impl MulAssign for $t {
            fn mul_assign(&mut self, other: $t) { *self *= other }
        }

        impl Div for $t {
            type Output = $t;
            fn div(self, other: $t) -> $t { self / other }
        }

        impl DivAssign for $t {
            fn div_assign(&mut self, other: $t) { *self /= other }
        }

        impl Rem for $t {
            type Output = $t;
            fn rem(self, other: $t) -> $t { self % other }
        }

        impl RemAssign for $t {
            fn rem_assign(&mut self, other: $t) { *self %= other }
        }

        impl Iterator for Range<$t> {
            type Item = $t;
            fn next(&mut self) -> Option<$t> {
                if self.start < self.end {
                    let ret = self.start;
                    self.start += 1.0;
                    Some(ret)
                } else {
                    None
                }
            }
        }

        unsafe impl Pod for $t { }
    }
}

float_impls!(f32; fabsf32; copysignf32; sqrtf32; powf32; powif32; sinf32; cosf32; expf32; exp2f32; logf32; log10f32; log2f32; floorf32; ceilf32; "f32");
float_impls!(f64; fabsf64; copysignf64; sqrtf64; powf64; powif64; sinf64; cosf64; expf64; exp2f64; logf64; log10f64; log2f64; floorf64; ceilf64; "f64");
