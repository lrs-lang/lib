// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use arch_fns::{memchr};
use base::undef::{UndefState};
use base::default::{Default};
use core::{mem};
use str_one::{NoNullStr, AsNoNullStr, AsMutNoNullStr, AsMutCStr, CStr};
use vec::{Vec};
use fmt::{Debug, Write};
use alloc::{self, Allocator};

/// An owned byte slice with no null bytes.
pub struct NoNullString<Heap = alloc::Heap>
    where Heap: Allocator,
{
    data: Vec<u8, Heap>,
}

impl<H> NoNullString<H>
    where H: Allocator,
          H::Pool: Default,
{
    /// Creates a new, allocated `NoNullString`.
    pub fn new() -> NoNullString<H> {
        NoNullString { data: Vec::new() }
    }
}

impl<'a> NoNullString<alloc::NoMem<'a>> {
    /// Creates a `NoNullString` that is backed by borrowed memory.
    pub fn buffered(buf: &'a mut [u8]) -> NoNullString<alloc::NoMem<'a>> {
        NoNullString { data: Vec::buffered(buf) }
    }
}

impl<H> NoNullString<H>
    where H: Allocator,
{
    pub fn from_bytes(bytes: Vec<u8, H>) -> Result<NoNullString<H>, Vec<u8, H>> {
        if memchr(&bytes, 0).is_some() {
            Err(bytes)
        } else {
            Ok(NoNullString { data: bytes })
        }
    }

    /// Creates a `NoNullString` by wrapping a vector without checking it for validity.
    ///
    /// [argument, bytes]
    /// The vector to be wrapped.
    ///
    /// = Remarks
    ///
    /// If the vector contains null bytes, the behavior is undefined.
    pub unsafe fn from_bytes_unchecked(bytes: Vec<u8, H>) -> NoNullString<H> {
        NoNullString { data: bytes }
    }

    /// Truncates the string to length `0`.
    pub fn clear(&mut self) {
        self.data.truncate(0);
    }

    /// Truncates the string to a certain size.
    ///
    /// [argument, size]
    /// The new length of the string.
    pub fn truncate(&mut self, size: usize) {
        self.data.truncate(size);
    }

    /// Reserves memory for new bytes in the string.
    ///
    /// [argument, size]
    /// The amount of memory to reserve.
    pub fn reserve(&mut self, size: usize) -> Result {
        self.data.reserve(size)
    }

    /// Returns a slice to the unused but reserved memory in the underlying vector.
    pub fn unused(&mut self) -> &mut [u8] {
        self.data.unused()
    }

    /// Truncates the string to a certain size that can be greater than the current size.
    ///
    /// [argument, size]
    /// The new length of the string.
    ///
    /// = Remarks
    ///
    /// If the new range of the string contains null bytes, the behavior is undefined.
    pub unsafe fn set_len(&mut self, size: usize) {
        self.data.set_len(size);
    }

    /// Appends a filename to the string.
    ///
    /// [argument, name]
    /// The name of the file.
    ///
    /// = Remarks
    ///
    /// This first appends a '/' and then the provided filename to the buffer.
    pub fn push_file<F>(&mut self, name: F) -> Result
        where F: AsNoNullStr,
    {
        let bytes: &[u8] = try!(name.as_no_null_str()).as_ref();
        try!(self.data.reserve(bytes.len() + 1));
        self.data.push(b'/');
        self.data.push_all(bytes)
    }

    /// Removes the file-part of the string, returning a reference to it.
    ///
    /// [return_value]
    /// Returns the now removed file-part.
    ///
    /// = Remarks
    ///
    /// This first searches for the last '/' in the string, removes the trailing part up
    /// to and including the '/' and returns a reference to the part after the '/'.
    pub fn pop_file(&mut self) -> &mut NoNullStr {
        if self.len() == 0 {
            return &mut self[..];
        }

        let dir_len = self.dir().len();
        unsafe {
            let file: &'static mut NoNullStr = mem::cast(&mut self[dir_len + 1..]);
            self.data.set_len(dir_len);
            file
        }
    }

    /// Clears the string and sets it to a new value.
    ///
    /// [argument, path]
    /// The new contents of the string.
    pub fn set_path<F>(&mut self, path: F) -> Result
        where F: AsNoNullStr,
    {
        let bytes: &[u8] = try!(path.as_no_null_str()).as_ref();
        self.clear();
        try!(self.data.reserve(bytes.len()));
        self.data.push_all(bytes)
    }
}

impl<H> Deref for NoNullString<H>
    where H: Allocator,
{
    type Target = NoNullStr;
    fn deref(&self) -> &NoNullStr {
        self.as_ref()
    }
}

unsafe impl<H> UndefState for NoNullString<H>
    where H: Allocator,
{
    fn num() -> usize { <Vec<u8, H> as UndefState>::num() }

    unsafe fn set_undef(val: *mut NoNullString<H>, n: usize) {
        <Vec<u8, H> as UndefState>::set_undef(&mut (*val).data, n)
    }

    unsafe fn is_undef(val: *const NoNullString<H>, n: usize) -> bool {
        <Vec<u8, H> as UndefState>::is_undef(&(*val).data, n)
    }
}

impl<H> DerefMut for NoNullString<H>
    where H: Allocator,
{
    fn deref_mut(&mut self) -> &mut NoNullStr {
        self.as_mut()
    }
}

impl<H> AsRef<[u8]> for NoNullString<H>
    where H: Allocator,
{
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl<H> AsRef<NoNullStr> for NoNullString<H>
    where H: Allocator,
{
    fn as_ref(&self) -> &NoNullStr {
        unsafe { NoNullStr::from_bytes_unchecked(&self.data) }
    }
}

impl<H> AsMut<NoNullStr> for NoNullString<H>
    where H: Allocator,
{
    fn as_mut(&mut self) -> &mut NoNullStr {
        unsafe { NoNullStr::from_mut_bytes_unchecked(&mut self.data) }
    }
}

impl<H> AsNoNullStr for NoNullString<H>
    where H: Allocator,
{
    fn as_no_null_str(&self) -> Result<&NoNullStr> {
        unsafe { Ok(NoNullStr::from_bytes_unchecked(&self.data)) }
    }
}

impl<H> AsMutNoNullStr for NoNullString<H>
    where H: Allocator,
{
    fn as_mut_no_null_str(&mut self) -> Result<&mut NoNullStr> {
        unsafe { Ok(NoNullStr::from_mut_bytes_unchecked(&mut self.data)) }
    }
}

impl<H> Debug for NoNullString<H>
    where H: Allocator,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        let nns: &NoNullStr = self.as_ref();
        nns.fmt(w)
    }
}

impl<H> AsMutCStr for NoNullString<H>
    where H: Allocator,
{
    fn as_mut_cstr(&mut self) -> Result<&mut CStr> {
        // We push a 0 at the end, create a slice, then truncate without reallocating so
        // that after the reference is dropped the null is gone.

        try!(self.data.reserve(1));
        self.data.push(0);
        let cstr: &'static mut CStr = unsafe {
            mem::cast(CStr::from_mut_bytes_unchecked(&mut self.data[..]))
        };
        unsafe { self.data.set_len(cstr.len()); }
        Ok(cstr)
    }
}
