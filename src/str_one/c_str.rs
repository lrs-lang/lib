// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::ops::{Index, IndexMut, RangeFrom, RangeTo, Range, RangeFull};
use core::{mem, slice};
// use base::unused::{UnusedState};
use base::{error};
use arch_fns::{all_bytes, memchr, strlen};
use cty_base::types::{c_char};
use fmt::{Debug, Write};
use parse::{Parse, Parsable};

use byte_str::{AsByteStr};
use no_null_str::{AsNoNullStr, AsMutNoNullStr, NoNullStr};

/// A byte slice that has exactly one null byte at the very end.
pub struct CStr {
    data: [u8]
}

/// Objects that can be interpreted as `CStr`s.
///
/// = Remarks
///
/// This operation can fail if the object has interior null bytes, e.g.,
/// `"Hello World\0\0\0"` will succeed but `"Hello\0World\0"` will fail.
pub trait AsCStr : ToCStr+AsRef<[u8]> {
    /// Borrows the object as a `CStr`.
    fn as_cstr(&self) -> Result<&CStr>;
}

/// Objects that can be interpreted as mutable `CStr`s.
///
/// = Remarks
///
/// This operation can fail if the object has interior null bytes, e.g.,
/// `"Hello World\0\0\0"` will succeed but `"Hello\0World\0"` will fail.
pub trait AsMutCStr {
    /// Borrows the object mutably as a `CStr`.
    fn as_mut_cstr(&mut self) -> Result<&mut CStr>;
}

/// Objects that can be transformed into `CStr`s provided they have some scratch space
/// available.
///
/// = Remarks
///
/// For example, "Hello World" needs to be copied and a `0` appended to form a valid
/// `CStr`. This operation can fail under the same conditions as `AsCStr`.
pub trait ToCStr {
    /// Converts the object by copying it.
    ///
    /// [argument, buf]
    /// The buffer in which the `CStr` will be stored.
    ///
    /// [return_value]
    /// Returns the created `CStr`.
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr>;

    /// Tries to create a `CStr` without copying and copies if that's not possible.
    ///
    /// [argument, buf]
    /// The buffer in which the `CStr` will be created if it has to be copied.
    ///
    /// [return_value]
    /// Returns the borrowed or created `CStr`.
    ///
    /// = Remarks
    ///
    /// For example, `"Hello World\0"` does not have to be copied. The default
    /// implementation simply calls `to_cstr`.
    fn to_or_as_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a CStr> {
        self.to_cstr(buf).map(|r| &*r)
    }

    /// Tries to create a mutable `CStr` without copying and copies if that's not
    /// possible.
    ///
    /// [argument, buf]
    /// The buffer in which the `CStr` will be created if it has to be copied.
    ///
    /// [return_value]
    /// Returns the borrowed or created `CStr`.
    ///
    /// = Remarks
    ///
    /// The default implementation simply calls `to_cstr`.
    fn to_or_as_mut_cstr<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        self.to_cstr(buf)
    }
}

impl CStr {
    /// Creates a new `CStr` from a pointer.
    ///
    /// [argument, ptr]
    /// A pointer to a null-terminated string.
    ///
    /// = Remarks
    ///
    /// If `ptr` is not a null terminated array of bytes, the behavior is undefined.
    pub unsafe fn from_ptr(ptr: *const c_char) -> &'static CStr {
        mem::cast(slice::from_ptr(ptr, strlen(ptr as *const _) + 1))
    }

    /// Returns an empty `CStr`.
    pub fn empty() -> &'static CStr {
        static EMPTY: [u8; 1] = [0];
        unsafe { mem::cast(&EMPTY[..]) }
    }

    /// Returns a pointer to the first byte in the `CStr`.
    pub fn as_ptr(&self) -> *const c_char {
        self.data.as_ptr() as *const c_char
    }

    /// Returns a mutable pointer to the first byte in the `CStr`.
    pub fn as_mut_ptr(&mut self) -> *mut c_char {
        self.data.as_mut_ptr() as *mut c_char
    }

    /// Casts a byte slice to a `CStr` without checking it for validity.
    ///
    /// [argument, bytes]
    /// The slice to be interpreted as a `CStr`.
    ///
    /// = Remarks
    ///
    /// If the slice doesn't have exactly one null byte as its last entry, the behavior is
    /// undefined.
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &CStr {
        mem::cast(bytes)
    }

    /// Casts a byte slice to a mutable `CStr` without checking it for validity.
    ///
    /// [argument, bytes]
    /// The slice to be interpreted as a `CStr`.
    ///
    /// = Remarks
    ///
    /// If the slice doesn't have exactly one null byte as its last entry, the behavior is
    /// undefined.
    pub unsafe fn from_mut_bytes_unchecked(bytes: &mut [u8]) -> &mut CStr {
        mem::cast(bytes)
    }

    /// Returns the contained bytes including the null byte.
    pub fn bytes_with_null(&self) -> &[u8] {
        &self.data
    }
}

