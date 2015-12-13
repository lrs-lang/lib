// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {CStr, NoNullStr, ByteStr};
use core::{mem, slice};
use base::prelude::*;
use base::{error};
use arch_fns::{memchr};

// TODO: Remove this

/// Objects that can be transformed into `CStr`s provided they have some scratch space
/// available.
///
/// = Remarks
///
/// For example, "Hello World" needs to be copied and a `0` appended to form a valid
/// `CStr`. This operation can fail under the same conditions as `AsCStr`.
pub trait ToCStr {
    /// Converts the object by copying it.
    ///
    /// [argument, buf]
    /// The buffer in which the `CStr` will be stored.
    ///
    /// [return_value]
    /// Returns the created `CStr`.
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr>;

    /// Tries to create a `CStr` without copying and copies if that's not possible.
    ///
    /// [argument, buf]
    /// The buffer in which the `CStr` will be created if it has to be copied.
    ///
    /// [return_value]
    /// Returns the borrowed or created `CStr`.
    ///
    /// = Remarks
    ///
    /// For example, `"Hello World\0"` does not have to be copied. The default
    /// implementation simply calls `to_cstr`.
    fn to_or_as_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a CStr> {
        self.to_cstr(buf).map(|r| &*r)
    }

    /// Tries to create a mutable `CStr` without copying and copies if that's not
    /// possible.
    ///
    /// [argument, buf]
    /// The buffer in which the `CStr` will be created if it has to be copied.
    ///
    /// [return_value]
    /// Returns the borrowed or created `CStr`.
    ///
    /// = Remarks
    ///
    /// The default implementation simply calls `to_cstr`.
    fn to_or_as_mut_cstr<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        self.to_cstr(buf)
    }
}

impl ToCStr for CStr {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        let bytes = self.bytes_with_null();
        if bytes.len() <= buf.len() {
            mem::copy(buf, bytes);
            Ok(unsafe { mem::cast(&mut buf[..self.len()]) })
        } else {
            Err(error::NoMemory)
        }
    }

    fn to_or_as_cstr<'a>(&'a self, _: &'a mut [u8]) -> Result<&'a CStr> {
        Ok(self)
    }

    fn to_or_as_mut_cstr<'a>(&'a mut self, _: &'a mut [u8]) -> Result<&'a mut CStr> {
        Ok(self)
    }
}

fn bytes_to_or_as_cstr(b: &[u8], buf: &mut [u8]) -> Result<(*const u8, usize)> {
    if let Some(idx) = memchr(b, 0) {
        if idx == b.len() - 1 {
            Ok((b.as_ptr(), idx))
        } else {
            Err(error::InvalidArgument)
        }
    } else if b.len() >= buf.len() {
        Err(error::NoMemory)
    } else {
        mem::copy(buf, b);
        buf[b.len()] = 0;
        Ok((buf.as_ptr(), b.len()))
    }
}

impl ToCStr for [u8] {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        let len = match memchr(self, 0) {
            Some(idx) => {
                if idx == self.len() - 1 {
                    self.len() - 1
                } else {
                    return Err(error::InvalidArgument);
                }
            },
            _ => self.len(),
        };
        if len >= buf.len() {
            return Err(error::NoMemory);
        }
        mem::copy(buf, &self[..len]);
        buf[len] = 0;
        Ok(unsafe { mem::cast(&mut buf[..len]) })
    }

    fn to_or_as_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a CStr> {
        let (ptr, len) = try!(bytes_to_or_as_cstr(self, buf));
        Ok(unsafe { mem::cast(slice::from_ptr(ptr, len)) })
    }

    fn to_or_as_mut_cstr<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        let (ptr, len) = try!(bytes_to_or_as_cstr(self, buf));
        Ok(unsafe { mem::cast(slice::from_ptr(ptr, len)) })
    }
}

impl ToCStr for [i8] {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        let bytes: &[u8] = self.as_ref();
        bytes.to_cstr(buf)
    }

    fn to_or_as_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a CStr> {
        let bytes: &[u8] = self.as_ref();
        bytes.to_or_as_cstr(buf)
    }

    fn to_or_as_mut_cstr<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        let bytes: &mut [u8] = self.as_mut();
        bytes.to_or_as_mut_cstr(buf)
    }
}

impl ToCStr for str {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        self.as_bytes().to_cstr(buf)
    }

    fn to_or_as_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a CStr> {
        self.as_bytes().to_or_as_cstr(buf)
    }

    fn to_or_as_mut_cstr<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        self.as_bytes().to_cstr(buf)
    }
}

impl<'b, T: ToCStr+?Sized> ToCStr for &'b T {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        (**self).to_cstr(buf)
    }

    fn to_or_as_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a CStr> {
        (**self).to_or_as_cstr(buf)
    }

    fn to_or_as_mut_cstr<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        (**self).to_cstr(buf)
    }
}

impl ToCStr for NoNullStr {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        if self.len() < buf.len() {
            mem::copy(buf, self.as_ref());
            buf[self.len()] = 0;
            unsafe { Ok(mem::cast(&mut buf[..self.len()])) }
        } else {
            Err(error::NoMemory)
        }
    }
}

impl ToCStr for ByteStr {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        let bytes: &[u8] = self.as_ref();
        bytes.to_cstr(buf)
    }
}
