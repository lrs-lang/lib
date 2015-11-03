// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use arch_fns::{memchr, all_bytes};
use core::{mem};
use base::{error};
use base::default::{Default};
use rmo::{Rmo, ToOwned};
use str_one::{CStr, NoNullStr, ByteStr, AsByteStr};
use str_two::{CString, NoNullString, ByteString};
use alloc::{Allocator};

/// Objects that can be turned into a `CString` by allocating.
pub trait ToCString {
    /// Converts the object into an allocated `CString`.
    fn to_cstring<H>(&self) -> Result<CString<H>>
        where H: Allocator,
              H::Pool: Default;

    /// Tries to convert the object into an `Rmo<CStr>` with or without allocating.
    ///
    /// [argument, _buf]
    /// Scratch space.
    ///
    /// = Remarks
    ///
    /// The general strategy is as follows: If the object can be interpreted as a `CStr`,
    /// a borrowed `Rmo` is returned. If it can be interpreted as a `CStr` by copying it
    /// into the provided buffer, it's copied and returned as a borrowed `Rmo`. Otherwise,
    /// a `CString` is allocated.
    ///
    /// The default implementation simply calls `to_cstring`.
    fn rmo_cstr<'a, H>(&'a self, _buf: &'a mut [u8]) -> Result<Rmo<'a, CStr, H>>
        where H: Allocator,
              H::Pool: Default,
    {
        self.to_cstring().map(|r| Rmo::Owned(r))
    }
}

impl<'b, T: ToCString+?Sized> ToCString for &'b T {
    fn to_cstring<H>(&self) -> Result<CString<H>>
        where H: Allocator,
              H::Pool: Default,
    {
        (**self).to_cstring()
    }

    fn rmo_cstr<'a, H>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr, H>>
        where H: Allocator,
              H::Pool: Default,
    {
        (**self).rmo_cstr(buf)
    }
}

impl<'b, T: ToCString+?Sized> ToCString for &'b mut T {
    fn to_cstring<H>(&self) -> Result<CString<H>>
        where H: Allocator,
              H::Pool: Default,
    {
        (**self).to_cstring()
    }

    fn rmo_cstr<'a, H>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr, H>>
        where H: Allocator,
              H::Pool: Default,
    {
        (**self).rmo_cstr(buf)
    }
}

impl ToCString for CStr {
    fn to_cstring<H>(&self) -> Result<CString<H>>
        where H: Allocator,
              H::Pool: Default,
    {
        let bytes: &[u8] = self.bytes_with_null();
        bytes.to_owned().map(|o| unsafe { CString::from_bytes_unchecked(o) })
    }

    fn rmo_cstr<'a, H>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr, H>>
        where H: Allocator,
              H::Pool: Default,
    {
        let bytes: &[u8] = self.bytes_with_null();
        if bytes.len() <= buf.len() {
            mem::copy(buf, bytes);
            Ok(Rmo::Ref(unsafe { CStr::from_bytes_unchecked(&buf[..bytes.len()]) }))
        } else {
            self.to_cstring().map(|o| Rmo::Owned(o))
        }
    }
}

impl ToCString for [u8] {
    fn to_cstring<H>(&self) -> Result<CString<H>>
        where H: Allocator,
              H::Pool: Default,
    {
        if let Some(idx) = memchr(self, 0) {
            if idx == self.len() - 1 || all_bytes(&self[idx+1..], 0) {
                self[..idx+1].to_owned().map(|o| unsafe { CString::from_bytes_unchecked(o) })
            } else {
                Err(error::InvalidArgument)
            }
        } else {
            let mut vec = try!(self.to_owned());
            try!(vec.reserve(1));
            vec.push(0);
            Ok(unsafe { CString::from_bytes_unchecked(vec) })
        }
    }

