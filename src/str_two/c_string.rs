// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{mem};
use base::rmo::{AsRef, AsMut};
use base::unused::{UnusedState};
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

unsafe impl<H> UnusedState for CString<H>
    where H: Allocator<Pool = ()>,
{
    type Plain = <Vec<u8, H> as UnusedState>::Plain;
    // FIXME: Should be Vec<u8, H>
    const NUM: usize = <Vec<u8, alloc::Heap> as UnusedState>::NUM;

    fn unused_state(n: usize) -> [usize; 4] {
        assert!(mem::size_of::<CString<H>>() == mem::size_of::<Self::Plain>());
        <Vec<u8, H> as UnusedState>::unused_state(n)
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
