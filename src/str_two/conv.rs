// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use str_one::{NoNullStr, CStr, ByteStr};
use {String, CString};
use vec::{Vec};
use bx::{Box};
use alloc::{MemPool};

// Conversions
//
// &String<H> -> &[u8]
// &String<H> -> &ByteStr
// &String<H> -> &str
// &String<H> -> Result<&NoNullStr>
// &String<H> -> Result<&CStr>
//
// &CString<H> -> &[u8]
// &CString<H> -> &ByteStr
// &CString<H> -> Result<&str>
// &CString<H> -> &NoNullStr
// &CString<H> -> &CStr
//
// &mut CString<H> -> &mut NoNullStr
// &mut CString<H> -> &mut CStr
//
// &T: TryAsRef<str> -> Result<String<H>>
// &T: TryAsRef<NoNullStr> -> Result<CString<H>>
//
// String<H> -> Vec<u8, H>
// String<H> -> Box<str, H>
// CString<H> -> Vec<u8, H>
// CString<H> -> Box<CStr, H>

impl<H> AsRef<[u8]> for String<H>
    where H: MemPool,
{
    fn as_ref(&self) -> &[u8] {
        self.deref().as_ref()
    }
}

impl<H> TryAsRef<[u8]> for String<H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&[u8]> {
        Ok(self.as_ref())
    }
}

impl<H> AsRef<ByteStr> for String<H>
    where H: MemPool,
{
    fn as_ref(&self) -> &ByteStr {
        self.deref().as_ref()
    }
}

impl<H> TryAsRef<ByteStr> for String<H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&ByteStr> {
        Ok(self.as_ref())
    }
}

impl<H> AsRef<str> for String<H>
    where H: MemPool,
{
    fn as_ref(&self) -> &str {
        self.deref()
    }
}

impl<H> TryAsRef<str> for String<H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&str> {
        Ok(self.as_ref())
    }
}

impl<H> TryAsRef<NoNullStr> for String<H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&NoNullStr> {
        let bytes: &[u8] = self.as_ref();
        bytes.try_as_ref()
    }
}

impl<H> TryAsRef<CStr> for String<H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&CStr> {
        let bytes: &[u8] = self.as_ref();
        bytes.try_as_ref()
    }
}

impl<H> AsRef<[u8]> for CString<H>
    where H: MemPool,
{
    fn as_ref(&self) -> &[u8] {
        self.deref().as_ref()
    }
}

impl<H> TryAsRef<[u8]> for CString<H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&[u8]> {
        Ok(self.as_ref())
    }
}

impl<H> AsRef<ByteStr> for CString<H>
    where H: MemPool,
{
    fn as_ref(&self) -> &ByteStr {
        self.deref().as_ref()
    }
}

impl<H> TryAsRef<ByteStr> for CString<H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&ByteStr> {
        Ok(self.as_ref())
    }
}

impl<H> TryAsRef<str> for CString<H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&str> {
        self.deref().try_as_ref()
    }
}

impl<H> AsRef<NoNullStr> for CString<H>
    where H: MemPool,
{
    fn as_ref(&self) -> &NoNullStr {
        self.deref().as_ref()
    }
}

impl<H> TryAsRef<NoNullStr> for CString<H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&NoNullStr> {
        Ok(self.as_ref())
    }
}

impl<H> AsRef<CStr> for CString<H>
    where H: MemPool,
{
    fn as_ref(&self) -> &CStr {
        self.deref()
    }
}

impl<H> TryAsRef<CStr> for CString<H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&CStr> {
        Ok(self.as_ref())
    }
}

impl<H> AsMut<NoNullStr> for CString<H>
    where H: MemPool,
{
    fn as_mut(&mut self) -> &mut NoNullStr {
        self.deref_mut().as_mut()
    }
}

impl<H> TryAsMut<NoNullStr> for CString<H>
    where H: MemPool,
{
    fn try_as_mut(&mut self) -> Result<&mut NoNullStr> {
        Ok(self.as_mut())
    }
}

impl<H> AsMut<CStr> for CString<H>
    where H: MemPool,
{
    fn as_mut(&mut self) -> &mut CStr {
        self.deref_mut()
    }
}

impl<H> TryAsMut<CStr> for CString<H>
    where H: MemPool,
{
    fn try_as_mut(&mut self) -> Result<&mut CStr> {
        Ok(self.as_mut())
    }
}

impl<T: ?Sized, H> TryFrom<T> for String<H>
    where H: MemPool+OutOf,
          T: TryAsRef<str>,
{
    fn try_from(t: &T) -> Result<String<H>> {
        let bytes: &[u8] = try!(t.try_as_ref()).as_ref();
        let vec = try!(bytes.try_to());
        unsafe { Ok(String::from_bytes_unchecked(vec)) }
    }
}

impl<T: ?Sized, H> TryFrom<T> for CString<H>
    where H: MemPool+OutOf,
          T: TryAsRef<NoNullStr>,
{
    fn try_from(t: &T) -> Result<CString<H>> {
        let bytes: &[u8] = try!(t.try_as_ref()).as_ref();
        let mut vec = try!(Vec::with_capacity(bytes.len() + 1));
        vec.push_all(bytes);
        vec.push(0);
        unsafe { Ok(CString::from_bytes_unchecked(vec)) }
    }
}

impl<H> Into<Box<str, H>> for String<H>
    where H: MemPool,
{
    fn into(self) -> Box<str, H> {
        let vec: Vec<u8, H> = self.into();
        let bx: Box<[u8], H> = vec.into();
        unsafe {
            let (ptr, pool) = bx.into_raw_parts();
            Box::from_raw_parts(ptr as *mut _, pool)
        }
    }
}

impl<H> Into<Box<CStr, H>> for CString<H>
    where H: MemPool,
{
    fn into(self) -> Box<CStr, H> {
        let vec: Vec<u8, H> = self.into();
        let bx: Box<[u8], H> = vec.into();
        unsafe {
            let (ptr, pool) = bx.into_raw_parts();
            Box::from_raw_parts(ptr as *mut _, pool)
        }
    }
}
