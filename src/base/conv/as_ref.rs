// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{slice, mem};
use {error};
use result::{Result};
use result::Result::{Ok, Err};
use wrapping::{W8, W16, W32, W64, Wsize};

/// Objects that can be immutably borrowed.
pub trait AsRef<Target: ?Sized>: TryAsRef<Target> {
    /// Borrows the object.
    fn as_ref(&self) -> &Target;
}

/// Objects that can be mutably borrowed.
pub trait AsMut<Target: ?Sized>: TryAsMut<Target> {
    /// Borrows the object.
    fn as_mut(&mut self) -> &mut Target;
}

pub trait TryAsRef<Target: ?Sized> {
    fn try_as_ref(&self) -> Result<&Target>;
}

pub trait TryAsMut<Target: ?Sized> {
    fn try_as_mut(&mut self) -> Result<&mut Target>;
}

// General conversions:
//
// &T -> &T
// &T -> &[T] where T: Sized
// &mut T -> &mut T
// &mut T -> &mut [T] where T: Sized
// &T -> &[d8] where T: Pod
// &mut T -> &mut [d8] where T: Pod
//
// &[&[u8]] -> &[&[d8]]
// &[&mut [u8]] -> &[&[d8]]
// &mut [&mut [u8]] -> &mut [&mut [d8]]

impl<T: ?Sized> AsRef<T> for T {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T: ?Sized> AsMut<T> for T {
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<T: ?Sized> TryAsRef<T> for T {
    fn try_as_ref(&self) -> Result<&T> {
        Ok(self)
    }
}

impl<T: ?Sized> TryAsMut<T> for T {
    fn try_as_mut(&mut self) -> Result<&mut T> {
        Ok(self)
    }
}

impl<T> AsRef<[T]> for T {
    fn as_ref(&self) -> &[T] {
        mem::as_slice(self)
    }
}

impl<T> AsMut<[T]> for T {
    fn as_mut(&mut self) -> &mut [T] {
        mem::as_mut_slice(self)
    }
}

impl<T> TryAsRef<[T]> for T {
    fn try_as_ref(&self) -> Result<&[T]> {
        Ok(self.as_ref())
    }
}

impl<T> TryAsMut<[T]> for T {
    fn try_as_mut(&mut self) -> Result<&mut [T]> {
        Ok(self.as_mut())
    }
}

impl<T: Pod+?Sized> AsRef<[d8]> for T {
    fn as_ref(&self) -> &[d8] {
        unsafe { slice::from_ptr(self as *const _ as *const _, mem::size_of_val(self)) }
    }
}

impl<T: Pod+?Sized> TryAsRef<[d8]> for T {
    fn try_as_ref(&self) -> Result<&[d8]> {
        Ok(self.as_ref())
    }
}

impl<T: Pod+?Sized> AsMut<[d8]> for T {
    fn as_mut(&mut self) -> &mut [d8] {
        unsafe { slice::from_ptr(self as *const _ as *const _, mem::size_of_val(self)) }
    }
}

impl<T: Pod+?Sized> TryAsMut<[d8]> for T {
    fn try_as_mut(&mut self) -> Result<&mut [d8]> {
        Ok(self.as_mut())
    }
}

impl<'a> AsRef<[&'a [d8]]> for [&'a [u8]] {
    fn as_ref(&self) -> &[&'a [d8]] {
        unsafe { mem::cast(self) }
    }
}

impl<'a> TryAsRef<[&'a [d8]]> for [&'a [u8]] {
    fn try_as_ref(&self) -> Result<&[&'a [d8]]> {
        Ok(self.as_ref())
    }
}

impl<'a> AsRef<[&'a [d8]]> for [&'a mut [u8]] {
    fn as_ref(&self) -> &[&'a [d8]] {
        unsafe { mem::cast(self) }
    }
}

impl<'a> TryAsRef<[&'a [d8]]> for [&'a mut [u8]] {
    fn try_as_ref(&self) -> Result<&[&'a [d8]]> {
        Ok(self.as_ref())
    }
}

impl<'a> AsMut<[&'a mut [d8]]> for [&'a mut [u8]] {
    fn as_mut(&mut self) -> &mut [&'a mut [d8]] {
        unsafe { mem::cast(self) }
    }
}

impl<'a> TryAsMut<[&'a mut [d8]]> for [&'a mut [u8]] {
    fn try_as_mut(&mut self) -> Result<&mut [&'a mut [d8]]> {
        Ok(self.as_mut())
    }
}

