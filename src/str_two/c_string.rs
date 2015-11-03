// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use base::undef::{UndefState};
use str_one::c_str::{CStr, ToCStr};
use fmt::{Debug, Write};
use vec::{Vec};
use alloc::{self, Allocator};

/// An owned byte slice that has exactly one null byte at the very end.
pub struct CString<Heap = alloc::Heap>
    where Heap: Allocator,
{
    data: Vec<u8, Heap>,
}

impl<H> CString<H>
    where H: Allocator,
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

unsafe impl<H> UndefState for CString<H>
    where H: Allocator, 
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
    where H: Allocator,
{
    type Target = CStr;
    fn deref(&self) -> &CStr {
        unsafe { mem::cast(self.data.deref()) }
    }
}

impl<H> Debug for CString<H>
    where H: Allocator,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(self.deref(), w)
    }
}

impl<H> AsRef<CStr> for CString<H>
    where H: Allocator,
{
    fn as_ref(&self) -> &CStr {
        self.deref()
    }
}

impl<H> AsMut<CStr> for CString<H>
    where H: Allocator,
{
    fn as_mut(&mut self) -> &mut CStr {
        unsafe { CStr::from_mut_bytes_unchecked(&mut self.data[..]) }
    }
}

impl<H> ToCStr for CString<H>
    where H: Allocator,
{
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        self.deref().to_cstr(buf)
    }
}
