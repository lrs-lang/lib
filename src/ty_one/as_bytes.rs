// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::prelude::*;
use core::{slice, mem};

pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

pub trait AsMutBytes : AsBytes {
    fn as_mut_bytes(&mut self) -> &mut [u8];
}

macro_rules! impl_as_bytes_for_int {
    ($t:ty) => {
        impl AsBytes for $t {
            fn as_bytes(&self) -> &[u8] {
                unsafe {
                    slice::from_ptr(self as *const _ as *const _, mem::size_of::<$t>())
                }
            }
        }
        impl AsMutBytes for $t {
            fn as_mut_bytes(&mut self) -> &mut [u8] {
                unsafe {
                    slice::from_ptr(self as *mut _ as *mut _, mem::size_of::<$t>())
                }
            }
        }

        impl AsBytes for [$t] {
            fn as_bytes(&self) -> &[u8] {
                unsafe {
                    let ptr = self.as_ptr();
                    let size = mem::size_of::<$t>() * self.len();
                    slice::from_ptr(ptr as *const _, size)
                }
            }
        }
        impl AsMutBytes for [$t] {
            fn as_mut_bytes(&mut self) -> &mut [u8] {
                unsafe {
                    let ptr = self.as_ptr();
                    let size = mem::size_of::<$t>() * self.len();
                    slice::from_ptr(ptr as *const _, size)
                }
            }
        }
    }
}

impl_as_bytes_for_int!(i8);
impl_as_bytes_for_int!(u8);
impl_as_bytes_for_int!(i16);
impl_as_bytes_for_int!(u16);
impl_as_bytes_for_int!(i32);
impl_as_bytes_for_int!(u32);
impl_as_bytes_for_int!(i64);
impl_as_bytes_for_int!(u64);
impl_as_bytes_for_int!(isize);
impl_as_bytes_for_int!(usize);

impl<'a, T: AsBytes+?Sized> AsBytes for &'a T {
    fn as_bytes(&self) -> &[u8] { (**self).as_bytes() }
}

impl<'a, T: AsBytes+?Sized> AsBytes for &'a mut T {
    fn as_bytes(&self) -> &[u8] { (**self).as_bytes() }
}

impl<'a, T: AsMutBytes+?Sized> AsMutBytes for &'a mut T {
    fn as_mut_bytes(&mut self) -> &mut [u8] { (**self).as_mut_bytes() }
}
