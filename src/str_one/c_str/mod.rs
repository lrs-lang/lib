// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem, slice};
use arch_fns::{strlen};
use cty_base::types::{c_char};
use fmt::{Display, Debug, Write};
use parse::{Parse, Parsable};

use byte_str::{ByteStr};
use no_null_str::{NoNullStr};

pub use self::to_cstr::{ToCStr};

mod to_cstr;
mod index;

/// A byte slice that has exactly one null byte at the very end.
pub struct CStr([u8]);

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
        mem::cast(slice::from_ptr(ptr, strlen(ptr as *const _)))
    }

    /// Returns an empty `CStr`.
    pub fn empty() -> &'static CStr {
        static EMPTY: [u8; 1] = [0];
        unsafe { mem::cast(&EMPTY[..0]) }
    }

    /// Returns a pointer to the first byte in the `CStr`.
    pub fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr() as *const c_char
    }

    /// Returns a mutable pointer to the first byte in the `CStr`.
    pub fn as_mut_ptr(&mut self) -> *mut c_char {
        self.0.as_mut_ptr() as *mut c_char
    }

    /// Returns the contained bytes including the null byte.
    pub fn bytes_with_null(&self) -> &[u8] {
        unsafe { slice::from_ptr(self.0.as_ptr(), self.len() + 1) }
    }

    /// Returns the contained bytes including the null byte.
    pub unsafe fn bytes_with_null_mut(&mut self) -> &mut [u8] {
        slice::from_ptr(self.0.as_mut_ptr(), self.len() + 1)
    }

    /// Returns a `CStr` that consists of the segment after the last '/'.
    pub fn file(&self) -> &CStr {
        self.split_path().1
    }

    /// Returns a mutable `NoNullStr` that consists of the segment after the last '/'.
    pub fn file_mut(&mut self) -> &mut CStr {
        self.split_path_mut().1
    }

    /// Splits the string into its directory and file components.
    pub fn split_path(&self) -> (&NoNullStr, &CStr) {
        unsafe { mem::cast(self.deref().split_path()) }
    }

    /// Splits the string into its directory and file components.
    pub fn split_path_mut(&mut self) -> (&mut NoNullStr, &mut CStr) {
        unsafe { mem::cast(self.deref_mut().split_path_mut()) }
    }
}

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

impl Debug for CStr {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        let bs: &ByteStr = self.as_ref();
        Debug::fmt(bs, w)
    }
}

impl Display for CStr {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        let bs: &ByteStr = self.as_ref();
        Display::fmt(bs, w)
    }
}

impl Parse for CStr {
    fn parse<P: Parsable>(&self) -> Result<P> {
        let bs: &ByteStr = self.as_ref();
        bs.parse()
    }
}