    fn rmo_cstr<'a, H>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr, H>>
        where H: Allocator,
              H::Pool: Default,
    {
        if let Some(idx) = memchr(self, 0) {
            if idx == self.len() - 1 || all_bytes(&self[idx+1..], 0) {
                Ok(unsafe { Rmo::Ref(CStr::from_bytes_unchecked(&self[..idx+1])) })
            } else {
                Err(error::InvalidArgument)
            }
        } else {
            if self.len() >= buf.len() {
                let mut vec = try!(self.to_owned());
                try!(vec.reserve(1));
                vec.push(0);
                Ok(Rmo::Owned(unsafe { CString::from_bytes_unchecked(vec) }))
            } else {
                mem::copy(buf, self);
                buf[self.len()] = 0;
                Ok(unsafe { Rmo::Ref(CStr::from_bytes_unchecked(&buf[..self.len()+1])) })
            }
        }
    }
}

impl ToCString for [i8] {
    fn to_cstring<H>(&self) -> Result<CString<H>>
        where H: Allocator,
              H::Pool: Default,
    {
        let bytes: &[u8] = self.as_ref();
        bytes.to_cstring()
    }

    fn rmo_cstr<'a, H>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr, H>>
        where H: Allocator,
              H::Pool: Default,
    {
        let bytes: &[u8] = self.as_ref();
        bytes.rmo_cstr(buf)
    }
}

impl ToCString for str {
    fn to_cstring<H>(&self) -> Result<CString<H>>
        where H: Allocator,
              H::Pool: Default,
    {
        let bytes: &[u8] = self.as_ref();
        bytes.to_cstring()
    }

    fn rmo_cstr<'a, H>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr, H>>
        where H: Allocator,
              H::Pool: Default,
    {
        let bytes: &[u8] = self.as_ref();
        bytes.rmo_cstr(buf)
    }
}

impl ToCString for NoNullStr {
    fn to_cstring<H>(&self) -> Result<CString<H>>
        where H: Allocator,
              H::Pool: Default,
    {
        let bytes: &[u8] = self.as_ref();
        match bytes.to_owned() {
            Ok(mut o) => {
                try!(o.reserve(1));
                o.push(0);
                Ok(unsafe { CString::from_bytes_unchecked(o) })
            },
            Err(e) => Err(e),
        }
    }

    fn rmo_cstr<'a, H>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr, H>>
        where H: Allocator,
              H::Pool: Default,
    {
        let bytes: &[u8] = self.as_ref();
        if bytes.len() < buf.len() {
            mem::copy(buf, bytes);
            buf[bytes.len()] = 0;
            Ok(Rmo::Ref(unsafe { CStr::from_bytes_unchecked(&buf[..bytes.len()]) }))
        } else {
            self.to_cstring().map(|o| Rmo::Owned(o))
        }
    }
}

impl<A> ToCString for NoNullString<A>
    where A: Allocator,
{
    fn to_cstring<H>(&self) -> Result<CString<H>>
        where H: Allocator,
              H::Pool: Default,
    {
        self.deref().to_cstring()
    }

    fn rmo_cstr<'a, H>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr, H>>
        where H: Allocator,
              H::Pool: Default,
    {
        self.deref().rmo_cstr(buf)
    }
}

impl ToCString for ByteStr {
    fn to_cstring<H>(&self) -> Result<CString<H>>
        where H: Allocator,
              H::Pool: Default,
    {
        self.as_ref().to_cstring()
    }

    fn rmo_cstr<'a, H>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr, H>>
        where H: Allocator,
              H::Pool: Default,
    {
        self.as_ref().rmo_cstr(buf)
    }
}

impl<A> ToCString for ByteString<A>
    where A: Allocator,
{
    fn to_cstring<H>(&self) -> Result<CString<H>>
        where H: Allocator,
              H::Pool: Default,
    {
        self.as_byte_str().to_cstring()
    }

    fn rmo_cstr<'a, H>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr, H>>
        where H: Allocator,
              H::Pool: Default,
    {
        self.as_byte_str().rmo_cstr(buf)
    }
}
