// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use prelude::*;
use core::{slice, mem};
use {error};

pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

pub trait AsMutBytes : AsBytes {
    fn as_mut_bytes(&mut self) -> &mut [u8];
}

pub trait ToBytes {
    fn to_bytes<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut [u8]>;

    fn to_or_as_bytes<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a [u8]> {
        self.to_bytes(buf).map(|r| &*r)
    }

    fn to_or_as_mut_bytes<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        self.to_bytes(buf)
    }
}

impl<'a, T: AsBytes+?Sized> AsBytes for &'a T {
    fn as_bytes(&self) -> &[u8] { (**self).as_bytes() }
}

impl<'a, T: AsBytes+?Sized> AsBytes for &'a mut T {
    fn as_bytes(&self) -> &[u8] { (**self).as_bytes() }
}

impl<'a, T: AsMutBytes+?Sized> AsMutBytes for &'a mut T {
    fn as_mut_bytes(&mut self) -> &mut [u8] { (**self).as_mut_bytes() }
}

impl<'b, T: ToBytes+?Sized> ToBytes for &'b mut T {
    fn to_bytes<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        (**self).to_bytes(buf)
    }

    fn to_or_as_bytes<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a [u8]> {
        (**self).to_or_as_bytes(buf)
    }

    fn to_or_as_mut_bytes<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        (**self).to_or_as_mut_bytes(buf)
    }
}

impl<'b, T: ToBytes+?Sized> ToBytes for &'b T {
    fn to_bytes<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        (**self).to_bytes(buf)
    }

    fn to_or_as_bytes<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a [u8]> {
        (**self).to_or_as_bytes(buf)
    }

    fn to_or_as_mut_bytes<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        (**self).to_bytes(buf)
    }
}

macro_rules! impl_bytes_for_int {
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
        impl ToBytes for $t {
            fn to_bytes<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
                let bytes = self.as_bytes();
                if bytes.len() <= buf.len() {
                    mem::copy(buf, bytes);
                    Ok(&mut buf[..bytes.len()])
                } else {
                    Err(error::NoMemory)
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
        impl ToBytes for [$t] {
            fn to_bytes<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
                let bytes = self.as_bytes();
                if bytes.len() <= buf.len() {
                    mem::copy(buf, bytes);
                    Ok(&mut buf[..bytes.len()])
                } else {
                    Err(error::NoMemory)
                }
            }
        }
    }
}

impl_bytes_for_int!(i8);
impl_bytes_for_int!(u8);
impl_bytes_for_int!(i16);
impl_bytes_for_int!(u16);
impl_bytes_for_int!(i32);
impl_bytes_for_int!(u32);
impl_bytes_for_int!(i64);
impl_bytes_for_int!(u64);
impl_bytes_for_int!(isize);
impl_bytes_for_int!(usize);

impl AsBytes for str { fn as_bytes(&self) -> &[u8] { self.as_bytes() } }

impl ToBytes for str {
    fn to_bytes<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        self.as_bytes().to_bytes(buf)
    }
}
