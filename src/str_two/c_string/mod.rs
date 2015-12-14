// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use core::marker::{Leak};
use base::undef::{UndefState};
use str_one::{CStr, NoNullStr};
use fmt::{Debug, Display, Write};
use vec::{Vec};
use arch_fns::{memchr};
use alloc::{self, MemPool};

/// An owned byte slice that has exactly one null byte at the very end.
pub struct CString<Heap: MemPool+?Sized = alloc::Heap>(Vec<u8, Heap>);

impl<H> CString<H>
    where H: MemPool,
{
    /// Creates a new `CString`.
    pub fn new() -> Result<Self>
        where H: OutOf,
    {
        Self::with_pool(H::out_of(()))
    }

    pub fn with_pool(pool: H) -> Result<Self> {
        let mut v = Vec::with_pool(pool);
        try!(v.reserve(1));
        v.push(0);
        v.pop();
        Ok(CString(v))
    }

    pub fn from_bytes(mut bytes: Vec<u8, H>) -> Result<Self, Vec<u8, H>> {
        match memchr(&bytes, 0) {
            Some(pos) if pos == bytes.len() - 1 => {
                bytes.pop();
                Ok(CString(bytes))
            },
            Some(_) => Err(bytes),
            None => if bytes.reserve(1).is_ok() {
                bytes.push(0);
                bytes.pop();
                Ok(CString(bytes))
            } else {
                Err(bytes)
            },
        }
    }

    /// Creates a `CString` by wrapping a vector without checking the vector for validity.
    ///
    /// [argument, bytes]
    /// The vector to be wrapped.
    ///
    /// = Remarks
    ///
    /// If the vector doesn't have exactly one null byte as its last entry, the behavior
    /// is undefined.
    pub unsafe fn from_bytes_unchecked(mut bytes: Vec<u8, H>) -> Self {
        bytes.pop();
        CString(bytes)
    }

    pub fn leak<'a>(mut self) -> &'a mut CStr
        where Self: Leak,
              H: 'a,
    {
        let p = unsafe { mem::cast(self.deref_mut()) };
        mem::forget(self);
        p
    }
}

impl<H: ?Sized> CString<H>
    where H: MemPool,
{
    /// Truncates the string to length `0`.
    pub fn clear(&mut self) {
        self.truncate(0);
    }

    /// Truncates the string to a certain size.
    ///
    /// [argument, size]
    /// The new length of the string.
    pub fn truncate(&mut self, size: usize) {
        self.0.truncate(size);
        self.0.push(0);
        self.0.pop();
    }

    /// Reserves memory for new bytes in the string.
    ///
    /// [argument, size]
    /// The amount of memory to reserve.
    pub fn reserve(&mut self, size: usize) -> Result {
        self.0.reserve(size + 1)
    }

    /// Appends a filename to the string.
    ///
    /// [argument, name]
    /// The name of the file.
    ///
    /// = Remarks
    ///
    /// This first appends a '/' and then the provided filename to the buffer.
    pub fn push_file<F: ?Sized>(&mut self, name: &F) -> Result
        where F: TryAsRef<NoNullStr>,
    {
        let bytes: &[u8] = try!(name.try_as_ref()).as_ref();
        try!(self.reserve(bytes.len() + 1));
        self.0.push(b'/');
        self.0.push_all(bytes);
        self.0.push(0);
        self.0.pop();
        Ok(())
    }

    /// Removes the file-part of the string.
    ///
    /// = Remarks
    ///
    /// This first searches for the last '/' in the string, removes the trailing part up
    /// to and including the '/' and returns a reference to the part after the '/'.
    pub fn pop_file(&mut self) {
        let dir_len = self.dir().len();
        self.truncate(dir_len);
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
    pub fn pop_file_ref(&mut self) -> &mut CStr {
        if self.len() == 0 {
            return &mut self[..];
        }

        let (dir_len, file_len) = match self.split_path() {
            (d, f) => (d.len(), f.len())
        };

        if file_len == self.len() {
            self.0.insert(0, 0);
        }

        unsafe {
            let file: &'static mut _ = mem::cast(&mut self[dir_len+1..]);
            self.truncate(dir_len);
            file
        }
    }

    /// Clears the string and sets it to a new value.
    ///
    /// [argument, path]
    /// The new contents of the string.
    pub fn set_path<F: ?Sized>(&mut self, path: &F) -> Result
        where F: TryAsRef<NoNullStr>,
    {
        let bytes: &[u8] = try!(path.try_as_ref()).as_ref();
        self.clear();
        try!(self.reserve(bytes.len()));
        self.0.push_all(bytes);
        self.0.push(0);
        self.0.pop();
        Ok(())
    }

    pub fn shrink_to_fit(&mut self) -> Result {
        self.0.push(0);
        let res = self.0.shrink_to_fit();
        self.0.pop();
        res
    }
}

unsafe impl<H> UndefState for CString<H>
    where H: MemPool, 
{
    fn num() -> usize { <Vec<u8, H> as UndefState>::num() }

    unsafe fn set_undef(val: *mut CString<H>, n: usize) {
        <Vec<u8, H> as UndefState>::set_undef(&mut (*val).0, n)
    }

    unsafe fn is_undef(val: *const CString<H>, n: usize) -> bool {
        <Vec<u8, H> as UndefState>::is_undef(&(*val).0, n)
    }
}

impl<H: ?Sized> Deref for CString<H>
    where H: MemPool,
{
    type Target = CStr;
    fn deref(&self) -> &CStr {
        unsafe { mem::cast(self.0.deref()) }
    }
}

impl<H: ?Sized> DerefMut for CString<H>
    where H: MemPool,
{
    fn deref_mut(&mut self) -> &mut CStr {
        unsafe { mem::cast(self.0.deref_mut()) }
    }
}

impl<H: ?Sized> Debug for CString<H>
    where H: MemPool,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(self.deref(), w)
    }
}

impl<H: ?Sized> Display for CString<H>
    where H: MemPool,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Display::fmt(self.deref(), w)
    }
}

impl<H> Into<Vec<u8, H>> for CString<H>
    where H: MemPool,
{
    fn into(self) -> Vec<u8, H> {
        self.0
    }
}
