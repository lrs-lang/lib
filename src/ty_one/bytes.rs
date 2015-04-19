// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use prelude::*;
use rmo::{AsRef};
use core::{mem};
use {error};

// XXX: Not sure if this is needed.

pub trait ToBytes {
    fn to_bytes<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut [u8]>;

    fn to_or_as_bytes<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a [u8]> {
        self.to_bytes(buf).map(|r| &*r)
    }

    fn to_or_as_mut_bytes<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        self.to_bytes(buf)
    }
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
        impl ToBytes for $t {
            fn to_bytes<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
                let bytes = self.as_ref();
                if bytes.len() <= buf.len() {
                    mem::copy(buf, bytes);
                    Ok(&mut buf[..bytes.len()])
                } else {
                    Err(error::NoMemory)
                }
            }
        }

        impl ToBytes for [$t] {
            fn to_bytes<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
                let bytes = self.as_ref();
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