impl AsRef<[u8]> for str {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
impl_try_as_ref!([u8], str);

impl TryAsRef<str> for [u8] {
    fn try_as_ref(&self) -> Result<&str> {
        match str::from_bytes(self) {
            Some(s) => Ok(s),
            _ => Err(error::InvalidArgument),
        }
    }
}

impl AsRef<[u8]> for char {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            slice::from_ptr(self as *const _ as *const _, mem::size_of::<char>())
        }
    }
}
impl_try_as_ref!([u8], char);

impl AsRef<[u8]> for [char] {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            let ptr = self.as_ptr();
            let size = mem::size_of::<char>() * self.len();
            slice::from_ptr(ptr as *const _, size)
        }
    }
}
impl_try_as_ref!([u8], [char]);

macro_rules! as_mut_bytes {
    ($ty:ty) => {
        impl AsRef<[u8]> for $ty {
            fn as_ref(&self) -> &[u8] {
                unsafe {
                    slice::from_ptr(self as *const _ as *const _, mem::size_of::<$ty>())
                }
            }
        }
        impl_try_as_ref!([u8], $ty);

        impl AsMut<[u8]> for $ty {
            fn as_mut(&mut self) -> &mut [u8] {
                unsafe {
                    slice::from_ptr(self as *const _ as *const _, mem::size_of::<$ty>())
                }
            }
        }
        impl_try_as_mut!([u8], $ty);

        impl AsRef<[u8]> for [$ty] {
            fn as_ref(&self) -> &[u8] {
                unsafe {
                    let ptr = self.as_ptr();
                    let size = mem::size_of::<$ty>() * self.len();
                    slice::from_ptr(ptr as *const _, size)
                }
            }
        }
        impl_try_as_ref!([u8], [$ty]);

        impl AsMut<[u8]> for [$ty] {
            fn as_mut(&mut self) -> &mut [u8] {
                unsafe {
                    let ptr = self.as_ptr();
                    let size = mem::size_of::<$ty>() * self.len();
                    slice::from_ptr(ptr as *const _, size)
                }
            }
        }
        impl_try_as_mut!([u8], [$ty]);
    }
}

as_mut_bytes!(i8);
as_mut_bytes!(i16);
as_mut_bytes!(i32);
as_mut_bytes!(i64);
as_mut_bytes!(isize);
as_mut_bytes!(u16);
as_mut_bytes!(u32);
as_mut_bytes!(u64);
as_mut_bytes!(usize);
as_mut_bytes!(W8);
as_mut_bytes!(W16);
as_mut_bytes!(W32);
as_mut_bytes!(W64);
as_mut_bytes!(Wsize);

macro_rules! ptr_as_mut_bytes {
    ($ty:ty) => {
        impl<T> AsRef<[u8]> for $ty {
            fn as_ref(&self) -> &[u8] {
                unsafe {
                    slice::from_ptr(self as *const _ as *const _, mem::size_of::<$ty>())
                }
            }
        }

        impl<T> TryAsRef<[u8]> for $ty {
            fn try_as_ref(&self) -> Result<&[u8]> {
                Ok(self.as_ref())
            }
        }

        impl<T> AsMut<[u8]> for $ty {
            fn as_mut(&mut self) -> &mut [u8] {
                unsafe {
                    slice::from_ptr(self as *const _ as *const _, mem::size_of::<$ty>())
                }
            }
        }

        impl<T> TryAsMut<[u8]> for $ty {
            fn try_as_mut(&mut self) -> Result<&mut [u8]> {
                Ok(self.as_mut())
            }
        }

        impl<T> AsRef<[u8]> for [$ty] {
            fn as_ref(&self) -> &[u8] {
                unsafe {
                    let ptr = self.as_ptr();
                    let size = mem::size_of::<$ty>() * self.len();
                    slice::from_ptr(ptr as *const _, size)
                }
            }
        }

        impl<T> TryAsRef<[u8]> for [$ty] {
            fn try_as_ref(&self) -> Result<&[u8]> {
                Ok(self.as_ref())
            }
        }

        impl<T> AsMut<[u8]> for [$ty] {
            fn as_mut(&mut self) -> &mut [u8] {
                unsafe {
                    let ptr = self.as_ptr();
                    let size = mem::size_of::<$ty>() * self.len();
                    slice::from_ptr(ptr as *const _, size)
                }
            }
        }

        impl<T> TryAsMut<[u8]> for [$ty] {
            fn try_as_mut(&mut self) -> Result<&mut [u8]> {
                Ok(self.as_mut())
            }
        }
    }
}

ptr_as_mut_bytes!(*const T);
ptr_as_mut_bytes!(*mut T);
