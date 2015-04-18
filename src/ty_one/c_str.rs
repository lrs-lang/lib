// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use prelude::*;
use core::ops::{Index};
use core::{mem};
use cty_base::types::{c_char};
use bytes::{AsBytes, AsMutBytes, ToBytes};
use byte_str::{ByteStr, AsByteStr};
use {error};
use arch_fns::{all_bytes, memchr};
use path::{AsPath, AsMutPath, Path};

pub struct CStr {
    data: [u8]
}

impl CStr {
    pub fn empty() -> &'static CStr {
        static EMPTY: [u8; 1] = [0];
        unsafe { mem::cast(&EMPTY[..]) }
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.data.as_ptr() as *const c_char
    }

    pub fn as_mut_ptr(&mut self) -> *mut c_char {
        self.data.as_mut_ptr() as *mut c_char
    }

    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &CStr {
        mem::cast(bytes)
    }

    pub unsafe fn from_bytes_unchecked_mut(bytes: &mut [u8]) -> &mut CStr {
        mem::cast(bytes)
    }

    pub fn len(&self) -> usize {
        self.as_bytes().len() - 1
    }
}

impl Deref for CStr {
    type Target = ByteStr;
    fn deref(&self) -> &ByteStr {
        self.as_byte_str()
    }
}

impl Index<usize> for CStr {
    type Output = u8;
    fn index(&self, idx: usize) -> &u8 {
        &self.data[idx]
    }
}

impl AsBytes for CStr {
    fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

impl ToBytes for CStr {
    fn to_bytes<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        let bytes = self.as_bytes();
        if bytes.len() <= buf.len() {
            mem::copy(buf, bytes);
            Ok(&mut buf[..bytes.len()])
        } else {
            Err(error::NoMemory)
        }
    }

    fn to_or_as_bytes<'a>(&'a self, _: &'a mut [u8]) -> Result<&'a [u8]> {
        Ok(self.as_bytes())
    }
}

impl AsByteStr for CStr {
    fn as_byte_str(&self) -> &ByteStr {
        self.data[..self.data.len() - 1].as_byte_str()
    }
}

impl AsPath for CStr {
    fn as_path(&self) -> Result<&Path> {
        unsafe { Ok(Path::from_bytes_unchecked(&self.data[..self.data.len() - 1])) }
    }
}

impl AsMutPath for CStr {
    fn as_mut_path(&mut self) -> Result<&mut Path> {
        let len = self.data.len() - 1;
        unsafe { Ok(Path::from_bytes_unchecked_mut(&mut self.data[..len])) }
    }
}

////////////////////////

pub trait AsCStr : AsBytes {
    fn as_cstr(&self) -> Result<&CStr> {
        self.as_bytes().as_cstr()
    }
}

pub trait AsMutCStr {
    fn as_mut_cstr(&mut self) -> Result<&mut CStr>;
}

pub trait ToCStr {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr>;

    fn to_or_as_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a CStr> {
        self.to_cstr(buf).map(|r| &*r)
    }

    fn to_or_as_mut_cstr<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        self.to_cstr(buf)
    }
}

impl AsCStr for [u8] {
    fn as_cstr(&self) -> Result<&CStr> {
        match memchr(self, 0) {
            Some(idx) => {
                if idx == self.len() - 1 || all_bytes(&self[idx+1..], 0) {
                    Ok(unsafe { CStr::from_bytes_unchecked(&self[..idx+1]) })
                } else {
                    Err(error::InvalidArgument)
                }
            },
            _ => Err(error::InvalidArgument),
        }
    }
}

impl AsCStr for CStr { fn as_cstr(&self) -> Result<&CStr> { Ok(self) } }
impl AsCStr for [i8] { fn as_cstr(&self) -> Result<&CStr> { self.as_bytes().as_cstr() } }
impl AsCStr for str { fn as_cstr(&self) -> Result<&CStr> { self.as_bytes().as_cstr() } }

impl AsMutCStr for [u8] {
    fn as_mut_cstr(&mut self) -> Result<&mut CStr> {
        match self.as_cstr() {
            Ok(c) => Ok(unsafe { mem::cast(c) }),
            Err(e) => Err(e),
        }
    }
}

impl AsMutCStr for CStr { fn as_mut_cstr(&mut self) -> Result<&mut CStr> { Ok(self) } }
impl AsMutCStr for [i8] { fn as_mut_cstr(&mut self) -> Result<&mut CStr> { self.as_mut_bytes().as_mut_cstr() } }

impl ToCStr for CStr {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        let bytes = self.as_bytes();
        if bytes.len() <= buf.len() {
            mem::copy(buf, bytes);
            Ok(unsafe { CStr::from_bytes_unchecked_mut(&mut buf[..bytes.len()]) })
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

impl ToCStr for [u8] {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        let len = match memchr(self, 0) {
            Some(idx) => {
                if idx == self.len() - 1 || all_bytes(&self[idx+1..], 0) {
                    idx
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
        Ok(unsafe { CStr::from_bytes_unchecked_mut(&mut buf[..len+1]) })
    }

    fn to_or_as_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a CStr> {
        if let Some(idx) = memchr(self, 0) {
            if idx == self.len() - 1 || all_bytes(&self[idx+1..], 0) {
                Ok(unsafe { CStr::from_bytes_unchecked(&self[..idx+1]) })
            } else {
                Err(error::InvalidArgument)
            }
        } else if self.len() >= buf.len() {
            Err(error::NoMemory)
        } else {
            mem::copy(buf, self);
            buf[self.len()] = 0;
            Ok(unsafe { CStr::from_bytes_unchecked(&buf[..self.len()+1]) })
        }
    }

    fn to_or_as_mut_cstr<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        match self.to_or_as_cstr(buf) {
            Ok(b) => Ok(unsafe { mem::cast(b) }),
            Err(e) => Err(e),
        }
    }
}

impl ToCStr for [i8] {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> { self.as_bytes().to_cstr(buf) }
    fn to_or_as_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a CStr> { self.as_bytes().to_or_as_cstr(buf) }
    fn to_or_as_mut_cstr<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut CStr> { self.as_mut_bytes().to_or_as_mut_cstr(buf) }
}

impl ToCStr for str {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> { self.as_bytes().to_cstr(buf) }
    fn to_or_as_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a CStr> { self.as_bytes().to_or_as_cstr(buf) }
    fn to_or_as_mut_cstr<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut CStr> { self.as_bytes().to_cstr(buf) }
}
