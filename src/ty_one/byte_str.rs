// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use prelude::*;
use core::ops::{Index, IndexMut, Range, RangeFrom, RangeTo, RangeFull};
use core::{mem};
use core::ops::{Eq};

use c_str::{CStr, ToCStr};
use path::{AsPath, AsMutPath, Path};
use bytes::{AsBytes, AsMutBytes};

/// A borrowed byte sequence that can be interpreted as a string but doesn't necessarily
/// contain UTF-8.
pub struct ByteStr {
    data: [u8],
}

impl Deref for ByteStr {
    type Target = [u8];
    fn deref(&self) -> &[u8] { &self.data }
}
impl DerefMut for ByteStr {
    fn deref_mut(&mut self) -> &mut [u8] { &mut self.data }
}

impl Index<usize> for ByteStr {
    type Output = u8;
    fn index(&self, idx: usize) -> &u8 {
        &self.data[idx]
    }
}

impl IndexMut<usize> for ByteStr {
    fn index_mut(&mut self, idx: usize) -> &mut u8 {
        &mut self.data[idx]
    }
}

impl Index<RangeFull> for ByteStr {
    type Output = ByteStr;
    fn index(&self, _: RangeFull) -> &ByteStr { self }
}

impl IndexMut<RangeFull> for ByteStr {
    fn index_mut(&mut self, _: RangeFull) -> &mut ByteStr { self }
}

impl Index<RangeTo<usize>> for ByteStr {
    type Output = ByteStr;
    fn index(&self, idx: RangeTo<usize>) -> &ByteStr {
        self.data[idx].as_byte_str()
    }
}

impl IndexMut<RangeTo<usize>> for ByteStr {
    fn index_mut(&mut self, idx: RangeTo<usize>) -> &mut ByteStr {
        self.data[idx].as_mut_byte_str()
    }
}

impl Index<RangeFrom<usize>> for ByteStr {
    type Output = ByteStr;
    fn index(&self, idx: RangeFrom<usize>) -> &ByteStr {
        self.data[idx].as_byte_str()
    }
}

impl IndexMut<RangeFrom<usize>> for ByteStr {
    fn index_mut(&mut self, idx: RangeFrom<usize>) -> &mut ByteStr {
        self.data[idx].as_mut_byte_str()
    }
}

impl Index<Range<usize>> for ByteStr {
    type Output = ByteStr;
    fn index(&self, idx: Range<usize>) -> &ByteStr {
        self.data[idx].as_byte_str()
    }
}

impl IndexMut<Range<usize>> for ByteStr {
    fn index_mut(&mut self, idx: Range<usize>) -> &mut ByteStr {
        self.data[idx].as_mut_byte_str()
    }
}

impl AsBytes for ByteStr { fn as_bytes(&self) -> &[u8] { &self.data } }
impl AsMutBytes for ByteStr { fn as_mut_bytes(&mut self) -> &mut [u8] { &mut self.data } }

impl AsPath for ByteStr {
    fn as_path(&self) -> Result<&Path> {
        self.data.as_path()
    }
}

impl AsMutPath for ByteStr {
    fn as_mut_path(&mut self) -> Result<&mut Path> {
        self.data.as_mut_path()
    }
}

impl ToCStr for ByteStr {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        self.data.to_cstr(buf)
    }
}

impl Eq for ByteStr {
    fn eq(&self, other: &ByteStr) -> bool {
        self.data.eq(&other.data)
    }
}

////////////////////////

/// A trait for converting to a borrowed linux string.
pub trait AsByteStr {
    fn as_byte_str(&self) -> &ByteStr;
}

impl AsByteStr for ByteStr { fn as_byte_str(&self) -> &ByteStr { self } }
impl AsByteStr for [i8]    { fn as_byte_str(&self) -> &ByteStr { unsafe { mem::cast(self) } } }
impl AsByteStr for [u8]    { fn as_byte_str(&self) -> &ByteStr { unsafe { mem::cast(self) } } }
impl AsByteStr for str     { fn as_byte_str(&self) -> &ByteStr { unsafe { mem::cast(self) } } }

impl<'a, T: AsByteStr+?Sized> AsByteStr for &'a T {
    fn as_byte_str(&self) -> &ByteStr { (*self).as_byte_str() }
}

/// A trait for converting to a mutably borrowed linux string.
pub trait AsMutByteStr: AsByteStr {
    fn as_mut_byte_str(&mut self) -> &mut ByteStr;
}

impl AsMutByteStr for ByteStr { fn as_mut_byte_str(&mut self) -> &mut ByteStr { self   } }
impl AsMutByteStr for [u8]    { fn as_mut_byte_str(&mut self) -> &mut ByteStr { unsafe { mem::cast(self) } } }
