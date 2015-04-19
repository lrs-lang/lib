// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use prelude::*;
use arch_fns::{memrchr, memchr};
use byte_str::{ByteStr, AsByteStr};
use core::{mem};
use core::ops::{Index, IndexMut, Range, RangeFrom, RangeTo, RangeFull};
use c_str::{CStr, ToCStr};
use error::{self};
use rmo::{AsRef, AsMut};

pub struct Path {
    data: [u8],
}

impl Path {
    pub fn set(&mut self, idx: usize, val: u8) {
        assert!(val != 0);
        self.data[idx] = val;
    }

    pub fn file(&self) -> &Path {
        let bytes = &self.data;
        match memrchr(bytes, b'/') {
            Some(idx) => unsafe { Path::from_bytes_unchecked(&bytes[idx+1..]) },
            _ => self,
        }
    }

    pub fn file_mut(&mut self) -> &mut Path {
        unsafe { &mut *(self.file() as *const _ as *mut _) }
    }

    pub fn dir(&self) -> &Path {
        let bytes = &self.data;
        match memrchr(bytes, b'/') {
            Some(idx) => unsafe { Path::from_bytes_unchecked(&bytes[..idx]) },
            _ => unsafe { Path::from_bytes_unchecked(&[]) },
        }
    }

    pub fn dir_mut(&mut self) -> &mut Path {
        unsafe { &mut *(self.dir() as *const _ as *mut _) }
    }

    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Path {
        mem::cast(bytes)
    }

    pub unsafe fn from_bytes_unchecked_mut(bytes: &mut [u8]) -> &mut Path {
        mem::cast(bytes)
    }
}

impl Deref for Path {
    type Target = ByteStr;
    fn deref(&self) -> &ByteStr {
        self.as_byte_str()
    }
}

impl Index<usize> for Path {
    type Output = u8;
    fn index(&self, idx: usize) -> &u8 {
        &self.data[idx]
    }
}

impl Index<RangeFull> for Path {
    type Output = Path;
    fn index(&self, _: RangeFull) -> &Path { self }
}

impl IndexMut<RangeFull> for Path {
    fn index_mut(&mut self, _: RangeFull) -> &mut Path { self }
}

impl Index<RangeTo<usize>> for Path {
    type Output = Path;
    fn index(&self, idx: RangeTo<usize>) -> &Path {
        unsafe { Path::from_bytes_unchecked(&self.data[idx]) }
    }
}

impl IndexMut<RangeTo<usize>> for Path {
    fn index_mut(&mut self, idx: RangeTo<usize>) -> &mut Path {
        unsafe { Path::from_bytes_unchecked_mut(&mut self.data[idx]) }
    }
}

impl Index<RangeFrom<usize>> for Path {
    type Output = Path;
    fn index(&self, idx: RangeFrom<usize>) -> &Path {
        unsafe { Path::from_bytes_unchecked(&self.data[idx]) }
    }
}

impl IndexMut<RangeFrom<usize>> for Path {
    fn index_mut(&mut self, idx: RangeFrom<usize>) -> &mut Path {
        unsafe { Path::from_bytes_unchecked_mut(&mut self.data[idx]) }
    }
}

impl Index<Range<usize>> for Path {
    type Output = Path;
    fn index(&self, idx: Range<usize>) -> &Path {
        unsafe { Path::from_bytes_unchecked(&self.data[idx]) }
    }
}

impl IndexMut<Range<usize>> for Path {
    fn index_mut(&mut self, idx: Range<usize>) -> &mut Path {
        unsafe { Path::from_bytes_unchecked_mut(&mut self.data[idx]) }
    }
}

impl AsRef<[u8]> for Path {
    fn as_ref(&self) -> &[u8] { &self.data }
}

impl AsByteStr for Path {
    fn as_byte_str(&self) -> &ByteStr { self.data.as_byte_str() }
}

impl ToCStr for Path {
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

///////////////////////////////

pub trait AsPath {
    fn as_path(&self) -> Result<&Path>;
}

pub trait AsMutPath {
    fn as_mut_path(&mut self) -> Result<&mut Path>;
}

impl AsPath for [u8] {
    fn as_path(&self) -> Result<&Path> {
        match memchr(self, 0) {
            Some(_) => Err(error::InvalidArgument),
            _ => Ok(unsafe { Path::from_bytes_unchecked(self) })
        }
    }
}

impl AsPath for Path { fn as_path(&self) -> Result<&Path> { Ok(self) } }
impl AsPath for [i8] { fn as_path(&self) -> Result<&Path> { self.as_ref().as_path() } }
impl AsPath for str  { fn as_path(&self) -> Result<&Path> { self.as_ref().as_path() } }

impl AsMutPath for [u8] {
    fn as_mut_path(&mut self) -> Result<&mut Path> {
        match memchr(self, 0) {
            Some(_) => Err(error::InvalidArgument),
            _ => Ok(unsafe { Path::from_bytes_unchecked_mut(self) })
        }
    }
}

impl AsMutPath for Path { fn as_mut_path(&mut self) -> Result<&mut Path> { Ok(self) } }
impl AsMutPath for [i8] { fn as_mut_path(&mut self) -> Result<&mut Path> { self.as_mut().as_mut_path() } }
