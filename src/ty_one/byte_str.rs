// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::prelude::*;
use core::{mem};
use core::ops::{Eq};

use c_str::{CStr};
use parse::{Parse};

/// A borrowed byte sequence that can be interpreted as a string but doesn't necessarily
/// contain UTF-8.
pub struct ByteStr {
    inner: [u8],
}

impl ByteStr {
    /// Tries to interpret the contained slice as UTF-8.
    pub fn as_str(&self) -> Option<&str> {
        str::from_bytes(self.as_bytes())
    }

    /// Returns a reference to the contained slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner
    }

    /// Tries to parse the byte string into a type `P`.
    pub fn parse<P: Parse>(&self) -> Option<P> {
        P::parse(self.as_bytes())
    }
}

impl Eq for ByteStr {
    fn eq(&self, other: &ByteStr) -> bool {
        self.as_bytes()[..].eq(&other.as_bytes()[..])
    }
}

/// A trait for converting to a borrowed linux string.
pub trait AsByteStr {
    fn as_byte_str(&self) -> &ByteStr;
}

impl AsByteStr for ByteStr { fn as_byte_str(&self) -> &ByteStr { self                          } }
impl AsByteStr for [i8]    { fn as_byte_str(&self) -> &ByteStr { unsafe { mem::cast(self) }    } }
impl AsByteStr for [u8]    { fn as_byte_str(&self) -> &ByteStr { unsafe { mem::cast(self) }    } }
impl AsByteStr for str     { fn as_byte_str(&self) -> &ByteStr { self.as_bytes().as_byte_str() } }
impl AsByteStr for CStr    { fn as_byte_str(&self) -> &ByteStr { self.as_bytes().as_byte_str() } }

impl<'a, T: AsByteStr+?Sized> AsByteStr for &'a T {
    fn as_byte_str(&self) -> &ByteStr { (*self).as_byte_str() }
}

/// A trait for converting to a mutably borrowed linux string.
pub trait AsByteStrMut: AsByteStr {
    fn as_byte_str_mut(&mut self) -> &mut ByteStr;
}

impl AsByteStrMut for ByteStr { fn as_byte_str_mut(&mut self) -> &mut ByteStr { self   } }
impl AsByteStrMut for [u8]    { fn as_byte_str_mut(&mut self) -> &mut ByteStr { unsafe { mem::cast(self) } } }