// unsafe impl<'a> UnusedState for &'a CStr {
//     type Plain = <&'static [u8] as UnusedState>::Plain;
//     const NUM: usize = <&'static [u8] as UnusedState>::NUM;
// 
//     fn unused_state(n: usize) -> Self::Plain {
//         <&'static [u8] as UnusedState>::unused_state(n)
//     }
// }
// 
// unsafe impl<'a> UnusedState for &'a mut CStr {
//     type Plain = <&'static mut [u8] as UnusedState>::Plain;
//     const NUM: usize = <&'static mut [u8] as UnusedState>::NUM;
// 
//     fn unused_state(n: usize) -> Self::Plain {
//         <&'static mut [u8] as UnusedState>::unused_state(n)
//     }
// }

impl Deref for CStr {
    type Target = NoNullStr;
    fn deref(&self) -> &NoNullStr {
        self.as_ref()
    }
}

impl DerefMut for CStr {
    fn deref_mut(&mut self) -> &mut NoNullStr {
        self.as_mut()
    }
}

impl AsRef<[u8]> for CStr {
    fn as_ref(&self) -> &[u8] {
        &self.data[..self.data.len()-1]
    }
}

impl Index<usize> for CStr {
    type Output = u8;
    fn index(&self, idx: usize) -> &u8 {
        &self.data[idx]
    }
}

impl Index<RangeFull> for CStr {
    type Output = CStr;
    fn index(&self, _: RangeFull) -> &CStr {
        self
    }
}

impl IndexMut<RangeFull> for CStr {
    fn index_mut(&mut self, _: RangeFull) -> &mut CStr {
        self
    }
}

impl Index<RangeFrom<usize>> for CStr {
    type Output = CStr;
    fn index(&self, idx: RangeFrom<usize>) -> &CStr {
        unsafe { CStr::from_bytes_unchecked(&self.data[idx]) }
    }
}

impl IndexMut<RangeFrom<usize>> for CStr {
    fn index_mut(&mut self, idx: RangeFrom<usize>) -> &mut CStr {
        unsafe { CStr::from_mut_bytes_unchecked(&mut self.data[idx]) }
    }
}

impl Index<RangeTo<usize>> for CStr {
    type Output = NoNullStr;
    fn index(&self, idx: RangeTo<usize>) -> &NoNullStr {
        unsafe { NoNullStr::from_bytes_unchecked(&self.data[idx]) }
    }
}

impl IndexMut<RangeTo<usize>> for CStr {
    fn index_mut(&mut self, idx: RangeTo<usize>) -> &mut NoNullStr {
        unsafe { NoNullStr::from_mut_bytes_unchecked(&mut self.data[idx]) }
    }
}

impl Index<Range<usize>> for CStr {
    type Output = NoNullStr;
    fn index(&self, idx: Range<usize>) -> &NoNullStr {
        unsafe { NoNullStr::from_bytes_unchecked(&self.data[idx]) }
    }
}

impl IndexMut<Range<usize>> for CStr {
    fn index_mut(&mut self, idx: Range<usize>) -> &mut NoNullStr {
        unsafe { NoNullStr::from_mut_bytes_unchecked(&mut self.data[idx]) }
    }
}

impl AsNoNullStr for CStr {
    fn as_no_null_str(&self) -> Result<&NoNullStr> {
        Ok(self.as_ref())
    }
}

impl AsMutNoNullStr for CStr {
    fn as_mut_no_null_str(&mut self) -> Result<&mut NoNullStr> {
        Ok(self.as_mut())
    }
}

impl AsRef<NoNullStr> for CStr {
    fn as_ref(&self) -> &NoNullStr {
        unsafe { NoNullStr::from_bytes_unchecked(self.as_ref()) }
    }
}

impl AsMut<NoNullStr> for CStr {
    fn as_mut(&mut self) -> &mut NoNullStr {
        let len = self.data.len() - 1;
        unsafe { NoNullStr::from_mut_bytes_unchecked(&mut self.data[..len]) }
    }
}

impl Debug for CStr {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        Debug::fmt(self.as_byte_str(), w)
    }
}

impl Parse for CStr {
    fn parse<P: Parsable>(&self) -> Result<P> {
        self.as_byte_str().parse()
    }
}

fn bytes_are_valid(b: &[u8]) -> Result<usize> {
    match memchr(b, 0) {
        Some(idx) => {
            if idx == b.len() - 1 || all_bytes(&b[idx+1..], 0) {
                Ok(idx)
            } else {
                Err(error::InvalidArgument)
            }
        },
        _ => Err(error::InvalidArgument),
    }
}

