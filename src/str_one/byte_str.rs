// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::ops::{Index, IndexMut, Range, RangeFrom, RangeTo, RangeFull};
use core::{str};
use base::rmo::{AsRef, AsMut};
use fmt::{self, Debug, Display, UpperHex, Write};
use parse::{Parse, Parsable};

use c_str::{CStr, ToCStr};
use no_null_str::{AsNoNullStr, AsMutNoNullStr, NoNullStr};

/// A borrowed byte sequence that can be interpreted as a string but doesn't necessarily
/// contain UTF-8.
pub struct ByteStr {
    data: [u8],
}

impl ByteStr {
    /// Returns the length in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn trim(&self) -> &ByteStr {
        let mut start = 0;
        let mut end = self.data.len();
        while start < self.data.len() {
            match self.data[start] {
                b' ' | b'\t' => { },
                _ => break,
            }
            start += 1;
        }
        while end > start {
            match self.data[end-1] {
                b' ' | b'\t' => { },
                _ => break,
            }
            end -= 1;
        }
        self.data[start..end].as_ref()
    }
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
        self.data[idx].as_ref()
    }
}

impl IndexMut<RangeTo<usize>> for ByteStr {
    fn index_mut(&mut self, idx: RangeTo<usize>) -> &mut ByteStr {
        self.data[idx].as_mut()
    }
}

impl Index<RangeFrom<usize>> for ByteStr {
    type Output = ByteStr;
    fn index(&self, idx: RangeFrom<usize>) -> &ByteStr {
        self.data[idx].as_ref()
    }
}

impl IndexMut<RangeFrom<usize>> for ByteStr {
    fn index_mut(&mut self, idx: RangeFrom<usize>) -> &mut ByteStr {
        self.data[idx].as_mut()
    }
}

impl Index<Range<usize>> for ByteStr {
    type Output = ByteStr;
    fn index(&self, idx: Range<usize>) -> &ByteStr {
        self.data[idx].as_ref()
    }
}

impl IndexMut<Range<usize>> for ByteStr {
    fn index_mut(&mut self, idx: Range<usize>) -> &mut ByteStr {
        self.data[idx].as_mut()
    }
}

impl AsRef<[u8]> for ByteStr {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl AsMut<[u8]> for ByteStr {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

impl AsNoNullStr for ByteStr {
    fn as_no_null_str(&self) -> Result<&NoNullStr> {
        self.data.as_no_null_str()
    }
}

impl AsMutNoNullStr for ByteStr {
    fn as_mut_no_null_str(&mut self) -> Result<&mut NoNullStr> {
        self.data.as_mut_no_null_str()
    }
}

impl ToCStr for ByteStr {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        self.data.to_cstr(buf)
    }
}

impl Debug for ByteStr {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        let mut bytes = self.as_ref();
        try!(w.write_all(b"\""));
        while bytes.len() > 0 {
            let remove = {
                let as_str = str::longest_sequence(bytes);
                try!(fmt::impls::str::debug_str_no_quotes(as_str, w));
                as_str.len()
            };
            bytes = &bytes[remove..];
            if bytes.len() > 0 {
                try!(w.write_all(b"\\x"));
                try!(UpperHex::fmt(&bytes[0], w));
                bytes = &bytes[1..];
            }
        }
        try!(w.write_all(b"\""));
        Ok(())
    }
}

impl Display for ByteStr {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        w.write_all(&self.data).ignore_ok()
    }
}

impl Parse for ByteStr {
    fn parse<P: Parsable>(&self) -> Result<P> {
        self.as_ref().parse()
    }
}

////////////////////////

/// Objects that can be borrowed as a byte string.
pub trait AsByteStr {
    fn as_byte_str(&self) -> &ByteStr;
}

/// Objects that can be mutably borrowed as a byte string.
pub trait AsMutByteStr: AsByteStr {
    fn as_mut_byte_str(&mut self) -> &mut ByteStr;
}

impl<T: ?Sized> AsByteStr for T
    where T: AsRef<[u8]>,
{
    fn as_byte_str(&self) -> &ByteStr {
        self.as_ref().as_ref()
    }
}

impl<T: ?Sized> AsMutByteStr for T
    where T: AsRef<[u8]> + AsMut<[u8]>,
{
    fn as_mut_byte_str(&mut self) -> &mut ByteStr {
        self.as_mut().as_mut()
    }
}
