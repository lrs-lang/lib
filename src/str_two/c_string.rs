// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use base::undef::{UndefState};
use str_one::c_str::{CStr, ToCStr};
use fmt::{Debug, Write};
use vec::{Vec};
use alloc::{self, MemPool};
use {ByteString, NoNullString};

/// An owned byte slice that has exactly one null byte at the very end.
pub struct CString<Heap = alloc::Heap>
    where Heap: MemPool,
{
    data: Vec<u8, Heap>,
}

impl<H> CString<H>
    where H: MemPool,
{
    /// Creates a `CString` by wrapping a vector without checking the vector for validity.
    ///
    /// [argument, bytes]
    /// The vector to be wrapped.
    ///
    /// = Remarks
    ///
    /// If the vector doesn't have exactly one null byte as its last entry, the behavior
    /// is undefined.
    pub unsafe fn from_bytes_unchecked(bytes: Vec<u8, H>) -> CString<H> {
        CString { data: bytes }
    }
}

impl<H> Into<Vec<u8, H>> for CString<H>
    where H: MemPool, 
{
    fn into(mut self) -> Vec<u8, H> {
        self.data.pop();
        self.data
    }
}

impl<H> Into<ByteString<H>> for CString<H>
    where H: MemPool, 
{
    fn into(self) -> ByteString<H> {
        ByteString::from_vec(self.into())
    }
}

impl<H> Into<NoNullString<H>> for CString<H>
    where H: MemPool, 
{
    fn into(self) -> NoNullString<H> {
        unsafe { NoNullString::from_bytes_unchecked(self.into()) }
    }
}

unsafe impl<H> UndefState for CString<H>
    where H: MemPool, 
{
    fn num() -> usize { <Vec<u8, H> as UndefState>::num() }

    unsafe fn set_undef(val: *mut CString<H>, n: usize) {
        <Vec<u8, H> as UndefState>::set_undef(&mut (*val).data, n)
    }

    unsafe fn is_undef(val: *const CString<H>, n: usize) -> bool {
        <Vec<u8, H> as UndefState>::is_undef(&(*val).data, n)
    }
}

impl<H> Deref for CString<H>
    where H: MemPool,
{
    type Target = CStr;
    fn deref(&self) -> &CStr {
        unsafe { mem::cast(self.data.deref()) }
    }
}

impl<H> Debug for CString<H>
    where H: MemPool,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(self.deref(), w)
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
        Ok(self.deref())
    }
}

impl<H> AsMut<CStr> for CString<H>
    where H: MemPool,
{
    fn as_mut(&mut self) -> &mut CStr {
        unsafe { CStr::from_mut_bytes_unchecked(&mut self.data[..]) }
    }
}

impl<H> TryAsMut<CStr> for CString<H>
    where H: MemPool,
{
    fn try_as_mut(&mut self) -> Result<&mut CStr> {
        Ok(self.as_mut())
    }
}

impl<H> ToCStr for CString<H>
    where H: MemPool,
{
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        self.deref().to_cstr(buf)
    }
}

impl<H> TryFrom<CStr> for CString<H>
    where H: MemPool+Default,
{
    fn try_from(c: &CStr) -> Result<CString<H>> {
        let bytes = c.bytes_with_null();
        bytes.try_to().map(|o| unsafe { CString::from_bytes_unchecked(o) })
    }
}