impl AsCStr for [u8] {
    fn as_cstr(&self) -> Result<&CStr> {
        let idx = try!(bytes_are_valid(self));
        Ok(unsafe { CStr::from_bytes_unchecked(&self[..idx+1]) })
    }
}

impl AsCStr for CStr {
    fn as_cstr(&self) -> Result<&CStr> {
        Ok(self)
    }
}

impl AsCStr for [i8] {
    fn as_cstr(&self) -> Result<&CStr> {
        let bytes: &[u8] = self.as_ref();
        bytes.as_cstr()
    }
}

impl AsCStr for str {
    fn as_cstr(&self) -> Result<&CStr> {
        let bytes: &[u8] = self.as_ref();
        bytes.as_cstr()
    }
}

impl AsMutCStr for [u8] {
    fn as_mut_cstr(&mut self) -> Result<&mut CStr> {
        let idx = try!(bytes_are_valid(self));
        Ok(unsafe { CStr::from_mut_bytes_unchecked(&mut self[..idx+1]) })
    }
}

impl AsMutCStr for CStr {
    fn as_mut_cstr(&mut self) -> Result<&mut CStr> {
        Ok(self)
    }
}

impl AsMutCStr for [i8] {
    fn as_mut_cstr(&mut self) -> Result<&mut CStr> {
        let bytes: &mut [u8] = self.as_mut();
        bytes.as_mut_cstr()
    }
}

impl ToCStr for CStr {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        let bytes = &self.data;
        if bytes.len() <= buf.len() {
            mem::copy(buf, bytes);
            Ok(unsafe { CStr::from_mut_bytes_unchecked(&mut buf[..bytes.len()]) })
        } else {
            Err(error::NoMemory)
        }
    }

    fn to_or_as_cstr<'a>(&'a self, _: &'a mut [u8]) -> Result<&'a CStr> {
        Ok(self)
    }

    fn to_or_as_mut_cstr<'a>(&'a mut self, _: &'a mut [u8]) -> Result<&'a mut CStr> {
        Ok(self)
    }
}

fn bytes_to_or_as_cstr(b: &[u8], buf: &mut [u8]) -> Result<(*const u8, usize)> {
    if let Some(idx) = memchr(b, 0) {
        if idx == b.len() - 1 || all_bytes(&b[idx+1..], 0) {
            Ok((b.as_ptr(), idx+1))
        } else {
            Err(error::InvalidArgument)
        }
    } else if b.len() >= buf.len() {
        Err(error::NoMemory)
    } else {
        mem::copy(buf, b);
        buf[b.len()] = 0;
        Ok((buf.as_ptr(), b.len() + 1))
    }
}

impl ToCStr for [u8] {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        let len = match memchr(self, 0) {
            Some(idx) => {
                if idx == self.len() - 1 || all_bytes(&self[idx+1..], 0) {
                    idx
                } else {
                    return Err(error::InvalidArgument);
                }
            },
            _ => self.len(),
        };
        if len >= buf.len() {
            return Err(error::NoMemory);
        }
        mem::copy(buf, &self[..len]);
        buf[len] = 0;
        Ok(unsafe { CStr::from_mut_bytes_unchecked(&mut buf[..len+1]) })
    }

    fn to_or_as_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a CStr> {
        let (ptr, len) = try!(bytes_to_or_as_cstr(self, buf));
        Ok(unsafe { CStr::from_bytes_unchecked(slice::from_ptr(ptr, len)) })
    }

    fn to_or_as_mut_cstr<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        let (ptr, len) = try!(bytes_to_or_as_cstr(self, buf));
        Ok(unsafe { CStr::from_mut_bytes_unchecked(slice::from_ptr(ptr, len)) })
    }
}

impl ToCStr for [i8] {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        let bytes: &[u8] = self.as_ref();
        bytes.to_cstr(buf)
    }

    fn to_or_as_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a CStr> {
        let bytes: &[u8] = self.as_ref();
        bytes.to_or_as_cstr(buf)
    }

    fn to_or_as_mut_cstr<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        let bytes: &mut [u8] = self.as_mut();
        bytes.to_or_as_mut_cstr(buf)
    }
}

impl ToCStr for str {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        self.as_bytes().to_cstr(buf)
    }

    fn to_or_as_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a CStr> {
        self.as_bytes().to_or_as_cstr(buf)
    }

    fn to_or_as_mut_cstr<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        self.as_bytes().to_cstr(buf)
    }
}

impl<'b, T: ToCStr+?Sized> ToCStr for &'b T {
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        (**self).to_cstr(buf)
    }

    fn to_or_as_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a CStr> {
        (**self).to_or_as_cstr(buf)
    }

    fn to_or_as_mut_cstr<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        (**self).to_cstr(buf)
    }
}
