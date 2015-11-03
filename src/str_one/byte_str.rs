// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::ops::{Index, IndexMut, Range, RangeFrom, RangeTo, RangeFull, PartialOrd};
use core::cmp::{Ord, Ordering};
use core::{mem, str};
// use base::unused::{UnusedState};
use fmt::{self, Debug, Display, UpperHex, Write};
use parse::{Parse, Parsable};

use c_str::{CStr, ToCStr};
use no_null_str::{AsNoNullStr, AsMutNoNullStr, NoNullStr};

/// A borrowed byte sequence that can be interpreted as a string.
///
/// = Remarks
///
/// The Debug implementation prints strings in the formk `"string"` where all letters that
/// are not in the printable ASCII set are printed as escape sequences of the form
/// `\u{number}`.
///
/// The Display implementation writes the contained bytes directly to the output.
pub struct ByteStr {
    data: [u8],
}

/// Objects that can be borrowed as a byte string.
///
/// = Remarks
///
/// This will likely be replaced by type ascription.
pub trait AsByteStr {
    /// Borrows the object as a byte string.
    fn as_byte_str(&self) -> &ByteStr;
}

/// Objects that can be mutably borrowed as a byte string.
///
/// = Remarks
///
/// This will likely be replaced by type ascription.
pub trait AsMutByteStr {
    /// Borrows the object as a mutable byte string.
    fn as_mut_byte_str(&mut self) -> &mut ByteStr;
}

impl ByteStr {
    /// Returns a byte string created by removing spaces and tabs from the start and end
    /// of the string.
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

    /// Returns whether the string starts with a byte slice.
    ///
    /// [argument, arg]
    /// The byte slice to be checked.
    pub fn starts_with<A>(&self, arg: A) -> bool
        where A: AsRef<[u8]>,
    {
        self.data.starts_with(arg.as_ref())
    }
}

// unsafe impl<'a> UnusedState for &'a ByteStr {
//     type Plain = <&'static [u8] as UnusedState>::Plain;
//     const NUM: usize = <&'static [u8] as UnusedState>::NUM;
// 
//     fn unused_state(n: usize) -> Self::Plain {
//         <&'static [u8] as UnusedState>::unused_state(n)
//     }
// }
// 
// unsafe impl<'a> UnusedState for &'a mut ByteStr {
//     type Plain = <&'static mut [u8] as UnusedState>::Plain;
//     const NUM: usize = <&'static mut [u8] as UnusedState>::NUM;
// 
//     fn unused_state(n: usize) -> Self::Plain {
//         <&'static mut [u8] as UnusedState>::unused_state(n)
//     }
// }

impl Deref for ByteStr {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.data
    }
}

impl DerefMut for ByteStr {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.data
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

impl<T: AsRef<[u8]>> PartialOrd<T> for ByteStr {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: AsRef<[u8]>> Ord<T> for ByteStr {
    fn cmp(&self, other: &T) -> Ordering {
        self.data.cmp(other.as_ref())
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
        let mut bytes: &[u8] = self.as_ref();
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
        self.data.parse()
    }
}

impl AsRef<ByteStr> for [u8] {
    fn as_ref(&self) -> &ByteStr {
        unsafe { mem::cast(self) }
    }
}

impl AsMut<ByteStr> for [u8] {
    fn as_mut(&mut self) -> &mut ByteStr {
        unsafe { mem::cast(self) }
    }
}

impl AsRef<ByteStr> for [i8] {
    fn as_ref(&self) -> &ByteStr {
        let bytes: &[u8] = self.as_ref();
        bytes.as_ref()
    }
}

impl AsMut<ByteStr> for [i8] {
    fn as_mut(&mut self) -> &mut ByteStr {
        let bytes: &mut [u8] = self.as_mut();
        bytes.as_mut()
    }
}

impl AsRef<ByteStr> for str {
    fn as_ref(&self) -> &ByteStr {
        let bytes: &[u8] = self.as_ref();
        bytes.as_ref()
    }
}

impl AsRef<ByteStr> for NoNullStr {
    fn as_ref(&self) -> &ByteStr {
        let bytes: &[u8] = self.as_ref();
        bytes.as_ref()
    }
}

impl AsRef<ByteStr> for CStr {
    fn as_ref(&self) -> &ByteStr {
        let bytes: &[u8] = self.as_ref();
        bytes.as_ref()
    }
}

impl<T: ?Sized> AsByteStr for T
    where T: AsRef<ByteStr>,
{
    fn as_byte_str(&self) -> &ByteStr {
        self.as_ref()
    }
}

impl<T: ?Sized> AsMutByteStr for T
    where T: AsMut<ByteStr>,
{
    fn as_mut_byte_str(&mut self) -> &mut ByteStr {
        self.as_mut()
    }
}
