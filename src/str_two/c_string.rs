// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{mem};
use base::rmo::{AsRef, AsMut};
use str_one::c_str::{CStr, ToCStr};
use fmt::{Debug, Write};
use vec::{Vec};
use alloc::{self, Allocator};

pub struct CString<'a, Heap = alloc::Heap>
    where Heap: Allocator,
{
    data: Vec<'a, u8, Heap>,
}

impl<'a, H> CString<'a, H>
    where H: Allocator,
{
    /// Casts the byte vector directly to a `CString` without checking it for validity.
    pub unsafe fn from_bytes_unchecked(bytes: Vec<'a, u8, H>) -> CString<'a, H> {
        CString { data: bytes }
    }
}

impl<'a, H> Deref for CString<'a, H>
    where H: Allocator,
{
    type Target = CStr;
    fn deref(&self) -> &CStr {
        unsafe { mem::cast(self.data.deref()) }
    }
}

impl<'a, H> Debug for CString<'a, H>
    where H: Allocator,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(self.deref(), w)
    }
}

impl<'a, H> AsRef<CStr> for CString<'a, H>
    where H: Allocator,
{
    fn as_ref(&self) -> &CStr {
        unsafe { CStr::from_bytes_unchecked(&self.data[..]) }
    }
}

impl<'a, H> AsMut<CStr> for CString<'a, H>
    where H: Allocator,
{
    fn as_mut(&mut self) -> &mut CStr {
        unsafe { CStr::from_bytes_unchecked_mut(&mut self.data[..]) }
    }
}

impl<'b, H> ToCStr for CString<'b, H>
    where H: Allocator,
{
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        self.deref().to_cstr(buf)
    }
}
