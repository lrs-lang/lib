// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{mem};
use core::ops::{Index, IndexMut, Range, RangeFrom, RangeTo, RangeFull};
use base::rmo::{AsRef, AsMut};
use base::{error};
use arch_fns::{memrchr, memchr};
use fmt::{Debug, Write};

use byte_str::{ByteStr, AsByteStr};
use c_str::{CStr, ToCStr};

/// A byte slice with no null bytes.
pub struct NoNullStr {
    data: [u8],
}

impl NoNullStr {
    /// Sets a byte in the slice to the specified value.
    pub fn set(&mut self, idx: usize, val: u8) {
        assert!(val != 0);
        self.data[idx] = val;
    }

    /// Returns the part after the last '/'.
    pub fn file(&self) -> &NoNullStr {
        let bytes = &self.data;
        match memrchr(bytes, b'/') {
            Some(idx) => unsafe { NoNullStr::from_bytes_unchecked(&bytes[idx+1..]) },
            _ => self,
        }
    }

    /// Like `file`.
    pub fn file_mut(&mut self) -> &mut NoNullStr {
        unsafe { &mut *(self.file() as *const _ as *mut _) }
    }

    /// Returns the part before the last '/'.
    pub fn dir(&self) -> &NoNullStr {
        let bytes = &self.data;
        match memrchr(bytes, b'/') {
            Some(idx) => unsafe { NoNullStr::from_bytes_unchecked(&bytes[..idx]) },
            _ => unsafe { NoNullStr::from_bytes_unchecked(&[]) },
        }
    }

    /// Like `dir`.
    pub fn dir_mut(&mut self) -> &mut NoNullStr {
        unsafe { &mut *(self.dir() as *const _ as *mut _) }
    }

    /// Returns the length of the object.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn starts_with<A>(&self, arg: A) -> bool
        where A: AsRef<[u8]>,
    {
        self.as_ref().starts_with(arg.as_ref())
    }

    /// Converts the bytes to a `NoNullStr` without checking for null bytes.
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &NoNullStr {
        mem::cast(bytes)
    }

    /// Like `from_bytes_unchecked`.
    pub unsafe fn from_bytes_unchecked_mut(bytes: &mut [u8]) -> &mut NoNullStr {
        mem::cast(bytes)
    }
}

impl Index<RangeFull> for NoNullStr {
    type Output = NoNullStr;
    fn index(&self, _: RangeFull) -> &NoNullStr { self }
}

impl IndexMut<RangeFull> for NoNullStr {
    fn index_mut(&mut self, _: RangeFull) -> &mut NoNullStr { self }
}

impl Index<RangeTo<usize>> for NoNullStr {
    type Output = NoNullStr;
    fn index(&self, idx: RangeTo<usize>) -> &NoNullStr {
        unsafe { NoNullStr::from_bytes_unchecked(&self.data[idx]) }
    }
}

impl IndexMut<RangeTo<usize>> for NoNullStr {
    fn index_mut(&mut self, idx: RangeTo<usize>) -> &mut NoNullStr {
        unsafe { NoNullStr::from_bytes_unchecked_mut(&mut self.data[idx]) }
    }
}

impl Index<RangeFrom<usize>> for NoNullStr {
    type Output = NoNullStr;
    fn index(&self, idx: RangeFrom<usize>) -> &NoNullStr {
        unsafe { NoNullStr::from_bytes_unchecked(&self.data[idx]) }
    }
}

impl IndexMut<RangeFrom<usize>> for NoNullStr {
    fn index_mut(&mut self, idx: RangeFrom<usize>) -> &mut NoNullStr {
        unsafe { NoNullStr::from_bytes_unchecked_mut(&mut self.data[idx]) }
    }
}

impl Index<Range<usize>> for NoNullStr {
    type Output = NoNullStr;
    fn index(&self, idx: Range<usize>) -> &NoNullStr {
        unsafe { NoNullStr::from_bytes_unchecked(&self.data[idx]) }
    }
}

impl IndexMut<Range<usize>> for NoNullStr {
    fn index_mut(&mut self, idx: Range<usize>) -> &mut NoNullStr {
        unsafe { NoNullStr::from_bytes_unchecked_mut(&mut self.data[idx]) }
    }
}

impl AsRef<[u8]> for NoNullStr {
    fn as_ref(&self) -> &[u8] { &self.data }
}

impl AsByteStr for NoNullStr {
    fn as_byte_str(&self) -> &ByteStr { self.data.as_byte_str() }
}

impl ToCStr for NoNullStr {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        let bytes = &self.data;
        if bytes.len() < buf.len() {
            mem::copy(buf, bytes);
            buf[bytes.len()] = 0;
            unsafe { Ok(CStr::from_bytes_unchecked_mut(&mut buf[..bytes.len()+1])) }
        } else {
            Err(error::NoMemory)
        }
    }
}

impl Debug for NoNullStr {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        self.as_byte_str().fmt(w)
    }
}

///////////////////////////////

/// Objects that can be borrowed as a `NoNullStr`.
pub trait AsNoNullStr {
    /// Tries to borrow the object as a `NoNullStr`.
    fn as_no_null_str(&self) -> Result<&NoNullStr>;
}

/// Like `AsNoNullStr`.
pub trait AsMutNoNullStr {
    fn as_mut_no_null_str(&mut self) -> Result<&mut NoNullStr>;
}

impl<'a, T: AsNoNullStr+?Sized> AsNoNullStr for &'a T {
    fn as_no_null_str(&self) -> Result<&NoNullStr> { (**self).as_no_null_str() }
}

impl AsNoNullStr for [u8] {
    fn as_no_null_str(&self) -> Result<&NoNullStr> {
        match memchr(self, 0) {
            Some(_) => Err(error::InvalidArgument),
            _ => Ok(unsafe { NoNullStr::from_bytes_unchecked(self) })
        }
    }
}

impl AsNoNullStr for NoNullStr { fn as_no_null_str(&self) -> Result<&NoNullStr> { Ok(self) } }
impl AsNoNullStr for [i8] { fn as_no_null_str(&self) -> Result<&NoNullStr> { self.as_ref().as_no_null_str() } }
impl AsNoNullStr for str  { fn as_no_null_str(&self) -> Result<&NoNullStr> { self.as_ref().as_no_null_str() } }

impl AsMutNoNullStr for [u8] {
    fn as_mut_no_null_str(&mut self) -> Result<&mut NoNullStr> {
        match memchr(self, 0) {
            Some(_) => Err(error::InvalidArgument),
            _ => Ok(unsafe { NoNullStr::from_bytes_unchecked_mut(self) })
        }
    }
}

impl AsMutNoNullStr for NoNullStr { fn as_mut_no_null_str(&mut self) -> Result<&mut NoNullStr> { Ok(self) } }
impl AsMutNoNullStr for [i8] { fn as_mut_no_null_str(&mut self) -> Result<&mut NoNullStr> { self.as_mut().as_mut_no_null_str() } }
