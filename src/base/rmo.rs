// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
use core::{mem, slice};

/// Objects that can be immutably borrowed.
pub trait AsRef<Target: ?Sized> {
    /// Borrows the object.
    fn as_ref(&self) -> &Target;
}

/// Objects that can be mutably borrowed.
pub trait AsMut<Target: ?Sized> {
    /// Borrows the object.
    fn as_mut(&mut self) -> &mut Target;
}

impl<'a, Target: ?Sized, T: AsRef<Target>+?Sized> AsRef<Target> for &'a T {
    fn as_ref(&self) -> &Target { (**self).as_ref() }
}

impl<'a, Target: ?Sized, T: AsRef<Target>+?Sized> AsRef<Target> for &'a mut T {
    fn as_ref(&self) -> &Target { (**self).as_ref() }
}

impl<'a, Target: ?Sized, T: AsMut<Target>+?Sized> AsMut<Target> for &'a mut T {
    fn as_mut(&mut self) -> &mut Target { (**self).as_mut() }
}

macro_rules! impl_for_int {
    ($t:ty) => {
        impl AsRef<[u8]> for $t {
            fn as_ref(&self) -> &[u8] {
                unsafe {
                    slice::from_ptr(self as *const _ as *const _, mem::size_of::<$t>())
                }
            }
        }
        impl AsMut<[u8]> for $t {
            fn as_mut(&mut self) -> &mut [u8] {
                unsafe {
                    slice::from_ptr(self as *mut _ as *mut _, mem::size_of::<$t>())
                }
            }
        }

        impl AsRef<[u8]> for [$t] {
            fn as_ref(&self) -> &[u8] {
                unsafe {
                    let ptr = self.as_ptr();
                    let size = mem::size_of::<$t>() * self.len();
                    slice::from_ptr(ptr as *const _, size)
                }
            }
        }
        impl AsMut<[u8]> for [$t] {
            fn as_mut(&mut self) -> &mut [u8] {
                unsafe {
                    let ptr = self.as_ptr();
                    let size = mem::size_of::<$t>() * self.len();
                    slice::from_ptr(ptr as *const _, size)
                }
            }
        }
    }
}

impl_for_int!(i8);
impl_for_int!(u8);
impl_for_int!(i16);
impl_for_int!(u16);
impl_for_int!(i32);
impl_for_int!(u32);
impl_for_int!(i64);
impl_for_int!(u64);
impl_for_int!(isize);
impl_for_int!(usize);

impl AsRef<[u8]> for str { fn as_ref(&self) -> &[u8] { self.as_bytes() } }
