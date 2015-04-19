// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use ty_one::prelude::*;
use core::{mem};
use arch::{memchr, all_bytes};
use ty_one::c_str::{CStr};
use ty_one::{error};
use io::{Write};
use fmt::{Debug};
use rmo::{Rmo, AsRef, AsMut, ToOwned};
use vec::{Vec};

pub struct CString {
    data: Vec<u8>,
}

impl Deref for CString {
    type Target = CStr;
    fn deref(&self) -> &CStr {
        unsafe { mem::cast(self.data.deref()) }
    }
}

impl Debug for CString {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(self.deref(), w)
    }
}

impl AsRef<CStr> for CString {
    fn as_ref(&self) -> &CStr {
        unsafe { CStr::from_bytes_unchecked(&self.data[..]) }
    }
}

impl AsMut<CStr> for CString {
    fn as_mut(&mut self) -> &mut CStr {
        unsafe { CStr::from_bytes_unchecked_mut(&mut self.data[..]) }
    }
}

impl ToOwned for CStr {
    type Owned = CString;
    fn to_owned(&self) -> Result<CString> {
        Ok(CString { data: try!(self.as_ref().to_owned()) })
    }
}

///////////////////

pub trait ToCString {
    fn to_cstring(&self) -> Result<CString>;
    fn rmo_cstr<'a>(&'a self, _buf: &'a mut [u8]) -> Result<Rmo<'a, CStr>> {
        self.to_cstring().map(|r| Rmo::Owned(r))
    }
}

impl<'b, T: ToCString+?Sized> ToCString for &'b T {
    fn to_cstring(&self) -> Result<CString> { (**self).to_cstring() }
    fn rmo_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr>> {
        (**self).rmo_cstr(buf)
    }
}

impl ToCString for [u8] {
    fn to_cstring(&self) -> Result<CString> {
        if let Some(idx) = memchr(self, 0) {
            if idx == self.len() - 1 || all_bytes(&self[idx+1..], 0) {
                self[..idx+1].to_owned().map(|o| CString { data: o })
            } else {
                Err(error::InvalidArgument)
            }
        } else {
            let mut vec = try!(self.to_owned());
            try!(vec.reserve(1));
            vec.push(0);
            Ok(CString { data: vec })
        }
    }

    fn rmo_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr>> {
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
                Ok(Rmo::Owned(CString { data: vec }))
            } else {
                mem::copy(buf, self);
                buf[self.len()] = 0;
                Ok(unsafe { Rmo::Ref(CStr::from_bytes_unchecked(&buf[..self.len()+1])) })
            }
        }
    }
}

impl ToCString for [i8] {
    fn to_cstring(&self) -> Result<CString> { self.as_ref().to_cstring() }
    fn rmo_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr>> {
        self.as_ref().rmo_cstr(buf)
    }
}

impl ToCString for str {
    fn to_cstring(&self) -> Result<CString> { self.as_ref().to_cstring() }
    fn rmo_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr>> {
        self.as_ref().rmo_cstr(buf)
    }
}
